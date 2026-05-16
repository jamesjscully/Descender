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

    fn nonzero_mod(&mut self, q: u64) -> u64 {
        1 + self.modulo(q - 1)
    }
}

#[derive(Debug)]
struct Config {
    q: u64,
    n: usize,
    m1: usize,
    family: String,
    rows_per_free: Option<usize>,
    scalar_trials: usize,
    exhaustive_singletons: bool,
    seed: u64,
    target_l1: Option<u64>,
    output_json: PathBuf,
    output_md: PathBuf,
}

#[derive(Debug, Serialize)]
struct CertificateOutput {
    family: String,
    q: u64,
    n: usize,
    m1: usize,
    rows_per_free: usize,
    used_rows: usize,
    unused_rows: usize,
    target_l1: Option<u64>,
    certified_l1: u64,
    passes_target: Option<bool>,
    keygen_mul_ops: usize,
    exact_check: String,
    scalars_per_free_coordinate: Vec<Vec<u64>>,
    worst_coordinate: usize,
    worst_free_value_mod_q: u64,
    worst_relation_centered_l1: u64,
    worst_relation_support: usize,
    worst_relation_centered: Vec<i64>,
    notes: Vec<String>,
}

fn centered_abs(x: u64, q: u64) -> u64 {
    let r = x % q;
    r.min(q - r)
}

fn centered_value(x: u64, q: u64) -> i64 {
    let r = x % q;
    if r > q / 2 {
        -((q - r) as i64)
    } else {
        r as i64
    }
}

fn scalar_block_l1(q: u64, scalars: &[u64], x: u64) -> u64 {
    centered_abs(x, q)
        + scalars
            .iter()
            .map(|&a| centered_abs(((a as u128 * x as u128) % q as u128) as u64, q))
            .sum::<u64>()
}

fn scalar_block_min(q: u64, scalars: &[u64]) -> (u64, u64) {
    let mut best_l1 = u64::MAX;
    let mut best_x = 1;
    for x in 1..q {
        let l1 = scalar_block_l1(q, scalars, x);
        if l1 < best_l1 {
            best_l1 = l1;
            best_x = x;
        }
    }
    (best_l1, best_x)
}

fn relation_for_worst(
    q: u64,
    n: usize,
    m1: usize,
    rows_per_free: usize,
    scalars: &[Vec<u64>],
    coordinate: usize,
    x: u64,
) -> Vec<i64> {
    let mut relation = vec![0; n + m1];
    for (t, &a) in scalars[coordinate].iter().enumerate() {
        let row = coordinate * rows_per_free + t;
        relation[row] = centered_value(((a as u128 * x as u128) % q as u128) as u64, q);
    }
    relation[n + coordinate] = centered_value(x, q);
    relation
}

fn identity_scalars(m1: usize, rows_per_free: usize) -> Vec<Vec<u64>> {
    vec![vec![1; rows_per_free]; m1]
}

fn random_scalars(q: u64, m1: usize, rows_per_free: usize, seed: u64) -> Vec<Vec<u64>> {
    let mut rng = Rng64::new(seed);
    (0..m1)
        .map(|_| {
            (0..rows_per_free)
                .map(|_| rng.nonzero_mod(q))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn optimized_scalar_block(
    q: u64,
    rows_per_free: usize,
    scalar_trials: usize,
    seed: u64,
    exhaustive_singletons: bool,
) -> Vec<u64> {
    if rows_per_free == 0 {
        return Vec::new();
    }
    let mut best_scalars = vec![1; rows_per_free];
    let mut best_l1 = 0;

    if exhaustive_singletons && rows_per_free == 1 {
        for a in 1..q {
            let scalars = [a];
            let (l1, _) = scalar_block_min(q, &scalars);
            if l1 > best_l1 {
                best_l1 = l1;
                best_scalars = vec![a];
            }
        }
        return best_scalars;
    }

    let mut rng = Rng64::new(seed);
    let trials = scalar_trials.max(1);
    for _ in 0..trials {
        let scalars = (0..rows_per_free)
            .map(|_| rng.nonzero_mod(q))
            .collect::<Vec<_>>();
        let (l1, _) = scalar_block_min(q, &scalars);
        if l1 > best_l1 {
            best_l1 = l1;
            best_scalars = scalars;
        }
    }
    best_scalars
}

fn build_scalars(cfg: &Config, rows_per_free: usize) -> Vec<Vec<u64>> {
    match cfg.family.as_str() {
        "identity-repetition" => identity_scalars(cfg.m1, rows_per_free),
        "random-scalar-block" => random_scalars(cfg.q, cfg.m1, rows_per_free, cfg.seed),
        "optimized-scalar-block" => {
            let block = optimized_scalar_block(
                cfg.q,
                rows_per_free,
                cfg.scalar_trials,
                cfg.seed ^ 0x51CA1AB1E,
                cfg.exhaustive_singletons,
            );
            vec![block; cfg.m1]
        }
        other => panic!(
            "unknown family {other:?}; expected identity-repetition, random-scalar-block, or optimized-scalar-block"
        ),
    }
}

fn certify(cfg: &Config) -> CertificateOutput {
    let rows_per_free = cfg.rows_per_free.unwrap_or_else(|| cfg.n / cfg.m1);
    assert!(cfg.q > 2, "q must be greater than 2");
    assert!(cfg.m1 > 0, "m1 must be positive");
    assert!(
        rows_per_free > 0,
        "rows per free coordinate must be positive"
    );
    let used_rows = rows_per_free * cfg.m1;
    assert!(
        used_rows <= cfg.n,
        "rows_per_free * m1 must fit in n pivot rows"
    );

    let scalars = build_scalars(cfg, rows_per_free);
    let mut certified_l1 = u64::MAX;
    let mut worst_coordinate = 0;
    let mut worst_x = 1;
    for (j, block) in scalars.iter().enumerate() {
        let (l1, x) = scalar_block_min(cfg.q, block);
        if l1 < certified_l1 {
            certified_l1 = l1;
            worst_coordinate = j;
            worst_x = x;
        }
    }
    let witness = relation_for_worst(
        cfg.q,
        cfg.n,
        cfg.m1,
        rows_per_free,
        &scalars,
        worst_coordinate,
        worst_x,
    );
    let support = witness.iter().filter(|&&x| x != 0).count();
    let passes_target = cfg.target_l1.map(|target| certified_l1 >= target);
    CertificateOutput {
        family: cfg.family.clone(),
        q: cfg.q,
        n: cfg.n,
        m1: cfg.m1,
        rows_per_free,
        used_rows,
        unused_rows: cfg.n - used_rows,
        target_l1: cfg.target_l1,
        certified_l1,
        passes_target,
        keygen_mul_ops: used_rows,
        exact_check: "block_diagonal_scalar_exact_over_all_nonzero_free_values".to_string(),
        scalars_per_free_coordinate: scalars,
        worst_coordinate,
        worst_free_value_mod_q: worst_x,
        worst_relation_centered_l1: certified_l1,
        worst_relation_support: support,
        worst_relation_centered: witness,
        notes: vec![
            "Certificate covers only block-diagonal solved maps P where each pivot row reads one free coordinate with the recorded scalar.".to_string(),
            "The exact lower bound is min_j min_{x != 0 mod q} Lee(x) + sum_t Lee(a_{j,t} x).".to_string(),
            "This is a rigorous deployable sidecar for this family, but not a certificate for arbitrary dense solved maps.".to_string(),
        ],
    }
}

fn markdown(out: &CertificateOutput) -> String {
    let pass = out.passes_target.map_or("-".to_string(), |p| p.to_string());
    format!(
        "# Solved Lee Certificate\n\n\
         | field | value |\n\
         |---|---:|\n\
         | family | {} |\n\
         | q | {} |\n\
         | n | {} |\n\
         | m1 | {} |\n\
         | rows per free coordinate | {} |\n\
         | used pivot rows | {} |\n\
         | unused pivot rows | {} |\n\
         | certified centered Lee l1 | {} |\n\
         | target l1 | {} |\n\
         | passes target | {} |\n\
         | keygen multiply ops | {} |\n\
         | worst coordinate | {} |\n\
         | worst free value mod q | {} |\n\
         | worst relation support | {} |\n\n\
         Exact check: `{}`\n\n\
         This certificate is rigorous for the block-diagonal scalar solved-map \
         family. It should be treated as a construction-side certificate for \
         `direct-solved-lee`, not as evidence about arbitrary generated tilt \
         matrices.\n",
        out.family,
        out.q,
        out.n,
        out.m1,
        out.rows_per_free,
        out.used_rows,
        out.unused_rows,
        out.certified_l1,
        out.target_l1
            .map_or("-".to_string(), |target| target.to_string()),
        pass,
        out.keygen_mul_ops,
        out.worst_coordinate,
        out.worst_free_value_mod_q,
        out.worst_relation_support,
        out.exact_check
    )
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        q: 32771,
        n: 32,
        m1: 32,
        family: "optimized-scalar-block".to_string(),
        rows_per_free: None,
        scalar_trials: 512,
        exhaustive_singletons: false,
        seed: 1,
        target_l1: None,
        output_json: PathBuf::from("research/parameter_search/solved_lee_cert.json"),
        output_md: PathBuf::from("research/parameter_search/solved_lee_cert.md"),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--q" => cfg.q = args.next().unwrap().parse().unwrap(),
            "--n" => cfg.n = args.next().unwrap().parse().unwrap(),
            "--m1" => cfg.m1 = args.next().unwrap().parse().unwrap(),
            "--family" => cfg.family = args.next().unwrap(),
            "--rows-per-free" => cfg.rows_per_free = Some(args.next().unwrap().parse().unwrap()),
            "--scalar-trials" => cfg.scalar_trials = args.next().unwrap().parse().unwrap(),
            "--exhaustive-singletons" => cfg.exhaustive_singletons = true,
            "--seed" => cfg.seed = args.next().unwrap().parse().unwrap(),
            "--target-l1" => cfg.target_l1 = Some(args.next().unwrap().parse().unwrap()),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    let out = certify(&cfg);
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
