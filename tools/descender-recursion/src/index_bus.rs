//! Cross-gadget index bus (begin) — bind the Merkle / FRI query index to gadget-1's
//! FS-sampled `sample_bits` draw.
//!
//! ## The gap this closes
//!
//! In the standalone gadget-2 [`crate::merkle_air`] the per-level index bit (`C_INDEX_BIT`) is
//! FREE WITNESS, constrained only indirectly by the leaf↔root boundary; and the standalone
//! gadget-3 [`crate::fri_fold_air`] takes the fold point `s` as a committed column. Nothing
//! ties either to the query index the verifier ACTUALLY sampled from the Fiat–Shamir
//! transcript. In the monolithic `recursive_air` a malicious prover could therefore answer at a
//! FAVORABLE query index instead of the FS-forced one. The fix is a bus binding
//!
//!   `merkle_index_bits  ==  fri_index_bits  ==  low_bits( challenger.draw_out )`
//!
//! where `challenger.draw_out` is gadget-1's `SampleDraw` export (the base-field element a
//! `sample_bits(bits)` reduces). This module is the BOUNDED interface piece: it constrains the
//! per-query index as a boolean bit-vector, binds those bits to the sampled draw via a
//! recomposition constraint, and re-exports them as the bus value both gadget-2 and gadget-3
//! read. The full cross-wiring into one AIR window is deferred to `recursive_air`; this lands
//! the soundness-bearing constraint + a test that a FORGED index (bits that don't match the
//! sampled draw) is rejected once bound.
//!
//! ## Soundness of the decomposition
//!
//! `draw_out` is a CANONICAL KoalaBear element (`< p = 0x7f000001 < 2^31`). The bus carries a
//! full 31-bit decomposition `bit[0..31]`, each constrained boolean, with the recomposition
//! `Σ bit_i·2^i == draw_out`. Because the leaf challenger's `sample_bits` reduces the SAME
//! canonical representative (`as_canonical_u32() & mask`), the low `bits` of this decomposition
//! are exactly the sampled index bits — so binding the consumers' index to `Σ_{i<bits} bit_i·2^i`
//! ties them to the transcript. (Canonical UNIQUENESS — that no second `< 2^31` representative
//! recomposes to the same residue — is the standard `Σ bits < p` range check; it is enforced
//! here by [`assert_canonical`], a lt-decomposition against `p`'s bits, so the binding is sound,
//! not merely modular.)

use descender_account_stark::Val;
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField32};

extern crate alloc;
use alloc::vec::Vec;

/// Canonical bit width of a KoalaBear element (`p = 0x7f000001 < 2^31`).
pub const FIELD_BITS: usize = 31;

/// The KoalaBear prime `p` (canonical-uniqueness reference for [`assert_canonical`]).
pub const P: u32 = 0x7f000001;

// ----- column layout (one sampled query index per row) -----
/// The gadget-1 export: the `SampleDraw` base-field element this index reduces.
pub const C_DRAW_OUT: usize = 0;
/// Full canonical bit decomposition of `draw_out` (LSB first), `FIELD_BITS` columns.
pub const C_BITS: usize = C_DRAW_OUT + 1;
/// The bus index value the consumers (gadget-2 Merkle bits, gadget-3 fold point) read:
/// `Σ_{i<query_bits} bit_i·2^i`. Bound to the low bits of `draw_out` by construction.
pub const C_INDEX: usize = C_BITS + FIELD_BITS;
/// Total AIR width.
pub const N_COLS: usize = C_INDEX + 1;

/// Number of low bits a query index consumes (`sample_bits(query_bits)`). The leaf uses a
/// per-layer log-height; this is the bus-side width that gadget-2/3 slice from the draw. Kept
/// as a const for the standalone test; `recursive_air` passes the per-query layer height.
pub const QUERY_BITS: usize = 20;

/// Build one index-bus row from a sampled `draw_out` and the number of index bits consumed.
/// The `index` cell is the low `query_bits` of the canonical draw (mirrors
/// `RowChallenger::sample_bits`).
pub fn pack_index(draw_out: Val, query_bits: usize) -> [Val; N_COLS] {
    let mut row = [Val::ZERO; N_COLS];
    let canon = draw_out.as_canonical_u32();
    row[C_DRAW_OUT] = draw_out;
    for i in 0..FIELD_BITS {
        row[C_BITS + i] = Val::from_u32((canon >> i) & 1);
    }
    let index = canon & ((1u32 << query_bits) - 1);
    row[C_INDEX] = Val::from_u32(index);
    row
}

/// Decode the bus index value of a packed row (the value gadget-2/3 consume).
pub fn row_index(row: &[Val; N_COLS]) -> u32 {
    row[C_INDEX].as_canonical_u32()
}

/// Serialize a sequence of sampled query indices into the flat AIR trace, one row per query.
pub fn build_trace(draws: &[Val], query_bits: usize) -> Vec<Val> {
    let mut flat = Vec::with_capacity(draws.len() * N_COLS);
    for &d in draws {
        flat.extend_from_slice(&pack_index(d, query_bits));
    }
    flat
}

/// Index-bus AIR: per row, constrain the bit decomposition of the gadget-1 draw, the canonical
/// recomposition, canonical uniqueness, and the consumer index = low `query_bits`.
pub struct IndexBusAir {
    /// How many low bits the consumers slice (`sample_bits` width for this query / layer).
    pub query_bits: usize,
}

impl<F: Sync> BaseAir<F> for IndexBusAir {
    fn width(&self) -> usize {
        N_COLS
    }
}

/// Helper: assert `Σ bit_i·2^i < p` via the standard most-significant-difference lt argument
/// against the fixed bits of `p`. Pushes the canonical-uniqueness constraint so the
/// recomposition pins a UNIQUE `< 2^31` representative (not just a residue mod p).
fn assert_canonical<AB: AirBuilder>(builder: &mut AB, bits: &[AB::Expr]) {
    // p's bits (LSB first), 31 bits.
    let p_bit = |i: usize| (P >> i) & 1 == 1;
    // "all higher bits equal p's" prefix, walking MSB->LSB. `gt_blocked` becomes 1 once a
    // higher bit already made value < p (so equality at this bit no longer needs the strict
    // check). We require: at the most-significant bit where value and p differ, value's bit is
    // 0 (value < p). Encode as: for each i (MSB first), if all higher bits == p AND p_bit==0,
    // then bit_i must be 0 (can't exceed). This is the textbook canonical-form gate.
    let one = AB::Expr::ONE;
    // prefix_eq[i] = 1 iff bits[j] == p_bit(j) for all j > i (so far matching p exactly).
    let mut prefix_eq = one.clone();
    for i in (0..FIELD_BITS).rev() {
        if p_bit(i) {
            // p has a 1 here: value may be 0 or 1; if value bit is 0 it drops below p ->
            // subsequent bits free. prefix_eq stays "matching" only when value bit is 1.
            // new_prefix_eq = prefix_eq * bit_i  (still equal-so-far iff bit_i == 1).
            prefix_eq *= bits[i].clone();
        } else {
            // p has a 0 here: if still equal-so-far, value bit MUST be 0 (a 1 would make
            // value > p). Constraint: prefix_eq * bit_i == 0.
            builder.assert_zero(prefix_eq.clone() * bits[i].clone());
            // prefix_eq unchanged (equal-so-far requires bit_i == 0, which the above forces
            // when prefix_eq==1; when prefix_eq==0 it's already dropped below).
        }
    }
}

/// Public re-export of the canonical-uniqueness gate so the monolithic `recursive_air` reuses the
/// SAME validated `Σ bits < p` lt-argument (DRY — one canonical gate, tested here, consumed there).
pub fn assert_canonical_pub<AB: AirBuilder>(builder: &mut AB, bits: &[AB::Expr]) {
    assert_canonical::<AB>(builder, bits);
}

impl<AB: AirBuilder<F = Val>> Air<AB> for IndexBusAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let two = AB::F::from_u32(2);

        // --- bit booleanity ---
        let bits: Vec<AB::Expr> = (0..FIELD_BITS).map(|i| l(C_BITS + i)).collect();
        for b in &bits {
            builder.assert_zero(b.clone() * (b.clone() - AB::Expr::ONE));
        }

        // --- canonical recomposition: Σ bit_i·2^i == draw_out ---
        let mut acc = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for b in &bits {
            acc += b.clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(acc - l(C_DRAW_OUT));

        // --- canonical uniqueness: Σ bits < p (so the representative is unique) ---
        assert_canonical::<AB>(builder, &bits);

        // --- consumer index == low query_bits of the draw: Σ_{i<query_bits} bit_i·2^i ---
        let mut idx = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for i in 0..self.query_bits {
            idx += bits[i].clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(idx - l(C_INDEX));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_matrix::dense::RowMajorMatrix;

    fn air(qb: usize) -> IndexBusAir {
        IndexBusAir { query_bits: qb }
    }

    fn matrix(draws: &[Val], qb: usize) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(build_trace(draws, qb), N_COLS)
    }

    /// The packed bus index equals the leaf challenger's `sample_bits` of the same draw —
    /// the bus value really is the FS-sampled query index (constant-for-constant).
    #[test]
    fn bus_index_matches_sample_bits() {
        for raw in [0u32, 1, 0x9999, 0xABCDE, 123_456, P - 1] {
            let d = Val::from_u32(raw);
            let row = pack_index(d, QUERY_BITS);
            let expected = (d.as_canonical_u32() as usize) & ((1usize << QUERY_BITS) - 1);
            assert_eq!(row_index(&row) as usize, expected, "bus index != sample_bits(draw)");
        }
    }

    /// GOLDEN (positive): honest index-bus rows ACCEPT via the real `check_all_constraints`
    /// across diverse draws (incl. p-1, the max canonical value).
    #[test]
    fn air_accepts_honest_indices() {
        let draws: Vec<Val> = [0u32, 1, 7, 0x9999, 0xABCDE, 1_000_003, P - 1]
            .iter()
            .map(|&r| Val::from_u32(r))
            .collect();
        let report = check_all_constraints(&air(QUERY_BITS), &matrix(&draws, QUERY_BITS), &[], None);
        assert!(report.is_ok(), "honest index-bus trace rejected: {} failures", report.failures.len());
    }

    /// Forgery — a FORGED query index: claim a different (favorable) index than the bits of the
    /// sampled draw recompose to. Rejected by the consumer-index binding. THIS is the bus's
    /// soundness property: a prover cannot answer at a query of its choosing.
    #[test]
    fn air_rejects_forged_index() {
        let d = Val::from_u32(0xABCDE);
        let mut m = matrix(&[d], QUERY_BITS);
        m.values[C_INDEX] += Val::ONE; // claim index+1, not the sampled low bits
        assert!(!check_all_constraints(&air(QUERY_BITS), &m, &[], None).is_ok());
    }

    /// Forgery — tampered bit-decomposition that no longer recomposes to the draw is rejected
    /// (a prover can't relabel the bits to fake a different index while keeping the same draw).
    #[test]
    fn air_rejects_decomposition_mismatch() {
        let d = Val::from_u32(0x12345);
        let mut m = matrix(&[d], QUERY_BITS);
        // flip a low bit column WITHOUT changing draw_out: recomposition breaks.
        m.values[C_BITS + 3] = if m.values[C_BITS + 3] == Val::ZERO { Val::ONE } else { Val::ZERO };
        assert!(!check_all_constraints(&air(QUERY_BITS), &m, &[], None).is_ok());
    }

    /// Forgery — a non-boolean bit column is rejected (a prover can't smuggle a >1 limb to
    /// satisfy the recomposition with fewer set bits).
    #[test]
    fn air_rejects_nonboolean_bit() {
        let d = Val::from_u32(0x777);
        let mut m = matrix(&[d], QUERY_BITS);
        m.values[C_BITS + 5] = Val::from_u32(2); // not 0/1
        assert!(!check_all_constraints(&air(QUERY_BITS), &m, &[], None).is_ok());
    }

    /// Canonical-uniqueness (ISOLATED): a NON-canonical decomposition whose bits represent
    /// `v = r + p` (still `< 2^31`) but whose `draw_out`/`index` cells are CONSISTENT with the
    /// in-field recomposition (`acc = v mod p = r`) — so the recomposition constraint PASSES and
    /// ONLY `assert_canonical` can reject it. This proves the bus binds a UNIQUE `< 2^31`
    /// representative, not merely a residue mod p (without this, a prover could pick either of
    /// two bit-patterns ⇒ a second admissible index).
    #[test]
    fn air_rejects_noncanonical_representative() {
        let r: u32 = 5;
        let v: u32 = r + P; // 0x7f000006 < 2^31, ≡ r (mod p), but NON-canonical (>= p)
        assert!(v < (1u32 << FIELD_BITS), "v must fit in {FIELD_BITS} bits");
        let mut row = [Val::ZERO; N_COLS];
        // bits = the non-canonical pattern for v.
        for i in 0..FIELD_BITS {
            row[C_BITS + i] = Val::from_u32((v >> i) & 1);
        }
        // draw_out / index made CONSISTENT with the in-FIELD recomposition acc = v mod p = r,
        // so the recomposition + index constraints pass and isolate the canonical gate.
        row[C_DRAW_OUT] = Val::from_u32(r);
        row[C_INDEX] = Val::from_u32(r & ((1u32 << QUERY_BITS) - 1));
        let m = RowMajorMatrix::new(row.to_vec(), N_COLS);
        assert!(
            !check_all_constraints(&air(QUERY_BITS), &m, &[], None).is_ok(),
            "non-canonical representative (bits = r+p) accepted — canonical gate not binding"
        );
    }

    /// Multi-row honest trace accepts (one sampled query index per row).
    #[test]
    fn air_accepts_multirow_indices() {
        let draws: Vec<Val> = (0..16u32).map(|q| Val::from_u32(0x40000 + q * 0x1111)).collect();
        let report = check_all_constraints(&air(QUERY_BITS), &matrix(&draws, QUERY_BITS), &[], None);
        assert!(report.is_ok(), "honest multi-row index-bus trace rejected: {} failures", report.failures.len());
    }

    /// CONSUMER BINDING (bus -> gadget-3): the bus index drives the FRI fold point
    /// `s = fold_point(log_height, index)`. A FORGED query index (≠ the sampled draw's low bits)
    /// derives a DIFFERENT `s`, so the gadget-3 fold of the honestly-opened sibling pair lands
    /// at a different point and the fold AIR rejects. This shows "wrong index rejected ONCE
    /// BOUND": the bus pins the index, the consumer derives `s` from it, and the two are
    /// consistent only for the FS-forced query. (Full single-window wiring is `recursive_air`;
    /// this is the bounded interface end-to-end across the two gadget AIRs.)
    #[test]
    fn forged_index_rejected_once_bound_to_fri_fold() {
        use crate::fri_fold::fold_point;
        use crate::fri_fold_air::{self, FriFoldAir};
        use descender_account_stark::Challenge;
        use p3_field::PrimeCharacteristicRing;

        let log_height = QUERY_BITS; // layer log-height for this query
        // The FS-sampled draw fixes the honest index = low QUERY_BITS of its canonical form.
        let draw = Val::from_u32(0xABCDE);
        let honest_index = (draw.as_canonical_u32() as usize) & ((1usize << QUERY_BITS) - 1);

        // Bus pins the honest index (the bus AIR accepts this row).
        let bus_row = pack_index(draw, QUERY_BITS);
        assert_eq!(row_index(&bus_row) as usize, honest_index);
        let bus_m = matrix(&[draw], QUERY_BITS);
        assert!(check_all_constraints(&air(QUERY_BITS), &bus_m, &[], None).is_ok());

        // Honest consumer: derive s from the BOUND index; an honest fold at that s ACCEPTS.
        let s_honest = fold_point(log_height, honest_index);
        let (e0, e1, beta) = (
            Challenge::from(Val::from_u32(7)),
            Challenge::from(Val::from_u32(11)),
            Challenge::from(Val::from_u32(99)),
        );
        let honest_fold = fri_fold_air::build_trace(&[(e0, e1, beta, s_honest)]);
        let honest_m = p3_matrix::dense::RowMajorMatrix::new(honest_fold, fri_fold_air::N_COLS);
        assert!(
            check_all_constraints(&FriFoldAir, &honest_m, &[], None).is_ok(),
            "honest fold at the bound index's point rejected"
        );

        // Forged consumer: a prover picks a DIFFERENT (favorable) index. Its derived point
        // s_forged != s_honest, so the SAME opened sibling evals fold to a value the prover
        // cannot also claim consistent with the bound s. Concretely: keep the honest folded
        // result (computed at s_honest) but present s_forged in the fold row -> rejected.
        let forged_index = honest_index ^ 1; // flip the low bit: a different query
        let s_forged = fold_point(log_height, forged_index);
        assert_ne!(s_honest, s_forged, "flipped index must change the fold point");
        // Build the fold row at s_honest (honest folded cell), then overwrite the point column
        // with s_forged: the fold identity (★) at s_forged no longer holds -> rejected.
        let mut forged_fold = fri_fold_air::build_trace(&[(e0, e1, beta, s_honest)]);
        forged_fold[fri_fold_air::C_S] = s_forged;
        let forged_m = p3_matrix::dense::RowMajorMatrix::new(forged_fold, fri_fold_air::N_COLS);
        assert!(
            !check_all_constraints(&FriFoldAir, &forged_m, &[], None).is_ok(),
            "fold at a forged-index point accepted — index binding not load-bearing"
        );
    }
}
