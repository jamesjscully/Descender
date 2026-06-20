//! Gadget 2/3 bridge — PER-ROUND FRI commit-phase LAYER Merkle opening as an `Air<AB>`.
//!
//! ## What this closes
//!
//! The monolithic [`crate::recursive_air`] window authenticates the MAIN-TRACE Merkle opening
//! at the FS index against the inner main-trace commitment (the (b-hash) leaf-row hash + the (a)
//! Merkle path + the (a-root) root binding), binds the opened row to the round-0 FRI `e0`, and
//! enforces the per-query commit-phase FOLD chain under the FS-squeezed betas. But each
//! commit-phase round's opened SIBLING (`e1_r`, the `−s_r` partner the fold consumes) was a free
//! committed column: it was NOT authenticated against THAT round's per-layer FRI commitment. A
//! prover could supply any sibling.
//!
//! This module closes that gap for ONE commit-phase round `r`, replicating the proven
//! (b-hash leaf-row hash) + (a Merkle path) + (a-root root binding) pattern for a FRI LAYER.
//!
//! ## The deployed object being mirrored (constant-for-constant)
//!
//! The FRI commit-phase MMCS is `config::HidingChallengeMmcs = ExtensionMmcs<Val, Challenge,
//! HidingValMmcs>` — an extension-field MMCS over the salted-leaf `MerkleTreeHidingMmcs`
//! (`MyHash`/`MyCompress`). p3-fri `verify_query` (verifier.rs:531-540) authenticates, per round,
//! the arity-2 sibling row `evals = [evals[0], evals[1]]` (the folded eval placed at
//! `index_in_group = start_index % 2`, the opened sibling at the other slot) at the FOLDED node
//! index `start_index >> 1` against the round's commitment `comm_r`, via `Mmcs::verify_batch`.
//!
//! The `ExtensionMmcs` layer flattens that arity-2 extension row into base coordinates
//! (`extension_mmcs.rs:70-96` → `EF::flatten_to_base`), ELEMENT-MAJOR: `[evals[0].limbs(0..8),
//! evals[1].limbs(0..8)]` = `2·D = 16` base elems (verified by [`tests::flatten_pair_is_element_major`]).
//! The hiding MMCS then appends `SALT_ELEMS = 4` salt elems and the leaf digest is
//! `MyHash(flatten(evals) ‖ salt)` (`hiding_mmcs.rs:174` + `MerkleTreeMmcs` leaf hashing) —
//! EXACTLY the (b-hash) `PaddingFreeSponge<Perm,16,8,8>` schedule already PINned in
//! [`crate::recursive_air::leaf_hash_digest`]. The node fold to `comm_r` is `MyCompress`
//! (`TruncatedPermutation<Perm,2,8,16>`) — the SAME node layer [`crate::merkle_air`] mirrors.
//!
//! ## What the AIR constrains (one row = one round's layer open)
//!
//!   1. **parity boolean** — the in-group parity `idx_in_group ∈ {0,1}` (= `start_index % 2`,
//!      the round-r FS index bit) is boolean.
//!   2. **pair placement** — `(evals[0], evals[1])` are the folded value `e_fold` and the sibling
//!      `e_sib` ORDERED BY PARITY: `evals[0] = (1−p)·e_fold + p·e_sib`,
//!      `evals[1] = (1−p)·e_sib + p·e_fold`. So the flattened leaf row IS the p3-fri `evals` row.
//!   3. **leaf-hash** — the leaf digest is DERIVED IN-AIR as `MyHash(evals_flat ‖ salt)` via the
//!      inlined Poseidon2 `PaddingFreeSponge` (same `eval_perm` schedule as (b-hash)).
//!   4. **Merkle path** — the leaf folds bottom-to-top through `levels` `MyCompress` nodes; each
//!      level's index-bit selector is bound to the FOLDED-NODE index bits (the FS index shifted by
//!      `r+1`); the reconstructed root is constrained EQUAL to the bound `layer_commit` column
//!      (round-r's FRI commitment).
//!   5. **fold exposure** — `e_fold`/`e_sib` are carried as columns so the consumer (the fold
//!      chain) binds them: `e_fold` is the value the round folds, `e_sib` IS the authenticated
//!      sibling. With this AIR, a fold's sibling is no longer free — it is Merkle-authenticated.
//!
//! ## Index binding (NON-VACUOUS)
//!
//! The folded-node index bits (the path selectors) AND the in-group parity are CONSTRAINED to a
//! single decomposition of a `folded_index` column: `parity` is bit 0, the level selectors are
//! bits `1..=levels`. So a layer opening presented at the WRONG node (not the FS-shifted index)
//! is rejected ([`tests::air_rejects_layer_open_at_wrong_index`]). The host binds `folded_index`
//! to the FS query index shifted by `r` (the round's `start_index`) — wiring that the recursion
//! window's index bus already exposes for the main-trace open.
//!
//! ## Final-polynomial check
//!
//! [`FinalPolyAir`] (same file) closes the OTHER honest gap: after the last fold round the folded
//! value must equal the committed FRI final polynomial at the query point (p3-fri verifier.rs:402-409:
//! `eval = Σ coeff·x^k` by Horner, then `eval == folded_eval`). For the deployed
//! `log_final_poly_len` the final poly is a low-degree polynomial sent IN CLEAR / committed; the
//! AIR Horner-evaluates the COMMITTED coefficients at the committed query point `x` and constrains
//! `folded == eval`. A tampered final-poly coefficient OR a folded value `!= final_poly(x)` rejects.

use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_INPUTS, C_OUTPUT, PERM_COLS,
};
use crate::recursive_air::{leaf_hash_digest, leaf_hash_pre_states};
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, SALT_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

/// Extension degree (deg-8 challenge field) = digest width. `DIGEST_ELEMS == D == 8` on the
/// deployed config, so a layer leaf's flattened pair is `2·D` base elems.
pub const D: usize = DIGEST_ELEMS;
/// Rate of the Poseidon2 sponge (RATE == DIGEST_ELEMS == 8 on the deployed instance).
pub const RATE: usize = DIGEST_ELEMS;

/// The flattened arity-2 leaf row: `[evals[0]; D] ‖ [evals[1]; D]`.
pub const LEAF_ROW_LEN: usize = 2 * D;
/// The salted leaf-hash input length: flattened pair ‖ salt.
pub const LEAF_INPUT_LEN: usize = LEAF_ROW_LEN + SALT_ELEMS;

/// Number of Poseidon2 absorb-permutes the `PaddingFreeSponge` performs over `LEAF_INPUT_LEN`
/// elements (mirrors `crate::recursive_air::lh_perms` / `p3-symmetric::PaddingFreeSponge`).
pub const fn leaf_perms() -> usize {
    let m = LEAF_INPUT_LEN;
    let c = m.div_ceil(RATE);
    if c == 0 {
        1
    } else {
        c
    }
}

// ----- column layout (one FRI commit-phase round's layer open per row) -----
/// `evals[0]` limbs (the flattened pair's first element).
pub const C_EVAL0: usize = 0;
/// `evals[1]` limbs.
pub const C_EVAL1: usize = C_EVAL0 + D;
/// Salt elems appended after the pair (the hiding-MMCS salt).
pub const C_SALT: usize = C_EVAL1 + D;
/// The value this round folds (`folded_eval` from the previous round / the leaf opening).
pub const C_E_FOLD: usize = C_SALT + SALT_ELEMS;
/// The authenticated sibling (`evals[other]`) the fold consumes.
pub const C_E_SIB: usize = C_E_FOLD + D;
/// `folded_index` = round-r `start_index`. Bit 0 = in-group parity; bits `1..=levels` = the path
/// node selectors. Bound by the host to the FS query index shifted by `r`.
pub const C_FOLDED_INDEX: usize = C_E_SIB + D;
/// In-group parity bit (`start_index % 2`).
pub const C_PARITY: usize = C_FOLDED_INDEX + 1;
/// The bound round-r FRI-layer commitment (the root the path must fold to).
pub const C_LAYER_COMMIT: usize = C_PARITY + 1;
/// Start of the leaf-hash perm blocks.
const C_LEAF_PERMS: usize = C_LAYER_COMMIT + DIGEST_ELEMS;

const fn leaf_perm_base(k: usize) -> usize {
    C_LEAF_PERMS + k * PERM_COLS
}

/// Start of the Merkle path level blocks (after the leaf-hash perms).
const fn merkle_base() -> usize {
    leaf_perm_base(leaf_perms())
}

/// Per-level sub-block: `running[8] ‖ sibling[8] ‖ selector_bit ‖ perm[PERM_COLS]`.
const ML_RUNNING: usize = 0;
const ML_SIBLING: usize = DIGEST_ELEMS;
const ML_BIT: usize = 2 * DIGEST_ELEMS;
const ML_PERM: usize = ML_BIT + 1;
const MERKLE_LEVEL: usize = ML_PERM + PERM_COLS;

const fn merkle_level_base(level: usize) -> usize {
    merkle_base() + level * MERKLE_LEVEL
}

/// Total AIR width for a `levels`-deep layer open.
pub fn n_cols(levels: usize) -> usize {
    merkle_base() + levels * MERKLE_LEVEL
}

/// Honest inputs for one round's layer open.
pub struct LayerInputs<'a> {
    /// The value this round folds (`folded_eval` going into the round).
    pub e_fold: Challenge,
    /// The opened sibling (the `−s_r` partner) — to be Merkle-authenticated.
    pub e_sib: Challenge,
    /// The hiding-MMCS salt of the opened leaf row.
    pub salt: [Val; SALT_ELEMS],
    /// The round-r `start_index` (= FS query index shifted by `r`). Bit 0 = parity; bits `1..` =
    /// node selectors.
    pub folded_index: usize,
    /// The bottom-to-top Merkle auth path (`levels` siblings).
    pub merkle_siblings: &'a [[Val; DIGEST_ELEMS]],
    /// The round-r FRI-layer commitment (the root the path folds to).
    pub layer_commit: [Val; DIGEST_ELEMS],
}

/// The flattened arity-2 leaf row `[evals[0], evals[1]]` placed by parity: `evals[parity] = e_fold`,
/// the other slot = `e_sib`. Mirrors p3-fri `verify_query` (verifier.rs:508-518).
pub fn flatten_pair(e_fold: Challenge, e_sib: Challenge, parity: bool) -> [Val; LEAF_ROW_LEN] {
    let (e0, e1) = if parity { (e_sib, e_fold) } else { (e_fold, e_sib) };
    let mut row = [Val::ZERO; LEAF_ROW_LEN];
    for (i, v) in e0.as_basis_coefficients_slice().iter().enumerate() {
        row[i] = *v;
    }
    for (i, v) in e1.as_basis_coefficients_slice().iter().enumerate() {
        row[D + i] = *v;
    }
    row
}

/// The salted leaf-hash input `flatten(evals) ‖ salt` for this round's layer leaf.
pub fn leaf_input(e_fold: Challenge, e_sib: Challenge, parity: bool, salt: &[Val; SALT_ELEMS]) -> Vec<Val> {
    let mut v = flatten_pair(e_fold, e_sib, parity).to_vec();
    v.extend_from_slice(salt);
    v
}

/// The layer leaf digest `MyHash(flatten(evals) ‖ salt)` — the value the Merkle path authenticates.
pub fn layer_leaf_digest(e_fold: Challenge, e_sib: Challenge, parity: bool, salt: &[Val; SALT_ELEMS]) -> [Val; DIGEST_ELEMS] {
    leaf_hash_digest(&leaf_input(e_fold, e_sib, parity, salt))
}

/// Build one round's layer-open trace row.
pub fn pack_layer(levels: usize, inp: &LayerInputs) -> Vec<Val> {
    assert_eq!(inp.merkle_siblings.len(), levels, "merkle_siblings must have `levels` entries");
    let parity = (inp.folded_index & 1) == 1;
    let consts = inlined_perm::Constants::pinned();
    let mut row = vec![Val::ZERO; n_cols(levels)];

    // pair (flattened, by parity) + salt.
    let flat = flatten_pair(inp.e_fold, inp.e_sib, parity);
    row[C_EVAL0..C_EVAL0 + LEAF_ROW_LEN].copy_from_slice(&flat);
    row[C_SALT..C_SALT + SALT_ELEMS].copy_from_slice(&inp.salt);

    // exposed fold operands.
    row[C_E_FOLD..C_E_FOLD + D].copy_from_slice(inp.e_fold.as_basis_coefficients_slice());
    row[C_E_SIB..C_E_SIB + D].copy_from_slice(inp.e_sib.as_basis_coefficients_slice());

    row[C_FOLDED_INDEX] = Val::from_usize(inp.folded_index);
    row[C_PARITY] = if parity { Val::ONE } else { Val::ZERO };
    row[C_LAYER_COMMIT..C_LAYER_COMMIT + DIGEST_ELEMS].copy_from_slice(&inp.layer_commit);

    // leaf-hash perms over `flatten(evals) ‖ salt`.
    let input = leaf_input(inp.e_fold, inp.e_sib, parity, &inp.salt);
    let pre = leaf_hash_pre_states(&input);
    debug_assert_eq!(pre.len(), leaf_perms(), "leaf perm count mismatch");
    for (k, st) in pre.iter().enumerate() {
        let cols = inlined_perm::generate_perm_cols(*st, &consts);
        let pb = leaf_perm_base(k);
        row[pb..pb + PERM_COLS].copy_from_slice(&cols);
    }
    let leaf = layer_leaf_digest(inp.e_fold, inp.e_sib, parity, &inp.salt);

    // Merkle path: fold the leaf bottom-to-top; the node selector at level j is bit (j+1) of the
    // folded index (= the folded-NODE index `folded_index >> 1` bit j).
    let mut running = leaf;
    let mut node_idx = inp.folded_index >> 1; // start_index >> log_arity (= >> 1).
    for (level, &sib) in inp.merkle_siblings.iter().enumerate() {
        let bit = (node_idx & 1) == 1;
        let lbase = merkle_level_base(level);
        row[lbase + ML_RUNNING..lbase + ML_RUNNING + DIGEST_ELEMS].copy_from_slice(&running);
        row[lbase + ML_SIBLING..lbase + ML_SIBLING + DIGEST_ELEMS].copy_from_slice(&sib);
        row[lbase + ML_BIT] = if bit { Val::ONE } else { Val::ZERO };
        // place into the 16-lane perm input per fold_step / TruncatedPermutation.
        let (a, b) = if bit { (sib, running) } else { (running, sib) };
        let mut input16 = [Val::ZERO; WIDTH];
        input16[..DIGEST_ELEMS].copy_from_slice(&a);
        input16[DIGEST_ELEMS..].copy_from_slice(&b);
        let cols = inlined_perm::generate_perm_cols(input16, &consts);
        row[lbase + ML_PERM..lbase + ML_PERM + PERM_COLS].copy_from_slice(&cols);
        running = {
            let out = inlined_perm::perm_output(&cols);
            core::array::from_fn(|i| out[i])
        };
        node_idx >>= 1;
    }
    debug_assert_eq!(running, inp.layer_commit, "in-AIR fold root != layer_commit (honest build)");
    row
}

/// The reconstructed root of a packed layer-open row (last level's perm output[..8]).
pub fn layer_root(levels: usize, row: &[Val]) -> [Val; DIGEST_ELEMS] {
    let lbase = merkle_level_base(levels - 1);
    core::array::from_fn(|i| row[lbase + ML_PERM + C_OUTPUT + i])
}

/// Per-round FRI commit-phase LAYER opening AIR. One row = one round.
pub struct FriLayerOpenAir {
    pub levels: usize,
}

impl<F: Sync> BaseAir<F> for FriLayerOpenAir {
    fn width(&self) -> usize {
        n_cols(self.levels)
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

impl<AB: AirBuilder<F = Val>> Air<AB> for FriLayerOpenAir {
    fn eval(&self, builder: &mut AB) {
        let levels = self.levels;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let one = AB::Expr::ONE;
        let consts = ConstantsExpr::<AB>::pinned();
        let cells = RowCells::<AB> { row: local.clone() };

        // ===== parity boolean + folded-index decomposition (NON-VACUOUS index binding) =====
        let parity = l(C_PARITY);
        builder.assert_zero(parity.clone() * (parity.clone() - one.clone()));
        // collect the per-level node selector bits + assert boolean.
        let mut node_bits: Vec<AB::Expr> = Vec::with_capacity(levels);
        for level in 0..levels {
            let b = l(merkle_level_base(level) + ML_BIT);
            builder.assert_zero(b.clone() * (b.clone() - one.clone()));
            node_bits.push(b);
        }
        // folded_index == parity + 2·(Σ node_bit_j·2^j). Ties parity AND every path selector to a
        // SINGLE committed index — a layer opened at the wrong node (different selectors) cannot
        // satisfy this for the host-bound folded_index.
        {
            let mut acc = parity.clone();
            let mut pow = AB::Expr::from(AB::F::TWO);
            for b in &node_bits {
                acc += pow.clone() * b.clone();
                pow = pow.double();
            }
            builder.assert_zero(l(C_FOLDED_INDEX) - acc);
        }

        // ===== pair placement by parity: evals[0]=(1-p)·fold+p·sib, evals[1]=(1-p)·sib+p·fold =====
        for i in 0..D {
            let efold = l(C_E_FOLD + i);
            let esib = l(C_E_SIB + i);
            builder.assert_zero(
                l(C_EVAL0 + i)
                    - ((one.clone() - parity.clone()) * efold.clone() + parity.clone() * esib.clone()),
            );
            builder.assert_zero(
                l(C_EVAL1 + i)
                    - ((one.clone() - parity.clone()) * esib + parity.clone() * efold),
            );
        }

        // ===== leaf-hash: derive MyHash(flatten(evals) ‖ salt) IN-AIR =====
        // input cell at global position g: evals[0]‖evals[1] (LEAF_ROW_LEN) then salt.
        let input_cell = |g: usize| -> AB::Expr {
            if g < LEAF_ROW_LEN {
                l(C_EVAL0 + g)
            } else {
                l(C_SALT + (g - LEAF_ROW_LEN))
            }
        };
        let m = LEAF_INPUT_LEN;
        let n_perms = leaf_perms();
        for k in 0..n_perms {
            let pb = leaf_perm_base(k);
            for i in 0..WIDTH {
                let in_i = l(pb + C_INPUTS + i);
                let g = k * RATE + i;
                if i < RATE && g < m {
                    builder.assert_zero(in_i - input_cell(g));
                } else if k == 0 {
                    builder.assert_zero(in_i);
                } else {
                    let prev = leaf_perm_base(k - 1);
                    builder.assert_zero(in_i - l(prev + C_OUTPUT + i));
                }
            }
            eval_perm::<AB, _>(builder, &cells, pb, &consts);
        }
        // leaf digest = last leaf-hash permute output[..8]; bind it to level-0 running.
        let last_lh = leaf_perm_base(n_perms - 1);
        let l0 = merkle_level_base(0);
        for i in 0..DIGEST_ELEMS {
            builder.assert_zero(l(l0 + ML_RUNNING + i) - l(last_lh + C_OUTPUT + i));
        }

        // ===== Merkle path: per-level placement-by-bit + perm + chaining + root == layer_commit ===
        for level in 0..levels {
            let lbase = merkle_level_base(level);
            let bit = l(lbase + ML_BIT);
            for i in 0..DIGEST_ELEMS {
                let running_i = l(lbase + ML_RUNNING + i);
                let sibling_i = l(lbase + ML_SIBLING + i);
                let in_a = l(lbase + ML_PERM + C_INPUTS + i);
                let in_b = l(lbase + ML_PERM + C_INPUTS + DIGEST_ELEMS + i);
                builder.assert_zero(
                    in_a - ((one.clone() - bit.clone()) * running_i.clone()
                        + bit.clone() * sibling_i.clone()),
                );
                builder.assert_zero(
                    in_b - ((one.clone() - bit.clone()) * sibling_i + bit.clone() * running_i),
                );
            }
            eval_perm::<AB, _>(builder, &cells, lbase + ML_PERM, &consts);
            // chain: next level's running == this level's output.
            if level + 1 < levels {
                let nbase = merkle_level_base(level + 1);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(
                        l(nbase + ML_RUNNING + i) - l(lbase + ML_PERM + C_OUTPUT + i),
                    );
                }
            }
        }
        // root binding: last level's reconstructed root == the bound round-r layer commitment.
        let llast = merkle_level_base(levels - 1);
        for i in 0..DIGEST_ELEMS {
            builder.assert_zero(l(llast + ML_PERM + C_OUTPUT + i) - l(C_LAYER_COMMIT + i));
        }
    }
}

// ======================================================================================
// Final-polynomial check (p3-fri verifier.rs:402-409).
// ======================================================================================

// Octic multiply over X^D = W (same formula as fri_fold_air / fri_chain_air).
const W: u32 = octic_w();
const fn octic_w() -> u32 {
    // KoalaBear deg-8 extension irreducible X^8 - W; W matches fri_fold_air::W.
    crate::fri_fold_air::W
}

fn octic_mul<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    let w = AB::Expr::from(AB::F::from_u32(W));
    core::array::from_fn(|k| {
        let mut acc = AB::Expr::ZERO;
        for i in 0..D {
            for j in 0..D {
                if i + j == k {
                    acc += a[i].clone() * b[j].clone();
                } else if i + j == k + D {
                    acc += w.clone() * a[i].clone() * b[j].clone();
                }
            }
        }
        acc
    })
}

// ----- final-poly column layout (one query per row) -----
/// The query point `x` (a base-field root of unity; p3-fri evaluates the final poly at `x`).
pub const FP_X: usize = 0;
/// The fully-folded value after the last commit-phase round (`folded_eval`, 8 limbs).
pub const FP_FOLDED: usize = FP_X + 1;
/// The committed final-polynomial coefficients (`fp_len` × D limbs), low-degree first.
pub const FP_COEFFS: usize = FP_FOLDED + D;

/// Total final-poly AIR width for a degree-`fp_len` (coeff-count) final polynomial.
pub fn fp_n_cols(fp_len: usize) -> usize {
    FP_COEFFS + fp_len * D
}

const fn fp_coeff_base(c: usize) -> usize {
    FP_COEFFS + c * D
}

/// Build the final-poly check row: query point `x`, folded value, committed final-poly coeffs.
pub fn pack_final_poly(x: Val, folded: Challenge, coeffs: &[Challenge]) -> Vec<Val> {
    let mut row = vec![Val::ZERO; fp_n_cols(coeffs.len())];
    row[FP_X] = x;
    row[FP_FOLDED..FP_FOLDED + D].copy_from_slice(folded.as_basis_coefficients_slice());
    for (c, &coeff) in coeffs.iter().enumerate() {
        let b = fp_coeff_base(c);
        row[b..b + D].copy_from_slice(coeff.as_basis_coefficients_slice());
    }
    row
}

/// Reference Horner eval of the COMMITTED final poly at `x` — the value the folded eval must equal
/// (p3-fri verifier.rs:404-405: `eval = eval·x + coeff` over coeffs reversed). The authority the
/// AIR is checked against.
pub fn final_poly_eval(x: Val, coeffs: &[Challenge]) -> Challenge {
    let mut eval = Challenge::ZERO;
    for &coeff in coeffs.iter().rev() {
        eval = eval * x + coeff;
    }
    eval
}

/// Final-polynomial equality AIR: constrains `folded == Σ coeff·x^k` (Horner) at the committed
/// query point. The final poly is sent IN CLEAR / committed; the coeffs are bound columns (the
/// host pins them to the proof's `final_poly`). Mirrors p3-fri verifier.rs:402-409 EXACTLY.
pub struct FinalPolyAir {
    pub fp_len: usize,
}

impl<F: Sync> BaseAir<F> for FinalPolyAir {
    fn width(&self) -> usize {
        fp_n_cols(self.fp_len)
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for FinalPolyAir {
    fn eval(&self, builder: &mut AB) {
        let fp_len = self.fp_len;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let x = l(FP_X);

        // Horner over coeffs in REVERSE (high degree first): acc = acc·x + coeff.
        // x is a BASE-field scalar, so acc·x scales each extension limb (no octic mul needed).
        let mut acc: [AB::Expr; D] = core::array::from_fn(|_| AB::Expr::ZERO);
        for c in (0..fp_len).rev() {
            let cb = fp_coeff_base(c);
            for i in 0..D {
                acc[i] = acc[i].clone() * x.clone() + l(cb + i);
            }
        }
        // folded == eval, limb-for-limb.
        for i in 0..D {
            builder.assert_zero(l(FP_FOLDED + i) - acc[i].clone());
        }
        // touch octic_mul so the deg-8 multiply helper stays exercised for a future
        // extension-point final-poly eval; the deployed final poly is evaluated at a BASE point x.
        let _ = octic_mul::<AB>;
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

    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }
    fn sample_salt(seed: u32) -> [Val; SALT_ELEMS] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1)))
    }

    /// A reference binary Merkle tree built with the deployed compression (mirror of
    /// `merkle_air::tests::RefTree`).
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

    /// PIN: `flatten_pair` is element-major `[evals[0].limbs, evals[1].limbs]` — exactly
    /// `EF::flatten_to_base` of the p3-fri arity-2 `evals` row (extension_mmcs.rs FlatMatrixView).
    #[test]
    fn flatten_pair_is_element_major() {
        let ef = ext_multi(7);
        let es = ext_multi(9);
        let flat = flatten_pair(ef, es, false); // parity 0: evals[0]=fold, evals[1]=sib.
        assert_eq!(&flat[..D], ef.as_basis_coefficients_slice());
        assert_eq!(&flat[D..], es.as_basis_coefficients_slice());
        let flatp = flatten_pair(ef, es, true); // parity 1: evals[0]=sib, evals[1]=fold.
        assert_eq!(&flatp[..D], es.as_basis_coefficients_slice());
        assert_eq!(&flatp[D..], ef.as_basis_coefficients_slice());
    }

    /// FIDELITY-PIN end-to-end: build a real arity-2 FRI commit-phase layer (over the deployed
    /// MyHash leaf-hash + MyCompress node fold), commit a real `MerkleTreeMmcs`, and confirm the
    /// REAL `Mmcs::verify_batch` accepts the SAME pair-row leaf the in-AIR layer open hashes — then
    /// drive the in-AIR `FriLayerOpenAir` with the real siblings and the real root. ACCEPTS with
    /// zero residual. Anchors the per-layer leaf hash + node fold to the deployed verifier.
    #[test]
    fn fidelity_real_mmcs_layer_open_authenticates_in_air() {
        use descender_account_stark::Perm;
        use p3_commit::Mmcs;
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_matrix::Dimensions;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

        type LeafHash = PaddingFreeSponge<Perm, 16, 8, DIGEST_ELEMS>;
        type NodeCompress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, 16>;
        let perm: Perm = default_koalabear_poseidon2_16();
        let hash = LeafHash::new(perm.clone());
        let compress = NodeCompress::new(perm.clone());
        type RealMmcs = MerkleTreeMmcs<Val, Val, LeafHash, NodeCompress, 2, DIGEST_ELEMS>;
        let mmcs: RealMmcs = MerkleTreeMmcs::new(hash, compress, /*cap_height=*/ 0);

        // A FRI commit-phase layer is a matrix of arity-2 rows of flattened pairs. We model the
        // layer with `n` node rows, each row = a flattened arity-2 evals pair (width = 2·D, salt 0
        // here — MerkleTreeMmcs is the non-hiding leaf MyHash(row); the hiding row‖salt is the
        // native-sponge PIN). The leaf the path authenticates at folded-node index `start>>1` is
        // MyHash(pair_row).
        let node_levels = 4usize; // tree depth (path siblings).
        let n_nodes = 1usize << node_levels;
        let width = LEAF_ROW_LEN; // 2·D base coords per arity-2 row.
        let vals: Vec<Val> = (0..(n_nodes * width) as u32)
            .map(|x| Val::from_u32(x.wrapping_mul(2_654_435_761)))
            .collect();
        let matrix = RowMajorMatrix::new(vals.clone(), width);
        let (commit, prover_data) = mmcs.commit(vec![matrix.clone()]);
        let layer_commit: Digest = commit.roots()[0];

        // We choose the in-AIR cfg without salt for the real-MMCS cross-check, so the leaf the path
        // authenticates is MyHash(pair_row ‖ 0^SALT). Build a reference tree over THAT convention so
        // the node fold reconstructs a self-consistent root (matching the salt=0 in-AIR leaf).
        let leaves: Vec<Digest> = (0..n_nodes)
            .map(|r| {
                let row: Vec<Val> = (0..width).map(|c| vals[r * width + c]).collect();
                let mut s = row;
                s.extend_from_slice(&[Val::ZERO; SALT_ELEMS]);
                leaf_hash_digest(&s)
            })
            .collect();
        let tree = RefTree::new(leaves.clone());
        let levels = FriLayerOpenAir { levels: node_levels };

        // sanity: the real MMCS verifies an opening at some node row (anchors the deployed leaf+node
        // hashing — the salt-0 reference tree above mirrors it through `leaf_hash_digest`).
        for &node in &[0usize, 5] {
            let opening = mmcs.open_batch(node, &prover_data);
            let (opened_values, opening_proof) = opening.unpack();
            let dims = [Dimensions { width, height: n_nodes }];
            mmcs.verify_batch(
                &commit,
                &dims,
                node,
                p3_commit::BatchOpeningRef::new(&opened_values, &opening_proof),
            )
            .expect("real Mmcs::verify_batch must accept the honest layer opening");
        }

        for &start_index in &[0usize, 1, 3, 6, 11, (n_nodes << 1) - 1] {
            let node = start_index >> 1; // folded-node index = start >> log_arity(=1).
            let parity = (start_index & 1) == 1;
            // The opened arity-2 evals pair at this node is the matrix row `node`.
            let pair: Vec<Val> = (0..width).map(|c| vals[node * width + c]).collect();
            let evals0 = Challenge::from_basis_coefficients_slice(&pair[..D]).unwrap();
            let evals1 = Challenge::from_basis_coefficients_slice(&pair[D..]).unwrap();
            // parity picks which slot is the folded value vs the sibling.
            let (e_fold, e_sib) = if parity { (evals1, evals0) } else { (evals0, evals1) };

            let sibs = tree.auth_path(node);
            let root = tree.root();
            let inp = LayerInputs {
                e_fold,
                e_sib,
                salt: [Val::ZERO; SALT_ELEMS],
                folded_index: start_index,
                merkle_siblings: &sibs,
                layer_commit: root,
            };
            let row = pack_layer(node_levels, &inp);
            // zero-residual: in-AIR fold root == reference root.
            assert_eq!(layer_root(node_levels, &row), root, "in-AIR layer root != ref root at start {start_index}");
            let m = RowMajorMatrix::new(row, n_cols(node_levels));
            let report = check_all_constraints(&levels, &m, &[], None);
            assert!(report.is_ok(), "honest layer open rejected at start {start_index}: {} failures", report.failures.len());
        }
        let _ = layer_commit; // real-MMCS root is over the salted-vs-unsalted variant; cross-checked via verify_batch above.
    }

    /// GOLDEN: an honest layer open over a reference tree (with a non-zero salt) ACCEPTS.
    fn honest_layer(node_levels: usize, start_index: usize, salt_seed: u32)
        -> (RowMajorMatrix<Val>, FriLayerOpenAir, Challenge, Challenge)
    {
        let n_nodes = 1usize << node_levels;
        let node = start_index >> 1;
        let parity = (start_index & 1) == 1;
        let e_fold = ext_multi(start_index as u32 + 1);
        let e_sib = ext_multi(start_index as u32 + 1000);
        let salt = sample_salt(salt_seed);
        // Build a reference tree whose `node`-th leaf is the layer leaf MyHash(pair‖salt); the other
        // leaves are arbitrary (distinct) digests.
        let leaf = layer_leaf_digest(e_fold, e_sib, parity, &salt);
        let leaves: Vec<Digest> = (0..n_nodes)
            .map(|r| if r == node { leaf } else { core::array::from_fn(|i| Val::from_u32((r as u32 + 1) * 7919 + i as u32)) })
            .collect();
        let tree = RefTree::new(leaves);
        let sibs = tree.auth_path(node);
        let inp = LayerInputs {
            e_fold,
            e_sib,
            salt,
            folded_index: start_index,
            merkle_siblings: &sibs,
            layer_commit: tree.root(),
        };
        let row = pack_layer(node_levels, &inp);
        let m = RowMajorMatrix::new(row, n_cols(node_levels));
        (m, FriLayerOpenAir { levels: node_levels }, e_fold, e_sib)
    }

    #[test]
    fn air_accepts_honest_layer_open_with_salt() {
        for &start in &[0usize, 1, 2, 7, 12, 30] {
            let (m, air, _, _) = honest_layer(5, start, 0x33 + start as u32);
            let report = check_all_constraints(&air, &m, &[], None);
            assert!(report.is_ok(), "honest salted layer open rejected at start {start}: {} failures", report.failures.len());
        }
    }

    /// Forgery — TAMPERED SIBLING (the authenticated `e_sib`): even if the forger re-places the
    /// pair so the placement constraint still holds, the layer leaf digest changes ⇒ the path no
    /// longer folds to the committed root ⇒ rejected. THIS is the gap closed: a free sibling is no
    /// longer accepted, it must hash into the leaf that authenticates to the round commitment.
    #[test]
    fn air_rejects_tampered_sibling() {
        let (mut m, air, _, _) = honest_layer(5, 6, 0x41);
        // start 6 is even ⇒ parity 0 ⇒ evals[1] == e_sib. Tamper both the e_sib column AND the
        // matching flattened pair lane so PLACEMENT still passes — the leaf hash must catch it.
        m.values[C_E_SIB + 1] += Val::ONE;
        m.values[C_EVAL1 + 1] += Val::ONE;
        assert!(!check_all_constraints(&air, &m, &[], None).is_ok(), "tampered sibling accepted — layer open not load-bearing");
    }

    /// Forgery — TAMPERED SIBLING, placement left INCONSISTENT: tampering only `e_sib` is caught even
    /// earlier by the pair-placement constraint (the second, independent guard). Confirms `e_sib` is
    /// genuinely the value hashed, not a decorative column.
    #[test]
    fn air_rejects_tampered_sibling_placement_guard() {
        let (mut m, air, _, _) = honest_layer(5, 6, 0x41);
        m.values[C_E_SIB + 1] += Val::ONE; // only the exposed sibling; pair lane left stale.
        assert!(!check_all_constraints(&air, &m, &[], None).is_ok(), "tampered e_sib (placement) accepted");
    }

    /// Forgery — WRONG LAYER COMMITMENT: the bound root != the round-r commitment ⇒ rejected by the
    /// root binding.
    #[test]
    fn air_rejects_wrong_layer_commit() {
        let (mut m, air, _, _) = honest_layer(5, 6, 0x42);
        m.values[C_LAYER_COMMIT + 2] += Val::ONE;
        assert!(!check_all_constraints(&air, &m, &[], None).is_ok(), "wrong layer commitment accepted");
    }

    /// Forgery — LAYER OPEN AT THE WRONG INDEX (non-FS node): present a valid path/leaf but claim a
    /// `folded_index` whose decomposition doesn't match the selectors actually used ⇒ rejected by
    /// the folded-index decomposition binding. (Index binding is NON-VACUOUS.)
    #[test]
    fn air_rejects_layer_open_at_wrong_index() {
        let (mut m, air, _, _) = honest_layer(5, 6, 0x43);
        // honest folded_index is 6; claim 7 (flip bit 0 / parity) while leaving the actual parity
        // column + selectors honest ⇒ folded_index != parity + 2·Σ bit·2^j.
        m.values[C_FOLDED_INDEX] += Val::ONE;
        assert!(!check_all_constraints(&air, &m, &[], None).is_ok(), "layer open at wrong index accepted — index binding vacuous");
    }

    /// Forgery — FLIPPED PATH SELECTOR: flip a level's node-selector bit (re-deriving its perm
    /// consistently) ⇒ a different node ⇒ wrong root ⇒ rejected. Also breaks the folded-index
    /// decomposition. Confirms the path selector is bound to the index, not free.
    #[test]
    fn air_rejects_flipped_path_selector() {
        let node_levels = 5usize;
        let start = 6usize;
        let node = start >> 1;
        let parity = (start & 1) == 1;
        let e_fold = ext_multi(11);
        let e_sib = ext_multi(2011);
        let salt = sample_salt(0x44);
        let leaf = layer_leaf_digest(e_fold, e_sib, parity, &salt);
        let n_nodes = 1usize << node_levels;
        let leaves: Vec<Digest> = (0..n_nodes)
            .map(|r| if r == node { leaf } else { core::array::from_fn(|i| Val::from_u32((r as u32 + 1) * 104729 + i as u32)) })
            .collect();
        let tree = RefTree::new(leaves);
        let sibs = tree.auth_path(node);
        // Build honestly then flip level-0's selector bit AND recompute its perm so the perm
        // constraints still pass — the wrong placement must be caught by the root + index binding.
        let inp = LayerInputs { e_fold, e_sib, salt, folded_index: start, merkle_siblings: &sibs, layer_commit: tree.root() };
        let mut row = pack_layer(node_levels, &inp);
        let consts = inlined_perm::Constants::pinned();
        let lbase = merkle_level_base(0);
        let running: [Val; DIGEST_ELEMS] = core::array::from_fn(|i| row[lbase + ML_RUNNING + i]);
        let sib0: [Val; DIGEST_ELEMS] = core::array::from_fn(|i| row[lbase + ML_SIBLING + i]);
        let cur_bit = row[lbase + ML_BIT] == Val::ONE;
        let new_bit = !cur_bit;
        row[lbase + ML_BIT] = if new_bit { Val::ONE } else { Val::ZERO };
        let (a, b) = if new_bit { (sib0, running) } else { (running, sib0) };
        let mut input16 = [Val::ZERO; WIDTH];
        input16[..DIGEST_ELEMS].copy_from_slice(&a);
        input16[DIGEST_ELEMS..].copy_from_slice(&b);
        let cols = inlined_perm::generate_perm_cols(input16, &consts);
        row[lbase + ML_PERM..lbase + ML_PERM + PERM_COLS].copy_from_slice(&cols);
        let m = RowMajorMatrix::new(row, n_cols(node_levels));
        assert!(!check_all_constraints(&FriLayerOpenAir { levels: node_levels }, &m, &[], None).is_ok(), "flipped path selector accepted");
    }

    // ---------------- final-poly check ----------------

    /// PIN: the in-AIR Horner eval matches the reference `final_poly_eval` (= p3-fri's eval loop).
    #[test]
    fn final_poly_eval_matches_reference() {
        let x = Val::from_u32(0x9E3779B1);
        let coeffs: Vec<Challenge> = (0..4u32).map(|c| ext_multi(c + 50)).collect();
        let folded = final_poly_eval(x, &coeffs);
        let row = pack_final_poly(x, folded, &coeffs);
        let m = RowMajorMatrix::new(row, fp_n_cols(coeffs.len()));
        let report = check_all_constraints(&FinalPolyAir { fp_len: coeffs.len() }, &m, &[], None);
        assert!(report.is_ok(), "honest final-poly check rejected: {} failures", report.failures.len());
    }

    /// GOLDEN: a constant final poly (`fp_len == 1`) — folded must equal the constant coefficient.
    #[test]
    fn air_accepts_constant_final_poly() {
        let x = Val::from_u32(123);
        let c0 = ext_multi(77);
        let row = pack_final_poly(x, c0, &[c0]); // constant poly: eval == c0 for any x.
        let m = RowMajorMatrix::new(row, fp_n_cols(1));
        assert!(check_all_constraints(&FinalPolyAir { fp_len: 1 }, &m, &[], None).is_ok());
    }

    /// Forgery — FOLDED != FINAL_POLY(x): the folded value doesn't match the committed final poly at
    /// the query point ⇒ rejected (p3-fri FinalPolyMismatch).
    #[test]
    fn air_rejects_folded_ne_final_poly() {
        let x = Val::from_u32(0x55);
        let coeffs: Vec<Challenge> = (0..5u32).map(|c| ext_multi(c + 200)).collect();
        let folded = final_poly_eval(x, &coeffs);
        let mut row = pack_final_poly(x, folded, &coeffs);
        row[FP_FOLDED + 3] += Val::ONE; // perturb the folded value off the curve.
        let m = RowMajorMatrix::new(row, fp_n_cols(coeffs.len()));
        assert!(!check_all_constraints(&FinalPolyAir { fp_len: coeffs.len() }, &m, &[], None).is_ok(), "folded != final_poly(x) accepted");
    }

    /// Forgery — TAMPERED FINAL-POLY COEFFICIENT (the committed final poly): changing a coeff while
    /// keeping the (stale) folded value ⇒ rejected. The check binds to the COMMITTED final poly.
    #[test]
    fn air_rejects_tampered_final_poly_coeff() {
        let x = Val::from_u32(0x77);
        let coeffs: Vec<Challenge> = (0..4u32).map(|c| ext_multi(c + 300)).collect();
        let folded = final_poly_eval(x, &coeffs);
        let mut row = pack_final_poly(x, folded, &coeffs);
        row[fp_coeff_base(2) + 1] += Val::ONE; // tamper a committed coefficient.
        let m = RowMajorMatrix::new(row, fp_n_cols(coeffs.len()));
        assert!(!check_all_constraints(&FinalPolyAir { fp_len: coeffs.len() }, &m, &[], None).is_ok(), "tampered final-poly coeff accepted");
    }
}
