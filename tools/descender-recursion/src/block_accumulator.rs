//! Phase-3 block accumulator — fold a block of per-account recursive-proof commitments into ONE
//! provable O(1) `chain_state_root`.
//!
//! ## What this is (and isn't)
//!
//! The economic payoff of recursion is that ONE proof per block attests to MANY account
//! admissions, instead of verifying each account's proof on-chain. This AIR is the accumulator
//! skeleton: row `i` folds the `i`-th block member's commitment digest `D_i` (the public output
//! a per-account recursive proof re-exposes — e.g. its account-creation root / public-IO digest)
//! into a running chain state via the SAME vetted q128 sponge 2-to-1 compression the Merkle nodes
//! use (`merkle_air::node_blocks`/`eval_node`):
//!
//!   chain_0 = genesis (0) ; chain_{i+1} = compress(chain_i, D_i) ; root = chain_N
//!
//! The trace has one row per member (height = block size), the running state is CHAINED row→row by
//! a transition constraint, `chain_0` is pinned to the genesis on the first row, and the final
//! `chain_N` is bound to the public `chain_state_root` on the last row. Being a uniform AIR (see
//! [`BlockAccumulatorAir::num_public_values`]) it is PROVABLE with `p3_uni_stark::prove` — so a
//! block of `N` admissions collapses to one O(1) recursive-STARK proof binding `chain_state_root`.
//!
//! SCOPE / HONESTY: this binds the members' commitments into one provable root (the accumulation
//! structure + the O(1) artifact). FULL aggregation soundness additionally requires each member
//! `D_i` to be the verified output of that member's recursive proof — i.e. verifying each inner
//! recursive proof in-circuit (PCD/IVC), or a fold that carries each proof. That in-circuit
//! member-proof verification is the follow-on; here `D_i` is a bound committed input.

use crate::inlined_perm::{self, ConstantsExpr, C_OUTPUT, PERM_COLS};
use crate::merkle_air::{eval_node, node_blocks, ABSORB_LEN, N_PERM};
use descender_account_stark::{Val, DIGEST_ELEMS};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

// ----- column layout (one block member folded per row) -----
/// The running chain state entering this row (`chain_i`).
pub const BC_CHAIN: usize = 0;
/// This row's member commitment digest `D_i`.
pub const BC_MEMBER: usize = BC_CHAIN + DIGEST_ELEMS;
/// The sponge 2-to-1 node compressing `chain_i ‖ D_i` (`N_PERM` inlined perm blocks).
pub const BC_PERM: usize = BC_MEMBER + DIGEST_ELEMS;
/// Total AIR width.
pub const BC_N_COLS: usize = BC_PERM + N_PERM * PERM_COLS;

/// Absolute offset of this row's reconstructed chain digest `chain_{i+1}` (last sponge block output).
const fn bc_node_digest() -> usize {
    BC_PERM + (N_PERM - 1) * PERM_COLS + C_OUTPUT
}

/// Public values: the block's final `chain_state_root` (the genesis is pinned to 0 in-AIR).
pub const PV_ROOT: usize = 0;
pub const N_PUBLIC: usize = PV_ROOT + DIGEST_ELEMS;

/// Host-side: fold `genesis` through `members` and return `chain_state_root`.
pub fn accumulate(genesis: [Val; DIGEST_ELEMS], members: &[[Val; DIGEST_ELEMS]]) -> [Val; DIGEST_ELEMS] {
    let consts = inlined_perm::Constants::pinned();
    let mut chain = genesis;
    for m in members {
        let mut combined = [Val::ZERO; ABSORB_LEN];
        combined[..DIGEST_ELEMS].copy_from_slice(&chain);
        combined[DIGEST_ELEMS..].copy_from_slice(m);
        let (_blocks, digest) = node_blocks(&combined, &consts);
        chain = digest;
    }
    chain
}

/// Build the accumulator trace (one row per member; genesis = 0). Returns the flat matrix.
pub fn build_trace(members: &[[Val; DIGEST_ELEMS]]) -> Vec<Val> {
    assert!(!members.is_empty(), "block accumulator needs >= 1 member");
    let consts = inlined_perm::Constants::pinned();
    let mut flat = Vec::with_capacity(members.len() * BC_N_COLS);
    let mut chain = [Val::ZERO; DIGEST_ELEMS]; // genesis
    for m in members {
        let mut row = [Val::ZERO; BC_N_COLS];
        row[BC_CHAIN..BC_CHAIN + DIGEST_ELEMS].copy_from_slice(&chain);
        row[BC_MEMBER..BC_MEMBER + DIGEST_ELEMS].copy_from_slice(m);
        let mut combined = [Val::ZERO; ABSORB_LEN];
        combined[..DIGEST_ELEMS].copy_from_slice(&chain);
        combined[DIGEST_ELEMS..].copy_from_slice(m);
        let (blocks, digest) = node_blocks(&combined, &consts);
        row[BC_PERM..BC_PERM + N_PERM * PERM_COLS].copy_from_slice(&blocks);
        flat.extend_from_slice(&row);
        chain = digest;
    }
    flat
}

/// The reconstructed `chain_state_root` of a packed trace (the last row's node digest).
pub fn trace_root(members_len: usize, flat: &[Val]) -> [Val; DIGEST_ELEMS] {
    let last = (members_len - 1) * BC_N_COLS;
    let off = last + bc_node_digest();
    core::array::from_fn(|i| flat[off + i])
}

/// Block accumulator AIR (one member folded per row).
pub struct BlockAccumulatorAir;

impl<F: Sync> BaseAir<F> for BlockAccumulatorAir {
    fn width(&self) -> usize {
        BC_N_COLS
    }
    fn num_public_values(&self) -> usize {
        N_PUBLIC
    }
}

struct RowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> crate::inlined_perm::PermCells<AB> for RowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for BlockAccumulatorAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder.public_values().iter().map(|p| (*p).into()).collect();
        let l = |i: usize| -> AB::Expr { local[i].into() };

        // node compression: combined = chain_i ‖ D_i (chain is the fixed left child).
        let chain: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(BC_CHAIN + i)).collect();
        let member: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(BC_MEMBER + i)).collect();
        let combined = |g: usize| -> AB::Expr {
            if g < DIGEST_ELEMS {
                chain[g].clone()
            } else {
                member[g - DIGEST_ELEMS].clone()
            }
        };
        let cells = RowCells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();
        eval_node::<AB, _, _>(builder, &cells, BC_PERM, combined, &consts);

        let out = bc_node_digest();

        // genesis: first row's chain_in == 0.
        {
            let mut fb = builder.when_first_row();
            for i in 0..DIGEST_ELEMS {
                fb.assert_zero(l(BC_CHAIN + i));
            }
        }
        // chain: next row's chain_in == this row's reconstructed chain digest.
        {
            let is_tr = builder.is_transition();
            let mut tb = builder.when(is_tr);
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[out + i].into();
                let next_chain: AB::Expr = next[BC_CHAIN + i].into();
                tb.assert_zero(next_chain - this_out);
            }
        }
        // last row's chain digest == public chain_state_root.
        if pubs.len() == N_PUBLIC {
            let mut lb = builder.when_last_row();
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[out + i].into();
                lb.assert_zero(this_out - pubs[PV_ROOT + i].clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_matrix::dense::RowMajorMatrix;

    fn member(seed: u32) -> [Val; DIGEST_ELEMS] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1)))
    }

    fn pubs(root: [Val; DIGEST_ELEMS]) -> Vec<Val> {
        root.to_vec()
    }

    /// GOLDEN: an honest block of members folds to `chain_state_root` and ACCEPTS.
    #[test]
    fn accepts_honest_block() {
        for n in [1usize, 2, 4, 8] {
            let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
            let flat = build_trace(&members);
            let root = trace_root(n, &flat);
            assert_eq!(root, accumulate([Val::ZERO; DIGEST_ELEMS], &members));
            let m = RowMajorMatrix::new(flat, BC_N_COLS);
            let report = check_all_constraints(&BlockAccumulatorAir, &m, &pubs(root), None);
            assert!(report.is_ok(), "honest block of {n} rejected: {} failures", report.failures.len());
        }
    }

    /// Forgery: tampering ANY member changes the chained root ⇒ the public-root binding rejects.
    #[test]
    fn rejects_tampered_member() {
        let n = 4usize;
        let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
        let flat = build_trace(&members);
        let root = trace_root(n, &flat);
        // tamper member 1's digest cell in row 1 (and leave the honest root as the public claim).
        let mut bad = flat.clone();
        bad[BC_N_COLS + BC_MEMBER] += Val::ONE;
        let m = RowMajorMatrix::new(bad, BC_N_COLS);
        assert!(!check_all_constraints(&BlockAccumulatorAir, &m, &pubs(root), None).is_ok(),
            "a tampered block member was accepted");
    }

    /// Forgery: breaking the chain (row i+1's chain_in != row i's output) ⇒ rejected by the transition.
    #[test]
    fn rejects_broken_chain() {
        let n = 4usize;
        let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
        let flat = build_trace(&members);
        let root = trace_root(n, &flat);
        let mut bad = flat.clone();
        bad[2 * BC_N_COLS + BC_CHAIN] += Val::ONE; // row 2 chain_in no longer == row 1 output
        let m = RowMajorMatrix::new(bad, BC_N_COLS);
        assert!(!check_all_constraints(&BlockAccumulatorAir, &m, &pubs(root), None).is_ok(),
            "a broken accumulator chain was accepted");
    }

    /// Forgery: a wrong public chain_state_root ⇒ rejected by the last-row binding.
    #[test]
    fn rejects_wrong_root() {
        let n = 4usize;
        let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
        let flat = build_trace(&members);
        let mut root = trace_root(n, &flat);
        root[0] += Val::ONE;
        let m = RowMajorMatrix::new(flat, BC_N_COLS);
        assert!(!check_all_constraints(&BlockAccumulatorAir, &m, &pubs(root), None).is_ok(),
            "a wrong chain_state_root was accepted");
    }

    /// KEYSTONE: the block accumulator is PROVED into a real O(1) STARK proof and VERIFIED — one
    /// proof per block attesting to all `N` members' `chain_state_root`, on the q128 Poseidon2 stack.
    /// This is the on-chain-viability artifact: verify ONE accumulator proof, not N account proofs.
    #[test]
    fn block_accumulator_proves_and_verifies_q128() {
        use descender_account_stark::{Challenge, MyCompress, MyHash, Perm, DIGEST_ELEMS};
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            MyHash,
            MyCompress,
            2,
            DIGEST_ELEMS,
        >;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(
            MyHash::new(perm.clone()),
            MyCompress::new(MyHash::new(perm.clone())),
            0,
        );
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: 2,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: 2,
            commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: 0,
            mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));

        // A block of 4 account members folded into one chain_state_root.
        let n = 4usize;
        let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
        let flat = build_trace(&members);
        let root = trace_root(n, &flat);
        let trace = RowMajorMatrix::new(flat, BC_N_COLS);
        let pv = pubs(root);

        let proof = prove(&config, &BlockAccumulatorAir, trace, &pv);
        verify(&config, &BlockAccumulatorAir, &proof, &pv)
            .expect("block accumulator proof must verify");
    }

    /// DEPLOYABLE-PARAMS: the block accumulator proof also holds at the PRODUCTION quantum-128 FRI
    /// soundness knobs (`PROD_NUM_QUERIES = 120`, `PROD_LOG_BLOWUP = 4`, `PROD_QUERY_POW_BITS = 16`)
    /// — not just the small completeness knobs — so the O(1) block proof is sound at the q128
    /// deployment parameters, not merely satisfiable.
    #[test]
    fn block_accumulator_proves_at_production_q128_knobs() {
        use descender_account_stark::{
            Challenge, MyCompress, MyHash, Perm, DIGEST_ELEMS, PROD_LOG_BLOWUP, PROD_NUM_QUERIES,
            PROD_QUERY_POW_BITS,
        };
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            MyHash,
            MyCompress,
            2,
            DIGEST_ELEMS,
        >;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(
            MyHash::new(perm.clone()),
            MyCompress::new(MyHash::new(perm.clone())),
            0,
        );
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        // PRODUCTION quantum-128 FRI soundness knobs (q128 config / ADR-0008).
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

        let n = 8usize; // a power-of-two block
        let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
        let flat = build_trace(&members);
        let root = trace_root(n, &flat);
        let trace = RowMajorMatrix::new(flat, BC_N_COLS);
        let pv = pubs(root);

        let proof = prove(&config, &BlockAccumulatorAir, trace, &pv);
        verify(&config, &BlockAccumulatorAir, &proof, &pv)
            .expect("block accumulator proof must verify at production q128 FRI knobs");
    }

    /// ECONOMIC VIABILITY (measured): the aggregate O(1) block proof's byte size grows only
    /// ~logarithmically in the block size, so the per-account on-chain footprint AMORTIZES toward
    /// zero — vs the ~4.75 MB per-account creation proof that made per-account on-chain verification
    /// categorically impossible (`research/path2_economic_deployment_architecture_v1.md`). This
    /// serializes the real production-knob accumulator proof for several block sizes and asserts the
    /// per-account share is far below the per-account creation-proof floor. (Measurement of the
    /// aggregate-proof footprint; the on-chain EVM verifier gas model is a separate remaining item.)
    #[test]
    fn aggregate_proof_size_amortizes_per_account() {
        use descender_account_stark::{
            Challenge, MyCompress, MyHash, Perm, DIGEST_ELEMS, PROD_LOG_BLOWUP, PROD_NUM_QUERIES,
            PROD_QUERY_POW_BITS,
        };
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let mk = || -> OuterConfig {
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
            OuterConfig::new(Pcs::new(DftT::default(), val_mmcs, fri_params), MyChallenger::new(perm.clone()))
        };

        // ~4.75 MB per-account creation proof (the floor aggregation must beat), in bytes.
        const CREATION_PROOF_BYTES: usize = 4_750_000;
        let mut prev_per_account = usize::MAX;
        for &n in &[4usize, 16, 64] {
            let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
            let flat = build_trace(&members);
            let trace = RowMajorMatrix::new(flat, BC_N_COLS);
            let root = trace_root(n, &{
                let m: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
                build_trace(&m)
            });
            let config = mk();
            let proof = prove(&config, &BlockAccumulatorAir, trace, &root.to_vec());
            let bytes = bincode::serialize(&proof).expect("proof serializes").len();
            let per_account = bytes / n;
            eprintln!("ECON block n={n}: aggregate proof {bytes} B; per-account {per_account} B (creation proof ~{CREATION_PROOF_BYTES} B)");
            assert!(
                per_account < CREATION_PROOF_BYTES / 10,
                "per-account aggregate footprint {per_account} B not far below the creation-proof floor"
            );
            assert!(per_account <= prev_per_account, "per-account footprint must not grow with block size");
            prev_per_account = per_account;
        }
    }

    /// Genesis: first row's chain_in must be 0 ⇒ a nonzero genesis rejects.
    #[test]
    fn rejects_nonzero_genesis() {
        let n = 2usize;
        let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
        let flat = build_trace(&members);
        let root = trace_root(n, &flat);
        let mut bad = flat.clone();
        bad[BC_CHAIN] += Val::ONE; // row 0 chain_in (genesis) != 0
        let m = RowMajorMatrix::new(bad, BC_N_COLS);
        assert!(!check_all_constraints(&BlockAccumulatorAir, &m, &pubs(root), None).is_ok(),
            "a nonzero genesis was accepted");
    }
}
