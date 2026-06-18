//! DRIFT-PREVENTION EQUIVALENCE TEST.
//!
//! Proves the standalone, NO-Plonky3 `descender-mask-prf` crate (which the
//! deployed signer will depend on) is BYTE-IDENTICAL to the Plonky3-based
//! account-creation PROOF, at two levels:
//!
//!   1. PERMUTATION: the standalone width-16 Poseidon2
//!      (`descender_mask_prf::poseidon2::permute_u32`) == Plonky3's
//!      `default_koalabear_poseidon2_16().permute_mut` for many random width-16
//!      inputs.
//!
//!   2. MASK DERIVATION: the standalone `projection_masks(pad_seed, slot)` ==
//!      the proof's reference C2 derivation
//!      (`descender_account_stark::reference_projection_mask_prf`, which delegates
//!      to the SAME private trace primitives the prover uses) for many random
//!      (pad_seed, slot) — including the intermediate 30 squeezed lanes.
//!
//! If ANY constant or layout in the standalone crate diverges from Plonky3 /
//! the proof, one of these assertions fails. There is therefore no way for the
//! deployed-signer PRF and the in-proof PRF to silently drift apart.

use p3_field::{PrimeCharacteristicRing, PrimeField32};
use p3_koala_bear::{default_koalabear_poseidon2_16, KoalaBear};
use p3_symmetric::Permutation;

use descender_account_stark::reference_projection_mask_prf;
use descender_mask_prf::{mask, poseidon2, projection_masks};

/// A tiny deterministic xorshift RNG (no rng dev-dep needed; reproducible KAT).
struct XorShift64(u64);
impl XorShift64 {
    fn new(seed: u64) -> Self {
        XorShift64(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    fn fill32(&mut self, out: &mut [u8; 32]) {
        for chunk in out.chunks_mut(8) {
            let v = self.next_u64().to_le_bytes();
            chunk.copy_from_slice(&v[..chunk.len()]);
        }
    }
}

const P: u32 = 0x7f00_0001; // KoalaBear prime

/// (1) Standalone Poseidon2 permutation == Plonky3 default_koalabear_poseidon2_16.
#[test]
fn standalone_permutation_byte_identical_to_plonky3() {
    let perm = default_koalabear_poseidon2_16();
    let mut rng = XorShift64::new(0xC0FFEE_1234_5678);

    const TRIALS: usize = 2000;
    for _ in 0..TRIALS {
        // Random canonical width-16 input.
        let input_u32: [u32; 16] = core::array::from_fn(|_| rng.next_u32() % P);

        // Plonky3 reference.
        let mut p3_state: [KoalaBear; 16] =
            core::array::from_fn(|i| KoalaBear::from_u32(input_u32[i]));
        perm.permute_mut(&mut p3_state);
        let p3_out: [u32; 16] = core::array::from_fn(|i| p3_state[i].as_canonical_u32());

        // Standalone.
        let standalone_out = poseidon2::permute_u32(&input_u32);

        assert_eq!(
            standalone_out, p3_out,
            "standalone Poseidon2 != Plonky3 default_koalabear_poseidon2_16 on input {input_u32:?}"
        );
    }

    // Also pin the published Plonky3 KAT explicitly.
    let kat_in: [u32; 16] = [
        894848333, 1437655012, 1200606629, 1690012884, 71131202, 1749206695, 1717947831,
        120589055, 19776022, 42382981, 1831865506, 724844064, 171220207, 1299207443, 227047920,
        1783754913,
    ];
    let mut p3_state: [KoalaBear; 16] = core::array::from_fn(|i| KoalaBear::from_u32(kat_in[i]));
    perm.permute_mut(&mut p3_state);
    let p3_out: [u32; 16] = core::array::from_fn(|i| p3_state[i].as_canonical_u32());
    assert_eq!(poseidon2::permute_u32(&kat_in), p3_out);
}

/// (2) Standalone projection_masks == the proof's in-circuit C2 derivation,
/// including the intermediate 30 squeezed lanes.
#[test]
fn standalone_masks_byte_identical_to_proof() {
    let mut rng = XorShift64::new(0xDEAD_BEEF_FEED_FACE);

    const TRIALS: usize = 2000;
    for _ in 0..TRIALS {
        let mut pad_seed = [0u8; 32];
        rng.fill32(&mut pad_seed);
        let slot_id = rng.next_u64();

        // Proof's reference (delegates to the prover's own trace primitives).
        let (proof_lanes, proof_masks) = reference_projection_mask_prf(&pad_seed, slot_id);

        // Standalone derivation (NO Plonky3).
        let standalone_lanes = mask::squeezed_lanes(&pad_seed, slot_id);
        let standalone_masks_u32 = mask::projection_masks_u32(&pad_seed, slot_id);
        let standalone_masks_u16 = projection_masks(&pad_seed, slot_id);

        assert_eq!(
            standalone_lanes, proof_lanes,
            "squeezed lanes diverge for pad_seed={pad_seed:?} slot={slot_id}"
        );
        assert_eq!(
            standalone_masks_u32, proof_masks,
            "reduced masks diverge for pad_seed={pad_seed:?} slot={slot_id}"
        );
        // u16 public API agrees with the u32 internal masks.
        for j in 0..proof_masks.len() {
            assert_eq!(standalone_masks_u16[j] as u32, proof_masks[j]);
        }
    }
}

/// Edge inputs: all-zero / all-FF pad_seed, slot 0 and u64::MAX.
#[test]
fn standalone_masks_match_proof_on_edge_inputs() {
    let cases: [([u8; 32], u64); 4] = [
        ([0x00; 32], 0),
        ([0xFF; 32], u64::MAX),
        ([0x00; 32], u64::MAX),
        ([0xFF; 32], 0),
    ];
    for (pad_seed, slot_id) in cases {
        let (proof_lanes, proof_masks) = reference_projection_mask_prf(&pad_seed, slot_id);
        assert_eq!(mask::squeezed_lanes(&pad_seed, slot_id), proof_lanes);
        assert_eq!(mask::projection_masks_u32(&pad_seed, slot_id), proof_masks);
    }
}
