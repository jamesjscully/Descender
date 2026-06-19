//! Gadget 2 building block — the in-circuit Poseidon2 permutation (compression sub-AIR).
//!
//! The leaf STARK's Merkle node compression is `MyCompress = TruncatedPermutation<Perm, 2,
//! 8, 16>` — one width-16 Poseidon2 permutation applied to `[left(8) ‖ right(8)]`, output
//! truncated to the first 8 lanes. To constrain that compression IN-CIRCUIT (what gadget 1
//! deferred and gadget 2's path-fold needs), we reuse `p3-poseidon2-air`'s `Poseidon2Air`,
//! configured with the SAME const shape as the leaf's Poseidon2-16 (SBOX_DEGREE=3,
//! SBOX_REGISTERS=0, HALF_FULL_ROUNDS=4, PARTIAL_ROUNDS=20, KoalaBear linear layers).
//!
//! This module validates the INTEGRATION: that `Poseidon2Air` builds, generates a trace, and
//! is accepted/rejected by the real `p3_air::check_constraints` in this crate — the heaviest
//! mechanical dependency of the recursion verifier-AIR. DEFERRED for gadget-2-proper: pin the
//! RoundConstants to `default_koalabear_poseidon2_16`'s exact constants (here the test uses a
//! fixed-seed RNG — enough to validate the AIR machinery, not yet byte-identical to the
//! deployed permutation), and wire the input lanes `[left‖right]` / output `[..8]` to the
//! merkle fold columns (`merkle.rs`).

use descender_account_stark::{Val, WIDTH};
use p3_koala_bear::GenericPoseidon2LinearLayersKoalaBear;
use p3_poseidon2_air::Poseidon2Air;

/// S-box degree of the leaf Poseidon2 (`x^3`).
pub const SBOX_DEGREE: u64 = 3;
/// S-box helper registers (0 = inline, matches the leaf config).
pub const SBOX_REGISTERS: usize = 0;
/// Half the number of full rounds (KoalaBear-16: 4 ⇒ 8 full rounds).
pub const HALF_FULL_ROUNDS: usize = 4;
/// Partial rounds (KoalaBear-16).
pub const PARTIAL_ROUNDS: usize = 20;

/// The configured in-circuit Poseidon2-16 permutation AIR — gadget-2's compression core.
/// Same const shape as the leaf `Perm = Poseidon2KoalaBear<16>`.
pub type CompressAir = Poseidon2Air<
    Val,
    GenericPoseidon2LinearLayersKoalaBear,
    WIDTH,
    SBOX_DEGREE,
    SBOX_REGISTERS,
    HALF_FULL_ROUNDS,
    PARTIAL_ROUNDS,
>;

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_poseidon2_air::RoundConstants;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    /// INTEGRATION: a Poseidon2Air honest trace is ACCEPTED and a tampered cell is REJECTED
    /// by the real Plonky3 `check_all_constraints` in this crate — proving the in-circuit
    /// permutation (gadget-2's compression core) works end-to-end here. The heaviest
    /// mechanical dependency of the verifier-AIR, de-risked.
    #[test]
    fn poseidon2_compression_air_accepts_honest_rejects_tamper() {
        let mut rng = SmallRng::seed_from_u64(0xC0FFEE_u64);
        let constants = RoundConstants::from_rng(&mut rng);
        let air = CompressAir::new(constants);

        // A power-of-two batch of permutation traces (the air samples the inputs itself).
        let trace = air.generate_trace_rows(4, 0);

        let report = check_all_constraints(&air, &trace, &[], None);
        assert!(
            report.is_ok(),
            "honest Poseidon2 compression trace failed: {} failures",
            report.failures.len()
        );

        // Tamper an interior trace cell ⇒ the permutation constraints must reject.
        let mut bad = trace.clone();
        bad.values[7] += Val::ONE;
        assert!(
            check_all_constraints(&air, &bad, &[], None).is_ok() == false,
            "tampered Poseidon2 compression trace was NOT rejected (underconstrained)"
        );
    }
}
