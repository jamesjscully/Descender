//! Gadget 4 — in-AIR DEEP-ALI / out-of-domain constraint check (reference layer, core).
//!
//! Mirrors the soundness heart of `p3-uni-stark`'s out-of-domain check
//! (`verifier.rs::verify_opening_at_zeta` / line 195: `folded_constraints *
//! sels.inv_vanishing == quotient`): at the OOD point `zeta`, the verifier checks
//!
//!   `C(zeta) = Z_H(zeta) · Q(zeta)`,   where `Z_H(zeta) = zeta^{|H|} − 1`
//!
//! is the trace-domain vanishing polynomial and `C` is the AIR constraint polynomial. This
//! is the DEEP-ALI link: a quotient `Q` exists (is a polynomial, which FRI then certifies
//! low-degree) IFF the constraints vanish on the trace domain `H`. This module validates
//! that relation + the vanishing-poly evaluation directly.
//!
//! DEFERRED (the larger, mechanical pieces of gadget 4):
//!   * `C(zeta)` itself — evaluating the leaf `FullRelationAir` constraint polynomial at the
//!     OOD row. That re-expresses the ENTIRE leaf AIR (C1/C2/C4/C6 + Poseidon2 + reduce) and
//!     is a direct mirror of the leaf `air.rs` folder, not novel soundness content.
//!   * The quotient-CHUNK recomposition across coset domains
//!     (`verifier.rs::recompose_quotient_from_chunks`, Lagrange `zps`) — coset-domain
//!     arithmetic that reassembles `Q(zeta)` from the FRI-committed chunks.
//!   * The DEEP quotient `(opened − p(zeta))/(x − zeta)` combination fed to FRI (gadget 3).

use descender_account_stark::Challenge;
use p3_field::{Field, PrimeCharacteristicRing};

/// Trace-domain vanishing polynomial at `zeta`: `Z_H(zeta) = zeta^{2^log_degree} − 1`
/// (the standard subgroup vanishing poly for `|H| = 2^log_degree`).
pub fn vanishing_poly_at(zeta: Challenge, log_degree: usize) -> Challenge {
    zeta.exp_power_of_2(log_degree) - Challenge::ONE
}

/// The DEEP-ALI out-of-domain check: `C(zeta) == Z_H(zeta) · Q(zeta)`. `constraint_eval`
/// is `C(zeta)` (supplied by evaluating the leaf AIR folder — deferred), `quotient_at_zeta`
/// is `Q(zeta)` (recomposed from the committed chunks — deferred).
pub fn verify_quotient_relation(
    constraint_eval: Challenge,
    quotient_at_zeta: Challenge,
    zeta: Challenge,
    log_degree: usize,
) -> bool {
    constraint_eval == vanishing_poly_at(zeta, log_degree) * quotient_at_zeta
}

/// Equivalent native form `C(zeta) · Z_H(zeta)^{-1} == Q(zeta)` (matches
/// `verifier.rs:195`), valid off the trace domain where `Z_H(zeta) != 0`.
pub fn verify_quotient_relation_div(
    constraint_eval: Challenge,
    quotient_at_zeta: Challenge,
    zeta: Challenge,
    log_degree: usize,
) -> bool {
    let zh = vanishing_poly_at(zeta, log_degree);
    debug_assert_ne!(zh, Challenge::ZERO, "zeta must be off the trace domain");
    constraint_eval * zh.inverse() == quotient_at_zeta
}

#[cfg(test)]
mod tests {
    use super::*;
    use descender_account_stark::Val;
    use p3_field::TwoAdicField;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }

    /// SOUNDNESS CORE: an honest `(C, Q)` with `C = Z_H(zeta)·Q` passes; tampering either
    /// side fails. This is the DEEP-ALI quotient relation — the link from "constraints
    /// vanish on H" to "FRI tests a real low-degree quotient".
    #[test]
    fn honest_quotient_relation_holds_tampers_rejected() {
        let log_degree = 10;
        let zeta = ext(123_457); // off-domain OOD point
        let q = ext(98_765);
        let c = vanishing_poly_at(zeta, log_degree) * q; // honest constraint eval
        assert!(verify_quotient_relation(c, q, zeta, log_degree));
        assert!(verify_quotient_relation_div(c, q, zeta, log_degree));
        // tampered constraint eval / quotient must be rejected
        assert!(!verify_quotient_relation(
            c + Challenge::ONE,
            q,
            zeta,
            log_degree
        ));
        assert!(!verify_quotient_relation(
            c,
            q + Challenge::ONE,
            zeta,
            log_degree
        ));
    }

    /// `Z_H` vanishes EXACTLY on the trace domain `H = <omega>` (order 2^log_degree) and is
    /// nonzero off it — the defining property the OOD check relies on (zeta must be chosen
    /// off-domain, which FS sampling over the large deg-8 field gives w.h.p.).
    #[test]
    fn vanishing_poly_zeroes_on_domain_only() {
        let log_degree = 8;
        let omega = Val::two_adic_generator(log_degree); // primitive 2^log_degree root
                                                         // On-domain points omega^k: Z_H = 0.
        for k in [0u64, 1, 7, 100, 255] {
            let pt = Challenge::from(omega.exp_u64(k));
            assert_eq!(
                vanishing_poly_at(pt, log_degree),
                Challenge::ZERO,
                "Z_H(omega^{k}) != 0"
            );
        }
        // Off-domain point: Z_H != 0.
        let off = ext(3) * Challenge::from(omega) + Challenge::ONE; // not a pure root of unity
        assert_ne!(vanishing_poly_at(off, log_degree), Challenge::ZERO);
    }

    /// A false statement (constraints do NOT vanish on H) forces a non-polynomial quotient:
    /// for a fixed honest `Q`, a constraint eval that is NOT `Z_H(zeta)·Q` is rejected — the
    /// prover cannot pass the OOD check without a genuine quotient. (Soundness by attempted
    /// forgery.)
    #[test]
    fn false_constraint_eval_cannot_pass() {
        let log_degree = 12;
        let zeta = ext(777);
        let q = ext(42);
        let honest_c = vanishing_poly_at(zeta, log_degree) * q;
        // Any C' != honest_c fails for this (Q, zeta).
        for delta in [2u32, 5, 9999] {
            assert!(!verify_quotient_relation(
                honest_c + ext(delta),
                q,
                zeta,
                log_degree
            ));
        }
    }
}
