//! Phase-3 TRUE single-proof O(1) block aggregator — verify N members IN-CIRCUIT and chain their
//! verified roots in ONE proof.
//!
//! [`crate::accumulated_member`] folds in-circuit-verified Merkle ROOTS; the two-proof pipeline
//! (`recursive_air` per member + a separate accumulator) verifies members but yields N+1 proofs.
//! This AIR composes them into ONE: each trace ROW is a full `recursive_air` window (verifying that
//! member's opening in-circuit, leaf/root INTERNAL — the leaf/root public binding auto-skips because
//! this AIR's public-value count is the single `chain_state_root`, not the per-window
//! `N_PUBLIC_MERKLE`) followed by a chain-fold node that folds the window's VERIFIED root into the
//! running chain. With `recursive_air`'s only cross-row/boundary constraints gated behind
//! `fan_out_queries > 0` (off here), each row is an independent member verification, and this AIR
//! adds the genesis pin, the row→row chain, and the public `chain_state_root` on the last row.
//!
//! Result: ONE O(1) proof attests that every block member was verified in-circuit AND that they
//! chain to `chain_state_root` — the single on-chain artifact (no N-proof fan-in). The member-cfg
//! used here is the Merkle-opening verifier (`cfg_merkle`); swapping in the full FRI/DEEP recursive
//! cfg (same composition, heavier per-row window + its fixtures) upgrades each row to a complete
//! inner-proof verification.

use crate::inlined_perm::{self, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS};
use crate::merkle_air::{eval_node, node_blocks, ABSORB_LEN, N_PERM};
use crate::recursive_air::{
    n_cols as ra_n_cols, window_merkle_root, window_root_col, RecursiveAir, RecursiveConfig,
};
use descender_account_stark::{Val, DIGEST_ELEMS};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

/// Column where the running chain state enters this row (after the recursive_air window columns).
fn br_chain(cfg: &RecursiveConfig) -> usize {
    ra_n_cols(cfg)
}
/// The chain-fold sponge node (folds chain_i ‖ verified_root_i).
fn br_fold(cfg: &RecursiveConfig) -> usize {
    br_chain(cfg) + DIGEST_ELEMS
}
/// Total width: one recursive_air window + chain state + the fold node.
pub fn n_cols(cfg: &RecursiveConfig) -> usize {
    br_fold(cfg) + N_PERM * PERM_COLS
}
/// Offset of this row's reconstructed chain digest `chain_{i+1}`.
fn fold_digest(cfg: &RecursiveConfig) -> usize {
    br_fold(cfg) + (N_PERM - 1) * PERM_COLS + C_OUTPUT
}

pub const PV_ROOT: usize = 0;
pub const N_PUBLIC: usize = DIGEST_ELEMS;

/// Build the block trace from pre-packed `recursive_air` member windows (each `ra_n_cols(cfg)` wide,
/// e.g. from `recursive_air::pack_window`). Appends, per row, the incoming chain + the fold node that
/// folds the window's verified Merkle root into the chain. Genesis chain = 0.
pub fn build_trace(cfg: &RecursiveConfig, member_windows: &[Vec<Val>]) -> Vec<Val> {
    assert!(!member_windows.is_empty(), "need >= 1 member");
    let consts = inlined_perm::Constants::pinned();
    let ra = ra_n_cols(cfg);
    let width = n_cols(cfg);
    let mut flat = Vec::with_capacity(member_windows.len() * width);
    let mut chain = [Val::ZERO; DIGEST_ELEMS];
    for win in member_windows {
        assert_eq!(win.len(), ra, "member window width must equal ra_n_cols(cfg)");
        let root = window_merkle_root(cfg, win); // the IN-CIRCUIT-verified member root
        let mut row = win.clone();
        row.extend_from_slice(&chain); // BR_CHAIN
        let mut combined = [Val::ZERO; ABSORB_LEN];
        combined[..DIGEST_ELEMS].copy_from_slice(&chain);
        combined[DIGEST_ELEMS..].copy_from_slice(&root);
        let (blocks, chain_out) = node_blocks(&combined, &consts);
        row.extend_from_slice(&blocks); // BR_FOLD
        debug_assert_eq!(row.len(), width);
        flat.extend_from_slice(&row);
        chain = chain_out;
    }
    flat
}

/// The block `chain_state_root` (last row's chain-out).
pub fn trace_root(cfg: &RecursiveConfig, n: usize, flat: &[Val]) -> [Val; DIGEST_ELEMS] {
    let off = (n - 1) * n_cols(cfg) + fold_digest(cfg);
    core::array::from_fn(|i| flat[off + i])
}

/// True single-proof O(1) block aggregator AIR.
pub struct BlockRecursiveAir {
    pub cfg: RecursiveConfig,
}

impl<F: Sync> BaseAir<F> for BlockRecursiveAir {
    fn width(&self) -> usize {
        n_cols(&self.cfg)
    }
    fn num_public_values(&self) -> usize {
        N_PUBLIC
    }
}

struct RowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> PermCells<AB> for RowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for BlockRecursiveAir {
    fn eval(&self, builder: &mut AB) {
        let cfg = &self.cfg;
        // 1. Per-row member verification: the full recursive_air window constraints (leaf/root stay
        //    internal — its public binding is gated on pubs.len() == N_PUBLIC_MERKLE, and this AIR's
        //    public values are the single chain root). fan_out is off ⇒ no cross-row constraints.
        RecursiveAir { cfg: cfg.clone() }.eval(builder);

        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder.public_values().iter().map(|p| (*p).into()).collect();
        let l = |i: usize| -> AB::Expr { local[i].into() };

        let cells = RowCells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();

        // 2. chain fold: combined = chain_i ‖ verified_root_i (root at the window-root column).
        let wr = window_root_col(cfg);
        let chain_off = br_chain(cfg);
        let chain_in: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(chain_off + i)).collect();
        let root_e: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(wr + i)).collect();
        let combined = |g: usize| -> AB::Expr {
            if g < DIGEST_ELEMS {
                chain_in[g].clone()
            } else {
                root_e[g - DIGEST_ELEMS].clone()
            }
        };
        eval_node::<AB, _, _>(builder, &cells, br_fold(cfg), combined, &consts);
        let chain_out = fold_digest(cfg);

        // 3. genesis / chain / public root.
        {
            let mut fb = builder.when_first_row();
            for i in 0..DIGEST_ELEMS {
                fb.assert_zero(l(chain_off + i));
            }
        }
        {
            let is_tr = builder.is_transition();
            let mut tb = builder.when(is_tr);
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[chain_out + i].into();
                let next_chain: AB::Expr = next[chain_off + i].into();
                tb.assert_zero(next_chain - this_out);
            }
        }
        if pubs.len() == N_PUBLIC {
            let mut lb = builder.when_last_row();
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[chain_out + i].into();
                lb.assert_zero(this_out - pubs[PV_ROOT + i].clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenger::RATE;
    use crate::merkle::{compressor, Digest};
    use crate::recursive_air::{pack_window, WindowInputs};
    use descender_account_stark::{Challenge, SALT_ELEMS, WIDTH};
    use p3_air::check_all_constraints;
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};
    use p3_matrix::dense::RowMajorMatrix;
    use p3_symmetric::PseudoCompressionFunction;

    fn d(seed: u32) -> Digest {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1)))
    }
    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1)))
    }

    /// Reference Merkle tree (mirror of merkle_air's RefTree).
    struct RefTree {
        levels: Vec<Vec<Digest>>,
    }
    impl RefTree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                let next: Vec<Digest> = leaves.chunks_exact(2).map(|p| c.compress([p[0], p[1]])).collect();
                levels.push(next.clone());
                leaves = next;
            }
            Self { levels }
        }
        fn root(&self) -> Digest {
            self.levels.last().unwrap()[0]
        }
        fn auth_path(&self, index: usize) -> Vec<Digest> {
            let mut sibs = Vec::new();
            let mut idx = index;
            for level in &self.levels[..self.levels.len() - 1] {
                sibs.push(level[idx ^ 1]);
                idx >>= 1;
            }
            sibs
        }
    }

    fn cfg_merkle(levels: usize) -> RecursiveConfig {
        RecursiveConfig { merkle_levels: levels, ..RecursiveConfig::single_window(levels) }
    }

    /// Pack one member's recursive_air window (a Merkle opening at index `idx`) + the reference root.
    fn member_window(cfg: &RecursiveConfig, levels: usize, member: u32) -> (Vec<Val>, Digest) {
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(|x| d(x + member * 1000 + 1)).collect();
        let tree = RefTree::new(leaves.clone());
        let idx = (member as usize) % n;
        let sibs = tree.auth_path(idx);
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0: ext(1), e1: ext(2), beta: ext(3), descent: &[],
            merkle_leaf: leaves[idx], merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO,
            fs_blocks: &[], sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: &[], fs_ood_blocks: &[], fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &[], fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS], layer_opens: &[],
            final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
            dr_value_at_zeta: &[], dr_value_at_zeta_next: &[], dr_x: Val::ZERO,
            lc_value_at_zeta: &[], lc_c_committed: Challenge::ZERO,
            quot_commit: [Val::ZERO; DIGEST_ELEMS], quot_opened_at_x: &[], quot_at_zeta: &[],
            quot_zps: &[], quot_salt: [Val::ZERO; SALT_ELEMS], quot_x: Val::ZERO, quot_siblings: &[],
        };
        (pack_window(cfg, &inp), tree.root())
    }

    fn windows(cfg: &RecursiveConfig, levels: usize, n: usize) -> Vec<Vec<Val>> {
        (0..n as u32).map(|m| member_window(cfg, levels, m).0).collect()
    }

    // ---- DEEPER per-member verification: leaf-hash (b-hash) + Merkle path ----
    fn cfg_leaf_hash(levels: usize, row_len: usize) -> RecursiveConfig {
        RecursiveConfig { merkle_levels: levels, leaf_row_len: row_len, ..RecursiveConfig::single_window(levels) }
    }

    /// Pack one member window that verifies (b-hash) `leaf = MyHash(row ‖ salt)` AND the Merkle path
    /// to the root — a strictly deeper in-circuit member verification than the bare Merkle opening.
    fn member_window_leaf_hash(cfg: &RecursiveConfig, levels: usize, row_len: usize, member: u32) -> Vec<Val> {
        let n = 1usize << levels;
        let row: Vec<Val> = (0..row_len as u32).map(|i| Val::from_u32(member.wrapping_mul(7919).wrapping_add(i + 1))).collect();
        let salt: [Val; SALT_ELEMS] = core::array::from_fn(|i| Val::from_u32(member.wrapping_mul(40503).wrapping_add(i as u32 + 1)));
        let mut input = row.clone();
        input.extend_from_slice(&salt);
        let leaf = crate::leaf_hash::leaf_hash_digest(&input);
        let idx = (member as usize) % n;
        let leaves: Vec<Digest> = (0..n)
            .map(|r| if r == idx { leaf } else { d((r as u32 + 1) * 1000 + member + 1) })
            .collect();
        let tree = RefTree::new(leaves);
        let sibs = tree.auth_path(idx);
        let e0 = Challenge::from_basis_coefficients_slice(&leaf[..crate::fri_fold_air::D]).unwrap();
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0, e1: ext(2), beta: ext(3), descent: &[],
            merkle_leaf: leaf, merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO,
            fs_blocks: &[], sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: &[], fs_ood_blocks: &[], fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &[], fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS], layer_opens: &[],
            final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &row, leaf_salt: salt,
            dr_value_at_zeta: &[], dr_value_at_zeta_next: &[], dr_x: Val::ZERO,
            lc_value_at_zeta: &[], lc_c_committed: Challenge::ZERO,
            quot_commit: [Val::ZERO; DIGEST_ELEMS], quot_opened_at_x: &[], quot_at_zeta: &[],
            quot_zps: &[], quot_salt: [Val::ZERO; SALT_ELEMS], quot_x: Val::ZERO, quot_siblings: &[],
        };
        pack_window(cfg, &inp)
    }

    // ---- ULTIMATE: ALL gadgets combined per member (merkle + leaf-hash + DEEP + quotient + FS + descent) ----
    fn cfg_all(levels: usize, width: usize, combine: usize, k: usize, q: usize, pre_rounds: usize, beta_rounds: usize, final_n: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            leaf_row_len: width,
            deep_log_degree: Some(k),
            deep_e0_combine: combine,
            deep_e0_next: false,
            quot_chunks: q,
            quot_merkle_levels: levels,
            quot_alpha_offset: combine,
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            fs_beta_rounds: beta_rounds,
            fs_batch_combo_alpha: true,
            fs_final_blocks: final_n,
            num_rounds: beta_rounds,
            ..RecursiveConfig::single_window(levels)
        }
    }

    /// The COMPLETE recursive verifier per member: Merkle + leaf-hash + DEEP reduced-e0 + quotient
    /// OOD recombination/opening + the FULL Fiat-Shamir transcript (α/ζ/batch-α/β/final/query-index)
    /// with the descent folded by the SQUEEZED betas and the DEEP α/ζ BOUND to the FS squeeze — every
    /// gadget wired together and FS-consistent. (`deep_alpha`/`deep_zeta` = `fs_squeeze_alpha_zeta`, and
    /// e0 = DEEP(trace)+DEEP(quotient) under those squeezed challenges.)
    #[allow(clippy::too_many_arguments)]
    fn member_window_all(cfg: &RecursiveConfig, levels: usize, width: usize, combine: usize, q: usize, pre_rounds: usize, beta_rounds: usize, final_n: usize, member: u32) -> Vec<Val> {
        use crate::fri_fold_air::limbs;
        use p3_field::PrimeField32;
        let n = 1usize << levels;
        let x = Val::from_u32(0x1234 + member);
        let fs_blocks: Vec<[Val; RATE]> = (0..(pre_rounds + 1) as u32).map(|r| fs_block(member * 50 + r)).collect();
        let beta_blocks: Vec<Digest> = (0..beta_rounds as u32).map(|r| d(member * 60 + r + 1)).collect();
        let finals: Vec<[Val; RATE]> = (0..final_n as u32).map(|r| fq_block(member * 80 + r)).collect();
        // FS-squeezed DEEP alpha/zeta (bound to the transcript in-circuit).
        let (alpha, zeta) = crate::recursive_air::fs_squeeze_alpha_zeta(&fs_blocks[..pre_rounds], fs_blocks[pre_rounds]);
        // The opening index is FS-DERIVED (index-bus): compute it so the trees are opened at the
        // genuine FS query index (mirrors pack_window's draw_out for query_ordinal 0, query-PoW 0).
        let post_zeta = crate::recursive_air::fs_post_zeta_state(&fs_blocks[..pre_rounds], fs_blocks[pre_rounds]);
        let beta_start = crate::recursive_air::fs_squeeze_batch_combo_alpha(post_zeta).1;
        let (_betas, post_beta_state) = crate::fs_observe::fs_squeeze_betas_digest(beta_start, &beta_blocks);
        let p0 = crate::recursive_air::fs_squeeze_query_p0_state(post_beta_state, &finals, [Val::ZERO; RATE], 0);
        let idx = (p0[RATE - 1].as_canonical_u32() & ((1u32 << levels) - 1)) as usize;
        let vz: Vec<Challenge> = (0..combine as u32).map(|i| ext(member * 10 + i + 1)).collect();
        let at_zeta: Vec<Challenge> = (0..q as u32).map(|c| ext(member * 20 + c + 1)).collect();
        let zps: Vec<Challenge> = (0..q as u32).map(|c| ext(member * 30 + c + 1)).collect();
        let deep_q = crate::recursive_air::quotient_recombine(&zps, &at_zeta);
        // trace tree.
        let tsalt: [Val; SALT_ELEMS] = core::array::from_fn(|i| Val::from_u32(member.wrapping_mul(101).wrapping_add(i as u32 + 1)));
        let trow: Vec<Val> = (0..width as u32).map(|i| Val::from_u32(member.wrapping_mul(7919).wrapping_add(i + 1))).collect();
        let mut tinput = trow.clone();
        tinput.extend_from_slice(&tsalt);
        let trace_leaf = crate::leaf_hash::leaf_hash_digest(&tinput);
        let tleaves: Vec<Digest> = (0..n).map(|r| if r == idx { trace_leaf } else { d((r as u32 + 1) * 13 + member) }).collect();
        let ttree = RefTree::new(tleaves);
        let tsibs = ttree.auth_path(idx);
        // quot tree.
        let qsalt: [Val; SALT_ELEMS] = core::array::from_fn(|i| Val::from_u32(member.wrapping_mul(211).wrapping_add(i as u32 + 1)));
        let qopened: Vec<Challenge> = (0..q as u32).map(|c| ext(member * 40 + c + 1)).collect();
        let mut qflat: Vec<Val> = Vec::with_capacity(q * crate::fri_fold_air::D);
        for ch in &qopened {
            qflat.extend_from_slice(&limbs(*ch));
        }
        qflat.extend_from_slice(&qsalt);
        let quot_leaf = crate::leaf_hash::leaf_hash_digest(&qflat);
        let qleaves: Vec<Digest> = (0..n).map(|r| if r == idx { quot_leaf } else { d((r as u32 + 1) * 17 + member) }).collect();
        let qtree = RefTree::new(qleaves);
        let qsibs = qtree.auth_path(idx);
        // e0 = DEEP(trace) + DEEP(quotient) under the SQUEEZED alpha/zeta.
        let trace_e0 = crate::recursive_air::deep_reduced_e0(alpha, zeta, x, &trow[..combine], &vz);
        let quot_e0 = crate::recursive_air::quotient_reduced_e0(alpha, zeta, x, combine, &qopened, &at_zeta);
        let descent: Vec<(Challenge, Challenge)> = (1..beta_rounds).map(|r| (ext(member * 70 + r as u32), Challenge::ZERO)).collect();
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0: trace_e0 + quot_e0, e1: ext(member + 2), beta: Challenge::ZERO, descent: &descent,
            merkle_leaf: trace_leaf, merkle_siblings: &tsibs,
            deep_zeta: zeta, deep_q, deep_alpha: alpha,
            fs_blocks: &fs_blocks, sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: &beta_blocks, fs_ood_blocks: &[], fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &finals, fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS], layer_opens: &[],
            final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &trow, leaf_salt: tsalt,
            dr_value_at_zeta: &vz, dr_value_at_zeta_next: &[], dr_x: x,
            lc_value_at_zeta: &[], lc_c_committed: Challenge::ZERO,
            quot_commit: qtree.root(), quot_opened_at_x: &qopened, quot_at_zeta: &at_zeta,
            quot_zps: &zps, quot_salt: qsalt, quot_x: x, quot_siblings: &qsibs,
        };
        pack_window(cfg, &inp)
    }

    /// CAPSTONE: a block whose per-member verification is the COMPLETE recursive verifier (all
    /// gadgets combined, FS-bound), proven in ONE O(1) proof at production q128 knobs — true
    /// single-proof O(1) on-chain attestation of N fully-verified members.
    #[test]
    fn proves_block_with_full_recursive_members_q128() {
        use descender_account_stark::{MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS};
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let levels = 3usize;
        let width = 12usize;
        let combine = 12usize;
        let k = 10usize;
        let q = 8usize;
        let pre_rounds = 1usize;
        let beta_rounds = 2usize;
        let final_n = 2usize;
        let cfg = cfg_all(levels, width, combine, k, q, pre_rounds, beta_rounds, final_n);
        let n = 4usize;
        let ws: Vec<Vec<Val>> = (0..n as u32).map(|m| member_window_all(&cfg, levels, width, combine, q, pre_rounds, beta_rounds, final_n, m)).collect();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let m = RowMajorMatrix::new(flat.clone(), n_cols(&cfg));
        assert!(check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None).is_ok(),
            "honest full-recursive-verifier block rejected");

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(MyHash::new(perm.clone()), MyCompress::new(MyHash::new(perm.clone())), 0);
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP, log_final_poly_len: 0, max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES, commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS, mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("full-recursive-verifier block proof must verify at production q128 knobs");
    }

    // ---- FS-transcript-driven FRI consumer per member: merkle + FS(alpha/zeta/beta) + descent ----
    fn cfg_fs(levels: usize, k: usize, pre_rounds: usize, beta_rounds: usize, final_n: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            num_rounds: beta_rounds,
            deep_log_degree: Some(k),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            fs_beta_rounds: beta_rounds,
            fs_batch_combo_alpha: true,
            fs_final_blocks: final_n,
            ..RecursiveConfig::single_window(levels)
        }
    }

    fn fs_block(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2246822519).wrapping_add(i as u32 + 3)))
    }
    fn fq_block(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 * 131 + 7)))
    }

    /// Pack a member window verifying merkle + the FS-transcript-driven FRI descent: the FS sponge
    /// squeezes alpha/zeta (prefix) and the per-round betas from the commitment blocks, and the
    /// descent folds with THOSE squeezed betas (Fiat-Shamir-bound), plus the Merkle path. The fold
    /// betas are not free — they come from the in-circuit FS squeeze.
    fn member_window_fs(cfg: &RecursiveConfig, levels: usize, pre_rounds: usize, beta_rounds: usize, final_n: usize, member: u32) -> Vec<Val> {
        let n = 1usize << levels;
        let idx = (member as usize) % n;
        let leaves: Vec<Digest> = (0..n as u32).map(|r| d(r + member * 1000 + 3)).collect();
        let tree = RefTree::new(leaves.clone());
        let sibs = tree.auth_path(idx);
        let fs_blocks: Vec<[Val; RATE]> = (0..(pre_rounds + 1) as u32).map(|r| fs_block(member * 50 + r)).collect();
        let beta_blocks: Vec<Digest> = (0..beta_rounds as u32).map(|r| d(member * 60 + r + 1)).collect();
        let finals: Vec<[Val; RATE]> = (0..final_n as u32).map(|r| fq_block(member * 80 + r)).collect();
        let descent: Vec<(Challenge, Challenge)> = (1..beta_rounds).map(|r| (ext(member * 70 + r as u32), Challenge::ZERO)).collect();
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0: ext(member + 1), e1: ext(member + 2), beta: Challenge::ZERO, descent: &descent,
            merkle_leaf: leaves[idx], merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: ext(member + 9), deep_alpha: Challenge::ZERO,
            fs_blocks: &fs_blocks, sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: &beta_blocks, fs_ood_blocks: &[], fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &finals, fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS], layer_opens: &[],
            final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
            dr_value_at_zeta: &[], dr_value_at_zeta_next: &[], dr_x: Val::ZERO,
            lc_value_at_zeta: &[], lc_c_committed: Challenge::ZERO,
            quot_commit: [Val::ZERO; DIGEST_ELEMS], quot_opened_at_x: &[], quot_at_zeta: &[],
            quot_zps: &[], quot_salt: [Val::ZERO; SALT_ELEMS], quot_x: Val::ZERO, quot_siblings: &[],
        };
        pack_window(cfg, &inp)
    }

    /// DEPTH (FS-transcript): a block whose per-member verification is merkle + Fiat-Shamir
    /// alpha/zeta/beta squeeze + FRI descent folded with the SQUEEZED betas, in ONE O(1) proof at
    /// production q128 knobs — the FS-bound FRI consumer (betas not free).
    #[test]
    fn proves_block_with_fs_transcript_members_q128() {
        use descender_account_stark::{MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS};
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let levels = 3usize;
        let k = 10usize;
        let pre_rounds = 1usize;
        let beta_rounds = 2usize;
        let final_n = 2usize;
        let cfg = cfg_fs(levels, k, pre_rounds, beta_rounds, final_n);
        let n = 4usize;
        let ws: Vec<Vec<Val>> = (0..n as u32).map(|m| member_window_fs(&cfg, levels, pre_rounds, beta_rounds, final_n, m)).collect();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let m = RowMajorMatrix::new(flat.clone(), n_cols(&cfg));
        assert!(check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None).is_ok(),
            "honest FS-transcript block rejected");

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(MyHash::new(perm.clone()), MyCompress::new(MyHash::new(perm.clone())), 0);
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP, log_final_poly_len: 0, max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES, commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS, mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("FS-transcript-verified-member block proof must verify at production q128 knobs");
    }

    // ---- FULL inner-proof consumer per member: DEEP + quotient + leaf-hash + Merkle ----
    fn cfg_full(levels: usize, width: usize, combine: usize, k: usize, q: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            leaf_row_len: width,
            deep_log_degree: Some(k),
            deep_e0_combine: combine,
            deep_e0_next: false,
            quot_chunks: q,
            quot_merkle_levels: levels,
            quot_alpha_offset: combine,
            ..RecursiveConfig::single_window(levels)
        }
    }

    /// Pack a member window verifying the FULL inner-proof consumer in-circuit: the DEEP reduced-e0
    /// combination (trace + quotient), the quotient OOD recombination, the quotient Merkle opening,
    /// the leaf-hash, and the trace Merkle path — i.e. each member is a complete recursive
    /// verification. Per-member trees give distinct roots.
    fn member_window_full(
        cfg: &RecursiveConfig,
        levels: usize,
        width: usize,
        combine: usize,
        q: usize,
        member: u32,
    ) -> Vec<Val> {
        use crate::fri_fold_air::limbs;
        let n = 1usize << levels;
        let idx = 0usize;
        let alpha = ext(member.wrapping_add(0x9999));
        let zeta = ext(member.wrapping_add(0x5151));
        let x = Val::from_u32(0x1234 + member);
        // shared OOD data for this member.
        let vz: Vec<Challenge> = (0..combine as u32).map(|i| ext(member * 10 + i + 1)).collect();
        let at_zeta: Vec<Challenge> = (0..q as u32).map(|c| ext(member * 20 + c + 1)).collect();
        let zps: Vec<Challenge> = (0..q as u32).map(|c| ext(member * 30 + c + 1)).collect();
        let deep_q = crate::recursive_air::quotient_recombine(&zps, &at_zeta);
        // trace tree: leaf idx = MyHash(trow ‖ tsalt).
        let tsalt: [Val; SALT_ELEMS] = core::array::from_fn(|i| Val::from_u32(member.wrapping_mul(101).wrapping_add(i as u32 + 1)));
        let trow: Vec<Val> = (0..width as u32).map(|i| Val::from_u32(member.wrapping_mul(7919).wrapping_add(i + 1))).collect();
        let mut tinput = trow.clone();
        tinput.extend_from_slice(&tsalt);
        let trace_leaf = crate::leaf_hash::leaf_hash_digest(&tinput);
        let tleaves: Vec<Digest> = (0..n).map(|r| if r == idx { trace_leaf } else { d((r as u32 + 1) * 13 + member) }).collect();
        let ttree = RefTree::new(tleaves);
        let tsibs = ttree.auth_path(idx);
        // quot tree: leaf idx = MyHash(flat(qopened) ‖ qsalt).
        let qsalt: [Val; SALT_ELEMS] = core::array::from_fn(|i| Val::from_u32(member.wrapping_mul(211).wrapping_add(i as u32 + 1)));
        let qopened: Vec<Challenge> = (0..q as u32).map(|c| ext(member * 40 + c + 1)).collect();
        let mut qflat: Vec<Val> = Vec::with_capacity(q * crate::fri_fold_air::D);
        for ch in &qopened {
            qflat.extend_from_slice(&limbs(*ch));
        }
        qflat.extend_from_slice(&qsalt);
        let quot_leaf = crate::leaf_hash::leaf_hash_digest(&qflat);
        let qleaves: Vec<Digest> = (0..n).map(|r| if r == idx { quot_leaf } else { d((r as u32 + 1) * 17 + member) }).collect();
        let qtree = RefTree::new(qleaves);
        let qsibs = qtree.auth_path(idx);
        // consistent e0 = DEEP(trace) + DEEP(quotient).
        let trace_e0 = crate::recursive_air::deep_reduced_e0(alpha, zeta, x, &trow[..combine], &vz);
        let quot_e0 = crate::recursive_air::quotient_reduced_e0(alpha, zeta, x, combine, &qopened, &at_zeta);
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0: trace_e0 + quot_e0, e1: ext(2), beta: ext(3), descent: &[],
            merkle_leaf: trace_leaf, merkle_siblings: &tsibs,
            deep_zeta: zeta, deep_q, deep_alpha: alpha,
            fs_blocks: &[], sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: &[], fs_ood_blocks: &[], fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &[], fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS], layer_opens: &[],
            final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &trow, leaf_salt: tsalt,
            dr_value_at_zeta: &vz, dr_value_at_zeta_next: &[], dr_x: x,
            lc_value_at_zeta: &[], lc_c_committed: Challenge::ZERO,
            quot_commit: qtree.root(), quot_opened_at_x: &qopened, quot_at_zeta: &at_zeta,
            quot_zps: &zps, quot_salt: qsalt, quot_x: x, quot_siblings: &qsibs,
        };
        pack_window(cfg, &inp)
    }

    /// DEPTH (top rung): a block whose per-member verification is the FULL inner-proof consumer
    /// (DEEP + quotient + leaf-hash + Merkle), proven in ONE O(1) proof at production q128 knobs.
    /// Each member is a complete recursive verification; the block proof attests them all + the chain.
    #[test]
    fn proves_block_with_full_consumer_members_q128() {
        use descender_account_stark::{MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS};
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let levels = 3usize;
        let width = 12usize;
        let combine = 12usize;
        let k = 10usize;
        let q = 8usize;
        let cfg = cfg_full(levels, width, combine, k, q);
        let n = 4usize;
        let ws: Vec<Vec<Val>> = (0..n as u32).map(|m| member_window_full(&cfg, levels, width, combine, q, m)).collect();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let m = RowMajorMatrix::new(flat.clone(), n_cols(&cfg));
        assert!(check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None).is_ok(),
            "honest full-consumer-verified block rejected");

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(MyHash::new(perm.clone()), MyCompress::new(MyHash::new(perm.clone())), 0);
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP, log_final_poly_len: 0, max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES, commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS, mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("full-consumer-verified-member block proof must verify at production q128 knobs");
    }

    // ---- DEEPER still: FRI-fold descent + leaf-hash + Merkle per member ----
    fn cfg_deep(levels: usize, row_len: usize, rounds: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            leaf_row_len: row_len,
            num_rounds: rounds,
            ..RecursiveConfig::single_window(levels)
        }
    }

    /// Pack a member window verifying the FRI commit-phase FOLD descent (`rounds` rounds, the fold
    /// relation checked in-circuit) AND leaf-hash AND the Merkle path — the deepest member
    /// verification short of the full DEEP/quotient consumer.
    fn member_window_deep(
        cfg: &RecursiveConfig,
        levels: usize,
        row_len: usize,
        rounds: usize,
        member: u32,
        descent: &[(Challenge, Challenge)],
    ) -> Vec<Val> {
        let _ = rounds;
        let n = 1usize << levels;
        let row: Vec<Val> = (0..row_len as u32).map(|i| Val::from_u32(member.wrapping_mul(7919).wrapping_add(i + 1))).collect();
        let salt: [Val; SALT_ELEMS] = core::array::from_fn(|i| Val::from_u32(member.wrapping_mul(40503).wrapping_add(i as u32 + 1)));
        let mut input = row.clone();
        input.extend_from_slice(&salt);
        let leaf = crate::leaf_hash::leaf_hash_digest(&input);
        let idx = (member as usize) % n;
        let leaves: Vec<Digest> = (0..n)
            .map(|r| if r == idx { leaf } else { d((r as u32 + 1) * 1000 + member + 7) })
            .collect();
        let tree = RefTree::new(leaves);
        let sibs = tree.auth_path(idx);
        let e0 = Challenge::from_basis_coefficients_slice(&leaf[..crate::fri_fold_air::D]).unwrap();
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0, e1: ext(member + 11), beta: ext(member + 23), descent,
            merkle_leaf: leaf, merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO,
            fs_blocks: &[], sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: &[], fs_ood_blocks: &[], fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &[], fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS], layer_opens: &[],
            final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &row, leaf_salt: salt,
            dr_value_at_zeta: &[], dr_value_at_zeta_next: &[], dr_x: Val::ZERO,
            lc_value_at_zeta: &[], lc_c_committed: Challenge::ZERO,
            quot_commit: [Val::ZERO; DIGEST_ELEMS], quot_opened_at_x: &[], quot_at_zeta: &[],
            quot_zps: &[], quot_salt: [Val::ZERO; SALT_ELEMS], quot_x: Val::ZERO, quot_siblings: &[],
        };
        pack_window(cfg, &inp)
    }

    /// DEPTH (FRI fold): a block whose per-member verification is FRI-descent + leaf-hash + Merkle,
    /// proven in ONE O(1) proof at production q128 knobs. Top rung short of the DEEP/quotient consumer.
    #[test]
    fn proves_block_with_fri_descent_verified_members_q128() {
        use descender_account_stark::{MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS};
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let levels = 3usize;
        let row_len = 12usize;
        let rounds = 4usize;
        let cfg = cfg_deep(levels, row_len, rounds);
        let n = 4usize;
        let ws: Vec<Vec<Val>> = (0..n as u32)
            .map(|m| {
                let descent: Vec<(Challenge, Challenge)> =
                    (1..rounds).map(|r| (ext(m * 100 + r as u32), ext(m * 100 + r as u32 + 50))).collect();
                member_window_deep(&cfg, levels, row_len, rounds, m, &descent)
            })
            .collect();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let m = RowMajorMatrix::new(flat.clone(), n_cols(&cfg));
        assert!(check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None).is_ok(),
            "honest FRI-descent-verified block rejected");

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(MyHash::new(perm.clone()), MyCompress::new(MyHash::new(perm.clone())), 0);
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP, log_final_poly_len: 0, max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES, commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS, mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("FRI-descent-verified-member block proof must verify at production q128 knobs");
    }

    /// DEPTH: a block whose per-member verification is leaf-hash + Merkle (each member's leaf is
    /// CONSTRAINED == MyHash(row ‖ salt) in-circuit, then the path folds to the root) folds to one
    /// chain_state_root and PROVES + VERIFIES at production q128 FRI knobs — strictly deeper than the
    /// bare-Merkle block, on the same single-proof O(1) aggregator.
    #[test]
    fn proves_block_with_leaf_hash_verified_members_q128() {
        use descender_account_stark::{MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS};
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let levels = 3usize;
        let row_len = 12usize;
        let cfg = cfg_leaf_hash(levels, row_len);
        let n = 4usize;
        let ws: Vec<Vec<Val>> = (0..n as u32).map(|m| member_window_leaf_hash(&cfg, levels, row_len, m)).collect();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);

        // honest accept (constraint check).
        let m = RowMajorMatrix::new(flat.clone(), n_cols(&cfg));
        assert!(check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None).is_ok(),
            "honest leaf-hash-verified block rejected");

        // prove + verify at production q128 knobs.
        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(MyHash::new(perm.clone()), MyCompress::new(MyHash::new(perm.clone())), 0);
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP, log_final_poly_len: 0, max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES, commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS, mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("leaf-hash-verified-member block proof must verify at production q128 knobs");
    }

    /// GOLDEN: a block of N in-circuit-verified members folds to chain_state_root and ACCEPTS — one
    /// AIR verifies every member's opening AND chains the verified roots.
    #[test]
    fn accepts_block_of_in_circuit_verified_members() {
        let levels = 3usize;
        let cfg = cfg_merkle(levels);
        for n in [1usize, 2, 4] {
            let ws = windows(&cfg, levels, n);
            let flat = build_trace(&cfg, &ws);
            let root = trace_root(&cfg, n, &flat);
            let m = RowMajorMatrix::new(flat, n_cols(&cfg));
            let report = check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None);
            assert!(report.is_ok(), "honest block of {n} rejected: {} failures", report.failures.len());
        }
    }

    /// Forgery: tampering a member's window sibling breaks that member's in-circuit verification ⇒
    /// rejected (the member is verified in-circuit, not trusted).
    #[test]
    fn rejects_tampered_member_window() {
        let levels = 3usize;
        let cfg = cfg_merkle(levels);
        let n = 4usize;
        let ws = windows(&cfg, levels, n);
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let mut bad = flat.clone();
        // tamper member row 1's first Merkle sibling cell (ML_SIBLING = DIGEST_ELEMS within its
        // recursive_air window) — breaks that member's in-AIR Merkle fold.
        bad[n_cols(&cfg) + DIGEST_ELEMS] += Val::ONE;
        let m = RowMajorMatrix::new(bad, n_cols(&cfg));
        assert!(!check_all_constraints(&BlockRecursiveAir { cfg: cfg.clone() }, &m, &root.to_vec(), None).is_ok(),
            "a tampered member window was accepted");
    }

    /// KEYSTONE: the true single-proof O(1) block aggregator PROVES + VERIFIES at production q128 FRI
    /// knobs — ONE proof attesting N in-circuit-verified members chain to chain_state_root.
    #[test]
    fn proves_at_production_q128_knobs() {
        use descender_account_stark::{
            MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS,
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
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

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

        let levels = 3usize;
        let cfg = cfg_merkle(levels);
        let n = 4usize;
        let ws = windows(&cfg, levels, n);
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("true O(1) block proof must verify at production q128 knobs");
    }
}
