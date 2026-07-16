//! Gadget 3 (chaining) — multi-round FRI commit-phase fold CHAINING as a transition AIR.
//!
//! [`crate::fri_fold_air`] constrains ONE binary fold per row in isolation (the cleared fold
//! identity (★) `folded·2s == e0(beta+s) − e1(beta−s)`). That is the per-round consistency
//! check, but a real FRI commit phase folds a single query DOWN through every commit-phase
//! round, and the rounds must be LINKED: the value the prover opens at the next round must be
//! the value the previous round folded to. This module adds that link as a multi-row
//! TRANSITION constraint, so one trace (rows = consecutive rounds of one query's descent)
//! constrains the whole folding chain, not just isolated folds.
//!
//! ## The chaining identity (the new soundness content)
//!
//! For a query of domain index `i`, the leaf-layer fold point is `s_0 = fold_point(h_0, i)` (a
//! root of unity of order `2^{h_0+1}`). FRI folds a coset pair `{s, −s}` into the SQUARED point
//! `s²` on the next layer's domain (the next layer's coset is `{s², −s²}`). So the per-layer
//! descent of the fold point is exactly
//!
//!   `s_{r+1} = s_r²`                                                          (point-square)
//!
//! (each layer halves the domain, i.e. `h_{r+1} = h_r − 1`; verified by
//! [`tests::squared_point_is_next_layer_root`]). NOTE: `s²` is NOT `fold_point(h−1, i>>1)` in
//! general — it is `± fold_point(h−1, i>>1)` (the sign is the dropped index bit). The squaring is
//! the canonical next-layer point, which is why the AIR pins `next.s == local.s²` directly rather
//! than re-deriving from a per-layer `(h, index)`. The fold of round `r` produces
//! `folded_r = g_r(s_r²) = g_r(s_{r+1})`, and round `r+1`'s opened pair is
//! `(e0_{r+1}, e1_{r+1}) = (g_r(s_{r+1}), g_r(−s_{r+1}))`. Therefore the FIRST opened eval of
//! the next round must equal the previous round's folded output:
//!
//!   `e0_{r+1} == folded_r`                                                     (chain-link)
//!
//! The SECOND eval `e1_{r+1}` (the sibling at `−s_{r+1}`) is a freshly Merkle-opened value of
//! the next layer — NOT derivable from round `r` — so it stays a committed column (bound by
//! gadget 2). The chaining constraint is exactly the two equalities (point-square) ∧
//! (chain-link), enforced row-to-row by `is_transition`. A broken chain (round r+1 opening a
//! value inconsistent with round r's fold) is rejected — soundness by attempted forgery.
//!
//! ## Per-layer log-height (replacing the const)
//!
//! `fri_fold_air`/`index_bus` carried a const `QUERY_BITS`/`log_height`. A real descent has a
//! DIFFERENT log-height per round (`h_0, h_0−1, …`). Here the per-row point `s` is the carried
//! column and the (point-square) transition pins the per-layer relation directly in-AIR, so the
//! AIR no longer depends on a single const height; the host derives each row's `s` from its own
//! `(h_r, index_r)` via [`crate::fri_fold::fold_point`] (the [`pack_round`] builder does this and
//! the chaining test threads a real decreasing `h_r`).

use crate::fri_fold::{fold_pair, fold_point};
use crate::fri_fold_air::{limbs, D, W};
use descender_account_stark::{Challenge, Val};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, Field, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

// ----- column layout (one commit-phase ROUND of one query's descent per row) -----
/// First sibling eval `e0 = g_{r-1}(s_r)` (8 limbs). For round 0 this is the leaf-layer
/// opening; for r>0 the (chain-link) transition pins it to the previous row's `folded`.
pub const C_E0: usize = 0;
/// Second sibling eval `e1 = g_{r-1}(−s_r)` (8 limbs) — a fresh next-layer Merkle opening.
pub const C_E1: usize = C_E0 + D;
/// Round fold challenge `beta_r` (8 limbs).
pub const C_BETA: usize = C_E1 + D;
/// The fold point `s_r` for this round (a base-field root of unity; 1 column).
pub const C_S: usize = C_BETA + D;
/// The claimed folded eval `folded_r = g_r(s_r²)` (8 limbs).
pub const C_FOLDED: usize = C_S + 1;
/// Zero-based commit-phase round ordinal.  This makes each public beta
/// addressable inside the AIR rather than leaving the fold challenges as
/// prover-selected witness cells.
pub const C_ROUND: usize = C_FOLDED + D;
/// Total AIR width.
pub const N_COLS: usize = C_ROUND + 1;
pub const PV_INITIAL_E0: usize = 0;
pub const PV_INITIAL_PARITY: usize = PV_INITIAL_E0 + D;
pub const PV_TERMINAL_FOLD: usize = PV_INITIAL_PARITY + 1;
pub const PV_BETAS: usize = PV_TERMINAL_FOLD + D;

pub const fn n_public(rounds: usize) -> usize {
    PV_BETAS + rounds * D
}

/// One descent ROUND of one query at an explicit fold point `s`: sibling evals `(e0, e1)`,
/// round challenge `beta`. `folded` is the reference [`fold_pair`] (the authority the per-row
/// identity is checked against).
pub fn pack_round(
    e0: Challenge,
    e1: Challenge,
    beta: Challenge,
    s: Val,
    round: usize,
) -> [Val; N_COLS] {
    let folded = fold_pair(e0, e1, beta, s);
    let mut row = [Val::ZERO; N_COLS];
    row[C_E0..C_E0 + D].copy_from_slice(&limbs(e0));
    row[C_E1..C_E1 + D].copy_from_slice(&limbs(e1));
    row[C_BETA..C_BETA + D].copy_from_slice(&limbs(beta));
    row[C_S] = s;
    row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded));
    row[C_ROUND] = Val::from_usize(round);
    row
}

/// The folded output of a packed round row (the value the NEXT round must open as its `e0`).
pub fn row_folded(row: &[Val; N_COLS]) -> Challenge {
    Challenge::from_basis_coefficients_slice(&row[C_FOLDED..C_FOLDED + D])
        .expect("folded limbs decode")
}

/// Build the full commit-phase descent of ONE query into the AIR trace, one row per round.
///
/// `(e0_0, e1_0)` is the leaf-layer opened pair; `siblings[r-1]` is round `r`'s freshly opened
/// SECOND eval `e1_r` (the `−s_r` partner). The FIRST eval of each subsequent round is the
/// previous round's fold output (the chain-link the AIR enforces), so it is NOT supplied — it
/// is recomputed here exactly as the honest prover would, and the AIR pins it. `h0`/`index` are
/// the leaf-layer log-height and query index; each round `r` uses `(h0−r, index>>r)`.
pub fn build_query_chain(
    e0_0: Challenge,
    e1_0: Challenge,
    betas: &[Challenge],
    siblings: &[Challenge],
    h0: usize,
    index: usize,
) -> Vec<Val> {
    assert_eq!(
        betas.len(),
        siblings.len() + 1,
        "one beta per round; one sibling per round>0"
    );
    let rounds = betas.len();
    let mut flat = Vec::with_capacity(rounds * N_COLS);

    // round 0: the leaf-layer point s_0 = fold_point(h0, index); subsequent points square.
    let mut s = fold_point(h0, index);
    let mut row = pack_round(e0_0, e1_0, betas[0], s, 0);
    flat.extend_from_slice(&row);
    let mut prev_folded = row_folded(&row);

    // rounds 1.. : s_r = s_{r-1}^2 (next-layer point); e0 = previous fold (chain-link);
    // e1 = freshly opened sibling at -s_r.
    for r in 1..rounds {
        s = s * s;
        row = pack_round(prev_folded, siblings[r - 1], betas[r], s, r);
        flat.extend_from_slice(&row);
        prev_folded = row_folded(&row);
    }
    flat
}

/// Build the same semantic `(f(s), f(-s))` chain from a native p3 query proof.
/// The first reduced opening is placed according to the dropped low query bit;
/// subsequent rows keep the previous fold at `s^2` as semantic `e0`, regardless
/// of the storage ordering used by the next MMCS opening.
pub fn build_query_chain_from_opening(
    reduced_opening: Challenge,
    round_siblings: &[Challenge],
    betas: &[Challenge],
    log_global_height: usize,
    index: usize,
) -> Vec<Val> {
    assert_eq!(round_siblings.len(), betas.len());
    assert!(!betas.is_empty());
    assert!(log_global_height >= betas.len());

    let parity = index & 1 == 1;
    let mut s = fold_point(log_global_height - 1, index >> 1);
    let (e0, e1) = if parity {
        (round_siblings[0], reduced_opening)
    } else {
        (reduced_opening, round_siblings[0])
    };
    let mut row = pack_round(e0, e1, betas[0], s, 0);
    let mut flat = Vec::with_capacity(betas.len() * N_COLS);
    flat.extend_from_slice(&row);
    let mut folded = row_folded(&row);
    for r in 1..betas.len() {
        s *= s;
        row = pack_round(folded, round_siblings[r], betas[r], s, r);
        flat.extend_from_slice(&row);
        folded = row_folded(&row);
    }
    flat
}

/// Octic multiply over `X^D = W` as in-AIR expressions (same formula as `fri_fold_air`):
/// `(a·b)[k] = Σ_{i+j=k} a[i]b[j] + W·Σ_{i+j=k+D} a[i]b[j]`.
fn octic_mul<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    let w = AB::Expr::from(AB::F::from_u32(W));
    core::array::from_fn(|k| {
        let mut acc = AB::Expr::ZERO;
        for i in 0..D {
            for j in 0..D {
                if i + j == k {
                    acc += a[i].clone() * b[j].clone();
                } else if i + j == k + D {
                    acc += w.clone() * a[i].clone() * b[j].clone();
                }
            }
        }
        acc
    })
}

/// Multi-round FRI fold CHAINING AIR. Per row: the cleared fold identity (★) (same as
/// `fri_fold_air`). Per consecutive-row transition: the chain — `next.s == local.s²`
/// (point-square) and `next.e0 == local.folded` (chain-link), so the next round's primary
/// opening IS the previous round's fold output. Degree ≤ 3 (the (★) octic products are deg-3;
/// the transition equalities are deg ≤ 2).
#[derive(Clone, Copy, Debug)]
pub struct FriChainAir {
    pub rounds: usize,
}

impl FriChainAir {
    pub fn new(rounds: usize) -> Self {
        assert!(rounds > 0, "FRI chain requires at least one round");
        Self { rounds }
    }
}

impl<F: Sync> BaseAir<F> for FriChainAir {
    fn width(&self) -> usize {
        N_COLS
    }

    fn num_public_values(&self) -> usize {
        n_public(self.rounds)
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for FriChainAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder
            .public_values()
            .iter()
            .map(|p| (*p).into())
            .collect();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let n = |i: usize| -> AB::Expr { next[i].into() };

        builder.when_first_row().assert_zero(l(C_ROUND));
        builder
            .when_transition()
            .assert_zero(n(C_ROUND) - l(C_ROUND) - AB::Expr::ONE);
        builder
            .when_last_row()
            .assert_zero(l(C_ROUND) - AB::Expr::from(Val::from_usize(self.rounds - 1)));

        // ---- per-row: cleared fold identity (★) over the deg-8 extension ----
        let e0: [AB::Expr; D] = core::array::from_fn(|i| l(C_E0 + i));
        let e1: [AB::Expr; D] = core::array::from_fn(|i| l(C_E1 + i));
        let beta: [AB::Expr; D] = core::array::from_fn(|i| l(C_BETA + i));
        let folded: [AB::Expr; D] = core::array::from_fn(|i| l(C_FOLDED + i));
        let s = l(C_S);

        let mut beta_plus_s = beta.clone();
        beta_plus_s[0] = beta[0].clone() + s.clone();
        let mut beta_minus_s = beta.clone();
        beta_minus_s[0] = beta[0].clone() - s.clone();

        let p0 = octic_mul::<AB>(&e0, &beta_plus_s);
        let p1 = octic_mul::<AB>(&e1, &beta_minus_s);

        let two_s = s.double();
        for k in 0..D {
            let lhs = folded[k].clone() * two_s.clone();
            let rhs = p0[k].clone() - p1[k].clone();
            builder.assert_zero(lhs - rhs);
        }

        // ---- transition: chain round r -> r+1 ----
        {
            let is_tr = builder.is_transition();
            let mut tb = builder.when(is_tr);
            // (point-square): next round's fold point is this round's point squared.
            tb.assert_zero(n(C_S) - l(C_S) * l(C_S));
            // (chain-link): next round's FIRST opened eval is THIS round's folded output,
            // limb-for-limb over the deg-8 extension.
            for k in 0..D {
                tb.assert_zero(n(C_E0 + k) - l(C_FOLDED + k));
            }
        }

        // Settlement composition boundary: the opening reducer supplies the
        // first FRI evaluation, and the final-polynomial check consumes the
        // terminal fold. Every round beta is public-bound as well: accepting a
        // structurally valid chain under prover-selected betas would not compose
        // with the Fiat--Shamir transcript.
        if pubs.len() == n_public(self.rounds) {
            {
                let mut first = builder.when_first_row();
                let parity = pubs[PV_INITIAL_PARITY].clone();
                first.assert_bool(parity.clone());
                for i in 0..D {
                    let selected = (AB::Expr::ONE - parity.clone()) * l(C_E0 + i)
                        + parity.clone() * l(C_E1 + i);
                    first.assert_eq(selected, pubs[PV_INITIAL_E0 + i].clone());
                }
            }
            let round = l(C_ROUND);
            for limb in 0..D {
                let mut selected = AB::Expr::ZERO;
                for r in 0..self.rounds {
                    let mut numerator = AB::Expr::ONE;
                    let mut denominator = Val::ONE;
                    for j in 0..self.rounds {
                        if j == r {
                            continue;
                        }
                        numerator *= round.clone() - AB::Expr::from(Val::from_usize(j));
                        denominator *= Val::from_usize(r) - Val::from_usize(j);
                    }
                    selected += numerator
                        * AB::Expr::from(denominator.inverse())
                        * pubs[PV_BETAS + r * D + limb].clone();
                }
                builder.assert_eq(l(C_BETA + limb), selected);
            }
            {
                let mut last = builder.when_last_row();
                for i in 0..D {
                    last.assert_eq(l(C_FOLDED + i), pubs[PV_TERMINAL_FOLD + i].clone());
                }
            }
        }
    }
}

pub fn public_values(
    initial_reduced_opening: Challenge,
    initial_parity: bool,
    terminal_fold: Challenge,
    betas: &[Challenge],
) -> Vec<Val> {
    assert!(!betas.is_empty());
    let mut out = Vec::with_capacity(n_public(betas.len()));
    out.extend_from_slice(&limbs(initial_reduced_opening));
    out.push(Val::from_bool(initial_parity));
    out.extend_from_slice(&limbs(terminal_fold));
    for beta in betas {
        out.extend_from_slice(&limbs(*beta));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_matrix::dense::RowMajorMatrix;

    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    /// PIN: the next-layer point is the SQUARE of this layer's point, and it lands on the
    /// next layer's domain — `s_0 = fold_point(h, i)` has multiplicative order dividing
    /// `2^{h+1}`, and `s_0²` has order dividing `2^h` (the next layer, halved domain). The
    /// chaining transition pins `next.s == local.s²`; this verifies the squaring really is the
    /// next-layer root, and that `± fold_point(h−1, i>>1)` equals `s²` (sign = dropped index
    /// bit) — by attempted equality, not assumption.
    #[test]
    fn squared_point_is_next_layer_root() {
        for h in [6usize, 8, 12, 20] {
            for &i in &[0usize, 1, 3, 6, 17, 30, 255, 1023] {
                let s = fold_point(h, i);
                let s2 = s * s;
                // s2 lands on the next layer's root set: it equals +/- fold_point(h-1, i>>1).
                let nxt = fold_point(h - 1, i >> 1);
                assert!(
                    s2 == nxt || s2 == -nxt,
                    "s^2 not on next layer at h={h}, i={i}"
                );
                // and s2 has order dividing 2^h (one halving): (s^2)^(2^h) == 1.
                assert_eq!(
                    s2.exp_power_of_2(h),
                    Val::ONE,
                    "s^2 order > 2^h at h={h}, i={i}"
                );
            }
        }
    }

    /// Build a real descent of ONE query through `betas.len()` commit-phase folds of a
    /// degree-`coeffs.len()` polynomial, returning the leaf pair + the freshly-opened siblings —
    /// the honest inputs to [`build_query_chain`]. The folded polynomial after each round is
    /// `g = f_even + beta·f_odd` (coeff k = c[2k] + beta·c[2k+1]), so the next round's opened
    /// pair is `(g(s²), g(−s²))`.
    fn honest_chain_inputs(
        coeffs: &[Challenge],
        betas: &[Challenge],
        h0: usize,
        index: usize,
    ) -> (Challenge, Challenge, Vec<Challenge>) {
        let eval = |c: &[Challenge], x: Val| -> Challenge {
            let mut acc = Challenge::ZERO;
            let mut p = Challenge::ONE;
            for &ci in c {
                acc += ci * p;
                p *= Challenge::from(x);
            }
            acc
        };
        let fold_poly = |c: &[Challenge], beta: Challenge| -> Vec<Challenge> {
            let half = c.len() / 2;
            (0..half).map(|k| c[2 * k] + beta * c[2 * k + 1]).collect()
        };

        let mut c = coeffs.to_vec();
        let s0 = fold_point(h0, index);
        let e0_0 = eval(&c, s0);
        let e1_0 = eval(&c, -s0);

        let mut siblings = Vec::new();
        let mut s_r = s0;
        for r in 1..betas.len() {
            c = fold_poly(&c, betas[r - 1]); // fold with PREVIOUS round's beta -> this round's poly
            s_r = s_r * s_r; // next-layer point = previous point squared (matches build_query_chain)
            siblings.push(eval(&c, -s_r)); // round r's freshly opened SECOND eval (the -s_r partner)
        }
        (e0_0, e1_0, siblings)
    }

    /// GOLDEN (positive): an honest multi-round descent of one query — leaf pair + per-round
    /// betas + freshly-opened siblings, with the chain-links recomputed by [`build_query_chain`]
    /// — ACCEPTS via the real `check_all_constraints`. The fold of each round really does feed
    /// the next round's primary opening (chain-link) at the squared point (point-square).
    #[test]
    fn air_accepts_honest_multiround_chain() {
        // degree-16-coeff poly -> 4 commit-phase rounds.
        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 1)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(100 + r)).collect();
        let h0 = 12usize;
        let index = 0b1011usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let m = RowMajorMatrix::new(flat, N_COLS);
        let report = check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None);
        assert!(
            report.is_ok(),
            "honest multi-round chain rejected: {} failures",
            report.failures.len()
        );
    }

    /// The chain really threads a DECREASING per-layer log-height (h0, h0-1, ...) and the derived
    /// points square round-to-round — the const-height assumption is gone.
    #[test]
    fn chain_threads_decreasing_log_height() {
        let coeffs: Vec<Challenge> = (0..8u32).map(|k| ext_multi(k + 5)).collect();
        let betas: Vec<Challenge> = (0..3u32).map(|r| ext_multi(200 + r)).collect();
        let h0 = 9usize;
        let index = 0b101usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let m = RowMajorMatrix::new(flat, N_COLS);
        for r in 0..betas.len() - 1 {
            let s_r = m.values[r * N_COLS + C_S];
            let s_next = m.values[(r + 1) * N_COLS + C_S];
            assert_eq!(
                s_r * s_r,
                s_next,
                "row {r}: point did not square to next row"
            );
        }
        assert!(check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None).is_ok());
    }

    /// Forgery — BROKEN CHAIN LINK: round r+1 opens a primary eval `e0` inconsistent with round
    /// r's fold output. The (chain-link) transition catches it. THIS is the chaining gadget's
    /// soundness property: a prover cannot substitute an unrelated value between rounds.
    #[test]
    fn air_rejects_broken_chain_link() {
        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 1)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(100 + r)).collect();
        let h0 = 12usize;
        let index = 0b1011usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        // Break the link into round 1: change row 1's primary eval e0 (no longer == row 0 folded).
        m.values[N_COLS + C_E0 + 2] += Val::ONE;
        assert!(
            !check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None).is_ok(),
            "broken chain link accepted — chaining not load-bearing"
        );
    }

    /// Forgery — a tampered fold output of round r breaks the NEXT round's chain-link (the (★)
    /// identity also fails at row r). Either way rejected: the prover cannot fudge an intermediate
    /// folded value to make the chain close.
    #[test]
    fn air_rejects_tampered_intermediate_fold() {
        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 3)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(50 + r)).collect();
        let h0 = 14usize;
        let index = 0b0110usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        // Tamper row 0's folded output: breaks (★) at row 0 AND the chain-link into row 1.
        m.values[C_FOLDED + 1] += Val::ONE;
        assert!(!check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None).is_ok());
    }

    /// Forgery — POINT-SQUARE violated: present a next-round point that is NOT the square of the
    /// previous round's point. Rejected by the (point-square) transition (and (★) at that row),
    /// so the prover cannot fold the next round at an off-descent point.
    #[test]
    fn air_rejects_wrong_next_point() {
        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 2)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(70 + r)).collect();
        let h0 = 12usize;
        let index = 0b1101usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        m.values[N_COLS + C_S] += Val::ONE; // row 1 point no longer == row 0 point squared
        assert!(!check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None).is_ok());
    }

    /// Forgery — a tampered round challenge `beta` in a middle round ⇒ (★) at that row fails ⇒
    /// rejected. (The in-AIR per-round beta is bound to the transcript by gadget 1.)
    #[test]
    fn air_rejects_tampered_round_beta() {
        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 7)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(90 + r)).collect();
        let h0 = 10usize;
        let index = 0b0011usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        m.values[2 * N_COLS + C_BETA + 3] += Val::ONE; // tamper round-2 beta
        assert!(!check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None).is_ok());
    }

    /// Forgery — a tampered next-round SIBLING `e1` (the fresh `−s` opening) ⇒ (★) at that row
    /// fails ⇒ rejected. (Bound to gadget-2 Merkle openings in the full wiring.)
    #[test]
    fn air_rejects_tampered_sibling() {
        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 11)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(30 + r)).collect();
        let h0 = 11usize;
        let index = 0b1110usize;
        let (e0_0, e1_0, sibs) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(e0_0, e1_0, &betas, &sibs, h0, index);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        m.values[N_COLS + C_E1] += Val::ONE; // tamper round-1 sibling (constant limb)
        assert!(!check_all_constraints(&FriChainAir::new(betas.len()), &m, &[], None).is_ok());
    }

    #[test]
    fn public_reduced_opening_feeds_q128_fri_chain() {
        use p3_uni_stark::{prove, verify};

        let coeffs: Vec<Challenge> = (0..16u32).map(|k| ext_multi(k + 19)).collect();
        let betas: Vec<Challenge> = (0..4u32).map(|r| ext_multi(220 + r)).collect();
        let h0 = 12usize;
        let index = 0b1011usize;
        let (initial, first_sibling, siblings) = honest_chain_inputs(&coeffs, &betas, h0, index);
        let flat = build_query_chain(initial, first_sibling, &betas, &siblings, h0, index);
        let terminal = Challenge::from_basis_coefficients_slice(
            &flat[(betas.len() - 1) * N_COLS + C_FOLDED..(betas.len() - 1) * N_COLS + C_FOLDED + D],
        )
        .expect("terminal fold limbs");
        let trace = p3_matrix::dense::RowMajorMatrix::new(flat, N_COLS);
        let pubs = public_values(initial, false, terminal, &betas);
        let air = FriChainAir::new(betas.len());
        assert!(check_all_constraints(&air, &trace, &pubs, None).is_ok());

        let mut wrong_initial = pubs.clone();
        wrong_initial[PV_INITIAL_E0] += Val::ONE;
        assert!(!check_all_constraints(&air, &trace, &wrong_initial, None).is_ok());

        let mut wrong_beta = pubs.clone();
        wrong_beta[PV_BETAS + D] += Val::ONE;
        assert!(!check_all_constraints(&air, &trace, &wrong_beta, None).is_ok());

        let config = crate::settlement::prod_config();
        let proof = prove(&config, &air, trace, &pubs);
        verify(&config, &air, &proof, &pubs)
            .expect("public reduced opening must feed the q128 FRI chain proof");
    }
}
