//! Gadget 2 (proper) — binary Merkle path-verification as a real `Air<AB>` (Option A).
//!
//! Composes the byte-pinned inlined Poseidon2-16 round constraints ([`crate::inlined_perm`])
//! with the binary-fold wiring validated in the [`crate::merkle`] reference: one trace ROW per
//! tree level, each row constraining ONE node compression plus the index-bit placement of the
//! running digest vs. the sibling, with the running digest CHAINED level-to-level by a
//! transition constraint, the leaf bound on the first row and the reconstructed root bound on
//! the last row (both via public values).
//!
//! ## Quantum-128 node compression is a SPONGE, not a truncated permutation
//!
//! The deployed leaf node compression is now `MyCompress =
//! SpongeCompression<PaddingFreeSponge<Perm, 16, RATE=7, OUT=13>>` (a 13-element / ≈403-bit
//! digest; `research/path2_quantum128_parameterization_v1.md`): a single truncated permutation
//! cannot host a 13-element 2-to-1 node because `2·13 > WIDTH = 16`. So one node compression
//! `compress([a‖b]) = PaddingFreeSponge::hash_iter(a ‖ b)` absorbs `ABSORB_LEN = 2·13 = 26`
//! field elements over rate 7 — i.e. `N_PERM = ceil(26/7) = 4` permutations (chunk sizes
//! 7,7,7,5) — mirroring `p3-symmetric`'s `PaddingFreeSponge::hash_iter` (sponge.rs:172-204)
//! constant-for-constant:
//!   * the state starts all-zero;
//!   * each block OVERWRITES `state[0..absorb_k]` with the chunk (overwrite absorption — NOT
//!     addition), leaves the unfilled rate lanes and the whole capacity `state[absorb_k..16]`
//!     carrying the previous block's permutation output (zero for block 0), then permutes;
//!   * the squeeze is `state[0..13]` of the final block's output.
//! Each block's Poseidon2 permutation is INLINED via [`inlined_perm::eval_perm`] over its own
//! column sub-window (`C_PERM0 + k·PERM_COLS`), and the absorb/carry wiring between blocks is
//! added as ordinary degree-≤2 `AirBuilder` constraints. This row therefore holds 4 perm
//! blocks where the digest-8 truncated-permutation form held 1.
//!
//! The index-bit convention matches `p3-merkle-tree`'s `verify_batch` (`merkle.rs::fold_step`):
//! bit 0 ⇒ running digest is the LEFT child (`a`), bit 1 ⇒ the RIGHT child (`b`), and the
//! sponge input is `a ‖ b`.
//!
//! Tested against a REAL `merkle.rs::RefTree` (built with the deployed q128 sponge compression):
//! every honest leaf opening's fold trace ACCEPTS via `p3_air::check_all_constraints`, and a
//! tampered sibling / wrong index bit / broken chaining / wrong leaf / wrong root / tampered
//! interior perm cell each REJECTS — soundness by attempted forgery, never by inspection.

use crate::inlined_perm::{self, eval_perm, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS};
use descender_account_stark::{Val, DIGEST_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

// ----- sponge node-compression shape (must match config's `MyHash` rate) -----
/// Sponge rate of the leaf `MyHash = PaddingFreeSponge<Perm, 16, 7, 13>`. Capacity is
/// `WIDTH - RATE = 9` (`c/2 ≈ 139`-bit indifferentiability). The AIR's node compression is
/// pinned to this rate by the end-to-end test against the real `merkle::compressor()`.
pub const RATE: usize = 7;
const _: () = assert!(RATE < WIDTH);
/// Each child digest is `DIGEST_ELEMS` lanes; a 2-to-1 node absorbs both children.
pub const ABSORB_LEN: usize = 2 * DIGEST_ELEMS;
/// Number of permutations `PaddingFreeSponge::hash_iter` runs over `ABSORB_LEN` elements at
/// rate `RATE`: one per (full or final partial) absorbed block. For 26 over rate 7 ⇒ 4.
pub const N_PERM: usize = ABSORB_LEN.div_ceil(RATE);

// ----- column layout (one fold level per row) -----
/// The running digest entering this level (`DIGEST_ELEMS` lanes).
pub const C_RUNNING: usize = 0;
/// The sibling digest at this level.
pub const C_SIBLING: usize = C_RUNNING + DIGEST_ELEMS;
/// The index bit at this level (0 ⇒ running is the left child, 1 ⇒ right).
pub const C_INDEX_BIT: usize = C_SIBLING + DIGEST_ELEMS;
/// The first inlined Poseidon2 block of this level's sponge node compression; block `k` starts
/// at `C_PERM0 + k * PERM_COLS`.
pub const C_PERM0: usize = C_INDEX_BIT + 1;
/// Total AIR width: the fold lanes plus `N_PERM` inlined permutation blocks.
pub const N_COLS: usize = C_PERM0 + N_PERM * PERM_COLS;

/// Absolute column offset of permutation block `k`.
#[inline]
pub const fn perm_off(k: usize) -> usize {
    C_PERM0 + k * PERM_COLS
}

/// Number of input elements block `k` absorbs (full `RATE`, except a possibly-partial last block).
#[inline]
const fn absorb_count(k: usize) -> usize {
    let start = k * RATE;
    let remaining = ABSORB_LEN - start;
    if remaining < RATE {
        remaining
    } else {
        RATE
    }
}

// ======================================================================================
// Reusable sponge-mode 2-to-1 node compression (shared by this AIR AND `fri_layer_air`'s
// per-level Merkle nodes, so both mirror `SpongeCompression` over the SAME schedule).
// ======================================================================================

/// Host-side: generate the `N_PERM` inlined perm blocks that compress `combined = a ‖ b` (the two
/// ordered child digests, `ABSORB_LEN` elements) via the rate-`RATE` overwrite sponge. Returns the
/// flattened blocks (to splice into a trace row) and the node digest (`output[0..DIGEST_ELEMS]` of
/// the last block).
pub fn node_blocks(
    combined: &[Val; ABSORB_LEN],
    consts: &inlined_perm::Constants,
) -> ([Val; N_PERM * PERM_COLS], [Val; DIGEST_ELEMS]) {
    let mut out = [Val::ZERO; N_PERM * PERM_COLS];
    let mut prev = [Val::ZERO; WIDTH]; // all-zero initial state
    for k in 0..N_PERM {
        let start = k * RATE;
        let absorb = absorb_count(k);
        let mut input = [Val::ZERO; WIDTH];
        for j in 0..WIDTH {
            input[j] = if j < absorb {
                combined[start + j] // overwrite rate lane
            } else if k == 0 {
                Val::ZERO
            } else {
                prev[j] // carry previous output (unfilled rate + capacity)
            };
        }
        let cols = inlined_perm::generate_perm_cols(input, consts);
        out[k * PERM_COLS..(k + 1) * PERM_COLS].copy_from_slice(&cols);
        prev = inlined_perm::perm_output(&cols);
    }
    let digest = core::array::from_fn(|i| prev[i]);
    (out, digest)
}

/// In-AIR: constrain a sponge-mode node compression over `N_PERM` consecutive perm blocks starting
/// at absolute column `perm0_off`, whose combined input `a ‖ b` is supplied by `combined(i)` for
/// `i in 0..ABSORB_LEN`. Mirrors [`node_blocks`]: each block's rate lanes are the absorbed chunk,
/// the carry lanes are the previous block's output (zero for block 0). The node digest is the last
/// block's `output[0..DIGEST_ELEMS]`.
pub fn eval_node<AB, C, F>(
    builder: &mut AB,
    cells: &C,
    perm0_off: usize,
    combined: F,
    consts: &ConstantsExpr<AB>,
) where
    AB: AirBuilder<F = Val>,
    C: PermCells<AB>,
    F: Fn(usize) -> AB::Expr,
{
    for k in 0..N_PERM {
        let off = perm0_off + k * PERM_COLS;
        let start = k * RATE;
        let absorb = absorb_count(k);
        for j in 0..WIDTH {
            let in_lane = cells.at(off + inlined_perm::C_INPUTS + j);
            let expected = if j < absorb {
                combined(start + j)
            } else if k == 0 {
                AB::Expr::ZERO
            } else {
                cells.at(perm0_off + (k - 1) * PERM_COLS + C_OUTPUT + j)
            };
            builder.assert_zero(in_lane - expected);
        }
        eval_perm::<AB, _>(builder, cells, off, consts);
    }
}

/// Build one fold-level row: place running/sibling per `index_bit` into the sponge input
/// `a ‖ b`, run the `N_PERM`-block sponge, record every perm block's cells. Mirrors
/// `merkle.rs::fold_step` for the input placement and `PaddingFreeSponge::hash_iter` for the
/// overwrite/carry sponge schedule.
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

    // Sponge input is the concatenation a ‖ b, where (a, b) = (running, sibling) when bit 0,
    // else (sibling, running) — exactly `fold_step` / `compress([a, b])`.
    let mut combined = [Val::ZERO; ABSORB_LEN];
    let (a, b) = if index_bit { (sibling, running) } else { (running, sibling) };
    combined[..DIGEST_ELEMS].copy_from_slice(&a);
    combined[DIGEST_ELEMS..].copy_from_slice(&b);

    // Run the overwrite sponge over `combined`, recording each block's full perm trace.
    let (blocks, _digest) = node_blocks(&combined, consts);
    row[C_PERM0..C_PERM0 + N_PERM * PERM_COLS].copy_from_slice(&blocks);
    row
}

/// The reconstructed digest of a level row: the first `DIGEST_ELEMS` squeeze lanes of the LAST
/// sponge block's output.
pub fn level_output(row: &[Val; N_COLS]) -> [Val; DIGEST_ELEMS] {
    let off = perm_off(N_PERM - 1) + C_OUTPUT;
    let mut out = [Val::ZERO; DIGEST_ELEMS];
    out.copy_from_slice(&row[off..off + DIGEST_ELEMS]);
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

/// Reads the current-row's perm sub-block at absolute index (block `k` starts at `perm_off(k)`,
/// so [`eval_perm`] is called with that offset).
struct RowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> PermCells<AB> for RowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

/// `combined[i]` of the sponge input `a ‖ b` as an expression of running/sibling/bit:
/// first `DIGEST_ELEMS` lanes are child `a = (1-bit)·running + bit·sibling`, the next
/// `DIGEST_ELEMS` are child `b = (1-bit)·sibling + bit·running`.
fn combined_expr<AB: AirBuilder<F = Val>>(
    i: usize,
    run: &[AB::Expr],
    sib: &[AB::Expr],
    bit: &AB::Expr,
) -> AB::Expr {
    let one = AB::Expr::ONE;
    if i < DIGEST_ELEMS {
        (one - bit.clone()) * run[i].clone() + bit.clone() * sib[i].clone()
    } else {
        let k = i - DIGEST_ELEMS;
        (one - bit.clone()) * sib[k].clone() + bit.clone() * run[k].clone()
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

        // running / sibling as expression vectors (for the combined-input wiring).
        let run: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(C_RUNNING + i)).collect();
        let sib: Vec<AB::Expr> = (0..DIGEST_ELEMS).map(|i| l(C_SIBLING + i)).collect();

        // --- sponge node compression: absorb/carry wiring + inlined perm over all N_PERM blocks ---
        // Combined input is a ‖ b ordered by the index bit; `eval_node` mirrors `node_blocks`.
        let cells = RowCells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();
        let (run_c, sib_c, bit_c) = (run.clone(), sib.clone(), bit.clone());
        eval_node::<AB, _, _>(
            builder,
            &cells,
            C_PERM0,
            |i| combined_expr::<AB>(i, &run_c, &sib_c, &bit_c),
            &consts,
        );

        // The node digest = squeeze of the last block (first DIGEST_ELEMS output lanes).
        let last_out = perm_off(N_PERM - 1) + C_OUTPUT;

        // --- chaining: next.running == this.node_digest ---
        {
            let is_tr = builder.is_transition();
            let mut tb = builder.when(is_tr);
            for i in 0..DIGEST_ELEMS {
                let this_out: AB::Expr = local[last_out + i].into();
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
                    let this_out: AB::Expr = local[last_out + i].into();
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

    /// A reference binary Merkle tree built with the deployed q128 sponge compression (mirror of
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

    /// The AIR's per-row node compression must equal the REAL q128 sponge `compressor()` — this
    /// pins the 4-block overwrite-sponge schedule (rate 7, carry) to `PaddingFreeSponge`.
    #[test]
    fn air_node_matches_real_sponge_compression() {
        let c = inlined_perm::Constants::pinned();
        let comp = compressor();
        let running = digest(7);
        let sibling = digest(99);
        for &bit in &[false, true] {
            let row = pack_level(running, sibling, bit, &c);
            let got = level_output(&row);
            let (a, b) = if bit { (sibling, running) } else { (running, sibling) };
            let want = comp.compress([a, b]);
            assert_eq!(got, want, "AIR sponge node (bit={bit}) != real compression");
        }
    }

    /// GOLDEN (positive): every leaf opening of a real tree folds to the root and ACCEPTS via
    /// the real `check_all_constraints` — the 4-block sponge perm constraints + absorb/carry
    /// wiring + leaf/root binding validated end-to-end as an AIR.
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
        // sponge consistently (so the perm constraints still pass) — the wrong placement must
        // still be caught by the root boundary.
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

    /// Forgery: a tampered INTERIOR perm cell (a committed round post) in a MIDDLE sponge block ⇒
    /// rejected by the inlined Poseidon2 round constraints — confirms every one of the `N_PERM`
    /// perm sub-blocks is fully constrained inside the fold AIR, not just the last output.
    #[test]
    fn air_rejects_tampered_perm_interior() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let sibs = tree.auth_path(4);
        let c = inlined_perm::Constants::pinned();
        let flat = build_trace(leaves[4], 4, &sibs, &c);
        let mut m = RowMajorMatrix::new(flat, N_COLS);
        // Corrupt a beginning-full-round post cell of level 0's SECOND sponge block (block 1).
        m.values[perm_off(1) + inlined_perm::C_BEGIN_POST] += Val::ONE;
        assert!(check_all_constraints(&MerkleFoldAir, &m, &pubs(leaves[4], root), None).is_ok() == false);
    }
}
