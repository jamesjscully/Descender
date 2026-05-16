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
    rows_per_group: usize,
    trials: usize,
    seed: u64,
    target_l1: u64,
    output_json: PathBuf,
    output_md: PathBuf,
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

fn certify(cfg: &Config) -> CertificateOutput {
    assert!(cfg.q > 2, "q must be greater than 2");
    assert!(
        cfg.m1 > 0 && cfg.m1.is_multiple_of(2),
        "m1 must be positive and even"
    );
    assert!(cfg.rows_per_group > 0, "rows per group must be positive");
    let group_size = 2;
    let group_count = cfg.m1 / group_size;
    let used_rows = group_count * cfg.rows_per_group;
    assert!(used_rows <= cfg.n, "rows_per_group * m1 / 2 must fit in n");

    let mut rng = Rng64::new(cfg.seed);
    let mut best: Option<(usize, PairBlockCertificate)> = None;
    for trial in 0..cfg.trials.max(1) {
        let block = random_pair_block(cfg.q, cfg.rows_per_group, &mut rng);
        let cert = verify_pair_block(cfg.q, block, cfg.target_l1);
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
        q: cfg.q,
        n: cfg.n,
        m1: cfg.m1,
        group_size,
        group_count,
        rows_per_group: cfg.rows_per_group,
        used_rows,
        unused_rows: cfg.n - used_rows,
        target_l1: cfg.target_l1,
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
            "Certificate covers a block-diagonal direct sum of identical two-coordinate shared-row blocks.".to_string(),
            "Each local pivot row reads both local free coordinates, so rows are shared within a pair but not across blocks.".to_string(),
            "Pass means every centered local pair with free l1 below target was checked and none had total Lee l1 below target.".to_string(),
            "For the direct-sum global matrix, nonzero global y activates at least one local block and the local Lee norm is a sub-sum of the global Lee norm.".to_string(),
            "That local-to-global step is represented in Lean by productBlockComposableSystematicLeeCertificate.".to_string(),
        ],
    }
}

fn markdown(out: &CertificateOutput) -> String {
    format!(
        "# Shared Row Lee Certificate\n\n\
         | field | value |\n\
         |---|---:|\n\
         | family | {} |\n\
         | q | {} |\n\
         | n | {} |\n\
         | m1 | {} |\n\
         | group size | {} |\n\
         | group count | {} |\n\
         | rows per group | {} |\n\
         | used pivot rows | {} |\n\
         | unused pivot rows | {} |\n\
         | target centered Lee l1 | {} |\n\
         | certified centered Lee l1 | {} |\n\
         | passes target | {} |\n\
         | global systematic Lee certified | {} |\n\
         | keygen multiply ops | {} |\n\
         | selected trial | {} |\n\
         | checked centered pairs | {} |\n\
         | worst pair | ({}, {}) |\n\
         | worst relation support | {} |\n\
         | Lean route | {} |\n\n\
         Exact check: `{}`\n\n\
         Global statement: {}\n\n\
         Coefficients for the selected two-free-coordinate block:\n\n\
         ```json\n{}\n```\n",
        out.family,
        out.q,
        out.n,
        out.m1,
        out.group_size,
        out.group_count,
        out.rows_per_group,
        out.used_rows,
        out.unused_rows,
        out.target_l1,
        out.certified_l1,
        out.passes_target,
        out.global_systematic_lee_distance_certified,
        out.keygen_mul_ops,
        out.selected_trial,
        out.pair_block.checked_centered_pair_count,
        out.pair_block.worst_pair_centered[0],
        out.pair_block.worst_pair_centered[1],
        out.pair_block.worst_relation_support,
        out.lean_route,
        out.exact_check,
        out.global_certificate_statement,
        serde_json::to_string_pretty(&out.pair_block.coefficients).unwrap()
    )
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        q: 32771,
        n: 64,
        m1: 32,
        rows_per_group: 4,
        trials: 16,
        seed: 1,
        target_l1: 3277,
        output_json: PathBuf::from("research/parameter_search/shared_lee_cert.json"),
        output_md: PathBuf::from("research/parameter_search/shared_lee_cert.md"),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--q" => cfg.q = args.next().unwrap().parse().unwrap(),
            "--n" => cfg.n = args.next().unwrap().parse().unwrap(),
            "--m1" => cfg.m1 = args.next().unwrap().parse().unwrap(),
            "--rows-per-group" => cfg.rows_per_group = args.next().unwrap().parse().unwrap(),
            "--trials" => cfg.trials = args.next().unwrap().parse().unwrap(),
            "--seed" => cfg.seed = args.next().unwrap().parse().unwrap(),
            "--target-l1" => cfg.target_l1 = args.next().unwrap().parse().unwrap(),
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
