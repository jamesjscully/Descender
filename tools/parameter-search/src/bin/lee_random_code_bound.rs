use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Config {
    q: usize,
    target_l1: usize,
    m1: usize,
    group_sizes: Vec<usize>,
    max_rows: usize,
    failure_bits: f64,
    output_json: PathBuf,
    output_md: PathBuf,
}

#[derive(Debug, Serialize)]
struct GroupBound {
    group_size: usize,
    group_count: usize,
    min_rows_per_group: Option<usize>,
    total_rows: Option<usize>,
    keygen_mul_ops: Option<usize>,
    log2_expected_bad_per_group: Option<f64>,
    log2_expected_bad_all_groups: Option<f64>,
    rows: Vec<RowBound>,
}

#[derive(Debug, Serialize)]
struct RowBound {
    rows_per_group: usize,
    log2_expected_bad_per_group: f64,
    log2_expected_bad_all_groups: f64,
    passes_failure_budget: bool,
}

#[derive(Debug, Serialize)]
struct BoundOutput {
    q: usize,
    target_l1: usize,
    m1: usize,
    failure_bits: f64,
    family: String,
    groups: Vec<GroupBound>,
    notes: Vec<String>,
}

fn parse_csv_usize(s: &str) -> Vec<usize> {
    s.split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.trim().parse().unwrap())
        .collect()
}

fn log2_or_neg_inf(x: f64) -> f64 {
    if x == 0.0 {
        f64::NEG_INFINITY
    } else {
        x.log2()
    }
}

fn log2_sum(logs: &[f64]) -> f64 {
    let max = logs
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, |acc, x| acc.max(x));
    if !max.is_finite() {
        return max;
    }
    max + logs.iter().map(|x| 2f64.powf(*x - max)).sum::<f64>().log2()
}

fn free_lee_counts(group_size: usize, target_l1: usize, q: usize) -> Vec<f64> {
    let max_centered = (q - 1) / 2;
    let mut dist = vec![0.0; target_l1];
    dist[0] = 1.0;
    for _ in 0..group_size {
        let mut next = vec![0.0; target_l1];
        let mut prefix = vec![0.0; target_l1 + 1];
        for i in 0..target_l1 {
            prefix[i + 1] = prefix[i] + dist[i];
        }
        for t in 0..target_l1 {
            next[t] += dist[t];
            let lo = t.saturating_sub(max_centered);
            let sum_nonzero = prefix[t] - prefix[lo];
            next[t] += 2.0 * sum_nonzero;
        }
        dist = next;
    }
    dist[0] -= 1.0; // exclude the all-zero free vector.
    dist
}

fn row_sum_cdf(rows_per_group: usize, target_l1: usize, q: usize) -> Vec<f64> {
    let max_centered = (q - 1) / 2;
    let qf = q as f64;
    let mut dist = vec![0.0; target_l1];
    dist[0] = 1.0;
    for _ in 0..rows_per_group {
        let mut next = vec![0.0; target_l1];
        let mut prefix = vec![0.0; target_l1 + 1];
        for i in 0..target_l1 {
            prefix[i + 1] = prefix[i] + dist[i];
        }
        for t in 0..target_l1 {
            let lo = t.saturating_sub(max_centered);
            let sum_nonzero = prefix[t] - prefix[lo];
            next[t] = (dist[t] + 2.0 * sum_nonzero) / qf;
        }
        dist = next;
    }
    let mut cdf = vec![0.0; target_l1];
    let mut acc = 0.0;
    for i in 0..target_l1 {
        acc += dist[i];
        cdf[i] = acc.min(1.0);
    }
    cdf
}

fn expected_bad_log2(group_size: usize, rows_per_group: usize, target_l1: usize, q: usize) -> f64 {
    let free_counts = free_lee_counts(group_size, target_l1, q);
    let cdf = row_sum_cdf(rows_per_group, target_l1, q);
    let mut terms = Vec::new();
    for free_l1 in 1..target_l1 {
        let count = free_counts[free_l1];
        if count == 0.0 {
            continue;
        }
        let max_row_sum = target_l1 - 1 - free_l1;
        let probability = cdf[max_row_sum];
        if probability == 0.0 {
            continue;
        }
        terms.push(count.log2() + probability.log2());
    }
    log2_sum(&terms)
}

fn analyze_group(cfg: &Config, group_size: usize) -> GroupBound {
    assert!(group_size > 0, "group size must be positive");
    assert!(
        cfg.m1.is_multiple_of(group_size),
        "m1 must be divisible by every group size"
    );
    let group_count = cfg.m1 / group_size;
    let group_count_log2 = log2_or_neg_inf(group_count as f64);
    let budget_log2 = -cfg.failure_bits;
    let mut rows = Vec::new();
    let mut min = None;
    for rows_per_group in 1..=cfg.max_rows {
        let per_group = expected_bad_log2(group_size, rows_per_group, cfg.target_l1, cfg.q);
        let all_groups = per_group + group_count_log2;
        let passes = all_groups <= budget_log2;
        rows.push(RowBound {
            rows_per_group,
            log2_expected_bad_per_group: per_group,
            log2_expected_bad_all_groups: all_groups,
            passes_failure_budget: passes,
        });
        if passes && min.is_none() {
            min = Some((rows_per_group, per_group, all_groups));
        }
    }
    let (min_rows_per_group, per_group, all_groups) = min
        .map(|(r, p, a)| (Some(r), Some(p), Some(a)))
        .unwrap_or((None, None, None));
    GroupBound {
        group_size,
        group_count,
        min_rows_per_group,
        total_rows: min_rows_per_group.map(|r| r * group_count),
        keygen_mul_ops: min_rows_per_group.map(|r| r * group_count * group_size),
        log2_expected_bad_per_group: per_group,
        log2_expected_bad_all_groups: all_groups,
        rows,
    }
}

fn markdown(out: &BoundOutput) -> String {
    let mut lines = vec![
        "# Lee Random-Code Bound".to_string(),
        String::new(),
        format!("- q: `{}`", out.q),
        format!("- target centered Lee l1: `{}`", out.target_l1),
        format!("- m1: `{}`", out.m1),
        format!("- failure budget: `2^-{:.1}`", out.failure_bits),
        String::new(),
        "| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |"
            .to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for group in &out.groups {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            group.group_size,
            group.group_count,
            group
                .min_rows_per_group
                .map_or("-".to_string(), |x| x.to_string()),
            group.total_rows.map_or("-".to_string(), |x| x.to_string()),
            group
                .keygen_mul_ops
                .map_or("-".to_string(), |x| x.to_string()),
            group
                .log2_expected_bad_all_groups
                .map_or("-".to_string(), |x| format!("{x:.2}")),
        ));
    }
    lines.extend([
        String::new(),
        "This is a random-code design bound, not a deterministic sidecar by itself.".to_string(),
        "It predicts how many shared random rows should make low-Lee relations rare.".to_string(),
        "A concrete deployment still needs a deterministic sidecar checker for the selected family.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        q: 32771,
        target_l1: 3277,
        m1: 32,
        group_sizes: vec![1, 2, 4, 8],
        max_rows: 32,
        failure_bits: 80.0,
        output_json: PathBuf::from("research/parameter_search/lee_random_code_bound.json"),
        output_md: PathBuf::from("research/parameter_search/lee_random_code_bound.md"),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--q" => cfg.q = args.next().unwrap().parse().unwrap(),
            "--target-l1" => cfg.target_l1 = args.next().unwrap().parse().unwrap(),
            "--m1" => cfg.m1 = args.next().unwrap().parse().unwrap(),
            "--group-sizes" => cfg.group_sizes = parse_csv_usize(&args.next().unwrap()),
            "--max-rows" => cfg.max_rows = args.next().unwrap().parse().unwrap(),
            "--failure-bits" => cfg.failure_bits = args.next().unwrap().parse().unwrap(),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.q % 2 == 1, "this tool currently assumes odd q");
    let groups = cfg
        .group_sizes
        .iter()
        .map(|&group_size| analyze_group(&cfg, group_size))
        .collect();
    let out = BoundOutput {
        q: cfg.q,
        target_l1: cfg.target_l1,
        m1: cfg.m1,
        failure_bits: cfg.failure_bits,
        family: "random_shared_linear_block_union_bound".to_string(),
        groups,
        notes: vec![
            "For any fixed nonzero free vector, one random row dot product is uniform over F_q.".to_string(),
            "The bound sums Lee-ball volume by free l1 weight times the exact lower-tail probability for row Lee sums.".to_string(),
            "Expected bad count below one proves existence of a good random block; below 2^-lambda supports rejection-sampling with lambda bits of margin.".to_string(),
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
