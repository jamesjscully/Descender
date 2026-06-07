//! Production-shaped Descender reference API boundary.
//!
//! This crate intentionally exposes the production-reference surface without
//! exposing the parameter-search binaries.  It is still a reference boundary,
//! not a deployment-ready cryptographic implementation: the reviewed exact
//! 256-byte account-carried codec is present, and the selected 314-byte
//! `258 B/Fq12 + projection carrier` deployment-shaped verifier is present
//! only as a fail-closed surface while the active-lane relation is promoted
//! through preflight and the remaining pad-binding/hidden-source semantics land.

pub use descender_parameter_search::production_reference::{
    ACTIVE_RESPONSE_BYTES, ALGEBRAIC_CHALLENGE_BYTES, ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK,
    CHALLENGE_BITS_PER_REP, CHALLENGE_BYTES, CHALLENGE_HARDENING_BYTES, CHALLENGE_HASH_DOMAIN,
    DIGEST_BYTES, FIELD_BITS, GROUPS, HIDDEN_SOURCE_FALLBACK_ACCOUNT_OPENING_FRAME_DOMAIN,
    HIDDEN_SOURCE_FALLBACK_ACCOUNT_OPENING_FRAME_PROFILE,
    HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES,
    HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_DOMAIN,
    HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_PROFILE,
    HIDDEN_SOURCE_FALLBACK_DIRECT_EXPANDED_OPENING_DEPLOYMENT_STATUS,
    HIDDEN_SOURCE_FALLBACK_EXPANDED_OPENING_COMMITMENT_DOMAIN,
    HIDDEN_SOURCE_FALLBACK_PAD_PROJECTION_LANES, HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_CHECK_BYTES,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_EXPORTER_UNIMPLEMENTED,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_COEFFICIENTS,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS, HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES,
    HiddenSourceFallbackProjectionAccountCommitmentBreakExport,
    HiddenSourceFallbackProjectionAccountCommitmentSplitFrameReplayInput,
    HiddenSourceFallbackProjectionAccountCommitmentSplitReplayInput,
    HiddenSourceFallbackProjectionAccountSlotFrameSeed,
    HiddenSourceFallbackProjectionAccountWitnessOpening, HiddenSourceFallbackProjectionBreakExport,
    HiddenSourceFallbackProjectionCarrier, HiddenSourceFallbackProjectionCheckedCoreBreakExport,
    HiddenSourceFallbackProjectionCheckedCoreBreakInput,
    HiddenSourceFallbackProjectionCheckedCoreFailure,
    HiddenSourceFallbackProjectionCheckedCoreTranscriptDifferenceInput,
    HiddenSourceFallbackProjectionDecodedCarrier,
    HiddenSourceFallbackProjectionExpandedOpeningCommitmentReplayInput,
    HiddenSourceFallbackProjectionExpandedOpeningFrameReplayInput,
    HiddenSourceFallbackProjectionHiddenBreakInput, HiddenSourceFallbackProjectionHiddenFailure,
    HiddenSourceFallbackProjectionHiddenOpeningBreakExport,
    HiddenSourceFallbackProjectionLeakageAccounting,
    HiddenSourceFallbackProjectionOpeningProjectionReplayInput,
    HiddenSourceFallbackProjectionPadBreakExport,
    HiddenSourceFallbackProjectionPadPrfProjectionReplayInput,
    HiddenSourceFallbackProjectionPadPrfSeedReplayInput,
    HiddenSourceFallbackProjectionPublicBridgeStatement,
    HiddenSourceFallbackProjectionResponseLinkKernelCertificate,
    HiddenSourceFallbackProjectionResponseLinkKernelReplayInput,
    HiddenSourceFallbackProjectionSemanticPredicateResult,
    HiddenSourceFallbackProjectionVerifierState, LINEAR_SIGMA_MASKED_WITNESS_BYTES,
    LINEAR_SIGMA_PROOF_COMMITMENT_BYTES, LINEAR_SIGMA_TARGET_PROOF_BYTES,
    LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES, LOCAL_FREE, LOCAL_OPENING_BYTES,
    LOCAL_OPENING_BYTES_PER_REP, LOCAL_PIVOT, LOCAL_SYNDROME_SLOPE,
    PATH2_NATIVE_ACCEPTED_SPEND_NUMERIC_EVIDENCE_SCHEMA,
    PATH2_NATIVE_AGGREGATION_MODE_AGGREGATE_COMPATIBLE, PATH2_NATIVE_AGGREGATION_MODE_DIRECT,
    PATH2_NATIVE_AUTHORIZATION_CLASS_DOMAIN, PATH2_NATIVE_AUTHORIZATION_LOCAL_MODE_DIRECT,
    PATH2_NATIVE_BOUNDARY_DOMAIN, PATH2_NATIVE_CHECKED_CORE_BYTE_INSTANTIATION_EVIDENCE_SCHEMA,
    PATH2_NATIVE_CHECKED_CORE_FRAME_BODY_BYTES, PATH2_NATIVE_CHECKED_CORE_FRAME_BYTES,
    PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN, PATH2_NATIVE_CHECKED_CORE_FRAME_VERSION,
    PATH2_NATIVE_CHECKED_CORE_PARAMETER_SET_Q12289, PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES,
    PATH2_NATIVE_CHECKED_CORE_VERIFIER_FAMILY_FQ12, PATH2_NATIVE_CONSENSUS_CARRIER_BODY_BYTES,
    PATH2_NATIVE_CONSENSUS_CARRIER_BYTES, PATH2_NATIVE_CONSENSUS_CARRIER_DOMAIN,
    PATH2_NATIVE_CONSENSUS_CARRIER_VERSION, PATH2_NATIVE_CONSENSUS_PREFLIGHT_REPORT_SCHEMA,
    PATH2_NATIVE_CONSTANT_TIME_AUDIT_SCOPE_SCHEMA,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_ACCEPTED_SPEND_NUMERIC_BOUND,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_COMPOSITION,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_FRAME,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_CONSENSUS_CARRIER,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_CONSTANT_TIME, PATH2_NATIVE_DEPLOYMENT_BLOCKER_FACADE_EXPORTS,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_EXECUTION_SOURCE,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_REDUCTION_CERTIFICATE,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_GLOBAL_DOMAIN_REGISTRY,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_HASH_QROM_REVIEW,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_SEMANTICS,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_PROJECTION_ENCODERS,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_PUBLIC_VIEW_SERIALIZERS,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_QROM_TARGET_HORIZON,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_SIGNER_VERIFIER_PIPELINE,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_STORAGE,
    PATH2_NATIVE_DEPLOYMENT_BLOCKER_VERIFIER_PREFLIGHT_REPORT,
    PATH2_NATIVE_DEPLOYMENT_PREFLIGHT_SCHEMA,
    PATH2_NATIVE_FULL_SIGNATURE_REDUCTION_EVIDENCE_SCHEMA,
    PATH2_NATIVE_HASH_BYTE_DOMAIN_EVIDENCE_SCHEMA, PATH2_NATIVE_HASH_QROM_REVIEW_SCHEMA,
    PATH2_NATIVE_NO_EXPORT_SERIALIZER_EVIDENCE_SCHEMA, PATH2_NATIVE_NO_OBSTRUCTION_FLAGS_DOMAIN,
    PATH2_NATIVE_PROJECTION_ACCOUNT_DOMAIN, PATH2_NATIVE_PROJECTION_CONTEXT_DOMAIN,
    PATH2_NATIVE_PROJECTION_LIFECYCLE_DOMAIN, PATH2_NATIVE_PROJECTION_PREIMAGE_DA_DOMAIN,
    PATH2_NATIVE_PROJECTION_SPEND_DOMAIN, PATH2_NATIVE_PROJECTION_STATE_DOMAIN,
    PATH2_NATIVE_PROJECTION_TRANSCRIPT_DOMAIN, PATH2_NATIVE_PUBLIC_KEY_IDENTITY_DOMAIN,
    PATH2_NATIVE_SIGNER_PIPELINE_PUBLIC_OUTPUT_SCHEMA,
    PATH2_NATIVE_STORAGE_LIFECYCLE_HARDENING_SCOPE_SCHEMA,
    PATH2_NATIVE_VERIFIER_PIPELINE_VERDICT_SCHEMA, PATH2_PUBLIC_NATIVE_ACCOUNT_VIEW_SCHEMA,
    PATH2_PUBLIC_NATIVE_SPEND_VIEW_SCHEMA, PATH2_REVIEWED_FQ12_FIVE_SOURCE_CLOSURE_EVIDENCE_SCHEMA,
    PATH2_SELECTED_RUST_MODEL_CONCRETE_DEFINITION_CHECKLIST_SCHEMA, PIVOT_COORDS, PK_BYTES,
    PK_PAYLOAD_BYTES, PK_SEED_BYTES, PRODUCTION_CALLBACK_BOUNDARIES,
    PRODUCTION_DEPLOYMENT_BLOCKER_CALLBACK_BOUNDARIES, PRODUCTION_DEPLOYMENT_BLOCKER_CONSTANT_TIME,
    PRODUCTION_DEPLOYMENT_BLOCKER_EXTERNAL_REVIEW, PRODUCTION_DEPLOYMENT_BLOCKER_FINAL_VERIFIER,
    PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS,
    PRODUCTION_DEPLOYMENT_BLOCKER_PAD_OPENING_BINDING,
    PRODUCTION_DEPLOYMENT_BLOCKER_SELECTED_VERIFIER, PRODUCTION_DEPLOYMENT_BLOCKER_STORAGE,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_DOMAIN,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_REPORT_DOMAIN,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_CERTIFICATE_DOMAIN, PRODUCTION_DEPLOYMENT_PREFLIGHT_NO_BLOCKER,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_READY_ASSERTION_DOMAIN,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_REPORT_DOMAIN, PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_BLOCKED,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_DEPLOYABLE,
    PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_DOMAIN,
    PRODUCTION_DEPLOYMENT_REQUIRED_CALLBACK_BOUNDARIES,
    PRODUCTION_FILE_BACKED_TRANSACTIONAL_KV_SCHEMA, PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON,
    PRODUCTION_LIFECYCLE_LOCK_SCHEMA, PRODUCTION_LIFECYCLE_STORE_SCHEMA,
    PRODUCTION_LOCAL_CANONICAL_NODE_KV_STORE_SCHEMA, PRODUCTION_REFERENCE_FACADE_PROOF_MODEL,
    PRODUCTION_REVIEWED_FQ12_STATE_ASSISTED_LIFECYCLE_STORE_SCHEMA,
    PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON,
    PRODUCTION_SELECTED_PROJECTION_INVALID_OPENING_REASON,
    PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON, PRODUCTION_SPENT_SLOT_LEDGER_SCHEMA,
    PRODUCTION_SQLITE_CANONICAL_NODE_KV_STORE_SCHEMA,
    PRODUCTION_TARGET_CHAIN_DATABASE_ADAPTER_PROFILE_SCHEMA,
    PRODUCTION_TARGET_CHAIN_KV_ANCHOR_SCHEMA, PRODUCTION_TARGET_CHAIN_KV_LIFECYCLE_SCHEMA,
    PRODUCTION_TARGET_CHAIN_KV_NAMESPACE, PRODUCTION_TARGET_CHAIN_KV_SPENT_SCHEMA,
    PRODUCTION_TARGET_CHAIN_SINGLE_TRANSACTION_SEMANTICS,
    PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
    PRODUCTION_TRUSTED_RESTORE_ANCHOR_SCHEMA, PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON, PROFILE,
    Path2NativeAcceptedSpendNumericEvidenceV1, Path2NativeAccountProjectionV1,
    Path2NativeBoundaryAssemblyV1, Path2NativeCheckedCoreByteInstantiationEvidenceV1,
    Path2NativeCheckedCoreFrameV1, Path2NativeConsensusCarrierPreflightReportV1,
    Path2NativeConsensusCarrierV1, Path2NativeConstantTimeAuditScopeV1,
    Path2NativeContextProjectionV1, Path2NativeDeploymentPreflightV1,
    Path2NativeFullSignatureReductionEvidenceV1, Path2NativeHashByteDomainEvidenceV1,
    Path2NativeHashQromReviewV1, Path2NativeLifecycleProjectionV1,
    Path2NativeNoExportSerializerEvidenceV1, Path2NativeProjectionBundleV1,
    Path2NativeSignerPipelinePublicOutputV1, Path2NativeSignerVerifierPipelineEvidenceV1,
    Path2NativeSpendProjectionV1, Path2NativeStateProjectionV1,
    Path2NativeStorageLifecycleHardeningScopeV1, Path2NativeTranscriptProjectionV1,
    Path2NativeVerifierPipelineVerdictV1, Path2ReviewedFq12FiveSourceClosureEvidenceV1,
    Path2SelectedRustModelConcreteDefinitionChecklistV1, ProductionAccountBindingError,
    ProductionAccountCommitment, ProductionAccountLifecycleManager, ProductionAccountSlot,
    ProductionAccountSlotRecord, ProductionAccountSlotSeed, ProductionAccountState,
    ProductionCallbackBoundaryClassification, ProductionCallbackErrorPolicy,
    ProductionCanonicalNodeKvAdapter, ProductionCanonicalNodeKvStore,
    ProductionChainStateFileStores, ProductionCompactChallenge, ProductionCompactResponses,
    ProductionDeploymentPreflight, ProductionDeploymentPreflightBlocker,
    ProductionDeploymentPreflightBlockerReport, ProductionDeploymentPreflightCertificate,
    ProductionDeploymentPreflightReadyAssertion, ProductionDeploymentPreflightReport,
    ProductionDeploymentPreflightVerdict, ProductionFileBackedTransactionalKv,
    ProductionFileBackedTransactionalKvLock, ProductionInMemorySelectedProjectionTargetChainDb,
    ProductionInMemoryTransactionalKv, ProductionKeypair, ProductionLifecycleError,
    ProductionLifecycleFileStore, ProductionLifecycleLockRecord, ProductionLifecycleRestoreAnchor,
    ProductionLifecycleSnapshot, ProductionLifecycleStoreLock, ProductionLinearSigmaTargetProof,
    ProductionLocalCanonicalNodeKvStore, ProductionLocalCanonicalNodeKvStoreLock,
    ProductionLocalOpenings, ProductionMockBlockchain, ProductionMockBlockchainBlock,
    ProductionMockBlockchainDb, ProductionMockBlockchainTransaction,
    ProductionProfiledTargetChainKvDb, ProductionPublicKey, ProductionPublicLocalConsistency,
    ProductionPublicPadOpeningConsistency, ProductionPublicPadOpeningStatement,
    ProductionRejectionReportFields, ProductionReorgBurnRecord,
    ProductionReviewedAccountCarriedProof, ProductionReviewedAccountCarriedSignatureParts,
    ProductionReviewedFq12AccountCarriedProof, ProductionReviewedFq12AccountCarriedSignatureParts,
    ProductionReviewedFq12AccountCarriedWithFallbackProjection,
    ProductionReviewedFq12AccountCarriedWithHiddenSourceProof,
    ProductionReviewedFq12DeploymentBlockerReport, ProductionReviewedFq12HiddenSourceProof,
    ProductionReviewedFq12HiddenSourceProofAcceptanceContract,
    ProductionReviewedFq12HiddenSourceProofAcceptanceObligation,
    ProductionReviewedFq12HiddenSourceProofAcceptanceObligationReport,
    ProductionReviewedFq12HiddenSourceProofContext,
    ProductionReviewedFq12HiddenSourceProofReadinessReport,
    ProductionReviewedFq12HiddenSourceProofReadinessVerdict,
    ProductionReviewedFq12HiddenSourceProofSemanticsGate,
    ProductionReviewedFq12HiddenSourceProofTranscript, ProductionReviewedFq12HiddenSourceStatement,
    ProductionReviewedFq12MaskedProjectionSoundnessCertificate,
    ProductionReviewedFq12StateAssistedAccountLifecycleManager,
    ProductionReviewedFq12StateAssistedAccountSlot,
    ProductionReviewedFq12StateAssistedAccountSlotRecord,
    ProductionReviewedFq12StateAssistedAccountState,
    ProductionReviewedFq12StateAssistedCarrierHandle,
    ProductionReviewedFq12StateAssistedLifecycleFileStore,
    ProductionReviewedFq12StateAssistedLifecycleSnapshot,
    ProductionReviewedFq12StateAssistedOpening,
    ProductionReviewedFq12StateAssistedRoPokPublicInput, ProductionSecretKey,
    ProductionSelectedProjectionChainStateFileStores,
    ProductionSelectedProjectionConsensusSpendRecord,
    ProductionSelectedProjectionConsensusTransition, ProductionSelectedProjectionTargetChainDb,
    ProductionSelectedProjectionTargetChainDbAdapter, ProductionSelectedProjectionTargetChainKvDb,
    ProductionSignature, ProductionSignatureParts, ProductionSlotReservation,
    ProductionSpentSlotKey, ProductionSpentSlotLedger, ProductionSpentSlotLedgerFileStore,
    ProductionSpentSlotLedgerStoreLock, ProductionSpentSlotRecord,
    ProductionSqliteCanonicalNodeKvStore, ProductionTargetChainCanonicalStoreConformanceReport,
    ProductionTargetChainDatabaseAdapterProfile, ProductionTargetChainKvWrite,
    ProductionTargetChainTransactionalKv, ProductionTrustedRestoreAnchorFileStore,
    ProductionTrustedRestoreAnchorRecord, ProductionTrustedRestoreAnchorStoreLock,
    ProductionVerification, ProductionVerificationRejection, ProductionVerifiedSignature,
    PublicNativeAccountViewV1, PublicNativeSpendViewV1, Q, REPS, RESPONSE_BOUND,
    RESPONSE_LINK_BOUNDED_KERNEL_BOUND, RESPONSE_LINK_BOUNDED_KERNEL_CANONICAL_BASIS_SHA256,
    RESPONSE_LINK_BOUNDED_KERNEL_COVER_BALL_VECTOR_SHA256,
    RESPONSE_LINK_BOUNDED_KERNEL_MANIFEST_PAYLOAD_SHA256,
    RESPONSE_LINK_BOUNDED_KERNEL_MATRIX_TRANSCRIPT_SHA256,
    RESPONSE_LINK_BOUNDED_KERNEL_NONZERO_COVER_BALL_VECTORS,
    RESPONSE_LINK_BOUNDED_KERNEL_REDUCED_BASIS_SHA256, RESPONSE_LINK_BOUNDED_KERNEL_VISITED_LEAVES,
    RESPONSE_LINK_BOUNDED_KERNEL_VISITED_NODES, REVIEWED_ACCOUNT_CARRIED_MASKED_RESPONSE_BITS,
    REVIEWED_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES,
    REVIEWED_ACCOUNT_CARRIED_MASKED_RESPONSE_SLACK_BITS, REVIEWED_ACCOUNT_CARRIED_MATRIX_DOMAIN,
    REVIEWED_ACCOUNT_CARRIED_PROOF_BYTES, REVIEWED_ACCOUNT_CARRIED_PUBLIC_ROWS,
    REVIEWED_ACCOUNT_CARRIED_RESPONSE_RESIDUES, REVIEWED_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS,
    REVIEWED_ACCOUNT_CARRIED_SYNDROME_SLACK_BITS,
    REVIEWED_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES, REVIEWED_ACCOUNT_LINK_BYTES,
    REVIEWED_ACCOUNT_RANDOMIZER_COMMITMENT_BYTES, REVIEWED_ACCOUNT_RESPONSE_LINK_CODE_BYTES,
    REVIEWED_ACCOUNT_WITNESS_COMMITMENT_BYTES,
    REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATION_COUNT,
    REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATIONS,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_BITS, REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_LABELS,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_MASK_PRF_DOMAIN_SEPARATION,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_PAD_SEED_OPENING,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_SOUNDNESS_ANALYSIS,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_STATEMENT_BINDING,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_TRANSCRIPT_BINDING,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_WITNESS_COMMITMENT_OPENING,
    REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_ZERO_KNOWLEDGE_ANALYSIS,
    REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT,
    REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATIONS,
    REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH,
    REVIEWED_FQ12_ACCOUNT_CARRIED_CHALLENGE_ACTION_DOMAIN,
    REVIEWED_FQ12_ACCOUNT_CARRIED_MASKED_RESPONSE_BITS,
    REVIEWED_FQ12_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES,
    REVIEWED_FQ12_ACCOUNT_CARRIED_MASKED_RESPONSE_SLACK_BITS,
    REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN, REVIEWED_FQ12_ACCOUNT_CARRIED_PROOF_BYTES,
    REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS, REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES,
    REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS,
    REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_SLACK_BITS,
    REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES,
    REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES,
    REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_HIDDEN_SOURCE_PROOF_BYTES,
    REVIEWED_FQ12_ACCOUNT_WITNESS_PROJECTION_OPENING_DOMAIN,
    REVIEWED_FQ12_DEPLOYMENT_BLOCKER_REPORT_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_ACCEPTANCE_CONTRACT_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_ACCEPTANCE_OBLIGATION_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_ACCEPTANCE_OBLIGATION_REPORT_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_BYTES, REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_REPORT_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_BLOCKED,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_DEPLOYABLE,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_SEMANTICS_GATE_DOMAIN,
    REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_SEMANTICS_PROFILE,
    REVIEWED_FQ12_HIDDEN_SOURCE_STATEMENT_DOMAIN,
    REVIEWED_FQ12_MASKED_STATE18_PACKED_WITNESS_OPENING_DOMAIN,
    REVIEWED_FQ12_PACKED_WITNESS_STATE18_PACKED_BITS,
    REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN,
    REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_OFFSET,
    REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS, REVIEWED_FQ12_PACKED_WITNESS_STATE18_SLACK_BITS,
    REVIEWED_FQ12_PAD_DOMAIN, REVIEWED_FQ12_PAD_FRAME_BYTES, REVIEWED_FQ12_PAD_OPENING_VIEW_DOMAIN,
    REVIEWED_FQ12_PAD_PRF_COUNTER_LIMIT, REVIEWED_FQ12_PAD_SEED_BYTES,
    REVIEWED_FQ12_PAD_SEED_OPENING_DOMAIN, REVIEWED_FQ12_STATE_ASSISTED_CARRIER_DOMAIN,
    REVIEWED_FQ12_STATE_ASSISTED_CARRIER_FRAME_BYTES, REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN,
    REVIEWED_FQ12_STATE_ASSISTED_OPENING_FRAME_BYTES, REVIEWED_FQ12_STATE_ASSISTED_OPENING_PROFILE,
    REVIEWED_FQ12_STATE_ASSISTED_PROJECTION_CARRIER_BINDING_DOMAIN,
    REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN, REVIEWED_FQ12_STATE_ASSISTED_RO_POK_FIXED_BYTES,
    REVIEWED_FQ12_STATE_ASSISTED_RO_POK_PROFILE, REVIEWED_RESPONSE_LINK_COORDS,
    REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES, SIS_FULL_RESPONSE_PUBLIC_ROWS,
    SIS_FULL_RESPONSE_RESIDUES, SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS,
    SIS_FULL_RESPONSE_SYNDROME_SLACK_BITS, SLOT_ID_BYTES, STATEFUL_SPEND_SIGNATURE_BYTES,
    STATEFUL_TRANSACTION_SIGNATURE_BYTES, VIEW_COORDS, account_state_from_slot_seeds,
    apply_selected_projection_consensus_opening_frame, assemble_path2_native_boundary_v1,
    challenge_bytes, challenge_hash_frame, classify_hidden_source_fallback_checked_core_break,
    classify_hidden_source_fallback_checked_core_transcript_difference_break,
    classify_hidden_source_fallback_opening_break, complete_verified_signature,
    consume_verified_signature, decode_challenge,
    decode_hidden_source_fallback_account_witness_opening_frame,
    decode_hidden_source_fallback_checked_core_transcript_frame,
    decode_hidden_source_fallback_projection_carrier, decode_linear_sigma_target_proof,
    decode_openings, decode_path2_native_checked_core_frame_v1,
    decode_path2_native_consensus_carrier_v1, decode_production_signature, decode_responses,
    decode_reviewed_account_carried_proof, decode_reviewed_account_carried_signature,
    decode_reviewed_fq12_account_carried_proof, decode_reviewed_fq12_account_carried_signature,
    decode_reviewed_fq12_account_carried_with_fallback_projection,
    decode_reviewed_fq12_account_carried_with_hidden_source_proof,
    encode_hidden_source_fallback_account_witness_opening_frame,
    encode_hidden_source_fallback_checked_core_transcript_frame, encode_linear_sigma_target_proof,
    encode_openings, encode_production_signature, encode_responses,
    encode_reviewed_account_carried_masked_response, encode_reviewed_account_carried_proof,
    encode_reviewed_account_carried_signature,
    encode_reviewed_fq12_account_carried_masked_response,
    encode_reviewed_fq12_account_carried_proof, encode_reviewed_fq12_account_carried_signature,
    encode_reviewed_fq12_account_carried_with_fallback_projection,
    encode_reviewed_fq12_account_carried_with_hidden_source_proof, hash_frame_to_challenge_bytes,
    hidden_source_fallback_account_projection_values_from_opening,
    hidden_source_fallback_account_witness_opening_frame_bytes,
    hidden_source_fallback_account_witness_opening_semantic_value,
    hidden_source_fallback_checked_core_response_difference_from_transcripts,
    hidden_source_fallback_checked_core_transcript_frame_bytes,
    hidden_source_fallback_expanded_account_opening_commitment,
    hidden_source_fallback_pad_prf_projection_values_from_seed,
    hidden_source_fallback_projection_account_state_from_slot_frames,
    hidden_source_fallback_projection_leakage_accounting,
    hidden_source_fallback_projection_values_from_hidden_material,
    legacy_stateful_challenge_hash_frame, pack_reviewed_account_carried_syndrome_digest,
    pack_reviewed_fq12_account_carried_syndrome_digest, pack_reviewed_fq12_packed_witness_state18,
    pack_reviewed_fq12_packed_witness_state18_from_parts,
    path2_native_accepted_spend_numeric_evidence_all_facts_true,
    path2_native_accepted_spend_numeric_evidence_report_verified_v1,
    path2_native_accepted_spend_numeric_evidence_v1,
    path2_native_accepted_spend_numeric_evidence_v1_digest,
    path2_native_accepted_spend_numeric_evidence_v1_frame,
    path2_native_account_projection_v1_bytes, path2_native_account_projection_v1_digest,
    path2_native_boundary_v1_digest, path2_native_checked_core_accept_v1,
    path2_native_checked_core_byte_instantiation_evidence_all_facts_true,
    path2_native_checked_core_byte_instantiation_evidence_v1,
    path2_native_checked_core_byte_instantiation_evidence_v1_digest,
    path2_native_checked_core_byte_instantiation_evidence_v1_frame,
    path2_native_checked_core_frame_v1_bytes, path2_native_checked_core_frame_v1_challenge,
    path2_native_checked_core_frame_v1_digest, path2_native_consensus_carrier_preflight_report_v1,
    path2_native_consensus_carrier_preflight_report_v1_digest,
    path2_native_consensus_carrier_preflight_report_v1_frame,
    path2_native_consensus_carrier_v1_bytes, path2_native_consensus_carrier_v1_digest,
    path2_native_consensus_carrier_v1_from_assembly, path2_native_constant_time_audit_scope_v1,
    path2_native_constant_time_audit_scope_v1_digest, path2_native_context_projection_v1_bytes,
    path2_native_context_projection_v1_digest, path2_native_deployment_preflight_v1,
    path2_native_deployment_preflight_v1_digest, path2_native_deployment_preflight_v1_frame,
    path2_native_full_signature_reduction_evidence_all_facts_true,
    path2_native_full_signature_reduction_evidence_report_verified_v1,
    path2_native_full_signature_reduction_evidence_v1,
    path2_native_full_signature_reduction_evidence_v1_digest,
    path2_native_full_signature_reduction_evidence_v1_frame,
    path2_native_hash_byte_domain_evidence_all_facts_true,
    path2_native_hash_byte_domain_evidence_v1, path2_native_hash_byte_domain_evidence_v1_digest,
    path2_native_hash_byte_domain_evidence_v1_frame, path2_native_hash_qrom_review_v1,
    path2_native_lifecycle_projection_v1_bytes, path2_native_lifecycle_projection_v1_digest,
    path2_native_no_export_serializer_evidence_all_facts_true,
    path2_native_no_export_serializer_evidence_v1,
    path2_native_no_export_serializer_evidence_v1_digest,
    path2_native_no_export_serializer_evidence_v1_frame, path2_native_no_obstruction_flags_digest,
    path2_native_projection_preimage_da_digest_v1, path2_native_projection_preimage_da_v1_bytes,
    path2_native_public_key_identity_digest_v1, path2_native_signer_pipeline_public_output_v1,
    path2_native_signer_pipeline_public_output_v1_digest,
    path2_native_signer_pipeline_public_output_v1_frame,
    path2_native_signer_verifier_pipeline_evidence_from_prevalidated_entropy_root_v1,
    path2_native_signer_verifier_pipeline_evidence_v1_digest,
    path2_native_signer_verifier_pipeline_evidence_v1_frame,
    path2_native_spend_authorization_class_digest_v1, path2_native_spend_projection_v1_bytes,
    path2_native_spend_projection_v1_digest, path2_native_state_projection_v1_bytes,
    path2_native_state_projection_v1_digest, path2_native_storage_lifecycle_hardening_scope_v1,
    path2_native_storage_lifecycle_hardening_scope_v1_digest,
    path2_native_transcript_projection_v1_bytes, path2_native_transcript_projection_v1_digest,
    path2_native_verifier_pipeline_verdict_v1_digest,
    path2_native_verifier_pipeline_verdict_v1_frame,
    path2_reviewed_fq12_five_source_closure_evidence_all_facts_true,
    path2_reviewed_fq12_five_source_closure_evidence_report_verified_v1,
    path2_reviewed_fq12_five_source_closure_evidence_v1,
    path2_reviewed_fq12_five_source_closure_evidence_v1_digest,
    path2_reviewed_fq12_five_source_closure_evidence_v1_frame,
    path2_selected_rust_model_concrete_definition_checklist_v1,
    path2_selected_rust_model_concrete_definition_checklist_v1_digest,
    path2_selected_rust_model_concrete_definition_checklist_v1_frame,
    production_callback_boundary_classification, production_canonical_node_store_conformance_check,
    production_deployment_preflight, production_deployment_preflight_blocker_digest,
    production_deployment_preflight_blocker_report,
    production_deployment_preflight_blocker_report_digest,
    production_deployment_preflight_blockers, production_deployment_preflight_certificate,
    production_deployment_preflight_certificate_digest,
    production_deployment_preflight_ready_assertion,
    production_deployment_preflight_ready_assertion_digest, production_deployment_preflight_report,
    production_deployment_preflight_report_digest, production_deployment_preflight_verdict,
    production_deployment_preflight_verdict_digest,
    production_profiled_target_chain_db_from_canonical_store,
    production_registered_domain_schema_constants, public_native_account_view_v1,
    public_native_account_view_v1_json, public_native_spend_view_v1,
    public_native_spend_view_v1_json, public_pad_opening_statement,
    public_pad_opening_statement_from_consistency, rejection_report_fields,
    require_deployment_callback_boundaries, require_production_deployment_preflight_ready,
    require_reviewed_fq12_account_carried_signature_carried_deployable,
    require_reviewed_fq12_hidden_source_proof_deployable_verdict,
    reviewed_account_carried_matrix_entry, reviewed_account_carried_response_syndrome,
    reviewed_account_carried_response_syndrome_digest,
    reviewed_fq12_account_carried_expected_response_syndrome,
    reviewed_fq12_account_carried_hidden_source_statement,
    reviewed_fq12_account_carried_matrix_entry,
    reviewed_fq12_account_carried_pad_opening_statement,
    reviewed_fq12_account_carried_response_syndrome,
    reviewed_fq12_account_carried_response_syndrome_digest,
    reviewed_fq12_account_carried_signature_carried_deployment_blocker_report,
    reviewed_fq12_account_carried_signature_carried_hidden_source_proof_context,
    reviewed_fq12_account_carried_signature_carried_readiness_report,
    reviewed_fq12_account_witness_projection_opening_digest, reviewed_fq12_challenge_action_vector,
    reviewed_fq12_deployment_blocker_report_digest,
    reviewed_fq12_hidden_source_proof_acceptance_contract,
    reviewed_fq12_hidden_source_proof_acceptance_contract_digest,
    reviewed_fq12_hidden_source_proof_acceptance_obligation_digest,
    reviewed_fq12_hidden_source_proof_acceptance_obligation_report,
    reviewed_fq12_hidden_source_proof_acceptance_obligation_report_digest,
    reviewed_fq12_hidden_source_proof_acceptance_obligations,
    reviewed_fq12_hidden_source_proof_readiness_report,
    reviewed_fq12_hidden_source_proof_readiness_report_digest,
    reviewed_fq12_hidden_source_proof_readiness_verdict,
    reviewed_fq12_hidden_source_proof_readiness_verdict_digest,
    reviewed_fq12_hidden_source_proof_semantics_gate,
    reviewed_fq12_hidden_source_proof_semantics_gate_digest,
    reviewed_fq12_hidden_source_proof_transcript,
    reviewed_fq12_hidden_source_proof_transcript_digest,
    reviewed_fq12_hidden_source_statement_digest, reviewed_fq12_pad_opening_consistency_from_seed,
    reviewed_fq12_pad_opening_statement_from_seed, reviewed_fq12_pad_prf_input_frame,
    reviewed_fq12_pad_residues_from_seed, reviewed_fq12_pad_residues_from_seed_bounded,
    reviewed_fq12_pad_seed_opening_digest,
    reviewed_fq12_selected_projection_invalid_carrier_public_reason,
    reviewed_fq12_selected_projection_invalid_opening_public_reason,
    reviewed_fq12_state_assisted_account_state_from_slot_records,
    reviewed_fq12_state_assisted_carrier_handle_digest,
    reviewed_fq12_state_assisted_carrier_handle_frame,
    reviewed_fq12_state_assisted_expected_response_syndrome,
    reviewed_fq12_state_assisted_hidden_source_statement,
    reviewed_fq12_state_assisted_opening_commitment_digest,
    reviewed_fq12_state_assisted_opening_commitment_frame,
    reviewed_fq12_state_assisted_projection_carrier_binding,
    reviewed_fq12_state_assisted_ro_pok_transcript_digest,
    reviewed_fq12_state_assisted_ro_pok_transcript_frame,
    reviewed_fq12_state_assisted_witness_syndrome,
    reviewed_response_link_bounded_kernel_certificate, reviewed_response_link_code,
    sign_reviewed_account_carried_with_account_binding,
    sign_reviewed_account_carried_with_transcript_binding,
    sign_reviewed_fq12_account_carried_with_transcript_binding, sign_with_account_binding,
    sign_with_account_binding_sanitized, sign_with_secret_key, sign_with_secret_key_sanitized,
    unpack_q_elements, unpack_q_syndrome_digest, unpack_reviewed_account_carried_masked_response,
    unpack_reviewed_account_carried_syndrome_digest,
    unpack_reviewed_fq12_account_carried_masked_response,
    unpack_reviewed_fq12_account_carried_syndrome_digest,
    unpack_reviewed_fq12_packed_witness_state18, validate_path2_native_checked_core_frame_v1,
    validate_path2_native_consensus_carrier_v1, validate_production_deployment_preflight_blocker,
    validate_production_deployment_preflight_blocker_report,
    validate_production_deployment_preflight_certificate,
    validate_production_deployment_preflight_ready_assertion,
    validate_production_deployment_preflight_report,
    validate_production_deployment_preflight_verdict,
    validate_production_target_chain_database_adapter_profile, validate_public_key_bytes,
    validate_reviewed_fq12_hidden_source_proof_acceptance_contract,
    validate_reviewed_fq12_hidden_source_proof_acceptance_obligation,
    validate_reviewed_fq12_hidden_source_proof_acceptance_obligation_report,
    validate_reviewed_fq12_hidden_source_proof_readiness_report,
    validate_reviewed_fq12_hidden_source_proof_readiness_verdict,
    validate_reviewed_fq12_masked_state18_packed_witness_opening,
    validate_reviewed_fq12_state_assisted_account_commitment, verify_account_binding,
    verify_default_deployment_callback_boundaries,
    verify_hidden_source_fallback_account_commitment_split_frames_or_export,
    verify_hidden_source_fallback_account_commitment_split_or_export,
    verify_hidden_source_fallback_checked_core_or_export,
    verify_hidden_source_fallback_expanded_opening_commitment,
    verify_hidden_source_fallback_expanded_opening_frame_commitment,
    verify_hidden_source_fallback_expanded_opening_state_slot,
    verify_hidden_source_fallback_expanded_opening_state_slot_frame,
    verify_hidden_source_fallback_hidden_material_projection_or_export,
    verify_hidden_source_fallback_opening_projection_or_export,
    verify_hidden_source_fallback_pad_prf_projection_or_export,
    verify_hidden_source_fallback_pad_prf_seed_or_export,
    verify_hidden_source_fallback_projection_public_bridge,
    verify_hidden_source_fallback_response_link_kernel_or_export,
    verify_path2_native_accepted_spend_numeric_evidence_v1,
    verify_path2_native_checked_core_byte_instantiation_evidence_v1,
    verify_path2_native_consensus_carrier_v1_against_bundle,
    verify_path2_native_deployment_pipeline_v1,
    verify_path2_native_full_signature_reduction_evidence_v1,
    verify_path2_native_hash_byte_domain_evidence_v1,
    verify_path2_native_no_export_serializer_evidence_v1,
    verify_path2_reviewed_fq12_five_source_closure_evidence_v1,
    verify_path2_selected_rust_model_concrete_definition_checklist_v1,
    verify_public_local_fiber_consistency, verify_public_pad_opening_consistency,
    verify_reviewed_account_carried_account_binding,
    verify_reviewed_account_carried_transcript_binding,
    verify_reviewed_fq12_account_carried_checked_core,
    verify_reviewed_fq12_account_carried_deployment_verifier,
    verify_reviewed_fq12_account_carried_hidden_source_proof,
    verify_reviewed_fq12_account_carried_hidden_source_proof_with_transcript,
    verify_reviewed_fq12_account_carried_hidden_source_witness,
    verify_reviewed_fq12_account_carried_sigma_equation,
    verify_reviewed_fq12_account_carried_signature_carried_deployment_verifier,
    verify_reviewed_fq12_account_carried_signature_carried_hidden_source_proof,
    verify_reviewed_fq12_account_carried_signature_carried_hidden_source_proof_with_transcript,
    verify_reviewed_fq12_account_carried_transcript_binding,
    verify_reviewed_fq12_hidden_source_proof_acceptance_contract,
    verify_reviewed_fq12_hidden_source_proof_semantics_gate,
    verify_reviewed_fq12_hidden_source_statement_witness,
    verify_reviewed_fq12_pad_opening_statement_witness, verify_reviewed_fq12_pad_seed_opening,
    verify_reviewed_fq12_state_assisted_account_witness_projection_opening,
    verify_reviewed_fq12_state_assisted_carrier_handle,
    verify_reviewed_fq12_state_assisted_checked_core,
    verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier,
    verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate,
    verify_reviewed_fq12_state_assisted_packed_witness_state18_projection_carrier_tuple,
    verify_reviewed_fq12_state_assisted_packed_witness_state18_projection_opening,
    verify_reviewed_fq12_state_assisted_pad_seed_opening,
    verify_reviewed_fq12_state_assisted_projection_carrier_tuple,
    verify_reviewed_fq12_state_assisted_ro_pok, verify_reviewed_fq12_state_assisted_slot_opening,
    verify_with_account_binding, verify_with_account_binding_sanitized,
};

/// Sign through the deployment-facing sanitized account-binding boundary.
pub fn deployment_sign_with_secret_key<ReferenceKeypair, F>(
    secret_key: &ProductionSecretKey<ReferenceKeypair>,
    account_slot: &ProductionAccountSlot,
    build_signature: F,
) -> Result<ProductionSignature, String>
where
    F: FnOnce(&ReferenceKeypair, u32, [u8; DIGEST_BYTES]) -> Result<ProductionSignature, String>,
{
    sign_with_secret_key_sanitized(secret_key, account_slot, build_signature)
}

/// Reserve, sign, and consume a slot through the deployment-facing sanitized boundary.
pub fn deployment_sign_reserved_and_consume<F>(
    manager: &mut ProductionAccountLifecycleManager,
    reservation: ProductionSlotReservation,
    build_signature: F,
) -> Result<ProductionSignature, ProductionLifecycleError>
where
    F: FnOnce(u32, [u8; DIGEST_BYTES]) -> Result<ProductionSignature, String>,
{
    manager.sign_reserved_and_consume_sanitized(reservation, build_signature)
}

/// Verify through the sanitized account-binding boundary and apply the account transition.
pub fn deployment_verify_with_account_binding<R, F, A>(
    public_key: &ProductionPublicKey,
    message: &[u8],
    account: &ProductionAccountState,
    signature: &ProductionSignature,
    verify_signature: F,
    accepted: A,
) -> Result<ProductionVerification<R>, ProductionVerificationRejection>
where
    F: FnOnce(
        &ProductionPublicKey,
        &[u8],
        &ProductionAccountState,
        &ProductionSignature,
        &ProductionVerifiedSignature,
    ) -> Result<R, String>,
    A: FnOnce(&R) -> bool,
{
    let (checked, report) = verify_with_account_binding_sanitized(
        public_key,
        message,
        account,
        signature,
        verify_signature,
    )
    .map_err(ProductionAccountBindingError::into_rejection)?;
    let is_accepted = accepted(&report);
    complete_verified_signature(account, checked, report, is_accepted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use descender_parameter_search::production_reference::{
        HiddenSourceFallbackProjectionHiddenMaterialReplayInput,
        reviewed_fq12_masked_packed_witness_state18_projection_opening,
        reviewed_fq12_masked_state18_packed_witness_opening_digest,
        reviewed_fq12_pack_masked_witness_state18_from_parts,
        reviewed_fq12_packed_witness_state18_projection_masks,
        reviewed_fq12_packed_witness_state18_projection_opening,
        verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening,
        verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple,
        verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple,
    };

    fn canonical_digest(byte: u8) -> [u8; DIGEST_BYTES] {
        let mut digest = [0_u8; DIGEST_BYTES];
        digest[0] = byte;
        digest
    }

    fn sha256_bytes(bytes: &[u8]) -> [u8; DIGEST_BYTES] {
        use sha2::{Digest, Sha256};

        Sha256::digest(bytes).into()
    }

    fn synthetic_bytes(label: &str) -> [u8; DIGEST_BYTES] {
        sha256_bytes(format!("Descender.test_vector.v1:{label}:0").as_bytes())
    }

    fn hex_digest(input: &str) -> [u8; DIGEST_BYTES] {
        assert_eq!(input.len(), DIGEST_BYTES * 2);
        let mut out = [0_u8; DIGEST_BYTES];
        for index in 0..DIGEST_BYTES {
            out[index] = u8::from_str_radix(&input[2 * index..2 * index + 2], 16)
                .expect("test digest is hex");
        }
        out
    }

    fn test_account() -> ProductionAccountState {
        account_state_from_slot_seeds(
            [ProductionAccountSlotSeed {
                witness_commitment: canonical_digest(0),
                unused: true,
            }],
            canonical_digest(0),
        )
        .expect("test account is canonical")
    }

    fn test_public_key() -> ProductionPublicKey {
        ProductionPublicKey::new(vec![0_u8; PK_BYTES]).expect("zero public key is canonical")
    }

    #[test]
    fn target_chain_database_adapter_profile_rejects_reference_or_unreviewed_bindings() {
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-testnet-finalized-state-v1",
            "chain-canonical-rocksdb-transaction-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        validate_production_target_chain_database_adapter_profile(&profile)
            .expect("concrete reviewed target-chain database profile validates");

        let mut reference_engine = profile.clone();
        reference_engine.canonical_database_engine =
            "ProductionFileBackedTransactionalKv".to_string();
        assert_eq!(
            validate_production_target_chain_database_adapter_profile(&reference_engine)
                .unwrap_err()
                .reason(),
            "target-chain database adapter profile uses a reference database engine"
        );

        let mut unsafe_anchor = profile.clone();
        unsafe_anchor.trusted_anchor_placement = "account_state_backup".to_string();
        assert_eq!(
            validate_production_target_chain_database_adapter_profile(&unsafe_anchor)
                .unwrap_err()
                .reason(),
            "target-chain database adapter profile has unapproved trusted anchor placement"
        );

        let mut missing_review = profile.clone();
        missing_review.external_storage_review_digest = [0_u8; DIGEST_BYTES];
        assert_eq!(
            validate_production_target_chain_database_adapter_profile(&missing_review)
                .unwrap_err()
                .reason(),
            "target-chain database adapter profile missing external storage review digest"
        );

        let mut non_atomic = profile.clone();
        non_atomic.transaction_semantics = "eventual_multi_write".to_string();
        assert_eq!(
            validate_production_target_chain_database_adapter_profile(&non_atomic)
                .unwrap_err()
                .reason(),
            "target-chain database adapter profile lacks single transaction semantics"
        );
    }

    #[test]
    fn production_reference_facade_excludes_selected_material_helpers() {
        let source = std::fs::read_to_string(file!()).unwrap();
        let facade = source
            .split("#[cfg(test)]")
            .next()
            .expect("facade section precedes tests");
        assert!(
            !facade.contains("pub use descender_parameter_search::production_reference::*"),
            "production-reference facade must not glob re-export the lower reference module"
        );

        let facade_exports = |symbol: &str| {
            facade
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .any(|token| token == symbol)
        };

        for forbidden in [
            "ProductionReviewedFq12HiddenSourceWitness",
            "ProductionReviewedFq12MaskedState18PackedWitnessOpening",
            "HiddenSourceFallbackProjectionHiddenMaterialReplayInput",
            "reviewed_fq12_packed_witness_state18_projection_opening",
            "reviewed_fq12_packed_witness_state18_projection_masks",
            "reviewed_fq12_pack_masked_witness_state18_from_parts",
            "reviewed_fq12_masked_packed_witness_state18_projection_opening",
            "reviewed_fq12_masked_state18_packed_witness_opening_frame",
            "reviewed_fq12_masked_state18_packed_witness_opening_digest",
            "reviewed_fq12_masked_state18_packed_witness_opening",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple",
        ] {
            assert!(
                !facade_exports(forbidden),
                "selected-material helper remains exported by production-reference facade: {forbidden}"
            );
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestCanonicalTransactionalKv {
        inner: ProductionInMemoryTransactionalKv,
    }

    impl TestCanonicalTransactionalKv {
        fn new() -> Self {
            Self {
                inner: ProductionInMemoryTransactionalKv::new(),
            }
        }
    }

    impl ProductionTargetChainTransactionalKv for TestCanonicalTransactionalKv {
        fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, ProductionLifecycleError> {
            self.inner.get(key)
        }

        fn commit_batch(
            &mut self,
            writes: Vec<ProductionTargetChainKvWrite>,
        ) -> Result<(), ProductionLifecycleError> {
            self.inner.commit_batch(writes)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestCanonicalNodeKvStore {
        canonical_database_engine: String,
        entries: std::collections::BTreeMap<Vec<u8>, Vec<u8>>,
    }

    impl TestCanonicalNodeKvStore {
        fn new(canonical_database_engine: impl Into<String>) -> Self {
            Self {
                canonical_database_engine: canonical_database_engine.into(),
                entries: std::collections::BTreeMap::new(),
            }
        }
    }

    impl ProductionCanonicalNodeKvStore for TestCanonicalNodeKvStore {
        fn canonical_database_engine(&self) -> &str {
            &self.canonical_database_engine
        }

        fn load_committed_value(
            &self,
            profile: &ProductionTargetChainDatabaseAdapterProfile,
            key: &[u8],
        ) -> Result<Option<Vec<u8>>, ProductionLifecycleError> {
            if self.canonical_database_engine != profile.canonical_database_engine {
                return Err(ProductionLifecycleError::new(
                    None,
                    "test canonical store profile mismatch",
                ));
            }
            Ok(self.entries.get(key).cloned())
        }

        fn commit_canonical_batch(
            &mut self,
            profile: &ProductionTargetChainDatabaseAdapterProfile,
            writes: Vec<ProductionTargetChainKvWrite>,
        ) -> Result<(), ProductionLifecycleError> {
            if self.canonical_database_engine != profile.canonical_database_engine {
                return Err(ProductionLifecycleError::new(
                    None,
                    "test canonical store profile mismatch",
                ));
            }
            let mut next_entries = self.entries.clone();
            for write in writes {
                next_entries.insert(write.key, write.value);
            }
            self.entries = next_entries;
            Ok(())
        }
    }

    #[test]
    fn profiled_target_chain_kv_db_gates_adapter_construction() {
        let public_key = test_public_key();
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-testnet-finalized-state-v1",
            "chain-canonical-rocksdb-transaction-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        assert_eq!(
            ProductionProfiledTargetChainKvDb::new(
                profile.clone(),
                ProductionInMemoryTransactionalKv::new(),
            )
            .unwrap_err()
            .reason(),
            "profiled target-chain kv db cannot wrap a reference kv engine"
        );

        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame,
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("state-assisted account state is canonical");
        let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts state");
        let snapshot = manager
            .snapshot()
            .expect("state-assisted lifecycle snapshot is serializable");
        let anchor = ProductionTrustedRestoreAnchorRecord::new(
            &public_key,
            0,
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
        )
        .expect("trusted anchor placement is approved");

        let mut db =
            ProductionProfiledTargetChainKvDb::new(profile, TestCanonicalTransactionalKv::new())
                .expect("profiled canonical target-chain kv db constructs");
        db.initialize_account_state(&public_key, snapshot, anchor)
            .expect("profiled canonical target-chain kv db initializes account state");
        assert_eq!(
            db.load_selected_projection_trusted_anchor(&public_key)
                .expect("profiled target-chain kv anchor loads")
                .expect("profiled target-chain kv anchor is present")
                .minimum_snapshot_sequence,
            0
        );

        let wrong_anchor =
            ProductionTrustedRestoreAnchorRecord::new(&public_key, 0, "hardware_monotonic_counter")
                .expect("hardware counter anchor placement is approved");
        assert_eq!(
            db.initialize_account_state(
                &public_key,
                manager
                    .snapshot()
                    .expect("state-assisted lifecycle snapshot is serializable"),
                wrong_anchor,
            )
            .unwrap_err()
            .reason(),
            "profiled target-chain kv db trusted anchor placement mismatch"
        );
    }

    #[test]
    fn canonical_node_kv_adapter_builds_profiled_target_chain_db() {
        let public_key = test_public_key();
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-testnet-finalized-state-v1",
            "chain-canonical-rocksdb-transaction-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        assert_eq!(
            ProductionCanonicalNodeKvAdapter::new(
                profile.clone(),
                TestCanonicalNodeKvStore::new("other-canonical-engine"),
            )
            .unwrap_err()
            .reason(),
            "canonical node kv adapter engine mismatch"
        );

        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame,
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("state-assisted account state is canonical");
        let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts state");
        let snapshot = manager
            .snapshot()
            .expect("state-assisted lifecycle snapshot is serializable");
        let anchor = ProductionTrustedRestoreAnchorRecord::new(
            &public_key,
            0,
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
        )
        .expect("trusted anchor placement is approved");

        let mut db = production_profiled_target_chain_db_from_canonical_store(
            profile,
            TestCanonicalNodeKvStore::new("chain-canonical-rocksdb-transaction-v1"),
        )
        .expect("canonical node store builds profiled target-chain db");
        db.initialize_account_state(&public_key, snapshot, anchor)
            .expect("canonical node store initializes account state");
        assert_eq!(
            db.load_selected_projection_lifecycle_snapshot(&public_key)
                .expect("canonical node store lifecycle loads")
                .expect("canonical node store lifecycle is present")
                .snapshot_sequence,
            0
        );
    }

    #[test]
    fn local_canonical_node_kv_store_persists_profile_bound_state() {
        let public_key = test_public_key();
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-local-finalized-state-v1",
            "descender-local-canonical-json-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame,
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("state-assisted account state is canonical");
        let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts state");
        let snapshot = manager
            .snapshot()
            .expect("state-assisted lifecycle snapshot is serializable");
        let anchor = ProductionTrustedRestoreAnchorRecord::new(
            &public_key,
            0,
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
        )
        .expect("trusted anchor placement is approved");

        let path = temp_json_path("descender-local-canonical-node-kv");
        let store =
            ProductionLocalCanonicalNodeKvStore::new(&path, "descender-local-canonical-json-v1")
                .expect("local canonical node kv store constructs");
        let mut db =
            production_profiled_target_chain_db_from_canonical_store(profile.clone(), store)
                .expect("local canonical node store builds profiled db");
        db.initialize_account_state(&public_key, snapshot, anchor)
            .expect("local canonical node store initializes profile-bound state");

        let reloaded_store =
            ProductionLocalCanonicalNodeKvStore::new(&path, "descender-local-canonical-json-v1")
                .expect("local canonical node kv store reloads");
        let reloaded_db = production_profiled_target_chain_db_from_canonical_store(
            profile.clone(),
            reloaded_store,
        )
        .expect("local canonical node store rebuilds profiled db");
        assert_eq!(
            reloaded_db
                .load_selected_projection_trusted_anchor(&public_key)
                .expect("local canonical node anchor reloads")
                .expect("local canonical node anchor is present")
                .minimum_snapshot_sequence,
            0
        );

        let lock_store =
            ProductionLocalCanonicalNodeKvStore::new(&path, "descender-local-canonical-json-v1")
                .expect("local canonical node kv store reloads for lock");
        let lock = lock_store
            .try_lock()
            .expect("local canonical node kv lock can be acquired");
        assert_eq!(
            lock_store.try_lock().unwrap_err().reason(),
            "local canonical node kv store is locked"
        );
        drop(lock);

        let wrong_profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-other-finalized-state-v1",
            "descender-local-canonical-json-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            synthetic_bytes("different-external-storage-review"),
        );
        let wrong_profile_store =
            ProductionLocalCanonicalNodeKvStore::new(&path, "descender-local-canonical-json-v1")
                .expect("local canonical node kv store reloads for mismatch");
        let wrong_profile_db = production_profiled_target_chain_db_from_canonical_store(
            wrong_profile,
            wrong_profile_store,
        )
        .expect("wrong profile can construct before committed read");
        assert_eq!(
            wrong_profile_db
                .load_selected_projection_lifecycle_snapshot(&public_key)
                .unwrap_err()
                .reason(),
            "local canonical node kv profile digest mismatch"
        );

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(lock_store.lock_path());
        let _ = std::fs::remove_file(lock_store.tmp_path());
    }

    #[test]
    fn mock_blockchain_finalizes_selected_projection_spend_and_rejects_replay() {
        let public_key = test_public_key();
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-mock-chain-finalized-state-v1",
            "descender-local-canonical-json-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let expanded_opening_commitment =
            hidden_source_fallback_expanded_account_opening_commitment(&account_opening)
                .expect("expanded account opening commitment hashes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("state-assisted account state is canonical");
        let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts state");
        let snapshot = manager
            .snapshot()
            .expect("state-assisted lifecycle snapshot is serializable");
        let anchor = ProductionTrustedRestoreAnchorRecord::new(
            &public_key,
            0,
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
        )
        .expect("trusted anchor placement is approved");

        let path = temp_json_path("descender-mock-blockchain");
        let mut chain =
            ProductionMockBlockchain::new(&path, profile.clone()).expect("mock chain constructs");
        chain
            .initialize_account_state(&public_key, snapshot, anchor)
            .expect("mock chain initializes account state");
        let genesis_root = chain
            .finalized_state_root()
            .expect("mock chain genesis state root is available");
        let tx = ProductionMockBlockchainTransaction {
            public_key: public_key.clone(),
            slot_id: 0,
            account_witness_opening_frame: account_opening_frame,
        };
        let block = chain
            .finalize_block(vec![tx.clone()])
            .expect("mock chain finalizes selected projection spend");
        assert_eq!(block.height, 1);
        assert_eq!(chain.finalized_height(), 1);
        assert_eq!(block.transaction_count, 1);
        assert_eq!(block.accepted_spends.len(), 1);
        assert_eq!(
            block.accepted_spends[0].hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_ne!(block.finalized_state_root, genesis_root);
        assert_eq!(
            chain.finalize_block(vec![tx]).unwrap_err().reason(),
            "slot is already in spent ledger"
        );

        let reloaded_chain =
            ProductionMockBlockchain::new(&path, profile).expect("mock chain reloads from store");
        assert_eq!(
            reloaded_chain
                .db()
                .load_selected_projection_trusted_anchor(&public_key)
                .expect("mock chain anchor reloads")
                .expect("mock chain anchor is present")
                .minimum_snapshot_sequence,
            1
        );
        assert_eq!(
            reloaded_chain
                .finalized_state_root()
                .expect("mock chain reloaded state root is available"),
            block.finalized_state_root
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn canonical_node_store_conformance_check_exercises_reopen_and_replay() {
        let public_key = test_public_key();
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-local-conformance-chain-v1",
            "descender-local-canonical-json-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [7, 8, 9, 10, 11, 12],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let expanded_opening_commitment =
            hidden_source_fallback_expanded_account_opening_commitment(&account_opening)
                .expect("expanded account opening commitment hashes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("state-assisted account state is canonical");
        let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts state");
        let snapshot = manager
            .snapshot()
            .expect("state-assisted lifecycle snapshot is serializable");
        let anchor = ProductionTrustedRestoreAnchorRecord::new(
            &public_key,
            0,
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
        )
        .expect("trusted anchor placement is approved");
        let path = temp_json_path("descender-canonical-node-store-conformance");
        let report = production_canonical_node_store_conformance_check(
            profile,
            || ProductionLocalCanonicalNodeKvStore::new(&path, "descender-local-canonical-json-v1"),
            &public_key,
            snapshot,
            anchor,
            0,
            &account_opening_frame,
        )
        .expect("local canonical node store passes conformance check");
        assert_eq!(
            report.schema,
            "Descender.production_target_chain_canonical_store_conformance.v1"
        );
        assert_eq!(report.chain_id, "descender-local-conformance-chain-v1");
        assert_eq!(report.slot_id, 0);
        assert_eq!(
            report.accepted_spend.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(report.accepted_spend.pad_seed_digest, canonical_digest(3));
        assert_eq!(report.reloaded_snapshot_sequence, 1);
        assert_eq!(report.reloaded_spent_record_count, 1);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check == "replay is rejected from reloaded state")
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn sqlite_canonical_node_store_passes_target_chain_conformance_check() {
        let public_key = test_public_key();
        let review_digest = synthetic_bytes("external-storage-review");
        let profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-sqlite-conformance-chain-v1",
            "descender-sqlite-canonical-node-kv-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            review_digest,
        );
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [13, 14, 15, 16, 17, 18],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let expanded_opening_commitment =
            hidden_source_fallback_expanded_account_opening_commitment(&account_opening)
                .expect("expanded account opening commitment hashes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("state-assisted account state is canonical");
        let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts state");
        let snapshot = manager
            .snapshot()
            .expect("state-assisted lifecycle snapshot is serializable");
        let anchor = ProductionTrustedRestoreAnchorRecord::new(
            &public_key,
            0,
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
        )
        .expect("trusted anchor placement is approved");
        let path = temp_json_path("descender-sqlite-canonical-node-store").with_extension("sqlite");
        let report = production_canonical_node_store_conformance_check(
            profile.clone(),
            || {
                ProductionSqliteCanonicalNodeKvStore::new(
                    &path,
                    "descender-sqlite-canonical-node-kv-v1",
                )
            },
            &public_key,
            snapshot,
            anchor,
            0,
            &account_opening_frame,
        )
        .expect("sqlite canonical node store passes conformance check");
        assert_eq!(report.chain_id, "descender-sqlite-conformance-chain-v1");
        assert_eq!(
            report.accepted_spend.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(report.reloaded_snapshot_sequence, 1);
        assert_eq!(report.reloaded_spent_record_count, 1);

        let wrong_profile = ProductionTargetChainDatabaseAdapterProfile::new(
            "descender-sqlite-conformance-chain-v1",
            "descender-sqlite-canonical-node-kv-v1",
            PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
            synthetic_bytes("wrong-external-storage-review"),
        );
        let wrong_store = ProductionSqliteCanonicalNodeKvStore::new(
            &path,
            "descender-sqlite-canonical-node-kv-v1",
        )
        .expect("sqlite canonical store reloads for mismatch");
        let wrong_db =
            production_profiled_target_chain_db_from_canonical_store(wrong_profile, wrong_store)
                .expect("wrong sqlite profile can construct before committed read");
        assert_eq!(
            wrong_db
                .load_selected_projection_lifecycle_snapshot(&public_key)
                .unwrap_err()
                .reason(),
            "sqlite canonical node kv profile digest mismatch"
        );

        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(path.with_extension("sqlite-shm"));
        let _ = std::fs::remove_file(path.with_extension("sqlite-wal"));
    }

    fn test_signature(slot_id: u32) -> ProductionSignature {
        encode_production_signature(
            slot_id,
            &[0_u8; CHALLENGE_BYTES],
            &ProductionCompactResponses {
                active_free: [[0_i16; LOCAL_FREE]; REPS],
            },
            &ProductionLocalOpenings {
                active_free: [[0_u16; LOCAL_FREE]; REPS],
            },
            &ProductionLinearSigmaTargetProof {
                proof_commitment: canonical_digest(0),
                masked_witness_response: [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
            },
        )
        .expect("test signature is canonical")
    }

    fn test_reviewed_account_carried_signature(slot_id: u32) -> ProductionSignature {
        let randomizer = pack_reviewed_account_carried_syndrome_digest(
            &[0_u16; REVIEWED_ACCOUNT_CARRIED_PUBLIC_ROWS],
        )
        .expect("test randomizer digest packs canonically");
        encode_reviewed_account_carried_signature(
            slot_id,
            &[0_u8; CHALLENGE_BYTES],
            &ProductionCompactResponses {
                active_free: [[0_i16; LOCAL_FREE]; REPS],
            },
            &ProductionLocalOpenings {
                active_free: [[0_u16; LOCAL_FREE]; REPS],
            },
            &ProductionReviewedAccountCarriedProof {
                proof_commitment: randomizer,
                masked_quotient_response: encode_reviewed_account_carried_masked_response(
                    &[0_u16; REVIEWED_ACCOUNT_CARRIED_RESPONSE_RESIDUES],
                )
                .expect("test response packs canonically"),
            },
        )
        .expect("reviewed test signature is canonical")
    }

    fn test_reviewed_fq12_account_carried_signature(slot_id: u32) -> ProductionSignature {
        let randomizer = pack_reviewed_fq12_account_carried_syndrome_digest(
            &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
        )
        .expect("test Fq12 randomizer digest packs canonically");
        encode_reviewed_fq12_account_carried_signature(
            slot_id,
            &[0_u8; CHALLENGE_BYTES],
            &ProductionCompactResponses {
                active_free: [[0_i16; LOCAL_FREE]; REPS],
            },
            &ProductionLocalOpenings {
                active_free: [[0_u16; LOCAL_FREE]; REPS],
            },
            &ProductionReviewedFq12AccountCarriedProof {
                proof_commitment: randomizer,
                masked_quotient_response: encode_reviewed_fq12_account_carried_masked_response(
                    &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES],
                )
                .expect("test Fq12 response packs canonically"),
            },
        )
        .expect("reviewed Fq12 test signature is canonical")
    }

    fn test_state_assisted_opening() -> ProductionReviewedFq12StateAssistedOpening {
        ProductionReviewedFq12StateAssistedOpening {
            slot_id: 7,
            account_witness_commitment: synthetic_bytes("account_witness_commitment"),
            account_randomizer_commitment: synthetic_bytes("account_randomizer_commitment"),
            restore_anchor_digest: synthetic_bytes("restore_anchor_digest"),
            witness_opening_digest: synthetic_bytes("witness_opening_digest"),
            pad_seed_digest: synthetic_bytes("pad_seed_digest"),
            carrier_handle_secret_digest: synthetic_bytes("carrier_handle_secret_digest"),
        }
    }

    fn test_state_assisted_carrier(
        opening_commitment: [u8; DIGEST_BYTES],
    ) -> ProductionReviewedFq12StateAssistedCarrierHandle {
        ProductionReviewedFq12StateAssistedCarrierHandle {
            hidden_source_opening_commitment: opening_commitment,
            proof_transcript_digest: synthetic_bytes("proof_transcript_digest"),
            signature_digest: synthetic_bytes("signature_digest"),
            public_key_digest: synthetic_bytes("public_key_digest"),
            message_digest: synthetic_bytes("message_digest"),
            slot_id: 7,
            carrier_handle_secret_digest: synthetic_bytes("carrier_handle_secret_digest"),
        }
    }

    fn test_state_assisted_slot(
        opening: &ProductionReviewedFq12StateAssistedOpening,
    ) -> ProductionReviewedFq12StateAssistedAccountSlot {
        ProductionReviewedFq12StateAssistedAccountSlot::new(
            opening.slot_id as u32,
            ProductionAccountCommitment::new(
                opening.account_witness_commitment,
                opening.account_randomizer_commitment,
            ),
            reviewed_fq12_state_assisted_opening_commitment_digest(opening),
            opening.restore_anchor_digest,
            opening.pad_seed_digest,
        )
    }

    fn temp_json_path(prefix: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock is after unix epoch")
                .as_nanos()
        ))
    }

    fn assert_serialized_public_state_excludes_secret_sentinels(serialized: &str) {
        for forbidden in [
            "DESCENDER_TEST_SECRET_DO_NOT_SERIALIZE",
            "reference-secret",
            "reference_keypair",
            "secret_key",
            "ProductionSecretKey",
            "hidden_witness",
            "randomizer_preimage",
            "active_pad",
            "sampler_randomness",
            "masked_witness_response",
        ] {
            assert!(
                !serialized.contains(forbidden),
                "serialized public state leaked forbidden marker {forbidden}"
            );
        }
    }

    #[test]
    fn crate_boundary_exposes_account_carried_constants() {
        assert_eq!(REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES, 256);
        assert_eq!(REVIEWED_ACCOUNT_LINK_BYTES, 68);
        assert_eq!(REVIEWED_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES, 37);
        assert_eq!(REVIEWED_ACCOUNT_CARRIED_PROOF_BYTES, 69);
        assert_eq!(REVIEWED_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES, 256);
        assert_eq!(REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS, 12);
        assert_eq!(REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES, 22);
        assert_eq!(REVIEWED_FQ12_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES, 39);
        assert_eq!(REVIEWED_FQ12_ACCOUNT_CARRIED_PROOF_BYTES, 71);
        assert_eq!(
            REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES,
            258
        );
        assert_eq!(LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES, 260);
        assert_ne!(
            REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert_eq!(REVIEWED_FQ12_STATE_ASSISTED_OPENING_FRAME_BYTES, 258);
        assert_eq!(REVIEWED_FQ12_STATE_ASSISTED_CARRIER_FRAME_BYTES, 234);
        assert_eq!(HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES, 56);
        assert_eq!(
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES,
            314
        );
    }

    #[test]
    fn crate_boundary_fails_closed_until_selected_verifier_lands() {
        let preflight = production_deployment_preflight();
        assert!(!preflight.deployment_ready);
        assert!(preflight.callback_boundaries_ok);
        assert_eq!(preflight.executable_signature_bytes, 260);
        assert_eq!(preflight.reviewed_target_signature_bytes, 256);
        assert_eq!(preflight.reviewed_fq12_fallback_signature_bytes, 258);
        assert_eq!(preflight.reviewed_fq12_hidden_source_proof_bytes, 32);
        assert_eq!(preflight.reviewed_fq12_with_hidden_source_proof_bytes, 290);
        assert_eq!(
            preflight.reviewed_fq12_fallback_projection_carrier_bytes,
            56
        );
        assert_eq!(preflight.reviewed_fq12_with_fallback_projection_bytes, 314);
        assert!(preflight.reviewed_target_codec_present);
        assert!(preflight.reviewed_target_account_binding_present);
        assert!(preflight.reviewed_target_challenge_binding_present);
        assert!(preflight.reviewed_fq12_checked_core_present);
        assert!(preflight.reviewed_fq12_fallback_projection_codec_present);
        assert!(preflight.reviewed_fq12_hidden_source_proof_callback_present);
        assert!(preflight.reviewed_fq12_hidden_source_proof_transcript_callback_present);
        assert!(preflight.reviewed_fq12_hidden_source_readiness_report_present);
        assert!(preflight.reviewed_fq12_hidden_source_readiness_verdict_present);
        assert!(preflight.reviewed_fq12_hidden_source_deployability_assertion_present);
        assert!(preflight.reviewed_fq12_active_lane_relation_certificate_present);
        assert!(preflight.reviewed_fq12_account_witness_projection_opening_binding_present);
        assert!(preflight.reviewed_fq12_direct_expanded_opening_path_archived);
        assert!(preflight.reviewed_fq12_pad_opening_witness_binding_present);
        assert!(preflight.reviewed_fq12_state_assisted_projection_carrier_tuple_binding_present);
        assert!(preflight.reviewed_fq12_fail_closed_deployment_verifier_present);
        assert!(
            preflight.reviewed_fq12_state_assisted_masked_projection_candidate_verifier_present
        );
        assert!(!preflight.reviewed_fq12_deployment_verifier_present);
        assert!(
            !preflight
                .blockers
                .contains(&PRODUCTION_DEPLOYMENT_BLOCKER_FINAL_VERIFIER)
        );
        assert!(
            !preflight
                .blockers
                .contains(&PRODUCTION_DEPLOYMENT_BLOCKER_PAD_OPENING_BINDING)
        );
        assert!(
            preflight
                .blockers
                .contains(&PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS)
        );
        let selected_source_checklist =
            path2_selected_rust_model_concrete_definition_checklist_v1();
        assert_eq!(
            selected_source_checklist.schema,
            PATH2_SELECTED_RUST_MODEL_CONCRETE_DEFINITION_CHECKLIST_SCHEMA
        );
        assert!(selected_source_checklist.evidence_report_definitions_present);
        assert!(selected_source_checklist.evidence_digest_definitions_present);
        assert!(!selected_source_checklist.evidence_proof_definitions_present);
        assert!(!selected_source_checklist.source_record_definitions_present);
        assert!(!selected_source_checklist.sub_evidence_digest_match_proof_present);
        assert!(!selected_source_checklist.all_definitions_present);
        assert_eq!(
            selected_source_checklist.missing_definition_category_count,
            3
        );
        assert!(
            !path2_selected_rust_model_concrete_definition_checklist_v1_frame(
                &selected_source_checklist
            )
            .is_empty()
        );
        let selected_source_checklist_digest =
            verify_path2_selected_rust_model_concrete_definition_checklist_v1(
                &selected_source_checklist,
            )
            .expect("selected-source concrete-definition checklist is canonical");
        assert_eq!(
            selected_source_checklist_digest,
            path2_selected_rust_model_concrete_definition_checklist_v1_digest(
                &selected_source_checklist
            )
        );
        let report = production_deployment_preflight_report();
        assert_ne!(report.digest, [0_u8; DIGEST_BYTES]);
        assert_eq!(report.preflight, preflight);
        assert_eq!(
            production_deployment_preflight_report_digest(&report),
            report.digest
        );
        validate_production_deployment_preflight_report(&report)
            .expect("production deployment preflight report is canonical");
        let blockers = production_deployment_preflight_blockers(&report)
            .expect("production deployment preflight blocker rows are canonical");
        assert_eq!(blockers.len(), preflight.blockers.len());
        assert_eq!(
            blockers[0].blocker,
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        for (index, blocker) in blockers.iter().enumerate() {
            assert_eq!(blocker.preflight_report_digest, report.digest);
            assert_eq!(blocker.index, index);
            assert_eq!(blocker.blocker, preflight.blockers[index]);
            validate_production_deployment_preflight_blocker(&report, blocker)
                .expect("production deployment preflight blocker row is canonical");
        }
        let blocker_report = production_deployment_preflight_blocker_report(&report)
            .expect("production deployment preflight blocker report is canonical");
        assert_eq!(blocker_report.preflight_report_digest, report.digest);
        assert_eq!(blocker_report.blocker_count, blockers.len());
        assert_eq!(blocker_report.blocker_digests.len(), blockers.len());
        validate_production_deployment_preflight_blocker_report(&report, &blocker_report)
            .expect("production deployment preflight blocker report is internally consistent");
        let verdict = production_deployment_preflight_verdict(&report)
            .expect("production deployment preflight verdict is canonical");
        assert_eq!(verdict.preflight_report_digest, report.digest);
        assert_eq!(verdict.blocker_report_digest, blocker_report.digest);
        assert_eq!(
            verdict.verdict,
            PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_BLOCKED
        );
        assert_eq!(
            verdict.primary_blocker,
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        validate_production_deployment_preflight_verdict(&report, &verdict)
            .expect("production deployment preflight verdict is internally consistent");
        let ready_assertion = production_deployment_preflight_ready_assertion(&report, &verdict)
            .expect("production deployment preflight ready assertion is canonical");
        assert_eq!(ready_assertion.preflight_report_digest, report.digest);
        assert_eq!(ready_assertion.blocker_report_digest, blocker_report.digest);
        assert_eq!(ready_assertion.verdict_digest, verdict.digest);
        assert!(!ready_assertion.assertion_passed);
        assert_eq!(
            ready_assertion.blocker,
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        validate_production_deployment_preflight_ready_assertion(
            &report,
            &verdict,
            &ready_assertion,
        )
        .expect("production deployment preflight ready assertion is internally consistent");
        let certificate = production_deployment_preflight_certificate(&report)
            .expect("production deployment preflight certificate is canonical");
        assert_eq!(certificate.preflight_report_digest, report.digest);
        assert_eq!(certificate.blocker_report_digest, blocker_report.digest);
        assert_eq!(certificate.verdict_digest, verdict.digest);
        assert_eq!(certificate.ready_assertion_digest, ready_assertion.digest);
        assert!(!certificate.deployment_ready);
        assert_eq!(
            certificate.primary_blocker,
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        validate_production_deployment_preflight_certificate(&report, &certificate)
            .expect("production deployment preflight certificate is internally consistent");
        assert_eq!(
            require_production_deployment_preflight_ready(&report, &verdict).unwrap_err(),
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
    }

    #[test]
    fn state_assisted_opening_frame_matches_research_vectors() {
        let opening = test_state_assisted_opening();
        let opening_frame = reviewed_fq12_state_assisted_opening_commitment_frame(&opening);
        assert_eq!(
            opening_frame.len(),
            REVIEWED_FQ12_STATE_ASSISTED_OPENING_FRAME_BYTES
        );
        let opening_digest = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);
        assert_eq!(
            opening_digest,
            hex_digest("515cdc8d8d386d25061768c9801c582ec1a75a25c24b7fffbd522b83f259d77a")
        );

        let carrier = test_state_assisted_carrier(opening_digest);
        let carrier_frame = reviewed_fq12_state_assisted_carrier_handle_frame(&carrier);
        assert_eq!(
            carrier_frame.len(),
            REVIEWED_FQ12_STATE_ASSISTED_CARRIER_FRAME_BYTES
        );
        let carrier_digest = reviewed_fq12_state_assisted_carrier_handle_digest(&carrier);
        assert_eq!(
            carrier_digest,
            hex_digest("50cc26f7d65d551955ef525b8a933f5ff32882e6c52b115b644d0fd8518635a1")
        );

        let proof = ProductionReviewedFq12HiddenSourceProof::new(carrier_digest);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_carrier_handle(&carrier, &proof)
                .expect("canonical carrier handle verifies"),
            carrier_digest
        );
        let bad_proof = ProductionReviewedFq12HiddenSourceProof::new([0_u8; DIGEST_BYTES]);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_carrier_handle(&carrier, &bad_proof).unwrap_err(),
            PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON
        );
    }

    #[test]
    fn state_assisted_opening_frame_mutations_match_research_vectors() {
        let opening = test_state_assisted_opening();
        let opening_digest = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);

        let mut swapped_frame = Vec::new();
        swapped_frame.extend_from_slice(REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN.as_bytes());
        swapped_frame.extend_from_slice(REVIEWED_FQ12_STATE_ASSISTED_OPENING_PROFILE.as_bytes());
        swapped_frame.extend_from_slice(&opening.slot_id.to_le_bytes());
        swapped_frame.extend_from_slice(&opening.account_randomizer_commitment);
        swapped_frame.extend_from_slice(&opening.account_witness_commitment);
        swapped_frame.extend_from_slice(&opening.restore_anchor_digest);
        swapped_frame.extend_from_slice(&opening.witness_opening_digest);
        swapped_frame.extend_from_slice(&opening.pad_seed_digest);
        swapped_frame.extend_from_slice(&opening.carrier_handle_secret_digest);
        let swapped_digest = sha256_bytes(&swapped_frame);
        assert_ne!(swapped_digest, opening_digest);
        assert_eq!(
            swapped_digest,
            hex_digest("219bf5718c3159034765b1ecea74facce5890e711482345124dd4474484f5e5b")
        );

        let mut mutated_domain = REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN
            .as_bytes()
            .to_vec();
        mutated_domain[0] ^= 1;
        let mut domain_frame = Vec::new();
        domain_frame.extend_from_slice(&mutated_domain);
        domain_frame.extend_from_slice(REVIEWED_FQ12_STATE_ASSISTED_OPENING_PROFILE.as_bytes());
        domain_frame.extend_from_slice(&opening.slot_id.to_le_bytes());
        domain_frame.extend_from_slice(&opening.account_witness_commitment);
        domain_frame.extend_from_slice(&opening.account_randomizer_commitment);
        domain_frame.extend_from_slice(&opening.restore_anchor_digest);
        domain_frame.extend_from_slice(&opening.witness_opening_digest);
        domain_frame.extend_from_slice(&opening.pad_seed_digest);
        domain_frame.extend_from_slice(&opening.carrier_handle_secret_digest);
        let domain_digest = sha256_bytes(&domain_frame);
        assert_ne!(domain_digest, opening_digest);
        assert_eq!(
            domain_digest,
            hex_digest("11021ca998d376e4cb09e15d2282dc2d054a1ce0e15257b6ed1ca24bfc6d4398")
        );

        let mut pad_mutated = opening;
        pad_mutated.pad_seed_digest[0] ^= 1;
        let pad_digest = reviewed_fq12_state_assisted_opening_commitment_digest(&pad_mutated);
        assert_ne!(pad_digest, opening_digest);
        assert_eq!(
            pad_digest,
            hex_digest("e808ea4e8d2df79880af305942858e5f46f2dd9b9e0d7b1646f72ce471fe0ab8")
        );

        let carrier = test_state_assisted_carrier(opening_digest);
        let carrier_digest = reviewed_fq12_state_assisted_carrier_handle_digest(&carrier);
        let mut message_mutated = carrier;
        message_mutated.message_digest[0] ^= 1;
        let message_digest = reviewed_fq12_state_assisted_carrier_handle_digest(&message_mutated);
        assert_ne!(message_digest, carrier_digest);
        assert_eq!(
            message_digest,
            hex_digest("1b401f098efeb73df0d2309de468e52324b54c38c5c37818307ded52aebe42fc")
        );

        let mut slot_mutated = carrier;
        slot_mutated.slot_id = 8;
        let slot_digest = reviewed_fq12_state_assisted_carrier_handle_digest(&slot_mutated);
        assert_ne!(slot_digest, carrier_digest);
        assert_eq!(
            slot_digest,
            hex_digest("27ba2773c486fe077edc8f6fa2f3d9fd0cd6eb4bd6012d344e820d1279c80a51")
        );
    }

    #[test]
    fn state_assisted_ro_pok_boundary_is_byte_bound_and_fail_closed() {
        let opening = test_state_assisted_opening();
        let opening_frame = reviewed_fq12_state_assisted_opening_commitment_frame(&opening);
        let opening_digest = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);
        let carrier_handle = test_state_assisted_carrier(opening_digest);
        let carrier_digest = reviewed_fq12_state_assisted_carrier_handle_digest(&carrier_handle);
        let input = ProductionReviewedFq12StateAssistedRoPokPublicInput {
            statement_frame: opening_frame.clone(),
            opening_commitment: opening_digest,
            proof_transcript_digest: carrier_handle.proof_transcript_digest,
            carrier: carrier_digest,
        };

        assert_eq!(
            REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN,
            "Descender.hidden_source.state_assisted.ro_pok.v1"
        );
        assert_eq!(
            REVIEWED_FQ12_STATE_ASSISTED_RO_POK_PROFILE,
            "n32.q12289.fq12.state.ro_pok.v1"
        );
        assert_eq!(
            verify_reviewed_fq12_state_assisted_ro_pok(&input).unwrap_err(),
            PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON
        );

        let canonical_frame = reviewed_fq12_state_assisted_ro_pok_transcript_frame(&input).unwrap();
        let canonical_digest =
            reviewed_fq12_state_assisted_ro_pok_transcript_digest(&input).unwrap();
        assert_ne!(canonical_digest, [0_u8; DIGEST_BYTES]);
        assert_eq!(
            canonical_frame.len(),
            REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN.len()
                + REVIEWED_FQ12_STATE_ASSISTED_RO_POK_PROFILE.len()
                + 4
                + opening_frame.len()
                + REVIEWED_FQ12_STATE_ASSISTED_RO_POK_FIXED_BYTES
        );

        let mut statement_byte_mutated = input.clone();
        statement_byte_mutated.statement_frame[0] ^= 1;
        assert_ne!(
            reviewed_fq12_state_assisted_ro_pok_transcript_digest(&statement_byte_mutated).unwrap(),
            canonical_digest
        );

        let mut statement_length_mutated = input.clone();
        statement_length_mutated.statement_frame.push(1);
        assert_ne!(
            reviewed_fq12_state_assisted_ro_pok_transcript_digest(&statement_length_mutated)
                .unwrap(),
            canonical_digest
        );

        let mut opening_mutated = input.clone();
        opening_mutated.opening_commitment[0] ^= 1;
        assert_ne!(
            reviewed_fq12_state_assisted_ro_pok_transcript_digest(&opening_mutated).unwrap(),
            canonical_digest
        );

        let mut transcript_mutated = input.clone();
        transcript_mutated.proof_transcript_digest[0] ^= 1;
        assert_ne!(
            reviewed_fq12_state_assisted_ro_pok_transcript_digest(&transcript_mutated).unwrap(),
            canonical_digest
        );

        let mut carrier_mutated = input.clone();
        carrier_mutated.carrier[0] ^= 1;
        assert_ne!(
            reviewed_fq12_state_assisted_ro_pok_transcript_digest(&carrier_mutated).unwrap(),
            canonical_digest
        );
        assert_eq!(
            verify_reviewed_fq12_state_assisted_ro_pok(&carrier_mutated).unwrap_err(),
            PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON
        );

        let mut wrong_domain_frame = Vec::new();
        wrong_domain_frame.extend_from_slice(b"Descender.hidden_source.state_assisted.ro_pok.bad");
        wrong_domain_frame
            .extend_from_slice(REVIEWED_FQ12_STATE_ASSISTED_RO_POK_PROFILE.as_bytes());
        wrong_domain_frame.extend_from_slice(&(input.statement_frame.len() as u32).to_le_bytes());
        wrong_domain_frame.extend_from_slice(&input.statement_frame);
        wrong_domain_frame.extend_from_slice(&input.opening_commitment);
        wrong_domain_frame.extend_from_slice(&input.proof_transcript_digest);
        wrong_domain_frame.extend_from_slice(&input.carrier);
        assert_ne!(sha256_bytes(&wrong_domain_frame), canonical_digest);

        let mut wrong_profile_frame = Vec::new();
        wrong_profile_frame
            .extend_from_slice(REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN.as_bytes());
        wrong_profile_frame.extend_from_slice(b"n32.q12289.fq12.state.ro_pok.bad");
        wrong_profile_frame.extend_from_slice(&(input.statement_frame.len() as u32).to_le_bytes());
        wrong_profile_frame.extend_from_slice(&input.statement_frame);
        wrong_profile_frame.extend_from_slice(&input.opening_commitment);
        wrong_profile_frame.extend_from_slice(&input.proof_transcript_digest);
        wrong_profile_frame.extend_from_slice(&input.carrier);
        assert_ne!(sha256_bytes(&wrong_profile_frame), canonical_digest);

        let mut digest_only_oracle = input;
        digest_only_oracle.carrier = canonical_digest;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_ro_pok(&digest_only_oracle).unwrap_err(),
            PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON
        );
    }

    #[test]
    fn state_assisted_account_schema_checks_prechallenge_opening() {
        let mut opening = test_state_assisted_opening();
        opening.slot_id = 0;
        opening.account_witness_commitment = canonical_digest(0);
        opening.account_randomizer_commitment = canonical_digest(0);
        let slot = test_state_assisted_slot(&opening);
        let state = ProductionReviewedFq12StateAssistedAccountState::new(vec![slot.clone()])
            .expect("state-assisted account state is canonical");
        assert_eq!(state.slots().len(), 1);
        let checked_slot = state.get_slot(0).expect("slot 0 exists");
        assert_eq!(
            verify_reviewed_fq12_state_assisted_slot_opening(checked_slot, &opening)
                .expect("opening matches prechallenge state"),
            reviewed_fq12_state_assisted_opening_commitment_digest(&opening)
        );

        let mut wrong_opening = opening;
        wrong_opening.pad_seed_digest[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_slot_opening(checked_slot, &wrong_opening)
                .unwrap_err(),
            "state-assisted pad seed digest mismatch"
        );

        let mut wrong_anchor = opening;
        wrong_anchor.restore_anchor_digest[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_slot_opening(checked_slot, &wrong_anchor)
                .unwrap_err(),
            "state-assisted restore anchor mismatch"
        );

        let mut consumed_state = state;
        consumed_state
            .mark_consumed(0)
            .expect("state-assisted slot can be consumed");
        assert_eq!(
            verify_reviewed_fq12_state_assisted_slot_opening(
                consumed_state.get_slot(0).unwrap(),
                &opening
            )
            .unwrap_err(),
            "state-assisted slot is already consumed"
        );
    }

    #[test]
    fn state_assisted_account_schema_enforces_unique_pad_seed_digest() {
        let mut opening0 = test_state_assisted_opening();
        opening0.slot_id = 0;
        opening0.account_witness_commitment = canonical_digest(0);
        opening0.account_randomizer_commitment = canonical_digest(1);
        opening0.pad_seed_digest = canonical_digest(7);
        let slot0 = test_state_assisted_slot(&opening0);

        let mut opening1 = test_state_assisted_opening();
        opening1.slot_id = 1;
        opening1.account_witness_commitment = canonical_digest(2);
        opening1.account_randomizer_commitment = canonical_digest(3);
        opening1.restore_anchor_digest = canonical_digest(4);
        opening1.witness_opening_digest = canonical_digest(5);
        opening1.carrier_handle_secret_digest = canonical_digest(6);
        opening1.pad_seed_digest = opening0.pad_seed_digest;
        let duplicate_slot1 = test_state_assisted_slot(&opening1);

        assert_eq!(
            ProductionReviewedFq12StateAssistedAccountState::new(vec![
                slot0.clone(),
                duplicate_slot1
            ])
            .unwrap_err(),
            "state-assisted pad seed digest must be unique per account"
        );

        opening1.pad_seed_digest = canonical_digest(8);
        let slot1 = test_state_assisted_slot(&opening1);
        let state = ProductionReviewedFq12StateAssistedAccountState::new(vec![slot0, slot1])
            .expect("distinct pad seed digests are accepted");
        let mut records = state
            .checked_slot_records()
            .expect("distinct pad seed digest records validate");
        records[1].pad_seed_digest = records[0].pad_seed_digest;
        assert_eq!(
            reviewed_fq12_state_assisted_account_state_from_slot_records(records).unwrap_err(),
            "state-assisted pad seed digest must be unique per account"
        );
    }

    #[test]
    fn state_assisted_pad_seed_opening_binds_public_statement() {
        let pad_seed = [0x42_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let mut observed_blocks = [false; GROUPS];
        observed_blocks[1] = true;
        observed_blocks[7] = true;
        observed_blocks[12] = true;
        let statement = reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);

        let mut opening = test_state_assisted_opening();
        opening.slot_id = 0;
        opening.account_witness_commitment = canonical_digest(0);
        opening.account_randomizer_commitment = canonical_digest(0);
        opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
        let slot = test_state_assisted_slot(&opening);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_pad_seed_opening(
                &slot, &opening, &statement, &pad_seed
            )
            .expect("pad seed opens the committed state-assisted public statement"),
            reviewed_fq12_state_assisted_opening_commitment_digest(&opening)
        );

        let mut wrong_seed = pad_seed;
        wrong_seed[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_pad_seed_opening(
                &slot,
                &opening,
                &statement,
                &wrong_seed
            )
            .unwrap_err(),
            "reviewed Fq12 pad seed opening digest mismatch"
        );

        let mut wrong_statement = statement;
        wrong_statement.digest[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_pad_seed_opening(
                &slot,
                &opening,
                &wrong_statement,
                &pad_seed
            )
            .unwrap_err(),
            "reviewed Fq12 pad-opening statement witness mismatch"
        );

        let mut wrong_opening = opening;
        wrong_opening.pad_seed_digest[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_pad_seed_opening(
                &slot,
                &wrong_opening,
                &statement,
                &pad_seed
            )
            .unwrap_err(),
            "state-assisted pad seed digest mismatch"
        );
    }

    #[test]
    fn state_assisted_account_witness_projection_opening_binds_statement() {
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness projection opening frame encodes");
        let account_opening_digest =
            reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                .expect("account witness projection opening digest is canonical");

        let mut opening = test_state_assisted_opening();
        opening.slot_id = 0;
        opening.account_witness_commitment = canonical_digest(0);
        opening.account_randomizer_commitment = canonical_digest(1);
        opening.witness_opening_digest = account_opening_digest;
        let slot = test_state_assisted_slot(&opening);

        let pad_seed = [0x24_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let observed_blocks = [false; GROUPS];
        let pad_opening_statement =
            reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);
        let mut statement = ProductionReviewedFq12HiddenSourceStatement {
            digest: [0_u8; DIGEST_BYTES],
            slot_id: 0,
            witness_commitment: opening.account_witness_commitment,
            randomizer_commitment: opening.account_randomizer_commitment,
            public_key_digest: synthetic_bytes("public_key_digest"),
            message_digest: synthetic_bytes("message_digest"),
            signature_digest: synthetic_bytes("signature_digest"),
            pad_opening_statement,
        };
        statement.digest = reviewed_fq12_hidden_source_statement_digest(&statement);

        assert_eq!(
            verify_reviewed_fq12_state_assisted_account_witness_projection_opening(
                &slot,
                &opening,
                &statement,
                &account_opening_frame,
            )
            .expect("account witness projection frame opens the committed state-assisted tuple"),
            reviewed_fq12_state_assisted_opening_commitment_digest(&opening)
        );

        let mut wrong_frame = account_opening_frame.clone();
        let last = wrong_frame
            .len()
            .checked_sub(1)
            .expect("account opening frame is nonempty");
        wrong_frame[last] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_account_witness_projection_opening(
                &slot,
                &opening,
                &statement,
                &wrong_frame,
            )
            .unwrap_err(),
            "reviewed Fq12 account witness projection opening digest mismatch"
        );

        let mut wrong_statement = statement;
        wrong_statement.witness_commitment[0] ^= 1;
        wrong_statement.digest = reviewed_fq12_hidden_source_statement_digest(&wrong_statement);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_account_witness_projection_opening(
                &slot,
                &opening,
                &wrong_statement,
                &account_opening_frame,
            )
            .unwrap_err(),
            "reviewed Fq12 account witness projection opening statement commitment mismatch"
        );

        let mut stale_statement = statement;
        stale_statement.message_digest[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_account_witness_projection_opening(
                &slot,
                &opening,
                &stale_statement,
                &account_opening_frame,
            )
            .unwrap_err(),
            "reviewed Fq12 account witness projection opening statement digest mismatch"
        );
    }

    #[test]
    fn state_assisted_projection_carrier_tuple_binds_selected_path() {
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [3, 5, 8, 13, 21, 34],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness projection opening frame encodes");
        let account_opening_digest =
            reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                .expect("account witness projection opening digest is canonical");
        let pad_seed = [0x5a_u8; REVIEWED_FQ12_PAD_SEED_BYTES];

        let mut opening = test_state_assisted_opening();
        opening.slot_id = 0;
        opening.account_witness_commitment = canonical_digest(0);
        opening.account_randomizer_commitment = canonical_digest(1);
        opening.witness_opening_digest = account_opening_digest;
        opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
        opening.carrier_handle_secret_digest = synthetic_bytes("selected_carrier_secret");
        let slot = test_state_assisted_slot(&opening);
        let opening_commitment = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);

        let observed_blocks = [false; GROUPS];
        let pad_opening_statement =
            reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);
        let mut statement = ProductionReviewedFq12HiddenSourceStatement {
            digest: [0_u8; DIGEST_BYTES],
            slot_id: 0,
            witness_commitment: opening.account_witness_commitment,
            randomizer_commitment: opening.account_randomizer_commitment,
            public_key_digest: synthetic_bytes("public_key_digest"),
            message_digest: synthetic_bytes("message_digest"),
            signature_digest: synthetic_bytes("signature_digest"),
            pad_opening_statement,
        };
        statement.digest = reviewed_fq12_hidden_source_statement_digest(&statement);

        let projection_lanes = hidden_source_fallback_projection_values_from_hidden_material(
            &account_opening,
            &pad_seed,
        )
        .expect("selected hidden material projects to carrier lanes");
        let (check_bytes, transcript_binding) =
            reviewed_fq12_state_assisted_projection_carrier_binding(
                &opening_commitment,
                &opening,
                &statement,
                &projection_lanes,
            )
            .expect("selected projection carrier binding derives");
        let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
        for (lane, value) in projection_lanes.iter().enumerate() {
            carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        carrier_bytes[24..27].copy_from_slice(&check_bytes);
        carrier_bytes[27..56].copy_from_slice(&transcript_binding);
        let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
            .expect("selected projection carrier is canonical");

        assert_eq!(
            verify_reviewed_fq12_state_assisted_projection_carrier_tuple(
                &slot,
                &opening,
                &statement,
                &account_opening_frame,
                &pad_seed,
                &carrier,
            )
            .expect("selected state-assisted tuple binds projection carrier"),
            opening_commitment
        );

        let mut wrong_check = carrier_bytes;
        wrong_check[24] ^= 1;
        let wrong_check_carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&wrong_check)
            .expect("wrong-check carrier remains canonical");
        assert_eq!(
            verify_reviewed_fq12_state_assisted_projection_carrier_tuple(
                &slot,
                &opening,
                &statement,
                &account_opening_frame,
                &pad_seed,
                &wrong_check_carrier,
            )
            .unwrap_err(),
            "reviewed Fq12 projection carrier check bytes mismatch"
        );

        let mut wrong_statement = statement;
        wrong_statement.message_digest[0] ^= 1;
        wrong_statement.digest = reviewed_fq12_hidden_source_statement_digest(&wrong_statement);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_projection_carrier_tuple(
                &slot,
                &opening,
                &wrong_statement,
                &account_opening_frame,
                &pad_seed,
                &carrier,
            )
            .unwrap_err(),
            "reviewed Fq12 projection carrier check bytes mismatch"
        );

        let mut wrong_pad_seed = pad_seed;
        wrong_pad_seed[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_projection_carrier_tuple(
                &slot,
                &opening,
                &statement,
                &account_opening_frame,
                &wrong_pad_seed,
                &carrier,
            )
            .unwrap_err(),
            "reviewed Fq12 pad seed opening digest mismatch"
        );
    }

    #[test]
    fn projection_carrier_tuple_does_not_prove_full_witness_opening() {
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [3, 5, 8, 13, 21, 34],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness projection opening frame encodes");
        let account_opening_digest =
            reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                .expect("account witness projection opening digest is canonical");
        let pad_seed = [0x5a_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let observed_blocks = [false; GROUPS];
        let pad_opening_statement =
            reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);
        let projection_lanes = hidden_source_fallback_projection_values_from_hidden_material(
            &account_opening,
            &pad_seed,
        )
        .expect("selected hidden material projects to carrier lanes");

        let witness_a = pack_reviewed_fq12_account_carried_syndrome_digest(
            &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
        )
        .expect("zero witness commitment packs canonically");
        let mut syndrome_b = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        syndrome_b[0] = 1;
        let witness_b = pack_reviewed_fq12_account_carried_syndrome_digest(&syndrome_b)
            .expect("alternate witness commitment packs canonically");
        assert_ne!(witness_a, witness_b);

        let build_accepting_tuple =
            |witness_commitment: [u8; DIGEST_BYTES], carrier_secret: &str| -> [u8; DIGEST_BYTES] {
                let mut opening = test_state_assisted_opening();
                opening.slot_id = 0;
                opening.account_witness_commitment = witness_commitment;
                opening.account_randomizer_commitment = canonical_digest(1);
                opening.witness_opening_digest = account_opening_digest;
                opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
                opening.carrier_handle_secret_digest = synthetic_bytes(carrier_secret);
                let slot = test_state_assisted_slot(&opening);
                let opening_commitment =
                    reviewed_fq12_state_assisted_opening_commitment_digest(&opening);

                let mut statement = ProductionReviewedFq12HiddenSourceStatement {
                    digest: [0_u8; DIGEST_BYTES],
                    slot_id: 0,
                    witness_commitment,
                    randomizer_commitment: opening.account_randomizer_commitment,
                    public_key_digest: synthetic_bytes("public_key_digest"),
                    message_digest: synthetic_bytes("message_digest"),
                    signature_digest: synthetic_bytes("signature_digest"),
                    pad_opening_statement,
                };
                statement.digest = reviewed_fq12_hidden_source_statement_digest(&statement);

                let (check_bytes, transcript_binding) =
                    reviewed_fq12_state_assisted_projection_carrier_binding(
                        &opening_commitment,
                        &opening,
                        &statement,
                        &projection_lanes,
                    )
                    .expect("selected projection carrier binding derives");
                let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
                for (lane, value) in projection_lanes.iter().enumerate() {
                    carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
                }
                carrier_bytes[24..27].copy_from_slice(&check_bytes);
                carrier_bytes[27..56].copy_from_slice(&transcript_binding);
                let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
                    .expect("selected projection carrier is canonical");

                verify_reviewed_fq12_state_assisted_projection_carrier_tuple(
                    &slot,
                    &opening,
                    &statement,
                    &account_opening_frame,
                    &pad_seed,
                    &carrier,
                )
                .expect("tuple accepts after rebinding to the selected projection frame")
            };

        let accepted_a = build_accepting_tuple(witness_a, "carrier_secret_a");
        let accepted_b = build_accepting_tuple(witness_b, "carrier_secret_b");
        assert_ne!(accepted_a, accepted_b);
        assert_eq!(
            unpack_reviewed_fq12_account_carried_syndrome_digest(&witness_a).unwrap(),
            [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
        );
        assert_eq!(
            unpack_reviewed_fq12_account_carried_syndrome_digest(&witness_b).unwrap(),
            syndrome_b,
        );
    }

    #[test]
    fn packed_witness_state18_projection_carrier_closes_projection_rebinding() {
        let pad_seed = [0x5a_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let observed_blocks = [false; GROUPS];
        let pad_opening_statement =
            reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);

        let build_accepting_tuple =
            |syndrome: [u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
             account_projection: [u16; HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
             carrier_secret: &str|
             -> (
                ProductionReviewedFq12StateAssistedOpening,
                ProductionReviewedFq12HiddenSourceStatement,
                HiddenSourceFallbackProjectionCarrier,
                Vec<u8>,
                [u8; DIGEST_BYTES],
            ) {
                let witness_commitment = pack_reviewed_fq12_packed_witness_state18_from_parts(
                    &syndrome,
                    &account_projection,
                )
                .expect("18-residue witness state packs canonically");
                let unpacked = unpack_reviewed_fq12_packed_witness_state18(&witness_commitment)
                    .expect("18-residue witness state unpacks canonically");
                assert_eq!(
                    &unpacked[..REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
                    syndrome.as_slice(),
                );
                assert_eq!(
                    &unpacked[REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_OFFSET
                        ..REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_OFFSET
                            + HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
                    account_projection.as_slice(),
                );
                if account_projection.iter().any(|value| *value != 0) {
                    assert_eq!(
                        unpack_reviewed_fq12_account_carried_syndrome_digest(&witness_commitment,)
                            .unwrap_err(),
                        "reviewed Fq12 account-carried syndrome digest is not canonical: high bits are set",
                    );
                }
                let account_opening =
                    reviewed_fq12_packed_witness_state18_projection_opening(&witness_commitment)
                        .expect("18-residue witness state exposes account projection opening");
                let account_opening_frame =
                    encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                        .expect("derived account opening frame encodes");
                let account_opening_digest =
                    reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                        .expect("derived account opening digest is canonical");

                let mut opening = test_state_assisted_opening();
                opening.slot_id = 0;
                opening.account_witness_commitment = witness_commitment;
                opening.account_randomizer_commitment = canonical_digest(1);
                opening.witness_opening_digest = account_opening_digest;
                opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
                opening.carrier_handle_secret_digest = synthetic_bytes(carrier_secret);
                let opening_commitment =
                    reviewed_fq12_state_assisted_opening_commitment_digest(&opening);

                let mut statement = ProductionReviewedFq12HiddenSourceStatement {
                    digest: [0_u8; DIGEST_BYTES],
                    slot_id: 0,
                    witness_commitment,
                    randomizer_commitment: opening.account_randomizer_commitment,
                    public_key_digest: synthetic_bytes("public_key_digest"),
                    message_digest: synthetic_bytes("message_digest"),
                    signature_digest: synthetic_bytes("signature_digest"),
                    pad_opening_statement,
                };
                statement.digest = reviewed_fq12_hidden_source_statement_digest(&statement);

                let projection_lanes =
                    hidden_source_fallback_projection_values_from_hidden_material(
                        &account_opening,
                        &pad_seed,
                    )
                    .expect("derived hidden material projects to carrier lanes");
                let (check_bytes, transcript_binding) =
                    reviewed_fq12_state_assisted_projection_carrier_binding(
                        &opening_commitment,
                        &opening,
                        &statement,
                        &projection_lanes,
                    )
                    .expect("selected projection carrier binding derives");
                let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
                for (lane, value) in projection_lanes.iter().enumerate() {
                    carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
                }
                carrier_bytes[24..27].copy_from_slice(&check_bytes);
                carrier_bytes[27..56].copy_from_slice(&transcript_binding);
                let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
                    .expect("selected projection carrier is canonical");

                (
                    opening,
                    statement,
                    carrier,
                    account_opening_frame,
                    opening_commitment,
                )
            };

        let syndrome_a = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        let projection_a = [3, 5, 8, 13, 21, 34];
        let (opening_a, statement_a, carrier_a, frame_a, opening_commitment_a) =
            build_accepting_tuple(syndrome_a, projection_a, "carrier_secret_a");
        assert_eq!(
            verify_reviewed_fq12_state_assisted_packed_witness_state18_projection_carrier_tuple(
                &opening_commitment_a,
                &opening_a,
                &statement_a,
                &pad_seed,
                &carrier_a,
            )
            .expect("18-residue packed witness tuple accepts"),
            frame_a,
        );

        let mut syndrome_b = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        syndrome_b[0] = 1;
        let projection_b = [4, 5, 8, 13, 21, 34];
        let (opening_b, statement_b, carrier_b, frame_b, opening_commitment_b) =
            build_accepting_tuple(syndrome_b, projection_b, "carrier_secret_b");
        assert_eq!(
            verify_reviewed_fq12_state_assisted_packed_witness_state18_projection_carrier_tuple(
                &opening_commitment_b,
                &opening_b,
                &statement_b,
                &pad_seed,
                &carrier_b,
            )
            .expect("alternate 18-residue packed witness tuple accepts"),
            frame_b,
        );

        assert_ne!(
            opening_a.account_witness_commitment,
            opening_b.account_witness_commitment
        );
        assert_ne!(frame_a, frame_b);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_packed_witness_state18_projection_carrier_tuple(
                &opening_commitment_b,
                &opening_b,
                &statement_b,
                &pad_seed,
                &carrier_a,
            )
            .unwrap_err(),
            "reviewed Fq12 packed witness state18 projection carrier tuple exported a hidden-material break"
        );
    }

    #[test]
    fn masked_witness_state18_unmasks_projection_with_pad_seed() {
        let pad_seed = [0x6b_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let observed_blocks = [false; GROUPS];
        let pad_opening_statement =
            reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);

        let build_accepting_tuple =
            |syndrome: [u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
             account_projection: [u16; HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
             carrier_secret: &str|
             -> (
                ProductionReviewedFq12StateAssistedOpening,
                ProductionReviewedFq12HiddenSourceStatement,
                HiddenSourceFallbackProjectionCarrier,
                Vec<u8>,
                [u8; DIGEST_BYTES],
            ) {
                let witness_commitment = reviewed_fq12_pack_masked_witness_state18_from_parts(
                    &syndrome,
                    &account_projection,
                    &pad_seed,
                    0,
                )
                .expect("masked 18-residue witness state packs canonically");
                let unpacked = unpack_reviewed_fq12_packed_witness_state18(&witness_commitment)
                    .expect("masked 18-residue witness state unpacks canonically");
                assert_ne!(
                    &unpacked[REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_OFFSET
                        ..REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_OFFSET
                            + HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
                    account_projection.as_slice(),
                );

                let account_opening =
                    reviewed_fq12_masked_packed_witness_state18_projection_opening(
                        &witness_commitment,
                        &pad_seed,
                        0,
                    )
                    .expect("pad seed unmasks account projection opening");
                assert_eq!(account_opening.selected_source_values, account_projection);
                let account_opening_frame =
                    encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                        .expect("derived account opening frame encodes");
                let account_opening_digest =
                    reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                        .expect("derived account opening digest is canonical");

                let mut opening = test_state_assisted_opening();
                opening.slot_id = 0;
                opening.account_witness_commitment = witness_commitment;
                opening.account_randomizer_commitment = canonical_digest(1);
                opening.witness_opening_digest = account_opening_digest;
                opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
                opening.carrier_handle_secret_digest = synthetic_bytes(carrier_secret);
                let opening_commitment =
                    reviewed_fq12_state_assisted_opening_commitment_digest(&opening);

                let mut statement = ProductionReviewedFq12HiddenSourceStatement {
                    digest: [0_u8; DIGEST_BYTES],
                    slot_id: 0,
                    witness_commitment,
                    randomizer_commitment: opening.account_randomizer_commitment,
                    public_key_digest: synthetic_bytes("public_key_digest"),
                    message_digest: synthetic_bytes("message_digest"),
                    signature_digest: synthetic_bytes("signature_digest"),
                    pad_opening_statement,
                };
                statement.digest = reviewed_fq12_hidden_source_statement_digest(&statement);

                let projection_lanes =
                    hidden_source_fallback_projection_values_from_hidden_material(
                        &account_opening,
                        &pad_seed,
                    )
                    .expect("derived hidden material projects to carrier lanes");
                let (check_bytes, transcript_binding) =
                    reviewed_fq12_state_assisted_projection_carrier_binding(
                        &opening_commitment,
                        &opening,
                        &statement,
                        &projection_lanes,
                    )
                    .expect("selected projection carrier binding derives");
                let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
                for (lane, value) in projection_lanes.iter().enumerate() {
                    carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
                }
                carrier_bytes[24..27].copy_from_slice(&check_bytes);
                carrier_bytes[27..56].copy_from_slice(&transcript_binding);
                let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
                    .expect("selected projection carrier is canonical");

                (
                    opening,
                    statement,
                    carrier,
                    account_opening_frame,
                    opening_commitment,
                )
            };

        let syndrome_a = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        let projection_a = [3, 5, 8, 13, 21, 34];
        let (opening_a, statement_a, carrier_a, frame_a, opening_commitment_a) =
            build_accepting_tuple(syndrome_a, projection_a, "masked_carrier_secret_a");
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple(
                &opening_commitment_a,
                &opening_a,
                &statement_a,
                &pad_seed,
                &carrier_a,
            )
            .expect("masked 18-residue packed witness tuple accepts"),
            frame_a,
        );
        let packed_opening_a =
            verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening(
                &opening_a,
                &statement_a,
                &pad_seed,
            )
            .expect("masked state18 full packed witness opening extracts");
        assert_eq!(packed_opening_a.witness_syndrome, syndrome_a);
        assert_eq!(packed_opening_a.account_projection, projection_a);
        assert_eq!(
            packed_opening_a.repacked_witness_commitment,
            opening_a.account_witness_commitment
        );
        validate_reviewed_fq12_masked_state18_packed_witness_opening(&packed_opening_a, &pad_seed)
            .expect("masked state18 full packed witness opening validates");
        let mut malformed_packed_opening = packed_opening_a;
        malformed_packed_opening.account_projection[0] ^= 1;
        malformed_packed_opening.digest =
            reviewed_fq12_masked_state18_packed_witness_opening_digest(&malformed_packed_opening);
        assert_eq!(
            validate_reviewed_fq12_masked_state18_packed_witness_opening(
                &malformed_packed_opening,
                &pad_seed,
            )
            .unwrap_err(),
            "reviewed Fq12 masked state18 packed witness opening fields mismatch",
        );
        let slot_a = test_state_assisted_slot(&opening_a);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple(
                &slot_a,
                &opening_a,
                &statement_a,
                &pad_seed,
                &carrier_a,
            )
            .expect("slot-bound masked 18-residue tuple accepts"),
            frame_a,
        );

        let mut wrong_pad_seed = pad_seed;
        wrong_pad_seed[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple(
                &opening_commitment_a,
                &opening_a,
                &statement_a,
                &wrong_pad_seed,
                &carrier_a,
            )
            .unwrap_err(),
            "reviewed Fq12 masked state18 packed witness projection digest mismatch",
        );
        let mut wrong_pad_opening = opening_a;
        wrong_pad_opening.pad_seed_digest[0] ^= 1;
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple(
                &slot_a,
                &wrong_pad_opening,
                &statement_a,
                &pad_seed,
                &carrier_a,
            )
            .unwrap_err(),
            "state-assisted pad seed digest mismatch",
        );

        let slot0_masks =
            reviewed_fq12_packed_witness_state18_projection_masks(&pad_seed, opening_a.slot_id);
        let slot1_masks = reviewed_fq12_packed_witness_state18_projection_masks(&pad_seed, 1);
        assert_ne!(slot0_masks, slot1_masks);
        let wrong_slot_opening = reviewed_fq12_masked_packed_witness_state18_projection_opening(
            &opening_a.account_witness_commitment,
            &pad_seed,
            1,
        )
        .expect("wrong slot id still derives a canonical but incorrect projection");
        assert_ne!(wrong_slot_opening.selected_source_values, projection_a);

        let mut syndrome_b = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        syndrome_b[0] = 1;
        let projection_b = [4, 5, 8, 13, 21, 34];
        let (opening_b, statement_b, _carrier_b, _frame_b, opening_commitment_b) =
            build_accepting_tuple(syndrome_b, projection_b, "masked_carrier_secret_b");
        assert_ne!(
            opening_a.account_witness_commitment,
            opening_b.account_witness_commitment
        );
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple(
                &slot_a,
                &opening_b,
                &statement_b,
                &pad_seed,
                &carrier_a,
            )
            .unwrap_err(),
            "state-assisted witness commitment mismatch",
        );
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple(
                &opening_commitment_b,
                &opening_b,
                &statement_b,
                &pad_seed,
                &carrier_a,
            )
            .unwrap_err(),
            "reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break",
        );
    }

    #[test]
    fn state_assisted_masked_projection_candidate_verifier_accepts_executable_tuple() {
        let public_key = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();
        let message = b"state assisted masked projection candidate verifier";
        let pad_seed = [0x42_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let syndrome = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        let projection = [3, 5, 8, 13, 21, 34];
        let witness_commitment = reviewed_fq12_pack_masked_witness_state18_from_parts(
            &syndrome,
            &projection,
            &pad_seed,
            0,
        )
        .expect("masked state18 witness packs");
        let randomizer = pack_reviewed_fq12_account_carried_syndrome_digest(&syndrome)
            .expect("zero randomizer packs");
        let proof = ProductionReviewedFq12AccountCarriedProof {
            proof_commitment: randomizer,
            masked_quotient_response: encode_reviewed_fq12_account_carried_masked_response(
                &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES],
            )
            .expect("zero response packs"),
        };
        let challenge = challenge_bytes(
            public_key.as_bytes(),
            message,
            0,
            &witness_commitment,
            &proof.proof_commitment,
        );
        let decoded_challenge = decode_challenge(&challenge).expect("challenge decodes");
        let mut openings = ProductionLocalOpenings {
            active_free: [[0_u16; LOCAL_FREE]; REPS],
        };
        for rep in 0..REPS {
            openings.active_free[rep] = reviewed_fq12_pad_residues_from_seed(
                &pad_seed,
                decoded_challenge.active_block[rep],
            );
        }
        let signature = encode_reviewed_fq12_account_carried_signature(
            0,
            &challenge,
            &ProductionCompactResponses {
                active_free: [[0_i16; LOCAL_FREE]; REPS],
            },
            &openings,
            &proof,
        )
        .expect("checked-core signature encodes");

        let account_opening = reviewed_fq12_masked_packed_witness_state18_projection_opening(
            &witness_commitment,
            &pad_seed,
            0,
        )
        .expect("masked projection opening derives");
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("projection opening frame encodes");
        let mut opening = test_state_assisted_opening();
        opening.slot_id = 0;
        opening.account_witness_commitment = witness_commitment;
        opening.account_randomizer_commitment = randomizer;
        opening.witness_opening_digest =
            reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                .expect("projection opening digest hashes");
        opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
        opening.carrier_handle_secret_digest = synthetic_bytes("candidate_carrier_secret");
        let opening_commitment = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);
        let slot = test_state_assisted_slot(&opening);
        let account = ProductionReviewedFq12StateAssistedAccountState::new(vec![slot])
            .expect("state-assisted account accepts masked state18 slot");
        let statement = reviewed_fq12_state_assisted_hidden_source_statement(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("state-assisted hidden-source statement derives");
        let projection_lanes = hidden_source_fallback_projection_values_from_hidden_material(
            &account_opening,
            &pad_seed,
        )
        .expect("hidden material projection derives");
        let (check_bytes, transcript_binding) =
            reviewed_fq12_state_assisted_projection_carrier_binding(
                &opening_commitment,
                &opening,
                &statement,
                &projection_lanes,
            )
            .expect("carrier binding derives");
        let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
        for (lane, value) in projection_lanes.iter().enumerate() {
            carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        carrier_bytes[24..27].copy_from_slice(&check_bytes);
        carrier_bytes[27..56].copy_from_slice(&transcript_binding);
        let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
            .expect("carrier encodes");
        let envelope =
            encode_reviewed_fq12_account_carried_with_fallback_projection(&signature, &carrier)
                .expect("314-byte envelope encodes");

        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier(
                &public_key,
                message,
                &account,
                &envelope,
                &opening,
                &statement,
                &pad_seed,
            )
            .expect("candidate verifier accepts executable selected tuple")
            .slot_id,
            0
        );
        let soundness =
            verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate(
                &public_key,
                message,
                &account,
                &envelope,
                &opening,
                &statement,
                &pad_seed,
            )
            .expect("candidate acceptance yields executable soundness certificate");
        assert_eq!(soundness.slot_id, 0);
        assert_eq!(soundness.statement_digest, statement.digest);
        assert_eq!(soundness.opening_commitment, opening_commitment);
        assert_eq!(soundness.checked_core_witness_syndrome, syndrome);
        assert_eq!(
            soundness.packed_witness_opening.account_projection,
            projection
        );
        assert_eq!(
            soundness.packed_witness_opening.repacked_witness_commitment,
            witness_commitment
        );
        assert_eq!(soundness.carrier_lanes, projection_lanes);
        assert_eq!(soundness.carrier_check_bytes, check_bytes);
        assert_eq!(soundness.carrier_transcript_binding, transcript_binding);

        let mut wrong_statement = statement;
        wrong_statement.message_digest[0] ^= 1;
        wrong_statement.digest = reviewed_fq12_hidden_source_statement_digest(&wrong_statement);
        assert_eq!(
            verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier(
                &public_key,
                message,
                &account,
                &envelope,
                &opening,
                &wrong_statement,
                &pad_seed,
            )
            .unwrap_err()
            .reason(),
            PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON
        );
    }

    #[test]
    fn req_pad_reject_kats_preserve_state_and_export_no_partial_material() {
        let public_key = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();
        let message = b"REQ-PAD selected-route reject KATs";
        let pad_seed = [0x42_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let syndrome = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
        let projection = [3, 5, 8, 13, 21, 34];
        let witness_commitment = reviewed_fq12_pack_masked_witness_state18_from_parts(
            &syndrome,
            &projection,
            &pad_seed,
            0,
        )
        .expect("masked state18 witness packs");
        let randomizer = pack_reviewed_fq12_account_carried_syndrome_digest(&syndrome)
            .expect("zero randomizer packs");
        let proof = ProductionReviewedFq12AccountCarriedProof {
            proof_commitment: randomizer,
            masked_quotient_response: encode_reviewed_fq12_account_carried_masked_response(
                &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES],
            )
            .expect("zero response packs"),
        };
        let challenge = challenge_bytes(
            public_key.as_bytes(),
            message,
            0,
            &witness_commitment,
            &proof.proof_commitment,
        );
        let decoded_challenge = decode_challenge(&challenge).expect("challenge decodes");
        let mut openings = ProductionLocalOpenings {
            active_free: [[0_u16; LOCAL_FREE]; REPS],
        };
        for rep in 0..REPS {
            openings.active_free[rep] = reviewed_fq12_pad_residues_from_seed(
                &pad_seed,
                decoded_challenge.active_block[rep],
            );
        }
        let signature = encode_reviewed_fq12_account_carried_signature(
            0,
            &challenge,
            &ProductionCompactResponses {
                active_free: [[0_i16; LOCAL_FREE]; REPS],
            },
            &openings,
            &proof,
        )
        .expect("checked-core signature encodes");
        let account_opening = reviewed_fq12_masked_packed_witness_state18_projection_opening(
            &witness_commitment,
            &pad_seed,
            0,
        )
        .expect("masked projection opening derives");
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("projection opening frame encodes");
        let mut opening = test_state_assisted_opening();
        opening.slot_id = 0;
        opening.account_witness_commitment = witness_commitment;
        opening.account_randomizer_commitment = randomizer;
        opening.witness_opening_digest =
            reviewed_fq12_account_witness_projection_opening_digest(&account_opening_frame)
                .expect("projection opening digest hashes");
        opening.pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
        opening.carrier_handle_secret_digest = synthetic_bytes("req_pad_reject_carrier_secret");
        let opening_commitment = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);
        let slot = test_state_assisted_slot(&opening);
        let account = ProductionReviewedFq12StateAssistedAccountState::new(vec![slot])
            .expect("state-assisted account accepts masked state18 slot");
        let statement = reviewed_fq12_state_assisted_hidden_source_statement(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("state-assisted hidden-source statement derives");
        let projection_lanes = hidden_source_fallback_projection_values_from_hidden_material(
            &account_opening,
            &pad_seed,
        )
        .expect("hidden material projection derives");
        let (check_bytes, transcript_binding) =
            reviewed_fq12_state_assisted_projection_carrier_binding(
                &opening_commitment,
                &opening,
                &statement,
                &projection_lanes,
            )
            .expect("carrier binding derives");
        let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
        for (lane, value) in projection_lanes.iter().enumerate() {
            carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        carrier_bytes[24..27].copy_from_slice(&check_bytes);
        carrier_bytes[27..56].copy_from_slice(&transcript_binding);
        let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
            .expect("carrier encodes");
        let envelope =
            encode_reviewed_fq12_account_carried_with_fallback_projection(&signature, &carrier)
                .expect("314-byte envelope encodes");

        let assert_reject = |label: &str,
                             envelope: &ProductionSignature,
                             opening: &ProductionReviewedFq12StateAssistedOpening,
                             statement: &ProductionReviewedFq12HiddenSourceStatement,
                             pad_seed: &[u8; REVIEWED_FQ12_PAD_SEED_BYTES],
                             expected_reason: &str| {
            let before = account.clone();
            let err = verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier(
                &public_key,
                message,
                &account,
                envelope,
                opening,
                statement,
                pad_seed,
            )
            .expect_err(label);
            assert_eq!(err.reason(), expected_reason, "{label}");
            assert_eq!(account, before, "{label} mutated account state");
            assert!(
                verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate(
                    &public_key,
                    message,
                    &account,
                    envelope,
                    opening,
                    statement,
                    pad_seed,
                )
                .is_err(),
                "{label} returned partial soundness material"
            );
            assert_eq!(
                account, before,
                "{label} mutated account state after certificate path"
            );
        };

        let mut wrong_seed = pad_seed;
        wrong_seed[0] ^= 1;
        assert_reject(
            "wrong pad seed",
            &envelope,
            &opening,
            &statement,
            &wrong_seed,
            PRODUCTION_SELECTED_PROJECTION_INVALID_OPENING_REASON,
        );

        let mut wrong_public_pad_statement = statement;
        wrong_public_pad_statement.pad_opening_statement.digest[0] ^= 1;
        wrong_public_pad_statement.digest =
            reviewed_fq12_hidden_source_statement_digest(&wrong_public_pad_statement);
        assert_reject(
            "wrong public pad statement",
            &envelope,
            &opening,
            &wrong_public_pad_statement,
            &pad_seed,
            PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON,
        );

        let mut observed_block_mutation = statement;
        observed_block_mutation
            .pad_opening_statement
            .observed_blocks[0] = !observed_block_mutation
            .pad_opening_statement
            .observed_blocks[0];
        observed_block_mutation
            .pad_opening_statement
            .observed_block_count = observed_block_mutation
            .pad_opening_statement
            .observed_blocks
            .iter()
            .filter(|&&observed| observed)
            .count();
        observed_block_mutation.digest =
            reviewed_fq12_hidden_source_statement_digest(&observed_block_mutation);
        assert_reject(
            "observed block mutation",
            &envelope,
            &opening,
            &observed_block_mutation,
            &pad_seed,
            PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON,
        );

        let mut local_opening_bytes = signature.as_bytes().to_vec();
        let local_opening_start = SLOT_ID_BYTES + CHALLENGE_BYTES + ACTIVE_RESPONSE_BYTES;
        local_opening_bytes[local_opening_start] ^= 1;
        let local_opening_signature = ProductionSignature::new(local_opening_bytes);
        let local_opening_envelope = encode_reviewed_fq12_account_carried_with_fallback_projection(
            &local_opening_signature,
            &carrier,
        )
        .expect("mutated local-opening envelope encodes");
        assert_reject(
            "local opening mutation",
            &local_opening_envelope,
            &opening,
            &statement,
            &pad_seed,
            "public pad-opening mismatch for block 2 in round 11",
        );

        let mut committed_opening_mismatch = opening;
        committed_opening_mismatch.pad_seed_digest[0] ^= 1;
        assert_reject(
            "committed opening mismatch",
            &envelope,
            &committed_opening_mismatch,
            &statement,
            &pad_seed,
            PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON,
        );
    }

    #[test]
    fn projection_mask_prf_frame_is_disjoint_from_pad_prf_frame() {
        let pad_seed = [0x91_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let pad_frame =
            reviewed_fq12_pad_prf_input_frame(&pad_seed, 0, 0).expect("pad frame builds");

        assert_eq!(
            pad_frame.len(),
            REVIEWED_FQ12_PAD_SEED_BYTES + REVIEWED_FQ12_PAD_FRAME_BYTES + 4
        );
        assert_eq!(
            &pad_frame[REVIEWED_FQ12_PAD_SEED_BYTES
                ..REVIEWED_FQ12_PAD_SEED_BYTES + REVIEWED_FQ12_PAD_DOMAIN.len()],
            REVIEWED_FQ12_PAD_DOMAIN.as_bytes(),
        );

        for block in 0..GROUPS {
            for counter in 0..REVIEWED_FQ12_PAD_PRF_COUNTER_LIMIT {
                reviewed_fq12_pad_prf_input_frame(&pad_seed, block, counter)
                    .expect("bounded pad frame builds");
            }
        }

        assert_eq!(
            reviewed_fq12_pad_prf_input_frame(&pad_seed, GROUPS, 0).unwrap_err(),
            format!("reviewed Fq12 pad PRF block {GROUPS} is out of range"),
        );
    }

    #[test]
    fn crate_boundary_exposes_reviewed_256_byte_codec_and_binding_check() {
        let witness = pack_reviewed_account_carried_syndrome_digest(
            &[0_u16; REVIEWED_ACCOUNT_CARRIED_PUBLIC_ROWS],
        )
        .expect("test witness digest packs canonically");
        let public_key = test_public_key();
        let message = b"reviewed crate boundary transcript binding";
        let account = account_state_from_slot_seeds(
            [ProductionAccountSlotSeed {
                witness_commitment: witness,
                unused: true,
            }],
            witness,
        )
        .expect("reviewed account state is canonical");
        let mut signature = test_reviewed_account_carried_signature(0);
        let mut parts = signature.decode_reviewed_account_carried().unwrap();
        let challenge = challenge_bytes(
            public_key.as_bytes(),
            message,
            0,
            &witness,
            &parts.proof.proof_commitment,
        );
        signature = encode_reviewed_account_carried_signature(
            0,
            &challenge,
            &parts.responses,
            &parts.openings,
            &parts.proof,
        )
        .unwrap();
        assert_eq!(signature.len(), REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES);
        parts = signature.decode_reviewed_account_carried().unwrap();
        let decoded = parts;
        assert_eq!(decoded.slot_id, 0);
        let checked =
            verify_reviewed_account_carried_account_binding(&account, &signature).unwrap();
        assert_eq!(checked.slot_id, 0);
        verify_reviewed_account_carried_transcript_binding(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("reviewed transcript binding accepts matching frame");
        let digest = reviewed_account_carried_response_syndrome_digest(
            &checked.proof.masked_quotient_response,
        )
        .expect("reviewed quotient digest is canonical");
        assert_eq!(
            unpack_reviewed_account_carried_syndrome_digest(&digest).unwrap(),
            [0_u16; REVIEWED_ACCOUNT_CARRIED_PUBLIC_ROWS]
        );
    }

    #[test]
    fn crate_boundary_exposes_reviewed_258_byte_fq12_codec_and_binding_check() {
        let witness = pack_reviewed_fq12_account_carried_syndrome_digest(
            &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
        )
        .expect("test Fq12 witness digest packs canonically");
        let public_key = test_public_key();
        let message = b"reviewed Fq12 crate boundary transcript binding";
        let account = account_state_from_slot_seeds(
            [ProductionAccountSlotSeed {
                witness_commitment: witness,
                unused: true,
            }],
            witness,
        )
        .expect("reviewed Fq12 account state is canonical");
        let mut signature = test_reviewed_fq12_account_carried_signature(0);
        let mut parts = signature.decode_reviewed_fq12_account_carried().unwrap();
        let challenge = challenge_bytes(
            public_key.as_bytes(),
            message,
            0,
            &witness,
            &parts.proof.proof_commitment,
        );
        signature = encode_reviewed_fq12_account_carried_signature(
            0,
            &challenge,
            &parts.responses,
            &parts.openings,
            &parts.proof,
        )
        .unwrap();
        assert_eq!(
            signature.len(),
            REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES
        );
        parts = signature.decode_reviewed_fq12_account_carried().unwrap();
        assert_eq!(parts.slot_id, 0);
        let checked = verify_reviewed_fq12_account_carried_transcript_binding(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("reviewed Fq12 transcript binding accepts matching frame");
        assert_eq!(checked.slot_id, 0);
        let equation_checked = verify_reviewed_fq12_account_carried_sigma_equation(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("reviewed Fq12 sigma equation accepts zero witness/randomizer sample");
        assert_eq!(equation_checked.slot_id, 0);
        let core_checked = verify_reviewed_fq12_account_carried_checked_core(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("reviewed Fq12 checked core accepts zero witness/randomizer sample");
        assert_eq!(core_checked.slot_id, 0);
        let pad_statement = reviewed_fq12_account_carried_pad_opening_statement(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("reviewed Fq12 pad-opening statement is canonical");
        assert!(pad_statement.observed_block_count > 0);
        assert_ne!(pad_statement.digest, [0_u8; DIGEST_BYTES]);
        let pad_seed = [0x42_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        let seed_statement = reviewed_fq12_pad_opening_statement_from_seed(
            &pad_seed,
            &pad_statement.observed_blocks,
        );
        verify_reviewed_fq12_pad_opening_statement_witness(&seed_statement, &pad_seed)
            .expect("reviewed Fq12 pad-opening seed witness matches derived statement");
        let hidden_statement = reviewed_fq12_account_carried_hidden_source_statement(
            &public_key,
            message,
            &account,
            &signature,
        )
        .expect("reviewed Fq12 hidden-source statement is canonical");
        assert_ne!(hidden_statement.digest, [0_u8; DIGEST_BYTES]);
        let hidden_proof = ProductionReviewedFq12HiddenSourceProof::new(
            [0xA5_u8; REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_BYTES],
        );
        let proof_transcript =
            reviewed_fq12_hidden_source_proof_transcript_digest(&hidden_statement, &hidden_proof);
        let proof_transcript_object =
            reviewed_fq12_hidden_source_proof_transcript(&hidden_statement, &hidden_proof);
        assert_ne!(proof_transcript, [0_u8; DIGEST_BYTES]);
        assert_eq!(proof_transcript_object.digest, proof_transcript);
        assert_eq!(
            proof_transcript_object.statement_digest,
            hidden_statement.digest
        );
        assert_eq!(
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_HIDDEN_SOURCE_PROOF_BYTES,
            290
        );
        let signature_with_hidden_proof =
            encode_reviewed_fq12_account_carried_with_hidden_source_proof(
                &signature,
                &hidden_proof,
            )
            .expect("reviewed Fq12 signature-plus-proof envelope is canonical");
        assert_eq!(signature_with_hidden_proof.len(), 290);
        let decoded_envelope = signature_with_hidden_proof
            .decode_reviewed_fq12_account_carried_with_hidden_source_proof()
            .expect("reviewed Fq12 signature-plus-proof envelope decodes");
        assert_eq!(decoded_envelope.signature, signature);
        assert_eq!(decoded_envelope.hidden_source_proof, hidden_proof);
        let hidden_context =
            reviewed_fq12_account_carried_signature_carried_hidden_source_proof_context(
                &public_key,
                message,
                &account,
                &signature_with_hidden_proof,
            )
            .expect("reviewed Fq12 hidden-source proof context is canonical");
        assert_eq!(hidden_context.statement, hidden_statement);
        assert_eq!(hidden_context.proof, hidden_proof);
        assert_eq!(hidden_context.transcript, proof_transcript_object);
        let deployment_report =
            reviewed_fq12_account_carried_signature_carried_deployment_blocker_report(
                &public_key,
                message,
                &account,
                &signature_with_hidden_proof,
            )
            .expect("reviewed Fq12 deployment blocker report is canonical");
        assert_eq!(
            deployment_report.blocker,
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        assert_eq!(deployment_report.context, hidden_context);
        assert_ne!(deployment_report.digest, [0_u8; DIGEST_BYTES]);
        assert_eq!(
            reviewed_fq12_deployment_blocker_report_digest(&deployment_report),
            deployment_report.digest
        );
        let semantics_gate = reviewed_fq12_hidden_source_proof_semantics_gate(&deployment_report)
            .expect("reviewed Fq12 semantics gate is canonical");
        assert_ne!(semantics_gate.digest, [0_u8; DIGEST_BYTES]);
        assert_eq!(
            reviewed_fq12_hidden_source_proof_semantics_gate_digest(&semantics_gate),
            semantics_gate.digest
        );
        let acceptance_contract =
            reviewed_fq12_hidden_source_proof_acceptance_contract(&semantics_gate)
                .expect("reviewed Fq12 acceptance contract is canonical");
        assert_ne!(acceptance_contract.digest, [0_u8; DIGEST_BYTES]);
        assert_eq!(
            reviewed_fq12_hidden_source_proof_acceptance_contract_digest(&acceptance_contract),
            acceptance_contract.digest
        );
        validate_reviewed_fq12_hidden_source_proof_acceptance_contract(&acceptance_contract)
            .expect("reviewed Fq12 acceptance contract is internally consistent");
        let obligations =
            reviewed_fq12_hidden_source_proof_acceptance_obligations(&acceptance_contract)
                .expect("reviewed Fq12 acceptance obligations are canonical");
        assert_eq!(
            obligations.len(),
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT as usize
        );
        for obligation in obligations {
            assert_eq!(obligation.contract_digest, acceptance_contract.digest);
            let expected_implemented =
                REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATIONS & obligation.bit != 0;
            assert_eq!(obligation.implemented, expected_implemented);
            assert_eq!(obligation.missing, !expected_implemented);
            validate_reviewed_fq12_hidden_source_proof_acceptance_obligation(
                &acceptance_contract,
                &obligation,
            )
            .expect("reviewed Fq12 acceptance obligation is internally consistent");
        }
        let obligation_report =
            reviewed_fq12_hidden_source_proof_acceptance_obligation_report(&acceptance_contract)
                .expect("reviewed Fq12 acceptance obligation report is canonical");
        assert_eq!(
            obligation_report.contract_digest,
            acceptance_contract.digest
        );
        assert_eq!(
            obligation_report.missing_obligation_count,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT
                - REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATION_COUNT
        );
        assert!(!obligation_report.all_obligations_implemented);
        validate_reviewed_fq12_hidden_source_proof_acceptance_obligation_report(
            &acceptance_contract,
            &obligation_report,
        )
        .expect("reviewed Fq12 acceptance obligation report is internally consistent");
        let readiness_report =
            reviewed_fq12_hidden_source_proof_readiness_report(&deployment_report)
                .expect("reviewed Fq12 readiness report is canonical");
        assert_eq!(
            readiness_report.deployment_report_digest,
            deployment_report.digest
        );
        assert_eq!(
            readiness_report.semantics_gate_digest,
            semantics_gate.digest
        );
        assert_eq!(
            readiness_report.acceptance_contract_digest,
            acceptance_contract.digest
        );
        assert_eq!(
            readiness_report.obligation_report_digest,
            obligation_report.digest
        );
        assert_eq!(
            readiness_report.missing_obligation_count,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT
                - REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATION_COUNT
        );
        assert!(!readiness_report.deployment_ready);
        validate_reviewed_fq12_hidden_source_proof_readiness_report(
            &deployment_report,
            &readiness_report,
        )
        .expect("reviewed Fq12 readiness report is internally consistent");
        assert_eq!(
            reviewed_fq12_account_carried_signature_carried_readiness_report(
                &public_key,
                message,
                &account,
                &signature_with_hidden_proof,
            )
            .expect("reviewed Fq12 signature-carried readiness report is canonical"),
            readiness_report
        );
        let readiness_verdict =
            reviewed_fq12_hidden_source_proof_readiness_verdict(&readiness_report)
                .expect("reviewed Fq12 readiness verdict is canonical");
        assert_eq!(
            readiness_verdict.readiness_report_digest,
            readiness_report.digest
        );
        assert_eq!(
            readiness_verdict.verdict,
            REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_BLOCKED
        );
        assert!(!readiness_verdict.deployment_ready);
        assert_eq!(
            readiness_verdict.missing_obligation_count,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT
                - REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATION_COUNT
        );
        validate_reviewed_fq12_hidden_source_proof_readiness_verdict(
            &readiness_report,
            &readiness_verdict,
        )
        .expect("reviewed Fq12 readiness verdict is internally consistent");
        assert_eq!(
            require_reviewed_fq12_hidden_source_proof_deployable_verdict(
                &readiness_report,
                &readiness_verdict,
            )
            .unwrap_err(),
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        assert_eq!(
            require_reviewed_fq12_account_carried_signature_carried_deployable(
                &public_key,
                message,
                &account,
                &signature_with_hidden_proof,
            )
            .unwrap_err()
            .reason(),
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        assert_eq!(
            acceptance_contract.required_obligation_bitmap,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATIONS
        );
        assert_eq!(
            acceptance_contract.implemented_obligation_bitmap,
            REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATIONS
        );
        assert_eq!(
            acceptance_contract.missing_obligation_bitmap,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATIONS
                & !REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATIONS
        );
        assert_eq!(
            acceptance_contract.required_obligation_count,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT
        );
        assert_eq!(
            acceptance_contract.implemented_obligation_count,
            REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATION_COUNT
        );
        assert_eq!(
            acceptance_contract.missing_obligation_count,
            REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT
                - REVIEWED_FQ12_ACCEPTANCE_EXECUTABLE_RELATION_OBLIGATION_COUNT
        );
        assert!(acceptance_contract.requires_statement_binding);
        assert!(acceptance_contract.requires_witness_commitment_opening);
        assert!(acceptance_contract.requires_pad_seed_opening);
        assert!(acceptance_contract.requires_transcript_binding);
        assert!(acceptance_contract.requires_soundness_analysis);
        assert!(acceptance_contract.requires_zero_knowledge_analysis);
        assert!(!acceptance_contract.all_obligations_implemented);
        assert_eq!(
            semantics_gate.profile,
            REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_SEMANTICS_PROFILE
        );
        assert_eq!(semantics_gate.transcript_digest, proof_transcript);
        assert!(!semantics_gate.acceptance_implemented);
        assert_eq!(
            verify_reviewed_fq12_hidden_source_proof_semantics_gate(&deployment_report)
                .unwrap_err(),
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        verify_reviewed_fq12_account_carried_hidden_source_proof(
            &public_key,
            message,
            &account,
            &signature,
            &hidden_proof,
            |statement, proof| {
                assert_eq!(statement.digest, hidden_statement.digest);
                assert_eq!(proof.as_bytes(), hidden_proof.as_bytes());
                assert_eq!(
                    reviewed_fq12_hidden_source_proof_transcript_digest(statement, proof),
                    proof_transcript
                );
                Ok(())
            },
        )
        .expect("reviewed Fq12 hidden-source proof callback accepts canonical statement");
        verify_reviewed_fq12_account_carried_signature_carried_hidden_source_proof_with_transcript(
            &public_key,
            message,
            &account,
            &signature_with_hidden_proof,
            |statement, proof, transcript| {
                assert_eq!(statement.digest, hidden_statement.digest);
                assert_eq!(proof.as_bytes(), hidden_proof.as_bytes());
                assert_eq!(*transcript, proof_transcript_object);
                Ok(())
            },
        )
        .expect("reviewed Fq12 transcript-aware hidden-source proof callback accepts canonical statement");
        verify_reviewed_fq12_account_carried_signature_carried_hidden_source_proof(
            &public_key,
            message,
            &account,
            &signature_with_hidden_proof,
            |statement, proof| {
                assert_eq!(statement.digest, hidden_statement.digest);
                assert_eq!(proof.as_bytes(), hidden_proof.as_bytes());
                Ok(())
            },
        )
        .expect("reviewed Fq12 signature-carried hidden-source proof callback accepts canonical statement");
        assert_eq!(
            verify_reviewed_fq12_account_carried_signature_carried_deployment_verifier(
                &public_key,
                message,
                &account,
                &signature_with_hidden_proof,
            )
            .unwrap_err()
            .reason(),
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        assert_eq!(
            verify_reviewed_fq12_account_carried_hidden_source_proof(
                &public_key,
                message,
                &account,
                &signature,
                &hidden_proof,
                |_statement, _proof| Err("private verifier detail".to_string()),
            )
            .unwrap_err()
            .reason(),
            PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON
        );
        let deployment_err = verify_reviewed_fq12_account_carried_deployment_verifier(
            &public_key,
            message,
            &account,
            &signature,
        )
        .unwrap_err();
        assert_eq!(
            deployment_err.reason(),
            PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS
        );
        let digest = reviewed_fq12_account_carried_response_syndrome_digest(
            &checked.proof.masked_quotient_response,
        )
        .expect("reviewed Fq12 quotient digest is canonical");
        assert_eq!(
            unpack_reviewed_fq12_account_carried_syndrome_digest(&digest).unwrap(),
            [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS]
        );
    }

    #[test]
    fn reviewed_258_byte_fq12_nonzero_parser_vector_matches_lean() {
        let mut challenge_bytes = [0_u8; CHALLENGE_BYTES];
        challenge_bytes[0] = 2;
        let responses = ProductionCompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        };
        let openings = ProductionLocalOpenings {
            active_free: [[1_u16; LOCAL_FREE]; REPS],
        };
        let masked_quotient_response = encode_reviewed_fq12_account_carried_masked_response(
            &[5_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES],
        )
        .expect("nonzero Fq12 masked response packs canonically");
        let expected_masked_quotient_response = [
            5_u8, 64, 1, 80, 0, 20, 0, 5, 64, 1, 80, 0, 20, 0, 5, 64, 1, 80, 0, 20, 0, 5, 64, 1,
            80, 0, 20, 0, 5, 64, 1, 80, 0, 20, 0, 5, 64, 1, 0,
        ];
        assert_eq!(masked_quotient_response, expected_masked_quotient_response);
        let proof = ProductionReviewedFq12AccountCarriedProof {
            proof_commitment: [3_u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
            masked_quotient_response,
        };
        let signature = encode_reviewed_fq12_account_carried_signature(
            7,
            &challenge_bytes,
            &responses,
            &openings,
            &proof,
        )
        .expect("nonzero Fq12 parser vector encodes canonically");
        let mut expected_signature = Vec::new();
        expected_signature.extend_from_slice(&7_u32.to_le_bytes());
        expected_signature.push(2);
        expected_signature.extend_from_slice(&[0_u8; CHALLENGE_BYTES - 1]);
        expected_signature.extend_from_slice(&[RESPONSE_BOUND as u8; ACTIVE_RESPONSE_BYTES]);
        for _ in 0..REPS * LOCAL_FREE {
            expected_signature.extend_from_slice(&1_u16.to_le_bytes());
        }
        expected_signature.extend_from_slice(&[3_u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES]);
        expected_signature.extend_from_slice(&expected_masked_quotient_response);
        assert_eq!(signature.as_bytes(), expected_signature.as_slice());
        assert_eq!(signature.len(), 258);
        let decoded = signature.decode_reviewed_fq12_account_carried().unwrap();
        assert_eq!(decoded.slot_id, 7);
        assert_eq!(decoded.challenge.active_block[0], 2);
        assert_eq!(decoded.responses, responses);
        assert_eq!(decoded.openings, openings);
        assert_eq!(decoded.proof, proof);
        assert_eq!(
            unpack_reviewed_fq12_account_carried_masked_response(
                &decoded.proof.masked_quotient_response
            )
            .unwrap(),
            [5_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES]
        );
    }

    #[test]
    fn crate_boundary_exposes_default_sanitized_callback_profile() {
        verify_default_deployment_callback_boundaries()
            .expect("default deployment callback boundary profile is sanitized");
    }

    #[test]
    fn hidden_source_fallback_projection_negative_tests_are_fail_closed() {
        assert_eq!(HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES, 56);
        assert_eq!(HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES, 12);
        assert_eq!(HIDDEN_SOURCE_FALLBACK_PROJECTION_CHECK_BYTES, 3);
        assert_eq!(
            HIDDEN_SOURCE_FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES,
            29
        );
        let leakage = hidden_source_fallback_projection_leakage_accounting();
        assert_eq!(leakage.carrier_bytes, 56);
        assert_eq!(leakage.envelope_bytes, 314);
        assert_eq!(leakage.algebraic_lane_count, 12);
        assert_eq!(leakage.algebraic_lane_bytes, 24);
        assert_eq!(leakage.check_bytes, 3);
        assert_eq!(leakage.charged_projection_bytes, 27);
        assert_eq!(leakage.transcript_binding_bytes, 29);
        assert_eq!(leakage.context_binding_bytes, 29);
        assert_eq!(leakage.accounted_carrier_bytes, 56);
        assert_eq!(leakage.unaccounted_carrier_bytes, 0);
        assert!(leakage.all_carrier_bytes_accounted);
        assert_eq!(
            HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS,
            [
                "account_witness_projection[1,0]",
                "account_witness_projection[4,1]",
                "account_witness_projection[7,0]",
                "account_witness_projection[10,1]",
                "account_witness_projection[13,0]",
                "account_witness_projection[0,1]",
                "pad_seed_projection[2,1]",
                "pad_seed_projection[7,0]",
                "pad_seed_projection[12,1]",
                "pad_seed_projection[1,0]",
                "pad_seed_projection[6,1]",
                "pad_seed_projection[11,0]",
            ]
        );
        assert_eq!(
            HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_COEFFICIENTS,
            [
                6234, 2199, 6252, 1488, 2113, 823, 1934, 5142, 10681, 10066, 5429, 5754
            ]
        );

        let carrier = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
        let decoded = decode_hidden_source_fallback_projection_carrier(&carrier)
            .expect("zero projection carrier is byte-canonical");
        assert_eq!(
            decoded.lanes,
            [0_u16; HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES]
        );
        assert_eq!(
            decoded.check_bytes,
            [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CHECK_BYTES]
        );
        assert_eq!(
            decoded.transcript_binding,
            [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES]
        );
        let projection_carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier)
            .expect("zero projection carrier wrapper is byte-canonical");
        assert_eq!(projection_carrier.as_bytes(), &carrier);
        assert_eq!(projection_carrier.decode().unwrap(), decoded);

        let mut nonzero_carrier = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
        for lane in 0..HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES {
            nonzero_carrier[2 * lane..2 * lane + 2].copy_from_slice(&(lane as u16).to_le_bytes());
        }
        nonzero_carrier[24..27].copy_from_slice(&[12, 13, 14]);
        nonzero_carrier[27..56].copy_from_slice(&[15_u8; 29]);
        let nonzero_decoded = decode_hidden_source_fallback_projection_carrier(&nonzero_carrier)
            .expect("nonzero projection carrier is byte-canonical");
        assert_eq!(
            nonzero_decoded.lanes,
            core::array::from_fn::<_, HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES, _>(|idx| {
                idx as u16
            })
        );
        assert_eq!(nonzero_decoded.check_bytes, [12, 13, 14]);
        assert_eq!(nonzero_decoded.transcript_binding, [15_u8; 29]);
        verify_hidden_source_fallback_projection_public_bridge(
            &nonzero_decoded,
            &HiddenSourceFallbackProjectionPublicBridgeStatement {
                projection_values: core::array::from_fn::<
                    _,
                    HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES,
                    _,
                >(|idx| idx as u16),
                check_bytes: [12, 13, 14],
                transcript_binding: [15_u8; 29],
            },
        )
        .expect("nonzero projection carrier bridge statement accepts");

        let fq12_signature = test_reviewed_fq12_account_carried_signature(0);
        let fallback_envelope = encode_reviewed_fq12_account_carried_with_fallback_projection(
            &fq12_signature,
            &projection_carrier,
        )
        .expect("fallback projection envelope encodes from canonical parts");
        assert_eq!(
            fallback_envelope.len(),
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES
        );
        assert_eq!(
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES,
            314
        );
        let decoded_envelope = fallback_envelope
            .decode_reviewed_fq12_account_carried_with_fallback_projection()
            .expect("fallback projection envelope decodes");
        assert_eq!(decoded_envelope.signature, fq12_signature);
        assert_eq!(decoded_envelope.projection_carrier, projection_carrier);
        assert_eq!(decoded_envelope.decoded_carrier, decoded);
        assert!(
            decode_reviewed_fq12_account_carried_with_fallback_projection(
                &fallback_envelope.as_bytes()[..313],
            )
            .unwrap_err()
            .contains("expected 314")
        );

        let short = decode_hidden_source_fallback_projection_carrier(&carrier[..55]).unwrap_err();
        assert!(short.contains("expected 56"));

        let mut noncanonical = carrier;
        noncanonical[..2].copy_from_slice(&Q.to_le_bytes());
        let noncanonical_err =
            decode_hidden_source_fallback_projection_carrier(&noncanonical).unwrap_err();
        assert!(noncanonical_err.contains("noncanonical q12289"));
        let mut noncanonical_envelope = fallback_envelope.as_bytes().to_vec();
        noncanonical_envelope[REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES
            ..REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES + 2]
            .copy_from_slice(&Q.to_le_bytes());
        assert!(
            decode_reviewed_fq12_account_carried_with_fallback_projection(&noncanonical_envelope)
                .unwrap_err()
                .contains("noncanonical q12289")
        );

        let statement = HiddenSourceFallbackProjectionPublicBridgeStatement {
            projection_values: [0_u16; HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES],
            check_bytes: [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CHECK_BYTES],
            transcript_binding: [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES],
        };
        verify_hidden_source_fallback_projection_public_bridge(&decoded, &statement)
            .expect("matching projection carrier bridge statement accepts");

        let mut bad_lane = statement;
        bad_lane.projection_values[0] = 1;
        let bridge_lane_err =
            verify_hidden_source_fallback_projection_public_bridge(&decoded, &bad_lane)
                .unwrap_err();
        assert!(bridge_lane_err.contains("lane 0"));

        let mut bad_check = statement;
        bad_check.check_bytes[0] = 1;
        let bridge_check_err =
            verify_hidden_source_fallback_projection_public_bridge(&decoded, &bad_check)
                .unwrap_err();
        assert!(bridge_check_err.contains("check bytes"));

        let mut bad_transcript = statement;
        bad_transcript.transcript_binding[0] = 1;
        let bridge_transcript_err =
            verify_hidden_source_fallback_projection_public_bridge(&decoded, &bad_transcript)
                .unwrap_err();
        assert!(bridge_transcript_err.contains("transcript binding"));

        let mut bad_expected = statement;
        bad_expected.projection_values[0] = Q;
        let bridge_noncanonical_err =
            verify_hidden_source_fallback_projection_public_bridge(&decoded, &bad_expected)
                .unwrap_err();
        assert!(bridge_noncanonical_err.contains("noncanonical q12289"));

        let checked_core_input = HiddenSourceFallbackProjectionCheckedCoreBreakInput {
            serialized_checked_core_transcript: b"checked-core transcript".to_vec(),
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            response_difference: [0_u16; SIS_FULL_RESPONSE_RESIDUES],
            checked_core_matrix_domain: REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN.to_string(),
            checked_core_matrix_transcript_sha256: [0x33_u8; DIGEST_BYTES],
            failure: HiddenSourceFallbackProjectionCheckedCoreFailure::ResponseRelationMismatch {
                failed_equation_index: 7,
            },
        };
        let checked_core_export =
            classify_hidden_source_fallback_checked_core_break(&checked_core_input)
                .expect("checked-core failure classifier exports replay material");
        let HiddenSourceFallbackProjectionBreakExport::CheckedCoreOrResponseLink(
            checked_core_record,
        ) = checked_core_export
        else {
            panic!("expected checked-core break export");
        };
        assert_eq!(
            checked_core_record.response_difference_23_field_vector,
            [0_u16; SIS_FULL_RESPONSE_RESIDUES]
        );
        assert_eq!(checked_core_record.field_modulus_q, Q);
        assert!(
            checked_core_record
                .failed_equation_index_or_kernel_certificate_id
                .contains("failed_equation_index:7")
        );
        let checked_core_response_a = [10_u16; SIS_FULL_RESPONSE_RESIDUES];
        let mut checked_core_response_b = [10_u16; SIS_FULL_RESPONSE_RESIDUES];
        checked_core_response_b[1] = 9;
        checked_core_response_b[3] = 11;
        let checked_core_transcript_a =
            encode_hidden_source_fallback_checked_core_transcript_frame(&checked_core_response_a)
                .expect("checked-core transcript a encodes");
        let mut expected_checked_core_transcript_a = Vec::new();
        expected_checked_core_transcript_a.extend_from_slice(
            HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_DOMAIN.as_bytes(),
        );
        expected_checked_core_transcript_a.extend_from_slice(
            HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_PROFILE.as_bytes(),
        );
        for _ in 0..SIS_FULL_RESPONSE_RESIDUES {
            expected_checked_core_transcript_a.extend_from_slice(&10_u16.to_le_bytes());
        }
        assert_eq!(
            checked_core_transcript_a,
            expected_checked_core_transcript_a
        );
        let checked_core_transcript_b =
            encode_hidden_source_fallback_checked_core_transcript_frame(&checked_core_response_b)
                .expect("checked-core transcript b encodes");
        assert_eq!(
            checked_core_transcript_a.len(),
            hidden_source_fallback_checked_core_transcript_frame_bytes()
        );
        assert_eq!(
            decode_hidden_source_fallback_checked_core_transcript_frame(&checked_core_transcript_a)
                .expect("checked-core transcript a decodes"),
            checked_core_response_a
        );
        let transcript_difference =
            hidden_source_fallback_checked_core_response_difference_from_transcripts(
                &checked_core_transcript_a,
                &checked_core_transcript_b,
            )
            .expect("checked-core transcript difference derives");
        assert_eq!(transcript_difference[1], 1);
        assert_eq!(transcript_difference[3], Q - 1);
        let mut wrong_domain_transcript = checked_core_transcript_a.clone();
        wrong_domain_transcript[0] ^= 1;
        assert!(
            decode_hidden_source_fallback_checked_core_transcript_frame(&wrong_domain_transcript)
                .unwrap_err()
                .contains("domain mismatch")
        );
        let mut wrong_profile_transcript = checked_core_transcript_a.clone();
        let profile_start = HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_DOMAIN
            .as_bytes()
            .len();
        wrong_profile_transcript[profile_start] ^= 1;
        assert!(
            decode_hidden_source_fallback_checked_core_transcript_frame(&wrong_profile_transcript)
                .unwrap_err()
                .contains("profile mismatch")
        );
        let transcript_difference_input =
            HiddenSourceFallbackProjectionCheckedCoreTranscriptDifferenceInput {
                checked_core_transcript_a: checked_core_transcript_a.clone(),
                checked_core_transcript_b: checked_core_transcript_b.clone(),
                projection_carrier_bytes: carrier,
                decoded_carrier: decoded,
                checked_core_matrix_domain: REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN.to_string(),
                checked_core_matrix_transcript_sha256: [0x33_u8; DIGEST_BYTES],
                failure:
                    HiddenSourceFallbackProjectionCheckedCoreFailure::ResponseRelationMismatch {
                        failed_equation_index: 9,
                    },
            };
        let HiddenSourceFallbackProjectionBreakExport::CheckedCoreOrResponseLink(
            transcript_difference_record,
        ) = classify_hidden_source_fallback_checked_core_transcript_difference_break(
            &transcript_difference_input,
        )
        .expect("checked-core transcript difference classifier exports replay material")
        else {
            panic!("expected checked-core transcript-difference break export");
        };
        assert_eq!(
            transcript_difference_record.response_difference_23_field_vector,
            transcript_difference
        );
        assert!(
            transcript_difference_record
                .failed_equation_index_or_kernel_certificate_id
                .contains("failed_equation_index:9")
        );
        let mut bad_checked_core_transcript = checked_core_transcript_a.clone();
        let last = bad_checked_core_transcript.len() - 2;
        bad_checked_core_transcript[last..last + 2].copy_from_slice(&Q.to_le_bytes());
        assert!(
            hidden_source_fallback_checked_core_response_difference_from_transcripts(
                &bad_checked_core_transcript,
                &checked_core_transcript_b,
            )
            .unwrap_err()
            .contains("noncanonical q12289")
        );

        let hidden_input = HiddenSourceFallbackProjectionHiddenBreakInput {
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            failure: HiddenSourceFallbackProjectionHiddenFailure::OpeningProjectionMismatch {
                serialized_hidden_relation_statement: b"hidden relation statement".to_vec(),
                account_witness_opening_frame: b"account opening".to_vec(),
                pad_seed_opening_frame: b"pad opening".to_vec(),
                claimed_projection_symbols: [0_u16; HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES],
                recomputed_projection_values: [1_u16; HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES],
                mismatching_lane_or_symbol: HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS[0]
                    .to_string(),
            },
        };
        let hidden_export = classify_hidden_source_fallback_opening_break(&hidden_input)
            .expect("hidden-source opening failure classifier exports replay material");
        let HiddenSourceFallbackProjectionBreakExport::HiddenSourceOpeningBinding(hidden_record) =
            hidden_export
        else {
            panic!("expected hidden-source opening break export");
        };
        assert_eq!(
            hidden_record.mismatching_lane_or_symbol,
            HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS[0]
        );

        let account_input = HiddenSourceFallbackProjectionHiddenBreakInput {
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            failure: HiddenSourceFallbackProjectionHiddenFailure::AccountCommitmentSplitOpening {
                commitment_bytes: b"commitment".to_vec(),
                opening_frame_a: b"opening a".to_vec(),
                opening_frame_b: b"opening b".to_vec(),
                semantic_value_a: b"value a".to_vec(),
                semantic_value_b: b"value b".to_vec(),
                statement_digest: [0x44_u8; DIGEST_BYTES],
            },
        };
        assert!(matches!(
            classify_hidden_source_fallback_opening_break(&account_input)
                .expect("account split opening classifier exports replay material"),
            HiddenSourceFallbackProjectionBreakExport::AccountCommitmentBinding(_)
        ));

        let pad_input = HiddenSourceFallbackProjectionHiddenBreakInput {
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            failure:
                HiddenSourceFallbackProjectionHiddenFailure::PadPrfProjectionDerivationMismatch {
                    pad_seed_opening_frame: b"pad seed opening".to_vec(),
                    pad_statement_bytes: b"pad statement".to_vec(),
                    pad_projection_symbols: vec!["pad_seed_projection[1,0]".to_string()],
                    recomputed_pad_projection_values: vec![0],
                    sha256_pad_prf_domain: REVIEWED_FQ12_PAD_DOMAIN.to_string(),
                },
        };
        assert!(matches!(
            classify_hidden_source_fallback_opening_break(&pad_input)
                .expect("pad PRF classifier exports replay material"),
            HiddenSourceFallbackProjectionBreakExport::PadPrfOrOpening(_)
        ));

        let semantic_outcome = verify_hidden_source_fallback_checked_core_or_export(
            &test_public_key(),
            b"projection checked-core semantic predicate",
            &test_account(),
            &ProductionSignature::new(Vec::new()),
            &checked_core_input,
        );
        assert!(matches!(
            semantic_outcome,
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::CheckedCoreOrResponseLink(_)
            )
        ));

        let certificate = reviewed_response_link_bounded_kernel_certificate();
        assert_eq!(certificate.q, Q);
        assert_eq!(certificate.coords, REVIEWED_RESPONSE_LINK_COORDS);
        assert_eq!(
            certificate.manifest_payload_sha256,
            RESPONSE_LINK_BOUNDED_KERNEL_MANIFEST_PAYLOAD_SHA256
        );

        let zero_kernel_input = HiddenSourceFallbackProjectionResponseLinkKernelReplayInput {
            certificate: certificate.clone(),
            checked_core_break_input: checked_core_input.clone(),
        };
        assert!(matches!(
            verify_hidden_source_fallback_response_link_kernel_or_export(&zero_kernel_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));

        let mut nonzero_survivor_break = checked_core_input.clone();
        nonzero_survivor_break.response_difference[1] = 1;
        let nonzero_survivor_input = HiddenSourceFallbackProjectionResponseLinkKernelReplayInput {
            certificate: certificate.clone(),
            checked_core_break_input: nonzero_survivor_break,
        };
        assert!(matches!(
            verify_hidden_source_fallback_response_link_kernel_or_export(&nonzero_survivor_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::CheckedCoreOrResponseLink(_)
            )
        ));

        let mut bad_certificate = certificate;
        bad_certificate.manifest_payload_sha256 = "bad manifest hash".to_string();
        let bad_certificate_input = HiddenSourceFallbackProjectionResponseLinkKernelReplayInput {
            certificate: bad_certificate,
            checked_core_break_input: checked_core_input.clone(),
        };
        assert!(matches!(
            verify_hidden_source_fallback_response_link_kernel_or_export(&bad_certificate_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::CheckedCoreOrResponseLink(_)
            )
        ));

        let opening_projection_input = HiddenSourceFallbackProjectionOpeningProjectionReplayInput {
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            serialized_hidden_relation_statement: b"hidden relation statement".to_vec(),
            account_witness_opening_frame: b"account opening".to_vec(),
            pad_seed_opening_frame: b"pad opening".to_vec(),
            claimed_projection_symbols: [0_u16; HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES],
            recomputed_projection_values: [0_u16; HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES],
        };
        assert!(matches!(
            verify_hidden_source_fallback_opening_projection_or_export(&opening_projection_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));

        let mut opening_projection_mismatch = opening_projection_input.clone();
        opening_projection_mismatch.recomputed_projection_values[2] = 1;
        let opening_projection_outcome = verify_hidden_source_fallback_opening_projection_or_export(
            &opening_projection_mismatch,
        );
        let HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
            HiddenSourceFallbackProjectionBreakExport::HiddenSourceOpeningBinding(
                opening_projection_record,
            ),
        ) = opening_projection_outcome
        else {
            panic!("expected hidden-source opening projection export");
        };
        assert!(
            opening_projection_record
                .mismatching_lane_or_symbol
                .contains("lane:2")
        );

        let mut opening_projection_noncanonical = opening_projection_input;
        opening_projection_noncanonical.claimed_projection_symbols[1] = Q;
        assert!(matches!(
            verify_hidden_source_fallback_opening_projection_or_export(
                &opening_projection_noncanonical
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::RejectBeforeAcceptance(_)
        ));

        let account_split_input = HiddenSourceFallbackProjectionAccountCommitmentSplitReplayInput {
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            commitment_bytes: b"commitment".to_vec(),
            opening_frame_a: b"opening a".to_vec(),
            opening_frame_b: b"opening b".to_vec(),
            semantic_value_a: b"same value".to_vec(),
            semantic_value_b: b"same value".to_vec(),
            statement_digest: [0x55_u8; DIGEST_BYTES],
        };
        assert!(matches!(
            verify_hidden_source_fallback_account_commitment_split_or_export(&account_split_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));

        let mut account_split_mismatch = account_split_input.clone();
        account_split_mismatch.semantic_value_b = b"different value".to_vec();
        assert!(matches!(
            verify_hidden_source_fallback_account_commitment_split_or_export(
                &account_split_mismatch
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::AccountCommitmentBinding(_)
            )
        ));

        let mut account_split_empty_commitment = account_split_input;
        account_split_empty_commitment.commitment_bytes.clear();
        assert!(matches!(
            verify_hidden_source_fallback_account_commitment_split_or_export(
                &account_split_empty_commitment
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::RejectBeforeAcceptance(_)
        ));

        let pad_prf_input = HiddenSourceFallbackProjectionPadPrfProjectionReplayInput {
            projection_carrier_bytes: carrier,
            decoded_carrier: decoded,
            pad_seed_opening_frame: b"pad seed opening".to_vec(),
            pad_statement_bytes: b"pad statement".to_vec(),
            pad_projection_symbols: vec![
                HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS[6].to_string(),
                HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS[7].to_string(),
            ],
            recomputed_pad_projection_values: vec![0, 0],
            sha256_pad_prf_domain: REVIEWED_FQ12_PAD_DOMAIN.to_string(),
        };
        assert!(matches!(
            verify_hidden_source_fallback_pad_prf_projection_or_export(&pad_prf_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));

        let mut pad_prf_mismatch = pad_prf_input.clone();
        pad_prf_mismatch.recomputed_pad_projection_values[1] = 1;
        assert!(matches!(
            verify_hidden_source_fallback_pad_prf_projection_or_export(&pad_prf_mismatch),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::PadPrfOrOpening(_)
            )
        ));

        let mut pad_prf_unknown_symbol = pad_prf_input;
        pad_prf_unknown_symbol.pad_projection_symbols[0] =
            "pad_seed_projection[unknown]".to_string();
        assert!(matches!(
            verify_hidden_source_fallback_pad_prf_projection_or_export(&pad_prf_unknown_symbol),
            HiddenSourceFallbackProjectionSemanticPredicateResult::RejectBeforeAcceptance(_)
        ));

        let seed_pad = [0x42_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
        assert_eq!(REVIEWED_FQ12_PAD_SEED_BYTES, 32);
        assert_eq!(REVIEWED_FQ12_PAD_PRF_COUNTER_LIMIT, 16);
        assert_eq!(seed_pad.len(), REVIEWED_FQ12_PAD_SEED_BYTES);
        assert_eq!(
            reviewed_fq12_pad_residues_from_seed_bounded(&seed_pad, 1)
                .expect("bounded seed block 1 derives residues"),
            [2674, 9115]
        );
        assert_eq!(
            reviewed_fq12_pad_residues_from_seed_bounded(&seed_pad, 2)
                .expect("bounded seed block 2 derives residues"),
            [1311, 6011]
        );
        assert_eq!(
            reviewed_fq12_pad_residues_from_seed_bounded(&seed_pad, 6)
                .expect("bounded seed block 6 derives residues"),
            [4034, 7068]
        );
        assert_eq!(
            reviewed_fq12_pad_residues_from_seed_bounded(&seed_pad, 7)
                .expect("bounded seed block 7 derives residues"),
            [6329, 9937]
        );
        assert_eq!(
            reviewed_fq12_pad_residues_from_seed_bounded(&seed_pad, 11)
                .expect("bounded seed block 11 derives residues"),
            [6870, 8128]
        );
        assert_eq!(
            reviewed_fq12_pad_residues_from_seed_bounded(&seed_pad, 12)
                .expect("bounded seed block 12 derives residues"),
            [9169, 11132]
        );
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account witness opening frame encodes");
        let mut expected_account_opening_frame = Vec::new();
        expected_account_opening_frame
            .extend_from_slice(HIDDEN_SOURCE_FALLBACK_ACCOUNT_OPENING_FRAME_DOMAIN.as_bytes());
        expected_account_opening_frame
            .extend_from_slice(HIDDEN_SOURCE_FALLBACK_ACCOUNT_OPENING_FRAME_PROFILE.as_bytes());
        for value in account_opening.selected_source_values {
            expected_account_opening_frame.extend_from_slice(&value.to_le_bytes());
        }
        assert_eq!(account_opening_frame, expected_account_opening_frame);
        assert_eq!(
            account_opening_frame.len(),
            hidden_source_fallback_account_witness_opening_frame_bytes()
        );
        assert_eq!(
            decode_hidden_source_fallback_account_witness_opening_frame(&account_opening_frame)
                .expect("account witness opening frame decodes"),
            account_opening
        );
        let mut bad_account_opening_frame = account_opening_frame.clone();
        let last = bad_account_opening_frame.len() - 2;
        bad_account_opening_frame[last..last + 2].copy_from_slice(&Q.to_le_bytes());
        assert!(
            decode_hidden_source_fallback_account_witness_opening_frame(&bad_account_opening_frame)
                .unwrap_err()
                .contains("noncanonical q12289")
        );
        let (account_symbols, account_values) =
            hidden_source_fallback_account_projection_values_from_opening(&account_opening)
                .expect("test account opening derives projection values");
        assert_eq!(account_symbols.len(), 6);
        assert_eq!(account_values, [6234, 4398, 6467, 5952, 10565, 4938]);
        let expanded_opening_commitment =
            hidden_source_fallback_expanded_account_opening_commitment(&account_opening)
                .expect("expanded opening commitment is canonical");
        let expanded_opening_input =
            HiddenSourceFallbackProjectionExpandedOpeningCommitmentReplayInput {
                expected_expanded_opening_commitment: expanded_opening_commitment,
                account_witness_opening_frame: b"account witness opening".to_vec(),
                account_witness_opening: account_opening,
            };
        assert!(matches!(
            verify_hidden_source_fallback_expanded_opening_commitment(&expanded_opening_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));
        let expanded_opening_frame_input =
            HiddenSourceFallbackProjectionExpandedOpeningFrameReplayInput {
                expected_expanded_opening_commitment: expanded_opening_commitment,
                account_witness_opening_frame: account_opening_frame.clone(),
            };
        assert!(matches!(
            verify_hidden_source_fallback_expanded_opening_frame_commitment(
                &expanded_opening_frame_input
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));
        let mut bad_expanded_opening_input = expanded_opening_input;
        bad_expanded_opening_input
            .account_witness_opening
            .selected_source_values[0] ^= 1;
        assert!(matches!(
            verify_hidden_source_fallback_expanded_opening_commitment(&bad_expanded_opening_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::RejectBeforeAcceptance(_)
        ));
        let projection_slot = ProductionReviewedFq12StateAssistedAccountSlot::new(
            0,
            ProductionAccountCommitment::new(canonical_digest(0), canonical_digest(1)),
            expanded_opening_commitment,
            canonical_digest(2),
            canonical_digest(3),
        );
        assert_eq!(
            verify_hidden_source_fallback_expanded_opening_state_slot(
                &projection_slot,
                b"account witness opening",
                &account_opening,
            )
            .expect("projection expanded opening matches stored slot commitment"),
            expanded_opening_commitment
        );
        assert_eq!(
            verify_hidden_source_fallback_expanded_opening_state_slot_frame(
                &projection_slot,
                &account_opening_frame,
            )
            .expect("projection expanded opening frame matches stored slot commitment"),
            expanded_opening_commitment
        );
        let projection_state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("projection account state persists expanded opening commitment at creation");
        let projection_state_slot = projection_state
            .get_slot(0)
            .expect("projection slot exists");
        assert_eq!(
            projection_state_slot.hidden_source_opening_commitment(),
            expanded_opening_commitment
        );
        let projection_records = projection_state
            .checked_slot_records()
            .expect("projection state records validate");
        assert_eq!(projection_records.len(), 1);
        assert_eq!(
            projection_records[0].hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        let projection_record_json =
            serde_json::to_string(&projection_records[0]).expect("projection record serializes");
        assert!(projection_record_json.contains("hidden_source_opening_commitment"));
        assert!(projection_record_json.contains("restore_anchor_digest"));
        assert!(projection_record_json.contains("pad_seed_digest"));
        let restored_projection_state =
            reviewed_fq12_state_assisted_account_state_from_slot_records(projection_records)
                .expect("projection persisted records restore");
        assert_eq!(
            restored_projection_state
                .get_slot(0)
                .expect("restored projection slot exists")
                .hidden_source_opening_commitment(),
            expanded_opening_commitment
        );
        let wrong_projection_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [2, 2, 3, 4, 5, 6],
        };
        let wrong_projection_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&wrong_projection_opening)
                .expect("wrong account witness opening frame encodes");
        assert_eq!(
            verify_hidden_source_fallback_expanded_opening_state_slot(
                &projection_slot,
                b"account witness opening",
                &wrong_projection_opening,
            )
            .unwrap_err(),
            "hidden-source fallback expanded account opening commitment mismatch"
        );
        assert_eq!(
            verify_hidden_source_fallback_expanded_opening_state_slot_frame(
                &projection_slot,
                &wrong_projection_opening_frame,
            )
            .unwrap_err(),
            "hidden-source fallback expanded account opening commitment mismatch"
        );
        let mut bad_creation_frame = account_opening_frame.clone();
        bad_creation_frame[0] ^= 1;
        assert!(
            hidden_source_fallback_projection_account_state_from_slot_frames([
                HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                    witness_commitment: canonical_digest(0),
                    randomizer_commitment: canonical_digest(1),
                    account_witness_opening_frame: bad_creation_frame,
                    restore_anchor_digest: canonical_digest(2),
                    pad_seed_digest: canonical_digest(3),
                    unused: true,
                },
            ])
            .unwrap_err()
            .contains("domain mismatch")
        );
        let account_split_frame_input =
            HiddenSourceFallbackProjectionAccountCommitmentSplitFrameReplayInput {
                projection_carrier_bytes: carrier,
                decoded_carrier: decoded,
                commitment_bytes: b"commitment".to_vec(),
                opening_frame_a: account_opening_frame.clone(),
                opening_frame_b: account_opening_frame.clone(),
                statement_digest: [0x56_u8; DIGEST_BYTES],
            };
        assert!(matches!(
            verify_hidden_source_fallback_account_commitment_split_frames_or_export(
                &account_split_frame_input
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));
        let mut account_split_frame_mismatch = account_split_frame_input.clone();
        account_split_frame_mismatch.opening_frame_b = wrong_projection_opening_frame.clone();
        assert!(matches!(
            verify_hidden_source_fallback_account_commitment_split_frames_or_export(
                &account_split_frame_mismatch
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::AccountCommitmentBinding(_)
            )
        ));
        let (seed_pad_symbols, seed_pad_values) =
            hidden_source_fallback_pad_prf_projection_values_from_seed(&seed_pad)
                .expect("test pad seed derives projection values");
        assert_eq!(seed_pad_symbols.len(), 6);
        assert_eq!(seed_pad_values, [12169, 2446, 4817, 3574, 5914, 8556]);
        let mut seed_carrier = carrier;
        for (symbol, value) in seed_pad_symbols.iter().zip(seed_pad_values.iter()) {
            let lane = HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS
                .iter()
                .position(|candidate| candidate == symbol)
                .expect("derived pad symbol is selected");
            seed_carrier[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        let seed_decoded = decode_hidden_source_fallback_projection_carrier(&seed_carrier)
            .expect("derived seed carrier is canonical");
        let pad_seed_input = HiddenSourceFallbackProjectionPadPrfSeedReplayInput {
            projection_carrier_bytes: seed_carrier,
            decoded_carrier: seed_decoded,
            pad_seed: seed_pad,
            pad_seed_opening_frame: b"pad seed opening".to_vec(),
            pad_statement_bytes: b"pad statement".to_vec(),
            sha256_pad_prf_domain: REVIEWED_FQ12_PAD_DOMAIN.to_string(),
        };
        assert!(matches!(
            verify_hidden_source_fallback_pad_prf_seed_or_export(&pad_seed_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));

        let mut bad_pad_seed_input = pad_seed_input;
        bad_pad_seed_input.pad_seed[0] ^= 1;
        assert!(matches!(
            verify_hidden_source_fallback_pad_prf_seed_or_export(&bad_pad_seed_input),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::PadPrfOrOpening(_)
            )
        ));

        let full_projection = hidden_source_fallback_projection_values_from_hidden_material(
            &account_opening,
            &seed_pad,
        )
        .expect("test hidden material fills all projection lanes");
        assert_eq!(&full_projection[..6], account_values.as_slice());
        assert_eq!(&full_projection[6..], seed_pad_values.as_slice());
        let mut full_carrier = carrier;
        for (lane, value) in full_projection.iter().enumerate() {
            full_carrier[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        let full_decoded = decode_hidden_source_fallback_projection_carrier(&full_carrier)
            .expect("full hidden-material carrier is canonical");
        let hidden_material_input = HiddenSourceFallbackProjectionHiddenMaterialReplayInput {
            projection_carrier_bytes: full_carrier,
            decoded_carrier: full_decoded,
            serialized_hidden_relation_statement: b"hidden relation statement".to_vec(),
            account_witness_opening_frame: account_opening_frame.clone(),
            account_witness_opening: account_opening,
            pad_seed_opening_frame: b"pad seed opening".to_vec(),
            pad_seed: seed_pad,
        };
        assert!(matches!(
            verify_hidden_source_fallback_hidden_material_projection_or_export(
                &hidden_material_input
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Accept(())
        ));
        let mut bad_hidden_material_input = hidden_material_input;
        bad_hidden_material_input
            .account_witness_opening
            .selected_source_values[0] ^= 1;
        assert!(matches!(
            verify_hidden_source_fallback_hidden_material_projection_or_export(
                &bad_hidden_material_input
            ),
            HiddenSourceFallbackProjectionSemanticPredicateResult::Export(
                HiddenSourceFallbackProjectionBreakExport::HiddenSourceOpeningBinding(_)
            )
        ));
        assert_eq!(
            HiddenSourceFallbackProjectionVerifierState::InternalErrorFailClosed,
            HiddenSourceFallbackProjectionVerifierState::InternalErrorFailClosed
        );
    }

    #[test]
    fn selected_projection_lifecycle_store_persists_opening_commitment_and_anchor() {
        let account_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: [1, 2, 3, 4, 5, 6],
        };
        let account_opening_frame =
            encode_hidden_source_fallback_account_witness_opening_frame(&account_opening)
                .expect("account opening frame encodes");
        let expanded_opening_commitment =
            hidden_source_fallback_expanded_account_opening_commitment(&account_opening)
                .expect("expanded account opening commitment hashes");
        let state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("projection account state is created from canonical frame");
        let mut manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(state)
            .expect("state-assisted lifecycle manager accepts projection state");
        let reservation = manager.reserve_slot(0).expect("slot reserves");
        assert_eq!(manager.snapshot_sequence(), 1);
        assert_eq!(manager.active_reservation_count(), 1);
        manager
            .burn_reserved_slot(reservation)
            .expect("reserved projection slot burns on recovery/reorg policy");
        assert_eq!(manager.snapshot_sequence(), 2);

        let path = temp_json_path("descender-selected-projection-lifecycle");
        let store = ProductionReviewedFq12StateAssistedLifecycleFileStore::new(&path);
        store
            .save_manager(&manager)
            .expect("projection manager stores");
        let serialized = std::fs::read_to_string(&path).expect("projection snapshot reads");
        assert!(
            serialized.contains(PRODUCTION_REVIEWED_FQ12_STATE_ASSISTED_LIFECYCLE_STORE_SCHEMA)
        );
        assert!(serialized.contains("hidden_source_opening_commitment"));
        assert!(serialized.contains("restore_anchor_digest"));
        assert!(serialized.contains("pad_seed_digest"));
        assert!(!serialized.contains("account_witness_opening_frame"));
        assert_serialized_public_state_excludes_secret_sentinels(&serialized);

        let restored = store
            .load_manager()
            .expect("projection manager loads")
            .unwrap();
        let restored_slot = restored
            .account()
            .get_slot(0)
            .expect("restored projection slot exists");
        assert_eq!(
            restored_slot.hidden_source_opening_commitment(),
            expanded_opening_commitment
        );
        assert!(!restored_slot.is_unused());
        assert!(
            store
                .load_manager_with_restore_anchor(ProductionLifecycleRestoreAnchor {
                    minimum_snapshot_sequence: 2,
                })
                .expect("current projection snapshot satisfies trusted restore anchor")
                .is_some()
        );
        assert_eq!(
            store
                .load_manager_with_restore_anchor(ProductionLifecycleRestoreAnchor {
                    minimum_snapshot_sequence: 3,
                })
                .unwrap_err()
                .reason(),
            "snapshot sequence is older than restore anchor"
        );

        let fresh_state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("fresh projection state is created from canonical frame");
        let mut fresh_manager =
            ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(fresh_state)
                .expect("fresh projection lifecycle manager accepts projection state");
        assert_eq!(
            fresh_manager
                .verify_expanded_opening_frame_and_consume(0, &account_opening_frame)
                .expect("matching opening frame consumes projection slot"),
            expanded_opening_commitment
        );
        assert!(!fresh_manager.account().get_slot(0).unwrap().is_unused());
        assert_eq!(
            fresh_manager
                .verify_expanded_opening_frame_and_consume(0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "hidden-source fallback projection slot is already consumed"
        );

        let consensus_state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("consensus projection state is created from canonical frame");
        let consensus_manager =
            ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(consensus_state)
                .expect("consensus projection lifecycle manager accepts projection state");
        let public_key = test_public_key();
        let transition = apply_selected_projection_consensus_opening_frame(
            consensus_manager
                .snapshot()
                .expect("consensus projection snapshot exports"),
            Vec::new(),
            ProductionLifecycleRestoreAnchor {
                minimum_snapshot_sequence: 0,
            },
            &public_key,
            0,
            &account_opening_frame,
        )
        .expect("consensus transition consumes and records selected projection slot");
        assert_eq!(
            transition.spend_record.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(transition.spend_record.pad_seed_digest, canonical_digest(3));
        assert_eq!(
            transition.spend_record.spent_record.pad_seed_digest,
            Some(canonical_digest(3))
        );
        assert_eq!(transition.spend_record.snapshot_sequence, 1);
        assert_eq!(transition.next_spent_slot_records.len(), 1);
        assert_eq!(
            transition.next_spent_slot_records[0].pad_seed_digest,
            Some(canonical_digest(3))
        );
        assert!(!transition.next_lifecycle_snapshot.account_slots[0].unused);
        let mut duplicate_pad_transition = transition.clone();
        let mut duplicate_pad_record = duplicate_pad_transition.next_spent_slot_records[0].clone();
        duplicate_pad_record.key.slot_id = 1;
        duplicate_pad_transition
            .next_spent_slot_records
            .push(duplicate_pad_record);
        let mut malformed_db = ProductionInMemorySelectedProjectionTargetChainDb::new();
        assert_eq!(
            malformed_db
                .commit_selected_projection_transition(
                    &public_key,
                    duplicate_pad_transition,
                    ProductionTrustedRestoreAnchorRecord::new(
                        &public_key,
                        1,
                        PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
                    )
                    .expect("target-chain trusted anchor placement is approved"),
                )
                .unwrap_err()
                .reason(),
            "pad seed digest is already in spent ledger"
        );
        assert_eq!(
            apply_selected_projection_consensus_opening_frame(
                transition.next_lifecycle_snapshot.clone(),
                transition.next_spent_slot_records.clone(),
                ProductionLifecycleRestoreAnchor {
                    minimum_snapshot_sequence: 1,
                },
                &public_key,
                0,
                &account_opening_frame,
            )
            .unwrap_err()
            .reason(),
            "slot is already in spent ledger"
        );
        assert_eq!(
            apply_selected_projection_consensus_opening_frame(
                transition.next_lifecycle_snapshot,
                transition.next_spent_slot_records,
                ProductionLifecycleRestoreAnchor {
                    minimum_snapshot_sequence: 2,
                },
                &public_key,
                0,
                &account_opening_frame,
            )
            .unwrap_err()
            .reason(),
            "snapshot sequence is older than restore anchor"
        );

        let lifecycle_path = temp_json_path("descender-selected-projection-chain-lifecycle");
        let ledger_path = temp_json_path("descender-selected-projection-chain-ledger");
        let anchor_path = temp_json_path("descender-selected-projection-chain-anchor");
        let chain_stores = ProductionSelectedProjectionChainStateFileStores::new(
            &lifecycle_path,
            &ledger_path,
            &anchor_path,
        );
        let anchor_lock = chain_stores
            .trusted_anchor_store()
            .try_lock()
            .expect("trusted anchor store lock can be acquired");
        assert_eq!(
            chain_stores
                .trusted_anchor_store()
                .try_lock()
                .unwrap_err()
                .reason(),
            "trusted restore anchor store is locked"
        );
        drop(anchor_lock);
        let stored_state = hidden_source_fallback_projection_account_state_from_slot_frames([
            HiddenSourceFallbackProjectionAccountSlotFrameSeed {
                witness_commitment: canonical_digest(0),
                randomizer_commitment: canonical_digest(1),
                account_witness_opening_frame: account_opening_frame.clone(),
                restore_anchor_digest: canonical_digest(2),
                pad_seed_digest: canonical_digest(3),
                unused: true,
            },
        ])
        .expect("stored projection state is created from canonical frame");
        let stored_manager =
            ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(stored_state)
                .expect("stored projection lifecycle manager accepts projection state");
        chain_stores
            .lifecycle_store()
            .save_manager(&stored_manager)
            .expect("selected projection lifecycle snapshot stores");
        assert_eq!(
            chain_stores
                .apply_opening_frame_and_save(&public_key, 0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "missing trusted restore anchor record"
        );
        chain_stores
            .trusted_anchor_store()
            .update_monotonic(
                ProductionTrustedRestoreAnchorRecord::new(
                    &public_key,
                    0,
                    "chain_observed_finalized_or_burned_state",
                )
                .expect("trusted anchor placement is approved"),
            )
            .expect("trusted anchor stores separately");
        let saved_spend = chain_stores
            .apply_opening_frame_and_save(&public_key, 0, &account_opening_frame)
            .expect("selected projection chain stores apply consensus transition");
        assert_eq!(
            saved_spend.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(
            chain_stores
                .trusted_anchor_store()
                .require_anchor(&public_key)
                .expect("trusted anchor loads")
                .minimum_snapshot_sequence,
            1
        );
        assert_eq!(
            chain_stores
                .apply_opening_frame_and_save(&public_key, 0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "slot is already in spent ledger"
        );
        assert_eq!(
            ProductionTrustedRestoreAnchorRecord::new(&public_key, 0, "account_state_backup")
                .unwrap_err()
                .reason(),
            "trusted restore anchor placement is not deployment-approved"
        );

        let mut target_db = ProductionInMemorySelectedProjectionTargetChainDb::new();
        target_db.set_lifecycle_snapshot(
            &public_key,
            stored_manager
                .snapshot()
                .expect("selected projection snapshot is serializable"),
        );
        target_db
            .update_trusted_anchor(
                ProductionTrustedRestoreAnchorRecord::new(
                    &public_key,
                    0,
                    PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
                )
                .expect("target-chain trusted anchor placement is approved"),
            )
            .expect("target-chain trusted anchor stores");
        let mut target_adapter = ProductionSelectedProjectionTargetChainDbAdapter::new(target_db);
        let target_spend = target_adapter
            .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
            .expect("target-chain adapter applies selected projection transition");
        assert_eq!(
            target_spend.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(target_spend.pad_seed_digest, canonical_digest(3));
        assert_eq!(
            target_adapter.db().spent_slot_records(&public_key)[0].pad_seed_digest,
            Some(canonical_digest(3))
        );
        assert_eq!(
            target_adapter
                .db()
                .trusted_anchor_record(&public_key)
                .expect("target-chain trusted anchor committed")
                .minimum_snapshot_sequence,
            1
        );
        assert_eq!(target_adapter.db().spent_slot_records(&public_key).len(), 1);
        assert_eq!(
            target_adapter
                .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "slot is already in spent ledger"
        );

        let mut failing_db = ProductionInMemorySelectedProjectionTargetChainDb::new();
        failing_db.set_lifecycle_snapshot(
            &public_key,
            stored_manager
                .snapshot()
                .expect("selected projection snapshot is serializable"),
        );
        failing_db
            .update_trusted_anchor(
                ProductionTrustedRestoreAnchorRecord::new(
                    &public_key,
                    0,
                    PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
                )
                .expect("target-chain trusted anchor placement is approved"),
            )
            .expect("target-chain trusted anchor stores");
        failing_db.fail_next_commit();
        let mut failing_adapter = ProductionSelectedProjectionTargetChainDbAdapter::new(failing_db);
        assert_eq!(
            failing_adapter
                .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "target-chain commit failed"
        );
        assert_eq!(
            failing_adapter
                .db()
                .lifecycle_snapshot(&public_key)
                .expect("failed target-chain commit keeps old lifecycle")
                .snapshot_sequence,
            0
        );
        assert!(
            failing_adapter
                .db()
                .spent_slot_records(&public_key)
                .is_empty()
        );
        assert_eq!(
            failing_adapter
                .db()
                .trusted_anchor_record(&public_key)
                .expect("failed target-chain commit keeps old anchor")
                .minimum_snapshot_sequence,
            0
        );

        let mut kv_db = ProductionSelectedProjectionTargetChainKvDb::new(
            ProductionInMemoryTransactionalKv::new(),
        );
        kv_db
            .initialize_account_state(
                &public_key,
                stored_manager
                    .snapshot()
                    .expect("selected projection snapshot is serializable"),
                ProductionTrustedRestoreAnchorRecord::new(
                    &public_key,
                    0,
                    PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
                )
                .expect("target-chain trusted anchor placement is approved"),
            )
            .expect("target-chain kv account state initializes atomically");
        assert!(
            kv_db
                .kv()
                .raw_get(&ProductionSelectedProjectionTargetChainKvDb::<
                    ProductionInMemoryTransactionalKv,
                >::lifecycle_key(&public_key))
                .is_some()
        );
        let mut kv_adapter = ProductionSelectedProjectionTargetChainDbAdapter::new(kv_db);
        let kv_spend = kv_adapter
            .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
            .expect("target-chain kv adapter applies selected projection transition");
        assert_eq!(
            kv_spend.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(kv_spend.pad_seed_digest, canonical_digest(3));
        assert_eq!(
            kv_adapter
                .db()
                .load_selected_projection_spent_slot_records(&public_key)
                .expect("target-chain kv spent records load")[0]
                .pad_seed_digest,
            Some(canonical_digest(3))
        );
        assert_eq!(
            kv_adapter
                .db()
                .load_selected_projection_trusted_anchor(&public_key)
                .expect("target-chain kv trusted anchor loads")
                .expect("target-chain kv trusted anchor is present")
                .minimum_snapshot_sequence,
            1
        );
        assert_eq!(
            kv_adapter
                .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "slot is already in spent ledger"
        );

        let mut failing_kv_db = ProductionSelectedProjectionTargetChainKvDb::new(
            ProductionInMemoryTransactionalKv::new(),
        );
        failing_kv_db
            .initialize_account_state(
                &public_key,
                stored_manager
                    .snapshot()
                    .expect("selected projection snapshot is serializable"),
                ProductionTrustedRestoreAnchorRecord::new(
                    &public_key,
                    0,
                    PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
                )
                .expect("target-chain trusted anchor placement is approved"),
            )
            .expect("target-chain kv account state initializes atomically");
        failing_kv_db.kv_mut().fail_next_commit();
        let mut failing_kv_adapter =
            ProductionSelectedProjectionTargetChainDbAdapter::new(failing_kv_db);
        assert_eq!(
            failing_kv_adapter
                .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
                .unwrap_err()
                .reason(),
            "target-chain kv transaction failed"
        );
        assert_eq!(
            failing_kv_adapter
                .db()
                .load_selected_projection_lifecycle_snapshot(&public_key)
                .expect("failed target-chain kv commit keeps old lifecycle")
                .expect("target-chain kv lifecycle is present")
                .snapshot_sequence,
            0
        );
        assert!(
            failing_kv_adapter
                .db()
                .load_selected_projection_spent_slot_records(&public_key)
                .expect("failed target-chain kv commit keeps old spent slots")
                .is_empty()
        );
        assert_eq!(
            failing_kv_adapter
                .db()
                .load_selected_projection_trusted_anchor(&public_key)
                .expect("failed target-chain kv commit keeps old anchor")
                .expect("target-chain kv anchor is present")
                .minimum_snapshot_sequence,
            0
        );

        let file_kv_path = temp_json_path("descender-target-chain-kv");
        let mut file_kv_db = ProductionSelectedProjectionTargetChainKvDb::new(
            ProductionFileBackedTransactionalKv::new(&file_kv_path),
        );
        file_kv_db
            .initialize_account_state(
                &public_key,
                stored_manager
                    .snapshot()
                    .expect("selected projection snapshot is serializable"),
                ProductionTrustedRestoreAnchorRecord::new(
                    &public_key,
                    0,
                    PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
                )
                .expect("file-backed target-chain trusted anchor placement is approved"),
            )
            .expect("file-backed target-chain kv initializes atomically");
        let reloaded_file_kv_db = ProductionSelectedProjectionTargetChainKvDb::new(
            ProductionFileBackedTransactionalKv::new(&file_kv_path),
        );
        assert_eq!(
            reloaded_file_kv_db
                .load_selected_projection_lifecycle_snapshot(&public_key)
                .expect("file-backed target-chain lifecycle reloads")
                .expect("file-backed target-chain lifecycle is present")
                .snapshot_sequence,
            0
        );
        let mut file_kv_adapter =
            ProductionSelectedProjectionTargetChainDbAdapter::new(reloaded_file_kv_db);
        let file_kv_spend = file_kv_adapter
            .apply_opening_frame_and_commit(&public_key, 0, &account_opening_frame)
            .expect("file-backed target-chain kv adapter applies transition");
        assert_eq!(
            file_kv_spend.hidden_source_opening_commitment,
            expanded_opening_commitment
        );
        assert_eq!(file_kv_spend.pad_seed_digest, canonical_digest(3));
        let post_commit_file_kv_db = ProductionSelectedProjectionTargetChainKvDb::new(
            ProductionFileBackedTransactionalKv::new(&file_kv_path),
        );
        assert_eq!(
            post_commit_file_kv_db
                .load_selected_projection_trusted_anchor(&public_key)
                .expect("file-backed target-chain trusted anchor reloads")
                .expect("file-backed target-chain trusted anchor is present")
                .minimum_snapshot_sequence,
            1
        );
        assert_eq!(
            post_commit_file_kv_db
                .load_selected_projection_spent_slot_records(&public_key)
                .expect("file-backed target-chain spent slots reload")
                .len(),
            1
        );
        let file_kv_lock = post_commit_file_kv_db
            .kv()
            .try_lock()
            .expect("file-backed target-chain kv lock can be acquired");
        assert_eq!(
            post_commit_file_kv_db.kv().try_lock().unwrap_err().reason(),
            "file-backed target-chain kv store is locked"
        );
        drop(file_kv_lock);
        let _ = std::fs::write(post_commit_file_kv_db.kv().tmp_path(), b"not committed");
        let reloaded_after_tmp = ProductionSelectedProjectionTargetChainKvDb::new(
            ProductionFileBackedTransactionalKv::new(&file_kv_path),
        );
        assert_eq!(
            reloaded_after_tmp
                .load_selected_projection_trusted_anchor(&public_key)
                .expect("file-backed target-chain trusted anchor ignores temp file")
                .expect("file-backed target-chain trusted anchor is present")
                .minimum_snapshot_sequence,
            1
        );

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(store.lock_path());
        let _ = std::fs::remove_file(lifecycle_path);
        let _ = std::fs::remove_file(ledger_path);
        let _ = std::fs::remove_file(anchor_path);
        let _ = std::fs::remove_file(file_kv_path);
        let _ = std::fs::remove_file(post_commit_file_kv_db.kv().lock_path());
        let _ = std::fs::remove_file(post_commit_file_kv_db.kv().tmp_path());
        let _ = std::fs::remove_file(chain_stores.lifecycle_store().lock_path());
        let _ = std::fs::remove_file(chain_stores.spent_ledger_store().lock_path());
        let _ = std::fs::remove_file(chain_stores.trusted_anchor_store().lock_path());
    }

    #[test]
    fn facade_sign_uses_sanitized_secret_key_boundary() {
        let account = test_account();
        let secret = ProductionSecretKey::new(());
        let slot = account.get_slot(0).unwrap();
        let signature = deployment_sign_with_secret_key(
            &secret,
            slot,
            |_secret, slot_id, witness_commitment| {
                assert_eq!(slot_id, 0);
                assert_eq!(witness_commitment, canonical_digest(0));
                Ok(test_signature(slot_id))
            },
        )
        .expect("facade signer accepts canonical callback output");
        assert_eq!(
            signature.len(),
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );

        let err =
            deployment_sign_with_secret_key(&secret, slot, |_secret, _slot_id, _commitment| {
                Err("secret callback detail".to_string())
            })
            .unwrap_err();
        assert_eq!(err, PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON);
    }

    #[test]
    fn facade_verify_uses_sanitized_boundary_and_consumes_only_on_accept() {
        let public_key = test_public_key();
        let account = test_account();
        let signature = test_signature(0);
        let verification = deployment_verify_with_account_binding(
            &public_key,
            b"message",
            &account,
            &signature,
            |_public_key, message, _account, _signature, checked| {
                assert_eq!(message, b"message");
                assert_eq!(checked.slot_id(), 0);
                Ok(true)
            },
            |accepted| *accepted,
        )
        .expect("accepted verifier report consumes account slot");
        assert!(*verification.report());
        assert!(!verification.next_account().get_slot(0).unwrap().is_unused());

        let rejection = deployment_verify_with_account_binding(
            &public_key,
            b"message",
            &account,
            &signature,
            |_public_key, _message, _account, _signature, _checked| {
                Err("secret verifier detail".to_string())
            },
            |_accepted: &bool| true,
        )
        .unwrap_err();
        assert_eq!(
            rejection.reason(),
            PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON
        );
    }

    #[test]
    fn facade_reserved_sign_consumes_reserved_slot() {
        let mut manager =
            ProductionAccountLifecycleManager::new(test_account()).expect("manager initializes");
        let reservation = manager.reserve_slot(0).expect("slot reserves");
        let signature = deployment_sign_reserved_and_consume(
            &mut manager,
            reservation,
            |slot_id, _commitment| Ok(test_signature(slot_id)),
        )
        .expect("reserved sign consumes");
        assert_eq!(signature.decode().unwrap().slot_id, 0);
        assert!(!manager.account().get_slot(0).unwrap().is_unused());
    }
}
