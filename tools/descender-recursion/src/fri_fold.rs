//! Gadget 3 — in-AIR FRI commit-phase fold (reference layer).
//!
//! Mirrors `p3-fri`'s binary fold (`two_adic_pcs.rs::fold_row`, log_arity = 1)
//! constant-for-constant: at a query of domain index `i` and layer log-height `h`, the
//! evaluation point is `s = two_adic_generator(h+1)^{reverse_bits_len(i, h)}`, the sibling
//! pair `(e0, e1)` at `(s, -s)` is folded by the round challenge `beta` via 2-point Lagrange
//! interpolation at `beta`:
//!
//!   `fold = [e0·(beta + s) − e1·(beta − s)] / (2·s)`
//!         `= (e0+e1)/2  +  beta·(e0−e1)/(2·s)`   (the classic even/odd FRI fold)
//!
//! which is exactly `f_even(s²) + beta·f_odd(s²)` when `(e0,e1) = (f(s), f(-s))` — i.e. the
//! evaluation of the folded polynomial `g = f_even + beta·f_odd` at the next-layer point
//! `s²`. This algebraic identity is the FRI soundness content and is what
//! [`tests::fold_is_the_folded_polynomial_evaluation`] proves directly (self-contained, no
//! native FRI instance needed). The deg-8 `Challenge` is the post-bump extension field
//! (commit 4c06f6b); over it the fold's degree-4-vs-8 margin no longer binds.
//!
//! DEFERRED (golden follow-on): asserting `fold_point`'s index→point convention against a
//! REAL `p3-fri` proof (the formula is validated here; the convention is transcribed from
//! `two_adic_pcs.rs:123-124` and structurally checked). gadget-3's AIR will constrain this
//! fold per query per round, consuming `beta` from gadget-1 and the sibling evals from
//! gadget-2's openings.

use descender_account_stark::{Challenge, Val};
use p3_field::{Field, PrimeCharacteristicRing, TwoAdicField};

/// Reverse the low `bits` bits of `index` (mirrors `p3_util::reverse_bits_len`).
pub fn reverse_bits_len(index: usize, bits: usize) -> usize {
    let mut out = 0usize;
    for b in 0..bits {
        if (index >> b) & 1 == 1 {
            out |= 1 << (bits - 1 - b);
        }
    }
    out
}

/// The binary-fold evaluation point for query `index` at layer log-height `log_height`
/// (`two_adic_pcs.rs:123-124`, `log_arity = 1`).
pub fn fold_point(log_height: usize, index: usize) -> Val {
    Val::two_adic_generator(log_height + 1).exp_u64(reverse_bits_len(index, log_height) as u64)
}

/// The binary FRI fold of sibling evals `(e0, e1)` at point `s` with round challenge `beta`:
/// 2-point Lagrange interpolation through `(s, e0), (-s, e1)` evaluated at `beta`.
pub fn fold_pair(e0: Challenge, e1: Challenge, beta: Challenge, s: Val) -> Challenge {
    let two_s = s.double();
    let inv = two_s.inverse(); // Val: 2s != 0 since s != 0 (a root of unity)
    // [e0·(beta + s) − e1·(beta − s)] · (2s)^{-1}
    (e0 * (beta + s) - e1 * (beta - s)) * inv
}

/// Full per-query fold: derive the point from `(log_height, index)`, then fold.
pub fn fold_query(
    e0: Challenge,
    e1: Challenge,
    beta: Challenge,
    log_height: usize,
    index: usize,
) -> Challenge {
    fold_pair(e0, e1, beta, fold_point(log_height, index))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }

    /// Evaluate `f(x) = c0 + c1 x + c2 x² + c3 x³` at a base point `x` (embedded into EF).
    fn eval_cubic(c: &[Challenge; 4], x: Val) -> Challenge {
        let x2 = x * x;
        let x3 = x2 * x;
        c[0] + c[1] * x + c[2] * x2 + c[3] * x3
    }

    /// SOUNDNESS CORE (spec): the FRI fold of `(f(s), f(-s))` equals the folded polynomial
    /// `g(s²)` where `g = f_even + beta·f_odd`. Proves `fold_pair` is the correct FRI fold.
    /// Uses the NATIVE-convention point `s = fold_point(...)`, tying formula + point together.
    #[test]
    fn fold_is_the_folded_polynomial_evaluation() {
        let c = [ext(7), ext(11), ext(13), ext(17)]; // f = c0 + c1 x + c2 x² + c3 x³
        let beta = ext(99);
        let log_height = 5;
        for index in [0usize, 1, 3, 6, 17, 30] {
            let s = fold_point(log_height, index);
            assert_ne!(s, Val::ZERO, "fold point must be a nonzero root of unity");

            let e0 = eval_cubic(&c, s);
            let e1 = eval_cubic(&c, -s);
            let fold = fold_pair(e0, e1, beta, s);

            // g(y) = f_even(y) + beta·f_odd(y), f_even = c0 + c2 y, f_odd = c1 + c3 y.
            let s2 = s * s;
            let f_even = c[0] + c[2] * s2;
            let f_odd = c[1] + c[3] * s2;
            let g_at_s2 = f_even + beta * f_odd;

            assert_eq!(fold, g_at_s2, "FRI fold != folded-polynomial evaluation at index {index}");
        }
    }

    /// Forgery: a tampered sibling evaluation changes the fold (the fold binds both evals).
    #[test]
    fn tampered_eval_changes_fold() {
        let s = fold_point(4, 5);
        let beta = ext(42);
        let base = fold_pair(ext(3), ext(8), beta, s);
        assert_ne!(base, fold_pair(ext(3) + Challenge::ONE, ext(8), beta, s));
        assert_ne!(base, fold_pair(ext(3), ext(8) + Challenge::ONE, beta, s));
    }

    /// Forgery: a different round challenge `beta` changes the fold (so a prover cannot
    /// swap in a favorable challenge — the in-AIR `beta` is bound to the transcript by
    /// gadget 1).
    #[test]
    fn different_beta_changes_fold() {
        let s = fold_point(4, 5);
        let f0 = fold_pair(ext(3), ext(8), ext(1), s);
        let f1 = fold_pair(ext(3), ext(8), ext(2), s);
        assert_ne!(f0, f1);
    }

    /// `reverse_bits_len` matches the bit-reversal convention (sanity).
    #[test]
    fn reverse_bits_len_is_correct() {
        assert_eq!(reverse_bits_len(0b001, 3), 0b100);
        assert_eq!(reverse_bits_len(0b110, 3), 0b011);
        assert_eq!(reverse_bits_len(0, 5), 0);
        assert_eq!(reverse_bits_len(0b10110, 5), 0b01101);
    }

    /// The fold partner is `-s` WITHIN one query's opened coset pair `(s, -s)` (native
    /// `xs = [subgroup_start, subgroup_start · two_adic_generator(1)]`, and
    /// `two_adic_generator(1) = -1`). Confirms the second eval point is the negation.
    #[test]
    fn coset_pair_second_point_is_negation() {
        // two_adic_generator(1) is the primitive 2nd root of unity = -1.
        assert_eq!(Val::two_adic_generator(1), -Val::ONE);
        let s = fold_point(4, 5);
        assert_eq!(s * Val::two_adic_generator(1), -s);
    }
}
