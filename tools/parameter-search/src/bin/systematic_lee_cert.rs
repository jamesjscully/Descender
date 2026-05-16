use serde::{Deserialize, Serialize};
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

    fn unit_mod(&mut self, q: u64) -> u64 {
        1 + self.modulo(q - 1)
    }
}

#[derive(Debug)]
struct Config {
    input_json: PathBuf,
    output_json: PathBuf,
    output_md: PathBuf,
    max_candidates: usize,
    max_support: usize,
    coeff_bound: i64,
    emit_matrix: bool,
    basis_dir: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
struct SearchOutput {
    q: u64,
    pivot_rows: usize,
    free_cols: usize,
    eta: u64,
    target_l1: u64,
    candidates: Vec<CandidateReport>,
}

#[derive(Clone, Debug, Deserialize)]
struct CandidateReport {
    descriptor: MatrixDescriptor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MatrixDescriptor {
    family: String,
    seed: u64,
    q: u64,
    pivot_rows: usize,
    free_cols: usize,
    #[serde(default)]
    block_group_size: Option<usize>,
    #[serde(default)]
    seed_bits: u64,
    #[serde(default)]
    public_sample_bits: u64,
    #[serde(default)]
    mul_work: u64,
    #[serde(default)]
    certificate_work: u64,
}

#[derive(Clone, Debug, Serialize)]
struct BestRelation {
    centered_l1: u64,
    support: usize,
    free_l1: u64,
    free_support: usize,
    free_vector_centered: Vec<i64>,
    relation_prefix_centered: Vec<i64>,
}

#[derive(Clone, Debug, Serialize)]
struct BoundedCertificate {
    descriptor: MatrixDescriptor,
    target_l1: u64,
    bounded_pass: bool,
    found_counterexample: bool,
    certified_lower_bound_in_domain: u64,
    checked_vector_count: u64,
    complete_for_support_at_most: usize,
    complete_for_coeff_abs_at_most: i64,
    complete_for_free_l1_below: u64,
    best_relation: BestRelation,
    matrix_rows: Option<Vec<Vec<u64>>>,
    basis_file: Option<String>,
    notes: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CertificateOutput {
    source: String,
    family: String,
    q: u64,
    pivot_rows: usize,
    free_cols: usize,
    eta: u64,
    target_l1: u64,
    max_candidates: usize,
    max_support: usize,
    coeff_bound: i64,
    candidates_checked: usize,
    certificates: Vec<BoundedCertificate>,
    notes: Vec<String>,
}

#[derive(Clone)]
struct Best {
    l1: u64,
    free: Vec<i64>,
    relation: Vec<i64>,
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

fn relation_from_support(
    matrix: &[Vec<u64>],
    support: &[usize],
    free: &[i64],
    q: u64,
) -> (u64, Vec<i64>) {
    let mut relation = Vec::with_capacity(matrix.len() + free.len());
    let mut total = support.iter().map(|&j| free[j].unsigned_abs()).sum();
    for row in matrix {
        let mut acc = 0u128;
        for &j in support {
            let a = row[j];
            let x = free[j];
            if x != 0 {
                acc += u128::from(a) * u128::from(signed_to_mod(x, q));
            }
        }
        let centered = centered_value((acc % u128::from(q)) as u64, q);
        total += centered.unsigned_abs();
        relation.push(centered);
    }
    relation.extend_from_slice(free);
    (total, relation)
}

fn maybe_update(
    best: &mut Option<Best>,
    matrix: &[Vec<u64>],
    support: &[usize],
    free: &[i64],
    q: u64,
) {
    if support.is_empty() {
        return;
    }
    let (relation_l1, relation) = relation_from_support(matrix, support, free, q);
    if best.as_ref().is_none_or(|old| relation_l1 < old.l1) {
        *best = Some(Best {
            l1: relation_l1,
            free: free.to_vec(),
            relation,
        });
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

fn generate_matrix(d: &MatrixDescriptor) -> Vec<Vec<u64>> {
    match d.family.as_str() {
        "dense-random" => dense_random_matrix(d.q, d.pivot_rows, d.free_cols, d.seed),
        "block-random" => block_random_matrix(
            d.q,
            d.pivot_rows,
            d.free_cols,
            d.block_group_size.unwrap_or(8),
            d.seed,
        ),
        "toeplitz-scrambled" => toeplitz_scrambled_matrix(d.q, d.pivot_rows, d.free_cols, d.seed),
        "vandermonde-scrambled" => {
            vandermonde_scrambled_matrix(d.q, d.pivot_rows, d.free_cols, d.seed)
        }
        other => panic!("unknown Lee family {other:?}"),
    }
}

fn enumerate_coefficients(
    matrix: &[Vec<u64>],
    q: u64,
    target_l1: u64,
    coeff_bound: i64,
    support: &[usize],
    pos: usize,
    free_l1: u64,
    free: &mut [i64],
    checked: &mut u64,
    best: &mut Option<Best>,
    found_counterexample: &mut bool,
) {
    if *found_counterexample {
        return;
    }
    if pos == support.len() {
        *checked = checked.saturating_add(1);
        maybe_update(best, matrix, support, free, q);
        if best.as_ref().is_some_and(|b| b.l1 < target_l1) {
            *found_counterexample = true;
        }
        return;
    }
    let j = support[pos];
    for x in -coeff_bound..=coeff_bound {
        if x == 0 {
            continue;
        }
        let next_l1 = free_l1 + x.unsigned_abs();
        if next_l1 >= target_l1 {
            continue;
        }
        free[j] = x;
        enumerate_coefficients(
            matrix,
            q,
            target_l1,
            coeff_bound,
            support,
            pos + 1,
            next_l1,
            free,
            checked,
            best,
            found_counterexample,
        );
        free[j] = 0;
        if *found_counterexample {
            return;
        }
    }
}

fn enumerate_supports(
    matrix: &[Vec<u64>],
    q: u64,
    target_l1: u64,
    coeff_bound: i64,
    free_cols: usize,
    support_size: usize,
    start: usize,
    support: &mut Vec<usize>,
    free: &mut [i64],
    checked: &mut u64,
    best: &mut Option<Best>,
    found_counterexample: &mut bool,
) {
    if *found_counterexample {
        return;
    }
    if support.len() == support_size {
        enumerate_coefficients(
            matrix,
            q,
            target_l1,
            coeff_bound,
            support,
            0,
            0,
            free,
            checked,
            best,
            found_counterexample,
        );
        return;
    }
    let remaining = support_size - support.len();
    if free_cols.saturating_sub(start) < remaining {
        return;
    }
    for j in start..=free_cols - remaining {
        support.push(j);
        enumerate_supports(
            matrix,
            q,
            target_l1,
            coeff_bound,
            free_cols,
            support_size,
            j + 1,
            support,
            free,
            checked,
            best,
            found_counterexample,
        );
        support.pop();
        if *found_counterexample {
            return;
        }
    }
}

fn certify_candidate(
    descriptor: MatrixDescriptor,
    target_l1: u64,
    max_support: usize,
    coeff_bound: i64,
    emit_matrix: bool,
    basis_path: Option<&Path>,
    basis_file: Option<String>,
) -> BoundedCertificate {
    let matrix = generate_matrix(&descriptor);
    let mut checked = 0u64;
    let mut best = None;
    let mut found_counterexample = false;
    let mut free = vec![0; descriptor.free_cols];
    let max_support = max_support.min(descriptor.free_cols);
    for support_size in 1..=max_support {
        let mut support = Vec::with_capacity(support_size);
        enumerate_supports(
            &matrix,
            descriptor.q,
            target_l1,
            coeff_bound,
            descriptor.free_cols,
            support_size,
            0,
            &mut support,
            &mut free,
            &mut checked,
            &mut best,
            &mut found_counterexample,
        );
        if found_counterexample {
            break;
        }
    }
    let best = best.expect("at least one bounded vector checked");
    let free_l1 = best.free.iter().map(|x| x.unsigned_abs()).sum();
    let relation_support = best.relation.iter().filter(|&&x| x != 0).count();
    if let Some(path) = basis_path {
        let basis = dual_qary_basis(&matrix, descriptor.q);
        fs::write(path, serde_json::to_string(&basis).unwrap() + "\n").expect("write basis");
    }

    BoundedCertificate {
        descriptor,
        target_l1,
        bounded_pass: !found_counterexample,
        found_counterexample,
        certified_lower_bound_in_domain: if found_counterexample {
            best.l1
        } else {
            target_l1
        },
        checked_vector_count: checked,
        complete_for_support_at_most: max_support,
        complete_for_coeff_abs_at_most: coeff_bound,
        complete_for_free_l1_below: target_l1,
        best_relation: BestRelation {
            centered_l1: best.l1,
            support: relation_support,
            free_l1,
            free_support: best.free.iter().filter(|&&x| x != 0).count(),
            free_vector_centered: best.free,
            relation_prefix_centered: best.relation.into_iter().take(64).collect(),
        },
        matrix_rows: emit_matrix.then_some(matrix),
        basis_file,
        notes: vec![
            "This is an exact exhaustive check only for the declared bounded free-vector domain.".to_string(),
            "A counterexample is a real systematic q-ary Lee-dual relation.".to_string(),
            "A pass is not a full SystematicLeeDistanceAtLeast proof unless the bounded domain covers all nonzero free vectors below the target.".to_string(),
        ],
    }
}

fn dual_qary_basis(matrix: &[Vec<u64>], q: u64) -> Vec<Vec<i64>> {
    let pivot_rows = matrix.len();
    let free_cols = matrix.first().map_or(0, Vec::len);
    let mut basis = Vec::with_capacity(pivot_rows + free_cols);
    for free_col in 0..free_cols {
        let mut row = Vec::with_capacity(pivot_rows + free_cols);
        for matrix_row in matrix.iter().take(pivot_rows) {
            row.push(centered_value((q - matrix_row[free_col] % q) % q, q));
        }
        row.extend((0..free_cols).map(|j| if j == free_col { 1 } else { 0 }));
        basis.push(row);
    }
    for i in 0..pivot_rows {
        let mut row = vec![0; pivot_rows + free_cols];
        row[i] = q as i64;
        basis.push(row);
    }
    basis
}

fn basis_stem(index: usize, descriptor: &MatrixDescriptor) -> String {
    format!(
        "{index:04}_{}_{}",
        descriptor.family.replace('-', "_"),
        descriptor.seed
    )
}

fn markdown(out: &CertificateOutput) -> String {
    let mut lines = vec![
        "# Systematic Lee Bounded Certificate".to_string(),
        String::new(),
        format!("- source: `{}`", out.source),
        format!("- q: `{}`", out.q),
        format!("- pivot rows: `{}`", out.pivot_rows),
        format!("- free columns: `{}`", out.free_cols),
        format!("- eta: `{}`", out.eta),
        format!("- target centered Lee l1: `{}`", out.target_l1),
        format!("- max support: `{}`", out.max_support),
        format!("- coefficient bound: `{}`", out.coeff_bound),
        String::new(),
        "| family | seed | pass | best l1 | checked | support scope | coeff scope | best free support | best relation support | basis |".to_string(),
        "|---|---:|---|---:|---:|---:|---:|---:|---:|---|".to_string(),
    ];
    for c in &out.certificates {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            c.descriptor.family,
            c.descriptor.seed,
            c.bounded_pass,
            c.best_relation.centered_l1,
            c.checked_vector_count,
            c.complete_for_support_at_most,
            c.complete_for_coeff_abs_at_most,
            c.best_relation.free_support,
            c.best_relation.support,
            c.basis_file.as_deref().unwrap_or(""),
        ));
    }
    lines.extend([
        String::new(),
        "The checked relation is `(-P y, y)` for the regenerated systematic candidate.".to_string(),
        "Pass means exact absence of a bad vector only inside the declared bounded domain.".to_string(),
        "Full Lean use still requires a global `SystematicLeeDistanceAtLeast` proof or a solver certificate whose domain is global.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        input_json: PathBuf::from("research/parameter_search/systematic_lee_search.json"),
        output_json: PathBuf::from("research/parameter_search/systematic_lee_cert.json"),
        output_md: PathBuf::from("research/parameter_search/systematic_lee_cert.md"),
        max_candidates: 4,
        max_support: 2,
        coeff_bound: 32,
        emit_matrix: false,
        basis_dir: None,
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" | "--input" => cfg.input_json = PathBuf::from(args.next().unwrap()),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            "--max-candidates" | "--top" => {
                cfg.max_candidates = args.next().unwrap().parse().unwrap()
            }
            "--max-support" => cfg.max_support = args.next().unwrap().parse().unwrap(),
            "--coeff-bound" => cfg.coeff_bound = args.next().unwrap().parse().unwrap(),
            "--emit-matrix" => cfg.emit_matrix = true,
            "--basis-dir" => cfg.basis_dir = Some(PathBuf::from(args.next().unwrap())),
            other => panic!("unknown argument: {other}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    assert!(cfg.max_candidates > 0, "max candidates must be positive");
    assert!(cfg.max_support > 0, "max support must be positive");
    assert!(cfg.coeff_bound > 0, "coefficient bound must be positive");
    let payload: SearchOutput =
        serde_json::from_str(&fs::read_to_string(&cfg.input_json).expect("read input json"))
            .expect("parse systematic Lee search json");
    if let Some(basis_dir) = &cfg.basis_dir {
        fs::create_dir_all(basis_dir).expect("create basis dir");
    }
    let certificates = payload
        .candidates
        .into_iter()
        .take(cfg.max_candidates)
        .enumerate()
        .map(|(index, candidate)| {
            let (basis_path, basis_file) = cfg
                .basis_dir
                .as_ref()
                .map(|basis_dir| {
                    let file_name = format!(
                        "{}.dual_qary_basis.txt",
                        basis_stem(index, &candidate.descriptor)
                    );
                    let path = basis_dir.join(&file_name);
                    (Some(path), Some(file_name))
                })
                .unwrap_or((None, None));
            certify_candidate(
                candidate.descriptor,
                payload.target_l1,
                cfg.max_support,
                cfg.coeff_bound,
                cfg.emit_matrix,
                basis_path.as_deref(),
                basis_file,
            )
        })
        .collect::<Vec<_>>();
    let out = CertificateOutput {
        source: cfg.input_json.display().to_string(),
        family: "systematic-lee-bounded-certificate".to_string(),
        q: payload.q,
        pivot_rows: payload.pivot_rows,
        free_cols: payload.free_cols,
        eta: payload.eta,
        target_l1: payload.target_l1,
        max_candidates: cfg.max_candidates,
        max_support: cfg.max_support,
        coeff_bound: cfg.coeff_bound,
        candidates_checked: certificates.len(),
        certificates,
        notes: vec![
            "This verifier consumes systematic_lee_search output and regenerates each P from its descriptor.".to_string(),
            "The bounded domain is deterministic and exhaustive: support <= max_support, |coefficient| <= coeff_bound, free l1 < target.".to_string(),
            "It is designed to be the exact bounded sidecar before adding global CP-SAT/Z3 or family theorem certificates.".to_string(),
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
