use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Candidate {
    #[serde(default = "default_proof_strategy")]
    proof_strategy: String,
    #[serde(default = "default_primal_distance_model")]
    primal_distance_model: String,
    q: u64,
    n: u64,
    m0: u64,
    m1: u64,
    fiber_dim: u64,
    eta: u64,
    #[serde(default)]
    sample_bits: u64,
    #[serde(default)]
    view_coords: u64,
    #[serde(default)]
    pk_seed_bytes: u64,
    #[serde(default = "default_qary_amplitude_strategy")]
    qary_amplitude_strategy: String,
    #[serde(default = "default_lee_repetition")]
    lee_repetition: u64,
    #[serde(default = "default_lee_rows_per_group")]
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

fn default_proof_strategy() -> String {
    "full-surjective".to_string()
}

fn default_primal_distance_model() -> String {
    "guard-only".to_string()
}

fn default_qary_amplitude_strategy() -> String {
    "fallback-support".to_string()
}

fn default_lee_repetition() -> u64 {
    1
}

fn default_lee_rows_per_group() -> u64 {
    1
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Metrics {
    compact_public_key_bytes: u64,
    ciphertext_bytes: u64,
    guard_distance: i64,
    guard_slack: i64,
    cauchy_field_slack: i64,
    #[serde(default)]
    sample_compression_error: u64,
    #[serde(default)]
    effective_eta: u64,
    #[serde(default)]
    qary_amplitude_l1_required: Option<u64>,
    #[serde(default)]
    qary_amplitude_l1_certified: Option<u64>,
    #[serde(default)]
    qary_amplitude_at_noise_pass: Option<bool>,
    #[serde(default)]
    qary_amplitude_certificate_rows: Option<u64>,
    #[serde(default)]
    qary_amplitude_keygen_mul_ops: Option<u64>,
    #[serde(default)]
    qary_amplitude_product_block_group_size: u64,
    #[serde(default)]
    qary_amplitude_product_block_group_count: u64,
    #[serde(default)]
    qary_amplitude_product_block_rows_per_group: u64,
    #[serde(default)]
    qary_amplitude_product_block_used_rows: u64,
    #[serde(default)]
    qary_amplitude_certificate_route: String,
    #[serde(default)]
    lossy_diagonal_required_distance: u64,
    #[serde(default)]
    lossy_diagonal_slack_bits: f64,
    security_proxy_bits: f64,
    average_to_worst_slack: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Row {
    candidate: Candidate,
    metrics: Metrics,
    objective_score: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Profile {
    id: String,
    target_security_bits: u64,
    min_guard_slack: i64,
    min_average_to_worst_slack: i64,
}

#[derive(Debug, Deserialize)]
struct Payload {
    profile: Profile,
    accepted_top: Vec<Row>,
    pareto_frontier: Vec<Row>,
}

#[derive(Debug, Serialize)]
struct ValidationPayload {
    generated_at: String,
    validation_model: String,
    source_files: Vec<String>,
    top_per_profile: usize,
    rows_checked: usize,
    summary: ValidationSummary,
    rows: Vec<ValidationRow>,
    notes: Vec<String>,
}

#[derive(Default, Debug, Serialize)]
struct ValidationSummary {
    by_profile: BTreeMap<String, ProfileSummary>,
    failures_by_reason: BTreeMap<String, u64>,
}

#[derive(Default, Debug, Serialize)]
struct ProfileSummary {
    rows_checked: u64,
    rows_passed: u64,
    min_ciphertext_bytes: Option<u64>,
    min_public_key_bytes: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
struct ValidationRow {
    profile: String,
    source: String,
    candidate: Candidate,
    reported: Metrics,
    generated_setup: GeneratedSetupSummary,
    exact: ExactDiagnostics,
    verdict: String,
    failures: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct GeneratedSetupSummary {
    coefficient_seed: u64,
    incidence_seed: u64,
    diagonal_seed: u64,
    tilt_columns: u64,
    tilt_max_column_weight: usize,
    tilt_min_column_weight: usize,
    tilt_rank: usize,
    tilted_adjoint_rank: usize,
    dual_search_weight: usize,
    no_dual_witness_up_to: usize,
}

#[derive(Clone, Debug, Serialize)]
struct ExactDiagnostics {
    q_is_prime: bool,
    field_size_pass: bool,
    cauchy_full_rank: usize,
    cauchy_guard_rank: usize,
    full_column_rank: bool,
    guard_column_rank: bool,
    tilted_adjoint_full_rank: bool,
    tilt_column_support_pass: bool,
    bounded_dual_witness_pass: bool,
    guard_distance_exact: i64,
    measurement_distance_exact: i64,
    dual_distance_exact: i64,
    qary_amplitude_l1_required: u64,
    qary_amplitude_l1_certified: u64,
    qary_amplitude_at_noise_pass: bool,
    qary_amplitude_view_rank: usize,
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
    shared_pair_global_certificate_pass: bool,
    shared_pair_certificate_source: String,
    shared_pair_certificate_used_rows: u64,
    shared_pair_certificate_checked_pairs: u64,
    correctness_required_distance: i64,
    correctness_distance_exact: i64,
    guard_distance_pass: bool,
    correctness_distance_pass: bool,
    combined_primal_distance_certificate_pass: bool,
    full_view: bool,
    view_coordinate_count: u64,
    view_coordinate_bound_pass: bool,
    proof_strategy_view_pass: bool,
    view_contains_guard_block: bool,
    primal_distance_view_pass: bool,
    view_compression_ratio: f64,
    support_covered_candidate_pass: bool,
    lossy_budget_noise_budget_pass: bool,
    lossy_budget_separation_proxy_pass: bool,
    lossy_diagonal_required_distance: u64,
    lossy_diagonal_slack_bits: f64,
    lossy_diagonal_proxy_pass: bool,
    view_pair_coordinate_min_support: i64,
    view_pair_coordinate_certificate_pass: bool,
    lossy_budget_candidate_pass: bool,
    view_certificate_route: String,
    view_reduction_certificate_pass: bool,
    average_to_worst_pass: bool,
    security_proxy_pass: bool,
    reported_guard_distance_matches: bool,
}

#[derive(Clone, Debug)]
struct Config {
    input_dir: PathBuf,
    output_dir: PathBuf,
    top: usize,
    include_pareto: bool,
    dual_search_weight: usize,
    shared_lee_certs: Vec<SharedLeeCertificate>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("research/parameter_search/bandwidth_seeded"),
            output_dir: PathBuf::from(
                "research/parameter_search/bandwidth_seeded_expanded_validation",
            ),
            top: 32,
            include_pareto: true,
            dual_search_weight: 2,
            shared_lee_certs: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct SharedLeePairBlockCertificate {
    #[serde(default)]
    checked_centered_pair_count: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct SharedLeeCertificate {
    #[serde(default)]
    source: String,
    q: u64,
    n: u64,
    m1: u64,
    #[serde(default)]
    group_size: u64,
    #[serde(default)]
    used_rows: u64,
    target_l1: u64,
    #[serde(default)]
    passes_target: bool,
    #[serde(default)]
    certified_l1: u64,
    #[serde(default)]
    global_systematic_lee_distance_certified: bool,
    #[serde(default)]
    pair_block: Option<SharedLeePairBlockCertificate>,
}

#[derive(Clone, Debug, Deserialize)]
struct SharedLeeManifest {
    entries: Vec<SharedLeeManifestEntry>,
}

#[derive(Clone, Debug, Deserialize)]
struct SharedLeeManifestEntry {
    #[serde(default)]
    passes_target: bool,
    #[serde(default)]
    sidecar_json: Option<String>,
}

fn load_shared_lee_cert(path: &Path) -> Result<SharedLeeCertificate, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut cert: SharedLeeCertificate =
        serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    cert.source = display_path(path);
    Ok(cert)
}

fn load_shared_lee_cert_dir(path: &Path) -> Result<Vec<SharedLeeCertificate>, String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| format!("read dir {}: {e}", path.display()))? {
        let entry = entry.map_err(|e| format!("read dir entry {}: {e}", path.display()))?;
        let file_path = entry.path();
        if !file_path.is_file() {
            continue;
        }
        if file_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("manifest") && name.ends_with(".json"))
        {
            continue;
        }
        if file_path.extension().is_some_and(|ext| ext == "json") {
            files.push(file_path);
        }
    }
    files.sort();
    files
        .iter()
        .map(|file| load_shared_lee_cert(file))
        .collect()
}

fn load_shared_lee_cert_manifest(path: &Path) -> Result<Vec<SharedLeeCertificate>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let manifest: SharedLeeManifest =
        serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut certs = Vec::new();
    for entry in manifest.entries {
        if !entry.passes_target {
            continue;
        }
        let Some(sidecar_json) = entry.sidecar_json else {
            continue;
        };
        certs.push(load_shared_lee_cert(Path::new(&sidecar_json))?);
    }
    Ok(certs)
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
            "--input-dir" => cfg.input_dir = PathBuf::from(value(&mut args, "--input-dir")?),
            "--output-dir" => cfg.output_dir = PathBuf::from(value(&mut args, "--output-dir")?),
            "--top" => {
                cfg.top = value(&mut args, "--top")?
                    .parse()
                    .map_err(|_| "invalid --top")?
            }
            "--dual-search-weight" => {
                cfg.dual_search_weight = value(&mut args, "--dual-search-weight")?
                    .parse()
                    .map_err(|_| "invalid --dual-search-weight")?
            }
            "--shared-lee-cert-json" => {
                let path = PathBuf::from(value(&mut args, "--shared-lee-cert-json")?);
                cfg.shared_lee_certs.push(load_shared_lee_cert(&path)?);
            }
            "--shared-lee-cert-dir" => {
                let path = PathBuf::from(value(&mut args, "--shared-lee-cert-dir")?);
                cfg.shared_lee_certs
                    .extend(load_shared_lee_cert_dir(&path)?);
            }
            "--shared-lee-cert-manifest" => {
                let path = PathBuf::from(value(&mut args, "--shared-lee-cert-manifest")?);
                cfg.shared_lee_certs
                    .extend(load_shared_lee_cert_manifest(&path)?);
            }
            "--no-pareto" => cfg.include_pareto = false,
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
        "Descender exact frontier validator\n\
         \n\
         Options:\n\
           --input-dir PATH\n\
           --output-dir PATH\n\
           --top N\n\
           --dual-search-weight N\n\
           --shared-lee-cert-json PATH   exact global shared-pair Lee certificate; repeatable\n\
           --shared-lee-cert-dir PATH    load every sidecar JSON in a directory\n\
           --shared-lee-cert-manifest PATH load sidecars referenced by a manifest\n\
           --no-pareto"
    );
}

fn generated_at() -> String {
    match std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
    {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "unknown".to_string(),
    }
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

fn qary_dual_minkowski_l1_upper(q: u64, view_len: u64, rank: u64) -> f64 {
    if view_len == 0 {
        return 0.0;
    }
    ((log_factorial(view_len) + rank.min(view_len) as f64 * (q as f64).ln()) / view_len as f64)
        .exp()
}

fn qary_amplitude_l1_certified(c: &Candidate, fallback: u64) -> u64 {
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => fallback,
        "solved-repetition" => c.lee_repetition.saturating_add(1),
        "direct-solved-lee" | "shared-pair-lee" | "product-block-lee" | "moment-lee" => {
            c.lee_repetition
        }
        _ => fallback,
    }
}

fn qary_amplitude_certificate_rows(c: &Candidate, fallback: u64) -> u64 {
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => fallback,
        "solved-repetition" => c.lee_repetition.saturating_mul(c.m1),
        "direct-solved-lee" | "moment-lee" => c.n,
        "shared-pair-lee" | "product-block-lee" => product_block_used_rows(c),
        _ => fallback,
    }
}

fn qary_amplitude_keygen_mul_ops(c: &Candidate) -> u64 {
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => 0,
        "solved-repetition" => c.n.saturating_mul(c.m1).saturating_mul(c.lee_repetition),
        "direct-solved-lee" | "moment-lee" => c.n.saturating_mul(c.m1),
        "shared-pair-lee" | "product-block-lee" => 2u64.saturating_mul(product_block_used_rows(c)),
        _ => 0,
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

fn matching_shared_lee_cert<'a>(
    c: &Candidate,
    cfg: &'a Config,
) -> Option<&'a SharedLeeCertificate> {
    if !matches!(
        c.qary_amplitude_strategy.as_str(),
        "shared-pair-lee" | "product-block-lee"
    ) {
        return None;
    }
    cfg.shared_lee_certs.iter().find(|cert| {
        cert.q == c.q
            && cert.m1 == c.m1
            && cert.n <= c.n
            && cert.group_size == 2
            && cert.used_rows == product_block_used_rows(c)
            && cert.target_l1 >= c.lee_repetition
            && cert.certified_l1 >= c.lee_repetition
            && cert.passes_target
            && cert.global_systematic_lee_distance_certified
    })
}

fn guard_block_lossy_diagonal_certificate_pass(
    c: &Candidate,
    q_is_prime: bool,
    view_contains_guard_block: bool,
    lossy_budget_candidate_pass: bool,
    lossy_diagonal_proxy_pass: bool,
    correctness_distance_exact: i64,
    lossy_diagonal_required_distance: u64,
) -> bool {
    c.proof_strategy == "lossy-budget"
        && matches!(
            c.primal_distance_model.as_str(),
            "guard-only" | "combined-proxy"
        )
        && q_is_prime
        && view_contains_guard_block
        && lossy_budget_candidate_pass
        && lossy_diagonal_proxy_pass
        && correctness_distance_exact >= 0
        && lossy_diagonal_required_distance != u64::MAX
        && correctness_distance_exact as u64 >= lossy_diagonal_required_distance
}

fn coordinate_subview_lee_dual_restriction_pass(
    c: &Candidate,
    full_view: bool,
    view_coordinate_bound_pass: bool,
    view_contains_guard_block: bool,
    correctness_distance_pass: bool,
    shared_pair_global_certificate_pass: bool,
) -> bool {
    c.proof_strategy == "lossy-budget"
        && !full_view
        && c.sample_bits == 0
        && view_coordinate_bound_pass
        && view_contains_guard_block
        && correctness_distance_pass
        && shared_pair_global_certificate_pass
}

fn mod_pow(mut base: u64, mut exp: u64, q: u64) -> u64 {
    let mut acc = 1_u64;
    base %= q;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = ((acc as u128 * base as u128) % q as u128) as u64;
        }
        base = ((base as u128 * base as u128) % q as u128) as u64;
        exp >>= 1;
    }
    acc
}

fn mod_inv(x: u64, q: u64) -> u64 {
    mod_pow(x, q - 2, q)
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

    fn modulo(&mut self, modulus: u64) -> u64 {
        self.next_u64() % modulus
    }

    fn nonzero_mod(&mut self, q: u64) -> u64 {
        1 + self.modulo(q - 1)
    }
}

fn mix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

fn candidate_seed(profile: &str, candidate: &Candidate, domain: u64) -> u64 {
    let mut h = 0xD35CE5DE_5EED_0001_u64 ^ domain;
    for byte in profile.bytes() {
        h = mix64(h ^ byte as u64);
    }
    for value in [
        candidate.q,
        candidate.n,
        candidate.m0,
        candidate.m1,
        candidate.fiber_dim,
        candidate.eta,
        candidate.sample_bits,
    ] {
        h = mix64(h ^ value);
    }
    h
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3_u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn cauchy_matrix(rows: usize, cols: usize, q: u64) -> Vec<Vec<u64>> {
    let mut matrix = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for j in 0..cols {
            let x = i as u64;
            let y = rows as u64 + j as u64;
            let denom = (x + q - y % q) % q;
            row.push(mod_inv(denom, q));
        }
        matrix.push(row);
    }
    matrix
}

fn cauchy_block_matrix(
    n: usize,
    cols: usize,
    q: u64,
    row_offset: usize,
    col_offset: usize,
) -> Vec<Vec<u64>> {
    let mut matrix = vec![vec![0; cols]; n];
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let x = (row_offset + i) as u64;
            let y = (col_offset + n + j) as u64;
            let denom = (x + q - y % q) % q;
            *cell = mod_inv(denom, q);
        }
    }
    matrix
}

fn horizontal_concat(left: &[Vec<u64>], right: &[Vec<u64>]) -> Vec<Vec<u64>> {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| {
            let mut row = Vec::with_capacity(l.len() + r.len());
            row.extend_from_slice(l);
            row.extend_from_slice(r);
            row
        })
        .collect()
}

fn generated_tilt_matrix(
    profile: &str,
    candidate: &Candidate,
) -> (Vec<Vec<u64>>, GeneratedSetupSummary) {
    let n = candidate.n as usize;
    let m1 = candidate.m1 as usize;
    let q = candidate.q;
    let coefficient_seed = candidate_seed(profile, candidate, 0xC0EFF1C1E_u64);
    let incidence_seed = candidate_seed(profile, candidate, 0x1AC1D3CE_u64);
    let diagonal_seed = candidate_seed(profile, candidate, 0xD1A60A1_u64);
    let mut incidence_rng = Rng64::new(incidence_seed);
    let mut coefficient_rng = Rng64::new(coefficient_seed);
    let mut diagonal_rng = Rng64::new(diagonal_seed);
    let mut matrix = vec![vec![0; m1]; n];
    let mut min_weight = usize::MAX;
    let mut max_weight = 0_usize;

    for col in 0..m1 {
        let a = incidence_rng.modulo(candidate.n) as usize;
        let mut b = incidence_rng.modulo(candidate.n) as usize;
        while b == a {
            b = incidence_rng.modulo(candidate.n) as usize;
        }
        let mut c = incidence_rng.modulo(candidate.n) as usize;
        while c == a || c == b {
            c = incidence_rng.modulo(candidate.n) as usize;
        }

        let diag = diagonal_rng.nonzero_mod(q);
        let x = coefficient_rng.nonzero_mod(q);
        let y = coefficient_rng.nonzero_mod(q);
        let mut center = (q - ((x + y) % q)) % q;
        while center == 0 {
            let y_new = coefficient_rng.nonzero_mod(q);
            center = (q - ((x + y_new) % q)) % q;
            if center != 0 {
                matrix[c][col] = ((diag as u128 * y_new as u128) % q as u128) as u64;
                break;
            }
        }
        if matrix[c][col] == 0 {
            matrix[c][col] = ((diag as u128 * y as u128) % q as u128) as u64;
        }
        matrix[a][col] = ((diag as u128 * center as u128) % q as u128) as u64;
        matrix[b][col] = ((diag as u128 * x as u128) % q as u128) as u64;

        let weight = [matrix[a][col], matrix[b][col], matrix[c][col]]
            .into_iter()
            .filter(|&v| v != 0)
            .count();
        min_weight = min_weight.min(weight);
        max_weight = max_weight.max(weight);
    }

    let tilt_rank = rank_mod(matrix.clone(), q);
    (
        matrix,
        GeneratedSetupSummary {
            coefficient_seed,
            incidence_seed,
            diagonal_seed,
            tilt_columns: candidate.m1,
            tilt_max_column_weight: max_weight,
            tilt_min_column_weight: min_weight,
            tilt_rank,
            tilted_adjoint_rank: 0,
            dual_search_weight: 0,
            no_dual_witness_up_to: 0,
        },
    )
}

fn rank_mod(mut matrix: Vec<Vec<u64>>, q: u64) -> usize {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rank = 0_usize;
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&r| matrix[r][col] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let inv = mod_inv(matrix[rank][col], q);
        for c in col..cols {
            matrix[rank][c] = ((matrix[rank][c] as u128 * inv as u128) % q as u128) as u64;
        }
        for r in 0..rows {
            if r == rank || matrix[r][col] == 0 {
                continue;
            }
            let factor = matrix[r][col];
            for c in col..cols {
                let subtract = ((factor as u128 * matrix[rank][c] as u128) % q as u128) as u64;
                matrix[r][c] = (matrix[r][c] + q - subtract) % q;
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn selected_columns(matrix: &[Vec<u64>], cols: &[usize]) -> Vec<Vec<u64>> {
    matrix
        .iter()
        .map(|row| cols.iter().map(|&col| row[col]).collect())
        .collect()
}

fn has_dependent_support_of_size(
    matrix: &[Vec<u64>],
    q: u64,
    size: usize,
    start: usize,
    selected: &mut Vec<usize>,
) -> bool {
    if selected.len() == size {
        return rank_mod(selected_columns(matrix, selected), q) < size;
    }
    let cols = matrix.first().map_or(0, Vec::len);
    let remaining = size - selected.len();
    if cols < remaining || start > cols - remaining {
        return false;
    }
    for col in start..=cols - remaining {
        selected.push(col);
        if has_dependent_support_of_size(matrix, q, size, col + 1, selected) {
            return true;
        }
        selected.pop();
    }
    false
}

fn no_dual_witness_up_to(matrix: &[Vec<u64>], q: u64, limit: usize) -> usize {
    let cols = matrix.first().map_or(0, Vec::len);
    let capped = limit.min(cols);
    for size in 1..=capped {
        let mut selected = Vec::with_capacity(size);
        if has_dependent_support_of_size(matrix, q, size, 0, &mut selected) {
            return size - 1;
        }
    }
    capped
}

fn validate_row(profile: &Profile, source: &str, row: &Row, cfg: &Config) -> ValidationRow {
    let c = &row.candidate;
    let q_is_prime = is_prime(c.q);
    let field_size_pass = c.q > c.n + c.m();
    let full_matrix = if q_is_prime && field_size_pass {
        cauchy_matrix(c.m() as usize, c.n as usize, c.q)
    } else {
        Vec::new()
    };
    let guard_matrix = if q_is_prime && field_size_pass {
        cauchy_matrix(c.m0 as usize, c.n as usize, c.q)
    } else {
        Vec::new()
    };
    let full_rank = if full_matrix.is_empty() {
        0
    } else {
        rank_mod(full_matrix, c.q)
    };
    let guard_rank = if guard_matrix.is_empty() {
        0
    } else {
        rank_mod(guard_matrix, c.q)
    };
    let (tilt, mut generated_setup) = if q_is_prime && field_size_pass {
        generated_tilt_matrix(&profile.id, c)
    } else {
        (
            Vec::new(),
            GeneratedSetupSummary {
                coefficient_seed: 0,
                incidence_seed: 0,
                diagonal_seed: 0,
                tilt_columns: c.m1,
                tilt_max_column_weight: 0,
                tilt_min_column_weight: 0,
                tilt_rank: 0,
                tilted_adjoint_rank: 0,
                dual_search_weight: cfg.dual_search_weight,
                no_dual_witness_up_to: 0,
            },
        )
    };
    let tilted_adjoint = if q_is_prime && field_size_pass {
        let b0 = cauchy_block_matrix(c.n as usize, c.m0 as usize, c.q, 0, 0);
        let mut b1 = cauchy_block_matrix(c.n as usize, c.m1 as usize, c.q, 0, c.m0 as usize);
        for i in 0..b1.len() {
            for j in 0..b1[i].len() {
                b1[i][j] = (b1[i][j] + tilt[i][j]) % c.q;
            }
        }
        horizontal_concat(&b0, &b1)
    } else {
        Vec::new()
    };
    let tilted_adjoint_rank = if tilted_adjoint.is_empty() {
        0
    } else {
        rank_mod(tilted_adjoint.clone(), c.q)
    };
    let no_dual_witness_up_to = if tilted_adjoint.is_empty() {
        0
    } else {
        no_dual_witness_up_to(&tilted_adjoint, c.q, cfg.dual_search_weight)
    };
    let qary_view_len = c.view_len().min(c.m()) as usize;
    let qary_view_rank = if tilted_adjoint.is_empty() || qary_view_len == 0 {
        0
    } else {
        let view_adjoint: Vec<Vec<u64>> = tilted_adjoint
            .iter()
            .map(|row| row[..qary_view_len].to_vec())
            .collect();
        rank_mod(view_adjoint, c.q)
    };
    generated_setup.tilted_adjoint_rank = tilted_adjoint_rank;
    generated_setup.dual_search_weight = cfg.dual_search_weight;
    generated_setup.no_dual_witness_up_to = no_dual_witness_up_to;

    let guard_distance_exact = c.m0 as i64 - c.n as i64 + 1;
    let measurement_distance_exact = c.m() as i64 - c.n as i64 + 1;
    let dual_distance_exact = c.n as i64 + 1;
    let effective_eta = row.metrics.effective_eta.max(c.eta);
    let qary_amplitude_l1_required = row
        .metrics
        .qary_amplitude_l1_required
        .unwrap_or_else(|| qary_amplitude_l1_required(c.q, effective_eta));
    let qary_amplitude_l1_certified = row
        .metrics
        .qary_amplitude_l1_certified
        .unwrap_or_else(|| qary_amplitude_l1_certified(c, dual_distance_exact.max(0) as u64));
    let qary_amplitude_at_noise_pass = row
        .metrics
        .qary_amplitude_at_noise_pass
        .unwrap_or(qary_amplitude_l1_certified >= qary_amplitude_l1_required);
    let qary_amplitude_minkowski_l1_upper =
        qary_dual_minkowski_l1_upper(c.q, c.view_len(), qary_view_rank as u64);
    let qary_amplitude_minkowski_margin =
        qary_amplitude_minkowski_l1_upper - qary_amplitude_l1_required as f64;
    let qary_amplitude_geometry_possible = qary_amplitude_minkowski_margin >= 0.0;
    let qary_amplitude_certificate_rows = row
        .metrics
        .qary_amplitude_certificate_rows
        .unwrap_or_else(|| qary_amplitude_certificate_rows(c, c.n));
    let qary_amplitude_keygen_mul_ops = row
        .metrics
        .qary_amplitude_keygen_mul_ops
        .unwrap_or_else(|| qary_amplitude_keygen_mul_ops(c));
    let qary_amplitude_product_block_group_size =
        row.metrics.qary_amplitude_product_block_group_size.max(product_block_group_size(c));
    let qary_amplitude_product_block_group_count =
        row.metrics.qary_amplitude_product_block_group_count.max(product_block_group_count(c));
    let qary_amplitude_product_block_rows_per_group = row
        .metrics
        .qary_amplitude_product_block_rows_per_group
        .max(if is_product_block_lee(c) { c.lee_rows_per_group } else { 0 });
    let qary_amplitude_product_block_used_rows =
        row.metrics.qary_amplitude_product_block_used_rows.max(product_block_used_rows(c));
    let qary_amplitude_certificate_route = if row.metrics.qary_amplitude_certificate_route.is_empty() {
        qary_amplitude_certificate_route(c)
    } else {
        row.metrics.qary_amplitude_certificate_route.clone()
    };
    let shared_pair_cert = matching_shared_lee_cert(c, cfg);
    let shared_pair_global_certificate_pass = !matches!(
        c.qary_amplitude_strategy.as_str(),
        "shared-pair-lee" | "product-block-lee"
    )
        || shared_pair_cert.is_some();
    let shared_pair_certificate_source = shared_pair_cert
        .map(|cert| cert.source.clone())
        .unwrap_or_default();
    let shared_pair_certificate_used_rows =
        shared_pair_cert.map(|cert| cert.used_rows).unwrap_or_default();
    let shared_pair_certificate_checked_pairs = shared_pair_cert
        .and_then(|cert| cert.pair_block.as_ref())
        .map(|pair| pair.checked_centered_pair_count)
        .unwrap_or_default();
    let correctness_required_distance = 2 * effective_eta as i64 + 1;
    let correctness_distance_exact = match c.primal_distance_model.as_str() {
        "guard-only" => guard_distance_exact,
        "combined-proxy" => c.view_len() as i64 - c.n as i64 + 1,
        _ => guard_distance_exact,
    };
    let guard_distance_pass = guard_distance_exact >= correctness_required_distance
        && row.metrics.guard_slack >= profile.min_guard_slack;
    let correctness_distance_pass = correctness_distance_exact >= correctness_required_distance
        && row.metrics.guard_slack >= profile.min_guard_slack;
    let full_view = c.view_len() == c.m();
    let view_coordinate_bound_pass = c.view_len() <= c.m();
    let proof_strategy_view_pass = c.proof_strategy != "full-surjective" || full_view;
    let view_contains_guard_block = c.view_len() >= c.m0;
    let combined_primal_distance_certificate_pass = match c.primal_distance_model.as_str() {
        "guard-only" => true,
        "combined-proxy" => view_contains_guard_block && guard_distance_pass,
        _ => false,
    };
    let primal_distance_view_pass =
        c.primal_distance_model != "guard-only" || view_contains_guard_block;
    let view_compression_ratio = if c.m() == 0 {
        1.0
    } else {
        c.view_len() as f64 / c.m() as f64
    };
    let support_covered_candidate_pass =
        c.proof_strategy == "support-covered" && full_view && view_coordinate_bound_pass;
    let lossy_budget_noise_budget_pass =
        c.proof_strategy == "lossy-budget" && row.metrics.effective_eta >= c.eta;
    let lossy_budget_separation_proxy_pass =
        c.proof_strategy == "lossy-budget" && correctness_distance_pass;
    let lossy_diagonal_required_distance =
        lossy_diagonal_required_distance(c.q, c.n, effective_eta);
    let lossy_diagonal_slack_bits =
        lossy_diagonal_slack_bits(c.q, c.n, correctness_distance_exact, effective_eta);
    let lossy_diagonal_proxy_pass = c.proof_strategy == "lossy-budget"
        && lossy_diagonal_slack_bits >= 0.0
        && lossy_diagonal_required_distance != u64::MAX
        && correctness_distance_exact >= 0
        && correctness_distance_exact as u64 >= lossy_diagonal_required_distance;
    let lossy_budget_candidate_pass = lossy_budget_noise_budget_pass
        && lossy_budget_separation_proxy_pass
        && view_coordinate_bound_pass
        && proof_strategy_view_pass
        && primal_distance_view_pass;
    let guard_block_view_candidate_pass = matches!(
        c.primal_distance_model.as_str(),
        "guard-only" | "combined-proxy"
    ) && view_contains_guard_block
        && c.proof_strategy == "lossy-budget"
        && lossy_budget_candidate_pass;
    let view_pair_coordinate_min_support = if guard_block_view_candidate_pass {
        guard_distance_exact
    } else {
        0
    };
    let view_pair_coordinate_certificate_pass = guard_block_lossy_diagonal_certificate_pass(
        c,
        q_is_prime,
        view_contains_guard_block,
        lossy_budget_candidate_pass,
        lossy_diagonal_proxy_pass,
        view_pair_coordinate_min_support,
        lossy_diagonal_required_distance,
    );
    let coordinate_subview_lee_dual_restriction_pass =
        coordinate_subview_lee_dual_restriction_pass(
            c,
            full_view,
            view_coordinate_bound_pass,
            view_contains_guard_block,
            correctness_distance_pass,
            shared_pair_global_certificate_pass,
        );
    let view_certificate_route = if full_view {
        "full-view".to_string()
    } else if coordinate_subview_lee_dual_restriction_pass {
        "coordinate-subview-lee-dual-restricted".to_string()
    } else if view_pair_coordinate_certificate_pass && c.primal_distance_model == "combined-proxy" {
        "guard-block-combined-lossy-diagonal-certified".to_string()
    } else if view_pair_coordinate_certificate_pass {
        "guard-block-lossy-diagonal-certified".to_string()
    } else if guard_block_view_candidate_pass && lossy_diagonal_proxy_pass {
        "guard-block-lossy-diagonal-needs-certificate".to_string()
    } else if guard_block_view_candidate_pass {
        "guard-block-lossy-candidate".to_string()
    } else if c.proof_strategy == "support-covered" {
        "support-covered-needs-hitting-set".to_string()
    } else if c.proof_strategy == "lossy-budget"
        && lossy_budget_candidate_pass
        && lossy_diagonal_proxy_pass
    {
        "lossy-diagonal-candidate".to_string()
    } else if c.proof_strategy == "lossy-budget" && lossy_budget_candidate_pass {
        "lossy-budget-candidate".to_string()
    } else if c.proof_strategy == "lossy-budget" {
        "lossy-budget-needs-noise-or-separation".to_string()
    } else {
        "unsupported-compressed-view".to_string()
    };
    let view_reduction_certificate_pass = full_view
        || view_pair_coordinate_certificate_pass
        || coordinate_subview_lee_dual_restriction_pass;

    let exact = ExactDiagnostics {
        q_is_prime,
        field_size_pass,
        cauchy_full_rank: full_rank,
        cauchy_guard_rank: guard_rank,
        full_column_rank: full_rank == c.n as usize,
        guard_column_rank: guard_rank == c.n as usize,
        tilted_adjoint_full_rank: tilted_adjoint_rank == c.n as usize,
        tilt_column_support_pass: generated_setup.tilt_min_column_weight == 3
            && generated_setup.tilt_max_column_weight <= 3,
        bounded_dual_witness_pass: no_dual_witness_up_to >= cfg.dual_search_weight,
        guard_distance_exact,
        measurement_distance_exact,
        dual_distance_exact,
        qary_amplitude_l1_required,
        qary_amplitude_l1_certified,
        qary_amplitude_at_noise_pass,
        qary_amplitude_view_rank: qary_view_rank,
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
        shared_pair_global_certificate_pass,
        shared_pair_certificate_source,
        shared_pair_certificate_used_rows,
        shared_pair_certificate_checked_pairs,
        correctness_required_distance,
        correctness_distance_exact,
        guard_distance_pass,
        correctness_distance_pass,
        combined_primal_distance_certificate_pass,
        full_view,
        view_coordinate_count: c.view_len(),
        view_coordinate_bound_pass,
        proof_strategy_view_pass,
        view_contains_guard_block,
        primal_distance_view_pass,
        view_compression_ratio,
        support_covered_candidate_pass,
        lossy_budget_noise_budget_pass,
        lossy_budget_separation_proxy_pass,
        lossy_diagonal_required_distance,
        lossy_diagonal_slack_bits,
        lossy_diagonal_proxy_pass,
        view_pair_coordinate_min_support,
        view_pair_coordinate_certificate_pass,
        lossy_budget_candidate_pass,
        view_certificate_route,
        view_reduction_certificate_pass,
        average_to_worst_pass: row.metrics.average_to_worst_slack
            >= profile.min_average_to_worst_slack as f64,
        security_proxy_pass: row.metrics.security_proxy_bits >= profile.target_security_bits as f64,
        reported_guard_distance_matches: row.metrics.guard_distance == guard_distance_exact,
    };

    let mut failures = Vec::new();
    if !exact.q_is_prime {
        failures.push("q_not_prime".to_string());
    }
    if !exact.field_size_pass {
        failures.push("field_size".to_string());
    }
    if !exact.full_column_rank {
        failures.push("full_rank".to_string());
    }
    if !exact.guard_column_rank {
        failures.push("guard_rank".to_string());
    }
    if !exact.tilted_adjoint_full_rank {
        failures.push("tilted_adjoint_rank".to_string());
    }
    if !exact.tilt_column_support_pass {
        failures.push("tilt_column_support".to_string());
    }
    if !exact.bounded_dual_witness_pass {
        failures.push("bounded_dual_witness".to_string());
    }
    if !exact.guard_distance_pass {
        failures.push("guard_distance".to_string());
    }
    if !exact.correctness_distance_pass {
        failures.push("correctness_distance".to_string());
    }
    if !exact.combined_primal_distance_certificate_pass {
        failures.push("combined_primal_distance_certificate_missing".to_string());
    }
    if !exact.view_coordinate_bound_pass {
        failures.push("view_coords_exceed_full_coordinates".to_string());
    }
    if !exact.proof_strategy_view_pass {
        failures.push("proof_strategy_view_mismatch".to_string());
    }
    if !exact.primal_distance_view_pass {
        failures.push("guard_only_requires_guard_view".to_string());
    }
    if !exact.view_reduction_certificate_pass {
        if exact.lossy_diagonal_proxy_pass {
            failures.push("view_pair_coordinate_certificate_missing".to_string());
        } else {
            failures.push("view_reduction_certificate_missing".to_string());
        }
    }
    if !exact.average_to_worst_pass {
        failures.push("average_to_worst_proxy".to_string());
    }
    if !exact.security_proxy_pass {
        failures.push("security_proxy".to_string());
    }
    if !exact.qary_amplitude_at_noise_pass {
        failures.push("qary_amplitude_at_noise".to_string());
    }
    if !exact.qary_amplitude_geometry_possible {
        failures.push("qary_amplitude_minkowski_obstruction".to_string());
    }
    match c.qary_amplitude_strategy.as_str() {
        "fallback-support" => {}
        "solved-repetition" => {
            if c.lee_repetition == 0 {
                failures.push("lee_repetition_positive".to_string());
            }
            if c.lee_repetition.saturating_mul(c.m1) > c.n {
                failures.push("solved_repetition_requires_pivot_capacity".to_string());
            }
        }
        "direct-solved-lee" => {
            if c.lee_repetition == 0 {
                failures.push("direct_solved_lee_positive".to_string());
            }
        }
        "shared-pair-lee" | "product-block-lee" => {
            if c.lee_repetition == 0 {
                failures.push("shared_pair_lee_positive".to_string());
            }
            if c.lee_rows_per_group == 0 {
                failures.push("product_block_rows_per_group_positive".to_string());
            }
            if c.m1 % 2 != 0 {
                failures.push("shared_pair_requires_even_m1".to_string());
            }
            if product_block_used_rows(c) > c.n {
                failures.push("shared_pair_requires_group_rows".to_string());
            }
            if !exact.shared_pair_global_certificate_pass {
                failures.push("shared_pair_global_certificate_missing".to_string());
            }
        }
        "moment-lee" => {
            if c.lee_repetition == 0 {
                failures.push("moment_lee_positive".to_string());
            }
        }
        _ => failures.push("unknown_qary_amplitude_strategy".to_string()),
    }
    if !exact.reported_guard_distance_matches {
        failures.push("reported_guard_distance_mismatch".to_string());
    }

    ValidationRow {
        profile: profile.id.clone(),
        source: source.to_string(),
        candidate: c.clone(),
        reported: row.metrics.clone(),
        generated_setup,
        exact,
        verdict: if failures.is_empty() {
            "pass".to_string()
        } else {
            "fail".to_string()
        },
        failures,
    }
}

fn collect_rows(payload: &Payload, top: usize, include_pareto: bool) -> Vec<(String, Row)> {
    let mut rows = Vec::new();
    let mut seen = BTreeMap::new();
    for row in payload.accepted_top.iter().take(top) {
        seen.insert(row.candidate.key(), true);
        rows.push(("accepted_top".to_string(), row.clone()));
    }
    if include_pareto {
        for row in &payload.pareto_frontier {
            if seen.insert(row.candidate.key(), true).is_none() {
                rows.push(("pareto_frontier".to_string(), row.clone()));
            }
        }
    }
    rows
}

fn update_summary(summary: &mut ValidationSummary, row: &ValidationRow) {
    let entry = summary.by_profile.entry(row.profile.clone()).or_default();
    entry.rows_checked += 1;
    if row.verdict == "pass" {
        entry.rows_passed += 1;
        entry.min_ciphertext_bytes = Some(
            entry
                .min_ciphertext_bytes
                .map_or(row.reported.ciphertext_bytes, |v| {
                    v.min(row.reported.ciphertext_bytes)
                }),
        );
        entry.min_public_key_bytes = Some(
            entry
                .min_public_key_bytes
                .map_or(row.reported.compact_public_key_bytes, |v| {
                    v.min(row.reported.compact_public_key_bytes)
                }),
        );
    }
    for failure in &row.failures {
        *summary
            .failures_by_reason
            .entry(failure.clone())
            .or_insert(0) += 1;
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

fn markdown_report(payload: &ValidationPayload) -> String {
    let mut lines = vec![
        "# Bandwidth Frontier Validation".to_string(),
        String::new(),
        format!("- validation model: `{}`", payload.validation_model),
        format!("- rows checked: `{}`", payload.rows_checked),
        format!("- top rows per profile: `{}`", payload.top_per_profile),
        String::new(),
        "This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.".to_string(),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        "| profile | rows checked | rows passed | min passing pk | min passing ct |".to_string(),
        "|---|---:|---:|---:|---:|".to_string(),
    ];

    for (profile, summary) in &payload.summary.by_profile {
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            profile,
            summary.rows_checked,
            summary.rows_passed,
            summary
                .min_public_key_bytes
                .map_or("-".to_string(), |v| format!("{v} B")),
            summary
                .min_ciphertext_bytes
                .map_or("-".to_string(), |v| format!("{v} B")),
        ));
    }

    lines.extend([
        String::new(),
        "## First Passing Rows".to_string(),
        String::new(),
        "| profile | source | proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | lossy diag req | lossy diag slack | amp l1 req | amp l1 cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | bounded dual | view route | verdict |".to_string(),
        "|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|".to_string(),
    ]);

    let mut emitted = BTreeMap::new();
    for row in &payload.rows {
        if row.verdict != "pass" || emitted.contains_key(&row.profile) {
            continue;
        }
        emitted.insert(row.profile.clone(), true);
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} B | {} B | {} | {} | {:.1} | {} | {} | {:.1} | {:.1} | {} | {} | {} | {} | {} |",
            row.profile,
            row.source,
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
            row.reported.compact_public_key_bytes,
            row.reported.ciphertext_bytes,
            row.exact.correctness_distance_exact,
            row.exact.lossy_diagonal_required_distance,
            row.exact.lossy_diagonal_slack_bits,
            row.exact.qary_amplitude_l1_required,
            row.exact.qary_amplitude_l1_certified,
            row.exact.qary_amplitude_minkowski_l1_upper,
            row.exact.qary_amplitude_minkowski_margin,
            row.exact.qary_amplitude_certificate_rows,
            row.exact.qary_amplitude_keygen_mul_ops,
            row.generated_setup.no_dual_witness_up_to,
            row.exact.view_certificate_route,
            row.verdict,
        ));
    }

    lines.extend([
        String::new(),
        "## Next Checks".to_string(),
        String::new(),
        "1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.".to_string(),
        "2. Reject rows with `qary_amplitude_minkowski_obstruction`; no arrangement with the same public-view dimensions can meet the required centered-L1 threshold.".to_string(),
        "3. Run view-only q-ary centered-L1/BKZ reductions, then exact bounded Lee-dual checks, before estimator work.".to_string(),
        "4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.".to_string(),
    ]);

    lines.push(String::new());
    lines.join("\n")
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    let mut source_files = Vec::new();
    let mut rows = Vec::new();
    let mut summary = ValidationSummary::default();

    let mut files = fs::read_dir(&cfg.input_dir)
        .map_err(|e| format!("read {}: {e}", cfg.input_dir.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("read {}: {e}", cfg.input_dir.display()))?;
    files.sort();

    for path in files {
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let text =
            fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let payload: Payload =
            serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
        source_files.push(display_path(&path));
        for (source, row) in collect_rows(&payload, cfg.top, cfg.include_pareto) {
            let validated = validate_row(&payload.profile, &source, &row, &cfg);
            update_summary(&mut summary, &validated);
            rows.push(validated);
        }
    }

    let payload = ValidationPayload {
        generated_at: generated_at(),
        validation_model: "seeded_sparse_tilt_v1".to_string(),
        source_files,
        top_per_profile: cfg.top,
        rows_checked: rows.len(),
        summary,
        rows,
        notes: vec![
            "This materializes deterministic public coefficient, incidence, and diagonal seeds for each row.".to_string(),
            "The generated tilt has exactly three nonzero row entries per tilt column and is combined with Cauchy guard/base blocks over the row's prime field.".to_string(),
            "Compressed view rows now report route diagnostics: support-covered rows need a concrete hitting-set certificate, while lossy-budget rows report viewed-noise, viewed-separation, and multi-coordinate diagonal-amplification checks.".to_string(),
            "A guard-block lossy diagonal view is accepted when the selected prefix contains the full Cauchy guard block, the field is prime, the guard-block support distance meets the diagonal bad-seed bound, and the lossy noise/separation budget passes.".to_string(),
            "Rows using product-block-lee/shared-pair-lee require a matching --shared-lee-cert-json sidecar that globally certifies the product-block Lee distance.".to_string(),
            "Passing rows should still be treated as candidates for deeper bounded dual-witness search and estimator work, not as deployed parameters.".to_string(),
        ],
    };

    fs::create_dir_all(&cfg.output_dir)
        .map_err(|e| format!("create {}: {e}", cfg.output_dir.display()))?;
    let json_path = cfg.output_dir.join("validation.json");
    let md_path = cfg.output_dir.join("validation.md");
    let json =
        serde_json::to_string_pretty(&payload).map_err(|e| format!("serialize json: {e}"))?;
    fs::write(&json_path, format!("{json}\n"))
        .map_err(|e| format!("write {}: {e}", json_path.display()))?;
    fs::write(&md_path, markdown_report(&payload))
        .map_err(|e| format!("write {}: {e}", md_path.display()))?;

    println!("wrote {}", display_path(&json_path));
    println!("wrote {}", display_path(&md_path));
    println!("rows_checked={}", payload.rows_checked);
    Ok(())
}
