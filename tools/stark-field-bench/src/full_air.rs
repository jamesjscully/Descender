//! The FULL account-creation relation (C1 ∧ C2 ∧ C4 ∧ C6 + cross-component
//! seams) as a SINGLE field-generic Plonky3 AIR over a 31-bit STARK field.
//!
//! This builds on the WORKING C2 re-host in `c2_air.rs` and reuses its exact
//! single-row, multi-permutation Poseidon2 sponge arithmetic. Where C2 re-hosts
//! ONE sponge (the mask PRF), this AIR co-locates THREE sponges plus the C4
//! range/carrier in ONE wide row, and binds the shared hidden values across the
//! component blocks with SAME-ROW column-equality constraints (the seams — the
//! whole point of one proof instead of four).
//!
//! # The relation (mirrors tools/parameter-search/.../full_relation_air_q12289.rs)
//!
//! * C1 — `pad_source_commitment = Poseidon2-commit(pad_seed[32] ‖ nonce_c1[16])`
//!   absorbed as byte-lanes, squeezed to `N_COMMIT_LANES` 31-bit outputs (public).
//! * C2 — `masks[6] = Poseidon2-PRF(pad_seed[32] ‖ slot[8])` reduced mod q12289
//!   (REUSED verbatim from the C2 design: the 6 masks feed the carrier, NOT
//!   public on their own).
//! * C6 — `opening_tuple_commitment =
//!   Poseidon2-commit(account_projection[6] field-lanes ‖ lifecycle_anchor[32] ‖
//!   nonce_c6[16])`, squeezed to `N_COMMIT_LANES` outputs (public).
//! * C4 — `masked_projection[6] = account_projection[6] + masks[6]` (carrier,
//!   mod q12289) with `account_projection` constrained to a CANONICAL base-128
//!   representation (`q = 96*128+1` canonicity gadget).
//!
//! # Seams (the bindings) — all SAME-ROW column equalities
//!
//! Because the whole sponge lives in one row, every shared value is a column in
//! that row, so each seam is just `assert_eq(col_a, col_b)`:
//!   * `SeamPadSeed`   : C1's 32 pad_seed lanes == C2's 32 pad_seed lanes.
//!   * `SeamOpeningAcc`: C4's 6 account_projection values == C6's 6 field lanes.
//!   * `SeamMask`      : C2's 6 reduced masks == the 6 masks used in the carrier.
//! One `pad_seed` therefore feeds C1+C2, and one `account_projection` feeds
//! C4+C6+carrier, certified by a single STARK proof.
//!
//! # Commitment-output collision margin
//!
//! On a 31-bit field each squeezed output carries ~31 bits, so a 128-bit
//! collision bound needs ceil(256/31)=9 lanes; we squeeze `N_COMMIT_LANES = 10`
//! for margin (matching the reference's 20-of-q12289 ≈ 280-bit choice in spirit,
//! sized to the bigger 31-bit field).
//!
//! # Field-generic
//!
//! Identical to `c2_air.rs`: generic over the Plonky3 `PrimeField32` bound and
//! the const-generic Poseidon2 parameters, so the same code path runs on
//! BabyBear and KoalaBear.

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::marker::PhantomData;
use core::mem::MaybeUninit;

use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::integers::QuotientMap;
use p3_field::{PrimeCharacteristicRing, PrimeField, PrimeField32, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_poseidon2::GenericPoseidon2LinearLayers;
use p3_poseidon2_air::{generate_trace_rows_for_perm, Poseidon2Cols, RoundConstants};

use crate::c2_air::{
    eval_reduce_gadget, fill_reduce_gadget, perm_num_cols, wide_reduce_masks, NUM_SQUEEZE, RATE,
    REDUCE_CELLS,
};

// ===========================================================================
// Relation-wide constants
// ===========================================================================

/// Projection lanes (masks / account_projection / masked_projection).
pub const N_LANES: usize = 6;
/// Commitment output lanes (C1 / C6). 10 lanes * 31 bits ≈ 310 bits ⇒ ≥128-bit
/// collision margin.
pub const N_COMMIT_LANES: usize = 10;

/// C1 input byte-lanes: pad_seed[32] ‖ blinding_nonce_c1[16].
pub const C1_INPUT_LANES: usize = 32 + 16;
/// C2 input byte-lanes: pad_seed[32] ‖ slot_id[8].
pub const C2_INPUT_LANES: usize = 32 + 8;
/// C6 input lanes: account_projection[6] (FIELD lanes, NOT byte-ranged) ‖
/// lifecycle_anchor[32] ‖ blinding_nonce_c6[16] (byte-lanes).
pub const C6_FIELD_LANES: usize = N_LANES;
pub const C6_BYTE_LANES: usize = 32 + 16;
#[allow(dead_code)]
pub const C6_INPUT_LANES: usize = C6_FIELD_LANES + C6_BYTE_LANES;

const BYTE_BITS: usize = 8;

// --- C4 canonical base-128 (mirrors base128_range_air_q12289) ---
const C4_BASE: u32 = 128;
const C4_DIGIT_BITS: usize = 7;
const C4_MAX_HI: u32 = 96; // q = 96*128 + 1 ⇒ canonical d_hi in [0,96]

// ===========================================================================
// Sponge-chain plan (which lanes get absorbed, how many perms, byte-range flags)
// ===========================================================================

/// A static description of one sponge: how many input lanes it absorbs, whether
/// each lane is byte-ranged, and how many outputs it squeezes. Used both to size
/// the column layout and to drive the witness + constraints uniformly.
#[derive(Clone)]
pub struct SpongePlan {
    /// number of input lanes (data lanes, before the 10* padding).
    pub n_input: usize,
    /// which input lanes are byte-ranged (true) vs raw field lanes (false).
    pub byte_ranged: Vec<bool>,
    /// number of squeezed outputs.
    pub n_out: usize,
    /// number of absorb permutations (data + pad chunked at RATE).
    pub n_absorb: usize,
    /// total permutations (absorb + squeeze).
    pub n_perms: usize,
}

impl SpongePlan {
    pub fn new(byte_ranged: Vec<bool>, n_out: usize) -> Self {
        let n_input = byte_ranged.len();
        // 10* padding: append a 1 marker then zero-fill to a RATE multiple.
        let padded = {
            let mut len = n_input + 1;
            while len % RATE != 0 {
                len += 1;
            }
            len
        };
        let n_absorb = padded / RATE;
        // squeeze read-off: take min(remaining, RATE) per perm, permute if more.
        let mut remaining = n_out;
        let mut n_squeeze = 0usize;
        loop {
            let take = remaining.min(RATE);
            remaining -= take;
            if remaining == 0 {
                break;
            }
            n_squeeze += 1;
        }
        SpongePlan {
            n_input,
            byte_ranged,
            n_out,
            n_absorb,
            n_perms: n_absorb + n_squeeze,
        }
    }

    /// total padded absorb-stream length.
    fn padded_len(&self) -> usize {
        self.n_absorb * RATE
    }

    /// The (perm, rate lane) that output `out` is squeezed from.
    fn output_source(&self, out: usize) -> (usize, usize) {
        let mut filled = 0usize;
        let mut read_perm = self.n_absorb - 1;
        let mut remaining = self.n_out;
        loop {
            let take = remaining.min(RATE);
            for i in 0..take {
                if filled == out {
                    return (read_perm, i);
                }
                filled += 1;
            }
            remaining -= take;
            if remaining == 0 {
                break;
            }
            read_perm += 1;
        }
        unreachable!("output index out of range")
    }
}

// ===========================================================================
// Column layout
// ===========================================================================
//
// One wide row = one full account-creation instance. Column blocks, in order:
//
//   [C1 sponge block] [C2 sponge block] [C6 sponge block] [C4 block] [carrier]
//
// Each sponge block is:   lanes[n_input] | lane_bits[byte lanes * 8] |
//                         out[n_out] | (C2 only: k,k_bits,mask,mask_bits,
//                                       slack,slack_bits) |
//                         Poseidon2 perm sub-blocks[n_perms]
//
// The C4 block is N_LANES residues * C4_CELLS each.
// The carrier block is masked[N_LANES] (public) + acc[N_LANES] (the C4
// reconstructed values, copied here for the seam + carrier same-row equalities).

const C4_CELLS: usize = 3 + 3 * C4_DIGIT_BITS + 2; // v,d_lo,d_hi, 3*7 bits, is96,inv = 26

/// Per-sponge column offsets within a block, computed from its plan.
#[allow(dead_code)]
struct SpongeLayout {
    lanes: usize,     // n_input lanes
    lane_bits: usize, // byte lanes * 8
    /// C2-only WIDE-THEN-REDUCE gadget block (REDUCE_CELLS wide when present, else
    /// zero-width). Squeezed outputs themselves are read from the perm post cols.
    reduce: usize,
    with_reduction: bool,
    perms: usize, // n_perms perm sub-blocks (each perm_cols wide)
    width: usize,
}

impl SpongeLayout {
    fn new(plan: &SpongePlan, with_reduction: bool, perm_cols: usize) -> Self {
        let n_byte = plan.byte_ranged.iter().filter(|&&b| b).count();
        let lanes = 0;
        let lane_bits = lanes + plan.n_input;
        let reduce = lane_bits + n_byte * BYTE_BITS;
        let perms = if with_reduction {
            reduce + REDUCE_CELLS
        } else {
            reduce
        };
        let width = perms + plan.n_perms * perm_cols;
        SpongeLayout {
            lanes,
            lane_bits,
            reduce,
            with_reduction,
            perms,
            width,
        }
    }
}

/// Full per-row layout: block offsets + total width.
pub struct FullLayout {
    pub c1_plan: SpongePlan,
    pub c2_plan: SpongePlan,
    pub c6_plan: SpongePlan,
    pub perm_cols: usize,
    c1: SpongeLayout,
    c2: SpongeLayout,
    c6: SpongeLayout,
    pub c1_off: usize,
    pub c2_off: usize,
    pub c6_off: usize,
    pub c4_off: usize,
    pub carry_masked_off: usize,
    pub carry_acc_off: usize,
    /// Offset of the per-row PUBLIC-IO column block (batch mode only):
    /// pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖ masked[6].
    /// In single-instance mode these are bound to the global public values; in
    /// batch mode each row's public IO is a committed trace column (so N rows can
    /// carry N distinct public IO), bound to the row's sponge outputs.
    pub pubio_off: usize,
    pub width: usize,
}

/// Number of per-row public-IO field elements (the per-instance "statement").
pub const PUBIO_ELEMS: usize = N_COMMIT_LANES + N_COMMIT_LANES + N_LANES; // 26

impl FullLayout {
    pub fn new(perm_cols: usize) -> Self {
        let c1_plan = SpongePlan::new(vec![true; C1_INPUT_LANES], N_COMMIT_LANES);
        // C2 squeezes 30 lanes (5 per mask) for the WIDE-THEN-REDUCE gadget.
        let c2_plan = SpongePlan::new(vec![true; C2_INPUT_LANES], NUM_SQUEEZE);
        let mut c6_br = vec![false; C6_FIELD_LANES];
        c6_br.extend(vec![true; C6_BYTE_LANES]);
        let c6_plan = SpongePlan::new(c6_br, N_COMMIT_LANES);

        let c1 = SpongeLayout::new(&c1_plan, false, perm_cols);
        let c2 = SpongeLayout::new(&c2_plan, true, perm_cols);
        let c6 = SpongeLayout::new(&c6_plan, false, perm_cols);

        let c1_off = 0;
        let c2_off = c1_off + c1.width;
        let c6_off = c2_off + c2.width;
        let c4_off = c6_off + c6.width;
        let carry_masked_off = c4_off + N_LANES * C4_CELLS;
        let carry_acc_off = carry_masked_off + N_LANES;
        let pubio_off = carry_acc_off + N_LANES;
        let width = pubio_off + PUBIO_ELEMS;

        FullLayout {
            c1_plan,
            c2_plan,
            c6_plan,
            perm_cols,
            c1,
            c2,
            c6,
            c1_off,
            c2_off,
            c6_off,
            c4_off,
            carry_masked_off,
            carry_acc_off,
            pubio_off,
            width,
        }
    }

    /// total Poseidon2 permutations across the three sponges.
    pub fn total_perms(&self) -> usize {
        self.c1_plan.n_perms + self.c2_plan.n_perms + self.c6_plan.n_perms
    }

    /// Column of C1's first input lane (lanes are contiguous from here).
    pub fn c1_lanes_off(&self) -> usize {
        self.c1.lanes
    }
    /// Column of C2's first input lane.
    #[allow(dead_code)]
    pub fn c2_lanes_off(&self) -> usize {
        self.c2.lanes
    }
    /// Column of C6's first input lane.
    pub fn c6_lanes_off(&self) -> usize {
        self.c6.lanes
    }
}

// ===========================================================================
// The field-generic AIR
// ===========================================================================

pub struct FullRelationAir<
    F: PrimeCharacteristicRing,
    LinearLayers,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
> {
    constants: RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: [F; PARTIAL_ROUNDS],
    ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    layout: FullLayout,
    /// When true: per-row public IO binds to the row's `pubio` trace columns (so a
    /// proof can carry N distinct instances). The single STARK public value is then
    /// a digest over all rows' public IO, recomputed by the verifier. When false:
    /// per-row public IO binds to the global public-values vector (single instance).
    batch: bool,
    _phantom: PhantomData<LinearLayers>,
}

impl<
        F: PrimeField32,
        LinearLayers,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    >
    FullRelationAir<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
{
    pub fn from_arrays(
        beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        partial_rc: [F; PARTIAL_ROUNDS],
        ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    ) -> Self {
        Self::from_arrays_mode(beginning_full_rc, partial_rc, ending_full_rc, false)
    }

    /// Like `from_arrays` but selects batch mode (per-row public IO -> trace cols).
    pub fn from_arrays_mode(
        beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        partial_rc: [F; PARTIAL_ROUNDS],
        ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        batch: bool,
    ) -> Self {
        let constants = RoundConstants::new(beginning_full_rc, partial_rc, ending_full_rc);
        let perm_cols =
            perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
        Self {
            constants,
            beginning_full_rc,
            partial_rc,
            ending_full_rc,
            layout: FullLayout::new(perm_cols),
            batch,
            _phantom: PhantomData,
        }
    }

    pub fn constants(&self) -> &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS> {
        &self.constants
    }
    pub fn layout(&self) -> &FullLayout {
        &self.layout
    }
}

impl<
        F: PrimeCharacteristicRing + Sync,
        LinearLayers: Sync,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    > BaseAir<F>
    for FullRelationAir<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
{
    fn width(&self) -> usize {
        self.layout.width
    }

    fn main_next_row_columns(&self) -> Vec<usize> {
        // Single-row layout: no constraint reads the next row.
        Vec::new()
    }

    fn num_public_values(&self) -> usize {
        if self.batch {
            // Batch mode: per-row public IO lives in the `pubio` trace columns; the
            // only global public value is the public-IO digest, recomputed and
            // checked OUTSIDE the AIR by the verifier (see batch_bench.rs). The AIR
            // itself reads no public values.
            0
        } else {
            // pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖ masked[6]
            N_COMMIT_LANES + N_COMMIT_LANES + N_LANES
        }
    }
}

// --- shared S-box / const helpers (copied from c2_air's private ones) ---

fn eval_sbox_expr<AB: AirBuilder, const DEGREE: u64, const REGISTERS: usize>(
    builder: &mut AB,
    registers: &[AB::Var],
    x: AB::Expr,
) -> AB::Expr {
    match (DEGREE, REGISTERS) {
        (3, 0) => x.cube(),
        (5, 0) => x.exp_const_u64::<5>(),
        (7, 0) => x.exp_const_u64::<7>(),
        (5, 1) => {
            let committed_x3: AB::Expr = registers[0].into();
            let x2 = x.square();
            builder.assert_eq(committed_x3.clone(), x2.clone() * x.clone());
            committed_x3 * x2
        }
        (7, 1) => {
            let committed_x3: AB::Expr = registers[0].into();
            builder.assert_eq(committed_x3.clone(), x.cube());
            committed_x3.square() * x
        }
        (11, 2) => {
            let committed_x3: AB::Expr = registers[0].into();
            let committed_x9: AB::Expr = registers[1].into();
            let x2 = x.square();
            builder.assert_eq(committed_x3.clone(), x2.clone() * x.clone());
            builder.assert_eq(committed_x9.clone(), committed_x3.cube());
            committed_x9 * x2
        }
        _ => panic!("Unsupported (DEGREE, REGISTERS) = ({DEGREE}, {REGISTERS})"),
    }
}

#[inline]
fn ce<AB: AirBuilder>(x: u64) -> AB::Expr
where
    AB::F: PrimeField,
{
    <AB::F as QuotientMap<u64>>::from_int(x).into()
}

impl<
        AB: AirBuilder,
        LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    > Air<AB>
    for FullRelationAir<
        AB::F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
where
    AB::F: PrimeField64,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.current_slice().to_vec();
        let public = builder.public_values().to_vec();
        let lay = &self.layout;
        let batch = self.batch;

        // Per-row public-IO binding target: in batch mode, bind the sponge/carrier
        // outputs to the row's own `pubio` trace columns (so N rows carry N distinct
        // statements); in single-instance mode, bind to the global public-values
        // vector. `pubio` layout: [pad_src_commit 0..10][opening_commit 10..20][masked 20..26].

        // ----- C1 sponge -> pad_source_commitment[10] (pubio 0..10) -----
        let c1_out = self.eval_sponge::<AB, LinearLayers>(
            builder,
            &local,
            lay.c1_off,
            &lay.c1,
            &lay.c1_plan,
        );
        for j in 0..N_COMMIT_LANES {
            if batch {
                builder.assert_eq(c1_out[j].clone(), local[lay.pubio_off + j].into());
            } else {
                builder.assert_eq(c1_out[j].clone(), public[j]);
            }
        }

        // ----- C2 sponge -> 30 squeezed lanes -> WIDE-THEN-REDUCE masks[6] -----
        // (masks NOT public directly; they feed the C4 carrier). The 6 masks are
        // derived from 30 squeezed lanes (5 per mask) so each mask is statistically
        // uniform over Z_q to bias <= q/p^5 ~ 2^-141 (vs the old 1-lane ~2^-17).
        let c2_out = self.eval_sponge::<AB, LinearLayers>(
            builder,
            &local,
            lay.c2_off,
            &lay.c2,
            &lay.c2_plan,
        );
        let c2_masks =
            eval_reduce_gadget::<AB>(builder, lay.c2_off + lay.c2.reduce, &local, &c2_out);

        // ----- C6 sponge -> opening_tuple_commitment[10] (public 10..20) -----
        let c6_out = self.eval_sponge::<AB, LinearLayers>(
            builder,
            &local,
            lay.c6_off,
            &lay.c6,
            &lay.c6_plan,
        );
        for j in 0..N_COMMIT_LANES {
            if batch {
                builder.assert_eq(
                    c6_out[j].clone(),
                    local[lay.pubio_off + N_COMMIT_LANES + j].into(),
                );
            } else {
                builder.assert_eq(c6_out[j].clone(), public[N_COMMIT_LANES + j]);
            }
        }

        // ----- C4 canonical base-128 range, reconstructed acc into carry_acc -----
        for j in 0..N_LANES {
            let base = lay.c4_off + j * C4_CELLS;
            let v = self.eval_c4_residue::<AB>(builder, &local, base);
            // SeamOpeningAcc + carrier acc share: copy v into the carrier acc col.
            builder.assert_eq(local[lay.carry_acc_off + j].into(), v.clone());
        }

        // ----- SEAM: pad_seed C1 == C2 (32 same-row lane equalities) -----
        for b in 0..32 {
            builder.assert_eq(
                local[lay.c1_off + lay.c1.lanes + b].into(),
                local[lay.c2_off + lay.c2.lanes + b].into(),
            );
        }

        // ----- SEAM: account_projection C4 == C6 (6 same-row equalities) -----
        // C6 field lane i is input lane i (the first 6 input lanes).
        for i in 0..N_LANES {
            builder.assert_eq(
                local[lay.carry_acc_off + i].into(),
                local[lay.c6_off + lay.c6.lanes + i].into(),
            );
        }

        // ----- Carrier (+ SeamMask): masked = mask(C2) + acc(C4) -----
        for j in 0..N_LANES {
            let masked = local[lay.carry_masked_off + j];
            let mask = c2_masks[j].clone();
            let acc = local[lay.carry_acc_off + j];
            // carrier
            builder.assert_eq(masked.into(), mask + acc.into());
            // public bind: masked == masked_projection lane j (pubio col or global).
            if batch {
                builder.assert_eq(
                    local[lay.carry_masked_off + j].into(),
                    local[lay.pubio_off + 2 * N_COMMIT_LANES + j].into(),
                );
            } else {
                builder.assert_eq(
                    local[lay.carry_masked_off + j].into(),
                    public[2 * N_COMMIT_LANES + j],
                );
            }
        }
    }
}

impl<
        F: PrimeField,
        LinearLayers,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    >
    FullRelationAir<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
{
    /// Constrain one sponge block: byte-range its byte lanes, run all perm
    /// blocks with additive-absorb linking + Poseidon2 round constraints, and
    /// return the squeezed output exprs.
    #[allow(clippy::type_complexity)]
    fn eval_sponge<AB, LL>(
        &self,
        builder: &mut AB,
        local: &[AB::Var],
        off: usize,
        slay: &SpongeLayout,
        plan: &SpongePlan,
    ) -> Vec<AB::Expr>
    where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
        LL: GenericPoseidon2LinearLayers<WIDTH>,
    {
        let lane = |i: usize| local[off + slay.lanes + i];
        // byte-range each byte-ranged lane.
        let mut bit_cursor = 0usize;
        for i in 0..plan.n_input {
            if plan.byte_ranged[i] {
                let mut acc = AB::Expr::ZERO;
                for b in 0..BYTE_BITS {
                    let bit = local[off + slay.lane_bits + bit_cursor * BYTE_BITS + b];
                    builder.assert_bool(bit);
                    acc += bit.into() * ce::<AB>(1u64 << b);
                }
                builder.assert_eq(lane(i).into(), acc);
                bit_cursor += 1;
            }
        }

        // perm blocks with additive-absorb linking.
        let padded = plan.padded_len();
        let mut prev_post: Option<[AB::Expr; WIDTH]> = None;
        for p in 0..plan.n_perms {
            let base = off + slay.perms + p * self.layout.perm_cols;
            let block: Vec<AB::Var> = local[base..base + self.layout.perm_cols].to_vec();
            let pcols: &Poseidon2Cols<
                AB::Var,
                WIDTH,
                SBOX_DEGREE,
                SBOX_REGISTERS,
                HALF_FULL_ROUNDS,
                PARTIAL_ROUNDS,
            > = (block.as_slice()).borrow();

            let inputs: [AB::Expr; WIDTH] = core::array::from_fn(|c| pcols.inputs[c].into());
            let is_absorb = p < plan.n_absorb;
            match &prev_post {
                None => {
                    // first perm: rate lanes = absorbed block elems, capacity = 0.
                    for r in 0..RATE {
                        let pos = r; // perm 0, rate lane r
                        let elem = self.absorb_elem_expr::<AB>(local, off, slay, plan, pos);
                        builder.assert_eq(inputs[r].clone(), elem);
                    }
                    for c in RATE..WIDTH {
                        builder.assert_eq(inputs[c].clone(), AB::Expr::ZERO);
                    }
                }
                Some(prev) => {
                    for r in 0..RATE {
                        if is_absorb {
                            let pos = p * RATE + r;
                            let elem = if pos < padded {
                                self.absorb_elem_expr::<AB>(local, off, slay, plan, pos)
                            } else {
                                AB::Expr::ZERO
                            };
                            builder.assert_eq(inputs[r].clone(), prev[r].clone() + elem);
                        } else {
                            // squeeze perm: no absorb, state carried unchanged.
                            builder.assert_eq(inputs[r].clone(), prev[r].clone());
                        }
                    }
                    for c in RATE..WIDTH {
                        builder.assert_eq(inputs[c].clone(), prev[c].clone());
                    }
                }
            }

            let post = self.eval_perm_block::<AB, LL>(builder, pcols);
            prev_post = Some(post);
        }

        // squeeze: collect outputs from their source perms' post states.
        // We need the post state of every source perm. Recompute by re-walking:
        // we kept only the last prev_post, so instead read squeeze sources from
        // the committed perm post columns directly.
        let mut outs = vec![AB::Expr::ZERO; plan.n_out];
        for o in 0..plan.n_out {
            let (sp, lane_idx) = plan.output_source(o);
            let base = off + slay.perms + sp * self.layout.perm_cols;
            let block: Vec<AB::Var> = local[base..base + self.layout.perm_cols].to_vec();
            let pcols: &Poseidon2Cols<
                AB::Var,
                WIDTH,
                SBOX_DEGREE,
                SBOX_REGISTERS,
                HALF_FULL_ROUNDS,
                PARTIAL_ROUNDS,
            > = (block.as_slice()).borrow();
            outs[o] = pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post[lane_idx].into();
        }
        outs
    }

    /// The absorbed expr at absorb position `pos`: a data lane (if pos maps to a
    /// data lane), the 10* pad marker (1) at the first pad slot, or 0.
    fn absorb_elem_expr<AB>(
        &self,
        local: &[AB::Var],
        off: usize,
        slay: &SpongeLayout,
        plan: &SpongePlan,
        pos: usize,
    ) -> AB::Expr
    where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
    {
        if pos < plan.n_input {
            local[off + slay.lanes + pos].into()
        } else if pos == plan.n_input {
            AB::Expr::ONE // 10* pad marker
        } else {
            AB::Expr::ZERO
        }
    }

    /// C4 canonical base-128 residue gadget on cells at `base`. Returns the
    /// reconstructed value `v` as an expr. Mirrors base128_range_air_q12289.
    fn eval_c4_residue<AB>(&self, builder: &mut AB, local: &[AB::Var], base: usize) -> AB::Expr
    where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
    {
        let v = local[base];
        let d_lo = local[base + 1];
        let d_hi = local[base + 2];
        let lo_bits: Vec<AB::Var> = (0..C4_DIGIT_BITS).map(|k| local[base + 3 + k]).collect();
        let hi_bits: Vec<AB::Var> = (0..C4_DIGIT_BITS).map(|k| local[base + 10 + k]).collect();
        let slack_bits: Vec<AB::Var> = (0..C4_DIGIT_BITS).map(|k| local[base + 17 + k]).collect();
        let is96 = local[base + 24];
        let inv = local[base + 25];

        let recompose = |bits: &[AB::Var]| -> AB::Expr {
            let mut acc = AB::Expr::ZERO;
            for (b, bit) in bits.iter().enumerate() {
                acc += (*bit).into() * ce::<AB>(1u64 << b);
            }
            acc
        };
        for bit in lo_bits.iter().chain(hi_bits.iter()).chain(slack_bits.iter()) {
            builder.assert_bool(*bit);
        }
        // d_lo, d_hi recompose
        builder.assert_eq(d_lo.into(), recompose(&lo_bits));
        builder.assert_eq(d_hi.into(), recompose(&hi_bits));
        // slack = 96 - d_hi  ⇒ d_hi <= 96
        builder.assert_eq(
            ce::<AB>(C4_MAX_HI as u64) - d_hi.into(),
            recompose(&slack_bits),
        );
        // is96 boolean
        builder.assert_bool(is96);
        // is96 * (d_hi - 96) == 0
        builder.assert_eq(
            is96.into() * (d_hi.into() - ce::<AB>(C4_MAX_HI as u64)),
            AB::Expr::ZERO,
        );
        // (d_hi - 96) * inv == 1 - is96
        builder.assert_eq(
            (d_hi.into() - ce::<AB>(C4_MAX_HI as u64)) * inv.into(),
            AB::Expr::ONE - is96.into(),
        );
        // is96 * d_lo == 0
        builder.assert_eq(is96.into() * d_lo.into(), AB::Expr::ZERO);
        // reconstruction v = d_lo + 128*d_hi
        builder.assert_eq(
            v.into(),
            d_lo.into() + ce::<AB>(C4_BASE as u64) * d_hi.into(),
        );
        v.into()
    }

    /// Poseidon2 round constraints for one perm block (identical to c2_air).
    fn eval_perm_block<AB, LL>(
        &self,
        builder: &mut AB,
        pcols: &Poseidon2Cols<
            AB::Var,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >,
    ) -> [AB::Expr; WIDTH]
    where
        AB: AirBuilder<F = F>,
        LL: GenericPoseidon2LinearLayers<WIDTH>,
    {
        let mut state: [AB::Expr; WIDTH] = core::array::from_fn(|i| pcols.inputs[i].into());
        LL::external_linear_layer(&mut state);

        for round in 0..HALF_FULL_ROUNDS {
            let fr = &pcols.beginning_full_rounds[round];
            for i in 0..WIDTH {
                let after_rc = state[i].clone()
                    + Into::<AB::Expr>::into(self.beginning_full_rc[round][i].dup());
                let sb = eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(
                    builder,
                    &fr.sbox[i].0,
                    after_rc,
                );
                state[i] = sb;
            }
            LL::external_linear_layer(&mut state);
            for i in 0..WIDTH {
                builder.assert_eq(state[i].clone(), fr.post[i]);
                state[i] = fr.post[i].into();
            }
        }

        for round in 0..PARTIAL_ROUNDS {
            let pr = &pcols.partial_rounds[round];
            let after_rc =
                state[0].clone() + Into::<AB::Expr>::into(self.partial_rc[round].dup());
            let sb =
                eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(builder, &pr.sbox.0, after_rc);
            builder.assert_eq(sb, pr.post_sbox);
            state[0] = pr.post_sbox.into();
            LL::internal_linear_layer(&mut state);
        }

        for round in 0..HALF_FULL_ROUNDS {
            let fr = &pcols.ending_full_rounds[round];
            for i in 0..WIDTH {
                let after_rc =
                    state[i].clone() + Into::<AB::Expr>::into(self.ending_full_rc[round][i].dup());
                let sb = eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(
                    builder,
                    &fr.sbox[i].0,
                    after_rc,
                );
                state[i] = sb;
            }
            LL::external_linear_layer(&mut state);
            for i in 0..WIDTH {
                builder.assert_eq(state[i].clone(), fr.post[i]);
                state[i] = fr.post[i].into();
            }
        }

        state
    }
}

// ===========================================================================
// Witness / trace generation
// ===========================================================================

/// Honest secrets for one account-creation instance.
#[derive(Clone)]
pub struct FullSecrets {
    pub pad_seed: [u8; 32],
    pub slot: u64,
    pub account_projection: [u16; N_LANES],
    pub nonce_c1: [u8; 16],
    pub lifecycle_anchor: [u8; 32],
    pub nonce_c6: [u8; 16],
}

/// Computed public outputs + intermediate values from the honest sponge run.
#[derive(Clone)]
pub struct FullInstance<F> {
    pub pad_source_commitment: [F; N_COMMIT_LANES],
    pub opening_tuple_commitment: [F; N_COMMIT_LANES],
    pub masked_projection: [F; N_LANES],
    /// the 6 reduced C2 masks (carrier inputs); exposed for cross-checks.
    #[allow(dead_code)]
    pub masks: [u32; N_LANES],
    pub _phantom: PhantomData<F>,
}

/// Run one honest sponge chain, returning per-perm post states and squeezed
/// outputs. `elems` are the absorbed field values per padded position.
fn run_sponge<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    elems: &[F],
    plan: &SpongePlan,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
) -> (Vec<Vec<F>>, [F; WIDTH], Vec<F>) {
    // returns (per-perm packed col blocks, unused, outputs); also need post[]
    let mut perm_blocks: Vec<Vec<F>> = Vec::with_capacity(plan.n_perms);
    let mut posts: Vec<[F; WIDTH]> = Vec::with_capacity(plan.n_perms);
    let mut prev_post = [F::ZERO; WIDTH];
    for p in 0..plan.n_perms {
        let mut state = [F::ZERO; WIDTH];
        if p == 0 {
            for r in 0..RATE {
                state[r] = elems[r];
            }
        } else if p < plan.n_absorb {
            for r in 0..RATE {
                let pos = p * RATE + r;
                let add = if pos < plan.padded_len() { elems[pos] } else { F::ZERO };
                state[r] = prev_post[r] + add;
            }
            for c in RATE..WIDTH {
                state[c] = prev_post[c];
            }
        } else {
            state = prev_post;
        }

        let mut block: Vec<MaybeUninit<F>> = Vec::with_capacity(perm_cols);
        block.resize_with(perm_cols, MaybeUninit::uninit);
        let pcols: &mut Poseidon2Cols<
            MaybeUninit<F>,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        > = block.as_mut_slice().borrow_mut();
        generate_trace_rows_for_perm::<
            F,
            LinearLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(pcols, state, constants);
        let post = unsafe {
            let pp = &pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post;
            core::array::from_fn::<F, WIDTH, _>(|c| pp[c].assume_init())
        };
        let packed: Vec<F> = unsafe {
            block.iter().map(|m| m.assume_init()).collect()
        };
        perm_blocks.push(packed);
        posts.push(post);
        prev_post = post;
    }

    let mut outputs = vec![F::ZERO; plan.n_out];
    for o in 0..plan.n_out {
        let (sp, lane) = plan.output_source(o);
        outputs[o] = posts[sp][lane];
    }
    (perm_blocks, prev_post, outputs)
}

/// Build the absorbed field elements (with 10* pad) for a sponge from data lanes.
fn build_elems<F: PrimeField32>(data: &[u32], plan: &SpongePlan) -> Vec<F> {
    let mut elems = vec![F::ZERO; plan.padded_len()];
    for (i, &v) in data.iter().enumerate() {
        elems[i] = F::from_u32(v);
    }
    elems[plan.n_input] = F::ONE; // 10* marker
    elems
}

/// Build the C4 residue witness cells (as field elements) for value `v`.
///
/// `inv` is the FIELD inverse of `(d_hi - 96)` (when d_hi != 96), so it must be
/// computed over `F`, NOT mod q12289: the constraint `(d_hi - 96)*inv = 1 - is96`
/// is evaluated over the STARK field, where `d_hi - 96` (d_hi < 96) is the field
/// element `96 - (96 - d_hi)` reduced mod the field prime.
fn c4_witness<F: PrimeField32>(v: u32) -> ([F; C4_CELLS], F) {
    let d_lo = v % C4_BASE;
    let d_hi = v / C4_BASE;
    let slack = C4_MAX_HI - d_hi;
    let is96 = d_hi == C4_MAX_HI;
    // diff = d_hi - 96 in the field.
    let diff = F::from_u32(d_hi) - F::from_u32(C4_MAX_HI);
    let inv = if is96 { F::ZERO } else { diff.inverse() };
    let bits = |x: u32| -> [F; C4_DIGIT_BITS] {
        core::array::from_fn(|k| F::from_u32((x >> k) & 1))
    };
    let mut cells = [F::ZERO; C4_CELLS];
    cells[0] = F::from_u32(v);
    cells[1] = F::from_u32(d_lo);
    cells[2] = F::from_u32(d_hi);
    cells[3..10].copy_from_slice(&bits(d_lo));
    cells[10..17].copy_from_slice(&bits(d_hi));
    cells[17..24].copy_from_slice(&bits(slack));
    cells[24] = if is96 { F::ONE } else { F::ZERO };
    cells[25] = inv;
    (cells, F::from_u32(v))
}

/// Generate the full trace: `num_rows` identical honest rows + the instance.
#[allow(clippy::type_complexity)]
pub fn generate_full_trace<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    secrets: &FullSecrets,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    num_rows: usize,
) -> (RowMajorMatrix<F>, FullInstance<F>) {
    assert!(num_rows.is_power_of_two());
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let lay = FullLayout::new(perm_cols);
    let width = lay.width;

    let row = build_full_row::<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(secrets, &lay, constants, perm_cols);
    let inst = row.1;
    let row = row.0;

    let mut values = Vec::with_capacity(width * num_rows);
    for _ in 0..num_rows {
        values.extend_from_slice(&row);
    }
    (RowMajorMatrix::new(values, width), inst)
}

/// Generate a BATCHED trace: one DISTINCT account-creation instance per row.
/// `secrets[r]` is instance r; rows are padded up to the next power of two by
/// repeating the LAST honest instance (padding rows are still valid instances, so
/// they satisfy the relation; their public IO is included in the digest by the
/// caller only for the real rows 0..n_real). Returns the trace and the per-row
/// `FullInstance`s for the REAL instances (length = secrets.len()).
#[allow(clippy::type_complexity)]
pub fn generate_batch_trace<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    secrets: &[FullSecrets],
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
) -> (RowMajorMatrix<F>, Vec<FullInstance<F>>) {
    assert!(!secrets.is_empty());
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let lay = FullLayout::new(perm_cols);
    let width = lay.width;

    let n_real = secrets.len();
    let num_rows = n_real.next_power_of_two();

    let mut values = vec![F::ZERO; width * num_rows];
    let mut insts = Vec::with_capacity(n_real);

    // Real instances.
    let mut last_row: Vec<F> = Vec::new();
    for (r, s) in secrets.iter().enumerate() {
        let (row, inst) = build_full_row::<
            F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(s, &lay, constants, perm_cols);
        values[r * width..(r + 1) * width].copy_from_slice(&row);
        last_row = row;
        insts.push(inst);
    }
    // Padding rows: repeat the last real row (a valid instance) so every committed
    // row satisfies the relation. Padding-row public IO is NOT part of the digest.
    for r in n_real..num_rows {
        values[r * width..(r + 1) * width].copy_from_slice(&last_row);
    }

    (RowMajorMatrix::new(values, width), insts)
}

#[allow(clippy::type_complexity)]
fn build_full_row<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    secrets: &FullSecrets,
    lay: &FullLayout,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
) -> (Vec<F>, FullInstance<F>) {
    let mut row = vec![F::ZERO; lay.width];

    // ---- C1 data: pad_seed[32] ‖ nonce_c1[16] (bytes) ----
    let mut c1_data: Vec<u32> = secrets.pad_seed.iter().map(|&b| b as u32).collect();
    c1_data.extend(secrets.nonce_c1.iter().map(|&b| b as u32));
    // ---- C2 data: pad_seed[32] ‖ slot[8] (bytes) ----
    let mut c2_data: Vec<u32> = secrets.pad_seed.iter().map(|&b| b as u32).collect();
    c2_data.extend(secrets.slot.to_le_bytes().iter().map(|&b| b as u32));
    // ---- C6 data: account_projection[6] (field) ‖ anchor[32] ‖ nonce_c6[16] ----
    let mut c6_data: Vec<u32> = secrets.account_projection.iter().map(|&v| v as u32).collect();
    c6_data.extend(secrets.lifecycle_anchor.iter().map(|&b| b as u32));
    c6_data.extend(secrets.nonce_c6.iter().map(|&b| b as u32));

    let fill_sponge = |row: &mut [F], off: usize, slay: &SpongeLayout, plan: &SpongePlan, data: &[u32]| -> Vec<F> {
        // lanes
        for (i, &v) in data.iter().enumerate() {
            row[off + slay.lanes + i] = F::from_u32(v);
        }
        // lane bits (byte lanes only, packed)
        let mut bit_cursor = 0usize;
        for (i, &v) in data.iter().enumerate() {
            if plan.byte_ranged[i] {
                for b in 0..BYTE_BITS {
                    row[off + slay.lane_bits + bit_cursor * BYTE_BITS + b] = F::from_u32((v >> b) & 1);
                }
                bit_cursor += 1;
            }
        }
        let elems = build_elems::<F>(data, plan);
        let (blocks, _post, outputs) = run_sponge::<
            F,
            LinearLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(&elems, plan, constants, perm_cols);
        for (p, b) in blocks.iter().enumerate() {
            let base = off + slay.perms + p * perm_cols;
            row[base..base + perm_cols].copy_from_slice(b);
        }
        outputs
    };

    let c1_out = fill_sponge(&mut row, lay.c1_off, &lay.c1, &lay.c1_plan, &c1_data);
    let c2_out = fill_sponge(&mut row, lay.c2_off, &lay.c2, &lay.c2_plan, &c2_data);
    let c6_out = fill_sponge(&mut row, lay.c6_off, &lay.c6, &lay.c6_plan, &c6_data);

    // ---- C2 WIDE-THEN-REDUCE gadget columns (30 squeezed lanes -> 6 masks) ----
    let mut squeezed = [0u32; NUM_SQUEEZE];
    for (i, s) in squeezed.iter_mut().enumerate() {
        *s = c2_out[i].as_canonical_u32();
    }
    let masks = fill_reduce_gadget::<F>(&mut row, lay.c2_off + lay.c2.reduce, &squeezed);
    debug_assert_eq!(masks, wide_reduce_masks::<F>(&squeezed));

    // ---- C4 residues + carrier ----
    let mut masked_projection = [F::ZERO; N_LANES];
    for j in 0..N_LANES {
        let v = secrets.account_projection[j] as u32;
        let (cells, vv) = c4_witness::<F>(v);
        let base = lay.c4_off + j * C4_CELLS;
        row[base..base + C4_CELLS].copy_from_slice(&cells);
        row[lay.carry_acc_off + j] = vv;
        // carrier: masked = mask + acc in the field (matches the AIR constraint).
        let masked = F::from_u32(masks[j]) + vv;
        row[lay.carry_masked_off + j] = masked;
        masked_projection[j] = masked;
    }

    let mut pad_source_commitment = [F::ZERO; N_COMMIT_LANES];
    for j in 0..N_COMMIT_LANES {
        pad_source_commitment[j] = c1_out[j];
    }
    let mut opening_tuple_commitment = [F::ZERO; N_COMMIT_LANES];
    for j in 0..N_COMMIT_LANES {
        opening_tuple_commitment[j] = c6_out[j];
    }

    // ---- per-row PUBLIC-IO columns (used in batch mode; harmless otherwise) ----
    // [pad_src_commit 0..10][opening_commit 10..20][masked 20..26].
    for j in 0..N_COMMIT_LANES {
        row[lay.pubio_off + j] = pad_source_commitment[j];
        row[lay.pubio_off + N_COMMIT_LANES + j] = opening_tuple_commitment[j];
    }
    for j in 0..N_LANES {
        row[lay.pubio_off + 2 * N_COMMIT_LANES + j] = masked_projection[j];
    }

    let inst = FullInstance {
        pad_source_commitment,
        opening_tuple_commitment,
        masked_projection,
        masks,
        _phantom: PhantomData,
    };
    (row, inst)
}

/// Public-values vector: pad_source_commitment[10] ‖ opening_tuple[10] ‖ masked[6].
pub fn full_public_values<F: PrimeField32>(inst: &FullInstance<F>) -> Vec<F> {
    let mut v = Vec::with_capacity(2 * N_COMMIT_LANES + N_LANES);
    v.extend_from_slice(&inst.pad_source_commitment);
    v.extend_from_slice(&inst.opening_tuple_commitment);
    v.extend_from_slice(&inst.masked_projection);
    v
}
