//! Low-degree alpha-batch (Horner) fold over the degree-8 challenge extension — the re-arithmetization
//! primitive that makes the full inner-leaf C(zeta) re-derivation PROVABLE.
//!
//! # Why this exists
//!
//! `leaf_constraint_air::recompute_c_partial` folds ~13000 degree-≤3 leaf residuals into one DEEP
//! C(zeta) via `acc = Σ_r alpha^r · res_r`, computing `alpha^r` by repeated multiplication INLINE.
//! With `alpha` an OPENED column, the returned expression has algebraic degree ~13000 in `alpha`, so
//! `uni_stark`'s quotient LDE explodes (tens of GB) — the rung constraint-checks but cannot be proven.
//!
//! This module folds the SAME sum but COMMITS the running Horner state as columns, so every constraint
//! is degree ≤ 2:
//!   * `a_pow[0]   = 1`
//!   * `a_pow[i]   = a_pow[i-1] · alpha`                  (degree 2)
//!   * `acc[0]     = a_pow[0] · res[0] = res[0]`
//!   * `acc[i]     = acc[i-1] + a_pow[i] · res[i]`        (degree 2)
//!   * final `acc[K-1] = Σ_r alpha^r · res_r`             (the fold result)
//!
//! The fold result is independent of K's effect on degree — it stays degree ≤ 2 regardless of how many
//! residuals are folded, so the leaf's full C(zeta) becomes provable.

use crate::fri_fold_air::{limbs, D, W};
use descender_account_stark::{Challenge, Val};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

extern crate alloc;
use alloc::vec::Vec;

/// Octic add over the degree-8 extension (componentwise), as in-AIR expressions.
pub fn octic_add<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() + b[i].clone())
}

/// Octic multiply `(a · b)` over `X^D = W` as in-AIR expressions:
/// `(a·b)[k] = Σ_{i+j=k} a[i]·b[j] + W · Σ_{i+j=k+D} a[i]·b[j]`. Degree 2 in the operands.
pub fn octic_mul<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    let w = AB::Expr::from(AB::F::from_u64(W as u64));
    core::array::from_fn(|k| {
        let mut lo = AB::Expr::ZERO;
        for i in 0..=k {
            lo += a[i].clone() * b[k - i].clone();
        }
        let mut hi = AB::Expr::ZERO;
        for i in (k + 1)..D {
            // i + j = k + D ⇒ j = k + D - i, with j in 0..D.
            hi += a[i].clone() * b[k + D - i].clone();
        }
        lo + w.clone() * hi
    })
}

/// `octic_one()` as in-AIR limbs `[1, 0, ..., 0]`.
fn octic_one<AB: AirBuilder>() -> [AB::Expr; D] {
    core::array::from_fn(|i| if i == 0 { AB::Expr::ONE } else { AB::Expr::ZERO })
}

/// Column layout for a K-term fold block at base `off`: `alpha[D] ‖ res[K·D] ‖ a_pow[K·D] ‖ acc[K·D]`.
pub const fn fold_block_width(k: usize) -> usize {
    D + 3 * k * D
}
const fn alpha_off(off: usize) -> usize {
    off
}
const fn res_off(off: usize, i: usize) -> usize {
    off + D + i * D
}
const fn apow_off(off: usize, k: usize, i: usize) -> usize {
    off + D + k * D + i * D
}
const fn acc_off(off: usize, k: usize, i: usize) -> usize {
    off + D + 2 * k * D + i * D
}

/// Assert the committed-accumulator Horner fold over `K` residuals (degree ≤ 2 per constraint), and
/// return the fold result `acc[K-1] = Σ_r alpha^r · res_r` as an expression. `local` is the trace row.
pub fn eval_alpha_fold<AB: AirBuilder<F = Val>>(
    builder: &mut AB,
    local: &[AB::Var],
    off: usize,
    k: usize,
) -> [AB::Expr; D] {
    assert!(k >= 1);
    let get = |base: usize| -> [AB::Expr; D] { core::array::from_fn(|i| local[base + i].into()) };

    let alpha = get(alpha_off(off));
    // a_pow[0] == 1
    let a0 = get(apow_off(off, k, 0));
    let one = octic_one::<AB>();
    for i in 0..D {
        builder.assert_eq(a0[i].clone(), one[i].clone());
    }
    // acc[0] == res[0]   (since a_pow[0] = 1)
    let acc0 = get(acc_off(off, k, 0));
    let res0 = get(res_off(off, 0));
    for i in 0..D {
        builder.assert_eq(acc0[i].clone(), res0[i].clone());
    }
    for step in 1..k {
        // a_pow[step] == a_pow[step-1] · alpha
        let prev_pow = get(apow_off(off, k, step - 1));
        let pow = get(apow_off(off, k, step));
        let want_pow = octic_mul::<AB>(&prev_pow, &alpha);
        for i in 0..D {
            builder.assert_eq(pow[i].clone(), want_pow[i].clone());
        }
        // acc[step] == acc[step-1] + a_pow[step] · res[step]
        let prev_acc = get(acc_off(off, k, step - 1));
        let res = get(res_off(off, step));
        let acc = get(acc_off(off, k, step));
        let term = octic_mul::<AB>(&pow, &res);
        let want_acc = octic_add::<AB>(&prev_acc, &term);
        for i in 0..D {
            builder.assert_eq(acc[i].clone(), want_acc[i].clone());
        }
    }
    get(acc_off(off, k, k - 1))
}

/// GENERIC committed-accumulator fold over residuals supplied as EXPRESSIONS (e.g. a relation's
/// C(zeta) residuals re-derived from opened-at-zeta cells). Asserts the Horner chain against committed
/// `apow(i)`/`acc(i)` extension columns and returns the fold result `Σ_r alpha^r · res_r`. Every
/// emitted constraint is degree ≤ (1 + deg(res_r)) — independent of the residual COUNT — so any
/// relation's full C(zeta) fold becomes provable. This is the re-arithmetization primitive; both the
/// creation leaf mirror and the (future) spend mirror fold through it.
pub fn assert_committed_fold<AB, FApow, FAcc>(
    builder: &mut AB,
    residuals: &[[AB::Expr; D]],
    apow: &FApow,
    acc: &FAcc,
    alpha: &[AB::Expr; D],
) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FApow: Fn(usize) -> [AB::Expr; D],
    FAcc: Fn(usize) -> [AB::Expr; D],
{
    let n = residuals.len();
    assert!(n >= 1);
    let one = octic_one::<AB>();
    let a0 = apow(0);
    for i in 0..D {
        builder.assert_eq(a0[i].clone(), one[i].clone());
    }
    let acc0 = acc(0);
    for i in 0..D {
        builder.assert_eq(acc0[i].clone(), residuals[0][i].clone());
    }
    for step in 1..n {
        let pow = apow(step);
        let want_pow = octic_mul::<AB>(&apow(step - 1), alpha);
        for i in 0..D {
            builder.assert_eq(pow[i].clone(), want_pow[i].clone());
        }
        let cur = acc(step);
        let term = octic_mul::<AB>(&pow, &residuals[step]);
        let want = octic_add::<AB>(&acc(step - 1), &term);
        for i in 0..D {
            builder.assert_eq(cur[i].clone(), want[i].clone());
        }
    }
    acc(n - 1)
}

/// GENERIC native witness fill for [`assert_committed_fold`]: given the residual VALUES, returns the
/// committed Horner state `(a_pow[n], acc[n])` (`a_pow[r]=alpha^r`, `acc[r]=Σ_{j≤r} alpha^j·res_j`).
pub fn committed_fold_values(alpha: Challenge, residuals: &[Challenge]) -> (Vec<Challenge>, Vec<Challenge>) {
    let mut apow = Vec::with_capacity(residuals.len());
    let mut accv = Vec::with_capacity(residuals.len());
    let mut a = Challenge::ONE;
    let mut s = Challenge::ZERO;
    for &r in residuals {
        s += a * r;
        apow.push(a);
        accv.push(s);
        a *= alpha;
    }
    (apow, accv)
}

/// Native reference fold: `Σ_r alpha^r · res_r` (the value the in-AIR fold reconstructs).
pub fn native_alpha_fold(alpha: Challenge, residuals: &[Challenge]) -> Challenge {
    let mut acc = Challenge::ZERO;
    let mut a_pow = Challenge::ONE;
    for &r in residuals {
        acc += a_pow * r;
        a_pow *= alpha;
    }
    acc
}

/// Fill a fold block's columns (`alpha ‖ res ‖ a_pow ‖ acc`) into `row` at `off`, returning the fold
/// result. The native arithmetic mirrors `eval_alpha_fold` exactly.
pub fn fill_alpha_fold(row: &mut [Val], off: usize, alpha: Challenge, residuals: &[Challenge]) -> Challenge {
    let k = residuals.len();
    row[alpha_off(off)..alpha_off(off) + D].copy_from_slice(&limbs(alpha));
    let mut a_pow = Challenge::ONE;
    let mut acc = Challenge::ZERO;
    for (i, &r) in residuals.iter().enumerate() {
        acc += a_pow * r;
        row[res_off(off, i)..res_off(off, i) + D].copy_from_slice(&limbs(r));
        row[apow_off(off, k, i)..apow_off(off, k, i) + D].copy_from_slice(&limbs(a_pow));
        row[acc_off(off, k, i)..acc_off(off, k, i) + D].copy_from_slice(&limbs(acc));
        a_pow *= alpha;
    }
    acc
}

/// A minimal AIR that folds `k` residuals via the committed-accumulator Horner chain and exposes the
/// fold result as public values — exists to PROVE the re-arithmetized fold at production knobs.
pub struct AlphaFoldAir {
    pub k: usize,
}

impl<F: Sync> BaseAir<F> for AlphaFoldAir {
    fn width(&self) -> usize {
        fold_block_width(self.k)
    }
    fn num_public_values(&self) -> usize {
        D
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for AlphaFoldAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.current_slice().to_vec();
        let public = builder.public_values().to_vec();
        let result = eval_alpha_fold(builder, &local, 0, self.k);
        for i in 0..D {
            builder.assert_eq(result[i].clone(), public[i].clone().into());
        }
    }
}

/// Build a 2-row trace + public values folding the given residuals.
pub fn build_fold_trace(alpha: Challenge, residuals: &[Challenge]) -> (RowMajorMatrix<Val>, Vec<Val>) {
    let k = residuals.len();
    let w = fold_block_width(k);
    let mut row = alloc::vec![Val::ZERO; w];
    let result = fill_alpha_fold(&mut row, 0, alpha, residuals);
    let pv = limbs(result).to_vec();
    let mut values = Vec::with_capacity(2 * w);
    values.extend_from_slice(&row);
    values.extend_from_slice(&row);
    (RowMajorMatrix::new(values, w), pv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::BasedVectorSpace;

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1)))
    }

    /// The native fold matches a direct Σ alpha^r·res_r.
    #[test]
    fn native_fold_matches_direct() {
        let alpha = ext(7);
        let res: Vec<Challenge> = (0..50).map(|i| ext(100 + i)).collect();
        let mut direct = Challenge::ZERO;
        let mut p = Challenge::ONE;
        for &r in &res {
            direct += p * r;
            p *= alpha;
        }
        assert_eq!(native_alpha_fold(alpha, &res), direct);
    }

    /// CONSTRAINT-LEVEL: an honest fold block satisfies the committed-accumulator constraints.
    #[test]
    fn honest_fold_constraints_hold() {
        use p3_air::check_all_constraints;
        let alpha = ext(11);
        let res: Vec<Challenge> = (0..300).map(|i| ext(200 + i)).collect();
        let (trace, pv) = build_fold_trace(alpha, &res);
        let report = check_all_constraints(&AlphaFoldAir { k: res.len() }, &trace, &pv, None);
        assert!(report.is_ok(), "honest fold rejected: {} failures", report.failures.len());
    }

    /// THE POINT: the re-arithmetized fold of MANY residuals PROVES at PRODUCTION q128 knobs (degree
    /// ≤ 2, regardless of K) — where the inline `Σ alpha^r·res_r` would be degree-K and explode.
    #[test]
    fn proves_large_fold_q128() {
        use descender_account_stark::{
            Challenge as Ch, MyCompress, MyHash, Perm, DIGEST_ELEMS, PROD_LOG_BLOWUP,
            PROD_NUM_QUERIES, PROD_QUERY_POW_BITS,
        };
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Ch, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Ch, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(MyHash::new(perm.clone()), MyCompress::new(MyHash::new(perm.clone())), 0);
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES,
            commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS,
            mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));

        let alpha = ext(42);
        let res: Vec<Challenge> = (0..2000u32).map(|i| ext(500 + i)).collect();
        let (trace, pv) = build_fold_trace(alpha, &res);
        let proof = prove(&config, &AlphaFoldAir { k: res.len() }, trace, &pv);
        verify(&config, &AlphaFoldAir { k: res.len() }, &proof, &pv)
            .expect("re-arithmetized 2000-term fold must prove+verify at production q128 knobs");
    }
}
