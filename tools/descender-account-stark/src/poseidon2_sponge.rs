//! Shared in-AIR Poseidon2 sponge primitives (free-function form).
//!
//! This is a FAITHFUL free-function copy of the account-creation leaf's in-AIR Poseidon2 sponge
//! (`air.rs`: `eval_sponge` / `eval_perm_block` / `absorb_elem_expr` / `run_sponge` / `build_elems`),
//! extracted so the Phase-2 SPEND leaf (and the recursion co-verification) can build their OWN
//! sponges WITHOUT touching the byte-pinned creation AIR (whose frozen KAT must not drift). The
//! creation AIR keeps its own copy; this module is validated independently against a native recompute
//! (`run_sponge`) — `eval_sponge` (constraints) and `run_sponge` (witness) are mirror constructions.
//!
//! Layout of one sponge block, at base offset `off` (mirrors `air.rs` `SpongeLayout`, no reduction):
//!   * `[off, off + n_input)`                                  — input lanes
//!   * `[off + n_input, off + n_input + n_byte*8)`             — byte-decomposition bits
//!   * `[off + perms_off, ..)`                                 — `n_perms` Poseidon2 perm blocks
//! where `perms_off = n_input + n_byte*8` and `n_byte` = number of byte-ranged input lanes.

use core::borrow::{Borrow, BorrowMut};
use core::mem::MaybeUninit;
use p3_air::AirBuilder;
use p3_field::{PrimeCharacteristicRing, PrimeField, PrimeField32};
use p3_poseidon2::GenericPoseidon2LinearLayers;
use p3_poseidon2_air::{generate_trace_rows_for_perm, Poseidon2Cols, RoundConstants};

use crate::reduce_gadget::{ce, RATE};

extern crate alloc;
use alloc::vec::Vec;

const BYTE_BITS: usize = 8;

/// A static description of one sponge: input lanes (each byte-ranged or not) and squeezed outputs.
/// Mirror of `air.rs::SpongePlan`.
#[derive(Clone)]
pub struct SpongeSpec {
    pub n_input: usize,
    pub byte_ranged: Vec<bool>,
    pub n_out: usize,
    pub n_absorb: usize,
    pub n_perms: usize,
}

impl SpongeSpec {
    pub fn new(byte_ranged: Vec<bool>, n_out: usize) -> Self {
        let n_input = byte_ranged.len();
        let padded = {
            let mut len = n_input + 1;
            while len % RATE != 0 {
                len += 1;
            }
            len
        };
        let n_absorb = padded / RATE;
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
        SpongeSpec {
            n_input,
            byte_ranged,
            n_out,
            n_absorb,
            n_perms: n_absorb + n_squeeze,
        }
    }

    pub fn padded_len(&self) -> usize {
        self.n_absorb * RATE
    }

    pub fn n_byte(&self) -> usize {
        self.byte_ranged.iter().filter(|&&b| b).count()
    }

    /// Column offset (relative to the block base) of the first perm block.
    pub fn perms_off(&self) -> usize {
        self.n_input + self.n_byte() * BYTE_BITS
    }

    /// Total column width of this sponge block.
    pub fn width(&self, perm_cols: usize) -> usize {
        self.perms_off() + self.n_perms * perm_cols
    }

    pub fn output_source(&self, out: usize) -> (usize, usize) {
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

/// Poseidon2 s-box, free-function copy of `air.rs::eval_sbox_expr`.
pub fn eval_sbox<AB: AirBuilder, const DEGREE: u64, const REGISTERS: usize>(
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

/// One Poseidon2 permutation as constraints, free-function copy of `air.rs::eval_perm_block`.
#[allow(clippy::too_many_arguments)]
pub fn eval_perm_block<
    AB,
    LL,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    builder: &mut AB,
    pcols: &Poseidon2Cols<AB::Var, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    beginning_full_rc: &[[AB::F; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: &[AB::F; PARTIAL_ROUNDS],
    ending_full_rc: &[[AB::F; WIDTH]; HALF_FULL_ROUNDS],
) -> [AB::Expr; WIDTH]
where
    AB: AirBuilder,
    AB::F: PrimeField,
    LL: GenericPoseidon2LinearLayers<WIDTH>,
{
    let mut state: [AB::Expr; WIDTH] = core::array::from_fn(|i| pcols.inputs[i].into());
    LL::external_linear_layer(&mut state);

    for round in 0..HALF_FULL_ROUNDS {
        let fr = &pcols.beginning_full_rounds[round];
        for i in 0..WIDTH {
            let after_rc = state[i].clone() + Into::<AB::Expr>::into(beginning_full_rc[round][i]);
            let sb = eval_sbox::<AB, SBOX_DEGREE, SBOX_REGISTERS>(builder, &fr.sbox[i].0, after_rc);
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
        let after_rc = state[0].clone() + Into::<AB::Expr>::into(partial_rc[round]);
        let sb = eval_sbox::<AB, SBOX_DEGREE, SBOX_REGISTERS>(builder, &pr.sbox.0, after_rc);
        builder.assert_eq(sb, pr.post_sbox);
        state[0] = pr.post_sbox.into();
        LL::internal_linear_layer(&mut state);
    }

    for round in 0..HALF_FULL_ROUNDS {
        let fr = &pcols.ending_full_rounds[round];
        for i in 0..WIDTH {
            let after_rc = state[i].clone() + Into::<AB::Expr>::into(ending_full_rc[round][i]);
            let sb = eval_sbox::<AB, SBOX_DEGREE, SBOX_REGISTERS>(builder, &fr.sbox[i].0, after_rc);
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

fn absorb_elem_expr<AB: AirBuilder>(
    local: &[AB::Var],
    off: usize,
    spec: &SpongeSpec,
    pos: usize,
) -> AB::Expr {
    if pos < spec.n_input {
        local[off + pos].into()
    } else if pos == spec.n_input {
        AB::Expr::ONE
    } else {
        AB::Expr::ZERO
    }
}

/// The sponge as constraints, free-function copy of `air.rs::eval_sponge`. Byte-ranged input lanes
/// are 8-bit range-checked and bound to their value; the sponge absorbs all input lanes + a padding
/// `1`, then squeezes `spec.n_out` outputs. Returns the squeezed output exprs.
#[allow(clippy::too_many_arguments)]
pub fn eval_sponge<
    AB,
    LL,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    builder: &mut AB,
    local: &[AB::Var],
    off: usize,
    perm_cols: usize,
    spec: &SpongeSpec,
    beginning_full_rc: &[[AB::F; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: &[AB::F; PARTIAL_ROUNDS],
    ending_full_rc: &[[AB::F; WIDTH]; HALF_FULL_ROUNDS],
) -> Vec<AB::Expr>
where
    AB: AirBuilder,
    AB::F: PrimeField,
    LL: GenericPoseidon2LinearLayers<WIDTH>,
{
    // Byte-decompose each byte-ranged input lane and bind it to its value.
    let lane_bits_off = spec.n_input;
    let mut bit_cursor = 0usize;
    for i in 0..spec.n_input {
        if spec.byte_ranged[i] {
            let mut acc = AB::Expr::ZERO;
            for b in 0..BYTE_BITS {
                let bit = local[off + lane_bits_off + bit_cursor * BYTE_BITS + b];
                builder.assert_bool(bit);
                acc += bit.into() * ce::<AB>(1u64 << b);
            }
            builder.assert_eq(local[off + i].into(), acc);
            bit_cursor += 1;
        }
    }

    let perms_off = spec.perms_off();
    let padded = spec.padded_len();
    let mut prev_post: Option<[AB::Expr; WIDTH]> = None;
    for p in 0..spec.n_perms {
        let base = off + perms_off + p * perm_cols;
        let block: Vec<AB::Var> = local[base..base + perm_cols].to_vec();
        let pcols: &Poseidon2Cols<
            AB::Var,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        > = (block.as_slice()).borrow();

        let inputs: [AB::Expr; WIDTH] = core::array::from_fn(|c| pcols.inputs[c].into());
        let is_absorb = p < spec.n_absorb;
        match &prev_post {
            None => {
                for r in 0..RATE {
                    let elem = absorb_elem_expr::<AB>(local, off, spec, r);
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
                            absorb_elem_expr::<AB>(local, off, spec, pos)
                        } else {
                            AB::Expr::ZERO
                        };
                        builder.assert_eq(inputs[r].clone(), prev[r].clone() + elem);
                    } else {
                        builder.assert_eq(inputs[r].clone(), prev[r].clone());
                    }
                }
                for c in RATE..WIDTH {
                    builder.assert_eq(inputs[c].clone(), prev[c].clone());
                }
            }
        }

        let post = eval_perm_block::<AB, LL, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>(
            builder,
            pcols,
            beginning_full_rc,
            partial_rc,
            ending_full_rc,
        );
        prev_post = Some(post);
    }

    let mut outs = vec![AB::Expr::ZERO; spec.n_out];
    for o in 0..spec.n_out {
        let (sp, lane_idx) = spec.output_source(o);
        let base = off + perms_off + sp * perm_cols;
        let block: Vec<AB::Var> = local[base..base + perm_cols].to_vec();
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

/// Build the padded absorb-element vector (input data ‖ padding `1` ‖ zeros). Copy of
/// `air.rs::build_elems`.
pub fn build_elems<F: PrimeField32>(data: &[u32], spec: &SpongeSpec) -> Vec<F> {
    let mut elems = vec![F::ZERO; spec.padded_len()];
    for (i, &v) in data.iter().enumerate() {
        elems[i] = F::from_u32(v);
    }
    elems[spec.n_input] = F::ONE;
    elems
}

/// Run the sponge natively, filling each perm block's columns. Copy of `air.rs::run_sponge`.
/// Returns `(per-perm filled column blocks, final post-state, squeezed outputs)`.
#[allow(clippy::type_complexity)]
pub fn run_sponge<
    F: PrimeField32,
    LL: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    elems: &[F],
    spec: &SpongeSpec,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
) -> (Vec<Vec<F>>, [F; WIDTH], Vec<F>) {
    let mut perm_blocks: Vec<Vec<F>> = Vec::with_capacity(spec.n_perms);
    let mut posts: Vec<[F; WIDTH]> = Vec::with_capacity(spec.n_perms);
    let mut prev_post = [F::ZERO; WIDTH];
    for p in 0..spec.n_perms {
        let mut state = [F::ZERO; WIDTH];
        if p == 0 {
            for r in 0..RATE {
                state[r] = elems[r];
            }
        } else if p < spec.n_absorb {
            for r in 0..RATE {
                let pos = p * RATE + r;
                let add = if pos < spec.padded_len() { elems[pos] } else { F::ZERO };
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
        generate_trace_rows_for_perm::<F, LL, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>(
            pcols, state, constants,
        );
        let post = unsafe {
            let pp = &pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post;
            core::array::from_fn::<F, WIDTH, _>(|c| pp[c].assume_init())
        };
        let packed: Vec<F> = unsafe { block.iter().map(|m| m.assume_init()).collect() };
        perm_blocks.push(packed);
        posts.push(post);
        prev_post = post;
    }

    let mut outputs = vec![F::ZERO; spec.n_out];
    for o in 0..spec.n_out {
        let (sp, lane) = spec.output_source(o);
        outputs[o] = posts[sp][lane];
    }
    (perm_blocks, prev_post, outputs)
}
