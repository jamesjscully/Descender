//! Constant-width salted-MMCS row opening.
//!
//! A production main-trace opening contains 23,142 committed columns, four
//! hiding-MMCS salt columns, and a six-level binary authentication path.  Laying
//! the leaf hash horizontally would dominate the recursive verifier width.  This
//! AIR instead uses one row per rate-7 leaf-hash block, followed by one row per
//! Merkle level.  Four reusable Poseidon2 permutation blocks are allocated per
//! row: leaf rows use the first, while path rows use all four for the deployed
//! digest-13 sponge compression.
//!
//! The public statement is `root[13] || index_bits[levels] || leaf_digest[13]`.
//! Consequently this
//! proves that the supplied flattened `opened_row || salt` is authenticated at
//! the verifier-selected query index, not merely at a prover-selected leaf.
//! Composition with [`crate::pcs_opening_reduction_air`] must directly bind the
//! opened-row cells consumed as `p_at_x`; this module supplies the production-
//! width MMCS consumer for that next step.

use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_INPUTS, C_OUTPUT, PERM_COLS,
};
use crate::leaf_hash::{leaf_hash_digest, LEAF_HASH_RATE};
use crate::merkle_air::{node_blocks, ABSORB_LEN, N_PERM};
use descender_account_stark::{Val, DIGEST_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

const S_LEAF: usize = 0;
const S_PATH: usize = 1;

const fn n_sel(levels: usize) -> usize {
    levels + 2 // leaf, one selector per path level, pad
}
const fn s_pad(levels: usize) -> usize {
    S_PATH + levels
}
const fn c_leaf_last(levels: usize) -> usize {
    n_sel(levels)
}
const fn c_leaf_pos(levels: usize) -> usize {
    c_leaf_last(levels) + 1
}
const fn c_leaf_inv(levels: usize) -> usize {
    c_leaf_pos(levels) + 1
}
const fn c_state(levels: usize) -> usize {
    c_leaf_inv(levels) + 1
}
const fn c_block(levels: usize) -> usize {
    c_state(levels) + WIDTH
}
const fn c_running(levels: usize) -> usize {
    c_block(levels) + LEAF_HASH_RATE
}
const fn c_sibling(levels: usize) -> usize {
    c_running(levels) + DIGEST_ELEMS
}
const fn c_index_bit(levels: usize) -> usize {
    c_sibling(levels) + DIGEST_ELEMS
}
const fn c_perm(levels: usize) -> usize {
    c_index_bit(levels) + 1
}
pub const fn n_cols(levels: usize) -> usize {
    c_perm(levels) + N_PERM * PERM_COLS
}
pub const fn n_public(levels: usize) -> usize {
    2 * DIGEST_ELEMS + levels
}

#[derive(Clone, Copy, Debug)]
pub struct VerticalMmcsOpeningAir {
    pub input_len: usize,
    pub levels: usize,
}

impl VerticalMmcsOpeningAir {
    pub fn new(input_len: usize, levels: usize) -> Self {
        assert!(input_len > 0, "MMCS leaf input must be nonempty");
        assert!(levels > 0, "production MMCS opening needs a path");
        Self { input_len, levels }
    }

    pub fn leaf_rounds(self) -> usize {
        self.input_len.div_ceil(LEAF_HASH_RATE)
    }

    pub fn final_n(self) -> usize {
        let rem = self.input_len % LEAF_HASH_RATE;
        if rem == 0 {
            LEAF_HASH_RATE
        } else {
            rem
        }
    }

    pub fn active_rows(self) -> usize {
        self.leaf_rounds() + self.levels
    }

    pub fn trace_height(self) -> usize {
        (self.active_rows() + 1).next_power_of_two().max(2)
    }

    pub fn base_witness_bytes(self) -> usize {
        self.trace_height() * n_cols(self.levels) * core::mem::size_of::<Val>()
    }
}

impl<F: Sync> BaseAir<F> for VerticalMmcsOpeningAir {
    fn width(&self) -> usize {
        n_cols(self.levels)
    }

    fn num_public_values(&self) -> usize {
        n_public(self.levels)
    }
}

struct Cells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}

impl<AB: AirBuilder<F = Val>> PermCells<AB> for Cells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for VerticalMmcsOpeningAir {
    fn eval(&self, builder: &mut AB) {
        let levels = self.levels;
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
        let sel = |s: usize| -> AB::Expr { l(s) };
        let one = AB::Expr::ONE;

        let mut sel_sum = AB::Expr::ZERO;
        for s in 0..n_sel(levels) {
            builder.assert_bool(sel(s));
            sel_sum += sel(s);
        }
        builder.assert_one(sel_sum);
        let leaf = sel(S_LEAF);
        let leaf_last = l(c_leaf_last(levels));
        builder.assert_bool(leaf_last.clone());

        // Every row carries valid permutation columns.  Phase wiring below decides
        // which permutations are load-bearing on that row.
        let cells = Cells::<AB> { row: local.clone() };
        let constants = ConstantsExpr::<AB>::pinned();
        for k in 0..N_PERM {
            eval_perm::<AB, _>(builder, &cells, c_perm(levels) + k * PERM_COLS, &constants);
        }

        // Exact leaf-round endpoint (inverse-or-zero equality gadget).
        let last_pos = AB::Expr::from(AB::F::from_usize(self.leaf_rounds() - 1));
        let diff = l(c_leaf_pos(levels)) - last_pos;
        builder.assert_zero(leaf.clone() * diff.clone() * leaf_last.clone());
        builder.assert_zero(
            leaf.clone() * (diff * l(c_leaf_inv(levels)) - (one.clone() - leaf_last.clone())),
        );
        builder.assert_zero((one.clone() - leaf.clone()) * leaf_last.clone());

        // Leaf PaddingFreeSponge input: overwrite the filled rate prefix and
        // carry every unfilled lane from the pre-absorb state.
        let final_n = self.final_n();
        for j in 0..WIDTH {
            let perm_input = l(c_perm(levels) + C_INPUTS + j);
            let state = l(c_state(levels) + j);
            let expected = if j < LEAF_HASH_RATE {
                let block = l(c_block(levels) + j);
                if j < final_n {
                    block
                } else {
                    (one.clone() - leaf_last.clone()) * block + leaf_last.clone() * state
                }
            } else {
                state
            };
            builder.assert_zero(leaf.clone() * (perm_input - expected));
        }
        for j in final_n..LEAF_HASH_RATE {
            builder.assert_zero(leaf.clone() * leaf_last.clone() * l(c_block(levels) + j));
        }

        // Merkle path row: digest-13 sponge compression over the ordered children.
        let path_gate = (0..levels)
            .map(|k| sel(S_PATH + k))
            .fold(AB::Expr::ZERO, |a, b| a + b);
        let bit = l(c_index_bit(levels));
        builder.assert_zero(path_gate.clone() * bit.clone() * (bit.clone() - one.clone()));
        let combined = |g: usize| -> AB::Expr {
            let (running, sibling) = if g < DIGEST_ELEMS {
                (l(c_running(levels) + g), l(c_sibling(levels) + g))
            } else {
                let i = g - DIGEST_ELEMS;
                (l(c_running(levels) + i), l(c_sibling(levels) + i))
            };
            if g < DIGEST_ELEMS {
                (one.clone() - bit.clone()) * running + bit.clone() * sibling
            } else {
                (one.clone() - bit.clone()) * sibling + bit.clone() * running
            }
        };
        for k in 0..N_PERM {
            let off = c_perm(levels) + k * PERM_COLS;
            let start = k * LEAF_HASH_RATE;
            let absorb = core::cmp::min(LEAF_HASH_RATE, ABSORB_LEN - start);
            for j in 0..WIDTH {
                let expected = if j < absorb {
                    combined(start + j)
                } else if k == 0 {
                    AB::Expr::ZERO
                } else {
                    l(c_perm(levels) + (k - 1) * PERM_COLS + C_OUTPUT + j)
                };
                builder.assert_zero(path_gate.clone() * (l(off + C_INPUTS + j) - expected));
            }
        }

        // Bound each path placement bit to the verifier-supplied query-index bit.
        if pubs.len() == n_public(levels) {
            let leaf_out = c_perm(levels) + C_OUTPUT;
            for i in 0..DIGEST_ELEMS {
                builder.assert_zero(
                    leaf.clone()
                        * leaf_last.clone()
                        * (l(leaf_out + i) - pubs[DIGEST_ELEMS + levels + i].clone()),
                );
            }
            for k in 0..levels {
                builder
                    .assert_zero(sel(S_PATH + k) * (bit.clone() - pubs[DIGEST_ELEMS + k].clone()));
            }
            let last_path = sel(S_PATH + levels - 1);
            let out = c_perm(levels) + (N_PERM - 1) * PERM_COLS + C_OUTPUT;
            for i in 0..DIGEST_ELEMS {
                builder.assert_zero(last_path.clone() * (l(out + i) - pubs[i].clone()));
            }
        }

        // Deterministic phase schedule and carries.
        {
            let mut first = builder.when_first_row();
            first.assert_one(leaf.clone());
            first.assert_zero(l(c_leaf_pos(levels)));
            for i in 0..WIDTH {
                first.assert_zero(l(c_state(levels) + i));
            }
        }
        let mut tr = builder.when_transition();
        tr.assert_eq(n(S_LEAF), leaf.clone() * (one.clone() - leaf_last.clone()));
        tr.assert_eq(n(S_PATH), leaf.clone() * leaf_last.clone());
        for k in 1..levels {
            tr.assert_eq(n(S_PATH + k), sel(S_PATH + k - 1));
        }
        tr.assert_eq(
            n(s_pad(levels)),
            sel(s_pad(levels)) + sel(S_PATH + levels - 1),
        );
        tr.assert_eq(
            n(c_leaf_pos(levels)),
            leaf.clone()
                * (one.clone() - leaf_last.clone())
                * (l(c_leaf_pos(levels)) + one.clone()),
        );
        for i in 0..WIDTH {
            tr.assert_zero(
                leaf.clone()
                    * (one.clone() - leaf_last.clone())
                    * (n(c_state(levels) + i) - l(c_perm(levels) + C_OUTPUT + i)),
            );
        }
        for i in 0..DIGEST_ELEMS {
            tr.assert_zero(
                leaf.clone()
                    * leaf_last.clone()
                    * (n(c_running(levels) + i) - l(c_perm(levels) + C_OUTPUT + i)),
            );
            for k in 0..levels - 1 {
                tr.assert_zero(
                    sel(S_PATH + k)
                        * (n(c_running(levels) + i)
                            - l(c_perm(levels) + (N_PERM - 1) * PERM_COLS + C_OUTPUT + i)),
                );
            }
        }
    }
}

pub fn public_values(
    root: [Val; DIGEST_ELEMS],
    index: usize,
    levels: usize,
    leaf: [Val; DIGEST_ELEMS],
) -> Vec<Val> {
    assert!(index < (1usize << levels));
    let mut out = root.to_vec();
    out.extend((0..levels).map(|k| Val::from_bool((index >> k) & 1 == 1)));
    out.extend_from_slice(&leaf);
    out
}

pub fn build_trace(
    input: &[Val],
    index: usize,
    siblings: &[[Val; DIGEST_ELEMS]],
) -> (RowMajorMatrix<Val>, [Val; DIGEST_ELEMS]) {
    let levels = siblings.len();
    let air = VerticalMmcsOpeningAir::new(input.len(), levels);
    assert!(index < (1usize << levels));
    let width = n_cols(levels);
    let height = air.trace_height();
    let mut flat = vec![Val::ZERO; width * height];
    let constants = inlined_perm::Constants::pinned();
    let mut state = [Val::ZERO; WIDTH];
    let leaf_rounds = air.leaf_rounds();

    for r in 0..leaf_rounds {
        let row = &mut flat[r * width..(r + 1) * width];
        row[S_LEAF] = Val::ONE;
        let last = r + 1 == leaf_rounds;
        row[c_leaf_last(levels)] = Val::from_bool(last);
        row[c_leaf_pos(levels)] = Val::from_usize(r);
        let diff = Val::from_usize(r) - Val::from_usize(leaf_rounds - 1);
        row[c_leaf_inv(levels)] = if last { Val::ZERO } else { diff.inverse() };
        row[c_state(levels)..c_state(levels) + WIDTH].copy_from_slice(&state);
        let begin = r * LEAF_HASH_RATE;
        let end = core::cmp::min(begin + LEAF_HASH_RATE, input.len());
        row[c_block(levels)..c_block(levels) + end - begin].copy_from_slice(&input[begin..end]);
        let mut perm_input = state;
        perm_input[..end - begin].copy_from_slice(&input[begin..end]);
        let cols = inlined_perm::generate_perm_cols(perm_input, &constants);
        row[c_perm(levels)..c_perm(levels) + PERM_COLS].copy_from_slice(&cols);
        state = inlined_perm::perm_output(&cols);
        for k in 1..N_PERM {
            let dummy = inlined_perm::generate_perm_cols([Val::ZERO; WIDTH], &constants);
            let off = c_perm(levels) + k * PERM_COLS;
            row[off..off + PERM_COLS].copy_from_slice(&dummy);
        }
    }

    let leaf = core::array::from_fn(|i| state[i]);
    debug_assert_eq!(leaf, leaf_hash_digest(input));
    let mut running = leaf;
    for (k, &sibling) in siblings.iter().enumerate() {
        let r = leaf_rounds + k;
        let row = &mut flat[r * width..(r + 1) * width];
        row[S_PATH + k] = Val::ONE;
        row[c_running(levels)..c_running(levels) + DIGEST_ELEMS].copy_from_slice(&running);
        row[c_sibling(levels)..c_sibling(levels) + DIGEST_ELEMS].copy_from_slice(&sibling);
        let bit = (index >> k) & 1 == 1;
        row[c_index_bit(levels)] = Val::from_bool(bit);
        let mut combined = [Val::ZERO; ABSORB_LEN];
        let (left, right) = if bit {
            (sibling, running)
        } else {
            (running, sibling)
        };
        combined[..DIGEST_ELEMS].copy_from_slice(&left);
        combined[DIGEST_ELEMS..].copy_from_slice(&right);
        let (blocks, digest) = node_blocks(&combined, &constants);
        row[c_perm(levels)..c_perm(levels) + N_PERM * PERM_COLS].copy_from_slice(&blocks);
        running = digest;
    }

    for r in air.active_rows()..height {
        let row = &mut flat[r * width..(r + 1) * width];
        row[s_pad(levels)] = Val::ONE;
        for k in 0..N_PERM {
            let dummy = inlined_perm::generate_perm_cols([Val::ZERO; WIDTH], &constants);
            let off = c_perm(levels) + k * PERM_COLS;
            row[off..off + PERM_COLS].copy_from_slice(&dummy);
        }
    }

    (RowMajorMatrix::new(flat, width), running)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merkle::{compressor, Digest};
    use p3_air::check_all_constraints;
    use p3_symmetric::PseudoCompressionFunction;

    struct Tree {
        levels: Vec<Vec<Digest>>,
    }

    impl Tree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                leaves = leaves
                    .chunks_exact(2)
                    .map(|p| c.compress([p[0], p[1]]))
                    .collect();
                levels.push(leaves.clone());
            }
            Self { levels }
        }

        fn root(&self) -> Digest {
            self.levels.last().unwrap()[0]
        }

        fn path(&self, mut index: usize) -> Vec<Digest> {
            let mut out = Vec::with_capacity(self.levels.len() - 1);
            for level in &self.levels[..self.levels.len() - 1] {
                out.push(level[index ^ 1]);
                index >>= 1;
            }
            out
        }
    }

    fn digest(seed: u32) -> Digest {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(7919) + i as u32 + 1))
    }

    #[test]
    fn vertical_leaf_and_path_accept_and_reject_tampering() {
        let levels = 6usize;
        let index = 37usize;
        let input: Vec<Val> = (0..123u32).map(|i| Val::from_u32(1000 + i * 17)).collect();
        let leaf = leaf_hash_digest(&input);
        let mut leaves: Vec<Digest> = (0..1u32 << levels).map(|i| digest(200 + i)).collect();
        leaves[index] = leaf;
        let tree = Tree::new(leaves);
        let siblings = tree.path(index);
        let (trace, root) = build_trace(&input, index, &siblings);
        assert_eq!(root, tree.root());
        let air = VerticalMmcsOpeningAir::new(input.len(), levels);
        let pubs = public_values(root, index, levels, leaf);
        assert!(check_all_constraints(&air, &trace, &pubs, None).is_ok());

        for (label, row, col) in [
            ("leaf input", 0usize, c_block(levels) + 2),
            ("leaf permutation", 3, c_perm(levels) + C_OUTPUT + 1),
            ("sibling", air.leaf_rounds() + 2, c_sibling(levels) + 4),
            ("path bit", air.leaf_rounds() + 3, c_index_bit(levels)),
        ] {
            let mut bad = trace.values.clone();
            bad[row * n_cols(levels) + col] += Val::ONE;
            assert!(
                !check_all_constraints(
                    &air,
                    &RowMajorMatrix::new(bad, n_cols(levels)),
                    &pubs,
                    None
                )
                .is_ok(),
                "tampered {label} accepted"
            );
        }
        let mut bad_root = pubs;
        bad_root[0] += Val::ONE;
        assert!(!check_all_constraints(&air, &trace, &bad_root, None).is_ok());
    }

    #[test]
    fn production_trace_opening_shape_is_vertical_and_bounded() {
        let input_len =
            crate::pcs_opening_shape::TRACE_MATRIX_WIDTH_V1 + descender_account_stark::SALT_ELEMS;
        let air = VerticalMmcsOpeningAir::new(input_len, 6);
        assert_eq!(input_len, 23_150);
        assert_eq!(air.leaf_rounds(), 3_308);
        assert_eq!(air.active_rows(), 3_314);
        assert_eq!(air.trace_height(), 4_096);
        assert_eq!(n_cols(6), 717);
        assert_eq!(air.base_witness_bytes(), 11_747_328);
    }

    #[test]
    fn vertical_mmcs_opening_proves_at_production_q128_knobs() {
        use p3_uni_stark::{prove, verify};

        let levels = 4usize;
        let index = 9usize;
        let input: Vec<Val> = (0..51u32).map(|i| Val::from_u32(7000 + i * 11)).collect();
        let leaf = leaf_hash_digest(&input);
        let mut leaves: Vec<Digest> = (0..1u32 << levels).map(|i| digest(900 + i)).collect();
        leaves[index] = leaf;
        let tree = Tree::new(leaves);
        let (trace, root) = build_trace(&input, index, &tree.path(index));
        let pubs = public_values(root, index, levels, leaf_hash_digest(&input));
        let air = VerticalMmcsOpeningAir::new(input.len(), levels);
        let config = crate::settlement::prod_config();
        let proof = prove(&config, &air, trace, &pubs);
        verify(&config, &air, &proof, &pubs)
            .expect("vertical MMCS opening must prove and verify at q128 knobs");
    }
}
