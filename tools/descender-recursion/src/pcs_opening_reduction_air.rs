//! Constant-width vertical form of Plonky3's PCS `open_input` reduction.
//!
//! For every authenticated query, `p3_fri::verifier::open_input` walks the
//! canonical `(batch, matrix, point, column)` term order and updates
//!
//! ```text
//! ro        <- ro + alpha_pow * (p_at_z - p_at_x) / (z - x)
//! alpha_pow <- alpha_pow * alpha
//! ```
//!
//! for all matrices in one log-height bucket.  Relation-v3 has one such bucket,
//! so [`crate::pcs_opening_shape::PRODUCTION_OPENING_TERMS_V1`] rows reproduce a
//! complete production query without a 23,254-term horizontal expression.  The
//! committed `alpha_pow` and `ro` states keep every constraint degree bounded.
//!
//! This AIR makes every ordered `p_at_z` claim load-bearing in the reduction.  It
//! is not, alone, a PCS verifier: deployment composition must still (1) bind this
//! exact `p_at_z` sequence to the vertical Fiat--Shamir OOD stream, (2) authenticate
//! every `p_at_x` row against its MMCS commitment, and (3) feed the public terminal
//! reduced opening into the corresponding FRI query chain.  Those links are kept
//! explicit rather than treating a correct arithmetic reducer as full soundness.

use crate::alpha_fold::{octic_add, octic_mul, octic_sub};
use crate::fri_fold_air::{limbs, D};
use descender_account_stark::{Challenge, Val};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

pub const C_ACTIVE: usize = 0;
pub const C_REMAINING: usize = C_ACTIVE + 1;
pub const C_ALPHA: usize = C_REMAINING + 1;
pub const C_Z: usize = C_ALPHA + D;
pub const C_X: usize = C_Z + D;
pub const C_P_AT_X: usize = C_X + 1;
pub const C_P_AT_Z: usize = C_P_AT_X + 1;
pub const C_INV: usize = C_P_AT_Z + D;
pub const C_ALPHA_POW: usize = C_INV + D;
pub const C_RO: usize = C_ALPHA_POW + D;
pub const C_ALPHA_POW_OUT: usize = C_RO + D;
pub const C_RO_OUT: usize = C_ALPHA_POW_OUT + D;
pub const N_COLS: usize = C_RO_OUT + D;

pub const PV_ALPHA: usize = 0;
pub const PV_REDUCED_OPENING: usize = PV_ALPHA + D;
pub const N_PUBLIC: usize = PV_REDUCED_OPENING + D;

#[derive(Clone, Copy, Debug)]
pub struct OpeningReductionTerm {
    pub z: Challenge,
    pub x: Val,
    pub p_at_x: Val,
    pub p_at_z: Challenge,
}

#[derive(Clone, Copy, Debug)]
pub struct PcsOpeningReductionAir {
    pub terms: usize,
}

impl PcsOpeningReductionAir {
    pub fn new(terms: usize) -> Self {
        assert!(terms > 0, "opening reduction needs at least one term");
        Self { terms }
    }

    /// Keep at least one padding row so the final boundary exposes the state
    /// after every active term, including when `terms` itself is a power of two.
    pub fn trace_height(self) -> usize {
        (self.terms + 1).next_power_of_two().max(2)
    }

    pub fn base_witness_bytes(self) -> usize {
        self.trace_height() * N_COLS * core::mem::size_of::<Val>()
    }
}

impl<F: Sync> BaseAir<F> for PcsOpeningReductionAir {
    fn width(&self) -> usize {
        N_COLS
    }

    fn num_public_values(&self) -> usize {
        N_PUBLIC
    }
}

fn ext_expr<AB: AirBuilder>(local: &[AB::Var], base: usize) -> [AB::Expr; D] {
    core::array::from_fn(|i| local[base + i].into())
}

fn ext_scalar_expr<AB: AirBuilder>(value: AB::Expr) -> [AB::Expr; D] {
    core::array::from_fn(|i| {
        if i == 0 {
            value.clone()
        } else {
            AB::Expr::ZERO
        }
    })
}

fn ext_one_expr<AB: AirBuilder>() -> [AB::Expr; D] {
    core::array::from_fn(|i| {
        if i == 0 {
            AB::Expr::ONE
        } else {
            AB::Expr::ZERO
        }
    })
}

impl<AB: AirBuilder<F = Val>> Air<AB> for PcsOpeningReductionAir {
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

        let active = l(C_ACTIVE);
        builder.assert_bool(active.clone());

        let alpha = ext_expr::<AB>(&local, C_ALPHA);
        let z = ext_expr::<AB>(&local, C_Z);
        let p_at_z = ext_expr::<AB>(&local, C_P_AT_Z);
        let inv = ext_expr::<AB>(&local, C_INV);
        let alpha_pow = ext_expr::<AB>(&local, C_ALPHA_POW);
        let ro = ext_expr::<AB>(&local, C_RO);
        let alpha_pow_out = ext_expr::<AB>(&local, C_ALPHA_POW_OUT);
        let ro_out = ext_expr::<AB>(&local, C_RO_OUT);
        let x = ext_scalar_expr::<AB>(l(C_X));
        let p_at_x = ext_scalar_expr::<AB>(l(C_P_AT_X));

        // The witnessed denominator inverse is checked in the extension field.
        // This is exactly the verifier's reject-on-z==x condition.
        let denominator = octic_sub::<AB>(&z, &x);
        let inverse_check = octic_mul::<AB>(&denominator, &inv);
        let one = ext_one_expr::<AB>();
        for i in 0..D {
            builder.assert_zero(active.clone() * (inverse_check[i].clone() - one[i].clone()));
        }

        let delta = octic_sub::<AB>(&p_at_z, &p_at_x);
        let quotient = octic_mul::<AB>(&delta, &inv);
        let contribution = octic_mul::<AB>(&alpha_pow, &quotient);
        let want_ro_out = octic_add::<AB>(&ro, &contribution);
        let want_alpha_pow_out = octic_mul::<AB>(&alpha_pow, &alpha);
        for i in 0..D {
            // Active rows consume one term; padding rows carry the terminal state.
            builder.assert_zero(
                active.clone() * (alpha_pow_out[i].clone() - want_alpha_pow_out[i].clone()),
            );
            builder.assert_zero(active.clone() * (ro_out[i].clone() - want_ro_out[i].clone()));
            let inactive = AB::Expr::ONE - active.clone();
            builder
                .assert_zero(inactive.clone() * (alpha_pow_out[i].clone() - alpha_pow[i].clone()));
            builder.assert_zero(inactive * (ro_out[i].clone() - ro[i].clone()));
        }

        // Exact active-prefix shape: `remaining` starts at the configured term
        // count, decreases once per active row, ends at zero, and active rows
        // cannot restart after padding.  This rules out truncating the claim list.
        {
            let mut first = builder.when_first_row();
            first.assert_one(active.clone());
            first.assert_eq(
                l(C_REMAINING),
                AB::Expr::from(AB::F::from_usize(self.terms)),
            );
            for i in 0..D {
                first.assert_eq(alpha_pow[i].clone(), one[i].clone());
                first.assert_zero(ro[i].clone());
                if pubs.len() == N_PUBLIC {
                    first.assert_eq(alpha[i].clone(), pubs[PV_ALPHA + i].clone());
                }
            }
        }
        {
            let mut tr = builder.when_transition();
            tr.assert_eq(n(C_REMAINING), l(C_REMAINING) - active.clone());
            tr.assert_zero(n(C_ACTIVE) * (AB::Expr::ONE - active.clone()));
            for i in 0..D {
                tr.assert_eq(n(C_ALPHA + i), alpha[i].clone());
                tr.assert_eq(n(C_ALPHA_POW + i), alpha_pow_out[i].clone());
                tr.assert_eq(n(C_RO + i), ro_out[i].clone());
            }
        }
        {
            let mut last = builder.when_last_row();
            last.assert_zero(active);
            last.assert_zero(l(C_REMAINING));
            if pubs.len() == N_PUBLIC {
                for i in 0..D {
                    last.assert_eq(ro[i].clone(), pubs[PV_REDUCED_OPENING + i].clone());
                }
            }
        }
    }
}

/// Native reference with the same iteration order and arithmetic as Plonky3
/// `open_input` for one log-height bucket.
pub fn reduce_opening_terms(alpha: Challenge, terms: &[OpeningReductionTerm]) -> Challenge {
    let mut alpha_pow = Challenge::ONE;
    let mut ro = Challenge::ZERO;
    for term in terms {
        let inv = (term.z - Challenge::from(term.x))
            .try_inverse()
            .expect("opening point must differ from query point");
        ro += alpha_pow * (term.p_at_z - Challenge::from(term.p_at_x)) * inv;
        alpha_pow *= alpha;
    }
    ro
}

pub fn public_values(alpha: Challenge, reduced_opening: Challenge) -> Vec<Val> {
    let mut out = Vec::with_capacity(N_PUBLIC);
    out.extend_from_slice(&limbs(alpha));
    out.extend_from_slice(&limbs(reduced_opening));
    out
}

pub fn build_trace(
    alpha: Challenge,
    terms: &[OpeningReductionTerm],
) -> (RowMajorMatrix<Val>, Challenge) {
    let air = PcsOpeningReductionAir::new(terms.len());
    let height = air.trace_height();
    let mut flat = vec![Val::ZERO; height * N_COLS];
    let alpha_limbs = limbs(alpha);
    let mut alpha_pow = Challenge::ONE;
    let mut ro = Challenge::ZERO;

    for row_index in 0..height {
        let row = &mut flat[row_index * N_COLS..(row_index + 1) * N_COLS];
        let active = row_index < terms.len();
        row[C_ACTIVE] = Val::from_bool(active);
        row[C_REMAINING] = Val::from_usize(terms.len().saturating_sub(row_index));
        row[C_ALPHA..C_ALPHA + D].copy_from_slice(&alpha_limbs);
        row[C_ALPHA_POW..C_ALPHA_POW + D].copy_from_slice(&limbs(alpha_pow));
        row[C_RO..C_RO + D].copy_from_slice(&limbs(ro));

        if active {
            let term = terms[row_index];
            row[C_Z..C_Z + D].copy_from_slice(&limbs(term.z));
            row[C_X] = term.x;
            row[C_P_AT_X] = term.p_at_x;
            row[C_P_AT_Z..C_P_AT_Z + D].copy_from_slice(&limbs(term.p_at_z));
            let inv = (term.z - Challenge::from(term.x))
                .try_inverse()
                .expect("opening point must differ from query point");
            row[C_INV..C_INV + D].copy_from_slice(&limbs(inv));
            ro += alpha_pow * (term.p_at_z - Challenge::from(term.p_at_x)) * inv;
            alpha_pow *= alpha;
        }
        row[C_ALPHA_POW_OUT..C_ALPHA_POW_OUT + D].copy_from_slice(&limbs(alpha_pow));
        row[C_RO_OUT..C_RO_OUT + D].copy_from_slice(&limbs(ro));
    }

    (RowMajorMatrix::new(flat, N_COLS), ro)
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::BasedVectorSpace;

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(65_537).wrapping_add(i as u32 + 1))
        })
    }

    fn terms(n: usize) -> Vec<OpeningReductionTerm> {
        (0..n)
            .map(|i| OpeningReductionTerm {
                z: ext(100 + (i % 3) as u32),
                x: Val::from_u32(10_000 + i as u32),
                p_at_x: Val::from_u32(20_000 + 3 * i as u32),
                p_at_z: ext(30_000 + i as u32),
            })
            .collect()
    }

    #[test]
    fn vertical_reduction_matches_native_formula_and_accepts() {
        for n in [1usize, 2, 7, 8, 17] {
            let alpha = ext(77 + n as u32);
            let input = terms(n);
            let expected = reduce_opening_terms(alpha, &input);
            let (trace, got) = build_trace(alpha, &input);
            assert_eq!(got, expected);
            let report = check_all_constraints(
                &PcsOpeningReductionAir::new(n),
                &trace,
                &public_values(alpha, expected),
                None,
            );
            assert!(
                report.is_ok(),
                "honest {n}-term reduction rejected: {} failures",
                report.failures.len()
            );
        }
    }

    #[test]
    fn rejects_claim_row_and_order_tampering() {
        let alpha = ext(91);
        let input = terms(9);
        let expected = reduce_opening_terms(alpha, &input);
        let (trace, _) = build_trace(alpha, &input);
        let pubs = public_values(alpha, expected);
        let air = PcsOpeningReductionAir::new(input.len());

        for (label, column) in [
            ("p_at_z", C_P_AT_Z + 3),
            ("p_at_x", C_P_AT_X),
            ("z", C_Z + 2),
            ("inverse", C_INV + 1),
            ("alpha power", C_ALPHA_POW + 4),
            ("reduced opening", C_RO_OUT + 5),
        ] {
            let mut bad = trace.values.clone();
            bad[3 * N_COLS + column] += Val::ONE;
            let report =
                check_all_constraints(&air, &RowMajorMatrix::new(bad, N_COLS), &pubs, None);
            assert!(!report.is_ok(), "tampered {label} was accepted");
        }

        let mut reordered = input.clone();
        reordered.swap(2, 3);
        let (reordered_trace, _) = build_trace(alpha, &reordered);
        assert!(
            !check_all_constraints(&air, &reordered_trace, &pubs, None).is_ok(),
            "reordered claims accepted against the original terminal reduction"
        );
    }

    #[test]
    fn rejects_truncated_active_prefix_and_wrong_public_result() {
        let alpha = ext(123);
        let input = terms(6);
        let expected = reduce_opening_terms(alpha, &input);
        let (trace, _) = build_trace(alpha, &input);
        let air = PcsOpeningReductionAir::new(input.len());
        let pubs = public_values(alpha, expected);

        let mut truncated = trace.values.clone();
        truncated[4 * N_COLS + C_ACTIVE] = Val::ZERO;
        assert!(
            !check_all_constraints(&air, &RowMajorMatrix::new(truncated, N_COLS), &pubs, None)
                .is_ok(),
            "truncated active prefix accepted"
        );

        let mut wrong = pubs;
        wrong[PV_REDUCED_OPENING] += Val::ONE;
        assert!(
            !check_all_constraints(&air, &trace, &wrong, None).is_ok(),
            "wrong terminal reduced opening accepted"
        );
    }

    #[test]
    fn production_shape_is_constant_width_and_sized() {
        let air =
            PcsOpeningReductionAir::new(crate::pcs_opening_shape::PRODUCTION_OPENING_TERMS_V1);
        assert_eq!(N_COLS, 68);
        assert_eq!(air.trace_height(), 32_768);
        assert_eq!(air.base_witness_bytes(), 8_912_896);
        assert_eq!(
            crate::pcs_opening_shape::PRODUCTION_OPENING_TERMS_V1,
            23_254
        );
    }

    #[test]
    fn vertical_opening_reduction_proves_at_production_q128_knobs() {
        use p3_uni_stark::{prove, verify};

        let alpha = ext(404);
        let input = terms(17);
        let (trace, reduced_opening) = build_trace(alpha, &input);
        let pubs = public_values(alpha, reduced_opening);
        let air = PcsOpeningReductionAir::new(input.len());
        let config = crate::settlement::prod_config();
        let proof = prove(&config, &air, trace, &pubs);
        verify(&config, &air, &proof, &pubs)
            .expect("vertical opening reduction must prove and verify at q128 knobs");
    }
}
