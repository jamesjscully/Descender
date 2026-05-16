use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Config {
    input_json: PathBuf,
    output_json: PathBuf,
    output_md: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct CertificateOutput {
    family: String,
    q: u64,
    n: usize,
    m1: usize,
    group_size: usize,
    group_count: usize,
    rows_per_group: usize,
    used_rows: usize,
    unused_rows: usize,
    target_l1: u64,
    passes_target: bool,
    certified_l1: u64,
    selected_trial: usize,
    pair_block: PairBlockCertificate,
}

#[derive(Clone, Debug, Deserialize)]
struct PairBlockCertificate {
    coefficients: Vec<[u64; 2]>,
    passes_target: bool,
    certified_l1: u64,
    checked_centered_pair_count: u64,
    worst_pair_centered: [i64; 2],
    worst_relation_centered: Vec<i64>,
    worst_relation_support: usize,
}

#[derive(Clone, Debug, Serialize)]
struct VerificationOutput {
    input_json: String,
    family: String,
    q: u64,
    n: usize,
    m1: usize,
    group_size: usize,
    group_count: usize,
    rows_per_group: usize,
    used_rows: usize,
    target_l1: u64,
    selected_trial: usize,
    artifact_passes_target: bool,
    verifier_passes_target: bool,
    checked_centered_pair_count: u64,
    artifact_checked_centered_pair_count: u64,
    min_l1: u64,
    worst_pair_centered: [i64; 2],
    worst_relation_centered: Vec<i64>,
    worst_relation_support: usize,
    artifact_worst_pair_centered: [i64; 2],
    artifact_worst_relation_centered: Vec<i64>,
    artifact_worst_relation_support: usize,
    artifact_consistent: bool,
    errors: Vec<String>,
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
        if y == 0 {
            0
        } else {
            q - y
        }
    }
}

fn relation_for_pair(q: u64, block: &[[u64; 2]], x: i64, y: i64) -> Vec<i64> {
    let xm = signed_to_mod(x, q);
    let ym = signed_to_mod(y, q);
    let mut relation = Vec::with_capacity(block.len() + 2);
    for &[a, b] in block {
        let v = (u128::from(a) * u128::from(xm) + u128::from(b) * u128::from(ym)) % u128::from(q);
        relation.push(centered_value(v as u64, q));
    }
    relation.push(x);
    relation.push(y);
    relation
}

fn relation_l1(relation: &[i64]) -> u64 {
    relation.iter().map(|x| x.unsigned_abs()).sum()
}

fn verify(cert: &CertificateOutput, input_json: String) -> VerificationOutput {
    let mut errors = Vec::new();

    if cert.q <= 2 {
        errors.push("q must be greater than 2".to_string());
    }
    if cert.group_size != 2 {
        errors.push(format!("expected group_size=2, got {}", cert.group_size));
    }
    if cert.m1 != cert.group_count * cert.group_size {
        errors.push("m1 does not equal group_count * group_size".to_string());
    }
    if cert.used_rows != cert.group_count * cert.rows_per_group {
        errors.push("used_rows does not equal group_count * rows_per_group".to_string());
    }
    if cert.n != cert.used_rows + cert.unused_rows {
        errors.push("n does not equal used_rows + unused_rows".to_string());
    }
    if cert.pair_block.coefficients.len() != cert.rows_per_group {
        errors.push("coefficient row count does not match rows_per_group".to_string());
    }
    for (i, [a, b]) in cert.pair_block.coefficients.iter().copied().enumerate() {
        if a >= cert.q || b >= cert.q {
            errors.push(format!("coefficient row {i} has entry outside Z/qZ"));
        }
    }

    let radius = cert.target_l1 as i64 - 1;
    let mut checked = 0u64;
    let mut min_l1 = u64::MAX;
    let mut worst_pair = [0, 0];
    let mut worst_relation = Vec::new();
    let mut found_counterexample = false;

    for x in -radius..=radius {
        let x_abs = x.unsigned_abs();
        for y in -radius..=radius {
            if x == 0 && y == 0 {
                continue;
            }
            let free_l1 = x_abs + y.unsigned_abs();
            if free_l1 >= cert.target_l1 {
                continue;
            }
            checked += 1;
            let relation = relation_for_pair(cert.q, &cert.pair_block.coefficients, x, y);
            let l1 = relation_l1(&relation);
            if l1 < min_l1 {
                min_l1 = l1;
                worst_pair = [x, y];
                worst_relation = relation;
            }
            if l1 < cert.target_l1 {
                found_counterexample = true;
            }
        }
    }

    if min_l1 == u64::MAX {
        errors.push("no centered pairs were checked".to_string());
        min_l1 = 0;
    }

    let verifier_passes_target = !found_counterexample && min_l1 >= cert.target_l1;
    let worst_relation_support = worst_relation.iter().filter(|&&z| z != 0).count();

    let artifact_consistent = verifier_passes_target == cert.passes_target
        && verifier_passes_target == cert.pair_block.passes_target
        && cert.certified_l1 == cert.target_l1
        && cert.pair_block.certified_l1 == cert.target_l1
        && checked == cert.pair_block.checked_centered_pair_count
        && worst_pair == cert.pair_block.worst_pair_centered
        && worst_relation == cert.pair_block.worst_relation_centered
        && worst_relation_support == cert.pair_block.worst_relation_support;

    if !artifact_consistent {
        errors.push("artifact fields do not match independent recomputation".to_string());
    }

    VerificationOutput {
        input_json,
        family: cert.family.clone(),
        q: cert.q,
        n: cert.n,
        m1: cert.m1,
        group_size: cert.group_size,
        group_count: cert.group_count,
        rows_per_group: cert.rows_per_group,
        used_rows: cert.used_rows,
        target_l1: cert.target_l1,
        selected_trial: cert.selected_trial,
        artifact_passes_target: cert.passes_target,
        verifier_passes_target,
        checked_centered_pair_count: checked,
        artifact_checked_centered_pair_count: cert.pair_block.checked_centered_pair_count,
        min_l1,
        worst_pair_centered: worst_pair,
        worst_relation_centered: worst_relation,
        worst_relation_support,
        artifact_worst_pair_centered: cert.pair_block.worst_pair_centered,
        artifact_worst_relation_centered: cert.pair_block.worst_relation_centered.clone(),
        artifact_worst_relation_support: cert.pair_block.worst_relation_support,
        artifact_consistent,
        errors,
    }
}

fn markdown(out: &VerificationOutput) -> String {
    let mut s = format!(
        "# Shared Lee Certificate Verification\n\n\
         | field | value |\n\
         |---|---:|\n\
         | input | `{}` |\n\
         | q | {} |\n\
         | n | {} |\n\
         | m1 | {} |\n\
         | group count | {} |\n\
         | rows per group | {} |\n\
         | target l1 | {} |\n\
         | artifact passes target | {} |\n\
         | verifier passes target | {} |\n\
         | artifact consistent | {} |\n\
         | checked centered pairs | {} |\n\
         | min l1 | {} |\n\
         | worst pair | ({}, {}) |\n\
         | worst relation support | {} |\n\n\
         Worst relation:\n\n```text\n{:?}\n```\n",
        out.input_json,
        out.q,
        out.n,
        out.m1,
        out.group_count,
        out.rows_per_group,
        out.target_l1,
        out.artifact_passes_target,
        out.verifier_passes_target,
        out.artifact_consistent,
        out.checked_centered_pair_count,
        out.min_l1,
        out.worst_pair_centered[0],
        out.worst_pair_centered[1],
        out.worst_relation_support,
        out.worst_relation_centered,
    );
    if !out.errors.is_empty() {
        s.push_str("\nErrors:\n\n");
        for err in &out.errors {
            s.push_str(&format!("- {err}\n"));
        }
    }
    s
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        input_json: PathBuf::from(
            "research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json",
        ),
        output_json: PathBuf::from(
            "research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.json",
        ),
        output_md: PathBuf::from(
            "research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.md",
        ),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => cfg.input_json = PathBuf::from(args.next().unwrap()),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    let input = fs::read_to_string(&cfg.input_json).expect("read input json");
    let cert: CertificateOutput = serde_json::from_str(&input).expect("parse input json");
    let out = verify(&cert, cfg.input_json.display().to_string());
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
    .expect("write output json");
    fs::write(&cfg.output_md, markdown(&out)).expect("write output markdown");
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
    if !out.artifact_consistent || !out.verifier_passes_target {
        std::process::exit(1);
    }
}
