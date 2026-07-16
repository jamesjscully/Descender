use std::{
    env, fs,
    path::{Path, PathBuf},
};

use descender_parameter_search::production_reference::{
    PATH2_ACCOUNT_STARK_ADAPTIVE_BUDGET_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_ADAPTIVE_ORACLE_TERM_BOUND_BITS,
    PATH2_ACCOUNT_STARK_ADAPTIVE_QROM_POLICY_ADR,
    PATH2_ACCOUNT_STARK_ADAPTIVE_QROM_POLICY_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_ADAPTIVE_RBR_TERM_BOUND_BITS,
    PATH2_ACCOUNT_STARK_BCS_ACCOUNT_TARGET_MULTIPLIER,
    PATH2_ACCOUNT_STARK_BCS_CANDIDATE_OUTPUT_BITS, PATH2_ACCOUNT_STARK_BCS_DIMENSIONS_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_BCS_MULTI_TARGET_HORIZON_LOG2,
    PATH2_ACCOUNT_STARK_BCS_NORMALIZED_BUDGET_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_BCS_ORACLE_TERM_BOUND_BITS,
    PATH2_ACCOUNT_STARK_BCS_ORACLE_TERM_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_BCS_QROM_LIFT_CONSTANT_LOG2_BOUND,
    PATH2_ACCOUNT_STARK_BCS_QUANTUM_QUERY_BOUND_LOG2,
    PATH2_ACCOUNT_STARK_BCS_RBR_SCHEDULE_LABEL_BITS, PATH2_ACCOUNT_STARK_BCS_RBR_TERM_BOUND_BITS,
    PATH2_ACCOUNT_STARK_BCS_RBR_TERM_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_ACTIVE_QUERIES_V1,
    PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_AIR_SOURCE, PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_HASH_PERMS_V1,
    PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_PREIMAGE_ELEMS_V1,
    PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_Q128_TEST, PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_SHAPE_TEST,
    PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_TRACE_HEIGHT_V1, PATH2_ACCOUNT_STARK_FRI_CHAIN_AIR_SOURCE,
    PATH2_ACCOUNT_STARK_FRI_CHAIN_PUBLIC_Q128_TEST, PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_ROUND_0_V1,
    PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_ROUND_1_V1, PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_TEST,
    PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_AIR_SOURCE,
    PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_ATTACK_TEST,
    PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_Q128_TEST,
    PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_SHAPE_TEST,
    PATH2_ACCOUNT_STARK_FS_BOUND_SETTLEMENT_Q128_TEST,
    PATH2_ACCOUNT_STARK_FS_BOUND_SETTLEMENT_SOURCE,
    PATH2_ACCOUNT_STARK_FS_C9_REFERENCE_NATIVE_FIDELITY_TEST,
    PATH2_ACCOUNT_STARK_FS_C9_REFERENCE_SOURCE, PATH2_ACCOUNT_STARK_FS_CHALLENGER_CAPACITY,
    PATH2_ACCOUNT_STARK_FS_CHALLENGER_CAPACITY_BITS_FLOOR, PATH2_ACCOUNT_STARK_FS_CHALLENGER_RATE,
    PATH2_ACCOUNT_STARK_FS_CHALLENGER_SPONGE_FLOOR_BITS,
    PATH2_ACCOUNT_STARK_FS_PRODUCTION_OOD_STREAM_ELEMS_V1,
    PATH2_ACCOUNT_STARK_FS_RECURSIVE_AIR_SOURCE, PATH2_ACCOUNT_STARK_FS_RECURSIVE_OOD_NATIVE_TEST,
    PATH2_ACCOUNT_STARK_FS_RECURSIVE_PARTIAL_NATIVE_TEST,
    PATH2_ACCOUNT_STARK_FS_RECURSIVE_PARTIAL_TAMPER_TEST,
    PATH2_ACCOUNT_STARK_FS_RECURSIVE_RATE_MARKER, PATH2_ACCOUNT_STARK_FS_RECURSIVE_REAL_PROOF_TEST,
    PATH2_ACCOUNT_STARK_FS_RECURSIVE_TAMPER_TEST, PATH2_ACCOUNT_STARK_FS_TARGET_CAPACITY,
    PATH2_ACCOUNT_STARK_FS_TARGET_RATE, PATH2_ACCOUNT_STARK_FS_TARGET_SPONGE_FLOOR_BITS,
    PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_AIR_SOURCE,
    PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_NATIVE_TEST, PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_Q128_TEST,
    PATH2_ACCOUNT_STARK_LIFETIME_FIELD_BUDGET_NOT_LE_Q128_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_MERKLE_SPONGE_CAPACITY, PATH2_ACCOUNT_STARK_MERKLE_SPONGE_RATE,
    PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_COMPLETE_STATEMENT_ROOT_V1,
    PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_STATEMENT_KAT_PATH,
    PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_STATEMENT_KAT_SCHEMA,
    PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_TRANSCRIPT_ANCHOR_ROOT_V1,
    PATH2_ACCOUNT_STARK_PCS_OPENING_SHAPE_SOURCE, PATH2_ACCOUNT_STARK_PCS_OPENING_SHAPE_TEST,
    PATH2_ACCOUNT_STARK_PCS_OPENING_TERMS_V1, PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_ACTIVE_ROWS_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_AIR_SOURCE, PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_ATTACK_TEST,
    PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_BASE_WITNESS_BYTES_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_HEIGHT_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_PUBLIC_VALUES_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_Q128_TEST, PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_SHAPE_TEST,
    PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_TERMS_V1, PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_WIDTH_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_BASE_WITNESS_ALL_QUERIES_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_BASE_WITNESS_PER_QUERY_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_FRI_ROUNDS_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_LOG_HEIGHT_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_PROOFS_ALL_QUERIES_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_PROOFS_PER_QUERY_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_Q128_TEST,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_SHAPE_TEST,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_SOURCE,
    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_VERSION_V1,
    PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_CHECKED_MATERIALIZER,
    PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_CHECKED_MATERIALIZER_TEST,
    PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_MATERIALIZER,
    PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_MATERIALIZER_TEST,
    PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_VERIFIER,
    PATH2_ACCOUNT_STARK_PCS_REAL_PROOF_CONSUMER_TEST, PATH2_ACCOUNT_STARK_PCS_REDUCTION_AIR_SOURCE,
    PATH2_ACCOUNT_STARK_PCS_REDUCTION_ATTACK_TEST, PATH2_ACCOUNT_STARK_PCS_REDUCTION_Q128_TEST,
    PATH2_ACCOUNT_STARK_PCS_TRACE_WIDTH_V1, PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_AIR_SOURCE,
    PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_ATTACK_TEST,
    PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_Q128_TEST, PATH2_ACCOUNT_STARK_PUBLISHED_RBR_ANALYSIS_M,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_BOUND_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_DELTA_DENOMINATOR,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_DELTA_NUMERATOR,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_DIMENSIONS_LEAN_THEOREM,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_ERROR_SHAPE,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_FIELD_TERM_BOUND_BITS,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_QUERY_TERM_BOUND_BITS,
    PATH2_ACCOUNT_STARK_PUBLISHED_RBR_SOURCE, PATH2_ACCOUNT_STARK_SINGLE_LEAF_DEGREE_BITS,
    PATH2_ACCOUNT_STARK_SINGLE_LEAF_FRI_DOMAIN_LOG2,
    PATH2_ACCOUNT_STARK_TRANSCRIPT_ANCHORED_BLOCK_ROOT_DOMAIN_V1,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
};
use serde_json::Value;

fn require_contains(source: &str, needle: &str, label: &str) -> Result<(), String> {
    if source.contains(needle) {
        Ok(())
    } else {
        Err(format!("account-STARK config is missing {label}: {needle}"))
    }
}

fn rust_usize_literal(value: usize) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push('_');
        }
        out.push(ch);
    }
    out
}

fn repo_root() -> Result<PathBuf, String> {
    let cwd = env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let mut candidates = Vec::new();
    candidates.push(cwd.clone());
    if let Some(parent) = cwd.parent() {
        candidates.push(parent.to_path_buf());
        if let Some(grandparent) = parent.parent() {
            candidates.push(grandparent.to_path_buf());
        }
    }

    for candidate in candidates {
        if candidate.join("spec/README.md").exists()
            && candidate
                .join("tools/descender-account-stark/src/config.rs")
                .exists()
        {
            return Ok(candidate);
        }
    }
    Err("failed to locate Descender repository root".to_string())
}

fn load_json(root: &Path, path: &str) -> Result<Value, String> {
    let bytes = fs::read(root.join(path)).map_err(|err| format!("failed to read {path}: {err}"))?;
    serde_json::from_slice(&bytes).map_err(|err| format!("failed to parse {path}: {err}"))
}

fn json_usize<'a>(value: &'a Value, path: &[&str]) -> Result<usize, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    let value = cursor
        .as_u64()
        .ok_or_else(|| format!("JSON key {} is not an unsigned integer", path.join(".")))?;
    usize::try_from(value).map_err(|_| format!("JSON key {} does not fit usize", path.join(".")))
}

fn json_str<'a>(value: &'a Value, path: &[&str]) -> Result<&'a str, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    cursor
        .as_str()
        .ok_or_else(|| format!("JSON key {} is not a string", path.join(".")))
}

fn json_bool(value: &Value, path: &[&str]) -> Result<bool, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    cursor
        .as_bool()
        .ok_or_else(|| format!("JSON key {} is not a boolean", path.join(".")))
}

fn json_u32_vec(value: &Value, path: &[&str]) -> Result<Vec<u32>, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    cursor
        .as_array()
        .ok_or_else(|| format!("JSON key {} is not an array", path.join(".")))?
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let value = item.as_u64().ok_or_else(|| {
                format!(
                    "JSON key {}[{index}] is not an unsigned integer",
                    path.join(".")
                )
            })?;
            u32::try_from(value)
                .map_err(|_| format!("JSON key {}[{index}] does not fit u32", path.join(".")))
        })
        .collect()
}

fn json_string_vec<'a>(value: &'a Value, path: &[&str]) -> Result<Vec<&'a str>, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    let array = cursor
        .as_array()
        .ok_or_else(|| format!("JSON key {} is not an array", path.join(".")))?;
    array
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_str()
                .ok_or_else(|| format!("JSON key {}[{index}] is not a string", path.join(".")))
        })
        .collect()
}

fn check_equal<T: Eq + std::fmt::Debug>(label: &str, actual: T, expected: T) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label}: got {actual:?}, expected {expected:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root()?;
    let report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&report)?;
    if path2_account_stark_fri_qrom_residual_report_v1_digest(&report) != report.digest {
        return Err("account-STARK FRI/QROM residual report digest mismatch".to_string());
    }
    if report.residual_pricing_complete
        || report.deployment_ready
        || !report.account_stark_fri_qrom_transcript_reduction_bound
        || !report.bcs_dimensions_bound
        || !report.bcs_rbr_term_bound
        || !report.bcs_oracle_term_bound
        || !report.bcs_normalized_budget_q128_bound
        || !report.adaptive_budget_q128_bound
        || !report.fs_c9_reference_native_fidelity_bound
        || report.fs_c9_production_migration_complete
        || !report.adaptive_global_query_bound
        || report.bcs_account_target_multiplier != PATH2_ACCOUNT_STARK_BCS_ACCOUNT_TARGET_MULTIPLIER
        || report.lifetime_union_profile_selected
        || report.lifetime_union_bound_q128_established
        || report.bcs_challenger_output_mapping_instantiated
        || report.bcs_rbr_error_theorem_instantiated
        || report.bcs_qrom_lift_constant_reviewed
    {
        return Err("FRI/QROM residual lost fail-closed reduction/open-boundary state".to_string());
    }

    let config_source =
        fs::read_to_string(root.join(report.account_stark_config_source)).map_err(|err| {
            format!(
                "failed to read account-STARK config {}: {err}",
                report.account_stark_config_source
            )
        })?;
    require_contains(
        &config_source,
        &format!("pub const WIDTH: usize = {};", report.poseidon2_width),
        "Poseidon2 width",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_LOG_BLOWUP: usize = {};",
            report.fri_log_blowup
        ),
        "FRI log_blowup",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_NUM_QUERIES: usize = {};",
            report.fri_num_queries
        ),
        "FRI num_queries",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_QUERY_POW_BITS: usize = {};",
            report.fri_query_pow_bits
        ),
        "FRI query proof-of-work bits",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_COMMIT_POW_BITS: usize = {};",
            report.fri_commit_pow_bits
        ),
        "FRI commit proof-of-work bits",
    )?;
    require_contains(
        &config_source,
        "pub const SINGLE_PROOF_TRACE_ROWS: usize = 2;",
        "single-proof trace row count",
    )?;
    require_contains(
        &config_source,
        "pub type Challenge = BinomialExtensionField<Val, 8>;",
        "degree-8 challenge field",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub type MyHash = PaddingFreeSponge<Perm, 16, {}, DIGEST_ELEMS>;",
            PATH2_ACCOUNT_STARK_MERKLE_SPONGE_RATE
        ),
        "rate-7 Merkle/MMCS sponge",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub type Challenger = DuplexChallenger<Val, Perm, 16, {}>;",
            PATH2_ACCOUNT_STARK_FS_CHALLENGER_RATE
        ),
        "rate-7 Fiat-Shamir challenger",
    )?;
    check_equal(
        "Merkle/MMCS sponge capacity",
        report.poseidon2_width - PATH2_ACCOUNT_STARK_MERKLE_SPONGE_RATE,
        PATH2_ACCOUNT_STARK_MERKLE_SPONGE_CAPACITY,
    )?;
    check_equal(
        "Fiat-Shamir challenger capacity",
        report.poseidon2_width - PATH2_ACCOUNT_STARK_FS_CHALLENGER_RATE,
        PATH2_ACCOUNT_STARK_FS_CHALLENGER_CAPACITY,
    )?;
    check_equal(
        "Fiat-Shamir challenger rate",
        report.fs_challenger_rate,
        PATH2_ACCOUNT_STARK_FS_CHALLENGER_RATE,
    )?;
    check_equal(
        "Fiat-Shamir challenger report capacity",
        report.fs_challenger_capacity,
        PATH2_ACCOUNT_STARK_FS_CHALLENGER_CAPACITY,
    )?;
    check_equal(
        "Fiat-Shamir challenger capacity bits",
        report.fs_challenger_capacity_bits_floor,
        PATH2_ACCOUNT_STARK_FS_CHALLENGER_CAPACITY_BITS_FLOOR,
    )?;
    check_equal(
        "Fiat-Shamir challenger generic collision floor",
        report.fs_challenger_generic_collision_bits_floor,
        PATH2_ACCOUNT_STARK_FS_CHALLENGER_SPONGE_FLOOR_BITS,
    )?;
    check_equal(
        "target Fiat-Shamir challenger rate",
        report.fs_target_rate,
        PATH2_ACCOUNT_STARK_FS_TARGET_RATE,
    )?;
    check_equal(
        "target Fiat-Shamir challenger capacity",
        report.fs_target_capacity,
        PATH2_ACCOUNT_STARK_FS_TARGET_CAPACITY,
    )?;
    check_equal(
        "target Fiat-Shamir challenger floor",
        report.fs_target_generic_collision_bits_floor,
        PATH2_ACCOUNT_STARK_FS_TARGET_SPONGE_FLOOR_BITS,
    )?;
    check_equal(
        "capacity-9 reference source",
        report.fs_c9_reference_source,
        PATH2_ACCOUNT_STARK_FS_C9_REFERENCE_SOURCE,
    )?;
    check_equal(
        "capacity-9 native fidelity test",
        report.fs_c9_reference_native_fidelity_test,
        PATH2_ACCOUNT_STARK_FS_C9_REFERENCE_NATIVE_FIDELITY_TEST,
    )?;
    let recursive_air_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_FS_RECURSIVE_AIR_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read recursive AIR source {}: {err}",
            PATH2_ACCOUNT_STARK_FS_RECURSIVE_AIR_SOURCE
        )
    })?;
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_FS_RECURSIVE_RATE_MARKER,
        "recursive AIR rate-7/capacity-9 marker",
    )?;
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_FS_RECURSIVE_OOD_NATIVE_TEST,
        "recursive AIR OOD/batch-alpha/beta native-fidelity test",
    )?;
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_FS_RECURSIVE_TAMPER_TEST,
        "recursive AIR capacity-9 OOD permutation tamper test",
    )?;
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_FS_RECURSIVE_PARTIAL_NATIVE_TEST,
        "recursive AIR partial pre-alpha/quotient native-fidelity test",
    )?;
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_FS_RECURSIVE_PARTIAL_TAMPER_TEST,
        "recursive AIR partial transcript canonical-tail tamper test",
    )?;
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_FS_RECURSIVE_REAL_PROOF_TEST,
        "recursive AIR real-proof transcript capstone",
    )?;
    let vertical_ood_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_AIR_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read vertical OOD AIR source {}: {err}",
            PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_AIR_SOURCE
        )
    })?;
    require_contains(
        &vertical_ood_source,
        PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_NATIVE_TEST,
        "vertical production-shape OOD transcript native/tamper test",
    )?;
    require_contains(
        &vertical_ood_source,
        PATH2_ACCOUNT_STARK_FS_VERTICAL_OOD_Q128_TEST,
        "vertical OOD transcript q128 proof test",
    )?;
    let block_ood_source =
        fs::read_to_string(root.join(PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_AIR_SOURCE))
            .map_err(|err| {
                format!(
                    "failed to read block OOD binding AIR source {}: {err}",
                    PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_AIR_SOURCE
                )
            })?;
    require_contains(
        &block_ood_source,
        PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_ATTACK_TEST,
        "block OOD ordered-bus/shape/metadata attack test",
    )?;
    require_contains(
        &block_ood_source,
        PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_Q128_TEST,
        "block OOD binding q128 proof test",
    )?;
    require_contains(
        &block_ood_source,
        PATH2_ACCOUNT_STARK_FS_BLOCK_OOD_BINDING_SHAPE_TEST,
        "production OOD companion trace-shape test",
    )?;
    require_contains(
        &block_ood_source,
        &format!(
            "pub const PRODUCTION_OOD_STREAM_ELEMS_V1: usize = {}_{:03};",
            PATH2_ACCOUNT_STARK_FS_PRODUCTION_OOD_STREAM_ELEMS_V1 / 1000,
            PATH2_ACCOUNT_STARK_FS_PRODUCTION_OOD_STREAM_ELEMS_V1 % 1000
        ),
        "production OOD stream element count pin",
    )?;
    let opening_shape_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_PCS_OPENING_SHAPE_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read PCS opening shape source {}: {err}",
            PATH2_ACCOUNT_STARK_PCS_OPENING_SHAPE_SOURCE
        )
    })?;
    require_contains(
        &opening_shape_source,
        &format!(
            "pub const PRODUCTION_TRACE_WIDTH_V1: usize = {}_{:03};",
            PATH2_ACCOUNT_STARK_PCS_TRACE_WIDTH_V1 / 1000,
            PATH2_ACCOUNT_STARK_PCS_TRACE_WIDTH_V1 % 1000
        ),
        "production PCS trace-width pin",
    )?;
    require_contains(
        &opening_shape_source,
        &format!(
            "const _: [(); {}_{:03}] = [(); PRODUCTION_OPENING_TERMS_V1];",
            PATH2_ACCOUNT_STARK_PCS_OPENING_TERMS_V1 / 1000,
            PATH2_ACCOUNT_STARK_PCS_OPENING_TERMS_V1 % 1000
        ),
        "production PCS opening-term count pin",
    )?;
    require_contains(
        &opening_shape_source,
        PATH2_ACCOUNT_STARK_PCS_OPENING_SHAPE_TEST,
        "production PCS shape relation/stream fidelity test",
    )?;
    let reduction_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_PCS_REDUCTION_AIR_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read PCS reduction AIR source {}: {err}",
            PATH2_ACCOUNT_STARK_PCS_REDUCTION_AIR_SOURCE
        )
    })?;
    require_contains(
        &reduction_source,
        PATH2_ACCOUNT_STARK_PCS_REDUCTION_Q128_TEST,
        "vertical PCS reduction q128 proof test",
    )?;
    require_contains(
        &reduction_source,
        PATH2_ACCOUNT_STARK_PCS_REDUCTION_ATTACK_TEST,
        "vertical PCS reduction claim/order attack test",
    )?;
    let vertical_mmcs_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_AIR_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read vertical MMCS AIR source {}: {err}",
            PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_AIR_SOURCE
        )
    })?;
    require_contains(
        &vertical_mmcs_source,
        PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_Q128_TEST,
        "vertical MMCS q128 proof test",
    )?;
    require_contains(
        &vertical_mmcs_source,
        PATH2_ACCOUNT_STARK_PCS_VERTICAL_MMCS_ATTACK_TEST,
        "vertical MMCS input/path/root attack test",
    )?;
    let query_link_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_AIR_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read PCS query-link AIR source {}: {err}",
            PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_AIR_SOURCE
        )
    })?;
    for (needle, label) in [
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_Q128_TEST.to_string(),
            "PCS query-link q128 proof test",
        ),
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_ATTACK_TEST.to_string(),
            "PCS query-link cross-binding attack test",
        ),
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_SHAPE_TEST.to_string(),
            "PCS query-link production shape test",
        ),
        (
            format!(
                "assert_eq!(air.terms(), {});",
                rust_usize_literal(PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_TERMS_V1)
            ),
            "PCS query-link term count",
        ),
        (
            format!(
                "assert_eq!(air.active_rows(), {});",
                rust_usize_literal(PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_ACTIVE_ROWS_V1)
            ),
            "PCS query-link active rows",
        ),
        (
            format!(
                "assert_eq!(air.trace_height(), {});",
                rust_usize_literal(PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_HEIGHT_V1)
            ),
            "PCS query-link trace height",
        ),
        (
            format!(
                "assert_eq!(air.width(), {});",
                rust_usize_literal(PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_WIDTH_V1)
            ),
            "PCS query-link width",
        ),
        (
            format!(
                "assert_eq!(air.public_width(), {});",
                PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_PUBLIC_VALUES_V1
            ),
            "PCS query-link public width",
        ),
        (
            format!(
                "assert_eq!(air.base_witness_bytes(), {});",
                rust_usize_literal(PATH2_ACCOUNT_STARK_PCS_QUERY_LINK_BASE_WITNESS_BYTES_V1)
            ),
            "PCS query-link base witness bytes",
        ),
    ] {
        require_contains(&query_link_source, &needle, label)?;
    }
    let fri_chain_source = fs::read_to_string(root.join(PATH2_ACCOUNT_STARK_FRI_CHAIN_AIR_SOURCE))
        .map_err(|err| {
            format!(
                "failed to read FRI chain AIR source {}: {err}",
                PATH2_ACCOUNT_STARK_FRI_CHAIN_AIR_SOURCE
            )
        })?;
    require_contains(
        &fri_chain_source,
        PATH2_ACCOUNT_STARK_FRI_CHAIN_PUBLIC_Q128_TEST,
        "public reduced-opening to q128 FRI-chain proof test",
    )?;
    let query_settlement_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read PCS query settlement source {}: {err}",
            PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_SOURCE
        )
    })?;
    for (needle, label) in [
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_Q128_TEST.to_string(),
            "versioned PCS query settlement q128 composition test",
        ),
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_SHAPE_TEST.to_string(),
            "production PCS query settlement shape test",
        ),
        (
            format!(
                "pub fn {}(",
                PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_MATERIALIZER
            ),
            "all-query compact statement materializer",
        ),
        (
            format!(
                "pub fn {}(",
                PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_VERIFIER
            ),
            "all-query compact statement verifier",
        ),
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_MATERIALIZER_TEST.to_string(),
            "compact statement materialization root/count test",
        ),
        (
            format!(
                "pub fn {}(",
                PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_CHECKED_MATERIALIZER
            ),
            "all-query constraint-checked statement materializer",
        ),
        (
            PATH2_ACCOUNT_STARK_PCS_QUERY_STATEMENT_CHECKED_MATERIALIZER_TEST.to_string(),
            "component-trace constraint tamper test",
        ),
        (
            format!(
                "pub const PCS_QUERY_SETTLEMENT_VERSION_V1: u16 = {};",
                PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_VERSION_V1
            ),
            "PCS query settlement version",
        ),
        (
            format!(
                "assert_eq!(profile.log_height as usize, {});",
                PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_LOG_HEIGHT_V1
            ),
            "PCS query settlement log height",
        ),
        (
            format!(
                "assert_eq!(profile.fri_rounds as usize, {});",
                PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_FRI_ROUNDS_V1
            ),
            "PCS query settlement FRI rounds",
        ),
        (
            format!(
                "assert_eq!(profile.base_witness_bytes_per_query(), {});",
                rust_usize_literal(
                    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_BASE_WITNESS_PER_QUERY_V1
                )
            ),
            "PCS query settlement per-query base witness bytes",
        ),
        (
            format!(
                "assert_eq!(profile.base_witness_bytes_all_queries(), {});",
                rust_usize_literal(
                    PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_BASE_WITNESS_ALL_QUERIES_V1
                )
            ),
            "PCS query settlement all-query base witness bytes",
        ),
        (
            format!(
                "pub const PCS_QUERY_COMPONENT_PROOFS_V1: usize = {};",
                PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_PROOFS_PER_QUERY_V1
            ),
            "PCS query settlement component proofs per query",
        ),
        (
            format!(
                "assert_eq!(profile.component_proof_count(), {});",
                PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_PROOFS_ALL_QUERIES_V1
            ),
            "PCS query settlement all-query proof count",
        ),
    ] {
        require_contains(&query_settlement_source, &needle, label)?;
    }
    let all_query_kat = load_json(&root, PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_STATEMENT_KAT_PATH)?;
    check_equal(
        "all-query statement KAT schema",
        json_str(&all_query_kat, &["manifest", "schema"])?,
        PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_STATEMENT_KAT_SCHEMA,
    )?;
    check_equal(
        "all-query statement KAT query count",
        json_usize(&all_query_kat, &["manifest", "profile", "num_queries"])?,
        report.fri_num_queries,
    )?;
    check_equal(
        "all-query statement KAT deterministic proof worker count",
        json_usize(&all_query_kat, &["manifest", "proof_worker_threads"])?,
        1,
    )?;
    check_equal(
        "all-query statement KAT log height",
        json_usize(&all_query_kat, &["manifest", "profile", "log_height"])?,
        PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_LOG_HEIGHT_V1,
    )?;
    check_equal(
        "all-query statement KAT FRI rounds",
        json_usize(&all_query_kat, &["manifest", "profile", "fri_rounds"])?,
        PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_FRI_ROUNDS_V1,
    )?;
    check_equal(
        "all-query statement KAT constraint-checked component trace count",
        json_usize(
            &all_query_kat,
            &[
                "manifest",
                "profile",
                "constraint_checked_component_trace_count",
            ],
        )?,
        PATH2_ACCOUNT_STARK_PCS_QUERY_SETTLEMENT_PROOFS_ALL_QUERIES_V1,
    )?;
    check_equal(
        "all-query statement KAT FRI layer depths",
        json_u32_vec(
            &all_query_kat,
            &["manifest", "profile", "fri_layer_merkle_depths"],
        )?,
        vec![
            PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_ROUND_0_V1 as u32,
            PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_ROUND_1_V1 as u32,
        ],
    )?;
    check_equal(
        "all-query statement KAT query-index count",
        json_u32_vec(&all_query_kat, &["manifest", "query_indices"])?.len(),
        report.fri_num_queries,
    )?;
    check_equal(
        "all-query statement KAT transcript-anchor root",
        json_u32_vec(
            &all_query_kat,
            &["manifest", "transcript_anchor_root_koalabear"],
        )?,
        PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_TRANSCRIPT_ANCHOR_ROOT_V1.to_vec(),
    )?;
    check_equal(
        "all-query statement KAT complete-statement root",
        json_u32_vec(
            &all_query_kat,
            &["manifest", "complete_statement_root_koalabear"],
        )?,
        PATH2_ACCOUNT_STARK_PCS_ALL_QUERY_COMPLETE_STATEMENT_ROOT_V1.to_vec(),
    )?;
    for (key, expected) in [
        ("verified_native_relation_v3_proof", true),
        ("all_query_statements_materialized", true),
        ("all_query_component_air_traces_constraint_checked", true),
        ("all_query_statement_contract_verified", true),
        ("all_anchor_preimages_match_native_statements", true),
        ("one_five_component_query_envelope_proved", true),
        ("all_120_recursive_windows_proved", false),
        ("economical_all_query_aggregation_proved", false),
        ("deployment_ready", false),
    ] {
        check_equal(
            &format!("all-query statement KAT coverage.{key}"),
            json_bool(&all_query_kat, &["manifest", "coverage", key])?,
            expected,
        )?;
    }
    let fanout_anchor_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_AIR_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read fan-out anchor AIR source {}: {err}",
            PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_AIR_SOURCE
        )
    })?;
    for (needle, label) in [
        (
            PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_Q128_TEST.to_string(),
            "q128 padded fan-out anchor proof and block-root composition test",
        ),
        (
            PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_SHAPE_TEST.to_string(),
            "production fan-out anchor shape test",
        ),
        (
            format!(
                "assert_eq!(air.active_queries(), {});",
                PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_ACTIVE_QUERIES_V1
            ),
            "fan-out anchor active query count",
        ),
        (
            format!(
                "assert_eq!(air.trace_height(), {});",
                PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_TRACE_HEIGHT_V1
            ),
            "fan-out anchor padded trace height",
        ),
        (
            format!(
                "assert_eq!(TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1, {});",
                PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_PREIMAGE_ELEMS_V1
            ),
            "fan-out anchor preimage width",
        ),
        (
            format!(
                "assert_eq!(ANCHOR_PERMS, {});",
                PATH2_ACCOUNT_STARK_FANOUT_ANCHOR_HASH_PERMS_V1
            ),
            "fan-out anchor hash permutation count",
        ),
    ] {
        require_contains(&fanout_anchor_source, &needle, label)?;
    }
    require_contains(
        &recursive_air_source,
        PATH2_ACCOUNT_STARK_PCS_REAL_PROOF_CONSUMER_TEST,
        "genuine-proof all-120 PCS statement/query-link/MMCS/FRI capstone",
    )?;
    for (needle, label) in [
        (
            PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_TEST.to_string(),
            "production decreasing FRI layer-depth constraint/tamper test",
        ),
        (
            format!(
                "assert_eq!(layer_open_levels_for_round(&cfg, 0), {});",
                PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_ROUND_0_V1
            ),
            "production FRI round-0 Merkle depth",
        ),
        (
            format!(
                "assert_eq!(layer_open_levels_for_round(&cfg, 1), {});",
                PATH2_ACCOUNT_STARK_FRI_LAYER_DEPTH_ROUND_1_V1
            ),
            "production FRI round-1 Merkle depth",
        ),
    ] {
        require_contains(&recursive_air_source, &needle, label)?;
    }
    let settlement_source = fs::read_to_string(
        root.join(PATH2_ACCOUNT_STARK_FS_BOUND_SETTLEMENT_SOURCE),
    )
    .map_err(|err| {
        format!(
            "failed to read bound settlement source {}: {err}",
            PATH2_ACCOUNT_STARK_FS_BOUND_SETTLEMENT_SOURCE
        )
    })?;
    require_contains(
        &settlement_source,
        PATH2_ACCOUNT_STARK_FS_BOUND_SETTLEMENT_Q128_TEST,
        "paired block/OOD settlement q128 mixup-rejection test",
    )?;
    require_contains(
        &settlement_source,
        &format!(
            "pub const TRANSCRIPT_ANCHORED_BLOCK_ROOT_DOMAIN_V1: u32 = 0x{:04x}_{:04x};",
            PATH2_ACCOUNT_STARK_TRANSCRIPT_ANCHORED_BLOCK_ROOT_DOMAIN_V1 >> 16,
            PATH2_ACCOUNT_STARK_TRANSCRIPT_ANCHORED_BLOCK_ROOT_DOMAIN_V1 & 0xffff
        ),
        "transcript-anchored block-root domain",
    )?;
    if report.poseidon2_sponge_floor_bits != PATH2_ACCOUNT_STARK_FS_CHALLENGER_SPONGE_FLOOR_BITS {
        return Err(
            "FRI/QROM report has not converged the Merkle and Fiat-Shamir capacity-9 sponge floors"
                .to_string(),
        );
    }
    check_equal(
        "BCS quantum query bound",
        report.bcs_quantum_query_bound_log2,
        PATH2_ACCOUNT_STARK_BCS_QUANTUM_QUERY_BOUND_LOG2,
    )?;
    check_equal(
        "BCS multi-target horizon",
        report.bcs_multi_target_horizon_log2,
        PATH2_ACCOUNT_STARK_BCS_MULTI_TARGET_HORIZON_LOG2,
    )?;
    check_equal(
        "BCS candidate output bits",
        report.bcs_candidate_output_bits,
        PATH2_ACCOUNT_STARK_BCS_CANDIDATE_OUTPUT_BITS,
    )?;
    check_equal(
        "BCS RBR schedule label",
        report.bcs_rbr_schedule_label_bits,
        PATH2_ACCOUNT_STARK_BCS_RBR_SCHEDULE_LABEL_BITS,
    )?;
    check_equal(
        "BCS QROM lift constant bound",
        report.bcs_qrom_lift_constant_log2_bound,
        PATH2_ACCOUNT_STARK_BCS_QROM_LIFT_CONSTANT_LOG2_BOUND,
    )?;
    check_equal(
        "BCS RBR term bound",
        report.bcs_rbr_term_bound_bits,
        PATH2_ACCOUNT_STARK_BCS_RBR_TERM_BOUND_BITS,
    )?;
    check_equal(
        "BCS oracle term bound",
        report.bcs_oracle_term_bound_bits,
        PATH2_ACCOUNT_STARK_BCS_ORACLE_TERM_BOUND_BITS,
    )?;
    check_equal(
        "BCS query budget imports checked-core bound",
        report.bcs_quantum_query_bound_log2,
        report.checked_core_qrom_quantum_hash_query_bound_log2,
    )?;
    check_equal(
        "BCS target horizon imports checked-core bound",
        report.bcs_multi_target_horizon_log2,
        report.checked_core_qrom_selected_multi_target_horizon_log2,
    )?;
    check_equal(
        "BCS candidate output matches challenge field floor",
        report.bcs_candidate_output_bits,
        report.challenge_field_bits_floor,
    )?;
    check_equal(
        "BCS conditional RBR bits match schedule label",
        report.bcs_rbr_schedule_label_bits,
        report.fri_conservative_johnson_bits,
    )?;
    for (label, actual, expected) in [
        (
            "BCS dimensions theorem",
            report.bcs_dimensions_lean_theorem,
            PATH2_ACCOUNT_STARK_BCS_DIMENSIONS_LEAN_THEOREM,
        ),
        (
            "BCS RBR term theorem",
            report.bcs_rbr_term_lean_theorem,
            PATH2_ACCOUNT_STARK_BCS_RBR_TERM_LEAN_THEOREM,
        ),
        (
            "BCS oracle term theorem",
            report.bcs_oracle_term_lean_theorem,
            PATH2_ACCOUNT_STARK_BCS_ORACLE_TERM_LEAN_THEOREM,
        ),
        (
            "BCS normalized budget theorem",
            report.bcs_normalized_budget_lean_theorem,
            PATH2_ACCOUNT_STARK_BCS_NORMALIZED_BUDGET_LEAN_THEOREM,
        ),
    ] {
        check_equal(label, actual, expected)?;
    }
    for (label, actual, expected) in [
        (
            "published RBR source",
            report.published_rbr_source,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_SOURCE,
        ),
        (
            "published RBR error shape",
            report.published_rbr_error_shape,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_ERROR_SHAPE,
        ),
        (
            "published RBR dimensions theorem",
            report.published_rbr_dimensions_lean_theorem,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_DIMENSIONS_LEAN_THEOREM,
        ),
        (
            "published RBR bound theorem",
            report.published_rbr_bound_lean_theorem,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_BOUND_LEAN_THEOREM,
        ),
        (
            "adaptive budget theorem",
            report.adaptive_budget_lean_theorem,
            PATH2_ACCOUNT_STARK_ADAPTIVE_BUDGET_LEAN_THEOREM,
        ),
        (
            "adaptive QROM policy ADR",
            report.adaptive_qrom_policy_adr,
            PATH2_ACCOUNT_STARK_ADAPTIVE_QROM_POLICY_ADR,
        ),
        (
            "adaptive QROM policy theorem",
            report.adaptive_qrom_policy_lean_theorem,
            PATH2_ACCOUNT_STARK_ADAPTIVE_QROM_POLICY_LEAN_THEOREM,
        ),
        (
            "lifetime field budget theorem",
            report.lifetime_field_budget_not_le_q128_lean_theorem,
            PATH2_ACCOUNT_STARK_LIFETIME_FIELD_BUDGET_NOT_LE_Q128_LEAN_THEOREM,
        ),
    ] {
        check_equal(label, actual, expected)?;
    }
    for (label, actual, expected) in [
        (
            "single-leaf degree bits",
            report.single_leaf_degree_bits,
            PATH2_ACCOUNT_STARK_SINGLE_LEAF_DEGREE_BITS,
        ),
        (
            "single-leaf FRI domain log2",
            report.single_leaf_fri_domain_log2,
            PATH2_ACCOUNT_STARK_SINGLE_LEAF_FRI_DOMAIN_LOG2,
        ),
        (
            "published RBR analysis m",
            report.published_rbr_analysis_m,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_ANALYSIS_M,
        ),
        (
            "published RBR delta numerator",
            report.published_rbr_delta_numerator,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_DELTA_NUMERATOR,
        ),
        (
            "published RBR delta denominator",
            report.published_rbr_delta_denominator,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_DELTA_DENOMINATOR,
        ),
        (
            "published RBR field term bits",
            report.published_rbr_field_term_bound_bits,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_FIELD_TERM_BOUND_BITS,
        ),
        (
            "published RBR query term bits",
            report.published_rbr_query_term_bound_bits,
            PATH2_ACCOUNT_STARK_PUBLISHED_RBR_QUERY_TERM_BOUND_BITS,
        ),
        (
            "adaptive RBR term bits",
            report.adaptive_rbr_term_bound_bits,
            PATH2_ACCOUNT_STARK_ADAPTIVE_RBR_TERM_BOUND_BITS,
        ),
        (
            "adaptive oracle term bits",
            report.adaptive_oracle_term_bound_bits,
            PATH2_ACCOUNT_STARK_ADAPTIVE_ORACLE_TERM_BOUND_BITS,
        ),
    ] {
        check_equal(label, actual, expected)?;
    }

    check_equal(
        "conservative Johnson FRI bits",
        report.fri_num_queries * (report.fri_log_blowup / 2) + report.fri_query_pow_bits,
        report.fri_conservative_johnson_bits,
    )?;
    check_equal(
        "optimistic capacity FRI bits",
        report.fri_num_queries * report.fri_log_blowup + report.fri_query_pow_bits,
        report.fri_optimistic_capacity_bits,
    )?;

    let manifest = load_json(
        &root,
        "formal/kat/path2_account_stark_fri_qrom_residual_v1.json",
    )?;
    check_equal(
        "manifest transcript reduction theorem",
        json_str(
            &manifest,
            &[
                "manifest",
                "authority_snapshot",
                "account_stark_fri_qrom_transcript_reduction_lean_theorem",
            ],
        )?,
        report.account_stark_fri_qrom_transcript_reduction_lean_theorem,
    )?;
    check_equal(
        "manifest open transcript reduction bound",
        json_bool(
            &manifest,
            &[
                "manifest",
                "open_obligations",
                "account_stark_fri_qrom_transcript_reduction_bound",
            ],
        )?,
        report.account_stark_fri_qrom_transcript_reduction_bound,
    )?;
    for (label, key, expected) in [
        (
            "manifest BCS quantum query bound",
            "quantum_query_bound_log2",
            report.bcs_quantum_query_bound_log2,
        ),
        (
            "manifest BCS target horizon",
            "multi_target_horizon_log2",
            report.bcs_multi_target_horizon_log2,
        ),
        (
            "manifest BCS candidate output bits",
            "candidate_output_bits",
            report.bcs_candidate_output_bits,
        ),
        (
            "manifest BCS RBR schedule label",
            "rbr_schedule_label_bits",
            report.bcs_rbr_schedule_label_bits,
        ),
        (
            "manifest BCS QROM lift constant bound",
            "qrom_lift_constant_log2_bound",
            report.bcs_qrom_lift_constant_log2_bound,
        ),
        (
            "manifest BCS RBR term bound",
            "rbr_term_bound_bits",
            report.bcs_rbr_term_bound_bits,
        ),
        (
            "manifest BCS oracle term bound",
            "oracle_term_bound_bits",
            report.bcs_oracle_term_bound_bits,
        ),
    ] {
        check_equal(
            label,
            json_usize(&manifest, &["manifest", "conditional_bcs_qrom_budget", key])?,
            expected,
        )?;
    }
    for (label, key, expected) in [
        (
            "manifest BCS dimensions theorem",
            "dimensions_lean_theorem",
            report.bcs_dimensions_lean_theorem,
        ),
        (
            "manifest BCS RBR term theorem",
            "rbr_term_lean_theorem",
            report.bcs_rbr_term_lean_theorem,
        ),
        (
            "manifest BCS oracle term theorem",
            "oracle_term_lean_theorem",
            report.bcs_oracle_term_lean_theorem,
        ),
        (
            "manifest BCS normalized budget theorem",
            "normalized_budget_lean_theorem",
            report.bcs_normalized_budget_lean_theorem,
        ),
    ] {
        check_equal(
            label,
            json_str(&manifest, &["manifest", "conditional_bcs_qrom_budget", key])?,
            expected,
        )?;
    }
    for (label, key, expected) in [
        (
            "manifest BCS dimensions bound",
            "dimensions_bound",
            report.bcs_dimensions_bound,
        ),
        (
            "manifest BCS RBR term bound",
            "rbr_term_bound",
            report.bcs_rbr_term_bound,
        ),
        (
            "manifest BCS oracle term bound",
            "oracle_term_bound",
            report.bcs_oracle_term_bound,
        ),
        (
            "manifest BCS normalized q128 bound",
            "normalized_budget_q128_bound",
            report.bcs_normalized_budget_q128_bound,
        ),
    ] {
        check_equal(
            label,
            json_bool(&manifest, &["manifest", "conditional_bcs_qrom_budget", key])?,
            expected,
        )?;
    }
    for key in [
        "bcs_challenger_output_mapping_instantiated",
        "bcs_rbr_error_theorem_instantiated",
        "bcs_qrom_lift_constant_reviewed",
    ] {
        check_equal(
            "manifest open BCS theorem input",
            json_bool(&manifest, &["manifest", "open_obligations", key])?,
            false,
        )?;
    }
    for (label, key, expected) in [
        (
            "manifest single-leaf degree bits",
            "single_leaf_degree_bits",
            report.single_leaf_degree_bits,
        ),
        (
            "manifest single-leaf FRI domain log2",
            "single_leaf_fri_domain_log2",
            report.single_leaf_fri_domain_log2,
        ),
        (
            "manifest published RBR field term bits",
            "field_term_bound_bits",
            report.published_rbr_field_term_bound_bits,
        ),
        (
            "manifest published RBR query term bits",
            "query_term_bound_bits",
            report.published_rbr_query_term_bound_bits,
        ),
        (
            "manifest adaptive RBR term bits",
            "adaptive_rbr_term_bound_bits",
            report.adaptive_rbr_term_bound_bits,
        ),
        (
            "manifest adaptive oracle term bits",
            "adaptive_oracle_term_bound_bits",
            report.adaptive_oracle_term_bound_bits,
        ),
    ] {
        check_equal(
            label,
            json_usize(
                &manifest,
                &["manifest", "published_rbr_single_leaf_budget", key],
            )?,
            expected,
        )?;
    }
    check_equal(
        "manifest adaptive budget q128 bound",
        json_bool(
            &manifest,
            &[
                "manifest",
                "published_rbr_single_leaf_budget",
                "adaptive_budget_q128_bound",
            ],
        )?,
        true,
    )?;
    check_equal(
        "manifest adaptive global query policy",
        json_bool(
            &manifest,
            &[
                "manifest",
                "published_rbr_single_leaf_budget",
                "adaptive_global_query_bound",
            ],
        )?,
        true,
    )?;
    check_equal(
        "manifest account target multiplier",
        json_usize(
            &manifest,
            &[
                "manifest",
                "published_rbr_single_leaf_budget",
                "bcs_account_target_multiplier",
            ],
        )?,
        PATH2_ACCOUNT_STARK_BCS_ACCOUNT_TARGET_MULTIPLIER,
    )?;
    check_equal(
        "manifest lifetime profile selection",
        json_bool(
            &manifest,
            &[
                "manifest",
                "published_rbr_single_leaf_budget",
                "lifetime_union_profile_selected",
            ],
        )?,
        false,
    )?;
    check_equal(
        "manifest lifetime union-bound q128 status",
        json_bool(
            &manifest,
            &[
                "manifest",
                "published_rbr_single_leaf_budget",
                "lifetime_union_bound_q128_established",
            ],
        )?,
        false,
    )?;
    check_equal(
        "FRI/QROM domain vector case count",
        json_usize(
            &manifest,
            &[
                "manifest",
                "checked_core_qrom_budget",
                "qrom_domain_vector_case_count",
            ],
        )?,
        report.qrom_domain_vector_case_count,
    )?;
    check_equal(
        "FRI/QROM domain vector case ids",
        json_string_vec(
            &manifest,
            &[
                "manifest",
                "checked_core_qrom_budget",
                "qrom_domain_vector_case_ids",
            ],
        )?,
        report.qrom_domain_vector_case_ids.to_vec(),
    )?;

    let memo = load_json(&root, report.checked_core_qrom_artifacts[0])?;
    let review = load_json(&root, report.checked_core_qrom_artifacts[1])?;
    let frame_probe = load_json(&root, report.checked_core_qrom_artifacts[2])?;
    let multitarget = load_json(&root, report.checked_core_qrom_artifacts[3])?;

    check_equal(
        "QROM memo algebraic challenge bits",
        json_usize(&memo, &["challenge_derivation", "algebraic_challenge_bits"])?,
        report.checked_core_qrom_algebraic_challenge_bits,
    )?;
    check_equal(
        "QROM memo hardening bits",
        json_usize(&memo, &["challenge_derivation", "hardening_bits"])?,
        report.checked_core_qrom_hardening_bits,
    )?;
    check_equal(
        "QROM memo total transcript bits",
        json_usize(
            &memo,
            &["challenge_derivation", "total_transcript_challenge_bits"],
        )?,
        report.checked_core_qrom_total_transcript_bits,
    )?;
    check_equal(
        "QROM memo quantum query bound",
        json_usize(
            &memo,
            &["query_budget", "deployment_quantum_hash_query_bound_log2"],
        )?,
        report.checked_core_qrom_quantum_hash_query_bound_log2,
    )?;
    check_equal(
        "QROM memo classical query bound",
        json_usize(
            &memo,
            &["query_budget", "deployment_classical_hash_query_bound_log2"],
        )?,
        report.checked_core_qrom_classical_hash_query_bound_log2,
    )?;
    check_equal(
        "QROM memo selected target horizon",
        json_usize(
            &memo,
            &["query_budget", "selected_multi_target_horizon_log2"],
        )?,
        report.checked_core_qrom_selected_multi_target_horizon_log2,
    )?;
    check_equal(
        "QROM memo selected transcript term",
        json_usize(
            &memo,
            &["query_budget", "transcript_term_bits_after_qrom_loss"],
        )?,
        report.checked_core_qrom_selected_transcript_term_bits,
    )?;
    check_equal(
        "QROM review status",
        json_str(&review, &["status"])?,
        "checks_passed",
    )?;
    check_equal(
        "QROM review selected transcript term",
        json_usize(&review, &["derived_terms", "selected_transcript_term_bits"])?,
        report.checked_core_qrom_selected_transcript_term_bits,
    )?;
    check_equal(
        "QROM frame probe status",
        json_str(&frame_probe, &["status"])?,
        "checks_passed",
    )?;
    check_equal(
        "QROM frame probe max target cap",
        json_usize(
            &frame_probe,
            &["query_budget", "max_target_count_log2_at_128_bits"],
        )?,
        report.checked_core_qrom_max_target_count_log2_at_128_bits,
    )?;
    check_equal(
        "QROM multi-target status",
        json_str(&multitarget, &["status"])?,
        "closed_by_fail_closed_deployment_cap",
    )?;
    check_equal(
        "QROM multi-target fail-closed cap",
        json_usize(
            &multitarget,
            &["fail_closed_limit", "maximum_covered_target_count_log2"],
        )?,
        report.checked_core_qrom_max_target_count_log2_at_128_bits,
    )?;

    let source =
        fs::read_to_string(root.join("tools/parameter-search/src/production_reference.rs"))
            .map_err(|err| format!("failed to read production reference source: {err}"))?;
    for marker in [
        "PATH2_ACCOUNT_STARK_FRI_QROM_TRANSCRIPT_REDUCTION_LEAN_THEOREM",
        "account_stark_fri_qrom_transcript_reduction_lean_theorem",
        "account_stark_fri_qrom_transcript_reduction_bound",
        "PATH2_ACCOUNT_STARK_BCS_ROM_ERROR_SHAPE",
        "PATH2_ACCOUNT_STARK_BCS_QROM_ERROR_SHAPE",
        "bcs_challenger_output_mapping_instantiated",
        "bcs_rbr_error_theorem_instantiated",
        "bcs_qrom_lift_constant_reviewed",
        "published_rbr_source",
        "adaptive_qrom_policy_adr",
        "bcs_account_target_multiplier",
        "lifetime_union_bound_q128_established",
    ] {
        require_contains(&source, marker, marker)?;
    }

    let c9_reference = fs::read_to_string(root.join(report.fs_c9_reference_source))
        .map_err(|err| format!("failed to read capacity-9 challenger reference: {err}"))?;
    for marker in [
        "pub const C9_RATE: usize = 7;",
        "pub const C9_CAPACITY: usize = WIDTH - C9_RATE;",
        "extension_sample_spans_two_rate7_output_blocks",
        "mixed_plonky3_transcript_matches_native_rate7_challenger",
    ] {
        require_contains(&c9_reference, marker, marker)?;
    }

    let hash_budget = fs::read_to_string(root.join("Descender/Path2HashBudget.lean"))
        .map_err(|err| format!("failed to read Path2HashBudget.lean: {err}"))?;
    for marker in [
        "selectedAccountStarkFriQromDimensions_v1",
        "selectedAccountStarkFriQromRbrTerm_le_q130_v1",
        "selectedAccountStarkFriQromBcsOracleTerm_le_q130_v1",
        "selectedAccountStarkFriQromNormalizedBudget_le_q128_v1",
        "selectedAccountStarkPublishedFriRbrDimensions_v1",
        "selectedAccountStarkPublishedFriRbrBound_le_q218_v1",
        "selectedAccountStarkPublishedFriAdaptiveBudget_le_q128_v1",
        "selectedAccountStarkFriAdaptiveQromPolicy_v1",
        "selectedAccountStarkPublishedFriLifetimeFieldBudget_not_le_q128_v1",
    ] {
        require_contains(&hash_budget, marker, marker)?;
    }

    let lean = fs::read_to_string(root.join("Descender/Path2FullSpendConcrete.lean"))
        .map_err(|err| format!("failed to read Path2FullSpendConcrete.lean: {err}"))?;
    for marker in [
        "SelectedRustAcceptedBoundaryFriQromTranscriptWith",
        "selectedRustAcceptedBoundaryFriQromTranscriptReduction_v1",
    ] {
        require_contains(&lean, marker, marker)?;
    }

    println!("Path 2 account-STARK FRI/QROM residual consistency check passed.");
    Ok(())
}
