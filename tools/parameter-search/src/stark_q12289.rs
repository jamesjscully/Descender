//! AIR-agnostic DEEP-ALI STARK over the Fq12 FRI stack.
//!
//! This module is the GLUE that turns the FRI low-degree test
//! (`fri_q12289`) into a proof that a *committed execution trace* satisfies an
//! arbitrary [`Air`]'s boundary + transition constraints. It consumes any
//! `&dyn Air` (see `crate::air`): the single Poseidon2 permutation, the C2 PRF
//! chain, or any future relation component. It reuses, without reimplementing,
//! every verified building block:
//!
//!   * `crate::air::Air`        — the generic AIR interface: trace shape,
//!     max-degree, honest trace, boundary point constraints, and a per-row-pair
//!     transition residual evaluator over Fq12.
//!   * `ntt_q12289`            — base-field interpolation of each trace column on
//!     the trace subgroup -> column coefficient polynomials.
//!   * `fq12_ntt_q12289`       — `lde_coset` / `root_of_unity_fq12` to evaluate
//!     each column polynomial on the Fq12 FRI coset domain.
//!   * `merkle_poseidon2_q12289` — commit the LDE trace rows and the H codeword.
//!   * `fri_q12289`            — the low-degree test on the batched DEEP quotient.
//!   * `fq12_q12289` / `poseidon2_q12289` — field + Fiat-Shamir sponge.
//!
//! =============================================================================
//! PIPELINE (standard DEEP-ALI), parameterized by the AIR
//! =============================================================================
//! Let `n = air.n_rows()` padded to a power of two `TRACE_N = 2^log2_trace`,
//! `g = root_of_unity(log2_trace)` the trace-domain generator, `<g>` the size-n
//! trace subgroup. The execution trace is `air.n_cols()` columns; we interpolate
//! column `c` on `<g>` to a base-field polynomial `p_c(X)` of degree < n.
//!
//! The FRI domain `N = 2^log2_fri` is chosen so the FRI rate (1/4) certifies a
//! degree above every composition/DEEP quotient degree. With max constraint
//! degree `d`, the transition numerator C_T (per-row selector) has degree
//! <= d*(n-1) + (#transition_rows - 1), and C_T/Z_T has degree <= d*(n-1) - 1.
//! We pick the smallest `log2_fri` with `N/4 > d*(n-1)` and `log2_fri <= 14`
//! (Fq12 2-adicity) and `log2_fri - log2_trace >= 2` (rate <= 1/4).
//!
//! 1. TRACE & COMMIT. Interpolate the columns (base NTT) -> `p_c`. LDE each onto
//!    the coset (`lde_coset`) and onto the g-shifted coset, giving `p_c(x)` and
//!    `p_c(g x)` at the N domain points. Merkle-commit both.
//!
//! 2. COMPOSITION. From the roots derive FS coefficients. Build, on the coset,
//!        H(x) = alpha_T * C_T(x)/Z_T(x) + sum_b alpha_b * C_b(x)/(x - g^row_b)
//!    where C_T is the AIR's transition residuals through a per-row selector over
//!    `transition_rows`, and each boundary group b (one per distinct constrained
//!    row) batches that row's (col -> value) point constraints with FS lane
//!    combiners. H is a polynomial IFF every constraint holds. Commit H.
//!
//! 3. DEEP / OOD. FS out-of-domain point z. Send `p_c(z)`, `p_c(g z)`, `H(z)`.
//!    Verifier rechecks the composition identity at z, then forms DEEP quotients
//!    (one per column at z and gz, one for H), batched into one codeword.
//!
//! 4. FRI on the batched DEEP codeword (claim deg < N/4).
//!
//! 5. VERIFY: re-derive FS, check the OOD identity, run FRI, and at each FRI
//!    query Merkle-open the trace/H values and recompute the batched DEEP value,
//!    checking it equals the FRI layer-0 value.
//!
//! =============================================================================
//! SOUNDNESS — what is PROVEN vs ASSUMED
//! =============================================================================
//! The CODE mechanically enforces: (a) FS determinism of every challenge; (b)
//! Merkle binding of every opened trace/H value to its committed root; (c) the
//! algebraic OOD composition identity at z; (d) the DEEP-quotient value at every
//! FRI query equals the FRI layer-0 value; (e) the FRI low-degree verdict.
//!
//! It does NOT (and cannot locally) prove the FRI PROXIMITY-GAP theorem nor the
//! DEEP-ALI soundness argument (that a constraint-violating trace forces the
//! batched DEEP quotient to be NOT low-degree, with the OOD point catching an H
//! that disagrees with the true composition except with prob deg/|Fq12|). Those
//! are NAMED, standard STARK assumptions. CORRECTNESS of the arithmetic glue is
//! what this module + its tests establish.

use crate::air::{Air, BoundaryConstraint};
use crate::fq12_ntt_q12289::{lde_coset, root_of_unity_fq12};
use crate::fq12_q12289::{Fq12, N as FQ12_N};
use crate::fri_q12289::{self, coset_shift, fq12_to_leaf, FriProof};
use crate::merkle_poseidon2_q12289::{self as merkle, Digest, MerklePath, MerkleTree};
use crate::ntt_q12289::interpolate;
use crate::poseidon2_q12289::poseidon2_sponge;

// =============================================================================
// Domain selection
// =============================================================================

/// FRI rate exponent: FRI certifies degree < N / 2^RATE_LOG2 (= N/4).
const RATE_LOG2: u32 = fri_q12289::RATE_LOG2;
/// Max log2 FRI domain (Fq12 2-adicity).
const MAX_LOG2_FRI: u32 = 14;
/// Base-field NTT 2-adicity bound (trace interpolation domain).
const MAX_LOG2_TRACE: u32 = crate::ntt_q12289::MAX_LOG2N;

/// Number of FRI queries used (overridable per call via `prove_with_queries`).
pub const NUM_QUERIES: usize = fri_q12289::DEFAULT_NUM_QUERIES;

/// Domain-separation tag for the STARK Fiat-Shamir transcript.
const FS_DOMAIN: &[u8] = b"Descender.STARK.q12289.deep-ali.generic.v1";

/// Chosen domain sizing for an AIR.
#[derive(Clone, Copy, Debug)]
pub struct Domains {
    pub log2_trace: u32,
    pub trace_n: usize,
    pub log2_fri: u32,
    pub fri_n: usize,
}

/// Choose `log2_trace` (smallest power of two >= n_rows, within base-NTT bound)
/// and `log2_fri` (smallest with rate <= 1/4 and N/4 > d*(n-1), within Fq12
/// 2-adicity). Panics with a clear message if the AIR does not fit the field's
/// 2-adicity at the required rate.
pub fn choose_domains(air: &dyn Air) -> Domains {
    let n_rows = air.n_rows();
    let d = air.max_degree();
    let mut log2_trace = 0u32;
    while (1usize << log2_trace) < n_rows {
        log2_trace += 1;
    }
    assert!(
        log2_trace <= MAX_LOG2_TRACE,
        "trace rows {n_rows} (log2 {log2_trace}) exceed base-NTT 2-adicity {MAX_LOG2_TRACE}"
    );
    let trace_n = 1usize << log2_trace;
    // composition degree bound (DEEP/H quotient): d*(n-1) - 1 < d*(n-1).
    let comp_deg = d.saturating_mul(trace_n - 1);
    // need N/4 > comp_deg  AND  log2_fri >= log2_trace + RATE_LOG2.
    let mut log2_fri = log2_trace + RATE_LOG2;
    while (1usize << (log2_fri - RATE_LOG2)) <= comp_deg {
        log2_fri += 1;
    }
    assert!(
        log2_fri <= MAX_LOG2_FRI,
        "FRI domain log2 {log2_fri} exceeds Fq12 2-adicity {MAX_LOG2_FRI} \
         (trace_n={trace_n}, max_degree={d}, comp_deg={comp_deg}); degree-reduce the AIR"
    );
    Domains {
        log2_trace,
        trace_n,
        log2_fri,
        fri_n: 1usize << log2_fri,
    }
}

// =============================================================================
// Fiat-Shamir transcript (same sponge as FRI, distinct domain tag)
// =============================================================================

struct Transcript {
    lanes: Vec<u16>,
}

impl Transcript {
    fn new() -> Self {
        Transcript { lanes: Vec::new() }
    }
    fn absorb_root(&mut self, root: &Digest) {
        self.lanes.extend_from_slice(root);
    }
    fn absorb_fq12(&mut self, v: &Fq12) {
        for &c in v.c.iter() {
            self.lanes.push(c as u16);
        }
    }
    /// Squeeze an Fq12 challenge bound to the transcript and a `label`.
    fn challenge_fq12(&self, label: u32) -> Fq12 {
        let mut inp = self.lanes.clone();
        inp.push((label & 0xffff) as u16);
        inp.push(((label >> 16) & 0xffff) as u16);
        let out = poseidon2_sponge(FS_DOMAIN, &inp, FQ12_N);
        let mut c = [0u32; FQ12_N];
        for i in 0..FQ12_N {
            c[i] = out[i] as u32;
        }
        Fq12::from_coeffs(c)
    }
}

// =============================================================================
// Trace-domain points and vanishing polynomials
// =============================================================================

fn trace_gen(d: &Domains) -> Fq12 {
    root_of_unity_fq12(d.log2_trace)
}

/// g^i for i = 0..trace_n, in Fq12.
fn trace_points(d: &Domains) -> Vec<Fq12> {
    let g = trace_gen(d);
    let mut pts = Vec::with_capacity(d.trace_n);
    let mut p = Fq12::one();
    for _ in 0..d.trace_n {
        pts.push(p);
        p = p.mul(&g);
    }
    pts
}

/// Z_T(x) = prod_{i in transition_rows} (x - g^i), evaluated at `x`.
fn z_transition_at(x: &Fq12, pts: &[Fq12], transition_rows: &[usize]) -> Fq12 {
    let mut acc = Fq12::one();
    for &i in transition_rows {
        acc = acc.mul(&x.sub(&pts[i]));
    }
    acc
}

// =============================================================================
// Boundary grouping
// =============================================================================

/// Boundary constraints grouped by distinct row: each group is one quotient with
/// vanishing `(x - g^row)`, batching all (col -> value) at that row.
struct BoundaryGroup {
    row: usize,
    /// (col, value) pairs constrained at this row.
    entries: Vec<(usize, u32)>,
}

fn group_boundaries(bcs: &[BoundaryConstraint]) -> Vec<BoundaryGroup> {
    let mut rows: Vec<usize> = bcs.iter().map(|b| b.row).collect();
    rows.sort_unstable();
    rows.dedup();
    rows.into_iter()
        .map(|row| BoundaryGroup {
            row,
            entries: bcs
                .iter()
                .filter(|b| b.row == row)
                .map(|b| (b.col, b.value))
                .collect(),
        })
        .collect()
}

// =============================================================================
// FS coefficient derivation
// =============================================================================

/// One per-group FS challenge raised to powers gives the lane combiners; one
/// alpha per group composes the quotients. Deterministic from the transcript.
struct Coeffs {
    /// powers-of-challenge lane combiners for the transition group.
    trans_lane: Vec<Fq12>,
    /// one alpha for the transition group.
    alpha_trans: Fq12,
    /// per-boundary-group: (lane combiners, alpha).
    bnd: Vec<(Vec<Fq12>, Fq12)>,
}

fn powers(base: &Fq12, count: usize) -> Vec<Fq12> {
    let mut out = Vec::with_capacity(count);
    let mut p = Fq12::one();
    for _ in 0..count {
        out.push(p);
        p = p.mul(base);
    }
    out
}

fn derive_coeffs(t: &Transcript, n_trans: usize, groups: &[BoundaryGroup]) -> Coeffs {
    let trans_lane = powers(&t.challenge_fq12(0x1000), n_trans);
    let alpha_trans = t.challenge_fq12(0x2000);
    let mut bnd = Vec::with_capacity(groups.len());
    for (gi, g) in groups.iter().enumerate() {
        let lanes = powers(&t.challenge_fq12(0x3000 + gi as u32), g.entries.len());
        let alpha = t.challenge_fq12(0x4000 + gi as u32);
        bnd.push((lanes, alpha));
    }
    Coeffs {
        trans_lane,
        alpha_trans,
        bnd,
    }
}

// =============================================================================
// Constraint composition at a single point
// =============================================================================

/// Transition numerator C_T(x) via per-row Lagrange selectors over the real
/// transition rows. On real row g^i, C_T(g^i) = P_i(g^i) * resid_i(g^i) where
/// P_i(x) = prod_{j in rows, j!=i}(x - g^j) = Z_T(x)/(x - g^i). H's transition
/// term C_T/Z_T is a polynomial iff every real-row residual is 0. Valid for x
/// OFF the trace domain (FRI coset / OOD point), where all (x - g^i) are nonzero.
#[allow(clippy::too_many_arguments)]
fn transition_numerator_at(
    air: &dyn Air,
    x: &Fq12,
    pts: &[Fq12],
    transition_rows: &[usize],
    cur: &[Fq12],
    nxt: &[Fq12],
    lane: &[Fq12],
) -> Fq12 {
    let m = transition_rows.len();
    // Batched (Montgomery) inversion of all (x - g^i): one field inversion.
    let mut dd: Vec<Fq12> = Vec::with_capacity(m);
    for &i in transition_rows {
        dd.push(x.sub(&pts[i]));
    }
    let mut prefix: Vec<Fq12> = Vec::with_capacity(m + 1);
    prefix.push(Fq12::one());
    for di in &dd {
        prefix.push(prefix.last().unwrap().mul(di));
    }
    let z_t = prefix[m];
    let mut inv_total = z_t.inv().expect("x off trace domain: Z_T(x) nonzero");
    let mut inv_d = vec![Fq12::zero(); m];
    for i in (0..m).rev() {
        inv_d[i] = prefix[i].mul(&inv_total);
        inv_total = inv_total.mul(&dd[i]);
    }
    let mut acc = Fq12::zero();
    for (idx, &i) in transition_rows.iter().enumerate() {
        let p_i = z_t.mul(&inv_d[idx]); // = prod_{j != i}(x - g^j)
        let resids = air.transition_residuals(i, cur, nxt);
        let mut r = Fq12::zero();
        for (k, rk) in resids.iter().enumerate() {
            r = r.add(&lane[k].mul(rk));
        }
        acc = acc.add(&p_i.mul(&r));
    }
    acc
}

/// Boundary numerator for a group: sum_e lane[e] * (cur_col - value).
fn boundary_numerator(group: &BoundaryGroup, lane: &[Fq12], cur: &[Fq12]) -> Fq12 {
    let mut acc = Fq12::zero();
    for (e, &(col, value)) in group.entries.iter().enumerate() {
        let diff = cur[col].sub(&Fq12::from_base(value));
        acc = acc.add(&lane[e].mul(&diff));
    }
    acc
}

/// Compose H(x) value from the column values at x (and gx for transitions).
/// Returns None if a vanishing poly is zero at x (x on the trace domain).
fn compose_h_value(
    air: &dyn Air,
    x: &Fq12,
    pts: &[Fq12],
    transition_rows: &[usize],
    groups: &[BoundaryGroup],
    cur: &[Fq12],
    nxt: &[Fq12],
    coeffs: &Coeffs,
) -> Option<Fq12> {
    // Transition term.
    let c_trans = transition_numerator_at(air, x, pts, transition_rows, cur, nxt, &coeffs.trans_lane);
    let z_trans = z_transition_at(x, pts, transition_rows);
    let inv_zt = z_trans.inv()?;
    let mut h = coeffs.alpha_trans.mul(&c_trans.mul(&inv_zt));
    // Boundary terms.
    for (g, (lane, alpha)) in groups.iter().zip(coeffs.bnd.iter()) {
        let c_b = boundary_numerator(g, lane, cur);
        let z_b = x.sub(&pts[g.row]);
        let inv_zb = z_b.inv()?;
        h = h.add(&alpha.mul(&c_b.mul(&inv_zb)));
    }
    Some(h)
}

// =============================================================================
// Proof structure
// =============================================================================

/// One trace row opening at a FRI query position.
#[derive(Clone, Debug)]
pub struct QueryOpening {
    pub position: usize,
    pub cur: Vec<Fq12>,
    pub cur_path: MerklePath,
    pub nxt: Vec<Fq12>,
    pub nxt_path: MerklePath,
    pub h_val: Fq12,
    pub h_path: MerklePath,
}

/// A complete STARK proof for an AIR. The boundary constraints (rows, cols,
/// values) are part of the public statement; the verifier re-derives them from
/// the AIR it is given.
#[derive(Clone, Debug)]
pub struct StarkProof {
    pub trace_root: Digest,
    pub trace_shift_root: Digest,
    pub h_root: Digest,
    pub ood_cur: Vec<Fq12>,
    pub ood_nxt: Vec<Fq12>,
    pub ood_h: Fq12,
    pub fri: FriProof,
    pub openings: Vec<QueryOpening>,
}

// =============================================================================
// Prover
// =============================================================================

/// Prove that the AIR's trace (or a supplied possibly-tampered trace) satisfies
/// the AIR's constraints. Pass `tampered = None` for the honest trace.
pub fn prove(air: &dyn Air, tampered: Option<Vec<Vec<u32>>>) -> StarkProof {
    prove_with_queries(air, tampered, NUM_QUERIES)
}

/// As [`prove`], with an explicit FRI query count (lets slow large-domain proofs
/// trade soundness repetitions for speed in tests).
pub fn prove_with_queries(
    air: &dyn Air,
    tampered: Option<Vec<Vec<u32>>>,
    num_queries: usize,
) -> StarkProof {
    let d = choose_domains(air);
    let n_cols = air.n_cols();
    let n_rows = air.n_rows();
    let transition_rows = air.transition_rows();
    let bcs = air.boundary_constraints();
    let groups = group_boundaries(&bcs);
    let n_trans = air.n_transition_residuals();

    let trace = tampered.unwrap_or_else(|| air.generate_trace());
    assert_eq!(trace.len(), n_rows, "trace row count mismatch");

    let pts = trace_points(&d);
    let g = trace_gen(&d);
    let s = coset_shift();
    let s_shift = g.mul(&s);

    // --- 1. Interpolate each column on <g>; LDE onto coset and g-shifted coset.
    let mut col_coeffs: Vec<Vec<u32>> = Vec::with_capacity(n_cols);
    for c in 0..n_cols {
        let mut col_vals = vec![0u32; d.trace_n];
        for (i, slot) in col_vals.iter_mut().enumerate().take(n_rows) {
            *slot = trace[i][c];
        }
        col_coeffs.push(interpolate(&col_vals));
    }
    let col_lde: Vec<Vec<Fq12>> = col_coeffs
        .iter()
        .map(|cc| lde_coset(cc, d.log2_fri, s))
        .collect();
    let col_lde_shift: Vec<Vec<Fq12>> = col_coeffs
        .iter()
        .map(|cc| lde_coset(cc, d.log2_fri, s_shift))
        .collect();

    // Trace Merkle commitments: leaf k = the n_cols column values at point k.
    let trace_tree = commit_columns(&col_lde, d.fri_n, n_cols);
    let trace_shift_tree = commit_columns(&col_lde_shift, d.fri_n, n_cols);

    let mut transcript = Transcript::new();
    transcript.absorb_root(&trace_tree.root());
    transcript.absorb_root(&trace_shift_tree.root());
    let coeffs = derive_coeffs(&transcript, n_trans, &groups);

    // --- 2. Build H on the coset domain.
    let w = root_of_unity_fq12(d.log2_fri);
    let dom = coset_domain(&s, &w, d.fri_n);
    let mut h_codeword = Vec::with_capacity(d.fri_n);
    let mut cur = vec![Fq12::zero(); n_cols];
    let mut nxt = vec![Fq12::zero(); n_cols];
    for k in 0..d.fri_n {
        for c in 0..n_cols {
            cur[c] = col_lde[c][k];
            nxt[c] = col_lde_shift[c][k];
        }
        let h = compose_h_value(
            air, &dom[k], &pts, &transition_rows, &groups, &cur, &nxt, &coeffs,
        )
        .expect("coset point off <g>: all vanishing polys nonzero");
        h_codeword.push(h);
    }
    let h_leaves: Vec<Vec<u16>> = h_codeword.iter().map(fq12_to_leaf).collect();
    let h_tree = MerkleTree::commit(&h_leaves);
    transcript.absorb_root(&h_tree.root());

    // --- 3. OOD point z.
    let z = derive_ood_point(&transcript, &pts, &dom);
    let ood_cur: Vec<Fq12> = col_coeffs.iter().map(|cc| eval_base_poly_at(cc, &z)).collect();
    let gz = g.mul(&z);
    let ood_nxt: Vec<Fq12> = col_coeffs.iter().map(|cc| eval_base_poly_at(cc, &gz)).collect();
    let ood_h = compose_h_value(
        air, &z, &pts, &transition_rows, &groups, &ood_cur, &ood_nxt, &coeffs,
    )
    .expect("z off <g>");

    for v in &ood_cur {
        transcript.absorb_fq12(v);
    }
    for v in &ood_nxt {
        transcript.absorb_fq12(v);
    }
    transcript.absorb_fq12(&ood_h);

    // --- 4. Batched DEEP quotient codeword + FRI.
    let gammas = powers(&transcript.challenge_fq12(0x5000), 2 * n_cols + 1);
    let deep = build_deep_codeword(
        &dom, &col_lde, &col_lde_shift, &h_codeword, &ood_cur, &ood_nxt, &ood_h, &z, &gammas,
    );
    let fri = fri_q12289::prove(&deep, d.log2_fri, num_queries);

    // --- 5. Open committed trace + H at each FRI query position.
    let openings: Vec<QueryOpening> = fri
        .queries
        .iter()
        .map(|q| {
            let pos = q.position;
            QueryOpening {
                position: pos,
                cur: (0..n_cols).map(|c| col_lde[c][pos]).collect(),
                cur_path: trace_tree.open(pos),
                nxt: (0..n_cols).map(|c| col_lde_shift[c][pos]).collect(),
                nxt_path: trace_shift_tree.open(pos),
                h_val: h_codeword[pos],
                h_path: h_tree.open(pos),
            }
        })
        .collect();

    StarkProof {
        trace_root: trace_tree.root(),
        trace_shift_root: trace_shift_tree.root(),
        h_root: h_tree.root(),
        ood_cur,
        ood_nxt,
        ood_h,
        fri,
        openings,
    }
}

fn commit_columns(col_lde: &[Vec<Fq12>], fri_n: usize, n_cols: usize) -> MerkleTree {
    let leaves: Vec<Vec<u16>> = (0..fri_n)
        .map(|k| {
            let mut leaf = Vec::with_capacity(n_cols * FQ12_N);
            for c in 0..n_cols {
                leaf.extend_from_slice(&fq12_to_leaf(&col_lde[c][k]));
            }
            leaf
        })
        .collect();
    MerkleTree::commit(&leaves)
}

fn coset_domain(s: &Fq12, w: &Fq12, n: usize) -> Vec<Fq12> {
    let mut dom = Vec::with_capacity(n);
    let mut p = *s;
    for _ in 0..n {
        dom.push(p);
        p = p.mul(w);
    }
    dom
}

/// Evaluate a base-field-coefficient polynomial at an Fq12 point (Horner).
fn eval_base_poly_at(coeffs: &[u32], x: &Fq12) -> Fq12 {
    let mut acc = Fq12::zero();
    for &c in coeffs.iter().rev() {
        acc = acc.mul(x).add(&Fq12::from_base(c));
    }
    acc
}

/// Derive the OOD point z by FS, rejecting points on the coset or on <g>.
fn derive_ood_point(transcript: &Transcript, pts: &[Fq12], dom: &[Fq12]) -> Fq12 {
    let mut label = 0x6000u32;
    loop {
        let z = transcript.challenge_fq12(label);
        let on_trace = pts.iter().any(|p| *p == z);
        let on_coset = dom.iter().any(|p| *p == z);
        if !z.is_zero() && !on_trace && !on_coset {
            return z;
        }
        label = label.wrapping_add(1);
    }
}

/// Build the batched DEEP quotient codeword on the coset domain.
#[allow(clippy::too_many_arguments)]
fn build_deep_codeword(
    dom: &[Fq12],
    col_lde: &[Vec<Fq12>],
    col_lde_shift: &[Vec<Fq12>],
    h_codeword: &[Fq12],
    ood_cur: &[Fq12],
    ood_nxt: &[Fq12],
    ood_h: &Fq12,
    z: &Fq12,
    gammas: &[Fq12],
) -> Vec<Fq12> {
    let n = dom.len();
    let n_cols = col_lde.len();
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let inv_xz = dom[k].sub(z).inv().expect("x != z (z off coset)");
        let mut acc = Fq12::zero();
        let mut gi = 0usize;
        for c in 0..n_cols {
            let q = col_lde[c][k].sub(&ood_cur[c]).mul(&inv_xz);
            acc = acc.add(&gammas[gi].mul(&q));
            gi += 1;
        }
        for c in 0..n_cols {
            let q = col_lde_shift[c][k].sub(&ood_nxt[c]).mul(&inv_xz);
            acc = acc.add(&gammas[gi].mul(&q));
            gi += 1;
        }
        let qh = h_codeword[k].sub(ood_h).mul(&inv_xz);
        acc = acc.add(&gammas[gi].mul(&qh));
        out.push(acc);
    }
    out
}

// =============================================================================
// Verifier
// =============================================================================

/// Verify a STARK proof for `air`. Returns true iff every checkable obligation
/// holds. The AIR supplies the public boundary statement (rows/cols/values).
pub fn verify(air: &dyn Air, proof: &StarkProof) -> bool {
    let d = choose_domains(air);
    let n_cols = air.n_cols();
    let transition_rows = air.transition_rows();
    let bcs = air.boundary_constraints();
    let groups = group_boundaries(&bcs);
    let n_trans = air.n_transition_residuals();

    let pts = trace_points(&d);
    let g = trace_gen(&d);
    let s = coset_shift();
    let w = root_of_unity_fq12(d.log2_fri);
    let dom = coset_domain(&s, &w, d.fri_n);

    // --- Re-derive FS.
    let mut transcript = Transcript::new();
    transcript.absorb_root(&proof.trace_root);
    transcript.absorb_root(&proof.trace_shift_root);
    let coeffs = derive_coeffs(&transcript, n_trans, &groups);
    transcript.absorb_root(&proof.h_root);
    let z = derive_ood_point(&transcript, &pts, &dom);

    // --- (c) OOD composition identity at z.
    if proof.ood_cur.len() != n_cols || proof.ood_nxt.len() != n_cols {
        return false;
    }
    let expected_h = match compose_h_value(
        air, &z, &pts, &transition_rows, &groups, &proof.ood_cur, &proof.ood_nxt, &coeffs,
    ) {
        Some(v) => v,
        None => return false,
    };
    if expected_h != proof.ood_h {
        return false;
    }

    for v in &proof.ood_cur {
        transcript.absorb_fq12(v);
    }
    for v in &proof.ood_nxt {
        transcript.absorb_fq12(v);
    }
    transcript.absorb_fq12(&proof.ood_h);
    let gammas = powers(&transcript.challenge_fq12(0x5000), 2 * n_cols + 1);

    // --- (e) FRI verdict.
    if !fri_q12289::verify(&proof.fri) {
        return false;
    }
    if proof.fri.log2_domain != d.log2_fri {
        return false;
    }

    // --- (a,b,d) Per-FRI-query Merkle binding + DEEP recomputation.
    if proof.openings.len() != proof.fri.queries.len() {
        return false;
    }
    let _ = g;
    for (oi, op) in proof.openings.iter().enumerate() {
        let q = &proof.fri.queries[oi];
        if op.position != q.position {
            return false;
        }
        let pos = op.position;
        if op.cur.len() != n_cols || op.nxt.len() != n_cols {
            return false;
        }

        // (a) Merkle-bind opened rows + H value.
        let cur_leaf = pack_leaf(&op.cur);
        if !merkle::verify(&proof.trace_root, pos, &cur_leaf, &op.cur_path) {
            return false;
        }
        let nxt_leaf = pack_leaf(&op.nxt);
        if !merkle::verify(&proof.trace_shift_root, pos, &nxt_leaf, &op.nxt_path) {
            return false;
        }
        if !merkle::verify(&proof.h_root, pos, &fq12_to_leaf(&op.h_val), &op.h_path) {
            return false;
        }

        // (d) Recompute the batched DEEP value at this position.
        let x = dom[pos];
        let inv_xz = match x.sub(&z).inv() {
            Some(v) => v,
            None => return false,
        };
        let mut acc = Fq12::zero();
        let mut gi = 0usize;
        for c in 0..n_cols {
            let qd = op.cur[c].sub(&proof.ood_cur[c]).mul(&inv_xz);
            acc = acc.add(&gammas[gi].mul(&qd));
            gi += 1;
        }
        for c in 0..n_cols {
            let qd = op.nxt[c].sub(&proof.ood_nxt[c]).mul(&inv_xz);
            acc = acc.add(&gammas[gi].mul(&qd));
            gi += 1;
        }
        let qh = op.h_val.sub(&proof.ood_h).mul(&inv_xz);
        acc = acc.add(&gammas[gi].mul(&qh));

        if q.layers[0].value != acc {
            return false;
        }
    }

    true
}

fn pack_leaf(vals: &[Fq12]) -> Vec<u16> {
    let mut leaf = Vec::with_capacity(vals.len() * FQ12_N);
    for v in vals {
        leaf.extend_from_slice(&fq12_to_leaf(v));
    }
    leaf
}

// =============================================================================
// Tests — single Poseidon2 permutation AIR, through the generic interface.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_perm_air_adapter::Poseidon2PermAir;
    use crate::poseidon2_q12289::{fq_add, permute, Poseidon2Params, Q, T};

    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }
    fn random_state(seed: &mut u64) -> [u32; T] {
        let mut st = [0u32; T];
        for k in 0..T {
            st[k] = (xorshift(seed) % Q as u64) as u32;
        }
        st
    }

    /// COMPLETENESS: an honest single-permutation trace verifies through the
    /// generic AIR interface.
    #[test]
    fn stark_completeness_honest_trace_verifies() {
        let params = Poseidon2Params::new();
        let mut seed = 0xDEAD_BEEF_1234_5678u64;
        let input = random_state(&mut seed);
        let air = Poseidon2PermAir::new(&params, input);
        let proof = prove(&air, None);

        let mut reference = input;
        permute(&params, &mut reference);
        assert_eq!(air.output(), reference, "trace output must equal permute()");
        assert!(verify(&air, &proof), "honest STARK proof must verify");
    }

    /// FS DETERMINISM.
    #[test]
    fn stark_fiat_shamir_determinism() {
        let params = Poseidon2Params::new();
        let mut seed = 0x0BAD_F00D_CAFE_BABEu64;
        let input = random_state(&mut seed);
        let air = Poseidon2PermAir::new(&params, input);
        let p1 = prove(&air, None);
        let p2 = prove(&air, None);
        assert_eq!(p1.trace_root, p2.trace_root, "trace root deterministic");
        assert_eq!(p1.h_root, p2.h_root, "H root deterministic");
        assert_eq!(p1.ood_h, p2.ood_h, "OOD H deterministic");
        assert_eq!(p1.ood_cur, p2.ood_cur, "OOD cur deterministic");
        let pos1: Vec<usize> = p1.fri.queries.iter().map(|q| q.position).collect();
        let pos2: Vec<usize> = p2.fri.queries.iter().map(|q| q.position).collect();
        assert_eq!(pos1, pos2, "FRI positions deterministic");
    }

    /// SOUNDNESS: tamper one trace cell so a transition or boundary constraint is
    /// violated -> verify FAILS. Several positions.
    #[test]
    fn stark_soundness_tampered_trace_rejected() {
        use crate::poseidon2_air_q12289::N_ROWS;
        let params = Poseidon2Params::new();
        let mut seed = 0x1111_2222_3333_4444u64;
        let input = random_state(&mut seed);
        let air = Poseidon2PermAir::new(&params, input);
        let honest = air.generate_trace();

        let tampers: &[(usize, usize)] = &[
            (3, 7),
            (10, 0),
            (10, 13),
            (67, 5),
            (N_ROWS - 1, 0),
            (0, 4),
        ];
        for &(row, lane) in tampers {
            let mut t = honest.clone();
            t[row][lane] = fq_add(t[row][lane], 1);
            let proof = prove(&air, Some(t));
            assert!(
                !verify(&air, &proof),
                "tampered trace at row {row} lane {lane} must be REJECTED"
            );
        }
    }

    /// OOD-BINDING.
    #[test]
    fn stark_ood_binding_lie_rejected() {
        let params = Poseidon2Params::new();
        let mut seed = 0x9999_8888_7777_6666u64;
        let input = random_state(&mut seed);
        let air = Poseidon2PermAir::new(&params, input);
        let mut proof = prove(&air, None);
        assert!(verify(&air, &proof), "baseline honest proof must verify");

        proof.ood_cur[0] = proof.ood_cur[0].add(&Fq12::from_base(1));
        assert!(!verify(&air, &proof), "a lied-about OOD eval must be REJECTED");

        let mut proof2 = prove(&air, None);
        proof2.ood_h = proof2.ood_h.add(&Fq12::from_base(1));
        assert!(!verify(&air, &proof2), "a lied-about OOD H(z) must be REJECTED");
    }

    /// MERKLE-BINDING.
    #[test]
    fn stark_merkle_binding_tampered_opening_rejected() {
        let params = Poseidon2Params::new();
        let mut seed = 0xABCD_0001_0002_0003u64;
        let input = random_state(&mut seed);
        let air = Poseidon2PermAir::new(&params, input);
        let mut proof = prove(&air, None);
        assert!(verify(&air, &proof), "baseline honest proof must verify");
        let v = proof.openings[0].cur[0];
        proof.openings[0].cur[0] = v.add(&Fq12::from_base(1));
        assert!(!verify(&air, &proof), "tampered opened trace value must be REJECTED");
    }

    /// Domain sanity for the single-permutation AIR (regression on sizing).
    #[test]
    fn stark_param_sanity() {
        let params = Poseidon2Params::new();
        let air = Poseidon2PermAir::new(&params, [0u32; T]);
        let d = choose_domains(&air);
        assert_eq!(d.trace_n, 128, "70 rows -> 128");
        assert_eq!(d.fri_n, 4096, "degree-5 @ n=128 needs N/4 > 635 -> 4096");
        assert!(d.log2_fri - d.log2_trace >= RATE_LOG2, "rate <= 1/4");
    }
}
