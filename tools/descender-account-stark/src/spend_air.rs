//! Phase-2 SPEND leaf AIR (core) — proves a spend authorizes EXACTLY the packed witness the
//! account-creation leaf produced.
//!
//! # What this proves (and what it defers)
//!
//! The creation leaf ([`crate::air`], CLASS C) PUBLISHES a `packed_witness_commitment` — 32 bytes
//! that 14-bit-LE-pack the 18 canonical q12289 residues `syndrome[12] ‖ masked_projection[6]` (the
//! "reviewed-fq12 packed witness state-18" frame the deployed spend verifier consumes). For the
//! `{creation + spend}` co-verification to be sound, the spend must consume the SAME packed witness,
//! over canonical residues — otherwise creation and spend can diverge (audit MED-2).
//!
//! This AIR is the spend side of that invariant: it proves, IN-CIRCUIT, that the PUBLIC
//! `packed_witness_commitment` it carries is a well-formed packing of 18 residues, EACH canonical
//! `< q12289`, using the byte-for-byte-identical 14-bit-LE scheme as the creation leaf
//! (`pack_reviewed_fq12_packed_witness_state18_from_parts`). It then exposes the spend STATEMENT
//! (message ‖ account_id ‖ slot_id ‖ account_state_root ‖ packed_witness_commitment) as the
//! transcript-bound public values, and pins a `spend_relation_id` so a spend proof for this
//! relation/profile cannot be reinterpreted as another's.
//!
//! DEFERRED to later increments (documented, not hidden):
//!   * the Poseidon2 checked-core FRAME-DIGEST sponge that binds message/account/slot/state TO the
//!     packed witness in-circuit (the SHA→Poseidon2 rehost; today the deployed Rust spend verifier
//!     does this off-circuit — see `research/spend_fs_full_poseidon2_rehost_plan.md`);
//!   * the full Fq12 checked-core SIGNATURE equation over the syndrome (the heavy cryptographic core,
//!     performed by the deployed direct spend verifier in `parameter-search`).
//!
//! So the binding this AIR establishes is: a spend proof exists for a WELL-FORMED packed witness of
//! canonical residues, carrying a fixed statement — the consumption anchor the recursion / settlement
//! layer equates to the creation proof's `packed_witness_commitment` (closing MED-2's spend half).
//! The residue VALUES are private (the spend does not re-open them); only the public commitment and
//! the canonical-packing relation are proven, matching "no spend-time opening" in the spec.

use crate::config::Val;
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

extern crate alloc;
use alloc::vec::Vec;

// ---- Frame shape (PINNED to the creation leaf's CLASS-C frame; fidelity-checked in tests) -------

/// q12289 — the canonical residue modulus shared with creation.
pub const SPEND_Q: u32 = 12289;
/// Bits per packed residue lane (14-bit LE), matching the creation leaf's `PACKED_LANE_BITS`.
pub const SPEND_RESIDUE_BITS: usize = 14;
/// Checked-core syndrome lanes (residues 0..11).
pub const SPEND_SYNDROME_LANES: usize = 12;
/// Projection lanes (residues 12..17).
pub const SPEND_PROJECTION_LANES: usize = 6;
/// Total packed residues: syndrome[12] ‖ projection[6].
pub const SPEND_RESIDUES: usize = SPEND_SYNDROME_LANES + SPEND_PROJECTION_LANES;
/// Total residue-stream bits (18 × 14 = 252).
pub const SPEND_STREAM_BITS: usize = SPEND_RESIDUES * SPEND_RESIDUE_BITS;
/// Packed-witness commitment byte anchors (252 packed bits + 4 zero high bits).
pub const SPEND_COMMIT_BYTES: usize = 32;

const BYTE_BITS: usize = 8;

// ---- Per-residue column layout: value ‖ bits[14] ‖ slack ‖ slack_bits[14] (canonical < q) -------
//
// Identical to the creation leaf's `SYNDROME_CELLS` shape; ALL 18 residues are canonical-checked
// here (the spend consumes a fully-canonical frame).

const RESIDUE_CELLS: usize = 1 + SPEND_RESIDUE_BITS + 1 + SPEND_RESIDUE_BITS; // 30
/// Total main-trace width.
pub const SPEND_WIDTH: usize = SPEND_RESIDUES * RESIDUE_CELLS; // 540

const fn residue_base(r: usize) -> usize {
    r * RESIDUE_CELLS
}

// ---- Public-value layout (the spend statement) --------------------------------------------------

pub const PV_PACKED_OFF: usize = 0;
pub const PV_ACCOUNT_ID_OFF: usize = PV_PACKED_OFF + SPEND_COMMIT_BYTES; // 32
pub const PV_SLOT_OFF: usize = PV_ACCOUNT_ID_OFF + 32; // 64
pub const PV_STATE_ROOT_OFF: usize = PV_SLOT_OFF + 8; // 72
pub const PV_MESSAGE_OFF: usize = PV_STATE_ROOT_OFF + 32; // 104
pub const PV_RELATION_ID_OFF: usize = PV_MESSAGE_OFF + 32; // 136
pub const SPEND_RELATION_ID_ELEMS: usize = 4;
pub const SPEND_N_PUBLIC: usize = PV_RELATION_ID_OFF + SPEND_RELATION_ID_ELEMS; // 140

/// Pinned spend relation/profile id. Bump on any change to the spend relation / frame / public
/// layout. Encodes: tag "SPND" ‖ ("v1" ‖ public-width) ‖ (residues<<8 ‖ bits) ‖ q.
pub const SPEND_RELATION_ID: [u32; SPEND_RELATION_ID_ELEMS] = [
    0x5350_4E44, // "SPND"
    0x7631_0000 | (SPEND_N_PUBLIC as u32 & 0xFFFF),
    ((SPEND_RESIDUES as u32) << 8) | (SPEND_RESIDUE_BITS as u32),
    SPEND_Q,
];

fn ce<AB: AirBuilder>(v: u64) -> AB::Expr {
    AB::Expr::from(AB::F::from_u64(v))
}

/// The spend leaf AIR (core): packed-witness well-formedness + canonical residues + statement.
pub struct SpendAir;

impl<F: PrimeCharacteristicRing + Sync> BaseAir<F> for SpendAir {
    fn width(&self) -> usize {
        SPEND_WIDTH
    }
    fn num_public_values(&self) -> usize {
        SPEND_N_PUBLIC
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for SpendAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.current_slice().to_vec();
        let public = builder.public_values().to_vec();

        // The 252-bit residue stream, in residue order (residue r → bits [r*14, r*14+14)).
        let mut stream: Vec<AB::Expr> = Vec::with_capacity(SPEND_STREAM_BITS);

        for r in 0..SPEND_RESIDUES {
            let base = residue_base(r);
            let val_off = base;
            let bits_off = base + 1;
            let slack_off = base + 1 + SPEND_RESIDUE_BITS;
            let slack_bits_off = slack_off + 1;

            // value == recompose(bits[14]); each bit boolean; bits feed the stream.
            let mut val_rc = AB::Expr::ZERO;
            for b in 0..SPEND_RESIDUE_BITS {
                let bit = local[bits_off + b];
                builder.assert_bool(bit);
                val_rc += bit.into() * ce::<AB>(1u64 << b);
                stream.push(bit.into());
            }
            builder.assert_eq(local[val_off].into(), val_rc);

            // Canonical `< q`: value + slack == q-1 with slack ∈ [0, q) via its own 14-bit decomp.
            let mut slack_rc = AB::Expr::ZERO;
            for b in 0..SPEND_RESIDUE_BITS {
                let bit = local[slack_bits_off + b];
                builder.assert_bool(bit);
                slack_rc += bit.into() * ce::<AB>(1u64 << b);
            }
            builder.assert_eq(local[slack_off].into(), slack_rc);
            builder.assert_eq(
                local[val_off].into() + local[slack_off].into(),
                ce::<AB>((SPEND_Q - 1) as u64),
            );
        }
        debug_assert_eq!(stream.len(), SPEND_STREAM_BITS);

        // packed_witness_commitment[k] == LE byte of the stream (high bits 252..256 zero by
        // construction — out-of-range stream positions contribute ZERO). Binds the private residue
        // bits to the PUBLIC commitment, byte-for-byte identical to the creation leaf's packing.
        for k in 0..SPEND_COMMIT_BYTES {
            let mut byte = AB::Expr::ZERO;
            for b in 0..BYTE_BITS {
                let pos = k * BYTE_BITS + b;
                if pos < SPEND_STREAM_BITS {
                    byte += stream[pos].clone() * ce::<AB>(1u64 << b);
                }
            }
            builder.assert_eq(byte, public[PV_PACKED_OFF + k].clone().into());
        }

        // Pin the spend relation id (proof-system bookkeeping; the verifier also checks it).
        for j in 0..SPEND_RELATION_ID_ELEMS {
            builder.assert_eq(
                public[PV_RELATION_ID_OFF + j].clone().into(),
                ce::<AB>(SPEND_RELATION_ID[j] as u64),
            );
        }
    }
}

/// The spend statement supplied by the prover/caller. `packed_witness_commitment` is the EXACT
/// 32-byte value the creation leaf published (the consumption anchor); the residues are the prover's
/// private opening of it (canonical q12289 values). `syndrome[12] ‖ projection[6]`.
pub struct SpendWitness {
    pub residues: [u16; SPEND_RESIDUES],
    pub packed_witness_commitment: [u8; SPEND_COMMIT_BYTES],
    pub account_id: [u8; 32],
    pub slot_id: u64,
    pub account_state_root: [u8; 32],
    pub message_digest: [u8; 32],
}

/// 14-bit-LE pack of the 18 residues into 32 bytes — byte-for-bit identical to the deployed
/// `pack_reviewed_fq12_packed_witness_state18_from_parts` (syndrome[12] ‖ projection[6]).
pub fn pack_spend_witness(residues: &[u16; SPEND_RESIDUES]) -> [u8; SPEND_COMMIT_BYTES] {
    let mut bits = [0u8; SPEND_STREAM_BITS];
    for (r, &v) in residues.iter().enumerate() {
        for b in 0..SPEND_RESIDUE_BITS {
            bits[r * SPEND_RESIDUE_BITS + b] = ((v >> b) & 1) as u8;
        }
    }
    let mut out = [0u8; SPEND_COMMIT_BYTES];
    for (k, byte) in out.iter_mut().enumerate() {
        let mut acc = 0u8;
        for b in 0..BYTE_BITS {
            let pos = k * BYTE_BITS + b;
            if pos < SPEND_STREAM_BITS && bits[pos] == 1 {
                acc |= 1 << b;
            }
        }
        *byte = acc;
    }
    out
}

/// Build the single-row spend trace + the public-value vector for [`SpendAir`]. The trace is padded
/// to 2 identical rows (smallest power of two). Panics if a residue is non-canonical (`>= q`).
pub fn generate_spend_trace(w: &SpendWitness) -> (RowMajorMatrix<Val>, Vec<Val>) {
    let mut row = [Val::ZERO; SPEND_WIDTH];
    for (r, &v) in w.residues.iter().enumerate() {
        assert!((v as u32) < SPEND_Q, "spend residue {r} is non-canonical (>= q12289)");
        let base = residue_base(r);
        let slack = (SPEND_Q - 1) - (v as u32);
        row[base] = Val::from_u32(v as u32);
        for b in 0..SPEND_RESIDUE_BITS {
            row[base + 1 + b] = Val::from_u32(((v as u32) >> b) & 1);
        }
        row[base + 1 + SPEND_RESIDUE_BITS] = Val::from_u32(slack);
        for b in 0..SPEND_RESIDUE_BITS {
            row[base + 2 + SPEND_RESIDUE_BITS + b] = Val::from_u32((slack >> b) & 1);
        }
    }

    let mut pv = Vec::with_capacity(SPEND_N_PUBLIC);
    pv.extend(w.packed_witness_commitment.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.account_id.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.slot_id.to_le_bytes().iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.account_state_root.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.message_digest.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(SPEND_RELATION_ID.iter().map(|&id| Val::from_u32(id)));
    debug_assert_eq!(pv.len(), SPEND_N_PUBLIC);

    // Pad to 2 identical rows (smallest power-of-two height).
    let mut values = Vec::with_capacity(2 * SPEND_WIDTH);
    values.extend_from_slice(&row);
    values.extend_from_slice(&row);
    (RowMajorMatrix::new(values, SPEND_WIDTH), pv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        N_LANES, PACKED_LANE_BITS, PACKED_WITNESS_COMMITMENT_BYTES, PACKED_WITNESS_RESIDUES,
        SYNDROME_LANES,
    };
    use p3_air::check_constraints;

    fn sample_witness() -> SpendWitness {
        // 18 canonical residues (12 syndrome ‖ 6 projection).
        let residues: [u16; SPEND_RESIDUES] =
            core::array::from_fn(|i| ((i as u32 * 1009 + 17) % SPEND_Q) as u16);
        let packed = pack_spend_witness(&residues);
        SpendWitness {
            residues,
            packed_witness_commitment: packed,
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            slot_id: 0xDEAD_BEEF_0000_0042,
            account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
            message_digest: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
        }
    }

    /// FIDELITY-PIN: the spend frame shape matches the creation leaf's CLASS-C frame exactly, so the
    /// two crates pack/consume the identical reviewed-fq12 packed-witness state-18 layout.
    #[test]
    fn spend_frame_shape_matches_creation_leaf() {
        assert_eq!(SPEND_SYNDROME_LANES, SYNDROME_LANES);
        assert_eq!(SPEND_PROJECTION_LANES, N_LANES);
        assert_eq!(SPEND_RESIDUES, PACKED_WITNESS_RESIDUES);
        assert_eq!(SPEND_RESIDUE_BITS, PACKED_LANE_BITS);
        assert_eq!(SPEND_COMMIT_BYTES, PACKED_WITNESS_COMMITMENT_BYTES);
    }

    /// COMPLETENESS: an honest spend witness satisfies every constraint.
    #[test]
    fn honest_spend_satisfies_constraints() {
        let (trace, pv) = generate_spend_trace(&sample_witness());
        check_constraints(&SpendAir, &trace, &pv);
    }

    /// SOUNDNESS: claim a different packed_witness_commitment than the residues actually pack to →
    /// the byte-packing equality fails (the spend cannot consume a witness it did not open).
    #[test]
    #[should_panic]
    fn tampered_packed_commitment_violates_constraints() {
        let (trace, mut pv) = generate_spend_trace(&sample_witness());
        pv[PV_PACKED_OFF] += Val::ONE; // claim a different commitment byte 0
        check_constraints(&SpendAir, &trace, &pv);
    }

    /// SOUNDNESS: a non-canonical residue (value + slack != q-1) is rejected.
    #[test]
    #[should_panic]
    fn noncanonical_residue_violates_constraints() {
        let (mut trace, pv) = generate_spend_trace(&sample_witness());
        // Corrupt residue 0's slack so value+slack != q-1 (both rows).
        let slack_off = residue_base(0) + 1 + SPEND_RESIDUE_BITS;
        for r in 0..2 {
            trace.values[r * SPEND_WIDTH + slack_off] += Val::ONE;
        }
        check_constraints(&SpendAir, &trace, &pv);
    }

    /// SOUNDNESS: a wrong spend_relation_id (not the pinned constant) is rejected.
    #[test]
    #[should_panic]
    fn wrong_spend_relation_id_violates_constraints() {
        let (trace, mut pv) = generate_spend_trace(&sample_witness());
        pv[PV_RELATION_ID_OFF] += Val::ONE;
        check_constraints(&SpendAir, &trace, &pv);
    }

    /// The reference packer matches a hand bit-pack of residue 0 (LE), sanity-checking the layout.
    #[test]
    fn packer_roundtrip_low_residue() {
        let mut residues = [0u16; SPEND_RESIDUES];
        residues[0] = 0x1ABC & (SPEND_Q as u16 - 1).min(0x3FFF); // 14-bit value
        residues[0] = (residues[0] as u32 % SPEND_Q) as u16;
        let packed = pack_spend_witness(&residues);
        // residue 0 occupies bits 0..14 → bytes 0 (8 bits) + low 6 bits of byte 1.
        let recon = (packed[0] as u32) | (((packed[1] as u32) & 0x3F) << 8);
        assert_eq!(recon, residues[0] as u32);
    }
}
