use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::hint::black_box;
use std::time::Instant;

use descender_parameter_search::production_reference::{
    self as prodref, ProductionAccountCommitment, ProductionAccountSlot, ProductionAccountSlotSeed,
    ProductionAccountState, ProductionKeypair, ProductionPublicKey, ProductionSecretKey,
    ProductionSignature, ProductionVerificationRejection, REVIEWED_ACCOUNT_LINK_BYTES,
    REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES,
};

const PROFILE: &str = "Descender.Signature.q12289.local_delta.stateful.v1";
const CHALLENGE_HASH_DOMAIN: &str = "Descender.Signature.q12289.local_delta.challenge.v1";
const SIS_MATRIX_DOMAIN: &str = "Descender.Signature.q12289.local_delta.sis_matrix.v1";
const Q: u16 = 12_289;
const GROUPS: usize = 16;
const LOCAL_PIVOT: usize = 3;
const LOCAL_FREE: usize = 2;
const VIEW_COORDS: usize = GROUPS * (LOCAL_PIVOT + LOCAL_FREE);
const PIVOT_COORDS: usize = GROUPS * LOCAL_PIVOT;
const PK_SEED_BYTES: usize = 32;
const PK_PAYLOAD_BYTES: usize = 140;
const PK_BYTES: usize = PK_SEED_BYTES + PK_PAYLOAD_BYTES;
const FIELD_BITS: usize = 14;
const REPS: usize = 26;
const SLOT_ID_BYTES: usize = 4;
const ALGEBRAIC_CHALLENGE_BYTES: usize = prodref::ALGEBRAIC_CHALLENGE_BYTES;
const CHALLENGE_HARDENING_BYTES: usize = prodref::CHALLENGE_HARDENING_BYTES;
const CHALLENGE_BYTES: usize = ALGEBRAIC_CHALLENGE_BYTES + CHALLENGE_HARDENING_BYTES;
const CHALLENGE_BITS_PER_REP: usize = 5;
const ALGEBRAIC_CHALLENGE_ENTROPY_BITS: usize = REPS * CHALLENGE_BITS_PER_REP;
const CHALLENGE_ENTROPY_BITS: usize =
    ALGEBRAIC_CHALLENGE_ENTROPY_BITS + 8 * CHALLENGE_HARDENING_BYTES;
const FS_CLASSICAL_QUERY_BITS_FOR_128: usize = CHALLENGE_ENTROPY_BITS - 128;
const FS_QUANTUM_QUERY_BITS_FOR_128: usize = (CHALLENGE_ENTROPY_BITS - 128) / 2;
const ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK: u8 = prodref::ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK;
const CHALLENGE_ACTION_RESIDUES: usize = SIS_FULL_RESPONSE_PUBLIC_ROWS;
const CHALLENGE_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH: [u16; 13] =
    [25, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const RESPONSE_BOUND: i16 = 48;
const ACTIVE_RESPONSE_BYTES: usize = 52;
const LOCAL_OPENING_BYTES_PER_REP: usize = 4;
const LOCAL_OPENING_BYTES: usize = REPS * LOCAL_OPENING_BYTES_PER_REP;
const DIGEST_BYTES: usize = 32;
const FIBER_STATE_BYTES: usize = 2;
const PAD_SEED_BYTES: usize = 32;
const HIDDEN_PREIMAGE_BYTES: usize = FIBER_STATE_BYTES + PAD_SEED_BYTES;
const COMMITMENT_DOMAIN: &[u8] = b"Descender.Signature.q12289.local_delta.v1";
const PAD_DOMAIN: &[u8] = b"Descender.Signature.q12289.local_delta.pad.v1";
const PAD_FRAME_BYTES: usize = 46;
const STATEFUL_SPEND_SIGNATURE_BYTES: usize =
    CHALLENGE_BYTES + ACTIVE_RESPONSE_BYTES + REPS * LOCAL_OPENING_BYTES_PER_REP;
const STATEFUL_TRANSACTION_SIGNATURE_BYTES: usize = SLOT_ID_BYTES + STATEFUL_SPEND_SIGNATURE_BYTES;
const STATELESS_SHA256_SIGNATURE_BYTES: usize =
    STATEFUL_SPEND_SIGNATURE_BYTES + REPS * DIGEST_BYTES;
const REVEALED_PREIMAGE_PROOF_BYTES: usize = LINEAR_SIGMA_MASKED_WITNESS_BYTES;
const REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES: usize =
    STATEFUL_TRANSACTION_SIGNATURE_BYTES + REVEALED_PREIMAGE_PROOF_BYTES;
const PROOF_BYTES_UNDER_256: usize = 256 - STATEFUL_TRANSACTION_SIGNATURE_BYTES;
const PROOF_BYTES_BEAT_IMPLICIT_269: usize = 268 - STATEFUL_TRANSACTION_SIGNATURE_BYTES;
const PROOF_BYTES_TIE_IMPLICIT_269: usize = 269 - STATEFUL_TRANSACTION_SIGNATURE_BYTES;
const PROOF_BYTES_BEAT_STATELESS_SHA256: usize =
    STATELESS_SHA256_SIGNATURE_BYTES - STATEFUL_TRANSACTION_SIGNATURE_BYTES - 1;
const LINEAR_SIGMA_PROOF_COMMITMENT_BYTES: usize = 32;
const LINEAR_SIGMA_MASKED_WITNESS_BYTES: usize = 41;
const LINEAR_SIGMA_TARGET_PROOF_BYTES: usize =
    LINEAR_SIGMA_PROOF_COMMITMENT_BYTES + LINEAR_SIGMA_MASKED_WITNESS_BYTES;
const LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES: usize =
    STATEFUL_TRANSACTION_SIGNATURE_BYTES + LINEAR_SIGMA_TARGET_PROOF_BYTES;
const SIS_SYNDROME_RESIDUES: usize = 18;
const SIS_SEED_RESIDUES: usize = 18;
const SIS_RESIDUE_BITS: usize = FIELD_BITS;
const SIS_SYNDROME_PACKED_BITS: usize = SIS_SYNDROME_RESIDUES * SIS_RESIDUE_BITS;
const SIS_SYNDROME_PACKED_BYTES: usize = SIS_SYNDROME_PACKED_BITS.div_ceil(8);
const SIS_SYNDROME_SLACK_BITS: usize = DIGEST_BYTES * 8 - SIS_SYNDROME_PACKED_BITS;
const SIS_SEED_PACKED_BITS: usize = SIS_SEED_RESIDUES * SIS_RESIDUE_BITS;
const SIS_SEED_PACKED_BYTES: usize = SIS_SEED_PACKED_BITS.div_ceil(8);
const SIS_SEED_SLACK_BITS: usize = PAD_SEED_BYTES * 8 - SIS_SEED_PACKED_BITS;
const SIS_FULL_RESPONSE_RESIDUES: usize = 23;
const SIS_FULL_RESPONSE_PUBLIC_ROWS: usize = 12;
const SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS: usize =
    SIS_FULL_RESPONSE_PUBLIC_ROWS * SIS_RESIDUE_BITS;
const SIS_FULL_RESPONSE_SYNDROME_PACKED_BYTES: usize =
    SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS.div_ceil(8);
const SIS_FULL_RESPONSE_SYNDROME_SLACK_BITS: usize =
    DIGEST_BYTES * 8 - SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS;
const SIS_FULL_RESPONSE_PACKED_BITS: usize = SIS_FULL_RESPONSE_RESIDUES * SIS_RESIDUE_BITS;
const SIS_FULL_RESPONSE_PACKED_BYTES: usize = SIS_FULL_RESPONSE_PACKED_BITS.div_ceil(8);
const SIS_FULL_RESPONSE_SLACK_BITS: usize =
    LINEAR_SIGMA_MASKED_WITNESS_BYTES * 8 - SIS_FULL_RESPONSE_PACKED_BITS;
const SIS_FULL_RESPONSE_MATRIX_DOMAIN: &str =
    "Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1";
const LOCAL_SYNDROME_SLOPE: u16 = 50;

fn sample_proof_commitment() -> [u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES] {
    pack_q_syndrome_digest(&[7_u16; SIS_FULL_RESPONSE_PUBLIC_ROWS])
}

#[derive(Clone, Debug, Serialize)]
struct LocalDeltaStatefulReport {
    profile: &'static str,
    q: u16,
    groups: usize,
    local_free: usize,
    repetitions: usize,
    commitment_domain: String,
    commitment_domain_bytes: usize,
    pad_domain: String,
    pad_domain_bytes: usize,
    pad_frame_bytes: usize,
    fiber_state_bytes: usize,
    pad_seed_bytes: usize,
    hidden_preimage_bytes: usize,
    digest_bytes: usize,
    local_opening_bytes_per_rep: usize,
    stateful_spend_signature_bytes: usize,
    stateful_transaction_signature_bytes: usize,
    revealed_preimage_transaction_signature_bytes: usize,
    stateless_sha256_signature_bytes: usize,
    sample_fiber_state_hex: String,
    sample_pad_seed_hex: String,
    sample_hidden_preimage_hex: String,
    sample_commitment_digest_hex: String,
    sample_full_response_hidden_preimage_hex: String,
    sample_full_response_syndrome_digest_hex: String,
    sample_block: usize,
    sample_pad_frame_hex: String,
    sample_pad_bytes_hex: String,
    sample_pad_residues: [u16; LOCAL_FREE],
    sample_mask_view: [u16; LOCAL_FREE],
    sample_opening_bytes_hex: String,
    selected_opening_primitive: OpeningPrimitiveReport,
    sis_opening_parameters: SisOpeningParameterReport,
    sample_seed_derived_fiber_state_hex: String,
    sample_seed_restricted_hidden_preimage_hex: String,
    opening_proof_frontier: OpeningProofFrontier,
    committed_sigma_byte_frontier: CommittedSigmaByteFrontier,
    assumption_review_targets: Vec<AssumptionReviewTarget>,
    assumption_closure_report: Vec<AssumptionClosureReport>,
    concrete_security_budgets: Vec<ConcreteSecurityBudget>,
    full_response_matrix_sampler_certificate: MatrixSamplerCertificate,
    full_response_quotient_algebra_certificate: FullResponseQuotientAlgebraCertificate,
    cryptoanalysis_attack_surfaces: Vec<CryptoanalysisAttackSurface>,
    matrix_short_relation_probe: MatrixShortRelationProbe,
    lattice_sis_reduction_probe: LatticeSisReductionProbe,
    hidden_fiber_short_combination_probe: HiddenFiberShortCombinationProbe,
    quotient_witness_hiding_distribution_report: QuotientWitnessHidingDistributionReport,
    masked_response_sampler_certificate: MaskedResponseSamplerCertificate,
    conditioned_quotient_fiber_sampler_report: ConditionedQuotientFiberSamplerReport,
    public_proof_forgery_probe: PublicProofForgeryProbe,
    public_sigma_equation_forgery_probe: PublicSigmaEquationForgeryProbe,
    quotient_witness_hiding_game_bound: QuotientWitnessHidingGameBound,
    final_soundness_advantage_formula: FinalSoundnessAdvantageFormula,
    quotient_hiding_sampler_review: QuotientHidingSamplerReview,
    challenge_action_invertibility_report: ChallengeActionInvertibilityReport,
    fork_equation_algebra_report: ForkEquationAlgebraReport,
    canonical_response_lift_report: CanonicalResponseLiftReport,
    byte_response_arithmetic_report: ByteResponseArithmeticReport,
    fiat_shamir_fork_accounting_report: FiatShamirForkAccountingReport,
    fiat_shamir_fork_generation_ledger: FiatShamirForkGenerationLedger,
    accepted_fiber_constraint_report: AcceptedFiberConstraintReport,
    kernel_delta_acceptance_probe: KernelDeltaAcceptanceProbe,
    hiding_route_frontier: Vec<HidingRouteFrontierRow>,
    extraction_soundness_closure_report: ExtractionSoundnessClosureReport,
    fork_extraction_review_report: ForkExtractionReviewReport,
    stateful_lifecycle_policy: StatefulLifecyclePolicyReport,
    reference_implementation_report: ReferenceImplementationReport,
    production_reference_api_report: ProductionReferenceApiReport,
    third_party_review_packet: ThirdPartyReviewPacket,
    internal_security_review_audit: InternalSecurityReviewAudit,
    pad_sampler_certificate: PadSamplerCertificate,
    fiat_shamir_frame_certificate: FiatShamirFrameCertificate,
    blockchain_signature_size_comparison: Vec<SignatureSizeBenchmark>,
}

#[derive(Clone, Debug, Serialize)]
struct OpeningPrimitiveReport {
    name: &'static str,
    witness_domain: &'static str,
    proof_commitment_bytes: usize,
    masked_witness_response_bytes: usize,
    proof_bytes: usize,
    total_signature_bytes: usize,
    verifier_equation: &'static str,
    binding_requirement: &'static str,
    witness_hiding_requirement: &'static str,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct SisOpeningParameterReport {
    modulus: u16,
    matrix_domain: &'static str,
    matrix_domain_bytes: usize,
    seed_domain_bytes: usize,
    digest_bytes: usize,
    seed_residues: usize,
    syndrome_residues: usize,
    residue_bits: usize,
    packed_seed_bits: usize,
    packed_seed_bytes: usize,
    seed_slack_bits: usize,
    packed_syndrome_bits: usize,
    packed_syndrome_bytes: usize,
    syndrome_slack_bits: usize,
    relation_shape: &'static str,
    validity_requirement: &'static str,
    security_role: &'static str,
    matrix_rank: usize,
    matrix_full_rank: bool,
    matrix_determinant_mod_q: u16,
    sample_matrix_prefix: Vec<u16>,
    structural_warning: &'static str,
    fiber_split_options: Vec<SisFiberSplitReport>,
    linear_blinding_obstruction: &'static str,
    full_response_fiber_candidate: SisFullResponseFiberReport,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct SisFiberSplitReport {
    public_syndrome_residues: usize,
    public_capacity_bits: usize,
    hidden_fiber_residues: usize,
    hidden_fiber_capacity_bits: usize,
    prefix_matrix_rank: usize,
    public_capacity_meets_128: bool,
    hidden_capacity_meets_128: bool,
    balanced_128_possible: bool,
    note: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct SisFullResponseFiberReport {
    matrix_domain: &'static str,
    matrix_domain_bytes: usize,
    response_bytes: usize,
    response_residues: usize,
    response_packed_bits: usize,
    response_slack_bits: usize,
    public_syndrome_residues: usize,
    public_capacity_bits: usize,
    packed_public_syndrome_bits: usize,
    packed_public_syndrome_bytes: usize,
    public_syndrome_slack_bits: usize,
    hidden_fiber_residues: usize,
    hidden_fiber_capacity_bits: usize,
    prefix_matrix_rank: usize,
    kernel_dimension: usize,
    sample_kernel_vector_prefix: Vec<u16>,
    sample_kernel_vector_nonzero: bool,
    sample_kernel_syndrome_zero: bool,
    public_capacity_meets_128: bool,
    hidden_capacity_meets_128: bool,
    same_target_response_width: bool,
    lean_reduction_gate: &'static str,
    quotient_relation_gate: &'static str,
    existential_quotient_gate: &'static str,
    local_delta_quotient_predicate: &'static str,
    concrete_decoder_gate: &'static str,
    honest_representative_gate: &'static str,
    semantic_quotient_residues: usize,
    hidden_randomizer_residues: usize,
    full_preimage_binding_compatible: bool,
    fiber_saturation_requirement: &'static str,
    relation_shift: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct OpeningProofFrontier {
    base_transaction_bytes: usize,
    under_256_max_proof_bytes: usize,
    beat_269_max_proof_bytes: usize,
    tie_269_max_proof_bytes: usize,
    beat_stateless_sha256_max_proof_bytes: usize,
    candidates: Vec<OpeningProofCandidate>,
}

#[derive(Clone, Debug, Serialize)]
struct OpeningProofCandidate {
    name: &'static str,
    proof_bytes: usize,
    total_signature_bytes: usize,
    public_checkable: bool,
    witness_hiding: bool,
    implemented: bool,
    production_candidate: bool,
    note: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct CommittedSigmaByteFrontier {
    base_transaction_bytes: usize,
    proof_commitment_bytes: usize,
    sub_256_cap_bytes: usize,
    current_response_hidden_link_budget_under_256: usize,
    fiber_saturated_hidden_link_budget_under_256: usize,
    fiber_state_preserving_hidden_link_budget_under_256: usize,
    account_hidden_link_contract: AccountHiddenLinkContract,
    candidates: Vec<CommittedSigmaByteCandidate>,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct CommittedSigmaByteCandidate {
    name: &'static str,
    masked_response_bytes: usize,
    hidden_link_opening_bytes: usize,
    account_hidden_link_bytes: usize,
    proof_bytes: usize,
    transaction_bytes: usize,
    hidden_link_budget_under_256: usize,
    total_stateful_slot_cost_bytes: usize,
    structurally_admissible_without_extra_proof: bool,
    required_next_obligation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct AccountHiddenLinkContract {
    lean_surface: &'static str,
    verifier_accepts_surface: &'static str,
    soundness_certificate_surface: &'static str,
    account_opening_verifier_surface: &'static str,
    account_link_proof_system_surface: &'static str,
    homomorphic_account_link_surface: &'static str,
    pre_response_backing_surface: &'static str,
    response_link_soundness_surface: &'static str,
    bounded_response_link_kernel_surface: &'static str,
    bounded_response_link_soundness_theorem: &'static str,
    bounded_response_link_checker_script: &'static str,
    response_link_bridge_theorem: &'static str,
    response_link_code_frontier: Vec<ResponseLinkCodeCandidate>,
    slot_fields: Vec<&'static str>,
    signature_fields: Vec<&'static str>,
    homomorphic_link_equation: &'static str,
    verifier_equation: &'static str,
    accepting_implication: &'static str,
    unbacked_acceptance_theorem: &'static str,
    named_break_event: &'static str,
    rejected_instantiations: Vec<&'static str>,
    link_proof_frontier: Vec<AccountLinkProofCandidate>,
    preferred_instantiation: &'static str,
    open_cryptographic_obligation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct AccountLinkProofCandidate {
    name: &'static str,
    signature_link_proof_bytes: usize,
    account_link_bytes: usize,
    total_link_state_bytes: usize,
    reveals_witness_or_randomizer: bool,
    local_opening_only: bool,
    account_slot_aware: bool,
    instantiates_account_link_proof_system: bool,
    post_challenge_representative_rejected: bool,
    not_public_projection_of_sigma_equation: bool,
    witness_hiding_argument: bool,
    structurally_admissible: bool,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ResponseLinkCodeCandidate {
    name: &'static str,
    commitment_residues: usize,
    commitment_bytes: usize,
    coordinate_projection: Vec<usize>,
    public_kernel_dimension_before_link: usize,
    projection_rank_on_public_kernel: usize,
    residual_full_kernel_dimension: usize,
    combined_constraint_rank: usize,
    combined_kernel_dimension: usize,
    lll_iterations: usize,
    response_difference_coeff_bound: i64,
    shortest_intersection_vector_coeff_bound: i64,
    lll_margin_over_response_difference_bound: i64,
    shortest_intersection_vector_l1_weight: u128,
    shortest_intersection_vector_signed_support: usize,
    lll_box_probe_excludes_response_delta: bool,
    full_kernel_sound: bool,
    public_account_witness_hiding: bool,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct AssumptionReviewTarget {
    name: &'static str,
    lean_surface: &'static str,
    rust_surface: &'static str,
    review_question: &'static str,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct AssumptionClosureReport {
    gate: &'static str,
    closure_artifact: &'static str,
    lean_game: &'static str,
    break_event: &'static str,
    no_break_claim: &'static str,
    target_classical_security_bits: usize,
    target_quantum_security_bits: usize,
    query_bound_status: &'static str,
    residual_assumption: &'static str,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ConcreteSecurityBudget {
    game: &'static str,
    raw_security_bits: usize,
    adversary_query_bound_log2: usize,
    classical_security_bits_after_queries: usize,
    quantum_security_bits_after_queries: usize,
    meets_128_classical: bool,
    meets_128_quantum: bool,
    note: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct CryptoanalysisAttackSurface {
    gate: &'static str,
    target_object: &'static str,
    attacker_goal: &'static str,
    current_evidence: &'static str,
    next_test: &'static str,
    proof_status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct MatrixShortRelationProbe {
    matrix_domain: &'static str,
    relation_space: &'static str,
    coefficient_bound: i16,
    max_support: usize,
    candidates_tested: usize,
    zero_relations_found: usize,
    shortest_nonzero_syndrome_lee_weight: u32,
    shortest_nonzero_syndrome_support: usize,
    shortest_nonzero_relation: Vec<i16>,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct LatticeSisReductionProbe {
    matrix_domain: &'static str,
    lattice_dimension: usize,
    basis_vectors: usize,
    lll_delta: f64,
    lll_iterations: usize,
    shortest_vector_euclidean_squared: u128,
    shortest_vector_lee_weight: u32,
    shortest_vector_support: usize,
    shortest_vector_syndrome_zero: bool,
    shortest_vector_prefix: Vec<i64>,
    shortest_vector_coeff_bound: i64,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct HiddenFiberShortCombinationProbe {
    kernel_basis_vectors: usize,
    coefficient_bound: i16,
    combinations_tested: usize,
    shortest_vector_lee_weight: u32,
    shortest_vector_support: usize,
    shortest_coefficients: Vec<i16>,
    shortest_vector_prefix: Vec<u16>,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct PublicProofForgeryProbe {
    attack_name: &'static str,
    attack_model: &'static str,
    uses_private_key_material: bool,
    account_hidden_witness_available: bool,
    public_key_model: &'static str,
    signature_bytes: usize,
    verifier_accepts: bool,
    accepted_reason: String,
    sigma_equation_verifier_accepts: bool,
    sigma_equation_reason: String,
    forged_slot_digest_hex: String,
    forged_signature_sha256: String,
    chosen_mask_view: [u16; LOCAL_FREE],
    conditioned_sampler_rank: usize,
    sampled_free_residues: usize,
    proof_commitment_bound_into_challenge: bool,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct PublicSigmaEquationForgeryProbe {
    attack_name: &'static str,
    attack_model: &'static str,
    uses_private_key_material: bool,
    account_hidden_witness_available: bool,
    signature_bytes: usize,
    verifier_accepts: bool,
    accepted_reason: String,
    forged_slot_digest_hex: String,
    forged_signature_sha256: String,
    proof_commitment_syndrome: Vec<u16>,
    sigma_response_syndrome: Vec<u16>,
    chosen_fiber_state: u16,
    conditioned_sampler_rank: usize,
    sampled_free_residues: usize,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct QuotientWitnessHidingDistributionReport {
    public_quotient_residues: usize,
    hidden_kernel_dimension: usize,
    algebraic_hidden_fiber_bits: usize,
    packed_slack_bits: usize,
    canonical_representatives_per_public_syndrome_lower_bound_log2: usize,
    fiber_state_constraint_bits: usize,
    estimated_representatives_after_fiber_state_bits: usize,
    current_fixture_representatives_sampled_per_statement: usize,
    current_fixture_uses_hidden_fiber_entropy: bool,
    fiber_saturated_relation_required: bool,
    distribution_gap: &'static str,
    next_required_bound: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct MaskedResponseSamplerCertificate {
    sampler_name: &'static str,
    proof_surface: &'static str,
    raw_algebraic_fiber_bits: usize,
    modeled_fiber_state_constraint_bits: usize,
    residual_hiding_bits: usize,
    residual_hiding_meets_140: bool,
    exact_support_model: &'static str,
    kernel_projection_coordinates: Vec<usize>,
    kernel_projection_rank: usize,
    residue1_conditioning_modulus: usize,
    residue1_conditioning_class: usize,
    residue1_allowed_values: usize,
    exact_conditioned_support_formula: &'static str,
    exact_conditioned_support_decimal: String,
    exact_conditioned_support_log2: f64,
    exact_conditioned_min_entropy_floor_bits: usize,
    capacity_model_overstates_exact_floor_by_bits: usize,
    exact_quantum_query_bound_log2_for_128: usize,
    allowed_conditioning: Vec<&'static str>,
    excluded_conditioning: Vec<&'static str>,
    active_pad_outputs_conditioned: bool,
    pad_constraints_layer: &'static str,
    production_sampler_law: &'static str,
    ideal_sampler_law: &'static str,
    sampler_symmetry_target: &'static str,
    statistical_distance_metric: &'static str,
    statistical_distance_numerator: usize,
    statistical_distance_denominator: usize,
    statistical_distance_is_zero: bool,
    finite_distance_proof: &'static str,
    finite_distance_bound_status: &'static str,
    lean_certificate: &'static str,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct QuotientWitnessHidingGameBound {
    game: &'static str,
    bound_type: &'static str,
    samples_per_stateful_slot: usize,
    sampler_statistical_distance_numerator: usize,
    sampler_statistical_distance_denominator: usize,
    game_advantage_numerator: usize,
    game_advantage_denominator: usize,
    advantage_is_zero: bool,
    support_min_entropy_floor_bits: usize,
    support_role: &'static str,
    oracle_model: &'static str,
    reusable_multi_sample_status: &'static str,
    deployment_condition: &'static str,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct FinalSoundnessAdvantageFormula {
    lean_surface: &'static str,
    formula: &'static str,
    qrom_hash_query_bound_log2: usize,
    qrom_quadratic_loss_bits: usize,
    transcript_challenge_bits: usize,
    action_collision_floor_bits: usize,
    transcript_term_bits_after_qrom_loss: usize,
    action_collision_term_bits_after_qrom_loss: usize,
    union_safe_min_hash_action_bits: usize,
    matrix_mismatch_term: &'static str,
    extraction_break_term: &'static str,
    quotient_hiding_term: &'static str,
    pad_prf_term: &'static str,
    fiat_shamir_frame_term: &'static str,
    ledger_status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct QuotientHidingSamplerReview {
    lean_surface: &'static str,
    sampler_is_secret_independent: bool,
    only_allowed_conditioning: bool,
    active_pad_outputs_excluded: bool,
    rejection_predicate_matches_ideal_conditioning: bool,
    per_slot_tv_distance_zero: bool,
    independent_slots_compose_with_zero_tv: bool,
    support_floor_bits: usize,
    support_floor_role: &'static str,
    implementation_obligation: &'static str,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ConditionedQuotientFiberSamplerReport {
    sampler_name: &'static str,
    algorithm: &'static str,
    public_syndrome_rows: usize,
    packed_state_constraint_rows: usize,
    total_linear_rows: usize,
    linear_rank: usize,
    sampled_free_residues: usize,
    residue1_allowed_values: usize,
    sample_residue1_value: u16,
    exact_support_formula: &'static str,
    retry_loop_required: bool,
    max_retries: usize,
    expected_retries: usize,
    sample_preserves_public_syndrome: bool,
    sample_preserves_fiber_state_bytes: bool,
    sample_is_canonical_packed_q: bool,
    sample_digest_hex: String,
    sample_masked_witness_response_hex: String,
    timing_policy: &'static str,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct AcceptedFiberConstraintReport {
    repetitions: usize,
    distinct_active_blocks: usize,
    algebraic_hidden_fiber_bits: usize,
    fiber_state_constraint_bits: usize,
    max_bits_after_fiber_state_constraint: usize,
    meets_128_after_fiber_state_constraint: bool,
    pad_output_bits_per_active_block: usize,
    active_pad_constraint_bits: usize,
    estimated_remaining_fiber_bits_after_acceptance: usize,
    max_distinct_active_blocks_for_128_bits: usize,
    max_distinct_active_blocks_for_nonzero_estimate: usize,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct KernelDeltaAcceptanceProbe {
    delta_source: &'static str,
    delta_lee_weight: u32,
    delta_support: usize,
    delta_coeff_bound: i64,
    tested_multipliers: Vec<i64>,
    syndrome_preserved_for_all_tested: bool,
    canonical_packing_for_all_tested: bool,
    fiber_state_preserving_multipliers: Vec<i64>,
    active_pad_preserving_multipliers: Vec<i64>,
    accepting_multipliers: Vec<i64>,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct HidingRouteFrontierRow {
    route: &'static str,
    public_rows: usize,
    response_residues: usize,
    response_bytes: usize,
    masked_witness_response_bytes: usize,
    proof_bytes: usize,
    transaction_signature_bytes: usize,
    hidden_fiber_bits: usize,
    modeled_constraint_bits: usize,
    residual_hiding_bits: usize,
    meets_128_hiding: bool,
    byte_delta_from_247: isize,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ExtractionSoundnessClosureReport {
    game: &'static str,
    accepted_equation_surface: &'static str,
    extractor_input: &'static str,
    extractor_output: &'static str,
    closed_obligations: Vec<&'static str>,
    fork_extraction_surface: &'static str,
    fork_closed_obligations: Vec<&'static str>,
    fork_remaining_assumption: &'static str,
    remaining_assumption: &'static str,
    break_event: &'static str,
    review_status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ForkExtractionReviewReport {
    lean_surface: &'static str,
    fork_shape: &'static str,
    same_statement_fields: Vec<&'static str>,
    differing_field: &'static str,
    accepted_equations: Vec<&'static str>,
    extractor_claim: &'static str,
    break_event: &'static str,
    fs_forking_dependency: &'static str,
    qrom_dependency: &'static str,
    concrete_review_target: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ChallengeActionInvertibilityReport {
    action_name: &'static str,
    challenge_bytes: usize,
    action_residues: usize,
    modulus: u16,
    model: &'static str,
    extension_polynomial: &'static str,
    extension_polynomial_coefficients_low_to_high: Vec<u16>,
    extension_polynomial_degree: usize,
    extension_polynomial_monic: bool,
    irreducibility_status: &'static str,
    rabin_x_q12_minus_x_zero: bool,
    rabin_gcd_x_q6_minus_x_degree: usize,
    rabin_gcd_x_q4_minus_x_degree: usize,
    rabin_irreducible: bool,
    invertibility_rule: &'static str,
    bad_difference_event: &'static str,
    bad_difference_probability_formula: &'static str,
    bad_difference_denominator_decimal: String,
    bad_difference_floor_bits: usize,
    generic_quantum_margin_for_128_bits: usize,
    sample_challenge0_hex: String,
    sample_challenge1_hex: String,
    sample_action0: Vec<u16>,
    sample_action1: Vec<u16>,
    sample_actions_distinct: bool,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ForkEquationAlgebraReport {
    lean_surface: &'static str,
    equation_model: &'static str,
    extraction_formula: &'static str,
    modulus: u16,
    extension_polynomial: &'static str,
    action_difference_nonzero: bool,
    inverse_exists: bool,
    inverse_check_is_one: bool,
    fork_difference_matches_scaled_digest: bool,
    extracted_digest_matches: bool,
    sample_digest: Vec<u16>,
    sample_action0: Vec<u16>,
    sample_action1: Vec<u16>,
    sample_action_difference: Vec<u16>,
    sample_action_difference_inverse: Vec<u16>,
    sample_extracted_digest: Vec<u16>,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct StatefulLifecyclePolicyReport {
    policy_name: &'static str,
    slot_identifier_bytes: usize,
    slot_creation_rule: &'static str,
    signing_rule: &'static str,
    verification_rule: &'static str,
    consumption_rule: &'static str,
    concurrency_rule: &'static str,
    crash_consistency_rule: &'static str,
    backup_restore_rule: &'static str,
    reorg_rule: &'static str,
    tests_cover: Vec<&'static str>,
    remaining_deployment_work: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ReferenceImplementationReport {
    api_surface: Vec<&'static str>,
    keygen_slots: usize,
    public_key_bytes: usize,
    account_slots: usize,
    signature_bytes: usize,
    verifies: bool,
    consumes_slot: bool,
    rejects_replay: bool,
    known_answer_public_key_sha256: String,
    known_answer_slot_digest_hex: String,
    known_answer_signature_sha256: String,
    negative_tests: Vec<&'static str>,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ReferenceTimingReport {
    benchmark_name: &'static str,
    iterations: usize,
    keygen_slots: usize,
    message_bytes: usize,
    signature_bytes: usize,
    public_key_bytes: usize,
    account_slots: usize,
    timings: Vec<ReferenceTimingRow>,
    caveat: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ReferenceTimingRow {
    operation: &'static str,
    iterations: usize,
    total_ns: u128,
    avg_ns: u128,
    avg_us: f64,
    accepted_or_ok: bool,
}

#[derive(Clone, Debug, Serialize)]
struct ProductionReferenceApiReport {
    api_surface: Vec<&'static str>,
    account_commitment_bytes: usize,
    account_commitment_layout: Vec<&'static str>,
    target_transaction_signature_bytes: usize,
    executable_signature_bytes: usize,
    executable_matches_target: bool,
    keygen_slots: usize,
    verifies: bool,
    consumes_slot: bool,
    rejects_replay: bool,
    binds_account_randomizer_commitment: bool,
    known_answer_account_commitment_sha256: String,
    known_answer_signature_sha256: String,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct KnownAnswerVector {
    name: &'static str,
    bytes: usize,
    sha256_or_hex: String,
    review_role: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ThirdPartyReviewPacket {
    packet_name: &'static str,
    target_profile: &'static str,
    signature_bytes: usize,
    public_key_bytes: usize,
    exact_sha256_assumptions: Vec<&'static str>,
    extraction_break_target: &'static str,
    matrix_sampler_review: Vec<&'static str>,
    known_answer_vectors: Vec<KnownAnswerVector>,
    fuzz_property_tests: Vec<&'static str>,
    constant_time_status: &'static str,
    implementation_limitations: Vec<&'static str>,
    review_gate_status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct SecurityReviewFinding {
    severity: &'static str,
    area: &'static str,
    finding: &'static str,
    evidence: &'static str,
    required_closure: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct InternalSecurityReviewAudit {
    audit_name: &'static str,
    target_profile: &'static str,
    verdict: &'static str,
    findings: Vec<SecurityReviewFinding>,
    ready_for_external_review_as: &'static str,
    not_ready_for_external_review_as: &'static str,
    recommended_next_steps: Vec<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
struct MatrixSamplerCertificate {
    domain: &'static str,
    domain_hex: String,
    rows: usize,
    cols: usize,
    modulus: u16,
    rejection_cutoff: u16,
    row_encoding: &'static str,
    col_encoding: &'static str,
    counter_encoding: &'static str,
    matrix_transcript_sha256: String,
    first_row: Vec<u16>,
    first_column: Vec<u16>,
    all_entries_lt_modulus: bool,
}

#[derive(Clone, Debug, Serialize)]
struct FullResponseQuotientAlgebraCertificate {
    rank: usize,
    pivot_columns: Vec<usize>,
    free_columns: Vec<usize>,
    kernel_dimension: usize,
    kernel_basis_vectors: Vec<Vec<u16>>,
    kernel_basis_transcript_sha256: String,
    kernel_basis_vectors_nonzero: bool,
    kernel_basis_syndrome_zero: bool,
    right_inverse_transcript_sha256: String,
    right_inverse_check: bool,
    semantic_quotient_capacity_bits: usize,
    hidden_fiber_capacity_bits: usize,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct CanonicalResponseLiftReport {
    lean_surface: &'static str,
    lift_name: &'static str,
    modulus: u16,
    matrix_rows: usize,
    matrix_cols: usize,
    matrix_rank: usize,
    response_bytes: usize,
    response_residues: usize,
    response_packed_bits: usize,
    response_slack_bits: usize,
    right_inverse_check: bool,
    sample_input_digest: Vec<u16>,
    sample_input_digest_hex: String,
    lifted_vector: Vec<u16>,
    lifted_response_hex: String,
    decoded_is_canonical: bool,
    all_residues_lt_modulus: bool,
    slack_bits_zero: bool,
    lifted_syndrome_matches_input: bool,
    packed_digest_matches_input: bool,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct ByteResponseArithmeticReport {
    lean_surface: &'static str,
    operation_model: &'static str,
    response_bytes: usize,
    action_residues: usize,
    z0_is_canonical: bool,
    z1_is_canonical: bool,
    difference_response_is_canonical: bool,
    extracted_response_is_canonical: bool,
    z0_equation_holds: bool,
    z1_equation_holds: bool,
    response_difference_matches_field_difference: bool,
    inverse_scaled_response_hits_digest: bool,
    extracted_response_packed_digest_matches: bool,
    sample_proof_commitment_syndrome: Vec<u16>,
    sample_digest: Vec<u16>,
    sample_z0_syndrome: Vec<u16>,
    sample_z1_syndrome: Vec<u16>,
    sample_difference_syndrome: Vec<u16>,
    sample_extracted_syndrome: Vec<u16>,
    sample_extracted_response_hex: String,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct FiatShamirForkAccountingReport {
    lean_surface: &'static str,
    accounting_scope: &'static str,
    challenge_entropy_bits: usize,
    algebraic_challenge_entropy_bits: usize,
    transcript_hardening_bits: usize,
    action_collision_floor_bits: usize,
    fs_classical_query_bound_log2_for_128: usize,
    fs_quantum_query_bound_log2_for_128: usize,
    action_collision_classical_query_bound_log2_for_128: usize,
    action_collision_quantum_query_bound_log2_for_128: usize,
    bottleneck_classical_query_bound_log2_for_128: usize,
    bottleneck_quantum_query_bound_log2_for_128: usize,
    same_commitment_fork_generation_dependency: &'static str,
    qrom_dependency: &'static str,
    bad_event_terms: Vec<&'static str>,
    algebraic_fork_path_closed: bool,
    action_collision_budget_meets_128_at_bottleneck: bool,
    fs_budget_meets_128_at_bottleneck: bool,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct FiatShamirForkGenerationLedger {
    lean_surface: &'static str,
    accounting_scope: &'static str,
    selected_qrom_theorem: &'static str,
    qrom_theorem_source: &'static str,
    qrom_loss_model: &'static str,
    qrom_hash_query_bound_log2: usize,
    qrom_quadratic_loss_bits: usize,
    qrom_measured_oracle_input: &'static str,
    qrom_reprogrammed_output: &'static str,
    qrom_same_commitment_condition: &'static str,
    qrom_theorem_fits_protocol_shape: bool,
    single_term_quantum_query_bound_log2_for_128: usize,
    single_term_classical_query_bound_log2_for_128: usize,
    union_safe_quantum_query_bound_log2_for_128: usize,
    union_safe_classical_query_bound_log2_for_128: usize,
    transcript_challenge_bits: usize,
    action_collision_floor_bits: usize,
    transcript_quantum_bits_at_union_bound: usize,
    action_collision_quantum_bits_at_union_bound: usize,
    transcript_classical_bits_at_union_bound: usize,
    action_collision_classical_bits_at_union_bound: usize,
    union_bound_terms: Vec<&'static str>,
    union_bound_reserved_bits: usize,
    union_bound_meets_128: bool,
    same_commitment_fork_generation_assumption: &'static str,
    qrom_fork_generation_assumption: &'static str,
    conclusion: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct PadSamplerCertificate {
    domain: &'static str,
    domain_hex: String,
    frame_bytes: usize,
    counter_encoding: &'static str,
    rejection_cutoff: u32,
    sample_block: usize,
    sample_frame_hex: String,
    sample_seed_hex: String,
    sample_counter0_digest_hex: String,
    sample_residues: [u16; LOCAL_FREE],
    sample_residues_lt_modulus: bool,
}

#[derive(Clone, Debug, Serialize)]
struct FiatShamirFrameCertificate {
    domain: &'static str,
    domain_hex: String,
    frame_labels: Vec<&'static str>,
    binds_proof_commitment: bool,
    sample_frame_bytes: usize,
    sample_frame_sha256: String,
    sample_challenge_hex: String,
    challenge_bytes: usize,
    algebraic_challenge_bytes: usize,
    hardening_challenge_bytes: usize,
    algebraic_last_byte_mask: u8,
    algebraic_entropy_bits: usize,
    transcript_entropy_bits: usize,
    excludes_response_and_opening_bytes: bool,
}

#[derive(Clone, Debug, Serialize)]
struct SignatureSizeBenchmark {
    scheme: &'static str,
    post_quantum: bool,
    public_key_bytes: usize,
    signature_bytes: usize,
    transaction_signature_bytes: usize,
    stateful: bool,
    source: &'static str,
    note: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompactChallenge {
    active_block: [usize; REPS],
    sign: [i8; REPS],
}

#[derive(Clone, Debug)]
struct PublicView {
    _pivot: [[u16; LOCAL_PIVOT]; GROUPS],
    free: [[u16; LOCAL_FREE]; GROUPS],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompactResponses {
    active_free: [[i16; LOCAL_FREE]; REPS],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LocalOpenings {
    active_free: [[u16; LOCAL_FREE]; REPS],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinearSigmaTargetProof {
    proof_commitment: [u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
    masked_witness_response: [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
}

#[derive(Clone, Debug)]
struct AccountSlot {
    digest: [u8; DIGEST_BYTES],
    unused: bool,
    ideal_masked_witness_response: Option<[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES]>,
}

#[derive(Clone, Debug)]
struct AccountState {
    slots: Vec<AccountSlot>,
}

#[derive(Clone, Debug)]
struct ReferenceKeypair {
    public_key: Vec<u8>,
    account: AccountState,
}

type ProductionReferenceSecretKey = ProductionSecretKey<ReferenceKeypair>;
type ProductionReferenceKeypair = ProductionKeypair<ReferenceKeypair>;

#[derive(Clone, Debug)]
struct ConditionedFiberSample {
    masked_witness_response: [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
    linear_rank: usize,
    sampled_free_residues: usize,
    residue1_value: u16,
}

#[derive(Clone, Debug, Serialize)]
struct StatefulVerifyReport {
    accepted: bool,
    profile: &'static str,
    proof_model: &'static str,
    public_key_bytes: usize,
    message_bytes: usize,
    signature_bytes: usize,
    expected_signature_bytes: usize,
    slot: Option<u32>,
    slot_was_unused: Option<bool>,
    slot_consumed: bool,
    challenge_bytes: usize,
    response_bytes: usize,
    opening_bytes: usize,
    hash_frame_bytes: Option<usize>,
    hash_frame_sha256: Option<String>,
    claimed_challenge_hex: Option<String>,
    recomputed_challenge_hex: Option<String>,
    reason: String,
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn read_bits_le(bytes: &[u8], bit_offset: usize, bit_len: usize) -> u32 {
    let mut out = 0_u32;
    for i in 0..bit_len {
        let bit = (bytes[(bit_offset + i) / 8] >> ((bit_offset + i) % 8)) & 1;
        out |= (bit as u32) << i;
    }
    out
}

fn any_bits_set_le(bytes: &[u8], bit_offset: usize, bit_len: usize) -> bool {
    (0..bit_len).any(|i| {
        let bit_index = bit_offset + i;
        ((bytes[bit_index / 8] >> (bit_index % 8)) & 1) != 0
    })
}

fn write_bits_le(bytes: &mut [u8], bit_offset: usize, bit_len: usize, value: u16) {
    for i in 0..bit_len {
        let bit = ((value >> i) & 1) as u8;
        let byte = &mut bytes[(bit_offset + i) / 8];
        let mask = 1_u8 << ((bit_offset + i) % 8);
        if bit == 0 {
            *byte &= !mask;
        } else {
            *byte |= mask;
        }
    }
}

fn decode_challenge(bytes: &[u8]) -> Result<CompactChallenge, String> {
    if bytes.len() != CHALLENGE_BYTES {
        return Err(format!(
            "challenge has {} bytes, expected {CHALLENGE_BYTES}",
            bytes.len()
        ));
    }
    if (bytes[ALGEBRAIC_CHALLENGE_BYTES - 1] & !ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK) != 0 {
        return Err("challenge is not canonical: high bits are set".to_string());
    }
    let mut active_block = [0_usize; REPS];
    let mut sign = [0_i8; REPS];
    for rep in 0..REPS {
        let bit = rep * CHALLENGE_BITS_PER_REP;
        active_block[rep] = read_bits_le(bytes, bit, 4) as usize;
        if active_block[rep] >= GROUPS {
            return Err(format!(
                "challenge active block in round {rep} is out of range: {}",
                active_block[rep]
            ));
        }
        sign[rep] = if read_bits_le(bytes, bit + 4, 1) == 0 {
            1
        } else {
            -1
        };
    }
    Ok(CompactChallenge { active_block, sign })
}

fn unpack_q_elements(bytes: &[u8], count: usize) -> Result<Vec<u16>, String> {
    prodref::unpack_q_elements(bytes, count)
}

fn decode_public_key(public_key: &[u8]) -> Result<PublicView, String> {
    if public_key.len() != PK_BYTES {
        return Err(format!(
            "public key has {} bytes, expected {PK_BYTES}",
            public_key.len()
        ));
    }
    let coords = unpack_q_elements(&public_key[PK_SEED_BYTES..], VIEW_COORDS)?;
    let mut pivot = [[0_u16; LOCAL_PIVOT]; GROUPS];
    let mut free = [[0_u16; LOCAL_FREE]; GROUPS];
    for (idx, &value) in coords[..PIVOT_COORDS].iter().enumerate() {
        pivot[idx / LOCAL_PIVOT][idx % LOCAL_PIVOT] = value;
    }
    for (idx, &value) in coords[PIVOT_COORDS..].iter().enumerate() {
        free[idx / LOCAL_FREE][idx % LOCAL_FREE] = value;
    }
    Ok(PublicView {
        _pivot: pivot,
        free,
    })
}

fn decode_responses(bytes: &[u8]) -> Result<CompactResponses, String> {
    if bytes.len() != ACTIVE_RESPONSE_BYTES {
        return Err(format!(
            "response payload has {} bytes, expected {ACTIVE_RESPONSE_BYTES}",
            bytes.len()
        ));
    }
    let mut active_free = [[0_i16; LOCAL_FREE]; REPS];
    for rep in 0..REPS {
        for coord in 0..LOCAL_FREE {
            let raw = bytes[rep * LOCAL_FREE + coord];
            if raw > (2 * RESPONSE_BOUND) as u8 {
                return Err(format!(
                    "active response coordinate {coord} in round {rep} is non-canonical: {raw}"
                ));
            }
            active_free[rep][coord] = raw as i16 - RESPONSE_BOUND;
        }
    }
    Ok(CompactResponses { active_free })
}

fn decode_openings(bytes: &[u8]) -> Result<LocalOpenings, String> {
    if bytes.len() != LOCAL_OPENING_BYTES {
        return Err(format!(
            "opening payload has {} bytes, expected {LOCAL_OPENING_BYTES}",
            bytes.len()
        ));
    }
    let mut active_free = [[0_u16; LOCAL_FREE]; REPS];
    for rep in 0..REPS {
        let base = rep * LOCAL_OPENING_BYTES_PER_REP;
        let pair = [
            u16_le(&bytes[base..base + 2]),
            u16_le(&bytes[base + 2..base + 4]),
        ];
        for (coord, &value) in pair.iter().enumerate() {
            if value >= Q {
                return Err(format!(
                    "opening coordinate {coord} in round {rep} is non-canonical: {value}"
                ));
            }
            active_free[rep][coord] = value;
        }
    }
    Ok(LocalOpenings { active_free })
}

fn decode_linear_sigma_target_proof(bytes: &[u8]) -> Result<LinearSigmaTargetProof, String> {
    if bytes.len() != LINEAR_SIGMA_TARGET_PROOF_BYTES {
        return Err(format!(
            "linear sigma proof has {} bytes, expected {LINEAR_SIGMA_TARGET_PROOF_BYTES}",
            bytes.len()
        ));
    }
    let proof_commitment = bytes[..LINEAR_SIGMA_PROOF_COMMITMENT_BYTES]
        .try_into()
        .unwrap();
    let masked_witness_response = bytes[LINEAR_SIGMA_PROOF_COMMITMENT_BYTES..]
        .try_into()
        .unwrap();
    Ok(LinearSigmaTargetProof {
        proof_commitment,
        masked_witness_response,
    })
}

fn encode_responses(responses: &CompactResponses) -> Vec<u8> {
    prodref::encode_responses(&prodref::ProductionCompactResponses {
        active_free: responses.active_free,
    })
    .expect("reference responses are in canonical range")
    .to_vec()
}

fn encode_openings(openings: &LocalOpenings) -> Vec<u8> {
    prodref::encode_openings(&prodref::ProductionLocalOpenings {
        active_free: openings.active_free,
    })
    .expect("reference openings are canonical residues")
    .to_vec()
}

fn encode_linear_sigma_target_proof(proof: &LinearSigmaTargetProof) -> Vec<u8> {
    prodref::encode_linear_sigma_target_proof(&prodref::ProductionLinearSigmaTargetProof {
        proof_commitment: proof.proof_commitment,
        masked_witness_response: proof.masked_witness_response,
    })
    .to_vec()
}

fn encode_hidden_preimage(
    fiber_state: [u8; FIBER_STATE_BYTES],
    pad_seed: [u8; PAD_SEED_BYTES],
) -> [u8; HIDDEN_PREIMAGE_BYTES] {
    let mut out = [0_u8; HIDDEN_PREIMAGE_BYTES];
    out[..FIBER_STATE_BYTES].copy_from_slice(&fiber_state);
    out[FIBER_STATE_BYTES..].copy_from_slice(&pad_seed);
    out
}

fn commitment_digest(hidden_preimage: &[u8; HIDDEN_PREIMAGE_BYTES]) -> [u8; DIGEST_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(COMMITMENT_DOMAIN);
    hasher.update(hidden_preimage);
    hasher.finalize().into()
}

fn semantic_hidden_preimage_from_masked_witness(
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> [u8; HIDDEN_PREIMAGE_BYTES] {
    masked_witness_response[..HIDDEN_PREIMAGE_BYTES]
        .try_into()
        .unwrap()
}

fn full_response_vector_from_masked_witness(
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Option<[u16; SIS_FULL_RESPONSE_RESIDUES]> {
    if read_bits_le(
        masked_witness_response,
        SIS_FULL_RESPONSE_PACKED_BITS,
        SIS_FULL_RESPONSE_SLACK_BITS,
    ) != 0
    {
        return None;
    }
    let mut vector = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
    for (idx, value) in vector.iter_mut().enumerate() {
        let decoded = read_bits_le(
            masked_witness_response,
            idx * SIS_RESIDUE_BITS,
            SIS_RESIDUE_BITS,
        );
        if decoded >= Q as u32 {
            return None;
        }
        *value = decoded as u16;
    }
    Some(vector)
}

fn masked_witness_from_full_response_vector(
    vector: &[u16; SIS_FULL_RESPONSE_RESIDUES],
) -> [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES] {
    let mut out = [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES];
    for (idx, &value) in vector.iter().enumerate() {
        assert!(value < Q);
        write_bits_le(&mut out, idx * SIS_RESIDUE_BITS, SIS_RESIDUE_BITS, value);
    }
    out
}

fn full_response_syndrome_array(
    vector: &[u16; SIS_FULL_RESPONSE_RESIDUES],
) -> [u16; SIS_FULL_RESPONSE_PUBLIC_ROWS] {
    let mut syndrome = [0_u16; SIS_FULL_RESPONSE_PUBLIC_ROWS];
    for (row, out) in syndrome.iter_mut().enumerate() {
        let mut acc = 0_u32;
        for (col, &value) in vector.iter().enumerate() {
            acc += full_response_matrix_entry(row, col) as u32 * value as u32;
            acc %= Q as u32;
        }
        *out = acc as u16;
    }
    syndrome
}

fn pack_q_syndrome_digest(elements: &[u16; SIS_FULL_RESPONSE_PUBLIC_ROWS]) -> [u8; DIGEST_BYTES] {
    let mut digest = [0_u8; DIGEST_BYTES];
    for (idx, &value) in elements.iter().enumerate() {
        assert!(value < Q);
        write_bits_le(&mut digest, idx * SIS_RESIDUE_BITS, SIS_RESIDUE_BITS, value);
    }
    digest
}

fn unpack_q_syndrome_digest(
    digest: &[u8; DIGEST_BYTES],
) -> Option<[u16; SIS_FULL_RESPONSE_PUBLIC_ROWS]> {
    if any_bits_set_le(
        digest,
        SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS,
        SIS_FULL_RESPONSE_SYNDROME_SLACK_BITS,
    ) {
        return None;
    }
    let mut out = [0_u16; SIS_FULL_RESPONSE_PUBLIC_ROWS];
    for (idx, value) in out.iter_mut().enumerate() {
        let decoded = read_bits_le(digest, idx * SIS_RESIDUE_BITS, SIS_RESIDUE_BITS);
        if decoded >= Q as u32 {
            return None;
        }
        *value = decoded as u16;
    }
    Some(out)
}

fn full_response_syndrome_digest(
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Option<[u8; DIGEST_BYTES]> {
    let vector = full_response_vector_from_masked_witness(masked_witness_response)?;
    let syndrome = full_response_syndrome_array(&vector);
    Some(pack_q_syndrome_digest(&syndrome))
}

fn derive_fiber_state_from_seed(seed: &[u8; PAD_SEED_BYTES]) -> [u8; FIBER_STATE_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(b"Descender.Signature.q12289.local_delta.seed_fiber.v1");
    hasher.update(seed);
    let digest = hasher.finalize();
    [digest[0], digest[1]]
}

fn sis_matrix_entry(row: usize, col: usize) -> u16 {
    assert!(row < SIS_SYNDROME_RESIDUES);
    assert!(col < SIS_SEED_RESIDUES);
    domain_matrix_entry(SIS_MATRIX_DOMAIN, row, col)
}

fn full_response_matrix_entry(row: usize, col: usize) -> u16 {
    assert!(row < SIS_FULL_RESPONSE_PUBLIC_ROWS);
    assert!(col < SIS_FULL_RESPONSE_RESIDUES);
    domain_matrix_entry(SIS_FULL_RESPONSE_MATRIX_DOMAIN, row, col)
}

fn domain_matrix_entry(domain: &str, row: usize, col: usize) -> u16 {
    let rejection_limit = u16::MAX - (u16::MAX % Q);
    for counter in 0_u32.. {
        let mut hasher = Sha256::new();
        hasher.update(domain.as_bytes());
        hasher.update((row as u16).to_le_bytes());
        hasher.update((col as u16).to_le_bytes());
        hasher.update(counter.to_le_bytes());
        let digest = hasher.finalize();
        for chunk in digest.chunks_exact(2) {
            let value = u16::from_le_bytes([chunk[0], chunk[1]]);
            if value < rejection_limit {
                return value % Q;
            }
        }
    }
    unreachable!("unbounded rejection sampler loop");
}

fn sis_matrix() -> [[u16; SIS_SEED_RESIDUES]; SIS_SYNDROME_RESIDUES] {
    let mut matrix = [[0_u16; SIS_SEED_RESIDUES]; SIS_SYNDROME_RESIDUES];
    for (row_idx, row) in matrix.iter_mut().enumerate() {
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = sis_matrix_entry(row_idx, col_idx);
        }
    }
    matrix
}

fn mod_pow(mut base: u16, mut exponent: u16) -> u16 {
    let modulus = Q as u32;
    let mut out = 1_u32;
    let mut base_acc = base as u32;
    while exponent > 0 {
        if exponent & 1 == 1 {
            out = out * base_acc % modulus;
        }
        base_acc = base_acc * base_acc % modulus;
        exponent >>= 1;
    }
    base = out as u16;
    base
}

fn mod_inv(value: u16) -> u16 {
    assert_ne!(value, 0);
    mod_pow(value, Q - 2)
}

fn poly_trim(poly: &mut Vec<u16>) {
    while poly.last().is_some_and(|&value| value == 0) {
        poly.pop();
    }
}

fn poly_sub(lhs: &[u16], rhs: &[u16]) -> Vec<u16> {
    let mut out = vec![0_u16; lhs.len().max(rhs.len())];
    for idx in 0..out.len() {
        out[idx] = sub_mod(
            lhs.get(idx).copied().unwrap_or(0),
            rhs.get(idx).copied().unwrap_or(0),
        );
    }
    poly_trim(&mut out);
    out
}

fn poly_div_rem(lhs: &[u16], rhs: &[u16]) -> (Vec<u16>, Vec<u16>) {
    let mut rem = lhs.to_vec();
    poly_trim(&mut rem);
    let mut divisor = rhs.to_vec();
    poly_trim(&mut divisor);
    assert!(!divisor.is_empty());
    let mut quotient = vec![0_u16; rem.len().saturating_sub(divisor.len()) + 1];
    let inv_lead = mod_inv(*divisor.last().unwrap());
    while rem.len() >= divisor.len() && !rem.is_empty() {
        let degree_delta = rem.len() - divisor.len();
        let factor = ((rem[rem.len() - 1] as u32 * inv_lead as u32) % Q as u32) as u16;
        quotient[degree_delta] = factor;
        for (idx, &value) in divisor.iter().enumerate() {
            let rem_idx = degree_delta + idx;
            rem[rem_idx] = sub_mul_mod(rem[rem_idx], factor, value);
        }
        poly_trim(&mut rem);
    }
    poly_trim(&mut quotient);
    (quotient, rem)
}

fn poly_mod(lhs: &[u16], modulus: &[u16]) -> Vec<u16> {
    poly_div_rem(lhs, modulus).1
}

fn poly_mul_mod(lhs: &[u16], rhs: &[u16], modulus: &[u16]) -> Vec<u16> {
    if lhs.is_empty() || rhs.is_empty() {
        return Vec::new();
    }
    let mut product = vec![0_u16; lhs.len() + rhs.len() - 1];
    for (i, &x) in lhs.iter().enumerate() {
        for (j, &y) in rhs.iter().enumerate() {
            product[i + j] = add_mod(product[i + j], ((x as u32 * y as u32) % Q as u32) as u16);
        }
    }
    poly_mod(&product, modulus)
}

fn poly_pow_mod(base: &[u16], mut exponent: u128, modulus: &[u16]) -> Vec<u16> {
    let mut result = vec![1_u16];
    let mut power = poly_mod(base, modulus);
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = poly_mul_mod(&result, &power, modulus);
        }
        power = poly_mul_mod(&power, &power, modulus);
        exponent >>= 1;
    }
    result
}

fn poly_make_monic(poly: &mut [u16]) {
    if poly.is_empty() {
        return;
    }
    let inv = mod_inv(*poly.last().unwrap());
    for coeff in poly.iter_mut() {
        *coeff = ((*coeff as u32 * inv as u32) % Q as u32) as u16;
    }
}

fn poly_gcd(lhs: &[u16], rhs: &[u16]) -> Vec<u16> {
    let mut a = lhs.to_vec();
    let mut b = rhs.to_vec();
    poly_trim(&mut a);
    poly_trim(&mut b);
    while !b.is_empty() {
        let r = poly_div_rem(&a, &b).1;
        a = b;
        b = r;
    }
    poly_make_monic(&mut a);
    a
}

fn poly_degree(poly: &[u16]) -> usize {
    let mut trimmed = poly.to_vec();
    poly_trim(&mut trimmed);
    trimmed.len().saturating_sub(1)
}

fn poly_x_q_power_minus_x(power: usize, modulus: &[u16]) -> Vec<u16> {
    let x = vec![0_u16, 1_u16];
    let mut x_power = x.clone();
    for _ in 0..power {
        x_power = poly_pow_mod(&x_power, Q as u128, modulus);
    }
    poly_sub(&x_power, &x)
}

fn sub_mul_mod(lhs: u16, factor: u16, rhs: u16) -> u16 {
    let modulus = Q as u32;
    let subtrahend = factor as u32 * rhs as u32 % modulus;
    ((modulus + lhs as u32 - subtrahend) % modulus) as u16
}

fn matrix_rank_mod_q(matrix: &[[u16; SIS_SEED_RESIDUES]; SIS_SYNDROME_RESIDUES]) -> usize {
    let mut work = *matrix;
    let mut rank = 0_usize;
    for col in 0..SIS_SEED_RESIDUES {
        let Some(pivot) = (rank..SIS_SYNDROME_RESIDUES).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        for row in 0..SIS_SYNDROME_RESIDUES {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..SIS_SEED_RESIDUES {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
        }
        rank += 1;
        if rank == SIS_SYNDROME_RESIDUES {
            break;
        }
    }
    rank
}

fn matrix_prefix_rank_mod_q(rows: usize) -> usize {
    assert!(rows <= SIS_SYNDROME_RESIDUES);
    let mut work = vec![vec![0_u16; SIS_SEED_RESIDUES]; rows];
    for (row_idx, row) in work.iter_mut().enumerate() {
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = sis_matrix_entry(row_idx, col_idx);
        }
    }

    let mut rank = 0_usize;
    for col in 0..SIS_SEED_RESIDUES {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..SIS_SEED_RESIDUES {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn rank_mod_q_dynamic(mut work: Vec<Vec<u16>>) -> usize {
    let rows = work.len();
    let cols = work.first().map_or(0, Vec::len);
    let mut rank = 0_usize;
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..cols {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn full_response_prefix_rank_mod_q(rows: usize) -> usize {
    assert!(rows <= SIS_FULL_RESPONSE_PUBLIC_ROWS);
    let mut work = vec![vec![0_u16; SIS_FULL_RESPONSE_RESIDUES]; rows];
    for (row_idx, row) in work.iter_mut().enumerate() {
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = full_response_matrix_entry(row_idx, col_idx);
        }
    }

    let mut rank = 0_usize;
    for col in 0..SIS_FULL_RESPONSE_RESIDUES {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..SIS_FULL_RESPONSE_RESIDUES {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn full_response_kernel_vector() -> [u16; SIS_FULL_RESPONSE_RESIDUES] {
    let rows = SIS_FULL_RESPONSE_PUBLIC_ROWS;
    let cols = SIS_FULL_RESPONSE_RESIDUES;
    let mut work = vec![vec![0_u16; cols]; rows];
    for (row_idx, row) in work.iter_mut().enumerate() {
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = full_response_matrix_entry(row_idx, col_idx);
        }
    }

    let mut rank = 0_usize;
    let mut pivot_cols = Vec::with_capacity(rows);
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..cols {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
        }
        pivot_cols.push(col);
        rank += 1;
        if rank == rows {
            break;
        }
    }

    let free_col = (0..cols)
        .find(|col| !pivot_cols.contains(col))
        .expect("full-response matrix must have a free column");
    let mut kernel = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
    kernel[free_col] = 1;
    for (row, &pivot_col) in pivot_cols.iter().enumerate() {
        kernel[pivot_col] = if work[row][free_col] == 0 {
            0
        } else {
            Q - work[row][free_col]
        };
    }
    kernel
}

fn full_response_rref() -> (Vec<Vec<u16>>, Vec<usize>) {
    let cols = SIS_FULL_RESPONSE_RESIDUES;
    let mut work = vec![vec![0_u16; cols]; SIS_FULL_RESPONSE_PUBLIC_ROWS];
    for (row_idx, row) in work.iter_mut().enumerate() {
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = full_response_matrix_entry(row_idx, col_idx);
        }
    }
    rref_mod_q(work)
}

fn rref_mod_q(mut work: Vec<Vec<u16>>) -> (Vec<Vec<u16>>, Vec<usize>) {
    let rows = work.len();
    let cols = work.first().map_or(0, Vec::len);
    let mut rank = 0_usize;
    let mut pivot_cols = Vec::with_capacity(rows);
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..cols {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
        }
        pivot_cols.push(col);
        rank += 1;
        if rank == rows {
            break;
        }
    }
    (work, pivot_cols)
}

fn kernel_basis_mod_q(
    constraint_rows: Vec<Vec<u16>>,
    cols: usize,
) -> (Vec<[u16; SIS_FULL_RESPONSE_RESIDUES]>, Vec<usize>) {
    assert_eq!(cols, SIS_FULL_RESPONSE_RESIDUES);
    let (rref, pivot_cols) = rref_mod_q(constraint_rows);
    let free_cols = full_response_free_columns(&pivot_cols);
    let kernel = free_cols
        .iter()
        .map(|&free_col| {
            let mut vector = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
            vector[free_col] = 1;
            for (row, &pivot_col) in pivot_cols.iter().enumerate() {
                vector[pivot_col] = if rref[row][free_col] == 0 {
                    0
                } else {
                    Q - rref[row][free_col]
                };
            }
            vector
        })
        .collect();
    (kernel, pivot_cols)
}

fn full_response_free_columns(pivot_cols: &[usize]) -> Vec<usize> {
    (0..SIS_FULL_RESPONSE_RESIDUES)
        .filter(|col| !pivot_cols.contains(col))
        .collect()
}

fn full_response_kernel_basis() -> Vec<[u16; SIS_FULL_RESPONSE_RESIDUES]> {
    let (rref, pivot_cols) = full_response_rref();
    let free_cols = full_response_free_columns(&pivot_cols);
    free_cols
        .iter()
        .map(|&free_col| {
            let mut kernel = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
            kernel[free_col] = 1;
            for (row, &pivot_col) in pivot_cols.iter().enumerate() {
                kernel[pivot_col] = if rref[row][free_col] == 0 {
                    0
                } else {
                    Q - rref[row][free_col]
                };
            }
            kernel
        })
        .collect()
}

fn full_response_kernel_projection_rank(coords: &[usize]) -> usize {
    let kernel_basis = full_response_kernel_basis();
    let mut projection = vec![vec![0_u16; kernel_basis.len()]; coords.len()];
    for (row, &coord) in coords.iter().enumerate() {
        for (col, vector) in kernel_basis.iter().enumerate() {
            projection[row][col] = vector[coord];
        }
    }
    rank_mod_q_dynamic(projection)
}

fn rref_mod_q_with_rhs(
    mut work: Vec<Vec<u16>>,
    mut rhs: Vec<u16>,
) -> (Vec<Vec<u16>>, Vec<u16>, Vec<usize>) {
    let rows = work.len();
    let cols = work.first().map_or(0, Vec::len);
    assert_eq!(rhs.len(), rows);
    let mut rank = 0_usize;
    let mut pivot_cols = Vec::with_capacity(rows);
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        rhs.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        rhs[rank] = ((rhs[rank] as u32 * inv as u32) % Q as u32) as u16;
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..cols {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
            rhs[row] = sub_mul_mod(rhs[row], factor, rhs[rank]);
        }
        pivot_cols.push(col);
        rank += 1;
        if rank == rows {
            break;
        }
    }
    (work, rhs, pivot_cols)
}

fn solve_full_response_syndrome(
    target: &[u16; SIS_FULL_RESPONSE_PUBLIC_ROWS],
) -> Option<[u16; SIS_FULL_RESPONSE_RESIDUES]> {
    let rows = SIS_FULL_RESPONSE_PUBLIC_ROWS;
    let cols = SIS_FULL_RESPONSE_RESIDUES;
    let mut work = vec![vec![0_u16; cols]; rows];
    for (row_idx, row) in work.iter_mut().enumerate() {
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = full_response_matrix_entry(row_idx, col_idx);
        }
    }
    let mut rhs = *target;
    let mut rank = 0_usize;
    let mut pivot_cols = Vec::with_capacity(rows);
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| work[row][col] != 0) else {
            continue;
        };
        work.swap(rank, pivot);
        rhs.swap(rank, pivot);
        let inv = mod_inv(work[rank][col]);
        for entry in &mut work[rank] {
            *entry = ((*entry as u32 * inv as u32) % Q as u32) as u16;
        }
        rhs[rank] = ((rhs[rank] as u32 * inv as u32) % Q as u32) as u16;
        for row in 0..rows {
            if row == rank || work[row][col] == 0 {
                continue;
            }
            let factor = work[row][col];
            for idx in col..cols {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[rank][idx]);
            }
            rhs[row] = sub_mul_mod(rhs[row], factor, rhs[rank]);
        }
        pivot_cols.push(col);
        rank += 1;
        if rank == rows {
            break;
        }
    }
    if rank != rows {
        return None;
    }
    let mut solution = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
    for (row, &pivot_col) in pivot_cols.iter().enumerate() {
        solution[pivot_col] = rhs[row];
    }
    Some(solution)
}

fn digest_u16_vectors<const N: usize>(vectors: &[[u16; N]]) -> String {
    let mut hasher = Sha256::new();
    for vector in vectors {
        for value in vector {
            hasher.update(value.to_le_bytes());
        }
    }
    hex_encode(&hasher.finalize())
}

fn full_response_matrix_transcript_sha256() -> String {
    let mut hasher = Sha256::new();
    for row in 0..SIS_FULL_RESPONSE_PUBLIC_ROWS {
        for col in 0..SIS_FULL_RESPONSE_RESIDUES {
            hasher.update(full_response_matrix_entry(row, col).to_le_bytes());
        }
    }
    hex_encode(&hasher.finalize())
}

fn full_response_matrix_sampler_certificate() -> MatrixSamplerCertificate {
    let mut first_column = Vec::with_capacity(SIS_FULL_RESPONSE_PUBLIC_ROWS);
    for row in 0..SIS_FULL_RESPONSE_PUBLIC_ROWS {
        first_column.push(full_response_matrix_entry(row, 0));
    }
    let mut all_entries_lt_modulus = true;
    for row in 0..SIS_FULL_RESPONSE_PUBLIC_ROWS {
        for col in 0..SIS_FULL_RESPONSE_RESIDUES {
            all_entries_lt_modulus &= full_response_matrix_entry(row, col) < Q;
        }
    }
    MatrixSamplerCertificate {
        domain: SIS_FULL_RESPONSE_MATRIX_DOMAIN,
        domain_hex: hex_encode(SIS_FULL_RESPONSE_MATRIX_DOMAIN.as_bytes()),
        rows: SIS_FULL_RESPONSE_PUBLIC_ROWS,
        cols: SIS_FULL_RESPONSE_RESIDUES,
        modulus: Q,
        rejection_cutoff: u16::MAX - (u16::MAX % Q),
        row_encoding: "u16 little-endian",
        col_encoding: "u16 little-endian",
        counter_encoding: "u32 little-endian",
        matrix_transcript_sha256: full_response_matrix_transcript_sha256(),
        first_row: (0..SIS_FULL_RESPONSE_RESIDUES)
            .map(|col| full_response_matrix_entry(0, col))
            .collect(),
        first_column,
        all_entries_lt_modulus,
    }
}

fn full_response_quotient_algebra_certificate() -> FullResponseQuotientAlgebraCertificate {
    let (_rref, pivot_columns) = full_response_rref();
    let free_columns = full_response_free_columns(&pivot_columns);
    let kernel_basis = full_response_kernel_basis();
    let kernel_basis_syndrome_zero = kernel_basis.iter().all(|vector| {
        full_response_syndrome_array(vector)
            .iter()
            .all(|&entry| entry == 0)
    });
    let kernel_basis_vectors_nonzero = kernel_basis
        .iter()
        .all(|vector| vector.iter().any(|&entry| entry != 0));

    let mut right_inverse = Vec::with_capacity(SIS_FULL_RESPONSE_PUBLIC_ROWS);
    let mut right_inverse_check = true;
    for row in 0..SIS_FULL_RESPONSE_PUBLIC_ROWS {
        let mut target = [0_u16; SIS_FULL_RESPONSE_PUBLIC_ROWS];
        target[row] = 1;
        let Some(solution) = solve_full_response_syndrome(&target) else {
            right_inverse_check = false;
            continue;
        };
        right_inverse_check &= full_response_syndrome_array(&solution) == target;
        right_inverse.push(solution);
    }

    FullResponseQuotientAlgebraCertificate {
        rank: pivot_columns.len(),
        pivot_columns,
        free_columns,
        kernel_dimension: kernel_basis.len(),
        kernel_basis_vectors: kernel_basis.iter().map(|vector| vector.to_vec()).collect(),
        kernel_basis_transcript_sha256: digest_u16_vectors(&kernel_basis),
        kernel_basis_vectors_nonzero,
        kernel_basis_syndrome_zero,
        right_inverse_transcript_sha256: digest_u16_vectors(&right_inverse),
        right_inverse_check,
        semantic_quotient_capacity_bits: SIS_FULL_RESPONSE_PUBLIC_ROWS * SIS_RESIDUE_BITS,
        hidden_fiber_capacity_bits: (SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS)
            * SIS_RESIDUE_BITS
            + SIS_FULL_RESPONSE_SLACK_BITS,
        interpretation: "rank-12 public quotient is surjective; the 11-dimensional kernel is hidden randomizer capacity, not byte-for-byte committed semantic state",
    }
}

fn full_response_syndrome_of_vector(vector: &[u16; SIS_FULL_RESPONSE_RESIDUES]) -> Vec<u16> {
    full_response_syndrome_array(vector).to_vec()
}

fn centered_mod_q(value: i64) -> u16 {
    value.rem_euclid(Q as i64) as u16
}

fn lee_abs(value: u16) -> u32 {
    let value = value as u32;
    value.min(Q as u32 - value)
}

fn lee_weight(values: &[u16]) -> u32 {
    values.iter().map(|&value| lee_abs(value)).sum()
}

fn support(values: &[u16]) -> usize {
    values.iter().filter(|&&value| value != 0).count()
}

fn signed_relation_syndrome(coefficients: &[i16]) -> Vec<u16> {
    let mut syndrome = vec![0_u16; SIS_FULL_RESPONSE_PUBLIC_ROWS];
    for (col, &coeff) in coefficients.iter().enumerate() {
        if coeff == 0 {
            continue;
        }
        for (row, entry) in syndrome.iter_mut().enumerate() {
            let contribution = coeff as i64 * full_response_matrix_entry(row, col) as i64;
            *entry = centered_mod_q(*entry as i64 + contribution);
        }
    }
    syndrome
}

fn enumerate_sparse_relations(
    cols: usize,
    max_support: usize,
    coeff_bound: i16,
    mut visit: impl FnMut(&[i16]),
) {
    fn rec(
        start: usize,
        cols: usize,
        remaining: usize,
        coeff_bound: i16,
        coeffs: &mut [i16],
        visit: &mut impl FnMut(&[i16]),
    ) {
        if remaining == 0 {
            visit(coeffs);
            return;
        }
        for col in start..cols {
            for coeff in -coeff_bound..=coeff_bound {
                if coeff == 0 {
                    continue;
                }
                coeffs[col] = coeff;
                rec(col + 1, cols, remaining - 1, coeff_bound, coeffs, visit);
                coeffs[col] = 0;
            }
        }
    }

    let mut coeffs = vec![0_i16; cols];
    for weight in 1..=max_support {
        rec(0, cols, weight, coeff_bound, &mut coeffs, &mut visit);
    }
}

fn matrix_short_relation_probe() -> MatrixShortRelationProbe {
    let coefficient_bound = 2;
    let max_support = 3;
    let mut candidates_tested = 0_usize;
    let mut zero_relations_found = 0_usize;
    let mut shortest_nonzero_syndrome_lee_weight = u32::MAX;
    let mut shortest_nonzero_syndrome_support = 0_usize;
    let mut shortest_nonzero_relation = Vec::new();

    enumerate_sparse_relations(
        SIS_FULL_RESPONSE_RESIDUES,
        max_support,
        coefficient_bound,
        |coefficients| {
            candidates_tested += 1;
            let syndrome = signed_relation_syndrome(coefficients);
            if syndrome.iter().all(|&entry| entry == 0) {
                zero_relations_found += 1;
                return;
            }
            let weight = lee_weight(&syndrome);
            if weight < shortest_nonzero_syndrome_lee_weight {
                shortest_nonzero_syndrome_lee_weight = weight;
                shortest_nonzero_syndrome_support = support(&syndrome);
                shortest_nonzero_relation = coefficients.to_vec();
            }
        },
    );

    MatrixShortRelationProbe {
        matrix_domain: SIS_FULL_RESPONSE_MATRIX_DOMAIN,
        relation_space: "signed sparse public-syndrome relations over columns of the 12x23 full-response matrix",
        coefficient_bound,
        max_support,
        candidates_tested,
        zero_relations_found,
        shortest_nonzero_syndrome_lee_weight,
        shortest_nonzero_syndrome_support,
        shortest_nonzero_relation,
        interpretation: "No relation in this bounded sparse search proves only that obvious low-support signed column dependencies were not found; it is not a lattice-reduction proof.",
    }
}

fn centered_lift(value: u16) -> i64 {
    let value = value as i64;
    if value > Q as i64 / 2 {
        value - Q as i64
    } else {
        value
    }
}

fn integer_kernel_lattice_basis() -> Vec<Vec<i64>> {
    let (_rref, pivot_cols) = full_response_rref();
    integer_lattice_basis_from_modular_kernel(full_response_kernel_basis(), pivot_cols)
}

fn integer_lattice_basis_from_modular_kernel(
    kernel_basis: Vec<[u16; SIS_FULL_RESPONSE_RESIDUES]>,
    pivot_cols: Vec<usize>,
) -> Vec<Vec<i64>> {
    let mut basis = Vec::with_capacity(SIS_FULL_RESPONSE_RESIDUES);
    for vector in kernel_basis {
        basis.push(vector.iter().map(|&entry| centered_lift(entry)).collect());
    }
    for pivot_col in pivot_cols {
        let mut row = vec![0_i64; SIS_FULL_RESPONSE_RESIDUES];
        row[pivot_col] = Q as i64;
        basis.push(row);
    }
    basis
}

fn full_response_constraint_rows_with_coordinates(coords: &[usize]) -> Vec<Vec<u16>> {
    let mut rows = Vec::with_capacity(SIS_FULL_RESPONSE_PUBLIC_ROWS + coords.len());
    for row_idx in 0..SIS_FULL_RESPONSE_PUBLIC_ROWS {
        let mut row = vec![0_u16; SIS_FULL_RESPONSE_RESIDUES];
        for (col_idx, entry) in row.iter_mut().enumerate() {
            *entry = full_response_matrix_entry(row_idx, col_idx);
        }
        rows.push(row);
    }
    for &coord in coords {
        assert!(coord < SIS_FULL_RESPONSE_RESIDUES);
        let mut row = vec![0_u16; SIS_FULL_RESPONSE_RESIDUES];
        row[coord] = 1;
        rows.push(row);
    }
    rows
}

fn response_link_intersection_lattice_basis(coords: &[usize]) -> (Vec<Vec<i64>>, usize, usize) {
    let rows = full_response_constraint_rows_with_coordinates(coords);
    let (kernel_basis, pivot_cols) = kernel_basis_mod_q(rows, SIS_FULL_RESPONSE_RESIDUES);
    let combined_rank = pivot_cols.len();
    let combined_kernel_dimension = kernel_basis.len();
    (
        integer_lattice_basis_from_modular_kernel(kernel_basis, pivot_cols),
        combined_rank,
        combined_kernel_dimension,
    )
}

fn dot_i64_f64(lhs: &[i64], rhs: &[f64]) -> f64 {
    lhs.iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| (a as f64) * b)
        .sum()
}

fn euclidean_squared(vector: &[i64]) -> u128 {
    vector
        .iter()
        .map(|&value| {
            let abs = value.unsigned_abs() as u128;
            abs * abs
        })
        .sum()
}

fn signed_l1_weight(vector: &[i64]) -> u128 {
    vector
        .iter()
        .map(|value| value.unsigned_abs() as u128)
        .sum()
}

fn signed_support(vector: &[i64]) -> usize {
    vector.iter().filter(|&&value| value != 0).count()
}

fn vector_coeff_bound(vector: &[i64]) -> i64 {
    vector.iter().map(|value| value.abs()).max().unwrap_or(0)
}

fn vector_to_mod_q(vector: &[i64]) -> [u16; SIS_FULL_RESPONSE_RESIDUES] {
    let mut out = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
    for (dst, &value) in out.iter_mut().zip(vector.iter()) {
        *dst = centered_mod_q(value);
    }
    out
}

fn recompute_gram_schmidt(basis: &[Vec<i64>]) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = basis.len();
    let dim = basis[0].len();
    let mut bstar = vec![vec![0.0_f64; dim]; n];
    let mut mu = vec![vec![0.0_f64; n]; n];
    let mut norm = vec![0.0_f64; n];

    for i in 0..n {
        for col in 0..dim {
            bstar[i][col] = basis[i][col] as f64;
        }
        for j in 0..i {
            if norm[j] == 0.0 {
                continue;
            }
            mu[i][j] = dot_i64_f64(&basis[i], &bstar[j]) / norm[j];
            for col in 0..dim {
                bstar[i][col] -= mu[i][j] * bstar[j][col];
            }
        }
        norm[i] = bstar[i].iter().map(|value| value * value).sum();
    }
    (mu, norm)
}

fn lll_reduce(mut basis: Vec<Vec<i64>>, delta: f64) -> (Vec<Vec<i64>>, usize) {
    let n = basis.len();
    let mut iterations = 0_usize;
    let mut k = 1_usize;
    let (mut mu, mut norm) = recompute_gram_schmidt(&basis);

    while k < n {
        iterations += 1;
        for j in (0..k).rev() {
            let q = mu[k][j].round() as i64;
            if q == 0 {
                continue;
            }
            for col in 0..basis[k].len() {
                basis[k][col] -= q * basis[j][col];
            }
            (mu, norm) = recompute_gram_schmidt(&basis);
        }

        if norm[k] >= (delta - mu[k][k - 1] * mu[k][k - 1]) * norm[k - 1] {
            k += 1;
        } else {
            basis.swap(k, k - 1);
            (mu, norm) = recompute_gram_schmidt(&basis);
            k = k.saturating_sub(1).max(1);
        }
    }

    (basis, iterations)
}

fn lattice_sis_reduction_probe() -> LatticeSisReductionProbe {
    let (shortest, lll_iterations) = lll_shortest_kernel_vector();
    let shortest_mod_q = vector_to_mod_q(&shortest);
    let syndrome = full_response_syndrome_array(&shortest_mod_q);

    LatticeSisReductionProbe {
        matrix_domain: SIS_FULL_RESPONSE_MATRIX_DOMAIN,
        lattice_dimension: SIS_FULL_RESPONSE_RESIDUES,
        basis_vectors: SIS_FULL_RESPONSE_RESIDUES,
        lll_delta: 0.99,
        lll_iterations,
        shortest_vector_euclidean_squared: euclidean_squared(&shortest),
        shortest_vector_lee_weight: lee_weight(&shortest_mod_q),
        shortest_vector_support: support(&shortest_mod_q),
        shortest_vector_syndrome_zero: syndrome.iter().all(|&entry| entry == 0),
        shortest_vector_prefix: shortest[..6].to_vec(),
        shortest_vector_coeff_bound: vector_coeff_bound(&shortest),
        interpretation: "LLL is run on an integer basis for the modular SIS kernel generated by the 11 q-kernel vectors plus q times the 10 pivot coordinates. This is stronger than the sparse smoke test but still not a complete SVP/BKZ analysis.",
    }
}

fn lll_shortest_kernel_vector() -> (Vec<i64>, usize) {
    lll_shortest_vector_from_basis(integer_kernel_lattice_basis())
}

fn lll_shortest_vector_from_basis(basis: Vec<Vec<i64>>) -> (Vec<i64>, usize) {
    let delta = 0.99;
    let (reduced, lll_iterations) = lll_reduce(basis, delta);
    let shortest = reduced
        .iter()
        .filter(|vector| vector.iter().any(|&value| value != 0))
        .min_by_key(|vector| euclidean_squared(vector))
        .expect("kernel lattice basis contains nonzero vectors");
    (shortest.clone(), lll_iterations)
}

fn hidden_fiber_short_combination_probe() -> HiddenFiberShortCombinationProbe {
    let kernel_basis = full_response_kernel_basis();
    let coefficient_bound = 1_i16;
    let basis_len = kernel_basis.len();
    let mut coefficients = vec![-coefficient_bound; basis_len];
    let mut combinations_tested = 0_usize;
    let mut shortest_vector_lee_weight = u32::MAX;
    let mut shortest_vector_support = 0_usize;
    let mut shortest_coefficients = Vec::new();
    let mut shortest_vector = [0_u16; SIS_FULL_RESPONSE_RESIDUES];

    loop {
        if coefficients.iter().any(|&coeff| coeff != 0) {
            combinations_tested += 1;
            let mut candidate = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
            for (basis_idx, &coeff) in coefficients.iter().enumerate() {
                if coeff == 0 {
                    continue;
                }
                for (dst, &entry) in candidate.iter_mut().zip(kernel_basis[basis_idx].iter()) {
                    *dst = centered_mod_q(*dst as i64 + coeff as i64 * entry as i64);
                }
            }
            let weight = lee_weight(&candidate);
            if weight < shortest_vector_lee_weight {
                shortest_vector_lee_weight = weight;
                shortest_vector_support = support(&candidate);
                shortest_coefficients = coefficients.clone();
                shortest_vector = candidate;
            }
        }

        let mut idx = 0;
        while idx < basis_len {
            if coefficients[idx] < coefficient_bound {
                coefficients[idx] += 1;
                break;
            }
            coefficients[idx] = -coefficient_bound;
            idx += 1;
        }
        if idx == basis_len {
            break;
        }
    }

    HiddenFiberShortCombinationProbe {
        kernel_basis_vectors: basis_len,
        coefficient_bound,
        combinations_tested,
        shortest_vector_lee_weight,
        shortest_vector_support,
        shortest_coefficients,
        shortest_vector_prefix: shortest_vector[..6].to_vec(),
        interpretation: "This bounded search looks for suspiciously short representatives inside the certified 11-dimensional hidden fiber basis. It is a smoke test, not a shortest-vector proof.",
    }
}

fn quotient_witness_hiding_distribution_report() -> QuotientWitnessHidingDistributionReport {
    let algebraic_hidden_fiber_bits = (SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS)
        * SIS_RESIDUE_BITS
        + SIS_FULL_RESPONSE_SLACK_BITS;
    let fiber_state_constraint_bits = FIBER_STATE_BYTES * 8;
    QuotientWitnessHidingDistributionReport {
        public_quotient_residues: SIS_FULL_RESPONSE_PUBLIC_ROWS,
        hidden_kernel_dimension: SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS,
        algebraic_hidden_fiber_bits,
        packed_slack_bits: SIS_FULL_RESPONSE_SLACK_BITS,
        canonical_representatives_per_public_syndrome_lower_bound_log2: algebraic_hidden_fiber_bits,
        fiber_state_constraint_bits,
        estimated_representatives_after_fiber_state_bits: algebraic_hidden_fiber_bits
            .saturating_sub(fiber_state_constraint_bits),
        current_fixture_representatives_sampled_per_statement: 1,
        current_fixture_uses_hidden_fiber_entropy: false,
        fiber_saturated_relation_required: true,
        distribution_gap: "The hardened algebraic quotient has 160 raw hidden bits per public syndrome and 144 modeled bits after fixed fiber-state equality, but the current fixture chooses one canonical representative for a mask view. That is enough for an honest trace and verifier tests, not enough by itself to discharge quotient witness hiding.",
        next_required_bound: "Define the production masked-response sampler over the accepted quotient fiber, then prove its output is close to a fiber-symmetric distribution after conditioning on canonical q-packing, fixed 2-byte fiber state, and response bounds. Active pad openings are handled by the existential quotient-opening predicate, not by conditioning the hidden representative.",
    }
}

fn decimal_pow_mul(base: u32, exponent: usize, multiplier: u32) -> String {
    const DECIMAL_BASE: u64 = 1_000_000_000;
    let mut limbs = vec![1_u64];
    for factor in (0..exponent)
        .map(|_| base)
        .chain(std::iter::once(multiplier))
    {
        let mut carry = 0_u64;
        for limb in &mut limbs {
            let value = *limb * factor as u64 + carry;
            *limb = value % DECIMAL_BASE;
            carry = value / DECIMAL_BASE;
        }
        while carry > 0 {
            limbs.push(carry % DECIMAL_BASE);
            carry /= DECIMAL_BASE;
        }
    }
    let mut out = limbs.last().unwrap().to_string();
    for limb in limbs.iter().rev().skip(1) {
        out.push_str(&format!("{limb:09}"));
    }
    out
}

fn residue_class_count(modulus: usize, class: usize) -> usize {
    (0..Q as usize)
        .filter(|residue| residue % modulus == class)
        .count()
}

fn exact_conditioned_support_log2(
    free_residues_after_projection: usize,
    allowed_values: usize,
) -> f64 {
    free_residues_after_projection as f64 * (Q as f64).log2() + (allowed_values as f64).log2()
}

fn exact_security_query_bound_log2_for_128(raw_bits: usize) -> usize {
    raw_bits.saturating_sub(128) / 2
}

fn challenge_action_log2_floor() -> usize {
    (CHALLENGE_ACTION_RESIDUES as f64 * (Q as f64).log2()).floor() as usize
}

fn challenge_action_quantum_margin_for_128() -> usize {
    exact_security_query_bound_log2_for_128(challenge_action_log2_floor())
}

fn masked_response_sampler_certificate() -> MaskedResponseSamplerCertificate {
    let raw_algebraic_fiber_bits = (SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS)
        * SIS_RESIDUE_BITS
        + SIS_FULL_RESPONSE_SLACK_BITS;
    let modeled_fiber_state_constraint_bits = FIBER_STATE_BYTES * 8;
    let residual_hiding_bits =
        raw_algebraic_fiber_bits.saturating_sub(modeled_fiber_state_constraint_bits);
    let kernel_projection_coordinates = vec![0, 1];
    let kernel_projection_rank =
        full_response_kernel_projection_rank(&kernel_projection_coordinates);
    let residue1_conditioning_modulus = 1_usize << (FIBER_STATE_BYTES * 8 - SIS_RESIDUE_BITS);
    let residue1_conditioning_class = 0;
    let residue1_allowed_values =
        residue_class_count(residue1_conditioning_modulus, residue1_conditioning_class);
    let free_residues_after_projection =
        SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS - kernel_projection_rank;
    let exact_conditioned_support_log2 =
        exact_conditioned_support_log2(free_residues_after_projection, residue1_allowed_values);
    let exact_conditioned_min_entropy_floor_bits = exact_conditioned_support_log2.floor() as usize;
    MaskedResponseSamplerCertificate {
        sampler_name: "production-masked-response-fiber-sampler",
        proof_surface: "FullResponseMaskedResponseSamplerCertificate",
        raw_algebraic_fiber_bits,
        modeled_fiber_state_constraint_bits,
        residual_hiding_bits,
        residual_hiding_meets_140: residual_hiding_bits >= 140,
        exact_support_model: "14-bit little-endian q-packing fixes residue 0 and residue 1 modulo 4; rank-2 projection of the 11-dimensional kernel onto coordinates [0, 1] gives support q^9 times the number of q-residues congruent to 0 modulo 4",
        kernel_projection_coordinates,
        kernel_projection_rank,
        residue1_conditioning_modulus,
        residue1_conditioning_class,
        residue1_allowed_values,
        exact_conditioned_support_formula: "12289^9 * 3073",
        exact_conditioned_support_decimal: decimal_pow_mul(
            Q as u32,
            free_residues_after_projection,
            residue1_allowed_values as u32,
        ),
        exact_conditioned_support_log2,
        exact_conditioned_min_entropy_floor_bits,
        capacity_model_overstates_exact_floor_by_bits: residual_hiding_bits
            .saturating_sub(exact_conditioned_min_entropy_floor_bits),
        exact_quantum_query_bound_log2_for_128: exact_security_query_bound_log2_for_128(
            exact_conditioned_min_entropy_floor_bits,
        ),
        allowed_conditioning: vec![
            "canonical 23-residue q-packing",
            "fixed 2-byte local fiber-state equality",
            "response bounds/canonicality",
        ],
        excluded_conditioning: vec![
            "active pad outputs",
            "active local openings",
            "challenge-selected block pad equations",
        ],
        active_pad_outputs_conditioned: false,
        pad_constraints_layer: "existential quotient-opening predicate",
        production_sampler_law: "sample uniform kernel coefficients for the 12x23 quotient map, add them to a digest-syndrome representative, and reject only on canonical packing, fixed fiber-state equality, and response bounds",
        ideal_sampler_law: "uniform distribution over the same conditioned quotient-fiber representative set",
        sampler_symmetry_target: "fiber-symmetric distribution over representatives with the same 12-residue public quotient",
        statistical_distance_metric: "total variation distance",
        statistical_distance_numerator: 0,
        statistical_distance_denominator: 1,
        statistical_distance_is_zero: true,
        finite_distance_proof: "conditioning a uniform kernel-coefficient sampler by rejection on the same predicate that defines the ideal conditioned fiber law gives exactly the ideal law",
        finite_distance_bound_status: "exact TV distance is zero for the stated rejection-sampling law; exact support-size/min-entropy for q=12289 is tracked separately from the 14-bit capacity model",
        lean_certificate: "FullResponseMaskedResponseSamplerCertificate.witnessHiding",
        conclusion: "The 264-byte proof may spend the 16-bit fiber-state equality but must not condition the hidden masked response on active pad outputs.",
    }
}

fn quotient_witness_hiding_game_bound() -> QuotientWitnessHidingGameBound {
    let sampler_certificate = masked_response_sampler_certificate();
    QuotientWitnessHidingGameBound {
        game: "QuotientWitnessHidingGame",
        bound_type: "one-shot statistical witness-hiding",
        samples_per_stateful_slot: 1,
        sampler_statistical_distance_numerator: sampler_certificate.statistical_distance_numerator,
        sampler_statistical_distance_denominator: sampler_certificate
            .statistical_distance_denominator,
        game_advantage_numerator: 0,
        game_advantage_denominator: 1,
        advantage_is_zero: true,
        support_min_entropy_floor_bits: sampler_certificate
            .exact_conditioned_min_entropy_floor_bits,
        support_role: "support size bounds the conditioned sampler population and collision/retry sanity; it is not the distinguishing advantage when both witness branches induce the same conditioned fiber law",
        oracle_model: "stateful one-shot transcript: the adversary receives a sampled proof transcript, not an equality oracle for the hidden representative",
        reusable_multi_sample_status: "independent stateful slots compose by product distributions; with exact per-sample TV distance zero, the statistical hiding contribution remains zero for any fixed number of independent slots",
        deployment_condition: "the signer must sample by rejection from uniform kernel coefficients using only canonical packing, fixed fiber-state equality, and response bounds; active pad outputs remain outside the hidden representative distribution",
        conclusion: "the game-specific hiding bound is perfect statistical hiding under the sampler certificate; the 133-bit support floor is retained as an audit metric rather than charged through a generic square-root query-loss model",
    }
}

fn final_soundness_advantage_formula() -> FinalSoundnessAdvantageFormula {
    let ledger = fiat_shamir_fork_generation_ledger();
    let union_safe_min_hash_action_bits = ledger
        .transcript_quantum_bits_at_union_bound
        .min(ledger.action_collision_quantum_bits_at_union_bound);
    FinalSoundnessAdvantageFormula {
        lean_surface: "FullResponseQROMExtractionBranchTheorem + FullResponseDeploymentReviewCertificate.statefulVerifier_conditionalSoundness",
        formula: "Adv_sound <= Adv_QROM_measure_reprogram + Pr[Fq12 action collision] + Adv_extraction_break + Adv_matrix_mismatch + Adv_pad_prf + Adv_frame_mismatch",
        qrom_hash_query_bound_log2: ledger.qrom_hash_query_bound_log2,
        qrom_quadratic_loss_bits: ledger.qrom_quadratic_loss_bits,
        transcript_challenge_bits: ledger.transcript_challenge_bits,
        action_collision_floor_bits: ledger.action_collision_floor_bits,
        transcript_term_bits_after_qrom_loss: ledger.transcript_quantum_bits_at_union_bound,
        action_collision_term_bits_after_qrom_loss: ledger
            .action_collision_quantum_bits_at_union_bound,
        union_safe_min_hash_action_bits,
        matrix_mismatch_term: "implementation-equivalence review target: SHA-256 rejection matrix sampler must match Lean/Rust transcript",
        extraction_break_term: "PackedFullResponseExtractionGame.extractionBreak after QROM fork and non-colliding Fq12 action",
        quotient_hiding_term: "QuotientWitnessHidingGame has per-slot TV distance 0 under the sampler certificate; support floor is audit-only",
        pad_prf_term: "SHA256PadPRFGame.padPRFBreak for seeded local pad pseudorandomness",
        fiat_shamir_frame_term: "FiatShamirFrameAgreementGame.frameMismatchBreak plus QROM measure-and-reprogram loss over the fixed prechallenge frame",
        ledger_status: "with q <= 2^16 quantum hash queries, the transcript and action-collision terms retain at least 130 and 131 bits respectively before adding primitive/extraction assumptions",
    }
}

fn build_linear_sigma_signature_from_proof(
    public_key: &[u8],
    message: &[u8],
    slot_id: u32,
    slot_digest: &[u8; DIGEST_BYTES],
    proof: &LinearSigmaTargetProof,
) -> Result<Vec<u8>, String> {
    let frame = challenge_hash_frame(
        public_key,
        message,
        slot_id,
        slot_digest,
        &proof.proof_commitment,
    );
    let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
    let challenge = decode_challenge(&challenge_bytes)?;
    let (responses, openings) = build_local_delta_response_openings_for_masked_response(
        public_key,
        &challenge,
        &proof.masked_witness_response,
    )?;
    let mut signature = Vec::with_capacity(LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES);
    signature.extend_from_slice(&slot_id.to_le_bytes());
    signature.extend_from_slice(&challenge_bytes);
    signature.extend_from_slice(&encode_responses(&responses));
    signature.extend_from_slice(&encode_openings(&openings));
    signature.extend_from_slice(&encode_linear_sigma_target_proof(proof));
    Ok(signature)
}

fn quotient_hiding_sampler_review() -> QuotientHidingSamplerReview {
    let sampler = masked_response_sampler_certificate();
    QuotientHidingSamplerReview {
        lean_surface: "FullResponseMaskedResponseSamplerCertificate + QuotientWitnessHidingStatisticalBound",
        sampler_is_secret_independent: true,
        only_allowed_conditioning: true,
        active_pad_outputs_excluded: !sampler.active_pad_outputs_conditioned,
        rejection_predicate_matches_ideal_conditioning: sampler.statistical_distance_is_zero,
        per_slot_tv_distance_zero: sampler.statistical_distance_is_zero,
        independent_slots_compose_with_zero_tv: sampler.statistical_distance_is_zero,
        support_floor_bits: sampler.exact_conditioned_min_entropy_floor_bits,
        support_floor_role: "audit metric for sampler population, collision, and retry sanity; not charged as the distinguishing advantage",
        implementation_obligation: "production signing must sample uniform kernel coefficients and reject only on canonical packing, fixed fiber-state equality, and response bounds; any active-pad-output conditioning invalidates the zero-TV certificate",
        conclusion: "quotient hiding is acceptable under the stated sampler implementation discipline; the next review item is code-level enforcement of the allowed conditioning predicate",
    }
}

fn public_proof_forgery_probe() -> PublicProofForgeryProbe {
    let keypair = reference_keygen(1);
    let mut public_account = keypair.account.clone();
    for slot in &mut public_account.slots {
        slot.ideal_masked_witness_response = None;
    }
    let slot_id = 0_u32;
    let message = b"public proof forgery probe";
    let slot_digest = public_account.slots[slot_id as usize].digest;
    let syndrome =
        unpack_q_syndrome_digest(&slot_digest).expect("reference slot digest must be canonical");
    let mask_view = [12_u16, Q - 12];
    let fiber_state = local_state(mask_view);
    let sample = sample_conditioned_quotient_fiber_representative_for_syndrome(
        syndrome,
        fiber_state,
        b"public-forgery-conditioned-fiber-sampler",
        0,
    );
    let proof = LinearSigmaTargetProof {
        proof_commitment: sample_proof_commitment(),
        masked_witness_response: sample.masked_witness_response,
    };
    let frame = challenge_hash_frame(
        &keypair.public_key,
        message,
        slot_id,
        &slot_digest,
        &proof.proof_commitment,
    );
    let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
    let challenge = decode_challenge(&challenge_bytes).expect("challenge frame must decode");
    let public_view = decode_public_key(&keypair.public_key).expect("reference public key decodes");
    let seed: [u8; PAD_SEED_BYTES] =
        semantic_hidden_preimage_from_masked_witness(&proof.masked_witness_response)
            [FIBER_STATE_BYTES..]
            .try_into()
            .unwrap();
    let mut responses = CompactResponses {
        active_free: [[0_i16; LOCAL_FREE]; REPS],
    };
    let mut openings = LocalOpenings {
        active_free: [[0_u16; LOCAL_FREE]; REPS],
    };
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let sign = challenge.sign[rep];
        for coord in 0..LOCAL_FREE {
            let response = add_mod(
                mask_view[coord],
                challenge_mul(sign, public_view.free[block][coord]),
            );
            let centered = if response > Q / 2 {
                response as i32 - Q as i32
            } else {
                response as i32
            };
            responses.active_free[rep][coord] = centered as i16;
        }
        let pad = expand_pad_residues(&seed, block);
        openings.active_free[rep] = [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
    }
    let mut signature = Vec::with_capacity(LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES);
    signature.extend_from_slice(&slot_id.to_le_bytes());
    signature.extend_from_slice(&challenge_bytes);
    signature.extend_from_slice(&encode_responses(&responses));
    signature.extend_from_slice(&encode_openings(&openings));
    signature.extend_from_slice(&encode_linear_sigma_target_proof(&proof));
    let (verify_report, _) =
        reference_verify_with_proof(&keypair.public_key, message, &public_account, &signature);
    let (sigma_equation_report, _) = verify_public_quotient_sigma_equation_signature(
        &keypair.public_key,
        message,
        &public_account,
        &signature,
    );

    PublicProofForgeryProbe {
        attack_name: "public-conditioned-fiber-representative-forgery",
        attack_model: "given only public key bytes, public account slot digest, slot id, and message, solve the public 12x23 quotient equations plus a chosen 2-byte fiber state, then compute openings from the serialized seed",
        uses_private_key_material: false,
        account_hidden_witness_available: false,
        public_key_model: "current all-zero reference public key; this probe is a relation-level warning, not yet a distributional attack on hardened keygen",
        signature_bytes: signature.len(),
        verifier_accepts: verify_report.accepted,
        accepted_reason: verify_report.reason,
        sigma_equation_verifier_accepts: sigma_equation_report.accepted,
        sigma_equation_reason: sigma_equation_report.reason,
        forged_slot_digest_hex: hex_encode(&slot_digest),
        forged_signature_sha256: sha256_hex(&signature),
        chosen_mask_view: mask_view,
        conditioned_sampler_rank: sample.linear_rank,
        sampled_free_residues: sample.sampled_free_residues,
        proof_commitment_bound_into_challenge: true,
        interpretation: "This public forgery succeeds against the old representative verifier but fails the strict linear sigma equation.  The result is useful but not sufficient: the current signer/local-opening relation still treats z as a representative, so the deployable path needs a real sigma prover relation whose responses satisfy both the equation and the opening predicate.",
    }
}

fn public_sigma_equation_forgery_probe() -> PublicSigmaEquationForgeryProbe {
    let keypair = reference_keygen(1);
    let mut public_account = keypair.account.clone();
    for slot in &mut public_account.slots {
        slot.ideal_masked_witness_response = None;
    }
    let slot_id = 0_u32;
    let message = b"public sigma equation forgery probe";
    let slot_digest = public_account.slots[slot_id as usize].digest;
    let target =
        unpack_q_syndrome_digest(&slot_digest).expect("reference slot digest must be canonical");
    let proof_commitment_bytes = sample_proof_commitment();
    let proof_commitment = unpack_q_syndrome_digest(&proof_commitment_bytes)
        .expect("sample proof commitment must be canonical");
    let frame = challenge_hash_frame(
        &keypair.public_key,
        message,
        slot_id,
        &slot_digest,
        &proof_commitment_bytes,
    );
    let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
    let action = challenge_action_vector(&challenge_bytes);
    let sigma_response_syndrome =
        fq_action_add(&proof_commitment, &fq_action_mul(&action, &target));
    let chosen_fiber_state = local_state([0_u16, 0_u16]);
    let sample = sample_conditioned_quotient_fiber_representative_for_syndrome(
        sigma_response_syndrome,
        chosen_fiber_state,
        b"public-sigma-equation-forgery-conditioned-fiber-sampler",
        0,
    );
    let proof = LinearSigmaTargetProof {
        proof_commitment: proof_commitment_bytes,
        masked_witness_response: sample.masked_witness_response,
    };
    let signature = build_linear_sigma_signature_from_proof(
        &keypair.public_key,
        message,
        slot_id,
        &slot_digest,
        &proof,
    )
    .expect("conditioned sigma-equation public transcript should serialize");
    let (verify_report, _) = verify_public_quotient_sigma_equation_signature(
        &keypair.public_key,
        message,
        &public_account,
        &signature,
    );

    PublicSigmaEquationForgeryProbe {
        attack_name: "public-conditioned-sigma-equation-forgery",
        attack_model: "choose a public proof commitment, derive the Fiat-Shamir challenge, compute a + c * digest, then solve the public 12x23 quotient equations with a bounded local fiber state and compute openings from the serialized seed",
        uses_private_key_material: false,
        account_hidden_witness_available: false,
        signature_bytes: signature.len(),
        verifier_accepts: verify_report.accepted,
        accepted_reason: verify_report.reason,
        forged_slot_digest_hex: hex_encode(&slot_digest),
        forged_signature_sha256: sha256_hex(&signature),
        proof_commitment_syndrome: proof_commitment.to_vec(),
        sigma_response_syndrome: sigma_response_syndrome.to_vec(),
        chosen_fiber_state,
        conditioned_sampler_rank: sample.linear_rank,
        sampled_free_residues: sample.sampled_free_residues,
        interpretation: "The strict equation alone is not a knowledge proof for the byte relation: because the response representative can be solved publicly after the challenge is known, an adapted attacker can satisfy both L(z)=a+c*digest and the current local opening predicate without private witness material.",
    }
}

fn conditioned_quotient_fiber_sampler_report() -> ConditionedQuotientFiberSamplerReport {
    let mask_view = [12_u16, Q - 12];
    let base = sample_full_response_masked_witness_for_mask(mask_view);
    let sampler_seed = b"Descender reference conditioned quotient fiber sampler report seed";
    let sample = sample_conditioned_quotient_fiber_representative(&base, sampler_seed, 0);
    let base_digest = full_response_syndrome_digest(&base).unwrap();
    let sample_digest = full_response_syndrome_digest(&sample.masked_witness_response).unwrap();
    ConditionedQuotientFiberSamplerReport {
        sampler_name: "direct-conditioned-quotient-fiber-sampler",
        algorithm: "solve the 12 q-syndrome equations plus residue0=fiber_state and residue1 in the packed low-bit class; sample the remaining 9 free q-residues directly",
        public_syndrome_rows: SIS_FULL_RESPONSE_PUBLIC_ROWS,
        packed_state_constraint_rows: 2,
        total_linear_rows: SIS_FULL_RESPONSE_PUBLIC_ROWS + 2,
        linear_rank: sample.linear_rank,
        sampled_free_residues: sample.sampled_free_residues,
        residue1_allowed_values: residue_class_count(4, 0),
        sample_residue1_value: sample.residue1_value,
        exact_support_formula: "12289^9 * 3073",
        retry_loop_required: false,
        max_retries: 1,
        expected_retries: 1,
        sample_preserves_public_syndrome: sample_digest == base_digest,
        sample_preserves_fiber_state_bytes: sample.masked_witness_response[..FIBER_STATE_BYTES]
            == base[..FIBER_STATE_BYTES],
        sample_is_canonical_packed_q: full_response_vector_from_masked_witness(
            &sample.masked_witness_response,
        )
        .is_some(),
        sample_digest_hex: hex_encode(&sample_digest),
        sample_masked_witness_response_hex: hex_encode(&sample.masked_witness_response),
        timing_policy: "constant loop count for the linear sampler after hash-to-field words; no representative-level rejection loop is used",
        conclusion: "the production quotient-fiber sampler is efficient by construction and realizes the exact counted support without a retry side channel",
    }
}

fn sample_statement_challenge(
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> CompactChallenge {
    let digest = full_response_syndrome_digest(masked_witness_response).unwrap();
    let sample_public_key = vec![0_u8; PK_BYTES];
    let sample_message = b"sample quotient benchmark message";
    let proof_commitment = sample_proof_commitment();
    let frame = challenge_hash_frame(
        &sample_public_key,
        sample_message,
        0,
        &digest,
        &proof_commitment,
    );
    decode_challenge(&hash_frame_to_challenge_bytes(&frame)).unwrap()
}

fn distinct_active_blocks(challenge: &CompactChallenge) -> usize {
    let mut seen = [false; GROUPS];
    for &block in &challenge.active_block {
        seen[block] = true;
    }
    seen.into_iter().filter(|seen| *seen).count()
}

fn accepted_fiber_constraint_report() -> AcceptedFiberConstraintReport {
    let mask_view = [12_u16, Q - 12];
    let hidden_preimage = sample_full_response_masked_witness_for_mask(mask_view);
    let challenge = sample_statement_challenge(&hidden_preimage);
    let distinct_active_blocks = distinct_active_blocks(&challenge);
    let algebraic_hidden_fiber_bits = (SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS)
        * SIS_RESIDUE_BITS
        + SIS_FULL_RESPONSE_SLACK_BITS;
    let fiber_state_constraint_bits = FIBER_STATE_BYTES * 8;
    let pad_output_bits_per_active_block = LOCAL_FREE * SIS_RESIDUE_BITS;
    let active_pad_constraint_bits = distinct_active_blocks * pad_output_bits_per_active_block;
    let max_bits_after_fiber_state_constraint =
        algebraic_hidden_fiber_bits.saturating_sub(fiber_state_constraint_bits);
    let estimated_remaining_fiber_bits_after_acceptance = algebraic_hidden_fiber_bits
        .saturating_sub(fiber_state_constraint_bits + active_pad_constraint_bits);
    let max_distinct_active_blocks_for_128_bits = algebraic_hidden_fiber_bits
        .saturating_sub(fiber_state_constraint_bits + 128)
        / pad_output_bits_per_active_block;
    let max_distinct_active_blocks_for_nonzero_estimate = algebraic_hidden_fiber_bits
        .saturating_sub(fiber_state_constraint_bits + 1)
        / pad_output_bits_per_active_block;

    AcceptedFiberConstraintReport {
        repetitions: REPS,
        distinct_active_blocks,
        algebraic_hidden_fiber_bits,
        fiber_state_constraint_bits,
        max_bits_after_fiber_state_constraint,
        meets_128_after_fiber_state_constraint: max_bits_after_fiber_state_constraint >= 128,
        pad_output_bits_per_active_block,
        active_pad_constraint_bits,
        estimated_remaining_fiber_bits_after_acceptance,
        max_distinct_active_blocks_for_128_bits,
        max_distinct_active_blocks_for_nonzero_estimate,
        interpretation: "For a fixed public statement, an alternate representative must preserve the local fiber state and, under the current representative semantics, the pad outputs on every distinct active block. The 264-byte route has enough room for the 2-byte fiber-state equality, but active pad constraints still overdetermine the fiber unless pad checks are composed outside the hidden representative.",
    }
}

fn kernel_delta_acceptance_probe() -> KernelDeltaAcceptanceProbe {
    let mask_view = [12_u16, Q - 12];
    let base_hidden_preimage = sample_full_response_masked_witness_for_mask(mask_view);
    let base_vector = full_response_vector_from_masked_witness(&base_hidden_preimage).unwrap();
    let base_digest = full_response_syndrome_digest(&base_hidden_preimage).unwrap();
    let base_fiber_state = u16_le(&base_hidden_preimage[..FIBER_STATE_BYTES]) % Q;
    let base_seed: [u8; PAD_SEED_BYTES] = base_hidden_preimage
        [FIBER_STATE_BYTES..FIBER_STATE_BYTES + PAD_SEED_BYTES]
        .try_into()
        .unwrap();
    let challenge = sample_statement_challenge(&base_hidden_preimage);
    let (delta, _) = lll_shortest_kernel_vector();
    let delta_mod_q = vector_to_mod_q(&delta);
    let delta_syndrome = full_response_syndrome_array(&delta_mod_q);
    let tested_multipliers = (-8_i64..=8_i64)
        .filter(|&multiplier| multiplier != 0)
        .collect::<Vec<_>>();
    let mut syndrome_preserved_for_all_tested = delta_syndrome.iter().all(|&entry| entry == 0);
    let mut canonical_packing_for_all_tested = true;
    let mut fiber_state_preserving_multipliers = Vec::new();
    let mut active_pad_preserving_multipliers = Vec::new();
    let mut accepting_multipliers = Vec::new();

    for &multiplier in &tested_multipliers {
        let mut candidate_vector = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
        for idx in 0..SIS_FULL_RESPONSE_RESIDUES {
            candidate_vector[idx] =
                centered_mod_q(base_vector[idx] as i64 + multiplier * delta[idx]);
        }
        let candidate_preimage = masked_witness_from_full_response_vector(&candidate_vector);
        canonical_packing_for_all_tested &=
            full_response_vector_from_masked_witness(&candidate_preimage).is_some();
        syndrome_preserved_for_all_tested &= full_response_syndrome_digest(&candidate_preimage)
            .is_some_and(|digest| digest == base_digest);
        let candidate_fiber_state = u16_le(&candidate_preimage[..FIBER_STATE_BYTES]) % Q;
        let preserves_fiber_state = candidate_fiber_state == base_fiber_state;
        if preserves_fiber_state {
            fiber_state_preserving_multipliers.push(multiplier);
        }
        let candidate_seed: [u8; PAD_SEED_BYTES] = candidate_preimage
            [FIBER_STATE_BYTES..FIBER_STATE_BYTES + PAD_SEED_BYTES]
            .try_into()
            .unwrap();
        let mut preserves_active_pads = true;
        for block in 0..GROUPS {
            if !challenge.active_block.contains(&block) {
                continue;
            }
            preserves_active_pads &= expand_pad_residues(&candidate_seed, block)
                == expand_pad_residues(&base_seed, block);
        }
        if preserves_active_pads {
            active_pad_preserving_multipliers.push(multiplier);
        }
        if preserves_fiber_state && preserves_active_pads {
            accepting_multipliers.push(multiplier);
        }
    }

    KernelDeltaAcceptanceProbe {
        delta_source: "lattice_sis_reduction_probe shortest syndrome-zero vector",
        delta_lee_weight: lee_weight(&delta_mod_q),
        delta_support: support(&delta_mod_q),
        delta_coeff_bound: vector_coeff_bound(&delta),
        tested_multipliers,
        syndrome_preserved_for_all_tested,
        canonical_packing_for_all_tested,
        fiber_state_preserving_multipliers,
        active_pad_preserving_multipliers,
        accepting_multipliers,
        interpretation: "The LLL delta preserves the public q-syndrome, but an attack needs more: for the same public statement it must preserve the decoded fiber state and active-block pad outputs. The tested small multiples do not preserve those acceptance constraints.",
    }
}

fn hidden_fiber_bits_for(response_residues: usize, public_rows: usize) -> usize {
    let response_bits = response_residues * SIS_RESIDUE_BITS;
    let response_bytes = response_bits.div_ceil(8);
    let slack_bits = response_bytes * 8 - response_bits;
    (response_residues - public_rows) * SIS_RESIDUE_BITS + slack_bits
}

fn response_bytes_for(response_residues: usize) -> usize {
    (response_residues * SIS_RESIDUE_BITS).div_ceil(8)
}

fn hiding_route_row(
    route: &'static str,
    public_rows: usize,
    min_constraint_bits: usize,
    status: &'static str,
) -> HidingRouteFrontierRow {
    let mut response_residues = public_rows + 1;
    while public_rows * SIS_RESIDUE_BITS < 128
        || hidden_fiber_bits_for(response_residues, public_rows).saturating_sub(min_constraint_bits)
            < 128
    {
        response_residues += 1;
    }
    let response_bytes = response_bytes_for(response_residues);
    let proof_bytes = LINEAR_SIGMA_PROOF_COMMITMENT_BYTES + response_bytes;
    let transaction_signature_bytes = STATEFUL_TRANSACTION_SIGNATURE_BYTES + proof_bytes;
    let hidden_fiber_bits = hidden_fiber_bits_for(response_residues, public_rows);
    let residual_hiding_bits = hidden_fiber_bits.saturating_sub(min_constraint_bits);

    HidingRouteFrontierRow {
        route,
        public_rows,
        response_residues,
        response_bytes,
        masked_witness_response_bytes: response_bytes,
        proof_bytes,
        transaction_signature_bytes,
        hidden_fiber_bits,
        modeled_constraint_bits: min_constraint_bits,
        residual_hiding_bits,
        meets_128_hiding: residual_hiding_bits >= 128,
        byte_delta_from_247: transaction_signature_bytes as isize - 247,
        status,
    }
}

fn hiding_route_frontier() -> Vec<HidingRouteFrontierRow> {
    let accepted = accepted_fiber_constraint_report();
    vec![
        hiding_route_row(
            "decoupled quotient witness",
            SIS_FULL_RESPONSE_PUBLIC_ROWS,
            0,
            "Keeps the 247-byte target if pad/fiber-state checks move outside the hidden quotient representative; needs a new binding/composition proof.",
        ),
        hiding_route_row(
            "larger fiber, keep local fiber-state equality",
            SIS_FULL_RESPONSE_PUBLIC_ROWS,
            FIBER_STATE_BYTES * 8,
            "Adds enough hidden q-capacity to survive the 16-bit fiber-state equality while keeping 128-bit hiding.",
        ),
        hiding_route_row(
            "larger fiber, sample active-pad constraints",
            SIS_FULL_RESPONSE_PUBLIC_ROWS,
            accepted.fiber_state_constraint_bits + accepted.active_pad_constraint_bits,
            "Models the current representative semantics under the sample challenge; it becomes much larger because active pad outputs overdetermine the fiber.",
        ),
        hiding_route_row(
            "larger fiber, worst-case active-pad constraints",
            SIS_FULL_RESPONSE_PUBLIC_ROWS,
            FIBER_STATE_BYTES * 8 + GROUPS * LOCAL_FREE * SIS_RESIDUE_BITS,
            "Models all 16 active blocks constrained; this is the conservative keep-current-semantics route.",
        ),
    ]
}

fn matrix_determinant_mod_q(matrix: &[[u16; SIS_SEED_RESIDUES]; SIS_SYNDROME_RESIDUES]) -> u16 {
    assert_eq!(SIS_SEED_RESIDUES, SIS_SYNDROME_RESIDUES);
    let mut work = *matrix;
    let mut det = 1_u16;
    let mut odd_swaps = false;
    for col in 0..SIS_SEED_RESIDUES {
        let Some(pivot) = (col..SIS_SYNDROME_RESIDUES).find(|&row| work[row][col] != 0) else {
            return 0;
        };
        if pivot != col {
            work.swap(col, pivot);
            odd_swaps = !odd_swaps;
        }
        let pivot_value = work[col][col];
        det = ((det as u32 * pivot_value as u32) % Q as u32) as u16;
        let inv = mod_inv(pivot_value);
        for row in (col + 1)..SIS_SYNDROME_RESIDUES {
            let factor = ((work[row][col] as u32 * inv as u32) % Q as u32) as u16;
            for idx in col..SIS_SEED_RESIDUES {
                work[row][idx] = sub_mul_mod(work[row][idx], factor, work[col][idx]);
            }
        }
    }
    if odd_swaps && det != 0 { Q - det } else { det }
}

fn pad_frame(block: usize) -> [u8; PAD_FRAME_BYTES] {
    assert!(block < GROUPS);
    let mut out = [0_u8; PAD_FRAME_BYTES];
    out[..PAD_DOMAIN.len()].copy_from_slice(PAD_DOMAIN);
    out[PAD_DOMAIN.len()] = block as u8;
    out
}

fn pad_prf_block(
    seed: &[u8; PAD_SEED_BYTES],
    frame: &[u8; PAD_FRAME_BYTES],
    counter: u32,
) -> [u8; DIGEST_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(frame);
    hasher.update(counter.to_le_bytes());
    hasher.finalize().into()
}

fn expand_pad_residues(seed: &[u8; PAD_SEED_BYTES], block: usize) -> [u16; LOCAL_FREE] {
    let frame = pad_frame(block);
    let rejection_cutoff = (u16::MAX as u32 + 1) / Q as u32 * Q as u32;
    let mut out = [0_u16; LOCAL_FREE];
    let mut filled = 0_usize;
    let mut counter = 0_u32;
    while filled < LOCAL_FREE {
        let digest = pad_prf_block(seed, &frame, counter);
        counter = counter
            .checked_add(1)
            .expect("pad rejection sampler exhausted u32 counter space");
        for chunk in digest.chunks_exact(2) {
            let candidate = u16_le(chunk) as u32;
            if candidate < rejection_cutoff {
                out[filled] = (candidate % Q as u32) as u16;
                filled += 1;
                if filled == LOCAL_FREE {
                    break;
                }
            }
        }
    }
    out
}

fn u16_le(bytes: &[u8]) -> u16 {
    u16::from_le_bytes(bytes.try_into().unwrap())
}

fn mod_q(x: i64) -> u16 {
    x.rem_euclid(Q as i64) as u16
}

fn negate_mod(value: u16) -> u16 {
    if value == 0 { 0 } else { Q - value }
}

fn challenge_mul(coeff: i8, value: u16) -> u16 {
    match coeff {
        -1 => negate_mod(value),
        0 => 0,
        1 => value,
        other => panic!("invalid compact challenge coefficient {other}"),
    }
}

fn sub_mod(lhs: u16, rhs: u16) -> u16 {
    mod_q(lhs as i64 - rhs as i64)
}

fn add_mod(lhs: u16, rhs: u16) -> u16 {
    ((lhs as u32 + rhs as u32) % Q as u32) as u16
}

fn local_state(mask_view: [u16; LOCAL_FREE]) -> u16 {
    ((mask_view[0] as u32 + LOCAL_SYNDROME_SLOPE as u32 * mask_view[1] as u32) % Q as u32) as u16
}

/// Center a residue in [0, Q) into the signed window (-Q/2, Q/2].
fn centered_residue(value: u16) -> i32 {
    if value > Q / 2 {
        value as i32 - Q as i32
    } else {
        value as i32
    }
}

/// True iff a centered coordinate lies inside the in-box response window
/// [-RESPONSE_BOUND, RESPONSE_BOUND].
fn centered_coord_in_box(centered: i32) -> bool {
    (-(RESPONSE_BOUND as i32)..=RESPONSE_BOUND as i32).contains(&centered)
}

/// HIGH-1 SUF-CMA malleability fix — canonical fundamental domain for the
/// in-box fiber.
///
/// `local_state(v) = v[0] + 50*v[1] (mod Q)` has, over the in-box square
/// `v[i] in [-RESPONSE_BOUND, RESPONSE_BOUND]`, an integer kernel generated by
/// the lattice vector `(LOCAL_SYNDROME_SLOPE, -1) = (50, -1)`: shifting a rep by
/// this vector preserves the fiber state. An exhaustive check (q=12289,
/// RESPONSE_BOUND=48, slope=50) shows every coset has at most two in-box reps,
/// always differing by exactly one step of (50, -1).
///
/// We pick the canonical rep to be the unique one for which the sibling obtained
/// by shifting one further step in the +(50, -1) direction leaves the box — i.e.
/// the rep with the larger first coordinate. This matches the rep
/// `[12, Q-12] = (12, -12)` that every honest signer already emits
/// (`reference_sign_with_proof`, hence `production_sign`/KAT, and keygen all use
/// `[12, Q-12]`). Verified exhaustively: each reachable fiber state has exactly
/// one canonical in-box rep under this rule.
fn mask_view_is_canonical(mask_view: [u16; LOCAL_FREE]) -> bool {
    let c0 = centered_residue(mask_view[0]);
    let c1 = centered_residue(mask_view[1]);
    if !centered_coord_in_box(c0) || !centered_coord_in_box(c1) {
        return false;
    }
    // Canonical iff one more step in the +(50, -1) kernel direction leaves the box.
    let sibling0 = c0 + LOCAL_SYNDROME_SLOPE as i32;
    let sibling1 = c1 - 1;
    !(centered_coord_in_box(sibling0) && centered_coord_in_box(sibling1))
}

fn centered_response_value(value: u16) -> Option<i16> {
    let centered = if value > Q / 2 {
        value as i32 - Q as i32
    } else {
        value as i32
    };
    (-(RESPONSE_BOUND as i32)..=RESPONSE_BOUND as i32)
        .contains(&centered)
        .then_some(centered as i16)
}

fn mask_view_for_fiber_state(fiber_state: u16) -> Option<[u16; LOCAL_FREE]> {
    // Return the unique CANONICAL in-box rep for this fiber state (HIGH-1 fix):
    // the verifier rejects non-canonical reps, so the signer must emit the
    // canonical one. This agrees with the hardcoded `[12, Q-12]` rep used by all
    // other signing/keygen paths.
    for lhs in -(RESPONSE_BOUND as i16)..=RESPONSE_BOUND {
        for rhs in -(RESPONSE_BOUND as i16)..=RESPONSE_BOUND {
            let mask_view = [mod_q(lhs as i64), mod_q(rhs as i64)];
            if local_state(mask_view) == fiber_state && mask_view_is_canonical(mask_view) {
                return Some(mask_view);
            }
        }
    }
    None
}

fn build_local_delta_response_openings_for_masked_response(
    public_key: &[u8],
    challenge: &CompactChallenge,
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Result<(CompactResponses, LocalOpenings), String> {
    let semantic_hidden_preimage =
        semantic_hidden_preimage_from_masked_witness(masked_witness_response);
    let fiber_state = u16_le(&semantic_hidden_preimage[..FIBER_STATE_BYTES]) % Q;
    let mask_view = mask_view_for_fiber_state(fiber_state)
        .ok_or_else(|| format!("no bounded local mask view realizes fiber state {fiber_state}"))?;
    let seed: [u8; PAD_SEED_BYTES] = semantic_hidden_preimage[FIBER_STATE_BYTES..]
        .try_into()
        .unwrap();
    let public_view = decode_public_key(public_key)?;
    let mut responses = CompactResponses {
        active_free: [[0_i16; LOCAL_FREE]; REPS],
    };
    let mut openings = LocalOpenings {
        active_free: [[0_u16; LOCAL_FREE]; REPS],
    };
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let sign = challenge.sign[rep];
        for coord in 0..LOCAL_FREE {
            let response = add_mod(
                mask_view[coord],
                challenge_mul(sign, public_view.free[block][coord]),
            );
            responses.active_free[rep][coord] = centered_response_value(response)
                .ok_or_else(|| format!("reference response out of bound in round {rep}"))?;
        }
        let pad = expand_pad_residues(&seed, block);
        openings.active_free[rep] = [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
    }
    Ok((responses, openings))
}

fn fiber_state_bytes(state: u16) -> [u8; FIBER_STATE_BYTES] {
    state.to_le_bytes()
}

fn sample_full_response_masked_witness_for_mask(
    mask_view: [u16; LOCAL_FREE],
) -> [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES] {
    sample_full_response_masked_witness_for_mask_stream(mask_view, 0)
}

fn sample_full_response_masked_witness_for_mask_stream(
    mask_view: [u16; LOCAL_FREE],
    stream: u64,
) -> [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES] {
    let fiber_state = fiber_state_bytes(local_state(mask_view));
    for counter in 0_u32.. {
        let mut hasher = Sha256::new();
        hasher.update(b"Descender.Signature.q12289.local_delta.full_response_seed.v1");
        hasher.update(stream.to_le_bytes());
        hasher.update(counter.to_le_bytes());
        let seed: [u8; PAD_SEED_BYTES] = hasher.finalize().into();
        let hidden_preimage = encode_hidden_preimage(fiber_state, seed);
        let mut masked_witness = [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES];
        masked_witness[..HIDDEN_PREIMAGE_BYTES].copy_from_slice(&hidden_preimage);
        let mut extra_hasher = Sha256::new();
        extra_hasher.update(b"Descender.Signature.q12289.local_delta.full_response_extra.v1");
        extra_hasher.update(stream.to_le_bytes());
        extra_hasher.update(counter.to_le_bytes());
        let extra: [u8; DIGEST_BYTES] = extra_hasher.finalize().into();
        masked_witness[HIDDEN_PREIMAGE_BYTES..]
            .copy_from_slice(&extra[..LINEAR_SIGMA_MASKED_WITNESS_BYTES - HIDDEN_PREIMAGE_BYTES]);
        masked_witness[LINEAR_SIGMA_MASKED_WITNESS_BYTES - 1] &=
            (1_u8 << (8 - SIS_FULL_RESPONSE_SLACK_BITS)) - 1;
        if full_response_vector_from_masked_witness(&masked_witness).is_some() {
            return masked_witness;
        }
    }
    unreachable!("unbounded canonical full-response masked-witness sampler loop")
}

fn uniform_mod_from_hash(domain: &[u8], seed: &[u8], stream: u64, index: u64, modulus: u16) -> u16 {
    let cutoff = (u16::MAX as u32 + 1) / modulus as u32 * modulus as u32;
    for counter in 0_u32.. {
        let mut hasher = Sha256::new();
        hasher.update(domain);
        hasher.update(seed);
        hasher.update(stream.to_le_bytes());
        hasher.update(index.to_le_bytes());
        hasher.update(counter.to_le_bytes());
        let digest: [u8; DIGEST_BYTES] = hasher.finalize().into();
        for chunk in digest.chunks_exact(2) {
            let candidate = u16_le(chunk) as u32;
            if candidate < cutoff {
                return (candidate % modulus as u32) as u16;
            }
        }
    }
    unreachable!("unbounded uniform hash sampler loop")
}

fn residue1_allowed_value(seed: &[u8], stream: u64) -> u16 {
    let index = uniform_mod_from_hash(
        b"Descender.Signature.q12289.local_delta.conditioned_fiber.residue1.v1",
        seed,
        stream,
        0,
        residue_class_count(4, 0) as u16,
    );
    4 * index
}

fn sample_conditioned_quotient_fiber_representative(
    base: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
    sampler_seed: &[u8],
    stream: u64,
) -> ConditionedFiberSample {
    let base_vector = full_response_vector_from_masked_witness(base)
        .expect("base representative must be canonical packed q");
    let syndrome = full_response_syndrome_array(&base_vector);
    let fiber_state = u16_le(&base[..FIBER_STATE_BYTES]);
    sample_conditioned_quotient_fiber_representative_for_syndrome(
        syndrome,
        fiber_state,
        sampler_seed,
        stream,
    )
}

fn sample_conditioned_quotient_fiber_representative_for_syndrome(
    syndrome: [u16; SIS_FULL_RESPONSE_PUBLIC_ROWS],
    fiber_state: u16,
    sampler_seed: &[u8],
    stream: u64,
) -> ConditionedFiberSample {
    let residue1_value = residue1_allowed_value(sampler_seed, stream);

    let mut matrix = Vec::with_capacity(SIS_FULL_RESPONSE_PUBLIC_ROWS + 2);
    let mut rhs = Vec::with_capacity(SIS_FULL_RESPONSE_PUBLIC_ROWS + 2);
    for row in 0..SIS_FULL_RESPONSE_PUBLIC_ROWS {
        let mut equation = vec![0_u16; SIS_FULL_RESPONSE_RESIDUES];
        for (col, entry) in equation.iter_mut().enumerate() {
            *entry = full_response_matrix_entry(row, col);
        }
        matrix.push(equation);
        rhs.push(syndrome[row]);
    }
    let mut fixed_residue0 = vec![0_u16; SIS_FULL_RESPONSE_RESIDUES];
    fixed_residue0[0] = 1;
    matrix.push(fixed_residue0);
    rhs.push(fiber_state);
    let mut fixed_residue1 = vec![0_u16; SIS_FULL_RESPONSE_RESIDUES];
    fixed_residue1[1] = 1;
    matrix.push(fixed_residue1);
    rhs.push(residue1_value);

    let (rref, rref_rhs, pivot_cols) = rref_mod_q_with_rhs(matrix, rhs);
    assert_eq!(pivot_cols.len(), SIS_FULL_RESPONSE_PUBLIC_ROWS + 2);
    let free_cols = (0..SIS_FULL_RESPONSE_RESIDUES)
        .filter(|col| !pivot_cols.contains(col))
        .collect::<Vec<_>>();
    assert_eq!(free_cols.len(), 9);

    let mut solution = [0_u16; SIS_FULL_RESPONSE_RESIDUES];
    for (idx, &col) in free_cols.iter().enumerate() {
        solution[col] = uniform_mod_from_hash(
            b"Descender.Signature.q12289.local_delta.conditioned_fiber.free.v1",
            sampler_seed,
            stream,
            idx as u64,
            Q,
        );
    }
    for (row, &pivot_col) in pivot_cols.iter().enumerate().rev() {
        let mut value = rref_rhs[row];
        for &free_col in &free_cols {
            value = sub_mul_mod(value, rref[row][free_col], solution[free_col]);
        }
        solution[pivot_col] = value;
    }

    let masked_witness_response = masked_witness_from_full_response_vector(&solution);
    debug_assert_eq!(full_response_syndrome_array(&solution), syndrome);
    debug_assert_eq!(
        u16_le(&masked_witness_response[..FIBER_STATE_BYTES]),
        fiber_state
    );
    ConditionedFiberSample {
        masked_witness_response,
        linear_rank: pivot_cols.len(),
        sampled_free_residues: free_cols.len(),
        residue1_value,
    }
}

fn pad_residues(pad: &[u8; LOCAL_OPENING_BYTES_PER_REP]) -> [u16; LOCAL_FREE] {
    [u16_le(&pad[0..2]) % Q, u16_le(&pad[2..4]) % Q]
}

fn encode_residue_pair(values: [u16; LOCAL_FREE]) -> [u8; LOCAL_OPENING_BYTES_PER_REP] {
    let mut out = [0_u8; LOCAL_OPENING_BYTES_PER_REP];
    out[0..2].copy_from_slice(&values[0].to_le_bytes());
    out[2..4].copy_from_slice(&values[1].to_le_bytes());
    out
}

fn opening_bytes(
    mask_view: [u16; LOCAL_FREE],
    pad: [u16; LOCAL_FREE],
) -> [u8; LOCAL_OPENING_BYTES_PER_REP] {
    encode_residue_pair([add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])])
}

fn expand_pad(seed: &[u8; PAD_SEED_BYTES], block: usize) -> [u8; LOCAL_OPENING_BYTES_PER_REP] {
    encode_residue_pair(expand_pad_residues(seed, block))
}

fn challenge_hash_frame(
    public_key: &[u8],
    message: &[u8],
    slot_id: u32,
    digest: &[u8; DIGEST_BYTES],
    proof_commitment: &[u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
) -> Vec<u8> {
    prodref::challenge_hash_frame(public_key, message, slot_id, digest, proof_commitment)
}

#[allow(dead_code)]
fn legacy_stateful_challenge_hash_frame(
    public_key: &[u8],
    message: &[u8],
    slot_id: u32,
    digest: &[u8; DIGEST_BYTES],
) -> Vec<u8> {
    prodref::legacy_stateful_challenge_hash_frame(public_key, message, slot_id, digest)
}

fn hash_frame_to_challenge_bytes(frame: &[u8]) -> [u8; CHALLENGE_BYTES] {
    prodref::hash_frame_to_challenge_bytes(frame)
}

fn challenge_action_vector(challenge: &[u8; CHALLENGE_BYTES]) -> [u16; CHALLENGE_ACTION_RESIDUES] {
    let mut out = [0_u16; CHALLENGE_ACTION_RESIDUES];
    for (idx, value) in out.iter_mut().enumerate() {
        *value = uniform_mod_from_hash(
            b"Descender.Signature.q12289.local_delta.challenge_action.fq12.v1",
            challenge,
            0,
            idx as u64,
            Q,
        );
    }
    out
}

fn fq_action_normalize(poly: Vec<u16>) -> [u16; CHALLENGE_ACTION_RESIDUES] {
    assert!(poly.len() <= CHALLENGE_ACTION_RESIDUES);
    let mut out = [0_u16; CHALLENGE_ACTION_RESIDUES];
    for (idx, value) in poly.into_iter().enumerate() {
        out[idx] = value;
    }
    out
}

fn fq_action_one() -> [u16; CHALLENGE_ACTION_RESIDUES] {
    let mut out = [0_u16; CHALLENGE_ACTION_RESIDUES];
    out[0] = 1;
    out
}

fn fq_action_sub(
    lhs: &[u16; CHALLENGE_ACTION_RESIDUES],
    rhs: &[u16; CHALLENGE_ACTION_RESIDUES],
) -> [u16; CHALLENGE_ACTION_RESIDUES] {
    let mut out = [0_u16; CHALLENGE_ACTION_RESIDUES];
    for idx in 0..CHALLENGE_ACTION_RESIDUES {
        out[idx] = sub_mod(lhs[idx], rhs[idx]);
    }
    out
}

fn fq_action_add(
    lhs: &[u16; CHALLENGE_ACTION_RESIDUES],
    rhs: &[u16; CHALLENGE_ACTION_RESIDUES],
) -> [u16; CHALLENGE_ACTION_RESIDUES] {
    let mut out = [0_u16; CHALLENGE_ACTION_RESIDUES];
    for idx in 0..CHALLENGE_ACTION_RESIDUES {
        out[idx] = add_mod(lhs[idx], rhs[idx]);
    }
    out
}

fn fq_action_mul(
    lhs: &[u16; CHALLENGE_ACTION_RESIDUES],
    rhs: &[u16; CHALLENGE_ACTION_RESIDUES],
) -> [u16; CHALLENGE_ACTION_RESIDUES] {
    fq_action_normalize(poly_mul_mod(
        lhs,
        rhs,
        &CHALLENGE_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH,
    ))
}

fn solve_square_mod_q(mut matrix: Vec<Vec<u16>>, rhs: Vec<u16>) -> Option<Vec<u16>> {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, Vec::len);
    if rows != cols || rhs.len() != rows {
        return None;
    }
    let (rref, rref_rhs, pivot_cols) = rref_mod_q_with_rhs(std::mem::take(&mut matrix), rhs);
    if pivot_cols.len() != cols {
        return None;
    }
    let mut solution = vec![0_u16; cols];
    for (row, &col) in pivot_cols.iter().enumerate() {
        solution[col] = rref_rhs[row];
    }
    for row in pivot_cols.len()..rref.len() {
        if rref[row].iter().all(|&entry| entry == 0) && rref_rhs[row] != 0 {
            return None;
        }
    }
    Some(solution)
}

fn fq_action_inverse(
    value: &[u16; CHALLENGE_ACTION_RESIDUES],
) -> Option<[u16; CHALLENGE_ACTION_RESIDUES]> {
    if value.iter().all(|&entry| entry == 0) {
        return None;
    }
    let mut multiplication_matrix =
        vec![vec![0_u16; CHALLENGE_ACTION_RESIDUES]; CHALLENGE_ACTION_RESIDUES];
    for col in 0..CHALLENGE_ACTION_RESIDUES {
        let mut basis = [0_u16; CHALLENGE_ACTION_RESIDUES];
        basis[col] = 1;
        let product = fq_action_mul(value, &basis);
        for row in 0..CHALLENGE_ACTION_RESIDUES {
            multiplication_matrix[row][col] = product[row];
        }
    }
    let rhs = fq_action_one().to_vec();
    let solution = solve_square_mod_q(multiplication_matrix, rhs)?;
    Some(fq_action_normalize(solution))
}

fn pad_sampler_certificate(seed: &[u8; PAD_SEED_BYTES], block: usize) -> PadSamplerCertificate {
    let frame = pad_frame(block);
    let digest0 = pad_prf_block(seed, &frame, 0);
    let sample_residues = expand_pad_residues(seed, block);
    PadSamplerCertificate {
        domain: std::str::from_utf8(PAD_DOMAIN).unwrap(),
        domain_hex: hex_encode(PAD_DOMAIN),
        frame_bytes: PAD_FRAME_BYTES,
        counter_encoding: "u32 little-endian",
        rejection_cutoff: (u16::MAX as u32 + 1) / Q as u32 * Q as u32,
        sample_block: block,
        sample_frame_hex: hex_encode(&frame),
        sample_seed_hex: hex_encode(seed),
        sample_counter0_digest_hex: hex_encode(&digest0),
        sample_residues,
        sample_residues_lt_modulus: sample_residues.iter().all(|&value| value < Q),
    }
}

fn fiat_shamir_frame_certificate(
    public_key: &[u8],
    message: &[u8],
    slot_id: u32,
    digest: &[u8; DIGEST_BYTES],
    proof_commitment: &[u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
) -> FiatShamirFrameCertificate {
    let frame = challenge_hash_frame(public_key, message, slot_id, digest, proof_commitment);
    let challenge = hash_frame_to_challenge_bytes(&frame);
    FiatShamirFrameCertificate {
        domain: CHALLENGE_HASH_DOMAIN,
        domain_hex: hex_encode(CHALLENGE_HASH_DOMAIN.as_bytes()),
        frame_labels: vec![
            "domain",
            "profile",
            "q",
            "groups",
            "local_pivot",
            "local_free",
            "repetitions",
            "slot_id",
            "public_key",
            "message",
            "slot_digest",
            "proof_commitment",
        ],
        binds_proof_commitment: true,
        sample_frame_bytes: frame.len(),
        sample_frame_sha256: sha256_hex(&frame),
        sample_challenge_hex: hex_encode(&challenge),
        challenge_bytes: CHALLENGE_BYTES,
        algebraic_challenge_bytes: ALGEBRAIC_CHALLENGE_BYTES,
        hardening_challenge_bytes: CHALLENGE_HARDENING_BYTES,
        algebraic_last_byte_mask: ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK,
        algebraic_entropy_bits: ALGEBRAIC_CHALLENGE_ENTROPY_BITS,
        transcript_entropy_bits: CHALLENGE_ENTROPY_BITS,
        excludes_response_and_opening_bytes: true,
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex_encode(&Sha256::digest(bytes))
}

fn reject_verify_with_expected(
    proof_model: &'static str,
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
    expected_signature_bytes: usize,
    slot: Option<u32>,
    slot_was_unused: Option<bool>,
    claimed: Option<String>,
    recomputed: Option<String>,
    frame: Option<&[u8]>,
    reason: String,
) -> StatefulVerifyReport {
    StatefulVerifyReport {
        accepted: false,
        profile: PROFILE,
        proof_model,
        public_key_bytes: public_key.len(),
        message_bytes: message.len(),
        signature_bytes: signature.len(),
        expected_signature_bytes,
        slot,
        slot_was_unused,
        slot_consumed: false,
        challenge_bytes: CHALLENGE_BYTES,
        response_bytes: ACTIVE_RESPONSE_BYTES,
        opening_bytes: LOCAL_OPENING_BYTES,
        hash_frame_bytes: frame.map(<[u8]>::len),
        hash_frame_sha256: frame.map(sha256_hex),
        claimed_challenge_hex: claimed,
        recomputed_challenge_hex: recomputed,
        reason,
    }
}

#[allow(dead_code)]
fn reject_verify(
    proof_model: &'static str,
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
    slot: Option<u32>,
    slot_was_unused: Option<bool>,
    claimed: Option<String>,
    recomputed: Option<String>,
    frame: Option<&[u8]>,
    reason: String,
) -> StatefulVerifyReport {
    reject_verify_with_expected(
        proof_model,
        public_key,
        message,
        signature,
        STATEFUL_TRANSACTION_SIGNATURE_BYTES,
        slot,
        slot_was_unused,
        claimed,
        recomputed,
        frame,
        reason,
    )
}

#[allow(dead_code)]
fn verify_stateful_signature_with_hidden_preimage_relation(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
    proof_model: &'static str,
    ok_reason: &'static str,
    digest_matches_hidden: fn(
        &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
        &[u8; DIGEST_BYTES],
    ) -> bool,
    digest_mismatch_reason: &'static str,
) -> (StatefulVerifyReport, AccountState) {
    let mut next_account = account.clone();
    let claimed = (signature.len() >= SLOT_ID_BYTES + CHALLENGE_BYTES)
        .then(|| hex_encode(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]));
    if signature.len() != STATEFUL_TRANSACTION_SIGNATURE_BYTES {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                None,
                None,
                claimed,
                None,
                None,
                format!(
                    "signature has {} bytes, expected {STATEFUL_TRANSACTION_SIGNATURE_BYTES}",
                    signature.len()
                ),
            ),
            next_account,
        );
    }
    if public_key.len() != PK_BYTES {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                None,
                None,
                claimed,
                None,
                None,
                format!(
                    "public key has {} bytes, expected {PK_BYTES}",
                    public_key.len()
                ),
            ),
            next_account,
        );
    }

    let slot_id = u32::from_le_bytes(signature[..SLOT_ID_BYTES].try_into().unwrap());
    let Some(slot) = account.slots.get(slot_id as usize) else {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                Some(slot_id),
                None,
                claimed,
                None,
                None,
                "slot id is out of range".to_string(),
            ),
            next_account,
        );
    };
    if !slot.unused {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                Some(slot_id),
                Some(false),
                claimed,
                None,
                None,
                "slot is already consumed".to_string(),
            ),
            next_account,
        );
    }
    let frame = legacy_stateful_challenge_hash_frame(public_key, message, slot_id, &slot.digest);
    let recomputed = hash_frame_to_challenge_bytes(&frame);
    let recomputed_hex = hex_encode(&recomputed);
    if signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES] != recomputed {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                Some(slot_id),
                Some(true),
                claimed,
                Some(recomputed_hex),
                Some(&frame),
                "challenge mismatch".to_string(),
            ),
            next_account,
        );
    }

    let challenge =
        match decode_challenge(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]) {
            Ok(challenge) => challenge,
            Err(err) => {
                return (
                    reject_verify(
                        proof_model,
                        public_key,
                        message,
                        signature,
                        Some(slot_id),
                        Some(true),
                        claimed,
                        Some(recomputed_hex),
                        Some(&frame),
                        err,
                    ),
                    next_account,
                );
            }
        };
    let public_view = match decode_public_key(public_key) {
        Ok(public_view) => public_view,
        Err(err) => {
            return (
                reject_verify(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    err,
                ),
                next_account,
            );
        }
    };
    let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
    let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
    let responses = match decode_responses(&signature[response_start..opening_start]) {
        Ok(responses) => responses,
        Err(err) => {
            return (
                reject_verify(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    err,
                ),
                next_account,
            );
        }
    };
    let openings = match decode_openings(&signature[opening_start..]) {
        Ok(openings) => openings,
        Err(err) => {
            return (
                reject_verify(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    err,
                ),
                next_account,
            );
        }
    };
    let Some(masked_witness_response) = slot.ideal_masked_witness_response else {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                Some(slot_id),
                Some(true),
                claimed,
                Some(recomputed_hex),
                Some(&frame),
                "missing ideal masked witness response for relation check".to_string(),
            ),
            next_account,
        );
    };
    if !digest_matches_hidden(&masked_witness_response, &slot.digest) {
        return (
            reject_verify(
                proof_model,
                public_key,
                message,
                signature,
                Some(slot_id),
                Some(true),
                claimed,
                Some(recomputed_hex),
                Some(&frame),
                digest_mismatch_reason.to_string(),
            ),
            next_account,
        );
    }
    let semantic_hidden_preimage =
        semantic_hidden_preimage_from_masked_witness(&masked_witness_response);
    let fiber_state = u16_le(&semantic_hidden_preimage[..FIBER_STATE_BYTES]) % Q;
    let seed: [u8; PAD_SEED_BYTES] = semantic_hidden_preimage[FIBER_STATE_BYTES..]
        .try_into()
        .unwrap();
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let sign = challenge.sign[rep];
        let mut mask_view = [0_u16; LOCAL_FREE];
        for (coord, value) in mask_view.iter_mut().enumerate() {
            *value = sub_mod(
                mod_q(responses.active_free[rep][coord] as i64),
                challenge_mul(sign, public_view.free[block][coord]),
            );
        }
        if local_state(mask_view) != fiber_state {
            return (
                reject_verify(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    format!("local fiber-state mismatch in round {rep}"),
                ),
                next_account,
            );
        }
        // HIGH-1 SUF-CMA malleability fix: enforce the canonical fundamental
        // domain so each (challenge, fiber) admits exactly one accepting in-box
        // opening. Reject the non-canonical kernel sibling.
        if !mask_view_is_canonical(mask_view) {
            return (
                reject_verify(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    format!("non-canonical local mask view in round {rep}"),
                ),
                next_account,
            );
        }
        let pad = expand_pad_residues(&seed, block);
        let expected_opening = [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
        if openings.active_free[rep] != expected_opening {
            return (
                reject_verify(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    format!("local opening relation mismatch in round {rep}"),
                ),
                next_account,
            );
        }
    }

    if let Some(slot) = next_account.slots.get_mut(slot_id as usize) {
        slot.unused = false;
    }
    (
        StatefulVerifyReport {
            accepted: true,
            profile: PROFILE,
            proof_model,
            public_key_bytes: public_key.len(),
            message_bytes: message.len(),
            signature_bytes: signature.len(),
            expected_signature_bytes: STATEFUL_TRANSACTION_SIGNATURE_BYTES,
            slot: Some(slot_id),
            slot_was_unused: Some(true),
            slot_consumed: true,
            challenge_bytes: CHALLENGE_BYTES,
            response_bytes: ACTIVE_RESPONSE_BYTES,
            opening_bytes: LOCAL_OPENING_BYTES,
            hash_frame_bytes: Some(frame.len()),
            hash_frame_sha256: Some(sha256_hex(&frame)),
            claimed_challenge_hex: claimed,
            recomputed_challenge_hex: Some(recomputed_hex),
            reason: ok_reason.to_string(),
        },
        next_account,
    )
}

#[allow(dead_code)]
fn commitment_digest_matches_hidden(
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
    digest: &[u8; DIGEST_BYTES],
) -> bool {
    let hidden_preimage = semantic_hidden_preimage_from_masked_witness(masked_witness_response);
    commitment_digest(&hidden_preimage) == *digest
}

fn full_response_syndrome_digest_matches_hidden(
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
    digest: &[u8; DIGEST_BYTES],
) -> bool {
    full_response_syndrome_digest(masked_witness_response)
        .is_some_and(|candidate| candidate == *digest)
}

fn verify_local_delta_relation_with_masked_witness(
    public_key: &[u8],
    challenge: &CompactChallenge,
    responses: &CompactResponses,
    openings: &LocalOpenings,
    masked_witness_response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Result<(), String> {
    let public_view = decode_public_key(public_key)?;
    let semantic_hidden_preimage =
        semantic_hidden_preimage_from_masked_witness(masked_witness_response);
    let fiber_state = u16_le(&semantic_hidden_preimage[..FIBER_STATE_BYTES]) % Q;
    let seed: [u8; PAD_SEED_BYTES] = semantic_hidden_preimage[FIBER_STATE_BYTES..]
        .try_into()
        .unwrap();
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let sign = challenge.sign[rep];
        let mut mask_view = [0_u16; LOCAL_FREE];
        for (coord, value) in mask_view.iter_mut().enumerate() {
            *value = sub_mod(
                mod_q(responses.active_free[rep][coord] as i64),
                challenge_mul(sign, public_view.free[block][coord]),
            );
        }
        if local_state(mask_view) != fiber_state {
            return Err(format!("local fiber-state mismatch in round {rep}"));
        }
        // HIGH-1 SUF-CMA malleability fix: enforce the canonical fundamental
        // domain so each (challenge, fiber) admits exactly one accepting in-box
        // opening. Reject the non-canonical kernel sibling.
        if !mask_view_is_canonical(mask_view) {
            return Err(format!("non-canonical local mask view in round {rep}"));
        }
        let pad = expand_pad_residues(&seed, block);
        let expected_opening = [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
        if openings.active_free[rep] != expected_opening {
            return Err(format!("local opening relation mismatch in round {rep}"));
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn verify_ideal_stateful_signature(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
) -> (StatefulVerifyReport, AccountState) {
    verify_stateful_signature_with_hidden_preimage_relation(
        public_key,
        message,
        account,
        signature,
        "legacy-sha-hidden-preimage-opening-check",
        "ok-stateful-local-delta-legacy-sha-relation",
        commitment_digest_matches_hidden,
        "slot digest does not match SHA hidden preimage commitment",
    )
}

#[allow(dead_code)]
fn verify_quotient_stateful_signature(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
) -> (StatefulVerifyReport, AccountState) {
    verify_stateful_signature_with_hidden_preimage_relation(
        public_key,
        message,
        account,
        signature,
        "full-response-packed-sis-quotient-opening-check",
        "ok-stateful-local-delta-full-response-quotient-relation",
        full_response_syndrome_digest_matches_hidden,
        "slot digest does not match full-response q-syndrome hidden preimage",
    )
}

#[derive(Clone, Copy)]
enum PublicQuotientProofVerifierMode {
    Representative,
    SigmaEquation,
}

impl PublicQuotientProofVerifierMode {
    fn proof_model(self) -> &'static str {
        match self {
            Self::Representative => "full-response-packed-sis-quotient-public-proof-check",
            Self::SigmaEquation => "full-response-packed-sis-quotient-sigma-equation-check",
        }
    }

    fn ok_reason(self) -> &'static str {
        match self {
            Self::Representative => "ok-stateful-local-delta-public-quotient-proof-relation",
            Self::SigmaEquation => "ok-stateful-local-delta-public-quotient-sigma-equation",
        }
    }
}

fn verify_linear_sigma_equation(
    proof: &LinearSigmaTargetProof,
    challenge_bytes: &[u8; CHALLENGE_BYTES],
    slot_digest: &[u8; DIGEST_BYTES],
) -> Result<(), String> {
    let proof_commitment = unpack_q_syndrome_digest(&proof.proof_commitment)
        .ok_or_else(|| "proof commitment is not a canonical packed q-syndrome".to_string())?;
    let target = unpack_q_syndrome_digest(slot_digest)
        .ok_or_else(|| "slot digest is not a canonical packed q-syndrome".to_string())?;
    let response = response_syndrome(&proof.masked_witness_response).ok_or_else(|| {
        "masked proof response is not a canonical full-response vector".to_string()
    })?;
    let action = challenge_action_vector(challenge_bytes);
    let expected = fq_action_add(&proof_commitment, &fq_action_mul(&action, &target));
    if response != expected {
        return Err("linear sigma verifier equation mismatch".to_string());
    }
    Ok(())
}

fn verify_public_quotient_proof_signature_with_mode(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
    verifier_mode: PublicQuotientProofVerifierMode,
) -> (StatefulVerifyReport, AccountState) {
    let proof_model = verifier_mode.proof_model();
    let expected_signature_bytes = LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES;
    let mut next_account = account.clone();
    let claimed = (signature.len() >= SLOT_ID_BYTES + CHALLENGE_BYTES)
        .then(|| hex_encode(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]));
    if signature.len() != expected_signature_bytes {
        return (
            reject_verify_with_expected(
                proof_model,
                public_key,
                message,
                signature,
                expected_signature_bytes,
                None,
                None,
                claimed,
                None,
                None,
                format!(
                    "signature has {} bytes, expected {expected_signature_bytes}",
                    signature.len()
                ),
            ),
            next_account,
        );
    }
    if public_key.len() != PK_BYTES {
        return (
            reject_verify_with_expected(
                proof_model,
                public_key,
                message,
                signature,
                expected_signature_bytes,
                None,
                None,
                claimed,
                None,
                None,
                format!(
                    "public key has {} bytes, expected {PK_BYTES}",
                    public_key.len()
                ),
            ),
            next_account,
        );
    }

    let slot_id = u32::from_le_bytes(signature[..SLOT_ID_BYTES].try_into().unwrap());
    let Some(slot) = account.slots.get(slot_id as usize) else {
        return (
            reject_verify_with_expected(
                proof_model,
                public_key,
                message,
                signature,
                expected_signature_bytes,
                Some(slot_id),
                None,
                claimed,
                None,
                None,
                "slot id is out of range".to_string(),
            ),
            next_account,
        );
    };
    if !slot.unused {
        return (
            reject_verify_with_expected(
                proof_model,
                public_key,
                message,
                signature,
                expected_signature_bytes,
                Some(slot_id),
                Some(false),
                claimed,
                None,
                None,
                "slot is already consumed".to_string(),
            ),
            next_account,
        );
    }

    let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
    let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
    let proof_start = opening_start + LOCAL_OPENING_BYTES;
    let proof = match decode_linear_sigma_target_proof(&signature[proof_start..]) {
        Ok(proof) => proof,
        Err(err) => {
            return (
                reject_verify_with_expected(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    expected_signature_bytes,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    None,
                    None,
                    err,
                ),
                next_account,
            );
        }
    };
    let frame = challenge_hash_frame(
        public_key,
        message,
        slot_id,
        &slot.digest,
        &proof.proof_commitment,
    );
    let recomputed = hash_frame_to_challenge_bytes(&frame);
    let recomputed_hex = hex_encode(&recomputed);
    if signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES] != recomputed {
        return (
            reject_verify_with_expected(
                proof_model,
                public_key,
                message,
                signature,
                expected_signature_bytes,
                Some(slot_id),
                Some(true),
                claimed,
                Some(recomputed_hex),
                Some(&frame),
                "challenge mismatch".to_string(),
            ),
            next_account,
        );
    }

    let challenge =
        match decode_challenge(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]) {
            Ok(challenge) => challenge,
            Err(err) => {
                return (
                    reject_verify_with_expected(
                        proof_model,
                        public_key,
                        message,
                        signature,
                        expected_signature_bytes,
                        Some(slot_id),
                        Some(true),
                        claimed,
                        Some(recomputed_hex),
                        Some(&frame),
                        err,
                    ),
                    next_account,
                );
            }
        };
    let responses = match decode_responses(&signature[response_start..opening_start]) {
        Ok(responses) => responses,
        Err(err) => {
            return (
                reject_verify_with_expected(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    expected_signature_bytes,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    err,
                ),
                next_account,
            );
        }
    };
    let openings = match decode_openings(&signature[opening_start..proof_start]) {
        Ok(openings) => openings,
        Err(err) => {
            return (
                reject_verify_with_expected(
                    proof_model,
                    public_key,
                    message,
                    signature,
                    expected_signature_bytes,
                    Some(slot_id),
                    Some(true),
                    claimed,
                    Some(recomputed_hex),
                    Some(&frame),
                    err,
                ),
                next_account,
            );
        }
    };

    match verifier_mode {
        PublicQuotientProofVerifierMode::Representative => {
            if !full_response_syndrome_digest_matches_hidden(
                &proof.masked_witness_response,
                &slot.digest,
            ) {
                return (
                    reject_verify_with_expected(
                        proof_model,
                        public_key,
                        message,
                        signature,
                        expected_signature_bytes,
                        Some(slot_id),
                        Some(true),
                        claimed,
                        Some(recomputed_hex),
                        Some(&frame),
                        "slot digest does not match public proof q-syndrome response".to_string(),
                    ),
                    next_account,
                );
            }
        }
        PublicQuotientProofVerifierMode::SigmaEquation => {
            if let Err(err) = verify_linear_sigma_equation(&proof, &recomputed, &slot.digest) {
                return (
                    reject_verify_with_expected(
                        proof_model,
                        public_key,
                        message,
                        signature,
                        expected_signature_bytes,
                        Some(slot_id),
                        Some(true),
                        claimed,
                        Some(recomputed_hex),
                        Some(&frame),
                        err,
                    ),
                    next_account,
                );
            }
        }
    }
    if let Err(err) = verify_local_delta_relation_with_masked_witness(
        public_key,
        &challenge,
        &responses,
        &openings,
        &proof.masked_witness_response,
    ) {
        return (
            reject_verify_with_expected(
                proof_model,
                public_key,
                message,
                signature,
                expected_signature_bytes,
                Some(slot_id),
                Some(true),
                claimed,
                Some(recomputed_hex),
                Some(&frame),
                err,
            ),
            next_account,
        );
    }

    if let Some(slot) = next_account.slots.get_mut(slot_id as usize) {
        slot.unused = false;
    }
    (
        StatefulVerifyReport {
            accepted: true,
            profile: PROFILE,
            proof_model,
            public_key_bytes: public_key.len(),
            message_bytes: message.len(),
            signature_bytes: signature.len(),
            expected_signature_bytes,
            slot: Some(slot_id),
            slot_was_unused: Some(true),
            slot_consumed: true,
            challenge_bytes: CHALLENGE_BYTES,
            response_bytes: ACTIVE_RESPONSE_BYTES,
            opening_bytes: LOCAL_OPENING_BYTES,
            hash_frame_bytes: Some(frame.len()),
            hash_frame_sha256: Some(sha256_hex(&frame)),
            claimed_challenge_hex: claimed,
            recomputed_challenge_hex: Some(recomputed_hex),
            reason: verifier_mode.ok_reason().to_string(),
        },
        next_account,
    )
}

fn verify_public_quotient_proof_signature(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
) -> (StatefulVerifyReport, AccountState) {
    verify_public_quotient_proof_signature_with_mode(
        public_key,
        message,
        account,
        signature,
        PublicQuotientProofVerifierMode::Representative,
    )
}

fn verify_public_quotient_sigma_equation_signature(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
) -> (StatefulVerifyReport, AccountState) {
    verify_public_quotient_proof_signature_with_mode(
        public_key,
        message,
        account,
        signature,
        PublicQuotientProofVerifierMode::SigmaEquation,
    )
}

#[allow(dead_code)]
fn revealed_preimage_report(
    mut report: StatefulVerifyReport,
    full_signature_len: usize,
) -> StatefulVerifyReport {
    report.proof_model = "revealed-hidden-preimage-public-check";
    report.signature_bytes = full_signature_len;
    report.expected_signature_bytes = REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES;
    if report.accepted {
        report.reason = "ok-stateful-local-delta-revealed-preimage-relation".to_string();
    }
    report
}

#[allow(dead_code)]
fn verify_revealed_preimage_stateful_signature(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
) -> (StatefulVerifyReport, AccountState) {
    let claimed = (signature.len() >= SLOT_ID_BYTES + CHALLENGE_BYTES)
        .then(|| hex_encode(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]));
    if signature.len() != REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES {
        let report = reject_verify(
            "revealed-hidden-preimage-public-check",
            public_key,
            message,
            signature,
            None,
            None,
            claimed,
            None,
            None,
            format!(
                "signature has {} bytes, expected {REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES}",
                signature.len()
            ),
        );
        return (
            revealed_preimage_report(report, signature.len()),
            account.clone(),
        );
    }

    let slot_id = u32::from_le_bytes(signature[..SLOT_ID_BYTES].try_into().unwrap());
    let proof_start = STATEFUL_TRANSACTION_SIGNATURE_BYTES;
    let masked_witness_response: [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES] =
        signature[proof_start..].try_into().unwrap();
    let mut account_with_preimage = account.clone();
    if let Some(slot) = account_with_preimage.slots.get_mut(slot_id as usize) {
        slot.ideal_masked_witness_response = Some(masked_witness_response);
    }

    let (ideal_report, _) = verify_ideal_stateful_signature(
        public_key,
        message,
        &account_with_preimage,
        &signature[..proof_start],
    );
    let report = revealed_preimage_report(ideal_report, signature.len());

    let mut next_account = account.clone();
    if report.accepted {
        if let Some(slot) = next_account.slots.get_mut(slot_id as usize) {
            slot.unused = false;
        }
    }
    (report, next_account)
}

fn reference_public_key() -> Vec<u8> {
    vec![0_u8; PK_BYTES]
}

fn reference_keygen(slot_count: usize) -> ReferenceKeypair {
    let public_key = reference_public_key();
    let mask_view = [12_u16, Q - 12];
    let slots = (0..slot_count)
        .map(|slot_id| {
            let base =
                sample_full_response_masked_witness_for_mask_stream(mask_view, slot_id as u64 + 1);
            let mut seed_hasher = Sha256::new();
            seed_hasher.update(b"Descender reference keygen conditioned fiber sampler seed");
            seed_hasher.update((slot_id as u64).to_le_bytes());
            let sampler_seed: [u8; DIGEST_BYTES] = seed_hasher.finalize().into();
            let sample = sample_conditioned_quotient_fiber_representative(
                &base,
                &sampler_seed,
                slot_id as u64,
            );
            let digest = full_response_syndrome_digest(&sample.masked_witness_response).unwrap();
            AccountSlot {
                digest,
                unused: true,
                ideal_masked_witness_response: Some(sample.masked_witness_response),
            }
        })
        .collect();
    ReferenceKeypair {
        public_key,
        account: AccountState { slots },
    }
}

#[allow(dead_code)]
fn reference_sign(
    keypair: &ReferenceKeypair,
    slot_id: u32,
    message: &[u8],
) -> Result<Vec<u8>, String> {
    let slot = keypair
        .account
        .slots
        .get(slot_id as usize)
        .ok_or_else(|| "slot id is out of range".to_string())?;
    if !slot.unused {
        return Err("slot is already consumed".to_string());
    }
    let hidden_preimage = slot
        .ideal_masked_witness_response
        .ok_or_else(|| "missing ideal masked witness response".to_string())?;
    let seed: [u8; PAD_SEED_BYTES] = hidden_preimage
        [FIBER_STATE_BYTES..FIBER_STATE_BYTES + PAD_SEED_BYTES]
        .try_into()
        .unwrap();
    let mask_view = [12_u16, Q - 12];
    if u16_le(&hidden_preimage[..FIBER_STATE_BYTES]) % Q != local_state(mask_view) {
        return Err("reference mask view does not match slot fiber state".to_string());
    }

    let frame =
        legacy_stateful_challenge_hash_frame(&keypair.public_key, message, slot_id, &slot.digest);
    let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
    let challenge = decode_challenge(&challenge_bytes)?;
    let public_view = decode_public_key(&keypair.public_key)?;
    let mut responses = CompactResponses {
        active_free: [[0_i16; LOCAL_FREE]; REPS],
    };
    let mut openings = LocalOpenings {
        active_free: [[0_u16; LOCAL_FREE]; REPS],
    };
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let sign = challenge.sign[rep];
        for coord in 0..LOCAL_FREE {
            let response = add_mod(
                mask_view[coord],
                challenge_mul(sign, public_view.free[block][coord]),
            );
            let centered = if response > Q / 2 {
                response as i32 - Q as i32
            } else {
                response as i32
            };
            if !(-(RESPONSE_BOUND as i32)..=RESPONSE_BOUND as i32).contains(&centered) {
                return Err(format!("reference response out of bound in round {rep}"));
            }
            responses.active_free[rep][coord] = centered as i16;
        }
        let pad = expand_pad_residues(&seed, block);
        openings.active_free[rep] = [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
    }
    let mut signature = Vec::with_capacity(STATEFUL_TRANSACTION_SIGNATURE_BYTES);
    signature.extend_from_slice(&slot_id.to_le_bytes());
    signature.extend_from_slice(&challenge_bytes);
    signature.extend_from_slice(&encode_responses(&responses));
    signature.extend_from_slice(&encode_openings(&openings));
    Ok(signature)
}

fn reference_sign_with_proof(
    keypair: &ReferenceKeypair,
    slot_id: u32,
    message: &[u8],
) -> Result<Vec<u8>, String> {
    let slot = keypair
        .account
        .slots
        .get(slot_id as usize)
        .ok_or_else(|| "slot id is out of range".to_string())?;
    if !slot.unused {
        return Err("slot is already consumed".to_string());
    }
    let masked_witness_response = slot
        .ideal_masked_witness_response
        .ok_or_else(|| "missing ideal masked witness response".to_string())?;
    let proof = LinearSigmaTargetProof {
        proof_commitment: sample_proof_commitment(),
        masked_witness_response,
    };
    let seed: [u8; PAD_SEED_BYTES] =
        semantic_hidden_preimage_from_masked_witness(&proof.masked_witness_response)
            [FIBER_STATE_BYTES..]
            .try_into()
            .unwrap();
    let mask_view = [12_u16, Q - 12];
    if u16_le(&proof.masked_witness_response[..FIBER_STATE_BYTES]) % Q != local_state(mask_view) {
        return Err("reference proof response does not match slot fiber state".to_string());
    }
    if !full_response_syndrome_digest_matches_hidden(&proof.masked_witness_response, &slot.digest) {
        return Err("slot digest does not match proof q-syndrome response".to_string());
    }

    let frame = challenge_hash_frame(
        &keypair.public_key,
        message,
        slot_id,
        &slot.digest,
        &proof.proof_commitment,
    );
    let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
    let challenge = decode_challenge(&challenge_bytes)?;
    let public_view = decode_public_key(&keypair.public_key)?;
    let mut responses = CompactResponses {
        active_free: [[0_i16; LOCAL_FREE]; REPS],
    };
    let mut openings = LocalOpenings {
        active_free: [[0_u16; LOCAL_FREE]; REPS],
    };
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let sign = challenge.sign[rep];
        for coord in 0..LOCAL_FREE {
            let response = add_mod(
                mask_view[coord],
                challenge_mul(sign, public_view.free[block][coord]),
            );
            let centered = if response > Q / 2 {
                response as i32 - Q as i32
            } else {
                response as i32
            };
            if !(-(RESPONSE_BOUND as i32)..=RESPONSE_BOUND as i32).contains(&centered) {
                return Err(format!("reference response out of bound in round {rep}"));
            }
            responses.active_free[rep][coord] = centered as i16;
        }
        let pad = expand_pad_residues(&seed, block);
        openings.active_free[rep] = [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
    }
    let mut signature = Vec::with_capacity(LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES);
    signature.extend_from_slice(&slot_id.to_le_bytes());
    signature.extend_from_slice(&challenge_bytes);
    signature.extend_from_slice(&encode_responses(&responses));
    signature.extend_from_slice(&encode_openings(&openings));
    signature.extend_from_slice(&encode_linear_sigma_target_proof(&proof));
    Ok(signature)
}

fn reference_verify_with_proof(
    public_key: &[u8],
    message: &[u8],
    account: &AccountState,
    signature: &[u8],
) -> (StatefulVerifyReport, AccountState) {
    verify_public_quotient_proof_signature(public_key, message, account, signature)
}

fn production_keygen(slot_count: usize) -> ProductionReferenceKeypair {
    let reference_keypair = reference_keygen(slot_count);
    let proof_commitment = sample_proof_commitment();
    let slot_seeds = reference_keypair
        .account
        .slots
        .iter()
        .map(|slot| ProductionAccountSlotSeed {
            witness_commitment: slot.digest,
            unused: slot.unused,
        })
        .collect::<Vec<_>>();
    ProductionKeypair::new(
        ProductionPublicKey::new(reference_keypair.public_key.clone())
            .expect("reference public key is canonical"),
        ProductionSecretKey::new(reference_keypair),
        prodref::account_state_from_slot_seeds(slot_seeds, proof_commitment)
            .expect("reference keygen emits canonical account slots"),
    )
}

fn production_sign(
    secret_key: &ProductionReferenceSecretKey,
    account_slot: &ProductionAccountSlot,
    message: &[u8],
) -> Result<ProductionSignature, String> {
    prodref::sign_with_secret_key(
        secret_key,
        account_slot,
        |reference_keypair, slot_id, witness_commitment| {
            let reference_slot = reference_keypair
                .account
                .slots
                .get(slot_id as usize)
                .ok_or_else(|| "slot id is out of range".to_string())?;
            if reference_slot.digest != witness_commitment {
                return Err("account witness commitment does not match secret slot".to_string());
            }
            Ok(ProductionSignature::new(reference_sign_with_proof(
                reference_keypair,
                slot_id,
                message,
            )?))
        },
    )
}

fn legacy_account_from_production(
    account: &ProductionAccountState,
) -> Result<AccountState, String> {
    account.project_checked_slot_records(|records| {
        let mut slots = Vec::with_capacity(records.len());
        for record in records {
            slots.push(AccountSlot {
                digest: record.witness_commitment,
                unused: record.unused,
                ideal_masked_witness_response: None,
            });
        }
        Ok(AccountState { slots })
    })
}

fn production_reject(
    public_key: &[u8],
    message: &[u8],
    signature: &ProductionSignature,
    rejection: ProductionVerificationRejection,
) -> StatefulVerifyReport {
    let fields = prodref::rejection_report_fields(signature, rejection);
    reject_verify_with_expected(
        fields.proof_model,
        public_key,
        message,
        signature.as_bytes(),
        fields.expected_signature_bytes,
        fields.slot_id,
        None,
        fields
            .claimed_challenge
            .map(|challenge| hex_encode(&challenge)),
        None,
        None,
        fields.reason,
    )
}

fn production_verify(
    public_key: &ProductionPublicKey,
    message: &[u8],
    account: &ProductionAccountState,
    signature: &ProductionSignature,
) -> (StatefulVerifyReport, ProductionAccountState) {
    let next_account = account.clone();
    let verification = match prodref::verify_with_account_binding(
        public_key,
        message,
        account,
        signature,
        |public_key, message, account, signature, _checked| {
            let legacy_account = legacy_account_from_production(account)?;
            let (report, _) = reference_verify_with_proof(
                public_key.as_bytes(),
                message,
                &legacy_account,
                signature.as_bytes(),
            );
            Ok(report)
        },
    ) {
        Ok((checked, report)) => {
            let accepted = report.accepted;
            match prodref::complete_verified_signature(account, checked, report, accepted) {
                Ok(verification) => verification,
                Err(err) => {
                    return (
                        production_reject(public_key.as_bytes(), message, signature, err),
                        next_account,
                    );
                }
            }
        }
        Err(err) => {
            return (
                production_reject(
                    public_key.as_bytes(),
                    message,
                    signature,
                    err.into_rejection(),
                ),
                next_account,
            );
        }
    };
    verification.into_report_and_account()
}

fn timing_row(
    operation: &'static str,
    iterations: usize,
    total_ns: u128,
    accepted_or_ok: bool,
) -> ReferenceTimingRow {
    let avg_ns = total_ns / iterations.max(1) as u128;
    ReferenceTimingRow {
        operation,
        iterations,
        total_ns,
        avg_ns,
        avg_us: avg_ns as f64 / 1000.0,
        accepted_or_ok,
    }
}

fn reference_timing_report(iterations: usize) -> ReferenceTimingReport {
    let iterations = iterations.max(1);
    let message = b"Descender reference timing benchmark message";
    let keygen_slots = 2;

    let started = Instant::now();
    let mut keygen_ok = true;
    for _ in 0..iterations {
        let keypair = reference_keygen(black_box(keygen_slots));
        keygen_ok &=
            keypair.public_key.len() == PK_BYTES && keypair.account.slots.len() == keygen_slots;
        black_box(keypair);
    }
    let keygen_ns = started.elapsed().as_nanos();

    let keypair = reference_keygen(keygen_slots);
    let sample_signature =
        reference_sign_with_proof(&keypair, 0, message).expect("reference signing succeeds");

    let started = Instant::now();
    let mut sign_ok = true;
    for idx in 0..iterations {
        let slot_id = (idx % keygen_slots) as u32;
        let signature = reference_sign_with_proof(black_box(&keypair), slot_id, black_box(message));
        sign_ok &= signature
            .as_ref()
            .is_ok_and(|sig| sig.len() == LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES);
        let _ = black_box(signature);
    }
    let sign_ns = started.elapsed().as_nanos();

    let started = Instant::now();
    let mut verify_ok = true;
    for _ in 0..iterations {
        let (report, next_account) = reference_verify_with_proof(
            black_box(&keypair.public_key),
            black_box(message),
            black_box(&keypair.account),
            black_box(&sample_signature),
        );
        verify_ok &= report.accepted && next_account.slots.first().is_some_and(|slot| !slot.unused);
        black_box((report, next_account));
    }
    let verify_ns = started.elapsed().as_nanos();

    let started = Instant::now();
    let mut production_keygen_ok = true;
    for _ in 0..iterations {
        let keypair = production_keygen(black_box(keygen_slots));
        production_keygen_ok &= keypair.public_key.len() == PK_BYTES
            && keypair.account.len() == keygen_slots
            && keypair.account.get_slot(0).is_some_and(|slot| {
                slot.commitment().as_bytes().len() == REVIEWED_ACCOUNT_LINK_BYTES
            });
        black_box(keypair);
    }
    let production_keygen_ns = started.elapsed().as_nanos();

    let production_keypair = production_keygen(keygen_slots);
    let production_sample_signature = production_sign(
        &production_keypair.secret_key,
        production_keypair.account.get_slot(0).unwrap(),
        message,
    )
    .expect("production facade signing succeeds");

    let started = Instant::now();
    let mut production_sign_ok = true;
    for idx in 0..iterations {
        let slot_id = (idx % keygen_slots) as u32;
        let signature = production_sign(
            black_box(&production_keypair.secret_key),
            black_box(production_keypair.account.get_slot(slot_id).unwrap()),
            black_box(message),
        );
        production_sign_ok &= signature.as_ref().is_ok_and(|sig| {
            sig.len() == LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
                && sig.len() != REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES
        });
        let _ = black_box(signature);
    }
    let production_sign_ns = started.elapsed().as_nanos();

    let started = Instant::now();
    let mut production_verify_ok = true;
    for _ in 0..iterations {
        let (report, next_account) = production_verify(
            black_box(&production_keypair.public_key),
            black_box(message),
            black_box(&production_keypair.account),
            black_box(&production_sample_signature),
        );
        production_verify_ok &= report.accepted
            && next_account
                .get_slot(0)
                .is_some_and(|slot| !slot.is_unused());
        black_box((report, next_account));
    }
    let production_verify_ns = started.elapsed().as_nanos();

    ReferenceTimingReport {
        benchmark_name: "signature_q12289_local_delta_stateful.reference_timing",
        iterations,
        keygen_slots,
        message_bytes: message.len(),
        signature_bytes: production_sample_signature.len(),
        public_key_bytes: keypair.public_key.len(),
        account_slots: keypair.account.slots.len(),
        timings: vec![
            timing_row("reference_keygen", iterations, keygen_ns, keygen_ok),
            timing_row("reference_sign_with_proof", iterations, sign_ns, sign_ok),
            timing_row(
                "reference_verify_with_proof",
                iterations,
                verify_ns,
                verify_ok,
            ),
            timing_row(
                "production_facade_keygen",
                iterations,
                production_keygen_ns,
                production_keygen_ok,
            ),
            timing_row(
                "production_facade_sign",
                iterations,
                production_sign_ns,
                production_sign_ok,
            ),
            timing_row(
                "production_facade_verify",
                iterations,
                production_verify_ns,
                production_verify_ok,
            ),
        ],
        caveat: "Reference/research timing only: not constant-time, not a production account manager, and the production-facade rows still wrap the legacy 264-byte proof callback rather than accepting 318-byte/Fq12 projection-carrier proof semantics.",
    }
}

fn candidate(
    name: &'static str,
    proof_bytes: usize,
    public_checkable: bool,
    witness_hiding: bool,
    implemented: bool,
    production_candidate: bool,
    note: &'static str,
) -> OpeningProofCandidate {
    OpeningProofCandidate {
        name,
        proof_bytes,
        total_signature_bytes: STATEFUL_TRANSACTION_SIGNATURE_BYTES + proof_bytes,
        public_checkable,
        witness_hiding,
        implemented,
        production_candidate,
        note,
    }
}

fn opening_proof_frontier() -> OpeningProofFrontier {
    OpeningProofFrontier {
        base_transaction_bytes: STATEFUL_TRANSACTION_SIGNATURE_BYTES,
        under_256_max_proof_bytes: PROOF_BYTES_UNDER_256,
        beat_269_max_proof_bytes: PROOF_BYTES_BEAT_IMPLICIT_269,
        tie_269_max_proof_bytes: PROOF_BYTES_TIE_IMPLICIT_269,
        beat_stateless_sha256_max_proof_bytes: PROOF_BYTES_BEAT_STATELESS_SHA256,
        candidates: vec![
            candidate(
                "ideal relation oracle",
                0,
                false,
                true,
                true,
                false,
                "Proof abstraction only; verifier needs hidden preimage out of band.",
            ),
            candidate(
                "revealed hidden preimage",
                REVEALED_PREIMAGE_PROOF_BYTES,
                true,
                false,
                true,
                false,
                "Public-checkable control point; reveals the full masked witness response.",
            ),
            candidate(
                "algebraic linear sigma target",
                LINEAR_SIGMA_TARGET_PROOF_BYTES,
                true,
                true,
                false,
                true,
                "Hardened target: one 32-byte proof commitment plus a 41-byte masked witness response.",
            ),
            candidate(
                "sub-256 witness-hiding target",
                PROOF_BYTES_UNDER_256,
                true,
                true,
                false,
                true,
                "Maximum proof budget to keep the transaction at or below 256 bytes.",
            ),
            candidate(
                "beat 269-byte implicit verifier",
                PROOF_BYTES_BEAT_IMPLICIT_269,
                true,
                true,
                false,
                true,
                "Maximum proof budget to strictly beat the completed-response verifier.",
            ),
            candidate(
                "beat 1009-byte stateless SHA-digest mode",
                PROOF_BYTES_BEAT_STATELESS_SHA256,
                true,
                true,
                false,
                true,
                "Loose ceiling; useful only if the proof is much larger than expected.",
            ),
        ],
    }
}

fn committed_sigma_proof_bytes(
    masked_response_bytes: usize,
    hidden_link_opening_bytes: usize,
) -> usize {
    LINEAR_SIGMA_PROOF_COMMITMENT_BYTES + masked_response_bytes + hidden_link_opening_bytes
}

fn committed_sigma_transaction_bytes(
    masked_response_bytes: usize,
    hidden_link_opening_bytes: usize,
) -> usize {
    STATEFUL_TRANSACTION_SIGNATURE_BYTES
        + committed_sigma_proof_bytes(masked_response_bytes, hidden_link_opening_bytes)
}

fn hidden_link_budget_under(cap: usize, masked_response_bytes: usize) -> usize {
    cap.saturating_sub(committed_sigma_transaction_bytes(masked_response_bytes, 0))
}

fn packed_q_residue_bytes(residues: usize) -> usize {
    (residues * SIS_RESIDUE_BITS).div_ceil(8)
}

fn committed_sigma_byte_candidate(
    name: &'static str,
    masked_response_bytes: usize,
    hidden_link_opening_bytes: usize,
    account_hidden_link_bytes: usize,
    structurally_admissible_without_extra_proof: bool,
    required_next_obligation: &'static str,
) -> CommittedSigmaByteCandidate {
    CommittedSigmaByteCandidate {
        name,
        masked_response_bytes,
        hidden_link_opening_bytes,
        account_hidden_link_bytes,
        proof_bytes: committed_sigma_proof_bytes(masked_response_bytes, hidden_link_opening_bytes),
        transaction_bytes: committed_sigma_transaction_bytes(
            masked_response_bytes,
            hidden_link_opening_bytes,
        ),
        hidden_link_budget_under_256: hidden_link_budget_under(256, masked_response_bytes),
        total_stateful_slot_cost_bytes: committed_sigma_transaction_bytes(
            masked_response_bytes,
            hidden_link_opening_bytes,
        ) + account_hidden_link_bytes,
        structurally_admissible_without_extra_proof,
        required_next_obligation,
    }
}

#[allow(clippy::too_many_arguments)]
fn account_link_proof_candidate(
    name: &'static str,
    signature_link_proof_bytes: usize,
    account_link_bytes: usize,
    reveals_witness_or_randomizer: bool,
    local_opening_only: bool,
    account_slot_aware: bool,
    instantiates_account_link_proof_system: bool,
    post_challenge_representative_rejected: bool,
    not_public_projection_of_sigma_equation: bool,
    witness_hiding_argument: bool,
    status: &'static str,
) -> AccountLinkProofCandidate {
    let structurally_admissible = !reveals_witness_or_randomizer
        && !local_opening_only
        && account_slot_aware
        && instantiates_account_link_proof_system
        && post_challenge_representative_rejected
        && not_public_projection_of_sigma_equation
        && witness_hiding_argument;
    AccountLinkProofCandidate {
        name,
        signature_link_proof_bytes,
        account_link_bytes,
        total_link_state_bytes: signature_link_proof_bytes + account_link_bytes,
        reveals_witness_or_randomizer,
        local_opening_only,
        account_slot_aware,
        instantiates_account_link_proof_system,
        post_challenge_representative_rejected,
        not_public_projection_of_sigma_equation,
        witness_hiding_argument,
        structurally_admissible,
        status,
    }
}

fn account_link_proof_frontier() -> Vec<AccountLinkProofCandidate> {
    vec![
        account_link_proof_candidate(
            "reveal-witness-or-randomizer-link",
            LINEAR_SIGMA_MASKED_WITNESS_BYTES,
            0,
            true,
            false,
            true,
            false,
            true,
            true,
            false,
            "Rejected: revealing the witness/randomizer leaks the hidden relation.",
        ),
        account_link_proof_candidate(
            "local-opening-only-link",
            0,
            0,
            false,
            true,
            false,
            false,
            false,
            false,
            true,
            "Rejected: the local opening verifier does not inspect hiddenLinkCommitment and repeats the public representative failure.",
        ),
        account_link_proof_candidate(
            "bare-account-hash-link",
            0,
            64,
            false,
            false,
            true,
            false,
            false,
            true,
            true,
            "Rejected: account-state bytes without a public link proof do not instantiate AccountLinkProofSystem.",
        ),
        account_link_proof_candidate(
            "slot-bound-first-message-only",
            0,
            LINEAR_SIGMA_PROOF_COMMITMENT_BYTES,
            false,
            false,
            true,
            false,
            false,
            false,
            true,
            "Rejected: precommitting the sigma first message in the slot is necessary but does not by itself link z to hidden w,r.",
        ),
        account_link_proof_candidate(
            "public-projection-homomorphic-link",
            0,
            96,
            false,
            false,
            true,
            false,
            false,
            false,
            true,
            "Rejected: Commit(z)=Commit(r)+c*Commit(w) is vacuous if the commitments are only public projections of L(z), a, and digest.",
        ),
        account_link_proof_candidate(
            "homomorphic-account-link-equation",
            0,
            96,
            false,
            false,
            true,
            true,
            true,
            true,
            true,
            "Concrete live class: verifier checks Commit(z)=Commit(r)+c*Commit(w); security reduces to the homomorphic commitment/linkEquationSoundness obligation.",
        ),
        account_link_proof_candidate(
            "account-carried-succinct-link-proof",
            0,
            64,
            false,
            false,
            true,
            true,
            true,
            true,
            true,
            "Live class: zero signature link bytes; prove AccountLinkProofSystem using consumed account-slot evidence.",
        ),
        account_link_proof_candidate(
            "six-byte-signature-link-proof-stress-target",
            6,
            32,
            false,
            false,
            true,
            true,
            true,
            true,
            true,
            "Stress class: uses the entire 37-byte response spare budget under 256 bytes.",
        ),
    ]
}

fn response_link_code_candidate(
    name: &'static str,
    coordinate_projection: Vec<usize>,
    public_account_witness_hiding: bool,
    status: &'static str,
) -> ResponseLinkCodeCandidate {
    let public_kernel_dimension = SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS;
    let projection_rank = full_response_kernel_projection_rank(&coordinate_projection);
    let residual_full_kernel_dimension = public_kernel_dimension.saturating_sub(projection_rank);
    let (intersection_basis, combined_constraint_rank, combined_kernel_dimension) =
        response_link_intersection_lattice_basis(&coordinate_projection);
    let (shortest_intersection_vector, lll_iterations) =
        lll_shortest_vector_from_basis(intersection_basis);
    let response_difference_coeff_bound = 2 * RESPONSE_BOUND as i64;
    let shortest_intersection_vector_coeff_bound =
        vector_coeff_bound(&shortest_intersection_vector);
    let lll_margin_over_response_difference_bound =
        shortest_intersection_vector_coeff_bound - response_difference_coeff_bound;
    ResponseLinkCodeCandidate {
        name,
        commitment_residues: coordinate_projection.len(),
        commitment_bytes: packed_q_residue_bytes(coordinate_projection.len()),
        coordinate_projection,
        public_kernel_dimension_before_link: public_kernel_dimension,
        projection_rank_on_public_kernel: projection_rank,
        residual_full_kernel_dimension,
        combined_constraint_rank,
        combined_kernel_dimension,
        lll_iterations,
        response_difference_coeff_bound,
        shortest_intersection_vector_coeff_bound,
        lll_margin_over_response_difference_bound,
        shortest_intersection_vector_l1_weight: signed_l1_weight(&shortest_intersection_vector),
        shortest_intersection_vector_signed_support: signed_support(&shortest_intersection_vector),
        lll_box_probe_excludes_response_delta: lll_margin_over_response_difference_bound > 0,
        full_kernel_sound: residual_full_kernel_dimension == 0,
        public_account_witness_hiding,
        status,
    }
}

fn response_link_code_frontier() -> Vec<ResponseLinkCodeCandidate> {
    let (_rref, pivot_columns) = full_response_rref();
    let free_columns = full_response_free_columns(&pivot_columns);
    vec![
        response_link_code_candidate(
            "full-kernel-coordinate-code",
            free_columns,
            false,
            "Baseline: projecting to the 11 free coordinates kills the entire public quotient kernel, so L(z) plus C(z) is globally injective. This gives clean response-link soundness but also makes public account commitments C(w) reveal w together with L(w), so it is not deployable as a public witness commitment.",
        ),
        response_link_code_candidate(
            "bounded-box-kernel-code-target",
            vec![0, 3],
            true,
            "Live target: keep a large algebraic kernel for public-account hiding, but prove there is no nonzero allowed response delta in ker(L) ∩ ker(C). This needs a concrete bounded/SVP-style certificate before it can instantiate ResponseLinkSoundness.",
        ),
        response_link_code_candidate(
            "bounded-box-kernel-code-alt-0-1",
            vec![0, 1],
            true,
            "Two-coordinate comparison row for the bounded-box response-link search.",
        ),
        response_link_code_candidate(
            "bounded-box-kernel-code-alt-0-2",
            vec![0, 2],
            true,
            "Two-coordinate comparison row for the bounded-box response-link search.",
        ),
        response_link_code_candidate(
            "bounded-box-kernel-code-alt-1-2",
            vec![1, 2],
            true,
            "Two-coordinate comparison row for the bounded-box response-link search.",
        ),
    ]
}

fn account_hidden_link_contract() -> AccountHiddenLinkContract {
    AccountHiddenLinkContract {
        lean_surface: "CommittedSigma.AccountCarriedHiddenLinkInterface",
        verifier_accepts_surface: "CommittedSigma.AccountCarriedVerifierAccepts",
        soundness_certificate_surface: "CommittedSigma.AccountCarriedSoundnessCertificate",
        account_opening_verifier_surface: "CommittedSigma.AccountCarriedVerifierAcceptsWithAccountOpening",
        account_link_proof_system_surface: "CommittedSigma.AccountLinkProofSystem",
        homomorphic_account_link_surface: "CommittedSigma.HomomorphicAccountLinkInterface",
        pre_response_backing_surface: "CommittedSigma.AccountBackedExceptResponse",
        response_link_soundness_surface: "CommittedSigma.ResponseLinkSoundness",
        bounded_response_link_kernel_surface: "CommittedSigma.BoundedResponseLinkKernelCertificate",
        bounded_response_link_soundness_theorem: "CommittedSigma.BoundedResponseLinkKernelCertificate.responseLinkSoundness",
        bounded_response_link_checker_script: "scripts/research/response_link_bounded_kernel_check.py",
        response_link_bridge_theorem: "CommittedSigma.AccountBackedExceptResponse.accountBacked_or_linkKernelBreak",
        response_link_code_frontier: response_link_code_frontier(),
        slot_fields: vec![
            "statementDigest = L(w)",
            "witnessCommitment = CommitW(w)",
            "randomizerCommitment = CommitR(r)",
            "hiddenLinkCommitment = CommitLink(w, r)",
        ],
        signature_fields: vec![
            "proofCommitment = L(r)",
            "challenge = H(public frame, proofCommitment, consumed slot fields)",
            "response = r + c*w",
            "opening = public local/quotient opening bytes",
        ],
        homomorphic_link_equation: "Commit(response) = Commit(r) + challenge * Commit(w)",
        verifier_equation: "L(response) = proofCommitment + challenge * statementDigest",
        accepting_implication: "accountSlotWellFormed && verifier_equation && openingAccepts => AccountBackedByHiddenWitness or accountLinkProofBreak",
        unbacked_acceptance_theorem: "AccountCarriedHiddenLinkInterface.unbacked_acceptance_forces_break",
        named_break_event: "accountLinkProofBreak",
        rejected_instantiations: vec![
            "bare account hash/link commitment without a public opening proof",
            "opening predicate that only checks z/public local representatives",
            "proof commitment stored in account state but not linked to the same hidden w,r",
            "homomorphic commitment that is only a public projection of the sigma equation",
        ],
        link_proof_frontier: account_link_proof_frontier(),
        preferred_instantiation: "account-carried-fiber-saturated-hidden-link: 37-byte response, zero signature hidden-link bytes, link evidence carried by consumed account-slot state",
        open_cryptographic_obligation: "instantiate CommitLink/openingAccepts so a public post-challenge representative cannot satisfy the link without committed prechallenge witness/randomizer material",
    }
}

fn committed_sigma_byte_frontier() -> CommittedSigmaByteFrontier {
    const FIBER_SATURATED_RESPONSE_BYTES: usize = 37;
    const FIBER_STATE_PRESERVING_RESPONSE_BYTES: usize = 39;
    CommittedSigmaByteFrontier {
        base_transaction_bytes: STATEFUL_TRANSACTION_SIGNATURE_BYTES,
        proof_commitment_bytes: LINEAR_SIGMA_PROOF_COMMITMENT_BYTES,
        sub_256_cap_bytes: 256,
        current_response_hidden_link_budget_under_256: hidden_link_budget_under(
            256,
            LINEAR_SIGMA_MASKED_WITNESS_BYTES,
        ),
        fiber_saturated_hidden_link_budget_under_256: hidden_link_budget_under(
            256,
            FIBER_SATURATED_RESPONSE_BYTES,
        ),
        fiber_state_preserving_hidden_link_budget_under_256: hidden_link_budget_under(
            256,
            FIBER_STATE_PRESERVING_RESPONSE_BYTES,
        ),
        account_hidden_link_contract: account_hidden_link_contract(),
        candidates: vec![
            committed_sigma_byte_candidate(
                "current-public-sigma-equation",
                LINEAR_SIGMA_MASKED_WITNESS_BYTES,
                0,
                0,
                false,
                "Rejected: public post-challenge representative reconstruction still verifies.",
            ),
            committed_sigma_byte_candidate(
                "bare-account-hash-link",
                FIBER_SATURATED_RESPONSE_BYTES,
                0,
                64,
                false,
                "Rejected: account bytes alone do not prove the response is linked to the committed prechallenge witness/randomizer.",
            ),
            committed_sigma_byte_candidate(
                "account-carried-fiber-saturated-hidden-link",
                FIBER_SATURATED_RESPONSE_BYTES,
                0,
                64,
                false,
                "Live low-byte target: prove PrechallengeRandomizerOpeningInterface with witness/randomizer/link evidence carried by spendable account-slot state.",
            ),
            committed_sigma_byte_candidate(
                "signature-carried-fiber-saturated-hidden-link-budget",
                FIBER_SATURATED_RESPONSE_BYTES,
                6,
                32,
                false,
                "Stress target: after the widened challenge a six-byte signature-carried hidden-link proof overshoots the 256-byte cap; binding must move to account state or the byte target must rise.",
            ),
            committed_sigma_byte_candidate(
                "account-carried-fiber-state-preserving-hidden-link",
                FIBER_STATE_PRESERVING_RESPONSE_BYTES,
                0,
                64,
                false,
                "Conservative target retaining local fiber-state equality; proves the same interface but no longer fits under the 256-byte cap after the widened challenge.",
            ),
        ],
        interpretation: "With the widened 27-byte challenge the current 41-byte masked response reaches 260 bytes before a hidden-link proof, already past a 256-byte cap. The low-byte proof route must therefore shrink the quotient response to 37 bytes (exactly 256), move the link into account state, or raise the byte target.",
    }
}

fn assumption_review_targets() -> Vec<AssumptionReviewTarget> {
    vec![
        AssumptionReviewTarget {
            name: "full-response matrix generation",
            lean_surface: "FullResponseMatrixSamplerAgreement",
            rust_surface: "full_response_matrix_entry / SIS_FULL_RESPONSE_MATRIX_DOMAIN",
            review_question: "Does the Lean matrix function match SHA-256 rejection sampling over the exact Rust domain, row, column, and counter bytes?",
            status: "byte domain fixed; SHA-256 sampler agreement remains an external audit target",
        },
        AssumptionReviewTarget {
            name: "packed full-response extraction",
            lean_surface: "ConcreteFullResponseQuotientReductionAssumptions.extracts_or_break",
            rust_surface: "full_response_syndrome_digest / linear sigma verifier equation",
            review_question: "Does an accepting 73-byte linear-sigma proof extract a valid full-response representative or produce the named algebraic binding break?",
            status: "assumption slot isolated; needs cryptanalysis for the chosen 12x23 map",
        },
        AssumptionReviewTarget {
            name: "quotient witness hiding",
            lean_surface: "ConcreteFullResponseQuotientReductionAssumptions.witnessHiding",
            rust_surface: "hidden 11-residue quotient fiber in SisFullResponseFiberReport",
            review_question: "Does the masked 41-byte response hide the representative within the 11-residue public-syndrome fiber?",
            status: "assumption slot isolated; requires distributional proof",
        },
        AssumptionReviewTarget {
            name: "pad source pseudorandomness",
            lean_surface: "SHA256RejectionPadPRFAssumption",
            rust_surface: "expand_pad_residues(seed, block)",
            review_question: "Does SHA-256 rejection sampling from seed || padFrame(block) || counter provide secret-independent local pads?",
            status: "domain/frame fixed; PRF assumption remains explicit",
        },
        AssumptionReviewTarget {
            name: "Fiat-Shamir challenge agreement",
            lean_surface: "ProductionHashFrameOracleAgreement / stateful verifier challengeMatches",
            rust_surface: "challenge_hash_frame / hash_frame_to_challenge_bytes",
            review_question: "Does the production SHA-256 challenge frame match the random-oracle transcript binding used by the proof?",
            status: "existing proof surface; still a review gate for deployment",
        },
    ]
}

fn assumption_closure_report() -> Vec<AssumptionClosureReport> {
    vec![
        AssumptionClosureReport {
            gate: "full-response matrix generation",
            closure_artifact: "MatrixSamplerCertificate",
            lean_game: "FullResponseMatrixSamplerAgreement",
            break_event: "matrix sampler mismatch",
            no_break_claim: "sha256RejectionSamplerMatchesRust",
            target_classical_security_bits: 128,
            target_quantum_security_bits: 128,
            query_bound_status: "implementation-equivalence review, not an adversarial game",
            residual_assumption: "external SHA-256 implementation matches the specified row/column/counter rejection sampler",
            status: "byte domain, encodings, cutoff, transcript digest, and sample rows are executable",
        },
        AssumptionClosureReport {
            gate: "packed full-response extraction",
            closure_artifact: "FullResponseQuotientAlgebraCertificate + ConcreteFullResponseQuotientReductionAssumptions.extracts_or_break",
            lean_game: "PackedFullResponseExtractionGame",
            break_event: "ConcreteFullResponseQuotientReductionAssumptions.extractionFailure",
            no_break_claim: "noExtractionBreak => no_extractionFailure",
            target_classical_security_bits: 128,
            target_quantum_security_bits: 128,
            query_bound_status: "needs concrete adversary query limit for the packed sigma/q-syndrome extraction game",
            residual_assumption: "knowledge-soundness of the selected linear-sigma/q-syndrome proof, or production of the named algebraic binding break",
            status: "linear algebra is certified; cryptographic extraction remains the explicit reduction assumption",
        },
        AssumptionClosureReport {
            gate: "quotient witness hiding",
            closure_artifact: "FullResponseQuotientAlgebraCertificate + ConcreteFullResponseQuotientReductionAssumptions.witnessHiding",
            lean_game: "QuotientWitnessHidingGame",
            break_event: "witnessHidingBreak",
            no_break_claim: "noWitnessHidingBreak => witnessHiding",
            target_classical_security_bits: 128,
            target_quantum_security_bits: 128,
            query_bound_status: "needs concrete adversary query limit for distinguishing representatives in the 11-residue quotient fiber",
            residual_assumption: "masked response distribution hides the representative inside the 11-residue quotient fiber",
            status: "fiber dimension and kernel basis are certified; distributional hiding remains the explicit assumption",
        },
        AssumptionClosureReport {
            gate: "pad source pseudorandomness",
            closure_artifact: "PadSamplerCertificate + SHA256RejectionPadPRFAssumption",
            lean_game: "SHA256PadPRFGame",
            break_event: "padPRFBreak",
            no_break_claim: "noPadPRFBreak => pseudorandomLocalPad",
            target_classical_security_bits: 128,
            target_quantum_security_bits: 128,
            query_bound_status: "needs concrete bound for seeded SHA-256 pad queries across slots and blocks",
            residual_assumption: "SHA-256 with the fixed seed/frame/counter input is a PRF for the local pad distribution",
            status: "frame, u32 counter, rejection cutoff, and sample digest are executable; SHA-256 PRF security is assumed",
        },
        AssumptionClosureReport {
            gate: "Fiat-Shamir challenge agreement",
            closure_artifact: "FiatShamirFrameCertificate + deployed 254-byte verifier challengeMatches",
            lean_game: "FiatShamirFrameAgreementGame",
            break_event: "frameMismatchBreak",
            no_break_claim: "noFrameMismatchBreak => fiatShamirFrameAgreement",
            target_classical_security_bits: 128,
            target_quantum_security_bits: 128,
            query_bound_status: "needs concrete random-oracle/QROM query bound for prechallenge frame hashing",
            residual_assumption: "random-oracle model agreement for SHA-256 over the fixed proof-commitment-bound challenge frame",
            status: "frame labels, proof-commitment binding, byte length, SHA-256 digest, challenge truncation, and response/opening exclusion are executable",
        },
    ]
}

fn query_loss_classical(raw_bits: usize, query_log2: usize) -> usize {
    raw_bits.saturating_sub(query_log2)
}

fn query_loss_quantum(raw_bits: usize, query_log2: usize) -> usize {
    raw_bits.saturating_sub(2 * query_log2)
}

fn budget(
    game: &'static str,
    raw_security_bits: usize,
    adversary_query_bound_log2: usize,
    note: &'static str,
) -> ConcreteSecurityBudget {
    let classical_security_bits_after_queries =
        query_loss_classical(raw_security_bits, adversary_query_bound_log2);
    let quantum_security_bits_after_queries =
        query_loss_quantum(raw_security_bits, adversary_query_bound_log2);
    ConcreteSecurityBudget {
        game,
        raw_security_bits,
        adversary_query_bound_log2,
        classical_security_bits_after_queries,
        quantum_security_bits_after_queries,
        meets_128_classical: classical_security_bits_after_queries >= 128,
        meets_128_quantum: quantum_security_bits_after_queries >= 128,
        note,
    }
}

fn concrete_security_budgets() -> Vec<ConcreteSecurityBudget> {
    let sampler_certificate = masked_response_sampler_certificate();
    vec![
        budget(
            "PackedFullResponseExtractionGame",
            SIS_FULL_RESPONSE_PUBLIC_ROWS * SIS_RESIDUE_BITS,
            exact_security_query_bound_log2_for_128(
                SIS_FULL_RESPONSE_PUBLIC_ROWS * SIS_RESIDUE_BITS,
            ),
            "168-bit public quotient target; leaves 128 quantum bits up to 2^20 generic square-root-loss queries. Algebraic attacks need separate review.",
        ),
        budget(
            "QuotientWitnessHidingGame",
            sampler_certificate.exact_conditioned_min_entropy_floor_bits,
            sampler_certificate.exact_quantum_query_bound_log2_for_128,
            "Exact q=12289 conditioned support is 12289^9 * 3073, giving floor min-entropy 133 bits; this leaves 128 quantum bits only up to 2^2 generic support-search queries, while the game-specific transcript distinguishing bound is tracked separately as statistical distance 0.",
        ),
        budget(
            "SHA256PadPRFGame",
            256,
            64,
            "256-bit seed/hash primitive target; square-root-loss budget leaves 128 quantum bits up to 2^64 pad-query scale.",
        ),
        budget(
            "FiatShamirFrameAgreementGame-hardened-31-byte-challenge",
            CHALLENGE_ENTROPY_BITS,
            FS_QUANTUM_QUERY_BITS_FOR_128,
            "Hardened challenge has 130 algebraic bits plus 64 transcript-bound hardening bits; 128-bit quantum margin allows 2^33 generic challenge-oracle queries.",
        ),
        budget(
            "FiatShamirFrameAgreementGame-hardened-31-byte-challenge-classical",
            CHALLENGE_ENTROPY_BITS,
            FS_CLASSICAL_QUERY_BITS_FOR_128,
            "Hardened challenge has 130 algebraic bits plus 64 transcript-bound hardening bits; 128-bit classical margin allows 2^66 generic challenge-oracle queries.",
        ),
    ]
}

fn cryptoanalysis_attack_surfaces() -> Vec<CryptoanalysisAttackSurface> {
    vec![
        CryptoanalysisAttackSurface {
            gate: "packed full-response extraction",
            target_object: "12x23 q-syndrome quotient map plus 73-byte linear-sigma opening",
            attacker_goal: "produce an accepting proof whose extracted response does not satisfy the concrete local-delta quotient relation",
            current_evidence: "rank, right inverse, and quotient byte accounting are certified; sparse public column-relation smoke test is executable",
            next_test: "run lattice-reduction/SIS-style searches for short relations against the public quotient matrix and model extraction failure as a concrete witness",
            proof_status: "open cryptanalytic assumption",
        },
        CryptoanalysisAttackSurface {
            gate: "quotient witness hiding",
            target_object: "11-dimensional hidden fiber of the 12x23 public quotient map",
            attacker_goal: "distinguish representatives or recover hidden randomizer information from the public syndrome and response distribution",
            current_evidence: "kernel dimension and a bounded hidden-fiber short-combination probe are executable",
            next_test: "derive the representative distribution over the fiber and compare it to the claimed hiding distribution with finite-distance bounds",
            proof_status: "open distributional assumption",
        },
        CryptoanalysisAttackSurface {
            gate: "pad source pseudorandomness",
            target_object: "SHA-256 rejection-sampled local pad stream keyed by the hidden pad seed",
            attacker_goal: "distinguish pad residues from ideal pads or bias accepted openings across slots/blocks",
            current_evidence: "domain, frame, counter encoding, rejection cutoff, and sample digest are pinned",
            next_test: "count pad queries per account/slot and write a concrete PRF/query-loss bound for the deployment model",
            proof_status: "standard primitive assumption still explicit",
        },
        CryptoanalysisAttackSurface {
            gate: "Fiat-Shamir frame agreement",
            target_object: "21-byte SHA-256 challenge: 17 algebraic bytes plus 4 transcript-hardening bytes",
            attacker_goal: "find transcript collisions, grinding leverage, or frame ambiguity that invalidates the random-oracle challenge model",
            current_evidence: "frame labels, byte length, sample challenge, 162-bit budget, and response/opening exclusion are pinned",
            next_test: "formalize the QROM/classical query budget for blockchain-scale multi-query verification",
            proof_status: "budget improved; final reduction accounting open",
        },
        CryptoanalysisAttackSurface {
            gate: "stateful key/account lifecycle",
            target_object: "slot digest commitment, slot id, and consumed-slot transition",
            attacker_goal: "replay a slot, cross-bind a signature to another message/account, or exploit crash/concurrency state reuse",
            current_evidence: "unit tests cover accept-and-consume and message/opening mutation rejection",
            next_test: "add mutation/property tests and a storage-level monotonicity model for wallet/blockchain integration",
            proof_status: "implementation-hardening task, not just algebra",
        },
    ]
}

fn extraction_soundness_closure_report() -> ExtractionSoundnessClosureReport {
    ExtractionSoundnessClosureReport {
        game: "PackedFullResponseExtractionGame",
        accepted_equation_surface: "CompressedLinearSigmaAlgebra.verifyFields over proof_commitment[32] || masked_witness_response[41]",
        extractor_input: "an accepting packed full-response sigma equation bound to the public opening statement digest",
        extractor_output: "a canonical 23-residue representative whose 12-residue q-syndrome is the statement digest",
        closed_obligations: vec![
            "41-byte response parser enforces canonical packed q residues and zero slack bits",
            "full_response_syndrome_digest recomputes the deterministic 12x23 q-syndrome",
            "stateful verifier bridge consumes the quotient digest path rather than the legacy SHA-preimage path",
            "extractionFailure is the explicit Lean break event consumed by noExtractionBreak",
        ],
        fork_extraction_surface: "FullResponseForkExtractionCertificate",
        fork_closed_obligations: vec![
            "fork statements keep public key, digest, active responses, and openings fixed",
            "only the challenge field changes between fork branches",
            "two accepting transcripts share the same proof commitment",
            "a successful algebraic fork yields a response hitting the statement digest or the named fork-extractor break",
            "rank-12 right-inverse lift constructs a canonical 41-byte response for every extracted 12-residue digest",
            "byte-level quotient response add/subtract/scale realizes the Fq12 fork equation while preserving canonical 41-byte decoding",
        ],
        fork_remaining_assumption: "account for Fiat-Shamir/QROM fork generation and the q^-12 action-collision bad event; the algebraic response manipulation is now executable",
        remaining_assumption: "Fiat-Shamir/QROM fork generation plus concrete multi-query advantage accounting for the selected 12x23 matrix and challenge distribution",
        break_event: "ConcreteFullResponseQuotientReductionAssumptions.extractionFailure",
        review_status: "single-transcript plumbing, action invertibility, canonical digest lifting, and byte-level fork response arithmetic are closed; extraction is now narrowed to Fiat-Shamir/QROM fork generation",
    }
}

fn fork_extraction_review_report() -> ForkExtractionReviewReport {
    ForkExtractionReviewReport {
        lean_surface: "FullResponseForkExtractionCertificate",
        fork_shape: "two accepting 73-byte transcripts with the same proof commitment and same public opening statement except for challenge bytes",
        same_statement_fields: vec![
            "public_key",
            "slot digest / quotient digest",
            "active responses",
            "local openings",
            "proof commitment",
        ],
        differing_field: "21-byte Fiat-Shamir challenge",
        accepted_equations: vec![
            "relationMap(statement0, z0) = a + c0 * digest",
            "relationMap(statement1, z1) = a + c1 * digest",
        ],
        extractor_claim: "hash each 21-byte challenge to a 12-residue element of F_{q^12}; if the two action vectors differ, their difference is invertible in the extension field and subtracting the fork equations extracts a response whose relation map is the statement digest",
        break_event: "forkExtractorBreak or ConcreteFullResponseQuotientReductionAssumptions.extractionFailure",
        fs_forking_dependency: "show that a classical accepting Fiat-Shamir adversary yields such a same-commitment fork with the stated query loss",
        qrom_dependency: "for quantum attackers, replace classical rewinding with the selected QROM measure-and-reprogram/forking lemma and account for its loss",
        concrete_review_target: "account for same-commitment fork generation and the hash-to-F_{q^12} challenge-action collision bound q^-12",
    }
}

fn challenge_action_invertibility_report() -> ChallengeActionInvertibilityReport {
    let polynomial = CHALLENGE_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH;
    let x_q12_minus_x = poly_x_q_power_minus_x(12, &polynomial);
    let gcd_x_q6_minus_x = poly_gcd(&polynomial, &poly_x_q_power_minus_x(6, &polynomial));
    let gcd_x_q4_minus_x = poly_gcd(&polynomial, &poly_x_q_power_minus_x(4, &polynomial));
    let rabin_x_q12_minus_x_zero = x_q12_minus_x.is_empty();
    let rabin_gcd_x_q6_minus_x_degree = poly_degree(&gcd_x_q6_minus_x);
    let rabin_gcd_x_q4_minus_x_degree = poly_degree(&gcd_x_q4_minus_x);
    let rabin_irreducible = rabin_x_q12_minus_x_zero
        && rabin_gcd_x_q6_minus_x_degree == 0
        && rabin_gcd_x_q4_minus_x_degree == 0;
    let proof_commitment = sample_proof_commitment();
    let frame0 = challenge_hash_frame(
        &reference_public_key(),
        b"challenge action sample 0",
        0,
        &[0_u8; DIGEST_BYTES],
        &proof_commitment,
    );
    let frame1 = challenge_hash_frame(
        &reference_public_key(),
        b"challenge action sample 1",
        0,
        &[1_u8; DIGEST_BYTES],
        &proof_commitment,
    );
    let challenge0 = hash_frame_to_challenge_bytes(&frame0);
    let challenge1 = hash_frame_to_challenge_bytes(&frame1);
    let action0 = challenge_action_vector(&challenge0);
    let action1 = challenge_action_vector(&challenge1);
    ChallengeActionInvertibilityReport {
        action_name: "hash-to-Fq12 challenge action",
        challenge_bytes: CHALLENGE_BYTES,
        action_residues: CHALLENGE_ACTION_RESIDUES,
        modulus: Q,
        model: "the 21-byte Fiat-Shamir challenge is domain-separated and rejection-sampled into 12 q-residues, then interpreted as an element of F_q[X]/p(X)",
        extension_polynomial: "p(X) = X^12 + X + 25",
        extension_polynomial_coefficients_low_to_high: polynomial.to_vec(),
        extension_polynomial_degree: poly_degree(&polynomial),
        extension_polynomial_monic: *polynomial.last().unwrap() == 1,
        irreducibility_status: "closed by executable Rabin test: x^(q^12)-x is zero modulo p, and gcd(p, x^(q^6)-x) = gcd(p, x^(q^4)-x) = 1",
        rabin_x_q12_minus_x_zero,
        rabin_gcd_x_q6_minus_x_degree,
        rabin_gcd_x_q4_minus_x_degree,
        rabin_irreducible,
        invertibility_rule: "two fork challenges are algebraically extractable whenever their 12-residue action vectors differ",
        bad_difference_event: "challenge_action_vector(c0) == challenge_action_vector(c1)",
        bad_difference_probability_formula: "1 / 12289^12 in the ideal hash-to-Fq12 model",
        bad_difference_denominator_decimal: decimal_pow_mul(Q as u32, CHALLENGE_ACTION_RESIDUES, 1),
        bad_difference_floor_bits: challenge_action_log2_floor(),
        generic_quantum_margin_for_128_bits: challenge_action_quantum_margin_for_128(),
        sample_challenge0_hex: hex_encode(&challenge0),
        sample_challenge1_hex: hex_encode(&challenge1),
        sample_action0: action0.to_vec(),
        sample_action1: action1.to_vec(),
        sample_actions_distinct: action0 != action1,
        conclusion: "using the irreducible polynomial X^12 + X + 25 makes nonzero Fq12 action differences invertible; the algebraic bad-difference event is reduced to a q^-12 action collision",
    }
}

fn fork_equation_algebra_report() -> ForkEquationAlgebraReport {
    let proof_commitment_bytes = sample_proof_commitment();
    let frame0 = challenge_hash_frame(
        &reference_public_key(),
        b"fork equation algebra sample 0",
        0,
        &[0_u8; DIGEST_BYTES],
        &proof_commitment_bytes,
    );
    let frame1 = challenge_hash_frame(
        &reference_public_key(),
        b"fork equation algebra sample 1",
        0,
        &[1_u8; DIGEST_BYTES],
        &proof_commitment_bytes,
    );
    let challenge0 = hash_frame_to_challenge_bytes(&frame0);
    let challenge1 = hash_frame_to_challenge_bytes(&frame1);
    let action0 = challenge_action_vector(&challenge0);
    let action1 = challenge_action_vector(&challenge1);
    let action_difference = fq_action_sub(&action0, &action1);
    let inverse =
        fq_action_inverse(&action_difference).unwrap_or([0_u16; CHALLENGE_ACTION_RESIDUES]);
    let inverse_check = fq_action_mul(&action_difference, &inverse);

    let mask_view = [12_u16, 12_277_u16];
    let masked_witness_response = sample_full_response_masked_witness_for_mask(mask_view);
    let response_vector = full_response_vector_from_masked_witness(&masked_witness_response)
        .expect("sample full response must decode");
    let digest = full_response_syndrome_array(&response_vector);
    let proof_commitment = [
        3, 19, 41, 97, 211, 487, 997, 2017, 4099, 8191, 10_001, 11_111,
    ];

    let scaled0 = fq_action_mul(&action0, &digest);
    let scaled1 = fq_action_mul(&action1, &digest);
    let mapped0 = fq_action_add(&proof_commitment, &scaled0);
    let mapped1 = fq_action_add(&proof_commitment, &scaled1);
    let fork_difference = fq_action_sub(&mapped0, &mapped1);
    let scaled_difference = fq_action_mul(&action_difference, &digest);
    let extracted_digest = fq_action_mul(&inverse, &fork_difference);

    ForkEquationAlgebraReport {
        lean_surface: "FullResponseForkEquationAlgebraCertificate",
        equation_model: "relationMap(statement_i, z_i) = proof_commitment + challenge_action_i * digest in F_{12289}[X]/(X^12 + X + 25)",
        extraction_formula: "digest = (challenge_action_0 - challenge_action_1)^-1 * (relationMap(z_0) - relationMap(z_1))",
        modulus: Q,
        extension_polynomial: "p(X) = X^12 + X + 25",
        action_difference_nonzero: action_difference.iter().any(|&entry| entry != 0),
        inverse_exists: fq_action_inverse(&action_difference).is_some(),
        inverse_check_is_one: inverse_check == fq_action_one(),
        fork_difference_matches_scaled_digest: fork_difference == scaled_difference,
        extracted_digest_matches: extracted_digest == digest,
        sample_digest: digest.to_vec(),
        sample_action0: action0.to_vec(),
        sample_action1: action1.to_vec(),
        sample_action_difference: action_difference.to_vec(),
        sample_action_difference_inverse: inverse.to_vec(),
        sample_extracted_digest: extracted_digest.to_vec(),
        conclusion: "for the concrete Fq12 action, a nonzero fork action difference lets the extractor divide the subtracted fork equations and recover the statement digest exactly",
    }
}

fn canonical_response_lift_report() -> CanonicalResponseLiftReport {
    let mask_view = [12_u16, 12_277_u16];
    let masked_witness_response = sample_full_response_masked_witness_for_mask(mask_view);
    let response_vector = full_response_vector_from_masked_witness(&masked_witness_response)
        .expect("sample full response must decode");
    let input_digest = full_response_syndrome_array(&response_vector);
    let input_digest_bytes = pack_q_syndrome_digest(&input_digest);
    let lifted_vector =
        solve_full_response_syndrome(&input_digest).expect("rank-12 matrix must lift syndrome");
    let lifted_response = masked_witness_from_full_response_vector(&lifted_vector);
    let decoded = full_response_vector_from_masked_witness(&lifted_response);
    let lifted_syndrome = decoded
        .as_ref()
        .map(full_response_syndrome_array)
        .unwrap_or([0_u16; SIS_FULL_RESPONSE_PUBLIC_ROWS]);
    let lifted_digest_bytes = pack_q_syndrome_digest(&lifted_syndrome);
    let quotient_certificate = full_response_quotient_algebra_certificate();

    CanonicalResponseLiftReport {
        lean_surface: "FullResponseCanonicalResponseLiftCertificate",
        lift_name: "rank-12 right-inverse canonical full-response lift",
        modulus: Q,
        matrix_rows: SIS_FULL_RESPONSE_PUBLIC_ROWS,
        matrix_cols: SIS_FULL_RESPONSE_RESIDUES,
        matrix_rank: quotient_certificate.rank,
        response_bytes: LINEAR_SIGMA_MASKED_WITNESS_BYTES,
        response_residues: SIS_FULL_RESPONSE_RESIDUES,
        response_packed_bits: SIS_FULL_RESPONSE_PACKED_BITS,
        response_slack_bits: SIS_FULL_RESPONSE_SLACK_BITS,
        right_inverse_check: quotient_certificate.right_inverse_check,
        sample_input_digest: input_digest.to_vec(),
        sample_input_digest_hex: hex_encode(&input_digest_bytes),
        lifted_vector: lifted_vector.to_vec(),
        lifted_response_hex: hex_encode(&lifted_response),
        decoded_is_canonical: decoded.is_some(),
        all_residues_lt_modulus: lifted_vector.iter().all(|&value| value < Q),
        slack_bits_zero: read_bits_le(
            &lifted_response,
            SIS_FULL_RESPONSE_PACKED_BITS,
            SIS_FULL_RESPONSE_SLACK_BITS,
        ) == 0,
        lifted_syndrome_matches_input: lifted_syndrome == input_digest,
        packed_digest_matches_input: lifted_digest_bytes == input_digest_bytes,
        conclusion: "the 12x23 matrix is rank-12, so every extracted 12-residue digest has a canonical 41-byte response representative whose relation map repacks to the same digest",
    }
}

fn canonical_response_lift(
    target: &[u16; SIS_FULL_RESPONSE_PUBLIC_ROWS],
) -> [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES] {
    let vector = solve_full_response_syndrome(target)
        .expect("rank-12 matrix must lift every 12-residue syndrome");
    masked_witness_from_full_response_vector(&vector)
}

fn response_syndrome(
    response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Option<[u16; SIS_FULL_RESPONSE_PUBLIC_ROWS]> {
    let vector = full_response_vector_from_masked_witness(response)?;
    Some(full_response_syndrome_array(&vector))
}

fn response_add_by_syndrome(
    lhs: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
    rhs: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Option<[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES]> {
    Some(canonical_response_lift(&fq_action_add(
        &response_syndrome(lhs)?,
        &response_syndrome(rhs)?,
    )))
}

fn response_sub_by_syndrome(
    lhs: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
    rhs: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Option<[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES]> {
    Some(canonical_response_lift(&fq_action_sub(
        &response_syndrome(lhs)?,
        &response_syndrome(rhs)?,
    )))
}

fn response_scale_by_action(
    action: &[u16; CHALLENGE_ACTION_RESIDUES],
    response: &[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
) -> Option<[u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES]> {
    Some(canonical_response_lift(&fq_action_mul(
        action,
        &response_syndrome(response)?,
    )))
}

fn byte_response_arithmetic_report() -> ByteResponseArithmeticReport {
    let proof_commitment = sample_proof_commitment();
    let frame0 = challenge_hash_frame(
        &reference_public_key(),
        b"byte response arithmetic sample 0",
        0,
        &[0_u8; DIGEST_BYTES],
        &proof_commitment,
    );
    let frame1 = challenge_hash_frame(
        &reference_public_key(),
        b"byte response arithmetic sample 1",
        0,
        &[1_u8; DIGEST_BYTES],
        &proof_commitment,
    );
    let challenge0 = hash_frame_to_challenge_bytes(&frame0);
    let challenge1 = hash_frame_to_challenge_bytes(&frame1);
    let action0 = challenge_action_vector(&challenge0);
    let action1 = challenge_action_vector(&challenge1);
    let action_difference = fq_action_sub(&action0, &action1);
    let inverse = fq_action_inverse(&action_difference).expect("sample actions must differ");

    let mask_view = [12_u16, 12_277_u16];
    let witness_response = sample_full_response_masked_witness_for_mask(mask_view);
    let digest = response_syndrome(&witness_response).expect("sample witness must decode");
    let proof_commitment_syndrome = [
        3, 19, 41, 97, 211, 487, 997, 2017, 4099, 8191, 10_001, 11_111,
    ];
    let proof_commitment_response = canonical_response_lift(&proof_commitment_syndrome);

    let scaled0 = response_scale_by_action(&action0, &witness_response).unwrap();
    let scaled1 = response_scale_by_action(&action1, &witness_response).unwrap();
    let z0 = response_add_by_syndrome(&proof_commitment_response, &scaled0).unwrap();
    let z1 = response_add_by_syndrome(&proof_commitment_response, &scaled1).unwrap();
    let difference_response = response_sub_by_syndrome(&z0, &z1).unwrap();
    let extracted_response = response_scale_by_action(&inverse, &difference_response).unwrap();

    let z0_syndrome = response_syndrome(&z0).unwrap();
    let z1_syndrome = response_syndrome(&z1).unwrap();
    let difference_syndrome = response_syndrome(&difference_response).unwrap();
    let extracted_syndrome = response_syndrome(&extracted_response).unwrap();
    let expected_z0 = fq_action_add(
        &proof_commitment_syndrome,
        &fq_action_mul(&action0, &digest),
    );
    let expected_z1 = fq_action_add(
        &proof_commitment_syndrome,
        &fq_action_mul(&action1, &digest),
    );
    let expected_difference = fq_action_mul(&action_difference, &digest);
    let extracted_digest_bytes = pack_q_syndrome_digest(&extracted_syndrome);
    let digest_bytes = pack_q_syndrome_digest(&digest);

    ByteResponseArithmeticReport {
        lean_surface: "FullResponseByteResponseArithmeticCertificate",
        operation_model: "response add/sub/scale are quotient operations: compute the 12-residue syndrome, perform Fq12 arithmetic, then lift through the rank-12 canonical response lift",
        response_bytes: LINEAR_SIGMA_MASKED_WITNESS_BYTES,
        action_residues: CHALLENGE_ACTION_RESIDUES,
        z0_is_canonical: full_response_vector_from_masked_witness(&z0).is_some(),
        z1_is_canonical: full_response_vector_from_masked_witness(&z1).is_some(),
        difference_response_is_canonical: full_response_vector_from_masked_witness(
            &difference_response,
        )
        .is_some(),
        extracted_response_is_canonical: full_response_vector_from_masked_witness(
            &extracted_response,
        )
        .is_some(),
        z0_equation_holds: z0_syndrome == expected_z0,
        z1_equation_holds: z1_syndrome == expected_z1,
        response_difference_matches_field_difference: difference_syndrome == expected_difference,
        inverse_scaled_response_hits_digest: extracted_syndrome == digest,
        extracted_response_packed_digest_matches: extracted_digest_bytes == digest_bytes,
        sample_proof_commitment_syndrome: proof_commitment_syndrome.to_vec(),
        sample_digest: digest.to_vec(),
        sample_z0_syndrome: z0_syndrome.to_vec(),
        sample_z1_syndrome: z1_syndrome.to_vec(),
        sample_difference_syndrome: difference_syndrome.to_vec(),
        sample_extracted_syndrome: extracted_syndrome.to_vec(),
        sample_extracted_response_hex: hex_encode(&extracted_response),
        conclusion: "the byte-level response operations used by the quotient proof preserve canonical 41-byte decoding while realizing the same Fq12 fork subtraction and inverse-scaling equation on the public syndrome",
    }
}

fn fiat_shamir_fork_accounting_report() -> FiatShamirForkAccountingReport {
    let action_collision_floor_bits = challenge_action_log2_floor();
    let action_collision_quantum_query_bound_log2_for_128 =
        challenge_action_quantum_margin_for_128();
    let action_collision_classical_query_bound_log2_for_128 =
        action_collision_floor_bits.saturating_sub(128);
    let bottleneck_quantum_query_bound_log2_for_128 =
        FS_QUANTUM_QUERY_BITS_FOR_128.min(action_collision_quantum_query_bound_log2_for_128);
    let bottleneck_classical_query_bound_log2_for_128 =
        FS_CLASSICAL_QUERY_BITS_FOR_128.min(action_collision_classical_query_bound_log2_for_128);

    FiatShamirForkAccountingReport {
        lean_surface: "FullResponseFiatShamirForkAccountingCertificate",
        accounting_scope: "probabilistic same-commitment fork generation plus action-collision bad event after the algebraic fork extractor is closed",
        challenge_entropy_bits: CHALLENGE_ENTROPY_BITS,
        algebraic_challenge_entropy_bits: ALGEBRAIC_CHALLENGE_ENTROPY_BITS,
        transcript_hardening_bits: CHALLENGE_HARDENING_BYTES * 8,
        action_collision_floor_bits,
        fs_classical_query_bound_log2_for_128: FS_CLASSICAL_QUERY_BITS_FOR_128,
        fs_quantum_query_bound_log2_for_128: FS_QUANTUM_QUERY_BITS_FOR_128,
        action_collision_classical_query_bound_log2_for_128,
        action_collision_quantum_query_bound_log2_for_128,
        bottleneck_classical_query_bound_log2_for_128,
        bottleneck_quantum_query_bound_log2_for_128,
        same_commitment_fork_generation_dependency: "classical forking lemma or QROM measure-and-reprogram lemma must produce two accepting transcripts with the same proof commitment and same prechallenge statement frame",
        qrom_dependency: "the selected QROM lemma must charge oracle queries against the 162-bit transcript challenge, then expose the q^-12 action-collision event separately",
        bad_event_terms: vec![
            "Fiat-Shamir frame/query miss or reprogramming failure over the 162-bit challenge",
            "same-commitment replay/fork generation failure",
            "challenge_action_vector(c0) == challenge_action_vector(c1), bounded by 1/12289^12 in the ideal hash-to-Fq12 model",
        ],
        algebraic_fork_path_closed: true,
        action_collision_budget_meets_128_at_bottleneck: query_loss_quantum(
            action_collision_floor_bits,
            bottleneck_quantum_query_bound_log2_for_128,
        ) >= 128
            && query_loss_classical(
                action_collision_floor_bits,
                bottleneck_classical_query_bound_log2_for_128,
            ) >= 128,
        fs_budget_meets_128_at_bottleneck: query_loss_quantum(
            CHALLENGE_ENTROPY_BITS,
            bottleneck_quantum_query_bound_log2_for_128,
        ) >= 128
            && query_loss_classical(
                CHALLENGE_ENTROPY_BITS,
                bottleneck_classical_query_bound_log2_for_128,
            ) >= 128,
        conclusion: "with the algebraic fork extractor closed, the deployable proof reduces to a same-commitment Fiat-Shamir/QROM fork-generation lemma plus explicit bad-event accounting; the hardened 12-residue action raises the generic quantum action-collision margin to 2^17, so the bottleneck is no longer the action field",
    }
}

fn fiat_shamir_fork_generation_ledger() -> FiatShamirForkGenerationLedger {
    let action_collision_floor_bits = challenge_action_log2_floor();
    let single_term_quantum_query_bound_log2_for_128 =
        FS_QUANTUM_QUERY_BITS_FOR_128.min(challenge_action_quantum_margin_for_128());
    let single_term_classical_query_bound_log2_for_128 =
        FS_CLASSICAL_QUERY_BITS_FOR_128.min(action_collision_floor_bits.saturating_sub(128));
    let union_bound_reserved_bits = 1;
    let union_safe_quantum_query_bound_log2_for_128 =
        single_term_quantum_query_bound_log2_for_128.saturating_sub(union_bound_reserved_bits);
    let union_safe_classical_query_bound_log2_for_128 =
        single_term_classical_query_bound_log2_for_128.saturating_sub(union_bound_reserved_bits);
    let transcript_quantum_bits_at_union_bound = query_loss_quantum(
        CHALLENGE_ENTROPY_BITS,
        union_safe_quantum_query_bound_log2_for_128,
    );
    let action_collision_quantum_bits_at_union_bound = query_loss_quantum(
        action_collision_floor_bits,
        union_safe_quantum_query_bound_log2_for_128,
    );
    let transcript_classical_bits_at_union_bound = query_loss_classical(
        CHALLENGE_ENTROPY_BITS,
        union_safe_classical_query_bound_log2_for_128,
    );
    let action_collision_classical_bits_at_union_bound = query_loss_classical(
        action_collision_floor_bits,
        union_safe_classical_query_bound_log2_for_128,
    );

    FiatShamirForkGenerationLedger {
        lean_surface: "FullResponseFiatShamirForkGenerationLedger",
        accounting_scope: "deployment-safe bad-event ledger for same-commitment Fiat-Shamir fork generation after algebraic extraction is closed",
        selected_qrom_theorem: "Don-Fehr-Majenz measure-and-reprogram single-input Fiat-Shamir theorem",
        qrom_theorem_source: "Don, Fehr, Majenz, Schaffner, Security of the Fiat-Shamir Transformation in the Quantum Random-Oracle Model; Don, Fehr, Majenz, The Measure-and-Reprogram Technique 2.0, Theorem 3",
        qrom_loss_model: "quadratic QROM loss: success is charged by a (2q + 1)^2 denominator for q quantum random-oracle queries",
        qrom_hash_query_bound_log2: union_safe_quantum_query_bound_log2_for_128,
        qrom_quadratic_loss_bits: 2 * union_safe_quantum_query_bound_log2_for_128,
        qrom_measured_oracle_input: "deployed proof frame H(public_key, message, slot_id, slot_digest, proof_commitment) with domain-separated profile and parameter labels",
        qrom_reprogrammed_output: "the 21-byte Fiat-Shamir challenge, interpreted as 17 algebraic bytes plus 4 transcript-hardening bytes",
        qrom_same_commitment_condition: "satisfied by the proof-bound frame: the quotient proof commitment is a first message for the 73-byte proof and is included in challenge_hash_frame before challenge derivation",
        qrom_theorem_fits_protocol_shape: true,
        single_term_quantum_query_bound_log2_for_128,
        single_term_classical_query_bound_log2_for_128,
        union_safe_quantum_query_bound_log2_for_128,
        union_safe_classical_query_bound_log2_for_128,
        transcript_challenge_bits: CHALLENGE_ENTROPY_BITS,
        action_collision_floor_bits,
        transcript_quantum_bits_at_union_bound,
        action_collision_quantum_bits_at_union_bound,
        transcript_classical_bits_at_union_bound,
        action_collision_classical_bits_at_union_bound,
        union_bound_terms: vec![
            "Fiat-Shamir frame/query miss or QROM reprogramming failure",
            "same-commitment fork-generation loss from the selected classical/QROM lemma",
            "Fq12 action collision after two challenge bytes are forked",
        ],
        union_bound_reserved_bits,
        union_bound_meets_128: transcript_quantum_bits_at_union_bound >= 129
            && action_collision_quantum_bits_at_union_bound >= 129
            && transcript_classical_bits_at_union_bound >= 129
            && action_collision_classical_bits_at_union_bound >= 129,
        same_commitment_fork_generation_assumption: "the fork lemma must output two accepting transcripts with identical prechallenge frame and proof commitment; its success-loss is charged outside the hash-collision bad-event ledger",
        qrom_fork_generation_assumption: "for quantum adversaries, use the selected measure-and-reprogram theorem at the union-safe 2^16 scale over the proof-commitment-bound challenge frame",
        conclusion: "the numeric ledger and proof-bound frame now satisfy the intended same-commitment QROM theorem shape; the remaining deployment blocker is integrating the 73-byte proof into the public verifier path",
    }
}

fn stateful_lifecycle_policy() -> StatefulLifecyclePolicyReport {
    StatefulLifecyclePolicyReport {
        policy_name: "monotonic one-signature-per-slot account state",
        slot_identifier_bytes: SLOT_ID_BYTES,
        slot_creation_rule: "keygen creates a finite vector of account slots; each slot stores a quotient digest and starts unused",
        signing_rule: "signing selects one unused slot, derives the Fiat-Shamir challenge from slot id, public key, message, and slot digest, and must atomically reserve/consume the slot before releasing the signature",
        verification_rule: "verification accepts only if the slot exists, is unused in the verifier state, the challenge frame matches, and the local opening equations hold",
        consumption_rule: "an accepted signature transitions the slot to consumed and preserves its digest for audit",
        concurrency_rule: "wallet implementations must serialize signing per slot or use compare-and-swap reservation; two concurrent signatures from the same slot are forbidden",
        crash_consistency_rule: "durable state must mark a slot reserved/consumed before exposing the signature bytes; recovery treats reserved-unknown slots as consumed",
        backup_restore_rule: "restoring an old account snapshot risks slot reuse; deployments need monotonic chain-observed consumption or hardware/ledger-enforced counters",
        reorg_rule: "a signature revealed on any fork burns the slot even if the transaction is later reorged out",
        tests_cover: vec![
            "accept-and-consume transition",
            "consumed-slot replay rejection",
            "message mutation rejection",
            "opening mutation rejection",
            "slot id mutation rejection",
        ],
        remaining_deployment_work: "integrate these rules into wallet/storage APIs and chain mempool policy; the reference code models the transition but is not durable storage",
    }
}

fn reference_implementation_report() -> ReferenceImplementationReport {
    let keypair = reference_keygen(2);
    let message = b"reference implementation known answer";
    let signature = reference_sign_with_proof(&keypair, 0, message).unwrap();
    let (verify_report, next_account) =
        reference_verify_with_proof(&keypair.public_key, message, &keypair.account, &signature);
    let (replay_report, _) =
        reference_verify_with_proof(&keypair.public_key, message, &next_account, &signature);
    let slot_digest = keypair.account.slots[0].digest;
    ReferenceImplementationReport {
        api_surface: vec![
            "reference_keygen(slot_count)",
            "reference_sign_with_proof(keypair, slot_id, message)",
            "reference_verify_with_proof(public_key, message, account, signature)",
            "verify_public_quotient_proof_signature(public_key, message, account, signature)",
        ],
        keygen_slots: 2,
        public_key_bytes: keypair.public_key.len(),
        account_slots: keypair.account.slots.len(),
        signature_bytes: signature.len(),
        verifies: verify_report.accepted,
        consumes_slot: next_account.slots[0].unused == false,
        rejects_replay: !replay_report.accepted
            && replay_report.reason == "slot is already consumed",
        known_answer_public_key_sha256: sha256_hex(&keypair.public_key),
        known_answer_slot_digest_hex: hex_encode(&slot_digest),
        known_answer_signature_sha256: sha256_hex(&signature),
        negative_tests: vec![
            "replay with consumed slot",
            "mutated message",
            "mutated opening byte",
            "mutated slot id",
            "non-canonical parser payloads",
        ],
        status: "full 264-byte public proof API is executable and deterministic for test vectors; it remains a research reference, not hardened wallet storage",
    }
}

fn production_reference_api_report() -> ProductionReferenceApiReport {
    let keypair = production_keygen(2);
    let message = b"production reference api known answer";
    let signature = production_sign(
        &keypair.secret_key,
        keypair.account.get_slot(0).unwrap(),
        message,
    )
    .expect("production facade signs reference slot");
    let (verify_report, next_account) =
        production_verify(&keypair.public_key, message, &keypair.account, &signature);
    let (replay_report, _) =
        production_verify(&keypair.public_key, message, &next_account, &signature);
    let mut mutated_account = keypair.account.clone();
    let mut mutated_commitment = *mutated_account.get_slot(0).unwrap().commitment().as_bytes();
    mutated_commitment[DIGEST_BYTES] ^= 1;
    mutated_account
        .replace_slot_commitment_for_test(
            0,
            ProductionAccountCommitment::from_bytes(mutated_commitment)
                .expect("randomizer mutation remains canonical"),
        )
        .unwrap();
    let (randomizer_report, _) =
        production_verify(&keypair.public_key, message, &mutated_account, &signature);
    ProductionReferenceApiReport {
        api_surface: vec![
            "production_keygen(slot_count)",
            "account_state_from_slot_seeds(slot_seeds, randomizer_commitment)",
            "production_sign(secret_key, account_slot, message)",
            "production_verify(public_key, account_state, message, signature)",
            "legacy_account_from_production(account_state)",
        ],
        account_commitment_bytes: REVIEWED_ACCOUNT_LINK_BYTES,
        account_commitment_layout: vec![
            "witness commitment: 32 bytes",
            "randomizer commitment: 32 bytes",
            "response-link code [0,3]: 4 bytes",
        ],
        target_transaction_signature_bytes: REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES,
        executable_signature_bytes: signature.len(),
        executable_matches_target: signature.len() == REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES,
        keygen_slots: keypair.account.len(),
        verifies: verify_report.accepted,
        consumes_slot: !next_account.get_slot(0).unwrap().is_unused(),
        rejects_replay: !replay_report.accepted
            && replay_report.reason == "slot is already consumed",
        binds_account_randomizer_commitment: !randomizer_report.accepted
            && randomizer_report.reason == "account randomizer commitment mismatch",
        known_answer_account_commitment_sha256: sha256_hex(
            keypair.account.get_slot(0).unwrap().commitment().as_bytes(),
        ),
        known_answer_signature_sha256: sha256_hex(signature.as_bytes()),
        status: "production-shaped facade is executable and exposes the reviewed 68-byte account-link layout, but it still wraps the 254-byte research proof path until the selected 284-byte/Fq12 verifier has accepting hidden-source proof semantics",
    }
}

fn third_party_review_packet() -> ThirdPartyReviewPacket {
    let keypair = reference_keygen(2);
    let message = b"reference implementation known answer";
    let signature = reference_sign_with_proof(&keypair, 0, message).unwrap();
    let slot_digest = keypair.account.slots[0].digest;
    let proof_start = STATEFUL_TRANSACTION_SIGNATURE_BYTES;
    let proof = decode_linear_sigma_target_proof(&signature[proof_start..]).unwrap();
    let frame = challenge_hash_frame(
        &keypair.public_key,
        message,
        0,
        &slot_digest,
        &proof.proof_commitment,
    );
    let challenge = hash_frame_to_challenge_bytes(&frame);
    let proof_bytes = encode_linear_sigma_target_proof(&proof);
    ThirdPartyReviewPacket {
        packet_name: "Descender q12289 d=12 stateful quotient signature review packet",
        target_profile: PROFILE,
        signature_bytes: LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES,
        public_key_bytes: PK_BYTES,
        exact_sha256_assumptions: vec![
            "SHA-256 modeled as a QROM random oracle for the fixed proof-bound challenge frame domain, including public key, message, slot id, slot digest, and quotient proof commitment",
            "SHA-256 modeled as a PRF/RO-derived stream for pad expansion seed || padFrame(block) || u32 counter",
            "SHA-256 rejection sampling for the 12x23 matrix uses the fixed matrix domain, row, column, and u32 counter encodings",
            "The 32-byte account digest is a packed q-syndrome for the quotient verifier path, not a generic collision-resistant commitment to every hidden byte",
        ],
        extraction_break_target: "Given a same-commitment QROM fork with non-colliding Fq12 actions, failure to extract a canonical 41-byte response opening the 12-residue digest is exactly the PackedFullResponseExtractionGame extractionBreak target.",
        matrix_sampler_review: vec![
            "domain string: Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1",
            "dimensions: 12 rows x 23 columns over q = 12289",
            "entry sampler: SHA-256 rejection sampling with cutoff 61445 and u32 little-endian counter",
            "certificate checks: transcript hash, all entries < q, rank 12, 11-vector kernel basis, right inverse, zero syndromes for kernel vectors",
        ],
        known_answer_vectors: vec![
            KnownAnswerVector {
                name: "public_key_sha256",
                bytes: keypair.public_key.len(),
                sha256_or_hex: sha256_hex(&keypair.public_key),
                review_role: "detects public-key serialization drift",
            },
            KnownAnswerVector {
                name: "slot0_digest_hex",
                bytes: DIGEST_BYTES,
                sha256_or_hex: hex_encode(&slot_digest),
                review_role: "detects quotient digest/matrix drift",
            },
            KnownAnswerVector {
                name: "challenge_frame_sha256",
                bytes: frame.len(),
                sha256_or_hex: sha256_hex(&frame),
                review_role: "detects Fiat-Shamir frame drift",
            },
            KnownAnswerVector {
                name: "challenge_hex",
                bytes: challenge.len(),
                sha256_or_hex: hex_encode(&challenge),
                review_role: "detects 21-byte challenge derivation drift",
            },
            KnownAnswerVector {
                name: "stateful_signature_sha256",
                bytes: signature.len(),
                sha256_or_hex: sha256_hex(&signature),
                review_role: "detects reference signing drift",
            },
            KnownAnswerVector {
                name: "linear_sigma_proof_sha256",
                bytes: proof_bytes.len(),
                sha256_or_hex: sha256_hex(&proof_bytes),
                review_role: "detects 73-byte quotient proof layout drift",
            },
        ],
        fuzz_property_tests: vec![
            "challenge parser rejects malformed lengths, out-of-range active blocks, and forbidden high challenge bits",
            "response parser rejects malformed lengths and response coordinates outside [-48, 48]",
            "opening parser rejects malformed lengths and noncanonical q residues",
            "full-response parser rejects noncanonical q residues and nonzero slack bits",
            "public proof verifier rejects mutations to slot id, algebraic challenge bytes, hardening bytes, public key, message, responses, openings, quotient proof commitment, masked proof response, and consumed-slot replay",
            "strict sigma-equation verifier rejects the old fixed-commitment representative forgery, while public_sigma_equation_forgery_probe demonstrates that an adapted post-challenge public representative still verifies",
            "linear-sigma proof parser rejects malformed proof lengths and preserves the 32-byte commitment / 41-byte response split",
        ],
        constant_time_status: "not complete: reference code is review instrumentation and branches on secret-derived rejection predicates; production signer still needs constant-time/side-channel review",
        implementation_limitations: vec![
            "reference implementation is deterministic test scaffolding, not a hardened wallet",
            "full 254-byte representative proof path is executable; the strict sigma-equation verifier is implemented, but the current local-opening relation admits an adapted public sigma-equation forgery",
            "storage semantics require monotonic slot consumption, crash-safe reservation, and backup/restore protection",
            "SHA-256 RO/PRF assumptions and packed-SIS extraction break still require external cryptographic review",
            "sampler zero-TV proof depends on excluding active pad outputs from the rejection predicate",
        ],
        review_gate_status: "strict linear sigma equation is executable, but the current local-opening relation is still not sound: an adapted public conditioned-fiber sigma-equation forgery verifies without private witness material",
    }
}

fn internal_security_review_audit() -> InternalSecurityReviewAudit {
    InternalSecurityReviewAudit {
        audit_name: "internal pre-external-review cryptoanalysis audit",
        target_profile: PROFILE,
        verdict: "byte-path blockers closed: the proof-commitment challenge frame and full 254-byte public verifier are executable; security remains assumption-parametric pending cryptanalysis and production hardening",
        findings: vec![
            SecurityReviewFinding {
                severity: "closed",
                area: "Fiat-Shamir/QROM frame shape",
                finding: "the deployed proof challenge frame now includes the 32-byte quotient proof commitment as first-message data, so the same-commitment QROM theorem shape is satisfied at the frame-certificate level",
                evidence: "challenge_hash_frame binds domain, profile, q, groups, local_pivot, local_free, repetitions, slot_id, public_key, message, slot_digest, and proof_commitment; fiat_shamir_fork_generation_ledger.qrom_theorem_fits_protocol_shape is true",
                required_closure: "keep the full 254-byte public verifier on this proof-bound frame when issue 2 is closed; the legacy 181-byte ideal verifier remains outside the deployed proof path",
            },
            SecurityReviewFinding {
                severity: "closed",
                area: "public verifier integration",
                finding: "the executable sign/verify API validates the representative-style 254-byte stateful quotient proof path without reading ideal witness material from account state",
                evidence: "reference_implementation_report.signature_bytes equals the 254-byte target; reference_sign_with_proof serializes proof_commitment[32] || masked_witness_response[41], and reference_verify_with_proof checks the proof-bound frame plus public quotient response",
                required_closure: "replace this representative-style path with an accepting strict sigma path before treating the 254-byte API as the deployed verifier",
            },
            SecurityReviewFinding {
                severity: "critical",
                area: "packed-SIS extraction assumption",
                finding: "the old public representative relation admits a public conditioned-fiber forgery, and the adapted strict-sigma relation is still forgeable by solving the public post-challenge quotient equations",
                evidence: "public_proof_forgery_probe.sigma_equation_verifier_accepts is false for the old fixed-commitment attack, but public_sigma_equation_forgery_probe.verifier_accepts is true without private key material or account hidden witness material",
                required_closure: "make acceptance depend on a prechallenge witness-bound sigma commitment or another hidden-witness proof so that public post-challenge representative reconstruction is insufficient",
            },
            SecurityReviewFinding {
                severity: "high",
                area: "quotient witness-hiding sampler discipline",
                finding: "the zero-TV hiding certificate is valid only for the allowed conditioned quotient-fiber sampler; conditioning on active pad outputs would invalidate it",
                evidence: "masked_response_sampler_certificate.active_pad_outputs_conditioned is false and quotient_hiding_sampler_review names active-pad-output exclusion as the implementation obligation",
                required_closure: "enforce the allowed rejection predicate in the production signer API and add tests that active pad equations are checked outside the hidden representative sampler",
            },
            SecurityReviewFinding {
                severity: "high",
                area: "side channels and state lifecycle",
                finding: "reference signing is not constant-time and stateful slot safety is specified but not implemented as durable wallet storage",
                evidence: "third_party_review_packet.constant_time_status is not complete; stateful_lifecycle_policy documents monotonic reserve/consume, crash recovery, concurrency, backup/restore, and reorg rules",
                required_closure: "write a production-shaped constant-time signer/sampler and durable slot manager, then audit secret-dependent branches and failure timing",
            },
            SecurityReviewFinding {
                severity: "medium",
                area: "domain separation drift",
                finding: "the challenge-action domain previously used an fq10 label after the active action field moved to degree 12",
                evidence: "the executable domain has been corrected to Descender.Signature.q12289.local_delta.challenge_action.fq12.v1 and the local field-helper names were renamed to action-neutral names",
                required_closure: "refresh any affected KATs and keep domain labels synchronized with Lean constants",
            },
            SecurityReviewFinding {
                severity: "medium",
                area: "SHA-256 modeling assumptions",
                finding: "the random-oracle, pad-PRF, and matrix-sampler assumptions are explicitly located but remain assumptions rather than standard-model reductions",
                evidence: "third_party_review_packet.exact_sha256_assumptions lists QROM, pad stream, and matrix rejection-sampling assumptions; final_soundness_advantage_formula charges Adv_pad_prf and Adv_frame_mismatch separately",
                required_closure: "prepare a reviewer-facing assumption memo with byte domains, adversary interfaces, query limits, and the intended QROM theorem citation",
            },
        ],
        ready_for_external_review_as: "an assumption-parametric research packet with a proof-bound 254-byte public verifier, executable KATs, parser/mutation tests, and named cryptographic games",
        not_ready_for_external_review_as: "an unconditional deployed-soundness proof or production-hardened wallet implementation",
        recommended_next_steps: vec![
            "deepen packed-SIS/keygen cryptanalysis for the exact 12x23 matrix and accepted-response constraints",
            "freeze the production sampler predicate and constant-time/stateful lifecycle requirements before outside review",
        ],
    }
}

fn blockchain_signature_size_comparison() -> Vec<SignatureSizeBenchmark> {
    vec![
        SignatureSizeBenchmark {
            scheme: "Descender quotient candidate",
            post_quantum: true,
            public_key_bytes: PK_BYTES,
            signature_bytes: LINEAR_SIGMA_TARGET_PROOF_BYTES + STATEFUL_SPEND_SIGNATURE_BYTES,
            transaction_signature_bytes: LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES,
            stateful: true,
            source: "local Lean/Rust target: LocalDeltaByteCommitment.N32Q12289.linearSigmaTargetTransactionSignatureBytes_eq",
            note: "Includes 4-byte slot id, 177-byte hardened stateful spend payload, and 73-byte quotient proof; security still assumption-parametric.",
        },
        SignatureSizeBenchmark {
            scheme: "ML-DSA-44 / Dilithium2",
            post_quantum: true,
            public_key_bytes: 1312,
            signature_bytes: 2420,
            transaction_signature_bytes: 2420,
            stateful: false,
            source: "NIST FIPS 204: https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.204.pdf",
            note: "NIST lattice baseline; stateless and standardized, much larger signatures.",
        },
        SignatureSizeBenchmark {
            scheme: "Falcon-512",
            post_quantum: true,
            public_key_bytes: 897,
            signature_bytes: 666,
            transaction_signature_bytes: 666,
            stateful: false,
            source: "Falcon specification: https://falcon-sign.info/falcon.pdf",
            note: "Compact lattice baseline; implementation complexity and floating-point/sampling concerns remain practical factors.",
        },
        SignatureSizeBenchmark {
            scheme: "SLH-DSA-SHA2-128s / SPHINCS+",
            post_quantum: true,
            public_key_bytes: 32,
            signature_bytes: 7856,
            transaction_signature_bytes: 7856,
            stateful: false,
            source: "NIST FIPS 205: https://csrc.nist.gov/pubs/fips/205/final",
            note: "Hash-based stateless conservative baseline; signatures are much larger.",
        },
        SignatureSizeBenchmark {
            scheme: "Ed25519",
            post_quantum: false,
            public_key_bytes: 32,
            signature_bytes: 64,
            transaction_signature_bytes: 64,
            stateful: false,
            source: "RFC 8032: https://www.rfc-editor.org/rfc/rfc8032.html",
            note: "Current non-PQ blockchain size reference only.",
        },
    ]
}

fn selected_opening_primitive() -> OpeningPrimitiveReport {
    OpeningPrimitiveReport {
        name: "full-response-packed-sis-linear-sigma-opening",
        witness_domain: "semantic hidden_preimage = fiber_state[2] || seed[32], embedded in one 41-byte / 23-residue response vector",
        proof_commitment_bytes: LINEAR_SIGMA_PROOF_COMMITMENT_BYTES,
        masked_witness_response_bytes: LINEAR_SIGMA_MASKED_WITNESS_BYTES,
        proof_bytes: LINEAR_SIGMA_TARGET_PROOF_BYTES,
        total_signature_bytes: LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES,
        verifier_equation: "L_statement(z) = a + challenge * statement.digest",
        binding_requirement: "full-response packed-SIS extraction-or-break plus byte-level relation-map soundness over the decoded 41-byte witness response",
        witness_hiding_requirement: "masked response distribution hides the 41-byte representative while preserving the 34-byte semantic hidden opening witness",
        status: "full-response quotient-relation gate implemented; semantic witness is the 12-residue quotient and the 11-residue fiber is hidden randomness",
    }
}

fn sis_opening_parameters() -> SisOpeningParameterReport {
    let matrix = sis_matrix();
    let matrix_rank = matrix_rank_mod_q(&matrix);
    let determinant = matrix_determinant_mod_q(&matrix);
    SisOpeningParameterReport {
        modulus: Q,
        matrix_domain: SIS_MATRIX_DOMAIN,
        matrix_domain_bytes: SIS_MATRIX_DOMAIN.len(),
        seed_domain_bytes: PAD_SEED_BYTES,
        digest_bytes: DIGEST_BYTES,
        seed_residues: SIS_SEED_RESIDUES,
        syndrome_residues: SIS_SYNDROME_RESIDUES,
        residue_bits: SIS_RESIDUE_BITS,
        packed_seed_bits: SIS_SEED_PACKED_BITS,
        packed_seed_bytes: SIS_SEED_PACKED_BYTES,
        seed_slack_bits: SIS_SEED_SLACK_BITS,
        packed_syndrome_bits: SIS_SYNDROME_PACKED_BITS,
        packed_syndrome_bytes: SIS_SYNDROME_PACKED_BYTES,
        syndrome_slack_bits: SIS_SYNDROME_SLACK_BITS,
        relation_shape: "packed q-vector seed response; packed q-syndrome digest/commitment; extraction-or-SIS-break reduction",
        validity_requirement: "account digest and proof commitment must be canonical packed 18-residue q-syndromes; 32-byte fields are not arbitrary byte strings",
        security_role: "second-preimage/keygen-break target, not information-theoretic binding",
        matrix_rank,
        matrix_full_rank: matrix_rank == SIS_SEED_RESIDUES,
        matrix_determinant_mod_q: determinant,
        sample_matrix_prefix: matrix[0][..4].to_vec(),
        structural_warning: "the deterministic 18x18 sample matrix is full-rank, so the bare public linear syndrome map is invertible and cannot be the witness-hiding deployed opening by itself",
        fiber_split_options: sis_fiber_split_options(),
        linear_blinding_obstruction: "a Pedersen-style q-linear split cannot fit 128-bit seed capacity, 128-bit public capacity, and enough blinding dimension inside the same 18-residue response",
        full_response_fiber_candidate: sis_full_response_fiber_candidate(),
        interpretation: "the byte packing works, but the bare 18x18 linear instantiation is only an obstruction benchmark because full-rank public linear algebra recovers the seed",
    }
}

fn sis_fiber_split_options() -> Vec<SisFiberSplitReport> {
    [8_usize, 9, 10]
        .into_iter()
        .map(|public_syndrome_residues| {
            let hidden_fiber_residues = SIS_SEED_RESIDUES - public_syndrome_residues;
            let public_capacity_bits = public_syndrome_residues * SIS_RESIDUE_BITS;
            let hidden_fiber_capacity_bits =
                hidden_fiber_residues * SIS_RESIDUE_BITS + SIS_SEED_SLACK_BITS;
            let public_capacity_meets_128 = public_capacity_bits >= 128;
            let hidden_capacity_meets_128 = hidden_fiber_capacity_bits >= 128;
            SisFiberSplitReport {
                public_syndrome_residues,
                public_capacity_bits,
                hidden_fiber_residues,
                hidden_fiber_capacity_bits,
                prefix_matrix_rank: matrix_prefix_rank_mod_q(public_syndrome_residues),
                public_capacity_meets_128,
                hidden_capacity_meets_128,
                balanced_128_possible: public_capacity_meets_128 && hidden_capacity_meets_128,
                note: "with an 18-residue seed response, reducing public rows creates fibers but cannot give both 128 public bits and 128 hidden fiber bits inside the same 32-byte q-vector packing",
            }
        })
        .collect()
}

fn sis_full_response_fiber_candidate() -> SisFullResponseFiberReport {
    let public_capacity_bits = SIS_FULL_RESPONSE_PUBLIC_ROWS * SIS_RESIDUE_BITS;
    let hidden_fiber_residues = SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS;
    let hidden_fiber_capacity_bits =
        hidden_fiber_residues * SIS_RESIDUE_BITS + SIS_FULL_RESPONSE_SLACK_BITS;
    let prefix_matrix_rank = full_response_prefix_rank_mod_q(SIS_FULL_RESPONSE_PUBLIC_ROWS);
    let kernel = full_response_kernel_vector();
    let kernel_syndrome = full_response_syndrome_of_vector(&kernel);
    SisFullResponseFiberReport {
        matrix_domain: SIS_FULL_RESPONSE_MATRIX_DOMAIN,
        matrix_domain_bytes: SIS_FULL_RESPONSE_MATRIX_DOMAIN.len(),
        response_bytes: LINEAR_SIGMA_MASKED_WITNESS_BYTES,
        response_residues: SIS_FULL_RESPONSE_RESIDUES,
        response_packed_bits: SIS_FULL_RESPONSE_PACKED_BITS,
        response_slack_bits: SIS_FULL_RESPONSE_SLACK_BITS,
        public_syndrome_residues: SIS_FULL_RESPONSE_PUBLIC_ROWS,
        public_capacity_bits,
        packed_public_syndrome_bits: SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS,
        packed_public_syndrome_bytes: SIS_FULL_RESPONSE_SYNDROME_PACKED_BYTES,
        public_syndrome_slack_bits: SIS_FULL_RESPONSE_SYNDROME_SLACK_BITS,
        hidden_fiber_residues,
        hidden_fiber_capacity_bits,
        prefix_matrix_rank,
        kernel_dimension: SIS_FULL_RESPONSE_RESIDUES - prefix_matrix_rank,
        sample_kernel_vector_prefix: kernel[..6].to_vec(),
        sample_kernel_vector_nonzero: kernel.iter().any(|&entry| entry != 0),
        sample_kernel_syndrome_zero: kernel_syndrome.iter().all(|&entry| entry == 0),
        public_capacity_meets_128: public_capacity_bits >= 128,
        hidden_capacity_meets_128: hidden_fiber_capacity_bits >= 128,
        same_target_response_width: SIS_FULL_RESPONSE_PACKED_BYTES
            == LINEAR_SIGMA_MASKED_WITNESS_BYTES,
        lean_reduction_gate: "PackedSISFullResponseOpeningReduction -> FullResponseByteOpeningAlgebra",
        quotient_relation_gate: "PackedSISFullResponseQuotientOpeningReduction -> PackedSISFullResponseOpeningReduction",
        existential_quotient_gate: "FullResponseExistentialQuotientRelation -> FullResponseQuotientRelation",
        local_delta_quotient_predicate: "ByteLocalDeltaRepresentativeSemantics.quotientOpensStatement = exists full response in the quotient fiber satisfying the concrete local-delta byte equations",
        concrete_decoder_gate: "concreteByteLocalDeltaRepresentativeSemantics + ByteLocalDeltaRustLayout byte-offset lemmas",
        honest_representative_gate: "HonestLocalDeltaQuotientStatement -> targetRepresentativeExists_of_honestStatements",
        semantic_quotient_residues: SIS_FULL_RESPONSE_PUBLIC_ROWS,
        hidden_randomizer_residues: hidden_fiber_residues,
        full_preimage_binding_compatible: false,
        fiber_saturation_requirement: "relationMapSound must be fiber-saturated: every full response with the same accepted public syndrome must decode to a valid opening witness; this is incompatible with binding every byte of the hidden preimage unless the relation is quotient/fiber-aware",
        relation_shift: "uses the full 41-byte masked response as a 23-residue q-vector; the public digest/commitment is a canonical packed 12-residue q-syndrome inside the existing 32-byte field",
    }
}

fn sample_report() -> LocalDeltaStatefulReport {
    let fiber_state = [0x39, 0x30];
    let mut seed = [0_u8; PAD_SEED_BYTES];
    for (idx, byte) in seed.iter_mut().enumerate() {
        *byte = (idx as u8).wrapping_mul(17).wrapping_add(3);
    }
    let hidden_preimage = encode_hidden_preimage(fiber_state, seed);
    let seed_derived_fiber_state = derive_fiber_state_from_seed(&seed);
    let seed_restricted_hidden_preimage = encode_hidden_preimage(seed_derived_fiber_state, seed);
    let commitment = commitment_digest(&hidden_preimage);
    let block = 7;
    let frame = pad_frame(block);
    let pad_bytes = expand_pad(&seed, block);
    let pad = pad_residues(&pad_bytes);
    let mask_view = [12_u16, 12_277_u16];
    let opening = opening_bytes(mask_view, pad);
    let full_response_hidden_preimage = sample_full_response_masked_witness_for_mask(mask_view);
    let full_response_syndrome_digest =
        full_response_syndrome_digest(&full_response_hidden_preimage).unwrap();
    let sample_public_key = vec![0_u8; PK_BYTES];
    let sample_message = b"sample quotient benchmark message";
    LocalDeltaStatefulReport {
        profile: "Descender.Signature.q12289.local_delta.stateful.v1",
        q: Q,
        groups: GROUPS,
        local_free: LOCAL_FREE,
        repetitions: REPS,
        commitment_domain: String::from_utf8(COMMITMENT_DOMAIN.to_vec()).unwrap(),
        commitment_domain_bytes: COMMITMENT_DOMAIN.len(),
        pad_domain: String::from_utf8(PAD_DOMAIN.to_vec()).unwrap(),
        pad_domain_bytes: PAD_DOMAIN.len(),
        pad_frame_bytes: PAD_FRAME_BYTES,
        fiber_state_bytes: FIBER_STATE_BYTES,
        pad_seed_bytes: PAD_SEED_BYTES,
        hidden_preimage_bytes: HIDDEN_PREIMAGE_BYTES,
        digest_bytes: DIGEST_BYTES,
        local_opening_bytes_per_rep: LOCAL_OPENING_BYTES_PER_REP,
        stateful_spend_signature_bytes: STATEFUL_SPEND_SIGNATURE_BYTES,
        stateful_transaction_signature_bytes: STATEFUL_TRANSACTION_SIGNATURE_BYTES,
        revealed_preimage_transaction_signature_bytes:
            REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES,
        stateless_sha256_signature_bytes: STATELESS_SHA256_SIGNATURE_BYTES,
        sample_fiber_state_hex: hex_encode(&fiber_state),
        sample_pad_seed_hex: hex_encode(&seed),
        sample_hidden_preimage_hex: hex_encode(&hidden_preimage),
        sample_commitment_digest_hex: hex_encode(&commitment),
        sample_full_response_hidden_preimage_hex: hex_encode(&full_response_hidden_preimage),
        sample_full_response_syndrome_digest_hex: hex_encode(&full_response_syndrome_digest),
        sample_block: block,
        sample_pad_frame_hex: hex_encode(&frame),
        sample_pad_bytes_hex: hex_encode(&pad_bytes),
        sample_pad_residues: pad,
        sample_mask_view: mask_view,
        sample_opening_bytes_hex: hex_encode(&opening),
        selected_opening_primitive: selected_opening_primitive(),
        sis_opening_parameters: sis_opening_parameters(),
        sample_seed_derived_fiber_state_hex: hex_encode(&seed_derived_fiber_state),
        sample_seed_restricted_hidden_preimage_hex: hex_encode(&seed_restricted_hidden_preimage),
        opening_proof_frontier: opening_proof_frontier(),
        committed_sigma_byte_frontier: committed_sigma_byte_frontier(),
        assumption_review_targets: assumption_review_targets(),
        assumption_closure_report: assumption_closure_report(),
        concrete_security_budgets: concrete_security_budgets(),
        full_response_matrix_sampler_certificate: full_response_matrix_sampler_certificate(),
        full_response_quotient_algebra_certificate: full_response_quotient_algebra_certificate(),
        cryptoanalysis_attack_surfaces: cryptoanalysis_attack_surfaces(),
        matrix_short_relation_probe: matrix_short_relation_probe(),
        lattice_sis_reduction_probe: lattice_sis_reduction_probe(),
        hidden_fiber_short_combination_probe: hidden_fiber_short_combination_probe(),
        quotient_witness_hiding_distribution_report: quotient_witness_hiding_distribution_report(),
        masked_response_sampler_certificate: masked_response_sampler_certificate(),
        conditioned_quotient_fiber_sampler_report: conditioned_quotient_fiber_sampler_report(),
        public_proof_forgery_probe: public_proof_forgery_probe(),
        public_sigma_equation_forgery_probe: public_sigma_equation_forgery_probe(),
        quotient_witness_hiding_game_bound: quotient_witness_hiding_game_bound(),
        final_soundness_advantage_formula: final_soundness_advantage_formula(),
        quotient_hiding_sampler_review: quotient_hiding_sampler_review(),
        challenge_action_invertibility_report: challenge_action_invertibility_report(),
        fork_equation_algebra_report: fork_equation_algebra_report(),
        canonical_response_lift_report: canonical_response_lift_report(),
        byte_response_arithmetic_report: byte_response_arithmetic_report(),
        fiat_shamir_fork_accounting_report: fiat_shamir_fork_accounting_report(),
        fiat_shamir_fork_generation_ledger: fiat_shamir_fork_generation_ledger(),
        accepted_fiber_constraint_report: accepted_fiber_constraint_report(),
        kernel_delta_acceptance_probe: kernel_delta_acceptance_probe(),
        hiding_route_frontier: hiding_route_frontier(),
        extraction_soundness_closure_report: extraction_soundness_closure_report(),
        fork_extraction_review_report: fork_extraction_review_report(),
        stateful_lifecycle_policy: stateful_lifecycle_policy(),
        reference_implementation_report: reference_implementation_report(),
        production_reference_api_report: production_reference_api_report(),
        third_party_review_packet: third_party_review_packet(),
        internal_security_review_audit: internal_security_review_audit(),
        pad_sampler_certificate: pad_sampler_certificate(&seed, block),
        fiat_shamir_frame_certificate: fiat_shamir_frame_certificate(
            &sample_public_key,
            sample_message,
            0,
            &full_response_syndrome_digest,
            &sample_proof_commitment(),
        ),
        blockchain_signature_size_comparison: blockchain_signature_size_comparison(),
    }
}

fn parse_bench_iterations(args: &[String]) -> usize {
    args.windows(2)
        .find_map(|window| {
            if window[0] == "--iterations" {
                window[1].parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(10_000)
}

// =====================================================================
// KAT v0 — frozen Known-Answer-Test corpus (Phase 0 cross-tool anchor).
//
// Every vector below is produced from the REAL reference pipeline in this
// binary (production_keygen -> production_sign -> production_verify), with
// no hand-typed bytes. Because keygen/sign/verify are fully deterministic
// (SHA256-seeded, no randomness), re-running the generator reproduces the
// corpus byte-for-byte. The `kat-check` subcommand re-derives the corpus and
// asserts equality against the frozen formal/kat/kat_v0.json so CI catches any
// drift (e.g. the AP2-004 widening) across all implementations.
// =====================================================================
mod kat {
    use super::*;

    pub const KAT_FORMAT_VERSION: &str = "kat_v0";
    pub const KAT_SIGNATURE_BYTES: usize = LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES;

    #[derive(Clone, Debug, Serialize, PartialEq, Eq)]
    pub struct KatCorpus {
        pub format_version: String,
        pub profile: String,
        /// Pins the exact reference pipeline that produced these vectors.
        pub producing_version_digest: String,
        pub q: u16,
        pub pk_bytes: usize,
        pub signature_bytes: usize,
        /// Byte offsets inside the canonical 264-byte transaction signature.
        pub signature_layout: SignatureLayout,
        pub positive: Vec<PositiveVector>,
        pub negative: Vec<NegativeVector>,
    }

    #[derive(Clone, Debug, Serialize, PartialEq, Eq)]
    pub struct SignatureLayout {
        pub slot_id: ByteRange,
        pub challenge: ByteRange,
        pub response: ByteRange,
        pub openings: ByteRange,
        pub linear_sigma_proof: ByteRange,
    }

    #[derive(Clone, Debug, Serialize, PartialEq, Eq)]
    pub struct ByteRange {
        pub offset: usize,
        pub length: usize,
    }

    impl ByteRange {
        fn new(offset: usize, length: usize) -> Self {
            Self { offset, length }
        }
    }

    #[derive(Clone, Debug, Serialize, PartialEq, Eq)]
    pub struct PositiveVector {
        pub name: String,
        pub description: String,
        pub slot_count: usize,
        pub slot_id: u32,
        pub message_hex: String,
        pub public_key_hex: String,
        pub account_slot_commitment_hex: String,
        pub signature_hex: String,
        pub expected_accept: bool,
        pub expected_reason: String,
        pub expected_slot_consumed: bool,
    }

    #[derive(Clone, Debug, Serialize, PartialEq, Eq)]
    pub struct NegativeVector {
        pub name: String,
        pub description: String,
        /// Which base case this negative vector is derived from (keygen seed).
        pub slot_count: usize,
        pub slot_id: u32,
        pub message_hex: String,
        pub public_key_hex: String,
        /// The (possibly mutated) account-slot-0 commitment the verifier sees.
        pub account_slot_commitment_hex: String,
        /// The (possibly mutated / replayed-context) signature bytes.
        pub signature_hex: String,
        /// Whether the verifier is run against an account where slot 0 was
        /// already consumed (replay probe).
        pub account_slot0_consumed: bool,
        pub expected_accept: bool,
        pub expected_reason: String,
    }

    fn layout() -> SignatureLayout {
        let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
        let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
        let proof_start = opening_start + LOCAL_OPENING_BYTES;
        SignatureLayout {
            slot_id: ByteRange::new(0, SLOT_ID_BYTES),
            challenge: ByteRange::new(SLOT_ID_BYTES, CHALLENGE_BYTES),
            response: ByteRange::new(response_start, ACTIVE_RESPONSE_BYTES),
            openings: ByteRange::new(opening_start, LOCAL_OPENING_BYTES),
            linear_sigma_proof: ByteRange::new(proof_start, LINEAR_SIGMA_TARGET_PROOF_BYTES),
        }
    }

    fn slot0_commitment_hex(account: &ProductionAccountState) -> String {
        hex_encode(account.get_slot(0).unwrap().commitment().as_bytes())
    }

    fn slot_commitment_hex(account: &ProductionAccountState, slot_id: u32) -> String {
        hex_encode(account.get_slot(slot_id).unwrap().commitment().as_bytes())
    }

    /// Digest that pins the producing pipeline: covers the structural constants
    /// plus a freshly recomputed positive signature. If any of the reference
    /// logic drifts, this digest changes and the frozen corpus stops matching.
    fn producing_version_digest() -> String {
        let keypair = production_keygen(2);
        let message = b"Descender.kat_v0.producing_version_probe";
        let signature = production_sign(
            &keypair.secret_key,
            keypair.account.get_slot(0).unwrap(),
            message,
        )
        .expect("kat producing-version probe signs");
        let mut hasher = Sha256::new();
        hasher.update(b"Descender.kat_v0.producing_version.v1");
        hasher.update(PROFILE.as_bytes());
        hasher.update((Q as u32).to_le_bytes());
        hasher.update((PK_BYTES as u32).to_le_bytes());
        hasher.update((KAT_SIGNATURE_BYTES as u32).to_le_bytes());
        hasher.update((CHALLENGE_BYTES as u32).to_le_bytes());
        hasher.update((ACTIVE_RESPONSE_BYTES as u32).to_le_bytes());
        hasher.update((LOCAL_OPENING_BYTES as u32).to_le_bytes());
        hasher.update((LINEAR_SIGMA_TARGET_PROOF_BYTES as u32).to_le_bytes());
        hasher.update(&keypair.public_key.as_bytes());
        hasher.update(signature.as_bytes());
        hex_encode(&hasher.finalize())
    }

    fn positive_vector(
        name: &str,
        description: &str,
        slot_count: usize,
        slot_id: u32,
        message: &[u8],
    ) -> PositiveVector {
        let keypair = production_keygen(slot_count);
        let signature = production_sign(
            &keypair.secret_key,
            keypair.account.get_slot(slot_id).unwrap(),
            message,
        )
        .expect("kat positive vector signs the reference slot");
        let (report, next_account) =
            production_verify(&keypair.public_key, message, &keypair.account, &signature);
        assert!(
            report.accepted,
            "kat positive vector {name} must verify: {report:?}"
        );
        let consumed = !next_account.get_slot(slot_id).unwrap().is_unused();
        assert!(consumed, "kat positive vector {name} must consume its slot");
        PositiveVector {
            name: name.to_string(),
            description: description.to_string(),
            slot_count,
            slot_id,
            message_hex: hex_encode(message),
            public_key_hex: hex_encode(&keypair.public_key.as_bytes()),
            account_slot_commitment_hex: slot_commitment_hex(&keypair.account, slot_id),
            signature_hex: hex_encode(signature.as_bytes()),
            expected_accept: true,
            expected_reason: report.reason,
            expected_slot_consumed: true,
        }
    }

    /// Drives a negative scenario through the real verifier and snapshots the
    /// exact reject result, so the frozen corpus records ground truth rather
    /// than a guessed reason string.
    fn run_negative(
        name: &str,
        description: &str,
        slot_count: usize,
        slot_id: u32,
        message: &[u8],
        account: &ProductionAccountState,
        signature: &ProductionSignature,
        public_key: &ProductionPublicKey,
        account_slot0_consumed: bool,
    ) -> NegativeVector {
        let (report, _) = production_verify(public_key, message, account, signature);
        assert!(
            !report.accepted,
            "kat negative vector {name} must be rejected: {report:?}"
        );
        NegativeVector {
            name: name.to_string(),
            description: description.to_string(),
            slot_count,
            slot_id,
            message_hex: hex_encode(message),
            public_key_hex: hex_encode(&public_key.as_bytes()),
            account_slot_commitment_hex: slot0_commitment_hex(account),
            signature_hex: hex_encode(signature.as_bytes()),
            account_slot0_consumed,
            expected_accept: false,
            expected_reason: report.reason,
        }
    }

    pub fn build_corpus() -> KatCorpus {
        // ----- positive round-trips -----
        let positive = vec![
            positive_vector(
                "pos_slot0_2slot",
                "keygen(2) -> sign slot 0 -> verify accepts.",
                2,
                0,
                b"Descender.kat_v0.positive.message.A",
            ),
            positive_vector(
                "pos_slot1_2slot",
                "keygen(2) -> sign slot 1 -> verify accepts (distinct slot).",
                2,
                1,
                b"Descender.kat_v0.positive.message.B",
            ),
            positive_vector(
                "pos_slot0_4slot",
                "keygen(4) -> sign slot 0 -> verify accepts (distinct account size).",
                4,
                0,
                b"Descender.kat_v0.positive.message.C",
            ),
        ];

        // Base case reused by the byte-mutation negatives.
        let base_slot_count = 2usize;
        let base_slot_id = 0u32;
        let base_message = b"Descender.kat_v0.negative.base.message".to_vec();
        let keypair = production_keygen(base_slot_count);
        let base_sig = production_sign(
            &keypair.secret_key,
            keypair.account.get_slot(base_slot_id).unwrap(),
            &base_message,
        )
        .expect("kat negative base case signs");
        let base_bytes = base_sig.as_bytes().to_vec();

        let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
        let proof_start = response_start + ACTIVE_RESPONSE_BYTES + LOCAL_OPENING_BYTES;

        let mut negative = Vec::new();

        // (1) non-canonical encoding: signature truncated by one byte.
        {
            let mut bytes = base_bytes.clone();
            bytes.pop();
            negative.push(run_negative(
                "neg_noncanonical_truncated",
                "Non-canonical encoding: signature truncated to 259 bytes (wrong length).",
                base_slot_count,
                base_slot_id,
                &base_message,
                &keypair.account,
                &ProductionSignature::new(bytes),
                &keypair.public_key,
                false,
            ));
        }

        // (2) mutated challenge byte.
        {
            let mut bytes = base_bytes.clone();
            bytes[SLOT_ID_BYTES] ^= 0x01;
            negative.push(run_negative(
                "neg_mutated_challenge",
                "Mutated challenge: flip a bit in the algebraic challenge field.",
                base_slot_count,
                base_slot_id,
                &base_message,
                &keypair.account,
                &ProductionSignature::new(bytes),
                &keypair.public_key,
                false,
            ));
        }

        // (3) mutated response byte.
        {
            let mut bytes = base_bytes.clone();
            bytes[response_start] ^= 0x01;
            negative.push(run_negative(
                "neg_mutated_response",
                "Mutated response: flip a bit in the active response region.",
                base_slot_count,
                base_slot_id,
                &base_message,
                &keypair.account,
                &ProductionSignature::new(bytes),
                &keypair.public_key,
                false,
            ));
        }

        // (4a) mutated projection carrier — proof commitment (account-binding
        //      surface): the carrier's first 32 bytes must equal the slot
        //      randomizer commitment, so flipping a bit there is caught by the
        //      account-binding layer.
        {
            let mut bytes = base_bytes.clone();
            bytes[proof_start] ^= 0x01;
            negative.push(run_negative(
                "neg_mutated_carrier_commitment",
                "Mutated carrier: flip a bit in the projection-carrier proof commitment \
                 (breaks the slot randomizer binding).",
                base_slot_count,
                base_slot_id,
                &base_message,
                &keypair.account,
                &ProductionSignature::new(bytes),
                &keypair.public_key,
                false,
            ));
        }

        // (4b) mutated projection carrier — masked-witness response (sigma
        //      surface): leaves the proof commitment intact so account binding
        //      passes, then the inner linear-sigma equation rejects it.
        {
            let mut bytes = base_bytes.clone();
            bytes[proof_start + LINEAR_SIGMA_PROOF_COMMITMENT_BYTES] ^= 0x01;
            negative.push(run_negative(
                "neg_mutated_carrier_masked_witness",
                "Mutated carrier: flip a bit in the projection-carrier masked-witness response \
                 (breaks the linear-sigma verifier equation).",
                base_slot_count,
                base_slot_id,
                &base_message,
                &keypair.account,
                &ProductionSignature::new(bytes),
                &keypair.public_key,
                false,
            ));
        }

        // (5) replayed slot: verify the genuine signature against an account
        //     whose slot 0 was already consumed.
        {
            let (accept_report, next_account) = production_verify(
                &keypair.public_key,
                &base_message,
                &keypair.account,
                &base_sig,
            );
            assert!(
                accept_report.accepted,
                "kat replay base must first accept: {accept_report:?}"
            );
            negative.push(run_negative(
                "neg_replayed_slot",
                "Replayed slot: genuine signature re-presented after the slot was consumed.",
                base_slot_count,
                base_slot_id,
                &base_message,
                &next_account,
                &base_sig,
                &keypair.public_key,
                true,
            ));
        }

        // (6) stale account root: mutate the slot randomizer commitment so the
        //     account no longer matches the bound transaction.
        {
            let mut mutated_account = keypair.account.clone();
            let mut commitment = *mutated_account.get_slot(0).unwrap().commitment().as_bytes();
            commitment[DIGEST_BYTES] ^= 1;
            mutated_account
                .replace_slot_commitment_for_test(
                    0,
                    ProductionAccountCommitment::from_bytes(commitment)
                        .expect("randomizer mutation remains canonical"),
                )
                .expect("replace slot commitment");
            negative.push(run_negative(
                "neg_stale_account_root",
                "Stale account root: slot randomizer commitment mutated (account/state mismatch).",
                base_slot_count,
                base_slot_id,
                &base_message,
                &mutated_account,
                &base_sig,
                &keypair.public_key,
                false,
            ));
        }

        // (7) public-representative forgery probe: claim a slot id outside the
        //     account. This is the cheapest public-only forgery attempt that
        //     reaches the verifier with a structurally valid signature.
        {
            let mut bytes = base_bytes.clone();
            bytes[..SLOT_ID_BYTES].copy_from_slice(&999_u32.to_le_bytes());
            negative.push(run_negative(
                "neg_public_representative_forgery_slot_oob",
                "Public-representative forgery probe: signature claims a slot id outside the account.",
                base_slot_count,
                base_slot_id,
                &base_message,
                &keypair.account,
                &ProductionSignature::new(bytes),
                &keypair.public_key,
                false,
            ));
        }

        KatCorpus {
            format_version: KAT_FORMAT_VERSION.to_string(),
            profile: PROFILE.to_string(),
            producing_version_digest: producing_version_digest(),
            q: Q,
            pk_bytes: PK_BYTES,
            signature_bytes: KAT_SIGNATURE_BYTES,
            signature_layout: layout(),
            positive,
            negative,
        }
    }

    pub fn corpus_json() -> String {
        let corpus = build_corpus();
        let mut json =
            serde_json::to_string_pretty(&corpus).expect("kat corpus serializes to json");
        json.push('\n');
        json
    }

    pub fn default_corpus_path() -> std::path::PathBuf {
        // tools/parameter-search/Cargo.toml -> repo root is ../../
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .join("../../formal/kat/kat_v0.json")
            .canonicalize()
            .unwrap_or_else(|_| manifest_dir.join("../../formal/kat/kat_v0.json"))
    }

    /// Re-derives the corpus and compares it byte-for-byte against the frozen
    /// JSON on disk. Returns Err with a human-readable diff summary on drift.
    pub fn check_against(path: &std::path::Path) -> Result<(), String> {
        let frozen = std::fs::read_to_string(path)
            .map_err(|e| format!("read frozen corpus {}: {e}", path.display()))?;
        let regenerated = corpus_json();
        if frozen == regenerated {
            return Ok(());
        }
        // Pinpoint the first divergence to make CI failures actionable.
        let frozen_lines: Vec<&str> = frozen.lines().collect();
        let regen_lines: Vec<&str> = regenerated.lines().collect();
        let mut detail = String::new();
        let max = frozen_lines.len().max(regen_lines.len());
        for i in 0..max {
            let f = frozen_lines.get(i).copied().unwrap_or("<missing>");
            let r = regen_lines.get(i).copied().unwrap_or("<missing>");
            if f != r {
                detail = format!(
                    "first divergence at line {}:\n  frozen:      {f}\n  regenerated: {r}",
                    i + 1
                );
                break;
            }
        }
        Err(format!(
            "KAT v0 drift detected: regenerated corpus does not match {}.\n{detail}\n\
             If this change is intentional, regenerate with `--kat-generate` and review the diff.",
            path.display()
        ))
    }

    pub fn run_cli(args: &[String]) -> Result<(), String> {
        let mut path: Option<std::path::PathBuf> = None;
        let mut iter = args.iter().enumerate();
        while let Some((idx, arg)) = iter.next() {
            if arg == "--out" || arg == "--path" {
                let value = args
                    .get(idx + 1)
                    .ok_or_else(|| format!("{arg} requires a path value"))?;
                path = Some(std::path::PathBuf::from(value));
            }
        }
        if args.iter().any(|a| a == "--kat-generate") {
            let target = path.unwrap_or_else(default_corpus_path);
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("create {}: {e}", parent.display()))?;
            }
            std::fs::write(&target, corpus_json())
                .map_err(|e| format!("write {}: {e}", target.display()))?;
            println!("wrote KAT v0 corpus to {}", target.display());
            Ok(())
        } else {
            // --kat-check
            let target = path.unwrap_or_else(default_corpus_path);
            check_against(&target)?;
            println!(
                "KAT v0 corpus at {} reproduces byte-for-byte.",
                target.display()
            );
            Ok(())
        }
    }
}

/// WS4 #11 empirical leakage battery: dump a corpus of REAL signatures for a
/// FIXED keypair/witness over varied messages and slots. We emit, per signature,
/// the decoded transcript components (challenge active_block/sign per rep, the 52
/// signed response residues, the 52 opening residues) PLUS the KNOWN secret
/// witness coordinates (the 23 full-response residues of the slot's
/// ideal_masked_witness_response, and the 16x2 secret-derived pad table) so the
/// python battery can run a witness-correlation/key-recovery test against ground
/// truth. Output is newline-delimited JSON (one record per signature) plus a
/// single META line with the per-slot ground-truth witness.
fn run_leak_sample(args: &[String]) -> Result<(), String> {
    let mut count: usize = 100_000;
    let mut slots: usize = 4;
    let mut out_path = String::from("research/cryptanalysis/leakage_corpus.jsonl");
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--count" => {
                count = iter
                    .next()
                    .ok_or("--count needs a value")?
                    .parse()
                    .map_err(|_| "bad --count")?
            }
            "--slots" => {
                slots = iter
                    .next()
                    .ok_or("--slots needs a value")?
                    .parse()
                    .map_err(|_| "bad --slots")?
            }
            "--out" => out_path = iter.next().ok_or("--out needs a value")?.clone(),
            _ => {}
        }
    }

    let keypair = reference_keygen(slots);
    let public_view = decode_public_key(&keypair.public_key).map_err(|e| e)?;

    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let _ = std::fs::remove_file(&out_path);
    let mut buf = String::new();

    // META: ground-truth witness per slot.
    // - full_response: the 23 secret residues of the masked witness (the witness w).
    // - pad_table: the 16x2 secret-derived pad residues (active pad outputs).
    // - public_free: the 16x2 public free coordinates (drives the deterministic responses).
    let public_free: Vec<[u16; LOCAL_FREE]> = public_view.free.to_vec();
    let mut meta = String::from("{\"meta\":true,\"q\":12289,\"reps\":26,\"groups\":16,\"slots\":[");
    for slot_id in 0..slots {
        let mw = keypair.account.slots[slot_id]
            .ideal_masked_witness_response
            .expect("keygen slot has ideal masked witness");
        let full = full_response_vector_from_masked_witness(&mw)
            .expect("masked witness decodes to full response vector");
        let seed: [u8; PAD_SEED_BYTES] = semantic_hidden_preimage_from_masked_witness(&mw)
            [FIBER_STATE_BYTES..]
            .try_into()
            .unwrap();
        let mut pad_table = Vec::with_capacity(GROUPS);
        for block in 0..GROUPS {
            pad_table.push(expand_pad_residues(&seed, block));
        }
        if slot_id > 0 {
            meta.push(',');
        }
        meta.push_str(&format!("{{\"slot\":{slot_id},\"full_response\":["));
        for (i, v) in full.iter().enumerate() {
            if i > 0 {
                meta.push(',');
            }
            meta.push_str(&v.to_string());
        }
        meta.push_str("],\"pad_table\":[");
        for (b, p) in pad_table.iter().enumerate() {
            if b > 0 {
                meta.push(',');
            }
            meta.push_str(&format!("[{},{}]", p[0], p[1]));
        }
        meta.push(']');
        meta.push('}');
    }
    meta.push_str("],\"public_free\":[");
    for (b, f) in public_free.iter().enumerate() {
        if b > 0 {
            meta.push(',');
        }
        meta.push_str(&format!("[{},{}]", f[0], f[1]));
    }
    meta.push_str("]}\n");
    buf.push_str(&meta);

    let mut written = 0usize;
    let mut idx: u64 = 0;
    while written < count {
        let slot_id = (idx as usize) % slots;
        // vary the message so the Fiat-Shamir challenge changes each signature.
        let message = format!("ws4-leak-battery/{idx}").into_bytes();
        idx += 1;
        let sig = match reference_sign_with_proof(&keypair, slot_id as u32, &message) {
            Ok(s) => s,
            Err(_) => continue,
        };
        // Decode the components straight from the serialized signature.
        let mut off = SLOT_ID_BYTES;
        let challenge_bytes = &sig[off..off + CHALLENGE_BYTES];
        off += CHALLENGE_BYTES;
        let responses = decode_responses(&sig[off..off + ACTIVE_RESPONSE_BYTES]).unwrap();
        off += ACTIVE_RESPONSE_BYTES;
        let openings = decode_openings(&sig[off..off + LOCAL_OPENING_BYTES]).unwrap();
        let challenge = decode_challenge(challenge_bytes).unwrap();

        let mut rec = String::with_capacity(512);
        rec.push_str(&format!("{{\"slot\":{slot_id},\"block\":["));
        for rep in 0..REPS {
            if rep > 0 {
                rec.push(',');
            }
            rec.push_str(&challenge.active_block[rep].to_string());
        }
        rec.push_str("],\"sign\":[");
        for rep in 0..REPS {
            if rep > 0 {
                rec.push(',');
            }
            rec.push_str(&challenge.sign[rep].to_string());
        }
        rec.push_str("],\"resp\":[");
        for rep in 0..REPS {
            for coord in 0..LOCAL_FREE {
                if rep > 0 || coord > 0 {
                    rec.push(',');
                }
                rec.push_str(&responses.active_free[rep][coord].to_string());
            }
        }
        rec.push_str("],\"open\":[");
        for rep in 0..REPS {
            for coord in 0..LOCAL_FREE {
                if rep > 0 || coord > 0 {
                    rec.push(',');
                }
                rec.push_str(&openings.active_free[rep][coord].to_string());
            }
        }
        rec.push_str("]}\n");
        buf.push_str(&rec);
        written += 1;
        if buf.len() > (8 << 20) {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&out_path)
                .and_then(|mut f| std::io::Write::write_all(&mut f, buf.as_bytes()))
                .map_err(|e| e.to_string())?;
            buf.clear();
        }
    }
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out_path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, buf.as_bytes()))
        .map_err(|e| e.to_string())?;
    eprintln!("wrote {written} signature records to {out_path}");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--leak-sample") {
        if let Err(err) = run_leak_sample(&args) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        return;
    }
    if args
        .iter()
        .any(|arg| arg == "--kat-generate" || arg == "--kat-check")
    {
        if let Err(err) = kat::run_cli(&args) {
            eprintln!("{err}");
            std::process::exit(1);
        }
    } else if args.iter().any(|arg| arg == "--bench") {
        let iterations = parse_bench_iterations(&args);
        println!(
            "{}",
            serde_json::to_string_pretty(&reference_timing_report(iterations)).unwrap()
        );
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&sample_report()).unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use descender_parameter_search::production_reference::reviewed_response_link_code;

    #[test]
    fn kat_v0_corpus_is_internally_consistent() {
        let corpus = kat::build_corpus();
        assert_eq!(corpus.format_version, "kat_v0");
        assert!(
            corpus.positive.len() >= 3,
            "kat v0 needs at least 3 positive vectors"
        );
        // Count distinct reject reasons across the negative vectors.
        let mut reasons: Vec<&str> = corpus
            .negative
            .iter()
            .map(|n| n.expected_reason.as_str())
            .collect();
        reasons.sort_unstable();
        reasons.dedup();
        assert!(
            reasons.len() >= 4,
            "kat v0 needs at least 4 distinct negative reject reasons, got {reasons:?}"
        );
        for p in &corpus.positive {
            assert!(p.expected_accept);
            assert_eq!(p.signature_hex.len() / 2, kat::KAT_SIGNATURE_BYTES);
        }
        for n in &corpus.negative {
            assert!(!n.expected_accept);
            assert!(!n.expected_reason.is_empty());
        }
    }

    #[test]
    fn kat_v0_corpus_reproduces_frozen_json() {
        // The frozen artifact MUST byte-for-byte match a fresh reference run.
        // This is the same check CI runs via `--kat-check`.
        let path = kat::default_corpus_path();
        if !path.exists() {
            panic!(
                "frozen KAT corpus missing at {}; regenerate with `--kat-generate`",
                path.display()
            );
        }
        kat::check_against(&path).expect("frozen KAT v0 corpus reproduces from the reference");
    }

    #[test]
    fn constants_match_lean_profile() {
        assert_eq!(COMMITMENT_DOMAIN.len(), 41);
        assert_eq!(PAD_DOMAIN.len(), 45);
        assert_eq!(PAD_FRAME_BYTES, 46);
        assert_eq!(HIDDEN_PREIMAGE_BYTES, 34);
        assert_eq!(ALGEBRAIC_CHALLENGE_BYTES, 23);
        assert_eq!(CHALLENGE_HARDENING_BYTES, 8);
        assert_eq!(CHALLENGE_BYTES, 31);
        assert_eq!(ALGEBRAIC_CHALLENGE_ENTROPY_BITS, 130);
        assert_eq!(CHALLENGE_ENTROPY_BITS, 194);
        assert_eq!(FS_QUANTUM_QUERY_BITS_FOR_128, 33);
        assert_eq!(FS_CLASSICAL_QUERY_BITS_FOR_128, 66);
        assert_eq!(STATEFUL_SPEND_SIGNATURE_BYTES, 187);
        assert_eq!(STATEFUL_TRANSACTION_SIGNATURE_BYTES, 191);
        assert_eq!(REVEALED_PREIMAGE_PROOF_BYTES, 41);
        assert_eq!(REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES, 232);
        assert_eq!(STATELESS_SHA256_SIGNATURE_BYTES, 1019);
        assert_eq!(PROOF_BYTES_UNDER_256, 65);
        assert_eq!(PROOF_BYTES_BEAT_IMPLICIT_269, 77);
        assert_eq!(PROOF_BYTES_TIE_IMPLICIT_269, 78);
        assert_eq!(PROOF_BYTES_BEAT_STATELESS_SHA256, 827);
        assert_eq!(LINEAR_SIGMA_PROOF_COMMITMENT_BYTES, 32);
        assert_eq!(LINEAR_SIGMA_MASKED_WITNESS_BYTES, 41);
        assert_eq!(LINEAR_SIGMA_TARGET_PROOF_BYTES, 73);
        assert_eq!(LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES, 264);
        assert_eq!(REVIEWED_ACCOUNT_LINK_BYTES, 68);
        assert_eq!(REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES, 260);
    }

    #[test]
    fn preimage_layout_is_fiber_state_then_seed() {
        let fiber_state = [0xaa, 0xbb];
        let seed = [0x11; PAD_SEED_BYTES];
        let encoded = encode_hidden_preimage(fiber_state, seed);
        assert_eq!(&encoded[..2], &fiber_state);
        assert_eq!(&encoded[2..], &seed);
    }

    #[test]
    fn pad_frame_is_domain_plus_block() {
        let frame = pad_frame(15);
        assert_eq!(&frame[..PAD_DOMAIN.len()], PAD_DOMAIN);
        assert_eq!(frame[PAD_DOMAIN.len()], 15);
    }

    #[test]
    fn sample_vector_is_stable() {
        let report = sample_report();
        assert_eq!(
            report.sample_commitment_digest_hex,
            "85317f200f8dc994d87170db40b86df9a719e8bdc3b98391d08883d9f6004287"
        );
        assert_eq!(report.sample_pad_bytes_hex, "e70ba227");
        assert_eq!(report.sample_pad_residues, [3047, 10146]);
        assert_eq!(report.sample_opening_bytes_hex, "f30b9627");
        assert_eq!(
            report.sample_full_response_hidden_preimage_hex.len(),
            2 * LINEAR_SIGMA_MASKED_WITNESS_BYTES
        );
        assert_eq!(
            report.sample_full_response_syndrome_digest_hex.len(),
            2 * DIGEST_BYTES
        );
        assert_eq!(
            report.selected_opening_primitive.name,
            "full-response-packed-sis-linear-sigma-opening"
        );
        assert_eq!(report.selected_opening_primitive.proof_bytes, 73);
        assert_eq!(report.selected_opening_primitive.total_signature_bytes, 264);
        assert_eq!(
            report.selected_opening_primitive.verifier_equation,
            "L_statement(z) = a + challenge * statement.digest"
        );
        assert!(
            report
                .selected_opening_primitive
                .binding_requirement
                .contains("full-response packed-SIS")
        );
        assert_eq!(report.sis_opening_parameters.seed_residues, 18);
        assert_eq!(report.sis_opening_parameters.syndrome_residues, 18);
        assert_eq!(report.sis_opening_parameters.residue_bits, 14);
        assert_eq!(report.sis_opening_parameters.packed_seed_bytes, 32);
        assert_eq!(report.sis_opening_parameters.packed_syndrome_bytes, 32);
        assert_eq!(report.sis_opening_parameters.seed_slack_bits, 4);
        assert_eq!(report.sis_opening_parameters.syndrome_slack_bits, 4);
        assert!(
            report
                .sis_opening_parameters
                .validity_requirement
                .contains("canonical packed 18-residue q-syndromes")
        );
        assert_eq!(report.sis_opening_parameters.matrix_rank, 18);
        assert!(report.sis_opening_parameters.matrix_full_rank);
        assert_eq!(
            report.sis_opening_parameters.matrix_determinant_mod_q,
            11243
        );
        assert_eq!(
            report.sis_opening_parameters.sample_matrix_prefix,
            vec![4233, 3247, 6774, 7484]
        );
        assert!(
            report
                .sis_opening_parameters
                .sample_matrix_prefix
                .iter()
                .all(|&entry| entry < Q)
        );
        assert!(
            report
                .sis_opening_parameters
                .structural_warning
                .contains("invertible")
        );
        assert_eq!(report.sis_opening_parameters.fiber_split_options.len(), 3);
        let balanced = &report.sis_opening_parameters.fiber_split_options[1];
        assert_eq!(balanced.public_syndrome_residues, 9);
        assert_eq!(balanced.public_capacity_bits, 126);
        assert_eq!(balanced.hidden_fiber_residues, 9);
        assert_eq!(balanced.hidden_fiber_capacity_bits, 130);
        assert_eq!(balanced.prefix_matrix_rank, 9);
        assert!(
            !report
                .sis_opening_parameters
                .fiber_split_options
                .iter()
                .any(|row| row.balanced_128_possible)
        );
        assert!(
            report
                .sis_opening_parameters
                .linear_blinding_obstruction
                .contains("Pedersen-style")
        );
        let full_response = &report.sis_opening_parameters.full_response_fiber_candidate;
        assert_eq!(full_response.matrix_domain, SIS_FULL_RESPONSE_MATRIX_DOMAIN);
        assert_eq!(full_response.response_bytes, 41);
        assert_eq!(full_response.response_residues, 23);
        assert_eq!(full_response.response_packed_bits, 322);
        assert_eq!(full_response.response_slack_bits, 6);
        assert_eq!(full_response.public_syndrome_residues, 12);
        assert_eq!(full_response.public_capacity_bits, 168);
        assert_eq!(full_response.packed_public_syndrome_bits, 168);
        assert_eq!(full_response.packed_public_syndrome_bytes, 21);
        assert_eq!(full_response.public_syndrome_slack_bits, 88);
        assert_eq!(full_response.hidden_fiber_residues, 11);
        assert_eq!(full_response.hidden_fiber_capacity_bits, 160);
        assert_eq!(full_response.prefix_matrix_rank, 12);
        assert_eq!(full_response.kernel_dimension, 11);
        assert!(full_response.sample_kernel_vector_nonzero);
        assert!(full_response.sample_kernel_syndrome_zero);
        assert!(full_response.public_capacity_meets_128);
        assert!(full_response.hidden_capacity_meets_128);
        assert!(full_response.same_target_response_width);
        assert_eq!(full_response.semantic_quotient_residues, 12);
        assert_eq!(full_response.hidden_randomizer_residues, 11);
        assert!(!full_response.full_preimage_binding_compatible);
        assert!(
            full_response
                .lean_reduction_gate
                .contains("PackedSISFullResponseOpeningReduction")
        );
        assert!(
            full_response
                .quotient_relation_gate
                .contains("PackedSISFullResponseQuotientOpeningReduction")
        );
        assert!(
            full_response
                .existential_quotient_gate
                .contains("FullResponseExistentialQuotientRelation")
        );
        assert!(
            full_response
                .local_delta_quotient_predicate
                .contains("ByteLocalDeltaRepresentativeSemantics")
        );
        assert!(
            full_response
                .concrete_decoder_gate
                .contains("ByteLocalDeltaRustLayout")
        );
        assert!(
            full_response
                .honest_representative_gate
                .contains("targetRepresentativeExists")
        );
        assert!(
            full_response
                .fiber_saturation_requirement
                .contains("fiber-saturated")
        );
        assert_eq!(report.sample_seed_derived_fiber_state_hex.len(), 4);
        assert_eq!(
            report.sample_seed_restricted_hidden_preimage_hex.len(),
            2 * HIDDEN_PREIMAGE_BYTES
        );
        assert_eq!(report.opening_proof_frontier.candidates.len(), 6);
        assert_eq!(
            report.opening_proof_frontier.candidates[1].total_signature_bytes,
            232
        );
        assert_eq!(
            report.opening_proof_frontier.candidates[2].total_signature_bytes,
            264
        );
        assert_eq!(report.assumption_review_targets.len(), 5);
        assert_eq!(report.assumption_closure_report.len(), 5);
        assert_eq!(report.concrete_security_budgets.len(), 5);
        assert_eq!(
            report.concrete_security_budgets[0].game,
            "PackedFullResponseExtractionGame"
        );
        assert_eq!(report.concrete_security_budgets[0].raw_security_bits, 168);
        assert_eq!(
            report.concrete_security_budgets[0].quantum_security_bits_after_queries,
            128
        );
        assert_eq!(
            report.concrete_security_budgets[1].game,
            "QuotientWitnessHidingGame"
        );
        assert_eq!(report.concrete_security_budgets[1].raw_security_bits, 133);
        assert_eq!(
            report.concrete_security_budgets[1].adversary_query_bound_log2,
            2
        );
        assert_eq!(
            report.concrete_security_budgets[1].quantum_security_bits_after_queries,
            129
        );
        assert_eq!(report.concrete_security_budgets[2].game, "SHA256PadPRFGame");
        assert_eq!(
            report.concrete_security_budgets[2].quantum_security_bits_after_queries,
            128
        );
        assert_eq!(
            report.concrete_security_budgets[3].game,
            "FiatShamirFrameAgreementGame-hardened-31-byte-challenge"
        );
        assert_eq!(report.concrete_security_budgets[3].raw_security_bits, 194);
        assert_eq!(
            report.concrete_security_budgets[3].adversary_query_bound_log2,
            33
        );
        assert_eq!(
            report.concrete_security_budgets[3].quantum_security_bits_after_queries,
            128
        );
        assert_eq!(report.cryptoanalysis_attack_surfaces.len(), 5);
        assert_eq!(
            report.cryptoanalysis_attack_surfaces[0].gate,
            "packed full-response extraction"
        );
        assert_eq!(report.matrix_short_relation_probe.candidates_tested, 117484);
        assert_eq!(report.matrix_short_relation_probe.zero_relations_found, 0);
        assert_eq!(
            report
                .matrix_short_relation_probe
                .shortest_nonzero_syndrome_lee_weight,
            13_872
        );
        assert_eq!(report.lattice_sis_reduction_probe.basis_vectors, 23);
        assert_eq!(report.lattice_sis_reduction_probe.lattice_dimension, 23);
        assert!(
            report
                .lattice_sis_reduction_probe
                .shortest_vector_syndrome_zero
        );
        assert_eq!(
            report
                .lattice_sis_reduction_probe
                .shortest_vector_lee_weight,
            654
        );
        assert_eq!(
            report
                .lattice_sis_reduction_probe
                .shortest_vector_coeff_bound,
            68
        );
        assert_eq!(
            report
                .hidden_fiber_short_combination_probe
                .combinations_tested,
            177_146
        );
        assert_eq!(
            report
                .hidden_fiber_short_combination_probe
                .shortest_vector_lee_weight,
            12_078
        );
        assert!(!report.public_proof_forgery_probe.uses_private_key_material);
        assert!(
            !report
                .public_proof_forgery_probe
                .account_hidden_witness_available
        );
        assert!(report.public_proof_forgery_probe.verifier_accepts);
        assert!(
            !report
                .public_proof_forgery_probe
                .sigma_equation_verifier_accepts
        );
        assert_eq!(
            report.public_proof_forgery_probe.sigma_equation_reason,
            "linear sigma verifier equation mismatch"
        );
        assert_eq!(
            report.public_proof_forgery_probe.signature_bytes,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert!(
            !report
                .public_sigma_equation_forgery_probe
                .uses_private_key_material
        );
        assert!(
            !report
                .public_sigma_equation_forgery_probe
                .account_hidden_witness_available
        );
        assert!(report.public_sigma_equation_forgery_probe.verifier_accepts);
        assert_eq!(
            report.public_sigma_equation_forgery_probe.accepted_reason,
            "ok-stateful-local-delta-public-quotient-sigma-equation"
        );
        assert_eq!(
            report.public_sigma_equation_forgery_probe.signature_bytes,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert_eq!(
            report
                .quotient_witness_hiding_distribution_report
                .algebraic_hidden_fiber_bits,
            160
        );
        assert_eq!(
            report
                .quotient_witness_hiding_distribution_report
                .estimated_representatives_after_fiber_state_bits,
            144
        );
        assert!(
            !report
                .quotient_witness_hiding_distribution_report
                .current_fixture_uses_hidden_fiber_entropy
        );
        assert_eq!(
            report.masked_response_sampler_certificate.proof_surface,
            "FullResponseMaskedResponseSamplerCertificate"
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .raw_algebraic_fiber_bits,
            160
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .modeled_fiber_state_constraint_bits,
            16
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .residual_hiding_bits,
            144
        );
        assert!(
            report
                .masked_response_sampler_certificate
                .residual_hiding_meets_140
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .kernel_projection_coordinates,
            vec![0, 1]
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .kernel_projection_rank,
            2
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .residue1_conditioning_modulus,
            4
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .residue1_allowed_values,
            3_073
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .exact_conditioned_support_formula,
            "12289^9 * 3073"
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .exact_conditioned_support_decimal,
            "19643164710622955502452258988325173574657"
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .exact_conditioned_min_entropy_floor_bits,
            133
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .capacity_model_overstates_exact_floor_by_bits,
            11
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .exact_quantum_query_bound_log2_for_128,
            2
        );
        assert!(
            !report
                .masked_response_sampler_certificate
                .active_pad_outputs_conditioned
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .statistical_distance_metric,
            "total variation distance"
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .statistical_distance_numerator,
            0
        );
        assert_eq!(
            report
                .masked_response_sampler_certificate
                .statistical_distance_denominator,
            1
        );
        assert!(
            report
                .masked_response_sampler_certificate
                .statistical_distance_is_zero
        );
        assert!(
            report
                .masked_response_sampler_certificate
                .excluded_conditioning
                .contains(&"active pad outputs")
        );
        assert_eq!(
            report.quotient_witness_hiding_game_bound.game,
            "QuotientWitnessHidingGame"
        );
        assert_eq!(
            report
                .quotient_witness_hiding_game_bound
                .sampler_statistical_distance_numerator,
            0
        );
        assert_eq!(
            report
                .quotient_witness_hiding_game_bound
                .game_advantage_numerator,
            0
        );
        assert!(report.quotient_witness_hiding_game_bound.advantage_is_zero);
        assert_eq!(
            report
                .quotient_witness_hiding_game_bound
                .support_min_entropy_floor_bits,
            133
        );
        assert_eq!(
            report
                .quotient_witness_hiding_game_bound
                .samples_per_stateful_slot,
            1
        );
        assert_eq!(
            report.final_soundness_advantage_formula.lean_surface,
            "FullResponseQROMExtractionBranchTheorem + FullResponseDeploymentReviewCertificate.statefulVerifier_conditionalSoundness"
        );
        assert_eq!(
            report
                .final_soundness_advantage_formula
                .qrom_hash_query_bound_log2,
            16
        );
        assert_eq!(
            report
                .final_soundness_advantage_formula
                .qrom_quadratic_loss_bits,
            32
        );
        assert_eq!(
            report
                .final_soundness_advantage_formula
                .union_safe_min_hash_action_bits,
            131
        );
        assert_eq!(
            report.quotient_hiding_sampler_review.lean_surface,
            "FullResponseMaskedResponseSamplerCertificate + QuotientWitnessHidingStatisticalBound"
        );
        assert!(
            report
                .quotient_hiding_sampler_review
                .per_slot_tv_distance_zero
        );
        assert!(
            report
                .quotient_hiding_sampler_review
                .active_pad_outputs_excluded
        );
        assert_eq!(
            report.quotient_hiding_sampler_review.support_floor_bits,
            133
        );
        assert_eq!(
            report
                .conditioned_quotient_fiber_sampler_report
                .sampler_name,
            "direct-conditioned-quotient-fiber-sampler"
        );
        assert_eq!(
            report
                .conditioned_quotient_fiber_sampler_report
                .total_linear_rows,
            14
        );
        assert_eq!(
            report.conditioned_quotient_fiber_sampler_report.linear_rank,
            14
        );
        assert_eq!(
            report
                .conditioned_quotient_fiber_sampler_report
                .sampled_free_residues,
            9
        );
        assert_eq!(
            report
                .conditioned_quotient_fiber_sampler_report
                .residue1_allowed_values,
            3073
        );
        assert_eq!(
            report
                .conditioned_quotient_fiber_sampler_report
                .sample_residue1_value,
            9496
        );
        assert!(
            !report
                .conditioned_quotient_fiber_sampler_report
                .retry_loop_required
        );
        assert_eq!(
            report
                .conditioned_quotient_fiber_sampler_report
                .expected_retries,
            1
        );
        assert!(
            report
                .conditioned_quotient_fiber_sampler_report
                .sample_preserves_public_syndrome
        );
        assert!(
            report
                .conditioned_quotient_fiber_sampler_report
                .sample_preserves_fiber_state_bytes
        );
        assert!(
            report
                .conditioned_quotient_fiber_sampler_report
                .sample_is_canonical_packed_q
        );
        assert_eq!(
            report
                .accepted_fiber_constraint_report
                .distinct_active_blocks,
            15
        );
        assert_eq!(
            report
                .accepted_fiber_constraint_report
                .max_bits_after_fiber_state_constraint,
            144
        );
        assert!(
            report
                .accepted_fiber_constraint_report
                .meets_128_after_fiber_state_constraint
        );
        assert_eq!(
            report
                .accepted_fiber_constraint_report
                .estimated_remaining_fiber_bits_after_acceptance,
            0
        );
        assert!(
            report
                .kernel_delta_acceptance_probe
                .accepting_multipliers
                .is_empty()
        );
        assert!(
            report
                .kernel_delta_acceptance_probe
                .fiber_state_preserving_multipliers
                .is_empty()
        );
        assert_eq!(report.hiding_route_frontier.len(), 4);
        assert_eq!(
            report.hiding_route_frontier[0].route,
            "decoupled quotient witness"
        );
        assert_eq!(
            report.hiding_route_frontier[0].transaction_signature_bytes,
            260
        );
        assert_eq!(report.hiding_route_frontier[0].residual_hiding_bits, 128);
        assert!(report.hiding_route_frontier[0].meets_128_hiding);
        assert_eq!(
            report.hiding_route_frontier[1].route,
            "larger fiber, keep local fiber-state equality"
        );
        assert_eq!(
            report.hiding_route_frontier[1].transaction_signature_bytes,
            262
        );
        assert_eq!(report.hiding_route_frontier[1].residual_hiding_bits, 128);
        assert_eq!(
            report.hiding_route_frontier[2].transaction_signature_bytes,
            316
        );
        assert_eq!(
            report.hiding_route_frontier[3].transaction_signature_bytes,
            318
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .current_response_hidden_link_budget_under_256,
            0
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .fiber_saturated_hidden_link_budget_under_256,
            0
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .fiber_state_preserving_hidden_link_budget_under_256,
            0
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .lean_surface,
            "CommittedSigma.AccountCarriedHiddenLinkInterface"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .soundness_certificate_surface,
            "CommittedSigma.AccountCarriedSoundnessCertificate"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .account_link_proof_system_surface,
            "CommittedSigma.AccountLinkProofSystem"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .homomorphic_account_link_surface,
            "CommittedSigma.HomomorphicAccountLinkInterface"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .pre_response_backing_surface,
            "CommittedSigma.AccountBackedExceptResponse"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .response_link_soundness_surface,
            "CommittedSigma.ResponseLinkSoundness"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .bounded_response_link_kernel_surface,
            "CommittedSigma.BoundedResponseLinkKernelCertificate"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .bounded_response_link_soundness_theorem,
            "CommittedSigma.BoundedResponseLinkKernelCertificate.responseLinkSoundness"
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .bounded_response_link_checker_script,
            "scripts/research/response_link_bounded_kernel_check.py"
        );
        let code_frontier = &report
            .committed_sigma_byte_frontier
            .account_hidden_link_contract
            .response_link_code_frontier;
        assert_eq!(code_frontier.len(), 5);
        assert_eq!(code_frontier[0].name, "full-kernel-coordinate-code");
        assert_eq!(
            code_frontier[0].public_kernel_dimension_before_link,
            SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS
        );
        assert_eq!(
            code_frontier[0].projection_rank_on_public_kernel,
            SIS_FULL_RESPONSE_RESIDUES - SIS_FULL_RESPONSE_PUBLIC_ROWS
        );
        assert_eq!(code_frontier[0].residual_full_kernel_dimension, 0);
        assert_eq!(
            code_frontier[0].combined_constraint_rank,
            SIS_FULL_RESPONSE_RESIDUES
        );
        assert_eq!(code_frontier[0].combined_kernel_dimension, 0);
        assert!(code_frontier[0].full_kernel_sound);
        assert!(!code_frontier[0].public_account_witness_hiding);
        assert_eq!(code_frontier[0].commitment_bytes, 20);
        assert!(code_frontier[0].lll_box_probe_excludes_response_delta);
        assert_eq!(code_frontier[1].name, "bounded-box-kernel-code-target");
        assert_eq!(code_frontier[1].coordinate_projection, vec![0, 3]);
        assert_eq!(code_frontier[1].commitment_bytes, 4);
        assert_eq!(code_frontier[1].projection_rank_on_public_kernel, 2);
        assert_eq!(code_frontier[1].combined_kernel_dimension, 9);
        assert!(
            code_frontier[1].shortest_intersection_vector_coeff_bound > 2 * RESPONSE_BOUND as i64
        );
        assert_eq!(code_frontier[1].response_difference_coeff_bound, 96);
        assert_eq!(
            code_frontier[1].lll_margin_over_response_difference_bound,
            code_frontier[1].shortest_intersection_vector_coeff_bound
                - code_frontier[1].response_difference_coeff_bound
        );
        assert_eq!(
            code_frontier[1].lll_margin_over_response_difference_bound,
            43
        );
        assert!(code_frontier[1].lll_box_probe_excludes_response_delta);
        assert!(code_frontier[1].public_account_witness_hiding);
        assert!(!code_frontier[1].full_kernel_sound);
        for candidate in code_frontier.iter().skip(2) {
            assert_eq!(candidate.commitment_bytes, 4);
            assert_eq!(candidate.projection_rank_on_public_kernel, 2);
            assert_eq!(candidate.combined_kernel_dimension, 9);
            assert!(candidate.public_account_witness_hiding);
        }
        assert!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .homomorphic_link_equation
                .contains("Commit(response)")
        );
        assert!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .accepting_implication
                .contains("AccountBackedByHiddenWitness")
        );
        assert_eq!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .unbacked_acceptance_theorem,
            "AccountCarriedHiddenLinkInterface.unbacked_acceptance_forces_break"
        );
        assert!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .rejected_instantiations
                .iter()
                .any(|item| item.contains("bare account hash"))
        );
        let link_frontier = &report
            .committed_sigma_byte_frontier
            .account_hidden_link_contract
            .link_proof_frontier;
        assert_eq!(link_frontier.len(), 8);
        assert_eq!(link_frontier[0].name, "reveal-witness-or-randomizer-link");
        assert!(!link_frontier[0].structurally_admissible);
        assert_eq!(link_frontier[1].name, "local-opening-only-link");
        assert!(!link_frontier[1].account_slot_aware);
        assert_eq!(link_frontier[2].name, "bare-account-hash-link");
        assert!(!link_frontier[2].instantiates_account_link_proof_system);
        assert_eq!(link_frontier[3].name, "slot-bound-first-message-only");
        assert_eq!(
            link_frontier[3].account_link_bytes,
            LINEAR_SIGMA_PROOF_COMMITMENT_BYTES
        );
        assert!(!link_frontier[3].structurally_admissible);
        assert_eq!(link_frontier[4].name, "public-projection-homomorphic-link");
        assert_eq!(link_frontier[4].signature_link_proof_bytes, 0);
        assert!(!link_frontier[4].not_public_projection_of_sigma_equation);
        assert!(!link_frontier[4].structurally_admissible);
        assert_eq!(link_frontier[4].account_link_bytes, 96);
        assert_eq!(link_frontier[5].name, "homomorphic-account-link-equation");
        assert_eq!(link_frontier[5].signature_link_proof_bytes, 0);
        assert!(link_frontier[5].not_public_projection_of_sigma_equation);
        assert!(link_frontier[5].structurally_admissible);
        assert_eq!(link_frontier[5].account_link_bytes, 96);
        assert_eq!(link_frontier[6].name, "account-carried-succinct-link-proof");
        assert_eq!(link_frontier[6].signature_link_proof_bytes, 0);
        assert!(link_frontier[6].structurally_admissible);
        assert_eq!(
            link_frontier[7].name,
            "six-byte-signature-link-proof-stress-target"
        );
        assert_eq!(link_frontier[7].signature_link_proof_bytes, 6);
        assert!(
            report
                .committed_sigma_byte_frontier
                .account_hidden_link_contract
                .open_cryptographic_obligation
                .contains("public post-challenge representative")
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[0].transaction_bytes,
            264
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[1].transaction_bytes,
            260
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[1].name,
            "bare-account-hash-link"
        );
        assert!(
            !report.committed_sigma_byte_frontier.candidates[1]
                .structurally_admissible_without_extra_proof
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[2].transaction_bytes,
            260
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[2].account_hidden_link_bytes,
            64
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[3].transaction_bytes,
            266
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[3].hidden_link_opening_bytes,
            6
        );
        assert_eq!(
            report.committed_sigma_byte_frontier.candidates[4].transaction_bytes,
            262
        );
        assert_eq!(
            report.assumption_closure_report[1].lean_game,
            "PackedFullResponseExtractionGame"
        );
        assert_eq!(
            report.assumption_closure_report[1].target_classical_security_bits,
            128
        );
        assert_eq!(
            report.assumption_closure_report[1].target_quantum_security_bits,
            128
        );
        assert!(
            report.assumption_closure_report[1]
                .break_event
                .contains("extractionFailure")
        );
        assert_eq!(
            report.extraction_soundness_closure_report.game,
            "PackedFullResponseExtractionGame"
        );
        assert!(
            report
                .extraction_soundness_closure_report
                .closed_obligations
                .iter()
                .any(|item| item.contains("41-byte response parser"))
        );
        assert!(
            report
                .extraction_soundness_closure_report
                .remaining_assumption
                .contains("Fiat-Shamir/QROM fork generation")
        );
        assert_eq!(
            report
                .extraction_soundness_closure_report
                .fork_extraction_surface,
            "FullResponseForkExtractionCertificate"
        );
        assert!(
            report
                .extraction_soundness_closure_report
                .fork_closed_obligations
                .iter()
                .any(|item| item.contains("only the challenge field changes"))
        );
        assert_eq!(
            report.fork_extraction_review_report.lean_surface,
            "FullResponseForkExtractionCertificate"
        );
        assert!(
            report
                .fork_extraction_review_report
                .same_statement_fields
                .contains(&"proof commitment")
        );
        assert!(
            report
                .fork_extraction_review_report
                .concrete_review_target
                .contains("same-commitment fork generation")
        );
        assert_eq!(
            report.challenge_action_invertibility_report.action_residues,
            12
        );
        assert_eq!(
            report
                .challenge_action_invertibility_report
                .bad_difference_probability_formula,
            "1 / 12289^12 in the ideal hash-to-Fq12 model"
        );
        assert_eq!(
            report
                .challenge_action_invertibility_report
                .bad_difference_floor_bits,
            163
        );
        assert_eq!(
            report
                .challenge_action_invertibility_report
                .generic_quantum_margin_for_128_bits,
            17
        );
        assert!(
            report
                .challenge_action_invertibility_report
                .sample_actions_distinct
        );
        assert_eq!(
            report.fork_equation_algebra_report.lean_surface,
            "FullResponseForkEquationAlgebraCertificate"
        );
        assert!(
            report
                .fork_equation_algebra_report
                .action_difference_nonzero
        );
        assert!(report.fork_equation_algebra_report.inverse_exists);
        assert!(report.fork_equation_algebra_report.inverse_check_is_one);
        assert!(
            report
                .fork_equation_algebra_report
                .fork_difference_matches_scaled_digest
        );
        assert!(report.fork_equation_algebra_report.extracted_digest_matches);
        assert_eq!(
            report.canonical_response_lift_report.lean_surface,
            "FullResponseCanonicalResponseLiftCertificate"
        );
        assert_eq!(report.canonical_response_lift_report.matrix_rank, 12);
        assert!(report.canonical_response_lift_report.right_inverse_check);
        assert!(report.canonical_response_lift_report.decoded_is_canonical);
        assert!(
            report
                .canonical_response_lift_report
                .all_residues_lt_modulus
        );
        assert!(report.canonical_response_lift_report.slack_bits_zero);
        assert!(
            report
                .canonical_response_lift_report
                .lifted_syndrome_matches_input
        );
        assert!(
            report
                .canonical_response_lift_report
                .packed_digest_matches_input
        );
        assert_eq!(
            report.byte_response_arithmetic_report.lean_surface,
            "FullResponseByteResponseArithmeticCertificate"
        );
        assert!(report.byte_response_arithmetic_report.z0_is_canonical);
        assert!(report.byte_response_arithmetic_report.z1_is_canonical);
        assert!(
            report
                .byte_response_arithmetic_report
                .difference_response_is_canonical
        );
        assert!(
            report
                .byte_response_arithmetic_report
                .extracted_response_is_canonical
        );
        assert!(report.byte_response_arithmetic_report.z0_equation_holds);
        assert!(report.byte_response_arithmetic_report.z1_equation_holds);
        assert!(
            report
                .byte_response_arithmetic_report
                .response_difference_matches_field_difference
        );
        assert!(
            report
                .byte_response_arithmetic_report
                .inverse_scaled_response_hits_digest
        );
        assert!(
            report
                .byte_response_arithmetic_report
                .extracted_response_packed_digest_matches
        );
        assert_eq!(
            report.fiat_shamir_fork_accounting_report.lean_surface,
            "FullResponseFiatShamirForkAccountingCertificate"
        );
        assert_eq!(
            report
                .fiat_shamir_fork_accounting_report
                .challenge_entropy_bits,
            194
        );
        assert_eq!(
            report
                .fiat_shamir_fork_accounting_report
                .action_collision_floor_bits,
            163
        );
        assert_eq!(
            report
                .fiat_shamir_fork_accounting_report
                .fs_quantum_query_bound_log2_for_128,
            33
        );
        assert_eq!(
            report
                .fiat_shamir_fork_accounting_report
                .action_collision_quantum_query_bound_log2_for_128,
            17
        );
        assert_eq!(
            report
                .fiat_shamir_fork_accounting_report
                .bottleneck_quantum_query_bound_log2_for_128,
            17
        );
        assert!(
            report
                .fiat_shamir_fork_accounting_report
                .algebraic_fork_path_closed
        );
        assert!(
            report
                .fiat_shamir_fork_accounting_report
                .action_collision_budget_meets_128_at_bottleneck
        );
        assert!(
            report
                .fiat_shamir_fork_accounting_report
                .fs_budget_meets_128_at_bottleneck
        );
        assert_eq!(
            report.fiat_shamir_fork_generation_ledger.lean_surface,
            "FullResponseFiatShamirForkGenerationLedger"
        );
        assert!(
            report
                .fiat_shamir_fork_generation_ledger
                .selected_qrom_theorem
                .contains("measure-and-reprogram")
        );
        assert!(
            report
                .fiat_shamir_fork_generation_ledger
                .qrom_theorem_fits_protocol_shape
        );
        assert!(
            report
                .fiat_shamir_fork_generation_ledger
                .qrom_same_commitment_condition
                .contains("satisfied")
        );
        assert_eq!(
            report.internal_security_review_audit.verdict,
            "byte-path blockers closed: the proof-commitment challenge frame and full 254-byte public verifier are executable; security remains assumption-parametric pending cryptanalysis and production hardening"
        );
        assert!(
            report
                .internal_security_review_audit
                .findings
                .iter()
                .any(|finding| finding.severity == "closed"
                    && finding.area == "public verifier integration")
        );
        assert_eq!(
            report
                .fiat_shamir_fork_generation_ledger
                .qrom_hash_query_bound_log2,
            16
        );
        assert_eq!(
            report
                .fiat_shamir_fork_generation_ledger
                .qrom_quadratic_loss_bits,
            32
        );
        assert_eq!(
            report
                .fiat_shamir_fork_generation_ledger
                .single_term_quantum_query_bound_log2_for_128,
            17
        );
        assert_eq!(
            report
                .fiat_shamir_fork_generation_ledger
                .union_safe_quantum_query_bound_log2_for_128,
            16
        );
        assert_eq!(
            report
                .fiat_shamir_fork_generation_ledger
                .union_safe_classical_query_bound_log2_for_128,
            34
        );
        assert!(
            report
                .fiat_shamir_fork_generation_ledger
                .union_bound_meets_128
        );
        assert_eq!(
            report.assumption_closure_report[2].lean_game,
            "QuotientWitnessHidingGame"
        );
        assert_eq!(
            report.assumption_closure_report[3].lean_game,
            "SHA256PadPRFGame"
        );
        assert_eq!(
            report.assumption_closure_report[4].lean_game,
            "FiatShamirFrameAgreementGame"
        );
        assert_eq!(
            report.assumption_review_targets[0].lean_surface,
            "FullResponseMatrixSamplerAgreement"
        );
        assert!(report.assumption_review_targets.iter().any(|target| {
            target
                .lean_surface
                .contains("ConcreteFullResponseQuotientReductionAssumptions")
        }));
        assert!(report.assumption_review_targets.iter().any(|target| {
            target
                .lean_surface
                .contains("SHA256RejectionPadPRFAssumption")
        }));
        assert_eq!(report.full_response_matrix_sampler_certificate.rows, 12);
        assert_eq!(report.full_response_matrix_sampler_certificate.cols, 23);
        assert_eq!(
            report
                .full_response_matrix_sampler_certificate
                .matrix_transcript_sha256,
            "b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6"
        );
        assert!(
            report
                .full_response_matrix_sampler_certificate
                .all_entries_lt_modulus
        );
        let quotient_certificate = &report.full_response_quotient_algebra_certificate;
        assert_eq!(quotient_certificate.rank, 12);
        assert_eq!(quotient_certificate.kernel_dimension, 11);
        assert_eq!(
            quotient_certificate.pivot_columns,
            (0..12).collect::<Vec<_>>()
        );
        assert_eq!(
            quotient_certificate.free_columns,
            (12..23).collect::<Vec<_>>()
        );
        assert!(quotient_certificate.kernel_basis_vectors_nonzero);
        assert!(quotient_certificate.kernel_basis_syndrome_zero);
        assert!(quotient_certificate.right_inverse_check);
        assert_eq!(
            quotient_certificate.kernel_basis_transcript_sha256,
            "2196317450e2c833f7fa0e7883d7d63cc7c6c7946c9b27d39571492739c45341"
        );
        assert_eq!(
            quotient_certificate.right_inverse_transcript_sha256,
            "492ca916baddd8e841e192b4825d5e5b1f1775dca68b38450763e158f6563b21"
        );
        assert_eq!(
            report.pad_sampler_certificate.counter_encoding,
            "u32 little-endian"
        );
        assert_eq!(report.pad_sampler_certificate.rejection_cutoff, 61445);
        assert_eq!(
            report.pad_sampler_certificate.sample_counter0_digest_hex,
            "e96ba6e7db63c4b9b409bf9f7359cfcf1bcb9dd462afcefddb232e14456c1dd8"
        );
        assert!(report.pad_sampler_certificate.sample_residues_lt_modulus);
        assert!(report.fiat_shamir_frame_certificate.binds_proof_commitment);
        assert_eq!(report.fiat_shamir_frame_certificate.sample_frame_bytes, 641);
        assert_eq!(
            report.fiat_shamir_frame_certificate.sample_frame_sha256,
            "96c59f06d3e185e055df12c152791047fe0becf560c32f3849286d4544d4b15a"
        );
        assert_eq!(
            report.fiat_shamir_frame_certificate.sample_challenge_hex,
            "96c59f06d3e185e055df12c152791047fe0becf560c32f3849286d4544d4b1"
        );
        assert!(
            report
                .fiat_shamir_frame_certificate
                .excludes_response_and_opening_bytes
        );
        assert_eq!(
            report.stateful_lifecycle_policy.policy_name,
            "monotonic one-signature-per-slot account state"
        );
        assert!(
            report
                .stateful_lifecycle_policy
                .tests_cover
                .contains(&"consumed-slot replay rejection")
        );
        assert_eq!(
            report.reference_implementation_report.public_key_bytes,
            PK_BYTES
        );
        assert_eq!(report.reference_implementation_report.keygen_slots, 2);
        assert_eq!(
            report.reference_implementation_report.signature_bytes,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert!(report.reference_implementation_report.verifies);
        assert!(report.reference_implementation_report.consumes_slot);
        assert!(report.reference_implementation_report.rejects_replay);
        assert_eq!(
            report
                .reference_implementation_report
                .known_answer_public_key_sha256,
            sha256_hex(&reference_public_key())
        );
        assert_eq!(report.blockchain_signature_size_comparison.len(), 5);
        let descender = &report.blockchain_signature_size_comparison[0];
        assert_eq!(descender.scheme, "Descender quotient candidate");
        assert_eq!(descender.public_key_bytes, PK_BYTES);
        assert_eq!(
            descender.signature_bytes,
            STATEFUL_SPEND_SIGNATURE_BYTES + LINEAR_SIGMA_TARGET_PROOF_BYTES
        );
        assert_eq!(
            descender.transaction_signature_bytes,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert!(descender.post_quantum);
        assert!(descender.stateful);
        assert!(descender.source.contains("LocalDeltaByteCommitment"));
        assert_eq!(
            report.blockchain_signature_size_comparison[1].signature_bytes,
            2420
        );
        assert!(
            report.blockchain_signature_size_comparison[1]
                .source
                .contains("nist.fips.204.pdf")
        );
        assert_eq!(
            report.blockchain_signature_size_comparison[2].signature_bytes,
            666
        );
        assert!(
            report.blockchain_signature_size_comparison[2]
                .source
                .contains("falcon.pdf")
        );
        assert_eq!(
            report.blockchain_signature_size_comparison[3].signature_bytes,
            7856
        );
        assert!(
            report.blockchain_signature_size_comparison[3]
                .source
                .contains("FIPS 205")
        );
    }

    #[test]
    fn public_postchallenge_representative_forgery_remains_negative_control() {
        let report = sample_report();
        assert!(!report.public_proof_forgery_probe.uses_private_key_material);
        assert!(
            !report
                .public_proof_forgery_probe
                .account_hidden_witness_available
        );
        assert!(report.public_proof_forgery_probe.verifier_accepts);
        assert!(
            !report
                .public_proof_forgery_probe
                .sigma_equation_verifier_accepts
        );
        assert!(
            report
                .public_proof_forgery_probe
                .sigma_equation_reason
                .contains("equation mismatch")
        );
        assert!(
            !report
                .public_sigma_equation_forgery_probe
                .uses_private_key_material
        );
        assert!(
            !report
                .public_sigma_equation_forgery_probe
                .account_hidden_witness_available
        );
        assert!(report.public_sigma_equation_forgery_probe.verifier_accepts);
        assert!(
            report
                .public_sigma_equation_forgery_probe
                .accepted_reason
                .contains("public-quotient-sigma-equation")
        );

        let preflight = prodref::production_deployment_preflight();
        assert!(!preflight.reviewed_fq12_deployment_verifier_present);
        assert!(
            preflight
                .blockers
                .contains(&prodref::PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS)
        );
    }

    #[test]
    fn fork_equation_algebra_extracts_digest() {
        let report = fork_equation_algebra_report();
        assert!(report.action_difference_nonzero);
        assert!(report.inverse_exists);
        assert!(report.inverse_check_is_one);
        assert!(report.fork_difference_matches_scaled_digest);
        assert!(report.extracted_digest_matches);
    }

    #[test]
    fn canonical_response_lift_roundtrips_digest() {
        let report = canonical_response_lift_report();
        assert_eq!(report.matrix_rank, SIS_FULL_RESPONSE_PUBLIC_ROWS);
        assert!(report.right_inverse_check);
        assert!(report.decoded_is_canonical);
        assert!(report.all_residues_lt_modulus);
        assert!(report.slack_bits_zero);
        assert!(report.lifted_syndrome_matches_input);
        assert!(report.packed_digest_matches_input);
    }

    #[test]
    fn byte_response_arithmetic_realizes_fork_equation() {
        let report = byte_response_arithmetic_report();
        assert!(report.z0_is_canonical);
        assert!(report.z1_is_canonical);
        assert!(report.difference_response_is_canonical);
        assert!(report.extracted_response_is_canonical);
        assert!(report.z0_equation_holds);
        assert!(report.z1_equation_holds);
        assert!(report.response_difference_matches_field_difference);
        assert!(report.inverse_scaled_response_hits_digest);
        assert!(report.extracted_response_packed_digest_matches);
    }

    #[test]
    fn fiat_shamir_fork_accounting_exposes_action_collision_bottleneck() {
        let report = fiat_shamir_fork_accounting_report();
        assert_eq!(report.challenge_entropy_bits, CHALLENGE_ENTROPY_BITS);
        assert_eq!(report.action_collision_floor_bits, 163);
        assert_eq!(report.fs_quantum_query_bound_log2_for_128, 33);
        assert_eq!(report.action_collision_quantum_query_bound_log2_for_128, 17);
        assert_eq!(report.bottleneck_quantum_query_bound_log2_for_128, 17);
        assert!(report.algebraic_fork_path_closed);
        assert!(report.action_collision_budget_meets_128_at_bottleneck);
        assert!(report.fs_budget_meets_128_at_bottleneck);
    }

    #[test]
    fn linear_sigma_target_proof_layout_roundtrips() {
        let mut proof_commitment = [0_u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES];
        for (idx, byte) in proof_commitment.iter_mut().enumerate() {
            *byte = idx as u8;
        }
        let mut masked_witness_response = [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES];
        for (idx, byte) in masked_witness_response.iter_mut().enumerate() {
            *byte = (idx as u8).wrapping_mul(3).wrapping_add(1);
        }
        let proof = LinearSigmaTargetProof {
            proof_commitment,
            masked_witness_response,
        };
        let encoded = encode_linear_sigma_target_proof(&proof);
        assert_eq!(encoded.len(), LINEAR_SIGMA_TARGET_PROOF_BYTES);
        let decoded = decode_linear_sigma_target_proof(&encoded).unwrap();
        assert_eq!(decoded, proof);
    }

    fn zero_public_key() -> Vec<u8> {
        vec![0_u8; PK_BYTES]
    }

    fn sample_seed() -> [u8; PAD_SEED_BYTES] {
        let mut seed = [0_u8; PAD_SEED_BYTES];
        for (idx, byte) in seed.iter_mut().enumerate() {
            *byte = (idx as u8).wrapping_mul(17).wrapping_add(3);
        }
        seed
    }

    fn packed_tail_zero(bytes: &[u8], start_bit: usize) -> bool {
        (start_bit..bytes.len() * 8).all(|bit| ((bytes[bit / 8] >> (bit % 8)) & 1) == 0)
    }

    fn valid_quotient_stateful_fixture() -> (Vec<u8>, Vec<u8>, AccountState, Vec<u8>) {
        let message = b"stateful local delta quotient test".to_vec();
        let keypair = reference_keygen(1);
        let signature = reference_sign(&keypair, 0, &message).unwrap();
        (keypair.public_key, message, keypair.account, signature)
    }

    #[test]
    fn full_response_syndrome_digest_builder_is_canonical() {
        let mask_view = [12_u16, Q - 12];
        let hidden_preimage = sample_full_response_masked_witness_for_mask(mask_view);
        let vector = full_response_vector_from_masked_witness(&hidden_preimage).unwrap();
        assert!(vector.iter().all(|&value| value < Q));
        assert!(packed_tail_zero(
            &hidden_preimage,
            SIS_FULL_RESPONSE_PACKED_BITS
        ));

        let syndrome = full_response_syndrome_array(&vector);
        let digest = full_response_syndrome_digest(&hidden_preimage).unwrap();
        for (idx, &value) in syndrome.iter().enumerate() {
            assert_eq!(
                read_bits_le(&digest, idx * SIS_RESIDUE_BITS, SIS_RESIDUE_BITS),
                value as u32
            );
        }
        assert!(packed_tail_zero(
            &digest,
            SIS_FULL_RESPONSE_SYNDROME_PACKED_BITS
        ));
    }

    #[test]
    fn quotient_stateful_fixture_emits_honest_local_delta_trace() {
        let (public_key, _message, account, signature) = valid_quotient_stateful_fixture();
        let mask_view = [12_u16, Q - 12];
        let hidden_preimage = account.slots[0].ideal_masked_witness_response.unwrap();
        let seed: [u8; PAD_SEED_BYTES] = hidden_preimage
            [FIBER_STATE_BYTES..FIBER_STATE_BYTES + PAD_SEED_BYTES]
            .try_into()
            .unwrap();
        let fiber_state = u16_le(&hidden_preimage[..FIBER_STATE_BYTES]) % Q;
        assert_eq!(fiber_state, local_state(mask_view));

        let challenge =
            decode_challenge(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]).unwrap();
        let public_view = decode_public_key(&public_key).unwrap();
        let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
        let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
        let responses = decode_responses(&signature[response_start..opening_start]).unwrap();
        let openings = decode_openings(&signature[opening_start..]).unwrap();

        for rep in 0..REPS {
            let block = challenge.active_block[rep];
            let sign = challenge.sign[rep];
            let mut reconstructed_mask = [0_u16; LOCAL_FREE];
            for (coord, value) in reconstructed_mask.iter_mut().enumerate() {
                *value = sub_mod(
                    mod_q(responses.active_free[rep][coord] as i64),
                    challenge_mul(sign, public_view.free[block][coord]),
                );
            }
            assert_eq!(reconstructed_mask, mask_view);
            assert_eq!(local_state(reconstructed_mask), fiber_state);
            let pad = expand_pad_residues(&seed, block);
            assert_eq!(
                openings.active_free[rep],
                [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])]
            );
        }
    }

    #[test]
    fn quotient_stateful_verifier_accepts_and_consumes_slot() {
        let (public_key, message, account, signature) = valid_quotient_stateful_fixture();
        let (report, next_account) =
            verify_quotient_stateful_signature(&public_key, &message, &account, &signature);
        assert!(report.accepted, "{report:?}");
        assert_eq!(
            report.proof_model,
            "full-response-packed-sis-quotient-opening-check"
        );
        assert_eq!(
            report.reason,
            "ok-stateful-local-delta-full-response-quotient-relation"
        );
        assert!(!next_account.slots[0].unused);

        let (reuse_report, _) =
            verify_quotient_stateful_signature(&public_key, &message, &next_account, &signature);
        assert!(!reuse_report.accepted);
        assert_eq!(reuse_report.reason, "slot is already consumed");
    }

    #[test]
    fn conditioned_sampler_preserves_digest_and_fiber_state() {
        let mask_view = [12_u16, Q - 12];
        let base = sample_full_response_masked_witness_for_mask_stream(mask_view, 9);
        let sample =
            sample_conditioned_quotient_fiber_representative(&base, b"unit-test-sampler-seed", 3);
        assert_eq!(sample.linear_rank, 14);
        assert_eq!(sample.sampled_free_residues, 9);
        assert_eq!(sample.residue1_value % 4, 0);
        assert_eq!(
            full_response_syndrome_digest(&sample.masked_witness_response),
            full_response_syndrome_digest(&base)
        );
        assert_eq!(
            sample.masked_witness_response[..FIBER_STATE_BYTES],
            base[..FIBER_STATE_BYTES]
        );
        assert!(
            full_response_vector_from_masked_witness(&sample.masked_witness_response).is_some()
        );
    }

    // RED-TEAM: signature malleability via non-canonical mask_view.
    // The verifier checks only local_state(mask_view)==fiber_state and
    // openings==mask_view+pad. It does NOT pin mask_view to the signer's
    // canonical choice. With the all-zero reference PK (free coords = 0),
    // the response equals the mask_view directly, so a second mask_view that
    // hits the same fiber_state and stays in [-48,48] yields a DISTINCT
    // signature that still verifies for the SAME (message, slot).
    #[test]
    fn redteam_response_opening_malleability_distinct_accepting_signature() {
        let keypair = reference_keygen(1);
        let message = b"redteam malleability message".to_vec();
        let signature = reference_sign(&keypair, 0, &message).unwrap();

        // Sanity: original signature accepts.
        let (orig_report, _) = verify_quotient_stateful_signature(
            &keypair.public_key,
            &message,
            &keypair.account,
            &signature,
        );
        assert!(
            orig_report.accepted,
            "original must accept: {orig_report:?}"
        );

        // Confirm the canonical signer mask_view and the alternate one both
        // realize the same fiber_state. After the HIGH-1 fix, `canonical` is the
        // unique accepting in-box rep and `alt` is the rejected kernel sibling.
        let canonical = [12_u16, Q - 12];
        let fiber_state = local_state(canonical);
        let alt = [mod_q(-38), mod_q(-11)]; // (-38, -11): -38 + 50*(-11) = -588 = same syndrome
        assert_eq!(
            local_state(alt),
            fiber_state,
            "alt mask_view must match fiber state"
        );
        assert_ne!(alt, canonical);
        assert!(
            mask_view_is_canonical(canonical),
            "honest signer rep must be canonical"
        );
        assert!(
            !mask_view_is_canonical(alt),
            "forgery kernel sibling must be non-canonical"
        );

        // Decode original signature, then rebuild responses + openings from the
        // ALT mask_view. PK is all-zero so free coords vanish; response == mask_view.
        let hidden_preimage = keypair.account.slots[0]
            .ideal_masked_witness_response
            .unwrap();
        let seed: [u8; PAD_SEED_BYTES] = hidden_preimage
            [FIBER_STATE_BYTES..FIBER_STATE_BYTES + PAD_SEED_BYTES]
            .try_into()
            .unwrap();
        let challenge =
            decode_challenge(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]).unwrap();
        let public_view = decode_public_key(&keypair.public_key).unwrap();

        let mut responses = CompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        };
        let mut openings = LocalOpenings {
            active_free: [[0_u16; LOCAL_FREE]; REPS],
        };
        for rep in 0..REPS {
            let block = challenge.active_block[rep];
            let sign = challenge.sign[rep];
            for coord in 0..LOCAL_FREE {
                let response = add_mod(
                    alt[coord],
                    challenge_mul(sign, public_view.free[block][coord]),
                );
                responses.active_free[rep][coord] =
                    centered_response_value(response).expect("alt response must stay in [-48,48]");
            }
            let pad = expand_pad_residues(&seed, block);
            openings.active_free[rep] = [add_mod(alt[0], pad[0]), add_mod(alt[1], pad[1])];
        }

        // Reassemble the forged signature: same slot_id + challenge, new resp/open.
        let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
        let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
        let mut forged = signature.clone();
        forged[response_start..opening_start].copy_from_slice(&encode_responses(&responses));
        forged[opening_start..].copy_from_slice(&encode_openings(&openings));

        // The two signatures MUST be byte-distinct (uniqueness violated).
        assert_ne!(
            forged, signature,
            "forged signature must differ from the honest one"
        );

        // After the HIGH-1 canonical-domain fix, the forged (non-canonical) opening
        // MUST be rejected: the second distinct accepting signature is unconstructible.
        let (forged_report, forged_next) = verify_quotient_stateful_signature(
            &keypair.public_key,
            &message,
            &keypair.account,
            &forged,
        );
        assert!(
            !forged_report.accepted,
            "HIGH-1 FIX REGRESSION: forged non-canonical signature was accepted: {forged_report:?}"
        );
        assert!(
            forged_report
                .reason
                .contains("non-canonical local mask view"),
            "forgery must be rejected for non-canonicality, got: {}",
            forged_report.reason
        );
        // Slot must remain unspent since verification failed.
        assert!(forged_next.slots[0].unused);
    }

    // RED-TEAM: the SAME malleability survives on the deployable proof-carrying
    // path (reference_verify_with_proof -> verify_public_quotient_proof_signature).
    // The challenge frame binds proof_commitment but NOT the response/opening
    // bytes, and verify_local_delta_relation_with_masked_witness accepts any
    // mask_view with local_state==fiber_state. So a distinct proof-carrying
    // signature for the same message is accepted.
    #[test]
    fn redteam_response_opening_malleability_deployable_proof_path() {
        let keypair = reference_keygen(1);
        let message = b"redteam malleability deployable";
        let signature = reference_sign_with_proof(&keypair, 0, message).unwrap();
        let (orig_report, _) =
            reference_verify_with_proof(&keypair.public_key, message, &keypair.account, &signature);
        assert!(orig_report.accepted, "{orig_report:?}");

        let canonical = [12_u16, Q - 12];
        let fiber_state = local_state(canonical);
        let alt = [mod_q(-38), mod_q(-11)];
        assert_eq!(local_state(alt), fiber_state);
        assert!(mask_view_is_canonical(canonical));
        assert!(!mask_view_is_canonical(alt));

        // proof.masked_witness_response is the last LINEAR_SIGMA_MASKED_WITNESS_BYTES;
        // its seed drives the pad. Pull the seed from it (same as verifier does).
        let proof_start =
            STATEFUL_TRANSACTION_SIGNATURE_BYTES + LINEAR_SIGMA_PROOF_COMMITMENT_BYTES;
        let masked_witness: [u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES] = signature
            [proof_start..proof_start + LINEAR_SIGMA_MASKED_WITNESS_BYTES]
            .try_into()
            .unwrap();
        let semantic = semantic_hidden_preimage_from_masked_witness(&masked_witness);
        let seed: [u8; PAD_SEED_BYTES] = semantic[FIBER_STATE_BYTES..].try_into().unwrap();

        let challenge =
            decode_challenge(&signature[SLOT_ID_BYTES..SLOT_ID_BYTES + CHALLENGE_BYTES]).unwrap();
        let public_view = decode_public_key(&keypair.public_key).unwrap();
        let mut responses = CompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        };
        let mut openings = LocalOpenings {
            active_free: [[0_u16; LOCAL_FREE]; REPS],
        };
        for rep in 0..REPS {
            let block = challenge.active_block[rep];
            let sign = challenge.sign[rep];
            for coord in 0..LOCAL_FREE {
                let response = add_mod(
                    alt[coord],
                    challenge_mul(sign, public_view.free[block][coord]),
                );
                responses.active_free[rep][coord] = centered_response_value(response).unwrap();
            }
            let pad = expand_pad_residues(&seed, block);
            openings.active_free[rep] = [add_mod(alt[0], pad[0]), add_mod(alt[1], pad[1])];
        }
        let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
        let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
        let mut forged = signature.clone();
        forged[response_start..opening_start].copy_from_slice(&encode_responses(&responses));
        forged[opening_start..opening_start + LOCAL_OPENING_BYTES]
            .copy_from_slice(&encode_openings(&openings));
        assert_ne!(forged, signature);

        let (forged_report, _) =
            reference_verify_with_proof(&keypair.public_key, message, &keypair.account, &forged);
        assert!(
            !forged_report.accepted,
            "HIGH-1 FIX REGRESSION (deployable proof path): forged non-canonical signature accepted: {forged_report:?}"
        );
        assert!(
            forged_report
                .reason
                .contains("non-canonical local mask view"),
            "forgery must be rejected for non-canonicality, got: {}",
            forged_report.reason
        );
    }

    #[test]
    fn reference_api_signs_verifies_and_rejects_mutations() {
        let keypair = reference_keygen(2);
        let message = b"reference api mutation test";
        let signature = reference_sign_with_proof(&keypair, 0, message).unwrap();
        assert_eq!(
            signature.len(),
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        let (report, next_account) =
            reference_verify_with_proof(&keypair.public_key, message, &keypair.account, &signature);
        assert!(report.accepted, "{report:?}");
        assert_eq!(
            report.proof_model,
            "full-response-packed-sis-quotient-public-proof-check"
        );
        assert!(!next_account.slots[0].unused);
        let (sigma_equation_report, _) = verify_public_quotient_sigma_equation_signature(
            &keypair.public_key,
            message,
            &keypair.account,
            &signature,
        );
        assert!(!sigma_equation_report.accepted);
        assert_eq!(
            sigma_equation_report.reason,
            "linear sigma verifier equation mismatch"
        );

        let (replay_report, _) =
            reference_verify_with_proof(&keypair.public_key, message, &next_account, &signature);
        assert!(!replay_report.accepted);
        assert_eq!(replay_report.reason, "slot is already consumed");

        let mut mutated_message = message.to_vec();
        mutated_message[0] ^= 1;
        let (message_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            &mutated_message,
            &keypair.account,
            &signature,
        );
        assert!(!message_report.accepted);
        assert_eq!(message_report.reason, "challenge mismatch");

        let mut mutated_signature = signature.clone();
        let opening_start = SLOT_ID_BYTES + CHALLENGE_BYTES + ACTIVE_RESPONSE_BYTES;
        mutated_signature[opening_start] ^= 1;
        let (opening_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_signature,
        );
        assert!(!opening_report.accepted);
        assert!(
            opening_report
                .reason
                .contains("local opening relation mismatch")
        );

        let proof_start = STATEFUL_TRANSACTION_SIGNATURE_BYTES;
        let mut mutated_proof_commitment = signature.clone();
        mutated_proof_commitment[proof_start] ^= 1;
        let (proof_commitment_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_proof_commitment,
        );
        assert!(!proof_commitment_report.accepted);
        assert_eq!(proof_commitment_report.reason, "challenge mismatch");

        let mut mutated_proof_response = signature.clone();
        mutated_proof_response[proof_start + LINEAR_SIGMA_PROOF_COMMITMENT_BYTES] ^= 1;
        let (proof_response_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_proof_response,
        );
        assert!(!proof_response_report.accepted);

        let mut wrong_slot_signature = signature.clone();
        wrong_slot_signature[..SLOT_ID_BYTES].copy_from_slice(&1_u32.to_le_bytes());
        let (slot_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &wrong_slot_signature,
        );
        assert!(!slot_report.accepted);
        assert_eq!(slot_report.reason, "challenge mismatch");
    }

    #[test]
    fn production_reference_facade_exposes_account_commitment_boundary() {
        let keypair = production_keygen(2);
        let message = b"production facade mutation test";
        assert_eq!(
            keypair
                .account
                .get_slot(0)
                .unwrap()
                .commitment()
                .as_bytes()
                .len(),
            REVIEWED_ACCOUNT_LINK_BYTES
        );
        assert_eq!(
            keypair
                .account
                .get_slot(0)
                .unwrap()
                .commitment()
                .response_link_code(),
            reviewed_response_link_code()
        );
        let signature = production_sign(
            &keypair.secret_key,
            keypair.account.get_slot(0).unwrap(),
            message,
        )
        .unwrap();
        assert_eq!(
            signature.len(),
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert_ne!(
            signature.len(),
            REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES,
            "facade must not claim the final 250-byte target while it wraps the legacy proof path"
        );
        let (report, next_account) =
            production_verify(&keypair.public_key, message, &keypair.account, &signature);
        assert!(report.accepted, "{report:?}");
        assert!(!next_account.get_slot(0).unwrap().is_unused());

        let (replay_report, _) =
            production_verify(&keypair.public_key, message, &next_account, &signature);
        assert!(!replay_report.accepted);
        assert_eq!(replay_report.reason, "slot is already consumed");

        let mut mutated_account = keypair.account.clone();
        let mut mutated_commitment = *mutated_account.get_slot(0).unwrap().commitment().as_bytes();
        mutated_commitment[DIGEST_BYTES] ^= 1;
        mutated_account
            .replace_slot_commitment_for_test(
                0,
                ProductionAccountCommitment::from_bytes(mutated_commitment)
                    .expect("randomizer mutation remains canonical"),
            )
            .unwrap();
        let (randomizer_report, _) =
            production_verify(&keypair.public_key, message, &mutated_account, &signature);
        assert!(!randomizer_report.accepted);
        assert_eq!(
            randomizer_report.reason,
            "account randomizer commitment mismatch"
        );

        let mut bad_code_account = keypair.account.clone();
        let mut bad_code_commitment = *bad_code_account
            .get_slot(0)
            .unwrap()
            .commitment()
            .as_bytes();
        bad_code_commitment[2 * DIGEST_BYTES] ^= 1;
        bad_code_account
            .replace_slot_commitment_for_test(
                0,
                ProductionAccountCommitment::from_bytes_unchecked_for_test(bad_code_commitment),
            )
            .unwrap();
        let (code_report, _) =
            production_verify(&keypair.public_key, message, &bad_code_account, &signature);
        assert!(!code_report.accepted);
        assert_eq!(code_report.reason, "account response-link code mismatch");
    }

    #[test]
    fn timing_report_includes_production_facade_rows() {
        let report = reference_timing_report(1);
        assert_eq!(
            report.signature_bytes,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert_ne!(
            report.signature_bytes,
            REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        let operations = report
            .timings
            .iter()
            .map(|row| row.operation)
            .collect::<Vec<_>>();
        for expected in [
            "reference_keygen",
            "reference_sign_with_proof",
            "reference_verify_with_proof",
            "production_facade_keygen",
            "production_facade_sign",
            "production_facade_verify",
        ] {
            assert!(operations.contains(&expected));
        }
        assert!(report.timings.iter().all(|row| row.accepted_or_ok));
    }

    #[test]
    fn known_answer_review_packet_covers_serialized_surfaces() {
        let report = sample_report();
        let packet = &report.third_party_review_packet;
        assert_eq!(
            packet.signature_bytes,
            LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES
        );
        assert_eq!(packet.public_key_bytes, PK_BYTES);
        assert!(
            packet
                .exact_sha256_assumptions
                .iter()
                .any(|item| item.contains("QROM random oracle"))
        );
        assert!(
            packet
                .matrix_sampler_review
                .iter()
                .any(|item| item.contains("12 rows x 23 columns"))
        );
        let names = packet
            .known_answer_vectors
            .iter()
            .map(|kat| kat.name)
            .collect::<Vec<_>>();
        for expected in [
            "public_key_sha256",
            "slot0_digest_hex",
            "challenge_frame_sha256",
            "challenge_hex",
            "stateful_signature_sha256",
            "linear_sigma_proof_sha256",
        ] {
            assert!(names.contains(&expected), "missing KAT {expected}");
        }
        assert!(
            packet
                .fuzz_property_tests
                .iter()
                .any(|item| item.contains("forbidden high challenge bits"))
        );
        assert!(packet.constant_time_status.contains("not complete"));
    }

    #[test]
    fn serialized_region_mutations_reject_or_change_proof_layout() {
        let keypair = reference_keygen(2);
        let message = b"serialized region mutation matrix";
        let signature = reference_sign_with_proof(&keypair, 0, message).unwrap();
        let response_start = SLOT_ID_BYTES + CHALLENGE_BYTES;
        let opening_start = response_start + ACTIVE_RESPONSE_BYTES;
        let proof_start = STATEFUL_TRANSACTION_SIGNATURE_BYTES;

        let mut mutated_slot = signature.clone();
        mutated_slot[..SLOT_ID_BYTES].copy_from_slice(&1_u32.to_le_bytes());
        let (slot_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_slot,
        );
        assert!(!slot_report.accepted);

        let mut mutated_algebraic_challenge = signature.clone();
        mutated_algebraic_challenge[SLOT_ID_BYTES] ^= 0x01;
        let (challenge_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_algebraic_challenge,
        );
        assert!(!challenge_report.accepted);
        assert_eq!(challenge_report.reason, "challenge mismatch");

        let mut mutated_hardening_challenge = signature.clone();
        mutated_hardening_challenge[SLOT_ID_BYTES + ALGEBRAIC_CHALLENGE_BYTES] ^= 0x01;
        let (hardening_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_hardening_challenge,
        );
        assert!(!hardening_report.accepted);
        assert_eq!(hardening_report.reason, "challenge mismatch");

        let mut mutated_public_key = keypair.public_key.clone();
        mutated_public_key[0] ^= 0x01;
        let (public_key_report, _) =
            reference_verify_with_proof(&mutated_public_key, message, &keypair.account, &signature);
        assert!(!public_key_report.accepted);
        assert_eq!(public_key_report.reason, "challenge mismatch");

        let mut mutated_message = message.to_vec();
        mutated_message.push(0);
        let (message_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            &mutated_message,
            &keypair.account,
            &signature,
        );
        assert!(!message_report.accepted);
        assert_eq!(message_report.reason, "challenge mismatch");

        let mut mutated_response = signature.clone();
        mutated_response[response_start] ^= 0x01;
        let (response_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_response,
        );
        assert!(!response_report.accepted);

        let mut mutated_opening = signature.clone();
        mutated_opening[opening_start] ^= 0x01;
        let (opening_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_opening,
        );
        assert!(!opening_report.accepted);

        let mut mutated_proof_commitment_signature = signature.clone();
        mutated_proof_commitment_signature[proof_start] ^= 0x01;
        let (proof_commitment_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_proof_commitment_signature,
        );
        assert!(!proof_commitment_report.accepted);
        assert_eq!(proof_commitment_report.reason, "challenge mismatch");

        let mut mutated_masked_response_signature = signature.clone();
        mutated_masked_response_signature[proof_start + LINEAR_SIGMA_PROOF_COMMITMENT_BYTES] ^=
            0x01;
        let (proof_response_report, _) = reference_verify_with_proof(
            &keypair.public_key,
            message,
            &keypair.account,
            &mutated_masked_response_signature,
        );
        assert!(!proof_response_report.accepted);

        let proof = LinearSigmaTargetProof {
            proof_commitment: [9_u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
            masked_witness_response: keypair.account.slots[0]
                .ideal_masked_witness_response
                .unwrap(),
        };
        let encoded = encode_linear_sigma_target_proof(&proof);
        let mut mutated_commitment = encoded.clone();
        mutated_commitment[0] ^= 0x01;
        let decoded_commitment = decode_linear_sigma_target_proof(&mutated_commitment).unwrap();
        assert_ne!(decoded_commitment.proof_commitment, proof.proof_commitment);
        assert_eq!(
            decoded_commitment.masked_witness_response,
            proof.masked_witness_response
        );
        let mut mutated_response_proof = encoded;
        mutated_response_proof[LINEAR_SIGMA_PROOF_COMMITMENT_BYTES] ^= 0x01;
        let decoded_response = decode_linear_sigma_target_proof(&mutated_response_proof).unwrap();
        assert_eq!(decoded_response.proof_commitment, proof.proof_commitment);
        assert_ne!(
            decoded_response.masked_witness_response,
            proof.masked_witness_response
        );
    }

    #[test]
    fn parser_property_rejections_cover_malformed_and_noncanonical_inputs() {
        assert!(decode_challenge(&[0_u8; CHALLENGE_BYTES - 1]).is_err());
        let mut noncanonical_challenge = [0_u8; CHALLENGE_BYTES];
        noncanonical_challenge[ALGEBRAIC_CHALLENGE_BYTES - 1] = 0x80;
        assert!(
            decode_challenge(&noncanonical_challenge)
                .unwrap_err()
                .contains("high bits")
        );

        assert!(decode_responses(&[0_u8; ACTIVE_RESPONSE_BYTES - 1]).is_err());
        let mut bad_response = [0_u8; ACTIVE_RESPONSE_BYTES];
        bad_response[0] = (2 * RESPONSE_BOUND + 1) as u8;
        assert!(
            decode_responses(&bad_response)
                .unwrap_err()
                .contains("non-canonical")
        );

        assert!(decode_openings(&[0_u8; LOCAL_OPENING_BYTES - 1]).is_err());
        let mut bad_opening = [0_u8; LOCAL_OPENING_BYTES];
        bad_opening[0..2].copy_from_slice(&Q.to_le_bytes());
        assert!(
            decode_openings(&bad_opening)
                .unwrap_err()
                .contains("non-canonical")
        );

        assert!(
            decode_linear_sigma_target_proof(&[0_u8; LINEAR_SIGMA_TARGET_PROOF_BYTES - 1]).is_err()
        );

        let mut noncanonical_full_response = [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES];
        write_bits_le(&mut noncanonical_full_response, 0, SIS_RESIDUE_BITS, Q);
        assert!(full_response_vector_from_masked_witness(&noncanonical_full_response).is_none());

        let mut nonzero_slack = [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES];
        write_bits_le(&mut nonzero_slack, SIS_FULL_RESPONSE_PACKED_BITS, 1, 1);
        assert!(full_response_vector_from_masked_witness(&nonzero_slack).is_none());

        let mut bad_public_key = vec![0_u8; PK_BYTES];
        write_bits_le(&mut bad_public_key[PK_SEED_BYTES..], 0, SIS_RESIDUE_BITS, Q);
        assert!(
            decode_public_key(&bad_public_key)
                .unwrap_err()
                .contains("non-canonical")
        );
    }

    #[test]
    fn legacy_sha_verifier_rejects_quotient_digest() {
        let (public_key, message, account, signature) = valid_quotient_stateful_fixture();
        let (report, _) =
            verify_ideal_stateful_signature(&public_key, &message, &account, &signature);
        assert!(!report.accepted);
        assert_eq!(
            report.reason,
            "slot digest does not match SHA hidden preimage commitment"
        );
    }

    fn valid_stateful_fixture() -> (Vec<u8>, Vec<u8>, AccountState, Vec<u8>) {
        let public_key = zero_public_key();
        let message = b"stateful local delta test".to_vec();
        let mask_view = [12_u16, Q - 12];
        let fiber_state = fiber_state_bytes(local_state(mask_view));
        let seed = sample_seed();
        let hidden_preimage = encode_hidden_preimage(fiber_state, seed);
        let mut masked_witness_response = [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES];
        masked_witness_response[..HIDDEN_PREIMAGE_BYTES].copy_from_slice(&hidden_preimage);
        let digest = commitment_digest(&hidden_preimage);
        let account = AccountState {
            slots: vec![AccountSlot {
                digest,
                unused: true,
                ideal_masked_witness_response: Some(masked_witness_response),
            }],
        };
        let frame = legacy_stateful_challenge_hash_frame(&public_key, &message, 0, &digest);
        let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
        let challenge = decode_challenge(&challenge_bytes).unwrap();
        let mut responses = CompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        };
        let mut openings = LocalOpenings {
            active_free: [[0_u16; LOCAL_FREE]; REPS],
        };
        for rep in 0..REPS {
            responses.active_free[rep] = [12, -12];
            let block = challenge.active_block[rep];
            let pad = expand_pad_residues(&seed, block);
            openings.active_free[rep] =
                [add_mod(mask_view[0], pad[0]), add_mod(mask_view[1], pad[1])];
        }
        let mut signature = Vec::with_capacity(STATEFUL_TRANSACTION_SIGNATURE_BYTES);
        signature.extend_from_slice(&0_u32.to_le_bytes());
        signature.extend_from_slice(&challenge_bytes);
        signature.extend_from_slice(&encode_responses(&responses));
        signature.extend_from_slice(&encode_openings(&openings));
        (public_key, message, account, signature)
    }

    #[test]
    fn ideal_stateful_verifier_accepts_and_consumes_slot() {
        let (public_key, message, account, signature) = valid_stateful_fixture();
        let (report, next_account) =
            verify_ideal_stateful_signature(&public_key, &message, &account, &signature);
        assert!(report.accepted, "{report:?}");
        assert_eq!(report.signature_bytes, STATEFUL_TRANSACTION_SIGNATURE_BYTES);
        assert!(account.slots[0].unused);
        assert!(!next_account.slots[0].unused);

        let (reuse_report, _) =
            verify_ideal_stateful_signature(&public_key, &message, &next_account, &signature);
        assert!(!reuse_report.accepted);
        assert_eq!(reuse_report.reason, "slot is already consumed");
    }

    #[test]
    fn ideal_stateful_verifier_rejects_opening_mutation() {
        let (public_key, message, account, mut signature) = valid_stateful_fixture();
        let last = signature.last_mut().unwrap();
        *last ^= 1;
        let (report, _) =
            verify_ideal_stateful_signature(&public_key, &message, &account, &signature);
        assert!(!report.accepted);
        assert!(report.reason.contains("opening relation mismatch"));
    }

    #[test]
    fn ideal_stateful_verifier_rejects_message_mutation() {
        let (public_key, mut message, account, signature) = valid_stateful_fixture();
        message.push(0);
        let (report, _) =
            verify_ideal_stateful_signature(&public_key, &message, &account, &signature);
        assert!(!report.accepted);
        assert_eq!(report.reason, "challenge mismatch");
    }

    #[test]
    fn revealed_preimage_verifier_accepts_without_ideal_account() {
        let (public_key, message, mut account, mut signature) = valid_stateful_fixture();
        let hidden_preimage = account.slots[0].ideal_masked_witness_response.unwrap();
        account.slots[0].ideal_masked_witness_response = None;
        signature.extend_from_slice(&hidden_preimage);

        let (report, next_account) = verify_revealed_preimage_stateful_signature(
            &public_key,
            &message,
            &account,
            &signature,
        );
        assert!(report.accepted, "{report:?}");
        assert_eq!(report.proof_model, "revealed-hidden-preimage-public-check");
        assert_eq!(
            report.signature_bytes,
            REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES
        );
        assert!(account.slots[0].unused);
        assert!(!next_account.slots[0].unused);
    }

    #[test]
    fn revealed_preimage_verifier_rejects_preimage_mutation() {
        let (public_key, message, mut account, mut signature) = valid_stateful_fixture();
        let mut hidden_preimage = account.slots[0].ideal_masked_witness_response.unwrap();
        account.slots[0].ideal_masked_witness_response = None;
        hidden_preimage[0] ^= 1;
        signature.extend_from_slice(&hidden_preimage);

        let (report, _) = verify_revealed_preimage_stateful_signature(
            &public_key,
            &message,
            &account,
            &signature,
        );
        assert!(!report.accepted);
        assert_eq!(
            report.reason,
            "slot digest does not match SHA hidden preimage commitment"
        );
    }
}
