use serde::Serialize;
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

const MASK64: u64 = u64::MAX;

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
        x ^= (x << 25) & MASK64;
        x ^= x >> 27;
        self.state = x & MASK64;
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn modulo(&mut self, modulus: u64) -> u64 {
        self.next_u64() % modulus
    }

    fn unit_mod(&mut self, q: u64) -> u64 {
        1 + self.modulo(q - 1)
    }
}

#[derive(Clone, Debug)]
struct Config {
    q: u64,
    pivot_rows: usize,
    free_cols: usize,
    eta: u64,
    target_l1: Option<u64>,
    families: Vec<String>,
    seeds: u64,
    block_group_size: usize,
    unit_bound: i64,
    pair_bound: i64,
    coeff_bound: i64,
    random_samples: u64,
    max_random_sparsity: usize,
    anneal_restarts: u64,
    anneal_steps: u64,
    top: usize,
    output_json: PathBuf,
    output_md: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
struct MatrixDescriptor {
    family: String,
    seed: u64,
    q: u64,
    pivot_rows: usize,
    free_cols: usize,
    block_group_size: Option<usize>,
    seed_bits: u64,
    public_sample_bits: u64,
    mul_work: u64,
    certificate_work: u64,
}

#[derive(Clone, Debug, Serialize)]
struct SearchResult {
    target_l1: u64,
    best_centered_l1: u64,
    lee_margin: i64,
    found_counterexample: bool,
    best_method: String,
    best_free_sparsity: usize,
    relation_support: usize,
    relation_linf: u64,
    free_vector_centered: Vec<i64>,
    relation_prefix_centered: Vec<i64>,
}

#[derive(Clone, Debug, Serialize)]
struct CandidateReport {
    descriptor: MatrixDescriptor,
    result: SearchResult,
}

#[derive(Debug, Serialize)]
struct SearchOutput {
    family: String,
    q: u64,
    pivot_rows: usize,
    free_cols: usize,
    eta: u64,
    target_l1: u64,
    families_searched: Vec<String>,
    seeds_per_family: u64,
    unit_bound: i64,
    pair_bound: i64,
    coeff_bound: i64,
    random_samples: u64,
    max_random_sparsity: usize,
    anneal_restarts: u64,
    anneal_steps: u64,
    top: usize,
    candidates: Vec<CandidateReport>,
    notes: Vec<String>,
}

#[derive(Clone)]
struct Best {
    l1: u64,
    method: String,
    free: Vec<i64>,
    relation: Vec<i64>,
}

fn parse_csv(s: &str) -> Vec<String> {
    s.split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.trim().to_string())
        .collect()
}

fn mix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

fn family_seed(family: &str, seed_index: u64) -> u64 {
    let mut h = 0x5EED_1EE_u64 ^ seed_index.wrapping_mul(0x9E3779B97F4A7C15);
    for byte in family.bytes() {
        h = mix64(h ^ u64::from(byte));
    }
    mix64(h)
}

fn centered_value(x: u64, q: u64) -> i64 {
    let r = x % q;
    if r > q / 2 {
        -((q - r) as i64)
    } else {
        r as i64
    }
}

fn signed_to_mod(x: i64, q: u64) -> u64 {
    if x >= 0 {
        (x as u64) % q
    } else {
        let y = ((-x) as u64) % q;
        if y == 0 { 0 } else { q - y }
    }
}

fn mod_mul(a: u64, b: u64, q: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(q)) as u64
}

fn mod_pow(mut base: u64, mut exp: usize, q: u64) -> u64 {
    let mut acc = 1;
    base %= q;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mod_mul(acc, base, q);
        }
        base = mod_mul(base, base, q);
        exp >>= 1;
    }
    acc
}

fn qary_amplitude_l1_required(q: u64, eta: u64) -> u64 {
    if eta == 0 {
        u64::MAX
    } else {
        (q.saturating_sub(1)).div_ceil(2 * eta)
    }
}

fn relation_from_free(matrix: &[Vec<u64>], free: &[i64], q: u64) -> Vec<i64> {
    let mut relation = Vec::with_capacity(matrix.len() + free.len());
    for row in matrix {
        let mut acc = 0u128;
        for (a, &x) in row.iter().zip(free) {
            if x != 0 {
                acc += u128::from(*a) * u128::from(signed_to_mod(x, q));
            }
        }
        relation.push(centered_value((acc % u128::from(q)) as u64, q));
    }
    relation.extend_from_slice(free);
    relation
}

fn relation_l1(relation: &[i64]) -> u64 {
    relation.iter().map(|x| x.unsigned_abs()).sum()
}

fn maybe_update(
    best: &mut Option<Best>,
    matrix: &[Vec<u64>],
    free: Vec<i64>,
    q: u64,
    method: &str,
) {
    if free.iter().all(|&x| x == 0) {
        return;
    }
    let relation = relation_from_free(matrix, &free, q);
    let l1 = relation_l1(&relation);
    if best.as_ref().is_none_or(|b| l1 < b.l1) {
        *best = Some(Best {
            l1,
            method: method.to_string(),
            free,
            relation,
        });
    }
}

fn probe_matrix(cfg: &Config, matrix: &[Vec<u64>], seed: u64, target_l1: u64) -> SearchResult {
    let mut best = None;
    for j in 0..cfg.free_cols {
        for x in -cfg.unit_bound..=cfg.unit_bound {
            if x == 0 {
                continue;
            }
            let mut free = vec![0; cfg.free_cols];
            free[j] = x;
            maybe_update(&mut best, matrix, free, cfg.q, "unit-exact");
        }
    }
    for j0 in 0..cfg.free_cols {
        for j1 in (j0 + 1)..cfg.free_cols {
            for x0 in -cfg.pair_bound..=cfg.pair_bound {
                if x0 == 0 {
                    continue;
                }
                for x1 in -cfg.pair_bound..=cfg.pair_bound {
                    if x1 == 0 {
                        continue;
                    }
                    let mut free = vec![0; cfg.free_cols];
                    free[j0] = x0;
                    free[j1] = x1;
                    maybe_update(&mut best, matrix, free, cfg.q, "pair-exact");
                }
            }
        }
    }

    let mut rng = Rng64::new(seed ^ 0x51DECA11);
    for _ in 0..cfg.random_samples {
        let sparsity = 1 + rng.modulo(cfg.max_random_sparsity as u64) as usize;
        let mut free = vec![0; cfg.free_cols];
        let mut filled = 0;
        while filled < sparsity.min(cfg.free_cols) {
            let j = rng.modulo(cfg.free_cols as u64) as usize;
            if free[j] != 0 {
                continue;
            }
            let mag = 1 + rng.modulo(cfg.coeff_bound as u64) as i64;
            let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
            free[j] = sign * mag;
            filled += 1;
        }
        maybe_update(&mut best, matrix, free, cfg.q, "random-sparse");
    }

    for _ in 0..cfg.anneal_restarts {
        let mut free = vec![0; cfg.free_cols];
        let init = 1 + rng.modulo(cfg.max_random_sparsity as u64) as usize;
        let mut filled = 0;
        while filled < init.min(cfg.free_cols) {
            let j = rng.modulo(cfg.free_cols as u64) as usize;
            if free[j] != 0 {
                continue;
            }
            let mag = 1 + rng.modulo(cfg.coeff_bound as u64) as i64;
            let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
            free[j] = sign * mag;
            filled += 1;
        }
        let mut current = relation_l1(&relation_from_free(matrix, &free, cfg.q));
        maybe_update(&mut best, matrix, free.clone(), cfg.q, "anneal");
        for step in 0..cfg.anneal_steps {
            let j = rng.modulo(cfg.free_cols as u64) as usize;
            let old = free[j];
            let delta = match rng.modulo(8) {
                0 => -16,
                1 => -8,
                2 => -4,
                3 => -1,
                4 => 1,
                5 => 4,
                6 => 8,
                _ => 16,
            };
            let new = (old + delta).clamp(-cfg.coeff_bound, cfg.coeff_bound);
            if new == old {
                continue;
            }
            free[j] = new;
            if free.iter().all(|&x| x == 0) {
                free[j] = old;
                continue;
            }
            let candidate = relation_l1(&relation_from_free(matrix, &free, cfg.q));
            let temperature = (cfg.anneal_steps.saturating_sub(step) + 1).max(1);
            let uphill_budget = 1 + (current / 64).min(10_000);
            let accept_uphill =
                candidate <= current + uphill_budget && rng.modulo(temperature) == 0;
            if candidate <= current || accept_uphill {
                current = candidate;
                maybe_update(&mut best, matrix, free.clone(), cfg.q, "anneal");
            } else {
                free[j] = old;
            }
        }
    }

    let best = best.expect("at least one nonzero vector checked");
    let relation_support = best.relation.iter().filter(|&&x| x != 0).count();
    let relation_linf = best
        .relation
        .iter()
        .map(|x| x.unsigned_abs())
        .max()
        .unwrap_or(0);
    SearchResult {
        target_l1,
        best_centered_l1: best.l1,
        lee_margin: best.l1 as i64 - target_l1 as i64,
        found_counterexample: best.l1 < target_l1,
        best_method: best.method,
        best_free_sparsity: best.free.iter().filter(|&&x| x != 0).count(),
        relation_support,
        relation_linf,
        free_vector_centered: best.free,
        relation_prefix_centered: best.relation.into_iter().take(64).collect(),
    }
}

fn row_col_scales(q: u64, rows: usize, cols: usize, rng: &mut Rng64) -> (Vec<u64>, Vec<u64>) {
    (
        (0..rows).map(|_| rng.unit_mod(q)).collect(),
        (0..cols).map(|_| rng.unit_mod(q)).collect(),
    )
}

fn dense_random_matrix(q: u64, rows: usize, cols: usize, seed: u64) -> Vec<Vec<u64>> {
    let mut rng = Rng64::new(seed);
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.modulo(q)).collect())
        .collect()
}

fn block_random_matrix(
    q: u64,
    rows: usize,
    cols: usize,
    group_size: usize,
    seed: u64,
) -> Vec<Vec<u64>> {
    let group_size = group_size.max(1).min(cols);
    let groups = cols.div_ceil(group_size);
    let rows_per_group = rows / groups;
    let extra = rows % groups;
    let mut starts = Vec::with_capacity(groups + 1);
    let mut acc = 0;
    starts.push(acc);
    for g in 0..groups {
        acc += rows_per_group + usize::from(g < extra);
        starts.push(acc);
    }
    let mut rng = Rng64::new(seed);
    let mut matrix = vec![vec![0; cols]; rows];
    for group in 0..groups {
        let c0 = group * group_size;
        let c1 = ((group + 1) * group_size).min(cols);
        for row in matrix
            .iter_mut()
            .take(starts[group + 1])
            .skip(starts[group])
        {
            for cell in row.iter_mut().take(c1).skip(c0) {
                *cell = rng.modulo(q);
            }
        }
    }
    matrix
}

fn toeplitz_scrambled_matrix(q: u64, rows: usize, cols: usize, seed: u64) -> Vec<Vec<u64>> {
    let mut rng = Rng64::new(seed);
    let diagonal = (0..rows + cols - 1)
        .map(|_| rng.modulo(q))
        .collect::<Vec<_>>();
    let (row_scale, col_scale) = row_col_scales(q, rows, cols, &mut rng);
    let mut matrix = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            matrix[i][j] = mod_mul(row_scale[i], mod_mul(diagonal[i + j], col_scale[j], q), q);
        }
    }
    matrix
}

fn vandermonde_scrambled_matrix(q: u64, rows: usize, cols: usize, seed: u64) -> Vec<Vec<u64>> {
    let mut rng = Rng64::new(seed);
    let (row_scale, col_scale) = row_col_scales(q, rows, cols, &mut rng);
    let alphas = (0..rows).map(|_| rng.unit_mod(q)).collect::<Vec<_>>();
    let mut matrix = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let value = mod_pow(alphas[i], j + 1, q);
            matrix[i][j] = mod_mul(row_scale[i], mod_mul(value, col_scale[j], q), q);
        }
    }
    matrix
}

fn generate_matrix(cfg: &Config, family: &str, seed: u64) -> Vec<Vec<u64>> {
    match family {
        "dense-random" => dense_random_matrix(cfg.q, cfg.pivot_rows, cfg.free_cols, seed),
        "block-random" => block_random_matrix(
            cfg.q,
            cfg.pivot_rows,
            cfg.free_cols,
            cfg.block_group_size,
            seed,
        ),
        "toeplitz-scrambled" => {
            toeplitz_scrambled_matrix(cfg.q, cfg.pivot_rows, cfg.free_cols, seed)
        }
        "vandermonde-scrambled" => {
            vandermonde_scrambled_matrix(cfg.q, cfg.pivot_rows, cfg.free_cols, seed)
        }
        other => panic!("unknown Lee family {other:?}"),
    }
}

fn descriptor(cfg: &Config, family: &str, seed: u64) -> MatrixDescriptor {
    let seed_bits = match family {
        "dense-random" => 64,
        "block-random" => 64,
        "toeplitz-scrambled" => 64,
        "vandermonde-scrambled" => 64,
        _ => 64,
    };
    let unit_exact_work = cfg
        .free_cols
        .saturating_mul(2usize.saturating_mul(cfg.unit_bound.unsigned_abs() as usize))
        as u64;
    let pair_choices = cfg
        .free_cols
        .saturating_mul(cfg.free_cols.saturating_sub(1))
        / 2;
    let pair_values = 2u64
        .saturating_mul(cfg.pair_bound.unsigned_abs())
        .saturating_mul(2u64.saturating_mul(cfg.pair_bound.unsigned_abs()));

    MatrixDescriptor {
        family: family.to_string(),
        seed,
        q: cfg.q,
        pivot_rows: cfg.pivot_rows,
        free_cols: cfg.free_cols,
        block_group_size: (family == "block-random").then_some(cfg.block_group_size),
        seed_bits,
        public_sample_bits: (cfg.pivot_rows + cfg.free_cols) as u64
            * (cfg.q as f64).log2().ceil() as u64,
        mul_work: cfg.pivot_rows as u64 * cfg.free_cols as u64,
        certificate_work: cfg.random_samples
            + cfg.anneal_restarts.saturating_mul(cfg.anneal_steps)
            + unit_exact_work
            + (pair_choices as u64).saturating_mul(pair_values),
    }
}

fn candidate_cmp(a: &CandidateReport, b: &CandidateReport) -> Ordering {
    b.result
        .lee_margin
        .cmp(&a.result.lee_margin)
        .then_with(|| b.result.best_centered_l1.cmp(&a.result.best_centered_l1))
        .then_with(|| a.descriptor.mul_work.cmp(&b.descriptor.mul_work))
}

fn markdown(out: &SearchOutput) -> String {
    let mut lines = vec![
        "# Systematic Lee Search".to_string(),
        String::new(),
        format!("- q: `{}`", out.q),
        format!("- pivot rows: `{}`", out.pivot_rows),
        format!("- free columns: `{}`", out.free_cols),
        format!("- eta: `{}`", out.eta),
        format!("- target centered Lee l1: `{}`", out.target_l1),
        format!("- families: `{}`", out.families_searched.join(", ")),
        format!("- seeds per family: `{}`", out.seeds_per_family),
        String::new(),
        "| family | seed | best l1 | margin | counterexample | method | free sparsity | support | linf | mul work | cert work |".to_string(),
        "|---|---:|---:|---:|---|---|---:|---:|---:|---:|---:|".to_string(),
    ];
    for c in &out.candidates {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            c.descriptor.family,
            c.descriptor.seed,
            c.result.best_centered_l1,
            c.result.lee_margin,
            c.result.found_counterexample,
            c.result.best_method,
            c.result.best_free_sparsity,
            c.result.relation_support,
            c.result.relation_linf,
            c.descriptor.mul_work,
            c.descriptor.certificate_work,
        ));
    }
    lines.extend([
        String::new(),
        "Each row is a systematic public-view candidate `A = [I | P]`.".to_string(),
        "The relation checked is `(-P y, y)`, matching `SystematicLeeDistanceAtLeast`.".to_string(),
        "A found counterexample is real for the generated `P`; absence of one is a search sidecar, not a proof.".to_string(),
        "Survivors still need exact bounded Lee-dual certification before Lean instantiation.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        q: 65537,
        pivot_rows: 48,
        free_cols: 32,
        eta: 10,
        target_l1: None,
        families: vec![
            "dense-random".to_string(),
            "block-random".to_string(),
            "toeplitz-scrambled".to_string(),
            "vandermonde-scrambled".to_string(),
        ],
        seeds: 16,
        block_group_size: 8,
        unit_bound: 256,
        pair_bound: 32,
        coeff_bound: 128,
        random_samples: 20_000,
        max_random_sparsity: 8,
        anneal_restarts: 32,
        anneal_steps: 5_000,
        top: 16,
        output_json: PathBuf::from("research/parameter_search/systematic_lee_search.json"),
        output_md: PathBuf::from("research/parameter_search/systematic_lee_search.md"),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--q" => cfg.q = args.next().unwrap().parse().unwrap(),
            "--pivot-rows" | "--rows" => cfg.pivot_rows = args.next().unwrap().parse().unwrap(),
            "--free-cols" | "--m1" => cfg.free_cols = args.next().unwrap().parse().unwrap(),
            "--eta" => cfg.eta = args.next().unwrap().parse().unwrap(),
            "--target-l1" => cfg.target_l1 = Some(args.next().unwrap().parse().unwrap()),
            "--families" | "--lee-family" => cfg.families = parse_csv(&args.next().unwrap()),
            "--seeds" => cfg.seeds = args.next().unwrap().parse().unwrap(),
            "--block-group-size" => cfg.block_group_size = args.next().unwrap().parse().unwrap(),
            "--unit-bound" => cfg.unit_bound = args.next().unwrap().parse().unwrap(),
            "--pair-bound" => cfg.pair_bound = args.next().unwrap().parse().unwrap(),
            "--coeff-bound" => cfg.coeff_bound = args.next().unwrap().parse().unwrap(),
            "--random-samples" => cfg.random_samples = args.next().unwrap().parse().unwrap(),
            "--max-random-sparsity" => {
                cfg.max_random_sparsity = args.next().unwrap().parse().unwrap()
            }
            "--anneal-restarts" => cfg.anneal_restarts = args.next().unwrap().parse().unwrap(),
            "--anneal-steps" => cfg.anneal_steps = args.next().unwrap().parse().unwrap(),
            "--top" => cfg.top = args.next().unwrap().parse().unwrap(),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            other => panic!("unknown argument: {other}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.q > 2, "q must be greater than 2");
    assert!(cfg.pivot_rows > 0, "pivot rows must be positive");
    assert!(cfg.free_cols > 0, "free columns must be positive");
    assert!(cfg.eta > 0, "eta must be positive");
    assert!(cfg.unit_bound > 0 && cfg.pair_bound > 0 && cfg.coeff_bound > 0);
    let target_l1 = cfg
        .target_l1
        .unwrap_or_else(|| qary_amplitude_l1_required(cfg.q, cfg.eta));
    let mut reports = Vec::new();
    for family in &cfg.families {
        for seed_index in 0..cfg.seeds {
            let seed = family_seed(family, seed_index);
            let matrix = generate_matrix(&cfg, family, seed);
            let result = probe_matrix(&cfg, &matrix, seed, target_l1);
            reports.push(CandidateReport {
                descriptor: descriptor(&cfg, family, seed),
                result,
            });
        }
    }
    reports.sort_by(candidate_cmp);
    reports.truncate(cfg.top);
    let out = SearchOutput {
        family: "systematic-lee-expander".to_string(),
        q: cfg.q,
        pivot_rows: cfg.pivot_rows,
        free_cols: cfg.free_cols,
        eta: cfg.eta,
        target_l1,
        families_searched: cfg.families.clone(),
        seeds_per_family: cfg.seeds,
        unit_bound: cfg.unit_bound,
        pair_bound: cfg.pair_bound,
        coeff_bound: cfg.coeff_bound,
        random_samples: cfg.random_samples,
        max_random_sparsity: cfg.max_random_sparsity,
        anneal_restarts: cfg.anneal_restarts,
        anneal_steps: cfg.anneal_steps,
        top: cfg.top,
        candidates: reports,
        notes: vec![
            "Systematic relation surface: r(y) = (-P y, y).".to_string(),
            "This emits search sidecars for the Lean boundary SystematicLeeDistanceAtLeast.".to_string(),
            "Use view-only BKZ/LLL and exact bounded search before treating any survivor as a certificate.".to_string(),
        ],
    };
    if let Some(parent) = cfg.output_json.parent() {
        fs::create_dir_all(parent).expect("create output json parent");
    }
    if let Some(parent) = cfg.output_md.parent() {
        fs::create_dir_all(parent).expect("create output md parent");
    }
    fs::write(
        &cfg.output_json,
        serde_json::to_string_pretty(&out).unwrap() + "\n",
    )
    .expect("write json");
    fs::write(&cfg.output_md, markdown(&out)).expect("write markdown");
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
}
