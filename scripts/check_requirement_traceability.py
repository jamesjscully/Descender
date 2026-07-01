#!/usr/bin/env python3
"""Check that blockchain requirements trace to consolidated tracker rows.

This also guards selected normative spec sections that intentionally describe
unfinished evidence work. Those sections must name the tracker rows that own the
work, so the spec spine does not accumulate shadow TODOs.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "spec/blockchain_requirements.md"
SECURITY_MODEL = ROOT / "spec/security_model.md"
RUST_PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
TRACKER = ROOT / "TRACKER.md"
ETHEREUM_POSITIONING = ROOT / "research/ethereum_account_abstraction_positioning.md"

TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")
TRACKER_REF = re.compile(r"`([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3})`")
READINESS_STATUSES = {"not_ready", "partial", "ready"}
OBJECTIVE_PHRASES = [
    "not trying to be only a smaller post-quantum `CHECKSIG`",
    "authorization stack a chain, rollup, bridge, or custody system can adopt",
]
REQUIREMENT_CLASS_CONTRACTS = [
    ("`direct`", "required for Mode A single-slot deployment"),
    ("`mode_b`", "required for aggregation, rollups, committees, or light clients"),
    ("`policy`", "required above the slot abstraction in account/state logic"),
]
NON_CLAIMS_CONTRACTS = [
    "native BLS-style aggregation",
    "Schnorr-style adaptor signatures",
    "threshold signing as one compact native Descender signature",
    "validator finality or bridge safety without explicit account/committee state",
    "deployment-ready hard 128-bit security before q128 KAT freeze, external review, and production-gate acceptance",
    "deployability based only on internal notes or prototype tests",
]
PRODUCT_BAR_LINES = [
    "post-quantum spend authorization",
    "+ account-state lifecycle safety",
    "+ policy expressiveness",
    "+ optional proof aggregation",
    "+ conservative signer/verifier engineering",
    "+ explicit adoption evidence",
]
DIRECT_THEOREM_PREFLIGHT_MARKERS = [
    "PRODUCTION_DEPLOYMENT_BLOCKER_DIRECT_THEOREM_COMPOSITION",
    "PATH2_DIRECT_THEOREM_BOUNDARY_REPORT_DOMAIN",
    "PATH2_DIRECT_QROM_ACTION_BOUNDARY_REPORT_DOMAIN",
    "PATH2_DIRECT_LIFECYCLE_BOUNDARY_REPORT_DOMAIN",
    "Path2DirectTheoremBoundaryReportV1",
    "Path2DirectQromActionBoundaryReportV1",
    "Path2DirectLifecycleBoundaryReportV1",
    "Path2DirectTheoremObligationRowV1",
    "direct_qrom_action_boundary_report_digest",
    "direct_lifecycle_boundary_report_digest",
    "PATH2_DIRECT_QROM_ACTION_SUPPORT_FILE",
    "PATH2_DIRECT_LIFECYCLE_SUPPORT_FILE",
    "PATH2_DIRECT_LIFECYCLE_THEOREM_TARGET",
    "PATH2_DIRECT_LIFECYCLE_EXECUTABLE_TESTS",
    "PATH2_DIRECT_THEOREM_TARGET",
    "PATH2_DIRECT_THEOREM_CONCLUSION",
    "PATH2_DIRECT_THEOREM_ACCEPTED_TUPLE_FIELDS",
    "PATH2_DIRECT_THEOREM_ACCEPTANCE_HYPOTHESES",
    "PATH2_DIRECT_THEOREM_NAMED_BREAK_CLASSES",
    "PATH2_DIRECT_THEOREM_OBLIGATION_AUDIT_CASES",
    "PATH2_DIRECT_THEOREM_OBLIGATION_ROWS",
    "consensus_rejection_or_state_mutation_break",
    "path2_direct_qrom_action_boundary_report_v1",
    "validate_path2_direct_qrom_action_boundary_report_v1",
    "path2_direct_lifecycle_boundary_report_v1",
    "validate_path2_direct_lifecycle_boundary_report_v1",
    "path2_direct_theorem_boundary_report_v1",
    "validate_path2_direct_theorem_boundary_report_v1",
    "production_deployment_direct_theorem_blockers",
]
ACCOUNT_CREATION_PROOF_BOUNDARY_PREFLIGHT_MARKERS = [
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_CREATION_PROOF_BOUNDARY",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_REPORT_DOMAIN",
    "PATH2_ACCOUNT_CREATION_SAMPLER_TIMING_REPORT_DOMAIN",
    "PATH2_ACCOUNT_CREATION_FRAME_KAT_BOUNDARY_REPORT_DOMAIN",
    "PATH2_ACCOUNT_CREATION_NO_EXPORT_BOUNDARY_REPORT_DOMAIN",
    "Path2AccountCreationProofBoundaryReportV1",
    "Path2AccountCreationFrameKatBoundaryReportV1",
    "Path2AccountCreationProofBoundaryRequirementCoverageRowV1",
    "Path2AccountCreationSamplerTimingReportV1",
    "Path2AccountCreationNoExportBoundaryReportV1",
    "sampler_timing_report_digest",
    "no_export_boundary_report_digest",
    "account_stark_residual_matrix_report_digest",
    "PATH2_ACCOUNT_CREATION_SAMPLER_SUPPORT_FILE",
    "PATH2_ACCOUNT_CREATION_TIMING_SUPPORT_FILE",
    "PATH2_ACCOUNT_CREATION_NO_EXPORT_SUPPORT_FILE",
    "PATH2_ACCOUNT_CREATION_SAMPLER_THEOREM_TARGET",
    "PATH2_ACCOUNT_CREATION_SAMPLER_CONSTRUCTION_STEPS",
    "PATH2_ACCOUNT_CREATION_TIMING_ORDER_STEPS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_TARGET",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_CONCLUSION",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_REQUIREMENTS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_LEAF_EVIDENCE_BINDING_FIELDS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_CERTIFICATE_EVIDENCE_BINDING_FIELDS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_BINDING_LEAF_CONTEXT_FIELDS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_PAYLOAD_BINDING_FIELDS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_DEPLOYED_ANCHORS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_NO_EXPORT_PRIVATE_MATERIAL_FIELDS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_REQUIREMENT_COVERAGE_ROWS",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_NAMED_BREAK_CLASSES",
    "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_OBLIGATION_ROWS",
    "requirement_coverage_rows",
    "requirement_coverage_matrix_pinned",
    "every_accepting_requirement_has_coverage_row",
    "every_vector_covered_requirement_has_public_rejection_class",
    "every_current_vector_case_is_known_kat_row",
    "every_requirement_public_rejection_class_is_known",
    "proof_obligation_rows_pinned",
    "every_proof_obligation_has_named_break",
    "every_open_proof_obligation_blocks_deployment",
    "path2_account_creation_sampler_timing_report_v1",
    "validate_path2_account_creation_sampler_timing_report_v1",
    "path2_account_creation_no_export_boundary_report_v1",
    "validate_path2_account_creation_no_export_boundary_report_v1",
    "path2_account_creation_proof_boundary_report_v1",
    "validate_path2_account_creation_proof_boundary_report_v1",
    "production_deployment_account_creation_proof_boundary_blockers",
]
ACCOUNT_STARK_RESIDUAL_PREFLIGHT_MARKERS = [
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_PAYLOAD_KATS",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_AIR_REVIEW",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_FRI_AOK_EXTRACTOR",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_SIMULATOR",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_FRI_QROM_RESIDUAL",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_PRODUCTION_GATE",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_LEGACY_ROUTE_ADR",
    "PATH2_ACCOUNT_STARK_FRI_QROM_RESIDUAL_REPORT_DOMAIN",
    "Path2AccountStarkFriQromResidualReportV1",
    "path2_account_stark_fri_qrom_residual_report_v1",
    "validate_path2_account_stark_fri_qrom_residual_report_v1",
    "PATH2_ACCOUNT_STARK_FRI_AOK_BOUNDARY_REPORT_DOMAIN",
    "PATH2_ACCOUNT_STARK_FRI_AOK_SUPPORT_FILE",
    "Path2AccountStarkFriAokBoundaryReportV1",
    "fri_aok_boundary_report_digest",
    "path2_account_stark_fri_aok_boundary_report_v1",
    "validate_path2_account_stark_fri_aok_boundary_report_v1",
    "PATH2_ACCOUNT_STARK_RESIDUAL_MATRIX_REPORT_DOMAIN",
    "Path2AccountStarkResidualMatrixReportV1",
    "Path2AccountStarkResidualMatrixRowV1",
    "PATH2_ACCOUNT_STARK_RESIDUAL_MATRIX_ROWS",
    "PATH2_ACCOUNT_STARK_PAYLOAD_KAT_BOUNDARY_REPORT_DOMAIN",
    "PATH2_ACCOUNT_STARK_PAYLOAD_BYTE_MODEL_REPORT_DOMAIN",
    "Path2AccountStarkPayloadByteModelReportV1",
    "Path2AccountStarkPayloadByteModelFieldV1",
    "PATH2_ACCOUNT_STARK_PAYLOAD_BYTE_MODEL_FIELDS",
    "path2_account_stark_payload_byte_model_report_v1",
    "validate_path2_account_stark_payload_byte_model_report_v1",
    "Path2AccountStarkPayloadKatBoundaryReportV1",
    "Path2AccountStarkPayloadKatBoundaryRowV1",
    "PATH2_ACCOUNT_STARK_PAYLOAD_KAT_BOUNDARY_ROWS",
    "path2_account_stark_payload_kat_boundary_report_v1",
    "validate_path2_account_stark_payload_kat_boundary_report_v1",
    "path2_account_stark_payload_public_rejection_class",
    "path2_account_stark_residual_matrix_report_v1",
    "validate_path2_account_stark_residual_matrix_report_v1",
    "production_deployment_account_stark_residual_blockers",
]
ACCOUNT_STARK_PAYLOAD_BYTE_MODEL_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_stark_payload_byte_model_manifest.rs": [
        "Descender.Path2.AccountStarkPayloadByteModelManifest.v1",
        "Descender.Path2.AccountStarkPayloadByteModel.v1",
        "path2_account_stark_payload_byte_model_report_v1",
        "validate_path2_account_stark_payload_byte_model_report_v1",
        "payload_byte_model_report_digest",
        "fixed_frame_overhead_excluding_bundle",
        "deterministic_kat_digest_pins_bundle_bytes",
        "full_proof_bytes_inlined_in_manifest",
    ],
    "tools/parameter-search/src/bin/path2_account_stark_payload_byte_model_consistency.rs": [
        "formal/kat/path2_account_stark_payload_byte_model_v1.json",
        "formal/kat/path2_account_stark_payload_kat_vectors_v1.json",
        "path2_account_stark_payload_byte_model_report_v1",
        "payload_frame_function",
        "fixed_frame_overhead_excluding_bundle",
        "payload_digest_error",
        "Path 2 account-STARK payload byte model consistency check passed.",
    ],
    "formal/kat/path2_account_stark_payload_byte_model_v1.json": [
        "Descender.Path2.AccountStarkPayloadByteModelManifest.v1",
        "Descender.Path2.AccountStarkPayloadByteModel.v1",
        "draft_byte_model_manifest",
        "payload_byte_model_report_digest",
        "fixed_frame_overhead_excluding_bundle",
        '"deterministic_kat_digest_pins_bundle_bytes": true',
        '"full_proof_bytes_inlined_in_manifest": false',
        '"public_payload_exports_private_material": false',
        '"deployment_ready": false',
    ],
    "scripts/check_build.sh": [
        "account STARK payload byte model manifest check",
        "path2_account_stark_payload_byte_model_manifest",
        "path2_account_stark_payload_byte_model_v1.json",
        "account STARK payload byte model consistency check",
        "path2_account_stark_payload_byte_model_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account STARK payload byte model manifest check",
        "path2_account_stark_payload_byte_model_manifest",
        "path2_account_stark_payload_byte_model_v1.json",
        "Account STARK payload byte model consistency check",
        "path2_account_stark_payload_byte_model_consistency",
    ],
}
ACCOUNT_STARK_PAYLOAD_KAT_MANIFEST_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_stark_payload_kat_manifest.rs": [
        "Descender.Path2.AccountStarkPayloadKatBoundaryManifest.v1",
        "Descender.Path2.AccountStarkPayloadKatBoundary.v1",
        "path2_account_stark_payload_byte_model_report_v1",
        "path2_account_stark_payload_kat_boundary_report_v1",
        "path2_account_stark_residual_matrix_report_v1",
        "payload_byte_model_report_digest",
        "payload_kat_boundary_report_digest",
        "payload_kat_boundary_report_byte_model_digest",
        "byte_model_snapshot",
        "frozen_corpus_complete",
        "public_rejection_class",
    ],
    "formal/kat/path2_account_stark_payload_kat_boundary_v1.json": [
        "Descender.Path2.AccountStarkPayloadKatBoundaryManifest.v1",
        "Descender.Path2.AccountStarkPayloadKatBoundary.v1",
        "draft_boundary_manifest",
        "payload_byte_model_report_digest",
        "payload_kat_boundary_report_digest",
        "payload_kat_boundary_report_byte_model_digest",
        "byte_model_snapshot",
        '"frozen_corpus_complete": false',
        "public_rejection_class",
    ],
    "scripts/check_build.sh": [
        "path2_account_stark_payload_kat_manifest",
        "path2_account_stark_payload_kat_boundary_v1.json",
    ],
    ".github/workflows/ci.yml": [
        "path2_account_stark_payload_kat_manifest",
        "path2_account_stark_payload_kat_boundary_v1.json",
    ],
}
ACCOUNT_STARK_FRI_QROM_RESIDUAL_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_stark_fri_qrom_residual_manifest.rs": [
        "Descender.Path2.AccountStarkFriQromResidualManifest.v1",
        "Descender.Path2.AccountStarkFriQromResidual.v1",
        "path2_account_stark_fri_qrom_residual_report_v1",
        "account_stark_fri_qrom_residual_report_digest",
        "checked_core_qrom_budget",
        "qrom_domain_vector_case_ids",
        "qrom_domain_vector_case_count",
        "account_stark_transcript_qrom_theorem_instantiated",
        "residual_pricing_complete",
    ],
    "tools/parameter-search/src/bin/path2_account_stark_fri_qrom_domain_vectors.rs": [
        "Descender.Path2.AccountStarkFriQromDomainVectors.v1",
        "Descender.Path2.AccountStarkFriQromDomainBoundaryVectors.v1",
        "digest_pinned_fri_qrom_domain_audit_vectors",
        "path2_account_stark_fri_qrom_residual_report_v1",
        "validate_path2_account_stark_fri_qrom_residual_report_v1",
        "fri_qrom_selected_proof_family_domain",
        "fri_qrom_poseidon2_width_and_sponge_floor",
        "fri_qrom_challenge_field_and_schedule",
        "fri_qrom_checked_core_budget_import",
        "fri_qrom_open_theorem_flags_fail_closed",
        "account_stark_transcript_qrom_theorem_instantiated",
        "deployment_ready",
    ],
    "tools/parameter-search/src/bin/path2_account_stark_fri_qrom_domain_vector_consistency.rs": [
        "formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json",
        "path2_account_stark_fri_qrom_residual_report_v1",
        "validate_path2_account_stark_fri_qrom_residual_report_v1",
        "digest_pinned_fri_qrom_domain_audit_vectors",
        "checked_core_qrom_artifacts",
        "Path 2 account-STARK FRI/QROM domain vector consistency check passed.",
    ],
    "tools/parameter-search/src/bin/path2_account_stark_fri_qrom_residual_consistency.rs": [
        "path2_account_stark_fri_qrom_residual_report_v1",
        "validate_path2_account_stark_fri_qrom_residual_report_v1",
        "PROD_LOG_BLOWUP",
        "PROD_NUM_QUERIES",
        "checked_core_qrom_artifacts",
        "qrom_domain_vector_case_ids",
        "qrom_domain_vector_case_count",
        "Path 2 account-STARK FRI/QROM residual consistency check passed.",
    ],
    "formal/kat/path2_account_stark_fri_qrom_residual_v1.json": [
        "Descender.Path2.AccountStarkFriQromResidualManifest.v1",
        "draft_residual_pricing_manifest",
        "account_stark_fri_qrom_residual_report_digest",
        "qrom_domain_vector_case_ids",
        '"qrom_domain_vector_case_count": 5',
        "checked-core spend transcript ledger; not a substitute",
        '"conservative_johnson_bits": 256',
        '"poseidon2_sponge_floor_bits": 139',
        '"account_stark_transcript_qrom_theorem_instantiated": false',
        '"residual_pricing_complete": false',
        '"deployment_ready": false',
    ],
    "formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json": [
        "Descender.Path2.AccountStarkFriQromDomainVectors.v1",
        "digest_pinned_fri_qrom_domain_audit_vectors",
        "fri_qrom_selected_proof_family_domain",
        "fri_qrom_poseidon2_width_and_sponge_floor",
        "fri_qrom_challenge_field_and_schedule",
        "fri_qrom_checked_core_budget_import",
        "fri_qrom_open_theorem_flags_fail_closed",
        '"case_count": 5',
        '"poseidon2_sponge_floor_bits": 139',
        '"account_stark_transcript_qrom_theorem_instantiated": false',
        '"residual_pricing_complete": false',
        '"deployment_ready": false',
    ],
    "scripts/check_build.sh": [
        "account STARK FRI/QROM residual manifest check",
        "path2_account_stark_fri_qrom_residual_manifest",
        "path2_account_stark_fri_qrom_residual_v1.json",
        "account STARK FRI/QROM residual consistency check",
        "path2_account_stark_fri_qrom_residual_consistency",
        "account STARK FRI/QROM domain vector corpus check",
        "path2_account_stark_fri_qrom_domain_vectors",
        "path2_account_stark_fri_qrom_domain_vectors_v1.json",
        "account STARK FRI/QROM domain vector consistency check",
        "path2_account_stark_fri_qrom_domain_vector_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account STARK FRI/QROM residual manifest check",
        "path2_account_stark_fri_qrom_residual_manifest",
        "path2_account_stark_fri_qrom_residual_v1.json",
        "Account STARK FRI/QROM residual consistency check",
        "path2_account_stark_fri_qrom_residual_consistency",
        "Account STARK FRI/QROM domain vector corpus check",
        "path2_account_stark_fri_qrom_domain_vectors",
        "path2_account_stark_fri_qrom_domain_vectors_v1.json",
        "Account STARK FRI/QROM domain vector consistency check",
        "path2_account_stark_fri_qrom_domain_vector_consistency",
    ],
}
ACCOUNT_STARK_FRI_AOK_BOUNDARY_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_stark_fri_aok_boundary_manifest.rs": [
        "Descender.Path2.AccountStarkFriAokBoundaryManifest.v1",
        "Descender.Path2.AccountStarkFriAokBoundary.v1",
        "path2_account_stark_fri_aok_boundary_report_v1",
        "validate_path2_account_stark_fri_aok_boundary_report_v1",
        "path2_account_stark_fri_qrom_residual_report_v1",
        "path2_account_stark_residual_matrix_report_v1",
        "account_stark_fri_aok_boundary_report_digest",
        "fri_aok_boundary_report_fri_qrom_digest",
        "residual_matrix_report_fri_aok_digest",
        "air_audit_evidence",
        "fri_aok_extractor_theorem_instantiated",
        "accepting_proof_implies_extractable_witness",
    ],
    "tools/parameter-search/src/bin/path2_account_stark_fri_aok_boundary_consistency.rs": [
        "formal/kat/path2_account_stark_fri_aok_boundary_v1.json",
        "path2_account_stark_fri_aok_boundary_report_v1",
        "validate_path2_account_stark_fri_aok_boundary_report_v1",
        "validate_path2_account_stark_residual_matrix_report_v1",
        "FRI argument-of-knowledge",
        "p2_sis_extractor",
        "FRI-IOP extractor",
        "PROD_LOG_BLOWUP",
        "PROD_NUM_QUERIES",
        "Path 2 account-STARK FRI/AoK boundary consistency check passed.",
    ],
    "formal/kat/path2_account_stark_fri_aok_boundary_v1.json": [
        "Descender.Path2.AccountStarkFriAokBoundaryManifest.v1",
        "Descender.Path2.AccountStarkFriAokBoundary.v1",
        "draft_fri_aok_boundary_manifest",
        "account_stark_fri_aok_boundary_report_digest",
        "fri_aok_boundary_report_fri_qrom_digest",
        "residual_matrix_report_fri_aok_digest",
        "air_audit_evidence",
        '"air_relation_class_count": 8',
        '"med_findings_remediated": 3',
        '"fri_aok_extractor_theorem_instantiated": false',
        '"accepting_proof_implies_extractable_witness": false',
        '"deployment_ready": false',
    ],
    "research/path2_account_stark_air_soundness_audit.md": [
        "No CRITICAL \"false statement accepts\" hole in the AIR.",
        "all 8 relation classes",
        "ALL THREE MED findings remediated",
        "FRI argument-of-knowledge",
        "p2_sis_extractor",
        "FRI-IOP extractor",
    ],
    "scripts/check_build.sh": [
        "account STARK FRI/AoK boundary manifest check",
        "path2_account_stark_fri_aok_boundary_manifest",
        "path2_account_stark_fri_aok_boundary_v1.json",
        "account STARK FRI/AoK boundary consistency check",
        "path2_account_stark_fri_aok_boundary_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account STARK FRI/AoK boundary manifest check",
        "path2_account_stark_fri_aok_boundary_manifest",
        "path2_account_stark_fri_aok_boundary_v1.json",
        "Account STARK FRI/AoK boundary consistency check",
        "path2_account_stark_fri_aok_boundary_consistency",
    ],
}
ACCOUNT_CREATION_SAMPLER_TIMING_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_creation_sampler_timing_manifest.rs": [
        "Descender.Path2.AccountCreationSamplerTimingManifest.v1",
        "Descender.Path2.AccountCreationSamplerTiming.v1",
        "path2_account_creation_sampler_timing_report_v1",
        "validate_path2_account_creation_sampler_timing_report_v1",
        "path2_account_creation_proof_boundary_report_v1",
        "account_creation_sampler_timing_report_digest",
        "proof_boundary_report_sampler_timing_digest",
        "sampler_boundary",
        "timing_boundary",
        "executable_hooks",
        "open_obligations",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_sampler_timing_consistency.rs": [
        "formal/kat/path2_account_creation_sampler_timing_v1.json",
        "research/path2_honest_create_sampler_v1.md",
        "research/path2_no_postchallenge_creation_timing_game.md",
        "path2_account_creation_sampler_timing_report_v1",
        "validate_path2_account_creation_sampler_timing_report_v1",
        "path2_creation_hidden_source_statement_v1_frame",
        "path2_checked_core_authorization_challenge_frame",
        "validate_path2_direct_slot_authorization_state_namespace",
        "verify_path2_prevalidated_slot_consume_once",
        "verify_path2_direct_single_slot_spend_authorization",
        "noPostchallengeTiming_createdBeforeChallenge",
        "noPostchallengeTiming_lateAdmission_excluded",
        "Path 2 account-creation sampler/timing consistency check passed.",
    ],
    "formal/kat/path2_account_creation_sampler_timing_v1.json": [
        "Descender.Path2.AccountCreationSamplerTimingManifest.v1",
        "Descender.Path2.AccountCreationSamplerTiming.v1",
        "draft_sampler_timing_manifest",
        "account_creation_sampler_timing_report_digest",
        "proof_boundary_report_sampler_timing_digest",
        "honest_create_sampler_sound_v1",
        "postchallenge_slot_creation_or_rewrite_break",
        '"creation_statement_excludes_spend_message": true',
        '"checked_core_challenge_binds_state_root": true',
        '"state_namespace_membership_required": true',
        '"consume_once_required": true',
        '"honest_sampler_theorem_instantiated": false',
        '"authorization_class_binding_theorem_instantiated": false',
        '"no_postchallenge_timing_theorem_instantiated": true',
        '"deployment_ready": false',
    ],
    "scripts/check_build.sh": [
        "account-creation sampler timing manifest check",
        "path2_account_creation_sampler_timing_manifest",
        "path2_account_creation_sampler_timing_v1.json",
        "account-creation sampler timing consistency check",
        "path2_account_creation_sampler_timing_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account-creation sampler timing manifest check",
        "path2_account_creation_sampler_timing_manifest",
        "path2_account_creation_sampler_timing_v1.json",
        "Account-creation sampler timing consistency check",
        "path2_account_creation_sampler_timing_consistency",
    ],
}
ACCOUNT_CREATION_NO_EXPORT_BOUNDARY_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_creation_no_export_boundary_manifest.rs": [
        "Descender.Path2.AccountCreationNoExportBoundaryManifest.v1",
        "Descender.Path2.AccountCreationNoExportBoundary.v1",
        "path2_account_creation_no_export_boundary_report_v1",
        "validate_path2_account_creation_no_export_boundary_report_v1",
        "path2_account_creation_proof_boundary_report_v1",
        "account_creation_no_export_boundary_report_digest",
        "proof_boundary_report_no_export_digest",
        "payload_byte_model_report_digest",
        "payload_kat_boundary_report_digest",
        "no_export_boundary",
        "simulator_theorem_target",
        "no_export_theorem_target",
        "no_export_vector_case_ids",
        "no_export_vector_case_count",
        "no_export_vectors_present",
        "simulator_public_input_fields",
        "simulator_input_allowlist_pinned",
        "no_export_private_material_fields",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_no_export_boundary_consistency.rs": [
        "formal/kat/path2_account_creation_no_export_boundary_v1.json",
        "path2_account_creation_no_export_boundary_report_v1",
        "validate_path2_account_creation_no_export_boundary_report_v1",
        "Path2AccountCreationNoExportBoundaryReportV1",
        "zero_knowledge_no_export_semantics",
        "selected_account_stark_public_bundle_simulator_v1",
        "selected_account_stark_no_export_v1",
        "simulator_public_input_fields",
        "no_export_vector_case_ids",
        "no_export_vector_case_count",
        "no_export_vectors_present",
        "account_stark_simulator_or_no_export_break",
        "private_trace_material",
        "Path 2 account-creation no-export boundary consistency check passed.",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_no_export_vectors.rs": [
        "Descender.Path2.AccountCreationNoExportVectors.v1",
        "Descender.Path2.AccountCreationNoExportBoundaryVectors.v1",
        "path2_account_creation_no_export_boundary_report_v1",
        "path2_account_stark_payload_byte_model_report_v1",
        "path2_account_stark_payload_kat_boundary_report_v1",
        "no_export_public_payload_field_allowlist",
        "no_export_simulator_public_input_allowlist",
        "no_export_private_material_denylist_absence",
        "no_export_opaque_bundle_not_manifest_inlined",
        "digest_pinned_no_export_audit_vectors",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_no_export_vector_consistency.rs": [
        "formal/kat/path2_account_creation_no_export_vectors_v1.json",
        "path2_account_creation_no_export_boundary_report_v1",
        "path2_account_stark_payload_byte_model_report_v1",
        "path2_account_stark_payload_kat_boundary_report_v1",
        "digest_pinned_no_export_audit_vectors",
        "all_private_material_absent",
        "Path 2 account-creation no-export vector consistency check passed.",
    ],
    "formal/kat/path2_account_creation_no_export_boundary_v1.json": [
        "Descender.Path2.AccountCreationNoExportBoundaryManifest.v1",
        "Descender.Path2.AccountCreationNoExportBoundary.v1",
        "draft_no_export_boundary_manifest",
        "account_creation_no_export_boundary_report_digest",
        "proof_boundary_report_no_export_digest",
        "payload_byte_model_report_digest",
        "payload_kat_boundary_report_digest",
        "zero_knowledge_no_export_semantics",
        "selected_account_stark_public_bundle_simulator_v1",
        "selected_account_stark_no_export_v1",
        "simulator_public_input_fields",
        "no_export_vector_case_ids",
        '"no_export_vector_case_count": 4',
        '"simulator_input_allowlist_pinned": true',
        '"no_export_vectors_present": true',
        "account_stark_simulator_or_no_export_break",
        '"public_payload_exports_private_material": false',
        '"simulator_theorem_instantiated": false',
        '"no_export_theorem_instantiated": false',
        '"deployment_ready": false',
    ],
    "formal/kat/path2_account_creation_no_export_vectors_v1.json": [
        "Descender.Path2.AccountCreationNoExportVectors.v1",
        "Descender.Path2.AccountCreationNoExportBoundaryVectors.v1",
        "digest_pinned_no_export_audit_vectors",
        "no_export_public_payload_field_allowlist",
        "no_export_simulator_public_input_allowlist",
        "no_export_private_material_denylist_absence",
        "no_export_opaque_bundle_not_manifest_inlined",
        '"all_private_material_absent": true',
        '"public_payload_exports_private_material": false',
        '"full_proof_bytes_inlined_in_manifest": false',
        '"simulator_theorem_instantiated": false',
        '"no_export_theorem_instantiated": false',
        '"deployment_ready": false',
    ],
    "research/path2_fiber_privacy_no_export_v1.md": [
        "Path2AccountCreationNoExportBoundaryReportV1",
        "formal/kat/path2_account_creation_no_export_boundary_v1.json",
        "zero_knowledge_no_export_semantics",
        "selected_account_stark_public_bundle_simulator_v1",
        "selected_account_stark_no_export_v1",
        "simulator_public_input_fields",
        "account_stark_simulator_or_no_export_break",
        "pad_source",
        "private_trace_material",
        "simulator semantics, no-export theorem closure",
    ],
    "scripts/check_build.sh": [
        "account-creation no-export boundary manifest check",
        "path2_account_creation_no_export_boundary_manifest",
        "path2_account_creation_no_export_boundary_v1.json",
        "account-creation no-export boundary consistency check",
        "path2_account_creation_no_export_boundary_consistency",
        "account-creation no-export vector corpus check",
        "path2_account_creation_no_export_vectors",
        "path2_account_creation_no_export_vectors_v1.json",
        "account-creation no-export vector consistency check",
        "path2_account_creation_no_export_vector_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account-creation no-export boundary manifest check",
        "path2_account_creation_no_export_boundary_manifest",
        "path2_account_creation_no_export_boundary_v1.json",
        "Account-creation no-export boundary consistency check",
        "path2_account_creation_no_export_boundary_consistency",
        "Account-creation no-export vector corpus check",
        "path2_account_creation_no_export_vectors",
        "path2_account_creation_no_export_vectors_v1.json",
        "Account-creation no-export vector consistency check",
        "path2_account_creation_no_export_vector_consistency",
    ],
}
ACCOUNT_CREATION_PROOF_BOUNDARY_MANIFEST_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_creation_proof_boundary_manifest.rs": [
        "Descender.Path2.AccountCreationProofBoundaryManifest.v1",
        "Descender.Path2.AccountCreationProofBoundary.v1",
        "path2_account_creation_proof_boundary_report_v1",
        "path2_account_creation_sampler_timing_report_v1",
        "path2_account_creation_no_export_boundary_report_v1",
        "path2_account_stark_fri_qrom_residual_report_v1",
        "path2_account_stark_fri_aok_boundary_report_v1",
        "path2_account_stark_residual_matrix_report_v1",
        "path2_account_stark_payload_kat_boundary_report_v1",
        "proof_boundary_target",
        "proof_boundary_conclusion",
        "account_creation_proof_boundary_report_digest",
        "account_creation_sampler_timing_report_digest",
        "proof_boundary_report_sampler_timing_digest",
        "account_creation_no_export_boundary_report_digest",
        "proof_boundary_report_no_export_digest",
        "account_stark_fri_qrom_residual_report_digest",
        "account_stark_fri_aok_boundary_report_digest",
        "fri_aok_boundary_report_fri_qrom_digest",
        "residual_matrix_report_fri_aok_digest",
        "account_stark_residual_matrix_report_digest",
        "frame_kat_boundary_report_digest",
        "payload_kat_boundary_report_digest",
        "leaf_evidence_binding_fields",
        "requirement_coverage_rows",
        "vector_evidence_kind",
        "current_vector_covered_requirement_count",
        "every_accepting_requirement_has_coverage_row",
        "every_current_vector_case_is_known_kat_row",
        "proof_obligation_named_break_classes",
        "proof_obligation_audit_vector_case_ids",
        "proof_obligation_audit_vector_case_count",
        "proof_obligation_rows",
        "proof_obligation_rows_pinned",
        "proof_boundary_target",
        "proof_boundary_conclusion",
        "every_proof_obligation_has_named_break",
        "every_open_proof_obligation_blocks_deployment",
        "no_export_boundary_snapshot",
        "simulator_theorem_target",
        "no_export_theorem_target",
        "simulator_public_input_fields",
        "simulator_input_allowlist_pinned",
        "sampler_timing_snapshot",
        "selected_account_stark_residual_matrix",
        "fri_aok_boundary_snapshot",
        "fri_qrom_residual_snapshot",
    ],
    "formal/kat/path2_account_creation_proof_boundary_v1.json": [
        "Descender.Path2.AccountCreationProofBoundaryManifest.v1",
        "Descender.Path2.AccountCreationProofBoundary.v1",
        "draft_proof_boundary_manifest",
        "selected_account_creation_proof_boundary_v1",
        "accepted_account_creation_payload_implies_bound_leaf_evidence_selected_account_stark_acceptance_no_export_or_named_break",
        "proof_boundary_target",
        "proof_boundary_conclusion",
        "account_creation_proof_boundary_report_digest",
        "account_creation_sampler_timing_report_digest",
        "proof_boundary_report_sampler_timing_digest",
        "account_creation_no_export_boundary_report_digest",
        "proof_boundary_report_no_export_digest",
        "account_stark_fri_qrom_residual_report_digest",
        "account_stark_fri_aok_boundary_report_digest",
        "fri_aok_boundary_report_fri_qrom_digest",
        "residual_matrix_report_fri_aok_digest",
        "account_stark_residual_matrix_report_digest",
        "frame_kat_boundary_report_digest",
        "payload_kat_boundary_report_digest",
        "leaf_evidence_binding_fields",
        "requirement_coverage_rows",
        '"requirement_coverage_count": 10',
        '"current_vector_covered_requirement_count": 10',
        "vector_evidence_kind",
        "audit_vectors",
        "no_export_public_payload_field_allowlist",
        "no_export_simulator_public_input_allowlist",
        "no_export_private_material_denylist_absence",
        "no_export_opaque_bundle_not_manifest_inlined",
        "fri_qrom_selected_proof_family_domain",
        "fri_qrom_poseidon2_width_and_sponge_floor",
        "fri_qrom_challenge_field_and_schedule",
        "fri_qrom_checked_core_budget_import",
        "fri_qrom_open_theorem_flags_fail_closed",
        '"every_accepting_requirement_has_coverage_row": true',
        '"every_current_vector_case_is_known_kat_row": true',
        '"requirement_coverage_matrix_pinned": true',
        "canonical_leaf_and_evidence_framing",
        "payload_context_binding_leaf_proof_aggregation_slot",
        "deterministic_public_rejection",
        "proof_obligation_named_break_classes",
        "proof_obligation_audit_vector_case_ids",
        '"proof_obligation_audit_vector_case_count": 8',
        "proof_obligation_rows",
        "creation_proof_obligation_honest_sampler",
        "creation_proof_obligation_authorization_class_binding",
        "creation_proof_obligation_no_postchallenge_timing",
        "creation_proof_obligation_simulator_no_export",
        "creation_proof_obligation_fri_aok_extractor",
        "creation_proof_obligation_fri_qrom_residual_pricing",
        "creation_proof_obligation_frozen_payload_kats",
        "creation_proof_obligation_production_gate_and_review",
        '"proof_obligation_rows_pinned": true',
        '"every_proof_obligation_has_named_break": true',
        '"every_open_proof_obligation_blocks_deployment": true',
        "no_export_boundary_snapshot",
        "selected_account_stark_public_bundle_simulator_v1",
        "selected_account_stark_no_export_v1",
        "simulator_public_input_fields",
        '"simulator_input_allowlist_pinned": true',
        "sampler_timing_snapshot",
        "fri_aok_boundary_snapshot",
        "postchallenge_slot_creation_or_rewrite_break",
        "account_stark_simulator_or_no_export_break",
        "selected_account_stark_residual_matrix",
        "fri_qrom_residual_snapshot",
        "account_stark_fri_aok_extractor_break",
        "proof_route_authority_break",
        '"residual_pricing_complete": false',
        '"production_gate_fail_closed": true',
        '"deployment_ready": false',
    ],
    "scripts/check_build.sh": [
        "account-creation proof boundary manifest check",
        "path2_account_creation_proof_boundary_manifest",
        "path2_account_creation_proof_boundary_v1.json",
        "account-creation proof-obligation vector corpus check",
        "path2_account_creation_proof_obligation_vectors",
        "path2_account_creation_proof_obligation_vectors_v1.json",
        "account-creation proof-obligation vector consistency check",
        "path2_account_creation_proof_obligation_vector_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account-creation proof boundary manifest check",
        "path2_account_creation_proof_boundary_manifest",
        "path2_account_creation_proof_boundary_v1.json",
        "Account-creation proof-obligation vector corpus check",
        "path2_account_creation_proof_obligation_vectors",
        "path2_account_creation_proof_obligation_vectors_v1.json",
        "Account-creation proof-obligation vector consistency check",
        "path2_account_creation_proof_obligation_vector_consistency",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_proof_obligation_vectors.rs": [
        "Descender.Path2.AccountCreationProofObligationVectors.v1",
        "Descender.Path2.AccountCreationProofObligationAuditVectors.v1",
        "digest_pinned_proof_obligation_audit_vectors",
        "path2_account_creation_proof_boundary_report_v1",
        "path2_direct_theorem_boundary_report_v1",
        "validate_path2_account_creation_proof_boundary_report_v1",
        "validate_path2_direct_theorem_boundary_report_v1",
        "proof_obligation_audit_vector_case_ids",
        "all_open_cases_block_deployment",
        "composition_theorem_instantiated",
        "deployment_ready",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_proof_obligation_vector_consistency.rs": [
        "formal/kat/path2_account_creation_proof_obligation_vectors_v1.json",
        "path2_account_creation_proof_boundary_report_v1",
        "path2_direct_theorem_boundary_report_v1",
        "validate_path2_account_creation_proof_boundary_report_v1",
        "validate_path2_direct_theorem_boundary_report_v1",
        "digest_pinned_proof_obligation_audit_vectors",
        "proof_obligation_audit_vector_case_ids",
        "Path 2 account-creation proof-obligation vector consistency check passed.",
    ],
    "formal/kat/path2_account_creation_proof_obligation_vectors_v1.json": [
        "Descender.Path2.AccountCreationProofObligationVectors.v1",
        "digest_pinned_proof_obligation_audit_vectors",
        "creation_proof_obligation_honest_sampler",
        "creation_proof_obligation_authorization_class_binding",
        "creation_proof_obligation_no_postchallenge_timing",
        "creation_proof_obligation_simulator_no_export",
        "creation_proof_obligation_fri_aok_extractor",
        "creation_proof_obligation_fri_qrom_residual_pricing",
        "creation_proof_obligation_frozen_payload_kats",
        "creation_proof_obligation_production_gate_and_review",
        '"case_count": 8',
        '"open_proof_obligation_count": 7',
        '"closed_proof_obligation_count": 1',
        '"all_cases_open": false',
        '"all_open_cases_block_deployment": true',
        '"composition_theorem_instantiated": false',
        '"deployment_ready": false',
    ],
}
DIRECT_QROM_ACTION_BOUNDARY_MARKERS = {
    "tools/parameter-search/src/bin/path2_direct_qrom_action_boundary_manifest.rs": [
        "Descender.Path2.DirectQromActionBoundaryManifest.v1",
        "Descender.Path2.DirectQromActionBoundary.v1",
        "path2_direct_qrom_action_boundary_report_v1",
        "validate_path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_theorem_boundary_report_v1",
        "direct_qrom_action_boundary_report_digest",
        "direct_theorem_report_qrom_action_digest",
        "numeric_budget",
        "open_obligations",
    ],
    "tools/parameter-search/src/bin/path2_direct_qrom_action_boundary_consistency.rs": [
        "formal/kat/path2_direct_qrom_action_boundary_v1.json",
        "path2_direct_qrom_action_boundary_report_v1",
        "validate_path2_direct_qrom_action_boundary_report_v1",
        "transcript term after QROM loss | `182` bits",
        "minimum deployment margin | `53` bits",
        "PATH2_FQ12_ACTION_COLLISION_FLOOR_BITS",
        "Path 2 direct QROM/action boundary consistency check passed.",
    ],
    "formal/kat/path2_direct_qrom_action_boundary_v1.json": [
        "Descender.Path2.DirectQromActionBoundaryManifest.v1",
        "Descender.Path2.DirectQromActionBoundary.v1",
        "draft_qrom_action_boundary_manifest",
        "direct_qrom_action_boundary_report_digest",
        "direct_theorem_report_qrom_action_digest",
        "qrom_or_domain_separation_break",
        "DESCENDER_PATH2_DIRECT_QUERY_BUDGET_V1",
        '"transcript_term_bits_after_qrom_loss": 182',
        '"minimum_deployment_margin_bits": 53',
        '"action_collision_floor_bits": 163',
        '"direct_mode_q_batch_inactive": true',
        '"qrom_and_action_bounds_linked_to_theorem": false',
        '"deployment_ready": false',
    ],
    "research/path2_hash_qrom_assumption_ledger.md": [
        "Path2DirectQromActionBoundaryReportV1",
        "formal/kat/path2_direct_qrom_action_boundary_v1.json",
        "transcript term after QROM loss | `182` bits",
        "minimum deployment margin | `53` bits",
        "qrom_or_domain_separation_break",
    ],
    "scripts/check_build.sh": [
        "direct QROM/action boundary manifest check",
        "path2_direct_qrom_action_boundary_manifest",
        "path2_direct_qrom_action_boundary_v1.json",
        "direct QROM/action boundary consistency check",
        "path2_direct_qrom_action_boundary_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Direct QROM/action boundary manifest check",
        "path2_direct_qrom_action_boundary_manifest",
        "path2_direct_qrom_action_boundary_v1.json",
        "Direct QROM/action boundary consistency check",
        "path2_direct_qrom_action_boundary_consistency",
    ],
}
DIRECT_LIFECYCLE_BOUNDARY_MARKERS = {
    "tools/parameter-search/src/bin/path2_direct_lifecycle_boundary_manifest.rs": [
        "Descender.Path2.DirectLifecycleBoundaryManifest.v1",
        "Descender.Path2.DirectLifecycleBoundary.v1",
        "path2_direct_lifecycle_boundary_report_v1",
        "validate_path2_direct_lifecycle_boundary_report_v1",
        "path2_direct_theorem_boundary_report_v1",
        "direct_lifecycle_boundary_report_digest",
        "direct_theorem_report_lifecycle_digest",
        "local_successor_state_boundary",
        "open_obligations",
    ],
    "tools/parameter-search/src/bin/path2_direct_lifecycle_boundary_consistency.rs": [
        "formal/kat/path2_direct_lifecycle_boundary_v1.json",
        "research/path2_one_use_successor_state_theorem.md",
        "path2_direct_lifecycle_boundary_report_v1",
        "validate_path2_direct_lifecycle_boundary_report_v1",
        "verify_path2_prevalidated_slot_consume_once",
        "verify_path2_direct_single_slot_spend_authorization",
        "ProductionSpentSlotLedger",
        "ProductionChainStateFileStores",
        "Path 2 direct lifecycle boundary consistency check passed.",
    ],
    "formal/kat/path2_direct_lifecycle_boundary_v1.json": [
        "Descender.Path2.DirectLifecycleBoundaryManifest.v1",
        "Descender.Path2.DirectLifecycleBoundary.v1",
        "draft_lifecycle_boundary_manifest",
        "direct_lifecycle_boundary_report_digest",
        "direct_theorem_report_lifecycle_digest",
        "slot_replay_or_rollback_break",
        "verify_path2_prevalidated_slot_consume_once",
        "verify_path2_direct_single_slot_spend_authorization",
        "ProductionSpentSlotLedger",
        "ProductionChainStateFileStores",
        '"local_successor_state_model_pinned": true',
        '"target_chain_finality_profile_instantiated": true',
        '"lifecycle_transition_profile_verified": true',
        '"deployment_ready": false',
    ],
    "scripts/check_build.sh": [
        "direct lifecycle boundary manifest check",
        "path2_direct_lifecycle_boundary_manifest",
        "path2_direct_lifecycle_boundary_v1.json",
        "direct lifecycle boundary consistency check",
        "path2_direct_lifecycle_boundary_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Direct lifecycle boundary manifest check",
        "path2_direct_lifecycle_boundary_manifest",
        "path2_direct_lifecycle_boundary_v1.json",
        "Direct lifecycle boundary consistency check",
        "path2_direct_lifecycle_boundary_consistency",
    ],
}
DIRECT_THEOREM_BOUNDARY_MANIFEST_MARKERS = {
    "tools/parameter-search/src/bin/path2_direct_theorem_boundary_manifest.rs": [
        "Descender.Path2.DirectTheoremBoundaryManifest.v1",
        "Descender.Path2.DirectTheoremBoundary.v1",
        "path2_direct_theorem_boundary_report_v1",
        "path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_lifecycle_boundary_report_v1",
        "path2_account_creation_proof_boundary_report_v1",
        "path2_account_creation_sampler_timing_report_v1",
        "direct_theorem_target",
        "direct_theorem_conclusion",
        "direct_theorem_boundary_report_digest",
        "direct_qrom_action_boundary_report_digest",
        "direct_theorem_report_qrom_action_digest",
        "direct_lifecycle_boundary_report_digest",
        "direct_theorem_report_lifecycle_digest",
        "account_creation_proof_boundary_report_digest",
        "account_creation_sampler_timing_report_digest",
        "proof_boundary_report_sampler_timing_digest",
        "no_export_boundary_report_digest",
        "direct_qrom_action_boundary",
        "direct_lifecycle_boundary",
        "obligation_audit_vector_case_ids",
        "obligation_audit_vector_case_count",
        "obligation_rows",
        "proof_obligation_rows",
        "proof_obligation_rows_pinned",
        "proof_obligation_audit_vector_case_ids",
        "proof_obligation_audit_vector_case_count",
        "requirement_coverage_rows",
        "frame_kat_boundary_report_digest",
        "current_vector_covered_requirement_count",
        "vector_evidence_kind",
        "every_accepting_requirement_has_coverage_row",
        "every_current_vector_case_is_known_kat_row",
        "every_proof_obligation_has_named_break",
        "every_open_proof_obligation_blocks_deployment",
        "every_hypothesis_has_obligation",
        "leaf_evidence_binding_fields",
    ],
    "formal/kat/path2_direct_theorem_boundary_v1.json": [
        "Descender.Path2.DirectTheoremBoundaryManifest.v1",
        "Descender.Path2.DirectTheoremBoundary.v1",
        "draft_theorem_boundary_manifest",
        "direct_mode_selected_tuple_authorization_theorem_v1",
        "accepted_direct_mode_spend_implies_prior_admission_checked_core_acceptance_consume_once_or_named_break",
        "direct_theorem_target",
        "direct_theorem_conclusion",
        "direct_theorem_boundary_report_digest",
        "direct_qrom_action_boundary_report_digest",
        "direct_theorem_report_qrom_action_digest",
        "direct_lifecycle_boundary_report_digest",
        "direct_theorem_report_lifecycle_digest",
        "account_creation_proof_boundary_report_digest",
        "account_creation_sampler_timing_report_digest",
        "proof_boundary_report_sampler_timing_digest",
        "no_export_boundary_report_digest",
        "direct_qrom_action_boundary",
        "direct_lifecycle_boundary",
        "obligation_rows",
        "proof_obligation_rows",
        "requirement_coverage_rows",
        "frame_kat_boundary_report_digest",
        '"current_vector_covered_requirement_count": 10',
        '"proof_obligation_audit_vector_case_count": 8',
        "vector_evidence_kind",
        "audit_vectors",
        "no_export_public_payload_field_allowlist",
        "fri_qrom_selected_proof_family_domain",
        "fri_qrom_poseidon2_width_and_sponge_floor",
        "fri_qrom_challenge_field_and_schedule",
        "fri_qrom_checked_core_budget_import",
        "fri_qrom_open_theorem_flags_fail_closed",
        '"every_accepting_requirement_has_coverage_row": true',
        '"every_current_vector_case_is_known_kat_row": true',
        '"requirement_coverage_matrix_pinned": true',
        '"proof_obligation_rows_pinned": true',
        "proof_obligation_audit_vector_case_ids",
        "creation_proof_obligation_honest_sampler",
        "creation_proof_obligation_authorization_class_binding",
        "creation_proof_obligation_no_postchallenge_timing",
        "selected_account_creation_proof_boundary_v1",
        "accepted_account_creation_payload_implies_bound_leaf_evidence_selected_account_stark_acceptance_no_export_or_named_break",
        "proof_boundary_target",
        "proof_boundary_conclusion",
        '"every_open_proof_obligation_blocks_deployment": true',
        "consensus_rejection_or_state_mutation_break",
        "qrom_or_domain_separation_break",
        "slot_replay_or_rollback_break",
        "direct_obligation_leaf_canonical_live",
        "direct_obligation_creation_before_challenge",
        "direct_obligation_checked_core_acceptance",
        "direct_obligation_consume_once",
        "direct_obligation_public_rejection",
        "leaf_evidence_binding_fields",
        '"composition_theorem_instantiated": false',
        '"deployment_ready": false',
    ],
    "scripts/check_build.sh": [
        "direct theorem boundary manifest check",
        "path2_direct_theorem_boundary_manifest",
        "path2_direct_theorem_boundary_v1.json",
        "direct theorem obligation vector corpus check",
        "path2_direct_theorem_obligation_vectors",
        "path2_direct_theorem_obligation_vectors_v1.json",
        "direct theorem obligation vector consistency check",
        "path2_direct_theorem_obligation_vector_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Direct theorem boundary manifest check",
        "path2_direct_theorem_boundary_manifest",
        "path2_direct_theorem_boundary_v1.json",
        "Direct theorem obligation vector corpus check",
        "path2_direct_theorem_obligation_vectors",
        "path2_direct_theorem_obligation_vectors_v1.json",
        "Direct theorem obligation vector consistency check",
        "path2_direct_theorem_obligation_vector_consistency",
    ],
    "tools/parameter-search/src/bin/path2_direct_theorem_obligation_vectors.rs": [
        "Descender.Path2.DirectTheoremObligationVectors.v1",
        "Descender.Path2.DirectTheoremObligationAuditVectors.v1",
        "digest_pinned_direct_theorem_obligation_audit_vectors",
        "path2_direct_theorem_boundary_report_v1",
        "path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_lifecycle_boundary_report_v1",
        "path2_account_creation_proof_boundary_report_v1",
        "validate_path2_direct_theorem_boundary_report_v1",
        "obligation_audit_vector_case_ids",
        "all_open_cases_block_deployment",
        "composition_theorem_instantiated",
        "deployment_ready",
    ],
    "tools/parameter-search/src/bin/path2_direct_theorem_obligation_vector_consistency.rs": [
        "formal/kat/path2_direct_theorem_obligation_vectors_v1.json",
        "path2_direct_theorem_boundary_report_v1",
        "path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_lifecycle_boundary_report_v1",
        "path2_account_creation_proof_boundary_report_v1",
        "validate_path2_direct_theorem_boundary_report_v1",
        "digest_pinned_direct_theorem_obligation_audit_vectors",
        "obligation_audit_vector_case_ids",
        "Path 2 direct theorem obligation vector consistency check passed.",
    ],
    "formal/kat/path2_direct_theorem_obligation_vectors_v1.json": [
        "Descender.Path2.DirectTheoremObligationVectors.v1",
        "digest_pinned_direct_theorem_obligation_audit_vectors",
        "direct_obligation_leaf_canonical_live",
        "direct_obligation_evidence_leaf_bindings",
        "direct_obligation_slot_authorization_digest",
        "direct_obligation_creation_before_challenge",
        "direct_obligation_admission_namespace",
        "direct_obligation_checked_core_frame_binding",
        "direct_obligation_checked_core_acceptance",
        "direct_obligation_consume_once",
        "direct_obligation_public_rejection",
        '"case_count": 9',
        '"open_direct_obligation_count": 8',
        '"closed_direct_obligation_count": 1',
        '"all_cases_open": false',
        '"all_open_cases_block_deployment": true',
        '"composition_theorem_instantiated": false',
        '"account_creation_proof_boundary_verified": false',
        '"lifecycle_transition_profile_verified": true',
        '"qrom_and_action_bounds_linked_to_theorem": false',
        '"deployment_ready": false',
    ],
}
BOUNDARY_MANIFEST_CONSISTENCY_MARKERS = {
    "tools/parameter-search/src/bin/path2_boundary_manifest_consistency.rs": [
        "formal/kat/path2_account_creation_proof_boundary_v1.json",
        "formal/kat/path2_direct_theorem_boundary_v1.json",
        "account_creation_proof_boundary_report_digest",
        "sampler_timing_report_digest",
        "account_stark_residual_matrix_report_digest",
        "account_creation_proof_boundary_verified",
        "obligation_audit_vector_case_count",
        "proof_obligation_rows_pinned",
        "current_vector_covered_requirement_count",
        "every_proof_obligation_has_named_break",
        "every_open_proof_obligation_blocks_deployment",
        "selected_account_stark_residual_matrix",
        "fri_aok_boundary_snapshot",
        "payload_kat_boundary_snapshot",
        "Path 2 boundary manifest consistency check passed.",
    ],
    "scripts/check_build.sh": [
        "boundary manifest consistency check",
        "path2_boundary_manifest_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Boundary manifest consistency check",
        "path2_boundary_manifest_consistency",
    ],
}
PREFLIGHT_BOUNDARY_CONSISTENCY_MARKERS = {
    "tools/parameter-search/src/bin/path2_preflight_boundary_consistency.rs": [
        "production_deployment_preflight",
        "production_deployment_preflight_report",
        "production_deployment_preflight_report_digest",
        "validate_production_deployment_preflight_report",
        "path2_direct_theorem_boundary_report_v1",
        "path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_qrom_action_boundary_report_v1_digest",
        "validate_path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_lifecycle_boundary_report_v1",
        "path2_direct_lifecycle_boundary_report_v1_digest",
        "validate_path2_direct_lifecycle_boundary_report_v1",
        "path2_account_creation_proof_boundary_report_v1",
        "path2_account_stark_fri_aok_boundary_report_v1",
        "path2_account_stark_fri_qrom_residual_report_v1",
        "path2_account_stark_residual_matrix_report_v1",
        "validate_path2_account_stark_fri_aok_boundary_report_v1",
        "validate_path2_account_stark_fri_qrom_residual_report_v1",
        "fri_aok_boundary_report_digest",
        "FRI/AoK boundary",
        "checked_core_qrom_budget_pinned",
        "qrom_and_action_bounds_linked_to_theorem",
        "local_successor_state_model_pinned",
        "target_chain_finality_profile_instantiated",
        "production_deployment_direct_theorem_blockers",
        "production_deployment_account_creation_proof_boundary_blockers",
        "production_deployment_account_stark_residual_blockers",
        "Path 2 preflight boundary consistency check passed.",
    ],
    "scripts/check_build.sh": [
        "preflight boundary consistency check",
        "path2_preflight_boundary_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Preflight boundary consistency check",
        "path2_preflight_boundary_consistency",
    ],
}
ACCOUNT_STARK_PAYLOAD_KAT_VECTOR_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_stark_payload_kat_vectors.rs": [
        "Descender.Path2.AccountStarkPayloadKatVectors.v1",
        "path2_account_stark_payload_kat_fixture_v1",
        "path2_account_stark_prove_creation_with_mask_seed_v1",
        "verify_path2_direct_creation_account_stark_payload_boundary",
        "every_current_row_has_generated_vector",
        "deterministic KAT proof seed",
        "single-thread KAT",
        "account_stark_bundle_sha256",
    ],
    "formal/kat/path2_account_stark_payload_kat_vectors_v1.json": [
        "Descender.Path2.AccountStarkPayloadKatVectors.v1",
        "digest_pinned_boundary_vectors",
        "honest_payload_accepts",
        "every_current_row_has_generated_vector",
        "deterministic_kat_mask_seed",
        "single-thread KAT",
        "account_stark_bundle_sha256",
    ],
    "scripts/check_build.sh": [
        "account STARK payload KAT vector corpus check",
        "path2_account_stark_payload_kat_vectors",
        "path2_account_stark_payload_kat_vectors_v1.json",
        "account-creation-stark-verifier",
    ],
    ".github/workflows/ci.yml": [
        "Account STARK payload KAT vector corpus check",
        "path2_account_stark_payload_kat_vectors",
        "path2_account_stark_payload_kat_vectors_v1.json",
        "account-creation-stark-verifier",
    ],
}
ACCOUNT_STARK_PAYLOAD_KAT_VECTOR_CONSISTENCY_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_stark_payload_kat_vector_consistency.rs": [
        "formal/kat/path2_account_stark_payload_kat_boundary_v1.json",
        "formal/kat/path2_account_stark_payload_kat_vectors_v1.json",
        "payload_kat_boundary_report_digest",
        "every_current_row_has_generated_vector",
        "frozen_corpus_complete",
        "deployment_ready",
        "Path 2 account-STARK payload KAT vector consistency check passed.",
    ],
    "scripts/check_build.sh": [
        "account STARK payload KAT vector consistency check",
        "path2_account_stark_payload_kat_vector_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account STARK payload KAT vector consistency check",
        "path2_account_stark_payload_kat_vector_consistency",
    ],
}
ACCOUNT_CREATION_FRAME_KAT_VECTOR_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_creation_frame_kat_manifest.rs": [
        "Descender.Path2.AccountCreationFrameKatBoundaryManifest.v1",
        "path2_account_creation_frame_kat_boundary_report_v1",
        "frame_requirement_case_count",
        "binding_requirement_case_count",
        "frozen_corpus_complete",
        "deployment_ready",
    ],
    "tools/parameter-search/src/bin/path2_account_creation_frame_kat_vectors.rs": [
        "Descender.Path2.AccountCreationFrameKatVectors.v1",
        "path2_account_creation_frame_kat_fixture_v1",
        "verify_path2_prevalidated_slot_creation_frame_bindings",
        "honest_frame_accepts",
        "every_current_row_has_generated_vector",
    ],
    "formal/kat/path2_account_creation_frame_kat_boundary_v1.json": [
        "Descender.Path2.AccountCreationFrameKatBoundaryManifest.v1",
        "leaf_frame_version_mutation",
        "evidence_kind_mutation",
        "prevalidation_transcript_digest_mutation",
        '"row_count": 17',
        '"deployment_ready": false',
    ],
    "formal/kat/path2_account_creation_frame_kat_vectors_v1.json": [
        "Descender.Path2.AccountCreationFrameKatVectors.v1",
        "digest_pinned_boundary_vectors",
        "honest_frame_accepts",
        "leaf_frame_version_mutation",
        "slot_leaf_digest_mutation",
        "prevalidation_transcript_digest_mutation",
    ],
    "scripts/check_build.sh": [
        "account-creation frame KAT boundary manifest check",
        "path2_account_creation_frame_kat_manifest",
        "path2_account_creation_frame_kat_boundary_v1.json",
        "account-creation frame KAT vector corpus check",
        "path2_account_creation_frame_kat_vectors",
        "path2_account_creation_frame_kat_vectors_v1.json",
    ],
    ".github/workflows/ci.yml": [
        "Account-creation frame KAT boundary manifest check",
        "path2_account_creation_frame_kat_manifest",
        "path2_account_creation_frame_kat_boundary_v1.json",
        "Account-creation frame KAT vector corpus check",
        "path2_account_creation_frame_kat_vectors",
        "path2_account_creation_frame_kat_vectors_v1.json",
    ],
}
ACCOUNT_CREATION_FRAME_KAT_CONSISTENCY_MARKERS = {
    "tools/parameter-search/src/bin/path2_account_creation_frame_kat_consistency.rs": [
        "formal/kat/path2_account_creation_frame_kat_boundary_v1.json",
        "formal/kat/path2_account_creation_frame_kat_vectors_v1.json",
        "frame_kat_boundary_report_digest",
        "every_current_row_has_generated_vector",
        "frozen_corpus_complete",
        "deployment_ready",
        "Path 2 account-creation frame KAT consistency check passed.",
    ],
    "scripts/check_build.sh": [
        "account-creation frame KAT consistency check",
        "path2_account_creation_frame_kat_consistency",
    ],
    ".github/workflows/ci.yml": [
        "Account-creation frame KAT consistency check",
        "path2_account_creation_frame_kat_consistency",
    ],
}
SPEC_OPEN_WORK_SECTIONS = {
    "spec/conformance_kats.md": {
        "Required Evidence": {
            "SPEC-KAT-001",
            "DIR-KAT-001",
            "DIR-LEAF-001",
            "DIR-EVID-001",
            "DIR-SPEND-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "SPEC-VERIFY-001",
            "SPEC-DEPLOY-001",
            "POL-REQ-001",
            "ADOPT-HW-API-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-ETH-AA-001",
        },
    },
    "spec/consensus_verifier.md": {
        "Required Evidence": {
            "SPEC-VERIFY-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "POL-REQ-001",
            "ADOPT-ETH-AA-001",
            "ADOPT-ETH-7932-001",
            "ADOPT-ETH-8141-001",
            "ADOPT-HW-API-001",
        },
    },
    "spec/deployment_profiles.md": {
        "Required Evidence": {
            "SPEC-DEPLOY-001",
            "SPEC-LIFE-001",
            "SPEC-VERIFY-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "AGG-DA-001",
            "AGG-LC-001",
            "AGG-COMMITTEE-001",
            "ADOPT-ETH-AA-001",
            "ADOPT-ETH-7702-001",
            "ADOPT-ETH-7932-001",
            "ADOPT-ETH-8141-001",
            "ADOPT-ETH-ROLLUP-001",
            "ADOPT-HW-STATE-001",
            "ADOPT-HW-ENT-001",
            "SPEC-STATE-001",
            "SPEC-ETH-001",
        },
    },
    "spec/ethereum_profiles.md": {
        "Required Evidence": {
            "SPEC-ETH-001",
            "SPEC-DEPLOY-001",
            "SPEC-VERIFY-001",
            "SPEC-KAT-001",
            "SPEC-LIFE-001",
            "SPEC-POLICY-001",
            "SPEC-STATE-001",
            "ADOPT-ETH-001",
            "ADOPT-ETH-AA-001",
            "ADOPT-ETH-7702-001",
            "ADOPT-ETH-7932-001",
            "ADOPT-ETH-8141-001",
            "ADOPT-ETH-ROLLUP-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "AGG-ROLLUP-001",
            "AGG-DA-001",
            "AGG-LC-001",
            "POL-REQ-001",
            "POL-RECOVERY-001",
            "ADOPT-HW-STATE-001",
        },
    },
    "spec/entropy_profile.md": {
        "Required Evidence": {
            "ADOPT-HW-ENT-001",
            "SPEC-SIGNER-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "ADOPT-HW-API-001",
            "ADOPT-HW-STATE-001",
            "ADOPT-HW-CT-001",
            "DIR-CT-001",
            "SPEC-DEPLOY-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
        },
    },
    "spec/wire_formats.md": {
        "Path2AccountStarkCreationPayloadV1": {
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
        },
        "Rule For New Wire Formats": {
            "DIR-LEAF-001",
            "DIR-EVID-001",
            "DIR-CC-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
        },
    },
    "spec/security_model.md": {
        "Direct Theorem Acceptance Boundary": {
            "DIR-SEC-001",
            "DIR-CREATE-001",
            "DIR-HANDLE-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
        },
        "Account-Creation Proof Boundary Contract": {
            "DIR-CREATE-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-PAYLOAD-001",
            "DIR-AUTHCLASS-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
            "DIR-CT-001",
            "DIR-QROM-001",
            "DIR-REVIEW-001",
        },
        "Selected Account-STARK Residual Matrix": {
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
            "DIR-QROM-001",
            "DIR-REVIEW-001",
        },
        "Risk-First Deployment Blocker Order": {
            "DIR-RISK-001",
            "DIR-SEC-001",
            "DIR-CREATE-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-PAYLOAD-001",
            "DIR-AUTHCLASS-001",
            "DIR-CC-001",
            "DIR-LEAF-001",
            "DIR-EVID-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "DIR-REJECT-001",
            "DIR-CT-001",
            "SPEC-SIGNER-001",
            "SPEC-LEAKAGE-001",
            "ADOPT-HW-001",
            "ADOPT-HW-CT-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-HW-STATE-001",
            "DIR-LIFE-001",
            "SPEC-LIFE-001",
            "SPEC-STATE-001",
            "SPEC-DEPLOY-001",
            "DIR-REVIEW-001",
            "SPEC-REVIEW-001",
            "DIR-QROM-001",
            "DIR-ACTION-001",
        },
        "Required Evidence Before Deployment Claim": {
            "DIR-RISK-001",
            "DIR-SEC-001",
            "DIR-CREATE-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-AUTHCLASS-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "DIR-REJECT-001",
            "SPEC-SIGNER-001",
            "SPEC-LEAKAGE-001",
            "DIR-CT-001",
            "DIR-QROM-001",
            "DIR-REVIEW-001",
            "DIR-LIFE-001",
            "SPEC-DEPLOY-001",
            "SPEC-REVIEW-001",
        },
    },
    "spec/review_package.md": {
        "Required Evidence": {
            "SPEC-REVIEW-001",
            "DIR-REVIEW-001",
            "DIR-SEC-001",
            "DIR-QROM-001",
            "DIR-ACTION-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
            "DIR-LIFE-001",
            "DIR-CT-001",
            "SPEC-DEPLOY-001",
            "SPEC-SIGNER-001",
            "SPEC-LEAKAGE-001",
            "AGG-MODE-001",
            "AGG-ROLLUP-001",
            "AGG-COMMITTEE-001",
            "POL-REQ-001",
            "ADOPT-HW-ENT-001",
            "SPEC-STATE-001",
            "SPEC-ETH-001",
            "ADOPT-128-001",
        },
    },
    "spec/leakage_profile.md": {
        "Required Evidence": {
            "SPEC-LEAKAGE-001",
            "DIR-CT-001",
            "ADOPT-HW-CT-001",
            "SPEC-SIGNER-001",
            "ADOPT-HW-API-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-HW-STATE-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "SPEC-DEPLOY-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-REVIEW-001",
        },
    },
    "spec/signer_state_profile.md": {
        "Required Evidence": {
            "SPEC-STATE-001",
            "ADOPT-HW-STATE-001",
            "SPEC-LIFE-001",
            "DIR-LIFE-001",
            "SPEC-SIGNER-001",
            "ADOPT-HW-API-001",
            "SPEC-DEPLOY-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "SPEC-LEAKAGE-001",
            "POL-RECOVERY-001",
            "DIR-REVIEW-001",
        },
    },
    "spec/signer_api.md": {
        "Required Evidence": {
            "ADOPT-HW-API-001",
            "SPEC-SIGNER-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "SPEC-VERIFY-001",
            "SPEC-DEPLOY-001",
            "DIR-LIFE-001",
            "ADOPT-HW-STATE-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-HW-CT-001",
        },
    },
    "spec/lifecycle_model.md": {
        "Required Evidence": {
            "DIR-LIFE-001",
            "SPEC-DEPLOY-001",
            "SPEC-STATE-001",
            "ADOPT-HW-STATE-001",
            "AGG-ORDER-001",
            "AGG-DA-001",
        },
    },
    "spec/policy_model.md": {
        "Required Evidence": {
            "SPEC-POLICY-001",
            "POL-REQ-001",
            "POL-MSIG-001",
            "POL-THRESH-001",
            "POL-RECOVERY-001",
            "POL-ADAPTOR-001",
            "AGG-COMMITTEE-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "ADOPT-HW-STATE-001",
        },
    },
    "spec/signer_profile.md": {
        "Wallet / HSM API Shape": {
            "SPEC-SIGNER-001",
            "ADOPT-HW-API-001",
            "DIR-REJECT-001",
        },
        "Entropy And Domain Separation": {
            "SPEC-SIGNER-001",
            "ADOPT-HW-ENT-001",
            "DIR-KAT-001",
        },
        "Constant-Time And Leakage Evidence": {
            "SPEC-SIGNER-001",
            "DIR-CT-001",
            "ADOPT-HW-CT-001",
        },
        "Fault And Recovery Requirements": {
            "SPEC-SIGNER-001",
            "DIR-LIFE-001",
            "ADOPT-HW-STATE-001",
        },
    },
}
POLICY_REQUIREMENT_REFS = [
    ("policy domain separator and version", {"POL-REQ-001"}),
    ("account namespace", {"POL-REQ-001", "DIR-CC-001"}),
    (
        "signer/guardian/committee set representation",
        {"POL-MSIG-001", "POL-THRESH-001", "AGG-COMMITTEE-001"},
    ),
    ("threshold or weight rule", {"POL-THRESH-001", "AGG-COMMITTEE-001"}),
    (
        "timelock, recovery, or rotation semantics",
        {"POL-RECOVERY-001", "ADOPT-HW-STATE-001"},
    ),
    ("replay and duplicate-prevention rule", {"DIR-LIFE-001", "AGG-ORDER-001"}),
    ("lifecycle interaction with consumed slots", {"DIR-LIFE-001", "POL-REQ-001"}),
    (
        "state root fields bound into the spend or aggregate proof",
        {"DIR-CC-001", "POL-REQ-001", "AGG-LC-001"},
    ),
]
DIRECT_MODE_GATE_CONTRACTS = {
    1: (
        "final direct-mode theorem statement",
        {"DIR-RISK-001", "DIR-SEC-001", "DIR-QROM-001", "DIR-ACTION-001"},
    ),
    2: (
        "public wire formats are fixed and versioned",
        {"DIR-LEAF-001", "DIR-EVID-001", "DIR-CC-001"},
    ),
    3: (
        "every public field has positive and negative KAT coverage",
        {"DIR-KAT-001", "DIR-CC-001"},
    ),
    4: (
        "direct creation and direct spend have end-to-end vectors",
        {"DIR-KAT-001"},
    ),
    5: (
        "signer/keygen/sampler behavior satisfies",
        {"SPEC-SIGNER-001", "DIR-CT-001", "ADOPT-HW-001"},
    ),
    6: (
        "verifier rejection is deterministic, panic-free, and consensus-safe",
        {"DIR-REJECT-001", "DIR-CC-001"},
    ),
    7: (
        "lifecycle/reorg/finality policy is specified",
        {"SPEC-LIFE-001", "DIR-LIFE-001"},
    ),
    8: (
        "security level is stated honestly",
        {"DIR-QROM-001", "ADOPT-128-001"},
    ),
    9: (
        "third-party cryptanalysis has reviewed",
        {"DIR-REVIEW-001"},
    ),
}
MODE_B_GATE_CONTRACTS = {
    1: (
        "batch or recursive proof system is named and parameterized",
        {"AGG-BATCH-001", "AGG-ROLLUP-001"},
    ),
    2: (
        "aggregate statement format is fixed",
        {"AGG-SPEND-001", "AGG-LC-001"},
    ),
    3: (
        "canonical batch ordering and duplicate rejection",
        {"AGG-ORDER-001"},
    ),
    4: (
        "data availability rules exist",
        {"AGG-DA-001"},
    ),
    5: (
        "aggregate consume-once semantics",
        {"AGG-SPEND-001", "DIR-LIFE-001"},
    ),
    6: (
        "light-client proof semantics are specified",
        {"AGG-LC-001"},
    ),
    7: (
        "proof-system soundness, Fiat-Shamir/QROM assumptions",
        {"AGG-ROLLUP-001", "DIR-QROM-001", "DIR-SEC-001"},
    ),
}
CAPABILITY_MATRIX_CONTRACTS = [
    (
        "Simple transaction authorization",
        "`direct`",
        "checked-core spend",
        "required first deployable path",
        {"DIR-SPEND-001", "DIR-CC-001", "DIR-SEC-001"},
    ),
    (
        "One-use authorization lifecycle",
        "`direct`",
        "live slot registry",
        "D2 lifecycle profile verification",
        {"SPEC-LIFE-001", "DIR-LIFE-001"},
    ),
    (
        "Canonical public wire objects",
        "`direct`",
        "PrevalidatedSlotLeafV1",
        "KAT corpus still required",
        {"DIR-LEAF-001", "DIR-EVID-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Deterministic consensus verification",
        "`direct`",
        "canonical parse",
        "required before deployment claim",
        {"DIR-REJECT-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Hardware-wallet/HSM safety",
        "`direct`",
        "no floating point",
        "required before deployment claim",
        {"SPEC-SIGNER-001", "DIR-CT-001", "ADOPT-HW-001"},
    ),
    (
        "M-of-N multisig",
        "`policy`",
        "account policy over multiple admitted slots",
        "not native signature aggregation",
        {"POL-MSIG-001", "POL-REQ-001"},
    ),
    (
        "Threshold custody",
        "`policy` or `mode_b`",
        "true threshold construction or proof-compressed M-of-N",
        "open design decision",
        {"POL-THRESH-001", "AGG-COMMITTEE-001"},
    ),
    (
        "Validator quorum authorization",
        "`policy` and `mode_b`",
        "weighted signer set",
        "not a BLS replacement claim",
        {"POL-REQ-001", "AGG-COMMITTEE-001"},
    ),
    (
        "Bridge committee authorization",
        "`policy` and `mode_b`",
        "committee state root",
        "requires Mode B proof semantics",
        {"AGG-COMMITTEE-001", "AGG-DA-001", "AGG-LC-001"},
    ),
    (
        "Account recovery",
        "`policy`",
        "guardian/timelock/rotation rules",
        "must be explicit state policy",
        {"POL-RECOVERY-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Adaptor-style workflows",
        "`policy`",
        "state/protocol conditions",
        "not native Schnorr-style adaptor algebra",
        {"POL-ADAPTOR-001", "POL-REQ-001"},
    ),
    (
        "Batch verification",
        "`mode_b`",
        "batch account creation or batch spend proof",
        "optional after direct theorem closes",
        {"AGG-BATCH-001", "AGG-SPEND-001", "AGG-ORDER-001"},
    ),
    (
        "Signature/proof aggregation",
        "`mode_b`",
        "proof/state aggregation",
        "not native signature combine",
        {"AGG-MODE-001", "AGG-ROLLUP-001", "AGG-SPEND-001"},
    ),
    (
        "Rollup compression",
        "`mode_b`",
        "recursive proof",
        "concrete recursion proof still under construction",
        {"AGG-ROLLUP-001", "ADOPT-ETH-ROLLUP-001"},
    ),
    (
        "Light-client proofs",
        "`mode_b`",
        "compact state/proof handle",
        "needs final aggregate proof format",
        {"AGG-LC-001", "AGG-DA-001"},
    ),
]
SPEC_BASELINE_SURFACE_CONTRACTS = [
    (
        "Normative spec spine",
        "`direct`",
        "`partial`",
        "indexed current spec spine",
        {"SPEC-DIR-001", "DIR-RISK-001", "DIR-SEC-001"},
    ),
    (
        "Blockchain requirements spine",
        "`direct`",
        "`partial`",
        "capability matrix",
        {"SPEC-REQ-001", "DIR-KAT-001", "DIR-REVIEW-001"},
    ),
    (
        "Conformance and KAT corpus model",
        "`direct`",
        "`partial`",
        "active Path 2 corpus identity",
        {"SPEC-KAT-001", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Consensus verifier model",
        "`direct`",
        "`partial`",
        "public verifier pipeline",
        {"SPEC-VERIFY-001", "DIR-REJECT-001", "DIR-KAT-001"},
    ),
    (
        "Deployment profile model",
        "`direct`",
        "`partial`",
        "target-chain, rollup, bridge, Ethereum",
        {"SPEC-DEPLOY-001", "DIR-LIFE-001", "DIR-REJECT-001"},
    ),
    (
        "Ethereum adoption profile model",
        "`direct`",
        "`partial`",
        "Ethereum account-abstraction",
        {"SPEC-ETH-001", "ADOPT-ETH-001", "SPEC-DEPLOY-001"},
    ),
    (
        "Entropy and stream profile model",
        "`direct`",
        "`partial`",
        "entropy-source classes",
        {"ADOPT-HW-ENT-001", "SPEC-SIGNER-001", "DIR-KAT-001"},
    ),
    (
        "Constant-time and leakage evidence model",
        "`direct`",
        "`partial`",
        "secret/public classification",
        {"SPEC-LEAKAGE-001", "DIR-CT-001", "ADOPT-HW-CT-001"},
    ),
    (
        "Signer public API model",
        "`direct`",
        "`partial`",
        "public command/result classes",
        {"ADOPT-HW-API-001", "SPEC-SIGNER-001", "DIR-REJECT-001"},
    ),
    (
        "Signer state and fault model",
        "`direct`",
        "`partial`",
        "local slot records",
        {"SPEC-STATE-001", "ADOPT-HW-STATE-001", "DIR-LIFE-001"},
    ),
    (
        "Signer and custody profile",
        "`direct`",
        "`partial`",
        "signer, keygen, sampler, wallet, and HSM requirements",
        {"SPEC-SIGNER-001", "DIR-CT-001", "ADOPT-HW-001"},
    ),
    (
        "Lifecycle and state model",
        "`direct`",
        "`partial`",
        "consume-once lifecycle",
        {"SPEC-LIFE-001", "DIR-LIFE-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Policy model",
        "`policy`",
        "`partial`",
        "policy object boundary",
        {"SPEC-POLICY-001", "POL-REQ-001", "DIR-LIFE-001"},
    ),
    (
        "External review package model",
        "`direct`",
        "`partial`",
        "review lanes, manifest",
        {"SPEC-REVIEW-001", "DIR-REVIEW-001", "DIR-SEC-001"},
    ),
]
DIRECT_IMPLEMENTATION_SURFACE_CONTRACTS = [
    (
        "Normative direct-mode spine",
        "`direct`",
        "`partial`",
        "final direct-mode theorem",
        {"SPEC-DIR-001", "DIR-RISK-001", "DIR-SEC-001"},
    ),
    (
        "Risk-first deployment blocker ledger",
        "`direct`",
        "`partial`",
        "ranked blocker order",
        {"DIR-RISK-001", "DIR-SEC-001", "DIR-PROOF-002"},
    ),
    (
        "Slot leaf and creation evidence frames",
        "`direct`",
        "`partial`",
        "canonical frame bytes",
        {"DIR-LEAF-001", "DIR-EVID-001", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Account-creation STARK API",
        "`direct`",
        "`partial`",
        "account-creation prover/verifier API",
        {"DIR-STARK-001", "DIR-BIND-001", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Spend authorization relation",
        "`direct`",
        "`partial`",
        "spend relation connected to Rust evidence",
        {"DIR-SPEND-001", "DIR-CC-001", "DIR-LIFE-001", "DIR-KAT-001"},
    ),
    (
        "Slot authorization handle",
        "`direct`",
        "`partial`",
        "normative wire/state text plus executable direct-record digest",
        {"DIR-HANDLE-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Account-creation theorem boundary",
        "`direct`",
        "`partial`",
        "direct creation theorem boundary",
        {
            "DIR-CREATE-001",
            "DIR-SEC-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-CT-001",
        },
    ),
    (
        "Account-creation negative controls",
        "`direct`",
        "`not_ready`",
        "executable negative KATs",
        {"DIR-CREATE-NEG-001", "DIR-REJECT-001", "DIR-KAT-001"},
    ),
    (
        "Checked-core frame and proof support",
        "`direct`",
        "`partial`",
        "checked-core frame binding",
        {"DIR-CC-001", "DIR-CC-002", "DIR-QROM-002", "DIR-KAT-001"},
    ),
    (
        "Consume-once lifecycle transition",
        "`direct`",
        "`partial`",
        "digest-pinned target-chain lifecycle profile verification",
        {"DIR-LIFE-001", "SPEC-LIFE-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Account-creation proof family",
        "`direct`",
        "`not_ready`",
        "selected account-STARK proof family",
        {
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-REJECT-001",
        },
    ),
    (
        "Creation payload and fiber privacy",
        "`direct`",
        "`not_ready`",
        "concrete payload theorem",
        {"DIR-PAYLOAD-001", "DIR-PROOF-001", "DIR-CT-001"},
    ),
    (
        "Masked-trace relation stack",
        "`direct`",
        "`not_ready`",
        "trace relation",
        {"DIR-TRACE-001", "DIR-PROOF-001", "DIR-KAT-001"},
    ),
    (
        "Descent/SIS proof-route support",
        "`direct`",
        "`not_ready`",
        "descent/SIS support remains research",
        {"DIR-DESCENT-001", "DIR-PROOF-002"},
    ),
    (
        "Native-boundary model",
        "`direct`",
        "`not_ready`",
        "normative object/projection model",
        {"DIR-NB-001", "DIR-CC-002", "DIR-KAT-001"},
    ),
    (
        "Native-boundary knowledge stack",
        "`direct`",
        "`not_ready`",
        "parent-trace extraction",
        {"DIR-NB-002", "DIR-PROOF-001", "DIR-PROOF-002"},
    ),
    (
        "Native-boundary row and range model",
        "`direct`",
        "`not_ready`",
        "row-cost",
        {"DIR-NB-003", "DIR-CT-001"},
    ),
    (
        "Native-boundary implementation cross-check",
        "`direct`",
        "`not_ready`",
        "Rust/Lean assembly checks",
        {"DIR-NB-004", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Authorization-class binding",
        "`direct`",
        "`not_ready`",
        "private authorization-local material cannot change public spend semantics",
        {"DIR-AUTHCLASS-001", "DIR-SEC-001"},
    ),
    (
        "Hash/QROM and action-map budget",
        "`direct`",
        "`partial`",
        "final theorem-linked QROM ledger",
        {"DIR-QROM-001", "DIR-QROM-002", "DIR-ACTION-001", "DIR-REVIEW-001"},
    ),
    (
        "Conformance and KAT corpus",
        "`direct`",
        "`partial`",
        "active Path 2 corpus manifest",
        {"SPEC-KAT-001", "DIR-KAT-001", "DIR-REJECT-001", "DIR-CC-001"},
    ),
    (
        "Active test gates",
        "`direct`",
        "`partial`",
        "fast active Rust gate",
        {"DIR-TEST-001", "DIR-SLOWTEST-001"},
    ),
    (
        "Consensus-safe rejection",
        "`direct`",
        "`partial`",
        "deterministic public errors",
        {"DIR-REJECT-001", "DIR-KAT-001", "SPEC-VERIFY-001"},
    ),
    (
        "Constant-time signer evidence",
        "`direct`",
        "`not_ready`",
        "leakage-harness",
        {"DIR-CT-001", "SPEC-SIGNER-001", "ADOPT-HW-CT-001", "SPEC-LEAKAGE-001"},
    ),
    (
        "External review package",
        "`direct`",
        "`partial`",
        "external cryptanalysis package",
        {"DIR-REVIEW-001", "DIR-SEC-001", "SPEC-REVIEW-001"},
    ),
]
AGGREGATION_SURFACE_CONTRACTS = [
    (
        "Mode B theorem boundary",
        "`mode_b`",
        "`partial`",
        "direct mode does not depend on aggregation",
        {"AGG-MODE-001", "DIR-SEC-001"},
    ),
    (
        "Batch account creation API",
        "`mode_b`",
        "`partial`",
        "targeted tests and aggregate statement binding",
        {"AGG-BATCH-001", "AGG-ORDER-001"},
    ),
    (
        "Batch spend verification",
        "`mode_b`",
        "`not_ready`",
        "duplicate-slot rejection",
        {"AGG-SPEND-001", "DIR-LIFE-001", "AGG-ORDER-001"},
    ),
    (
        "Canonical ordering and duplicates",
        "`mode_b`",
        "`not_ready`",
        "ordering keys",
        {"AGG-ORDER-001", "DIR-REJECT-001"},
    ),
    (
        "Data availability rules",
        "`mode_b`",
        "`not_ready`",
        "consensus state, calldata, DA blobs, or proof public inputs",
        {"AGG-DA-001", "AGG-LC-001"},
    ),
    (
        "Recursive account-proof verifier",
        "`mode_b`",
        "`partial`",
        "concrete recursion proof",
        {"AGG-ROLLUP-001", "DIR-QROM-001"},
    ),
    (
        "Recursion challenger gadget",
        "`mode_b`",
        "`partial`",
        "challenger gadget KATs",
        {"AGG-ROLLUP-002", "DIR-KAT-001"},
    ),
    (
        "Light-client aggregate proof",
        "`mode_b`",
        "`not_ready`",
        "compact proof/state handle",
        {"AGG-LC-001", "AGG-DA-001"},
    ),
    (
        "Bridge or validator committee proof",
        "`mode_b`",
        "`not_ready`",
        "signer set, weights, bitmap/root, quorum rule",
        {"AGG-COMMITTEE-001", "POL-THRESH-001", "POL-REQ-001"},
    ),
]
SECURITY_LEVEL_SURFACE_CONTRACTS = [
    (
        "Hard-128 profile decision",
        "`direct`",
        "`not_ready`",
        "q128 practical floor",
        {"ADOPT-128-001", "DIR-QROM-001", "DIR-REVIEW-001"},
    ),
]
ETHEREUM_SURFACE_CONTRACTS = [
    (
        "Positioning baseline",
        "`policy`",
        "`partial`",
        "normative Ethereum profile split",
        {"ADOPT-ETH-001", "SPEC-ETH-001"},
    ),
    (
        "ERC-4337 smart-account verifier",
        "`direct`",
        "`partial`",
        "profile ABI/envelope exists",
        {"ADOPT-ETH-AA-001", "SPEC-ETH-001", "DIR-REJECT-001", "DIR-KAT-001"},
    ),
    (
        "EIP-7702 delegated EOA migration",
        "`policy`",
        "`partial`",
        "migration profile exists",
        {"ADOPT-ETH-7702-001", "SPEC-ETH-001", "POL-RECOVERY-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Secondary-signature/precompile profile",
        "`direct`",
        "`partial`",
        "registry/precompile profile exists",
        {"ADOPT-ETH-7932-001", "SPEC-ETH-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Native frame-transaction validation",
        "`direct`",
        "`partial`",
        "frame-validation profile exists",
        {"ADOPT-ETH-8141-001", "SPEC-ETH-001", "DIR-REJECT-001", "DIR-LIFE-001"},
    ),
    (
        "Rollup aggregate proof handle",
        "`mode_b`",
        "`partial`",
        "rollup handle profile exists",
        {"ADOPT-ETH-ROLLUP-001", "SPEC-ETH-001", "AGG-ROLLUP-001", "AGG-DA-001", "AGG-LC-001"},
    ),
]
HARDWARE_SURFACE_CONTRACTS = [
    (
        "Signer profile baseline",
        "`direct`",
        "`partial`",
        "normative signer/custody profile",
        {"ADOPT-HW-001", "SPEC-SIGNER-001"},
    ),
    (
        "Wallet/HSM API and public errors",
        "`direct`",
        "`partial`",
        "public command/result model exists",
        {"ADOPT-HW-API-001", "DIR-REJECT-001", "SPEC-SIGNER-001"},
    ),
    (
        "Entropy and domain separation",
        "`direct`",
        "`partial`",
        "stream model and mask-PRF drift evidence exist",
        {"ADOPT-HW-ENT-001", "SPEC-SIGNER-001", "DIR-KAT-001"},
    ),
    (
        "Secret-side leakage harness",
        "`direct`",
        "`not_ready`",
        "fixed-shape leakage tests",
        {"ADOPT-HW-CT-001", "DIR-CT-001", "SPEC-SIGNER-001", "SPEC-LEAKAGE-001"},
    ),
    (
        "Rollback and fault state model",
        "`direct`",
        "`partial`",
        "signer state/fault model exists",
        {"ADOPT-HW-STATE-001", "DIR-LIFE-001", "SPEC-LIFE-001", "SPEC-STATE-001"},
    ),
]
POLICY_SURFACE_CONTRACTS = [
    (
        "Policy object baseline",
        "`policy`",
        "`partial`",
        "domain separator, version, and account namespace",
        {"POL-REQ-001", "DIR-CC-001"},
    ),
    (
        "M-of-N account policy",
        "`policy`",
        "`not_ready`",
        "signer set representation",
        {"POL-MSIG-001", "POL-REQ-001", "DIR-LIFE-001"},
    ),
    (
        "Threshold custody route",
        "`policy`",
        "`not_ready`",
        "true threshold signing and proof-compressed M-of-N",
        {"POL-THRESH-001", "AGG-COMMITTEE-001"},
    ),
    (
        "Account recovery policy",
        "`policy`",
        "`not_ready`",
        "guardian, timelock, rotation",
        {"POL-RECOVERY-001", "ADOPT-HW-STATE-001", "DIR-LIFE-001"},
    ),
    (
        "Adaptor-style workflow policy",
        "`policy`",
        "`not_ready`",
        "without claiming native Schnorr-style adaptor algebra",
        {"POL-ADAPTOR-001", "POL-REQ-001", "DIR-LIFE-001"},
    ),
]


def split_table_row(line: str) -> list[str]:
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def is_separator_row(cells: list[str]) -> bool:
    return bool(cells) and all(set(cell) <= {"-", ":", " "} for cell in cells)


def tracker_statuses() -> dict[str, str]:
    statuses: dict[str, str] = {}
    for line in TRACKER.read_text(encoding="utf-8").splitlines():
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if cells and TRACKER_ITEM_ID.match(cells[0]):
            statuses[cells[0]] = cells[1] if len(cells) > 1 else ""
    return statuses


def section(text: str, heading: str, errors: list[str]) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.MULTILINE)
    match = pattern.search(text)
    if match is None:
        errors.append(f"spec/blockchain_requirements.md: missing section ## {heading}")
        return ""
    next_heading = re.search(r"^## ", text[match.end() :], re.MULTILINE)
    if next_heading is None:
        return text[match.end() :]
    return text[match.end() : match.end() + next_heading.start()]


def section_from_source(source: str, text: str, heading: str, errors: list[str]) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.MULTILINE)
    match = pattern.search(text)
    if match is None:
        errors.append(f"{source}: missing section ## {heading}")
        return ""
    next_heading = re.search(r"^## ", text[match.end() :], re.MULTILINE)
    if next_heading is None:
        return text[match.end() :]
    return text[match.end() : match.end() + next_heading.start()]


def table_rows(block: str) -> list[list[str]]:
    rows: list[list[str]] = []
    for line in block.splitlines():
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if is_separator_row(cells):
            continue
        rows.append(cells)
    return rows


def bullet_items(block: str) -> list[str]:
    items: list[str] = []
    current: list[str] | None = None
    for line in block.splitlines():
        if line.startswith("- "):
            if current is not None:
                items.append(" ".join(current).rstrip(" ;."))
            current = [line[2:].strip()]
        elif current is not None and line.startswith("  "):
            current.append(line.strip())
        elif current is not None and not line.strip():
            continue
        elif current is not None:
            items.append(" ".join(current).rstrip(" ;."))
            current = None
    if current is not None:
        items.append(" ".join(current).rstrip(" ;."))
    return items


def normalized_text(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


def check_refs(
    label: str,
    cell: str,
    tracker_rows: dict[str, str],
    errors: list[str],
) -> list[str]:
    refs = TRACKER_REF.findall(cell)
    if not refs:
        errors.append(f"{label}: missing backticked TRACKER.md row reference")
        return []
    for ref in refs:
        if ref not in tracker_rows:
            errors.append(f"{label}: unknown TRACKER.md row reference {ref}")
    return refs


def check_project_framing(text: str, errors: list[str]) -> None:
    for phrase in OBJECTIVE_PHRASES:
        if phrase not in text:
            errors.append(
                "Project framing: missing chain-native authorization phrase "
                f"{phrase!r}"
            )


def check_requirement_classes(text: str, errors: list[str]) -> None:
    rows = table_rows(section(text, "Requirement Classes", errors))
    if not rows:
        errors.append("Requirement Classes: missing table")
        return
    if rows[0] != ["Class", "Meaning"]:
        errors.append("Requirement Classes: header must be ['Class', 'Meaning']")
        return
    data_rows = rows[1:]
    expected_rows = [list(row) for row in REQUIREMENT_CLASS_CONTRACTS]
    if data_rows != expected_rows:
        errors.append(
            "Requirement Classes: rows must exactly match "
            f"{expected_rows!r}"
        )

    block = section(text, "Requirement Classes", errors)
    normalized_block = normalized_text(block)
    required_phrases = [
        "Direct requirements must not depend on `optional_aggregation_soundness`.",
        "Mode B requirements may depend on a named proof system, accumulator, or recursive wrapper.",
        "Policy requirements may depend on application/account state",
        "bind to the same `slot_authorization_digest`",
        "`TRACKER.md` rows that carry the corresponding work",
        "A requirement without a tracker row is not allowed to remain only prose.",
    ]
    for phrase in required_phrases:
        if phrase not in normalized_block:
            errors.append(f"Requirement Classes: missing policy phrase {phrase!r}")


def check_non_claims(text: str, errors: list[str]) -> None:
    block = section(text, "Non-Claims", errors)
    if "Descender must not imply these are solved by the base signature primitive:" not in block:
        errors.append("Non-Claims: missing base-signature non-claim framing")
    items = bullet_items(block)
    if items != NON_CLAIMS_CONTRACTS:
        errors.append(
            "Non-Claims: bullets must exactly match "
            f"{NON_CLAIMS_CONTRACTS!r}; got {items!r}"
        )


def check_product_bar(text: str, errors: list[str]) -> None:
    block = section(text, "Product Bar", errors)
    normalized_block = normalized_text(block)
    match = re.search(r"```text\n(.*?)\n```", block, re.DOTALL)
    if match is None:
        errors.append("Product Bar: missing text code block")
        return
    lines = match.group(1).splitlines()
    if lines != PRODUCT_BAR_LINES:
        errors.append(
            "Product Bar: target lines must exactly match "
            f"{PRODUCT_BAR_LINES!r}; got {lines!r}"
        )
    required_phrases = [
        "higher than \"a PQ signature verifies.\"",
        "necessary for a serious chain adopter",
        "belongs in `TRACKER.md` with a status below `verified`",
        "not in marketing language",
    ]
    for phrase in required_phrases:
        if phrase not in normalized_block:
            errors.append(f"Product Bar: missing policy phrase {phrase!r}")


def check_readiness_semantics(text: str, errors: list[str]) -> None:
    rows = table_rows(section(text, "Readiness Semantics", errors))
    if not rows:
        errors.append("Readiness Semantics: missing table")
        return

    header = rows[0]
    if header != ["Status", "Meaning"]:
        errors.append("Readiness Semantics: header must be ['Status', 'Meaning']")
        return

    seen: dict[str, str] = {}
    for row in rows[1:]:
        if len(row) != 2:
            errors.append(
                f"Readiness Semantics row {row[0]!r}: expected Status, Meaning"
            )
            continue
        status = row[0].strip("`")
        meaning = row[1]
        if status in seen:
            errors.append(f"Readiness Semantics: duplicate status {row[0]!r}")
        seen[status] = meaning
        if not meaning:
            errors.append(f"Readiness Semantics row {row[0]!r}: empty meaning")

    missing = sorted(READINESS_STATUSES - set(seen))
    extra = sorted(set(seen) - READINESS_STATUSES)
    if missing:
        errors.append(f"Readiness Semantics: missing statuses {missing}")
    if extra:
        errors.append(f"Readiness Semantics: unknown statuses {extra}")

    for status in ("not_ready", "partial"):
        meaning = seen.get(status, "").lower()
        if "not adoption evidence" not in meaning:
            errors.append(
                f"Readiness Semantics row `{status}`: must say it is not adoption evidence"
            )

    ready_meaning = seen.get("ready", "")
    if (
        "cited tracker rows" not in ready_meaning.lower()
        or "`verified`" not in ready_meaning
    ):
        errors.append(
            "Readiness Semantics row `ready`: must require cited tracker rows to be `verified`"
        )


def check_capability_matrix(
    text: str,
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    rows = table_rows(section(text, "Capability Matrix", errors))
    if not rows:
        errors.append("Capability Matrix: missing table")
        return

    header = rows[0]
    expected_header = [
        "Capability",
        "Required class",
        "Descender surface",
        "Current stance",
        "Tracker rows",
    ]
    if header != expected_header:
        errors.append(f"Capability Matrix: header must be {expected_header!r}")
        return

    data_rows = rows[1:]
    if not data_rows:
        errors.append("Capability Matrix: missing data rows")
        return
    if len(data_rows) != len(CAPABILITY_MATRIX_CONTRACTS):
        errors.append(
            "Capability Matrix: "
            f"{len(data_rows)} rows for {len(CAPABILITY_MATRIX_CONTRACTS)} expected capabilities"
        )

    seen_capabilities: list[str] = []
    for index, row in enumerate(data_rows):
        if len(row) != len(header):
            errors.append(
                f"Capability Matrix row {row[0]!r}: "
                f"expected {len(header)} cells, got {len(row)}"
            )
            continue
        capability, required_class, surface, stance, tracker_cell = row
        seen_capabilities.append(capability)
        if index >= len(CAPABILITY_MATRIX_CONTRACTS):
            errors.append(f"Capability Matrix row {capability!r}: unexpected extra row")
            check_refs(f"Capability Matrix row {capability!r}", tracker_cell, tracker_rows, errors)
            continue
        (
            expected_capability,
            expected_class,
            expected_surface_phrase,
            expected_stance_phrase,
            expected_refs,
        ) = CAPABILITY_MATRIX_CONTRACTS[index]
        if capability != expected_capability:
            errors.append(
                "Capability Matrix row order/content mismatch: "
                f"expected {expected_capability!r}, got {capability!r}"
            )
        if required_class != expected_class:
            errors.append(
                f"Capability Matrix row {capability!r}: required class "
                f"{required_class!r} != expected {expected_class!r}"
            )
        if expected_surface_phrase not in surface:
            errors.append(
                f"Capability Matrix row {capability!r}: expected surface phrase "
                f"{expected_surface_phrase!r} not found in {surface!r}"
            )
        if expected_stance_phrase not in stance:
            errors.append(
                f"Capability Matrix row {capability!r}: expected stance phrase "
                f"{expected_stance_phrase!r} not found in {stance!r}"
            )
        refs = set(
            check_refs(
                f"Capability Matrix row {capability!r}",
                tracker_cell,
                tracker_rows,
                errors,
            )
        )
        if refs != expected_refs:
            errors.append(
                f"Capability Matrix row {capability!r}: tracker refs {sorted(refs)} "
                f"!= expected {sorted(expected_refs)}"
            )

    expected_capabilities = [
        capability for capability, *_rest in CAPABILITY_MATRIX_CONTRACTS
    ]
    if seen_capabilities != expected_capabilities:
        missing = [
            capability
            for capability in expected_capabilities
            if capability not in seen_capabilities
        ]
        extra = [
            capability
            for capability in seen_capabilities
            if capability not in expected_capabilities
        ]
        if missing:
            errors.append("Capability Matrix: missing capabilities " + str(missing))
        if extra:
            errors.append("Capability Matrix: unexpected capabilities " + str(extra))


def numbered_gate_texts(block: str) -> dict[int, str]:
    gates: dict[int, list[str]] = {}
    current_gate: int | None = None
    for line in block.splitlines():
        match = re.match(r"^(\d+)\.\s+(.*)$", line)
        if match:
            current_gate = int(match.group(1))
            gates[current_gate] = [match.group(2).strip()]
            continue
        if current_gate is None:
            continue
        if line.startswith("|") or line.startswith("## "):
            current_gate = None
            continue
        if line.startswith("   "):
            gates[current_gate].append(line.strip())
    return {gate: " ".join(parts) for gate, parts in gates.items()}


def check_gate_traceability(
    text: str,
    section_name: str,
    table_label: str,
    tracker_rows: dict[str, str],
    errors: list[str],
    gate_contracts: dict[int, tuple[str, set[str]]] | None = None,
) -> None:
    block = section(text, section_name, errors)
    gate_texts = numbered_gate_texts(block)
    gates = set(gate_texts)
    if not gates:
        errors.append(f"{section_name}: no numbered adoption gates found")
        return
    if gate_contracts is not None:
        missing_contract_gates = sorted(set(gate_contracts) - gates)
        extra_contract_gates = sorted(gates - set(gate_contracts))
        if missing_contract_gates:
            errors.append(
                f"{section_name}: missing expected adoption gates {missing_contract_gates}"
            )
        if extra_contract_gates:
            errors.append(
                f"{section_name}: unexpected adoption gates {extra_contract_gates}"
            )
        for gate, (expected_phrase, _expected_refs) in sorted(gate_contracts.items()):
            text_for_gate = gate_texts.get(gate, "")
            if expected_phrase not in text_for_gate:
                errors.append(
                    f"{section_name} gate {gate}: expected phrase "
                    f"{expected_phrase!r} not found in {text_for_gate!r}"
                )

    rows = table_rows(block)
    headers = [row for row in rows if row and row[0] == "Gate"]
    if not headers:
        errors.append(f"{section_name}: missing {table_label} table header")
        return
    if headers[0] != ["Gate", "Readiness", "Tracker rows"]:
        errors.append(
            f"{table_label}: header must be ['Gate', 'Readiness', 'Tracker rows']"
        )
        return
    trace_rows = [row for row in rows if row and row[0].isdigit()]
    if not trace_rows:
        errors.append(f"{section_name}: missing {table_label} table rows")
        return

    seen: set[int] = set()
    for row in trace_rows:
        if len(row) != 3:
            errors.append(f"{table_label} gate {row[0]}: expected Gate, Readiness, Tracker rows")
            continue
        gate = int(row[0])
        if gate in seen:
            errors.append(f"{table_label}: duplicate gate {gate}")
        seen.add(gate)
        readiness = row[1].strip("`")
        if readiness not in READINESS_STATUSES:
            errors.append(
                f"{table_label} gate {gate}: unknown readiness {row[1]!r}; "
                f"expected one of {sorted(READINESS_STATUSES)}"
            )
        refs = check_refs(f"{table_label} gate {gate}", row[2], tracker_rows, errors)
        if gate_contracts is not None and gate in gate_contracts:
            expected_refs = gate_contracts[gate][1]
            if set(refs) != expected_refs:
                errors.append(
                    f"{table_label} gate {gate}: tracker refs {sorted(set(refs))} "
                    f"!= expected {sorted(expected_refs)}"
                )
        if readiness == "ready":
            not_verified = [ref for ref in refs if tracker_rows.get(ref) != "verified"]
            if not_verified:
                errors.append(
                    f"{table_label} gate {gate}: readiness is ready but tracker rows "
                    f"are not all verified: {not_verified}"
                )

    missing = sorted(gates - seen)
    extra = sorted(seen - gates)
    if missing:
        errors.append(f"{table_label}: missing traceability rows for gates {missing}")
    if extra:
        errors.append(f"{table_label}: traceability rows for non-existent gates {extra}")


def check_policy_traceability(
    text: str,
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    block = section(text, "Policy-Layer Requirements", errors)
    bullets = re.findall(r"^- ", block, re.MULTILINE)
    rows = table_rows(block)
    if not rows:
        errors.append("Policy requirement traceability: missing table")
        return
    if rows[0] != ["Requirement", "Tracker rows"]:
        errors.append(
            "Policy requirement traceability: header must be ['Requirement', 'Tracker rows']"
        )
        return
    trace_rows = [row for row in rows if row and row[0] != "Requirement"]
    if not trace_rows:
        errors.append("Policy requirement traceability: missing table rows")
        return
    if len(trace_rows) != len(bullets):
        errors.append(
            "Policy requirement traceability: "
            f"{len(trace_rows)} traceability rows for {len(bullets)} policy bullets"
        )
    expected_requirements = [requirement for requirement, _refs in POLICY_REQUIREMENT_REFS]
    seen_requirements: list[str] = []
    bullet_text = "\n".join(
        match.group(1).rstrip(" ;.")
        for match in re.finditer(r"^- (.+)$", block, re.MULTILINE)
    )

    for index, row in enumerate(trace_rows):
        if len(row) != 2:
            label = row[0] if row else f"#{index + 1}"
            errors.append(
                f"Policy requirement traceability row {label!r}: "
                "expected Requirement, Tracker rows"
            )
            continue
        requirement, tracker_cell = row
        seen_requirements.append(requirement)
        if index >= len(POLICY_REQUIREMENT_REFS):
            errors.append(
                f"Policy requirement traceability row {requirement!r}: unexpected extra row"
            )
            check_refs(
                f"Policy requirement traceability row {requirement!r}",
                tracker_cell,
                tracker_rows,
                errors,
            )
            continue
        expected_requirement, expected_refs = POLICY_REQUIREMENT_REFS[index]
        if requirement != expected_requirement:
            errors.append(
                "Policy requirement traceability row order/content mismatch: "
                f"expected {expected_requirement!r}, got {requirement!r}"
            )
        if expected_requirement not in bullet_text:
            errors.append(
                f"Policy-Layer Requirements bullets: missing {expected_requirement!r}"
            )
        refs = set(
            check_refs(
                f"Policy requirement traceability row {requirement!r}",
                tracker_cell,
                tracker_rows,
                errors,
            )
        )
        if refs != expected_refs:
            errors.append(
                f"Policy requirement traceability row {requirement!r}: "
                f"tracker refs {sorted(refs)} != expected {sorted(expected_refs)}"
            )

    if seen_requirements != expected_requirements:
        missing = [req for req in expected_requirements if req not in seen_requirements]
        extra = [req for req in seen_requirements if req not in expected_requirements]
        if missing:
            errors.append("Policy requirement traceability: missing requirements " + str(missing))
        if extra:
            errors.append("Policy requirement traceability: unexpected requirements " + str(extra))

    for _requirement, expected_refs in POLICY_REQUIREMENT_REFS:
        for ref in expected_refs:
            if ref not in tracker_rows:
                errors.append(
                    f"Policy requirement traceability: expected tracker row {ref} is missing"
                )


def check_surface_table(
    text: str,
    tracker_rows: dict[str, str],
    errors: list[str],
    section_name: str,
    tracker_prefix: str,
    surface_contracts: list[tuple[str, str, str, str, set[str]]] | None = None,
) -> None:
    required_refs = {ref for ref in tracker_rows if ref.startswith(tracker_prefix)}
    if not required_refs:
        errors.append(f"TRACKER.md: missing {tracker_prefix}* rows")
        return

    rows = table_rows(section(text, section_name, errors))
    if not rows:
        errors.append(f"{section_name}: missing table")
        return

    header = rows[0]
    expected_header = [
        "Surface",
        "Required class",
        "Readiness",
        "Required artifact",
        "Tracker rows",
    ]
    if header != expected_header:
        errors.append(f"{section_name}: header must be {expected_header!r}")
        return

    data_rows = rows[1:]
    if surface_contracts is not None and len(data_rows) != len(surface_contracts):
        errors.append(
            f"{section_name}: {len(data_rows)} rows for "
            f"{len(surface_contracts)} expected surfaces"
        )

    seen_surfaces: set[str] = set()
    seen_refs: set[str] = set()
    for index, row in enumerate(data_rows):
        if len(row) != len(header):
            errors.append(
                f"{section_name} row {row[0]!r}: "
                f"expected {len(header)} cells, got {len(row)}"
            )
            continue
        surface, required_class, readiness_cell, artifact, tracker_cell = row
        if surface in seen_surfaces:
            errors.append(f"{section_name}: duplicate surface {surface!r}")
        seen_surfaces.add(surface)
        if required_class.strip("`") not in {"direct", "mode_b", "policy"}:
            errors.append(
                f"{section_name} row {surface!r}: unknown required class {required_class!r}"
            )
        readiness = readiness_cell.strip("`")
        if readiness not in READINESS_STATUSES:
            errors.append(
                f"{section_name} row {surface!r}: unknown readiness "
                f"{readiness_cell!r}; expected one of {sorted(READINESS_STATUSES)}"
            )
        if not artifact:
            errors.append(f"{section_name} row {surface!r}: empty required artifact")
        refs = check_refs(
            f"{section_name} row {surface!r}",
            tracker_cell,
            tracker_rows,
            errors,
        )
        if surface_contracts is not None:
            if index >= len(surface_contracts):
                errors.append(f"{section_name} row {surface!r}: unexpected extra row")
            else:
                (
                    expected_surface,
                    expected_class,
                    expected_readiness,
                    expected_artifact_phrase,
                    expected_refs,
                ) = surface_contracts[index]
                if surface != expected_surface:
                    errors.append(
                        f"{section_name} row order/content mismatch: "
                        f"expected {expected_surface!r}, got {surface!r}"
                    )
                if required_class != expected_class:
                    errors.append(
                        f"{section_name} row {surface!r}: required class "
                        f"{required_class!r} != expected {expected_class!r}"
                    )
                if readiness_cell != expected_readiness:
                    errors.append(
                        f"{section_name} row {surface!r}: readiness "
                        f"{readiness_cell!r} != expected {expected_readiness!r}"
                    )
                if expected_artifact_phrase not in artifact:
                    errors.append(
                        f"{section_name} row {surface!r}: expected artifact phrase "
                        f"{expected_artifact_phrase!r} not found in {artifact!r}"
                    )
                if set(refs) != expected_refs:
                    errors.append(
                        f"{section_name} row {surface!r}: tracker refs "
                        f"{sorted(set(refs))} != expected {sorted(expected_refs)}"
                    )
        adoption_refs = [ref for ref in refs if ref.startswith(tracker_prefix)]
        if not adoption_refs:
            errors.append(
                f"{section_name} row {surface!r}: missing {tracker_prefix} tracker row"
            )
        seen_refs.update(ref for ref in adoption_refs if ref in required_refs)
        if readiness == "ready":
            not_verified = [ref for ref in refs if tracker_rows.get(ref) != "verified"]
            if not_verified:
                errors.append(
                    f"{section_name} row {surface!r}: readiness is ready "
                    f"but tracker rows are not all verified: {not_verified}"
                )

    missing = sorted(required_refs - seen_refs)
    if missing:
        errors.append(
            f"{section_name}: missing required tracker refs "
            + ", ".join(missing)
        )
    if surface_contracts is not None:
        expected_surfaces = [surface for surface, *_rest in surface_contracts]
        seen_surface_list = [row[0] for row in data_rows if row]
        if seen_surface_list != expected_surfaces:
            missing_surfaces = [
                surface for surface in expected_surfaces if surface not in seen_surface_list
            ]
            extra_surfaces = [
                surface for surface in seen_surface_list if surface not in expected_surfaces
            ]
            if missing_surfaces:
                errors.append(f"{section_name}: missing surfaces {missing_surfaces}")
            if extra_surfaces:
                errors.append(f"{section_name}: unexpected surfaces {extra_surfaces}")


def check_spec_open_work_sections(
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    for rel, sections in SPEC_OPEN_WORK_SECTIONS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing normative spec file")
            continue
        text = path.read_text(encoding="utf-8")
        for heading, required_refs in sections.items():
            block = section_from_source(rel, text, heading, errors)
            if not block:
                continue
            if "Tracker ownership:" not in block:
                errors.append(f"{rel} ## {heading}: missing Tracker ownership line")
            refs = set(TRACKER_REF.findall(block))
            if not refs:
                errors.append(f"{rel} ## {heading}: missing tracker references")
            for ref in refs:
                if ref not in tracker_rows:
                    errors.append(f"{rel} ## {heading}: unknown tracker reference {ref}")
            missing = sorted(required_refs - refs)
            if missing:
                errors.append(
                    f"{rel} ## {heading}: missing required tracker ownership refs {missing}"
                )


def check_selected_account_stark_residual_matrix(errors: list[str]) -> None:
    rel = "spec/security_model.md"
    text = SECURITY_MODEL.read_text(encoding="utf-8")
    block = section_from_source(rel, text, "Selected Account-STARK Residual Matrix", errors)
    if not block:
        return

    required_markers = [
        "Path2AccountStarkCreationPayloadV1",
        "Path2AccountStarkPayloadKatBoundaryReportV1",
        "Path2AccountStarkFriQromResidualReportV1",
        "Path2AccountStarkResidualMatrixReportV1",
        "account_stark_payload_binding_break",
        "AIR relation soundness",
        "account_stark_air_soundness_break",
        "FRI argument-of-knowledge extractor",
        "account_stark_fri_aok_extractor_break",
        "Zero-knowledge simulator and no-export statement",
        "account_stark_simulator_or_no_export_break",
        "FRI soundness margin and recursion ledger",
        "fri_soundness_margin_break",
        "account-STARK FRI transcript QROM theorem",
        "Production admission gate and review disposition",
        "account_stark_production_gate_break",
        "Legacy proof-route disposition",
        "proof_route_authority_break",
    ]
    for marker in required_markers:
        if marker not in block:
            errors.append(f"{rel} ## Selected Account-STARK Residual Matrix: missing {marker!r}")


def check_direct_theorem_preflight_markers(errors: list[str]) -> None:
    rel = "tools/parameter-search/src/production_reference.rs"
    if not RUST_PRODUCTION_REFERENCE.exists():
        errors.append(f"{rel}: missing production reference source")
        return

    text = RUST_PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    for marker in DIRECT_THEOREM_PREFLIGHT_MARKERS:
        if marker not in text:
            errors.append(f"{rel}: missing direct-theorem preflight marker {marker!r}")

    if "blockers.extend_from_slice(production_deployment_direct_theorem_blockers())" not in text:
        errors.append(f"{rel}: direct-theorem blockers are not wired into preflight")


def check_account_creation_proof_boundary_preflight_markers(errors: list[str]) -> None:
    rel = "tools/parameter-search/src/production_reference.rs"
    if not RUST_PRODUCTION_REFERENCE.exists():
        errors.append(f"{rel}: missing production reference source")
        return

    text = RUST_PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    for marker in ACCOUNT_CREATION_PROOF_BOUNDARY_PREFLIGHT_MARKERS:
        if marker not in text:
            errors.append(
                f"{rel}: missing account-creation proof-boundary preflight marker {marker!r}"
            )

    if (
        "blockers.extend_from_slice(production_deployment_account_creation_proof_boundary_blockers())"
        not in text
    ):
        errors.append(f"{rel}: account-creation proof-boundary blockers are not wired into preflight")


def check_account_stark_residual_preflight_markers(errors: list[str]) -> None:
    rel = "tools/parameter-search/src/production_reference.rs"
    if not RUST_PRODUCTION_REFERENCE.exists():
        errors.append(f"{rel}: missing production reference source")
        return

    text = RUST_PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    for marker in ACCOUNT_STARK_RESIDUAL_PREFLIGHT_MARKERS:
        if marker not in text:
            errors.append(f"{rel}: missing account-STARK residual preflight marker {marker!r}")

    if "blockers.extend_from_slice(production_deployment_account_stark_residual_blockers())" not in text:
        errors.append(f"{rel}: account-STARK residual blockers are not wired into preflight")


def check_account_stark_payload_kat_manifest_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_STARK_PAYLOAD_KAT_MANIFEST_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-STARK payload KAT manifest artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-STARK payload KAT manifest marker {marker!r}"
                )


def check_account_stark_payload_byte_model_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_STARK_PAYLOAD_BYTE_MODEL_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-STARK payload byte model artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-STARK payload byte model marker {marker!r}"
                )


def check_account_stark_fri_qrom_residual_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_STARK_FRI_QROM_RESIDUAL_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-STARK FRI/QROM residual artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-STARK FRI/QROM residual marker {marker!r}"
                )


def check_account_stark_fri_aok_boundary_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_STARK_FRI_AOK_BOUNDARY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-STARK FRI/AoK boundary artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-STARK FRI/AoK boundary marker {marker!r}"
                )


def check_account_creation_sampler_timing_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_CREATION_SAMPLER_TIMING_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-creation sampler/timing artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-creation sampler/timing marker {marker!r}"
                )


def check_account_creation_no_export_boundary_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_CREATION_NO_EXPORT_BOUNDARY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(
                f"{rel}: missing account-creation no-export boundary artifact"
            )
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-creation no-export boundary marker {marker!r}"
                )


def check_account_creation_proof_boundary_manifest_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_CREATION_PROOF_BOUNDARY_MANIFEST_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(
                f"{rel}: missing account-creation proof boundary manifest artifact"
            )
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-creation proof boundary manifest marker {marker!r}"
                )


def check_direct_lifecycle_boundary_markers(errors: list[str]) -> None:
    for rel, markers in DIRECT_LIFECYCLE_BOUNDARY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing direct lifecycle boundary artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing direct lifecycle boundary marker {marker!r}"
                )


def check_direct_qrom_action_boundary_markers(errors: list[str]) -> None:
    for rel, markers in DIRECT_QROM_ACTION_BOUNDARY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing direct QROM/action boundary artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing direct QROM/action boundary marker {marker!r}"
                )


def check_direct_theorem_boundary_manifest_markers(errors: list[str]) -> None:
    for rel, markers in DIRECT_THEOREM_BOUNDARY_MANIFEST_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing direct theorem boundary manifest artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing direct theorem boundary manifest marker {marker!r}"
                )


def check_boundary_manifest_consistency_markers(errors: list[str]) -> None:
    for rel, markers in BOUNDARY_MANIFEST_CONSISTENCY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing boundary manifest consistency artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing boundary manifest consistency marker {marker!r}"
                )


def check_preflight_boundary_consistency_markers(errors: list[str]) -> None:
    for rel, markers in PREFLIGHT_BOUNDARY_CONSISTENCY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing preflight boundary consistency artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing preflight boundary consistency marker {marker!r}"
                )


def check_account_stark_payload_kat_vector_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_STARK_PAYLOAD_KAT_VECTOR_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-STARK payload KAT vector artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-STARK payload KAT vector marker {marker!r}"
                )


def check_account_stark_payload_kat_vector_consistency_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_STARK_PAYLOAD_KAT_VECTOR_CONSISTENCY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(
                f"{rel}: missing account-STARK payload KAT vector consistency artifact"
            )
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-STARK payload KAT vector consistency marker {marker!r}"
                )


def check_account_creation_frame_kat_vector_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_CREATION_FRAME_KAT_VECTOR_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing account-creation frame KAT vector artifact")
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-creation frame KAT vector marker {marker!r}"
                )


def check_account_creation_frame_kat_consistency_markers(errors: list[str]) -> None:
    for rel, markers in ACCOUNT_CREATION_FRAME_KAT_CONSISTENCY_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(
                f"{rel}: missing account-creation frame KAT consistency artifact"
            )
            continue

        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(
                    f"{rel}: missing account-creation frame KAT consistency marker {marker!r}"
                )


def check_ethereum_positioning_note(
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    rel = "research/ethereum_account_abstraction_positioning.md"
    if not ETHEREUM_POSITIONING.exists():
        errors.append(f"{rel}: missing Ethereum positioning note")
        return

    text = ETHEREUM_POSITIONING.read_text(encoding="utf-8")
    if "Status: ACTIVE_SUPPORT" not in "\n".join(text.splitlines()[:8]):
        errors.append(f"{rel}: missing active support status marker")

    source_context = section_from_source(rel, text, "Source Context", errors)
    if source_context and "Primary-source snapshot:" not in source_context:
        errors.append(f"{rel} ## Source Context: missing primary-source snapshot marker")

    non_goals = section_from_source(rel, text, "Non-Goals For Ethereum Positioning", errors)
    if non_goals:
        required_non_claims = [
            "Ethereum has selected Descender",
            "EIP-8141, EIP-7932, or any other draft EIP is guaranteed",
            "smart-account prototype is equivalent to L1 protocol adoption",
        ]
        for phrase in required_non_claims:
            if phrase not in non_goals:
                errors.append(
                    f"{rel} ## Non-Goals For Ethereum Positioning: "
                    f"missing non-claim phrase {phrase!r}"
                )

    consequences = section_from_source(rel, text, "Tracker Consequences", errors)
    if not consequences:
        return

    refs = set(TRACKER_REF.findall(consequences))
    required_refs = {ref for ref in tracker_rows if ref.startswith("ADOPT-ETH-")}
    missing = sorted(required_refs - refs)
    extra = sorted(ref for ref in refs if ref not in tracker_rows)
    if missing:
        errors.append(f"{rel} ## Tracker Consequences: missing tracker refs {missing}")
    if extra:
        errors.append(f"{rel} ## Tracker Consequences: unknown tracker refs {extra}")


def main() -> int:
    errors: list[str] = []
    tracker_rows = tracker_statuses()
    if not tracker_rows:
        errors.append("TRACKER.md: no tracker IDs found")

    text = SPEC.read_text(encoding="utf-8")
    check_project_framing(text, errors)
    check_requirement_classes(text, errors)
    check_readiness_semantics(text, errors)
    check_capability_matrix(text, tracker_rows, errors)
    check_gate_traceability(
        text,
        "Direct-Mode Adoption Gates",
        "Direct-mode gate traceability",
        tracker_rows,
        errors,
        DIRECT_MODE_GATE_CONTRACTS,
    )
    check_gate_traceability(
        text,
        "Mode B Adoption Gates",
        "Mode B gate traceability",
        tracker_rows,
        errors,
        MODE_B_GATE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Specification Baseline Surfaces",
        "SPEC-",
        SPEC_BASELINE_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Direct-Mode Implementation And Proof Surfaces",
        "DIR-",
        DIRECT_IMPLEMENTATION_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Aggregation And Rollup Surfaces",
        "AGG-",
        AGGREGATION_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Ethereum Adoption Surfaces",
        "ADOPT-ETH-",
        ETHEREUM_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Security-Level Adoption Surfaces",
        "ADOPT-128-",
        SECURITY_LEVEL_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Hardware And Custody Adoption Surfaces",
        "ADOPT-HW-",
        HARDWARE_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Policy Feature Surfaces",
        "POL-",
        POLICY_SURFACE_CONTRACTS,
    )
    check_policy_traceability(text, tracker_rows, errors)
    check_non_claims(text, errors)
    check_product_bar(text, errors)
    check_spec_open_work_sections(tracker_rows, errors)
    check_selected_account_stark_residual_matrix(errors)
    check_direct_theorem_preflight_markers(errors)
    check_account_creation_proof_boundary_preflight_markers(errors)
    check_account_stark_residual_preflight_markers(errors)
    check_account_stark_payload_byte_model_markers(errors)
    check_account_stark_payload_kat_manifest_markers(errors)
    check_account_stark_fri_qrom_residual_markers(errors)
    check_account_stark_fri_aok_boundary_markers(errors)
    check_account_creation_sampler_timing_markers(errors)
    check_account_creation_no_export_boundary_markers(errors)
    check_account_creation_proof_boundary_manifest_markers(errors)
    check_direct_qrom_action_boundary_markers(errors)
    check_direct_lifecycle_boundary_markers(errors)
    check_direct_theorem_boundary_manifest_markers(errors)
    check_boundary_manifest_consistency_markers(errors)
    check_preflight_boundary_consistency_markers(errors)
    check_account_stark_payload_kat_vector_markers(errors)
    check_account_stark_payload_kat_vector_consistency_markers(errors)
    check_account_creation_frame_kat_vector_markers(errors)
    check_account_creation_frame_kat_consistency_markers(errors)
    check_ethereum_positioning_note(tracker_rows, errors)

    if errors:
        print("requirement traceability check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("requirement traceability check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
