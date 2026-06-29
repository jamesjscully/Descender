//! Quantum-128 Fiat-Shamir commitment observation — observing a `DIGEST_ELEMS`-wide Merkle
//! commitment into the FS `DuplexChallenger` when the digest is WIDER than the challenger rate.
//!
//! ## The conflation this resolves (the "deepest item")
//!
//! At the deployed digest-8 profile a FRI commit-phase commitment was a `DIGEST_ELEMS = 8`-element
//! digest and the FS `DuplexChallenger` rate was also 8, so observing a commitment was exactly ONE
//! full-rate absorb-permute, and `recursive_air::fs_squeeze_betas` modelled each commitment as a
//! single `[Val; RATE]` block. At the quantum-128 profile the digest widens to 13 while the
//! challenger KEEPS rate 8 (`config::Challenger = DuplexChallenger<Val,Perm,16,8>`), so a single
//! commitment no longer fits one rate block.
//!
//! The native `p3-challenger` `DuplexChallenger` (duplex_challenger.rs:86-176) observes a
//! 13-element digest as: push elements one at a time; on the 8th, `input_buffer.len() == RATE`
//! triggers a `duplexing()` (a FULL-rate absorb-permute, with the length tag `state[RATE] += 8` and
//! `state[8..8]` empty); the remaining 5 sit buffered; then the FIRST `sample()` of `beta_r` finds a
//! non-empty input buffer and triggers a second `duplexing()` (a PARTIAL absorb of 5: overwrite
//! `state[0..5]`, zero `state[5..8]`, length tag `state[8] += 5`, permute). So observing one
//! commitment is TWO permutes (8 + 5), not one — a change to the FS observation SCHEDULE that the
//! in-AIR FS mirror must follow. This module gives that schedule a tested reference.

use crate::challenger::RATE; // FS DuplexChallenger rate (= 8 at q128, unchanged from deployed).
use crate::inlined_perm;
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, WIDTH};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

/// One faithful `DuplexChallenger::duplexing()` (duplex_challenger.rs:86-112): overwrite `state[0..n]`
/// with `block[0..n]`, zero `state[n..RATE]`, add the length tag `state[RATE] += n` (only when
/// `n > 0`), then permute. `n == 0` is a squeeze-only permute (no rate overwrite, no tag).
fn duplexing(state: [Val; WIDTH], block: &[Val], n: usize) -> [Val; WIDTH] {
    debug_assert!(n <= RATE && n <= block.len());
    let mut s = state;
    for i in 0..n {
        s[i] = block[i];
    }
    if n > 0 {
        for i in n..RATE {
            s[i] = Val::ZERO;
        }
        s[RATE] += Val::from_u8(n as u8);
    }
    let consts = inlined_perm::Constants::pinned();
    let cols = inlined_perm::generate_perm_cols(s, &consts);
    inlined_perm::perm_output(&cols)
}

/// SQUEEZE the FRI commit-phase folding challenges `beta_0 .. beta_{R-1}` CONTINUING from
/// `start_state`, observing each commit-phase commitment as a full `DIGEST_ELEMS`-wide Merkle
/// digest (q128: 13 elements over rate 8 ⇒ a full block + a trailing partial). Mirrors the native
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
    let full = DIGEST_ELEMS / RATE; // number of full rate blocks in one digest
    let rem = DIGEST_ELEMS % RATE; // trailing partial absorb length
    let mut state = start_state;
    let mut betas = Vec::with_capacity(commit_digests.len());
    for digest in commit_digests {
        // observe(digest): full RATE blocks duplex on each 8th element pushed...
        for b in 0..full {
            state = duplexing(state, &digest[b * RATE..], RATE);
        }
        // ...and the trailing `rem` elements duplex on the FIRST sample() of beta_r.
        // (If rem == 0 the sample finds a full output buffer and empty input ⇒ no extra permute;
        //  beta is the LIFO of the last full block — handled by skipping the partial.)
        if rem > 0 {
            state = duplexing(state, &digest[full * RATE..], rem);
        }
        // beta_r: LIFO pops of the rate (sample_algebra_element = D base draws; D == RATE == 8).
        betas.push(Challenge::from_basis_coefficients_fn(|i| state[RATE - 1 - i]));
    }
    (betas, state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use descender_account_stark::Challenger;
    use p3_challenger::{CanObserve, CanSample};
    use p3_koala_bear::default_koalabear_poseidon2_16;

    fn digest_block(seed: u32) -> [Val; DIGEST_ELEMS] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    /// FIDELITY PIN: `fs_squeeze_betas_digest` reproduces the native `DuplexChallenger<Val,Perm,16,8>`
    /// observing each 13-element commitment digest and sampling a `Challenge` beta — i.e. the
    /// digest-13 / rate-8 observe schedule (full block + trailing partial) is byte-faithful. This is
    /// what makes the "deepest" FS item load-bearing rather than asserted.
    #[test]
    fn fs_squeeze_betas_digest_matches_native_challenger() {
        for beta_rounds in [1usize, 2, 3, 5] {
            let comms: Vec<[Val; DIGEST_ELEMS]> =
                (0..beta_rounds as u32).map(|r| digest_block(200 + r)).collect();

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
                assert_eq!(b, bn, "FS beta_{r} diverges from native challenger (rounds={beta_rounds})");
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
        assert_eq!(got[0], base[0], "round-0 beta must be unaffected by a round-1 change");
        assert_ne!(got[1], base[1], "round-1 beta must change");
        assert_ne!(got[2], base[2], "round-2 beta must change (carried state)");
        assert_ne!(got[3], base[3], "round-3 beta must change (carried state)");
    }
}
