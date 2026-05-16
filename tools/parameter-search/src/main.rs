use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

#[derive(Clone, Debug, Serialize)]
struct Candidate {
    proof_strategy: String,
    primal_distance_model: String,
    q: u64,
    n: u64,
    m0: u64,
    m1: u64,
    fiber_dim: u64,
    eta: u64,
    /// Bits used to bit-pack public samples and ciphertext coordinates.
    /// `0` means full field-element width.
    sample_bits: u64,
    /// Number of public/ciphertext sample coordinates in the viewed instance.
    /// `0` means the full coordinate set of size `m0 + m1`.
    view_coords: u64,
    /// Per-instance canonical public seed bytes. `0` uses the model default.
    pk_seed_bytes: u64,
    /// Proof strategy for the q-ary centered-Lee amplitude certificate.
    qary_amplitude_strategy: String,
    /// Repetition/row budget for certified solved-map Lee constructions.
    lee_repetition: u64,
    /// Rows per two-free-coordinate block for product-block Lee certificates.
    lee_rows_per_group: u64,
}

impl Candidate {
    fn m(&self) -> u64 {
        self.m0 + self.m1
    }

    fn view_len(&self) -> u64 {
        if self.view_coords == 0 {
            self.m()
        } else {
            self.view_coords
        }
    }

    fn key(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.proof_strategy,
            self.primal_distance_model,
            self.qary_amplitude_strategy,
            self.q,
            self.n,
            self.m0,
            self.m1,
            self.fiber_dim,
            self.eta,
            self.sample_bits,
            self.view_coords,
            self.pk_seed_bytes,
            self.lee_repetition,
            self.lee_rows_per_group,
        )
    }
}

#[derive(Clone, Debug, Serialize)]
struct Metrics {
    logq_bits: f64,
    elem_bytes: u64,
    dense_public_key_bytes: u64,
    setup_seed_public_bytes: u64,
    cauchy_public_bytes: u64,
    coefficient_public_bytes: u64,
    incidence_public_bytes: u64,
    diagonal_public_bytes: u64,
    sample_encoding_bits: u64,
    sample_compression_error: u64,
    effective_eta: u64,
    public_sample_bytes: u64,
    compact_public_key_bytes: u64,
    ciphertext_bytes: u64,
    secret_key_bytes: u64,
    guard_distance: i64,
    correctness_distance: i64,
    guard_slack: i64,
    correctness_slack: i64,
    combined_rank_deficit: i64,
    combined_distance_gap: i64,
    combined_certification_margin: i64,
    combined_certification_pressure: f64,
    cauchy_field_slack: i64,
    noise_headroom: i64,
    qary_amplitude_l1_required: u64,
    qary_amplitude_l1_certified: u64,
    qary_amplitude_at_noise_pass: bool,
    qary_amplitude_minkowski_l1_upper: f64,
    qary_amplitude_minkowski_margin: f64,
    qary_amplitude_geometry_possible: bool,
    qary_amplitude_certificate_rows: u64,
    qary_amplitude_keygen_mul_ops: u64,
    qary_amplitude_product_block_group_size: u64,
    qary_amplitude_product_block_group_count: u64,
    qary_amplitude_product_block_rows_per_group: u64,
    qary_amplitude_product_block_used_rows: u64,
    qary_amplitude_certificate_route: String,
    primal_proxy_bits: f64,
    dual_proxy_bits: f64,
    self_randomization_proxy_bits: f64,
    lossy_diagonal_required_distance: u64,
    lossy_diagonal_slack_bits: f64,
    security_proxy_bits: f64,
    average_to_worst_slack: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Profile {
    id: String,
    target_security_bits: u64,
    max_public_key_bytes: u64,
    max_ciphertext_bytes: u64,
    max_secret_key_bytes: u64,
    min_guard_slack: i64,
    min_average_to_worst_slack: i64,
    description: String,
}

#[derive(Debug, Deserialize)]
struct ProfilesFile {
    profiles: Vec<Profile>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProductBlockManifest {
    entries: Vec<ProductBlockManifestEntry>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProductBlockManifestEntry {
    q: u64,
    target_l1: u64,
    #[serde(default)]
    eta: Option<u64>,
    m1: u64,
    rows_per_group: u64,
    #[serde(default)]
    passes_target: bool,
}

#[derive(Clone, Debug, Serialize)]
struct ProductBlockChoice {
    q: u64,
    eta: Option<u64>,
    m1: u64,
    lee_repetition: u64,
    lee_rows_per_group: u64,
}

#[derive(Clone, Debug, Serialize)]
struct Row {
    candidate: Candidate,
    metrics: Metrics,
    objective_score: f64,
}

#[derive(Debug, Serialize)]
struct SearchSpace {
    mode: String,
    objective: String,
    proof_strategies: Vec<String>,
    primal_distance_models: Vec<String>,
    qary_amplitude_strategies: Vec<String>,
    lee_repetition_values: Vec<u64>,
    lee_rows_per_group_values: Vec<u64>,
    product_block_sidecar_manifests: Vec<String>,
    product_block_sidecar_choices: Vec<ProductBlockChoice>,
    public_key_model: String,
    coefficient_model: String,
    incidence_model: String,
    q_values: Vec<u64>,
    n_values: Vec<u64>,
    m0_overheads: Vec<u64>,
    m0_values: Vec<u64>,
    m1_multipliers: Vec<u64>,
    m1_values: Vec<u64>,
    fiber_dims: Vec<u64>,
    eta_values: Vec<u64>,
    sample_bits_values: Vec<u64>,
    view_coord_values: Vec<u64>,
    pk_seed_byte_values: Vec<u64>,
    total_candidates: u64,
    threads: usize,
    anneal_iterations: Option<u64>,
    anneal_restarts: Option<u64>,
    seed: Option<u64>,
    security_extra_cap: f64,
    guard_slack_target: Option<i64>,
    combined_certification_weight: f64,
}

#[derive(Debug, Serialize)]
struct Payload {
    generated_at: String,
    profile: Profile,
    search_space: SearchSpace,
    rejected_by_reason: BTreeMap<String, u64>,
    accepted_total: usize,
    accepted_top: Vec<Row>,
    pareto_frontier: Vec<Row>,
    notes: Vec<String>,
}

#[derive(Clone, Debug)]
struct Config {
    profiles_path: PathBuf,
    profile: String,
    output_dir: PathBuf,
    q_values: Vec<u64>,
    n_values: Vec<u64>,
    m0_overheads: Vec<u64>,
    m0_values: Vec<u64>,
    m1_multipliers: Vec<u64>,
    m1_values: Vec<u64>,
    fiber_dims: Vec<u64>,
    eta_values: Vec<u64>,
    sample_bits_values: Vec<u64>,
    view_coord_values: Vec<u64>,
    pk_seed_byte_values: Vec<u64>,
    proof_strategies: Vec<String>,
    primal_distance_models: Vec<String>,
    qary_amplitude_strategies: Vec<String>,
    lee_repetition_values: Vec<u64>,
    lee_rows_per_group_values: Vec<u64>,
    product_block_sidecar_manifests: Vec<PathBuf>,
    product_block_choices: Vec<ProductBlockChoice>,
    top: usize,
    threads: usize,
    mode: String,
    anneal_iterations: u64,
    anneal_restarts: u64,
    seed: u64,
    temp_start: f64,
    temp_end: f64,
    coefficient_model: String,
    incidence_model: String,
    public_key_model: String,
    objective: String,
    security_extra_cap: f64,
    guard_slack_target: Option<i64>,
    combined_certification_weight: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profiles_path: PathBuf::from("scripts/parameter_search/default_profiles.json"),
            profile: "descender-128".to_string(),
            output_dir: PathBuf::from("research/parameter_search"),
            q_values: parse_ints("3329,7681,12289,40961,65537"),
            n_values: parse_ints("64,96,128,192,256,384,512"),
            m0_overheads: parse_ints("8,16,32,64,128,256"),
            m0_values: Vec::new(),
            m1_multipliers: parse_ints("1,2,3,4"),
            m1_values: Vec::new(),
            fiber_dims: parse_ints("4,8,16"),
            eta_values: parse_ints("1,2,3,4,6,8"),
            sample_bits_values: parse_ints("0"),
            view_coord_values: parse_ints("0"),
            pk_seed_byte_values: parse_ints("0"),
            proof_strategies: vec!["full-surjective".to_string()],
            primal_distance_models: vec!["guard-only".to_string()],
            qary_amplitude_strategies: vec!["fallback-support".to_string()],
            lee_repetition_values: parse_ints("1"),
            lee_rows_per_group_values: parse_ints("1"),
            product_block_sidecar_manifests: Vec::new(),
            product_block_choices: Vec::new(),
            top: 50,
            threads: thread::available_parallelism().map_or(1, |n| n.get()),
            mode: "grid".to_string(),
            anneal_iterations: 100_000,
            anneal_restarts: 64,
            seed: 0xD35CE5DE_u64,
            temp_start: 64.0,
            temp_end: 0.05,
            coefficient_model: "explicit".to_string(),
            incidence_model: "explicit".to_string(),
            public_key_model: "component".to_string(),
            objective: "screening".to_string(),
            security_extra_cap: 32.0,
            guard_slack_target: None,
            combined_certification_weight: 0.25,
        }
    }
}

fn parse_ints(value: &str) -> Vec<u64> {
    value
        .split(',')
        .filter_map(|part| {
            let trimmed = part.trim();
            (!trimmed.is_empty()).then(|| trimmed.parse::<u64>().expect("invalid integer list"))
        })
        .collect()
}

fn parse_strings(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter_map(|part| {
            let trimmed = part.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })
        .collect()
}

fn load_product_block_choices(path: &Path) -> Result<Vec<ProductBlockChoice>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let manifest: ProductBlockManifest =
        serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    Ok(manifest
        .entries
        .into_iter()
        .filter(|entry| entry.passes_target)
        .map(|entry| ProductBlockChoice {
            q: entry.q,
            eta: entry.eta,
            m1: entry.m1,
            lee_repetition: entry.target_l1,
            lee_rows_per_group: entry.rows_per_group,
        })
        .collect())
}

fn dedup_product_block_choices(choices: &mut Vec<ProductBlockChoice>) {
    choices.sort_by_key(|choice| {
        (
            choice.q,
            choice.eta.unwrap_or(0),
            choice.m1,
            choice.lee_repetition,
            choice.lee_rows_per_group,
        )
    });
    choices.dedup_by_key(|choice| {
        (
            choice.q,
            choice.eta.unwrap_or(0),
            choice.m1,
            choice.lee_repetition,
            choice.lee_rows_per_group,
        )
    });
}

fn m0_choice_count(cfg: &Config) -> usize {
    if cfg.m0_values.is_empty() {
        cfg.m0_overheads.len()
    } else {
        cfg.m0_values.len()
    }
}

fn m0_choice(cfg: &Config, n: u64, index: usize) -> u64 {
    if cfg.m0_values.is_empty() {
        n + cfg.m0_overheads[index]
    } else {
        cfg.m0_values[index]
    }
}

fn m1_choice_count(cfg: &Config) -> usize {
    if cfg.m1_values.is_empty() {
        cfg.m1_multipliers.len()
    } else {
        cfg.m1_values.len()
    }
}

fn m1_choice(cfg: &Config, n: u64, index: usize) -> u64 {
    if cfg.m1_values.is_empty() {
        n * cfg.m1_multipliers[index]
    } else {
        cfg.m1_values[index]
    }
}

fn parse_args() -> Result<Config, String> {
    let mut cfg = Config::default();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = |args: &mut std::iter::Skip<env::Args>, flag: &str| -> Result<String, String> {
            args.next()
                .ok_or_else(|| format!("{flag} requires a value"))
        };
        match arg.as_str() {
            "--profiles" => cfg.profiles_path = PathBuf::from(value(&mut args, "--profiles")?),
            "--profile" => cfg.profile = value(&mut args, "--profile")?,
            "--output-dir" => cfg.output_dir = PathBuf::from(value(&mut args, "--output-dir")?),
            "--q-values" => cfg.q_values = parse_ints(&value(&mut args, "--q-values")?),
            "--n-values" => cfg.n_values = parse_ints(&value(&mut args, "--n-values")?),
            "--m0-overheads" => cfg.m0_overheads = parse_ints(&value(&mut args, "--m0-overheads")?),
            "--m0-values" => cfg.m0_values = parse_ints(&value(&mut args, "--m0-values")?),
            "--m1-multipliers" => {
                cfg.m1_multipliers = parse_ints(&value(&mut args, "--m1-multipliers")?)
            }
            "--m1-values" => cfg.m1_values = parse_ints(&value(&mut args, "--m1-values")?),
            "--fiber-dims" => cfg.fiber_dims = parse_ints(&value(&mut args, "--fiber-dims")?),
            "--eta-values" => cfg.eta_values = parse_ints(&value(&mut args, "--eta-values")?),
            "--sample-bits" => {
                cfg.sample_bits_values = parse_ints(&value(&mut args, "--sample-bits")?)
            }
            "--view-coords" => {
                cfg.view_coord_values = parse_ints(&value(&mut args, "--view-coords")?)
            }
            "--pk-seed-bytes" => {
                cfg.pk_seed_byte_values = parse_ints(&value(&mut args, "--pk-seed-bytes")?)
            }
            "--proof-strategies" | "--proof-strategy" => {
                cfg.proof_strategies = parse_strings(&value(&mut args, "--proof-strategies")?)
            }
            "--primal-distance-models" | "--primal-distance-model" => {
                cfg.primal_distance_models =
                    parse_strings(&value(&mut args, "--primal-distance-models")?)
            }
            "--qary-amplitude-strategies" | "--qary-amplitude-strategy" => {
                cfg.qary_amplitude_strategies =
                    parse_strings(&value(&mut args, "--qary-amplitude-strategies")?)
            }
            "--lee-repetitions" | "--lee-repetition" => {
                cfg.lee_repetition_values = parse_ints(&value(&mut args, "--lee-repetitions")?)
            }
            "--lee-rows-per-group" => {
                cfg.lee_rows_per_group_values =
                    parse_ints(&value(&mut args, "--lee-rows-per-group")?)
            }
            "--product-block-sidecar-manifest" => {
                let path = PathBuf::from(value(&mut args, "--product-block-sidecar-manifest")?);
                cfg.product_block_choices
                    .extend(load_product_block_choices(&path)?);
                cfg.product_block_sidecar_manifests.push(path);
            }
            "--top" => {
                cfg.top = value(&mut args, "--top")?
                    .parse()
                    .map_err(|_| "invalid --top")?
            }
            "--mode" => cfg.mode = value(&mut args, "--mode")?,
            "--anneal-iterations" => {
                cfg.anneal_iterations = value(&mut args, "--anneal-iterations")?
                    .parse()
                    .map_err(|_| "invalid --anneal-iterations")?
            }
            "--anneal-restarts" => {
                cfg.anneal_restarts = value(&mut args, "--anneal-restarts")?
                    .parse()
                    .map_err(|_| "invalid --anneal-restarts")?
            }
            "--seed" => {
                cfg.seed = value(&mut args, "--seed")?
                    .parse()
                    .map_err(|_| "invalid --seed")?
            }
            "--temp-start" => {
                cfg.temp_start = value(&mut args, "--temp-start")?
                    .parse()
                    .map_err(|_| "invalid --temp-start")?
            }
            "--temp-end" => {
                cfg.temp_end = value(&mut args, "--temp-end")?
                    .parse()
                    .map_err(|_| "invalid --temp-end")?
            }
            "--coefficient-model" => {
                cfg.coefficient_model = value(&mut args, "--coefficient-model")?
            }
            "--incidence-model" => cfg.incidence_model = value(&mut args, "--incidence-model")?,
            "--public-key-model" => cfg.public_key_model = value(&mut args, "--public-key-model")?,
            "--objective" => cfg.objective = value(&mut args, "--objective")?,
            "--security-extra-cap" => {
                cfg.security_extra_cap = value(&mut args, "--security-extra-cap")?
                    .parse()
                    .map_err(|_| "invalid --security-extra-cap")?
            }
            "--guard-slack-target" => {
                cfg.guard_slack_target = Some(
                    value(&mut args, "--guard-slack-target")?
                        .parse()
                        .map_err(|_| "invalid --guard-slack-target")?,
                )
            }
            "--combined-certification-weight" => {
                cfg.combined_certification_weight =
                    value(&mut args, "--combined-certification-weight")?
                        .parse()
                        .map_err(|_| "invalid --combined-certification-weight")?
            }
            "--threads" => {
                cfg.threads = value(&mut args, "--threads")?
                    .parse()
                    .map_err(|_| "invalid --threads")?
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    cfg.threads = cfg.threads.max(1);
    dedup_product_block_choices(&mut cfg.product_block_choices);
    match cfg.objective.as_str() {
        "screening" | "bandwidth" => {}
        other => {
            return Err(format!(
                "unknown --objective {other:?}; expected screening or bandwidth"
            ));
        }
    }
    match cfg.public_key_model.as_str() {
        "component" | "canonical" => {}
        other => {
            return Err(format!(
                "unknown --public-key-model {other:?}; expected component or canonical"
            ));
        }
    }
    if cfg.sample_bits_values.is_empty() {
        return Err("--sample-bits must contain at least one entry".to_string());
    }
    if cfg.m0_values.is_empty() && cfg.m0_overheads.is_empty() {
        return Err("--m0-overheads or --m0-values must contain at least one entry".to_string());
    }
    if cfg.m1_values.is_empty() && cfg.m1_multipliers.is_empty() {
        return Err("--m1-multipliers or --m1-values must contain at least one entry".to_string());
    }
    if cfg.view_coord_values.is_empty() {
        return Err("--view-coords must contain at least one entry".to_string());
    }
    if cfg.pk_seed_byte_values.is_empty() {
        return Err("--pk-seed-bytes must contain at least one entry".to_string());
    }
    if cfg.proof_strategies.is_empty() {
        return Err("--proof-strategies must contain at least one entry".to_string());
    }
    if cfg.primal_distance_models.is_empty() {
        return Err("--primal-distance-models must contain at least one entry".to_string());
    }
    if cfg.qary_amplitude_strategies.is_empty() {
        return Err("--qary-amplitude-strategies must contain at least one entry".to_string());
    }
    if cfg.lee_repetition_values.is_empty() {
        return Err("--lee-repetitions must contain at least one entry".to_string());
    }
    if cfg.lee_rows_per_group_values.is_empty() {
        return Err("--lee-rows-per-group must contain at least one entry".to_string());
    }
    for strategy in &cfg.proof_strategies {
        match strategy.as_str() {
            "full-surjective" | "support-covered" | "lossy-budget" => {}
            other => {
                return Err(format!(
                    "unknown proof strategy {other:?}; expected full-surjective, support-covered, or lossy-budget"
                ));
            }
        }
    }
    for model in &cfg.primal_distance_models {
        match model.as_str() {
            "guard-only" | "combined-proxy" => {}
            other => {
                return Err(format!(
                    "unknown primal distance model {other:?}; expected guard-only or combined-proxy"
                ));
            }
        }
    }
    for strategy in &cfg.qary_amplitude_strategies {
        match strategy.as_str() {
            "fallback-support" | "solved-repetition" | "direct-solved-lee" | "shared-pair-lee"
            | "product-block-lee" | "moment-lee" => {}
            other => {
                return Err(format!(
                    "unknown q-ary amplitude strategy {other:?}; expected fallback-support, solved-repetition, direct-solved-lee, shared-pair-lee, product-block-lee, or moment-lee"
                ));
            }
        }
    }
    if cfg.security_extra_cap < 0.0 {
        return Err("--security-extra-cap must be nonnegative".to_string());
    }
    if cfg.combined_certification_weight < 0.0 {
        return Err("--combined-certification-weight must be nonnegative".to_string());
    }
    Ok(cfg)
}

fn print_help() {
    println!(
        "Descender parameter search\n\
         \n\
         Options:\n\
           --profile ID\n\
           --profiles PATH\n\
           --output-dir PATH\n\
           --q-values CSV\n\
           --n-values CSV\n\
           --m0-overheads CSV\n\
           --m0-values CSV         direct guard sizes; overrides m0-overheads\n\
           --m1-multipliers CSV\n\
           --m1-values CSV         direct free-column counts; overrides m1-multipliers\n\
           --fiber-dims CSV\n\
           --eta-values CSV\n\
           --sample-bits CSV        0 means full field-element width\n\
           --view-coords CSV        0 means full coordinate count m0+m1\n\
           --pk-seed-bytes CSV      0 means model default seed bytes\n\
           --proof-strategies CSV   full-surjective,support-covered,lossy-budget\n\
           --primal-distance-models CSV guard-only,combined-proxy\n\
           --qary-amplitude-strategies CSV fallback-support,solved-repetition,direct-solved-lee,shared-pair-lee,product-block-lee,moment-lee\n\
           --lee-repetitions CSV     repetition counts or hypothetical solved-map Lee distances\n\
           --lee-rows-per-group CSV   product-block rows per two-free-coordinate block\n\
           --product-block-sidecar-manifest PATH  use passing product-block sidecars as certified search choices; repeatable\n\
           --top N\n\
           --threads N\n\
           --mode grid|anneal\n\
           --anneal-iterations N\n\
           --anneal-restarts N\n\
           --seed N\n\
           --temp-start FLOAT\n\
           --temp-end FLOAT\n\
           --objective screening|bandwidth\n\
           --security-extra-cap FLOAT\n\
           --guard-slack-target INT\n\
           --combined-certification-weight FLOAT\n\
           --public-key-model component|canonical\n\
           --coefficient-model explicit|seeded\n\
           --incidence-model explicit|seeded"
    );
}

fn load_profile(path: &Path, id: &str) -> Result<Profile, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let file: ProfilesFile =
        serde_json::from_str(&text).map_err(|e| format!("parse profiles: {e}"))?;
    file.profiles
        .into_iter()
        .find(|profile| profile.id == id)
        .ok_or_else(|| format!("unknown profile {id:?}"))
}

fn elem_bytes(q: u64) -> u64 {
    let bit_length = u64::BITS as u64 - q.leading_zeros() as u64;
    ((bit_length + 7) / 8).max(1)
}

fn field_element_bits(q: u64) -> u64 {
    (u64::BITS as u64 - q.leading_zeros() as u64).max(1)
}

fn encoded_sample_bits(c: &Candidate) -> u64 {
    if c.sample_bits == 0 {
        field_element_bits(c.q)
    } else {
        c.sample_bits
    }
}

fn packed_bytes(entries: u64, bits_per_entry: u64) -> u64 {
    entries.saturating_mul(bits_per_entry).saturating_add(7) / 8
}

fn sample_compression_error(q: u64, sample_bits: u64) -> u64 {
    let full_bits = field_element_bits(q);
    if sample_bits == 0 || sample_bits >= full_bits {
        return 0;
    }
    let denominator_bits = sample_bits.saturating_add(1).min(62);
    let denominator = 1_u64 << denominator_bits;
    q.saturating_add(denominator - 1) / denominator
}

fn lossy_diagonal_required_distance(q: u64, n: u64, effective_eta: u64) -> u64 {
    let logq = (q as f64).log2();
    let bad_scale_width = (4 * effective_eta + 1).max(1) as f64;
    let per_coordinate_saving = (logq - bad_scale_width.log2()).max(0.0);
    if per_coordinate_saving <= 0.0 {
        return u64::MAX;
    }
    ((n as f64 * logq) / per_coordinate_saving).ceil() as u64
}

fn lossy_diagonal_slack_bits(q: u64, n: u64, support_distance: i64, effective_eta: u64) -> f64 {
    let logq = (q as f64).log2();
    let bad_scale_width = (4 * effective_eta + 1).max(1) as f64;
    let per_coordinate_saving = (logq - bad_scale_width.log2()).max(0.0);
    support_distance.max(0) as f64 * per_coordinate_saving - n as f64 * logq
}

fn qary_amplitude_l1_required(q: u64, effective_eta: u64) -> u64 {
    if effective_eta == 0 {
        u64::MAX
    } else {
        (q.saturating_sub(1)).div_ceil(2 * effective_eta)
    }
}

fn log_factorial(n: u64) -> f64 {
    (1..=n).map(|i| (i as f64).ln()).sum()
}

fn qary_dual_minkowski_l1_upper(q: u64, view_len: u64, rank_bound: u64) -> f64 {
    if view_len == 0 {
        return 0.0;
    }
    let rank = rank_bound.min(view_len);
    ((log_factorial(view_len) + rank as f64 * (q as f64).ln()) / view_len as f64).exp()
}

fn qary_amplitude_l1_certified(c: &Candidate) -> u64 {
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => (c.n + 1).max(1),
        "solved-repetition" => c.lee_repetition.saturating_add(1),
        "direct-solved-lee" | "shared-pair-lee" | "product-block-lee" | "moment-lee" => {
            c.lee_repetition
        }
        other => panic!("unknown q-ary amplitude strategy {other:?}"),
    }
}

fn qary_amplitude_certificate_rows(c: &Candidate) -> u64 {
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => c.n,
        "solved-repetition" => c.lee_repetition.saturating_mul(c.m1),
        "direct-solved-lee" | "moment-lee" => c.n,
        "shared-pair-lee" | "product-block-lee" => product_block_used_rows(c),
        other => panic!("unknown q-ary amplitude strategy {other:?}"),
    }
}

fn qary_amplitude_keygen_mul_ops(c: &Candidate) -> u64 {
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => 0,
        "solved-repetition" => c.n.saturating_mul(c.m1).saturating_mul(c.lee_repetition),
        "direct-solved-lee" | "moment-lee" => c.n.saturating_mul(c.m1),
        "shared-pair-lee" | "product-block-lee" => 2u64.saturating_mul(product_block_used_rows(c)),
        other => panic!("unknown q-ary amplitude strategy {other:?}"),
    }
}

fn is_product_block_lee(c: &Candidate) -> bool {
    matches!(
        c.qary_amplitude_strategy.as_str(),
        "shared-pair-lee" | "product-block-lee"
    )
}

fn product_block_group_size(c: &Candidate) -> u64 {
    if is_product_block_lee(c) { 2 } else { 0 }
}

fn product_block_group_count(c: &Candidate) -> u64 {
    if is_product_block_lee(c) { c.m1 / 2 } else { 0 }
}

fn product_block_used_rows(c: &Candidate) -> u64 {
    product_block_group_count(c).saturating_mul(c.lee_rows_per_group)
}

fn qary_amplitude_certificate_route(c: &Candidate) -> String {
    match c.qary_amplitude_strategy.as_str() {
        "shared-pair-lee" | "product-block-lee" =>
            "productBlockComposableSystematicLeeCertificate -> composableSystematicLeeDualCertificate".to_string(),
        other => other.to_string(),
    }
}

fn has_product_block_manifest_match(c: &Candidate, cfg: &Config) -> bool {
    if !is_product_block_lee(c) || cfg.product_block_choices.is_empty() {
        return true;
    }
    cfg.product_block_choices.iter().any(|choice| {
        choice.q == c.q
            && choice.m1 == c.m1
            && choice.eta.is_none_or(|eta| eta == c.eta)
            && choice.lee_repetition >= c.lee_repetition
            && choice.lee_rows_per_group == c.lee_rows_per_group
    })
}

fn lee_choices_for(
    qary_amplitude_strategy: &str,
    q: u64,
    m1: u64,
    eta: u64,
    cfg: &Config,
) -> Vec<(u64, u64)> {
    if matches!(qary_amplitude_strategy, "shared-pair-lee" | "product-block-lee")
        && !cfg.product_block_choices.is_empty()
    {
        let mut choices: Vec<_> = cfg
            .product_block_choices
            .iter()
            .filter(|choice| {
                choice.q == q
                    && choice.m1 == m1
                    && choice.eta.is_none_or(|choice_eta| choice_eta == eta)
            })
            .map(|choice| (choice.lee_repetition, choice.lee_rows_per_group))
            .collect();
        choices.sort_unstable();
        choices.dedup();
        choices
    } else {
        let mut choices = Vec::new();
        for &lee_repetition in &cfg.lee_repetition_values {
            for &lee_rows_per_group in &cfg.lee_rows_per_group_values {
                choices.push((lee_repetition, lee_rows_per_group));
            }
        }
        choices
    }
}

fn log2_choose_upper(n: u64, k: u64) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    if k == 0 || k == n {
        return 0.0;
    }
    let k = k.min(n - k);
    (0..k)
        .map(|i| ((n - i) as f64).log2() - ((i + 1) as f64).log2())
        .sum()
}

fn coefficient_public_bytes(c: &Candidate, elem_bytes: u64, coefficient_model: &str) -> u64 {
    match coefficient_model {
        "explicit" => c.m1 * (2 * c.fiber_dim * elem_bytes),
        "seeded" => {
            let coefficient_seed_bytes = 64;
            let rule_descriptor_bytes = 32;
            let fiber_rule_bytes = 2 * c.fiber_dim * elem_bytes;
            coefficient_seed_bytes + rule_descriptor_bytes + fiber_rule_bytes
        }
        other => panic!("unknown coefficient model {other:?}"),
    }
}

fn incidence_public_bytes(_c: &Candidate, incidence_model: &str) -> u64 {
    match incidence_model {
        "explicit" => _c.m1 * 6,
        "seeded" => {
            let thinning_seed_bytes = 64;
            let nonce_and_counter_bytes = 16;
            let verifier_rule_bytes = 32;
            thinning_seed_bytes + nonce_and_counter_bytes + verifier_rule_bytes
        }
        other => panic!("unknown incidence model {other:?}"),
    }
}

fn diagonal_public_bytes(c: &Candidate, elem_bytes: u64, incidence_model: &str) -> u64 {
    match incidence_model {
        "explicit" => c.m1 * elem_bytes,
        "seeded" => {
            let diagonal_seed_bytes = 64;
            let diagonal_rule_bytes = 16;
            diagonal_seed_bytes + diagonal_rule_bytes
        }
        other => panic!("unknown incidence model {other:?}"),
    }
}

struct PublicKeyAccounting {
    setup_seed_bytes: u64,
    cauchy_public_bytes: u64,
    coefficient_public_bytes: u64,
    incidence_public_bytes: u64,
    diagonal_public_bytes: u64,
    public_sample_bytes: u64,
}

fn public_key_accounting(
    c: &Candidate,
    elem_bytes: u64,
    coefficient_model: &str,
    incidence_model: &str,
    public_key_model: &str,
) -> PublicKeyAccounting {
    let public_sample_bytes = packed_bytes(c.view_len(), encoded_sample_bits(c));
    let component_seed_bytes = if c.pk_seed_bytes == 0 {
        96
    } else {
        c.pk_seed_bytes
    };
    let canonical_seed_bytes = if c.pk_seed_bytes == 0 {
        64
    } else {
        c.pk_seed_bytes
    };
    match public_key_model {
        "component" => PublicKeyAccounting {
            setup_seed_bytes: component_seed_bytes,
            cauchy_public_bytes: (c.n + c.m()) * elem_bytes,
            coefficient_public_bytes: coefficient_public_bytes(c, elem_bytes, coefficient_model),
            incidence_public_bytes: incidence_public_bytes(c, incidence_model),
            diagonal_public_bytes: diagonal_public_bytes(c, elem_bytes, incidence_model),
            public_sample_bytes,
        },
        "canonical" => PublicKeyAccounting {
            setup_seed_bytes: canonical_seed_bytes,
            cauchy_public_bytes: 0,
            coefficient_public_bytes: 0,
            incidence_public_bytes: 0,
            diagonal_public_bytes: 0,
            public_sample_bytes,
        },
        other => panic!("unknown public key model {other:?}"),
    }
}

fn evaluate(
    c: &Candidate,
    coefficient_model: &str,
    incidence_model: &str,
    public_key_model: &str,
) -> Metrics {
    let eb = elem_bytes(c.q);
    let logq = (c.q as f64).log2();
    let dense_pk = c.n * c.m() * eb;

    let accounting =
        public_key_accounting(c, eb, coefficient_model, incidence_model, public_key_model);
    let compact_pk = accounting.setup_seed_bytes
        + accounting.cauchy_public_bytes
        + accounting.coefficient_public_bytes
        + accounting.incidence_public_bytes
        + accounting.diagonal_public_bytes
        + accounting.public_sample_bytes;

    let sample_bits = encoded_sample_bits(c);
    let sample_error = sample_compression_error(c.q, c.sample_bits);
    let effective_eta = c.eta.saturating_add(sample_error);

    let ct = packed_bytes(c.view_len(), sample_bits) + 32;
    let sk = c.n * eb + 64;

    let guard_distance = c.m0 as i64 - c.n as i64 + 1;
    let measurement_distance = c.view_len() as i64 - c.n as i64 + 1;
    let correctness_distance = match c.primal_distance_model.as_str() {
        "guard-only" => guard_distance,
        "combined-proxy" => measurement_distance,
        other => panic!("unknown primal distance model {other:?}"),
    };
    let required_distance = 2 * effective_eta as i64 + 1;
    let guard_slack = guard_distance - required_distance;
    let correctness_slack = correctness_distance - required_distance;
    let combined_rank_deficit = if c.primal_distance_model == "combined-proxy" {
        c.n as i64 - c.m0 as i64
    } else {
        0
    }
    .max(0);
    let combined_distance_gap = if c.primal_distance_model == "combined-proxy" {
        required_distance - guard_distance
    } else {
        0
    }
    .max(0);
    let combined_certification_margin = if c.primal_distance_model == "combined-proxy" {
        correctness_slack - combined_distance_gap
    } else {
        0
    };
    let combined_certification_pressure = (combined_rank_deficit + combined_distance_gap) as f64;
    let cauchy_slack = c.q as i64 - (c.n + c.m()) as i64;
    let noise_headroom = (c.q / 2) as i64 - effective_eta as i64;
    let qary_amplitude_l1_required = qary_amplitude_l1_required(c.q, effective_eta);
    let qary_amplitude_l1_certified = qary_amplitude_l1_certified(c);
    let qary_amplitude_at_noise_pass = qary_amplitude_l1_certified >= qary_amplitude_l1_required;
    let qary_amplitude_minkowski_l1_upper = qary_dual_minkowski_l1_upper(c.q, c.view_len(), c.n);
    let qary_amplitude_minkowski_margin =
        qary_amplitude_minkowski_l1_upper - qary_amplitude_l1_required as f64;
    let qary_amplitude_geometry_possible = qary_amplitude_minkowski_margin >= 0.0;
    let qary_amplitude_certificate_rows = qary_amplitude_certificate_rows(c);
    let qary_amplitude_keygen_mul_ops = qary_amplitude_keygen_mul_ops(c);
    let qary_amplitude_product_block_group_size = product_block_group_size(c);
    let qary_amplitude_product_block_group_count = product_block_group_count(c);
    let qary_amplitude_product_block_rows_per_group = if is_product_block_lee(c) {
        c.lee_rows_per_group
    } else {
        0
    };
    let qary_amplitude_product_block_used_rows = product_block_used_rows(c);
    let qary_amplitude_certificate_route = qary_amplitude_certificate_route(c);

    let noise_width = 2 * effective_eta + 1;
    let primal_proxy = 0.35 * c.n as f64 * ((c.q as f64 / noise_width as f64).log2()).max(0.0);

    let dual_window =
        1_u64.max((guard_distance.max(0) as u64).min(c.fiber_dim * c.m1 / c.n.max(1)));
    let dual_support_cost = log2_choose_upper(c.m(), dual_window);
    let dual_coeff_cost = dual_window as f64 * ((2_u64.max(c.q / noise_width)) as f64).log2();
    let dual_proxy = 0.5 * (dual_support_cost + dual_coeff_cost);

    let entropy_source = c.n as f64 * (c.q as f64).log2();
    let public_measurements = c.view_len() as f64 * (noise_width as f64).log2();
    let randomization_proxy = (entropy_source - public_measurements).max(0.0);
    let lossy_diagonal_required_distance =
        lossy_diagonal_required_distance(c.q, c.n, effective_eta);
    let lossy_diagonal_slack_bits =
        lossy_diagonal_slack_bits(c.q, c.n, correctness_distance, effective_eta);

    let security_proxy = primal_proxy.min(dual_proxy).min(randomization_proxy);
    let average_to_worst_slack = randomization_proxy - 0.125 * c.n as f64;

    Metrics {
        logq_bits: logq,
        elem_bytes: eb,
        dense_public_key_bytes: dense_pk,
        setup_seed_public_bytes: accounting.setup_seed_bytes,
        cauchy_public_bytes: accounting.cauchy_public_bytes,
        coefficient_public_bytes: accounting.coefficient_public_bytes,
        incidence_public_bytes: accounting.incidence_public_bytes,
        diagonal_public_bytes: accounting.diagonal_public_bytes,
        sample_encoding_bits: sample_bits,
        sample_compression_error: sample_error,
        effective_eta,
        public_sample_bytes: accounting.public_sample_bytes,
        compact_public_key_bytes: compact_pk,
        ciphertext_bytes: ct,
        secret_key_bytes: sk,
        guard_distance,
        correctness_distance,
        guard_slack,
        correctness_slack,
        combined_rank_deficit,
        combined_distance_gap,
        combined_certification_margin,
        combined_certification_pressure,
        cauchy_field_slack: cauchy_slack,
        noise_headroom,
        qary_amplitude_l1_required,
        qary_amplitude_l1_certified,
        qary_amplitude_at_noise_pass,
        qary_amplitude_minkowski_l1_upper,
        qary_amplitude_minkowski_margin,
        qary_amplitude_geometry_possible,
        qary_amplitude_certificate_rows,
        qary_amplitude_keygen_mul_ops,
        qary_amplitude_product_block_group_size,
        qary_amplitude_product_block_group_count,
        qary_amplitude_product_block_rows_per_group,
        qary_amplitude_product_block_used_rows,
        qary_amplitude_certificate_route,
        primal_proxy_bits: primal_proxy,
        dual_proxy_bits: dual_proxy,
        self_randomization_proxy_bits: randomization_proxy,
        lossy_diagonal_required_distance,
        lossy_diagonal_slack_bits,
        security_proxy_bits: security_proxy,
        average_to_worst_slack,
    }
}

fn hard_failures(c: &Candidate, m: &Metrics, p: &Profile) -> Vec<&'static str> {
    let mut failures = Vec::new();
    if c.view_len() > c.m() {
        failures.push("view_coords_exceed_full_coordinates");
    }
    if c.primal_distance_model == "guard-only" && c.m0 < c.n {
        failures.push("guard_only_requires_m0_ge_n");
    }
    if c.primal_distance_model == "guard-only" && c.view_len() < c.m0 {
        failures.push("guard_only_requires_guard_view");
    }
    if c.primal_distance_model == "combined-proxy" && c.view_len() < c.n {
        failures.push("combined_proxy_requires_view_ge_n");
    }
    match c.proof_strategy.as_str() {
        "full-surjective" | "support-covered" => {
            if c.proof_strategy == "full-surjective" && c.view_coords != 0 && c.view_coords != c.m()
            {
                failures.push("full_surjective_requires_full_view");
            }
        }
        "lossy-budget" => {}
        _ => failures.push("unknown_proof_strategy"),
    }
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => {}
        "solved-repetition" => {
            if c.lee_repetition == 0 {
                failures.push("lee_repetition_positive");
            }
            if c.lee_repetition.saturating_mul(c.m1) > c.n {
                failures.push("solved_repetition_requires_pivot_capacity");
            }
        }
        "direct-solved-lee" => {
            if c.lee_repetition == 0 {
                failures.push("direct_solved_lee_positive");
            }
        }
        "shared-pair-lee" | "product-block-lee" => {
            if c.lee_repetition == 0 {
                failures.push("shared_pair_lee_positive");
            }
            if c.lee_rows_per_group == 0 {
                failures.push("product_block_rows_per_group_positive");
            }
            if c.m1 % 2 != 0 {
                failures.push("shared_pair_requires_even_m1");
            }
            if product_block_used_rows(c) > c.n {
                failures.push("shared_pair_requires_group_rows");
            }
        }
        "moment-lee" => {
            if c.lee_repetition == 0 {
                failures.push("moment_lee_positive");
            }
        }
        _ => failures.push("unknown_qary_amplitude_strategy"),
    }
    if c.q <= c.n + c.m() {
        failures.push("cauchy_field_size");
    }
    if c.eta == 0 {
        failures.push("positive_noise");
    }
    if m.noise_headroom <= 0 {
        failures.push("centered_noise_headroom");
    }
    if !m.qary_amplitude_at_noise_pass {
        failures.push("qary_amplitude_at_noise");
    }
    if !m.qary_amplitude_geometry_possible {
        failures.push("qary_amplitude_minkowski_obstruction");
    }
    if m.correctness_slack < p.min_guard_slack {
        failures.push("correctness_distance_slack");
    }
    if m.average_to_worst_slack < p.min_average_to_worst_slack as f64 {
        failures.push("average_to_worst_slack_proxy");
    }
    if m.security_proxy_bits < p.target_security_bits as f64 {
        failures.push("security_proxy_bits");
    }
    if m.compact_public_key_bytes > p.max_public_key_bytes {
        failures.push("public_key_budget");
    }
    if m.ciphertext_bytes > p.max_ciphertext_bytes {
        failures.push("ciphertext_budget");
    }
    if m.secret_key_bytes > p.max_secret_key_bytes {
        failures.push("secret_key_budget");
    }
    failures
}

fn effective_guard_slack_target(cfg: &Config, p: &Profile) -> i64 {
    cfg.guard_slack_target
        .unwrap_or(p.min_guard_slack)
        .max(p.min_guard_slack)
}

fn screening_objective_score(c: &Candidate, m: &Metrics, p: &Profile, cfg: &Config) -> f64 {
    let target = p.target_security_bits as f64;
    let security_credit = m.security_proxy_bits.min(target + cfg.security_extra_cap);
    let pk_ratio = m.compact_public_key_bytes as f64 / p.max_public_key_bytes as f64;
    let ct_ratio = m.ciphertext_bytes as f64 / p.max_ciphertext_bytes as f64;
    let sk_ratio = m.secret_key_bytes as f64 / p.max_secret_key_bytes as f64;
    let arithmetic_penalty = 6.0 * (m.logq_bits - 16.0).max(0.0);
    let tiny_noise_penalty = match m.effective_eta {
        0 | 1 => 24.0,
        2 => 12.0,
        3 => 6.0,
        _ => 0.0,
    };
    let guard_ratio = c.m0 as f64 / c.n.max(1) as f64;
    let guard_penalty = 12.0 * guard_ratio.log2().max(0.0);
    let useful_guard_bonus = 0.025 * (m.correctness_slack.max(0).min(256) as f64);
    let combined_certification_penalty =
        cfg.combined_certification_weight * m.combined_certification_pressure;
    let lossy_diagonal_penalty = if c.proof_strategy == "lossy-budget" {
        0.10 * (-m.lossy_diagonal_slack_bits).max(0.0).min(512.0)
    } else {
        0.0
    };

    security_credit + useful_guard_bonus
        - 80.0 * pk_ratio
        - 40.0 * ct_ratio
        - 16.0 * sk_ratio
        - arithmetic_penalty
        - tiny_noise_penalty
        - guard_penalty
        - combined_certification_penalty
        - lossy_diagonal_penalty
}

fn bandwidth_objective_score(c: &Candidate, m: &Metrics, p: &Profile, cfg: &Config) -> f64 {
    let target = p.target_security_bits as f64;
    let security_extra = (m.security_proxy_bits - target)
        .max(0.0)
        .min(cfg.security_extra_cap);
    let guard_target = effective_guard_slack_target(cfg, p);
    let guard_extra = (m.correctness_slack - guard_target).max(0).min(64) as f64;
    let average_extra = (m.average_to_worst_slack - p.min_average_to_worst_slack as f64)
        .max(0.0)
        .min(64.0);

    let pk_ratio = m.compact_public_key_bytes as f64 / p.max_public_key_bytes as f64;
    let ct_ratio = m.ciphertext_bytes as f64 / p.max_ciphertext_bytes as f64;
    let sk_ratio = m.secret_key_bytes as f64 / p.max_secret_key_bytes as f64;
    let dimension_penalty = 0.10 * (c.n + c.m0 + c.m1) as f64;
    let modulus_penalty = 12.0 * (m.logq_bits - 16.0).max(0.0);
    let tiny_noise_penalty = match m.effective_eta {
        0 | 1 => 40.0,
        2 => 18.0,
        3 => 8.0,
        _ => 0.0,
    };
    let combined_certification_penalty =
        cfg.combined_certification_weight * m.combined_certification_pressure;
    let lossy_diagonal_penalty = if c.proof_strategy == "lossy-budget" {
        0.10 * (-m.lossy_diagonal_slack_bits).max(0.0).min(512.0)
    } else {
        0.0
    };

    0.05 * security_extra + 0.01 * guard_extra + 0.005 * average_extra
        - 10000.0 * ct_ratio
        - 250.0 * pk_ratio
        - 80.0 * sk_ratio
        - dimension_penalty
        - modulus_penalty
        - tiny_noise_penalty
        - combined_certification_penalty
        - lossy_diagonal_penalty
}

fn objective_score(c: &Candidate, m: &Metrics, p: &Profile, cfg: &Config) -> f64 {
    match cfg.objective.as_str() {
        "screening" => screening_objective_score(c, m, p, cfg),
        "bandwidth" => bandwidth_objective_score(c, m, p, cfg),
        other => panic!("unknown objective mode {other:?}"),
    }
}

fn make_row(candidate: Candidate, profile: &Profile, cfg: &Config) -> (Row, Vec<&'static str>) {
    let metrics = evaluate(
        &candidate,
        &cfg.coefficient_model,
        &cfg.incidence_model,
        &cfg.public_key_model,
    );
    let mut failures = hard_failures(&candidate, &metrics, profile);
    if !has_product_block_manifest_match(&candidate, cfg) {
        failures.push("product_block_sidecar_manifest_missing");
    }
    let objective_score = if failures.is_empty() {
        objective_score(&candidate, &metrics, profile, cfg)
    } else {
        f64::NEG_INFINITY
    };
    (
        Row {
            candidate,
            metrics,
            objective_score,
        },
        failures,
    )
}

fn row_cmp(a: &Row, b: &Row) -> Ordering {
    b.objective_score
        .partial_cmp(&a.objective_score)
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            b.metrics
                .security_proxy_bits
                .partial_cmp(&a.metrics.security_proxy_bits)
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| {
            a.metrics
                .compact_public_key_bytes
                .cmp(&b.metrics.compact_public_key_bytes)
        })
        .then_with(|| a.metrics.ciphertext_bytes.cmp(&b.metrics.ciphertext_bytes))
        .then_with(|| a.candidate.key().cmp(&b.candidate.key()))
}

fn pareto_frontier(rows: &[Row]) -> Vec<Row> {
    let mut frontier = Vec::new();
    for row in rows {
        let dominated = rows.iter().any(|other| {
            !std::ptr::eq(row, other)
                && other.metrics.security_proxy_bits >= row.metrics.security_proxy_bits
                && other.metrics.compact_public_key_bytes <= row.metrics.compact_public_key_bytes
                && other.metrics.ciphertext_bytes <= row.metrics.ciphertext_bytes
                && (other.metrics.security_proxy_bits > row.metrics.security_proxy_bits
                    || other.metrics.compact_public_key_bytes
                        < row.metrics.compact_public_key_bytes
                    || other.metrics.ciphertext_bytes < row.metrics.ciphertext_bytes)
        });
        if !dominated {
            frontier.push(row.clone());
        }
    }
    frontier.sort_by(row_cmp);
    frontier
}

fn truncate_rows(rows: &mut Vec<Row>, limit: usize) {
    rows.sort_by(row_cmp);
    rows.dedup_by(|a, b| a.candidate.key() == b.candidate.key());
    if rows.len() > limit {
        rows.truncate(limit);
    }
}

#[derive(Clone, Copy)]
struct Indices {
    proof_strategy: usize,
    primal_distance_model: usize,
    qary_amplitude_strategy: usize,
    q: usize,
    n: usize,
    m0: usize,
    m1: usize,
    fiber: usize,
    eta: usize,
    sample_bits: usize,
    view_coords: usize,
    pk_seed_bytes: usize,
    lee_repetition: usize,
    lee_rows_per_group: usize,
}

impl Indices {
    fn candidate(self, cfg: &Config) -> Candidate {
        let n = cfg.n_values[self.n];
        let qary_amplitude_strategy = cfg.qary_amplitude_strategies[self.qary_amplitude_strategy]
            .clone();
        let q = cfg.q_values[self.q];
        let m1 = m1_choice(cfg, n, self.m1);
        let eta = cfg.eta_values[self.eta];
        let manifest_lee_choice =
            lee_choices_for(&qary_amplitude_strategy, q, m1, eta, cfg).into_iter().next();
        let (lee_repetition, lee_rows_per_group) = manifest_lee_choice.unwrap_or((
            cfg.lee_repetition_values[self.lee_repetition],
            cfg.lee_rows_per_group_values[self.lee_rows_per_group],
        ));
        Candidate {
            proof_strategy: cfg.proof_strategies[self.proof_strategy].clone(),
            primal_distance_model: cfg.primal_distance_models[self.primal_distance_model].clone(),
            qary_amplitude_strategy,
            q,
            n,
            m0: m0_choice(cfg, n, self.m0),
            m1,
            fiber_dim: cfg.fiber_dims[self.fiber],
            eta,
            sample_bits: cfg.sample_bits_values[self.sample_bits],
            view_coords: cfg.view_coord_values[self.view_coords],
            pk_seed_bytes: cfg.pk_seed_byte_values[self.pk_seed_bytes],
            lee_repetition,
            lee_rows_per_group,
        }
    }
}

#[derive(Clone)]
struct Rng64 {
    state: u64,
}

impl Rng64 {
    fn new(seed: u64) -> Self {
        Self { state: seed | 1 }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn usize(&mut self, upper: usize) -> usize {
        if upper <= 1 {
            0
        } else {
            (self.next_u64() as usize) % upper
        }
    }

    fn unit(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * (1.0 / ((1_u64 << 53) as f64))
    }
}

fn random_indices(cfg: &Config, rng: &mut Rng64) -> Indices {
    Indices {
        proof_strategy: rng.usize(cfg.proof_strategies.len()),
        primal_distance_model: rng.usize(cfg.primal_distance_models.len()),
        qary_amplitude_strategy: rng.usize(cfg.qary_amplitude_strategies.len()),
        q: rng.usize(cfg.q_values.len()),
        n: rng.usize(cfg.n_values.len()),
        m0: rng.usize(m0_choice_count(cfg)),
        m1: rng.usize(m1_choice_count(cfg)),
        fiber: rng.usize(cfg.fiber_dims.len()),
        eta: rng.usize(cfg.eta_values.len()),
        sample_bits: rng.usize(cfg.sample_bits_values.len()),
        view_coords: rng.usize(cfg.view_coord_values.len()),
        pk_seed_bytes: rng.usize(cfg.pk_seed_byte_values.len()),
        lee_repetition: rng.usize(cfg.lee_repetition_values.len()),
        lee_rows_per_group: rng.usize(cfg.lee_rows_per_group_values.len()),
    }
}

fn shift_index(value: usize, len: usize, rng: &mut Rng64) -> usize {
    if len <= 1 {
        return value;
    }
    if rng.unit() < 0.08 {
        return rng.usize(len);
    }
    match rng.usize(2) {
        0 => value.saturating_sub(1),
        _ => (value + 1).min(len - 1),
    }
}

fn neighbor_indices(mut idx: Indices, cfg: &Config, rng: &mut Rng64) -> Indices {
    match rng.usize(14) {
        0 => idx.proof_strategy = shift_index(idx.proof_strategy, cfg.proof_strategies.len(), rng),
        1 => {
            idx.primal_distance_model = shift_index(
                idx.primal_distance_model,
                cfg.primal_distance_models.len(),
                rng,
            )
        }
        2 => {
            idx.qary_amplitude_strategy = shift_index(
                idx.qary_amplitude_strategy,
                cfg.qary_amplitude_strategies.len(),
                rng,
            )
        }
        3 => idx.q = shift_index(idx.q, cfg.q_values.len(), rng),
        4 => idx.n = shift_index(idx.n, cfg.n_values.len(), rng),
        5 => idx.m0 = shift_index(idx.m0, m0_choice_count(cfg), rng),
        6 => idx.m1 = shift_index(idx.m1, m1_choice_count(cfg), rng),
        7 => idx.fiber = shift_index(idx.fiber, cfg.fiber_dims.len(), rng),
        8 => idx.eta = shift_index(idx.eta, cfg.eta_values.len(), rng),
        9 => idx.sample_bits = shift_index(idx.sample_bits, cfg.sample_bits_values.len(), rng),
        10 => idx.view_coords = shift_index(idx.view_coords, cfg.view_coord_values.len(), rng),
        11 => {
            idx.pk_seed_bytes = shift_index(idx.pk_seed_bytes, cfg.pk_seed_byte_values.len(), rng)
        }
        12 => {
            idx.lee_repetition =
                shift_index(idx.lee_repetition, cfg.lee_repetition_values.len(), rng)
        }
        _ => {
            idx.lee_rows_per_group = shift_index(
                idx.lee_rows_per_group,
                cfg.lee_rows_per_group_values.len(),
                rng,
            )
        }
    }
    idx
}

fn temperature(cfg: &Config, iter: u64) -> f64 {
    if cfg.anneal_iterations <= 1 {
        return cfg.temp_end.max(1.0e-9);
    }
    let start = cfg.temp_start.max(1.0e-9);
    let end = cfg.temp_end.max(1.0e-9);
    let t = iter as f64 / (cfg.anneal_iterations - 1) as f64;
    start * (end / start).powf(t)
}

fn run_grid_search(
    cfg: &Config,
    profile: &Profile,
) -> (Vec<Row>, usize, BTreeMap<String, u64>, u64) {
    let mut tasks = Vec::new();
    for &q in &cfg.q_values {
        for &n in &cfg.n_values {
            tasks.push((q, n));
        }
    }
    let mut total_candidates = 0u64;
    for &(q, n) in &tasks {
        for qary_amplitude_strategy in &cfg.qary_amplitude_strategies {
            for m1_index in 0..m1_choice_count(cfg) {
                let m1 = m1_choice(cfg, n, m1_index);
                for &eta in &cfg.eta_values {
                    let lee_choices = lee_choices_for(qary_amplitude_strategy, q, m1, eta, cfg);
                    total_candidates = total_candidates.saturating_add(
                        (cfg.proof_strategies.len()
                            * cfg.primal_distance_models.len()
                            * m0_choice_count(cfg)
        * cfg.fiber_dims.len()
        * cfg.sample_bits_values.len()
        * cfg.view_coord_values.len()
        * cfg.pk_seed_byte_values.len()
        * lee_choices.len()) as u64,
                    );
                }
            }
        }
    }

    let chunk_size = (tasks.len() + cfg.threads - 1) / cfg.threads;
    let mut handles = Vec::new();
    for chunk in tasks.chunks(chunk_size.max(1)) {
        let chunk = chunk.to_vec();
        let profile = profile.clone();
        let cfg = cfg.clone();
        handles.push(thread::spawn(move || {
            let mut accepted = Vec::new();
            let mut accepted_total = 0_usize;
            let mut rejected = BTreeMap::new();
            for (q, n) in chunk {
                for proof_strategy in &cfg.proof_strategies {
                    for primal_distance_model in &cfg.primal_distance_models {
                        for qary_amplitude_strategy in &cfg.qary_amplitude_strategies {
                            for m0_index in 0..m0_choice_count(&cfg) {
                                let m0 = m0_choice(&cfg, n, m0_index);
                                for m1_index in 0..m1_choice_count(&cfg) {
                                    let m1 = m1_choice(&cfg, n, m1_index);
                                    for &fiber_dim in &cfg.fiber_dims {
                                        for &eta in &cfg.eta_values {
                                            for &sample_bits in &cfg.sample_bits_values {
                                                for &view_coords in &cfg.view_coord_values {
                                                    for &pk_seed_bytes in &cfg.pk_seed_byte_values {
                                                        for (lee_repetition, lee_rows_per_group) in
                                                            lee_choices_for(
                                                                qary_amplitude_strategy,
                                                                q,
                                                                m1,
                                                                eta,
                                                                &cfg,
                                                            )
                                                        {
                                                            let candidate = Candidate {
                                                                proof_strategy: proof_strategy.clone(),
                                                                primal_distance_model:
                                                                    primal_distance_model.clone(),
                                                                qary_amplitude_strategy:
                                                                    qary_amplitude_strategy.clone(),
                                                                q,
                                                                n,
                                                                m0,
                                                                m1,
                                                                fiber_dim,
                                                                eta,
                                                                sample_bits,
                                                                view_coords,
                                                                pk_seed_bytes,
                                                                lee_repetition,
                                                                lee_rows_per_group,
                                                            };
                                                            let (row, failures) =
                                                                make_row(candidate, &profile, &cfg);
                                                            if failures.is_empty() {
                                                                accepted_total += 1;
                                                                accepted.push(row);
                                                                if accepted.len()
                                                                    > cfg
                                                                        .top
                                                                        .saturating_mul(4)
                                                                        .max(1024)
                                                                {
                                                                    truncate_rows(
                                                                        &mut accepted,
                                                                        cfg.top,
                                                                    );
                                                                }
                                                            } else {
                                                                for failure in failures {
                                                                    *rejected
                                                                        .entry(failure.to_string())
                                                                        .or_insert(0) += 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            truncate_rows(&mut accepted, cfg.top);
            (accepted, accepted_total, rejected)
        }));
    }

    let mut accepted = Vec::new();
    let mut accepted_total = 0_usize;
    let mut rejected = BTreeMap::new();
    for handle in handles {
        let (mut rows, worker_accepted_total, counts) =
            handle.join().expect("search worker panicked");
        accepted_total += worker_accepted_total;
        accepted.append(&mut rows);
        truncate_rows(&mut accepted, cfg.top);
        for (reason, count) in counts {
            *rejected.entry(reason).or_insert(0) += count;
        }
    }
    accepted.sort_by(row_cmp);
    accepted.truncate(cfg.top);
    (accepted, accepted_total, rejected, total_candidates)
}

fn run_anneal_search(
    cfg: &Config,
    profile: &Profile,
) -> (Vec<Row>, usize, BTreeMap<String, u64>, u64) {
    let total_candidates = cfg.anneal_iterations.saturating_mul(cfg.anneal_restarts);
    let restarts_per_thread = (cfg.anneal_restarts as usize + cfg.threads - 1) / cfg.threads;
    let mut handles = Vec::new();

    for thread_id in 0..cfg.threads {
        let start_restart = thread_id * restarts_per_thread;
        if start_restart >= cfg.anneal_restarts as usize {
            continue;
        }
        let end_restart = (start_restart + restarts_per_thread).min(cfg.anneal_restarts as usize);
        let cfg = cfg.clone();
        let profile = profile.clone();
        handles.push(thread::spawn(move || {
            let mut rng = Rng64::new(cfg.seed ^ ((thread_id as u64 + 1) * 0x9E3779B97F4A7C15));
            let mut accepted = Vec::new();
            let mut accepted_total = 0_usize;
            let mut rejected = BTreeMap::new();

            for _restart in start_restart..end_restart {
                let mut idx = random_indices(&cfg, &mut rng);
                let (mut current, failures) = make_row(idx.candidate(&cfg), &profile, &cfg);
                if failures.is_empty() {
                    accepted_total += 1;
                    accepted.push(current.clone());
                } else {
                    for failure in failures {
                        *rejected.entry(failure.to_string()).or_insert(0) += 1;
                    }
                    current.objective_score = f64::NEG_INFINITY;
                }

                for iter in 0..cfg.anneal_iterations {
                    let proposal_idx = if current.objective_score.is_finite() {
                        neighbor_indices(idx, &cfg, &mut rng)
                    } else {
                        random_indices(&cfg, &mut rng)
                    };
                    let (proposal, failures) =
                        make_row(proposal_idx.candidate(&cfg), &profile, &cfg);
                    if failures.is_empty() {
                        accepted_total += 1;
                        accepted.push(proposal.clone());
                        if accepted.len() > cfg.top.saturating_mul(4).max(1024) {
                            truncate_rows(&mut accepted, cfg.top);
                        }

                        let delta = proposal.objective_score - current.objective_score;
                        let temp = temperature(&cfg, iter);
                        let accept = delta >= 0.0 || rng.unit() < (delta / temp).exp();
                        if accept {
                            idx = proposal_idx;
                            current = proposal;
                        }
                    } else {
                        for failure in failures {
                            *rejected.entry(failure.to_string()).or_insert(0) += 1;
                        }
                    }
                }
            }
            truncate_rows(&mut accepted, cfg.top);
            (accepted, accepted_total, rejected)
        }));
    }

    let mut accepted = Vec::new();
    let mut accepted_total = 0_usize;
    let mut rejected = BTreeMap::new();
    for handle in handles {
        let (mut rows, worker_accepted_total, counts) =
            handle.join().expect("anneal worker panicked");
        accepted_total += worker_accepted_total;
        accepted.append(&mut rows);
        truncate_rows(&mut accepted, cfg.top);
        for (reason, count) in counts {
            *rejected.entry(reason).or_insert(0) += count;
        }
    }
    accepted.sort_by(row_cmp);
    accepted.truncate(cfg.top);
    (accepted, accepted_total, rejected, total_candidates)
}

fn markdown_report(
    profile: &Profile,
    cfg: &Config,
    accepted_top: &[Row],
    accepted_total: usize,
    rejected_count: u64,
) -> String {
    let objective_description = match cfg.objective.as_str() {
        "bandwidth" => {
            "The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension."
        }
        _ => {
            "The screening objective caps excess security credit and penalizes public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy."
        }
    };
    let mut lines = vec![
        format!("# Parameter Search Report: {}", profile.id),
        String::new(),
        "This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;".to_string(),
        "they are filters for deciding which candidates deserve exact projection, dual-witness,".to_string(),
        "and lattice/coding-estimator follow-up.".to_string(),
        objective_description.to_string(),
        String::new(),
        format!("- target security bits: `{}`", profile.target_security_bits),
        format!("- objective mode: `{}`", cfg.objective),
        format!("- proof strategies: `{}`", cfg.proof_strategies.join(", ")),
        format!(
            "- primal distance models: `{}`",
            cfg.primal_distance_models.join(", ")
        ),
        format!(
            "- q-ary amplitude strategies: `{}`",
            cfg.qary_amplitude_strategies.join(", ")
        ),
        format!(
            "- Lee repetitions: `{}`",
            cfg.lee_repetition_values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!(
            "- Lee rows per product block: `{}`",
            cfg.lee_rows_per_group_values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!(
            "- product-block sidecar manifests: `{}`",
            if cfg.product_block_sidecar_manifests.is_empty() {
                "none".to_string()
            } else {
                cfg.product_block_sidecar_manifests
                    .iter()
                    .map(|path| display_path(path))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        ),
        if cfg.m0_values.is_empty() {
            format!(
                "- m0 source: `n + overhead`, overheads `{}`",
                cfg.m0_overheads
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!(
                "- m0 source: direct values `{}`",
                cfg.m0_values
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        },
        if cfg.m1_values.is_empty() {
            format!(
                "- m1 source: `n * multiplier`, multipliers `{}`",
                cfg.m1_multipliers
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!(
                "- m1 source: direct values `{}`",
                cfg.m1_values
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        },
        format!(
            "- view coordinate counts: `{}`",
            cfg.view_coord_values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!(
            "- pk seed bytes: `{}`",
            cfg.pk_seed_byte_values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!("- security extra cap: `{:.1}` bits", cfg.security_extra_cap),
        format!(
            "- guard slack target: `{}`",
            effective_guard_slack_target(cfg, profile)
        ),
        format!(
            "- combined certification weight: `{:.3}`",
            cfg.combined_certification_weight
        ),
        format!("- accepted candidates before truncation: `{accepted_total}`"),
        format!("- candidates shown: `{}`", accepted_top.len()),
        format!("- rejected candidates: `{rejected_count}`"),
        String::new(),
        "| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |".to_string(),
        "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for row in accepted_top.iter().take(20) {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.1} | {:.1} | {} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} | {:.1} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.1} | {} |",
            row.candidate.proof_strategy,
            row.candidate.primal_distance_model,
            row.candidate.qary_amplitude_strategy,
            row.candidate.lee_repetition,
            row.candidate.q,
            row.candidate.n,
            row.candidate.m0,
            row.candidate.m1,
            row.candidate.view_len(),
            row.candidate.fiber_dim,
            row.candidate.eta,
            row.metrics.sample_encoding_bits,
            row.metrics.sample_compression_error,
            row.metrics.effective_eta,
            row.metrics.qary_amplitude_l1_required,
            row.metrics.qary_amplitude_l1_certified,
            row.metrics.qary_amplitude_minkowski_l1_upper,
            row.metrics.qary_amplitude_minkowski_margin,
            row.metrics.qary_amplitude_certificate_rows,
            row.metrics.qary_amplitude_keygen_mul_ops,
            row.objective_score,
            row.metrics.security_proxy_bits,
            row.metrics.primal_proxy_bits,
            row.metrics.dual_proxy_bits,
            row.metrics.self_randomization_proxy_bits,
            row.metrics.lossy_diagonal_required_distance,
            row.metrics.lossy_diagonal_slack_bits,
            row.metrics.setup_seed_public_bytes,
            row.metrics.cauchy_public_bytes,
            row.metrics.coefficient_public_bytes,
            row.metrics.incidence_public_bytes,
            row.metrics.diagonal_public_bytes,
            row.metrics.public_sample_bytes,
            row.metrics.compact_public_key_bytes,
            row.metrics.ciphertext_bytes,
            row.metrics.correctness_slack,
            row.metrics.guard_slack,
            row.metrics.combined_certification_pressure,
            row.metrics.combined_certification_margin,
        ));
    }
    lines.extend([
        String::new(),
        "## Next Checks".to_string(),
        String::new(),
        "1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy."
            .to_string(),
        "2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work."
            .to_string(),
        "3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors."
            .to_string(),
        "4. Keep only candidates whose proof-side constraints can be instantiated in Lean."
            .to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn generated_at() -> String {
    match std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    }
}

fn display_path(path: &Path) -> String {
    match env::current_dir()
        .ok()
        .and_then(|cwd| path.strip_prefix(cwd).ok().map(Path::to_path_buf))
    {
        Some(relative) => relative.display().to_string(),
        None => path.display().to_string(),
    }
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    let profile = load_profile(&cfg.profiles_path, &cfg.profile)?;
    let (accepted_top, accepted_total, rejected, total_candidates) = match cfg.mode.as_str() {
        "grid" => run_grid_search(&cfg, &profile),
        "anneal" => run_anneal_search(&cfg, &profile),
        other => return Err(format!("unknown --mode {other:?}; expected grid or anneal")),
    };
    let frontier = pareto_frontier(&accepted_top);

    let payload = Payload {
        generated_at: generated_at(),
        profile: profile.clone(),
        search_space: SearchSpace {
            mode: cfg.mode.clone(),
            objective: cfg.objective.clone(),
            proof_strategies: cfg.proof_strategies.clone(),
            primal_distance_models: cfg.primal_distance_models.clone(),
            qary_amplitude_strategies: cfg.qary_amplitude_strategies.clone(),
            lee_repetition_values: cfg.lee_repetition_values.clone(),
            lee_rows_per_group_values: cfg.lee_rows_per_group_values.clone(),
            product_block_sidecar_manifests: cfg
                .product_block_sidecar_manifests
                .iter()
                .map(|path| display_path(path))
                .collect(),
            product_block_sidecar_choices: cfg.product_block_choices.clone(),
            public_key_model: cfg.public_key_model.clone(),
            coefficient_model: cfg.coefficient_model.clone(),
            incidence_model: cfg.incidence_model.clone(),
            q_values: cfg.q_values.clone(),
            n_values: cfg.n_values.clone(),
            m0_overheads: cfg.m0_overheads.clone(),
            m0_values: cfg.m0_values.clone(),
            m1_multipliers: cfg.m1_multipliers.clone(),
            m1_values: cfg.m1_values.clone(),
            fiber_dims: cfg.fiber_dims.clone(),
            eta_values: cfg.eta_values.clone(),
            sample_bits_values: cfg.sample_bits_values.clone(),
            view_coord_values: cfg.view_coord_values.clone(),
            pk_seed_byte_values: cfg.pk_seed_byte_values.clone(),
            total_candidates,
            threads: cfg.threads,
            anneal_iterations: (cfg.mode == "anneal").then_some(cfg.anneal_iterations),
            anneal_restarts: (cfg.mode == "anneal").then_some(cfg.anneal_restarts),
            seed: (cfg.mode == "anneal").then_some(cfg.seed),
            security_extra_cap: cfg.security_extra_cap,
            guard_slack_target: cfg.guard_slack_target,
            combined_certification_weight: cfg.combined_certification_weight,
        },
        rejected_by_reason: rejected,
        accepted_total,
        accepted_top: accepted_top.clone(),
        pareto_frontier: frontier.clone(),
        notes: vec![
            "security_proxy_bits and objective_score are screening scores, not cryptanalytic estimates".to_string(),
            format!("objective mode {:?} caps excess security credit at {} bits", cfg.objective, cfg.security_extra_cap),
            format!("proof strategies searched: {}", cfg.proof_strategies.join(", ")),
            format!("primal distance models searched: {}", cfg.primal_distance_models.join(", ")),
            format!("q-ary amplitude strategies searched: {}", cfg.qary_amplitude_strategies.join(", ")),
            "solved-repetition certificates require lee_repetition * m1 <= n and certify only lee_repetition + 1, so they expose the direct-coordinate repetition tradeoff rather than assuming it is viable".to_string(),
            "direct-solved-lee treats lee_repetition as a hypothetical proof-carrying systematic graph Lee distance and prices dense solved-map keygen as n*m1 multiply-adds".to_string(),
            "moment-lee treats lee_repetition as a proof-carrying shifted/scaled moment graph-code Lee distance and prices evaluation keygen as n*m1 multiply-adds".to_string(),
            format!("combined-proxy rows are ranked with certification pressure penalty weight {}", cfg.combined_certification_weight),
            "sample_bits=0 means full-width public samples; smaller sample_bits applies bit-packing plus a conservative centered rounding-error penalty".to_string(),
            "view_coords=0 means all m0+m1 coordinates; smaller values model public-sample coordinate views that require support-covered or lossy-budget proof obligations".to_string(),
            "guard-only uses the proven guard-block distance and requires m0>=n; combined-proxy prices distance from the viewed combined matrix and requires a later combined-distance certificate".to_string(),
            "combined_certification_pressure is a cheap proxy for how much rank/distance work the later combined certificate must do; lower is easier to certify".to_string(),
            "lossy_diagonal_slack_bits is a screening proxy for the multi-coordinate diagonal amplification proof route; positive slack is not a Lean certificate by itself".to_string(),
            "nonzero sample_bits applies deterministic public-sample compression; security is intended to follow by data processing from the uncompressed sample, while correctness and amplitude checks use effective_eta".to_string(),
            "hard filters encode current proof-side constraints such as q > n + m0 + m1 and m0 >= n".to_string(),
            "top candidates require exact finite-field and lattice/coding follow-up before any security claim".to_string(),
        ],
    };

    fs::create_dir_all(&cfg.output_dir)
        .map_err(|e| format!("create {}: {e}", cfg.output_dir.display()))?;
    let json_path = cfg.output_dir.join(format!("{}.json", profile.id));
    let md_path = cfg.output_dir.join(format!("{}.md", profile.id));
    let json =
        serde_json::to_string_pretty(&payload).map_err(|e| format!("serialize json: {e}"))?;
    fs::write(&json_path, format!("{json}\n"))
        .map_err(|e| format!("write {}: {e}", json_path.display()))?;
    fs::write(
        &md_path,
        markdown_report(
            &profile,
            &cfg,
            &accepted_top,
            accepted_total,
            total_candidates - accepted_total as u64,
        ),
    )
    .map_err(|e| format!("write {}: {e}", md_path.display()))?;

    println!("wrote {}", display_path(&json_path));
    println!("wrote {}", display_path(&md_path));
    println!(
        "accepted_total={} accepted_top={} pareto={} total={}",
        accepted_total,
        accepted_top.len(),
        frontier.len(),
        total_candidates
    );
    Ok(())
}
