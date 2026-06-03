#![allow(clippy::manual_is_multiple_of)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::{Path, PathBuf};

const RATIONAL_GAUSSIAN_WEIGHT_PRECISION_BITS: u32 = 20;

#[derive(Clone, Debug)]
struct Config {
    q: u64,
    n: u64,
    eta: u64,
    product_block_groups: u64,
    target_bits: f64,
    digest_bytes: u64,
    max_log2_retries: f64,
    max_extra_repetitions: u64,
    fork_challenges_per_round: u64,
    block_weights: Vec<u64>,
    z_bounds: Vec<u64>,
    leakage_sigmas: Vec<u64>,
    output_json: PathBuf,
    output_md: PathBuf,
    output_certificate: PathBuf,
    output_lean_certificate: Option<PathBuf>,
    verify_certificate: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            q: 12289,
            n: 48,
            eta: 12,
            product_block_groups: 16,
            target_bits: 128.0,
            digest_bytes: 32,
            max_log2_retries: 20.0,
            max_extra_repetitions: 4,
            fork_challenges_per_round: 6,
            block_weights: (1..=16).collect(),
            z_bounds: vec![
                12, 16, 24, 32, 48, 64, 80, 96, 128, 160, 192, 224, 256, 320, 384, 448, 512, 640,
                768, 1024,
            ],
            leakage_sigmas: vec![32, 48, 64, 96, 128, 160, 224, 320, 448, 640, 896],
            output_json: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/sampler_analysis_n48.json",
            ),
            output_md: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/sampler_analysis_n48.md",
            ),
            output_certificate: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/conditioned_gaussian_leakage_certificate_n48.json",
            ),
            output_lean_certificate: None,
            verify_certificate: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct SamplerOutput {
    q: u64,
    n: u64,
    eta: u64,
    product_block_groups: u64,
    local_free_coordinates_per_block: u64,
    target_bits: f64,
    digest_bytes: u64,
    max_log2_retries: f64,
    max_extra_repetitions: u64,
    fork_challenges_per_round: u64,
    best_entropy_retry_rows: Vec<SamplerRow>,
    best_compact_fork_delta_16_rows: Vec<SamplerRow>,
    best_compact_fork_delta_24_rows: Vec<SamplerRow>,
    best_compact_fork_delta_32_rows: Vec<SamplerRow>,
    best_full_coverage_rows: Vec<SamplerRow>,
    best_fast_interior_leakage_rows: Vec<LeakageSamplerRow>,
    best_gaussian_leakage_rows: Vec<LeakageSamplerRow>,
    lowest_gaussian_leakage_rows: Vec<LeakageSamplerRow>,
    conclusions: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct SamplerRow {
    model: String,
    proof_status: String,
    block_weight: u64,
    challenge_description: String,
    challenge_entropy_bits: f64,
    repetitions: u64,
    local_free_coordinates_per_block: u64,
    active_response_coordinates: u64,
    accepted_z_bound: u64,
    mask_bound: u64,
    z_bits_per_coordinate: u64,
    response_bytes_per_repetition: u64,
    total_response_bytes: u64,
    challenge_encoding_bytes: u64,
    estimated_signature_bytes: u64,
    signature_bytes_with_digest: u64,
    coordinate_accept_probability: f64,
    log2_retry_per_repetition: f64,
    log2_expected_retries: f64,
    active_single_block_miss_bits: Option<f64>,
    active_any_block_miss_bits: Option<f64>,
    full_response_fork_delta_any_block_miss_bits: Option<f64>,
    compact_fork_delta_any_block_miss_bits: Option<f64>,
    compact_fork_delta_direct_forgery_bits: Option<f64>,
    compact_one_fork_delta_any_block_miss_bits: Option<f64>,
    full_block_coverage: bool,
    verdict: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LeakageSamplerRow {
    sampler: String,
    proof_status: String,
    block_weight: u64,
    repetitions: u64,
    serialized_response_coordinates: u64,
    accepted_z_bound: u64,
    sigma: Option<u64>,
    estimated_signature_bytes: u64,
    signature_bytes_with_digest: u64,
    min_accept_probability_per_coordinate: f64,
    max_accept_probability_per_coordinate: f64,
    accept_probability_ratio: f64,
    log2_expected_retries: f64,
    per_coordinate_tv: f64,
    signature_tv_union_bound: f64,
    per_coordinate_kl_bits: Option<f64>,
    signature_kl_bits: Option<f64>,
    pinsker_signature_tv_bound: Option<f64>,
    verdict: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LeakageCertificate {
    schema: String,
    profile: String,
    q: u64,
    n: u64,
    eta: u64,
    product_block_groups: u64,
    local_free_coordinates_per_block: u64,
    target_bits: f64,
    digest_bytes: u64,
    max_log2_retries: f64,
    max_extra_repetitions: u64,
    fork_challenges_per_round: u64,
    block_weights: Vec<u64>,
    z_bounds: Vec<u64>,
    leakage_sigmas: Vec<u64>,
    row_source: String,
    row_limit: usize,
    metric_definitions: Vec<String>,
    selected_rows: Vec<LeakageSamplerRow>,
    rational_interval_rows: Vec<RationalIntervalLeakageProofRow>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RationalInterval {
    lower_numerator: String,
    upper_numerator: String,
    denominator: String,
    upper_decimal: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RationalIntervalLeakageProofRow {
    sampler: String,
    proof_status: String,
    block_weight: u64,
    repetitions: u64,
    serialized_response_coordinates: u64,
    accepted_z_bound: u64,
    eta: u64,
    sigma: u64,
    weight_precision_bits: u32,
    support_min: i64,
    support_max: i64,
    weights_by_support_offset: Vec<String>,
    worst_shift_pair: [i64; 2],
    per_coordinate_tv_exact: RationalInterval,
    signature_tv_bound: RationalInterval,
}

#[derive(Clone, Debug)]
struct CoverageDiagnostics {
    active_single_block_miss_bits: Option<f64>,
    active_any_block_miss_bits: Option<f64>,
    full_response_fork_delta_any_block_miss_bits: Option<f64>,
    compact_fork_delta_any_block_miss_bits: Option<f64>,
    compact_one_fork_delta_any_block_miss_bits: Option<f64>,
}

type GaussianLeakageMetrics = (f64, f64, f64, f64, f64);
type TvRational = (u128, u128, [i64; 2]);

fn parse_csv_u64(s: &str) -> Vec<u64> {
    s.split(',')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().expect("invalid integer CSV"))
        .collect()
}

fn parse_args() -> Result<Config, String> {
    let mut cfg = Config::default();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |flag: &str| {
            args.next()
                .ok_or_else(|| format!("{flag} requires a value"))
        };
        match arg.as_str() {
            "--q" => cfg.q = value("--q")?.parse().map_err(|_| "invalid --q")?,
            "--n" => cfg.n = value("--n")?.parse().map_err(|_| "invalid --n")?,
            "--eta" => cfg.eta = value("--eta")?.parse().map_err(|_| "invalid --eta")?,
            "--product-block-groups" => {
                cfg.product_block_groups = value("--product-block-groups")?
                    .parse()
                    .map_err(|_| "invalid --product-block-groups")?
            }
            "--target-bits" => {
                cfg.target_bits = value("--target-bits")?
                    .parse()
                    .map_err(|_| "invalid --target-bits")?
            }
            "--digest-bytes" => {
                cfg.digest_bytes = value("--digest-bytes")?
                    .parse()
                    .map_err(|_| "invalid --digest-bytes")?
            }
            "--max-log2-retries" => {
                cfg.max_log2_retries = value("--max-log2-retries")?
                    .parse()
                    .map_err(|_| "invalid --max-log2-retries")?
            }
            "--max-extra-repetitions" => {
                cfg.max_extra_repetitions = value("--max-extra-repetitions")?
                    .parse()
                    .map_err(|_| "invalid --max-extra-repetitions")?
            }
            "--fork-challenges-per-round" => {
                cfg.fork_challenges_per_round = value("--fork-challenges-per-round")?
                    .parse()
                    .map_err(|_| "invalid --fork-challenges-per-round")?
            }
            "--block-weights" => cfg.block_weights = parse_csv_u64(&value("--block-weights")?),
            "--z-bounds" => cfg.z_bounds = parse_csv_u64(&value("--z-bounds")?),
            "--leakage-sigmas" => cfg.leakage_sigmas = parse_csv_u64(&value("--leakage-sigmas")?),
            "--output-json" => cfg.output_json = PathBuf::from(value("--output-json")?),
            "--output-md" => cfg.output_md = PathBuf::from(value("--output-md")?),
            "--output-certificate" => {
                cfg.output_certificate = PathBuf::from(value("--output-certificate")?)
            }
            "--output-lean-certificate" => {
                cfg.output_lean_certificate =
                    Some(PathBuf::from(value("--output-lean-certificate")?))
            }
            "--verify-certificate" => {
                cfg.verify_certificate = Some(PathBuf::from(value("--verify-certificate")?))
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(cfg)
}

fn print_help() {
    println!(
        "Analyze exact-box product-block rejection samplers\n\
         \n\
         The sampler uses y uniform in [-z-eta,z+eta]^d, response z=y+shift,\n\
         and accepts iff z lies in [-z,z]^d.  This exactly realizes a uniform\n\
         target response distribution for every shift with |shift_i| <= eta.\n\
         \n\
         Options:\n\
           --q N\n\
           --n N\n\
           --eta N\n\
           --product-block-groups N\n\
           --target-bits FLOAT\n\
           --digest-bytes N\n\
           --max-log2-retries FLOAT\n\
           --max-extra-repetitions N\n\
           --fork-challenges-per-round N\n\
           --block-weights CSV\n\
           --z-bounds CSV\n\
           --leakage-sigmas CSV\n\
           --output-json PATH\n\
           --output-md PATH\n\
           --output-certificate PATH\n\
           --output-lean-certificate PATH\n\
           --verify-certificate PATH\n\
         \n\
         The certificate contains rational interval proof rows for a concrete\n\
         dyadic quantized conditioned sampler.  Verification recomputes those\n\
         TV bounds exactly from the integer weight tables.  The Lean output\n\
         emits the verified rational constants as importable definitions."
    );
}

fn ceil_log2(n: u64) -> u64 {
    if n <= 1 {
        0
    } else {
        u64::BITS as u64 - (n - 1).leading_zeros() as u64
    }
}

fn packed_bytes(entries: u64, bits_per_entry: u64) -> u64 {
    entries.saturating_mul(bits_per_entry).saturating_add(7) / 8
}

fn packed_bits_as_bytes(bits: f64) -> u64 {
    ((bits.ceil() as u64).saturating_add(7)) / 8
}

fn log2_binomial(n: u64, k: u64) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    let k = k.min(n - k);
    (0..k)
        .map(|i| ((n - i) as f64).log2() - ((i + 1) as f64).log2())
        .sum()
}

fn binomial_f64(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

fn coordinate_accept_probability(z_bound: u64, eta: u64) -> f64 {
    (2 * z_bound + 1) as f64 / (2 * (z_bound + eta) + 1) as f64
}

fn single_block_miss_bits(groups: u64, weight: u64, repetitions: u64) -> Option<f64> {
    if weight >= groups {
        None
    } else {
        let avoid_probability = (groups - weight) as f64 / groups as f64;
        Some(repetitions as f64 * (-avoid_probability.log2()))
    }
}

fn any_block_miss_probability(groups: u64, weight: u64, repetitions: u64) -> f64 {
    if weight >= groups {
        return 0.0;
    }
    let denom = binomial_f64(groups, weight);
    let mut probability = 0.0;
    for missed in 1..=groups {
        if weight > groups - missed {
            continue;
        }
        let choices = binomial_f64(groups, missed);
        let avoid = binomial_f64(groups - missed, weight) / denom;
        let term = choices * avoid.powf(repetitions as f64);
        if missed % 2 == 1 {
            probability += term;
        } else {
            probability -= term;
        }
    }
    probability.clamp(0.0, 1.0)
}

fn any_block_miss_from_fixed_probability<F>(
    groups: u64,
    repetitions: u64,
    mut fixed_probability: F,
) -> f64
where
    F: FnMut(u64) -> f64,
{
    let mut probability = 0.0;
    for missed in 1..=groups {
        let p_one_round = fixed_probability(missed);
        let term = binomial_f64(groups, missed) * p_one_round.powf(repetitions as f64);
        if missed % 2 == 1 {
            probability += term;
        } else {
            probability -= term;
        }
    }
    probability.clamp(0.0, 1.0)
}

fn full_response_delta_zero_on_fixed_blocks_probability(
    groups: u64,
    weight: u64,
    fixed_blocks: u64,
    challenge_draws: u64,
) -> f64 {
    if fixed_blocks > groups {
        return 0.0;
    }
    let total = binomial_f64(groups, weight) * 2.0_f64.powf(weight as f64);
    if total == 0.0 {
        return 0.0;
    }
    let mut count = 0.0;
    for active_fixed in 0..=fixed_blocks.min(weight) {
        if weight - active_fixed > groups - fixed_blocks {
            continue;
        }
        let fixed_choices = binomial_f64(fixed_blocks, active_fixed);
        let fixed_signs = 2.0_f64.powf(active_fixed as f64);
        let outside = binomial_f64(groups - fixed_blocks, weight - active_fixed)
            * 2.0_f64.powf((weight - active_fixed) as f64);
        count += fixed_choices * fixed_signs * outside.powf(challenge_draws as f64);
    }
    count / total.powf(challenge_draws as f64)
}

fn full_response_fork_delta_any_block_miss_probability(
    groups: u64,
    weight: u64,
    repetitions: u64,
    challenge_draws: u64,
) -> f64 {
    if challenge_draws <= 1 {
        return 1.0;
    }
    any_block_miss_from_fixed_probability(groups, repetitions, |fixed_blocks| {
        full_response_delta_zero_on_fixed_blocks_probability(
            groups,
            weight,
            fixed_blocks,
            challenge_draws,
        )
    })
}

fn compact_delta_no_opposite_on_fixed_blocks_probability(
    groups: u64,
    weight: u64,
    fixed_blocks: u64,
    challenge_draws: u64,
) -> f64 {
    if fixed_blocks > groups || challenge_draws == 0 {
        return 0.0;
    }
    let total = binomial_f64(groups, weight) * 2.0_f64.powf(weight as f64);
    if total == 0.0 {
        return 0.0;
    }

    let outside_blocks = groups - fixed_blocks;
    let mut states: HashMap<(u64, u64, u64), f64> = HashMap::new();
    states.insert((fixed_blocks, 0, 0), 1.0);
    for _ in 0..challenge_draws {
        let mut next: HashMap<(u64, u64, u64), f64> = HashMap::new();
        for ((unseen, plus_only, minus_only), count) in states {
            for new_plus in 0..=unseen {
                for new_minus in 0..=unseen - new_plus {
                    let choose_unseen =
                        binomial_f64(unseen, new_plus) * binomial_f64(unseen - new_plus, new_minus);
                    for old_plus_active in 0..=plus_only {
                        let choose_plus = binomial_f64(plus_only, old_plus_active);
                        for old_minus_active in 0..=minus_only {
                            let fixed_active =
                                new_plus + new_minus + old_plus_active + old_minus_active;
                            if fixed_active > weight || weight - fixed_active > outside_blocks {
                                continue;
                            }
                            let choose_minus = binomial_f64(minus_only, old_minus_active);
                            let outside = binomial_f64(outside_blocks, weight - fixed_active)
                                * 2.0_f64.powf((weight - fixed_active) as f64);
                            let new_state = (
                                unseen - new_plus - new_minus,
                                plus_only + new_plus,
                                minus_only + new_minus,
                            );
                            *next.entry(new_state).or_insert(0.0) +=
                                count * choose_unseen * choose_plus * choose_minus * outside;
                        }
                    }
                }
            }
        }
        states = next;
    }

    let count: f64 = states.into_values().sum();
    count / total.powf(challenge_draws as f64)
}

fn compact_fork_delta_any_block_miss_probability(
    groups: u64,
    weight: u64,
    repetitions: u64,
    challenge_draws: u64,
) -> f64 {
    if challenge_draws <= 1 {
        return 1.0;
    }
    any_block_miss_from_fixed_probability(groups, repetitions, |fixed_blocks| {
        compact_delta_no_opposite_on_fixed_blocks_probability(
            groups,
            weight,
            fixed_blocks,
            challenge_draws,
        )
    })
}

fn probability_bits(probability: f64) -> Option<f64> {
    if probability <= 0.0 {
        None
    } else if probability >= 1.0 {
        Some(0.0)
    } else {
        Some(-probability.log2())
    }
}

fn direct_forgery_bits(coverage_bits: Option<f64>, fork_challenges_per_round: u64) -> Option<f64> {
    coverage_bits.map(|bits| bits / fork_challenges_per_round as f64)
}

fn fast_interior_coordinate_tv(z_bound: u64, eta: u64) -> Option<f64> {
    if z_bound < eta {
        None
    } else {
        Some((eta as f64 / (2 * (z_bound - eta) + 1) as f64).min(1.0))
    }
}

fn log_sum_exp(values: &[f64]) -> f64 {
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !max.is_finite() {
        return max;
    }
    let scaled_sum: f64 = values.iter().map(|value| (value - max).exp()).sum();
    max + scaled_sum.ln()
}

fn gaussian_log_weight(x: i64, sigma: u64) -> f64 {
    let sigma = sigma as f64;
    let x = x as f64;
    -(x * x) / (2.0 * sigma * sigma)
}

fn kl_bits(log_p: &[f64], log_q: &[f64]) -> f64 {
    log_p
        .iter()
        .zip(log_q)
        .map(|(&p, &q)| {
            let probability = p.exp();
            probability * (p - q) / std::f64::consts::LN_2
        })
        .sum()
}

fn tv_from_logs(log_p: &[f64], log_q: &[f64]) -> f64 {
    0.5 * log_p
        .iter()
        .zip(log_q)
        .map(|(&p, &q)| (p.exp() - q.exp()).abs())
        .sum::<f64>()
}

fn gaussian_shifted_conditioned_metrics(
    z_bound: u64,
    eta: u64,
    sigma: u64,
) -> Option<(f64, f64, f64, f64, f64)> {
    if sigma == 0 {
        return None;
    }
    let z_bound = z_bound as i64;
    let eta = eta as i64;
    let proposal_logs: Vec<f64> = (-z_bound - eta..=z_bound + eta)
        .map(|y| gaussian_log_weight(y, sigma))
        .collect();
    let proposal_log_mass = log_sum_exp(&proposal_logs);

    let mut shifted_logs = Vec::new();
    let mut accept_probabilities = Vec::new();
    for shift in -eta..=eta {
        let response_logs: Vec<f64> = (-z_bound..=z_bound)
            .map(|z| gaussian_log_weight(z - shift, sigma))
            .collect();
        let accepted_log_mass = log_sum_exp(&response_logs);
        accept_probabilities.push((accepted_log_mass - proposal_log_mass).exp());
        shifted_logs.push(
            response_logs
                .iter()
                .map(|value| value - accepted_log_mass)
                .collect::<Vec<_>>(),
        );
    }

    let mut max_tv = 0.0_f64;
    let mut max_kl_bits = 0.0_f64;
    for left in 0..shifted_logs.len() {
        for right in 0..shifted_logs.len() {
            max_tv = max_tv.max(tv_from_logs(&shifted_logs[left], &shifted_logs[right]));
            max_kl_bits = max_kl_bits.max(kl_bits(&shifted_logs[left], &shifted_logs[right]));
        }
    }
    let min_accept = accept_probabilities
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let max_accept = accept_probabilities
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    Some((
        min_accept,
        max_accept,
        if min_accept > 0.0 {
            max_accept / min_accept
        } else {
            f64::INFINITY
        },
        max_tv,
        max_kl_bits,
    ))
}

fn leakage_verdict(
    sampler: &str,
    signature_tv_union_bound: f64,
    signature_kl_bits: Option<f64>,
    log2_expected_retries: f64,
    max_log2_retries: f64,
) -> String {
    if signature_tv_union_bound == 0.0 && signature_kl_bits == Some(0.0) {
        "secret-independent".to_string()
    } else if sampler == "fast-interior-shifted-uniform" {
        "support-shift-leakage-too-large-for-security-claim".to_string()
    } else if log2_expected_retries > max_log2_retries {
        "low-leakage-but-retry-too-high".to_string()
    } else if signature_kl_bits.unwrap_or(f64::INFINITY) <= 0.01 {
        "candidate-low-leakage".to_string()
    } else if signature_kl_bits.unwrap_or(f64::INFINITY) <= 1.0 {
        "bounded-leakage-but-needs-security-budget".to_string()
    } else {
        "leakage-too-large-for-direct-claim".to_string()
    }
}

fn pinsker_tv_bound(kl_bits: f64) -> f64 {
    ((std::f64::consts::LN_2 * kl_bits) / 2.0).sqrt().min(1.0)
}

fn rational_interval_exact(numerator: u128, denominator: u128) -> RationalInterval {
    RationalInterval {
        lower_numerator: numerator.to_string(),
        upper_numerator: numerator.to_string(),
        denominator: denominator.to_string(),
        upper_decimal: numerator as f64 / denominator as f64,
    }
}

fn rational_interval_upper(numerator: u128, denominator: u128) -> RationalInterval {
    RationalInterval {
        lower_numerator: "0".to_string(),
        upper_numerator: numerator.to_string(),
        denominator: denominator.to_string(),
        upper_decimal: numerator as f64 / denominator as f64,
    }
}

fn parse_u128_field(name: &str, value: &str) -> Result<u128, String> {
    value
        .parse::<u128>()
        .map_err(|err| format!("{name} is not a u128 decimal: {err}"))
}

fn quantized_gaussian_weights(
    z_bound: u64,
    eta: u64,
    sigma: u64,
    precision_bits: u32,
) -> Vec<u128> {
    assert!(precision_bits < u128::BITS);
    let scale = (1_u128 << precision_bits) as f64;
    let support = z_bound + eta;
    (-(support as i64)..=(support as i64))
        .map(|x| {
            let weight = (scale * gaussian_log_weight(x, sigma).exp()).round();
            (weight as u128).max(1)
        })
        .collect()
}

fn weight_at(weights: &[u128], support_min: i64, x: i64) -> u128 {
    weights[(x - support_min) as usize]
}

fn accepted_weight_sum(weights: &[u128], support_min: i64, z_bound: u64, shift: i64) -> u128 {
    let z = z_bound as i64;
    (-z..=z)
        .map(|r| weight_at(weights, support_min, r - shift))
        .sum()
}

fn rational_tv_from_weights(
    weights: &[u128],
    support_min: i64,
    z_bound: u64,
    eta: u64,
) -> TvRational {
    let z = z_bound as i64;
    let eta = eta as i64;
    let mut best_num = 0_u128;
    let mut best_den = 1_u128;
    let mut best_pair = [0_i64, 0_i64];
    for left_shift in -eta..=eta {
        let left_sum = accepted_weight_sum(weights, support_min, z_bound, left_shift);
        for right_shift in -eta..=eta {
            let right_sum = accepted_weight_sum(weights, support_min, z_bound, right_shift);
            let mut numerator = 0_u128;
            for r in -z..=z {
                let left = weight_at(weights, support_min, r - left_shift);
                let right = weight_at(weights, support_min, r - right_shift);
                let left_scaled = left
                    .checked_mul(right_sum)
                    .expect("TV numerator term overflow");
                let right_scaled = right
                    .checked_mul(left_sum)
                    .expect("TV numerator term overflow");
                numerator = numerator
                    .checked_add(left_scaled.abs_diff(right_scaled))
                    .expect("TV numerator overflow");
            }
            let denominator = 2_u128
                .checked_mul(left_sum)
                .and_then(|value| value.checked_mul(right_sum))
                .expect("TV denominator overflow");
            if numerator
                .checked_mul(best_den)
                .expect("TV comparison overflow")
                > best_num
                    .checked_mul(denominator)
                    .expect("TV comparison overflow")
            {
                best_num = numerator;
                best_den = denominator;
                best_pair = [left_shift, right_shift];
            }
        }
    }
    (best_num, best_den, best_pair)
}

fn signature_tv_bound_from_coordinate(
    serialized_response_coordinates: u64,
    numerator: u128,
    denominator: u128,
) -> (u128, u128) {
    let scaled = numerator
        .checked_mul(serialized_response_coordinates as u128)
        .expect("signature TV numerator overflow");
    if scaled >= denominator {
        (1, 1)
    } else {
        (scaled, denominator)
    }
}

fn rational_interval_row_from_leakage_row(
    row: &LeakageSamplerRow,
    eta: u64,
) -> Option<RationalIntervalLeakageProofRow> {
    if row.sampler != "conditioned-shifted-discrete-gaussian" {
        return None;
    }
    let sigma = row.sigma?;
    let support = row.accepted_z_bound + eta;
    let support_min = -(support as i64);
    let support_max = support as i64;
    let weights = quantized_gaussian_weights(
        row.accepted_z_bound,
        eta,
        sigma,
        RATIONAL_GAUSSIAN_WEIGHT_PRECISION_BITS,
    );
    let (tv_num, tv_den, worst_pair) =
        rational_tv_from_weights(&weights, support_min, row.accepted_z_bound, eta);
    let (sig_num, sig_den) =
        signature_tv_bound_from_coordinate(row.serialized_response_coordinates, tv_num, tv_den);
    Some(RationalIntervalLeakageProofRow {
        sampler: "dyadic-conditioned-shifted-discrete-gaussian".to_string(),
        proof_status: "exact-rational-tv-bound-for-quantized-weight-table".to_string(),
        block_weight: row.block_weight,
        repetitions: row.repetitions,
        serialized_response_coordinates: row.serialized_response_coordinates,
        accepted_z_bound: row.accepted_z_bound,
        eta,
        sigma,
        weight_precision_bits: RATIONAL_GAUSSIAN_WEIGHT_PRECISION_BITS,
        support_min,
        support_max,
        weights_by_support_offset: weights.iter().map(u128::to_string).collect(),
        worst_shift_pair: worst_pair,
        per_coordinate_tv_exact: rational_interval_exact(tv_num, tv_den),
        signature_tv_bound: rational_interval_upper(sig_num, sig_den),
    })
}

fn coverage_diagnostics(cfg: &Config, weight: u64, repetitions: u64) -> CoverageDiagnostics {
    CoverageDiagnostics {
        active_single_block_miss_bits: single_block_miss_bits(
            cfg.product_block_groups,
            weight,
            repetitions,
        ),
        active_any_block_miss_bits: probability_bits(any_block_miss_probability(
            cfg.product_block_groups,
            weight,
            repetitions,
        )),
        full_response_fork_delta_any_block_miss_bits: probability_bits(
            full_response_fork_delta_any_block_miss_probability(
                cfg.product_block_groups,
                weight,
                repetitions,
                cfg.fork_challenges_per_round,
            ),
        ),
        compact_fork_delta_any_block_miss_bits: probability_bits(
            compact_fork_delta_any_block_miss_probability(
                cfg.product_block_groups,
                weight,
                repetitions,
                cfg.fork_challenges_per_round,
            ),
        ),
        compact_one_fork_delta_any_block_miss_bits: probability_bits(
            compact_fork_delta_any_block_miss_probability(
                cfg.product_block_groups,
                weight,
                repetitions,
                2,
            ),
        ),
    }
}

fn build_row(
    cfg: &Config,
    local_free: u64,
    weight: u64,
    repetitions: u64,
    z_bound: u64,
    coverage: &CoverageDiagnostics,
) -> Option<SamplerRow> {
    if weight == 0 || weight > cfg.product_block_groups {
        return None;
    }
    let challenge_entropy_bits = log2_binomial(cfg.product_block_groups, weight) + weight as f64;
    if challenge_entropy_bits <= 0.0 {
        return None;
    }
    let active_response_coordinates = weight.saturating_mul(local_free);
    let coordinate_accept_probability = coordinate_accept_probability(z_bound, cfg.eta);
    let log2_retry_per_repetition =
        active_response_coordinates as f64 * (-coordinate_accept_probability.log2());
    let log2_expected_retries = repetitions as f64 * log2_retry_per_repetition;
    let z_bits_per_coordinate = ceil_log2(2 * z_bound + 1);
    let response_bytes_per_repetition =
        packed_bytes(active_response_coordinates, z_bits_per_coordinate);
    let total_response_bytes = repetitions.saturating_mul(response_bytes_per_repetition);
    let challenge_encoding_bytes =
        packed_bits_as_bytes(repetitions as f64 * challenge_entropy_bits);
    let estimated_signature_bytes = challenge_encoding_bytes.saturating_add(total_response_bytes);
    let signature_bytes_with_digest = cfg.digest_bytes.saturating_add(total_response_bytes);
    let full_block_coverage = weight == cfg.product_block_groups;
    let verdict = if log2_expected_retries > cfg.max_log2_retries {
        "retry-cost-too-high"
    } else if coverage
        .compact_fork_delta_any_block_miss_bits
        .unwrap_or(0.0)
        >= 32.0
    {
        "candidate-compact-fork-delta-32"
    } else if coverage
        .compact_fork_delta_any_block_miss_bits
        .unwrap_or(0.0)
        >= 24.0
    {
        "candidate-compact-fork-delta-24"
    } else if coverage
        .compact_fork_delta_any_block_miss_bits
        .unwrap_or(0.0)
        >= 16.0
    {
        "candidate-compact-fork-delta-16"
    } else if full_block_coverage {
        "full-block-response-weak-fork-delta"
    } else {
        "entropy-retry-pass-weak-fork-delta"
    }
    .to_string();
    Some(SamplerRow {
        model: "exact-box-active-product-block-response".to_string(),
        proof_status: "exact-uniform-target-if-rejection-certificate-instantiated".to_string(),
        block_weight: weight,
        challenge_description: format!(
            "choose {weight} signed product blocks among {}",
            cfg.product_block_groups
        ),
        challenge_entropy_bits,
        repetitions,
        local_free_coordinates_per_block: local_free,
        active_response_coordinates,
        accepted_z_bound: z_bound,
        mask_bound: z_bound + cfg.eta,
        z_bits_per_coordinate,
        response_bytes_per_repetition,
        total_response_bytes,
        challenge_encoding_bytes,
        estimated_signature_bytes,
        signature_bytes_with_digest,
        coordinate_accept_probability,
        log2_retry_per_repetition,
        log2_expected_retries,
        active_single_block_miss_bits: coverage.active_single_block_miss_bits,
        active_any_block_miss_bits: coverage.active_any_block_miss_bits,
        full_response_fork_delta_any_block_miss_bits: coverage
            .full_response_fork_delta_any_block_miss_bits,
        compact_fork_delta_any_block_miss_bits: coverage.compact_fork_delta_any_block_miss_bits,
        compact_fork_delta_direct_forgery_bits: direct_forgery_bits(
            coverage.compact_fork_delta_any_block_miss_bits,
            cfg.fork_challenges_per_round,
        ),
        compact_one_fork_delta_any_block_miss_bits: coverage
            .compact_one_fork_delta_any_block_miss_bits,
        full_block_coverage,
        verdict,
    })
}

fn keep_best(mut rows: Vec<SamplerRow>, limit: usize) -> Vec<SamplerRow> {
    rows.sort_by(|a, b| {
        a.estimated_signature_bytes
            .cmp(&b.estimated_signature_bytes)
            .then_with(|| {
                a.log2_expected_retries
                    .partial_cmp(&b.log2_expected_retries)
                    .unwrap()
            })
            .then_with(|| {
                b.challenge_entropy_bits
                    .partial_cmp(&a.challenge_entropy_bits)
                    .unwrap()
            })
    });
    rows.truncate(limit);
    rows
}

fn keep_leakage_best_by_size(
    mut rows: Vec<LeakageSamplerRow>,
    limit: usize,
) -> Vec<LeakageSamplerRow> {
    rows.sort_by(|a, b| {
        a.estimated_signature_bytes
            .cmp(&b.estimated_signature_bytes)
            .then_with(|| {
                a.signature_tv_union_bound
                    .partial_cmp(&b.signature_tv_union_bound)
                    .unwrap()
            })
            .then_with(|| {
                a.signature_kl_bits
                    .unwrap_or(f64::INFINITY)
                    .partial_cmp(&b.signature_kl_bits.unwrap_or(f64::INFINITY))
                    .unwrap()
            })
    });
    rows.truncate(limit);
    rows
}

fn keep_leakage_best_by_kl(
    mut rows: Vec<LeakageSamplerRow>,
    limit: usize,
) -> Vec<LeakageSamplerRow> {
    rows.sort_by(|a, b| {
        a.signature_kl_bits
            .unwrap_or(f64::INFINITY)
            .partial_cmp(&b.signature_kl_bits.unwrap_or(f64::INFINITY))
            .unwrap()
            .then_with(|| {
                a.signature_tv_union_bound
                    .partial_cmp(&b.signature_tv_union_bound)
                    .unwrap()
            })
            .then_with(|| {
                a.estimated_signature_bytes
                    .cmp(&b.estimated_signature_bytes)
            })
    });
    rows.truncate(limit);
    rows
}

fn fast_interior_leakage_row(cfg: &Config, row: &SamplerRow) -> Option<LeakageSamplerRow> {
    let per_coordinate_tv = fast_interior_coordinate_tv(row.accepted_z_bound, cfg.eta)?;
    let serialized_response_coordinates = row
        .repetitions
        .saturating_mul(row.active_response_coordinates);
    let signature_tv_union_bound =
        (serialized_response_coordinates as f64 * per_coordinate_tv).min(1.0);
    Some(LeakageSamplerRow {
        sampler: "fast-interior-shifted-uniform".to_string(),
        proof_status: "valid-signature-only; response law depends on the secret shift".to_string(),
        block_weight: row.block_weight,
        repetitions: row.repetitions,
        serialized_response_coordinates,
        accepted_z_bound: row.accepted_z_bound,
        sigma: None,
        estimated_signature_bytes: row.estimated_signature_bytes,
        signature_bytes_with_digest: row.signature_bytes_with_digest,
        min_accept_probability_per_coordinate: 1.0,
        max_accept_probability_per_coordinate: 1.0,
        accept_probability_ratio: 1.0,
        log2_expected_retries: 0.0,
        per_coordinate_tv,
        signature_tv_union_bound,
        per_coordinate_kl_bits: None,
        signature_kl_bits: None,
        pinsker_signature_tv_bound: None,
        verdict: leakage_verdict(
            "fast-interior-shifted-uniform",
            signature_tv_union_bound,
            None,
            0.0,
            cfg.max_log2_retries,
        ),
    })
}

fn gaussian_leakage_row(
    cfg: &Config,
    row: &SamplerRow,
    sigma: u64,
    metrics: GaussianLeakageMetrics,
) -> LeakageSamplerRow {
    let (min_accept, max_accept, accept_ratio, per_coordinate_tv, per_coordinate_kl_bits) = metrics;
    let serialized_response_coordinates = row
        .repetitions
        .saturating_mul(row.active_response_coordinates);
    let signature_tv_union_bound =
        (serialized_response_coordinates as f64 * per_coordinate_tv).min(1.0);
    let signature_kl_bits = serialized_response_coordinates as f64 * per_coordinate_kl_bits;
    let log2_expected_retries =
        serialized_response_coordinates as f64 * (-(min_accept.max(f64::MIN_POSITIVE)).log2());
    LeakageSamplerRow {
        sampler: "conditioned-shifted-discrete-gaussian".to_string(),
        proof_status: "leakage-bounded-candidate; not secret-independent".to_string(),
        block_weight: row.block_weight,
        repetitions: row.repetitions,
        serialized_response_coordinates,
        accepted_z_bound: row.accepted_z_bound,
        sigma: Some(sigma),
        estimated_signature_bytes: row.estimated_signature_bytes,
        signature_bytes_with_digest: row.signature_bytes_with_digest,
        min_accept_probability_per_coordinate: min_accept,
        max_accept_probability_per_coordinate: max_accept,
        accept_probability_ratio: accept_ratio,
        log2_expected_retries,
        per_coordinate_tv,
        signature_tv_union_bound,
        per_coordinate_kl_bits: Some(per_coordinate_kl_bits),
        signature_kl_bits: Some(signature_kl_bits),
        pinsker_signature_tv_bound: Some(pinsker_tv_bound(signature_kl_bits)),
        verdict: leakage_verdict(
            "conditioned-shifted-discrete-gaussian",
            signature_tv_union_bound,
            Some(signature_kl_bits),
            log2_expected_retries,
            cfg.max_log2_retries,
        ),
    }
}

fn run_analysis(cfg: &Config) -> Result<SamplerOutput, String> {
    if cfg.product_block_groups == 0 {
        return Err("--product-block-groups must be positive".to_string());
    }
    if cfg.fork_challenges_per_round < 2 {
        return Err("--fork-challenges-per-round must be at least 2".to_string());
    }
    if cfg.fork_challenges_per_round >= usize::BITS as u64 {
        return Err(format!(
            "--fork-challenges-per-round must be less than {}",
            usize::BITS
        ));
    }
    if cfg.n % cfg.product_block_groups != 0 {
        return Err(format!(
            "n={} must be divisible by product_block_groups={}",
            cfg.n, cfg.product_block_groups
        ));
    }
    let local_free = cfg.n / cfg.product_block_groups;
    let mut retry_pass = Vec::new();
    let mut fast_leakage_rows = Vec::new();
    let mut gaussian_leakage_rows = Vec::new();
    let mut gaussian_metrics: HashMap<(u64, u64), GaussianLeakageMetrics> = HashMap::new();
    for &weight in &cfg.block_weights {
        if weight == 0 || weight > cfg.product_block_groups {
            continue;
        }
        let entropy = log2_binomial(cfg.product_block_groups, weight) + weight as f64;
        let min_repetitions = (cfg.target_bits / entropy).ceil() as u64;
        for repetitions in min_repetitions..=min_repetitions + cfg.max_extra_repetitions {
            let coverage = coverage_diagnostics(cfg, weight, repetitions);
            for &z_bound in &cfg.z_bounds {
                if let Some(row) =
                    build_row(cfg, local_free, weight, repetitions, z_bound, &coverage)
                {
                    if let Some(leakage_row) = fast_interior_leakage_row(cfg, &row) {
                        fast_leakage_rows.push(leakage_row);
                    }
                    for &sigma in &cfg.leakage_sigmas {
                        let metrics =
                            if let Some(&metrics) = gaussian_metrics.get(&(z_bound, sigma)) {
                                metrics
                            } else {
                                let Some(metrics) =
                                    gaussian_shifted_conditioned_metrics(z_bound, cfg.eta, sigma)
                                else {
                                    continue;
                                };
                                gaussian_metrics.insert((z_bound, sigma), metrics);
                                metrics
                            };
                        gaussian_leakage_rows.push(gaussian_leakage_row(cfg, &row, sigma, metrics));
                    }
                    if row.log2_expected_retries <= cfg.max_log2_retries {
                        retry_pass.push(row);
                    }
                }
            }
        }
    }
    let compact_fork_delta_16 = retry_pass
        .iter()
        .filter(|row| {
            row.compact_fork_delta_any_block_miss_bits
                .unwrap_or(f64::INFINITY)
                >= 16.0
        })
        .cloned()
        .collect();
    let compact_fork_delta_24 = retry_pass
        .iter()
        .filter(|row| {
            row.compact_fork_delta_any_block_miss_bits
                .unwrap_or(f64::INFINITY)
                >= 24.0
        })
        .cloned()
        .collect();
    let compact_fork_delta_32 = retry_pass
        .iter()
        .filter(|row| {
            row.compact_fork_delta_any_block_miss_bits
                .unwrap_or(f64::INFINITY)
                >= 32.0
        })
        .cloned()
        .collect();
    let full_coverage = retry_pass
        .iter()
        .filter(|row| row.full_block_coverage)
        .cloned()
        .collect();
    let gaussian_retry_pass = gaussian_leakage_rows
        .iter()
        .filter(|row| row.log2_expected_retries <= cfg.max_log2_retries)
        .cloned()
        .collect();
    Ok(SamplerOutput {
        q: cfg.q,
        n: cfg.n,
        eta: cfg.eta,
        product_block_groups: cfg.product_block_groups,
        local_free_coordinates_per_block: local_free,
        target_bits: cfg.target_bits,
        digest_bytes: cfg.digest_bytes,
        max_log2_retries: cfg.max_log2_retries,
        max_extra_repetitions: cfg.max_extra_repetitions,
        fork_challenges_per_round: cfg.fork_challenges_per_round,
        best_entropy_retry_rows: keep_best(retry_pass, 20),
        best_compact_fork_delta_16_rows: keep_best(compact_fork_delta_16, 12),
        best_compact_fork_delta_24_rows: keep_best(compact_fork_delta_24, 12),
        best_compact_fork_delta_32_rows: keep_best(compact_fork_delta_32, 12),
        best_full_coverage_rows: keep_best(full_coverage, 12),
        best_fast_interior_leakage_rows: keep_leakage_best_by_size(fast_leakage_rows, 12),
        best_gaussian_leakage_rows: keep_leakage_best_by_size(gaussian_retry_pass, 12),
        lowest_gaussian_leakage_rows: keep_leakage_best_by_kl(gaussian_leakage_rows, 12),
        conclusions: conclusions(cfg, local_free),
    })
}

fn leakage_certificate(cfg: &Config, out: &SamplerOutput) -> LeakageCertificate {
    let rational_interval_rows = out
        .best_gaussian_leakage_rows
        .iter()
        .filter_map(|row| rational_interval_row_from_leakage_row(row, cfg.eta))
        .collect();
    LeakageCertificate {
        schema: "descender.conditioned-gaussian-leakage-certificate.v2".to_string(),
        profile: "dyadic-conditioned-shifted-discrete-gaussian-product-response".to_string(),
        q: cfg.q,
        n: cfg.n,
        eta: cfg.eta,
        product_block_groups: cfg.product_block_groups,
        local_free_coordinates_per_block: out.local_free_coordinates_per_block,
        target_bits: cfg.target_bits,
        digest_bytes: cfg.digest_bytes,
        max_log2_retries: cfg.max_log2_retries,
        max_extra_repetitions: cfg.max_extra_repetitions,
        fork_challenges_per_round: cfg.fork_challenges_per_round,
        block_weights: cfg.block_weights.clone(),
        z_bounds: cfg.z_bounds.clone(),
        leakage_sigmas: cfg.leakage_sigmas.clone(),
        row_source: "best_gaussian_leakage_rows_under_retry_budget".to_string(),
        row_limit: out.best_gaussian_leakage_rows.len(),
        metric_definitions: vec![
            "For each shift v in [-eta,eta], P_v is the discrete Gaussian proposal y over [-z-eta,z+eta], shifted to r=y+v and conditioned on r in [-z,z].".to_string(),
            "per_coordinate_tv is max_{v0,v1} 1/2 * sum_r |P_v0(r)-P_v1(r)| over the finite response interval.".to_string(),
            "per_coordinate_kl_bits is max_{v0,v1} sum_r P_v0(r) log2(P_v0(r)/P_v1(r)).".to_string(),
            "signature_kl_bits is serialized_response_coordinates * per_coordinate_kl_bits.".to_string(),
            "signature_tv_union_bound is min(1, serialized_response_coordinates * per_coordinate_tv).".to_string(),
            "pinsker_signature_tv_bound is min(1, sqrt(ln(2) * signature_kl_bits / 2)).".to_string(),
            "log2_expected_retries uses the worst per-coordinate acceptance probability in the selected row.".to_string(),
            "rational_interval_rows certify the dyadic quantized sampler actually used by the proof object: weights are positive integers indexed by support offset, and TV is recomputed exactly as a rational number from those weights.".to_string(),
        ],
        selected_rows: out.best_gaussian_leakage_rows.clone(),
        rational_interval_rows,
    }
}

fn config_from_certificate(cert: &LeakageCertificate) -> Config {
    Config {
        q: cert.q,
        n: cert.n,
        eta: cert.eta,
        product_block_groups: cert.product_block_groups,
        target_bits: cert.target_bits,
        digest_bytes: cert.digest_bytes,
        max_log2_retries: cert.max_log2_retries,
        max_extra_repetitions: cert.max_extra_repetitions,
        fork_challenges_per_round: cert.fork_challenges_per_round,
        block_weights: cert.block_weights.clone(),
        z_bounds: cert.z_bounds.clone(),
        leakage_sigmas: cert.leakage_sigmas.clone(),
        output_json: PathBuf::new(),
        output_md: PathBuf::new(),
        output_certificate: PathBuf::new(),
        output_lean_certificate: None,
        verify_certificate: None,
    }
}

fn close_f64(name: &str, expected: f64, actual: f64) -> Result<(), String> {
    let tolerance = 1e-12_f64.max(1e-12 * expected.abs().max(actual.abs()));
    if (expected - actual).abs() <= tolerance {
        Ok(())
    } else {
        Err(format!("{name} drifted: expected {expected}, got {actual}"))
    }
}

fn close_option_f64(name: &str, expected: Option<f64>, actual: Option<f64>) -> Result<(), String> {
    match (expected, actual) {
        (Some(expected), Some(actual)) => close_f64(name, expected, actual),
        (None, None) => Ok(()),
        _ => Err(format!(
            "{name} option drifted: expected {expected:?}, got {actual:?}"
        )),
    }
}

fn verify_leakage_row(
    idx: usize,
    expected: &LeakageSamplerRow,
    actual: &LeakageSamplerRow,
) -> Result<(), String> {
    let prefix = |field: &str| format!("selected_rows[{idx}].{field}");
    if expected.sampler != actual.sampler {
        return Err(format!(
            "{} drifted: expected {}, got {}",
            prefix("sampler"),
            expected.sampler,
            actual.sampler
        ));
    }
    if expected.proof_status != actual.proof_status {
        return Err(format!("{} drifted", prefix("proof_status")));
    }
    if expected.block_weight != actual.block_weight {
        return Err(format!("{} drifted", prefix("block_weight")));
    }
    if expected.repetitions != actual.repetitions {
        return Err(format!("{} drifted", prefix("repetitions")));
    }
    if expected.serialized_response_coordinates != actual.serialized_response_coordinates {
        return Err(format!(
            "{} drifted",
            prefix("serialized_response_coordinates")
        ));
    }
    if expected.accepted_z_bound != actual.accepted_z_bound {
        return Err(format!("{} drifted", prefix("accepted_z_bound")));
    }
    if expected.sigma != actual.sigma {
        return Err(format!("{} drifted", prefix("sigma")));
    }
    if expected.estimated_signature_bytes != actual.estimated_signature_bytes {
        return Err(format!("{} drifted", prefix("estimated_signature_bytes")));
    }
    if expected.signature_bytes_with_digest != actual.signature_bytes_with_digest {
        return Err(format!("{} drifted", prefix("signature_bytes_with_digest")));
    }
    close_f64(
        &prefix("min_accept_probability_per_coordinate"),
        expected.min_accept_probability_per_coordinate,
        actual.min_accept_probability_per_coordinate,
    )?;
    close_f64(
        &prefix("max_accept_probability_per_coordinate"),
        expected.max_accept_probability_per_coordinate,
        actual.max_accept_probability_per_coordinate,
    )?;
    close_f64(
        &prefix("accept_probability_ratio"),
        expected.accept_probability_ratio,
        actual.accept_probability_ratio,
    )?;
    close_f64(
        &prefix("log2_expected_retries"),
        expected.log2_expected_retries,
        actual.log2_expected_retries,
    )?;
    close_f64(
        &prefix("per_coordinate_tv"),
        expected.per_coordinate_tv,
        actual.per_coordinate_tv,
    )?;
    close_f64(
        &prefix("signature_tv_union_bound"),
        expected.signature_tv_union_bound,
        actual.signature_tv_union_bound,
    )?;
    close_option_f64(
        &prefix("per_coordinate_kl_bits"),
        expected.per_coordinate_kl_bits,
        actual.per_coordinate_kl_bits,
    )?;
    close_option_f64(
        &prefix("signature_kl_bits"),
        expected.signature_kl_bits,
        actual.signature_kl_bits,
    )?;
    close_option_f64(
        &prefix("pinsker_signature_tv_bound"),
        expected.pinsker_signature_tv_bound,
        actual.pinsker_signature_tv_bound,
    )?;
    if expected.verdict != actual.verdict {
        return Err(format!(
            "{} drifted: expected {}, got {}",
            prefix("verdict"),
            expected.verdict,
            actual.verdict
        ));
    }
    Ok(())
}

fn verify_exact_interval(
    name: &str,
    interval: &RationalInterval,
    numerator: u128,
    denominator: u128,
) -> Result<(), String> {
    let lower = parse_u128_field(
        &format!("{name}.lower_numerator"),
        &interval.lower_numerator,
    )?;
    let upper = parse_u128_field(
        &format!("{name}.upper_numerator"),
        &interval.upper_numerator,
    )?;
    let actual_den = parse_u128_field(&format!("{name}.denominator"), &interval.denominator)?;
    if lower != numerator || upper != numerator || actual_den != denominator {
        return Err(format!(
            "{name} drifted: expected exact {numerator}/{denominator}, got [{lower},{upper}]/{actual_den}"
        ));
    }
    close_f64(
        &format!("{name}.upper_decimal"),
        numerator as f64 / denominator as f64,
        interval.upper_decimal,
    )
}

fn verify_upper_interval(
    name: &str,
    interval: &RationalInterval,
    numerator: u128,
    denominator: u128,
) -> Result<(), String> {
    let lower = parse_u128_field(
        &format!("{name}.lower_numerator"),
        &interval.lower_numerator,
    )?;
    let upper = parse_u128_field(
        &format!("{name}.upper_numerator"),
        &interval.upper_numerator,
    )?;
    let actual_den = parse_u128_field(&format!("{name}.denominator"), &interval.denominator)?;
    if lower != 0 || upper != numerator || actual_den != denominator {
        return Err(format!(
            "{name} drifted: expected upper [0,{numerator}]/{denominator}, got [{lower},{upper}]/{actual_den}"
        ));
    }
    close_f64(
        &format!("{name}.upper_decimal"),
        numerator as f64 / denominator as f64,
        interval.upper_decimal,
    )
}

fn verify_rational_interval_row(
    idx: usize,
    eta: u64,
    selected: &LeakageSamplerRow,
    proof: &RationalIntervalLeakageProofRow,
) -> Result<(), String> {
    let prefix = |field: &str| format!("rational_interval_rows[{idx}].{field}");
    if proof.sampler != "dyadic-conditioned-shifted-discrete-gaussian" {
        return Err(format!("{} has unsupported sampler", prefix("sampler")));
    }
    if proof.proof_status != "exact-rational-tv-bound-for-quantized-weight-table" {
        return Err(format!(
            "{} has unsupported proof status",
            prefix("proof_status")
        ));
    }
    if proof.block_weight != selected.block_weight
        || proof.repetitions != selected.repetitions
        || proof.serialized_response_coordinates != selected.serialized_response_coordinates
        || proof.accepted_z_bound != selected.accepted_z_bound
        || Some(proof.sigma) != selected.sigma
    {
        return Err(format!(
            "{idx}: rational proof row does not match selected row parameters"
        ));
    }
    if proof.eta != eta {
        return Err(format!("{} drifted", prefix("eta")));
    }
    if proof.weight_precision_bits != RATIONAL_GAUSSIAN_WEIGHT_PRECISION_BITS {
        return Err(format!("{} drifted", prefix("weight_precision_bits")));
    }
    let support = proof.accepted_z_bound + proof.eta;
    if proof.support_min != -(support as i64) || proof.support_max != support as i64 {
        return Err(format!("{} or support_max drifted", prefix("support_min")));
    }
    let expected_len = (2 * support + 1) as usize;
    if proof.weights_by_support_offset.len() != expected_len {
        return Err(format!(
            "{} has {} weights, expected {expected_len}",
            prefix("weights_by_support_offset"),
            proof.weights_by_support_offset.len()
        ));
    }
    let mut weights = Vec::with_capacity(expected_len);
    for (weight_idx, value) in proof.weights_by_support_offset.iter().enumerate() {
        let weight = parse_u128_field(
            &format!("{}.{}", prefix("weights_by_support_offset"), weight_idx),
            value,
        )?;
        if weight == 0 {
            return Err(format!(
                "{} contains zero weight at {weight_idx}",
                prefix("weights")
            ));
        }
        weights.push(weight);
    }
    let expected_weights = quantized_gaussian_weights(
        proof.accepted_z_bound,
        proof.eta,
        proof.sigma,
        proof.weight_precision_bits,
    );
    if weights != expected_weights {
        return Err(format!(
            "{} does not match the canonical dyadic Gaussian table",
            prefix("weights_by_support_offset")
        ));
    }
    for idx in 0..weights.len() {
        if weights[idx] != weights[weights.len() - 1 - idx] {
            return Err(format!(
                "{} is not symmetric at offset {idx}",
                prefix("weights")
            ));
        }
    }
    let (tv_num, tv_den, worst_pair) = rational_tv_from_weights(
        &weights,
        proof.support_min,
        proof.accepted_z_bound,
        proof.eta,
    );
    if proof.worst_shift_pair != worst_pair {
        return Err(format!(
            "{} drifted: expected {:?}, got {:?}",
            prefix("worst_shift_pair"),
            worst_pair,
            proof.worst_shift_pair
        ));
    }
    verify_exact_interval(
        &prefix("per_coordinate_tv_exact"),
        &proof.per_coordinate_tv_exact,
        tv_num,
        tv_den,
    )?;
    let (sig_num, sig_den) =
        signature_tv_bound_from_coordinate(proof.serialized_response_coordinates, tv_num, tv_den);
    verify_upper_interval(
        &prefix("signature_tv_bound"),
        &proof.signature_tv_bound,
        sig_num,
        sig_den,
    )?;
    Ok(())
}

fn verify_leakage_certificate_file(path: &Path) -> Result<LeakageCertificate, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let cert: LeakageCertificate = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
    if cert.schema != "descender.conditioned-gaussian-leakage-certificate.v2" {
        return Err(format!(
            "unsupported leakage certificate schema {}",
            cert.schema
        ));
    }
    if cert.profile != "dyadic-conditioned-shifted-discrete-gaussian-product-response" {
        return Err(format!(
            "unsupported leakage certificate profile {}",
            cert.profile
        ));
    }
    let cfg = config_from_certificate(&cert);
    let out = run_analysis(&cfg)?;
    let expected = leakage_certificate(&cfg, &out);

    if cert.local_free_coordinates_per_block != expected.local_free_coordinates_per_block {
        return Err("local_free_coordinates_per_block drifted".to_string());
    }
    if cert.row_source != expected.row_source {
        return Err("row_source drifted".to_string());
    }
    if cert.row_limit != expected.row_limit {
        return Err(format!(
            "row_limit drifted: expected {}, got {}",
            expected.row_limit, cert.row_limit
        ));
    }
    if cert.metric_definitions != expected.metric_definitions {
        return Err("metric_definitions drifted".to_string());
    }
    if cert.selected_rows.len() != expected.selected_rows.len() {
        return Err(format!(
            "selected row count drifted: expected {}, got {}",
            expected.selected_rows.len(),
            cert.selected_rows.len()
        ));
    }
    for (idx, (actual, expected)) in cert
        .selected_rows
        .iter()
        .zip(expected.selected_rows.iter())
        .enumerate()
    {
        verify_leakage_row(idx, expected, actual)?;
    }
    if cert.rational_interval_rows.len() != cert.selected_rows.len() {
        return Err(format!(
            "rational interval row count drifted: expected {}, got {}",
            cert.selected_rows.len(),
            cert.rational_interval_rows.len()
        ));
    }
    for (idx, (selected, proof)) in cert
        .selected_rows
        .iter()
        .zip(cert.rational_interval_rows.iter())
        .enumerate()
    {
        verify_rational_interval_row(idx, cert.eta, selected, proof)?;
    }
    Ok(cert)
}

fn conclusions(cfg: &Config, local_free: u64) -> Vec<String> {
    vec![
        format!(
        "The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1)."
        ),
        "The new leakage-bounded diagnostics treat fast shifted responses and conditioned shifted discrete-Gaussian responses as valid-signature candidates, then charge their actual response-distribution dependence on the secret by exact finite TV/KL accounting.".to_string(),
        "The fast-interior sampler has zero rejection, but shifted interval supports give infinite KL for worst-case secret shifts and a product TV union bound that saturates quickly.  It is an implementation tool, not a security proof candidate by itself.".to_string(),
        format!(
            "The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes {local_free} free response coordinates."
        ),
        "Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.".to_string(),
        "Active-challenge coverage and fork-delta coverage are separate diagnostics.  The extraction proof needs a usable challenge delta on every block, not merely a challenge that touches the block once.".to_string(),
        format!(
            "Compact fork-delta coverage assumes {} challenge draws for the same commitment per repeated round.  It only counts opposite signed active openings, because a compact transcript omits inactive response coordinates.",
            cfg.fork_challenges_per_round
        ),
        format!(
            "Under the current moment proof, direct residual forgery bits are compact fork-delta coverage bits divided by {}.  A 128-bit residual target would require about {} compact coverage bits before the root is taken.",
            cfg.fork_challenges_per_round,
            cfg.target_bits * cfg.fork_challenges_per_round as f64
        ),
        "The full-response fork-delta column is an idealized diagnostic for a transcript that exposes every response coordinate; it should not be used as the compact-signature security claim.".to_string(),
        "All coverage bits are exact any-block-missed probabilities computed by inclusion-exclusion over the product blocks.  They are sharper than a union-bound-only diagnostic while still charging the real failure event.".to_string(),
        "The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their fork-delta diagnostic is weak under compact active-only openings.".to_string(),
        format!(
            "Full-block challenges respond on all {} coordinates; they remove the inactive-coordinate omission issue, but the forking proof still needs enough signed challenge draws to obtain opposite signs blockwise.",
            cfg.n
        ),
        "The most promising small-signature route is now an intermediate compact fork-delta row plus an explicit multi-fork Fiat-Shamir theorem, not another scalar-challenge size screen.".to_string(),
        "If bounded-leakage rows are to replace exact-box rows, the security theorem must explicitly consume their product KL/TV budget and any secret-dependent retry timing must be hidden or separately bounded.".to_string(),
    ]
}

fn markdown(out: &SamplerOutput) -> String {
    let mut lines = vec![
        "# Product-Block Signature Sampler Analysis".to_string(),
        String::new(),
        format!("- q: `{}`", out.q),
        format!("- n: `{}`", out.n),
        format!("- eta: `{}`", out.eta),
        format!("- product blocks: `{}`", out.product_block_groups),
        format!(
            "- local free coordinates per block: `{}`",
            out.local_free_coordinates_per_block
        ),
        format!("- target challenge bits: `{:.1}`", out.target_bits),
        format!("- digest bytes: `{}`", out.digest_bytes),
        format!("- max log2 expected retries: `{:.1}`", out.max_log2_retries),
        format!(
            "- max extra repetitions searched: `{}`",
            out.max_extra_repetitions
        ),
        format!(
            "- fork challenge draws per repeated commitment: `{}`",
            out.fork_challenges_per_round
        ),
        String::new(),
        "## Exact Sampler".to_string(),
        String::new(),
        "For each active response coordinate with shift `|v| <= eta`, sample `y` uniformly from `[-z-eta,z+eta]`, output `r = y + v`, and accept iff `r in [-z,z]`.  For every allowed shift, the accepted response is exactly uniform on `[-z,z]`.".to_string(),
        String::new(),
        "The per-coordinate acceptance probability is `(2z+1)/(2(z+eta)+1)`.  The total retry exponent is charged over active response coordinates only.".to_string(),
        String::new(),
        "## Best Entropy + Retry Rows".to_string(),
        String::new(),
    ];
    append_table(&mut lines, &out.best_entropy_retry_rows);
    lines.extend([
        String::new(),
        "## Best Rows With >=16 Bits Compact Fork-Delta Coverage".to_string(),
        String::new(),
    ]);
    append_table(&mut lines, &out.best_compact_fork_delta_16_rows);
    lines.extend([
        String::new(),
        "## Best Rows With >=24 Bits Compact Fork-Delta Coverage".to_string(),
        String::new(),
    ]);
    append_table(&mut lines, &out.best_compact_fork_delta_24_rows);
    lines.extend([
        String::new(),
        "## Best Rows With >=32 Bits Compact Fork-Delta Coverage".to_string(),
        String::new(),
    ]);
    append_table(&mut lines, &out.best_compact_fork_delta_32_rows);
    lines.extend([
        String::new(),
        "## Best Full-Block Coverage Rows".to_string(),
        String::new(),
    ]);
    append_table(&mut lines, &out.best_full_coverage_rows);
    lines.extend([
        String::new(),
        "## Leakage-Bounded Candidate Rows".to_string(),
        String::new(),
        "These rows are valid-signature sampler candidates, not secret-independent proofs.  `TV union` is the simple product union bound over serialized response coordinates.  `KL bits` is additive across serialized coordinates when finite.".to_string(),
        String::new(),
        "### Fast Interior Rows".to_string(),
        String::new(),
    ]);
    append_leakage_table(&mut lines, &out.best_fast_interior_leakage_rows);
    lines.extend([
        String::new(),
        "### Discrete-Gaussian Rows By Size Under Retry Budget".to_string(),
        String::new(),
    ]);
    append_leakage_table(&mut lines, &out.best_gaussian_leakage_rows);
    lines.extend([
        String::new(),
        "### Lowest Discrete-Gaussian Leakage Rows".to_string(),
        String::new(),
    ]);
    append_leakage_table(&mut lines, &out.lowest_gaussian_leakage_rows);
    lines.extend([String::new(), "## Conclusions".to_string(), String::new()]);
    for conclusion in &out.conclusions {
        lines.push(format!("- {conclusion}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn fmt_optional_bits(bits: Option<f64>) -> String {
    match bits {
        Some(bits) => format!("{bits:.2}"),
        None => "full".to_string(),
    }
}

fn append_table(lines: &mut Vec<String>, rows: &[SamplerRow]) {
    lines.push("| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | direct proof bits | one-fork compact bits | full-response delta bits | verdict |".to_string());
    lines.push(
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|"
            .to_string(),
    );
    for row in rows {
        lines.push(format!(
            "| {} | {:.2} | {} | {} | {} | {} | {} | {} | {} | {} | {:.2} | {} | {} | {} | {} | {} | {} |",
            row.block_weight,
            row.challenge_entropy_bits,
            row.repetitions,
            row.active_response_coordinates,
            row.accepted_z_bound,
            row.z_bits_per_coordinate,
            row.response_bytes_per_repetition,
            row.challenge_encoding_bytes,
            row.estimated_signature_bytes,
            row.signature_bytes_with_digest,
            row.log2_expected_retries,
            fmt_optional_bits(row.active_any_block_miss_bits),
            fmt_optional_bits(row.compact_fork_delta_any_block_miss_bits),
            fmt_optional_bits(row.compact_fork_delta_direct_forgery_bits),
            fmt_optional_bits(row.compact_one_fork_delta_any_block_miss_bits),
            fmt_optional_bits(row.full_response_fork_delta_any_block_miss_bits),
            row.verdict
        ));
    }
}

fn fmt_optional_float(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:.4}"),
        None => "inf/unsupported".to_string(),
    }
}

fn append_leakage_table(lines: &mut Vec<String>, rows: &[LeakageSamplerRow]) {
    lines.push("| sampler | weight | reps | coords | z bound | sigma | sig bytes | sig bytes w/32B hash | min accept | accept ratio | log2 retries | coord TV | TV union | coord KL bits | sig KL bits | Pinsker TV | verdict |".to_string());
    lines.push(
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|"
            .to_string(),
    );
    for row in rows {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {:.6} | {:.4} | {:.2} | {:.6} | {:.4} | {} | {} | {} | {} |",
            row.sampler,
            row.block_weight,
            row.repetitions,
            row.serialized_response_coordinates,
            row.accepted_z_bound,
            row.sigma.map_or("-".to_string(), |sigma| sigma.to_string()),
            row.estimated_signature_bytes,
            row.signature_bytes_with_digest,
            row.min_accept_probability_per_coordinate,
            row.accept_probability_ratio,
            row.log2_expected_retries,
            row.per_coordinate_tv,
            row.signature_tv_union_bound,
            fmt_optional_float(row.per_coordinate_kl_bits),
            fmt_optional_float(row.signature_kl_bits),
            fmt_optional_float(row.pinsker_signature_tv_bound),
            row.verdict
        ));
    }
}

fn sha256_hex(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn lean_string_literal(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization should not fail")
}

fn lean_certificate_namespace(cert: &LeakageCertificate) -> String {
    format!("ConditionedGaussianLeakageN{}", cert.n)
}

fn write_lean_interval_def(
    out: &mut String,
    name: &str,
    interval: &RationalInterval,
) -> Result<(), String> {
    writeln!(out, "def {name} : RationalIntervalProof := {{").map_err(|err| err.to_string())?;
    writeln!(out, "  lowerNumerator := {},", interval.lower_numerator)
        .map_err(|err| err.to_string())?;
    writeln!(out, "  upperNumerator := {},", interval.upper_numerator)
        .map_err(|err| err.to_string())?;
    writeln!(out, "  denominator := {},", interval.denominator).map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  upperDecimal := {}",
        lean_string_literal(&interval.upper_decimal.to_string())
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "}}\n").map_err(|err| err.to_string())
}

fn write_lean_row_defs(
    out: &mut String,
    idx: usize,
    row: &RationalIntervalLeakageProofRow,
) -> Result<(), String> {
    let per_coord = format!("row{idx}PerCoordinateTvExact");
    let sig = format!("row{idx}SignatureTvBound");
    write_lean_interval_def(out, &per_coord, &row.per_coordinate_tv_exact)?;
    write_lean_interval_def(out, &sig, &row.signature_tv_bound)?;
    writeln!(out, "def row{idx} : DyadicGaussianLeakageProofRow := {{")
        .map_err(|err| err.to_string())?;
    writeln!(out, "  sampler := {},", lean_string_literal(&row.sampler))
        .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  proofStatus := {},",
        lean_string_literal(&row.proof_status)
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  blockWeight := {},", row.block_weight).map_err(|err| err.to_string())?;
    writeln!(out, "  repetitions := {},", row.repetitions).map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  serializedResponseCoordinates := {},",
        row.serialized_response_coordinates
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  acceptedZBound := {},", row.accepted_z_bound)
        .map_err(|err| err.to_string())?;
    writeln!(out, "  eta := {},", row.eta).map_err(|err| err.to_string())?;
    writeln!(out, "  sigma := {},", row.sigma).map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  weightPrecisionBits := {},",
        row.weight_precision_bits
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  supportMin := {},", row.support_min).map_err(|err| err.to_string())?;
    writeln!(out, "  supportMax := {},", row.support_max).map_err(|err| err.to_string())?;
    writeln!(out, "  worstShiftLeft := {},", row.worst_shift_pair[0])
        .map_err(|err| err.to_string())?;
    writeln!(out, "  worstShiftRight := {},", row.worst_shift_pair[1])
        .map_err(|err| err.to_string())?;
    writeln!(out, "  perCoordinateTvExact := {per_coord},").map_err(|err| err.to_string())?;
    writeln!(out, "  signatureTvBound := {sig}").map_err(|err| err.to_string())?;
    writeln!(out, "}}\n").map_err(|err| err.to_string())?;

    writeln!(
        out,
        "def row{idx}SignatureLeakage : ℝ := row{idx}.signatureLeakage"
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "theorem row{idx}SignatureLeakage_eq :\n    row{idx}SignatureLeakage = ({} : ℝ) / {} := by",
        row.signature_tv_bound.upper_numerator, row.signature_tv_bound.denominator
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  norm_num [row{idx}SignatureLeakage, DyadicGaussianLeakageProofRow.signatureLeakage,\n    RationalIntervalProof.upperReal, row{idx}, {sig}]\n"
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "theorem row{idx}SignatureLeakage_nonneg : 0 ≤ row{idx}SignatureLeakage := by"
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  exact row{idx}.signatureLeakage_nonneg\n").map_err(|err| err.to_string())?;
    writeln!(
        out,
        "theorem row{idx}SignatureLeakage_le_one : row{idx}SignatureLeakage ≤ 1 := by"
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  exact row{idx}.signatureLeakage_le_one (by norm_num [row{idx}, {sig}])\n"
    )
    .map_err(|err| err.to_string())
}

fn lean_certificate_module(
    cert: &LeakageCertificate,
    source_path: &Path,
    source_text: &str,
) -> Result<String, String> {
    let mut out = String::new();
    let namespace = lean_certificate_namespace(cert);
    let source_path = source_path.display().to_string();
    let source_hash = sha256_hex(source_text);
    writeln!(out, "import Descender.SignatureLeakageCertificate\n")
        .map_err(|err| err.to_string())?;
    writeln!(out, "namespace Descender").map_err(|err| err.to_string())?;
    writeln!(out, "namespace SignatureDistribution").map_err(|err| err.to_string())?;
    writeln!(out, "namespace Generated").map_err(|err| err.to_string())?;
    writeln!(out, "namespace {namespace}\n").map_err(|err| err.to_string())?;
    writeln!(out, "noncomputable section\n").map_err(|err| err.to_string())?;
    writeln!(
        out,
        "/-!\nGenerated by `signature_sampler_analysis --output-lean-certificate`.\n\nSource certificate: `{}`\nSource SHA-256: `{}`\n\nThe Rust verifier checks the source JSON before emitting this module.\n-/\n",
        source_path, source_hash
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "def sourcePath : String := {}",
        lean_string_literal(&source_path)
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "def sourceSha256 : String := {}",
        lean_string_literal(&source_hash)
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "def schema : String := {}",
        lean_string_literal(&cert.schema)
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "def profile : String := {}\n",
        lean_string_literal(&cert.profile)
    )
    .map_err(|err| err.to_string())?;

    for (idx, row) in cert.rational_interval_rows.iter().enumerate() {
        write_lean_row_defs(&mut out, idx, row)?;
    }

    writeln!(out, "def rows : List DyadicGaussianLeakageProofRow := [")
        .map_err(|err| err.to_string())?;
    for idx in 0..cert.rational_interval_rows.len() {
        if idx + 1 == cert.rational_interval_rows.len() {
            writeln!(out, "  row{idx}").map_err(|err| err.to_string())?;
        } else {
            writeln!(out, "  row{idx},").map_err(|err| err.to_string())?;
        }
    }
    writeln!(out, "]\n").map_err(|err| err.to_string())?;
    writeln!(
        out,
        "theorem rows_length : rows.length = {} := by rfl\n",
        cert.rational_interval_rows.len()
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "def certificate : DyadicGaussianLeakageCertificate := {{"
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  schema := schema,").map_err(|err| err.to_string())?;
    writeln!(out, "  profile := profile,").map_err(|err| err.to_string())?;
    writeln!(out, "  sourcePath := sourcePath,").map_err(|err| err.to_string())?;
    writeln!(out, "  sourceSha256 := sourceSha256,").map_err(|err| err.to_string())?;
    writeln!(out, "  q := {},", cert.q).map_err(|err| err.to_string())?;
    writeln!(out, "  n := {},", cert.n).map_err(|err| err.to_string())?;
    writeln!(out, "  eta := {},", cert.eta).map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  productBlockGroups := {},",
        cert.product_block_groups
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  localFreeCoordinatesPerBlock := {},",
        cert.local_free_coordinates_per_block
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  targetBits := {},",
        lean_string_literal(&cert.target_bits.to_string())
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  digestBytes := {},", cert.digest_bytes).map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  maxLog2Retries := {},",
        lean_string_literal(&cert.max_log2_retries.to_string())
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  maxExtraRepetitions := {},",
        cert.max_extra_repetitions
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  forkChallengesPerRound := {},",
        cert.fork_challenges_per_round
    )
    .map_err(|err| err.to_string())?;
    writeln!(
        out,
        "  rowSource := {},",
        lean_string_literal(&cert.row_source)
    )
    .map_err(|err| err.to_string())?;
    writeln!(out, "  rowLimit := {},", cert.row_limit).map_err(|err| err.to_string())?;
    writeln!(out, "  rows := rows").map_err(|err| err.to_string())?;
    writeln!(out, "}}\n").map_err(|err| err.to_string())?;
    writeln!(
        out,
        "theorem certificate_rows_length : certificate.rows.length = {} := by rfl\n",
        cert.rational_interval_rows.len()
    )
    .map_err(|err| err.to_string())?;
    if !cert.rational_interval_rows.is_empty() {
        writeln!(
            out,
            "def leadingSignatureLeakage : ℝ := row0SignatureLeakage"
        )
        .map_err(|err| err.to_string())?;
        writeln!(
            out,
            "theorem leadingSignatureLeakage_eq : leadingSignatureLeakage = row0SignatureLeakage := by rfl\n"
        )
        .map_err(|err| err.to_string())?;
    }
    writeln!(out, "end\n").map_err(|err| err.to_string())?;
    writeln!(out, "end {namespace}").map_err(|err| err.to_string())?;
    writeln!(out, "end Generated").map_err(|err| err.to_string())?;
    writeln!(out, "end SignatureDistribution").map_err(|err| err.to_string())?;
    writeln!(out, "end Descender").map_err(|err| err.to_string())?;
    Ok(out)
}

fn write_lean_certificate(
    path: &Path,
    cert: &LeakageCertificate,
    source_path: &Path,
    source_text: &str,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let module = lean_certificate_module(cert, source_path, source_text)?;
    fs::write(path, module).map_err(|e| format!("write {}: {e}", path.display()))
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    if let Some(path) = cfg.verify_certificate.as_ref() {
        let source_text = fs::read_to_string(path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let cert = verify_leakage_certificate_file(path)?;
        if let Some(lean_path) = cfg.output_lean_certificate.as_ref() {
            write_lean_certificate(lean_path, &cert, path, &source_text)?;
            println!("wrote {}", lean_path.display());
        }
        println!(
            "verified {} conditioned Gaussian leakage rows in {}",
            cert.selected_rows.len(),
            path.display()
        );
        return Ok(());
    }
    let out = run_analysis(&cfg)?;
    if let Some(parent) = cfg.output_json.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    if let Some(parent) = cfg.output_md.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    if let Some(parent) = cfg.output_certificate.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&out).map_err(|e| format!("serialize json: {e}"))?;
    let cert_obj = leakage_certificate(&cfg, &out);
    let cert = serde_json::to_string_pretty(&cert_obj)
        .map_err(|e| format!("serialize leakage certificate: {e}"))?;
    let cert_text = format!("{cert}\n");
    fs::write(&cfg.output_json, format!("{json}\n"))
        .map_err(|e| format!("write {}: {e}", cfg.output_json.display()))?;
    fs::write(&cfg.output_md, markdown(&out))
        .map_err(|e| format!("write {}: {e}", cfg.output_md.display()))?;
    fs::write(&cfg.output_certificate, &cert_text)
        .map_err(|e| format!("write {}: {e}", cfg.output_certificate.display()))?;
    if let Some(lean_path) = cfg.output_lean_certificate.as_ref() {
        write_lean_certificate(lean_path, &cert_obj, &cfg.output_certificate, &cert_text)?;
        println!("wrote {}", lean_path.display());
    }
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
    println!("wrote {}", cfg.output_certificate.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_acceptance_matches_exact_box_count() {
        let p = coordinate_accept_probability(128, 12);
        let expected = 257.0 / 281.0;
        assert!((p - expected).abs() < 1e-12);
    }

    #[test]
    fn fast_interior_tv_matches_shifted_interval_count() {
        let tv = fast_interior_coordinate_tv(224, 12).unwrap();
        let expected = 12.0 / 425.0;
        assert!((tv - expected).abs() < 1e-12);
    }

    #[test]
    fn gaussian_shifted_metrics_are_finite_and_improve_with_sigma() {
        let small = gaussian_shifted_conditioned_metrics(224, 12, 64).unwrap();
        let large = gaussian_shifted_conditioned_metrics(224, 12, 320).unwrap();
        assert!(small.3 > large.3);
        assert!(small.4 > large.4);
        assert!(small.0 > 0.0);
        assert!(large.0 > 0.0);
    }

    #[test]
    fn leakage_rows_capture_fast_path_as_leaky() {
        let cfg = Config {
            n: 32,
            output_json: PathBuf::from("/tmp/unused.json"),
            output_md: PathBuf::from("/tmp/unused.md"),
            output_certificate: PathBuf::from("/tmp/unused-cert.json"),
            ..Config::default()
        };
        let out = run_analysis(&cfg).unwrap();
        assert!(out.best_fast_interior_leakage_rows.iter().any(|row| {
            row.verdict == "support-shift-leakage-too-large-for-security-claim"
                && row.signature_tv_union_bound >= 1.0
        }));
    }

    #[test]
    fn leakage_certificate_round_trips_through_verifier() {
        let path = PathBuf::from("/tmp/descender-leakage-certificate-test.json");
        let cfg = Config {
            n: 32,
            block_weights: vec![1, 2],
            z_bounds: vec![48, 64],
            leakage_sigmas: vec![640, 896],
            output_json: PathBuf::from("/tmp/unused.json"),
            output_md: PathBuf::from("/tmp/unused.md"),
            output_certificate: path.clone(),
            ..Config::default()
        };
        let out = run_analysis(&cfg).unwrap();
        let cert = serde_json::to_string_pretty(&leakage_certificate(&cfg, &out)).unwrap();
        fs::write(&path, format!("{cert}\n")).unwrap();
        let verified = verify_leakage_certificate_file(&path).unwrap();
        assert_eq!(
            verified.selected_rows.len(),
            out.best_gaussian_leakage_rows.len()
        );
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn leakage_certificate_rejects_noncanonical_weight_table() {
        let path = PathBuf::from("/tmp/descender-leakage-certificate-tampered-test.json");
        let cfg = Config {
            n: 32,
            block_weights: vec![1],
            z_bounds: vec![48],
            leakage_sigmas: vec![640],
            output_json: PathBuf::from("/tmp/unused.json"),
            output_md: PathBuf::from("/tmp/unused.md"),
            output_certificate: path.clone(),
            ..Config::default()
        };
        let out = run_analysis(&cfg).unwrap();
        let mut cert = leakage_certificate(&cfg, &out);
        let weights = &mut cert.rational_interval_rows[0].weights_by_support_offset;
        let mid = weights.len() / 2;
        let tampered = weights[mid].parse::<u128>().unwrap() + 1;
        weights[mid] = tampered.to_string();
        let cert_json = serde_json::to_string_pretty(&cert).unwrap();
        fs::write(&path, format!("{cert_json}\n")).unwrap();
        let err = verify_leakage_certificate_file(&path).unwrap_err();
        assert!(err.contains("canonical dyadic Gaussian table"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn lean_certificate_module_contains_verified_rational_constants() {
        let cfg = Config {
            n: 32,
            block_weights: vec![1],
            z_bounds: vec![48],
            leakage_sigmas: vec![896],
            output_json: PathBuf::from("/tmp/unused.json"),
            output_md: PathBuf::from("/tmp/unused.md"),
            output_certificate: PathBuf::from("/tmp/unused-cert.json"),
            ..Config::default()
        };
        let out = run_analysis(&cfg).unwrap();
        let cert = leakage_certificate(&cfg, &out);
        let source_text = serde_json::to_string_pretty(&cert).unwrap();
        let module =
            lean_certificate_module(&cert, &PathBuf::from("certificate.json"), &source_text)
                .unwrap();
        assert!(module.contains("namespace ConditionedGaussianLeakageN32"));
        assert!(module.contains(&sha256_hex(&source_text)));
        assert!(module.contains("def row0SignatureLeakage : ℝ"));
        assert!(module.contains("row0SignatureLeakage_le_one"));
        assert!(
            module.contains(
                cert.rational_interval_rows[0]
                    .signature_tv_bound
                    .upper_numerator
                    .as_str()
            )
        );
        assert!(
            module.contains(
                cert.rational_interval_rows[0]
                    .signature_tv_bound
                    .denominator
                    .as_str()
            )
        );
    }

    #[test]
    fn default_profile_has_subkilobyte_full_coverage_row() {
        let out = run_analysis(&Config::default()).unwrap();
        assert!(
            out.best_full_coverage_rows
                .iter()
                .any(|row| row.estimated_signature_bytes < 600)
        );
    }

    #[test]
    fn n32_profile_has_smaller_full_coverage_row() {
        let cfg = Config {
            n: 32,
            output_json: PathBuf::from("/tmp/unused.json"),
            output_md: PathBuf::from("/tmp/unused.md"),
            ..Config::default()
        };
        let out = run_analysis(&cfg).unwrap();
        assert!(
            out.best_full_coverage_rows
                .iter()
                .any(|row| row.estimated_signature_bytes < 400)
        );
    }

    #[test]
    fn inclusion_exclusion_charges_any_missed_block() {
        let one_block = single_block_miss_bits(16, 14, 7).unwrap();
        let any_block = probability_bits(any_block_miss_probability(16, 14, 7)).unwrap();
        assert!(one_block > 20.9);
        assert!(any_block > 16.9);
        assert!(any_block < one_block);
    }

    #[test]
    fn full_response_and_compact_fork_delta_are_different() {
        let full_response = probability_bits(full_response_fork_delta_any_block_miss_probability(
            16, 15, 7, 5,
        ))
        .unwrap();
        let compact =
            probability_bits(compact_fork_delta_any_block_miss_probability(16, 15, 7, 5)).unwrap();
        assert!(full_response > 24.0);
        assert!(compact < 24.0);
    }

    #[test]
    fn compact_fork_delta_coverage_needs_multi_fork() {
        let one_fork =
            probability_bits(compact_fork_delta_any_block_miss_probability(16, 15, 7, 2)).unwrap();
        let six_draws =
            probability_bits(compact_fork_delta_any_block_miss_probability(16, 15, 7, 6)).unwrap();
        assert!(one_fork < 4.0);
        assert!(six_draws > 24.0);
    }

    #[test]
    fn direct_forgery_bits_take_the_current_moment_root() {
        let coverage_bits =
            probability_bits(compact_fork_delta_any_block_miss_probability(16, 15, 7, 6)).unwrap();
        let direct_bits = direct_forgery_bits(Some(coverage_bits), 6).unwrap();
        assert!(direct_bits > 4.5);
        assert!(direct_bits < 4.6);
    }
}
