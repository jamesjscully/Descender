//! Gadget 3 (proper) — FRI commit-phase binary fold as a real `Air<AB>` (Option A).
//!
//! Lifts the VALIDATED [`crate::fri_fold`] reference (`fold_pair` = 2-point Lagrange
//! interpolation through `(s, e0), (-s, e1)` at the round challenge `beta`) into a Plonky3
//! `Air<AB>`, one trace ROW per per-query / per-round binary fold. The fold is over the
//! deg-8 challenge extension `Challenge = BinomialExtensionField<Val, 8>` (`X^8 = W`,
//! `W = 3` for KoalaBear-8), so each extension element is carried as 8 `Val` limb columns and
//! the extension arithmetic is reproduced IN-AIR over those limbs.
//!
//! ## The constraint (degree ≤ 3, division-free)
//!
//! The reference computes `folded = [e0·(beta + s) − e1·(beta − s)] · (2s)^{-1}` where
//! `e0, e1, beta ∈ Challenge` and `s ∈ Val` (a root of unity, so `2s ≠ 0`). A division can't
//! be a polynomial constraint, so we enforce the cleared form
//!
//!   `folded · (2s)  ==  e0·(beta + s) − e1·(beta − s)`               (★)
//!
//! limb-for-limb (8 equalities). `s` is a base-field scalar, so `beta ± s` only shifts limb 0
//! and `· (2s)` / `· s` scale a whole element limbwise (NO reduction — base scaling stays in
//! the same limb). The two extension PRODUCTS `e0·(beta+s)`, `e1·(beta−s)` are full octic
//! multiplications: with `X^8 = W`,
//!
//!   `(a·b)[k] = Σ_{i+j=k} a[i]·b[j]  +  W · Σ_{i+j=k+8} a[i]·b[j]`     (octic mul)
//!
//! mirroring `p3-field`'s `binomial_mul` for `D = 8`. Each product limb is degree 2 in the
//! committed limbs; the trailing `·(2s)` / `·s` adds one base-var factor ⇒ degree 3.
//!
//! ## Soundness discipline
//!
//! Honest fold traces (built from the reference [`fold_pair`]) ACCEPT via the real
//! `p3_air::check_all_constraints`; tampering `e0`, `e1`, `beta`, the committed point `s`, or
//! the claimed `folded` result each REJECTS. Soundness is shown by attempted forgery, never by
//! inspection — the same discipline as gadget-1 / gadget-2. The point `s` is a committed column
//! here; binding it to the FS-sampled query index (`fold_point(log_height, index)`) is the
//! cross-gadget index bus (see [`crate::index_bus`]).

use crate::fri_fold::fold_pair;
use descender_account_stark::{Challenge, Val};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

/// Extension degree (deg-8 challenge field). `Challenge = BinomialExtensionField<Val, 8>`.
pub const D: usize = 8;

/// The binomial extension modulus constant: `X^D = W`. For KoalaBear-8, `W = 3`
/// (`p3-koala-bear-0.6.1` `BinomialExtensionData<8>::W = KoalaBear::new(3)`). Re-derived /
/// pinned by [`tests::w_matches_native_extension`] against a real `Challenge` multiplication,
/// so a wrong constant is caught by attempted equality, never assumed.
pub const W: u32 = 3;

// ----- column layout (one binary fold per row) -----
/// First sibling eval `e0 = f(s)` (8 limbs, little-endian: limb 0 = constant term).
pub const C_E0: usize = 0;
/// Second sibling eval `e1 = f(-s)` (8 limbs).
pub const C_E1: usize = C_E0 + D;
/// Round fold challenge `beta` (8 limbs).
pub const C_BETA: usize = C_E1 + D;
/// The fold evaluation point `s` (a base-field root of unity; 1 column).
pub const C_S: usize = C_BETA + D;
/// The claimed folded eval `folded = g(s²)` (8 limbs).
pub const C_FOLDED: usize = C_S + 1;
/// Total AIR width.
pub const N_COLS: usize = C_FOLDED + D;

/// Limbs of a `Challenge` as `[Val; D]` (little-endian basis coefficients).
pub fn limbs(x: Challenge) -> [Val; D] {
    let s = x.as_basis_coefficients_slice();
    core::array::from_fn(|i| s[i])
}

/// Build one fold-level row from sibling evals `(e0, e1)`, round challenge `beta`, and point
/// `s`. The `folded` cell is the reference [`fold_pair`] (the authority the AIR is checked
/// against).
pub fn pack_fold(e0: Challenge, e1: Challenge, beta: Challenge, s: Val) -> [Val; N_COLS] {
    let folded = fold_pair(e0, e1, beta, s);
    let mut row = [Val::ZERO; N_COLS];
    row[C_E0..C_E0 + D].copy_from_slice(&limbs(e0));
    row[C_E1..C_E1 + D].copy_from_slice(&limbs(e1));
    row[C_BETA..C_BETA + D].copy_from_slice(&limbs(beta));
    row[C_S] = s;
    row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded));
    row
}

/// Serialize a sequence of independent per-query / per-round folds into the flat AIR trace,
/// one row per fold. (Power-of-two height is not required by `check_all_constraints`; the
/// assembled `recursive_air` pads.)
pub fn build_trace(folds: &[(Challenge, Challenge, Challenge, Val)]) -> Vec<Val> {
    let mut flat = Vec::with_capacity(folds.len() * N_COLS);
    for &(e0, e1, beta, s) in folds {
        flat.extend_from_slice(&pack_fold(e0, e1, beta, s));
    }
    flat
}

/// Octic multiply `(a · b)` over `X^D = W` as in-AIR expressions:
/// `(a·b)[k] = Σ_{i+j=k} a[i]b[j] + W·Σ_{i+j=k+D} a[i]b[j]`. Mirrors `p3-field` `binomial_mul`
/// for `D = 8`. Inputs/outputs are `AB::Expr` limb arrays.
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

/// FRI commit-phase binary-fold AIR (one fold per row). Enforces the cleared fold identity (★)
/// limb-for-limb over the deg-8 challenge extension.
pub struct FriFoldAir;

impl<F: Sync> BaseAir<F> for FriFoldAir {
    fn width(&self) -> usize {
        N_COLS
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for FriFoldAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };

        let e0: [AB::Expr; D] = core::array::from_fn(|i| l(C_E0 + i));
        let e1: [AB::Expr; D] = core::array::from_fn(|i| l(C_E1 + i));
        let beta: [AB::Expr; D] = core::array::from_fn(|i| l(C_BETA + i));
        let folded: [AB::Expr; D] = core::array::from_fn(|i| l(C_FOLDED + i));
        let s = l(C_S);

        // beta + s : base scalar shifts only limb 0.
        let mut beta_plus_s = beta.clone();
        beta_plus_s[0] = beta[0].clone() + s.clone();
        // beta - s : base scalar shifts only limb 0.
        let mut beta_minus_s = beta.clone();
        beta_minus_s[0] = beta[0].clone() - s.clone();

        // e0·(beta + s)  and  e1·(beta − s)  (full octic products over X^8 = W).
        let p0 = octic_mul::<AB>(&e0, &beta_plus_s);
        let p1 = octic_mul::<AB>(&e1, &beta_minus_s);

        // (★) per limb:  folded[k] · (2s)  ==  p0[k] − p1[k].
        let two_s = s.double();
        for k in 0..D {
            let lhs = folded[k].clone() * two_s.clone();
            let rhs = p0[k].clone() - p1[k].clone();
            builder.assert_zero(lhs - rhs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fri_fold::fold_point;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_matrix::dense::RowMajorMatrix;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }

    /// A representative challenge with several nonzero limbs (so the octic mul exercises the
    /// `X^8 = W` wraparound, not just the constant term).
    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    fn matrix(folds: &[(Challenge, Challenge, Challenge, Val)]) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(build_trace(folds), N_COLS)
    }

    /// PIN: the in-AIR extension modulus `W` matches the NATIVE `Challenge` multiplication.
    /// The in-AIR octic-product formula (with W = 3) must equal a real `Challenge * Challenge`
    /// limb-for-limb; a wrong `W` would diverge here (attempted equality, not assumption).
    #[test]
    fn w_matches_native_extension() {
        let a = ext_multi(11);
        let b = ext_multi(23);
        let prod = a * b; // native deg-8 multiplication
        let (al, bl) = (limbs(a), limbs(b));
        let w = Val::from_u32(W);
        let got: [Val; D] = core::array::from_fn(|k| {
            let mut acc = Val::ZERO;
            for i in 0..D {
                for j in 0..D {
                    if i + j == k {
                        acc += al[i] * bl[j];
                    } else if i + j == k + D {
                        acc += w * al[i] * bl[j];
                    }
                }
            }
            acc
        });
        assert_eq!(
            got,
            limbs(prod),
            "in-AIR octic mul (W={W}) != native Challenge mul"
        );
    }

    /// GOLDEN (positive): honest fold rows — across both base-only and multi-limb evals, and
    /// real `fold_point`-derived points — ACCEPT via the real `check_all_constraints`. This is
    /// the cleared fold identity (★) validated end-to-end as an AIR.
    #[test]
    fn air_accepts_honest_folds() {
        let mut folds = Vec::new();
        // base-only evals (limb-0-only) at several real fold points
        for (lh, idx) in [(5usize, 0usize), (5, 3), (6, 17), (4, 5)] {
            let s = fold_point(lh, idx);
            folds.push((ext(7), ext(11), ext(99), s));
        }
        // multi-limb evals (exercise the X^8 = W wraparound)
        for (k, &(lh, idx)) in [(5usize, 1usize), (6, 30), (7, 100)].iter().enumerate() {
            let s = fold_point(lh, idx);
            let kk = k as u32;
            folds.push((
                ext_multi(10 + kk),
                ext_multi(40 + kk),
                ext_multi(70 + kk),
                s,
            ));
        }
        let m = matrix(&folds);
        let report = check_all_constraints(&FriFoldAir, &m, &[], None);
        assert!(
            report.is_ok(),
            "honest fold trace rejected: {} failures",
            report.failures.len()
        );
    }

    /// Forgery: a tampered first sibling eval `e0` ⇒ (★) no longer holds ⇒ rejected. (The fold
    /// binds both sibling evals; a prover can't swap an opened value.)
    #[test]
    fn air_rejects_tampered_e0() {
        let s = fold_point(5, 3);
        let mut m = matrix(&[(ext_multi(1), ext_multi(2), ext_multi(3), s)]);
        m.values[C_E0 + 2] += Val::ONE; // tamper a non-constant limb of e0
        assert!(!check_all_constraints(&FriFoldAir, &m, &[], None).is_ok());
    }

    /// Forgery: a tampered second sibling eval `e1` ⇒ rejected.
    #[test]
    fn air_rejects_tampered_e1() {
        let s = fold_point(6, 9);
        let mut m = matrix(&[(ext_multi(4), ext_multi(5), ext_multi(6), s)]);
        m.values[C_E1] += Val::ONE; // tamper the constant limb of e1
        assert!(!check_all_constraints(&FriFoldAir, &m, &[], None).is_ok());
    }

    /// Forgery: a different round challenge `beta` ⇒ rejected. The in-AIR `beta` is bound to the
    /// transcript by gadget 1; a prover can't substitute a favorable challenge.
    #[test]
    fn air_rejects_tampered_beta() {
        let s = fold_point(5, 7);
        let mut m = matrix(&[(ext_multi(7), ext_multi(8), ext_multi(9), s)]);
        m.values[C_BETA + 4] += Val::ONE; // tamper a high limb of beta
        assert!(!check_all_constraints(&FriFoldAir, &m, &[], None).is_ok());
    }

    /// Forgery: a wrong claimed folded result ⇒ rejected (the AIR pins `folded` to the
    /// interpolation, so a prover can't claim an arbitrary next-layer value).
    #[test]
    fn air_rejects_wrong_folded_result() {
        let s = fold_point(7, 33);
        let mut m = matrix(&[(ext_multi(11), ext_multi(12), ext_multi(13), s)]);
        m.values[C_FOLDED + 1] += Val::ONE;
        assert!(!check_all_constraints(&FriFoldAir, &m, &[], None).is_ok());
    }

    /// Forgery: a tampered fold POINT `s` ⇒ the interpolation is at the wrong abscissa ⇒
    /// rejected. (Once the index bus binds `s = fold_point(index)`, this is what stops a prover
    /// from evaluating the fold at a favorable point.)
    #[test]
    fn air_rejects_tampered_point() {
        let s = fold_point(5, 3);
        let mut m = matrix(&[(ext_multi(14), ext_multi(15), ext_multi(16), s)]);
        m.values[C_S] += Val::ONE; // a different (non-root-of-unity) point
        assert!(!check_all_constraints(&FriFoldAir, &m, &[], None).is_ok());
    }

    /// Multi-row honest trace accepts (per-query / per-round folds laid out one per row).
    #[test]
    fn air_accepts_multirow_trace() {
        let folds: Vec<_> = (0..8u32)
            .map(|q| {
                let s = fold_point(6, (q as usize) * 5 + 1);
                (
                    ext_multi(100 + q),
                    ext_multi(200 + q),
                    ext_multi(300 + q),
                    s,
                )
            })
            .collect();
        let report = check_all_constraints(&FriFoldAir, &matrix(&folds), &[], None);
        assert!(
            report.is_ok(),
            "honest multi-row fold trace rejected: {} failures",
            report.failures.len()
        );
    }
}
