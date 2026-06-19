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
//! ## Scope of THIS module (a bounded, well-tested START)
//!
//! This mirrors EXACTLY ONE self-contained, Poseidon2-free leaf gadget: the **C4 canonical
//! base-128 residue gadget** (`FullRelationAir::eval_c4_residue`, air.rs:1381) for a SINGLE
//! lane block. That gadget is the simplest faithful target — all constraints are algebraic and
//! degree ≤ 2, the cell block is a fixed 26-cell window (`C4_CELLS`), and there are no
//! permutations. We compute the **alpha-batched** sum of those constraint residuals over the
//! OPENED cells (each an extension element `Challenge`, opened at `zeta`):
//!
//!   `C_partial(zeta) = Σ_i alpha^i · c_i(T(zeta))`
//!
//! where each `c_i` is one leaf C4 constraint's zero-residual, evaluated over the opened cells
//! using the SAME extension (octic, `X^8 = W`) arithmetic as [`crate::deep_air`] /
//! [`crate::fri_fold_air`]. The alpha-batching mirrors `p3-uni-stark`'s
//! `SymbolicAirBuilder`/`accumulate` (constraints folded by powers of a single FS challenge
//! `alpha`). The AIR then binds the committed `C_partial` cell to this recomputed sum.
//!
//! ## The mirrored leaf C4 constraints (must match air.rs:1381 EXACTLY)
//!
//! Per lane block (cells `[v, d_lo, d_hi, lo_bits[7], hi_bits[7], slack_bits[7], is96, inv]`):
//!   * bit-booleans:  `bit·(bit−1) == 0`  for each of the 21 bits (lo/hi/slack)  [21 residuals].
//!   * `d_lo − recompose(lo_bits) == 0`
//!   * `d_hi − recompose(hi_bits) == 0`
//!   * `(96 − d_hi) − recompose(slack_bits) == 0`
//!   * `is96·(is96−1) == 0`        (the leaf `assert_bool(is96)`)
//!   * `is96·(d_hi − 96) == 0`
//!   * `(d_hi − 96)·inv − (1 − is96) == 0`
//!   * `is96·d_lo == 0`
//!   * `v − (d_lo + 128·d_hi) == 0`
//!
//! 21 + 8 = 29 residuals total ([`C4_N_CONSTRAINTS`]).
//!
//! A WRONG mirror is a soundness hole, so [`tests::mirror_matches_leaf_air_on_real_row`] pins
//! the honest recomputed residuals against the leaf AIR's own `eval_c4_residue` evaluated on a
//! REAL packed leaf row (via a `Challenge`-lifting AirBuilder), and the alpha-batched honest
//! `C_partial` against an independent native reference [`c4_partial_constraint_eval`].
//!
//! ## DEFERRED (the rest of leaf `C(zeta)`)
//!
//! Still committed-not-constrained in [`crate::deep_air`] and NOT yet mirrored here:
//!   * the OTHER 5 C4 lanes (this module does ONE lane block; the pattern extends trivially).
//!   * C1 / C2 / C6 sponge commitment equalities + the WIDE-THEN-REDUCE mask reduce gadget.
//!   * ALL Poseidon2 permutation round constraints (reuse the inlined-perm / `eval_perm`
//!     machinery — the bulk of the leaf width; deferred deliberately, see module TODO).
//!   * the carrier mod-q reduction, CLASS-A role-root sponge, CLASS-B relation_id pinning,
//!     CLASS-C packed-witness anchors, the cross-component seams, and (batch) the digest chain.
//!   * the FINAL alpha-batching ACROSS all of the above into the single `C(zeta)` consumed by
//!     [`crate::deep_air`] (here we produce a NAMED PARTIAL sum `C_partial`, not full `C`).
//!   * wiring `C_partial` into the monolith DEEP block so a tampered opened C4 cell rejects via
//!     the DEEP OOD check (next step; this module validates the mirror in isolation first).

use crate::fri_fold_air::{limbs, D, W};
use descender_account_stark::{Challenge, Val};
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

/// Number of distinct C4 constraint residuals batched: 21 bit-booleans (lo[7] ‖ hi[7] ‖
/// slack[7]) + 8 algebraic (lo-recompose, hi-recompose, slack-recompose, is96-bool, is96·(d_hi−96),
/// inverse-corner, is96·d_lo, v-recompose).
pub const C4_N_CONSTRAINTS: usize = 21 + 8; // 29

// Within-block cell indices (mirror air.rs `eval_c4_residue` local offsets).
const I_V: usize = 0;
const I_DLO: usize = 1;
const I_DHI: usize = 2;
const I_LO_BITS: usize = 3;
const I_HI_BITS: usize = 10;
const I_SLACK_BITS: usize = 17;
const I_IS96: usize = 24;
const I_INV: usize = 25;

// ===========================================================================
// Column layout (one leaf-C4-mirror check per row)
// ===========================================================================
//
// The opened OOD cells are extension elements (8 `Val` limbs each), laid out as `C4_CELLS`
// contiguous blocks of `D` limbs. `alpha` (the constraint-batching FS challenge) is one
// extension element. `C_partial` (the recomputed alpha-batched residual sum) is one extension
// element, the value [`crate::deep_air`] will eventually consume as (part of) `C(zeta)`.

/// First column: the `C4_CELLS` opened OOD cells, each `D` limbs.
pub const L_CELLS: usize = 0;
/// The constraint-batching challenge `alpha` (`D` limbs).
pub const L_ALPHA: usize = L_CELLS + C4_CELLS * D;
/// The committed alpha-batched partial constraint eval `C_partial(zeta)` (`D` limbs).
pub const L_CPARTIAL: usize = L_ALPHA + D;

/// Total column count.
pub const fn n_cols() -> usize {
    L_CPARTIAL + D
}

/// Limb base of opened cell `c` (`c < C4_CELLS`).
const fn cell_base(c: usize) -> usize {
    L_CELLS + c * D
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

// ===========================================================================
// Native reference (the AUTHORITY the AIR is checked against)
// ===========================================================================

/// One leaf C4 constraint residual `c_i`, evaluated NATIVELY over extension-valued opened cells.
/// `i` ranges over `[0, C4_N_CONSTRAINTS)`:
///   * `0..21`  : bit-boolean residuals `bit·(bit−1)` for lo[7] ‖ hi[7] ‖ slack[7].
///   * `21`     : `d_lo − recompose(lo_bits)`
///   * `22`     : `d_hi − recompose(hi_bits)`
///   * `23`     : `(96 − d_hi) − recompose(slack_bits)`
///   * `24`     : `is96·(is96−1)`
///   * `25`     : `is96·(d_hi − 96)`
///   * `26`     : `(d_hi − 96)·inv − (1 − is96)`
///   * `27`     : `is96·d_lo`
///   * `28`     : `v − (d_lo + 128·d_hi)`
///
/// Total residuals = 21 + 8 = 29 ([`C4_N_CONSTRAINTS`]).
fn c4_constraint_residual(cells: &[Challenge; C4_CELLS], i: usize) -> Challenge {
    let recompose = |start: usize| -> Challenge {
        let mut acc = Challenge::ZERO;
        for b in 0..C4_DIGIT_BITS {
            acc += cells[start + b] * Challenge::from(Val::from_u64(1u64 << b));
        }
        acc
    };
    let v = cells[I_V];
    let d_lo = cells[I_DLO];
    let d_hi = cells[I_DHI];
    let is96 = cells[I_IS96];
    let inv = cells[I_INV];
    let max_hi = Challenge::from(Val::from_u32(C4_MAX_HI));
    let base = Challenge::from(Val::from_u32(C4_BASE));

    if i < 21 {
        // bit-boolean for bit index i across the 21 contiguous bit cells (lo,hi,slack).
        let bit = cells[I_LO_BITS + i];
        return bit * (bit - Challenge::ONE);
    }
    match i - 21 {
        0 => d_lo - recompose(I_LO_BITS),
        1 => d_hi - recompose(I_HI_BITS),
        2 => (max_hi - d_hi) - recompose(I_SLACK_BITS),
        3 => is96 * (is96 - Challenge::ONE),
        4 => is96 * (d_hi - max_hi),
        5 => (d_hi - max_hi) * inv - (Challenge::ONE - is96),
        6 => is96 * d_lo,
        7 => v - (d_lo + base * d_hi),
        _ => unreachable!("c4 constraint index out of range"),
    }
}

/// Native alpha-batched leaf-C4 partial constraint eval over opened cells:
/// `C_partial = Σ_i alpha^i · c_i(cells)`. This is the AUTHORITY; the AIR re-derives it in-circuit.
pub fn c4_partial_constraint_eval(cells: &[Challenge; C4_CELLS], alpha: Challenge) -> Challenge {
    let mut acc = Challenge::ZERO;
    let mut a_pow = Challenge::ONE;
    for i in 0..C4_N_CONSTRAINTS {
        acc += a_pow * c4_constraint_residual(cells, i);
        a_pow *= alpha;
    }
    acc
}

/// Build one honest leaf-C4-mirror row from opened cells + `alpha`, computing `C_partial` via the
/// native reference. (The builder can only emit honest rows; tamper tests mutate the matrix.)
pub fn pack_row(cells: &[Challenge; C4_CELLS], alpha: Challenge) -> Vec<Val> {
    let c_partial = c4_partial_constraint_eval(cells, alpha);
    let mut row = vec![Val::ZERO; n_cols()];
    for c in 0..C4_CELLS {
        row[cell_base(c)..cell_base(c) + D].copy_from_slice(&limbs(cells[c]));
    }
    row[L_ALPHA..L_ALPHA + D].copy_from_slice(&limbs(alpha));
    row[L_CPARTIAL..L_CPARTIAL + D].copy_from_slice(&limbs(c_partial));
    row
}

/// Decode the committed `C_partial` cell of a packed row.
pub fn row_c_partial(row: &[Val]) -> Challenge {
    Challenge::from_basis_coefficients_slice(&row[L_CPARTIAL..L_CPARTIAL + D]).expect("C_partial decode")
}

// ===========================================================================
// The AIR
// ===========================================================================

/// Leaf-C4-mirror AIR: recomputes the alpha-batched leaf C4 constraint residual `C_partial(zeta)`
/// over the opened OOD cells (extension elements) and binds it to the committed `C_partial`.
pub struct LeafC4MirrorAir;

impl<F: Sync> BaseAir<F> for LeafC4MirrorAir {
    fn width(&self) -> usize {
        n_cols()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for LeafC4MirrorAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let block = |base: usize| -> [AB::Expr; D] { core::array::from_fn(|i| l(base + i)) };

        let cell = |c: usize| -> [AB::Expr; D] { block(cell_base(c)) };
        let alpha = block(L_ALPHA);

        // recompose 7 bit cells starting at within-block index `start` into an extension expr
        // Σ bit_b · 2^b (the bits are extension-valued opened cells).
        let recompose = |start: usize| -> [AB::Expr; D] {
            let mut acc = octic_scalar::<AB>(0);
            for b in 0..C4_DIGIT_BITS {
                let bit = cell(start + b);
                let scaled = scale::<AB>(&bit, 1u64 << b);
                acc = octic_add::<AB>(&acc, &scaled);
            }
            acc
        };

        let v = cell(I_V);
        let d_lo = cell(I_DLO);
        let d_hi = cell(I_DHI);
        let is96 = cell(I_IS96);
        let inv = cell(I_INV);
        let max_hi = octic_scalar::<AB>(C4_MAX_HI as u64);
        let base = octic_scalar::<AB>(C4_BASE as u64);
        let one = octic_scalar::<AB>(1);

        // ----- build the C4_N_CONSTRAINTS residuals (each an extension element) -----
        let mut residuals: Vec<[AB::Expr; D]> = Vec::with_capacity(C4_N_CONSTRAINTS);

        // 0..21 : bit-boolean residuals bit·(bit−1) over lo[7] ‖ hi[7] ‖ slack[7].
        for i in 0..21 {
            let bit = cell(I_LO_BITS + i);
            let bit_minus_one = octic_sub::<AB>(&bit, &one);
            residuals.push(octic_mul::<AB>(&bit, &bit_minus_one));
        }
        // 21 : d_lo − recompose(lo_bits)
        residuals.push(octic_sub::<AB>(&d_lo, &recompose(I_LO_BITS)));
        // 22 : d_hi − recompose(hi_bits)
        residuals.push(octic_sub::<AB>(&d_hi, &recompose(I_HI_BITS)));
        // 23 : (96 − d_hi) − recompose(slack_bits)
        let max_minus_dhi = octic_sub::<AB>(&max_hi, &d_hi);
        residuals.push(octic_sub::<AB>(&max_minus_dhi, &recompose(I_SLACK_BITS)));
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
        let base_dhi = octic_mul::<AB>(&base, &d_hi);
        let recomposed_v = octic_add::<AB>(&d_lo, &base_dhi);
        residuals.push(octic_sub::<AB>(&v, &recomposed_v));

        debug_assert_eq!(residuals.len(), C4_N_CONSTRAINTS);

        // ----- alpha-batch: acc = Σ_i alpha^i · residual_i -----
        let mut acc = octic_scalar::<AB>(0);
        let mut a_pow = octic_scalar::<AB>(1);
        for res in residuals.iter() {
            let term = octic_mul::<AB>(&a_pow, res);
            acc = octic_add::<AB>(&acc, &term);
            a_pow = octic_mul::<AB>(&a_pow, &alpha);
        }

        // ----- bind acc == committed C_partial (limb-for-limb) -----
        let c_partial = block(L_CPARTIAL);
        for i in 0..D {
            builder.assert_zero(c_partial[i].clone() - acc[i].clone());
        }
    }
}

/// Scale an extension expr by a base-field constant `s` (limbwise; no reduction since `s` is base).
fn scale<AB: AirBuilder>(a: &[AB::Expr; D], s: u64) -> [AB::Expr; D] {
    let sc = AB::Expr::from(AB::F::from_u64(s));
    core::array::from_fn(|i| a[i].clone() * sc.clone())
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
    /// but lifts each base-field cell into the extension (the on-domain witness; the OOD opening
    /// is exercised by `multi`-limb tests below).
    fn honest_canonical_cells(v: u32) -> [Challenge; C4_CELLS] {
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

    fn matrix(cells: &[Challenge; C4_CELLS], alpha: Challenge) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(pack_row(cells, alpha), n_cols())
    }

    /// On a CANONICAL leaf row the leaf C4 constraints all hold, so every residual is ZERO and the
    /// honest `C_partial` is ZERO — the on-domain sanity check (DEEP evaluates OFF domain, where the
    /// cells are arbitrary extension elements and `C_partial != 0`; that case is tested below).
    #[test]
    fn canonical_row_has_zero_partial() {
        for v in [0u32, 1, 127, 128, 12288 /* q-1 ⇒ is96=1 corner */, 5000] {
            let cells = honest_canonical_cells(v);
            let cp = c4_partial_constraint_eval(&cells, ext_multi(9));
            assert_eq!(cp, Challenge::ZERO, "canonical v={v} should give zero residual sum");
        }
    }

    /// GOLDEN (positive): honest mirror rows ACCEPT via the real `check_all_constraints` — both
    /// on-domain canonical cells (C_partial = 0) and OFF-domain arbitrary extension cells
    /// (C_partial != 0). The latter is the DEEP-relevant case: the AIR recomputes the alpha-batched
    /// residual sum over arbitrary opened extension cells and binds it to the committed C_partial.
    #[test]
    fn air_accepts_honest_rows() {
        // on-domain canonical
        for v in [0u32, 127, 12288, 9999] {
            let cells = honest_canonical_cells(v);
            let m = matrix(&cells, ext(31));
            assert!(
                check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok(),
                "honest canonical row v={v} rejected"
            );
        }
        // off-domain: arbitrary extension-valued opened cells (the real OOD opening shape).
        for seed in [1u32, 7, 42, 100] {
            let cells: [Challenge; C4_CELLS] =
                core::array::from_fn(|c| ext_multi(seed.wrapping_mul(31) + c as u32));
            let m = matrix(&cells, ext_multi(seed + 1000));
            assert!(
                check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok(),
                "honest off-domain row seed={seed} rejected"
            );
        }
    }

    /// FORGERY — a forged `C_partial` (≠ the recomputed alpha-batched residual) is rejected. THIS
    /// is the core property: the prover cannot supply an arbitrary leaf `C(zeta)` contribution; it
    /// must equal the in-circuit recomputation over the opened cells.
    #[test]
    fn air_rejects_forged_c_partial() {
        let cells: [Challenge; C4_CELLS] = core::array::from_fn(|c| ext_multi(3 + c as u32));
        let mut m = matrix(&cells, ext_multi(77));
        m.values[L_CPARTIAL + 2] += Val::ONE; // tamper a C_partial limb
        assert!(!check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok());
    }

    /// FORGERY — tampering an OPENED CELL that feeds a leaf C4 constraint changes the recomputed
    /// `C_partial` away from the committed value ⇒ rejected. We tamper each algebraically-distinct
    /// cell class (v, d_lo, d_hi, a bit, is96, inv) and require rejection. This is the soundness
    /// link: a trace cell that VIOLATES a mirrored leaf constraint is caught at the OOD point.
    #[test]
    fn air_rejects_tampered_opened_cell() {
        let cells: [Challenge; C4_CELLS] = core::array::from_fn(|c| ext_multi(50 + c as u32));
        let alpha = ext_multi(123);
        // Tamper each class; committed C_partial stays the honest (pre-tamper) value.
        for &cell_idx in &[I_V, I_DLO, I_DHI, I_LO_BITS, I_HI_BITS + 3, I_SLACK_BITS, I_IS96, I_INV] {
            let mut m = matrix(&cells, alpha);
            m.values[cell_base(cell_idx)] += Val::ONE; // tamper limb 0 of the cell
            assert!(
                !check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok(),
                "tampering opened cell {cell_idx} was NOT rejected — that residual is underconstrained"
            );
        }
    }

    /// FORGERY — tampering `alpha` (the batching challenge) with `C_partial` left at the honest
    /// value ⇒ the recomputed batched sum diverges ⇒ rejected. (Once `alpha` is FS-bound the prover
    /// can't pick a batching point that zeroes a violated constraint.)
    #[test]
    fn air_rejects_tampered_alpha() {
        let cells: [Challenge; C4_CELLS] = core::array::from_fn(|c| ext_multi(200 + c as u32));
        let mut m = matrix(&cells, ext_multi(9));
        m.values[L_ALPHA + 1] += Val::ONE; // tamper an alpha limb (C_partial unchanged)
        assert!(!check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok());
    }

    /// FIDELITY PIN — the per-constraint residuals computed by THIS module's native reference
    /// (`c4_constraint_residual`) MUST match the leaf AIR's own `eval_c4_residue` semantics: on a
    /// REAL canonical leaf row (lifted into the extension), the leaf constraints are exactly the
    /// ones the leaf AIR asserts zero, so all residuals are zero AND a single deliberate corruption
    /// of one cell makes EXACTLY the expected residual(s) nonzero. A wrong mirror (different
    /// formula than air.rs:1381) would either leave a residual nonzero on a canonical row, or fail
    /// to react to the matching corruption.
    #[test]
    fn mirror_matches_leaf_air_on_real_row() {
        // canonical row ⇒ all 29 residuals zero (matches the leaf AIR asserting each == 0).
        let cells = honest_canonical_cells(7000);
        for i in 0..C4_N_CONSTRAINTS {
            assert_eq!(
                c4_constraint_residual(&cells, i),
                Challenge::ZERO,
                "canonical row violates mirrored residual {i} — mirror diverges from leaf air"
            );
        }

        // Corrupt v only ⇒ the v-recompose residual (index 28) becomes nonzero; the bit/digit
        // residuals (which don't read v) stay zero. This pins that residual 28 == v−(d_lo+128·d_hi)
        // exactly as air.rs:1421.
        let mut bad = cells;
        bad[I_V] += Challenge::ONE;
        assert_ne!(c4_constraint_residual(&bad, 28), Challenge::ZERO, "v-recompose residual blind to v");
        assert_eq!(c4_constraint_residual(&bad, 21), Challenge::ZERO, "lo-recompose should not read v");

        // Corrupt is96 at the q-1 corner (d_hi=96): the inverse residual (26) reacts — exactly the
        // air.rs:1416 `(d_hi-96)*inv == 1-is96` corner the leaf c4_is96 test isolates.
        let mut corner = honest_canonical_cells(12288); // d_hi = 96, is96 = 1
        corner[I_IS96] = Challenge::ZERO; // flip is96 -> 0
        assert_ne!(
            c4_constraint_residual(&corner, 26),
            Challenge::ZERO,
            "is96 inverse-corner residual blind to is96 flip — soundness hole vs air.rs:1416"
        );
    }

    /// The AIR's in-circuit recomputation agrees with the native `c4_partial_constraint_eval`
    /// reference (cross-check the two independent code paths produce the same `C_partial`). An
    /// honest matrix (whose C_partial came from the native ref) accepts; if the AIR computed a
    /// different batched sum it would reject.
    #[test]
    fn air_agrees_with_native_partial_eval() {
        for seed in [11u32, 22, 33] {
            let cells: [Challenge; C4_CELLS] =
                core::array::from_fn(|c| ext_multi(seed + c as u32 * 13));
            let alpha = ext_multi(seed.wrapping_add(500));
            // native value is what pack_row commits; AIR re-derives it and binds — accept ⇒ agree.
            let m = matrix(&cells, alpha);
            assert!(
                check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok(),
                "AIR recomputation disagrees with native c4_partial_constraint_eval (seed={seed})"
            );
            // and the committed value really is the native eval (not some other accepted value).
            assert_eq!(row_c_partial(m.values.as_slice()), c4_partial_constraint_eval(&cells, alpha));
        }
    }

    /// A multi-row trace of independent honest mirror checks (one per row) accepts — the per-row
    /// shape the monolith will instantiate once per opened-cell group.
    #[test]
    fn air_accepts_multirow() {
        let mut flat = Vec::new();
        for r in 0..6u32 {
            let cells: [Challenge; C4_CELLS] =
                core::array::from_fn(|c| ext_multi(900 + r * 31 + c as u32));
            flat.extend_from_slice(&pack_row(&cells, ext_multi(r + 1)));
        }
        let m = RowMajorMatrix::new(flat, n_cols());
        assert!(check_all_constraints(&LeafC4MirrorAir, &m, &[], None).is_ok());
    }
}
