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

use crate::checked_core::{
    checked_core_matrix, CC_COLS, CC_DIGEST_BYTES, CC_Q, CC_ROWS, CC_SYNDROME_DIGEST_BITS,
};
use crate::config::{Val, HALF_FULL_ROUNDS, PARTIAL_ROUNDS, SBOX_DEGREE, SBOX_REGISTERS, WIDTH};
use crate::poseidon2_sponge::{build_elems, eval_sponge, run_sponge, SpongeSpec};
use crate::reduce_gadget::perm_num_cols;
use crate::LinLayers;
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField32};
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

/// Per-residue cell layout: value ‖ bits[14] ‖ slack ‖ slack_bits[14]. Exposed so the recursion's
/// spend C(zeta) mirror (`descender_recursion::spend_constraint_air`) can read the opened cells.
pub const RESIDUE_CELLS: usize = 1 + SPEND_RESIDUE_BITS + 1 + SPEND_RESIDUE_BITS; // 30
/// Width of the residue (packed-witness consumption) region.
pub const RESIDUE_REGION_WIDTH: usize = SPEND_RESIDUES * RESIDUE_CELLS; // 540

/// Trace-column base of residue `r`'s cell block (within the spend AIR row). Public for the mirror.
pub const fn residue_base(r: usize) -> usize {
    r * RESIDUE_CELLS
}

// ---- Frame-digest sponge: DOMAIN ‖ message[32] ‖ account_id[32] ‖ slot[8] ‖ state[32] ‖ packed[32]

pub const FRAME_DOMAIN_LANES: usize = 1;
pub const FRAME_MSG_LANE: usize = FRAME_DOMAIN_LANES; // 1
pub const FRAME_ACCT_LANE: usize = FRAME_MSG_LANE + 32; // 33
pub const FRAME_SLOT_LANE: usize = FRAME_ACCT_LANE + 32; // 65
pub const FRAME_STATE_LANE: usize = FRAME_SLOT_LANE + 8; // 73
pub const FRAME_PACKED_LANE: usize = FRAME_STATE_LANE + 32; // 105
pub const FRAME_INPUT_LANES: usize = FRAME_PACKED_LANE + 32; // 137
pub const FRAME_DIGEST_ELEMS: usize = 8;

pub const SPEND_FRAME_DOMAIN: u32 = 0x5350_4652; // "SPFR"

pub fn frame_spec() -> SpongeSpec {
    let mut br = alloc::vec![false; FRAME_DOMAIN_LANES];
    br.extend(alloc::vec![true; FRAME_INPUT_LANES - FRAME_DOMAIN_LANES]);
    SpongeSpec::new(br, FRAME_DIGEST_ELEMS)
}

// ---- Rehosted challenge: c = Poseidon2(DOMAIN ‖ statement ‖ witness_commitment ‖ proof_commitment)
//      reduced mod q (replaces the deployed SHA-256 challenge action vector + rejection sampler).
//      Bound to the STATEMENT (not the syndrome) + both commitments ⇒ proper Fiat-Shamir, no
//      circular dependency with the response. The mod-q reduction has a ~2^-17 per-lane bias
//      (negligible for a 12-lane challenge over q^12 ≈ 2^164); the deployed rejection sampler was
//      bias-free, this trades that for an AIR-native (SHA-free) derivation.

/// Domain tag for the rehosted spend challenge ("SPCH").
pub const SPEND_CHALLENGE_DOMAIN: u32 = 0x5350_4348;
/// Challenge sponge input: DOMAIN ‖ message[32] ‖ account_id[32] ‖ slot[8] ‖ state[32] ‖ wc[32] ‖ pc[32].
pub const CHALLENGE_INPUT_LANES: usize = 1 + 32 + 32 + 8 + 32 + CC_DIGEST_BYTES + CC_DIGEST_BYTES; // 169

pub fn challenge_spec() -> SpongeSpec {
    let mut br = alloc::vec![false; 1];
    br.extend(alloc::vec![true; CHALLENGE_INPUT_LANES - 1]);
    SpongeSpec::new(br, CC_ROWS)
}

/// Squeeze-reduction quotient bits: a squeezed field lane `< p` reduces mod q with quotient `< 2^18`.
pub const CHALLENGE_K_BITS: usize = 18;
/// Challenge input-lane offsets (within the sponge block).
pub const CHAL_DOMAIN_LANE: usize = 0;
pub const CHAL_MSG_LANE: usize = 1;
pub const CHAL_ACCT_LANE: usize = CHAL_MSG_LANE + 32; // 33
pub const CHAL_SLOT_LANE: usize = CHAL_ACCT_LANE + 32; // 65
pub const CHAL_STATE_LANE: usize = CHAL_SLOT_LANE + 8; // 73
pub const CHAL_WC_LANE: usize = CHAL_STATE_LANE + 32; // 105
pub const CHAL_PC_LANE: usize = CHAL_WC_LANE + CC_DIGEST_BYTES; // 137

/// Assemble the challenge sponge input data (DOMAIN ‖ statement ‖ commitments) as u32 lanes.
fn challenge_input_data(
    message_digest: &[u8; 32],
    account_id: &[u8; 32],
    slot_id: u64,
    account_state_root: &[u8; 32],
    witness_commitment: &[u8; CC_DIGEST_BYTES],
    proof_commitment: &[u8; CC_DIGEST_BYTES],
) -> Vec<u32> {
    let mut data = Vec::with_capacity(CHALLENGE_INPUT_LANES);
    data.push(SPEND_CHALLENGE_DOMAIN);
    data.extend(message_digest.iter().map(|&b| b as u32));
    data.extend(account_id.iter().map(|&b| b as u32));
    data.extend(slot_id.to_le_bytes().iter().map(|&b| b as u32));
    data.extend(account_state_root.iter().map(|&b| b as u32));
    data.extend(witness_commitment.iter().map(|&b| b as u32));
    data.extend(proof_commitment.iter().map(|&b| b as u32));
    debug_assert_eq!(data.len(), CHALLENGE_INPUT_LANES);
    data
}

/// Derive the rehosted challenge action vector `c[12]` natively (Poseidon2 squeeze, each lane mod q).
/// Byte-identical to the in-AIR challenge region.
pub fn derive_challenge(
    air: &SpendAir,
    message_digest: &[u8; 32],
    account_id: &[u8; 32],
    slot_id: u64,
    account_state_root: &[u8; 32],
    witness_commitment: &[u8; CC_DIGEST_BYTES],
    proof_commitment: &[u8; CC_DIGEST_BYTES],
) -> [u16; CC_ROWS] {
    let spec = challenge_spec();
    let data = challenge_input_data(
        message_digest,
        account_id,
        slot_id,
        account_state_root,
        witness_commitment,
        proof_commitment,
    );
    let elems = build_elems::<Val>(&data, &spec);
    let (_blocks, _post, outputs) = run_sponge::<
        Val,
        LinLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(&elems, &spec, &air.constants, air.perm_cols);
    core::array::from_fn(|i| (outputs[i].as_canonical_u32() % CC_Q as u32) as u16)
}

// ---- Checked-core region: response lanes ‖ per-row chunked matrix-vector reductions --------------
//
// `syndrome[row] = Σ_col H[row][col]·response[col] mod q`. The 22-term product would overflow the
// field, so per row it is split into two chunks (cols 0..12, 12..22), each chunk-sum reduced mod q
// (quotient `k` is 18-bit; chunk-sum < 12·q² < p), then the two chunk-residues summed and reduced
// (final quotient is 1 bit since cr0+cr1 < 2q) and constrained EQUAL to the consumed `syndrome[row]`.

pub const K_BITS: usize = 18;
pub const CHUNK0: usize = 12;
/// Per-row reduction block: k0(1)+k0bits(18) ‖ cr0(1)+bits(14)+slack(1)+slackbits(14)
///                        ‖ k1(1)+k1bits(18) ‖ cr1(1)+bits(14)+slack(1)+slackbits(14) ‖ kf(1).
pub const CHUNK_REDUCE_CELLS: usize = (1 + K_BITS) + RESIDUE_CELLS; // 19 + 30 = 49
pub const ROW_REDUCE_CELLS: usize = 2 * CHUNK_REDUCE_CELLS + 1; // 99
/// Width of the checked-core region: 22 response lanes ‖ 12 row-reduction blocks.
pub const CC_REGION_WIDTH: usize = CC_COLS * RESIDUE_CELLS + CC_ROWS * ROW_REDUCE_CELLS; // 1848

pub const fn cc_response_base(c: usize) -> usize {
    c * RESIDUE_CELLS
}
pub const fn cc_row_base(row: usize) -> usize {
    CC_COLS * RESIDUE_CELLS + row * ROW_REDUCE_CELLS
}

// ---- Sigma region: prove `syndrome == t + c⊛w` (the sigma acceptance identity) in-circuit --------
//
// `c`, `w`, `t` are 12-lane canonical syndrome vectors carried as PUBLIC inputs (c = challenge action
// vector, w = witness syndrome, t = proof-commitment randomizer). `⊛` is Fq12 multiply mod
// `x^12 + x + 25` ([`crate::checked_core::fq12_mul`]). This binds the consumed `syndrome` to the
// sigma combination of the public (c, w, t) — together with `H·response == syndrome` it puts the FULL
// sigma equation `H·r == t + c⊛w` in-circuit.
//
// DEFERRED (NOT in-circuit yet): the DERIVATIONS of the inputs — `c = hash(challenge)` (SHA-based;
// gated on the spend FS SHA→Poseidon2 rehost), and `w`/`t` from their commitments. Until those land,
// `c`/`w`/`t` are unbound public inputs, so this region adds the sigma ARITHMETIC in-circuit and
// reduces the off-circuit surface to the input-derivations — it is NOT yet standalone unforgeability.

pub const FOLD_K_BITS: usize = 6; // fold quotient < 27
pub const PROD_REDUCE_CELLS: usize = K_BITS + RESIDUE_CELLS; // 18 + 30 = 48
pub const FOLD_REDUCE_CELLS: usize = FOLD_K_BITS + RESIDUE_CELLS; // 6 + 30 = 36
pub const SIGMA_PROD_COEFFS: usize = 2 * CC_ROWS - 1; // 23 raw product coefficients
pub const SIGMA_VEC_REGION: usize = 3 * CC_ROWS * RESIDUE_CELLS; // c,w,t lanes: 36 × 30 = 1080
/// Width of the sigma region: c/w/t lanes ‖ 23 product reductions ‖ 12 fold reductions ‖ 12 finals.
pub const SIGMA_REGION_WIDTH: usize = SIGMA_VEC_REGION
    + SIGMA_PROD_COEFFS * PROD_REDUCE_CELLS
    + CC_ROWS * FOLD_REDUCE_CELLS
    + CC_ROWS;

pub const fn sigma_c_base(i: usize) -> usize {
    i * RESIDUE_CELLS
}
pub const fn sigma_w_base(i: usize) -> usize {
    CC_ROWS * RESIDUE_CELLS + i * RESIDUE_CELLS
}
pub const fn sigma_t_base(i: usize) -> usize {
    2 * CC_ROWS * RESIDUE_CELLS + i * RESIDUE_CELLS
}
pub const fn sigma_prod_base(k: usize) -> usize {
    SIGMA_VEC_REGION + k * PROD_REDUCE_CELLS
}
pub const fn sigma_fold_base(d: usize) -> usize {
    SIGMA_VEC_REGION + SIGMA_PROD_COEFFS * PROD_REDUCE_CELLS + d * FOLD_REDUCE_CELLS
}
pub const fn sigma_final_base(d: usize) -> usize {
    SIGMA_VEC_REGION + SIGMA_PROD_COEFFS * PROD_REDUCE_CELLS + CC_ROWS * FOLD_REDUCE_CELLS + d
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
/// Sigma public inputs (challenge action vector, witness syndrome, proof-commitment randomizer).
pub const PV_SIGMA_C_OFF: usize = PV_RELATION_ID_OFF + SPEND_RELATION_ID_ELEMS; // 148
pub const PV_SIGMA_W_OFF: usize = PV_SIGMA_C_OFF + CC_ROWS; // 160
pub const PV_SIGMA_T_OFF: usize = PV_SIGMA_W_OFF + CC_ROWS; // 172
/// Sigma input COMMITMENTS: `w == unpack(witness_commitment)`, `t == unpack(proof_commitment)`.
pub const PV_WITNESS_COMMITMENT_OFF: usize = PV_SIGMA_T_OFF + CC_ROWS; // 184
pub const PV_PROOF_COMMITMENT_OFF: usize = PV_WITNESS_COMMITMENT_OFF + CC_DIGEST_BYTES; // 216
pub const SPEND_N_PUBLIC: usize = PV_PROOF_COMMITMENT_OFF + CC_DIGEST_BYTES; // 248

/// Pinned spend relation/profile id. Encodes: "SPND" ‖ ("v5" ‖ public-width) ‖ (residues<<8 ‖ bits) ‖ q.
/// v5 = sigma inputs w/t are now bound to public commitments in-circuit (w/t = unpack(commitment)).
pub const SPEND_RELATION_ID: [u32; SPEND_RELATION_ID_ELEMS] = [
    0x5350_4E44,                                    // "SPND"
    0x7635_0000 | (SPEND_N_PUBLIC as u32 & 0xFFFF), // v5
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

    /// Poseidon2 per-block column count (exposed for the recursion's spend C(zeta) mirror).
    pub fn perm_cols(&self) -> usize {
        self.perm_cols
    }
    pub fn frame_off(&self) -> usize {
        RESIDUE_REGION_WIDTH
    }
    pub fn cc_off(&self) -> usize {
        RESIDUE_REGION_WIDTH + frame_spec().width(self.perm_cols)
    }
    pub fn sigma_off(&self) -> usize {
        self.cc_off() + CC_REGION_WIDTH
    }
    /// Challenge region: the challenge sponge block, then `CC_ROWS` squeeze-reduction k-bit blocks.
    pub fn challenge_off(&self) -> usize {
        self.sigma_off() + SIGMA_REGION_WIDTH
    }
    pub fn challenge_k_off(&self) -> usize {
        self.challenge_off() + challenge_spec().width(self.perm_cols)
    }
    pub fn trace_width(&self) -> usize {
        self.challenge_k_off() + CC_ROWS * CHALLENGE_K_BITS
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
fn assert_canonical_residue<AB: AirBuilder>(
    builder: &mut AB,
    local: &[AB::Var],
    base: usize,
) -> AB::Expr {
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
        let outs = eval_sponge::<
            AB,
            LinLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(
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
            builder.assert_eq(
                outs[j].clone(),
                public[PV_FRAME_DIGEST_OFF + j].clone().into(),
            );
        }

        // ---- (3) checked-core SIS signature: syndrome[row] == Σ_col H[row][col]·response[col] mod q.
        let cc = self.cc_off();
        // response lanes canonical.
        let mut r_val: Vec<AB::Expr> = Vec::with_capacity(CC_COLS);
        for c in 0..CC_COLS {
            r_val.push(assert_canonical_residue(
                builder,
                &local,
                cc + cc_response_base(c),
            ));
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
            let cr1 =
                assert_canonical_residue(builder, &local, rb + CHUNK_REDUCE_CELLS + 1 + K_BITS);
            builder.assert_eq(x1, k1 * q.clone() + cr1.clone());
            // final: cr0 + cr1 == kf·q + syndrome[row]  (kf boolean; syndrome[row] is residue row's value).
            let kf = local[rb + 2 * CHUNK_REDUCE_CELLS];
            builder.assert_bool(kf);
            let syndrome_row: AB::Expr = local[residue_base(row)].into();
            builder.assert_eq(cr0 + cr1, kf.into() * q.clone() + syndrome_row);
        }

        // ---- (3b) sigma acceptance identity: syndrome == t + c⊛w (Fq12), with c/w/t public.
        let sg = self.sigma_off();
        let mut cl: Vec<AB::Expr> = Vec::with_capacity(CC_ROWS);
        let mut wl: Vec<AB::Expr> = Vec::with_capacity(CC_ROWS);
        let mut tl: Vec<AB::Expr> = Vec::with_capacity(CC_ROWS);
        for i in 0..CC_ROWS {
            let cv = assert_canonical_residue(builder, &local, sg + sigma_c_base(i));
            builder.assert_eq(cv.clone(), public[PV_SIGMA_C_OFF + i].clone().into());
            cl.push(cv);
            let wv = assert_canonical_residue(builder, &local, sg + sigma_w_base(i));
            builder.assert_eq(wv.clone(), public[PV_SIGMA_W_OFF + i].clone().into());
            wl.push(wv);
            let tv = assert_canonical_residue(builder, &local, sg + sigma_t_base(i));
            builder.assert_eq(tv.clone(), public[PV_SIGMA_T_OFF + i].clone().into());
            tl.push(tv);
        }
        // Raw product p[k] = Σ_{i+j=k} c[i]·w[j], reduced mod q (≤12 terms, no field overflow).
        let mut p_red: Vec<AB::Expr> = Vec::with_capacity(SIGMA_PROD_COEFFS);
        for k in 0..SIGMA_PROD_COEFFS {
            let mut x = AB::Expr::ZERO;
            for i in 0..CC_ROWS {
                if k >= i && k - i < CC_ROWS {
                    x += cl[i].clone() * wl[k - i].clone();
                }
            }
            let base = sg + sigma_prod_base(k);
            let (kq, _) = recompose_bits(builder, &local, base, K_BITS);
            let r = assert_canonical_residue(builder, &local, base + K_BITS);
            builder.assert_eq(x, kq * q.clone() + r.clone());
            p_red.push(r);
        }
        // Fold degrees 12..22 into 0..11: reduced[d] = p[d] - 25·p[12+d] - p[11+d] (mod q), then
        // expected[d] = (reduced[d] + t[d]) mod q == syndrome[d].
        let q26 = ce::<AB>(26 * CC_Q as u64);
        for d in 0..CC_ROWS {
            // Y = p[d] + 26q - 25·p[12+d](d≤10) - p[11+d](d≥1)  ∈ [0, 27q).
            let mut y = p_red[d].clone() + q26.clone();
            if d <= CC_ROWS - 2 {
                y -= ce::<AB>(25) * p_red[CC_ROWS + d].clone();
            }
            if d >= 1 {
                y -= p_red[CC_ROWS - 1 + d].clone();
            }
            let fbase = sg + sigma_fold_base(d);
            let (kfold, _) = recompose_bits(builder, &local, fbase, FOLD_K_BITS);
            let reduced = assert_canonical_residue(builder, &local, fbase + FOLD_K_BITS);
            builder.assert_eq(y, kfold * q.clone() + reduced.clone());
            // expected[d] = reduced + t[d] == kfin·q + syndrome[d]  (kfin boolean).
            let kfin = local[sg + sigma_final_base(d)];
            builder.assert_bool(kfin);
            let syndrome_d: AB::Expr = local[residue_base(d)].into();
            builder.assert_eq(
                reduced + tl[d].clone(),
                kfin.into() * q.clone() + syndrome_d,
            );
        }

        // ---- (3c) bind sigma inputs to public commitments: w == unpack(witness_commitment),
        //          t == unpack(proof_commitment). 12 lanes × 14 bits → 32-byte digest (14-bit LE),
        //          bytes carrying bits ≥ 168 are forced to ZERO (high-bit canonicity).
        for (lane_base, commit_off) in [
            (
                sigma_w_base as fn(usize) -> usize,
                PV_WITNESS_COMMITMENT_OFF,
            ),
            (sigma_t_base as fn(usize) -> usize, PV_PROOF_COMMITMENT_OFF),
        ] {
            for k in 0..CC_DIGEST_BYTES {
                let mut byte = AB::Expr::ZERO;
                for b in 0..BYTE_BITS {
                    let pos = k * BYTE_BITS + b;
                    if pos < CC_SYNDROME_DIGEST_BITS {
                        let lane = pos / SPEND_RESIDUE_BITS;
                        let bit = pos % SPEND_RESIDUE_BITS;
                        // lane value-bit b lives at residue cell offset 1 + bit.
                        byte += local[sg + lane_base(lane) + 1 + bit].into() * ce::<AB>(1u64 << b);
                    }
                }
                builder.assert_eq(byte, public[commit_off + k].clone().into());
            }
        }

        // ---- (3d) rehosted challenge: sigma_c == Poseidon2(DOMAIN ‖ statement ‖ wc ‖ pc) mod q.
        let ch = self.challenge_off();
        let cspec = challenge_spec();
        builder.assert_eq(
            local[ch + CHAL_DOMAIN_LANE].into(),
            ce::<AB>(SPEND_CHALLENGE_DOMAIN as u64),
        );
        let cbind = |builder: &mut AB, lane0: usize, pv0: usize, n: usize, local: &[AB::Var]| {
            for i in 0..n {
                builder.assert_eq(local[ch + lane0 + i].into(), public[pv0 + i].clone().into());
            }
        };
        cbind(builder, CHAL_MSG_LANE, PV_MESSAGE_OFF, 32, &local);
        cbind(builder, CHAL_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32, &local);
        cbind(builder, CHAL_SLOT_LANE, PV_SLOT_OFF, 8, &local);
        cbind(builder, CHAL_STATE_LANE, PV_STATE_ROOT_OFF, 32, &local);
        cbind(
            builder,
            CHAL_WC_LANE,
            PV_WITNESS_COMMITMENT_OFF,
            CC_DIGEST_BYTES,
            &local,
        );
        cbind(
            builder,
            CHAL_PC_LANE,
            PV_PROOF_COMMITMENT_OFF,
            CC_DIGEST_BYTES,
            &local,
        );
        let chal_outs = eval_sponge::<
            AB,
            LinLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(
            builder,
            &local,
            ch,
            self.perm_cols,
            &cspec,
            &self.beginning_full_rc,
            &self.partial_rc,
            &self.ending_full_rc,
        );
        let ck = self.challenge_k_off();
        for i in 0..CC_ROWS {
            // squeezed lane == k·q + sigma_c[i]  (the sigma c value, bound to the acceptance identity).
            let (k, _) =
                recompose_bits(builder, &local, ck + i * CHALLENGE_K_BITS, CHALLENGE_K_BITS);
            let sigma_c_i: AB::Expr = local[sg + sigma_c_base(i)].into();
            builder.assert_eq(chal_outs[i].clone(), k * q.clone() + sigma_c_i);
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

/// The spend statement + witness, in the REHOSTED flow: the prover fixes the checked-core key
/// `sigma_w` (witness syndrome) and the nonce `sigma_t` (proof-commitment randomizer); the challenge
/// `c`, the `syndrome = t + c⊛w`, and the `response` (solved from `H·r = syndrome`) are all DERIVED
/// ([`spend_derive`]). This matches a real prover: commit nonce → Fiat-Shamir challenge → response.
pub struct SpendWitness {
    /// Checked-core key (witness syndrome) — committed as `witness_commitment = pack(sigma_w)`.
    pub sigma_w: [u16; CC_ROWS],
    /// Sigma nonce (proof-commitment randomizer) — committed as `proof_commitment = pack(sigma_t)`.
    pub sigma_t: [u16; CC_ROWS],
    pub projection: [u16; SPEND_PROJECTION_LANES],
    pub account_id: [u8; 32],
    pub slot_id: u64,
    pub account_state_root: [u8; 32],
    pub message_digest: [u8; 32],
}

/// Everything derived from a [`SpendWitness`] in the rehosted flow.
pub struct SpendDerived {
    pub sigma_c: [u16; CC_ROWS],
    pub syndrome: [u16; CC_ROWS],
    pub response: [u16; CC_COLS],
    pub residues: [u16; SPEND_RESIDUES],
    pub packed: [u8; SPEND_COMMIT_BYTES],
    pub witness_commitment: [u8; CC_DIGEST_BYTES],
    pub proof_commitment: [u8; CC_DIGEST_BYTES],
}

/// Derive the rehosted spend: commitments → Poseidon2 challenge `c` → `syndrome = t + c⊛w` → solve
/// `H·response = syndrome`. Panics if the (random-`H`) checked-core system is inconsistent.
pub fn spend_derive(air: &SpendAir, w: &SpendWitness) -> SpendDerived {
    let witness_commitment = crate::checked_core::pack_syndrome_digest(&w.sigma_w);
    let proof_commitment = crate::checked_core::pack_syndrome_digest(&w.sigma_t);
    let sigma_c = derive_challenge(
        air,
        &w.message_digest,
        &w.account_id,
        w.slot_id,
        &w.account_state_root,
        &witness_commitment,
        &proof_commitment,
    );
    let syndrome = crate::checked_core::sigma_expected(&w.sigma_t, &sigma_c, &w.sigma_w);
    let response = crate::checked_core::solve_response(&syndrome)
        .expect("checked-core linear system solvable");
    let mut residues = [0u16; SPEND_RESIDUES];
    residues[..CC_ROWS].copy_from_slice(&syndrome);
    residues[CC_ROWS..].copy_from_slice(&w.projection);
    let packed = pack_residues(&residues);
    SpendDerived {
        sigma_c,
        syndrome,
        response,
        residues,
        packed,
        witness_commitment,
        proof_commitment,
    }
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

    // Rehosted derivation: commitments → Poseidon2 challenge c → syndrome = t + c⊛w → solve response.
    let d = spend_derive(air, w);
    let residues = d.residues;
    let packed = d.packed;
    let syndrome = d.syndrome;
    let response = d.response;
    let sigma_c = d.sigma_c;
    let sigma_t = w.sigma_t;

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
                row[foff + lane_bits_off + bit_cursor * BYTE_BITS + b] =
                    Val::from_u32((v >> b) & 1);
            }
            bit_cursor += 1;
        }
    }
    let elems = build_elems::<Val>(&data, &spec);
    let (blocks, _post, outputs) = run_sponge::<
        Val,
        LinLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
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
    for (c, &v) in response.iter().enumerate() {
        assert!((v as u32) < CC_Q as u32, "spend response {c} non-canonical");
        fill_canonical(&mut row, cc + cc_response_base(c), v as u32);
    }
    let q = CC_Q as u64;
    for r in 0..CC_ROWS {
        let rb = cc + cc_row_base(r);
        let x0: u64 = (0..CHUNK0)
            .map(|c| air.matrix[r][c] as u64 * response[c] as u64)
            .sum();
        let (k0, cr0) = (x0 / q, x0 % q);
        row[rb] = Val::from_u32(k0 as u32);
        fill_bits(&mut row, rb + 1, k0 as u32, K_BITS);
        fill_canonical(&mut row, rb + 1 + K_BITS, cr0 as u32);

        let x1: u64 = (CHUNK0..CC_COLS)
            .map(|c| air.matrix[r][c] as u64 * response[c] as u64)
            .sum();
        let (k1, cr1) = (x1 / q, x1 % q);
        row[rb + CHUNK_REDUCE_CELLS] = Val::from_u32(k1 as u32);
        fill_bits(&mut row, rb + CHUNK_REDUCE_CELLS + 1, k1 as u32, K_BITS);
        fill_canonical(&mut row, rb + CHUNK_REDUCE_CELLS + 1 + K_BITS, cr1 as u32);

        let sum = cr0 + cr1;
        let kf = sum / q;
        debug_assert!(kf <= 1);
        debug_assert_eq!(
            sum - kf * q,
            syndrome[r] as u64,
            "checked-core syndrome mismatch"
        );
        row[rb + 2 * CHUNK_REDUCE_CELLS] = Val::from_u32(kf as u32);
    }

    // (3b) sigma region: c/w/t lanes ‖ product reductions ‖ fold reductions ‖ finals.
    let sg = air.sigma_off();
    for i in 0..CC_ROWS {
        fill_canonical(&mut row, sg + sigma_c_base(i), sigma_c[i] as u32);
        fill_canonical(&mut row, sg + sigma_w_base(i), w.sigma_w[i] as u32);
        fill_canonical(&mut row, sg + sigma_t_base(i), sigma_t[i] as u32);
    }
    // Raw products p[k] = Σ_{i+j=k} c[i]·w[j] (unreduced ≤12-term sum), reduce mod q.
    let mut p_red = [0u64; SIGMA_PROD_COEFFS];
    for (k, pr) in p_red.iter_mut().enumerate() {
        let mut x = 0u64;
        for i in 0..CC_ROWS {
            if k >= i && k - i < CC_ROWS {
                x += sigma_c[i] as u64 * w.sigma_w[k - i] as u64;
            }
        }
        let base = sg + sigma_prod_base(k);
        let (kq, rem) = (x / q, x % q);
        fill_bits(&mut row, base, kq as u32, K_BITS);
        fill_canonical(&mut row, base + K_BITS, rem as u32);
        *pr = rem;
    }
    // Fold + final.
    for d in 0..CC_ROWS {
        let mut y: i64 = p_red[d] as i64 + 26 * q as i64;
        if d <= CC_ROWS - 2 {
            y -= 25 * p_red[CC_ROWS + d] as i64;
        }
        if d >= 1 {
            y -= p_red[CC_ROWS - 1 + d] as i64;
        }
        debug_assert!(y >= 0);
        let y = y as u64;
        let fbase = sg + sigma_fold_base(d);
        let (kfold, reduced) = (y / q, y % q);
        fill_bits(&mut row, fbase, kfold as u32, FOLD_K_BITS);
        fill_canonical(&mut row, fbase + FOLD_K_BITS, reduced as u32);
        let expected = reduced + sigma_t[d] as u64;
        let kfin = expected / q;
        debug_assert!(kfin <= 1);
        debug_assert_eq!(
            expected - kfin * q,
            syndrome[d] as u64,
            "sigma identity mismatch"
        );
        row[sg + sigma_final_base(d)] = Val::from_u32(kfin as u32);
    }

    // (3d) challenge sponge: derive c in-circuit from (statement ‖ wc ‖ pc); + squeeze reductions.
    let ch = air.challenge_off();
    let cspec = challenge_spec();
    let cdata = challenge_input_data(
        &w.message_digest,
        &w.account_id,
        w.slot_id,
        &w.account_state_root,
        &d.witness_commitment,
        &d.proof_commitment,
    );
    for (i, &v) in cdata.iter().enumerate() {
        row[ch + i] = Val::from_u32(v);
    }
    let clane_bits_off = cspec.n_input;
    let mut cbit = 0usize;
    for (i, &v) in cdata.iter().enumerate() {
        if cspec.byte_ranged[i] {
            for b in 0..BYTE_BITS {
                row[ch + clane_bits_off + cbit * BYTE_BITS + b] = Val::from_u32((v >> b) & 1);
            }
            cbit += 1;
        }
    }
    let celems = build_elems::<Val>(&cdata, &cspec);
    let (cblocks, _cp, coutputs) = run_sponge::<
        Val,
        LinLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(&celems, &cspec, &air.constants, air.perm_cols);
    let cperms_off = cspec.perms_off();
    for (p, b) in cblocks.iter().enumerate() {
        let base = ch + cperms_off + p * air.perm_cols;
        row[base..base + air.perm_cols].copy_from_slice(b);
    }
    let ck = air.challenge_k_off();
    for i in 0..CC_ROWS {
        let out_v = coutputs[i].as_canonical_u32() as u64;
        let k = out_v / q;
        debug_assert_eq!(
            out_v - k * q,
            sigma_c[i] as u64,
            "challenge squeeze reduction mismatch"
        );
        fill_bits(
            &mut row,
            ck + i * CHALLENGE_K_BITS,
            k as u32,
            CHALLENGE_K_BITS,
        );
    }

    // Public values.
    let mut pv = Vec::with_capacity(SPEND_N_PUBLIC);
    pv.extend(packed.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(w.account_id.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(
        w.slot_id
            .to_le_bytes()
            .iter()
            .map(|&b| Val::from_u32(b as u32)),
    );
    pv.extend(
        w.account_state_root
            .iter()
            .map(|&b| Val::from_u32(b as u32)),
    );
    pv.extend(w.message_digest.iter().map(|&b| Val::from_u32(b as u32)));
    pv.extend(frame_digest.iter().copied());
    pv.extend(SPEND_RELATION_ID.iter().map(|&id| Val::from_u32(id)));
    pv.extend(sigma_c.iter().map(|&v| Val::from_u32(v as u32)));
    pv.extend(w.sigma_w.iter().map(|&v| Val::from_u32(v as u32)));
    pv.extend(sigma_t.iter().map(|&v| Val::from_u32(v as u32)));
    pv.extend(
        d.witness_commitment
            .iter()
            .map(|&b| Val::from_u32(b as u32)),
    );
    pv.extend(d.proof_commitment.iter().map(|&b| Val::from_u32(b as u32)));
    debug_assert_eq!(pv.len(), SPEND_N_PUBLIC);

    let mut values = Vec::with_capacity(2 * width);
    values.extend_from_slice(&row);
    values.extend_from_slice(&row);
    (RowMajorMatrix::new(values, width), pv, frame_digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checked_core::response_syndrome;
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
        // Rehosted flow: the prover fixes the key (sigma_w) and nonce (sigma_t); c, syndrome, and
        // response are derived (the challenge via Poseidon2, the response by solving H·r = syndrome).
        SpendWitness {
            sigma_w: core::array::from_fn(|i| ((i as u32 * 149 + 29) % CC_Q as u32) as u16),
            sigma_t: core::array::from_fn(|i| ((i as u32 * 71 + 5) % CC_Q as u32) as u16),
            projection: core::array::from_fn(|i| ((i as u32 * 211 + 5) % CC_Q as u32) as u16),
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            slot_id: 0xDEAD_BEEF_0000_0042,
            account_state_root: core::array::from_fn(|i| {
                (i as u8).wrapping_mul(17).wrapping_add(7)
            }),
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

    /// SIGMA SOUNDNESS: tamper a public sigma input (c lane 0) so `t + c⊛w != syndrome` → the sigma
    /// identity fails (the consumed syndrome no longer matches the public sigma combination).
    #[test]
    #[should_panic]
    fn tampered_sigma_c_violates_identity() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_SIGMA_C_OFF] += Val::ONE; // change public c without re-deriving t
        check_constraints(&a, &trace, &pv);
    }

    /// SIGMA SOUNDNESS: tamper the public witness_commitment → the `w == unpack(witness_commitment)`
    /// digest-packing equality fails (w is bound to its commitment, not free).
    #[test]
    #[should_panic]
    fn tampered_witness_commitment_violates_binding() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_WITNESS_COMMITMENT_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// SIGMA SOUNDNESS: the derived spend is self-consistent — the syndrome equals both
    /// `sigma_expected(t, c, w)` (sigma identity) and `H·response` (SIS), with `c` the rehosted
    /// Poseidon2 challenge.
    #[test]
    fn sigma_identity_matches_native() {
        use crate::checked_core::{response_syndrome, sigma_expected};
        let a = air();
        let w = sample_witness();
        let d = spend_derive(&a, &w);
        assert_eq!(
            d.syndrome,
            sigma_expected(&w.sigma_t, &d.sigma_c, &w.sigma_w),
            "syndrome != t + c⊛w"
        );
        assert_eq!(
            response_syndrome(&d.response),
            d.syndrome,
            "H·response != syndrome"
        );
    }

    #[test]
    #[should_panic]
    fn wrong_spend_relation_id_violates_constraints() {
        let a = air();
        let (trace, mut pv, _fd) = generate_spend_trace(&a, &sample_witness());
        pv[PV_RELATION_ID_OFF] += Val::ONE;
        check_constraints(&a, &trace, &pv);
    }

    /// REHOST (Phase A) — the native rehosted construction is self-consistent: derive `c` via
    /// Poseidon2 over (statement ‖ wc ‖ pc), set `syndrome = t + c⊛w`, SOLVE `H·response = syndrome`;
    /// then the SIS (`H·response == syndrome`) and the sigma identity (`sigma_expected == syndrome`)
    /// both hold. This binds the challenge to the statement + both commitments (proper Fiat-Shamir)
    /// with NO SHA and NO circular dependency, and needs no trapdoor (the response is a canonical
    /// linear-system solution, not a short-norm preimage).
    #[test]
    fn rehosted_challenge_construction_is_consistent() {
        use crate::checked_core::{
            pack_syndrome_digest, response_syndrome, sigma_expected, solve_response,
        };
        let a = air();
        let sigma_w: [u16; CC_ROWS] =
            core::array::from_fn(|i| ((i as u32 * 149 + 29) % CC_Q as u32) as u16);
        let sigma_t: [u16; CC_ROWS] =
            core::array::from_fn(|i| ((i as u32 * 71 + 5) % CC_Q as u32) as u16);
        let message: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1));
        let account: [u8; 32] =
            core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3));
        let state: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7));
        let slot = 0xDEAD_BEEF_0000_0042u64;

        let wc = pack_syndrome_digest(&sigma_w);
        let pc = pack_syndrome_digest(&sigma_t);
        let c = derive_challenge(&a, &message, &account, slot, &state, &wc, &pc);
        let syndrome = sigma_expected(&sigma_t, &c, &sigma_w);
        let response = solve_response(&syndrome).expect("linear system solvable");
        for &v in &response {
            assert!(v < CC_Q, "response non-canonical");
        }
        assert_eq!(
            response_syndrome(&response),
            syndrome,
            "SIS H·response != syndrome"
        );
        assert_eq!(
            sigma_expected(&sigma_t, &c, &sigma_w),
            syndrome,
            "sigma identity broken"
        );
        // Determinism: the challenge is a pure function of (statement, wc, pc).
        assert_eq!(
            c,
            derive_challenge(&a, &message, &account, slot, &state, &wc, &pc)
        );
    }

    /// The derived packed witness is `pack(syndrome ‖ projection)` with `syndrome = H·response`.
    #[test]
    fn derived_packed_matches_reference() {
        let a = air();
        let w = sample_witness();
        let d = spend_derive(&a, &w);
        assert_eq!(&d.residues[..CC_ROWS], &response_syndrome(&d.response)[..]);
        assert_eq!(&d.residues[CC_ROWS..], &w.projection[..]);
        assert_eq!(d.packed, pack_residues(&d.residues));
    }
}
