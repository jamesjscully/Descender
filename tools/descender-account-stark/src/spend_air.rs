//! Phase-2 SPEND leaf AIR — proves a spend authorizes EXACTLY the packed witness the
//! account-creation leaf produced, binds the spend STATEMENT to it, and verifies the checked-core
//! signature relation in-circuit.
//!
//! # What this proves
//!
//! 1. **Packed-witness consumption (creation↔spend binding).** The PUBLIC `packed_witness_commitment`
//!    is a well-formed 14-bit-LE packing of 18 residues `syndrome[12] ‖ projection[6]`, each canonical
//!    `< q12289`, byte-for-bit identical to the creation leaf's CLASS-C packing. Residue VALUES are
//!    private ("no spend-time opening of the projection"); only the public commitment + canonical
//!    packing are proven.
//! 2. **Checked-core FRAME-DIGEST binding (statement ↔ witness).** `frame_digest = Poseidon2(DOMAIN ‖
//!    message ‖ account_id ‖ slot_id ‖ account_state_root ‖ packed_witness_commitment)` is computed
//!    IN CIRCUIT and exposed as a PUBLIC output, with the input lanes constrained to the pinned domain
//!    and the public statement bytes — so the statement is cryptographically bound to the witness.
//! 3. **Checked-core SIGNATURE relation (SIS), in-circuit.** A private canonical response `r[22]`
//!    (each `< q`) satisfies `syndrome[row] = Σ_col H[row][col]·r[col] (mod q)` for the FIXED public
//!    matrix `H` ([`crate::checked_core`]), with the result constrained EQUAL to the consumed
//!    `syndrome` lanes. Proving the committed syndrome has a known short canonical preimage under `H`
//!    IS the checked-core signature acceptance (`path2_native_checked_core_accept_v1`'s sigma
//!    equation) — finding such a preimage for an arbitrary syndrome is the SIS hardness.
//!
//! `frame_digest` is the value the recursion / settlement layer binds and equates to the creation
//! proof's `packed_witness_commitment` (closing audit MED-2). Together (1)+(2)+(3) prove: a spend
//! exists for a well-formed, canonical packed witness, bound to a statement, whose syndrome is a valid
//! checked-core signature.

use crate::checked_core::{checked_core_matrix, response_syndrome, CC_COLS, CC_ROWS, CC_Q};
use crate::config::{Val, HALF_FULL_ROUNDS, PARTIAL_ROUNDS, SBOX_DEGREE, SBOX_REGISTERS, WIDTH};
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

pub const SPEND_Q: u32 = CC_Q as u32;
pub const SPEND_RESIDUE_BITS: usize = 14;
pub const SPEND_SYNDROME_LANES: usize = CC_ROWS; // 12
pub const SPEND_PROJECTION_LANES: usize = 6;
pub const SPEND_RESIDUES: usize = SPEND_SYNDROME_LANES + SPEND_PROJECTION_LANES; // 18
pub const SPEND_STREAM_BITS: usize = SPEND_RESIDUES * SPEND_RESIDUE_BITS; // 252
pub const SPEND_COMMIT_BYTES: usize = 32;

const BYTE_BITS: usize = 8;

// ---- Canonical residue cell: value ‖ bits[14] ‖ slack ‖ slack_bits[14] (proves 0 <= v < q) -------

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
pub const FRAME_INPUT_LANES: usize = FRAME_PACKED_LANE + 32; // 137
pub const FRAME_DIGEST_ELEMS: usize = 8;

pub const SPEND_FRAME_DOMAIN: u32 = 0x5350_4652; // "SPFR"

fn frame_spec() -> SpongeSpec {
    let mut br = alloc::vec![false; FRAME_DOMAIN_LANES];
    br.extend(alloc::vec![true; FRAME_INPUT_LANES - FRAME_DOMAIN_LANES]);
    SpongeSpec::new(br, FRAME_DIGEST_ELEMS)
}

// ---- Checked-core region: response lanes ‖ per-row chunked matrix-vector reductions --------------
//
// `syndrome[row] = Σ_col H[row][col]·response[col] mod q`. The 22-term product would overflow the
// field, so per row it is split into two chunks (cols 0..12, 12..22), each chunk-sum reduced mod q
// (quotient `k` is 18-bit; chunk-sum < 12·q² < p), then the two chunk-residues summed and reduced
// (final quotient is 1 bit since cr0+cr1 < 2q) and constrained EQUAL to the consumed `syndrome[row]`.

const K_BITS: usize = 18;
const CHUNK0: usize = 12;
/// Per-row reduction block: k0(1)+k0bits(18) ‖ cr0(1)+bits(14)+slack(1)+slackbits(14)
///                        ‖ k1(1)+k1bits(18) ‖ cr1(1)+bits(14)+slack(1)+slackbits(14) ‖ kf(1).
const CHUNK_REDUCE_CELLS: usize = (1 + K_BITS) + RESIDUE_CELLS; // 19 + 30 = 49
const ROW_REDUCE_CELLS: usize = 2 * CHUNK_REDUCE_CELLS + 1; // 99
/// Width of the checked-core region: 22 response lanes ‖ 12 row-reduction blocks.
pub const CC_REGION_WIDTH: usize = CC_COLS * RESIDUE_CELLS + CC_ROWS * ROW_REDUCE_CELLS; // 1848

const fn cc_response_base(c: usize) -> usize {
    c * RESIDUE_CELLS
}
const fn cc_row_base(row: usize) -> usize {
    CC_COLS * RESIDUE_CELLS + row * ROW_REDUCE_CELLS
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

/// Pinned spend relation/profile id. Encodes: "SPND" ‖ ("v3" ‖ public-width) ‖ (residues<<8 ‖ bits) ‖ q.
/// v3 = the checked-core SIS signature relation is now in-circuit.
pub const SPEND_RELATION_ID: [u32; SPEND_RELATION_ID_ELEMS] = [
    0x5350_4E44, // "SPND"
    0x7633_0000 | (SPEND_N_PUBLIC as u32 & 0xFFFF), // v3
    ((SPEND_RESIDUES as u32) << 8) | (SPEND_RESIDUE_BITS as u32),
    SPEND_Q,
];

fn ce<AB: AirBuilder>(v: u64) -> AB::Expr {
    AB::Expr::from(AB::F::from_u64(v))
}

/// The spend leaf AIR.
pub struct SpendAir {
    constants: RoundConstants<Val, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    beginning_full_rc: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: [Val; PARTIAL_ROUNDS],
    ending_full_rc: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    perm_cols: usize,
    matrix: [[u16; CC_COLS]; CC_ROWS],
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
            matrix: checked_core_matrix(),
        }
    }

    fn frame_off(&self) -> usize {
        RESIDUE_REGION_WIDTH
    }
    fn cc_off(&self) -> usize {
        RESIDUE_REGION_WIDTH + frame_spec().width(self.perm_cols)
    }
    pub fn trace_width(&self) -> usize {
        self.cc_off() + CC_REGION_WIDTH
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

/// Assert `local[base..]` is a canonical residue (value + 14 bits + slack canonical), returning the
/// value expr. Layout: value ‖ bits[14] ‖ slack ‖ slack_bits[14].
fn assert_canonical_residue<AB: AirBuilder>(builder: &mut AB, local: &[AB::Var], base: usize) -> AB::Expr {
    let val_off = base;
    let bits_off = base + 1;
    let slack_off = base + 1 + 14;
    let slack_bits_off = slack_off + 1;
    let mut val_rc = AB::Expr::ZERO;
    for b in 0..14 {
        let bit = local[bits_off + b];
        builder.assert_bool(bit);
        val_rc += bit.into() * ce::<AB>(1u64 << b);
    }
    builder.assert_eq(local[val_off].into(), val_rc);
    let mut slack_rc = AB::Expr::ZERO;
    for b in 0..14 {
        let bit = local[slack_bits_off + b];
        builder.assert_bool(bit);
        slack_rc += bit.into() * ce::<AB>(1u64 << b);
    }
    builder.assert_eq(local[slack_off].into(), slack_rc);
    builder.assert_eq(
        local[val_off].into() + local[slack_off].into(),
        ce::<AB>((CC_Q as u32 - 1) as u64),
    );
    local[val_off].into()
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
            let _ = assert_canonical_residue(builder, &local, base);
            for b in 0..SPEND_RESIDUE_BITS {
                stream.push(local[base + 1 + b].into());
            }
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
        builder.assert_eq(local[foff].into(), ce::<AB>(SPEND_FRAME_DOMAIN as u64));
        let bind = |builder: &mut AB, lane0: usize, pv0: usize, n: usize, local: &[AB::Var]| {
            for i in 0..n {
                builder.assert_eq(local[foff + lane0 + i].into(), public[pv0 + i].clone().into());
            }
        };
        bind(builder, FRAME_MSG_LANE, PV_MESSAGE_OFF, 32, &local);
        bind(builder, FRAME_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32, &local);
        bind(builder, FRAME_SLOT_LANE, PV_SLOT_OFF, 8, &local);
        bind(builder, FRAME_STATE_LANE, PV_STATE_ROOT_OFF, 32, &local);
        bind(builder, FRAME_PACKED_LANE, PV_PACKED_OFF, 32, &local);
        let outs = eval_sponge::<AB, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>(
            builder, &local, foff, self.perm_cols, &spec,
            &self.beginning_full_rc, &self.partial_rc, &self.ending_full_rc,
        );
        for j in 0..FRAME_DIGEST_ELEMS {
            builder.assert_eq(outs[j].clone(), public[PV_FRAME_DIGEST_OFF + j].clone().into());
        }

        // ---- (3) checked-core SIS signature: syndrome[row] == Σ_col H[row][col]·response[col] mod q.
        let cc = self.cc_off();
        // response lanes canonical.
        let mut r_val: Vec<AB::Expr> = Vec::with_capacity(CC_COLS);
        for c in 0..CC_COLS {
            r_val.push(assert_canonical_residue(builder, &local, cc + cc_response_base(c)));
        }
        let q = ce::<AB>(CC_Q as u64);
        for row in 0..CC_ROWS {
            let rb = cc + cc_row_base(row);
            // chunk 0: cols 0..CHUNK0.
            let mut x0 = AB::Expr::ZERO;
            for c in 0..CHUNK0 {
                x0 += r_val[c].clone() * ce::<AB>(self.matrix[row][c] as u64);
            }
            let (k0, _k0b) = recompose_bits(builder, &local, rb + 1, K_BITS);
            let cr0 = assert_canonical_residue(builder, &local, rb + 1 + K_BITS);
            builder.assert_eq(x0, k0 * q.clone() + cr0.clone());
            // chunk 1: cols CHUNK0..CC_COLS.
            let mut x1 = AB::Expr::ZERO;
            for c in CHUNK0..CC_COLS {
                x1 += r_val[c].clone() * ce::<AB>(self.matrix[row][c] as u64);
            }
            let (k1, _k1b) = recompose_bits(builder, &local, rb + CHUNK_REDUCE_CELLS + 1, K_BITS);
            let cr1 = assert_canonical_residue(builder, &local, rb + CHUNK_REDUCE_CELLS + 1 + K_BITS);
            builder.assert_eq(x1, k1 * q.clone() + cr1.clone());
            // final: cr0 + cr1 == kf·q + syndrome[row]  (kf boolean; syndrome[row] is residue row's value).
            let kf = local[rb + 2 * CHUNK_REDUCE_CELLS];
            builder.assert_bool(kf);
            let syndrome_row: AB::Expr = local[residue_base(row)].into();
            builder.assert_eq(cr0 + cr1, kf.into() * q.clone() + syndrome_row);
        }

        // ---- (4) pin the spend relation id.
        for j in 0..SPEND_RELATION_ID_ELEMS {
            builder.assert_eq(
                public[PV_RELATION_ID_OFF + j].clone().into(),
                ce::<AB>(SPEND_RELATION_ID[j] as u64),
            );
        }
    }
}

/// Recompose `nbits` boolean columns at `local[off..off+nbits]` LE, asserting each boolean; returns
/// `(value_expr, ())`.
fn recompose_bits<AB: AirBuilder>(
    builder: &mut AB,
    local: &[AB::Var],
    off: usize,
    nbits: usize,
) -> (AB::Expr, ()) {
    let mut acc = AB::Expr::ZERO;
    for b in 0..nbits {
        let bit = local[off + b];
        builder.assert_bool(bit);
        acc += bit.into() * ce::<AB>(1u64 << b);
    }
    (acc, ())
}

/// The spend statement + witness. `response` is the private checked-core response (each lane `< q`);
/// the syndrome is DERIVED as `H·response mod q`, and the packed witness as
/// `pack(syndrome ‖ projection)` — so it equals the creation leaf's published commitment.
pub struct SpendWitness {
    pub response: [u16; CC_COLS],
    pub projection: [u16; SPEND_PROJECTION_LANES],
    pub account_id: [u8; 32],
    pub slot_id: u64,
    pub account_state_root: [u8; 32],
    pub message_digest: [u8; 32],
}

/// 14-bit-LE pack of the 18 residues into 32 bytes — byte-for-bit identical to the deployed
/// `pack_reviewed_fq12_packed_witness_state18_from_parts` (syndrome[12] ‖ projection[6]).
pub fn pack_residues(residues: &[u16; SPEND_RESIDUES]) -> [u8; SPEND_COMMIT_BYTES] {
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

/// Derive the 18 residues (`syndrome ‖ projection`) and packed commitment for a witness.
pub fn spend_residues_and_packed(w: &SpendWitness) -> ([u16; SPEND_RESIDUES], [u8; SPEND_COMMIT_BYTES]) {
    let syndrome = response_syndrome(&w.response);
    let mut residues = [0u16; SPEND_RESIDUES];
    residues[..CC_ROWS].copy_from_slice(&syndrome);
    residues[CC_ROWS..].copy_from_slice(&w.projection);
    let packed = pack_residues(&residues);
    (residues, packed)
}

fn fill_canonical(row: &mut [Val], base: usize, v: u32) {
    let slack = (CC_Q as u32 - 1) - v;
    row[base] = Val::from_u32(v);
    for b in 0..14 {
        row[base + 1 + b] = Val::from_u32((v >> b) & 1);
    }
    row[base + 1 + 14] = Val::from_u32(slack);
    for b in 0..14 {
        row[base + 2 + 14 + b] = Val::from_u32((slack >> b) & 1);
    }
}

fn fill_bits(row: &mut [Val], off: usize, v: u32, nbits: usize) {
    for b in 0..nbits {
        row[off + b] = Val::from_u32((v >> b) & 1);
    }
}

/// Build the single-row spend trace (padded to 2 rows) + public values for [`SpendAir`].
/// Returns `(trace, public_values, frame_digest)`. Panics on a non-canonical residue/response.
pub fn generate_spend_trace(
    air: &SpendAir,
    w: &SpendWitness,
) -> (RowMajorMatrix<Val>, Vec<Val>, [Val; FRAME_DIGEST_ELEMS]) {
    let width = air.trace_width();
    let mut row = alloc::vec![Val::ZERO; width];

    let (residues, packed) = spend_residues_and_packed(w);
    let syndrome = response_syndrome(&w.response);

    // (1) residue cells.
    for (r, &v) in residues.iter().enumerate() {
        assert!((v as u32) < CC_Q as u32, "spend residue {r} non-canonical");
        fill_canonical(&mut row, residue_base(r), v as u32);
    }

    // (2) frame-sponge block.
    let foff = air.frame_off();
    let spec = frame_spec();
    let mut data: Vec<u32> = Vec::with_capacity(FRAME_INPUT_LANES);
    data.push(SPEND_FRAME_DOMAIN);
    data.extend(w.message_digest.iter().map(|&b| b as u32));
    data.extend(w.account_id.iter().map(|&b| b as u32));
    data.extend(w.slot_id.to_le_bytes().iter().map(|&b| b as u32));
    data.extend(w.account_state_root.iter().map(|&b| b as u32));
    data.extend(packed.iter().map(|&b| b as u32));
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
    frame_digest[..FRAME_DIGEST_ELEMS].copy_from_slice(&outputs[..FRAME_DIGEST_ELEMS]);

    // (3) checked-core region: response lanes + per-row chunked reductions.
    let cc = air.cc_off();
    for (c, &v) in w.response.iter().enumerate() {
        assert!((v as u32) < CC_Q as u32, "spend response {c} non-canonical");
        fill_canonical(&mut row, cc + cc_response_base(c), v as u32);
    }
    let q = CC_Q as u64;
    for r in 0..CC_ROWS {
        let rb = cc + cc_row_base(r);
        let x0: u64 = (0..CHUNK0).map(|c| air.matrix[r][c] as u64 * w.response[c] as u64).sum();
        let (k0, cr0) = (x0 / q, x0 % q);
        row[rb] = Val::from_u32(k0 as u32);
        fill_bits(&mut row, rb + 1, k0 as u32, K_BITS);
        fill_canonical(&mut row, rb + 1 + K_BITS, cr0 as u32);

        let x1: u64 = (CHUNK0..CC_COLS).map(|c| air.matrix[r][c] as u64 * w.response[c] as u64).sum();
        let (k1, cr1) = (x1 / q, x1 % q);
        row[rb + CHUNK_REDUCE_CELLS] = Val::from_u32(k1 as u32);
        fill_bits(&mut row, rb + CHUNK_REDUCE_CELLS + 1, k1 as u32, K_BITS);
        fill_canonical(&mut row, rb + CHUNK_REDUCE_CELLS + 1 + K_BITS, cr1 as u32);

        let sum = cr0 + cr1;
        let kf = sum / q;
        debug_assert!(kf <= 1);
        debug_assert_eq!(sum - kf * q, syndrome[r] as u64, "checked-core syndrome mismatch");
        row[rb + 2 * CHUNK_REDUCE_CELLS] = Val::from_u32(kf as u32);
    }

    // Public values.
    let mut pv = Vec::with_capacity(SPEND_N_PUBLIC);
    pv.extend(packed.iter().map(|&b| Val::from_u32(b as u32)));
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
        // A canonical response; the syndrome is DERIVED as H·response (a valid checked-core sig).
        let response: [u16; CC_COLS] =
            core::array::from_fn(|i| ((i as u32 * 503 + 7) % CC_Q as u32) as u16);
        let projection: [u16; SPEND_PROJECTION_LANES] =
            core::array::from_fn(|i| ((i as u32 * 211 + 5) % CC_Q as u32) as u16);
        SpendWitness {
            response,
            projection,
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            slot_id: 0xDEAD_BEEF_0000_0042,
            account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
            message_digest: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
        }
    }

    #[test]
    fn spend_frame_shape_matches_creation_leaf() {
        assert_eq!(SPEND_SYNDROME_LANES, SYNDROME_LANES);
        assert_eq!(SPEND_PROJECTION_LANES, N_LANES);
        assert_eq!(SPEND_RESIDUES, PACKED_WITNESS_RESIDUES);
        assert_eq!(SPEND_RESIDUE_BITS, PACKED_LANE_BITS);
        assert_eq!(SPEND_COMMIT_BYTES, PACKED_WITNESS_COMMITMENT_BYTES);
    }

    #[test]
    fn honest_spend_satisfies_constraints() {
        let a = air();
        let (trace, pv, _fd) = generate_spend_trace(&a, &sample_witness());
        check_constraints(&a, &trace, &pv);
    }

    #[test]
    #[should_panic]
    fn tampered_packed_commitment_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_PACKED_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    #[test]
    #[should_panic]
    fn tampered_frame_digest_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_FRAME_DIGEST_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    #[test]
    #[should_panic]
    fn statement_swap_without_digest_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_MESSAGE_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// CHECKED-CORE SOUNDNESS: corrupt a response lane so `H·response != syndrome` → the SIS
    /// reduction equality fails (the response is no longer a valid preimage of the syndrome).
    #[test]
    #[should_panic]
    fn tampered_response_violates_checked_core() {
        let a = air();
        let (mut trace, pv, _fd) = generate_spend_trace(&a, &sample_witness());
        // Corrupt response lane 0's value (and its first bit) in both rows.
        let cc = a.cc_off();
        let w = a.trace_width();
        for r in 0..2 {
            trace.values[r * w + cc + cc_response_base(0)] += Val::ONE;
        }
        check_constraints(&a, &trace, &pv);
    }

    /// CHECKED-CORE SOUNDNESS: a non-canonical response lane (value+slack != q-1) is rejected.
    #[test]
    #[should_panic]
    fn noncanonical_response_violates_constraints() {
        let a = air();
        let (mut trace, pv, _fd) = generate_spend_trace(&a, &sample_witness());
        let cc = a.cc_off();
        let slack_off = cc + cc_response_base(0) + 1 + SPEND_RESIDUE_BITS;
        let w = a.trace_width();
        for r in 0..2 {
            trace.values[r * w + slack_off] += Val::ONE;
        }
        check_constraints(&a, &trace, &pv);
    }

    #[test]
    #[should_panic]
    fn wrong_spend_relation_id_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_RELATION_ID_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// The derived packed witness equals a hand pack of (H·response ‖ projection).
    #[test]
    fn derived_packed_matches_reference() {
        let w = sample_witness();
        let (residues, packed) = spend_residues_and_packed(&w);
        assert_eq!(&residues[..CC_ROWS], &response_syndrome(&w.response)[..]);
        assert_eq!(&residues[CC_ROWS..], &w.projection[..]);
        assert_eq!(packed, pack_residues(&residues));
    }
}
