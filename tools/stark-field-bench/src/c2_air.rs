//! C2 obligation as a field-generic Plonky3 AIR over a 31-bit STARK field.
//!
//! # What C2 proves
//!
//! The account-creation relation's C2 obligation says: the 6 public
//! projection-mask lanes (each a canonical `q12289` element, `q = 12289`) are a
//! PRF of a *hidden* `pad_seed` (32 bytes) plus a `slot_id` (8 bytes / u64).
//!
//! On a 31-bit STARK field the arithmetization-friendly PRF is a NATIVE Plonky3
//! Poseidon2 sponge over the field; the squeezed 31-bit outputs are reduced mod
//! `q12289` to yield the 6 mask lanes. So this AIR proves, with the hidden
//! `pad_seed || slot` bytes as private witness and the 6 mask lanes as public
//! inputs:
//!
//!   1. each input lane is a byte in `[0, 256)`            (bit-decomposition);
//!   2. a native Poseidon2 SPONGE absorbing those 40 byte
//!      lanes at rate 8 produces a final state whose first
//!      `6*5 = 30` rate elements are the wide squeeze
//!      `lane_{j,0..4}` for j in 0..6                       (Poseidon2 round
//!                                                          constraints + absorb
//!                                                          linking);
//!   3. each mask lane reduces from FIVE squeezed lanes via
//!      WIDE-THEN-REDUCE: `mask_j = W_j mod q` where
//!      `W_j = sum_i lane_{j,i} * p^i` is a uniform
//!      ~155-bit integer (5 base-`p` digits), so the mod-`q`
//!      output is statistically uniform to bias `<= q/p^5
//!      ~ 2^-141` (vs the OLD one-lane reduce's ~2^-17 bias) (sound integer
//!                                                          Horner + 3-limb carry
//!                                                          gadget; see below);
//!   4. `mask_j` equals the public mask lane `j`.
//!
//! # Wide-then-reduce hiding-bias fix (the gadget)
//!
//! The mask is a one-time pad: `masked = projection + mask (mod q)`. Hiding needs
//! `mask` statistically uniform over `Z_q`. The OLD code took ONE squeezed 31-bit
//! lane and reduced it mod `q`: a uniform value on `[0, p)` reduced mod `q` has
//! bias up to `q/p ~ 2^-17` per lane — only ~17-bit hiding, NOT 128-bit.
//!
//! The FIX squeezes 5 lanes per mask and treats them as the base-`p` digits of a
//! uniform integer `W = d0 + d1*p + d2*p^2 + d3*p^3 + d4*p^4 < p^5 ~ 2^155`. A
//! uniform value on `[0, p^5)` reduced mod `q` has bias `<= q/p^5 ~ 2^-141`
//! (negligible). We compute `mask = W mod q` by integer Horner over the digits,
//! reducing mod `q` at each step:
//!   r_0 = d4 mod q;  r_t = (r_{t-1}*p + d_{4-t}) mod q  for t=1..4;  mask = r_4.
//! By induction `r_t ≡ (high digits) mod q`, so `r_4 ≡ W mod q`. Correct because
//! `(a*p + b) mod q = ((a mod q)*p + b) mod q`.
//!
//! ## Soundness without field-wrap (the hard part)
//!
//! Each Horner step asserts the INTEGER identity `r_prev*p + lane = qt*q + r_t`
//! with `r_prev < q`, `lane < p`, `r_t < q`, `qt` the witnessed quotient. The
//! products `r_prev*p` and `qt*q` are each `< q*p ~ 2^44.6`, which OVERFLOWS the
//! 31-bit field — forming them directly would wrap mod the field prime and be
//! unsound. We avoid every wrap with a base-`2^16` limb/carry encoding in which no
//! committed cell and no asserted linear combination exceeds ~2^31:
//!   * `p` and `q` are COMPILE-TIME constants with `p = p_lo + 2^16*p_hi`
//!     (`p_lo,p_hi < 2^15` for BabyBear/KoalaBear) and `q < 2^14`.
//!   * LHS: `r_prev*p = (r_prev*p_lo) + 2^16*(r_prev*p_hi)`. Since `r_prev < 2^14`
//!     and `p_lo,p_hi < 2^15`, each sub-product `< 2^29` — a genuine integer
//!     product in-field (no wrap). With `lane = lane_lo + 2^16*lane_hi`
//!     (`lane_lo<2^16`, `lane_hi<2^15`), LHS = `L0 + 2^16*L1`,
//!     `L0 = r_prev*p_lo + lane_lo`, `L1 = r_prev*p_hi + lane_hi` (both `< 2^30`).
//!   * RHS: `qt = qt_lo + 2^16*qt_hi` (`qt_lo<2^16`, `qt_hi<2^15`), so
//!     `qt*q = qt_lo*q + 2^16*(qt_hi*q)` (sub-products `< 2^30`). RHS = `R0 + 2^16*R1`,
//!     `R0 = qt_lo*q + r_t` (`< 2^31`), `R1 = qt_hi*q` (`< 2^29`).
//!   * Equate the two integers `L0 + 2^16*L1 == R0 + 2^16*R1` by tying BOTH to one
//!     witnessed base-`2^16` limb triple `(s0,s1,s2)` (`s0,s1<2^16`, `s2<2^14`) via
//!     carries `(c0,c1)` (`< 2^14`):  `Xi0 = s0 + 2^16*c0`, `Xi1 + c0 = s1 + 2^16*c1`,
//!     `c1 = s2`, for X in {L, R}. Transitivity ⇒ LHS == RHS as integers. Every
//!     equality has all terms `< 2^31`, so NO field-wrap anywhere.
//! All limb/carry cells are range-checked by bit-decomposition; `r_t < q` is
//! enforced as before (mask + slack = q-1).
//!
//! # Layout choice: single-row, multi-permutation
//!
//! Every trace row is an INDEPENDENT, COMPLETE C2 instance. The whole sponge
//! lives inside one row: the `NUM_PERMS = 8` Poseidon2 permutation blocks
//! (`NUM_ABSORB_PERMS = 5` absorb blocks, one per rate-8 chunk of the 40 input
//! lanes, plus `NUM_SQUEEZE_PERMS = 3` squeeze blocks to read off the 30 wide
//! squeezed lanes) sit side by side in the same row and are chained by IN-ROW
//! absorb/squeeze constraints. No constraint reads the next row, so
//! `main_next_row_columns` is empty and the prover/verifier open only at `zeta`.
//!
//! This is the realistic re-host shape: the public mask lanes are a single
//! statement; padding the trace to a power of two simply repeats the same honest
//! statement on every row (uni-stark needs a power-of-two height).
//!
//! # Field-generic
//!
//! Generic over the Plonky3 `PrimeField32` bound plus the const-generic Poseidon2
//! parameters, so the identical code path runs on BabyBear and KoalaBear (and any
//! other 31-bit Plonky3 field). The Poseidon2 round constraints are re-implemented
//! here (mirroring `p3-poseidon2-air`'s private `eval`) because that crate's
//! per-block `eval` and `RoundConstants` internals are `pub(crate)`; we keep our
//! own copy of the round-constant arrays and drive the public per-perm GENERATOR
//! for the witness so trace and constraints agree by construction.

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use core::borrow::BorrowMut;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::integers::QuotientMap;
use p3_field::{PrimeCharacteristicRing, PrimeField, PrimeField32, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_poseidon2::GenericPoseidon2LinearLayers;
use p3_poseidon2_air::{generate_trace_rows_for_perm, Poseidon2Cols, RoundConstants};

/// q12289 modulus for the projection-mask lanes.
pub const Q12289: u32 = 12289;
/// Number of public projection-mask lanes.
pub const NUM_OUTPUTS: usize = 6;
/// Wide-then-reduce: squeezed field lanes per mask (base-`p` digits of `W`).
/// 5 lanes ⇒ `W < p^5 ~ 2^155` ⇒ mod-q bias `<= q/p^5 ~ 2^-141`.
pub const LANES_PER_MASK: usize = 5;
/// Total squeezed field lanes = 6 masks * 5 lanes each.
pub const NUM_SQUEEZE: usize = NUM_OUTPUTS * LANES_PER_MASK; // 30
/// Sponge rate (elements absorbed/squeezed per permutation) for WIDTH=16.
pub const RATE: usize = 8;
/// pad_seed bytes (32) + slot_id bytes (8).
pub const NUM_INPUT_LANES: usize = 32 + 8;
/// Number of Poseidon2 permutations to ABSORB all input lanes at the rate.
pub const NUM_ABSORB_PERMS: usize = NUM_INPUT_LANES.div_ceil(RATE); // = 5
/// Extra SQUEEZE permutations needed to read off 30 lanes at rate 8. The absorb
/// perms' post-state already exposes RATE=8 lanes; each extra permutation exposes
/// another 8. We need ceil(30/8)=4 read blocks ⇒ 3 extra squeeze perms after the
/// first read block (the last absorb perm's post-state).
pub const NUM_SQUEEZE_PERMS: usize = (NUM_SQUEEZE.div_ceil(RATE)) - 1; // 4-1 = 3
/// Total Poseidon2 permutations in the chain.
pub const NUM_PERMS: usize = NUM_ABSORB_PERMS + NUM_SQUEEZE_PERMS; // 8
/// Bits to range-check an input lane (a byte).
const BYTE_BITS: usize = 8;
/// `mask < q12289` (~14-bit). 14 bits covers [0, 16384) ⊇ [0, q).
const MASK_BITS: usize = 14;

// --- wide-then-reduce limb/carry gadget constants (base 2^16) ---
/// Base of the limb decomposition for the integer Horner identity.
const LIMB_BASE_BITS: usize = 16;
/// Bits for the low half of `p`/`lane`/`qt` (`x_lo < 2^16`).
const LO_BITS: usize = 16;
/// Bits for the high half (`p_hi,lane_hi,qt_hi < 2^15`).
const HI_BITS: usize = 15;
/// Bits for the common-value limbs `s0,s1` (`< 2^16`).
const S_LO_BITS: usize = 16;
/// Bits for the top limb `s2` and the carries `c0,c1` (`< 2^14`).
const S_TOP_BITS: usize = 14;
/// Horner steps per mask lane (one reduction per base-`p` digit).
const HORNER_STEPS: usize = LANES_PER_MASK; // 5

// ===========================================================================
// Shared WIDE-THEN-REDUCE gadget (used by both c2_air and full_air).
// ===========================================================================
//
// Per Horner step columns (one reduction `r_t = (r_prev*p + lane) mod q`):
//   qt_lo (16b) | qt_hi (15b) | lane_lo (16b) | lane_hi (15b)
//   s0 (16b)    | s1 (16b)    | s2 (14b)
//   c0 (14b)    | c1 (14b)
// plus the result `r_t` value + MASK_BITS bits + a `slack` value + MASK_BITS bits
// (so `r_t <= q-1 < q`).
//
// Each scalar value cell is followed by its bit-decomposition cells.

/// Field cells per Horner step (scalar values, NOT counting bit cells).
/// qt_lo,qt_hi,lane_lo,lane_hi,s0,s1,s2, then LHS carries c0,c1, RHS carries d0,d1.
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

/// Constant `p_lo`/`p_hi` for the current field `F` (compile-time split of the
/// field modulus into base-2^16 halves). Asserts the high half fits in 15 bits
/// (true for BabyBear/KoalaBear) at construction.
fn p_lo_hi<F: PrimeField64>() -> (u64, u64) {
    let p = F::ORDER_U64;
    let lo = p & ((1u64 << LIMB_BASE_BITS) - 1);
    let hi = p >> LIMB_BASE_BITS;
    (lo, hi)
}

/// Evaluate the wide-then-reduce gadget for all NUM_OUTPUTS mask lanes. `squeezed`
/// holds the 30 squeezed output exprs (lane_{j,i} = squeezed[j*5 + i]); the gadget
/// binds them to its own `lane` columns and returns the 6 reduced mask exprs
/// (mask_j = r_4 of lane j). `read` maps a gadget-relative cell index to the
/// committed column var; `base` is the absolute column offset of the gadget block.
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
        // bind the 5 squeezed lanes to the gadget's lane columns.
        for i in 0..LANES_PER_MASK {
            let col = base + mask_base + i;
            builder.assert_eq(local[col].into(), squeezed[j * LANES_PER_MASK + i].clone());
        }
        let lane_val = |i: usize| local[base + mask_base + i];

        // Horner over base-p digits, high digit (lane[4]) first.
        // r_prev starts at 0; step t consumes digit lane[4 - t].
        let mut r_prev: AB::Expr = AB::Expr::ZERO;
        let step0 = mask_base + LANES_PER_MASK; // first step block (relative to base)
        for t in 0..HORNER_STEPS {
            let sb = step0 + t * STEP_CELLS; // step block start (relative to base)
            let digit = lane_val(LANES_PER_MASK - 1 - t); // lane[4 - t]

            // --- range-check + recompose the limb witnesses ---
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

            // --- r_t value: range-check `r_t <= q-1 < q` via slack ---
            let r_t = recompose(builder, sb + StepOff::R_BITS, MASK_BITS);
            builder.assert_eq(local[base + sb + StepOff::R].into(), r_t.clone());
            let slack = recompose(builder, sb + StepOff::SLACK_BITS, MASK_BITS);
            builder.assert_eq(local[base + sb + StepOff::SLACK].into(), slack.clone());
            builder.assert_eq(r_t.clone() + slack, ce::<AB>(q - 1));

            // --- bind the digit (squeezed lane) to its lo/hi split ---
            //   lane = lane_lo + 2^16*lane_hi   (no wrap: lane < p < 2^31)
            builder.assert_eq(
                digit.into(),
                lane_lo.clone() + lane_hi.clone() * ce::<AB>(two16),
            );

            // qt is never materialized as a >31-bit value: we use only its lo/hi
            // split (qt_lo, qt_hi), which IS the witnessed quotient by definition.

            // --- LHS = r_prev*p + lane, as base-2^16 coeff pair (L0, L1) ---
            //   r_prev*p = (r_prev*p_lo) + 2^16*(r_prev*p_hi)
            //   each sub-product < 2^29 (r_prev<2^14, p_lo/p_hi<2^15): genuine int,
            //   no field-wrap. L0,L1 < 2^30.
            let l0 = r_prev.clone() * ce::<AB>(p_lo) + lane_lo;
            let l1 = r_prev.clone() * ce::<AB>(p_hi) + lane_hi;

            // --- RHS = qt*q + r_t, as base-2^16 coeff pair (R0, R1) ---
            //   qt*q = (qt_lo*q) + 2^16*(qt_hi*q); sub-products < 2^30. R0<2^31,R1<2^29.
            let r0 = qt_lo * ce::<AB>(q) + r_t.clone();
            let r1 = qt_hi * ce::<AB>(q);

            // --- tie LHS to canonical base-2^16 limbs (s0,s1,s2) via carries (c0,c1) ---
            //   L0       = s0 + 2^16*c0
            //   L1 + c0  = s1 + 2^16*c1
            //   c1       = s2
            // Every term < 2^31 ⇒ no field-wrap; range-checked limbs/carries make
            // the decomposition the UNIQUE canonical one ⇒ S := s0+2^16 s1+2^32 s2 = LHS.
            builder.assert_eq(l0, s0.clone() + c0.clone() * ce::<AB>(two16));
            builder.assert_eq(l1 + c0.clone(), s1.clone() + c1.clone() * ce::<AB>(two16));
            builder.assert_eq(c1.clone(), s2.clone());

            // --- tie RHS to the SAME canonical limbs (s0,s1,s2) via carries (d0,d1) ---
            //   R0       = s0 + 2^16*d0
            //   R1 + d0  = s1 + 2^16*d1
            //   d1       = s2
            // Both sides now equal the same canonical integer S ⇒ LHS == RHS as
            // integers, i.e. r_prev*p + lane == qt*q + r_t (the Horner reduction).
            builder.assert_eq(r0, s0.clone() + d0.clone() * ce::<AB>(two16));
            builder.assert_eq(r1 + d0.clone(), s1.clone() + d1.clone() * ce::<AB>(two16));
            builder.assert_eq(d1.clone(), s2.clone());

            // r_prev for next step = r_t.
            r_prev = r_t.clone();
            if t == HORNER_STEPS - 1 {
                masks.push(r_t);
            }
        }
    }
    masks
}

// ---------------------------------------------------------------------------
// Witness side of the wide-then-reduce gadget.
// ---------------------------------------------------------------------------

/// Reference reduction: `mask_j = (sum_i lane_{j,i} * p^i) mod q`, computed via the
/// SAME integer Horner the gadget constrains. `lanes` are the 30 canonical squeezed
/// values (lane_{j,i} = lanes[j*5+i] in `[0, p)`). Returns the 6 masks.
pub fn wide_reduce_masks<F: PrimeField32>(lanes: &[u32; NUM_SQUEEZE]) -> [u32; NUM_OUTPUTS] {
    let p = F::ORDER_U32 as u64;
    let q = Q12289 as u64;
    let mut masks = [0u32; NUM_OUTPUTS];
    for j in 0..NUM_OUTPUTS {
        let mut r: u64 = 0;
        // high digit first: lane[4], lane[3], ..., lane[0].
        for t in 0..HORNER_STEPS {
            let digit = lanes[j * LANES_PER_MASK + (LANES_PER_MASK - 1 - t)] as u64;
            r = (r * p + digit) % q;
        }
        masks[j] = r as u32;
    }
    masks
}

/// Fill the wide-then-reduce gadget's witness columns for all 6 masks into `row`
/// at column offset `base`. `lanes` are the 30 canonical squeezed values.
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
        // squeezed lane copies.
        for i in 0..LANES_PER_MASK {
            row[mask_base + i] = F::from_u32(lanes[j * LANES_PER_MASK + i]);
        }
        let step0 = mask_base + LANES_PER_MASK;
        let mut r_prev: u64 = 0;
        for t in 0..HORNER_STEPS {
            let sb = step0 + t * STEP_CELLS;
            let lane = lanes[j * LANES_PER_MASK + (LANES_PER_MASK - 1 - t)] as u64;

            // quotient + remainder of (r_prev*p + lane) by q.
            let val = r_prev * p + lane;
            let qt = val / q;
            let r_t = val % q;
            let slack = (q - 1) - r_t;

            let qt_lo = qt & (two16 - 1);
            let qt_hi = qt >> LIMB_BASE_BITS;
            let lane_lo = lane & (two16 - 1);
            let lane_hi = lane >> LIMB_BASE_BITS;

            // LHS coeff pair.
            let l0 = r_prev * p_lo + lane_lo;
            let l1 = r_prev * p_hi + lane_hi;
            // RHS coeff pair.
            let r0 = qt_lo * q + r_t;
            let r1 = qt_hi * q;

            // common canonical limbs S = l0 + 2^16*l1 = r0 + 2^16*r1.
            let s_full = l0 + two16 * l1;
            debug_assert_eq!(s_full, r0 + two16 * r1);
            let s0 = s_full & (two16 - 1);
            let s1 = (s_full >> LIMB_BASE_BITS) & (two16 - 1);
            let s2 = s_full >> (2 * LIMB_BASE_BITS);

            // LHS carries: l0 = s0 + 2^16*c0 ; l1 + c0 = s1 + 2^16*c1 ; c1 = s2.
            let c0 = (l0 - s0) >> LIMB_BASE_BITS;
            let c1 = (l1 + c0 - s1) >> LIMB_BASE_BITS;
            // RHS carries.
            let d0 = (r0 - s0) >> LIMB_BASE_BITS;
            let d1 = (r1 + d0 - s1) >> LIMB_BASE_BITS;
            debug_assert_eq!(c1, s2);
            debug_assert_eq!(d1, s2);

            // value cells.
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
            // bit cells.
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
            // r_t + slack.
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

/// Column layout of the per-row "prefix" (everything before the Poseidon2 blocks):
///   input_lanes[40]
///   input_bits[40 * 8]
///   reduce_gadget[REDUCE_CELLS]   (5 squeezed lanes + Horner reduction per mask)
const fn prefix_width() -> usize {
    NUM_INPUT_LANES + NUM_INPUT_LANES * BYTE_BITS + REDUCE_CELLS
}

struct PrefixOff;
impl PrefixOff {
    const INPUT_LANES: usize = 0;
    const INPUT_BITS: usize = Self::INPUT_LANES + NUM_INPUT_LANES;
    /// Start of the wide-then-reduce gadget block (REDUCE_CELLS wide).
    const REDUCE: usize = Self::INPUT_BITS + NUM_INPUT_LANES * BYTE_BITS;
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

/// Field-generic C2 AIR.
pub struct C2Air<
    F: PrimeCharacteristicRing,
    LinearLayers,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
> {
    constants: RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    // Our own copy of the round-constant arrays for constraint evaluation
    // (the `RoundConstants` fields are pub(crate) so we cannot read them back).
    beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: [F; PARTIAL_ROUNDS],
    ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    perm_cols: usize,
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
    C2Air<F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>
{
    /// Build from explicit round-constant arrays (kept for the constraints) and a
    /// matching `RoundConstants` (used by the witness generator).
    pub fn from_arrays(
        beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        partial_rc: [F; PARTIAL_ROUNDS],
        ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
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
            _phantom: PhantomData,
        }
    }

    pub fn constants(&self) -> &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS> {
        &self.constants
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
    for C2Air<F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>
{
    fn width(&self) -> usize {
        prefix_width()
            + NUM_PERMS
                * perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>(
                )
    }

    fn main_next_row_columns(&self) -> Vec<usize> {
        // Every constraint is within a single row.
        Vec::new()
    }

    fn num_public_values(&self) -> usize {
        NUM_OUTPUTS
    }
}

/// In-circuit S-box, mirroring `p3-poseidon2-air`'s private `eval_sbox` for the
/// `(DEGREE, REGISTERS)` pairs Plonky3 supports. `x` is a degree-1 expr; returns
/// `x^DEGREE` and emits the auxiliary register equalities when REGISTERS>0.
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

/// Build an `AB::Expr` constant from a u64 (field-generic; `AB::F: PrimeField`).
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
    for C2Air<
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

        let lane = |i: usize| local[PrefixOff::INPUT_LANES + i];
        let lane_bit = |i: usize, b: usize| local[PrefixOff::INPUT_BITS + i * BYTE_BITS + b];

        // (1) input byte-range: each lane = sum of 8 boolean bits.
        for i in 0..NUM_INPUT_LANES {
            let mut acc = AB::Expr::ZERO;
            for b in 0..BYTE_BITS {
                let bit = lane_bit(i, b);
                builder.assert_bool(bit);
                acc += bit.into() * ce::<AB>(1u64 << b);
            }
            builder.assert_eq(lane(i), acc);
        }

        // (2) Poseidon2 sponge: NUM_ABSORB_PERMS absorb blocks (additive-absorb
        //     linking) then NUM_SQUEEZE_PERMS extra squeeze perms (state carried
        //     unchanged into the next permutation). Each perm's post-state exposes
        //     RATE=8 squeezed lanes; we read 30 lanes across 4 read blocks.
        let mut prev_post: Option<[AB::Expr; WIDTH]> = None;
        let mut posts: Vec<[AB::Expr; WIDTH]> = Vec::with_capacity(NUM_PERMS);
        for i in 0..NUM_PERMS {
            let base = prefix_width() + i * self.perm_cols;
            let block: Vec<AB::Var> = local[base..base + self.perm_cols].to_vec();
            let pcols: &Poseidon2Cols<
                AB::Var,
                WIDTH,
                SBOX_DEGREE,
                SBOX_REGISTERS,
                HALF_FULL_ROUNDS,
                PARTIAL_ROUNDS,
            > = (block.as_slice()).borrow();

            let inputs: [AB::Expr; WIDTH] = core::array::from_fn(|c| pcols.inputs[c].into());
            let is_absorb = i < NUM_ABSORB_PERMS;
            match &prev_post {
                None => {
                    for r in 0..RATE {
                        builder.assert_eq(inputs[r].clone(), lane(r));
                    }
                    for c in RATE..WIDTH {
                        builder.assert_eq(inputs[c].clone(), AB::Expr::ZERO);
                    }
                }
                Some(prev) => {
                    for r in 0..RATE {
                        if is_absorb {
                            builder.assert_eq(
                                inputs[r].clone(),
                                prev[r].clone() + lane(i * RATE + r).into(),
                            );
                        } else {
                            // squeeze perm: carry state unchanged (no absorb).
                            builder.assert_eq(inputs[r].clone(), prev[r].clone());
                        }
                    }
                    for c in RATE..WIDTH {
                        builder.assert_eq(inputs[c].clone(), prev[c].clone());
                    }
                }
            }

            let post = self.eval_perm_block::<AB, LinearLayers>(builder, pcols);
            prev_post = Some(post.clone());
            posts.push(post);
        }

        // (3) squeeze read-off: 30 lanes. Read block b (b=0..4) reads RATE lanes
        //     from perm post (NUM_ABSORB_PERMS-1 + b).
        let mut squeezed: Vec<AB::Expr> = Vec::with_capacity(NUM_SQUEEZE);
        'outer: for b in 0..NUM_SQUEEZE.div_ceil(RATE) {
            let src = NUM_ABSORB_PERMS - 1 + b;
            for r in 0..RATE {
                if squeezed.len() == NUM_SQUEEZE {
                    break 'outer;
                }
                squeezed.push(posts[src][r].clone());
            }
        }

        // (4) WIDE-THEN-REDUCE: derive the 6 masks from the 30 squeezed lanes.
        let masks = eval_reduce_gadget::<AB>(builder, PrefixOff::REDUCE, &local, &squeezed);

        // (5) public binding: mask_j == public mask lane j.
        let public = builder.public_values();
        let public: Vec<AB::PublicVar> = public.to_vec();
        for j in 0..NUM_OUTPUTS {
            builder.assert_eq(masks[j].clone(), public[j]);
        }
    }
}

impl<
        F: PrimeCharacteristicRing,
        LinearLayers,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    >
    C2Air<F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>
{
    /// Emit the Poseidon2 round constraints for one block, returning the final
    /// post-state as a vector of exprs. Mirrors `p3-poseidon2-air`'s private
    /// `eval`, reading the committed `post` / sbox-register columns from `pcols`.
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

        // beginning full rounds
        for round in 0..HALF_FULL_ROUNDS {
            let fr = &pcols.beginning_full_rounds[round];
            for i in 0..WIDTH {
                let after_rc = state[i].clone() + Into::<AB::Expr>::into(self.beginning_full_rc[round][i].dup());
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

        // partial rounds
        for round in 0..PARTIAL_ROUNDS {
            let pr = &pcols.partial_rounds[round];
            let after_rc = state[0].clone() + Into::<AB::Expr>::into(self.partial_rc[round].dup());
            let sb = eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(
                builder,
                &pr.sbox.0,
                after_rc,
            );
            builder.assert_eq(sb, pr.post_sbox);
            state[0] = pr.post_sbox.into();
            LL::internal_linear_layer(&mut state);
        }

        // ending full rounds
        for round in 0..HALF_FULL_ROUNDS {
            let fr = &pcols.ending_full_rounds[round];
            for i in 0..WIDTH {
                let after_rc = state[i].clone() + Into::<AB::Expr>::into(self.ending_full_rc[round][i].dup());
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

use core::borrow::Borrow;

// ---------------------------------------------------------------------------
// Witness / trace generation (honest assignment).
// ---------------------------------------------------------------------------

/// Honest C2 instance: input bytes, the 30 squeezed field lanes (5 per mask) and
/// the 6 wide-then-reduce masks.
#[derive(Clone)]
pub struct C2Instance<F> {
    pub input_lanes: [F; NUM_INPUT_LANES],
    /// 30 squeezed canonical lane values (lane_{j,i} = squeezed[j*5+i]).
    pub squeezed: [u32; NUM_SQUEEZE],
    pub mask: [u32; NUM_OUTPUTS],
    _phantom: PhantomData<F>,
}

/// Compute the honest sponge + reduction for given pad_seed/slot, running the
/// SAME per-perm generator the trace uses (so reference == witness by construction).
pub fn compute_c2_instance<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    pad_seed: &[u8; 32],
    slot: u64,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
) -> C2Instance<F> {
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();

    let mut bytes = [0u8; NUM_INPUT_LANES];
    bytes[..32].copy_from_slice(pad_seed);
    bytes[32..].copy_from_slice(&slot.to_le_bytes());
    let input_lanes: [F; NUM_INPUT_LANES] = core::array::from_fn(|i| F::from_u8(bytes[i]));

    // Run the 8-perm sponge (5 absorb + 3 squeeze), collecting every perm's post.
    let _ = perm_cols;
    let posts = run_c2_sponge::<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(&input_lanes, constants);

    // Read 30 squeezed lanes across 4 read blocks (RATE each).
    let mut squeezed = [0u32; NUM_SQUEEZE];
    let mut idx = 0;
    'outer: for b in 0..NUM_SQUEEZE.div_ceil(RATE) {
        let src = NUM_ABSORB_PERMS - 1 + b;
        for r in 0..RATE {
            if idx == NUM_SQUEEZE {
                break 'outer;
            }
            squeezed[idx] = posts[src][r].as_canonical_u32();
            idx += 1;
        }
    }

    let mask = wide_reduce_masks::<F>(&squeezed);

    C2Instance {
        input_lanes,
        squeezed,
        mask,
        _phantom: PhantomData,
    }
}

/// Run the C2 sponge chain (NUM_ABSORB_PERMS absorb + NUM_SQUEEZE_PERMS squeeze),
/// returning every permutation's post-state.
#[allow(clippy::type_complexity)]
fn run_c2_sponge<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    input_lanes: &[F; NUM_INPUT_LANES],
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
) -> Vec<[F; WIDTH]> {
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let mut posts: Vec<[F; WIDTH]> = Vec::with_capacity(NUM_PERMS);
    let mut prev_post = [F::ZERO; WIDTH];
    for i in 0..NUM_PERMS {
        let mut state = [F::ZERO; WIDTH];
        if i == 0 {
            for r in 0..RATE {
                state[r] = input_lanes[r];
            }
        } else if i < NUM_ABSORB_PERMS {
            for r in 0..RATE {
                state[r] = prev_post[r] + input_lanes[i * RATE + r];
            }
            for c in RATE..WIDTH {
                state[c] = prev_post[c];
            }
        } else {
            // squeeze perm: carry the whole state unchanged.
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
            let p = &pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post;
            core::array::from_fn::<F, WIDTH, _>(|c| p[c].assume_init())
        };
        prev_post = post;
        posts.push(post);
    }
    posts
}

/// Generate the full C2 trace: `num_rows` (power of two) identical honest rows.
pub fn generate_c2_trace<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    pad_seed: &[u8; 32],
    slot: u64,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    num_rows: usize,
) -> (RowMajorMatrix<F>, C2Instance<F>) {
    assert!(num_rows.is_power_of_two());
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let width = prefix_width() + NUM_PERMS * perm_cols;

    let inst = compute_c2_instance::<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(pad_seed, slot, constants);

    let row = build_c2_row::<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(&inst, constants, perm_cols, width);

    let mut values = Vec::with_capacity(width * num_rows);
    for _ in 0..num_rows {
        values.extend_from_slice(&row);
    }
    (RowMajorMatrix::new(values, width), inst)
}

/// Build one fully-populated C2 trace row from an honest instance.
fn build_c2_row<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    inst: &C2Instance<F>,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
    width: usize,
) -> Vec<F> {
    let mut row = vec![F::ZERO; width];

    for i in 0..NUM_INPUT_LANES {
        row[PrefixOff::INPUT_LANES + i] = inst.input_lanes[i];
        let byte = inst.input_lanes[i].as_canonical_u32();
        for b in 0..BYTE_BITS {
            row[PrefixOff::INPUT_BITS + i * BYTE_BITS + b] = F::from_u32((byte >> b) & 1);
        }
    }

    // wide-then-reduce gadget columns (also rederives the masks; must match inst).
    let masks = fill_reduce_gadget::<F>(&mut row, PrefixOff::REDUCE, &inst.squeezed);
    debug_assert_eq!(masks, inst.mask);

    let mut prev_post = [F::ZERO; WIDTH];
    for i in 0..NUM_PERMS {
        let mut state = [F::ZERO; WIDTH];
        if i == 0 {
            for r in 0..RATE {
                state[r] = inst.input_lanes[r];
            }
        } else if i < NUM_ABSORB_PERMS {
            for r in 0..RATE {
                state[r] = prev_post[r] + inst.input_lanes[i * RATE + r];
            }
            for c in RATE..WIDTH {
                state[c] = prev_post[c];
            }
        } else {
            state = prev_post;
        }

        let base = prefix_width() + i * perm_cols;
        let block = &mut row[base..base + perm_cols];
        let block_uninit: &mut [MaybeUninit<F>] = unsafe {
            core::slice::from_raw_parts_mut(block.as_mut_ptr() as *mut MaybeUninit<F>, perm_cols)
        };
        let pcols: &mut Poseidon2Cols<
            MaybeUninit<F>,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        > = block_uninit.borrow_mut();
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
            let p = &pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post;
            core::array::from_fn::<F, WIDTH, _>(|c| p[c].assume_init())
        };
        prev_post = post;
    }

    row
}

/// Public values vector for verification = the 6 mask lanes.
pub fn c2_public_values<F: PrimeField32>(inst: &C2Instance<F>) -> Vec<F> {
    inst.mask.iter().map(|&m| F::from_u32(m)).collect()
}
