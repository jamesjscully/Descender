//! Sponge-mode 2-to-1 Merkle node compression for the quantum-128 binding profile.
//!
//! The deployed `MyCompress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, 16>`
//! compresses two child digests by applying the width-16 permutation to their
//! concatenation, so it structurally requires `2 * DIGEST_ELEMS <= WIDTH = 16` —
//! i.e. the Merkle digest is capped at 8 KoalaBear elements (~248 bits → ~83-bit
//! quantum BHT collision). Quantum-128 binding needs a >= 13-element digest
//! (`research/path2_quantum128_parameterization_v1.md`), which does NOT fit a
//! 2-to-1 truncated permutation (2*13 = 26 > 16; even width-24 caps at 12 ⇒ only
//! ~124-bit quantum).
//!
//! This module provides the fix: a 2-to-1 compression that hashes the two child
//! digests THROUGH a sponge hasher (e.g. `PaddingFreeSponge<Perm, 16, 7, 13>` —
//! rate 7, capacity c=9, output 13) and squeezes a wide digest. It reuses the SAME
//! vetted width-16 Poseidon2 permutation (no new round constants), but decouples
//! the digest width from the `2*d <= WIDTH` constraint. Cost: more permutations per
//! Merkle node (absorb 26 elements over rate 7 = 4 absorb blocks, vs one truncated
//! perm), which the in-AIR `merkle_verify` recursion gadget must mirror.
//!
//! This component is additive and standalone; wiring it into `HidingValMmcs` in
//! place of `MyCompress` (with `DIGEST_ELEMS = 13`, `RATE = 7`) is the q128
//! re-parameterization step and is NOT done here.

use p3_symmetric::{CryptographicHasher, PseudoCompressionFunction};

/// A 2-to-1 pseudo-compression that absorbs `input[0] ‖ input[1]` through the
/// sponge hasher `H` and returns its `OUT`-element squeeze. Unlike
/// `TruncatedPermutation`, the output width is the sponge's `OUT`, not bounded by
/// `WIDTH / 2`.
#[derive(Clone, Debug)]
pub struct SpongeCompression<H> {
    hasher: H,
}

impl<H> SpongeCompression<H> {
    /// Wrap a sponge hasher as a 2-to-1 Merkle node compressor.
    pub const fn new(hasher: H) -> Self {
        Self { hasher }
    }
}

impl<H, F, const OUT: usize> PseudoCompressionFunction<[F; OUT], 2> for SpongeCompression<H>
where
    H: CryptographicHasher<F, [F; OUT]> + Clone,
    F: Clone,
{
    fn compress(&self, input: [[F; OUT]; 2]) -> [F; OUT] {
        let [a, b] = input;
        self.hasher.hash_iter(a.into_iter().chain(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Perm, Val};
    use p3_field::PrimeCharacteristicRing;
    use p3_koala_bear::default_koalabear_poseidon2_16;
    use p3_symmetric::PaddingFreeSponge;

    /// q128 binding sponge-mode 2-to-1 compression: WIDTH=16, RATE=7 (capacity
    /// c=9 ⇒ c/2 ≈ 139-bit indifferentiability), OUT=13 (≈ 403 digest bits ⇒ BHT
    /// quantum collision ≈ 134 > 128). Same vetted width-16 permutation.
    type Q128Hash = PaddingFreeSponge<Perm, 16, 7, 13>;

    fn compressor() -> SpongeCompression<Q128Hash> {
        let perm: Perm = default_koalabear_poseidon2_16();
        SpongeCompression::new(Q128Hash::new(perm))
    }

    #[test]
    fn q128_sponge_compression_is_deterministic_and_binds_both_children() {
        let c = compressor();
        let a: [Val; 13] = core::array::from_fn(|i| Val::from_u32(i as u32 + 1));
        let b: [Val; 13] = core::array::from_fn(|i| Val::from_u32(100 + i as u32));

        // Deterministic, full-width, not degenerate.
        let d = c.compress([a, b]);
        assert_eq!(d, c.compress([a, b]), "compression must be deterministic");
        assert_eq!(d.len(), 13, "q128 binding requires a 13-element digest");
        assert!(
            d.iter().any(|&x| x != Val::ZERO),
            "digest must not be all-zero"
        );

        // Binds child b: flipping one element of b changes the digest.
        let mut b2 = b;
        b2[0] = b2[0] + Val::ONE;
        assert_ne!(c.compress([a, b]), c.compress([a, b2]), "must bind child b");

        // Binds child a: flipping the last element of a changes the digest.
        let mut a2 = a;
        a2[12] = a2[12] + Val::ONE;
        assert_ne!(c.compress([a, b]), c.compress([a2, b]), "must bind child a");

        // Order-sensitive (a‖b != b‖a in general): not symmetric.
        assert_ne!(
            c.compress([a, b]),
            c.compress([b, a]),
            "must be order-sensitive"
        );
    }

    /// End-to-end: the q128 Merkle stack (rate-7/digest-13 leaf hash + the
    /// sponge-mode compression) composes into a real `MerkleTreeMmcs`, and a
    /// commit/open/verify roundtrip succeeds while a tampered opening is rejected.
    /// This de-risks the binding-side q128 re-parameterization: the digest-13
    /// Merkle layer is implementable end-to-end on the vetted permutation.
    #[test]
    fn q128_merkle_stack_commit_open_verify_roundtrips() {
        use p3_commit::{BatchOpeningRef, Mmcs};
        use p3_field::Field;
        use p3_matrix::dense::RowMajorMatrix;
        use p3_matrix::Dimensions;
        use p3_merkle_tree::MerkleTreeMmcs;

        type Q128ValMmcs = MerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            Q128Hash,
            SpongeCompression<Q128Hash>,
            2,
            13,
        >;

        let perm: Perm = default_koalabear_poseidon2_16();
        let mmcs = Q128ValMmcs::new(
            Q128Hash::new(perm.clone()),
            SpongeCompression::new(Q128Hash::new(perm)),
            0,
        );

        let width = 3usize;
        let height = 4usize; // power of two
        let mat = RowMajorMatrix::new(
            (0..(width * height) as u32)
                .map(Val::from_u32)
                .collect::<Vec<_>>(),
            width,
        );
        let (commit, prover_data) = mmcs.commit(vec![mat]);

        let index = 2usize;
        let (opened, proof) = mmcs.open_batch(index, &prover_data).unpack();
        let dims = [Dimensions { width, height }];
        mmcs.verify_batch(&commit, &dims, index, BatchOpeningRef::new(&opened, &proof))
            .expect("q128 Merkle opening must verify");

        // Negative: tamper one opened value ⇒ verification must fail.
        let mut bad = opened.clone();
        bad[0][0] = bad[0][0] + Val::ONE;
        assert!(
            mmcs.verify_batch(&commit, &dims, index, BatchOpeningRef::new(&bad, &proof))
                .is_err(),
            "tampered q128 Merkle opening must be rejected"
        );
    }
}
