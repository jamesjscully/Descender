//! Gadget 5 — in-AIR public-IO statement preservation (reference layer).
//!
//! The recursion must be STATEMENT-PRESERVING: the outer proof re-exposes exactly the inner
//! account-creation proof's public values, and those same field elements are the ones folded
//! into the Fiat–Shamir transcript by gadget 1 (transcript step 6, `observe(public_values)`).
//! So gadget 5 binds three things equal:
//!
//!   `outer_exposed_public  ==  challenger_observed_public  ==  inner_public`
//!
//! Without this binding a prover could verify an inner proof for statement `P` but re-expose
//! a different statement `P'` (statement substitution). The check is a fixed element-wise
//! equality (degree 1) — trivial as an AIR constraint, but load-bearing for soundness.
//!
//! In the assembled `recursive_air`, `challenger_observed_public` are the cells gadget 1
//! absorbed and `outer_exposed_public` are this verifier-AIR's own public values; gadget 5
//! asserts they coincide cell-for-cell. The inner proof's public values arrive as the
//! shape-checked public-input vector (`descender_account_stark` `full_public_values` /
//! `batch_public_values`).

use descender_account_stark::Val;

/// Verify statement preservation: the public values FS-observed by gadget 1 equal the public
/// values the outer proof re-exposes (and are nonempty). Element-wise equality.
pub fn verify_statement_preserved(challenger_observed: &[Val], outer_exposed: &[Val]) -> bool {
    !challenger_observed.is_empty() && challenger_observed == outer_exposed
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::PrimeCharacteristicRing;

    fn pubs(seed: u32, n: usize) -> Vec<Val> {
        (0..n as u32)
            .map(|i| Val::from_u32(seed.wrapping_add(i)))
            .collect()
    }

    /// Honest: re-exposed public matches the FS-observed public.
    #[test]
    fn honest_statement_is_preserved() {
        let observed = pubs(1000, 24);
        let exposed = observed.clone();
        assert!(verify_statement_preserved(&observed, &exposed));
    }

    /// Forgery — statement substitution: re-exposing a DIFFERENT statement than the one
    /// FS-observed (and thus proven) is rejected. The core soundness property of gadget 5.
    #[test]
    fn statement_substitution_is_rejected() {
        let observed = pubs(1000, 24);
        let mut exposed = observed.clone();
        exposed[7] += Val::ONE; // claim a different public value
        assert!(!verify_statement_preserved(&observed, &exposed));
    }

    /// Forgery — length mismatch (truncated/extended public vector) is rejected.
    #[test]
    fn length_mismatch_is_rejected() {
        let observed = pubs(1000, 24);
        assert!(!verify_statement_preserved(&observed, &observed[..23]));
        let mut longer = observed.clone();
        longer.push(Val::ZERO);
        assert!(!verify_statement_preserved(&observed, &longer));
    }

    /// Degenerate: an empty public vector is rejected (a statement-preserving recursion must
    /// carry the inner statement; empty would expose nothing).
    #[test]
    fn empty_public_is_rejected() {
        assert!(!verify_statement_preserved(&[], &[]));
    }
}
