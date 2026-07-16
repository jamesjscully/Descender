//! Quantum-128 leaf-row hashing primitive — the `MyHash = PaddingFreeSponge<Perm,16,7,13>`
//! sponge at rate 7 / capacity 9 / digest 13, mirrored host-side and in-AIR.
//!
//! ## The conflation this resolves
//!
//! At the deployed digest-8 profile THREE distinct quantities all equalled 8, so the recursion
//! code reused a single constant for all of them:
//!   * the FS `DuplexChallenger` rate (`challenger::RATE` — observes the transcript),
//!   * the Merkle hash output width (`DIGEST_ELEMS`),
//!   * the leaf-row hash (`MyHash`) sponge rate.
//! At the quantum-128 profile the output width separates from the rate
//! (`research/path2_quantum128_parameterization_v1.md`): both the challenger and `MyHash` use
//! rate 7/capacity 9, while the Merkle digest is 13 elements. This module gives the leaf-hash
//! rate its own semantic name (`LEAF_HASH_RATE`) and a fidelity test against the real
//! `PaddingFreeSponge`, so future changes cannot silently re-conflate sponge rate and digest width.

use crate::inlined_perm;
use descender_account_stark::{Val, DIGEST_ELEMS, WIDTH};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

/// The leaf-row hash (`config::MyHash = PaddingFreeSponge<Perm,16,7,13>`) sponge RATE. It equals
/// the current FS challenger rate by profile choice but is semantically distinct from it and from
/// the Merkle digest width (`DIGEST_ELEMS = 13`). Capacity is
/// `WIDTH - LEAF_HASH_RATE = 9` (`c/2 ≈ 139`-bit
/// indifferentiability). Pinned to the leaf config by [`tests::leaf_hash_matches_real_padding_free_sponge`].
pub const LEAF_HASH_RATE: usize = 7;
const _: () = assert!(LEAF_HASH_RATE < WIDTH);
const _: () = assert!(
    WIDTH - LEAF_HASH_RATE == 9,
    "q128 leaf-hash capacity must be c = 9"
);

/// Number of Poseidon2 absorb-permutes `PaddingFreeSponge::hash_iter` performs over `m` input
/// elements at `LEAF_HASH_RATE` (one per full or trailing-partial block; `>= 1`).
pub const fn lh_perms(m: usize) -> usize {
    if m == 0 {
        1
    } else {
        m.div_ceil(LEAF_HASH_RATE)
    }
}

/// The per-permute PRE-permute input states of `MyHash(input)`, mirroring `p3-symmetric`'s
/// `PaddingFreeSponge::hash_iter` (sponge.rs:172-204) at `LEAF_HASH_RATE`: state starts all-zero;
/// each block OVERWRITES `state[0..fill]` with the chunk (no padding — a trailing partial block
/// leaves the unfilled rate + capacity lanes carrying the prior permute output); permute after
/// every block. `pre_states[k]` is permute `k`'s input.
pub fn leaf_hash_pre_states(input: &[Val]) -> Vec<[Val; WIDTH]> {
    assert!(!input.is_empty(), "leaf hash input must be non-empty");
    let consts = inlined_perm::Constants::pinned();
    let mut pre_states = Vec::new();
    let mut state = [Val::ZERO; WIDTH];
    let n = input.len();
    let mut pos = 0usize;
    loop {
        let fill = core::cmp::min(LEAF_HASH_RATE, n - pos);
        for i in 0..fill {
            state[i] = input[pos + i]; // overwrite absorption
        }
        pos += fill;
        pre_states.push(state);
        let cols = inlined_perm::generate_perm_cols(state, &consts);
        state = inlined_perm::perm_output(&cols);
        if pos >= n {
            break;
        }
    }
    pre_states
}

/// The leaf digest `MyHash(input)` — first `DIGEST_ELEMS` lanes of the final permute output.
pub fn leaf_hash_digest(input: &[Val]) -> [Val; DIGEST_ELEMS] {
    let consts = inlined_perm::Constants::pinned();
    let pre = leaf_hash_pre_states(input);
    let last = *pre.last().expect("at least one permute");
    let cols = inlined_perm::generate_perm_cols(last, &consts);
    let out = inlined_perm::perm_output(&cols);
    core::array::from_fn(|i| out[i])
}

#[cfg(test)]
mod tests {
    use super::*;
    use descender_account_stark::Perm;
    use p3_field::PrimeCharacteristicRing;
    use p3_koala_bear::default_koalabear_poseidon2_16;
    use p3_symmetric::{CryptographicHasher, PaddingFreeSponge};

    /// FIDELITY PIN: the in-recursion leaf-hash equals the REAL q128 `MyHash`
    /// (`PaddingFreeSponge<Perm,16,7,13>`) for inputs of every length class (full blocks, partial
    /// trailing block, single block, multi-block). This is what makes `LEAF_HASH_RATE = 7` /
    /// digest-13 load-bearing rather than asserted.
    #[test]
    fn leaf_hash_matches_real_padding_free_sponge() {
        type RealMyHash = PaddingFreeSponge<Perm, 16, 7, DIGEST_ELEMS>;
        let perm: Perm = default_koalabear_poseidon2_16();
        let real = RealMyHash::new(perm);
        // lengths spanning: < rate, == rate, rate+partial, several full blocks, full multiple.
        for &len in &[1usize, 6, 7, 8, 13, 14, 20, 21, 26] {
            let input: Vec<Val> = (0..len as u32)
                .map(|i| Val::from_u32(i.wrapping_mul(2654435761).wrapping_add(1)))
                .collect();
            let got = leaf_hash_digest(&input);
            let want: [Val; DIGEST_ELEMS] = real.hash_iter(input.iter().copied());
            assert_eq!(got, want, "leaf_hash_digest != real MyHash at len {len}");
            // perm count matches the sponge schedule.
            assert_eq!(
                leaf_hash_pre_states(&input).len(),
                lh_perms(len),
                "perm count mismatch at len {len}"
            );
        }
    }

    /// The salted layer-leaf input length `2·D + SALT_ELEMS = 20` (the dominant recursion case)
    /// absorbs over rate 7 in `ceil(20/7) = 3` permutes — confirm the count and digest width.
    #[test]
    fn salted_layer_leaf_input_schedule() {
        let m = 2 * crate::fri_fold_air::D + descender_account_stark::SALT_ELEMS; // 16 + 4 = 20
        assert_eq!(
            lh_perms(m),
            3,
            "salted layer-leaf must be a 3-permute rate-7 sponge"
        );
        let input: Vec<Val> = (0..m as u32).map(Val::from_u32).collect();
        assert_eq!(leaf_hash_digest(&input).len(), DIGEST_ELEMS);
    }

    /// Determinism + collision sensitivity: flipping any input element changes the digest.
    #[test]
    fn leaf_hash_binds_every_input_element() {
        let base: Vec<Val> = (0..20u32).map(|i| Val::from_u32(i + 1)).collect();
        let d = leaf_hash_digest(&base);
        assert_eq!(d, leaf_hash_digest(&base), "must be deterministic");
        for k in 0..base.len() {
            let mut t = base.clone();
            t[k] += Val::ONE;
            assert_ne!(
                leaf_hash_digest(&t),
                d,
                "digest must bind input element {k}"
            );
        }
    }
}
