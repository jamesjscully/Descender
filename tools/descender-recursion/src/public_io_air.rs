//! Gadget 5 (proper) — in-AIR public-IO statement preservation as a real `Air<AB>`.
//!
//! Lifts the VALIDATED [`crate::public_io`] reference (`verify_statement_preserved`:
//! `outer_exposed == challenger_observed == inner_public`, element-wise) into a Plonky3
//! `Air<AB>`, one public-value ELEMENT per trace ROW. The recursion is statement-preserving iff
//! the field elements the OUTER proof re-exposes are exactly (a) the ones gadget 1 folded into
//! the Fiat–Shamir transcript (`observe(public_values)`, transcript step 6) and (b) the inner
//! account-creation proof's own public values. Gadget 5 binds the three equal so a prover cannot
//! verify an inner proof for statement `P` yet re-expose a different statement `P'` (statement
//! substitution).
//!
//! ## The constraint (degree 1)
//!
//!   per row:  `outer_exposed == challenger_observed`  ∧  `challenger_observed == inner_public`
//!
//! i.e. the three element-wise columns coincide on EVERY row (two linear assert-zeros). To tie
//! the committed `outer_exposed` COLUMN to the proof's ACTUAL exposed statement (not just three
//! free columns to each other), the boundary rows bind `outer_exposed` to this verifier-AIR's
//! PUBLIC VALUES: `outer_exposed[first] == pubs[0]`, `outer_exposed[last] == pubs[last]`. The
//! per-row positional outer binding for the interior is the shared-window job of `recursive_air`
//! (where the public window is indexed directly); the standalone gadget lands the element-wise
//! core over all three columns + the boundary anchoring.
//!
//! ## Soundness discipline
//!
//! Honest rows (`outer == observed == inner`) ACCEPT via the real `p3_air::check_all_constraints`;
//! tampering any of the three columns (statement substitution on the outer cell, a forged observed
//! cell, or a forged inner-public cell) REJECTS, and a re-exposed statement whose boundary differs
//! from the public values REJECTS. Soundness by attempted forgery, never by inspection — the same
//! discipline as the other gadget AIRs.

use descender_account_stark::Val;
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

// ----- column layout (one public-IO element per row) -----
/// The public value THIS (outer) proof re-exposes for this index.
pub const C_OUTER: usize = 0;
/// The public value FS-observed by gadget 1 for this index (challenger absorb cell).
pub const C_OBSERVED: usize = C_OUTER + 1;
/// The inner account-creation proof's public value for this index.
pub const C_INNER: usize = C_OBSERVED + 1;
/// Total AIR width.
pub const N_COLS: usize = C_INNER + 1;

/// Build one statement-preservation row from the (outer, observed, inner) triple for one public
/// index. The honest builder sets them equal (the authority the AIR is checked against); tamper
/// tests mutate the matrix afterwards.
pub fn pack_element(outer: Val, observed: Val, inner: Val) -> [Val; N_COLS] {
    let mut row = [Val::ZERO; N_COLS];
    row[C_OUTER] = outer;
    row[C_OBSERVED] = observed;
    row[C_INNER] = inner;
    row
}

/// Serialize a full statement (one element per row) into the flat AIR trace. The outer-exposed
/// public values are ALSO supplied as the AIR's public-value vector (the `outer` slice passed to
/// `check_all_constraints`) so the boundary binding anchors the committed `C_OUTER` column to the
/// proof's exposed statement.
pub fn build_trace(outer: &[Val], observed: &[Val], inner: &[Val]) -> Vec<Val> {
    assert_eq!(outer.len(), observed.len(), "outer/observed length mismatch");
    assert_eq!(observed.len(), inner.len(), "observed/inner length mismatch");
    let mut flat = Vec::with_capacity(outer.len() * N_COLS);
    for ((&o, &b), &i) in outer.iter().zip(observed.iter()).zip(inner.iter()) {
        flat.extend_from_slice(&pack_element(o, b, i));
    }
    flat
}

/// Public-IO statement-preservation AIR (one public element per row). Per row binds
/// `outer == observed == inner`; boundary rows bind `outer` to the proof's public values.
pub struct PublicIoAir;

impl<F: Sync> BaseAir<F> for PublicIoAir {
    fn width(&self) -> usize {
        N_COLS
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for PublicIoAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder.public_values().iter().map(|p| (*p).into()).collect();
        let l = |i: usize| -> AB::Expr { local[i].into() };

        // --- element-wise core (every row): outer == observed == inner ---
        builder.assert_zero(l(C_OUTER) - l(C_OBSERVED));
        builder.assert_zero(l(C_OBSERVED) - l(C_INNER));

        // --- boundary anchoring: the re-exposed outer column == the proof's public statement at
        //     the first and last elements (the statement-window endpoints). ---
        if !pubs.is_empty() {
            let mut fb = builder.when_first_row();
            fb.assert_zero(l(C_OUTER) - pubs[0].clone());
            let mut lb = builder.when_last_row();
            lb.assert_zero(l(C_OUTER) - pubs[pubs.len() - 1].clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_matrix::dense::RowMajorMatrix;

    fn pubs(seed: u32, n: usize) -> Vec<Val> {
        (0..n as u32).map(|i| Val::from_u32(seed.wrapping_add(i))).collect()
    }

    fn matrix(outer: &[Val], observed: &[Val], inner: &[Val]) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(build_trace(outer, observed, inner), N_COLS)
    }

    /// GOLDEN (positive): an honest statement (outer == observed == inner, element-wise, with the
    /// outer column matching the public values) ACCEPTS via the real `check_all_constraints`.
    #[test]
    fn air_accepts_honest_statement() {
        let stmt = pubs(1000, 24);
        let m = matrix(&stmt, &stmt, &stmt);
        let report = check_all_constraints(&PublicIoAir, &m, &stmt, None);
        assert!(report.is_ok(), "honest statement rejected: {} failures", report.failures.len());
    }

    /// Forgery — STATEMENT SUBSTITUTION (interior element): the inner proof's public value differs
    /// from the outer re-exposed value at a middle index ⇒ `outer != inner` ⇒ rejected on that row.
    /// The core soundness property: the prover cannot verify P but expose P'.
    #[test]
    fn air_rejects_interior_substitution() {
        let stmt = pubs(1000, 24);
        let mut inner = stmt.clone();
        inner[7] += Val::ONE; // inner statement differs at index 7
        let m = matrix(&stmt, &stmt, &inner);
        assert!(!check_all_constraints(&PublicIoAir, &m, &stmt, None).is_ok());
    }

    /// Forgery — FORGED OBSERVED cell: the value gadget 1 folded into the transcript differs from
    /// the re-exposed/inner value ⇒ `observed != outer`/`observed != inner` ⇒ rejected.
    #[test]
    fn air_rejects_forged_observed_cell() {
        let stmt = pubs(2000, 16);
        let mut m = matrix(&stmt, &stmt, &stmt);
        m.values[5 * N_COLS + C_OBSERVED] += Val::ONE; // observed diverges at index 5
        assert!(!check_all_constraints(&PublicIoAir, &m, &stmt, None).is_ok());
    }

    /// Forgery — BOUNDARY SUBSTITUTION: re-expose (as the OUTER public value) a different statement
    /// than the committed column at the first AND last bound endpoints ⇒ rejected by the boundary
    /// anchoring (this is what ties the committed column to the proof's exposed statement).
    #[test]
    fn air_rejects_boundary_substitution() {
        let stmt = pubs(3000, 24);
        let m = matrix(&stmt, &stmt, &stmt);
        // outer public value at index 0 differs from the committed C_OUTER column.
        let mut outer0 = stmt.clone();
        outer0[0] += Val::ONE;
        assert!(!check_all_constraints(&PublicIoAir, &m, &outer0, None).is_ok());
        // ...and at the last index.
        let mut outer_n = stmt.clone();
        let last = outer_n.len() - 1;
        outer_n[last] += Val::ONE;
        assert!(!check_all_constraints(&PublicIoAir, &m, &outer_n, None).is_ok());
    }

    /// Forgery — LENGTH MISMATCH: a public vector shorter/longer than the proven statement (host
    /// supplies a different number of outer public values than the trace height) re-exposes a
    /// truncated/padded statement. `pubs[last]` then refers to a DIFFERENT element than the last
    /// trace row's outer cell ⇒ the last-row boundary binding fails ⇒ rejected.
    #[test]
    fn air_rejects_length_mismatch() {
        let stmt = pubs(4000, 16);
        let m = matrix(&stmt, &stmt, &stmt);
        let truncated = &stmt[..stmt.len() - 1];
        assert!(!check_all_constraints(&PublicIoAir, &m, truncated, None).is_ok());
        let mut extended = stmt.clone();
        extended.push(Val::ZERO);
        assert!(!check_all_constraints(&PublicIoAir, &m, &extended, None).is_ok());
    }

    /// Cross-check: the AIR's accept set agrees with the [`crate::public_io`] reference predicate —
    /// honest accepts in both; a substituted statement rejects in both.
    #[test]
    fn air_agrees_with_reference_predicate() {
        let stmt = pubs(5000, 24);
        assert!(crate::public_io::verify_statement_preserved(&stmt, &stmt));
        let m = matrix(&stmt, &stmt, &stmt);
        assert!(check_all_constraints(&PublicIoAir, &m, &stmt, None).is_ok());

        let mut exposed = stmt.clone();
        exposed[0] += Val::ONE;
        assert!(!crate::public_io::verify_statement_preserved(&stmt, &exposed));
        // mirror the substitution in the AIR (boundary element differs from public values).
        assert!(!check_all_constraints(&PublicIoAir, &m, &exposed, None).is_ok());
    }

    /// A single-element statement accepts (degenerate but valid: first row == last row, both bound).
    #[test]
    fn air_accepts_single_element_statement() {
        let stmt = pubs(6000, 1);
        let m = matrix(&stmt, &stmt, &stmt);
        assert!(check_all_constraints(&PublicIoAir, &m, &stmt, None).is_ok());
    }
}
