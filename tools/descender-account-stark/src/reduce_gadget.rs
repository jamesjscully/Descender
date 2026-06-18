//! Shared WIDE-THEN-REDUCE mask gadget + small AIR helpers.
//!
//! Extracted verbatim (constants, `StepOff` layout, `eval_reduce_gadget`,
//! `fill_reduce_gadget`, `wide_reduce_masks`, `perm_num_cols`, `ce`) from the
//! re-host benchmark's `c2_air.rs`. This is the 128-bit-hiding mask derivation:
//! each of the 6 mask lanes is `mask_j = (sum_i lane_{j,i} * p^i) mod q12289`
//! over FIVE squeezed base-`p` digits, so a uniform `W < p^5 ~ 2^155` reduced
//! mod `q` has bias `<= q/p^5 ~ 2^-141` (vs the naive 1-lane ~2^-17). The
//! integer Horner reduction is constrained wrap-free via a base-2^16 limb/carry
//! encoding; see the original module doc in `c2_air.rs` for the full soundness
//! argument.

extern crate alloc;
use alloc::vec::Vec;

use p3_air::AirBuilder;
use p3_field::integers::QuotientMap;
use p3_field::{PrimeCharacteristicRing, PrimeField, PrimeField32, PrimeField64};
use p3_poseidon2_air::Poseidon2Cols;

/// q12289 modulus for the projection-mask lanes.
pub const Q12289: u32 = 12289;
/// Number of projection-mask lanes.
pub const NUM_OUTPUTS: usize = 6;
/// Wide-then-reduce: squeezed field lanes per mask (base-`p` digits of `W`).
pub const LANES_PER_MASK: usize = 5;
/// Total squeezed field lanes = 6 masks * 5 lanes each.
pub const NUM_SQUEEZE: usize = NUM_OUTPUTS * LANES_PER_MASK; // 30
/// Sponge rate (elements absorbed/squeezed per permutation) for WIDTH=16.
pub const RATE: usize = 8;
/// `mask < q12289` (~14-bit). 14 bits covers [0, 16384) ⊇ [0, q).
const MASK_BITS: usize = 14;

// --- wide-then-reduce limb/carry gadget constants (base 2^16) ---
const LIMB_BASE_BITS: usize = 16;
const LO_BITS: usize = 16;
const HI_BITS: usize = 15;
const S_LO_BITS: usize = 16;
const S_TOP_BITS: usize = 14;
const HORNER_STEPS: usize = LANES_PER_MASK; // 5

/// Field cells per Horner step (scalar values, NOT counting bit cells).
const STEP_VALS: usize = 11;
/// Bit cells per Horner step.
const STEP_BITS: usize = LO_BITS
    + HI_BITS
    + LO_BITS
    + HI_BITS
    + S_LO_BITS
    + S_LO_BITS
    + S_TOP_BITS
    + S_TOP_BITS // c0
    + S_TOP_BITS // c1
    + S_TOP_BITS // d0
    + S_TOP_BITS; // d1
/// `r_t` value + bits + slack value + bits.
const STEP_R: usize = 1 + MASK_BITS + 1 + MASK_BITS;
/// Total field cells per Horner step.
const STEP_CELLS: usize = STEP_VALS + STEP_BITS + STEP_R;
/// Per mask lane: 5 squeezed `lane` value copies + 5 Horner steps.
const PER_MASK_CELLS: usize = LANES_PER_MASK + HORNER_STEPS * STEP_CELLS;
/// All 6 mask lanes' reduce-gadget columns.
pub const REDUCE_CELLS: usize = NUM_OUTPUTS * PER_MASK_CELLS;

/// Offsets within ONE Horner step block (relative to the step base).
struct StepOff;
impl StepOff {
    const QT_LO: usize = 0;
    const QT_HI: usize = 1;
    const LANE_LO: usize = 2;
    const LANE_HI: usize = 3;
    const S0: usize = 4;
    const S1: usize = 5;
    const S2: usize = 6;
    const C0: usize = 7; // LHS carries
    const C1: usize = 8;
    const D0: usize = 9; // RHS carries
    const D1: usize = 10;
    // bit blocks start after the value cells:
    const QT_LO_BITS: usize = STEP_VALS;
    const QT_HI_BITS: usize = Self::QT_LO_BITS + LO_BITS;
    const LANE_LO_BITS: usize = Self::QT_HI_BITS + HI_BITS;
    const LANE_HI_BITS: usize = Self::LANE_LO_BITS + LO_BITS;
    const S0_BITS: usize = Self::LANE_HI_BITS + HI_BITS;
    const S1_BITS: usize = Self::S0_BITS + S_LO_BITS;
    const S2_BITS: usize = Self::S1_BITS + S_LO_BITS;
    const C0_BITS: usize = Self::S2_BITS + S_TOP_BITS;
    const C1_BITS: usize = Self::C0_BITS + S_TOP_BITS;
    const D0_BITS: usize = Self::C1_BITS + S_TOP_BITS;
    const D1_BITS: usize = Self::D0_BITS + S_TOP_BITS;
    // r block after all bits:
    const R: usize = STEP_VALS + STEP_BITS;
    const R_BITS: usize = Self::R + 1;
    const SLACK: usize = Self::R_BITS + MASK_BITS;
    const SLACK_BITS: usize = Self::SLACK + 1;
}

/// Field-modulus base-2^16 split for `F`.
fn p_lo_hi<F: PrimeField64>() -> (u64, u64) {
    let p = F::ORDER_U64;
    let lo = p & ((1u64 << LIMB_BASE_BITS) - 1);
    let hi = p >> LIMB_BASE_BITS;
    (lo, hi)
}

/// Const-from-int helper for AIR expressions.
#[inline]
pub fn ce<AB: AirBuilder>(x: u64) -> AB::Expr
where
    AB::F: PrimeField,
{
    <AB::F as QuotientMap<u64>>::from_int(x).into()
}

/// `num_cols` of a single Poseidon2 block (p3-poseidon2-air's is `pub(crate)`;
/// reconstruct via the public `Poseidon2Cols` size).
pub const fn perm_num_cols<
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>() -> usize {
    core::mem::size_of::<
        Poseidon2Cols<u8, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    >()
}

/// Evaluate the wide-then-reduce gadget for all NUM_OUTPUTS mask lanes. `squeezed`
/// holds the 30 squeezed output exprs (lane_{j,i} = squeezed[j*5 + i]); the gadget
/// binds them to its own `lane` columns and returns the 6 reduced mask exprs.
pub fn eval_reduce_gadget<AB>(
    builder: &mut AB,
    base: usize,
    local: &[AB::Var],
    squeezed: &[AB::Expr],
) -> Vec<AB::Expr>
where
    AB: AirBuilder,
    AB::F: PrimeField64,
{
    let (p_lo, p_hi) = p_lo_hi::<AB::F>();
    let q = Q12289 as u64;
    let two16 = 1u64 << LIMB_BASE_BITS;

    let recompose = |builder: &mut AB, start: usize, nbits: usize| -> AB::Expr {
        let mut acc = AB::Expr::ZERO;
        for b in 0..nbits {
            let bit = local[base + start + b];
            builder.assert_bool(bit);
            acc += bit.into() * ce::<AB>(1u64 << b);
        }
        acc
    };

    let mut masks = Vec::with_capacity(NUM_OUTPUTS);
    for j in 0..NUM_OUTPUTS {
        let mask_base = j * PER_MASK_CELLS;
        for i in 0..LANES_PER_MASK {
            let col = base + mask_base + i;
            builder.assert_eq(local[col].into(), squeezed[j * LANES_PER_MASK + i].clone());
        }
        let lane_val = |i: usize| local[base + mask_base + i];

        let mut r_prev: AB::Expr = AB::Expr::ZERO;
        let step0 = mask_base + LANES_PER_MASK;
        for t in 0..HORNER_STEPS {
            let sb = step0 + t * STEP_CELLS;
            let digit = lane_val(LANES_PER_MASK - 1 - t);

            let qt_lo = recompose(builder, sb + StepOff::QT_LO_BITS, LO_BITS);
            builder.assert_eq(local[base + sb + StepOff::QT_LO].into(), qt_lo.clone());
            let qt_hi = recompose(builder, sb + StepOff::QT_HI_BITS, HI_BITS);
            builder.assert_eq(local[base + sb + StepOff::QT_HI].into(), qt_hi.clone());
            let lane_lo = recompose(builder, sb + StepOff::LANE_LO_BITS, LO_BITS);
            builder.assert_eq(local[base + sb + StepOff::LANE_LO].into(), lane_lo.clone());
            let lane_hi = recompose(builder, sb + StepOff::LANE_HI_BITS, HI_BITS);
            builder.assert_eq(local[base + sb + StepOff::LANE_HI].into(), lane_hi.clone());
            let s0 = recompose(builder, sb + StepOff::S0_BITS, S_LO_BITS);
            builder.assert_eq(local[base + sb + StepOff::S0].into(), s0.clone());
            let s1 = recompose(builder, sb + StepOff::S1_BITS, S_LO_BITS);
            builder.assert_eq(local[base + sb + StepOff::S1].into(), s1.clone());
            let s2 = recompose(builder, sb + StepOff::S2_BITS, S_TOP_BITS);
            builder.assert_eq(local[base + sb + StepOff::S2].into(), s2.clone());
            let c0 = recompose(builder, sb + StepOff::C0_BITS, S_TOP_BITS);
            builder.assert_eq(local[base + sb + StepOff::C0].into(), c0.clone());
            let c1 = recompose(builder, sb + StepOff::C1_BITS, S_TOP_BITS);
            builder.assert_eq(local[base + sb + StepOff::C1].into(), c1.clone());
            let d0 = recompose(builder, sb + StepOff::D0_BITS, S_TOP_BITS);
            builder.assert_eq(local[base + sb + StepOff::D0].into(), d0.clone());
            let d1 = recompose(builder, sb + StepOff::D1_BITS, S_TOP_BITS);
            builder.assert_eq(local[base + sb + StepOff::D1].into(), d1.clone());

            let r_t = recompose(builder, sb + StepOff::R_BITS, MASK_BITS);
            builder.assert_eq(local[base + sb + StepOff::R].into(), r_t.clone());
            let slack = recompose(builder, sb + StepOff::SLACK_BITS, MASK_BITS);
            builder.assert_eq(local[base + sb + StepOff::SLACK].into(), slack.clone());
            builder.assert_eq(r_t.clone() + slack, ce::<AB>(q - 1));

            builder.assert_eq(
                digit.into(),
                lane_lo.clone() + lane_hi.clone() * ce::<AB>(two16),
            );

            let l0 = r_prev.clone() * ce::<AB>(p_lo) + lane_lo;
            let l1 = r_prev.clone() * ce::<AB>(p_hi) + lane_hi;

            let r0 = qt_lo * ce::<AB>(q) + r_t.clone();
            let r1 = qt_hi * ce::<AB>(q);

            builder.assert_eq(l0, s0.clone() + c0.clone() * ce::<AB>(two16));
            builder.assert_eq(l1 + c0.clone(), s1.clone() + c1.clone() * ce::<AB>(two16));
            builder.assert_eq(c1.clone(), s2.clone());

            builder.assert_eq(r0, s0.clone() + d0.clone() * ce::<AB>(two16));
            builder.assert_eq(r1 + d0.clone(), s1.clone() + d1.clone() * ce::<AB>(two16));
            builder.assert_eq(d1.clone(), s2.clone());

            r_prev = r_t.clone();
            if t == HORNER_STEPS - 1 {
                masks.push(r_t);
            }
        }
    }
    masks
}

/// Reference reduction: `mask_j = (sum_i lane_{j,i} * p^i) mod q`.
pub fn wide_reduce_masks<F: PrimeField32>(lanes: &[u32; NUM_SQUEEZE]) -> [u32; NUM_OUTPUTS] {
    let p = F::ORDER_U32 as u64;
    let q = Q12289 as u64;
    let mut masks = [0u32; NUM_OUTPUTS];
    for j in 0..NUM_OUTPUTS {
        let mut r: u64 = 0;
        for t in 0..HORNER_STEPS {
            let digit = lanes[j * LANES_PER_MASK + (LANES_PER_MASK - 1 - t)] as u64;
            r = (r * p + digit) % q;
        }
        masks[j] = r as u32;
    }
    masks
}

/// Fill the wide-then-reduce gadget's witness columns for all 6 masks into `row`
/// at column offset `base`.
pub fn fill_reduce_gadget<F: PrimeField32>(
    row: &mut [F],
    base: usize,
    lanes: &[u32; NUM_SQUEEZE],
) -> [u32; NUM_OUTPUTS] {
    let p = F::ORDER_U32 as u64;
    let q = Q12289 as u64;
    let two16 = 1u64 << LIMB_BASE_BITS;
    let (p_lo, p_hi) = p_lo_hi::<F>();

    let put_bits = |row: &mut [F], at: usize, val: u64, n: usize| {
        for b in 0..n {
            row[at + b] = F::from_u32(((val >> b) & 1) as u32);
        }
    };

    let mut masks = [0u32; NUM_OUTPUTS];
    for j in 0..NUM_OUTPUTS {
        let mask_base = base + j * PER_MASK_CELLS;
        for i in 0..LANES_PER_MASK {
            row[mask_base + i] = F::from_u32(lanes[j * LANES_PER_MASK + i]);
        }
        let step0 = mask_base + LANES_PER_MASK;
        let mut r_prev: u64 = 0;
        for t in 0..HORNER_STEPS {
            let sb = step0 + t * STEP_CELLS;
            let lane = lanes[j * LANES_PER_MASK + (LANES_PER_MASK - 1 - t)] as u64;

            let val = r_prev * p + lane;
            let qt = val / q;
            let r_t = val % q;
            let slack = (q - 1) - r_t;

            let qt_lo = qt & (two16 - 1);
            let qt_hi = qt >> LIMB_BASE_BITS;
            let lane_lo = lane & (two16 - 1);
            let lane_hi = lane >> LIMB_BASE_BITS;

            let l0 = r_prev * p_lo + lane_lo;
            let l1 = r_prev * p_hi + lane_hi;
            let r0 = qt_lo * q + r_t;
            let r1 = qt_hi * q;

            let s_full = l0 + two16 * l1;
            debug_assert_eq!(s_full, r0 + two16 * r1);
            let s0 = s_full & (two16 - 1);
            let s1 = (s_full >> LIMB_BASE_BITS) & (two16 - 1);
            let s2 = s_full >> (2 * LIMB_BASE_BITS);

            let c0 = (l0 - s0) >> LIMB_BASE_BITS;
            let c1 = (l1 + c0 - s1) >> LIMB_BASE_BITS;
            let d0 = (r0 - s0) >> LIMB_BASE_BITS;
            let d1 = (r1 + d0 - s1) >> LIMB_BASE_BITS;
            debug_assert_eq!(c1, s2);
            debug_assert_eq!(d1, s2);

            row[sb + StepOff::QT_LO] = F::from_u32(qt_lo as u32);
            row[sb + StepOff::QT_HI] = F::from_u32(qt_hi as u32);
            row[sb + StepOff::LANE_LO] = F::from_u32(lane_lo as u32);
            row[sb + StepOff::LANE_HI] = F::from_u32(lane_hi as u32);
            row[sb + StepOff::S0] = F::from_u32(s0 as u32);
            row[sb + StepOff::S1] = F::from_u32(s1 as u32);
            row[sb + StepOff::S2] = F::from_u32(s2 as u32);
            row[sb + StepOff::C0] = F::from_u32(c0 as u32);
            row[sb + StepOff::C1] = F::from_u32(c1 as u32);
            row[sb + StepOff::D0] = F::from_u32(d0 as u32);
            row[sb + StepOff::D1] = F::from_u32(d1 as u32);
            put_bits(row, sb + StepOff::QT_LO_BITS, qt_lo, LO_BITS);
            put_bits(row, sb + StepOff::QT_HI_BITS, qt_hi, HI_BITS);
            put_bits(row, sb + StepOff::LANE_LO_BITS, lane_lo, LO_BITS);
            put_bits(row, sb + StepOff::LANE_HI_BITS, lane_hi, HI_BITS);
            put_bits(row, sb + StepOff::S0_BITS, s0, S_LO_BITS);
            put_bits(row, sb + StepOff::S1_BITS, s1, S_LO_BITS);
            put_bits(row, sb + StepOff::S2_BITS, s2, S_TOP_BITS);
            put_bits(row, sb + StepOff::C0_BITS, c0, S_TOP_BITS);
            put_bits(row, sb + StepOff::C1_BITS, c1, S_TOP_BITS);
            put_bits(row, sb + StepOff::D0_BITS, d0, S_TOP_BITS);
            put_bits(row, sb + StepOff::D1_BITS, d1, S_TOP_BITS);
            row[sb + StepOff::R] = F::from_u32(r_t as u32);
            put_bits(row, sb + StepOff::R_BITS, r_t, MASK_BITS);
            row[sb + StepOff::SLACK] = F::from_u32(slack as u32);
            put_bits(row, sb + StepOff::SLACK_BITS, slack, MASK_BITS);

            r_prev = r_t;
            if t == HORNER_STEPS - 1 {
                masks[j] = r_t as u32;
            }
        }
    }
    masks
}
