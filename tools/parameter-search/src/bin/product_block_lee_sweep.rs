use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

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
    q_values: Vec<u64>,
    target_l1_values: Vec<u64>,
    eta_values: Vec<u64>,
    m1: usize,
    rows_per_group_values: Vec<usize>,
    trials: usize,
    local_search_steps: usize,
    seed: u64,
    output_dir: PathBuf,
    emit_failures: bool,
}

#[derive(Clone, Debug, Serialize)]
struct PairBlockCertificate {
    coefficients: Vec<[u64; 2]>,
    passes_target: bool,
    certified_l1: u64,
    checked_centered_pair_count: u64,
    worst_pair_centered: [i64; 2],
    worst_relation_centered: Vec<i64>,
    worst_relation_support: usize,
}

#[derive(Debug, Serialize)]
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
    keygen_mul_ops: usize,
    selected_trial: usize,
    exact_check: String,
    lean_route: String,
    global_systematic_lee_distance_certified: bool,
    local_certificate_scope: String,
    global_certificate_statement: String,
    pair_block: PairBlockCertificate,
    notes: Vec<String>,
}

#[derive(Debug, Serialize)]
struct SweepEntry {
    q: u64,
    target_l1: u64,
    eta: Option<u64>,
    m1: usize,
    rows_per_group: usize,
    n: usize,
    used_rows: usize,
    keygen_mul_ops: usize,
    passes_target: bool,
    certified_l1: u64,
    selected_trial: usize,
    checked_centered_pair_count: u64,
    sidecar_json: Option<String>,
    validator_args: Option<String>,
}

#[derive(Debug, Serialize)]
struct SweepManifest {
    family: String,
    m1: usize,
    group_size: usize,
    trials_per_point: usize,
    local_search_steps: usize,
    seed: u64,
    q_values: Vec<u64>,
    target_l1_values: Vec<u64>,
    eta_values: Vec<u64>,
    rows_per_group_values: Vec<usize>,
    entries: Vec<SweepEntry>,
    notes: Vec<String>,
}

fn parse_csv_u64(s: &str) -> Vec<u64> {
    s.split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.trim().parse().unwrap())
        .collect()
}

fn parse_csv_usize(s: &str) -> Vec<usize> {
    s.split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.trim().parse().unwrap())
        .collect()
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

fn random_pair_block(q: u64, rows_per_group: usize, rng: &mut Rng64) -> Vec<[u64; 2]> {
    (0..rows_per_group)
        .map(|_| [rng.nonzero_mod(q), rng.nonzero_mod(q)])
        .collect()
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

fn verify_pair_block(q: u64, block: Vec<[u64; 2]>, target_l1: u64) -> PairBlockCertificate {
    let radius = target_l1 as i64 - 1;
    let mut checked = 0u64;
    let mut best_l1 = u64::MAX;
    let mut best_pair = [0, 0];
    let mut best_relation = Vec::new();

    for x in -radius..=radius {
        let x_abs = x.unsigned_abs();
        for y in -radius..=radius {
            if x == 0 && y == 0 {
                continue;
            }
            let free_l1 = x_abs + y.unsigned_abs();
            if free_l1 >= target_l1 {
                continue;
            }
            checked += 1;
            let xm = signed_to_mod(x, q);
            let ym = signed_to_mod(y, q);
            let mut total = free_l1;
            let mut relation_prefix = Vec::with_capacity(block.len() + 2);
            for &[a, b] in &block {
                let v = (u128::from(a) * u128::from(xm) + u128::from(b) * u128::from(ym))
                    % u128::from(q);
                let centered = centered_value(v as u64, q);
                total += centered.unsigned_abs();
                relation_prefix.push(centered);
                if total >= target_l1 && total >= best_l1 {
                    break;
                }
            }
            if total < best_l1 {
                relation_prefix.push(x);
                relation_prefix.push(y);
                best_l1 = total;
                best_pair = [x, y];
                best_relation = relation_prefix;
                if best_l1 < target_l1 {
                    return PairBlockCertificate {
                        coefficients: block,
                        passes_target: false,
                        certified_l1: best_l1,
                        checked_centered_pair_count: checked,
                        worst_pair_centered: best_pair,
                        worst_relation_support: best_relation.iter().filter(|&&z| z != 0).count(),
                        worst_relation_centered: best_relation,
                    };
                }
            }
        }
    }

    if best_relation.is_empty() {
        best_relation = relation_for_pair(q, &block, 1, 0);
        best_pair = [1, 0];
    }

    PairBlockCertificate {
        coefficients: block,
        passes_target: true,
        certified_l1: target_l1,
        checked_centered_pair_count: checked,
        worst_pair_centered: best_pair,
        worst_relation_support: best_relation.iter().filter(|&&z| z != 0).count(),
        worst_relation_centered: best_relation,
    }
}

fn repair_block(
    q: u64,
    block: &[[u64; 2]],
    cert: &PairBlockCertificate,
    step: usize,
    rng: &mut Rng64,
) -> Vec<[u64; 2]> {
    let mut candidate = block.to_vec();
    let row = step % candidate.len();
    let x = cert.worst_pair_centered[0];
    let y = cert.worst_pair_centered[1];
    if x == 0 && y == 0 {
        candidate[row] = [rng.nonzero_mod(q), rng.nonzero_mod(q)];
        return candidate;
    }

    let mut best = candidate[row];
    let mut best_abs = 0u64;
    for _ in 0..16 {
        let proposed = [rng.nonzero_mod(q), rng.nonzero_mod(q)];
        let xm = signed_to_mod(x, q);
        let ym = signed_to_mod(y, q);
        let v = (u128::from(proposed[0]) * u128::from(xm)
            + u128::from(proposed[1]) * u128::from(ym))
            % u128::from(q);
        let centered = centered_value(v as u64, q);
        let abs = centered.unsigned_abs();
        if abs > best_abs {
            best_abs = abs;
            best = proposed;
        }
    }
    candidate[row] = best;
    candidate
}

fn certify_point(
    q: u64,
    target_l1: u64,
    m1: usize,
    rows_per_group: usize,
    trials: usize,
    local_search_steps: usize,
    seed: u64,
) -> CertificateOutput {
    assert!(q > 2, "q must be greater than 2");
    assert!(m1 > 0 && m1.is_multiple_of(2), "m1 must be positive and even");
    assert!(rows_per_group > 0, "rows per group must be positive");
    let group_size = 2;
    let group_count = m1 / group_size;
    let used_rows = group_count * rows_per_group;
    let n = used_rows;
    let mut rng = Rng64::new(seed);
    let mut best: Option<(usize, PairBlockCertificate)> = None;
    for trial in 0..trials.max(1) {
        let mut block = random_pair_block(q, rows_per_group, &mut rng);
        let mut cert = verify_pair_block(q, block.clone(), target_l1);
        for step in 0..local_search_steps {
            if cert.passes_target {
                break;
            }
            let repaired = repair_block(q, &block, &cert, step, &mut rng);
            let repaired_cert = verify_pair_block(q, repaired.clone(), target_l1);
            if repaired_cert.passes_target || repaired_cert.certified_l1 > cert.certified_l1 {
                block = repaired;
                cert = repaired_cert;
            }
        }
        if cert.passes_target {
            best = Some((trial, cert));
            break;
        }
        if best
            .as_ref()
            .is_none_or(|(_, current)| cert.certified_l1 > current.certified_l1)
        {
            best = Some((trial, cert));
        }
    }
    let (selected_trial, pair_block) = best.expect("at least one trial");
    CertificateOutput {
        family: "shared-random-pair-block".to_string(),
        q,
        n,
        m1,
        group_size,
        group_count,
        rows_per_group,
        used_rows,
        unused_rows: 0,
        target_l1,
        passes_target: pair_block.passes_target,
        certified_l1: pair_block.certified_l1,
        keygen_mul_ops: used_rows * group_size,
        selected_trial,
        exact_check: "enumerated_all_centered_pairs_with_free_l1_below_target".to_string(),
        lean_route: "productBlockComposableSystematicLeeCertificate -> composableSystematicLeeDualCertificate".to_string(),
        global_systematic_lee_distance_certified: pair_block.passes_target,
        local_certificate_scope:
            "all nonzero centered local pairs with local free l1 below target_l1".to_string(),
        global_certificate_statement: format!(
            "For the direct sum of {group_count} identical certified pair blocks, every nonzero global free vector activates a certified local slice; therefore SystematicLeeDistanceAtLeast q P target_l1 holds globally when passes_target is true."
        ),
        pair_block,
        notes: vec![
            "Generated by product_block_lee_sweep for the sidecar library.".to_string(),
            "Certificate covers a block-diagonal direct sum of identical two-coordinate shared-row blocks.".to_string(),
            "The validator accepts this sidecar for product-block-lee rows with matching q, m1, target_l1, and used_rows.".to_string(),
        ],
    }
}

fn sidecar_path(output_dir: &Path, q: u64, m1: usize, target_l1: u64, rows: usize) -> PathBuf {
    output_dir.join(format!(
        "q{q}_m1{m1}_target{target_l1}_rows_per_group{rows}.json"
    ))
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn manifest_markdown(manifest: &SweepManifest) -> String {
    let mut lines = vec![
        "# Product-Block Lee Sidecar Sweep".to_string(),
        String::new(),
        format!("- family: `{}`", manifest.family),
        format!("- m1: `{}`", manifest.m1),
        format!("- group size: `{}`", manifest.group_size),
        format!("- trials per point: `{}`", manifest.trials_per_point),
        format!("- seed: `{}`", manifest.seed),
        String::new(),
        "| q | eta | target l1 | rows/group | used rows | qamp ops | pass | trial | checked pairs | sidecar |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---|---:|---:|---|".to_string(),
    ];
    for entry in &manifest.entries {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            entry.q,
            entry.eta.map_or("-".to_string(), |x| x.to_string()),
            entry.target_l1,
            entry.rows_per_group,
            entry.used_rows,
            entry.keygen_mul_ops,
            entry.passes_target,
            entry.selected_trial,
            entry.checked_centered_pair_count,
            entry.sidecar_json.as_deref().unwrap_or("-"),
        ));
    }
    lines.push(String::new());
    lines.extend(manifest.notes.iter().cloned());
    lines.push(String::new());
    lines.join("\n")
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        q_values: vec![32771],
        target_l1_values: vec![3277],
        eta_values: Vec::new(),
        m1: 32,
        rows_per_group_values: vec![4, 5, 6],
        trials: 16,
        local_search_steps: 0,
        seed: 1,
        output_dir: PathBuf::from("research/parameter_search/product_block_lee_sidecars"),
        emit_failures: false,
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--q-values" | "--q" => cfg.q_values = parse_csv_u64(&args.next().unwrap()),
            "--target-l1-values" | "--target-l1" => {
                cfg.target_l1_values = parse_csv_u64(&args.next().unwrap())
            }
            "--eta-values" | "--etas" => cfg.eta_values = parse_csv_u64(&args.next().unwrap()),
            "--m1" => cfg.m1 = args.next().unwrap().parse().unwrap(),
            "--rows-per-group-values" | "--rows-per-group" => {
                cfg.rows_per_group_values = parse_csv_usize(&args.next().unwrap())
            }
            "--trials" => cfg.trials = args.next().unwrap().parse().unwrap(),
            "--local-search-steps" => cfg.local_search_steps = args.next().unwrap().parse().unwrap(),
            "--seed" => cfg.seed = args.next().unwrap().parse().unwrap(),
            "--output-dir" => cfg.output_dir = PathBuf::from(args.next().unwrap()),
            "--emit-failures" => cfg.emit_failures = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn print_help() {
    println!(
        "Product-block Lee sidecar sweep\n\
         \n\
         Options:\n\
           --q-values CSV\n\
           --target-l1-values CSV\n\
           --eta-values CSV             derive target_l1 = ceil((q - 1) / (2 eta)) per q\n\
           --m1 N\n\
           --rows-per-group-values CSV\n\
           --trials N\n\
           --local-search-steps N      repair worst failing local pairs within each random trial\n\
           --seed N\n\
           --output-dir PATH\n\
           --emit-failures"
    );
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.m1 > 0 && cfg.m1.is_multiple_of(2), "m1 must be positive and even");
    fs::create_dir_all(&cfg.output_dir).expect("create output dir");
    let mut entries = Vec::new();
    for &q in &cfg.q_values {
        let targets: Vec<(u64, Option<u64>)> = if cfg.eta_values.is_empty() {
            cfg.target_l1_values.iter().map(|&t| (t, None)).collect()
        } else {
            cfg.eta_values
                .iter()
                .map(|&eta| {
                    assert!(eta > 0, "eta values must be positive");
                    ((q.saturating_sub(1)).div_ceil(2 * eta), Some(eta))
                })
                .collect()
        };
        for (target_l1, eta) in targets {
            for &rows_per_group in &cfg.rows_per_group_values {
                let point_seed = cfg.seed
                    ^ q.rotate_left(7)
                    ^ target_l1.rotate_left(17)
                    ^ (rows_per_group as u64).rotate_left(29);
                let cert = certify_point(
                    q,
                    target_l1,
                    cfg.m1,
                    rows_per_group,
                    cfg.trials,
                    cfg.local_search_steps,
                    point_seed,
                );
                let path = sidecar_path(&cfg.output_dir, q, cfg.m1, target_l1, rows_per_group);
                let should_write = cert.passes_target || cfg.emit_failures;
                let sidecar_json = if should_write {
                    fs::write(&path, serde_json::to_string_pretty(&cert).unwrap() + "\n")
                        .expect("write sidecar");
                    Some(display_path(&path))
                } else {
                    None
                };
                let validator_args = sidecar_json
                    .as_ref()
                    .filter(|_| cert.passes_target)
                    .map(|p| format!("--shared-lee-cert-json {p}"));
                entries.push(SweepEntry {
                    q,
                    target_l1,
                    eta,
                    m1: cfg.m1,
                    rows_per_group,
                    n: cert.n,
                    used_rows: cert.used_rows,
                    keygen_mul_ops: cert.keygen_mul_ops,
                    passes_target: cert.passes_target,
                    certified_l1: cert.certified_l1,
                    selected_trial: cert.selected_trial,
                    checked_centered_pair_count: cert.pair_block.checked_centered_pair_count,
                    sidecar_json,
                    validator_args,
                });
                println!(
                    "q={q} target={target_l1} rows/group={rows_per_group} pass={} trial={}",
                    cert.passes_target, cert.selected_trial
                );
            }
        }
    }
    let manifest = SweepManifest {
        family: "shared-random-pair-block".to_string(),
        m1: cfg.m1,
        group_size: 2,
        trials_per_point: cfg.trials,
        local_search_steps: cfg.local_search_steps,
        seed: cfg.seed,
        q_values: cfg.q_values,
        target_l1_values: cfg.target_l1_values,
        eta_values: cfg.eta_values,
        rows_per_group_values: cfg.rows_per_group_values,
        entries,
        notes: vec![
            "Passing entries are exact sidecars for product-block-lee validation.".to_string(),
            "The local check enumerates every nonzero centered pair whose free l1 is below target_l1.".to_string(),
            "Rows/group controls the proof-object cost: used_rows = rows_per_group * (m1 / 2).".to_string(),
        ],
    };
    let json_path = cfg.output_dir.join("manifest.json");
    let md_path = cfg.output_dir.join("manifest.md");
    fs::write(&json_path, serde_json::to_string_pretty(&manifest).unwrap() + "\n")
        .expect("write manifest json");
    fs::write(&md_path, manifest_markdown(&manifest)).expect("write manifest md");
    println!("wrote {}", json_path.display());
    println!("wrote {}", md_path.display());
}
