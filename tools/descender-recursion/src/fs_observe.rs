//! Quantum-128 Fiat-Shamir commitment observation — observing a `DIGEST_ELEMS`-wide Merkle
//! commitment into the rate-7/capacity-9 FS `DuplexChallenger` when the digest is wider than the
//! challenger rate.
//!
//! ## The conflation this resolves (the "deepest item")
//!
//! At the deployed digest-8 profile a FRI commit-phase commitment was a `DIGEST_ELEMS = 8`-element
//! digest and the FS `DuplexChallenger` rate was also 8, so observing a commitment was exactly ONE
//! full-rate absorb-permute, and `recursive_air::fs_squeeze_betas` modelled each commitment as a
//! single `[Val; RATE]` block. At the quantum-128 profile the digest widens to 13 while the
//! challenger uses rate 7 (`config::Challenger = DuplexChallenger<Val,Perm,16,7>`), so a single
//! commitment no longer fits one rate block.
//!
//! The native `p3-challenger` `DuplexChallenger` (duplex_challenger.rs:86-176) observes a
//! 13-element digest as: push elements one at a time; on the 7th, `input_buffer.len() == RATE`
//! triggers a full-rate duplexing, and the remaining 6 sit buffered until the first beta draw
//! triggers a partial duplexing. Sampling the eighth extension limb then triggers a squeeze-only
//! permutation. So one commitment plus its beta uses three permutations (7 + 6 + squeeze), not one.

use crate::c9_challenger::C9Challenger;
use crate::challenger::RATE; // ADR-0011 FS DuplexChallenger rate (= 7).
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, WIDTH};

extern crate alloc;
use alloc::vec::Vec;

/// SQUEEZE the FRI commit-phase folding challenges `beta_0 .. beta_{R-1}` CONTINUING from
/// `start_state`, observing each commit-phase commitment as a full `DIGEST_ELEMS`-wide Merkle
/// digest (q128: 13 elements over rate 7 ⇒ a full block + a trailing partial). Mirrors the native
/// `DuplexChallenger` observe(digest)+sample(beta) schedule constant-for-constant. The carried
/// sponge state threads round → round, so changing any commitment changes its beta and every later
/// beta. Returns `(betas, post_state)`.
///
/// This is the q128 replacement for `recursive_air::fs_squeeze_betas` (which observes `[Val; RATE]`
/// blocks, valid only when `DIGEST_ELEMS == RATE`).
pub fn fs_squeeze_betas_digest(
    start_state: [Val; WIDTH],
    commit_digests: &[[Val; DIGEST_ELEMS]],
) -> (Vec<Challenge>, [Val; WIDTH]) {
    debug_assert_eq!(RATE, 7);
    let mut challenger = C9Challenger::from_state(start_state);
    let mut betas = Vec::with_capacity(commit_digests.len());
    for digest in commit_digests {
        challenger.observe_slice(digest);
        betas.push(challenger.sample_extension());
    }
    (betas, challenger.state())
}

/// ADR-0011 replacement beta schedule over rate 7 / capacity 9.
///
/// Each 13-element commitment is observed by the faithful rate-7 challenger,
/// then one degree-8 extension challenge is sampled. The eighth extension limb
/// necessarily comes from a squeeze-only permutation. Any six outputs left
/// buffered after that sample are discarded by the next commitment observe,
/// matching Plonky3's `CanObserve` implementation.
pub fn fs_squeeze_betas_digest_c9(
    start_state: [Val; WIDTH],
    commit_digests: &[[Val; DIGEST_ELEMS]],
) -> (Vec<Challenge>, [Val; WIDTH]) {
    fs_squeeze_betas_digest(start_state, commit_digests)
}

#[cfg(test)]
mod tests {
    use super::*;
    use descender_account_stark::{Challenger, Perm};
    use p3_challenger::{CanObserve, CanSample};
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};
    use p3_koala_bear::default_koalabear_poseidon2_16;

    fn digest_block(seed: u32) -> [Val; DIGEST_ELEMS] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    /// FIDELITY PIN: `fs_squeeze_betas_digest` reproduces the native `DuplexChallenger<Val,Perm,16,7>`
    /// observing each 13-element commitment digest and sampling a `Challenge` beta — i.e. the
    /// digest-13 / rate-7 observe schedule (full block + trailing partial + extension refill) is
    /// byte-faithful. This is
    /// what makes the "deepest" FS item load-bearing rather than asserted.
    #[test]
    fn fs_squeeze_betas_digest_matches_native_challenger() {
        for beta_rounds in [1usize, 2, 3, 5] {
            let comms: Vec<[Val; DIGEST_ELEMS]> = (0..beta_rounds as u32)
                .map(|r| digest_block(200 + r))
                .collect();

            // native: fresh challenger, per round observe(all 13 digest elems) then sample beta_r.
            let mut ch = Challenger::new(default_koalabear_poseidon2_16());
            let betas_native: Vec<Challenge> = comms
                .iter()
                .map(|comm| {
                    for &x in comm.iter() {
                        ch.observe(x);
                    }
                    Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() })
                })
                .collect();

            // ours: continue from the all-zero boundary (matches the fresh native challenger).
            let (betas, _) = fs_squeeze_betas_digest([Val::ZERO; WIDTH], &comms);
            assert_eq!(betas.len(), beta_rounds);
            for (r, (b, bn)) in betas.iter().zip(betas_native.iter()).enumerate() {
                assert_eq!(
                    b, bn,
                    "FS beta_{r} diverges from native challenger (rounds={beta_rounds})"
                );
            }
        }
    }

    /// Non-vacuous: changing one commitment digest changes its beta AND every subsequent beta (the
    /// carried sponge threads forward) — so a tampered FRI-layer commitment cannot be absorbed for free.
    #[test]
    fn changing_a_commitment_changes_that_and_later_betas() {
        let comms: Vec<[Val; DIGEST_ELEMS]> = (0..4u32).map(|r| digest_block(50 + r)).collect();
        let (base, _) = fs_squeeze_betas_digest([Val::ZERO; WIDTH], &comms);
        let mut tampered = comms.clone();
        tampered[1][0] += Val::ONE; // perturb round-1's commitment
        let (got, _) = fs_squeeze_betas_digest([Val::ZERO; WIDTH], &tampered);
        assert_eq!(
            got[0], base[0],
            "round-0 beta must be unaffected by a round-1 change"
        );
        assert_ne!(got[1], base[1], "round-1 beta must change");
        assert_ne!(got[2], base[2], "round-2 beta must change (carried state)");
        assert_ne!(got[3], base[3], "round-3 beta must change (carried state)");
    }

    #[test]
    fn fs_squeeze_betas_digest_c9_matches_native_rate7_challenger() {
        use crate::c9_challenger::C9_RATE;
        use p3_challenger::DuplexChallenger;

        type NativeC9 = DuplexChallenger<Val, Perm, WIDTH, C9_RATE>;

        for beta_rounds in [1usize, 2, 3, 5] {
            let comms: Vec<[Val; DIGEST_ELEMS]> = (0..beta_rounds as u32)
                .map(|round| digest_block(500 + round))
                .collect();

            let mut native = NativeC9::new(default_koalabear_poseidon2_16());
            let native_betas: Vec<Challenge> = comms
                .iter()
                .map(|digest| {
                    for &value in digest {
                        native.observe(value);
                    }
                    native.sample()
                })
                .collect();

            let (betas, state) = fs_squeeze_betas_digest_c9([Val::ZERO; WIDTH], &comms);
            assert_eq!(betas, native_betas);
            assert_eq!(state, native.sponge_state);
        }
    }
}
