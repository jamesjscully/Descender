use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::fs;
use std::path::PathBuf;

const MASK64: u64 = u64::MAX;

#[derive(Clone, Debug, Deserialize)]
struct Candidate {
    q: u64,
    n: u64,
    m0: u64,
    m1: u64,
    fiber_dim: u64,
    eta: u64,
    #[serde(default)]
    sample_bits: u64,
    #[serde(default)]
    view_coords: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct Reported {
    compact_public_key_bytes: u64,
    ciphertext_bytes: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct Exact {
    qary_amplitude_l1_required: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct ValidationRow {
    profile: String,
    source: String,
    verdict: String,
    candidate: Candidate,
    reported: Reported,
    exact: Exact,
}

#[derive(Clone, Debug, Deserialize)]
struct ValidationRoot {
    rows: Vec<ValidationRow>,
}

#[derive(Clone, Debug, Serialize)]
struct ProbeOutput {
    input: String,
    rows_checked: usize,
    coeff_bound: i64,
    random_samples: u64,
    max_random_sparsity: usize,
    anneal_restarts: u64,
    anneal_steps: u64,
    rows: Vec<ProbeRow>,
}

#[derive(Clone, Debug, Serialize)]
struct ProbeRow {
    index: usize,
    profile: String,
    source: String,
    verdict: String,
    q: u64,
    n: u64,
    m0: u64,
    m1: u64,
    view_coords: u64,
    eta: u64,
    public_key_bytes: u64,
    ciphertext_bytes: u64,
    required_centered_l1: u64,
    best_centered_l1: u64,
    best_method: String,
    best_free_sparsity: usize,
    best_support: usize,
    found_counterexample: bool,
    witness: Vec<i64>,
}

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

fn mix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

fn candidate_seed(c: &Candidate, profile: &str, domain: u64) -> u64 {
    let mut h = 0xD35CE5DE_5EED_0001 ^ domain;
    for byte in profile.as_bytes() {
        h = mix64(h ^ u64::from(*byte));
    }
    for value in [c.q, c.n, c.m0, c.m1, c.fiber_dim, c.eta, c.sample_bits] {
        h = mix64(h ^ value);
    }
    h
}

fn mod_pow(base: u64, mut exp: u64, q: u64) -> u64 {
    let mut acc = 1u128;
    let mut b = u128::from(base % q);
    let modulus = u128::from(q);
    while exp > 0 {
        if exp & 1 == 1 {
            acc = (acc * b) % modulus;
        }
        b = (b * b) % modulus;
        exp >>= 1;
    }
    acc as u64
}

fn mod_inv(x: u64, q: u64) -> u64 {
    mod_pow(x % q, q - 2, q)
}

fn cauchy_block_matrix(
    n: usize,
    cols: usize,
    q: u64,
    row_offset: u64,
    col_offset: u64,
) -> Vec<Vec<u64>> {
    let mut matrix = vec![vec![0; cols]; n];
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let x = row_offset + i as u64;
            let y = col_offset + n as u64 + j as u64;
            let denom = (x + q - (y % q)) % q;
            *cell = mod_inv(denom, q);
        }
    }
    matrix
}

fn generated_tilt_matrix(c: &Candidate, profile: &str) -> Vec<Vec<u64>> {
    let n = c.n as usize;
    let m1 = c.m1 as usize;
    let mut rng_i = Rng64::new(candidate_seed(c, profile, 0x1AC1D3CE));
    let mut rng_c = Rng64::new(candidate_seed(c, profile, 0xC0EFF1C1E));
    let mut rng_d = Rng64::new(candidate_seed(c, profile, 0xD1A60A1));
    let mut matrix = vec![vec![0; m1]; n];
    for col in 0..m1 {
        let a = rng_i.modulo(c.n) as usize;
        let mut b = rng_i.modulo(c.n) as usize;
        while b == a {
            b = rng_i.modulo(c.n) as usize;
        }
        let mut cc = rng_i.modulo(c.n) as usize;
        while cc == a || cc == b {
            cc = rng_i.modulo(c.n) as usize;
        }

        let diag = rng_d.nonzero_mod(c.q);
        let x = rng_c.nonzero_mod(c.q);
        let y = rng_c.nonzero_mod(c.q);
        let mut center = (c.q - ((x + y) % c.q)) % c.q;
        while center == 0 {
            let y_new = rng_c.nonzero_mod(c.q);
            center = (c.q - ((x + y_new) % c.q)) % c.q;
            if center != 0 {
                matrix[cc][col] = (diag * y_new) % c.q;
                break;
            }
        }
        if matrix[cc][col] == 0 {
            matrix[cc][col] = (diag * y) % c.q;
        }
        matrix[a][col] = (diag * center) % c.q;
        matrix[b][col] = (diag * x) % c.q;
    }
    matrix
}

fn public_adjoint_matrix(c: &Candidate, profile: &str) -> Vec<Vec<u64>> {
    let n = c.n as usize;
    let m0 = c.m0 as usize;
    let m1 = c.m1 as usize;
    let b0 = cauchy_block_matrix(n, m0, c.q, 0, 0);
    let mut b1 = cauchy_block_matrix(n, m1, c.q, c.m0, c.m0);
    let tilt = generated_tilt_matrix(c, profile);
    for i in 0..n {
        for j in 0..m1 {
            b1[i][j] = (b1[i][j] + tilt[i][j]) % c.q;
        }
    }
    b0.into_iter()
        .zip(b1)
        .map(|(mut left, right)| {
            left.extend(right);
            left
        })
        .collect()
}

fn invert_square_mod(matrix: &[Vec<u64>], q: u64) -> Option<Vec<Vec<u64>>> {
    let n = matrix.len();
    let mut aug = Vec::with_capacity(n);
    for (i, row) in matrix.iter().enumerate() {
        let mut out = row.clone();
        out.extend((0..n).map(|j| if i == j { 1 } else { 0 }));
        aug.push(out);
    }
    for col in 0..n {
        let pivot = (col..n).find(|&r| aug[r][col] % q != 0)?;
        aug.swap(col, pivot);
        let inv = mod_inv(aug[col][col], q);
        for value in &mut aug[col] {
            *value = (u128::from(*value) * u128::from(inv) % u128::from(q)) as u64;
        }
        for r in 0..n {
            if r == col {
                continue;
            }
            let factor = aug[r][col] % q;
            if factor == 0 {
                continue;
            }
            for j in 0..2 * n {
                aug[r][j] = (aug[r][j] + q
                    - (u128::from(factor) * u128::from(aug[col][j]) % u128::from(q)) as u64)
                    % q;
            }
        }
    }
    Some(aug.into_iter().map(|row| row[n..].to_vec()).collect())
}

fn matmul_mod(a: &[Vec<u64>], b: &[Vec<u64>], q: u64) -> Vec<Vec<u64>> {
    let rows = a.len();
    let inner = b.len();
    let cols = b.first().map_or(0, Vec::len);
    let mut out = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for (k, b_row) in b.iter().enumerate().take(inner) {
            let aik = a[i][k];
            if aik == 0 {
                continue;
            }
            for (j, b_cell) in b_row.iter().enumerate().take(cols) {
                out[i][j] = (u128::from(out[i][j]) + u128::from(aik) * u128::from(*b_cell))
                    .rem_euclid(u128::from(q)) as u64;
            }
        }
    }
    out
}

fn kernel_free_map(adjoint: &[Vec<u64>], q: u64) -> Option<Vec<Vec<u64>>> {
    let n = adjoint.len();
    let m = adjoint.first()?.len();
    let pivot: Vec<Vec<u64>> = (0..n)
        .map(|i| (0..n).map(|j| adjoint[i][j]).collect())
        .collect();
    let rest: Vec<Vec<u64>> = (0..n)
        .map(|i| (n..m).map(|j| adjoint[i][j]).collect())
        .collect();
    let pivot_inv = invert_square_mod(&pivot, q)?;
    let solved = matmul_mod(&pivot_inv, &rest, q);
    Some(
        solved
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|x| if x == 0 { 0 } else { q - x })
                    .collect()
            })
            .collect(),
    )
}

fn centered_abs(x: u64, q: u64) -> u64 {
    let r = x % q;
    r.min(q - r)
}

fn signed_to_mod(x: i64, q: u64) -> u64 {
    if x >= 0 {
        (x as u64) % q
    } else {
        let y = ((-x) as u64) % q;
        if y == 0 { 0 } else { q - y }
    }
}

fn relation_from_free(map: &[Vec<u64>], free: &[(usize, i64)], q: u64) -> Vec<u64> {
    let n = map.len();
    let d = map.first().map_or(0, Vec::len);
    let mut out = vec![0; n + d];
    for i in 0..n {
        let mut acc = 0u128;
        for &(j, coeff) in free {
            let y = signed_to_mod(coeff, q);
            acc += u128::from(map[i][j]) * u128::from(y);
        }
        out[i] = (acc % u128::from(q)) as u64;
    }
    for &(j, coeff) in free {
        out[n + j] = signed_to_mod(coeff, q);
    }
    out
}

fn relation_from_free_vec(map: &[Vec<u64>], free: &[i64], q: u64) -> Vec<u64> {
    let n = map.len();
    let d = map.first().map_or(0, Vec::len);
    let mut out = vec![0; n + d];
    for i in 0..n {
        let mut acc = 0u128;
        for (j, &coeff) in free.iter().enumerate() {
            if coeff == 0 {
                continue;
            }
            let y = signed_to_mod(coeff, q);
            acc += u128::from(map[i][j]) * u128::from(y);
        }
        out[i] = (acc % u128::from(q)) as u64;
    }
    for (j, &coeff) in free.iter().enumerate().take(d) {
        out[n + j] = signed_to_mod(coeff, q);
    }
    out
}

fn centered_l1(vec: &[u64], q: u64) -> u64 {
    vec.iter().map(|&x| centered_abs(x, q)).sum()
}

fn centered_support(vec: &[u64], q: u64) -> usize {
    vec.iter().filter(|&&x| x % q != 0).count()
}

fn centered_witness(vec: &[u64], q: u64) -> Vec<i64> {
    vec.iter()
        .map(|&x| {
            let r = x % q;
            if r > q / 2 {
                -((q - r) as i64)
            } else {
                r as i64
            }
        })
        .collect()
}

#[derive(Clone)]
struct Best {
    l1: u64,
    method: String,
    free_sparsity: usize,
    relation: Vec<u64>,
}

fn maybe_update(
    best: &mut Option<Best>,
    relation: Vec<u64>,
    q: u64,
    method: &str,
    free_sparsity: usize,
) {
    let l1 = centered_l1(&relation, q);
    if l1 == 0 {
        return;
    }
    if best.as_ref().is_none_or(|b| l1 < b.l1) {
        *best = Some(Best {
            l1,
            method: method.to_string(),
            free_sparsity,
            relation,
        });
    }
}

fn probe_row(
    row: &ValidationRow,
    index: usize,
    coeff_bound: i64,
    random_samples: u64,
    max_random_sparsity: usize,
    anneal_restarts: u64,
    anneal_steps: u64,
) -> ProbeRow {
    let c = &row.candidate;
    let adjoint = public_adjoint_matrix(c, &row.profile);
    let Some(map) = kernel_free_map(&adjoint, c.q) else {
        return ProbeRow {
            index,
            profile: row.profile.clone(),
            source: row.source.clone(),
            verdict: row.verdict.clone(),
            q: c.q,
            n: c.n,
            m0: c.m0,
            m1: c.m1,
            view_coords: c.view_coords,
            eta: c.eta,
            public_key_bytes: row.reported.compact_public_key_bytes,
            ciphertext_bytes: row.reported.ciphertext_bytes,
            required_centered_l1: row.exact.qary_amplitude_l1_required,
            best_centered_l1: 0,
            best_method: "pivot_block_singular".to_string(),
            best_free_sparsity: 0,
            best_support: 0,
            found_counterexample: false,
            witness: Vec::new(),
        };
    };

    let d = map.first().map_or(0, Vec::len);
    let mut best: Option<Best> = None;
    for j in 0..d {
        for coeff in -coeff_bound..=coeff_bound {
            if coeff == 0 {
                continue;
            }
            let relation = relation_from_free(&map, &[(j, coeff)], c.q);
            maybe_update(&mut best, relation, c.q, "free-unit-exact", 1);
        }
    }
    for j0 in 0..d {
        for j1 in (j0 + 1)..d {
            for c0 in -coeff_bound..=coeff_bound {
                if c0 == 0 {
                    continue;
                }
                for c1 in -coeff_bound..=coeff_bound {
                    if c1 == 0 {
                        continue;
                    }
                    let relation = relation_from_free(&map, &[(j0, c0), (j1, c1)], c.q);
                    maybe_update(&mut best, relation, c.q, "free-pair-exact", 2);
                }
            }
        }
    }

    let mut rng = Rng64::new(candidate_seed(c, &row.profile, 0xD1A1_11));
    for _ in 0..random_samples {
        let sparsity = 1 + rng.modulo(max_random_sparsity as u64) as usize;
        let mut free = Vec::with_capacity(sparsity);
        while free.len() < sparsity {
            let j = rng.modulo(d as u64) as usize;
            if free.iter().any(|&(k, _)| k == j) {
                continue;
            }
            let mag = 1 + rng.modulo(coeff_bound as u64) as i64;
            let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
            free.push((j, sign * mag));
        }
        let relation = relation_from_free(&map, &free, c.q);
        maybe_update(&mut best, relation, c.q, "free-random", sparsity);
    }

    for restart in 0..anneal_restarts {
        let mut free = vec![0i64; d];
        let init_sparsity = 1 + rng.modulo(max_random_sparsity as u64) as usize;
        let mut filled = 0usize;
        while filled < init_sparsity {
            let j = rng.modulo(d as u64) as usize;
            if free[j] != 0 {
                continue;
            }
            let mag = 1 + rng.modulo(coeff_bound as u64) as i64;
            let sign = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
            free[j] = sign * mag;
            filled += 1;
        }
        let mut relation = relation_from_free_vec(&map, &free, c.q);
        let mut current = centered_l1(&relation, c.q);
        maybe_update(
            &mut best,
            relation.clone(),
            c.q,
            "free-anneal",
            init_sparsity,
        );

        for step in 0..anneal_steps {
            let j = rng.modulo(d as u64) as usize;
            let old = free[j];
            let delta = match rng.modulo(6) {
                0 => -4,
                1 => -2,
                2 => -1,
                3 => 1,
                4 => 2,
                _ => 4,
            };
            let new = (old + delta).clamp(-coeff_bound, coeff_bound);
            if new == old {
                continue;
            }
            free[j] = new;
            let candidate_relation = relation_from_free_vec(&map, &free, c.q);
            let candidate_l1 = centered_l1(&candidate_relation, c.q);
            let temperature = (anneal_steps.saturating_sub(step) + 1).max(1);
            let uphill_budget = 1 + (current / 128).min(10_000);
            let accept_uphill =
                candidate_l1 <= current + uphill_budget && rng.modulo(temperature) == 0;
            if candidate_l1 <= current || accept_uphill {
                relation = candidate_relation;
                current = candidate_l1;
                let sparsity = free.iter().filter(|&&x| x != 0).count();
                maybe_update(&mut best, relation.clone(), c.q, "free-anneal", sparsity);
            } else {
                free[j] = old;
            }
        }
        if restart % 16 == 15 {
            let relation = relation_from_free_vec(&map, &free, c.q);
            let sparsity = free.iter().filter(|&&x| x != 0).count();
            maybe_update(&mut best, relation, c.q, "free-anneal", sparsity);
        }
    }

    let best = best.expect("nonempty kernel free map");
    let support = centered_support(&best.relation, c.q);
    let witness = centered_witness(&best.relation, c.q);
    ProbeRow {
        index,
        profile: row.profile.clone(),
        source: row.source.clone(),
        verdict: row.verdict.clone(),
        q: c.q,
        n: c.n,
        m0: c.m0,
        m1: c.m1,
        view_coords: c.view_coords,
        eta: c.eta,
        public_key_bytes: row.reported.compact_public_key_bytes,
        ciphertext_bytes: row.reported.ciphertext_bytes,
        required_centered_l1: row.exact.qary_amplitude_l1_required,
        best_centered_l1: best.l1,
        best_method: best.method,
        best_free_sparsity: best.free_sparsity,
        best_support: support,
        found_counterexample: best.l1 < row.exact.qary_amplitude_l1_required,
        witness,
    }
}

fn parse_args() -> (PathBuf, PathBuf, usize, i64, u64, usize, u64, u64) {
    let mut args = std::env::args().skip(1);
    let mut input = None;
    let mut output = None;
    let mut top = 16usize;
    let mut coeff_bound = 4i64;
    let mut random_samples = 20_000u64;
    let mut max_random_sparsity = 8usize;
    let mut anneal_restarts = 128u64;
    let mut anneal_steps = 20_000u64;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            "--top" => top = args.next().unwrap().parse().unwrap(),
            "--coeff-bound" => coeff_bound = args.next().unwrap().parse().unwrap(),
            "--random-samples" => random_samples = args.next().unwrap().parse().unwrap(),
            "--max-random-sparsity" => max_random_sparsity = args.next().unwrap().parse().unwrap(),
            "--anneal-restarts" => anneal_restarts = args.next().unwrap().parse().unwrap(),
            "--anneal-steps" => anneal_steps = args.next().unwrap().parse().unwrap(),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    (
        input.expect("--input is required"),
        output.expect("--output is required"),
        top,
        coeff_bound,
        random_samples,
        max_random_sparsity,
        anneal_restarts,
        anneal_steps,
    )
}

fn main() {
    let (
        input,
        output,
        top,
        coeff_bound,
        random_samples,
        max_random_sparsity,
        anneal_restarts,
        anneal_steps,
    ) = parse_args();
    let payload: ValidationRoot =
        serde_json::from_str(&fs::read_to_string(&input).expect("read input"))
            .expect("parse validation json");
    let mut indexed: Vec<_> = payload.rows.into_iter().enumerate().collect();
    indexed.sort_by_key(|(_, row)| {
        (
            row.profile.clone(),
            row.reported.compact_public_key_bytes,
            row.reported.ciphertext_bytes,
            Reverse(row.verdict == "pass"),
        )
    });
    indexed.truncate(top);
    let rows: Vec<_> = indexed
        .iter()
        .map(|(index, row)| {
            probe_row(
                row,
                *index,
                coeff_bound,
                random_samples,
                max_random_sparsity,
                anneal_restarts,
                anneal_steps,
            )
        })
        .collect();
    let out = ProbeOutput {
        input: input.display().to_string(),
        rows_checked: rows.len(),
        coeff_bound,
        random_samples,
        max_random_sparsity,
        anneal_restarts,
        anneal_steps,
        rows,
    };
    fs::write(&output, serde_json::to_string_pretty(&out).unwrap() + "\n").expect("write output");
    println!("wrote {}", output.display());
}
