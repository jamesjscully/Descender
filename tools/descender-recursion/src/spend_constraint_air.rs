//! Phase-2 SPEND recursive-verifier constraint mirror (`spend_constraint_air`) — re-derives the spend
//! relation's C(zeta) residuals over opened-at-zeta cells, folded PROVABLY via the shared
//! re-arithmetization primitive [`crate::alpha_fold::assert_committed_fold`].
//!
//! This is the spend analogue of [`crate::leaf_constraint_air`]. The re-arithmetization (the
//! committed-accumulator fold) is already done and proven; this module begins transcribing
//! `descender_account_stark::spend_air::SpendAir::eval`'s constraints into opened-cell residual form.
//! Each residual is degree ≤ 3 (the spend AIR already proves), so they fold at degree ≤ 4 through the
//! shared primitive — provable regardless of count.
//!
//! Built incrementally by constraint class. THIS increment covers the **residue-canonical** class (the
//! 18 reviewed-fq12 residues: bit-booleanity, value/​slack recomposition, `value + slack = q-1`), which
//! is pure trace-column and degree ≤ 2. Remaining classes (packed-witness byte packing, the two
//! Poseidon2 sponges via octic per-round residuals, checked-core SIS, the sigma identity, w/t digest
//! binding, relation-id, and the public-value bindings) extend `spend_residuals` the same way.

use crate::alpha_fold::{octic_add, octic_mul, octic_scalar, octic_sub};
use crate::fri_fold_air::D;
use descender_account_stark::spend_air::{
    residue_base, RESIDUE_CELLS, SPEND_Q, SPEND_RESIDUES, SPEND_RESIDUE_BITS,
};
use descender_account_stark::{Challenge, Val};
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

/// Number of residue cells opened for the residue-canonical class (= the residue region width).
pub const SPEND_RESIDUE_CELLS: usize = SPEND_RESIDUES * RESIDUE_CELLS;
/// Residual count for the residue-canonical class: per residue, 14 bit-bools + 1 value-recompose +
/// 14 slack-bools + 1 slack-recompose + 1 canonical (`value + slack = q-1`).
pub const SPEND_RESIDUE_RESIDUALS: usize =
    SPEND_RESIDUES * (SPEND_RESIDUE_BITS + 1 + SPEND_RESIDUE_BITS + 1 + 1);

/// In-AIR residue-canonical residuals over opened-at-zeta cells. `cell(c)` returns the opened value
/// (octic extension limbs) of spend trace column `c`. Mirrors `spend_air`'s `assert_canonical_residue`
/// for all 18 residues, in a fixed order (matched by [`spend_residue_residual_values`]).
pub fn spend_residue_residuals<AB, FCell>(cell: &FCell) -> Vec<[AB::Expr; D]>
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let one = octic_scalar::<AB>(1);
    let qm1 = octic_scalar::<AB>((SPEND_Q - 1) as u64);
    let recompose = |bits_base: usize| -> [AB::Expr; D] {
        let mut acc = octic_scalar::<AB>(0);
        for b in 0..SPEND_RESIDUE_BITS {
            let scaled = octic_mul::<AB>(&cell(bits_base + b), &octic_scalar::<AB>(1u64 << b));
            acc = octic_add::<AB>(&acc, &scaled);
        }
        acc
    };
    let mut res: Vec<[AB::Expr; D]> = Vec::with_capacity(SPEND_RESIDUE_RESIDUALS);
    for r in 0..SPEND_RESIDUES {
        let base = residue_base(r);
        let val = cell(base);
        // value bits: bit·(bit−1) booleanity.
        for b in 0..SPEND_RESIDUE_BITS {
            let bit = cell(base + 1 + b);
            res.push(octic_mul::<AB>(&bit, &octic_sub::<AB>(&bit, &one)));
        }
        // value − recompose(value bits).
        res.push(octic_sub::<AB>(&val, &recompose(base + 1)));
        let slack = cell(base + 1 + SPEND_RESIDUE_BITS);
        // slack bits booleanity.
        for b in 0..SPEND_RESIDUE_BITS {
            let bit = cell(base + 2 + SPEND_RESIDUE_BITS + b);
            res.push(octic_mul::<AB>(&bit, &octic_sub::<AB>(&bit, &one)));
        }
        // slack − recompose(slack bits).
        res.push(octic_sub::<AB>(&slack, &recompose(base + 2 + SPEND_RESIDUE_BITS)));
        // canonical: (value + slack) − (q − 1).
        res.push(octic_sub::<AB>(&octic_add::<AB>(&val, &slack), &qm1));
    }
    debug_assert_eq!(res.len(), SPEND_RESIDUE_RESIDUALS);
    res
}

/// Native residue-canonical residual VALUES over opened cell values (same order/formula as
/// [`spend_residue_residuals`]) — for the committed-accumulator witness fill.
pub fn spend_residue_residual_values(cells: &[Challenge]) -> Vec<Challenge> {
    let one = Challenge::ONE;
    let qm1 = Challenge::from_u32(SPEND_Q - 1);
    let recompose = |bits_base: usize| -> Challenge {
        let mut acc = Challenge::ZERO;
        for b in 0..SPEND_RESIDUE_BITS {
            acc += cells[bits_base + b] * Challenge::from_u32(1u32 << b);
        }
        acc
    };
    let mut res = Vec::with_capacity(SPEND_RESIDUE_RESIDUALS);
    for r in 0..SPEND_RESIDUES {
        let base = residue_base(r);
        let val = cells[base];
        for b in 0..SPEND_RESIDUE_BITS {
            let bit = cells[base + 1 + b];
            res.push(bit * (bit - one));
        }
        res.push(val - recompose(base + 1));
        let slack = cells[base + 1 + SPEND_RESIDUE_BITS];
        for b in 0..SPEND_RESIDUE_BITS {
            let bit = cells[base + 2 + SPEND_RESIDUE_BITS + b];
            res.push(bit * (bit - one));
        }
        res.push(slack - recompose(base + 2 + SPEND_RESIDUE_BITS));
        res.push((val + slack) - qm1);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alpha_fold::{assert_committed_fold, committed_fold_values, native_alpha_fold};
    use crate::fri_fold_air::limbs;
    use p3_air::{Air, BaseAir, WindowAccess};
    use p3_field::BasedVectorSpace;
    use p3_matrix::dense::RowMajorMatrix;

    const CELLS: usize = SPEND_RESIDUE_CELLS;
    const K: usize = SPEND_RESIDUE_RESIDUALS;
    const ALPHA_OFF: usize = CELLS * D;
    const APOW_OFF: usize = ALPHA_OFF + D;
    const ACC_OFF: usize = APOW_OFF + K * D;
    const WIDTH_COLS: usize = ACC_OFF + K * D;

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1))
        })
    }

    /// A test AIR folding the spend residue-canonical residuals via the shared committed-accumulator
    /// primitive, exposing the fold result `C_residue(zeta) = Σ alpha^r · res_r` as public values.
    struct SpendResidueFoldAir;
    impl<F: Sync> BaseAir<F> for SpendResidueFoldAir {
        fn width(&self) -> usize {
            WIDTH_COLS
        }
        fn num_public_values(&self) -> usize {
            D
        }
    }
    impl<AB: AirBuilder<F = Val>> Air<AB> for SpendResidueFoldAir {
        fn eval(&self, builder: &mut AB) {
            let main = builder.main();
            let local = main.current_slice().to_vec();
            let public = builder.public_values().to_vec();
            let cell = |c: usize| -> [AB::Expr; D] {
                core::array::from_fn(|i| local[c * D + i].into())
            };
            let apow = |i: usize| -> [AB::Expr; D] {
                core::array::from_fn(|j| local[APOW_OFF + i * D + j].into())
            };
            let acc = |i: usize| -> [AB::Expr; D] {
                core::array::from_fn(|j| local[ACC_OFF + i * D + j].into())
            };
            let alpha: [AB::Expr; D] = core::array::from_fn(|j| local[ALPHA_OFF + j].into());
            let residuals = spend_residue_residuals::<AB, _>(&cell);
            let result = assert_committed_fold::<AB, _, _>(builder, &residuals, &apow, &acc, &alpha);
            for j in 0..D {
                builder.assert_eq(result[j].clone(), public[j].clone().into());
            }
        }
    }

    fn build(cells: &[Challenge], alpha: Challenge) -> (RowMajorMatrix<Val>, Vec<Val>) {
        let resv = spend_residue_residual_values(cells);
        let (apow, accv) = committed_fold_values(alpha, &resv);
        let mut row = alloc::vec![Val::ZERO; WIDTH_COLS];
        for (c, v) in cells.iter().enumerate() {
            row[c * D..c * D + D].copy_from_slice(&limbs(*v));
        }
        row[ALPHA_OFF..ALPHA_OFF + D].copy_from_slice(&limbs(alpha));
        for i in 0..K {
            row[APOW_OFF + i * D..APOW_OFF + i * D + D].copy_from_slice(&limbs(apow[i]));
            row[ACC_OFF + i * D..ACC_OFF + i * D + D].copy_from_slice(&limbs(accv[i]));
        }
        let result = *accv.last().unwrap();
        // FIDELITY: the committed fold result equals the native Σ alpha^r·res_r.
        assert_eq!(result, native_alpha_fold(alpha, &resv));
        let pv = limbs(result).to_vec();
        let mut values = Vec::with_capacity(2 * WIDTH_COLS);
        values.extend_from_slice(&row);
        values.extend_from_slice(&row);
        (RowMajorMatrix::new(values, WIDTH_COLS), pv)
    }

    /// CONSTRAINT-LEVEL: the spend residue-canonical mirror + committed fold hold on arbitrary opened
    /// cells (the fold reconstructs C_residue(zeta); a valid OOD opening is the separate DEEP concern).
    #[test]
    fn spend_residue_fold_constraints_hold() {
        use p3_air::check_all_constraints;
        let cells: Vec<Challenge> = (0..CELLS as u32).map(|c| ext(c + 1)).collect();
        let alpha = ext(777);
        let (trace, pv) = build(&cells, alpha);
        let report = check_all_constraints(&SpendResidueFoldAir, &trace, &pv, None);
        assert!(report.is_ok(), "spend residue fold rejected: {} failures", report.failures.len());
    }

    /// THE POINT: the spend residue-class C(zeta) folds PROVABLY at PRODUCTION q128 knobs via the shared
    /// re-arithmetization primitive — the spend recursive verifier's first constraint class, provable.
    #[test]
    #[ignore = "slow: spend residue-class C(zeta) fold prove at production q128 knobs"]
    fn spend_residue_fold_proves_q128() {
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

        let cells: Vec<Challenge> = (0..CELLS as u32).map(|c| ext(c + 1)).collect();
        let alpha = ext(777);
        let (trace, pv) = build(&cells, alpha);
        let proof = prove(&config, &SpendResidueFoldAir, trace, &pv);
        verify(&config, &SpendResidueFoldAir, &proof, &pv)
            .expect("spend residue-class C(zeta) fold must prove+verify at production q128 knobs");
    }
}
