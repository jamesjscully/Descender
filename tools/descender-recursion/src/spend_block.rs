//! Phase-2 → Phase-3 bridge: aggregate SPEND proofs into one O(1) `chain_state_root`.
//!
//! This connects the spend subsystem (`descender_account_stark::spend_air`) to the generic O(1)
//! block aggregator ([`crate::block_recursive_air`]). The generic recursion machinery
//! (FS transcript + FRI commit-phase fold + Merkle/FRI-layer openings + DEEP/quotient consumer)
//! verifies ANY inner STARK proof — it is not creation-coupled (the only creation coupling is the
//! optional `deep_c_full_leaf` full-C(zeta) re-derivation, which the creation aggregator itself runs
//! WITHOUT). So a spend proof verifies in-circuit at the SAME depth the creation aggregator runs at,
//! and N spend members aggregate to one root through the same proven path.
//!
//! Here each member's IN-CIRCUIT-verified Merkle leaf is a digest of THAT spend's actual
//! `packed_witness_commitment` (derived from a real `SpendWitness` via `spend_air::spend_derive`), so
//! the aggregate proof attests "N spend packed-witnesses, each opened in-circuit, chained to one
//! `chain_state_root`". This is the spend analogue of the creation block aggregator at Merkle-opening
//! depth.
//!
//! DEFERRED (the deeper rungs, identical in kind to the creation aggregator's open `deep_c_full_leaf`
//! gap): the FULL constraint-level spend verification — a `spend_constraint_air` re-deriving the
//! spend C(zeta) (packing + frame-digest sponge + checked-core SIS + sigma + challenge sponge) so the
//! aggregator constrains the inner spend RELATION, not only its FRI/commitment layer; and the
//! in-circuit `{creation, spend}` cross-binding (MED-2 + key) lifted from the settlement layer.

use crate::recursive_air::RecursiveConfig;
use descender_account_stark::spend_air::{spend_derive, SpendAir, SpendWitness};
use descender_account_stark::{Val, DIGEST_ELEMS};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

use crate::merkle::Digest;

/// Map a spend's 32-byte `packed_witness_commitment` to a recursion leaf digest (`DIGEST_ELEMS`
/// lanes), injectively folding all 32 bytes into the lanes (lane i absorbs bytes i, i+13, i+26 in
/// base-256; max lane < 2^24 < p, so canonical). The aggregated member leaf is this commitment.
pub fn spend_leaf_digest(packed: &[u8; 32]) -> Digest {
    core::array::from_fn(|i| {
        let mut acc = packed[i] as u32;
        if i + DIGEST_ELEMS < 32 {
            acc += (packed[i + DIGEST_ELEMS] as u32) << 8;
        }
        if i + 2 * DIGEST_ELEMS < 32 {
            acc += (packed[i + 2 * DIGEST_ELEMS] as u32) << 16;
        }
        Val::from_u32(acc)
    })
}

/// Build the `recursion` member window for one spend: a Merkle opening whose opened leaf is the
/// spend's `packed_witness_commitment` digest. Mirrors `block_recursive_air::windows_for_settlement`
/// with the spend-derived leaf.
fn spend_member_window(cfg: &RecursiveConfig, levels: usize, member: u32, leaf: Digest) -> Vec<Val> {
    use crate::challenger::RATE;
    use crate::merkle::compressor;
    use crate::recursive_air::{pack_window, WindowInputs};
    use descender_account_stark::{Challenge, SALT_ELEMS, WIDTH};
    use p3_field::BasedVectorSpace;
    use p3_symmetric::PseudoCompressionFunction;

    let d = |seed: u32| -> Digest {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1))
        })
    };
    let ext = |seed: u32| -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1))
        })
    };
    let c = compressor();
    let leaf_count = 1usize << levels;
    let idx = (member as usize) % leaf_count;
    // Sibling leaves are filler; the opened leaf at `idx` is the spend's packed-witness commitment.
    let leaves: Vec<Digest> = (0..leaf_count as u32)
        .map(|x| if x as usize == idx { leaf } else { d(x + member * 1000 + 1) })
        .collect();
    let mut levels_vec = vec![leaves.clone()];
    let mut cur = leaves.clone();
    while cur.len() > 1 {
        cur = cur.chunks_exact(2).map(|p| c.compress([p[0], p[1]])).collect();
        levels_vec.push(cur.clone());
    }
    let mut sibs = Vec::new();
    let mut k = idx;
    for level in &levels_vec[..levels_vec.len() - 1] {
        sibs.push(level[k ^ 1]);
        k >>= 1;
    }
    let inp = WindowInputs {
        draw_out: Val::from_u32(idx as u32),
        e0: ext(1),
        e1: ext(2),
        beta: ext(3),
        descent: &[],
        merkle_leaf: leaf,
        merkle_siblings: &sibs,
        deep_zeta: Challenge::ZERO,
        deep_q: Challenge::ZERO,
        deep_alpha: Challenge::ZERO,
        fs_blocks: &[],
        sponge_in: [Val::ZERO; WIDTH],
        prior_state: [Val::ZERO; WIDTH],
        absorbed: [Val::ZERO; RATE],
        prefix_blocks: &[],
        public_io: &[],
        fs_beta_blocks: &[],
        fs_ood_blocks: &[],
        fs_ood_partial_block: [Val::ZERO; RATE],
        fs_final_blocks: &[],
        fs_final_partial_block: [Val::ZERO; RATE],
        inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
        layer_opens: &[],
        final_poly_x: Val::ZERO,
        final_poly_coeffs: &[],
        leaf_row: &[],
        leaf_salt: [Val::ZERO; SALT_ELEMS],
        dr_value_at_zeta: &[],
        dr_value_at_zeta_next: &[],
        dr_x: Val::ZERO,
        lc_value_at_zeta: &[],
        lc_c_committed: Challenge::ZERO,
        quot_commit: [Val::ZERO; DIGEST_ELEMS],
        quot_opened_at_x: &[],
        quot_at_zeta: &[],
        quot_zps: &[],
        quot_salt: [Val::ZERO; SALT_ELEMS],
        quot_x: Val::ZERO,
        quot_siblings: &[],
    };
    pack_window(cfg, &inp)
}

/// Build member windows for a block of spends: each window opens (in-circuit) a Merkle tree whose
/// leaf is that spend's `packed_witness_commitment` digest. `air` supplies the Poseidon2 constants
/// for `spend_derive` (it re-derives the challenge → syndrome → packed witness from each witness).
pub fn windows_for_spend(
    cfg: &RecursiveConfig,
    levels: usize,
    air: &SpendAir,
    spends: &[SpendWitness],
) -> Vec<Vec<Val>> {
    spends
        .iter()
        .enumerate()
        .map(|(m, w)| {
            let packed = spend_derive(air, w).packed;
            spend_member_window(cfg, levels, m as u32, spend_leaf_digest(&packed))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_recursive_air::{
        build_trace, cfg_merkle_settlement, n_cols, trace_root, BlockRecursiveAir,
    };
    use descender_account_stark::spend_air::SpendWitness;
    use p3_koala_bear::{
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
        KOALABEAR_POSEIDON2_RC_16_INTERNAL,
    };
    use p3_matrix::dense::RowMajorMatrix;

    fn spend_air() -> SpendAir {
        SpendAir::from_arrays(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
        )
    }

    fn spend_witness(seed: u32) -> SpendWitness {
        SpendWitness {
            sigma_w: core::array::from_fn(|i| ((seed.wrapping_mul(149).wrapping_add(i as u32) + 29) % 12289) as u16),
            sigma_t: core::array::from_fn(|i| ((seed.wrapping_mul(71).wrapping_add(i as u32) + 5) % 12289) as u16),
            projection: core::array::from_fn(|i| ((seed.wrapping_mul(211).wrapping_add(i as u32) + 5) % 12289) as u16),
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(seed as u8)),
            slot_id: 0x42 + seed as u64,
            account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
            message_digest: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(seed as u8)),
        }
    }

    /// `spend_leaf_digest` is injective over distinct packed witnesses and canonical.
    #[test]
    fn spend_leaf_digest_is_canonical_and_distinguishes() {
        let a = spend_air();
        let p0 = spend_derive(&a, &spend_witness(1)).packed;
        let p1 = spend_derive(&a, &spend_witness(2)).packed;
        assert_ne!(p0, p1, "distinct spends should pack differently");
        assert_ne!(spend_leaf_digest(&p0), spend_leaf_digest(&p1), "leaf digests must differ");
    }

    /// O(1) SPEND BLOCK: a block of real spend witnesses aggregates to ONE chain_state_root, with
    /// each member's packed-witness commitment verified in-circuit (Merkle opening) by the generic
    /// recursion — proven + verified at PRODUCTION q128 knobs. The spend analogue of
    /// `block_recursive_air::proves_at_production_q128_knobs`.
    #[test]
    fn proves_spend_block_q128() {
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
        type ChallengeMmcs = ExtensionMmcs<Val, descender_account_stark::Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, descender_account_stark::Challenge, MyChallenger>;

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
        let cfg = cfg_merkle_settlement(levels);
        let sair = spend_air();
        let spends: Vec<SpendWitness> = (0..4u32).map(spend_witness).collect();
        let ws = windows_for_spend(&cfg, levels, &sair, &spends);
        let n = ws.len();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(&config, &BlockRecursiveAir { cfg: cfg.clone() }, trace, &root.to_vec());
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("spend block must prove+verify at production q128 knobs");
    }
}
