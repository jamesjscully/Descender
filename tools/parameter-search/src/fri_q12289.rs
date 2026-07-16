//! FRI low-degree test over GF(q^12), q = 12289 — the core of the
//! account-creation STARK proof system.
//!
//! This module implements the standard 2-to-1 folding FRI proximity test with a
//! Fiat-Shamir (non-interactive) transcript. It REUSES the project's verified
//! building blocks and introduces NO new field, NTT, hash, or Merkle code:
//!
//!   * `fq12_q12289`        — the GF(q^12) field (add/sub/neg/mul/inv/...).
//!   * `fq12_ntt_q12289`    — `root_of_unity_fq12`, `lde_coset` (build codewords).
//!   * `merkle_poseidon2_q12289` — Poseidon2 Merkle vector commitment (20-lane).
//!   * `poseidon2_q12289`   — `poseidon2_sponge` for the FS transcript hash.
//!
//! --------------------------------------------------------------------------
//! PROTOCOL (what the code does)
//! --------------------------------------------------------------------------
//! Input: a codeword = Fq12 evaluations of a polynomial on a COSET
//!   `D_0 = shift * <w_0>`  of the 2^k-subgroup `<w_0>`, |D_0| = N = 2^k.
//! The claim is `deg(f) < N * RATE`, with RATE = 1/4 (so `deg(f) < N/4`). The
//! coset shift (`coset_shift()` = 7 + x) lies OUTSIDE every base/extension
//! 2-power subgroup, so the test domain is disjoint from any trace domain
//! (standard for FRI/DEEP).
//!
//! COMMIT PHASE (prover):
//!   * Layer 0 codeword = the input; Merkle-commit it (one leaf per evaluation,
//!     leaf = the 12 u16 coefficients of that Fq12 value — see `fq12_to_leaf`).
//!   * For layer i: absorb the layer roots committed so far into the transcript
//!     and squeeze a folding challenge `beta_i ∈ Fq12`. Fold pairs (x, -x):
//!         f_{i+1}(x^2) = (f_i(x) + f_i(-x))/2 + beta_i * (f_i(x) - f_i(-x))/(2x)
//!     The domain halves each layer (x -> x^2), and the coset shift squares too
//!     (s_{i+1} = s_i^2). Merkle-commit each new layer.
//!   * STOP when the layer domain size drops to <= STOP_DOMAIN (= 2^MIN_LOG2N);
//!     the FINAL layer is sent in clear (it is small).
//!
//! QUERY PHASE (prover): derive Q query positions via Fiat-Shamir from ALL layer
//! roots + the final layer. For each query, open the layer-0 value and its
//! sibling (the -x partner) with Merkle paths, and likewise at every subsequent
//! folded layer the value needed to check the fold relation.
//!
//! VERIFY: re-derive every FS challenge and query position from the transcript;
//! for each query check (a) the fold relation at each layer (recompute the next
//! layer value from the opened pair + beta_i and compare to the opened value),
//! (b) every Merkle path against its layer root, and (c) that the final small
//! layer is consistent with the last folded value AND genuinely below the final
//! degree bound (low-degree by interpolation check).
//!
//! --------------------------------------------------------------------------
//! PARAMETERS
//! --------------------------------------------------------------------------
//!   RATE        = 1/4   (claimed degree bound d < N/4)
//!   MIN_LOG2N   = 3     (stop folding when the domain size <= 2^3 = 8)
//!   NUM_QUERIES = 80    (default Q)
//!
//! SOUNDNESS / NAMED ASSUMPTION
//! ----------------------------
//! The FRI soundness bound rests on the FRI PROXIMITY-GAP THEOREM (Ben-Sasson,
//! Carmon, Ishai, Kopparty, Saraf 2020 and the BBHR18 FRI paper): for rate ρ a
//! codeword δ-far from the RS code is rejected per query with prob ~ δ, and the
//! commit-phase soundness error is governed by the proximity gap. With ρ = 1/4
//! the (conservative Johnson-bound) per-query rejection for a far word is large;
//! Q = 80 queries push the soundness error well below 2^-128 UNDER that theorem.
//! This is a NAMED ASSUMPTION. The CODE here only mechanically enforces three
//! checkable things: (1) fold-consistency at every queried layer, (2) Merkle
//! binding of every opened value to its committed root, and (3) Fiat-Shamir
//! determinism of all challenges/positions from the transcript. It does NOT
//! (and cannot, locally) prove the proximity-gap bound.

use crate::fq12_ntt_q12289::root_of_unity_fq12;
use crate::fq12_q12289::{Fq12, N as FQ12_N};
use crate::merkle_poseidon2_q12289::{self as merkle, Digest, MerklePath, MerkleTree};
use crate::poseidon2_q12289::poseidon2_sponge;

// =============================================================================
// Parameters
// =============================================================================

/// FRI rate denominator: RATE = 1/4 ⇒ claimed degree bound d < N/4.
pub const RATE_LOG2: u32 = 2; // 1/2^2 = 1/4

/// Stop folding once the layer domain size is <= 2^MIN_LOG2N (= 8). The final
/// layer is then transmitted in clear.
pub const MIN_LOG2N: u32 = 3;

/// Default number of FRI queries. See the module-level soundness note: the
/// 128-bit bound rests on the proximity-gap theorem (named assumption).
pub const DEFAULT_NUM_QUERIES: usize = 80;

/// Domain-separation tag for the FRI Fiat-Shamir transcript.
const FS_DOMAIN: &[u8] = b"Descender.FRI.q12289.fq12.transcript.v1";

// =============================================================================
// Helpers: Fq12 <-> Merkle leaf, the coset shift, field /2
// =============================================================================

/// Serialize one Fq12 evaluation as a 12-lane Merkle leaf (its 12 u16 coeffs).
/// Coefficients are canonical (< Q) by `Fq12` invariant, so they fit in u16.
pub fn fq12_to_leaf(v: &Fq12) -> Vec<u16> {
    v.c.iter().map(|&c| c as u16).collect()
}

/// The fixed coset shift `7 + x`, an element outside every 2-power subgroup of
/// Fq12* (so the FRI test domain is disjoint from any trace subgroup domain).
pub fn coset_shift() -> Fq12 {
    let mut c = [0u32; FQ12_N];
    c[0] = 7;
    c[1] = 1;
    Fq12::from_coeffs(c)
}

/// 1/2 in Fq12 (base-field inverse of 2 embedded into the extension).
fn half() -> Fq12 {
    Fq12::from_base(2).inv().expect("2 is invertible mod q")
}

// =============================================================================
// Fiat-Shamir transcript
// =============================================================================

/// A growing Fiat-Shamir transcript: a flat vector of u16 lanes that we absorb
/// commitments / data into and squeeze challenges from via `poseidon2_sponge`.
struct Transcript {
    lanes: Vec<u16>,
}

impl Transcript {
    fn new() -> Self {
        Transcript { lanes: Vec::new() }
    }

    /// Absorb a Merkle root (20 lanes) into the transcript.
    fn absorb_root(&mut self, root: &Digest) {
        self.lanes.extend_from_slice(root);
    }

    /// Absorb an Fq12 element (12 lanes) into the transcript.
    fn absorb_fq12(&mut self, v: &Fq12) {
        for &c in v.c.iter() {
            self.lanes.push(c as u16);
        }
    }

    /// Squeeze an Fq12 challenge bound to the current transcript and a `label`.
    /// We squeeze 12 lanes and assemble them as the 12 coefficients of an Fq12.
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

    /// Squeeze `count` query positions in [0, domain_size) bound to the current
    /// transcript. `domain_size` is a power of two. We squeeze enough lanes and
    /// mask each into the position range.
    fn challenge_positions(&self, label: u32, count: usize, domain_size: usize) -> Vec<usize> {
        debug_assert!(domain_size.is_power_of_two());
        let mask = domain_size - 1;
        let mut out_positions = Vec::with_capacity(count);
        // Squeeze in chunks of 20 lanes, re-seeding with an incrementing counter
        // so we never run out of pseudo-random material for large `count`.
        let mut counter: u32 = 0;
        while out_positions.len() < count {
            let mut inp = self.lanes.clone();
            inp.push((label & 0xffff) as u16);
            inp.push(((label >> 16) & 0xffff) as u16);
            inp.push((counter & 0xffff) as u16);
            inp.push(((counter >> 16) & 0xffff) as u16);
            let out = poseidon2_sponge(FS_DOMAIN, &inp, merkle::DIGEST_LANES);
            // Combine pairs of lanes into a larger word, then mask.
            for chunk in out.chunks(2) {
                if out_positions.len() >= count {
                    break;
                }
                let mut w: u32 = chunk[0] as u32;
                if chunk.len() > 1 {
                    w = (w << 14) ^ (chunk[1] as u32);
                }
                out_positions.push((w as usize) & mask);
            }
            counter = counter.wrapping_add(1);
        }
        out_positions
    }
}

// =============================================================================
// Proof structures
// =============================================================================

/// One opened (value, sibling-value, paths) tuple for a single query at one
/// layer. `value` is f_i(x) at the query position, `sibling` is f_i(-x) at the
/// partner position. Both Merkle paths are against the layer-i root.
#[derive(Clone, Debug)]
pub struct LayerOpening {
    pub value: Fq12,
    pub value_path: MerklePath,
    pub sibling: Fq12,
    pub sibling_path: MerklePath,
}

/// All openings for one query: one `LayerOpening` per folded layer (layer 0 up
/// to but not including the final clear layer).
#[derive(Clone, Debug)]
pub struct QueryProof {
    /// The layer-0 query position (into the layer-0 domain).
    pub position: usize,
    /// One opening per folding layer.
    pub layers: Vec<LayerOpening>,
}

/// A complete FRI proof.
#[derive(Clone, Debug)]
pub struct FriProof {
    /// log2 of the layer-0 domain size.
    pub log2_domain: u32,
    /// Merkle roots of every committed layer (layer 0 .. last folded layer).
    pub layer_roots: Vec<Digest>,
    /// The final (small) layer, sent in clear: its evaluations on the final
    /// coset domain.
    pub final_layer: Vec<Fq12>,
    /// Per-query openings.
    pub queries: Vec<QueryProof>,
    /// Number of queries (== queries.len()), recorded for the verifier.
    pub num_queries: usize,
}

// =============================================================================
// Folding
// =============================================================================

/// Fold one layer codeword `f_i` on the coset domain `s_i * <w_i>` (size n) into
/// the next layer `f_{i+1}` on `s_i^2 * <w_i^2>` (size n/2), using challenge
/// `beta`. Returns the folded codeword.
///
/// Layout convention: the layer-i domain points are `d[j] = s_i * w_i^j`,
/// j = 0..n. Because `w_i^(n/2) = -1`, the partner of point j (i.e. `-d[j]`) is
/// point `j + n/2`. The folded value at index j (j = 0..n/2), living on point
/// `(s_i w_i^j)^2 = s_i^2 w_i^(2j) = s_{i+1} * w_{i+1}^j`, is
///     even + beta * odd,
///   even = (f(x) + f(-x))/2,  odd = (f(x) - f(-x))/(2x),   x = d[j].
pub fn fold_layer(codeword: &[Fq12], log2n: u32, shift: Fq12, beta: &Fq12) -> Vec<Fq12> {
    let n = codeword.len();
    assert_eq!(n, 1usize << log2n, "codeword length must be 2^log2n");
    assert!(n >= 2, "cannot fold a domain of size < 2");
    let half_n = n / 2;
    let w = root_of_unity_fq12(log2n);
    let inv_half = half(); // 1/2

    let mut out = Vec::with_capacity(half_n);
    // x_j = shift * w^j ; partner is index j + half_n (point -x_j).
    let mut x = shift; // shift * w^0
    for j in 0..half_n {
        let fx = codeword[j];
        let fmx = codeword[j + half_n];
        let even = fx.add(&fmx).mul(&inv_half); // (f(x)+f(-x))/2
        let diff = fx.sub(&fmx); // f(x)-f(-x)
        let inv_x = x.inv().expect("coset point is nonzero");
        let odd = diff.mul(&inv_x).mul(&inv_half); // (f(x)-f(-x))/(2x)
        out.push(even.add(&beta.mul(&odd)));
        x = x.mul(&w);
    }
    out
}

/// The single folded value at one query position, computed from the opened pair.
/// `x` is the layer-i domain point at the query position; `fx`/`fmx` are
/// f_i(x), f_i(-x); `beta` the layer challenge. Returns f_{i+1}(x^2).
fn fold_point(fx: &Fq12, fmx: &Fq12, x: &Fq12, beta: &Fq12) -> Fq12 {
    let inv_half = half();
    let even = fx.add(fmx).mul(&inv_half);
    let diff = fx.sub(fmx);
    let inv_x = x.inv().expect("coset point is nonzero");
    let odd = diff.mul(&inv_x).mul(&inv_half);
    even.add(&beta.mul(&odd))
}

// =============================================================================
// Prover
// =============================================================================

/// Commit-phase intermediate state: the per-layer codewords, their Merkle
/// trees, shifts, and the FS challenges, plus the final clear layer.
struct CommitState {
    codewords: Vec<Vec<Fq12>>,
    trees: Vec<MerkleTree>,
    final_layer: Vec<Fq12>,
    log2_domain: u32,
}

/// Commit phase: fold from the input codeword down to the stop size, building
/// per-layer Merkle commitments and deriving folding challenges by Fiat-Shamir.
fn commit_phase(codeword: &[Fq12], log2_domain: u32, transcript: &mut Transcript) -> CommitState {
    let mut codewords: Vec<Vec<Fq12>> = Vec::new();
    let mut trees: Vec<MerkleTree> = Vec::new();

    let mut cur: Vec<Fq12> = codeword.to_vec();
    let mut cur_log2 = log2_domain;
    let mut cur_shift = coset_shift();

    // Commit layer 0.
    let leaves: Vec<Vec<u16>> = cur.iter().map(fq12_to_leaf).collect();
    let tree0 = MerkleTree::commit(&leaves);
    transcript.absorb_root(&tree0.root());
    codewords.push(cur.clone());
    trees.push(tree0);

    let mut layer_idx: u32 = 0;
    while cur_log2 > MIN_LOG2N {
        // Derive folding challenge from the transcript (roots so far).
        let beta = transcript.challenge_fq12(layer_idx);

        // Fold.
        let next = fold_layer(&cur, cur_log2, cur_shift, &beta);
        let next_shift = cur_shift.mul(&cur_shift);
        let next_log2 = cur_log2 - 1;

        // Commit the new layer and absorb its root.
        let leaves: Vec<Vec<u16>> = next.iter().map(fq12_to_leaf).collect();
        let tree = MerkleTree::commit(&leaves);
        transcript.absorb_root(&tree.root());

        codewords.push(next.clone());
        trees.push(tree);

        cur = next;
        cur_shift = next_shift;
        cur_log2 = next_log2;
        layer_idx += 1;
    }

    // The remaining `cur` IS the final layer (size <= 2^MIN_LOG2N). Absorb it.
    for v in &cur {
        transcript.absorb_fq12(v);
    }

    CommitState {
        codewords,
        trees,
        final_layer: cur,
        log2_domain,
    }
}

/// Produce a FRI proof for `codeword` (Fq12 evaluations on `coset_shift() *
/// <root_of_unity_fq12(log2_domain)>`), with `num_queries` queries.
///
/// `log2_domain` must satisfy MIN_LOG2N < log2_domain <= 14, and
/// `codeword.len() == 2^log2_domain`.
pub fn prove(codeword: &[Fq12], log2_domain: u32, num_queries: usize) -> FriProof {
    assert_eq!(
        codeword.len(),
        1usize << log2_domain,
        "codeword length must equal 2^log2_domain"
    );
    assert!(
        log2_domain > MIN_LOG2N && log2_domain <= 14,
        "log2_domain must be in ({}..=14]",
        MIN_LOG2N
    );

    let mut transcript = Transcript::new();
    let cs = commit_phase(codeword, log2_domain, &mut transcript);

    // Collect roots for the proof.
    let layer_roots: Vec<Digest> = cs.trees.iter().map(|t| t.root()).collect();

    // Query phase: derive positions from the full transcript (roots + final).
    let positions = transcript.challenge_positions(0xF1FF, num_queries, codeword.len());

    // We open value+sibling at every committed layer i for which we fold i->i+1,
    // i.e. i in 0..(cs.trees.len()-1).
    let mut queries: Vec<QueryProof> = Vec::with_capacity(num_queries);
    for &pos0 in &positions {
        let mut layers: Vec<LayerOpening> = Vec::new();
        let mut pos = pos0;
        let mut log2n = log2_domain;
        for li in 0..(cs.trees.len() - 1) {
            let n = 1usize << log2n;
            let half_n = n / 2;
            // Normalize: the "value" index is in [0, half_n), the sibling is
            // index + half_n. If pos >= half_n, swap so value/sibling are the
            // canonical (x, -x) pair but keep the actual queried point as value.
            let idx = pos % n;
            let (val_idx, sib_idx) = if idx < half_n {
                (idx, idx + half_n)
            } else {
                (idx, idx - half_n)
            };
            let cw = &cs.codewords[li];
            let tree = &cs.trees[li];
            layers.push(LayerOpening {
                value: cw[val_idx],
                value_path: tree.open(val_idx),
                sibling: cw[sib_idx],
                sibling_path: tree.open(sib_idx),
            });
            // Next layer position: fold maps both idx and idx±half_n to
            // (idx mod half_n) in the size-(n/2) domain.
            pos = idx % half_n;
            log2n -= 1;
        }
        queries.push(QueryProof {
            position: pos0,
            layers,
        });
    }

    FriProof {
        log2_domain: cs.log2_domain,
        layer_roots,
        final_layer: cs.final_layer,
        queries,
        num_queries,
    }
}

// =============================================================================
// Verifier
// =============================================================================

/// Verify a FRI proof. Returns `true` iff every checkable obligation holds:
/// FS-rederived challenges/positions match, every Merkle path verifies against
/// its layer root, every fold relation holds, the chain lands on the final
/// layer's value, and the final layer is genuinely low degree.
pub fn verify(proof: &FriProof) -> bool {
    let log2_domain = proof.log2_domain;
    if log2_domain <= MIN_LOG2N || log2_domain > 14 {
        return false;
    }
    let domain_size = 1usize << log2_domain;

    // Number of committed layers expected: we fold while log2 > MIN_LOG2N.
    let mut expected_layers = 0usize; // committed layers
    {
        let mut l = log2_domain;
        expected_layers += 1; // layer 0
        while l > MIN_LOG2N {
            expected_layers += 1;
            l -= 1;
        }
    }
    if proof.layer_roots.len() != expected_layers {
        return false;
    }
    let final_log2 = MIN_LOG2N; // domain size after folding stops
    if proof.final_layer.len() != (1usize << final_log2) {
        return false;
    }
    if proof.queries.len() != proof.num_queries {
        return false;
    }

    // --- Re-derive the transcript exactly as the prover built it. ---
    let mut transcript = Transcript::new();
    let mut betas: Vec<Fq12> = Vec::new();
    transcript.absorb_root(&proof.layer_roots[0]);
    let mut l = log2_domain;
    let mut layer_idx: u32 = 0;
    let mut ri = 1usize; // next root index to absorb
    while l > MIN_LOG2N {
        let beta = transcript.challenge_fq12(layer_idx);
        betas.push(beta);
        transcript.absorb_root(&proof.layer_roots[ri]);
        ri += 1;
        layer_idx += 1;
        l -= 1;
    }
    // Absorb the final layer.
    for v in &proof.final_layer {
        transcript.absorb_fq12(v);
    }

    // Re-derive query positions.
    let positions = transcript.challenge_positions(0xF1FF, proof.num_queries, domain_size);
    if positions.len() != proof.queries.len() {
        return false;
    }

    // Precompute per-layer (shift, root-of-unity) for point reconstruction.
    let num_fold = expected_layers - 1; // number of folding steps == openings/query
    let mut shifts: Vec<Fq12> = Vec::with_capacity(num_fold + 1);
    let mut log2s: Vec<u32> = Vec::with_capacity(num_fold + 1);
    {
        let mut s = coset_shift();
        let mut ll = log2_domain;
        for _ in 0..=num_fold {
            shifts.push(s);
            log2s.push(ll);
            s = s.mul(&s);
            if ll > 0 {
                ll -= 1;
            }
        }
    }

    // --- Per-query checks. ---
    for (qi, q) in proof.queries.iter().enumerate() {
        if q.position != positions[qi] {
            return false;
        }
        if q.layers.len() != num_fold {
            return false;
        }

        let mut pos = q.position;
        let mut folded_expected: Option<Fq12> = None;

        for li in 0..num_fold {
            let log2n = log2s[li];
            let n = 1usize << log2n;
            let half_n = n / 2;
            let shift = shifts[li];
            let w = root_of_unity_fq12(log2n);

            let opening = &q.layers[li];
            let idx = pos % n;
            let (val_idx, sib_idx) = if idx < half_n {
                (idx, idx + half_n)
            } else {
                (idx, idx - half_n)
            };

            // (1) Merkle binding: both opened values verify against layer root.
            let root = &proof.layer_roots[li];
            if !merkle::verify(
                root,
                val_idx,
                &fq12_to_leaf(&opening.value),
                &opening.value_path,
            ) {
                return false;
            }
            if !merkle::verify(
                root,
                sib_idx,
                &fq12_to_leaf(&opening.sibling),
                &opening.sibling_path,
            ) {
                return false;
            }

            // (2) Consistency with the value folded down from the previous layer.
            if let Some(prev) = folded_expected {
                if prev != opening.value {
                    return false;
                }
            }

            // The fold formula needs f(x) and f(-x). To land on the canonical
            // next-layer index (lo mod half_n) we treat x as the point at the
            // LOWER of the two indices (lo < half_n), and -x as its partner.
            let lo = val_idx.min(sib_idx);
            let x_lo = shift.mul(&w.pow(&[lo as u64]));
            let (fx, fmx) = if val_idx == lo {
                (opening.value, opening.sibling)
            } else {
                (opening.sibling, opening.value)
            };

            // (3) Fold relation: next-layer value at canonical index (lo mod half_n).
            let next_val = fold_point(&fx, &fmx, &x_lo, &betas[li]);
            folded_expected = Some(next_val);

            pos = idx % half_n;
        }

        // (4) The final folded value must match the final layer at `pos`.
        let final_val = match folded_expected {
            Some(v) => v,
            None => return false,
        };
        if pos >= proof.final_layer.len() {
            return false;
        }
        if proof.final_layer[pos] != final_val {
            return false;
        }
    }

    // (5) The final layer must genuinely be below its degree bound. The final
    // layer is the LDE of a polynomial of degree < |final_domain| * RATE on the
    // final coset. Interpolate and check the high coefficients are zero.
    if !final_layer_is_low_degree(&proof.final_layer, final_log2, &shifts[num_fold]) {
        return false;
    }

    true
}

/// Check that the final-layer codeword (evaluations on `final_shift *
/// <root_of_unity_fq12(final_log2)>`) is the evaluation of a polynomial of
/// degree < |domain| * RATE. We inverse-NTT the de-shifted values to recover
/// coefficients and require all coefficients at index >= degree bound to vanish.
fn final_layer_is_low_degree(final_layer: &[Fq12], final_log2: u32, final_shift: &Fq12) -> bool {
    let n = final_layer.len();
    if n != (1usize << final_log2) {
        return false;
    }
    // De-shift: values are P(shift * w^k); divide coeffs by shift^i after the
    // inverse transform. Easiest: inverse-NTT the values, then the result is the
    // coefficients of P(shift * X) i.e. coeff_i' = c_i * shift^i. We then check
    // degree by undoing the shift scaling (which does not change which coeffs are
    // zero, since shift is nonzero) — so we can check zero-ness directly on the
    // inverse-NTT output.
    let mut coeffs: Vec<Fq12> = final_layer.to_vec();
    crate::fq12_ntt_q12289::intt_fq12(&mut coeffs, final_log2);
    // Degree bound: d < n * RATE = n / 4.
    let deg_bound = n >> RATE_LOG2; // n/4
    // De-shift the coefficients so we test the true polynomial coefficients.
    // coeff_i' = c_i * shift^i  ⇒  c_i = coeff_i' * shift^{-i}. Zero-ness is
    // unaffected by the nonzero scalar, but we de-shift for clarity/robustness.
    let inv_shift = final_shift.inv().expect("coset shift is nonzero");
    let mut p = Fq12::one(); // inv_shift^i
    for (i, ci) in coeffs.iter().enumerate() {
        let c = ci.mul(&p);
        if i >= deg_bound && !c.is_zero() {
            return false;
        }
        p = p.mul(&inv_shift);
    }
    true
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fq12_ntt_q12289::lde_coset;
    use crate::poseidon2_q12289::Q;

    // ---- deterministic xorshift PRNG (no external deps) ----
    struct Rng(u64);
    impl Rng {
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn fq(&mut self) -> u32 {
            (self.next_u64() % Q as u64) as u32
        }
        fn fq12(&mut self) -> Fq12 {
            let mut c = [0u32; FQ12_N];
            for i in 0..FQ12_N {
                c[i] = self.fq();
            }
            Fq12::from_coeffs(c)
        }
    }

    /// Build a genuine low-degree codeword: LDE of a random base-field poly of
    /// degree < N/4 onto the FRI coset of size 2^log2_domain.
    fn low_degree_codeword(log2_domain: u32, seed: u64) -> Vec<Fq12> {
        let n = 1usize << log2_domain;
        let deg_bound = n >> RATE_LOG2; // n/4
        let mut rng = Rng(seed);
        // degree strictly < deg_bound
        let ncoeffs = deg_bound - 1;
        let base_coeffs: Vec<u32> = (0..ncoeffs).map(|_| rng.fq()).collect();
        lde_coset(&base_coeffs, log2_domain, coset_shift())
    }

    // -------------------------------------------------------------------------
    // COMPLETENESS
    // -------------------------------------------------------------------------

    #[test]
    fn fri_completeness_k8() {
        let cw = low_degree_codeword(8, 0xC0FFEE_01);
        let proof = prove(&cw, 8, DEFAULT_NUM_QUERIES);
        assert!(
            verify(&proof),
            "genuine low-degree k=8 codeword must verify"
        );
    }

    #[test]
    fn fri_completeness_k12() {
        // One larger case (k=12, 4096-point domain). Slower due to Fq12 mul.
        let cw = low_degree_codeword(12, 0xC0FFEE_02);
        let proof = prove(&cw, 12, DEFAULT_NUM_QUERIES);
        assert!(
            verify(&proof),
            "genuine low-degree k=12 codeword must verify"
        );
    }

    #[test]
    fn fri_completeness_fq12_poly() {
        // Low-degree codeword whose coefficients are genuine Fq12 (not just
        // base-field-embedded): build it directly by evaluating on the coset.
        let log2_domain = 8u32;
        let n = 1usize << log2_domain;
        let deg_bound = n >> RATE_LOG2;
        let mut rng = Rng(0xABCDEF_77);
        let coeffs: Vec<Fq12> = (0..(deg_bound - 1)).map(|_| rng.fq12()).collect();
        // Evaluate P on the coset: point_k = shift * w^k, Horner.
        let w = root_of_unity_fq12(log2_domain);
        let shift = coset_shift();
        let cw: Vec<Fq12> = (0..n)
            .map(|k| {
                let point = shift.mul(&w.pow(&[k as u64]));
                let mut acc = Fq12::zero();
                for c in coeffs.iter().rev() {
                    acc = acc.mul(&point).add(c);
                }
                acc
            })
            .collect();
        let proof = prove(&cw, log2_domain, DEFAULT_NUM_QUERIES);
        assert!(
            verify(&proof),
            "genuine low-degree Fq12-coeff codeword must verify"
        );
    }

    // -------------------------------------------------------------------------
    // SOUNDNESS (the key anchor)
    // -------------------------------------------------------------------------

    #[test]
    fn fri_soundness_random_codeword_rejected() {
        // (a) Uniformly random Fq12 codewords must be rejected. Run several
        // trials and report the empirical rejection rate (expected: 100%).
        let log2_domain = 8u32;
        let n = 1usize << log2_domain;
        let trials = 30;
        let mut rejected = 0;
        for t in 0..trials {
            let mut rng = Rng(0x5EED_0000 + t as u64);
            let cw: Vec<Fq12> = (0..n).map(|_| rng.fq12()).collect();
            let proof = prove(&cw, log2_domain, DEFAULT_NUM_QUERIES);
            if !verify(&proof) {
                rejected += 1;
            }
        }
        eprintln!(
            "[fri soundness] random codeword: rejected {}/{} trials (Q={})",
            rejected, trials, DEFAULT_NUM_QUERIES
        );
        assert_eq!(
            rejected, trials,
            "every random codeword must be rejected (Q={})",
            DEFAULT_NUM_QUERIES
        );
    }

    #[test]
    fn fri_soundness_tampered_codeword_rejected() {
        // (b) A genuine low-degree codeword with a few positions tampered must
        // be rejected with high probability. Report the empirical rate.
        let log2_domain = 8u32;
        let n = 1usize << log2_domain;
        let trials = 30;
        let mut rejected = 0;
        for t in 0..trials {
            let mut cw = low_degree_codeword(log2_domain, 0xBADF00D_00 + t as u64);
            let mut rng = Rng(0x7A11_0000 + t as u64);
            // Tamper a few random positions.
            for _ in 0..4 {
                let pos = (rng.next_u64() as usize) % n;
                cw[pos] = cw[pos].add(&Fq12::from_base(1 + (rng.fq())));
            }
            let proof = prove(&cw, log2_domain, DEFAULT_NUM_QUERIES);
            if !verify(&proof) {
                rejected += 1;
            }
        }
        eprintln!(
            "[fri soundness] tampered (4 positions) codeword: rejected {}/{} trials (Q={})",
            rejected, trials, DEFAULT_NUM_QUERIES
        );
        // With Q=80 queries out of 256 positions and 4 tampered cells, the
        // probability ALL queries miss every tampered fold is astronomically
        // small; we require unanimous rejection.
        assert_eq!(
            rejected, trials,
            "every tampered codeword must be rejected (Q={})",
            DEFAULT_NUM_QUERIES
        );
    }

    #[test]
    fn fri_soundness_high_degree_codeword_rejected() {
        // A codeword that is a valid LDE but of degree ABOVE the bound (>= N/4)
        // must be rejected by the final low-degree check / fold mismatch.
        let log2_domain = 8u32;
        let n = 1usize << log2_domain;
        // degree exactly the full domain (max degree), well above N/4.
        let mut rng = Rng(0x4116_4DE6_0000_DEAD);
        let coeffs: Vec<u32> = (0..n).map(|_| rng.fq()).collect();
        let cw = lde_coset(&coeffs, log2_domain, coset_shift());
        let proof = prove(&cw, log2_domain, DEFAULT_NUM_QUERIES);
        assert!(
            !verify(&proof),
            "an above-bound-degree codeword must be rejected"
        );
    }

    // -------------------------------------------------------------------------
    // FOLD CORRECTNESS (cross-check vs direct even/odd decomposition)
    // -------------------------------------------------------------------------

    #[test]
    fn fri_fold_matches_even_odd_decomposition() {
        // For a known polynomial P(X) = sum c_i X^i, folding the codeword with
        // beta must equal the codeword of P_even(Y) + beta * P_odd(Y), where
        // P(X) = P_even(X^2) + X * P_odd(X^2). Check the folded codeword equals
        // the evaluation of (P_even + beta P_odd) on the squared coset.
        let log2_domain = 6u32;
        let n = 1usize << log2_domain;
        let mut rng = Rng(0xF01D_1234);
        let deg = n / 2; // arbitrary; fold identity holds for any degree
        let coeffs: Vec<Fq12> = (0..deg).map(|_| rng.fq12()).collect();

        let shift = coset_shift();
        let w = root_of_unity_fq12(log2_domain);
        // Codeword of P on the coset.
        let cw: Vec<Fq12> = (0..n)
            .map(|k| {
                let point = shift.mul(&w.pow(&[k as u64]));
                let mut acc = Fq12::zero();
                for c in coeffs.iter().rev() {
                    acc = acc.mul(&point).add(c);
                }
                acc
            })
            .collect();

        let beta = rng.fq12();
        let folded = fold_layer(&cw, log2_domain, shift, &beta);

        // Direct even/odd: P_even has coeffs c[0], c[2], c[4], ...; P_odd has
        // c[1], c[3], .... Folded poly G(Y) = P_even(Y) + beta * P_odd(Y).
        let even_coeffs: Vec<Fq12> = coeffs.iter().step_by(2).cloned().collect();
        let odd_coeffs: Vec<Fq12> = coeffs.iter().skip(1).step_by(2).cloned().collect();
        let g_coeffs: Vec<Fq12> = {
            let m = even_coeffs.len().max(odd_coeffs.len());
            (0..m)
                .map(|i| {
                    let e = even_coeffs.get(i).cloned().unwrap_or_else(Fq12::zero);
                    let o = odd_coeffs.get(i).cloned().unwrap_or_else(Fq12::zero);
                    e.add(&beta.mul(&o))
                })
                .collect()
        };
        // Evaluate G on the squared coset: domain points (shift * w^j)^2 =
        // shift^2 * (w^2)^j for j = 0..n/2.
        let next_log2 = log2_domain - 1;
        let w2 = root_of_unity_fq12(next_log2);
        let shift2 = shift.mul(&shift);
        let expected: Vec<Fq12> = (0..(n / 2))
            .map(|j| {
                let point = shift2.mul(&w2.pow(&[j as u64]));
                let mut acc = Fq12::zero();
                for c in g_coeffs.iter().rev() {
                    acc = acc.mul(&point).add(c);
                }
                acc
            })
            .collect();

        assert_eq!(
            folded, expected,
            "fold must equal even + beta*odd decomposition"
        );
    }

    // -------------------------------------------------------------------------
    // FIAT-SHAMIR DETERMINISM
    // -------------------------------------------------------------------------

    #[test]
    fn fri_fiat_shamir_determinism() {
        let cw = low_degree_codeword(8, 0xDDDD_0001);
        let p1 = prove(&cw, 8, DEFAULT_NUM_QUERIES);
        let p2 = prove(&cw, 8, DEFAULT_NUM_QUERIES);
        // Same input -> identical roots, challenges (positions), final layer.
        assert_eq!(
            p1.layer_roots, p2.layer_roots,
            "roots must be deterministic"
        );
        let pos1: Vec<usize> = p1.queries.iter().map(|q| q.position).collect();
        let pos2: Vec<usize> = p2.queries.iter().map(|q| q.position).collect();
        assert_eq!(pos1, pos2, "query positions must be deterministic");
        assert_eq!(
            p1.final_layer, p2.final_layer,
            "final layer must be deterministic"
        );

        // A changed codeword -> changed roots and (almost surely) positions.
        let mut cw2 = cw.clone();
        cw2[3] = cw2[3].add(&Fq12::from_base(1));
        let p3 = prove(&cw2, 8, DEFAULT_NUM_QUERIES);
        assert_ne!(
            p1.layer_roots[0], p3.layer_roots[0],
            "changed codeword must change root 0"
        );
        let pos3: Vec<usize> = p3.queries.iter().map(|q| q.position).collect();
        assert_ne!(pos1, pos3, "changed codeword must change query positions");
    }

    // -------------------------------------------------------------------------
    // MERKLE BINDING
    // -------------------------------------------------------------------------

    #[test]
    fn fri_merkle_binding_tampered_opening_fails() {
        let cw = low_degree_codeword(8, 0xEEEE_0002);
        let mut proof = prove(&cw, 8, DEFAULT_NUM_QUERIES);
        assert!(verify(&proof), "baseline honest proof must verify");

        // Tamper an opened value WITHOUT fixing its Merkle path/root. The fold
        // check and/or Merkle check must fail.
        let v = proof.queries[0].layers[0].value;
        proof.queries[0].layers[0].value = v.add(&Fq12::from_base(1));
        assert!(
            !verify(&proof),
            "tampering an opened value (without fixing path/root) must fail verify"
        );
    }

    #[test]
    fn fri_merkle_binding_tampered_path_fails() {
        let cw = low_degree_codeword(8, 0xEEEE_0003);
        let mut proof = prove(&cw, 8, DEFAULT_NUM_QUERIES);
        assert!(verify(&proof), "baseline honest proof must verify");
        // Tamper a sibling digest in a path.
        proof.queries[0].layers[0].value_path.siblings[0][0] =
            ((proof.queries[0].layers[0].value_path.siblings[0][0] as u32 + 1) % Q) as u16;
        assert!(
            !verify(&proof),
            "tampering a Merkle path sibling must fail verify"
        );
    }
}
