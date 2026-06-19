//! Gadget 2 (proper) — binary Merkle path-verification as a real `Air<AB>` (Option A).
//!
//! Composes the byte-pinned inlined Poseidon2-16 round constraints ([`crate::inlined_perm`])
//! with the binary-fold wiring validated in the [`crate::merkle`] reference: one trace ROW per
//! tree level, each row constraining ONE node compression `compress([a‖b]) =
//! perm([a‖b])[..8]` plus the index-bit placement of the running digest vs. the sibling, with
//! the running digest CHAINED level-to-level by a transition constraint, the leaf bound on the
//! first row and the reconstructed root bound on the last row (both via public values).
//!
//! This is the monolithic-AIR shape (Option A): the permutation constraints are INLINED via
//! [`inlined_perm::eval_perm`] over this AIR's own column layout (offset = the perm block's
//! start), and the fold wiring is added as ordinary degree-≤2 `AirBuilder` constraints on the
//! same row. The index-bit convention matches `p3-merkle-tree`'s `verify_batch`
//! (`merkle.rs::fold_step`): bit 0 ⇒ running digest is the LEFT child (`inputs[0..8]`), bit 1 ⇒
//! the RIGHT child (`inputs[8..16]`).
//!
//! Tested against a REAL `merkle.rs::RefTree` (built with the deployed compression): every
//! honest leaf opening's fold trace ACCEPTS via `p3_air::check_all_constraints`, and a tampered
//! sibling / wrong index bit / broken chaining / wrong leaf / wrong root each REJECTS — soundness
//! by attempted forgery, never by inspection.

use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS,
};
use descender_account_stark::{Val, DIGEST_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

// ----- column layout (one fold level per row) -----
/// The running digest entering this level (`DIGEST_ELEMS` lanes).
pub const C_RUNNING: usize = 0;
/// The sibling digest at this level.
pub const C_SIBLING: usize = C_RUNNING + DIGEST_ELEMS;
/// The index bit at this level (0 ⇒ running is the left child, 1 ⇒ right).
pub const C_INDEX_BIT: usize = C_SIBLING + DIGEST_ELEMS;
/// The inlined Poseidon2 compression block for this level's node.
pub const C_PERM: usize = C_INDEX_BIT + 1;
/// Total AIR width.
pub const N_COLS: usize = C_PERM + PERM_COLS;

/// Build one fold-level row: place running/sibling per `index_bit`, run the perm, record cells.
/// Mirrors `merkle.rs::fold_step` for the input placement and `inlined_perm` for the block.
pub fn pack_level(
    running: [Val; DIGEST_ELEMS],
    sibling: [Val; DIGEST_ELEMS],
    index_bit: bool,
    consts: &inlined_perm::Constants,
) -> [Val; N_COLS] {
    let mut row = [Val::ZERO; N_COLS];
    row[C_RUNNING..C_RUNNING + DIGEST_ELEMS].copy_from_slice(&running);
    row[C_SIBLING..C_SIBLING + DIGEST_ELEMS].copy_from_slice(&sibling);
    row[C_INDEX_BIT] = if index_bit { Val::ONE } else { Val::ZERO };

    // place into the 16-lane perm input exactly like fold_step / TruncatedPermutation pre.
    let mut input = [Val::ZERO; WIDTH];
    let (a, b) = if index_bit { (sibling, running) } else { (running, sibling) };
    input[..DIGEST_ELEMS].copy_from_slice(&a);
    input[DIGEST_ELEMS..].copy_from_slice(&b);

    let perm_cols = inlined_perm::generate_perm_cols(input, consts);
    row[C_PERM..C_PERM + PERM_COLS].copy_from_slice(&perm_cols);
    row
}

/// The reconstructed digest of a level row (first `DIGEST_ELEMS` lanes of the perm output).
pub fn level_output(row: &[Val; N_COLS]) -> [Val; DIGEST_ELEMS] {
    let mut out = [Val::ZERO; DIGEST_ELEMS];
    out.copy_from_slice(&row[C_PERM + C_OUTPUT..C_PERM + C_OUTPUT + DIGEST_ELEMS]);
    out
}

/// Serialize a full opening (leaf folded bottom-to-top) into the AIR trace, one row per level.
/// `index` selects the leaf; `siblings` is the bottom-to-top auth path (`merkle.rs` convention).
/// Returns the flat `Val` matrix (height = `siblings.len()`).
pub fn build_trace(
    leaf: [Val; DIGEST_ELEMS],
    index: usize,
    siblings: &[[Val; DIGEST_ELEMS]],
    consts: &inlined_perm::Constants,
) -> Vec<Val> {
    let mut flat = Vec::with_capacity(siblings.len() * N_COLS);
    let mut running = leaf;
    let mut idx = index;
    for &sib in siblings {
        let bit = idx & 1 == 1;
        let row = pack_level(running, sib, bit, consts);
        running = level_output(&row);
        flat.extend_from_slice(&row);
        idx >>= 1;
    }
    flat
}

/// Public values layout: `leaf(DIGEST_ELEMS) ‖ root(DIGEST_ELEMS)`.
pub const PV_LEAF: usize = 0;
pub const PV_ROOT: usize = PV_LEAF + DIGEST_ELEMS;
pub const N_PUBLIC: usize = PV_ROOT + DIGEST_ELEMS;

/// Binary Merkle path-verification AIR (one fold level per row).
pub struct MerkleFoldAir;

impl<F: Sync> BaseAir<F> for MerkleFoldAir {
    fn width(&self) -> usize {
        N_COLS
    }
}

/// Reads the current-row's perm sub-block at absolute index (the perm block starts at `C_PERM`,
/// so [`eval_perm`] is called with `offset = C_PERM`).
struct RowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> PermCells<AB> for RowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for MerkleFoldAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> =
            builder.public_values().iter().map(|p| (*p).into()).collect();
        let one = AB::Expr::ONE;

        let l = |i: usize| -> AB::Expr { local[i].into() };

        // --- index_bit boolean ---
        let bit = l(C_INDEX_BIT);
        builder.assert_zero(bit.clone() * (bit.clone() - one.clone()));

        // --- placement: perm.inputs[..8]/[8..] = running/sibling per index_bit (fold_step) ---
        // bit=0: a=running, b=sibling ; bit=1: a=sibling, b=running.
        for i in 0..DIGEST_ELEMS {
            let running_i = l(C_RUNNING + i);
            let sibling_i = l(C_SIBLING + i);
            let in_a = l(C_PERM + inlined_perm::C_INPUTS + i);
            let in_b = l(C_PERM + inlined_perm::C_INPUTS + DIGEST_ELEMS + i);
            // a = (1-bit)*running + bit*sibling
            builder.assert_zero(
                in_a - ((one.clone() - bit.clone()) * running_i.clone() + bit.clone() * sibling_i.clone()),
            );
            // b = (1-bit)*sibling + bit*running
            builder.assert_zero(
                in_b - ((one.clone() - bit.clone()) * sibling_i + bit.clone() * running_i),
            );
        }

        // --- inlined Poseidon2 round constraints over the perm block (offset = C_PERM) ---
        let cells = RowCells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();
        eval_perm::<AB, _>(builder, &cells, C_PERM, &consts);

        // --- chaining: next.running == this.output (first 8 perm-output lanes) ---
        {
            let is_tr = builder.is_transition();
            let mut tb = builder.when(is_tr);
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[C_PERM + C_OUTPUT + i].into();
                let next_running: AB::Expr = next[C_RUNNING + i].into();
                tb.assert_zero(next_running - this_out);
            }
        }

        // --- boundary: leaf on first row, root on last row (bound to public values) ---
        if pubs.len() == N_PUBLIC {
            {
                let mut fb = builder.when_first_row();
                for i in 0..DIGEST_ELEMS {
                    fb.assert_zero(l(C_RUNNING + i) - pubs[PV_LEAF + i].clone());
                }
            }
            {
                let mut lb = builder.when_last_row();
                for i in 0..DIGEST_ELEMS {
                    let this_out: AB::Expr = local[C_PERM + C_OUTPUT + i].into();
                    lb.assert_zero(this_out - pubs[PV_ROOT + i].clone());
                }
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

    fn digest(seed: u32) -> Digest {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32))
        })
    }

    /// A reference binary Merkle tree built with the deployed compression (mirror of
    /// `merkle.rs::tests::RefTree`) — the authority the fold AIR is checked against.
    struct RefTree {
        levels: Vec<Vec<Digest>>,
    }
    impl RefTree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                let next: Vec<Digest> =
                    leaves.chunks_exact(2).map(|p| c.compress([p[0], p[1]])).collect();
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

    fn pubs(leaf: Digest, root: Digest) -> Vec<Val> {
        let mut p = Vec::with_capacity(N_PUBLIC);
        p.extend_from_slice(&leaf);
        p.extend_from_slice(&root);
        p
    }

    fn trace(leaf: Digest, index: usize, sibs: &[Digest]) -> RowMajorMatrix<Val> {
        let c = inlined_perm::Constants::pinned();
        let flat = build_trace(leaf, index, sibs, &c);
        RowMajorMatrix::new(flat, N_COLS)
    }

    /// GOLDEN (positive): every leaf opening of a real tree folds to the root and ACCEPTS via
    /// the real `check_all_constraints` — the perm constraints + fold wiring + leaf/root binding
    /// validated end-to-end as an AIR.
    #[test]
    fn air_accepts_every_honest_opening() {
        let n = 16usize;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        for i in 0..n {
            let sibs = tree.auth_path(i);
            let m = trace(leaves[i], i, &sibs);
            let report = check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[i], root), None);
            assert!(report.is_ok(), "honest opening for leaf {i} rejected: {} failures", report.failures.len());
        }
    }

    /// Forgery: a tampered sibling cell ⇒ the recomputed node diverges, so the chained root no
    /// longer matches the public root ⇒ rejected.
    #[test]
    fn air_rejects_tampered_sibling() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let mut sibs = tree.auth_path(3);
        sibs[0][0] += Val::ONE; // tamper the first sibling
        let m = trace(leaves[3], 3, &sibs);
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[3], root), None).is_ok() == false);
    }

    /// Forgery: flipping ONE index bit re-places the running digest on the wrong side of the
    /// compression ⇒ a different node ⇒ wrong root ⇒ rejected. (Index-bit convention is
    /// load-bearing; both children placements are exercised.)
    #[test]
    fn air_rejects_wrong_index_bit() {
        let leaves: Vec<Digest> = (0..16u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let i = 6usize;
        let sibs = tree.auth_path(i);
        let c = inlined_perm::Constants::pinned();
        // Build the trace honestly, then flip the index bit of level 0 AND recompute that level's
        // perm consistently (so the perm constraints still pass) — the wrong placement must still
        // be caught by the root boundary.
        let mut running = leaves[i];
        let mut idx = i;
        let mut flat = Vec::new();
        for (lvl, &sib) in sibs.iter().enumerate() {
            let mut bit = idx & 1 == 1;
            if lvl == 0 {
                bit = !bit; // forge the placement at level 0
            }
            let row = pack_level(running, sib, bit, &c);
            running = level_output(&row);
            flat.extend_from_slice(&row);
            idx >>= 1;
        }
        let m = RowMajorMatrix::new(flat, N_COLS);
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[i], root), None).is_ok() == false);
    }

    /// Forgery: breaking the running-digest chain (next.running != this.output) ⇒ rejected by
    /// the transition constraint. Tamper a middle level's `running` to a stale value.
    #[test]
    fn air_rejects_broken_chaining() {
        let leaves: Vec<Digest> = (0..16u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let i = 5usize;
        let sibs = tree.auth_path(i);
        let c = inlined_perm::Constants::pinned();
        let flat = build_trace(leaves[i], i, &sibs, &c);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        // Corrupt row 1's running digest (no longer == row 0's output) — breaks the chain.
        m.values[N_COLS + C_RUNNING] += Val::ONE;
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[i], root), None).is_ok() == false);
    }

    /// Forgery: a wrong leaf (public value) doesn't match the first-row running digest ⇒ rejected.
    #[test]
    fn air_rejects_wrong_leaf_binding() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let sibs = tree.auth_path(2);
        let m = trace(leaves[2], 2, &sibs);
        let mut bad_leaf = leaves[2];
        bad_leaf[0] += Val::ONE; // claim a different leaf than the trace folds
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(bad_leaf, root), None).is_ok() == false);
    }

    /// Forgery: a wrong root (public value) ⇒ rejected by the last-row boundary.
    #[test]
    fn air_rejects_wrong_root_binding() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let sibs = tree.auth_path(1);
        let m = trace(leaves[1], 1, &sibs);
        let mut bad_root = tree.root();
        bad_root[0] += Val::ONE;
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[1], bad_root), None).is_ok() == false);
    }

    /// Forgery: a tampered INTERIOR perm cell (a committed round post) ⇒ rejected by the inlined
    /// Poseidon2 round constraints — confirms the perm sub-block is fully constrained inside the
    /// fold AIR, not just at its output.
    #[test]
    fn air_rejects_tampered_perm_interior() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let sibs = tree.auth_path(4);
        let c = inlined_perm::Constants::pinned();
        let flat = build_trace(leaves[4], 4, &sibs, &c);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        // Corrupt a beginning-full-round post cell of level 0's perm block.
        m.values[C_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[4], root), None).is_ok() == false);
    }
}
