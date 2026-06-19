//! `recursive_air` (BEGIN — the capstone monolithic verifier-AIR, Option A).
//!
//! Per the composition decision (`research/path2_recursion_air_composition_architecture.md`,
//! Option A) the recursion is ONE wide monolithic AIR over a shared column layout that INLINES
//! every gadget's constraints and — crucially — wires the CROSS-GADGET BUSES that the standalone
//! gadget AIRs leave as free columns. The standalone gadgets each validated their own constraints
//! ([`crate::challenger`], [`crate::merkle_air`], [`crate::fri_fold_air`], [`crate::fri_chain_air`],
//! [`crate::deep_air`], [`crate::public_io_air`]) but in isolation a malicious prover can still
//! choose, independently per gadget: which query index to answer ([`crate::index_bus`] note), and
//! what fold point `s` / challenge `beta` to fold at. The monolith closes those by binding the
//! gadgets' free columns to gadget-1's Fiat–Shamir exports OVER ONE WINDOW.
//!
//! ## What this BOUNDED start wires + tests (NOT the full monolith)
//!
//! This first slab assembles TWO consumers around the index/challenge bus in ONE AIR window,
//! one window per query, fully cross-constrained with a tamper test per binding:
//!
//!   1. **index ↔ challenger draw** (reuse [`crate::index_bus`]): the query index is the low
//!      `log_height` bits of gadget-1's `SampleDraw` export (`draw_out`), via the bit
//!      decomposition + canonical recomposition. A forged (favorable) index is rejected.
//!   2. **fold point `s` ↔ index** (NEW in-AIR binding, the core new soundness content): the FRI
//!      fold point must be `s = fold_point(log_height, index) = g^{rev_bits(index)}` with
//!      `g = two_adic_generator(log_height+1)`. That exponentiation is realized as a COMMITTED
//!      product tower over the index bits (degree-2 steps, the same "commit the tower" trick as
//!      [`crate::deep_air`]'s squaring tower), so `s` is pinned to the index — NOT a free column.
//!      A prover cannot fold at a favorable point.
//!   3. **FRI fold identity** (reuse [`crate::fri_fold_air`]): the cleared fold identity (★)
//!      `folded·2s == e0(beta+s) − e1(beta−s)` over the deg-8 extension, with `s` the BUS-bound
//!      point from (2) and `beta` a committed challenge column (bound to gadget-1 in the full
//!      monolith; see remaining-work below). A tampered sibling eval is rejected.
//!
//! So one window binds: `low_bits(challenger.draw) == index == reverse-exp of fold point s`, and
//! the fold of the opened pair happens at THAT `s`. Forging any link (the index, the point, or an
//! eval) is rejected — soundness by attempted forgery, never by inspection.
//!
//! ## Remaining assembly toward the full monolith (documented, NOT yet wired)
//!
//! - **merkle index ↔ same bus** (binding (i) second consumer): bind [`crate::merkle_air`]'s
//!   per-level `C_INDEX_BIT` to `bit_j` of THIS window's decomposition, so the Merkle opening and
//!   the FRI fold answer the SAME FS-forced query. (The point tower already consumes the bits; the
//!   Merkle path would consume the same `bit_j` columns level-by-level.) Tamper: a Merkle path at
//!   a different index than the fold point is rejected.
//! - **beta ↔ gadget-1 SampleDraw** (binding (ii)): bind each FRI-round `C_BETA` (and the DEEP
//!   `C_ZETA`) to the corresponding gadget-1 `draw_out` exports (the round challenges sampled from
//!   the transcript), so the prover cannot substitute a favorable challenge. Here `beta` is still a
//!   free committed column; the binding is a per-round equality to the challenger block's draw cell.
//! - **multi-round descent**: replace the single fold row with [`crate::fri_chain_air`]'s
//!   chaining (point-square + chain-link) so one query folds through every commit-phase round in
//!   the same window, with `s_0` bound by the tower and `s_{r+1} == s_r²`.
//! - **inline the challenger sponge + Poseidon2 perms**: fold [`crate::challenger`]'s sponge-wiring
//!   constraints and [`crate::inlined_perm::eval_perm`] (reused for EVERY perm instance) into the
//!   monolith so the `draw_out` exports are themselves the constrained output of the in-AIR duplex
//!   sponge (today the bus takes `draw_out` as a committed column; the full monolith derives it).
//! - **DEEP + public-IO**: add [`crate::deep_air`] (OOD quotient at the bus-bound `zeta`) and
//!   [`crate::public_io_air`] (statement preservation) blocks to the same window.
//!
//! Every binding above lands with its own tamper-reject test before it is considered wired (the
//! prompt's rigor gate: no underconstrained AIR). This module currently wires (1)–(3).

use crate::fri_fold::{fold_pair, fold_point};
use crate::fri_fold_air::{limbs, D, W};
use crate::index_bus::{assert_canonical_pub, FIELD_BITS};
use descender_account_stark::{Challenge, Val};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField32, TwoAdicField};

extern crate alloc;
use alloc::vec::Vec;

// ----- shared monolithic column layout (one query window per row) -----
//
// Index-bus block (gadget-1 export + decomposition):
/// Gadget-1 `SampleDraw` export: the base-field element this query index reduces.
pub const C_DRAW_OUT: usize = 0;
/// Full canonical bit decomposition of `draw_out` (LSB first), `FIELD_BITS` columns.
pub const C_BITS: usize = C_DRAW_OUT + 1;
/// The bus index value (`Σ_{i<log_height} bit_i·2^i`) the consumers read.
pub const C_INDEX: usize = C_BITS + FIELD_BITS;
// FRI-fold block (gadget-3 consumer):
/// First sibling eval `e0 = f(s)` (8 limbs).
pub const C_E0: usize = C_INDEX + 1;
/// Second sibling eval `e1 = f(-s)` (8 limbs).
pub const C_E1: usize = C_E0 + D;
/// Round fold challenge `beta` (8 limbs).
pub const C_BETA: usize = C_E1 + D;
/// The fold point `s` — BUS-BOUND to the index by the product tower (not free).
pub const C_S: usize = C_BETA + D;
/// The claimed folded eval (8 limbs).
pub const C_FOLDED: usize = C_S + 1;
/// Fold-point product tower `sp[0..=log_height]`: `sp[0]=1`, `sp[j+1]=sp[j]·sel_j`,
/// `sp[log_height]==s`. `(log_height+1)` columns. (The committed-tower trick; see module doc.)
pub const C_SP: usize = C_FOLDED + D;

/// Column count for a `recursive_air` window with FRI layer log-height `log_height`.
pub const fn n_cols(log_height: usize) -> usize {
    C_SP + (log_height + 1)
}

/// The `j`-th tower column (`sp[j]`).
const fn sp_col(j: usize) -> usize {
    C_SP + j
}

/// The fixed tower base constants `base_j = g^{2^j}` for `g = two_adic_generator(log_height+1)`,
/// `j = 0..log_height`. The product `∏_j base_j^{rev_bit_j}` equals `fold_point(log_height,index)`.
/// `rev_bit_j` is the index bit at position `log_height-1-j` (bit-reversal of the exponent).
fn tower_bases(log_height: usize) -> Vec<Val> {
    let g = Val::two_adic_generator(log_height + 1);
    let mut bases = Vec::with_capacity(log_height);
    let mut p = g; // base_0 = g^{2^0} = g
    for _ in 0..log_height {
        bases.push(p);
        p = p * p; // base_{j+1} = base_j² = g^{2^{j+1}}
    }
    bases
}

/// Build one query window: the FS-sampled `draw_out`, sibling evals `(e0,e1)`, round challenge
/// `beta`, at FRI layer `log_height`. The index, fold point, tower, and folded result are all
/// DERIVED honestly (the authority the AIR is checked against); tamper tests mutate the matrix.
pub fn pack_window(
    draw_out: Val,
    e0: Challenge,
    e1: Challenge,
    beta: Challenge,
    log_height: usize,
) -> Vec<Val> {
    let canon = draw_out.as_canonical_u32();
    let index = (canon & ((1u32 << log_height) - 1)) as usize;
    let s = fold_point(log_height, index);
    let folded = fold_pair(e0, e1, beta, s);

    let mut row = vec![Val::ZERO; n_cols(log_height)];
    row[C_DRAW_OUT] = draw_out;
    for i in 0..FIELD_BITS {
        row[C_BITS + i] = Val::from_u32((canon >> i) & 1);
    }
    row[C_INDEX] = Val::from_u32(index as u32);
    row[C_E0..C_E0 + D].copy_from_slice(&limbs(e0));
    row[C_E1..C_E1 + D].copy_from_slice(&limbs(e1));
    row[C_BETA..C_BETA + D].copy_from_slice(&limbs(beta));
    row[C_S] = s;
    row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded));

    // product tower: sp[0]=1, sp[j+1]=sp[j]·(rev_bit_j ? base_j : 1); rev_bit_j = bit_{h-1-j}.
    let bases = tower_bases(log_height);
    let mut acc = Val::ONE;
    row[sp_col(0)] = acc;
    for j in 0..log_height {
        let rev_bit = (canon >> (log_height - 1 - j)) & 1 == 1;
        if rev_bit {
            acc *= bases[j];
        }
        row[sp_col(j + 1)] = acc;
    }
    debug_assert_eq!(acc, s, "tower product != fold_point — base/bit convention mismatch");
    row
}

/// Decode the bus index value of a packed window.
pub fn window_index(row: &[Val]) -> u32 {
    row[C_INDEX].as_canonical_u32()
}

/// Serialize a sequence of per-query windows (one per row) into the flat AIR trace.
pub fn build_trace(
    queries: &[(Val, Challenge, Challenge, Challenge)],
    log_height: usize,
) -> Vec<Val> {
    let mut flat = Vec::with_capacity(queries.len() * n_cols(log_height));
    for &(d, e0, e1, beta) in queries {
        flat.extend_from_slice(&pack_window(d, e0, e1, beta, log_height));
    }
    flat
}

/// Octic multiply over `X^D = W` as in-AIR expressions (same formula as `fri_fold_air`).
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

/// `recursive_air` (BEGIN): the index/challenge bus + FRI fold consumer assembled in one window.
/// Wires cross-bindings (1) index↔draw, (2) fold point↔index (product tower), (3) FRI fold (★).
pub struct RecursiveAir {
    /// FRI layer log-height for this query window (`fold_point`'s `log_height`).
    pub log_height: usize,
}

impl<F: Sync> BaseAir<F> for RecursiveAir {
    fn width(&self) -> usize {
        n_cols(self.log_height)
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for RecursiveAir {
    fn eval(&self, builder: &mut AB) {
        let h = self.log_height;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let two = AB::F::from_u32(2);

        // ===== (1) index ↔ challenger draw (reuse index_bus logic) =====
        let bits: Vec<AB::Expr> = (0..FIELD_BITS).map(|i| l(C_BITS + i)).collect();
        for b in &bits {
            builder.assert_zero(b.clone() * (b.clone() - AB::Expr::ONE)); // booleanity
        }
        // canonical recomposition: Σ bit_i·2^i == draw_out.
        let mut acc = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for b in &bits {
            acc += b.clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(acc - l(C_DRAW_OUT));
        // canonical uniqueness (Σ bits < p): unique < 2^31 representative, not just a residue.
        assert_canonical_pub::<AB>(builder, &bits);
        // consumer index == low `h` bits of the draw.
        let mut idx = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for i in 0..h {
            idx += bits[i].clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(idx - l(C_INDEX));

        // ===== (2) fold point s ↔ index (committed product tower) =====
        // sp[0] == 1; sp[j+1] == sp[j]·(1 + rev_bit_j·(base_j − 1)), rev_bit_j = bit_{h-1-j};
        // s == sp[h]. Each step is degree 2 (running-product × linear bit selector).
        let bases = tower_bases(h);
        builder.assert_zero(l(sp_col(0)) - AB::Expr::ONE); // tower base = 1
        for j in 0..h {
            let rev_bit = bits[h - 1 - j].clone(); // exponent bit j = index bit (h-1-j)
            let base_j = AB::Expr::from(bases[j]);
            // selector = 1 + rev_bit·(base_j − 1)  ∈ {1, base_j}
            let sel = AB::Expr::ONE + rev_bit * (base_j - AB::Expr::ONE);
            // sp[j+1] == sp[j]·sel
            builder.assert_zero(l(sp_col(j + 1)) - l(sp_col(j)) * sel);
        }
        builder.assert_zero(l(C_S) - l(sp_col(h))); // fold point bound to the tower product

        // ===== (3) FRI fold identity (★) at the BUS-BOUND point s =====
        let e0: [AB::Expr; D] = core::array::from_fn(|i| l(C_E0 + i));
        let e1: [AB::Expr; D] = core::array::from_fn(|i| l(C_E1 + i));
        let beta: [AB::Expr; D] = core::array::from_fn(|i| l(C_BETA + i));
        let folded: [AB::Expr; D] = core::array::from_fn(|i| l(C_FOLDED + i));
        let s = l(C_S);

        let mut beta_plus_s = beta.clone();
        beta_plus_s[0] = beta[0].clone() + s.clone();
        let mut beta_minus_s = beta.clone();
        beta_minus_s[0] = beta[0].clone() - s.clone();

        let p0 = octic_mul::<AB>(&e0, &beta_plus_s);
        let p1 = octic_mul::<AB>(&e1, &beta_minus_s);

        let two_s = s.double();
        for k in 0..D {
            builder.assert_zero(folded[k].clone() * two_s.clone() - (p0[k].clone() - p1[k].clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fri_fold::reverse_bits_len;
    use p3_air::check_all_constraints;
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};
    use p3_matrix::dense::RowMajorMatrix;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }
    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }
    fn air(h: usize) -> RecursiveAir {
        RecursiveAir { log_height: h }
    }
    fn matrix(queries: &[(Val, Challenge, Challenge, Challenge)], h: usize) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(build_trace(queries, h), n_cols(h))
    }

    /// PIN: the in-AIR product tower reproduces the NATIVE `fold_point(h, index)`. The packed
    /// `sp[h]` must equal `fold_point` for diverse indices/heights — a wrong base or bit-reversal
    /// convention would diverge here (attempted equality, not assumption). This is the new
    /// cross-binding's anchor: `s` really is the index's fold point.
    #[test]
    fn tower_reproduces_native_fold_point() {
        for h in [4usize, 5, 8, 12, 20] {
            for raw in [0u32, 1, 3, 6, 17, 0x9999, 0xABCDE, 1_000_003] {
                let d = Val::from_u32(raw);
                let row = pack_window(d, ext(1), ext(2), ext(3), h);
                let index = (raw & ((1 << h) - 1)) as usize;
                let expected = fold_point(h, index);
                assert_eq!(row[C_S], expected, "tower s != fold_point at h={h}, raw={raw:#x}");
                assert_eq!(row[sp_col(h)], expected, "sp[h] != fold_point");
                // and the bit-reversal convention matches reverse_bits_len.
                let g = Val::two_adic_generator(h + 1);
                assert_eq!(expected, g.exp_u64(reverse_bits_len(index, h) as u64));
            }
        }
    }

    /// GOLDEN (positive): honest query windows — across base-only and multi-limb evals, several
    /// heights/indices — ACCEPT via the real `check_all_constraints`. The index↔draw bus, the
    /// fold-point tower, and the FRI fold (★) validated together in one window.
    #[test]
    fn air_accepts_honest_windows() {
        for &h in &[5usize, 8, 12] {
            let queries: Vec<_> = [0u32, 1, 7, 0x9999, 0xABCDE, 1_000_003]
                .iter()
                .enumerate()
                .map(|(k, &raw)| {
                    let kk = k as u32;
                    (Val::from_u32(raw), ext_multi(10 + kk), ext_multi(40 + kk), ext_multi(70 + kk))
                })
                .collect();
            let report = check_all_constraints(&air(h), &matrix(&queries, h), &[], None);
            assert!(report.is_ok(), "honest windows rejected at h={h}: {} failures", report.failures.len());
        }
    }

    /// Forgery — (1) FORGED INDEX: claim a different (favorable) index than the bits of the sampled
    /// draw recompose to. Rejected by the consumer-index binding. The prover cannot answer at a
    /// query of its choosing.
    #[test]
    fn air_rejects_forged_index() {
        let h = 12;
        let mut m = matrix(&[(Val::from_u32(0xABCDE), ext_multi(1), ext_multi(2), ext_multi(3))], h);
        m.values[C_INDEX] += Val::ONE; // index != low bits of the draw
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// Forgery — (2) FORGED FOLD POINT: present an `s` that is NOT `fold_point(h, index)` for the
    /// bound index (e.g. the fold point of a DIFFERENT, favorable index). Rejected by the product
    /// tower binding `s == sp[h]`. THIS is the new cross-binding's soundness property: the prover
    /// cannot fold at a point of its choosing — the point is pinned to the FS-forced index.
    #[test]
    fn air_rejects_forged_fold_point() {
        let h = 8;
        let draw = Val::from_u32(0xABCDE);
        let canon = draw.as_canonical_u32();
        let honest_index = (canon & ((1 << h) - 1)) as usize;
        let forged_index = honest_index ^ 1; // a different query
        let s_forged = fold_point(h, forged_index);
        assert_ne!(fold_point(h, honest_index), s_forged);
        // Build the honest window, then overwrite ONLY the fold point with the forged-index point:
        // the tower still computes the honest s, so s == sp[h] fails (and (★) fails too).
        let mut row = pack_window(draw, ext_multi(4), ext_multi(5), ext_multi(6), h);
        row[C_S] = s_forged;
        let m = RowMajorMatrix::new(row, n_cols(h));
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// Forgery — (2′) TAMPERED TOWER: corrupt an interior tower power `sp[j]` so the running product
    /// no longer equals the honest fold point. Rejected by the squaring/product chain — confirms the
    /// tower is fully constrained, not just its endpoints (a prover can't relabel an interior power
    /// to smuggle a favorable `s`).
    #[test]
    fn air_rejects_tampered_tower() {
        let h = 12;
        let mut row = pack_window(Val::from_u32(0x55555), ext_multi(7), ext_multi(8), ext_multi(9), h);
        row[sp_col(6)] += Val::ONE; // corrupt a middle tower power
        let m = RowMajorMatrix::new(row, n_cols(h));
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// Forgery — (2″) DESYNCED BITS vs INDEX: flip a HIGH index bit (above the consumer's
    /// `log_height` window) and adjust `draw_out` so the recomposition still holds — but this also
    /// changes a bit the TOWER consumes (`rev_bit_j = bit_{h-1-j}` is a LOW bit). Concretely, flip a
    /// low bit consumed by both index AND tower: index binding OR tower binding catches it. Here we
    /// flip a bit the tower reads and recompute draw_out+index consistently, leaving `s` honest for
    /// the OLD index ⇒ tower (using the NEW bit) disagrees with `s` ⇒ rejected.
    #[test]
    fn air_rejects_bit_index_point_desync() {
        let h = 8;
        let draw = Val::from_u32(0xABCD0); // low bit 0 = 0
        let mut row = pack_window(draw, ext_multi(1), ext_multi(2), ext_multi(3), h);
        // Flip bit 0 in the decomposition AND in draw_out + index so (1) still holds, but leave the
        // fold point s + tower computed for the ORIGINAL bits. Now rev uses the new bit -> tower
        // recomputes a different sp[h] than the (stale) committed s.
        row[C_BITS + 0] = Val::ONE;
        row[C_DRAW_OUT] += Val::ONE; // recomposition Σ bits·2^i now matches (bit0 0->1)
        row[C_INDEX] += Val::ONE; // index low bits now match the new decomposition
        // sp[] and s are still the honest tower for the OLD bit0=0; the AIR recomputes sp from the
        // NEW bits, so sp[h] != committed s -> rejected.
        let m = RowMajorMatrix::new(row, n_cols(h));
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// Forgery — (3) TAMPERED SIBLING EVAL: a tampered `e0` ⇒ the FRI fold identity (★) at the
    /// bus-bound point no longer holds ⇒ rejected. (The fold binds both opened evals.)
    #[test]
    fn air_rejects_tampered_eval() {
        let h = 12;
        let mut m = matrix(&[(Val::from_u32(0x1234), ext_multi(11), ext_multi(12), ext_multi(13))], h);
        m.values[C_E0 + 2] += Val::ONE;
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// Forgery — (3′) TAMPERED FOLDED RESULT: a wrong claimed `folded` ⇒ (★) fails ⇒ rejected.
    #[test]
    fn air_rejects_wrong_folded() {
        let h = 8;
        let mut m = matrix(&[(Val::from_u32(0x9999), ext_multi(14), ext_multi(15), ext_multi(16))], h);
        m.values[C_FOLDED + 1] += Val::ONE;
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// Forgery — (3″) TAMPERED BETA: a different round challenge ⇒ (★) fails ⇒ rejected. (In the
    /// full monolith `beta` is bound to gadget-1's SampleDraw; here it is the consumed challenge.)
    #[test]
    fn air_rejects_tampered_beta() {
        let h = 12;
        let mut m = matrix(&[(Val::from_u32(0x4242), ext_multi(17), ext_multi(18), ext_multi(19))], h);
        m.values[C_BETA + 4] += Val::ONE;
        assert!(!check_all_constraints(&air(h), &m, &[], None).is_ok());
    }

    /// CROSS-BINDING END-TO-END: an honest window where the fold point is DERIVED from the
    /// challenger-bound index accepts; substituting the fold of a DIFFERENT query (favorable index)
    /// — its evals AND point — is rejected because that point is not the tower product of THIS
    /// window's FS-forced index. This is "draw → index → fold point" pinned in one window.
    #[test]
    fn air_binds_draw_to_index_to_fold_point() {
        let h = 10;
        let draw = Val::from_u32(0x3ABCD);
        // Honest: accept.
        let honest = matrix(&[(draw, ext_multi(21), ext_multi(22), ext_multi(23))], h);
        assert!(check_all_constraints(&air(h), &honest, &[], None).is_ok());

        // Forge: prover wants to answer a DIFFERENT query. Build a window at a forged index's point
        // (different favorable index), but keep THIS window's draw/bits/index honest. The fold evals
        // + point are for the forged index; the tower (from the honest bits) yields the honest s, so
        // s != committed point -> rejected.
        let honest_index = (draw.as_canonical_u32() & ((1 << h) - 1)) as usize;
        let forged_index = honest_index ^ 0b101;
        let s_forged = fold_point(h, forged_index);
        let (e0, e1, beta) = (ext_multi(31), ext_multi(32), ext_multi(33));
        let folded = fold_pair(e0, e1, beta, s_forged);
        let mut row = pack_window(draw, e0, e1, beta, h); // honest bits/index/tower for `draw`
        // overwrite the fold block with the FORGED-index fold (consistent (★) at s_forged):
        row[C_S] = s_forged;
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded));
        let m = RowMajorMatrix::new(row, n_cols(h));
        assert!(
            !check_all_constraints(&air(h), &m, &[], None).is_ok(),
            "fold at a forged-index point accepted — draw→index→point binding not load-bearing"
        );
    }

    /// Multi-row honest trace (one query window per row) accepts.
    #[test]
    fn air_accepts_multirow() {
        let h = 12;
        let queries: Vec<_> = (0..8u32)
            .map(|q| {
                (Val::from_u32(0x40000 + q * 0x1111), ext_multi(100 + q), ext_multi(200 + q), ext_multi(300 + q))
            })
            .collect();
        let report = check_all_constraints(&air(h), &matrix(&queries, h), &[], None);
        assert!(report.is_ok(), "honest multi-row recursive_air trace rejected: {} failures", report.failures.len());
    }
}
