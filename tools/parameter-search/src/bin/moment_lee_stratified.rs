use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn modulo(&mut self, modulus: u64) -> u64 {
        self.next_u64() % modulus
    }
}

#[derive(Debug, Deserialize)]
struct ProbeResult {
    best_centered_l1: u64,
    found_counterexample: bool,
    free_vector_centered: Vec<i64>,
}

#[derive(Debug, Deserialize)]
struct ProbeOutput {
    family: String,
    q: u64,
    group_size: usize,
    rows: usize,
    target_l1: u64,
    alpha_start: u64,
    exponent_start: usize,
    column_scale: String,
    seed: u64,
    coefficient_matrix_rows: Vec<Vec<u64>>,
    result: ProbeResult,
}

#[derive(Debug)]
struct Config {
    input_json: PathBuf,
    anchor_json: Option<PathBuf>,
    output_json: PathBuf,
    output_md: PathBuf,
    coeff_bound: i64,
    max_support: usize,
    restarts_per_support: u64,
    coordinate_rounds: usize,
    seed: u64,
}

#[derive(Debug, Deserialize)]
struct PairExactOutput {
    best_pair: PairExactBest,
}

#[derive(Debug, Deserialize)]
struct PairExactBest {
    j0: usize,
    j1: usize,
    x0: i64,
    x1: i64,
}

#[derive(Clone, Debug, Serialize)]
struct BestVector {
    l1: u64,
    method: String,
    support_size: usize,
    free_vector_centered: Vec<i64>,
    row_vector_centered: Vec<i64>,
}

#[derive(Clone, Debug, Serialize)]
struct SupportReport {
    support_size: usize,
    complete: bool,
    checked_restarts: u64,
    best: BestVector,
}

#[derive(Debug, Serialize)]
struct Output {
    source: String,
    family: String,
    q: u64,
    group_size: usize,
    rows: usize,
    target_l1: u64,
    alpha_start: u64,
    exponent_start: usize,
    column_scale: String,
    source_seed: u64,
    coeff_bound: i64,
    max_support: usize,
    restarts_per_support: u64,
    coordinate_rounds: usize,
    exact_column_certificate_pass: bool,
    best_overall: BestVector,
    support_reports: Vec<SupportReport>,
    notes: Vec<String>,
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

fn row_vector(matrix: &[Vec<u64>], free: &[i64], q: u64) -> Vec<i64> {
    matrix
        .iter()
        .map(|row| {
            let acc = row.iter().zip(free).fold(0u128, |acc, (&a, &x)| {
                if x == 0 {
                    acc
                } else {
                    acc + u128::from(a) * u128::from(signed_to_mod(x, q))
                }
            });
            centered_value((acc % u128::from(q)) as u64, q)
        })
        .collect()
}

fn l1(xs: &[i64]) -> u64 {
    xs.iter().map(|x| x.unsigned_abs()).sum()
}

fn evaluate(matrix: &[Vec<u64>], free: Vec<i64>, q: u64, method: &str) -> BestVector {
    let rows = row_vector(matrix, &free, q);
    BestVector {
        l1: l1(&free) + l1(&rows),
        method: method.to_string(),
        support_size: free.iter().filter(|&&x| x != 0).count(),
        free_vector_centered: free,
        row_vector_centered: rows,
    }
}

fn better(a: &BestVector, b: &BestVector) -> bool {
    a.l1 < b.l1
        || (a.l1 == b.l1 && a.support_size < b.support_size)
        || (a.l1 == b.l1 && a.support_size == b.support_size && a.method < b.method)
}

fn exact_columns(matrix: &[Vec<u64>], q: u64, target_l1: u64, group_size: usize) -> SupportReport {
    let bound = ((q / 2).min(target_l1.saturating_sub(1))) as i64;
    let mut best: Option<BestVector> = None;
    for j in 0..group_size {
        for x in 1..=bound {
            let mut free = vec![0; group_size];
            free[j] = x;
            let candidate = evaluate(matrix, free, q, "exact-column");
            if best.as_ref().is_none_or(|old| better(&candidate, old)) {
                best = Some(candidate);
            }
        }
    }
    SupportReport {
        support_size: 1,
        complete: true,
        checked_restarts: group_size as u64 * bound.max(0) as u64,
        best: best.expect("at least one column checked"),
    }
}

fn random_support(rng: &mut Rng64, group_size: usize, support_size: usize) -> Vec<usize> {
    let mut support = Vec::with_capacity(support_size);
    while support.len() < support_size.min(group_size) {
        let j = rng.modulo(group_size as u64) as usize;
        if !support.contains(&j) {
            support.push(j);
        }
    }
    support
}

fn coordinate_minimize(
    matrix: &[Vec<u64>],
    q: u64,
    group_size: usize,
    support: &[usize],
    coeff_bound: i64,
    rounds: usize,
    rng: &mut Rng64,
) -> BestVector {
    let mut free = vec![0; group_size];
    for &j in support {
        let mag = 1 + rng.modulo(coeff_bound as u64) as i64;
        let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
        free[j] = sign * mag;
    }
    let mut best = evaluate(matrix, free.clone(), q, "support-coordinate-descent");
    for _ in 0..rounds {
        let mut changed = false;
        for &j in support {
            let old = free[j];
            let mut local_best = best.clone();
            let mut local_value = old;
            for x in -coeff_bound..=coeff_bound {
                if x == 0 && support.len() == 1 {
                    continue;
                }
                free[j] = x;
                if free.iter().all(|&v| v == 0) {
                    continue;
                }
                let candidate = evaluate(matrix, free.clone(), q, "support-coordinate-descent");
                if better(&candidate, &local_best) {
                    local_best = candidate;
                    local_value = x;
                }
            }
            free[j] = local_value;
            if local_value != old {
                changed = true;
            }
            if better(&local_best, &best) {
                best = local_best;
            }
        }
        if !changed {
            break;
        }
    }
    best
}

fn coordinate_minimize_from(
    matrix: &[Vec<u64>],
    q: u64,
    support: &[usize],
    coeff_bound: i64,
    rounds: usize,
    mut free: Vec<i64>,
) -> BestVector {
    let mut best = evaluate(matrix, free.clone(), q, "anchored-coordinate-descent");
    for _ in 0..rounds {
        let mut changed = false;
        for &j in support {
            let old = free[j];
            let mut local_best = best.clone();
            let mut local_value = old;
            for x in -coeff_bound..=coeff_bound {
                free[j] = x;
                if free.iter().all(|&v| v == 0) {
                    continue;
                }
                let candidate = evaluate(matrix, free.clone(), q, "anchored-coordinate-descent");
                if better(&candidate, &local_best) {
                    local_best = candidate;
                    local_value = x;
                }
            }
            free[j] = local_value;
            if local_value != old {
                changed = true;
            }
            if better(&local_best, &best) {
                best = local_best;
            }
        }
        if !changed {
            break;
        }
    }
    best
}

fn support_search(
    matrix: &[Vec<u64>],
    q: u64,
    group_size: usize,
    support_size: usize,
    anchor: &BestVector,
    cfg: &Config,
    rng: &mut Rng64,
) -> SupportReport {
    let mut best: Option<BestVector> = Some(anchor.clone());
    let anchor_support: Vec<_> = anchor
        .free_vector_centered
        .iter()
        .enumerate()
        .filter_map(|(j, &x)| (x != 0).then_some(j))
        .collect();
    if !anchor_support.is_empty() && support_size >= anchor_support.len() {
        for _ in 0..cfg.restarts_per_support {
            let mut support = anchor_support.clone();
            while support.len() < support_size.min(group_size) {
                let j = rng.modulo(group_size as u64) as usize;
                if !support.contains(&j) {
                    support.push(j);
                }
            }
            let mut free = anchor.free_vector_centered.clone();
            for &j in &support {
                if free[j] == 0 {
                    let mag = 1 + rng.modulo(cfg.coeff_bound as u64) as i64;
                    let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
                    free[j] = sign * mag;
                }
            }
            let candidate = coordinate_minimize_from(
                matrix,
                q,
                &support,
                cfg.coeff_bound,
                cfg.coordinate_rounds,
                free,
            );
            if best.as_ref().is_none_or(|old| better(&candidate, old)) {
                best = Some(candidate);
            }
        }
    }
    for _ in 0..cfg.restarts_per_support {
        let support = random_support(rng, group_size, support_size);
        let candidate = coordinate_minimize(
            matrix,
            q,
            group_size,
            &support,
            cfg.coeff_bound,
            cfg.coordinate_rounds,
            rng,
        );
        if best.as_ref().is_none_or(|old| better(&candidate, old)) {
            best = Some(candidate);
        }
    }
    SupportReport {
        support_size,
        complete: false,
        checked_restarts: cfg.restarts_per_support * 2,
        best: best.expect("at least one restart"),
    }
}

fn parse_args() -> Config {
    let mut cfg = Config {
        input_json: PathBuf::from(
            "research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json",
        ),
        anchor_json: None,
        output_json: PathBuf::from("research/parameter_search/moment_lee_stratified.json"),
        output_md: PathBuf::from("research/parameter_search/moment_lee_stratified.md"),
        coeff_bound: 3276,
        max_support: 4,
        restarts_per_support: 256,
        coordinate_rounds: 4,
        seed: 1,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => cfg.input_json = PathBuf::from(args.next().unwrap()),
            "--anchor-json" => cfg.anchor_json = Some(PathBuf::from(args.next().unwrap())),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            "--coeff-bound" => cfg.coeff_bound = args.next().unwrap().parse().unwrap(),
            "--max-support" => cfg.max_support = args.next().unwrap().parse().unwrap(),
            "--restarts-per-support" => {
                cfg.restarts_per_support = args.next().unwrap().parse().unwrap()
            }
            "--coordinate-rounds" => cfg.coordinate_rounds = args.next().unwrap().parse().unwrap(),
            "--seed" => cfg.seed = args.next().unwrap().parse().unwrap(),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn anchor_from_pair_exact(
    path: &PathBuf,
    group_size: usize,
    matrix: &[Vec<u64>],
    q: u64,
) -> BestVector {
    let text = fs::read_to_string(path).expect("read anchor json");
    let anchor: PairExactOutput = serde_json::from_str(&text).expect("parse anchor json");
    let mut free = vec![0; group_size];
    free[anchor.best_pair.j0] = anchor.best_pair.x0;
    free[anchor.best_pair.j1] = anchor.best_pair.x1;
    evaluate(matrix, free, q, "exact-pair-anchor")
}

fn markdown(out: &Output) -> String {
    let mut lines = vec![
        "# Moment Lee Stratified Certificate Probe".to_string(),
        String::new(),
        format!("- source: `{}`", out.source),
        format!("- q: `{}`", out.q),
        format!("- group size: `{}`", out.group_size),
        format!("- rows: `{}`", out.rows),
        format!("- target l1: `{}`", out.target_l1),
        format!("- exponent start: `{}`", out.exponent_start),
        format!("- column scale: `{}`", out.column_scale),
        format!(
            "- exact column certificate pass: `{}`",
            out.exact_column_certificate_pass
        ),
        format!("- best l1 found: `{}`", out.best_overall.l1),
        String::new(),
        "| support | complete | restarts/checks | best l1 | method |".to_string(),
        "|---:|---|---:|---:|---|".to_string(),
    ];
    for report in &out.support_reports {
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            report.support_size,
            report.complete,
            report.checked_restarts,
            report.best.l1,
            report.best.method
        ));
    }
    lines.extend([
        String::new(),
        "Only support 1 is exhaustive. Higher-support rows are heuristic coordinate-descent attacks."
            .to_string(),
        "`best l1 < target l1` is a real counterexample; otherwise this is not a proof."
            .to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.coeff_bound > 0, "coefficient bound must be positive");
    let text = fs::read_to_string(&cfg.input_json).expect("read input json");
    let probe: ProbeOutput = serde_json::from_str(&text).expect("parse input json");
    let matrix = probe.coefficient_matrix_rows.clone();
    let mut reports = vec![exact_columns(
        &matrix,
        probe.q,
        probe.target_l1,
        probe.group_size,
    )];
    let anchor = cfg.anchor_json.as_ref().map_or_else(
        || reports[0].best.clone(),
        |path| anchor_from_pair_exact(path, probe.group_size, &matrix, probe.q),
    );
    let mut rng = Rng64::new(cfg.seed);
    for support_size in 2..=cfg.max_support.min(probe.group_size) {
        reports.push(support_search(
            &matrix,
            probe.q,
            probe.group_size,
            support_size,
            &anchor,
            &cfg,
            &mut rng,
        ));
    }
    let best_overall = reports
        .iter()
        .map(|r| r.best.clone())
        .min_by(|a, b| {
            if better(a, b) {
                std::cmp::Ordering::Less
            } else if better(b, a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .expect("reports are nonempty");
    let exact_column_certificate_pass = reports[0].best.l1 >= probe.target_l1;
    let out = Output {
        source: cfg.input_json.display().to_string(),
        family: probe.family,
        q: probe.q,
        group_size: probe.group_size,
        rows: probe.rows,
        target_l1: probe.target_l1,
        alpha_start: probe.alpha_start,
        exponent_start: probe.exponent_start,
        column_scale: probe.column_scale,
        source_seed: probe.seed,
        coeff_bound: cfg.coeff_bound,
        max_support: cfg.max_support,
        restarts_per_support: cfg.restarts_per_support,
        coordinate_rounds: cfg.coordinate_rounds,
        exact_column_certificate_pass,
        best_overall,
        support_reports: reports,
        notes: vec![
            "Support 1 is exhaustive over centered coefficients with abs(x) < target_l1."
                .to_string(),
            "Higher supports are heuristic coordinate-descent attacks, not completeness certificates."
                .to_string(),
            format!(
                "Source probe best was {} and found_counterexample={}.",
                probe.result.best_centered_l1, probe.result.found_counterexample
            ),
            format!(
                "Source probe free vector: {:?}",
                probe.result.free_vector_centered
            ),
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
