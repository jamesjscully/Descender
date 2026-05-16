use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, mpsc};
use std::thread;

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
}

#[derive(Debug)]
struct Config {
    input_json: PathBuf,
    output_json: PathBuf,
    output_md: PathBuf,
    threads: usize,
}

#[derive(Clone, Debug, Serialize)]
struct PairBest {
    j0: usize,
    j1: usize,
    best_l1: u64,
    x0: i64,
    x1: i64,
    row_vector_centered: Vec<i64>,
    checked_assignments: u64,
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
    threads: usize,
    complete_support2: bool,
    found_counterexample: bool,
    checked_assignments: u64,
    best_pair: PairBest,
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

fn check_pair(matrix: &[Vec<u64>], q: u64, target: u64, j0: usize, j1: usize) -> PairBest {
    let bound = target.saturating_sub(1) as i64;
    let mut best = PairBest {
        j0,
        j1,
        best_l1: target,
        x0: 0,
        x1: 0,
        row_vector_centered: Vec::new(),
        checked_assignments: 0,
    };
    for ax0 in 1..=bound {
        let y_abs_max = bound - ax0;
        if y_abs_max <= 0 {
            break;
        }
        for sx0 in [-1, 1] {
            let x0 = sx0 * ax0;
            let x0m = signed_to_mod(x0, q);
            for ax1 in 1..=y_abs_max {
                let free_l1 = (ax0 + ax1) as u64;
                for sx1 in [-1, 1] {
                    best.checked_assignments += 1;
                    let x1 = sx1 * ax1;
                    let x1m = signed_to_mod(x1, q);
                    let mut total = free_l1;
                    let mut rows = Vec::with_capacity(matrix.len());
                    for row in matrix {
                        let v = (u128::from(row[j0]) * u128::from(x0m)
                            + u128::from(row[j1]) * u128::from(x1m))
                            % u128::from(q);
                        let centered = centered_value(v as u64, q);
                        rows.push(centered);
                        total = total.saturating_add(centered.unsigned_abs());
                        if total >= target {
                            break;
                        }
                    }
                    if rows.len() == matrix.len() && total < best.best_l1 {
                        best = PairBest {
                            j0,
                            j1,
                            best_l1: total,
                            x0,
                            x1,
                            row_vector_centered: rows,
                            checked_assignments: best.checked_assignments,
                        };
                    }
                }
            }
        }
    }
    best
}

fn parse_args() -> Config {
    let mut cfg = Config {
        input_json: PathBuf::from(
            "research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json",
        ),
        output_json: PathBuf::from("research/parameter_search/moment_lee_pair_exact.json"),
        output_md: PathBuf::from("research/parameter_search/moment_lee_pair_exact.md"),
        threads: thread::available_parallelism().map_or(1, |n| n.get()),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => cfg.input_json = PathBuf::from(args.next().unwrap()),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            "--threads" => cfg.threads = args.next().unwrap().parse().unwrap(),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn markdown(out: &Output) -> String {
    [
        "# Moment Lee Exact Support-2 Scan".to_string(),
        String::new(),
        format!("- source: `{}`", out.source),
        format!("- q: `{}`", out.q),
        format!("- group size: `{}`", out.group_size),
        format!("- rows: `{}`", out.rows),
        format!("- target l1: `{}`", out.target_l1),
        format!("- threads: `{}`", out.threads),
        format!("- complete support 2: `{}`", out.complete_support2),
        format!("- checked assignments: `{}`", out.checked_assignments),
        format!("- found counterexample: `{}`", out.found_counterexample),
        format!(
            "- best full below-target pair, if any: `({}, {})`, coefficients `({}, {})`, l1 `{}`",
            out.best_pair.j0, out.best_pair.j1, out.best_pair.x0, out.best_pair.x1, out.best_pair.best_l1
        ),
        String::new(),
        "This scan is exhaustive over support exactly 2 and centered coefficient l1 below the target.".to_string(),
        String::new(),
    ]
    .join("\n")
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.threads > 0, "threads must be positive");
    let text = fs::read_to_string(&cfg.input_json).expect("read input json");
    let probe: ProbeOutput = serde_json::from_str(&text).expect("parse input json");
    let pairs: Vec<_> = (0..probe.group_size)
        .flat_map(|j0| ((j0 + 1)..probe.group_size).map(move |j1| (j0, j1)))
        .collect();
    let matrix = Arc::new(probe.coefficient_matrix_rows.clone());
    let (tx, rx) = mpsc::channel();
    for worker in 0..cfg.threads {
        let tx = tx.clone();
        let pairs = pairs.clone();
        let matrix = Arc::clone(&matrix);
        let q = probe.q;
        let target = probe.target_l1;
        let threads = cfg.threads;
        thread::spawn(move || {
            for (idx, &(j0, j1)) in pairs.iter().enumerate() {
                if idx % threads != worker {
                    continue;
                }
                let result = check_pair(&matrix, q, target, j0, j1);
                tx.send(result).expect("send pair result");
            }
        });
    }
    drop(tx);
    let mut checked_assignments = 0;
    let mut best: Option<PairBest> = None;
    for result in rx {
        checked_assignments += result.checked_assignments;
        if best.as_ref().is_none_or(|old| result.best_l1 < old.best_l1) {
            best = Some(result);
        }
    }
    let best_pair = best.expect("at least one pair");
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
        threads: cfg.threads,
        complete_support2: true,
        found_counterexample: best_pair.best_l1 < probe.target_l1,
        checked_assignments,
        best_pair,
        notes: vec![
            "Support exactly 2 is exhaustive for centered coefficient l1 below target_l1."
                .to_string(),
            "Support 1 is handled by moment_lee_stratified exact-column enumeration.".to_string(),
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
