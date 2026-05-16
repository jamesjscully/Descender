use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

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
    output_json: PathBuf,
    output_md: PathBuf,
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
            output_json: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/sampler_analysis_n48.json",
            ),
            output_md: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/sampler_analysis_n48.md",
            ),
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
    compact_one_fork_delta_any_block_miss_bits: Option<f64>,
    full_block_coverage: bool,
    verdict: String,
}

#[derive(Clone, Debug)]
struct CoverageDiagnostics {
    active_single_block_miss_bits: Option<f64>,
    active_any_block_miss_bits: Option<f64>,
    full_response_fork_delta_any_block_miss_bits: Option<f64>,
    compact_fork_delta_any_block_miss_bits: Option<f64>,
    compact_one_fork_delta_any_block_miss_bits: Option<f64>,
}

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
            "--output-json" => cfg.output_json = PathBuf::from(value("--output-json")?),
            "--output-md" => cfg.output_md = PathBuf::from(value("--output-md")?),
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
           --output-json PATH\n\
           --output-md PATH"
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
        conclusions: conclusions(cfg, local_free),
    })
}

fn conclusions(cfg: &Config, local_free: u64) -> Vec<String> {
    vec![
        format!(
            "The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1)."
        ),
        format!(
            "The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes {local_free} free response coordinates."
        ),
        "Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.".to_string(),
        "Active-challenge coverage and fork-delta coverage are separate diagnostics.  The extraction proof needs a usable challenge delta on every block, not merely a challenge that touches the block once.".to_string(),
        format!(
            "Compact fork-delta coverage assumes {} challenge draws for the same commitment per repeated round.  It only counts opposite signed active openings, because a compact transcript omits inactive response coordinates.",
            cfg.fork_challenges_per_round
        ),
        "The full-response fork-delta column is an idealized diagnostic for a transcript that exposes every response coordinate; it should not be used as the compact-signature security claim.".to_string(),
        "All coverage bits are exact any-block-missed probabilities computed by inclusion-exclusion over the product blocks.  They are sharper than a union-bound-only diagnostic while still charging the real failure event.".to_string(),
        "The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their fork-delta diagnostic is weak under compact active-only openings.".to_string(),
        format!(
            "Full-block challenges respond on all {} coordinates; they remove the inactive-coordinate omission issue, but the forking proof still needs enough signed challenge draws to obtain opposite signs blockwise.",
            cfg.n
        ),
        "The most promising small-signature route is now an intermediate compact fork-delta row plus an explicit multi-fork Fiat-Shamir theorem, not another scalar-challenge size screen.".to_string(),
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
    lines.push("| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |".to_string());
    lines.push(
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|"
            .to_string(),
    );
    for row in rows {
        lines.push(format!(
            "| {} | {:.2} | {} | {} | {} | {} | {} | {} | {} | {} | {:.2} | {} | {} | {} | {} | {} |",
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
            fmt_optional_bits(row.compact_one_fork_delta_any_block_miss_bits),
            fmt_optional_bits(row.full_response_fork_delta_any_block_miss_bits),
            row.verdict
        ));
    }
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    let out = run_analysis(&cfg)?;
    if let Some(parent) = cfg.output_json.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    if let Some(parent) = cfg.output_md.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&out).map_err(|e| format!("serialize json: {e}"))?;
    fs::write(&cfg.output_json, format!("{json}\n"))
        .map_err(|e| format!("write {}: {e}", cfg.output_json.display()))?;
    fs::write(&cfg.output_md, markdown(&out))
        .map_err(|e| format!("write {}: {e}", cfg.output_md.display()))?;
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
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
}
