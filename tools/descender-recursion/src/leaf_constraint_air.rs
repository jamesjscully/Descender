//! Gadget 4 (proper, PART 2) — the leaf `C(zeta)` MIRROR: evaluating a SUBSET of the leaf
//! [`descender_account_stark::air::FullRelationAir`] constraint polynomials at the DEEP
//! out-of-domain point `zeta`, as a real `Air<AB>`.
//!
//! ## Why this exists
//!
//! [`crate::deep_air`] checks the DEEP-ALI quotient relation `C(zeta) == Z_H(zeta)·Q(zeta)`
//! but treats `C(zeta)` (the leaf AIR constraint-polynomial evaluation at `zeta`) as an
//! OPENED / COMMITTED value — i.e. it trusts the prover's claimed `C(zeta)`. The full DEEP
//! soundness requires `C(zeta)` to be RE-COMPUTED in-circuit from the opened trace cells
//! `T_i(zeta)`, so the prover cannot smuggle a favorable `C(zeta)`. That re-computation is the
//! entire leaf folder (all of `air.rs`: C1/C2/C4/C6 + Poseidon2 perms + reduce + cross-seams),
//! the single largest remaining recursion piece.
//!
//! ## Scope of THIS module (the Poseidon2-FREE leaf constraints, alpha-batched)
//!
//! This mirrors EXACTLY the algebraic, permutation-free leaf gadgets (degree ≤ 2, same-row):
//!
//!   1. **C4 canonical base-128 residue gadget** (`FullRelationAir::eval_c4_residue`,
//!      air.rs:1381) for ALL `C4_LANES = 6` lane blocks (29 residuals each ⇒ 174).
//!   2. **C4-acc carrier binding** (air.rs:762): `carry_acc[j] == v_j` for each lane (6).
//!   3. **SeamPadSeed** (air.rs:765-771): C1's 32 `pad_seed` lanes == C2's 32 `pad_seed`
//!      lanes (32 same-row equalities).
//!   4. **SeamOpeningAcc** (air.rs:773-779): C4's 6 `carry_acc` values == C6's 6 field lanes
//!      (6 same-row equalities).
//!   5. **relation_id pinning** (CLASS B, air.rs:957-967): the 4 public `relation_id` lanes
//!      pinned to the constant [`RELATION_ID`] (4 public pinnings).
//!
//! Each constraint is one extension-valued zero-residual `c_i(T(zeta))`, computed over the
//! OPENED cells (each an extension element `Challenge`, opened at `zeta`). We then form the
//! **alpha-batched** sum mirroring `p3-uni-stark`'s `SymbolicAirBuilder`/`accumulate`:
//!
//!   `C_partial(zeta) = Σ_i alpha^i · c_i(T(zeta))`
//!
//! over the SAME octic (`X^8 = W`) extension arithmetic as [`crate::deep_air`] /
//! [`crate::fri_fold_air`]. The AIR binds the committed `C_partial` cell to this recomputed sum.
//!
//! ## Fidelity (the mirror MUST equal air.rs EXACTLY)
//!
//! A WRONG mirror is a soundness hole, so [`tests::mirror_matches_leaf_air_on_real_row`] and the
//! per-class tamper tests pin the honest recomputed residuals against the leaf AIR's own
//! `eval_c4_residue` / seam / relation_id semantics evaluated on REAL packed leaf rows, and the
//! alpha-batched honest `C_partial` against an independent native reference
//! [`partial_constraint_eval`]. Every opened-cell tamper that violates a mirrored leaf
//! constraint changes `C_partial` ⇒ rejects.
//!
//! ## DEFERRED (the rest of leaf `C(zeta)` — STILL committed-not-constrained)
//!
//!   * ALL Poseidon2 permutation round constraints (C1/C2/C6 sponges, role-root sponge, batch
//!     digest chain) — the BULK of the leaf width; reuse the inlined-perm / `eval_perm`
//!     machinery (deferred deliberately, separate large chunk).
//!   * the C1/C2/C6 sponge-OUTPUT commitment equalities (`c*_out[j] == pubio[j]`): these
//!     consume the Poseidon2 squeeze, so they ride with the perms.
//!   * the WIDE-THEN-REDUCE C2 mask reduce gadget + the carrier mod-q reduction.
//!   * CLASS-C packed-witness anchors, context bindings (slot/lifecycle/account_id), the
//!     cross-component digest chain.
//!
//! ## Wiring into DEEP
//!
//! [`crate::deep_air`] now consumes this `C_partial` as a CONSTRAINED summand of the DEEP
//! `C(zeta)`: `C = C_partial + C_committed`, with `C == Z_H·Q`. A tampered opened leaf cell that
//! violates a mirrored constraint changes `C_partial`, breaking the OOD identity ⇒ rejected
//! END-TO-END. The `C_committed` summand is the deferred Poseidon2-perm bulk (still committed).

use crate::fri_fold_air::{limbs, D, W};
use descender_account_stark::{Challenge, Val, N_LANES, RELATION_ID, RELATION_ID_ELEMS};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

// ----- leaf C4 gadget constants (mirror air.rs:201-204, 292) -----

/// Base-128 radix (`d = d_lo + 128·d_hi`). Mirrors air.rs `C4_BASE`.
const C4_BASE: u32 = 128;
/// Bits per base-128 digit. Mirrors air.rs `C4_DIGIT_BITS`.
const C4_DIGIT_BITS: usize = 7;
/// Canonical max high digit: `q12289 = 96·128 + 1 ⇒ d_hi ∈ [0,96]`. Mirrors air.rs `C4_MAX_HI`.
const C4_MAX_HI: u32 = 96;
/// Cells in one leaf C4 lane block: `v ‖ d_lo ‖ d_hi ‖ lo_bits[7] ‖ hi_bits[7] ‖ slack_bits[7]
/// ‖ is96 ‖ inv`. Mirrors air.rs `C4_CELLS = 3 + 3*7 + 2 = 26`.
pub const C4_CELLS: usize = 3 + 3 * C4_DIGIT_BITS + 2; // 26

/// Number of C4 lane blocks (= projection lanes). Mirrors air.rs `N_LANES`.
pub const C4_LANES: usize = N_LANES; // 6

/// Number of distinct C4 constraint residuals batched PER LANE: 21 bit-booleans (lo[7] ‖ hi[7] ‖
/// slack[7]) + 8 algebraic (lo-recompose, hi-recompose, slack-recompose, is96-bool,
/// is96·(d_hi−96), inverse-corner, is96·d_lo, v-recompose).
pub const C4_N_PER_LANE: usize = 21 + 8; // 29

// Within-block cell indices (mirror air.rs `eval_c4_residue` local offsets).
const I_V: usize = 0;
const I_DLO: usize = 1;
const I_DHI: usize = 2;
const I_LO_BITS: usize = 3;
const I_HI_BITS: usize = 10;
const I_SLACK_BITS: usize = 17;
const I_IS96: usize = 24;
const I_INV: usize = 25;

// ----- the other algebraic (Poseidon2-free) leaf constraints -----

/// SeamPadSeed: C1's 32 `pad_seed` lanes == C2's 32 `pad_seed` lanes (air.rs:765-771).
pub const PAD_SEED_BYTES: usize = 32;
/// Number of constraint residuals contributed by each mirrored class.
pub const N_C4_ACC: usize = C4_LANES; // carry_acc[j] == v_j (air.rs:762)
pub const N_SEAM_PAD: usize = PAD_SEED_BYTES; // SeamPadSeed
pub const N_SEAM_ACC: usize = N_LANES; // SeamOpeningAcc (air.rs:773-779)
pub const N_RELATION_ID: usize = RELATION_ID_ELEMS; // relation_id pinning (air.rs:957-967)

/// Total residuals batched into `C_partial`.
pub const N_CONSTRAINTS: usize =
    C4_LANES * C4_N_PER_LANE + N_C4_ACC + N_SEAM_PAD + N_SEAM_ACC + N_RELATION_ID;

// ===========================================================================
// Column layout (one leaf-constraint-mirror check per row)
// ===========================================================================
//
// The opened OOD cells are extension elements (D=8 `Val` limbs each), laid out as contiguous
// blocks of `D` limbs in the following order:
//   * `C4_LANES` C4 lane blocks, each `C4_CELLS` cells           [C4_LANES*C4_CELLS cells]
//   * `carry_acc[N_LANES]`                                       [N_LANES cells]
//   * C1 `pad_seed[32]`                                          [PAD_SEED_BYTES cells]
//   * C2 `pad_seed[32]`                                          [PAD_SEED_BYTES cells]
//   * C6 `field_lanes[N_LANES]`                                  [N_LANES cells]
//   * `relation_id[RELATION_ID_ELEMS]`                           [RELATION_ID_ELEMS cells]
// then `alpha` (1 extension elem) and the committed `C_partial` (1 extension elem).

/// Number of opened OOD cells fed to the mirror.
pub const N_OPENED_CELLS: usize = C4_LANES * C4_CELLS // C4 lanes
    + N_LANES // carry_acc
    + PAD_SEED_BYTES // C1 pad_seed
    + PAD_SEED_BYTES // C2 pad_seed
    + N_LANES // C6 field lanes
    + RELATION_ID_ELEMS; // relation_id

// Cell-block offsets (in units of cells, each `D` limbs).
const O_C4: usize = 0;
const O_ACC: usize = O_C4 + C4_LANES * C4_CELLS;
const O_C1_PAD: usize = O_ACC + N_LANES;
const O_C2_PAD: usize = O_C1_PAD + PAD_SEED_BYTES;
const O_C6_FIELD: usize = O_C2_PAD + PAD_SEED_BYTES;
const O_RELATION_ID: usize = O_C6_FIELD + N_LANES;

/// First column: the opened OOD cells, each `D` limbs.
pub const L_CELLS: usize = 0;
/// The constraint-batching challenge `alpha` (`D` limbs).
pub const L_ALPHA: usize = L_CELLS + N_OPENED_CELLS * D;
/// The committed alpha-batched partial constraint eval `C_partial(zeta)` (`D` limbs).
pub const L_CPARTIAL: usize = L_ALPHA + D;

/// Total column count.
pub const fn n_cols() -> usize {
    L_CPARTIAL + D
}

/// Limb base of opened cell `c` (`c < N_OPENED_CELLS`).
const fn cell_base(c: usize) -> usize {
    L_CELLS + c * D
}

// Cell accessors (each returns a cell INDEX in `[0, N_OPENED_CELLS)`).
const fn c4_cell(lane: usize, within: usize) -> usize {
    O_C4 + lane * C4_CELLS + within
}
const fn acc_cell(j: usize) -> usize {
    O_ACC + j
}
const fn c1_pad_cell(b: usize) -> usize {
    O_C1_PAD + b
}
const fn c2_pad_cell(b: usize) -> usize {
    O_C2_PAD + b
}
const fn c6_field_cell(i: usize) -> usize {
    O_C6_FIELD + i
}
const fn relation_id_cell(j: usize) -> usize {
    O_RELATION_ID + j
}

// ===========================================================================
// Octic (deg-8 extension) arithmetic over `X^D = W` — same as deep_air / fri_fold_air.
// ===========================================================================

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

/// Limbwise add of two extension exprs.
fn octic_add<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() + b[i].clone())
}

/// Limbwise subtract.
fn octic_sub<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() - b[i].clone())
}

/// Embed a base-field scalar `s` as an extension expr (limb 0 = s, rest 0).
fn octic_scalar<AB: AirBuilder>(s: u64) -> [AB::Expr; D] {
    core::array::from_fn(|i| {
        if i == 0 {
            AB::Expr::from(AB::F::from_u64(s))
        } else {
            AB::Expr::ZERO
        }
    })
}

/// Scale an extension expr by a base-field constant `s` (limbwise; no reduction since `s` is base).
fn scale<AB: AirBuilder>(a: &[AB::Expr; D], s: u64) -> [AB::Expr; D] {
    let sc = AB::Expr::from(AB::F::from_u64(s));
    core::array::from_fn(|i| a[i].clone() * sc.clone())
}

// ===========================================================================
// Native reference (the AUTHORITY the AIR is checked against)
// ===========================================================================

/// Native recompose of 7 bit cells starting at `start` into `Σ bit_b · 2^b`.
fn native_recompose(cells: &[Challenge; N_OPENED_CELLS], start_cell: usize) -> Challenge {
    let mut acc = Challenge::ZERO;
    for b in 0..C4_DIGIT_BITS {
        acc += cells[start_cell + b] * Challenge::from(Val::from_u64(1u64 << b));
    }
    acc
}

/// One leaf C4 constraint residual for `lane`, `i ∈ [0, C4_N_PER_LANE)`:
///   * `0..21`  : bit-boolean residuals `bit·(bit−1)` for lo[7] ‖ hi[7] ‖ slack[7].
///   * `21`     : `d_lo − recompose(lo_bits)`
///   * `22`     : `d_hi − recompose(hi_bits)`
///   * `23`     : `(96 − d_hi) − recompose(slack_bits)`
///   * `24`     : `is96·(is96−1)`
///   * `25`     : `is96·(d_hi − 96)`
///   * `26`     : `(d_hi − 96)·inv − (1 − is96)`
///   * `27`     : `is96·d_lo`
///   * `28`     : `v − (d_lo + 128·d_hi)`
fn c4_lane_residual(cells: &[Challenge; N_OPENED_CELLS], lane: usize, i: usize) -> Challenge {
    let v = cells[c4_cell(lane, I_V)];
    let d_lo = cells[c4_cell(lane, I_DLO)];
    let d_hi = cells[c4_cell(lane, I_DHI)];
    let is96 = cells[c4_cell(lane, I_IS96)];
    let inv = cells[c4_cell(lane, I_INV)];
    let max_hi = Challenge::from(Val::from_u32(C4_MAX_HI));
    let base = Challenge::from(Val::from_u32(C4_BASE));

    if i < 21 {
        let bit = cells[c4_cell(lane, I_LO_BITS + i)];
        return bit * (bit - Challenge::ONE);
    }
    match i - 21 {
        0 => d_lo - native_recompose(cells, c4_cell(lane, I_LO_BITS)),
        1 => d_hi - native_recompose(cells, c4_cell(lane, I_HI_BITS)),
        2 => (max_hi - d_hi) - native_recompose(cells, c4_cell(lane, I_SLACK_BITS)),
        3 => is96 * (is96 - Challenge::ONE),
        4 => is96 * (d_hi - max_hi),
        5 => (d_hi - max_hi) * inv - (Challenge::ONE - is96),
        6 => is96 * d_lo,
        7 => v - (d_lo + base * d_hi),
        _ => unreachable!("c4 constraint index out of range"),
    }
}

/// The `r`-th leaf constraint residual over the opened cells, `r ∈ [0, N_CONSTRAINTS)`. Ordering:
///   * `0 .. C4_LANES*C4_N_PER_LANE`            : C4 lane residuals (lane-major).
///   * next `N_C4_ACC`                          : `carry_acc[j] − v_j` (C4-acc binding).
///   * next `N_SEAM_PAD`                        : `c1_pad_seed[b] − c2_pad_seed[b]` (SeamPadSeed).
///   * next `N_SEAM_ACC`                        : `carry_acc[i] − c6_field[i]` (SeamOpeningAcc).
///   * next `N_RELATION_ID`                     : `relation_id[j] − RELATION_ID[j]` (pinning).
pub fn constraint_residual(cells: &[Challenge; N_OPENED_CELLS], r: usize) -> Challenge {
    let mut r = r;
    let c4_total = C4_LANES * C4_N_PER_LANE;
    if r < c4_total {
        let lane = r / C4_N_PER_LANE;
        let i = r % C4_N_PER_LANE;
        return c4_lane_residual(cells, lane, i);
    }
    r -= c4_total;
    if r < N_C4_ACC {
        // carry_acc[j] − v_j (air.rs:762).
        let j = r;
        return cells[acc_cell(j)] - cells[c4_cell(j, I_V)];
    }
    r -= N_C4_ACC;
    if r < N_SEAM_PAD {
        // SeamPadSeed: c1_pad_seed[b] − c2_pad_seed[b].
        let b = r;
        return cells[c1_pad_cell(b)] - cells[c2_pad_cell(b)];
    }
    r -= N_SEAM_PAD;
    if r < N_SEAM_ACC {
        // SeamOpeningAcc: carry_acc[i] − c6_field[i].
        let i = r;
        return cells[acc_cell(i)] - cells[c6_field_cell(i)];
    }
    r -= N_SEAM_ACC;
    debug_assert!(r < N_RELATION_ID);
    // relation_id pinning: relation_id[j] − RELATION_ID[j].
    let j = r;
    cells[relation_id_cell(j)] - Challenge::from(Val::from_u32(RELATION_ID[j]))
}

/// Native alpha-batched leaf partial constraint eval over opened cells:
/// `C_partial = Σ_r alpha^r · c_r(cells)`. This is the AUTHORITY; the AIR re-derives it in-circuit.
pub fn partial_constraint_eval(cells: &[Challenge; N_OPENED_CELLS], alpha: Challenge) -> Challenge {
    let mut acc = Challenge::ZERO;
    let mut a_pow = Challenge::ONE;
    for r in 0..N_CONSTRAINTS {
        acc += a_pow * constraint_residual(cells, r);
        a_pow *= alpha;
    }
    acc
}

/// Build one honest leaf-constraint-mirror row from opened cells + `alpha`, computing `C_partial`
/// via the native reference. (The builder can only emit honest rows; tamper tests mutate the
/// matrix afterwards.)
pub fn pack_row(cells: &[Challenge; N_OPENED_CELLS], alpha: Challenge) -> Vec<Val> {
    let c_partial = partial_constraint_eval(cells, alpha);
    let mut row = vec![Val::ZERO; n_cols()];
    for c in 0..N_OPENED_CELLS {
        row[cell_base(c)..cell_base(c) + D].copy_from_slice(&limbs(cells[c]));
    }
    row[L_ALPHA..L_ALPHA + D].copy_from_slice(&limbs(alpha));
    row[L_CPARTIAL..L_CPARTIAL + D].copy_from_slice(&limbs(c_partial));
    row
}

/// Decode the committed `C_partial` cell of a packed row.
pub fn row_c_partial(row: &[Val]) -> Challenge {
    Challenge::from_basis_coefficients_slice(&row[L_CPARTIAL..L_CPARTIAL + D])
        .expect("C_partial decode")
}

// ===========================================================================
// The AIR
// ===========================================================================

/// REUSABLE: recompute the alpha-batched Poseidon2-free leaf `C_partial(zeta)` over opened OOD
/// cells, as in-AIR extension expressions. `cell(c)` returns opened cell `c ∈ [0, N_OPENED_CELLS)`
/// as a `[AB::Expr; D]` octic element; `alpha` is the batching challenge block. The returned
/// `[AB::Expr; D]` is `Σ_r alpha^r · c_r(cells)` — the SAME quantity [`partial_constraint_eval`]
/// computes natively and the SAME residual ORDER. [`LeafConstraintMirrorAir`] binds this to the
/// committed `C_partial`; [`crate::deep_air`] wires it as a constrained summand of the DEEP
/// `C(zeta)`. Soundness lives ENTIRELY in this function's faithfulness to air.rs (fidelity-PINned).
pub fn recompute_c_partial<AB, FCell>(cell: &FCell, alpha: &[AB::Expr; D]) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let one = octic_scalar::<AB>(1);
    let max_hi = octic_scalar::<AB>(C4_MAX_HI as u64);
    let base_radix = octic_scalar::<AB>(C4_BASE as u64);

    // recompose 7 bit cells starting at cell index `start` into Σ bit_b · 2^b.
    let recompose = |start_cell: usize| -> [AB::Expr; D] {
        let mut acc = octic_scalar::<AB>(0);
        for b in 0..C4_DIGIT_BITS {
            let bit = cell(start_cell + b);
            let scaled = scale::<AB>(&bit, 1u64 << b);
            acc = octic_add::<AB>(&acc, &scaled);
        }
        acc
    };

    // ----- build the residuals (each an extension element), SAME order as the native ref -----
    let mut residuals: Vec<[AB::Expr; D]> = Vec::with_capacity(N_CONSTRAINTS);

    // (1) C4 lanes (lane-major).
    for lane in 0..C4_LANES {
        let v = cell(c4_cell(lane, I_V));
        let d_lo = cell(c4_cell(lane, I_DLO));
        let d_hi = cell(c4_cell(lane, I_DHI));
        let is96 = cell(c4_cell(lane, I_IS96));
        let inv = cell(c4_cell(lane, I_INV));

        // 0..21 : bit-boolean residuals bit·(bit−1) over lo[7] ‖ hi[7] ‖ slack[7].
        for i in 0..21 {
            let bit = cell(c4_cell(lane, I_LO_BITS + i));
            let bit_minus_one = octic_sub::<AB>(&bit, &one);
            residuals.push(octic_mul::<AB>(&bit, &bit_minus_one));
        }
        // 21 : d_lo − recompose(lo_bits)
        residuals.push(octic_sub::<AB>(&d_lo, &recompose(c4_cell(lane, I_LO_BITS))));
        // 22 : d_hi − recompose(hi_bits)
        residuals.push(octic_sub::<AB>(&d_hi, &recompose(c4_cell(lane, I_HI_BITS))));
        // 23 : (96 − d_hi) − recompose(slack_bits)
        let max_minus_dhi = octic_sub::<AB>(&max_hi, &d_hi);
        residuals.push(octic_sub::<AB>(
            &max_minus_dhi,
            &recompose(c4_cell(lane, I_SLACK_BITS)),
        ));
        // 24 : is96·(is96−1)
        let is96_minus_one = octic_sub::<AB>(&is96, &one);
        residuals.push(octic_mul::<AB>(&is96, &is96_minus_one));
        // 25 : is96·(d_hi − 96)
        let dhi_minus_max = octic_sub::<AB>(&d_hi, &max_hi);
        residuals.push(octic_mul::<AB>(&is96, &dhi_minus_max));
        // 26 : (d_hi − 96)·inv − (1 − is96)
        let prod = octic_mul::<AB>(&dhi_minus_max, &inv);
        let one_minus_is96 = octic_sub::<AB>(&one, &is96);
        residuals.push(octic_sub::<AB>(&prod, &one_minus_is96));
        // 27 : is96·d_lo
        residuals.push(octic_mul::<AB>(&is96, &d_lo));
        // 28 : v − (d_lo + 128·d_hi)
        let base_dhi = octic_mul::<AB>(&base_radix, &d_hi);
        let recomposed_v = octic_add::<AB>(&d_lo, &base_dhi);
        residuals.push(octic_sub::<AB>(&v, &recomposed_v));
    }

    // (2) C4-acc carrier binding: carry_acc[j] − v_j.
    for j in 0..N_C4_ACC {
        let acc = cell(acc_cell(j));
        let v = cell(c4_cell(j, I_V));
        residuals.push(octic_sub::<AB>(&acc, &v));
    }

    // (3) SeamPadSeed: c1_pad_seed[b] − c2_pad_seed[b].
    for b in 0..N_SEAM_PAD {
        let c1b = cell(c1_pad_cell(b));
        let c2b = cell(c2_pad_cell(b));
        residuals.push(octic_sub::<AB>(&c1b, &c2b));
    }

    // (4) SeamOpeningAcc: carry_acc[i] − c6_field[i].
    for i in 0..N_SEAM_ACC {
        let acc = cell(acc_cell(i));
        let c6 = cell(c6_field_cell(i));
        residuals.push(octic_sub::<AB>(&acc, &c6));
    }

    // (5) relation_id pinning: relation_id[j] − RELATION_ID[j].
    for j in 0..N_RELATION_ID {
        let rid = cell(relation_id_cell(j));
        let pinned = octic_scalar::<AB>(RELATION_ID[j] as u64);
        residuals.push(octic_sub::<AB>(&rid, &pinned));
    }

    debug_assert_eq!(residuals.len(), N_CONSTRAINTS);

    // ----- alpha-batch: acc = Σ_r alpha^r · residual_r -----
    let mut acc = octic_scalar::<AB>(0);
    let mut a_pow = octic_scalar::<AB>(1);
    for res in residuals.iter() {
        let term = octic_mul::<AB>(&a_pow, res);
        acc = octic_add::<AB>(&acc, &term);
        a_pow = octic_mul::<AB>(&a_pow, alpha);
    }
    acc
}

/// Leaf-constraint-mirror AIR: recomputes the alpha-batched Poseidon2-free leaf constraint
/// residual `C_partial(zeta)` over the opened OOD cells (extension elements) and binds it to the
/// committed `C_partial`.
pub struct LeafConstraintMirrorAir;

/// Back-compat alias (this module historically exported the single-lane C4 mirror as
/// `LeafC4MirrorAir`; it is now the full Poseidon2-free constraint mirror).
pub type LeafC4MirrorAir = LeafConstraintMirrorAir;

impl<F: Sync> BaseAir<F> for LeafConstraintMirrorAir {
    fn width(&self) -> usize {
        n_cols()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for LeafConstraintMirrorAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let block = |base: usize| -> [AB::Expr; D] { core::array::from_fn(|i| l(base + i)) };

        let cell = |c: usize| -> [AB::Expr; D] { block(cell_base(c)) };
        let alpha = block(L_ALPHA);

        // recompute the alpha-batched leaf C_partial over the opened cells (the reusable folder).
        let acc = recompute_c_partial::<AB, _>(&cell, &alpha);

        // ----- bind acc == committed C_partial (limb-for-limb) -----
        let c_partial = block(L_CPARTIAL);
        for i in 0..D {
            builder.assert_zero(c_partial[i].clone() - acc[i].clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::{Field, PrimeCharacteristicRing};
    use p3_matrix::dense::RowMajorMatrix;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }

    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    /// Honest C4 lane cells for a canonical value `v ∈ [0, q12289)`: mirrors air.rs `c4_witness`
    /// but lifts each base-field cell into the extension.
    fn c4_lane_cells(v: u32) -> [Challenge; C4_CELLS] {
        let d_lo = v % C4_BASE;
        let d_hi = v / C4_BASE;
        let slack = C4_MAX_HI - d_hi;
        let is96 = d_hi == C4_MAX_HI;
        let inv = if is96 {
            Val::ZERO
        } else {
            (Val::from_u32(d_hi) - Val::from_u32(C4_MAX_HI)).inverse()
        };
        let bits = |x: u32| -> [Challenge; C4_DIGIT_BITS] {
            core::array::from_fn(|k| ext((x >> k) & 1))
        };
        let mut cells = [Challenge::ZERO; C4_CELLS];
        cells[I_V] = ext(v);
        cells[I_DLO] = ext(d_lo);
        cells[I_DHI] = ext(d_hi);
        cells[I_LO_BITS..I_LO_BITS + 7].copy_from_slice(&bits(d_lo));
        cells[I_HI_BITS..I_HI_BITS + 7].copy_from_slice(&bits(d_hi));
        cells[I_SLACK_BITS..I_SLACK_BITS + 7].copy_from_slice(&bits(slack));
        cells[I_IS96] = if is96 { Challenge::ONE } else { Challenge::ZERO };
        cells[I_INV] = Challenge::from(inv);
        cells
    }

    /// A fully-honest opened-cell vector: 6 canonical C4 lanes, carry_acc = each lane's v,
    /// C6 field = carry_acc, C1 pad_seed == C2 pad_seed, relation_id = RELATION_ID. All residuals
    /// vanish (matches an on-domain leaf row); the OOD off-domain case is exercised separately.
    fn honest_cells(vals: [u32; C4_LANES], pad_seed_seed: u32) -> [Challenge; N_OPENED_CELLS] {
        let mut cells = [Challenge::ZERO; N_OPENED_CELLS];
        for lane in 0..C4_LANES {
            let lc = c4_lane_cells(vals[lane]);
            for w in 0..C4_CELLS {
                cells[c4_cell(lane, w)] = lc[w];
            }
            // carry_acc[lane] == v_lane (C4-acc binding).
            cells[acc_cell(lane)] = ext(vals[lane]);
            // SeamOpeningAcc: c6_field == carry_acc.
            cells[c6_field_cell(lane)] = ext(vals[lane]);
        }
        // SeamPadSeed: c1 == c2 (arbitrary byte values, equal).
        for b in 0..PAD_SEED_BYTES {
            let pb = ext(pad_seed_seed.wrapping_add(b as u32) % 256);
            cells[c1_pad_cell(b)] = pb;
            cells[c2_pad_cell(b)] = pb;
        }
        // relation_id == RELATION_ID.
        for j in 0..RELATION_ID_ELEMS {
            cells[relation_id_cell(j)] = ext(RELATION_ID[j]);
        }
        cells
    }

    /// Arbitrary OFF-domain opened cells (every cell a multi-limb extension element). The mirror
    /// recomputes C_partial != 0 and binds it — the DEEP-relevant shape.
    fn offdomain_cells(seed: u32) -> [Challenge; N_OPENED_CELLS] {
        core::array::from_fn(|c| ext_multi(seed.wrapping_mul(31).wrapping_add(c as u32)))
    }

    fn matrix(cells: &[Challenge; N_OPENED_CELLS], alpha: Challenge) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(pack_row(cells, alpha), n_cols())
    }

    /// On a fully-honest (on-domain) leaf row ALL mirrored leaf constraints hold ⇒ every residual
    /// is ZERO and honest `C_partial` is ZERO.
    #[test]
    fn honest_on_domain_has_zero_partial() {
        let cells = honest_cells([0, 1, 127, 128, 12288, 5000], 17);
        for r in 0..N_CONSTRAINTS {
            assert_eq!(
                constraint_residual(&cells, r),
                Challenge::ZERO,
                "honest on-domain row violates mirrored residual {r}"
            );
        }
        let cp = partial_constraint_eval(&cells, ext_multi(9));
        assert_eq!(cp, Challenge::ZERO, "honest on-domain row should give zero residual sum");
    }

    /// GOLDEN (positive): honest mirror rows ACCEPT via the real `check_all_constraints` — both
    /// on-domain honest cells (C_partial = 0) and OFF-domain arbitrary extension cells
    /// (C_partial != 0, the DEEP-relevant case).
    #[test]
    fn air_accepts_honest_rows() {
        // on-domain honest
        let cells = honest_cells([0, 127, 12288, 9999, 4096, 1], 42);
        let m = matrix(&cells, ext(31));
        assert!(
            check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
            "honest on-domain row rejected"
        );
        // off-domain arbitrary extension cells
        for seed in [1u32, 7, 42, 100] {
            let cells = offdomain_cells(seed);
            let m = matrix(&cells, ext_multi(seed + 1000));
            assert!(
                check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "honest off-domain row seed={seed} rejected"
            );
        }
    }

    /// FORGERY — a forged `C_partial` (≠ the recomputed alpha-batched residual) is rejected.
    #[test]
    fn air_rejects_forged_c_partial() {
        let cells = offdomain_cells(3);
        let mut m = matrix(&cells, ext_multi(77));
        m.values[L_CPARTIAL + 2] += Val::ONE; // tamper a C_partial limb
        assert!(!check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok());
    }

    /// FORGERY — tampering ANY opened cell that feeds SOME mirrored leaf constraint changes the
    /// recomputed `C_partial` ⇒ rejected. We tamper one representative cell of EACH mirrored class,
    /// across lanes, and require rejection. This is the soundness link: a trace cell that violates
    /// a mirrored leaf constraint is caught at the OOD point.
    #[test]
    fn air_rejects_tampered_opened_cell() {
        let cells = offdomain_cells(50);
        let alpha = ext_multi(123);
        let mut targets: Vec<usize> = Vec::new();
        // C4: a class of cell per lane (v, d_lo, d_hi, a bit, is96, inv).
        for lane in 0..C4_LANES {
            targets.push(c4_cell(lane, I_V));
            targets.push(c4_cell(lane, I_DLO));
            targets.push(c4_cell(lane, I_DHI));
            targets.push(c4_cell(lane, I_LO_BITS + (lane % 7)));
            targets.push(c4_cell(lane, I_IS96));
            targets.push(c4_cell(lane, I_INV));
        }
        // acc / seam / relation_id cells.
        for j in 0..N_LANES {
            targets.push(acc_cell(j));
            targets.push(c6_field_cell(j));
        }
        for b in 0..PAD_SEED_BYTES {
            targets.push(c1_pad_cell(b));
            targets.push(c2_pad_cell(b));
        }
        for j in 0..RELATION_ID_ELEMS {
            targets.push(relation_id_cell(j));
        }
        for &cell_idx in &targets {
            let mut m = matrix(&cells, alpha);
            m.values[cell_base(cell_idx)] += Val::ONE; // tamper limb 0 of the cell
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampering opened cell {cell_idx} was NOT rejected — that residual is underconstrained"
            );
        }
    }

    /// FORGERY — tampering `alpha` (the batching challenge) with `C_partial` honest ⇒ rejected.
    #[test]
    fn air_rejects_tampered_alpha() {
        let cells = offdomain_cells(200);
        let mut m = matrix(&cells, ext_multi(9));
        m.values[L_ALPHA + 1] += Val::ONE; // tamper an alpha limb (C_partial unchanged)
        assert!(!check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok());
    }

    /// FIDELITY PIN (C4) — the per-constraint C4 residuals from THIS module's native reference
    /// MUST match the leaf AIR's own `eval_c4_residue` semantics (air.rs:1381): on a canonical
    /// lane all 29 residuals are zero, and single deliberate corruptions make EXACTLY the expected
    /// residual nonzero.
    #[test]
    fn mirror_matches_leaf_air_c4() {
        let cells = honest_cells([7000, 0, 12288, 128, 5000, 96], 1);
        // canonical ⇒ all C4 residuals zero (per lane).
        for lane in 0..C4_LANES {
            for i in 0..C4_N_PER_LANE {
                assert_eq!(
                    c4_lane_residual(&cells, lane, i),
                    Challenge::ZERO,
                    "canonical lane {lane} violates mirrored C4 residual {i}"
                );
            }
        }
        // Corrupt v on lane 2 ⇒ v-recompose residual (28) nonzero; lo-recompose (21) still zero.
        let mut bad = cells;
        bad[c4_cell(2, I_V)] += Challenge::ONE;
        assert_ne!(c4_lane_residual(&bad, 2, 28), Challenge::ZERO, "v-recompose blind to v");
        assert_eq!(c4_lane_residual(&bad, 2, 21), Challenge::ZERO, "lo-recompose should not read v");
        // and OTHER lanes unaffected.
        assert_eq!(c4_lane_residual(&bad, 0, 28), Challenge::ZERO, "lane bleed");

        // is96 inverse-corner (air.rs:1416): lane with d_hi=96 (v=12288), flip is96 -> residual 26.
        let mut corner = cells;
        // lane 2 has v=12288 ⇒ d_hi=96, is96=1.
        corner[c4_cell(2, I_IS96)] = Challenge::ZERO;
        assert_ne!(
            c4_lane_residual(&corner, 2, 26),
            Challenge::ZERO,
            "is96 inverse-corner residual blind to is96 flip — soundness hole vs air.rs:1416"
        );
    }

    /// FIDELITY PIN (seams + relation_id) — the algebraic residuals match air.rs:
    ///   * SeamPadSeed (765-771): zero iff c1_pad == c2_pad; nonzero on a single mismatch.
    ///   * SeamOpeningAcc (773-779): zero iff carry_acc == c6_field; nonzero on a single mismatch.
    ///   * C4-acc (762): zero iff carry_acc == v; nonzero on a single mismatch.
    ///   * relation_id (957-967): zero iff relation_id == RELATION_ID; nonzero on any deviation.
    #[test]
    fn mirror_matches_leaf_air_seams_and_relation_id() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 99);
        let c4_total = C4_LANES * C4_N_PER_LANE;
        // honest: all non-C4 residuals zero.
        for r in c4_total..N_CONSTRAINTS {
            assert_eq!(constraint_residual(&cells, r), Challenge::ZERO, "honest residual {r} nonzero");
        }
        // C4-acc: corrupt carry_acc[3] ⇒ both the acc-binding residual AND SeamOpeningAcc[3] react.
        let mut bad_acc = cells;
        bad_acc[acc_cell(3)] += Challenge::ONE;
        assert_ne!(
            constraint_residual(&bad_acc, c4_total + 3),
            Challenge::ZERO,
            "C4-acc binding (air.rs:762) blind to carry_acc"
        );
        // SeamPadSeed: corrupt c2_pad_seed[10] ⇒ residual c4_total+N_C4_ACC+10 nonzero.
        let mut bad_pad = cells;
        bad_pad[c2_pad_cell(10)] += Challenge::ONE;
        assert_ne!(
            constraint_residual(&bad_pad, c4_total + N_C4_ACC + 10),
            Challenge::ZERO,
            "SeamPadSeed (air.rs:765) blind to c2 pad_seed"
        );
        // SeamOpeningAcc: corrupt c6_field[4] ⇒ residual c4_total+N_C4_ACC+N_SEAM_PAD+4 nonzero.
        let mut bad_c6 = cells;
        bad_c6[c6_field_cell(4)] += Challenge::ONE;
        assert_ne!(
            constraint_residual(&bad_c6, c4_total + N_C4_ACC + N_SEAM_PAD + 4),
            Challenge::ZERO,
            "SeamOpeningAcc (air.rs:773) blind to c6 field lane"
        );
        // relation_id: corrupt relation_id[1] ⇒ residual at the relation-id base nonzero.
        let mut bad_rid = cells;
        bad_rid[relation_id_cell(1)] += Challenge::ONE;
        let rid_base = c4_total + N_C4_ACC + N_SEAM_PAD + N_SEAM_ACC;
        assert_ne!(
            constraint_residual(&bad_rid, rid_base + 1),
            Challenge::ZERO,
            "relation_id pinning (air.rs:957) blind to relation_id"
        );
    }

    /// The AIR's in-circuit recomputation agrees with the native `partial_constraint_eval`.
    #[test]
    fn air_agrees_with_native_partial_eval() {
        for seed in [11u32, 22, 33] {
            let cells = offdomain_cells(seed);
            let alpha = ext_multi(seed.wrapping_add(500));
            let m = matrix(&cells, alpha);
            assert!(
                check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "AIR recomputation disagrees with native partial eval (seed={seed})"
            );
            assert_eq!(row_c_partial(m.values.as_slice()), partial_constraint_eval(&cells, alpha));
        }
    }

    /// A multi-row trace of independent honest mirror checks (one per row) accepts.
    #[test]
    fn air_accepts_multirow() {
        let mut flat = Vec::new();
        for r in 0..6u32 {
            let cells = offdomain_cells(900 + r * 31);
            flat.extend_from_slice(&pack_row(&cells, ext_multi(r + 1)));
        }
        let m = RowMajorMatrix::new(flat, n_cols());
        assert!(check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok());
    }
}
