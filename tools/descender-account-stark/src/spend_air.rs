//! Phase-2 SPEND leaf AIR — proves a spend authorizes EXACTLY the packed witness the
//! account-creation leaf produced, and binds the spend STATEMENT to it via an in-AIR Poseidon2
//! checked-core frame digest.
//!
//! # What this proves
//!
//! 1. **Packed-witness consumption (creation↔spend binding).** The PUBLIC `packed_witness_commitment`
//!    (the 32-byte reviewed-fq12 packed-witness state-18 frame the creation leaf publishes) is a
//!    well-formed 14-bit-LE packing of 18 residues `syndrome[12] ‖ projection[6]`, EACH canonical
//!    `< q12289`, byte-for-bit identical to the creation leaf's CLASS-C packing
//!    (`pack_reviewed_fq12_packed_witness_state18_from_parts`). The residue VALUES are private — the
//!    spend does not re-open them ("no spend-time opening"); only the public commitment and the
//!    canonical-packing relation are proven.
//! 2. **Checked-core FRAME-DIGEST binding (statement ↔ witness).** A Poseidon2 sponge computes, IN
//!    CIRCUIT, `frame_digest = Poseidon2(DOMAIN ‖ message ‖ account_id ‖ slot_id ‖ account_state_root
//!    ‖ packed_witness_commitment)`, with the sponge's input lanes constrained EQUAL to the pinned
//!    domain and the PUBLIC statement bytes, and the squeezed digest exposed as the PUBLIC
//!    `frame_digest`. So the statement is no longer a free passenger: it is cryptographically bound to
//!    the exact packed witness under a domain-separated digest (the SHA→Poseidon2 rehost of the
//!    deployed direct spend verifier's challenge frame — see
//!    `research/spend_fs_full_poseidon2_rehost_plan.md`).
//!
//! `frame_digest` is the value the recursion / settlement layer (and the deployed registry) bind: it
//! commits the spend to one (message, account, slot, state) over one packed witness, which is equated
//! to the creation proof's `packed_witness_commitment` to close audit MED-2.
//!
//! DEFERRED to the next increment (documented): the full Fq12 checked-core SIGNATURE equation over
//! the syndrome (the heavy cryptographic core, performed by the deployed direct spend verifier in
//! `parameter-search`). This AIR proves the spend is over a well-formed, canonical packed witness and
//! a domain-bound statement; the signature-acceptance predicate is the remaining piece.

use crate::config::{
    Val, HALF_FULL_ROUNDS, PARTIAL_ROUNDS, SBOX_DEGREE, SBOX_REGISTERS, WIDTH,
};
use crate::poseidon2_sponge::{build_elems, eval_sponge, run_sponge, SpongeSpec};
use crate::reduce_gadget::perm_num_cols;
use crate::LinLayers;
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;
use p3_poseidon2_air::RoundConstants;

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

// ---- Residue column layout: value ‖ bits[14] ‖ slack ‖ slack_bits[14] (canonical < q) -----------

const RESIDUE_CELLS: usize = 1 + SPEND_RESIDUE_BITS + 1 + SPEND_RESIDUE_BITS; // 30
/// Width of the residue (packed-witness consumption) region.
pub const RESIDUE_REGION_WIDTH: usize = SPEND_RESIDUES * RESIDUE_CELLS; // 540

const fn residue_base(r: usize) -> usize {
    r * RESIDUE_CELLS
}

// ---- Frame-digest sponge: DOMAIN ‖ message[32] ‖ account_id[32] ‖ slot[8] ‖ state[32] ‖ packed[32]

const FRAME_DOMAIN_LANES: usize = 1;
const FRAME_MSG_LANE: usize = FRAME_DOMAIN_LANES; // 1
const FRAME_ACCT_LANE: usize = FRAME_MSG_LANE + 32; // 33
const FRAME_SLOT_LANE: usize = FRAME_ACCT_LANE + 32; // 65
const FRAME_STATE_LANE: usize = FRAME_SLOT_LANE + 8; // 73
const FRAME_PACKED_LANE: usize = FRAME_STATE_LANE + 32; // 105
/// Total frame-sponge input lanes (DOMAIN ‖ 136 statement bytes).
pub const FRAME_INPUT_LANES: usize = FRAME_PACKED_LANE + 32; // 137
/// Squeezed frame-digest lanes.
pub const FRAME_DIGEST_ELEMS: usize = 8;

/// Domain-separation tag for the spend frame digest ("SPFR"; canonical < the KoalaBear modulus).
pub const SPEND_FRAME_DOMAIN: u32 = 0x5350_4652;

/// The frame sponge spec: DOMAIN lane (not byte-ranged) ‖ 136 byte-ranged statement lanes → 8 out.
fn frame_spec() -> SpongeSpec {
    let mut br = alloc::vec![false; FRAME_DOMAIN_LANES];
    br.extend(alloc::vec![true; FRAME_INPUT_LANES - FRAME_DOMAIN_LANES]);
    SpongeSpec::new(br, FRAME_DIGEST_ELEMS)
}

// ---- Public-value layout (the spend statement + frame digest) -----------------------------------

pub const PV_PACKED_OFF: usize = 0;
pub const PV_ACCOUNT_ID_OFF: usize = PV_PACKED_OFF + SPEND_COMMIT_BYTES; // 32
pub const PV_SLOT_OFF: usize = PV_ACCOUNT_ID_OFF + 32; // 64
pub const PV_STATE_ROOT_OFF: usize = PV_SLOT_OFF + 8; // 72
pub const PV_MESSAGE_OFF: usize = PV_STATE_ROOT_OFF + 32; // 104
pub const PV_FRAME_DIGEST_OFF: usize = PV_MESSAGE_OFF + 32; // 136
pub const PV_RELATION_ID_OFF: usize = PV_FRAME_DIGEST_OFF + FRAME_DIGEST_ELEMS; // 144
pub const SPEND_RELATION_ID_ELEMS: usize = 4;
pub const SPEND_N_PUBLIC: usize = PV_RELATION_ID_OFF + SPEND_RELATION_ID_ELEMS; // 148

/// Pinned spend relation/profile id. Bump on any change to the spend relation / frame / public
/// layout. Encodes: tag "SPND" ‖ ("v2" ‖ public-width) ‖ (residues<<8 ‖ bits) ‖ q.
pub const SPEND_RELATION_ID: [u32; SPEND_RELATION_ID_ELEMS] = [
    0x5350_4E44, // "SPND"
    0x7632_0000 | (SPEND_N_PUBLIC as u32 & 0xFFFF), // v2 (frame-digest added)
    ((SPEND_RESIDUES as u32) << 8) | (SPEND_RESIDUE_BITS as u32),
    SPEND_Q,
];

fn ce<AB: AirBuilder>(v: u64) -> AB::Expr {
    AB::Expr::from(AB::F::from_u64(v))
}

/// The spend leaf AIR: packed-witness consumption + canonical residues + checked-core frame digest.
pub struct SpendAir {
    constants: RoundConstants<Val, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    beginning_full_rc: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: [Val; PARTIAL_ROUNDS],
    ending_full_rc: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    perm_cols: usize,
}

impl SpendAir {
    pub fn from_arrays(
        beginning_full_rc: [[Val; WIDTH]; HALF_FULL_ROUNDS],
        partial_rc: [Val; PARTIAL_ROUNDS],
        ending_full_rc: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    ) -> Self {
        let constants = RoundConstants::new(beginning_full_rc, partial_rc, ending_full_rc);
        let perm_cols =
            perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
        Self {
            constants,
            beginning_full_rc,
            partial_rc,
            ending_full_rc,
            perm_cols,
        }
    }

    /// Column offset of the frame-sponge block (after the residue region).
    fn frame_off(&self) -> usize {
        RESIDUE_REGION_WIDTH
    }

    pub fn trace_width(&self) -> usize {
        RESIDUE_REGION_WIDTH + frame_spec().width(self.perm_cols)
    }
}

impl BaseAir<Val> for SpendAir {
    fn width(&self) -> usize {
        self.trace_width()
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

        // ---- (1) packed-witness consumption: 18 canonical residues pack to the public commitment.
        let mut stream: Vec<AB::Expr> = Vec::with_capacity(SPEND_STREAM_BITS);
        for r in 0..SPEND_RESIDUES {
            let base = residue_base(r);
            let val_off = base;
            let bits_off = base + 1;
            let slack_off = base + 1 + SPEND_RESIDUE_BITS;
            let slack_bits_off = slack_off + 1;

            let mut val_rc = AB::Expr::ZERO;
            for b in 0..SPEND_RESIDUE_BITS {
                let bit = local[bits_off + b];
                builder.assert_bool(bit);
                val_rc += bit.into() * ce::<AB>(1u64 << b);
                stream.push(bit.into());
            }
            builder.assert_eq(local[val_off].into(), val_rc);

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

        // ---- (2) checked-core frame digest: bind statement ‖ packed witness via Poseidon2.
        let foff = self.frame_off();
        let spec = frame_spec();
        // Bind the sponge input lanes: DOMAIN ‖ message ‖ account_id ‖ slot ‖ state_root ‖ packed.
        builder.assert_eq(local[foff].into(), ce::<AB>(SPEND_FRAME_DOMAIN as u64));
        let bind = |builder: &mut AB, lane0: usize, pv0: usize, n: usize, local: &[AB::Var]| {
            for i in 0..n {
                builder.assert_eq(
                    local[foff + lane0 + i].into(),
                    public[pv0 + i].clone().into(),
                );
            }
        };
        bind(builder, FRAME_MSG_LANE, PV_MESSAGE_OFF, 32, &local);
        bind(builder, FRAME_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32, &local);
        bind(builder, FRAME_SLOT_LANE, PV_SLOT_OFF, 8, &local);
        bind(builder, FRAME_STATE_LANE, PV_STATE_ROOT_OFF, 32, &local);
        bind(builder, FRAME_PACKED_LANE, PV_PACKED_OFF, 32, &local);

        let outs = eval_sponge::<AB, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>(
            builder,
            &local,
            foff,
            self.perm_cols,
            &spec,
            &self.beginning_full_rc,
            &self.partial_rc,
            &self.ending_full_rc,
        );
        for j in 0..FRAME_DIGEST_ELEMS {
            builder.assert_eq(outs[j].clone(), public[PV_FRAME_DIGEST_OFF + j].clone().into());
        }

        // ---- (3) pin the spend relation id (proof-system bookkeeping).
        for j in 0..SPEND_RELATION_ID_ELEMS {
            builder.assert_eq(
                public[PV_RELATION_ID_OFF + j].clone().into(),
                ce::<AB>(SPEND_RELATION_ID[j] as u64),
            );
        }
    }
}

/// The spend statement supplied by the prover. `packed_witness_commitment` is the EXACT 32-byte value
/// the creation leaf published; `residues` are the prover's private canonical opening of it.
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

/// Build the single-row spend trace (padded to 2 rows) + the public-value vector for [`SpendAir`].
/// Returns `(trace, public_values, frame_digest)`. Panics on a non-canonical residue (`>= q`).
pub fn generate_spend_trace(
    air: &SpendAir,
    w: &SpendWitness,
) -> (RowMajorMatrix<Val>, Vec<Val>, [Val; FRAME_DIGEST_ELEMS]) {
    let width = air.trace_width();
    let mut row = alloc::vec![Val::ZERO; width];

    // (1) residue cells.
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

    // (2) frame-sponge block: input lanes ‖ byte bits ‖ perm blocks.
    let foff = air.frame_off();
    let spec = frame_spec();
    let mut data: Vec<u32> = Vec::with_capacity(FRAME_INPUT_LANES);
    data.push(SPEND_FRAME_DOMAIN);
    data.extend(w.message_digest.iter().map(|&b| b as u32));
    data.extend(w.account_id.iter().map(|&b| b as u32));
    data.extend(w.slot_id.to_le_bytes().iter().map(|&b| b as u32));
    data.extend(w.account_state_root.iter().map(|&b| b as u32));
    data.extend(w.packed_witness_commitment.iter().map(|&b| b as u32));
    debug_assert_eq!(data.len(), FRAME_INPUT_LANES);

    for (i, &v) in data.iter().enumerate() {
        row[foff + i] = Val::from_u32(v);
    }
    let lane_bits_off = spec.n_input;
    let mut bit_cursor = 0usize;
    for (i, &v) in data.iter().enumerate() {
        if spec.byte_ranged[i] {
            for b in 0..BYTE_BITS {
                row[foff + lane_bits_off + bit_cursor * BYTE_BITS + b] = Val::from_u32((v >> b) & 1);
            }
            bit_cursor += 1;
        }
    }
    let elems = build_elems::<Val>(&data, &spec);
    let (blocks, _post, outputs) = run_sponge::<
        Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    >(&elems, &spec, &air.constants, air.perm_cols);
    let perms_off = spec.perms_off();
    for (p, b) in blocks.iter().enumerate() {
        let base = foff + perms_off + p * air.perm_cols;
        row[base..base + air.perm_cols].copy_from_slice(b);
    }
    let mut frame_digest = [Val::ZERO; FRAME_DIGEST_ELEMS];
    for j in 0..FRAME_DIGEST_ELEMS {
        frame_digest[j] = outputs[j];
    }

    // Public values.
    let mut pv = Vec::with_capacity(SPEND_N_PUBLIC);
    pv.extend(w.packed_witness_commitment.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.account_id.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.slot_id.to_le_bytes().iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.account_state_root.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.message_digest.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(frame_digest.iter().copied());
    pv.extend(SPEND_RELATION_ID.iter().map(|&id| Val::from_u32(id)));
    debug_assert_eq!(pv.len(), SPEND_N_PUBLIC);

    let mut values = Vec::with_capacity(2 * width);
    values.extend_from_slice(&row);
    values.extend_from_slice(&row);
    (RowMajorMatrix::new(values, width), pv, frame_digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        N_LANES, PACKED_LANE_BITS, PACKED_WITNESS_COMMITMENT_BYTES, PACKED_WITNESS_RESIDUES,
        SYNDROME_LANES,
    };
    use p3_air::check_constraints;
    use p3_koala_bear::{
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
        KOALABEAR_POSEIDON2_RC_16_INTERNAL,
    };

    fn air() -> SpendAir {
        SpendAir::from_arrays(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
        )
    }

    fn sample_witness() -> SpendWitness {
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

    /// FIDELITY-PIN: the spend frame shape matches the creation leaf's CLASS-C frame exactly.
    #[test]
    fn spend_frame_shape_matches_creation_leaf() {
        assert_eq!(SPEND_SYNDROME_LANES, SYNDROME_LANES);
        assert_eq!(SPEND_PROJECTION_LANES, N_LANES);
        assert_eq!(SPEND_RESIDUES, PACKED_WITNESS_RESIDUES);
        assert_eq!(SPEND_RESIDUE_BITS, PACKED_LANE_BITS);
        assert_eq!(SPEND_COMMIT_BYTES, PACKED_WITNESS_COMMITMENT_BYTES);
    }

    /// COMPLETENESS: an honest spend witness (incl. the in-AIR frame sponge) satisfies all constraints.
    #[test]
    fn honest_spend_satisfies_constraints() {
        let a = air();
        let (trace, pv, _fd) = generate_spend_trace(&a, &sample_witness());
        check_constraints(&a, &trace, &pv);
    }

    /// SOUNDNESS: claim a different packed_witness_commitment than the residues pack to → the
    /// byte-packing equality fails.
    #[test]
    #[should_panic]
    fn tampered_packed_commitment_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_PACKED_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// SOUNDNESS: a non-canonical residue (value + slack != q-1) is rejected.
    #[test]
    #[should_panic]
    fn noncanonical_residue_violates_constraints() {
        let a = air();
        let (mut trace, pv, _fd) = generate_spend_trace(&a, &sample_witness());
        let slack_off = residue_base(0) + 1 + SPEND_RESIDUE_BITS;
        let w = a.trace_width();
        for r in 0..2 {
            trace.values[r * w + slack_off] += Val::ONE;
        }
        check_constraints(&a, &trace, &pv);
    }

    /// SOUNDNESS: tamper the claimed frame_digest → the squeeze-equality fails (statement is bound).
    #[test]
    #[should_panic]
    fn tampered_frame_digest_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_FRAME_DIGEST_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// SOUNDNESS: change a statement public (message byte) WITHOUT recomputing the digest → the
    /// frame input-lane binding (lane == public message byte) fails. The statement is bound to the
    /// digest, so it cannot be swapped post hoc.
    #[test]
    #[should_panic]
    fn statement_swap_without_digest_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_MESSAGE_OFF] += Val::ONE; // change the message but keep the old digest + trace
        check_constraints(&a, &trace, &pv);
    }

    /// SOUNDNESS: a wrong spend_relation_id is rejected.
    #[test]
    #[should_panic]
    fn wrong_spend_relation_id_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_RELATION_ID_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// The reference packer matches a hand bit-pack of residue 0 (LE).
    #[test]
    fn packer_roundtrip_low_residue() {
        let mut residues = [0u16; SPEND_RESIDUES];
        residues[0] = ((0x1ABCu32) % SPEND_Q) as u16;
        let packed = pack_spend_witness(&residues);
        let recon = (packed[0] as u32) | (((packed[1] as u32) & 0x3F) << 8);
        assert_eq!(recon, residues[0] as u32);
    }
}
