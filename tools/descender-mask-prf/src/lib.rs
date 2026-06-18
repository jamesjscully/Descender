//! # descender-mask-prf
//!
//! Standalone (NO Plonky3) projection-mask PRF for the DEPLOYED signer.
//!
//! The account-creation ZK-STARK proof (`tools/descender-account-stark`, which
//! depends on Plonky3) derives the per-account projection masks as:
//!
//!   `masks[6] = wide_then_reduce( Poseidon2-sponge( pad_seed[32] ‖ slot[8] ) )`
//!
//! using Plonky3's `default_koalabear_poseidon2_16` Poseidon2 over KoalaBear. The
//! deployed signer (`tools/parameter-search`) must NOT depend on Plonky3, so this
//! crate re-implements that derivation FROM SCRATCH with the SAME pinned
//! constants, and a separate dev test
//! (`../descender-account-stark/tests/mask_prf_equivalence.rs`) proves the two are
//! byte-identical for many random inputs — the drift-prevention guarantee.
//!
//! ## Public API
//!
//! * [`projection_masks`] `(pad_seed: &[u8;32], slot_id: u64) -> [u16; 6]`
//!   — the 6 q12289 projection masks.
//! * [`poseidon2::permute`] / [`poseidon2::permute_u32`] — the width-16 Poseidon2
//!   permutation (byte-identical to `default_koalabear_poseidon2_16`).
//! * [`mask::squeezed_lanes`] / [`mask::projection_masks_u32`] — intermediate hooks
//!   used by the equivalence test.

pub mod field;
pub mod mask;
pub mod poseidon2;

pub use mask::{poseidon2_hash_bytes, projection_masks, Q12289};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::F;

    /// KAT: the standalone permutation must match Plonky3's published test vector
    /// for `default_koalabear_poseidon2_16` (from
    /// `p3-koala-bear-0.6.1/src/poseidon2.rs::test_default_koalabear_poseidon2_width_16`).
    /// This anchors byte-identity to Plonky3 WITHOUT needing a p3 dependency here.
    #[test]
    fn permutation_matches_plonky3_published_kat() {
        let input: [u32; 16] = [
            894848333, 1437655012, 1200606629, 1690012884, 71131202, 1749206695, 1717947831,
            120589055, 19776022, 42382981, 1831865506, 724844064, 171220207, 1299207443,
            227047920, 1783754913,
        ];
        let expected: [u32; 16] = [
            1934285469, 604889435, 133449501, 1026180808, 1830659359, 176667110, 1391183747,
            351743874, 1238264085, 1292768839, 2023573270, 1201586780, 1360691759, 1230682461,
            748270449, 651545025,
        ];
        let got = poseidon2::permute_u32(&input);
        assert_eq!(got, expected, "standalone Poseidon2 != Plonky3 KAT");
    }

    /// Field sanity: a multiplication KAT lifted from p3-koala-bear's `test_koala_bear`.
    #[test]
    fn field_mul_kat() {
        let m1 = F::new(0x34167c58);
        let m2 = F::new(0x61f3207b);
        assert_eq!(m1.mul(m2).as_u32(), 0x54b46b81);
    }

    /// Field inverse sanity.
    #[test]
    fn field_inverse() {
        for x in [1u32, 2, 3, 7, 12289, 0x7f000000, 1420470955] {
            let f = F::new(x);
            assert_eq!(f.mul(f.inverse()).as_u32(), 1);
        }
    }

    /// Masks are always canonical q12289 residues.
    #[test]
    fn masks_in_range() {
        for s in 0u64..64 {
            let pad: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(31).wrapping_add(s as u8));
            let m = projection_masks(&pad, s);
            for &mj in m.iter() {
                assert!(mj < Q12289 as u16, "mask {mj} >= q12289");
            }
        }
    }

    /// WIDE-THEN-REDUCE bias property: a value `W` uniform over `[0, p^5)` reduced
    /// mod q has statistical distance from uniform `<= q / p^5 ~ 2^-141`. We can't
    /// observe 2^-141 empirically, but we CAN check the structural premise the
    /// bound rests on: `p^5` dwarfs `q` by the claimed margin, i.e.
    /// `log2(p^5 / q) >= 141`. (p = 2^31 - 2^24 + 1, q = 12289.)
    #[test]
    fn wide_reduce_bias_margin_at_least_141_bits() {
        // p^5 / q, in bits. Use f64 logs (exact enough at this scale).
        let p = field::P as f64;
        let q = Q12289 as f64;
        let log2_p5_over_q = 5.0 * p.log2() - q.log2();
        assert!(
            log2_p5_over_q >= 141.0,
            "wide-reduce bias margin {log2_p5_over_q} bits < 141"
        );
    }

    /// Small empirical uniformity sanity: over many (pad_seed, slot) inputs, the
    /// per-lane masks should spread across the q12289 range — not collapse to a
    /// constant or a tiny subrange. This is a cheap smoke test, NOT a statistical
    /// proof. We bucket each lane's outputs into 16 bins and require every bin to
    /// receive at least one sample (a constant/degenerate PRF would fail this).
    #[test]
    fn mask_uniformity_smoke() {
        const N: u64 = 4096;
        const BINS: usize = 16;
        let bin_width = (Q12289 as usize).div_ceil(BINS);
        let mut hit = [[false; BINS]; mask::NUM_OUTPUTS];
        for s in 0..N {
            let pad: [u8; 32] =
                core::array::from_fn(|i| ((i as u64).wrapping_mul(0x9E3779B1).wrapping_add(s.wrapping_mul(7)) & 0xff) as u8);
            let m = projection_masks(&pad, s);
            for j in 0..mask::NUM_OUTPUTS {
                let b = (m[j] as usize / bin_width).min(BINS - 1);
                hit[j][b] = true;
            }
        }
        for j in 0..mask::NUM_OUTPUTS {
            let covered = hit[j].iter().filter(|&&h| h).count();
            assert_eq!(
                covered, BINS,
                "lane {j}: only {covered}/{BINS} bins hit over {N} samples (suspicious non-uniformity)"
            );
        }
    }

    /// Determinism: same input -> same masks.
    #[test]
    fn masks_deterministic() {
        let pad = [0xABu8; 32];
        assert_eq!(projection_masks(&pad, 42), projection_masks(&pad, 42));
        assert_ne!(projection_masks(&pad, 42), projection_masks(&pad, 43));
    }

    /// Behavioural lock for the FS-challenge byte-hash: deterministic, domain-
    /// separated, input-sensitive, and decorrelated from the mask-PRF sponge.
    #[test]
    fn poseidon2_hash_bytes_properties() {
        let dom = b"Descender.test.fs.v1";
        let other_dom = b"Descender.test.fs.v2";
        let input = b"the quick brown fox checked-core frame bytes";
        let h1 = poseidon2_hash_bytes::<31>(dom, input);
        // deterministic
        assert_eq!(h1, poseidon2_hash_bytes::<31>(dom, input));
        // domain separation
        assert_ne!(h1, poseidon2_hash_bytes::<31>(other_dom, input));
        // input sensitivity (flip one byte)
        let mut input2 = input.to_vec();
        input2[0] ^= 1;
        assert_ne!(h1, poseidon2_hash_bytes::<31>(dom, &input2));
        // length sensitivity
        assert_ne!(h1, poseidon2_hash_bytes::<31>(dom, &input[..input.len() - 1]));
        // not all-zero
        assert_ne!(h1, [0u8; 31]);
    }
}
