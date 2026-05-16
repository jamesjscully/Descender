use serde::Serialize;
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
}

#[derive(Debug)]
struct Config {
    q: u64,
    group_size: usize,
    rows: usize,
    target_l1: u64,
    alpha_start: u64,
    exponent_start: usize,
    column_scale: String,
    unit_bound: i64,
    coeff_bound: i64,
    pair_bound: i64,
    random_samples: u64,
    max_random_sparsity: usize,
    anneal_restarts: u64,
    anneal_steps: u64,
    seed: u64,
    output_json: PathBuf,
    output_md: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
struct SearchResult {
    best_centered_l1: u64,
    best_method: String,
    best_free_sparsity: usize,
    found_counterexample: bool,
    free_vector_centered: Vec<i64>,
    relation_centered: Vec<i64>,
    relation_support: usize,
}

#[derive(Debug, Serialize)]
struct ProbeOutput {
    family: String,
    q: u64,
    group_size: usize,
    rows: usize,
    target_l1: u64,
    alpha_start: u64,
    exponent_start: usize,
    column_scale: String,
    unit_bound: i64,
    coeff_bound: i64,
    pair_bound: i64,
    random_samples: u64,
    max_random_sparsity: usize,
    anneal_restarts: u64,
    anneal_steps: u64,
    seed: u64,
    coefficient_matrix_rows: Vec<Vec<u64>>,
    result: SearchResult,
    notes: Vec<String>,
}

fn mod_mul(a: u64, b: u64, q: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(q)) as u64
}

fn signed_to_mod(x: i64, q: u64) -> u64 {
    if x >= 0 {
        (x as u64) % q
    } else {
        let y = ((-x) as u64) % q;
        if y == 0 { 0 } else { q - y }
    }
}

fn centered_value(x: u64, q: u64) -> i64 {
    let r = x % q;
    if r > q / 2 {
        -((q - r) as i64)
    } else {
        r as i64
    }
}

fn column_scales(q: u64, group_size: usize, mode: &str, seed: u64) -> Vec<u64> {
    match mode {
        "none" => vec![1; group_size],
        "midpoint" => vec![(q - 1) / 2; group_size],
        "geometric" => {
            let base = (q - 1) / 2;
            let mut out = Vec::with_capacity(group_size);
            let mut x = base;
            for _ in 0..group_size {
                out.push(x);
                x = mod_mul(x, base, q);
            }
            out
        }
        "random" => {
            let mut rng = Rng64::new(seed ^ 0xC01_5CA1E);
            (0..group_size)
                .map(|_| 1 + rng.modulo(q - 1))
                .collect::<Vec<_>>()
        }
        other => panic!(
            "unknown column scale mode {other:?}; expected none, midpoint, geometric, or random"
        ),
    }
}

fn vandermonde_matrix(
    q: u64,
    group_size: usize,
    rows: usize,
    alpha_start: u64,
    exponent_start: usize,
    column_scale: &str,
    seed: u64,
) -> Vec<Vec<u64>> {
    let scales = column_scales(q, group_size, column_scale, seed);
    let mut matrix = Vec::with_capacity(rows);
    for t in 0..rows {
        let alpha = (alpha_start + t as u64) % q;
        let mut row = Vec::with_capacity(group_size);
        let mut pow = 1;
        for _ in 0..exponent_start {
            pow = mod_mul(pow, alpha, q);
        }
        for &scale in scales.iter().take(group_size) {
            row.push(mod_mul(scale, pow, q));
            pow = mod_mul(pow, alpha, q);
        }
        matrix.push(row);
    }
    matrix
}

fn relation_from_free(matrix: &[Vec<u64>], free: &[i64], q: u64) -> Vec<i64> {
    let mut relation = Vec::with_capacity(matrix.len() + free.len());
    for row in matrix {
        let mut acc = 0u128;
        for (a, &x) in row.iter().zip(free) {
            if x == 0 {
                continue;
            }
            acc += u128::from(*a) * u128::from(signed_to_mod(x, q));
        }
        relation.push(centered_value((acc % u128::from(q)) as u64, q));
    }
    relation.extend_from_slice(free);
    relation
}

fn relation_l1(relation: &[i64]) -> u64 {
    relation.iter().map(|x| x.unsigned_abs()).sum()
}

#[derive(Clone)]
struct Best {
    l1: u64,
    method: String,
    free: Vec<i64>,
    relation: Vec<i64>,
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

fn exact_unit_and_pair(
    matrix: &[Vec<u64>],
    q: u64,
    group_size: usize,
    unit_bound: i64,
    pair_bound: i64,
) -> Option<Best> {
    let mut best = None;
    for j in 0..group_size {
        for x in -unit_bound..=unit_bound {
            if x == 0 {
                continue;
            }
            let mut free = vec![0; group_size];
            free[j] = x;
            maybe_update(&mut best, matrix, free, q, "unit-pair-enumeration");
        }
    }
    for j0 in 0..group_size {
        for j1 in (j0 + 1)..group_size {
            for x0 in -pair_bound..=pair_bound {
                if x0 == 0 {
                    continue;
                }
                for x1 in -pair_bound..=pair_bound {
                    if x1 == 0 {
                        continue;
                    }
                    let mut free = vec![0; group_size];
                    free[j0] = x0;
                    free[j1] = x1;
                    maybe_update(&mut best, matrix, free, q, "unit-pair-enumeration");
                }
            }
        }
    }
    best
}

fn search(cfg: &Config, matrix: &[Vec<u64>]) -> SearchResult {
    let mut best = exact_unit_and_pair(
        matrix,
        cfg.q,
        cfg.group_size,
        cfg.unit_bound,
        cfg.pair_bound,
    );
    let mut rng = Rng64::new(cfg.seed);
    for _ in 0..cfg.random_samples {
        let sparsity = 1 + rng.modulo(cfg.max_random_sparsity as u64) as usize;
        let mut free = vec![0; cfg.group_size];
        let mut filled = 0;
        while filled < sparsity.min(cfg.group_size) {
            let j = rng.modulo(cfg.group_size as u64) as usize;
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
        let mut free = vec![0; cfg.group_size];
        let init_sparsity = 1 + rng.modulo(cfg.max_random_sparsity as u64) as usize;
        let mut filled = 0;
        while filled < init_sparsity.min(cfg.group_size) {
            let j = rng.modulo(cfg.group_size as u64) as usize;
            if free[j] != 0 {
                continue;
            }
            let mag = 1 + rng.modulo(cfg.coeff_bound as u64) as i64;
            let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
            free[j] = sign * mag;
            filled += 1;
        }
        let mut current_relation = relation_from_free(matrix, &free, cfg.q);
        let mut current = relation_l1(&current_relation);
        maybe_update(&mut best, matrix, free.clone(), cfg.q, "anneal");

        for step in 0..cfg.anneal_steps {
            let j = rng.modulo(cfg.group_size as u64) as usize;
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
            let candidate_relation = relation_from_free(matrix, &free, cfg.q);
            let candidate = relation_l1(&candidate_relation);
            let temperature = (cfg.anneal_steps.saturating_sub(step) + 1).max(1);
            let uphill_budget = 1 + (current / 64).min(10_000);
            let accept_uphill =
                candidate <= current + uphill_budget && rng.modulo(temperature) == 0;
            if candidate <= current || accept_uphill {
                current_relation = candidate_relation;
                current = candidate;
                maybe_update(&mut best, matrix, free.clone(), cfg.q, "anneal");
            } else {
                free[j] = old;
            }
        }
        let _ = current_relation;
    }

    let best = best.expect("at least one nonzero vector checked");
    SearchResult {
        best_centered_l1: best.l1,
        best_method: best.method,
        best_free_sparsity: best.free.iter().filter(|&&x| x != 0).count(),
        found_counterexample: best.l1 < cfg.target_l1,
        free_vector_centered: best.free,
        relation_support: best.relation.iter().filter(|&&x| x != 0).count(),
        relation_centered: best.relation,
    }
}

fn markdown(out: &ProbeOutput) -> String {
    format!(
        "# Vandermonde Lee Probe\n\n\
         | field | value |\n\
         |---|---:|\n\
         | q | {} |\n\
         | group size | {} |\n\
         | rows | {} |\n\
         | target centered Lee l1 | {} |\n\
         | alpha start | {} |\n\
         | exponent start | {} |\n\
         | column scale | {} |\n\
         | unit enumeration bound | {} |\n\
         | coefficient bound | {} |\n\
         | pair enumeration bound | {} |\n\
         | random samples | {} |\n\
         | anneal restarts | {} |\n\
         | anneal steps | {} |\n\
         | seed | {} |\n\
         | best centered Lee l1 | {} |\n\
         | found counterexample | {} |\n\
         | best method | {} |\n\
         | best free sparsity | {} |\n\
         | relation support | {} |\n\n\
         Free vector:\n\n\
         ```json\n{}\n```\n\n\
         This is a counterexample/probe tool, not a proof of minimum distance \
         unless all below-target vectors are exhaustively checked.\n",
        out.q,
        out.group_size,
        out.rows,
        out.target_l1,
        out.alpha_start,
        out.exponent_start,
        out.column_scale,
        out.unit_bound,
        out.coeff_bound,
        out.pair_bound,
        out.random_samples,
        out.anneal_restarts,
        out.anneal_steps,
        out.seed,
        out.result.best_centered_l1,
        out.result.found_counterexample,
        out.result.best_method,
        out.result.best_free_sparsity,
        out.result.relation_support,
        serde_json::to_string_pretty(&out.result.free_vector_centered).unwrap()
    )
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        q: 32771,
        group_size: 8,
        rows: 26,
        target_l1: 3277,
        alpha_start: 1,
        exponent_start: 1,
        column_scale: "midpoint".to_string(),
        unit_bound: 1024,
        coeff_bound: 256,
        pair_bound: 64,
        random_samples: 50_000,
        max_random_sparsity: 8,
        anneal_restarts: 64,
        anneal_steps: 20_000,
        seed: 1,
        output_json: PathBuf::from("research/parameter_search/vandermonde_lee_probe.json"),
        output_md: PathBuf::from("research/parameter_search/vandermonde_lee_probe.md"),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--q" => cfg.q = args.next().unwrap().parse().unwrap(),
            "--group-size" => cfg.group_size = args.next().unwrap().parse().unwrap(),
            "--rows" => cfg.rows = args.next().unwrap().parse().unwrap(),
            "--target-l1" => cfg.target_l1 = args.next().unwrap().parse().unwrap(),
            "--alpha-start" => cfg.alpha_start = args.next().unwrap().parse().unwrap(),
            "--exponent-start" => cfg.exponent_start = args.next().unwrap().parse().unwrap(),
            "--column-scale" => cfg.column_scale = args.next().unwrap(),
            "--unit-bound" => cfg.unit_bound = args.next().unwrap().parse().unwrap(),
            "--coeff-bound" => cfg.coeff_bound = args.next().unwrap().parse().unwrap(),
            "--pair-bound" => cfg.pair_bound = args.next().unwrap().parse().unwrap(),
            "--random-samples" => cfg.random_samples = args.next().unwrap().parse().unwrap(),
            "--max-random-sparsity" => {
                cfg.max_random_sparsity = args.next().unwrap().parse().unwrap()
            }
            "--anneal-restarts" => cfg.anneal_restarts = args.next().unwrap().parse().unwrap(),
            "--anneal-steps" => cfg.anneal_steps = args.next().unwrap().parse().unwrap(),
            "--seed" => cfg.seed = args.next().unwrap().parse().unwrap(),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.q > 2, "q must be greater than 2");
    assert!(cfg.group_size > 0, "group size must be positive");
    assert!(cfg.rows > 0, "rows must be positive");
    assert!(cfg.coeff_bound > 0, "coefficient bound must be positive");
    assert!(cfg.unit_bound > 0, "unit bound must be positive");
    let matrix = vandermonde_matrix(
        cfg.q,
        cfg.group_size,
        cfg.rows,
        cfg.alpha_start,
        cfg.exponent_start,
        &cfg.column_scale,
        cfg.seed,
    );
    let result = search(&cfg, &matrix);
    let out = ProbeOutput {
        family: "vandermonde-moment-graph-code".to_string(),
        q: cfg.q,
        group_size: cfg.group_size,
        rows: cfg.rows,
        target_l1: cfg.target_l1,
        alpha_start: cfg.alpha_start,
        exponent_start: cfg.exponent_start,
        column_scale: cfg.column_scale.clone(),
        unit_bound: cfg.unit_bound,
        coeff_bound: cfg.coeff_bound,
        pair_bound: cfg.pair_bound,
        random_samples: cfg.random_samples,
        max_random_sparsity: cfg.max_random_sparsity,
        anneal_restarts: cfg.anneal_restarts,
        anneal_steps: cfg.anneal_steps,
        seed: cfg.seed,
        coefficient_matrix_rows: matrix,
        result,
        notes: vec![
            "Rows are scaled moment evaluations P[t,j] = scale_j * alpha_t^(exponent_start + j).".to_string(),
            "The search covers unit/pair coefficients exhaustively within coeff_bound, then sparse random and annealing probes.".to_string(),
            "A found low-Lee vector is a real counterexample for the checked block; absence of one is not a certificate.".to_string(),
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
    fs::write(&cfg.output_md, markdown(&out)).expect("write md");
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
}
