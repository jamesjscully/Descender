//! Phase-3 in-circuit composition (step 1) — fold IN-CIRCUIT-VERIFIED member roots into the block
//! chain, instead of free committed members. Verifies each member's full multi-LEVEL Merkle opening.
//!
//! [`crate::block_accumulator`] folds free member commitments `D_i` into one `chain_state_root`; it
//! binds the members but does not check `D_i` came from a valid admission. This AIR closes part of
//! that gap: each row VERIFIES the member's Merkle opening in-circuit — a leaf folds up `levels`
//! siblings (the deployment Merkle depth) through the vetted q128 sponge node to reconstruct the
//! member root — AND folds THAT reconstructed root into the running chain:
//!
//!   root_i  = fold(leaf_i, sibling_i[0..levels])        (nodes 1..=levels — verify the opening)
//!   chain_0 = 0 ; chain_{i+1} = compress(chain_i, root_i)   (fold node — fold the verified root)
//!   block_root = chain_N
//!
//! One row per member, chain CHAINED row→row, genesis pinned, `block_root` bound to the public
//! value on the last row. Uniform (`num_public_values = DIGEST_ELEMS`) ⇒ PROVABLE: one O(1) proof
//! attests every chained member root was reconstructed in-circuit from its (leaf, `levels`-deep path).
//!
//! SCOPE / HONESTY: this verifies the member's MERKLE-OPENING reconstruction in-circuit at the real
//! path depth. FULL member verification additionally requires the member's FRI/DEEP/Fiat-Shamir
//! checks in-circuit — i.e. composing the FULL `recursive_air` per member under this chain (the
//! remaining PCD step). This is the verified-root→chain composition pattern, provable at production
//! knobs, for realistic Merkle depth.

use crate::inlined_perm::{self, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS};
use crate::merkle_air::{eval_node, node_blocks, ABSORB_LEN, N_PERM};
use descender_account_stark::{Val, DIGEST_ELEMS};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

const NODE_COLS: usize = N_PERM * PERM_COLS;

// ----- column layout (one verified member folded per row; `levels`-deep opening) -----
/// The member's opened leaf digest.
pub const AM_LEAF: usize = 0;
/// Sibling `k` of the member's Merkle path (`levels` of them).
pub const fn am_sibling(k: usize) -> usize {
    AM_LEAF + DIGEST_ELEMS + k * DIGEST_ELEMS
}
fn am_bits(levels: usize) -> usize {
    am_sibling(levels)
}
/// Verify node `k` (folds running_k ‖ sibling_k); `levels` of them.
fn am_verify(levels: usize, k: usize) -> usize {
    am_bits(levels) + levels + k * NODE_COLS
}
/// The running chain state entering this row (`chain_i`).
fn am_chain(levels: usize) -> usize {
    am_verify(levels, levels)
}
/// The fold node (folds chain_i ‖ root_i).
fn am_fold(levels: usize) -> usize {
    am_chain(levels) + DIGEST_ELEMS
}
/// Total AIR width for a `levels`-deep member opening.
pub fn n_cols(levels: usize) -> usize {
    am_fold(levels) + NODE_COLS
}

/// Offset of verify-node `k`'s reconstructed digest (last sponge block output).
fn verify_node_digest(levels: usize, k: usize) -> usize {
    am_verify(levels, k) + (N_PERM - 1) * PERM_COLS + C_OUTPUT
}
/// Offset of the reconstructed chain digest `chain_{i+1}`.
fn chain_out_off(levels: usize) -> usize {
    am_fold(levels) + (N_PERM - 1) * PERM_COLS + C_OUTPUT
}

/// Public values: the block's final `chain_state_root`.
pub const PV_ROOT: usize = 0;
pub const N_PUBLIC: usize = PV_ROOT + DIGEST_ELEMS;

/// A member: opened `leaf`, a `levels`-deep bottom-to-top sibling path, and its `index`.
pub struct Member {
    pub leaf: [Val; DIGEST_ELEMS],
    pub siblings: Vec<[Val; DIGEST_ELEMS]>,
    pub index: usize,
}

fn node_combined(running: &[Val; DIGEST_ELEMS], sib: &[Val; DIGEST_ELEMS], bit: bool) -> [Val; ABSORB_LEN] {
    let (a, b) = if bit { (sib, running) } else { (running, sib) };
    let mut c = [Val::ZERO; ABSORB_LEN];
    c[..DIGEST_ELEMS].copy_from_slice(a);
    c[DIGEST_ELEMS..].copy_from_slice(b);
    c
}

/// Reconstruct a member root from its leaf + `levels`-deep path (the in-circuit verification target).
pub fn member_root(m: &Member, consts: &inlined_perm::Constants) -> [Val; DIGEST_ELEMS] {
    let mut running = m.leaf;
    let mut idx = m.index;
    for sib in &m.siblings {
        let (_b, digest) = node_blocks(&node_combined(&running, sib, idx & 1 == 1), consts);
        running = digest;
        idx >>= 1;
    }
    running
}

/// Host: fold a block of member openings into `chain_state_root` (genesis 0).
pub fn accumulate(members: &[Member]) -> [Val; DIGEST_ELEMS] {
    let consts = inlined_perm::Constants::pinned();
    let mut chain = [Val::ZERO; DIGEST_ELEMS];
    for m in members {
        let root = member_root(m, &consts);
        let mut c = [Val::ZERO; ABSORB_LEN];
        c[..DIGEST_ELEMS].copy_from_slice(&chain);
        c[DIGEST_ELEMS..].copy_from_slice(&root);
        let (_b, chain_out) = node_blocks(&c, &consts);
        chain = chain_out;
    }
    chain
}

/// Build the trace (one row per member; genesis = 0). All members must have `levels` siblings.
pub fn build_trace(levels: usize, members: &[Member]) -> Vec<Val> {
    assert!(!members.is_empty(), "need >= 1 member");
    let consts = inlined_perm::Constants::pinned();
    let ncols = n_cols(levels);
    let mut flat = Vec::with_capacity(members.len() * ncols);
    let mut chain = [Val::ZERO; DIGEST_ELEMS];
    for m in members {
        assert_eq!(m.siblings.len(), levels, "member sibling count must equal levels");
        let mut row = vec![Val::ZERO; ncols];
        row[AM_LEAF..AM_LEAF + DIGEST_ELEMS].copy_from_slice(&m.leaf);
        // verify nodes: fold leaf up the path.
        let mut running = m.leaf;
        let mut idx = m.index;
        for (k, sib) in m.siblings.iter().enumerate() {
            row[am_sibling(k)..am_sibling(k) + DIGEST_ELEMS].copy_from_slice(sib);
            let bit = idx & 1 == 1;
            row[am_bits(levels) + k] = if bit { Val::ONE } else { Val::ZERO };
            let (blocks, digest) = node_blocks(&node_combined(&running, sib, bit), &consts);
            let off = am_verify(levels, k);
            row[off..off + NODE_COLS].copy_from_slice(&blocks);
            running = digest;
            idx >>= 1;
        }
        let root = running;
        // fold node: chain ‖ root.
        row[am_chain(levels)..am_chain(levels) + DIGEST_ELEMS].copy_from_slice(&chain);
        let mut c = [Val::ZERO; ABSORB_LEN];
        c[..DIGEST_ELEMS].copy_from_slice(&chain);
        c[DIGEST_ELEMS..].copy_from_slice(&root);
        let (fblocks, chain_out) = node_blocks(&c, &consts);
        row[am_fold(levels)..am_fold(levels) + NODE_COLS].copy_from_slice(&fblocks);
        flat.extend_from_slice(&row);
        chain = chain_out;
    }
    flat
}

/// The block `chain_state_root` of a packed trace (last row's chain-out).
pub fn trace_root(levels: usize, n: usize, flat: &[Val]) -> [Val; DIGEST_ELEMS] {
    let off = (n - 1) * n_cols(levels) + chain_out_off(levels);
    core::array::from_fn(|i| flat[off + i])
}

/// In-circuit verified-member accumulator AIR (`levels`-deep member openings).
pub struct AccumulatedMemberAir {
    pub levels: usize,
}

impl<F: Sync> BaseAir<F> for AccumulatedMemberAir {
    fn width(&self) -> usize {
        n_cols(self.levels)
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

impl<AB: AirBuilder<F = Val>> Air<AB> for AccumulatedMemberAir {
    fn eval(&self, builder: &mut AB) {
        let levels = self.levels;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder.public_values().iter().map(|p| (*p).into()).collect();
        let one = AB::Expr::ONE;
        let l = |i: usize| -> AB::Expr { local[i].into() };

        let cells = RowCells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();

        // verify nodes: running_0 = leaf; running_{k+1} = compress(running_k, sibling_k) by bit_k.
        for k in 0..levels {
            let bit = l(am_bits(levels) + k);
            builder.assert_zero(bit.clone() * (bit.clone() - one.clone()));
            let run_off = if k == 0 { AM_LEAF } else { verify_node_digest(levels, k - 1) };
            let run: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(run_off + i)).collect();
            let sib: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(am_sibling(k) + i)).collect();
            let bc = bit.clone();
            let combined = |g: usize| -> AB::Expr {
                if g < DIGEST_ELEMS {
                    (one.clone() - bc.clone()) * run[g].clone() + bc.clone() * sib[g].clone()
                } else {
                    let j = g - DIGEST_ELEMS;
                    (one.clone() - bc.clone()) * sib[j].clone() + bc.clone() * run[j].clone()
                }
            };
            eval_node::<AB, _, _>(builder, &cells, am_verify(levels, k), combined, &consts);
        }
        let root_off = verify_node_digest(levels, levels - 1);

        // fold node: chain_i ‖ root_i (root_i is the VERIFIED reconstruction, not a free input).
        let chain_in: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(am_chain(levels) + i)).collect();
        let root_e: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(root_off + i)).collect();
        let combined_f = |g: usize| -> AB::Expr {
            if g < DIGEST_ELEMS {
                chain_in[g].clone()
            } else {
                root_e[g - DIGEST_ELEMS].clone()
            }
        };
        eval_node::<AB, _, _>(builder, &cells, am_fold(levels), combined_f, &consts);
        let chain_out = chain_out_off(levels);

        // genesis: first row's chain_in == 0.
        {
            let mut fb = builder.when_first_row();
            for i in 0..DIGEST_ELEMS {
                fb.assert_zero(l(am_chain(levels) + i));
            }
        }
        // chain: next row's chain_in == this row's chain_out.
        {
            let is_tr = builder.is_transition();
            let mut tb = builder.when(is_tr);
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[chain_out + i].into();
                let next_chain: AB::Expr = next[am_chain(levels) + i].into();
                tb.assert_zero(next_chain - this_out);
            }
        }
        // last row's chain_out == public block chain_state_root.
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
    use crate::merkle::{compressor, Digest};
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_matrix::dense::RowMajorMatrix;
    use p3_symmetric::PseudoCompressionFunction;

    fn d(seed: u32) -> [Val; DIGEST_ELEMS] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1)))
    }

    /// A real reference Merkle tree (mirror of merkle_air's RefTree) — the authority for member roots.
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

    /// A block of `n` members, each a real `levels`-deep opening into its own tree.
    fn block(levels: usize, n: usize) -> (Vec<Member>, Vec<Digest>) {
        let tree_n = 1usize << levels;
        let mut members = Vec::new();
        let mut roots = Vec::new();
        for mi in 0..n as u32 {
            let leaves: Vec<Digest> = (0..tree_n as u32).map(|x| d(x + mi * 1000 + 1)).collect();
            let tree = RefTree::new(leaves.clone());
            let index = (mi as usize) % tree_n;
            members.push(Member { leaf: leaves[index], siblings: tree.auth_path(index), index });
            roots.push(tree.root());
        }
        (members, roots)
    }

    fn pubs(root: [Val; DIGEST_ELEMS]) -> Vec<Val> {
        root.to_vec()
    }

    /// The in-circuit member root reconstruction equals the REAL reference tree root.
    #[test]
    fn member_root_matches_real_tree() {
        let consts = inlined_perm::Constants::pinned();
        for levels in [1usize, 3, 4] {
            let (members, roots) = block(levels, 3);
            for (m, r) in members.iter().zip(roots.iter()) {
                assert_eq!(&member_root(m, &consts), r, "in-circuit member root != real tree root (levels={levels})");
            }
        }
    }

    /// GOLDEN: a block of multi-level-verified members folds to chain_state_root and ACCEPTS.
    #[test]
    fn accepts_honest_verified_block() {
        for levels in [1usize, 3, 4] {
            for n in [1usize, 2, 4] {
                let (members, _) = block(levels, n);
                let flat = build_trace(levels, &members);
                let root = trace_root(levels, n, &flat);
                assert_eq!(root, accumulate(&members));
                let m = RowMajorMatrix::new(flat, n_cols(levels));
                let r = check_all_constraints(&AccumulatedMemberAir { levels }, &m, &pubs(root), None);
                assert!(r.is_ok(), "honest block (levels={levels}, n={n}) rejected: {} failures", r.failures.len());
            }
        }
    }

    /// Forgery: tampering a member's sibling changes its VERIFIED root ⇒ chained root mismatch ⇒ reject.
    #[test]
    fn rejects_tampered_member_path() {
        let levels = 4usize;
        let n = 4usize;
        let (members, _) = block(levels, n);
        let flat = build_trace(levels, &members);
        let root = trace_root(levels, n, &flat);
        let mut bad = flat.clone();
        bad[n_cols(levels) + am_sibling(2)] += Val::ONE; // row 1, sibling level 2
        let m = RowMajorMatrix::new(bad, n_cols(levels));
        assert!(!check_all_constraints(&AccumulatedMemberAir { levels }, &m, &pubs(root), None).is_ok(),
            "a tampered member path was accepted");
    }

    /// Forgery: breaking the chain ⇒ reject.
    #[test]
    fn rejects_broken_chain() {
        let levels = 3usize;
        let n = 4usize;
        let (members, _) = block(levels, n);
        let flat = build_trace(levels, &members);
        let root = trace_root(levels, n, &flat);
        let mut bad = flat.clone();
        bad[2 * n_cols(levels) + am_chain(levels)] += Val::ONE;
        let m = RowMajorMatrix::new(bad, n_cols(levels));
        assert!(!check_all_constraints(&AccumulatedMemberAir { levels }, &m, &pubs(root), None).is_ok(),
            "a broken chain was accepted");
    }

    /// KEYSTONE: the multi-level in-circuit verified-member accumulator PROVES + VERIFIES at
    /// production q128 FRI knobs — one O(1) proof attesting every chained root was reconstructed
    /// in-circuit from its leaf + `levels`-deep Merkle path.
    #[test]
    fn proves_at_production_q128_knobs() {
        use descender_account_stark::{
            Challenge, MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS,
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

        let levels = 4usize; // realistic Merkle depth
        let n = 8usize;
        let (members, _) = block(levels, n);
        let flat = build_trace(levels, &members);
        let root = trace_root(levels, n, &flat);
        let trace = RowMajorMatrix::new(flat, n_cols(levels));
        let pv = pubs(root);
        let proof = prove(&config, &AccumulatedMemberAir { levels }, trace, &pv);
        verify(&config, &AccumulatedMemberAir { levels }, &proof, &pv)
            .expect("multi-level in-circuit verified-member accumulator must verify at production q128 knobs");
    }
}
