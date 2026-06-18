//! Framework-agnostic AIR for the projection-mask sponge PRF
//! `projection_masks_poseidon2(pad_seed, slot_id)` over Fq = 12289.
//!
//! This proves, in-circuit, that 6 committed mask lanes ARE the Poseidon2-sponge
//! PRF outputs of a HIDDEN `pad_seed` (32 bytes) and `slot_id` (u64 little-endian
//! bytes). It is obligation **C2** of an account-creation STARK relation.
//!
//! CORRECTNESS and HONEST labeling are the priorities, not speed.
//!
//! This module does NOT reimplement the permutation or its constraints. It
//! CHAINS the single-permutation AIR (`crate::poseidon2_air_q12289`) — one
//! sub-trace per sponge permutation — and adds the sponge LINKING constraints
//! (capacity init, absorb-add, squeeze read-off) plus BYTE-RANGE constraints on
//! the input lanes. The per-permutation transition/boundary logic is REUSED
//! verbatim from `poseidon2_air_q12289`.
//!
//! =============================================================================
//! PERMUTATION COUNT (derived from the reference sponge, NOT guessed)
//! =============================================================================
//!
//! `projection_masks_poseidon2` builds 40 input byte-lanes (32 pad_seed + 8
//! slot_id LE) and calls `poseidon2_sponge(domain, inputs, n_out=6)`. The sponge
//! (see `poseidon2_q12289::poseidon2_sponge`) does:
//!
//!   * 10* padding in the Fq domain: push a single `1`, then `0`s until the
//!     element count is a multiple of RATE = 5.
//!       elems = 40 (inputs) + 1 (pad marker) = 41, padded up to 45 (next
//!       multiple of 5)  =>  45 / 5 = 9 absorb blocks.
//!     Each absorb block is added into the first RATE lanes and is followed by
//!     ONE permutation  =>  9 ABSORB permutations.
//!
//!   * Squeeze n_out = 6: the loop reads min(remaining, RATE) lanes WITHOUT a
//!     leading permutation, then permutes only if more lanes remain.
//!       iter 1: take 5 (lanes 0..5 of the post-absorb state), remaining = 1,
//!               permute  =>  1 squeeze permutation.
//!       iter 2: take 1 (lane 0 of the post-squeeze-permute state), remaining 0.
//!     =>  1 SQUEEZE permutation.
//!
//!   TOTAL PERMUTATIONS = 9 absorb + 1 squeeze = 10.
//!
//! These are computed from the actual sponge code at module load
//! (`derive_block_plan`) and asserted, so a change to RATE / input length / n_out
//! re-derives the count rather than relying on the literal 10.
//!
//! =============================================================================
//! TRACE LAYOUT
//! =============================================================================
//!
//!   * `subs[p]` for p in 0..N_PERMS (=10): the full single-permutation AIR
//!     sub-trace (70 rows x 24 cols) for the p-th permutation. Its row 0 is that
//!     permutation's INPUT state, its last row is its OUTPUT state.
//!
//!   * `input_lanes[40]`: the byte-lane-encoded hidden witness (pad_seed[0..32]
//!     then slot_id LE bytes[0..8]). Each is a field element that MUST be a real
//!     byte in [0, 256).
//!
//!   * `input_bits[40][8]`: an 8-bit little-endian decomposition of each input
//!     lane, used for the byte-range constraint.
//!
//!   * `mask_outputs[6]`: the COMMITTED mask lanes (public/committed inputs of
//!     the relation), boundary-bound to the squeezed state cells.
//!
//! =============================================================================
//! CONSTRAINTS (all degree <= 5)
//! =============================================================================
//!
//! 1. PER-PERMUTATION (reused, degree <= 5): for every sub-trace `subs[p]` we
//!    emit the full single-permutation AIR residual set
//!    (`poseidon2_air_q12289::all_residuals`) binding row 0 -> ... -> last row to
//!    the actual permutation. The boundary input/output of each sub-trace is its
//!    OWN row 0 / last row (the linking constraints below tie those across
//!    permutations).
//!
//! 2. CAPACITY INIT (boundary, degree 1): sub-trace 0's input row equals the
//!    constant initial sponge state `init_capacity(domain)` (rate lanes 0,
//!    capacity lanes pinned to the public domain-derived constants) PLUS the
//!    first absorb block (folded into the absorb-link rule below for p=0). We
//!    pin the capacity lanes (indices RATE..T) of sub-trace 0's input to the
//!    public constants directly; the rate lanes are pinned via the absorb-link.
//!
//! 3. ABSORB LINK (transition between permutations, degree 1): for the input of
//!    each absorb permutation p (0..9), the absorbed block B_p is added into the
//!    first RATE lanes of the PREVIOUS permutation's output (or, for p = 0, of
//!    the bare initial state which is 0 in the rate lanes):
//!       subs[p].input[i]  = prev_out[i] + block_p[i]   for i <  RATE
//!       subs[p].input[i]  = prev_out[i]                for i >= RATE
//!    where prev_out = subs[p-1].output() for p >= 1, and for p = 0
//!    prev_out = init_capacity(domain) (rate lanes 0). The block elements
//!    block_p[i] are the padded input stream chunked by RATE; each non-padding
//!    element is one of the `input_lanes` witnesses, the pad marker `1` and the
//!    `0` padding are PUBLIC CONSTANTS pinned in the constraint.
//!
//! 4. SQUEEZE LINK (transition between permutations, degree 1): the squeeze
//!    permutations are pure permutations of the running state, so for each
//!    squeeze permutation p (here just p = 9) the input equals the previous
//!    output unchanged:
//!       subs[p].input[i] = subs[p-1].output()[i]   for all i.
//!
//! 5. BYTE RANGE (degree 2): each input lane equals its 8-bit decomposition and
//!    every bit is boolean:
//!       bit_b * (bit_b - 1) = 0                      (degree 2)
//!       sum_b bit_b * 2^b   - input_lane = 0         (degree 1)
//!    This forces input_lane in [0, 256), i.e. a genuine byte.
//!
//! 6. MASK OUTPUT (boundary, degree 1): the 6 committed mask outputs equal the
//!    squeezed state cells:
//!       mask_outputs[j] = subs[8].output()[j]   for j in 0..5   (first squeeze)
//!       mask_outputs[5] = subs[9].output()[0]                   (second squeeze)
//!
//! MAX DEGREE = 5 (the S-box inside the reused per-permutation AIR). Every
//! linking/range constraint is degree <= 2.

use crate::poseidon2_air_q12289::{
    self as perm_air, generate_trace as gen_perm_trace, Trace as PermTrace,
};
use crate::poseidon2_q12289::{
    fq_add, fq_from_u32, fq_mul, fq_sub, init_capacity, Poseidon2Params, CAPACITY,
    POSEIDON2_PROJECTION_MASK_DOMAIN, PROJECTION_MASK_LANES, RATE, T,
};

// -----------------------------------------------------------------------------
// Static PRF shape constants (derived, not guessed)
// -----------------------------------------------------------------------------

/// Number of pad_seed byte-lanes.
pub const PAD_SEED_LANES: usize = 32;
/// Number of slot_id byte-lanes (u64, little-endian).
pub const SLOT_ID_LANES: usize = 8;
/// Total hidden input byte-lanes.
pub const N_INPUT_LANES: usize = PAD_SEED_LANES + SLOT_ID_LANES; // 40
/// Bits per byte-lane decomposition.
pub const BITS_PER_LANE: usize = 8;
/// Number of committed mask output lanes (= PROJECTION_MASK_LANES = 6).
pub const N_MASK_OUTPUTS: usize = PROJECTION_MASK_LANES;

/// A single absorb block, padded into RATE lanes.  Each entry is either a
/// reference to an `input_lanes` witness (`InputLane(idx)`) or a PUBLIC constant
/// (the 10* pad marker `1` or a `0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockElem {
    /// The j-th element absorbed is hidden witness `input_lanes[idx]`.
    InputLane(usize),
    /// A fixed public constant (pad marker 1, or 0 padding).
    Const(u32),
}

/// What a permutation does in the sponge: absorb a padded block, or squeeze.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermRole {
    /// Absorb permutation: `block` is added into the first RATE lanes of the
    /// running state, then the permutation runs.
    Absorb { block: [BlockElem; RATE] },
    /// Squeeze permutation: state passes through the permutation unchanged on
    /// input (no block added); outputs are read off the boundary states.
    Squeeze,
}

/// The full, deterministic permutation plan for the projection-mask PRF.
pub struct PrfPlan {
    /// One role per permutation, in order.
    pub roles: Vec<PermRole>,
}

/// Number of absorb permutations the PRF performs (derived).
pub fn n_absorb_perms() -> usize {
    derive_block_plan().0
}
/// Number of squeeze permutations the PRF performs (derived).
pub fn n_squeeze_perms() -> usize {
    derive_block_plan().1
}
/// Total permutations the PRF performs (derived).
pub fn n_perms() -> usize {
    let (a, s) = derive_block_plan();
    a + s
}

/// Derive (n_absorb, n_squeeze) EXACTLY as the reference sponge would, from
/// N_INPUT_LANES, RATE and N_MASK_OUTPUTS. This mirrors
/// `poseidon2_q12289::poseidon2_sponge` step for step (10* padding + squeeze).
fn derive_block_plan() -> (usize, usize) {
    // 10* padding: inputs + 1 pad marker, rounded up to a multiple of RATE.
    let mut n_elems = N_INPUT_LANES + 1;
    while n_elems % RATE != 0 {
        n_elems += 1;
    }
    let n_absorb = n_elems / RATE;

    // Squeeze: read min(remaining, RATE), permute only if more remain.
    let mut remaining = N_MASK_OUTPUTS;
    let mut n_squeeze = 0usize;
    loop {
        let take = remaining.min(RATE);
        remaining -= take;
        if remaining == 0 {
            break;
        }
        n_squeeze += 1; // a permutation happens between squeeze reads
    }
    (n_absorb, n_squeeze)
}

/// Build the deterministic plan: which `input_lanes`/constants go into each
/// absorb block, and which permutations are squeezes. The block layout mirrors
/// the reference's padded field-element stream chunked by RATE.
pub fn build_plan() -> PrfPlan {
    let (n_absorb, n_squeeze) = derive_block_plan();

    // The padded element stream as BlockElem references:
    //   input_lanes[0..40], then pad marker Const(1), then Const(0) padding.
    let mut stream: Vec<BlockElem> = Vec::new();
    for i in 0..N_INPUT_LANES {
        stream.push(BlockElem::InputLane(i));
    }
    stream.push(BlockElem::Const(1)); // 10* pad marker
    while stream.len() % RATE != 0 {
        stream.push(BlockElem::Const(0));
    }
    assert_eq!(stream.len(), n_absorb * RATE);

    let mut roles: Vec<PermRole> = Vec::with_capacity(n_absorb + n_squeeze);
    for chunk in stream.chunks(RATE) {
        let mut block = [BlockElem::Const(0); RATE];
        block.copy_from_slice(chunk);
        roles.push(PermRole::Absorb { block });
    }
    for _ in 0..n_squeeze {
        roles.push(PermRole::Squeeze);
    }
    PrfPlan { roles }
}

// -----------------------------------------------------------------------------
// Trace
// -----------------------------------------------------------------------------

/// The sponge-PRF execution trace.
#[derive(Clone, Debug)]
pub struct Trace {
    /// One single-permutation AIR sub-trace per permutation (length = n_perms()).
    pub subs: Vec<PermTrace>,
    /// The 40 hidden byte-lane input witnesses (pad_seed[0..32] then slot LE).
    pub input_lanes: [u32; N_INPUT_LANES],
    /// 8-bit little-endian decomposition of each input lane.
    pub input_bits: [[u32; BITS_PER_LANE]; N_INPUT_LANES],
    /// The 6 committed mask outputs (claimed = squeezed state cells).
    pub mask_outputs: [u32; N_MASK_OUTPUTS],
}

impl Trace {
    /// Output state of permutation `p`.
    fn perm_output(&self, p: usize) -> [u32; T] {
        self.subs[p].output()
    }
    /// Input state of permutation `p`.
    fn perm_input(&self, p: usize) -> [u32; T] {
        self.subs[p].input()
    }
}

/// Encode the 40 input byte-lanes exactly as the reference PRF does.
fn encode_input_lanes(pad_seed: &[u8; 32], slot_id: u64) -> [u32; N_INPUT_LANES] {
    let mut lanes = [0u32; N_INPUT_LANES];
    for (i, &b) in pad_seed.iter().enumerate() {
        lanes[i] = b as u32;
    }
    for (i, &b) in slot_id.to_le_bytes().iter().enumerate() {
        lanes[PAD_SEED_LANES + i] = b as u32;
    }
    lanes
}

/// 8-bit little-endian decomposition of a value in [0, 256).
fn byte_bits(v: u32) -> [u32; BITS_PER_LANE] {
    let mut bits = [0u32; BITS_PER_LANE];
    for (b, slot) in bits.iter_mut().enumerate() {
        *slot = (v >> b) & 1;
    }
    bits
}

/// Resolve a block element to its concrete field value, given the input lanes.
fn block_elem_value(elem: BlockElem, input_lanes: &[u32; N_INPUT_LANES]) -> u32 {
    match elem {
        BlockElem::InputLane(idx) => fq_from_u32(input_lanes[idx]),
        BlockElem::Const(c) => fq_from_u32(c),
    }
}

/// HONEST trace generator: runs the REAL PRF (its sponge state evolution) and
/// records each permutation as a single-permutation AIR sub-trace, plus all
/// linking witnesses. By construction the recorded states are exactly those the
/// reference sponge visits.
pub fn generate_prf_trace(pad_seed: &[u8; 32], slot_id: u64) -> Trace {
    let params = Poseidon2Params::new();
    let plan = build_plan();
    let input_lanes = encode_input_lanes(pad_seed, slot_id);

    let mut input_bits = [[0u32; BITS_PER_LANE]; N_INPUT_LANES];
    for i in 0..N_INPUT_LANES {
        input_bits[i] = byte_bits(input_lanes[i]);
    }

    // Initial sponge state from the (public, fixed) domain.
    let mut state = init_capacity(POSEIDON2_PROJECTION_MASK_DOMAIN);

    let mut subs: Vec<PermTrace> = Vec::with_capacity(plan.roles.len());
    for role in &plan.roles {
        // Apply the linking operation that produces THIS permutation's input.
        match role {
            PermRole::Absorb { block } => {
                for i in 0..RATE {
                    let v = block_elem_value(block[i], &input_lanes);
                    state[i] = fq_add(state[i], v);
                }
            }
            PermRole::Squeeze => { /* state passes through unchanged */ }
        }
        // Record the permutation as a single-permutation sub-trace; its last row
        // is the post-permutation state.
        let sub = gen_perm_trace(&params, &state);
        state = sub.output();
        subs.push(sub);
    }

    // Squeeze read-off: first squeeze reads RATE lanes from the LAST absorb
    // permutation's output; subsequent lanes from each squeeze permutation's
    // output lane 0.. .  We mirror the reference squeeze loop exactly to fill 6.
    let (n_absorb, _n_squeeze) = derive_block_plan();
    let mut mask_outputs = [0u32; N_MASK_OUTPUTS];
    let mut filled = 0usize;
    // The state to read FROM at squeeze step s: output of permutation
    // (n_absorb - 1 + s).
    let mut read_perm = n_absorb - 1;
    let mut remaining = N_MASK_OUTPUTS;
    loop {
        let take = remaining.min(RATE);
        let src = subs[read_perm].output();
        for i in 0..take {
            mask_outputs[filled] = src[i];
            filled += 1;
        }
        remaining -= take;
        if remaining == 0 {
            break;
        }
        read_perm += 1; // next squeeze permutation
    }

    Trace {
        subs,
        input_lanes,
        input_bits,
        mask_outputs,
    }
}

// -----------------------------------------------------------------------------
// Constraint residuals
// -----------------------------------------------------------------------------

/// A scalar constraint residual, tagged for auditing. 0 == satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Residual {
    pub kind: ResidualKind,
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualKind {
    /// A residual delegated from the per-permutation AIR for permutation `perm`.
    Perm { perm: usize, index: usize },
    /// Capacity-init boundary: capacity lane `lane` of permutation 0's input
    /// equals the public domain constant.
    CapacityInit { lane: usize },
    /// Absorb link for permutation `perm`, lane `lane`.
    AbsorbLink { perm: usize, lane: usize },
    /// Squeeze link for permutation `perm`, lane `lane`.
    SqueezeLink { perm: usize, lane: usize },
    /// Byte-range: bit `bit` of input lane `lane` is boolean.
    BitBoolean { lane: usize, bit: usize },
    /// Byte-range: input lane `lane` equals its bit recomposition.
    BitRecompose { lane: usize },
    /// Mask-output boundary: committed mask `out` equals the squeezed cell.
    MaskOutput { out: usize },
}

/// Evaluate EVERY constraint residual for `trace` against the committed
/// `mask_outputs` (the public/committed inputs of the C2 relation). An all-zero
/// result means the trace satisfies the entire C2 AIR: the 6 committed masks ARE
/// the PRF outputs of the byte-range-checked hidden input.
///
/// `committed_masks` is what a verifier holds; we boundary-bind the trace's
/// squeezed cells to it.
pub fn all_residuals(
    params: &Poseidon2Params,
    trace: &Trace,
    committed_masks: &[u32; N_MASK_OUTPUTS],
) -> Vec<Residual> {
    let plan = build_plan();
    let n_perms = plan.roles.len();
    let (n_absorb, _) = derive_block_plan();
    let mut out: Vec<Residual> = Vec::new();

    // 1. Per-permutation AIR residuals (reused verbatim). Each sub-trace's own
    //    row 0 / last row are its boundary input/output; the linking rules below
    //    pin those across permutations.
    for (p, sub) in trace.subs.iter().enumerate() {
        let inp = sub.input();
        let outp = sub.output();
        for (idx, r) in perm_air::all_residuals(params, sub, &inp, &outp)
            .iter()
            .enumerate()
        {
            out.push(Residual {
                kind: ResidualKind::Perm { perm: p, index: idx },
                value: r.value,
            });
        }
    }

    // 2. Capacity init (boundary): permutation 0's input capacity lanes equal the
    //    public domain-derived constants. (Rate lanes of perm 0's input are
    //    pinned by the absorb link for p = 0.)
    let init_state = init_capacity(POSEIDON2_PROJECTION_MASK_DOMAIN);
    let p0_in = trace.perm_input(0);
    for lane in RATE..T {
        let residual = fq_sub(p0_in[lane], init_state[lane]);
        out.push(Residual {
            kind: ResidualKind::CapacityInit { lane: lane - RATE },
            value: residual,
        });
    }

    // 3 & 4. Linking between permutations.
    for (p, role) in plan.roles.iter().enumerate() {
        // prev_out is the running state BEFORE this permutation's linking op:
        //   p == 0  -> the bare initial sponge state (capacity from domain,
        //              rate lanes 0)
        //   p >= 1  -> previous permutation's output
        let prev_out: [u32; T] = if p == 0 {
            init_state
        } else {
            trace.perm_output(p - 1)
        };
        let cur_in = trace.perm_input(p);
        match role {
            PermRole::Absorb { block } => {
                for lane in 0..T {
                    let expected = if lane < RATE {
                        let b = block_elem_value(block[lane], &trace.input_lanes);
                        fq_add(prev_out[lane], b)
                    } else {
                        prev_out[lane]
                    };
                    let residual = fq_sub(cur_in[lane], expected);
                    out.push(Residual {
                        kind: ResidualKind::AbsorbLink { perm: p, lane },
                        value: residual,
                    });
                }
            }
            PermRole::Squeeze => {
                for lane in 0..T {
                    let residual = fq_sub(cur_in[lane], prev_out[lane]);
                    out.push(Residual {
                        kind: ResidualKind::SqueezeLink { perm: p, lane },
                        value: residual,
                    });
                }
            }
        }
    }
    debug_assert_eq!(n_perms, trace.subs.len());

    // 5. Byte-range constraints on each input lane.
    for lane in 0..N_INPUT_LANES {
        let bits = &trace.input_bits[lane];
        // each bit boolean: b*(b-1) = 0
        for (bit, &b) in bits.iter().enumerate() {
            let residual = fq_mul(b, fq_sub(b, 1));
            out.push(Residual {
                kind: ResidualKind::BitBoolean { lane, bit },
                value: residual,
            });
        }
        // recomposition: sum_b b * 2^b == input_lane
        let mut acc = 0u32;
        let mut weight = 1u32;
        for &b in bits.iter() {
            acc = fq_add(acc, fq_mul(b, weight));
            weight = fq_mul(weight, 2);
        }
        let residual = fq_sub(acc, trace.input_lanes[lane]);
        out.push(Residual {
            kind: ResidualKind::BitRecompose { lane },
            value: residual,
        });
    }

    // 6. Mask output boundary: committed masks == squeezed cells.
    //    First squeeze reads lanes 0..take from perm (n_absorb-1); subsequent
    //    squeeze permutations contribute lane 0.. of their output.
    let mut filled = 0usize;
    let mut read_perm = n_absorb - 1;
    let mut remaining = N_MASK_OUTPUTS;
    loop {
        let take = remaining.min(RATE);
        let src = trace.perm_output(read_perm);
        for i in 0..take {
            let residual = fq_sub(committed_masks[filled], src[i]);
            out.push(Residual {
                kind: ResidualKind::MaskOutput { out: filled },
                value: residual,
            });
            filled += 1;
        }
        remaining -= take;
        if remaining == 0 {
            break;
        }
        read_perm += 1;
    }

    out
}

/// Maximum residual magnitude (0 == all constraints satisfied).
pub fn max_residual(
    params: &Poseidon2Params,
    trace: &Trace,
    committed_masks: &[u32; N_MASK_OUTPUTS],
) -> u32 {
    all_residuals(params, trace, committed_masks)
        .iter()
        .map(|r| r.value)
        .max()
        .unwrap_or(0)
}

/// The claimed 6 mask outputs of the AIR (what the relation commits to).
pub fn claimed_masks(trace: &Trace) -> [u32; N_MASK_OUTPUTS] {
    trace.mask_outputs
}

/// Total scalar constraint count emitted by `all_residuals`, derived from the
/// plan and the reused per-permutation AIR size.
pub fn n_constraints() -> usize {
    let (n_absorb, n_squeeze) = derive_block_plan();
    let n_perms = n_absorb + n_squeeze;
    let per_perm = perm_air::N_CONSTRAINTS; // 1704
    let cap_init = CAPACITY; // T - RATE
    let links = n_perms * T; // one vector (T lanes) per permutation's link
    let byte_range = N_INPUT_LANES * (BITS_PER_LANE + 1); // bools + recompose
    let mask = N_MASK_OUTPUTS;
    n_perms * per_perm + cap_init + links + byte_range + mask
}

/// Maximum polynomial degree of any constraint. The S-box `x^5` inside the
/// reused per-permutation AIR dominates; all linking/range constraints are
/// degree <= 2 (bit-boolean is degree 2). So the bound is 5.
pub const MAX_DEGREE: u32 = 5;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_q12289::{projection_masks_poseidon2, Q};

    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    fn random_pad(seed: &mut u64) -> [u8; 32] {
        let mut p = [0u8; 32];
        for b in p.iter_mut() {
            *b = (xorshift(seed) & 0xff) as u8;
        }
        p
    }

    /// Document the derived permutation count and why.
    #[test]
    fn poseidon2_prf_air_permutation_count_is_derived() {
        let (n_absorb, n_squeeze) = derive_block_plan();
        // 40 inputs + 1 pad marker = 41 -> padded to 45 -> 9 absorb blocks.
        assert_eq!(n_absorb, 9, "expected 9 absorb permutations");
        // n_out = 6: read 5, permute, read 1 -> 1 squeeze permutation.
        assert_eq!(n_squeeze, 1, "expected 1 squeeze permutation");
        assert_eq!(n_perms(), 10);
        assert_eq!(build_plan().roles.len(), 10);
        // first 9 roles absorb, last is squeeze
        let plan = build_plan();
        for (p, role) in plan.roles.iter().enumerate() {
            if p < 9 {
                assert!(matches!(role, PermRole::Absorb { .. }), "perm {p} not absorb");
            } else {
                assert!(matches!(role, PermRole::Squeeze), "perm {p} not squeeze");
            }
        }
    }

    /// Report trace size, constraint count, max degree.
    #[test]
    fn poseidon2_prf_air_sizes_report() {
        let pad = [3u8; 32];
        let trace = generate_prf_trace(&pad, 42);
        assert_eq!(trace.subs.len(), 10);
        for sub in &trace.subs {
            assert_eq!(sub.rows.len(), perm_air::N_ROWS); // 70
        }
        // total trace cells = 10 perms * 70 rows * 24 cols + input lanes + bits
        let perm_cells = 10 * perm_air::N_ROWS * T;
        let aux_cells = N_INPUT_LANES + N_INPUT_LANES * BITS_PER_LANE + N_MASK_OUTPUTS;
        println!(
            "PRF AIR: perms=10 perm_cells={perm_cells} aux_cells={aux_cells} \
             constraints={} max_degree={}",
            n_constraints(),
            MAX_DEGREE
        );
        // sanity: constraint count matches the actual residual vector length.
        let params = Poseidon2Params::new();
        let masks = claimed_masks(&trace);
        let residuals = all_residuals(&params, &trace, &masks);
        assert_eq!(residuals.len(), n_constraints(), "constraint count mismatch");
        assert_eq!(MAX_DEGREE, 5);
    }

    /// KEY CORRECTNESS ANCHOR: the AIR's claimed 6 mask outputs equal the
    /// reference PRF for many random (pad_seed, slot).
    #[test]
    fn poseidon2_prf_air_output_matches_reference_prf() {
        let mut seed = 0xC0FF_EE00_1234_5678u64;
        for _ in 0..32 {
            let pad = random_pad(&mut seed);
            let slot = xorshift(&mut seed);
            let trace = generate_prf_trace(&pad, slot);
            let reference = projection_masks_poseidon2(&pad, slot);
            let claimed = claimed_masks(&trace);
            for j in 0..N_MASK_OUTPUTS {
                assert_eq!(
                    claimed[j] as u16, reference[j],
                    "AIR mask lane {j} != reference PRF (pad first byte {}, slot {slot})",
                    pad[0]
                );
            }
            for &m in claimed.iter() {
                assert!(m < Q, "mask not canonical");
            }
        }
    }

    /// Honest trace satisfies ALL constraints (every residual 0).
    #[test]
    fn poseidon2_prf_air_honest_trace_satisfies_all_constraints() {
        let params = Poseidon2Params::new();
        let mut seed = 0x0BAD_1DEA_F00D_BABEu64;
        for _ in 0..16 {
            let pad = random_pad(&mut seed);
            let slot = xorshift(&mut seed);
            let trace = generate_prf_trace(&pad, slot);
            let masks = claimed_masks(&trace);
            let residuals = all_residuals(&params, &trace, &masks);
            assert_eq!(residuals.len(), n_constraints());
            for r in &residuals {
                assert_eq!(r.value, 0, "honest trace violated a constraint: {r:?}");
            }
            assert_eq!(max_residual(&params, &trace, &masks), 0);
        }
    }

    /// MUTATION: mutating a pad_seed byte-lane (and its bit decomposition kept
    /// consistent) changes the PRF output, so the absorb-link or downstream
    /// permutation residual fires; if bits are NOT updated, the recompose fires.
    #[test]
    fn poseidon2_prf_air_mutate_input_lane_breaks() {
        let params = Poseidon2Params::new();
        let pad = [9u8; 32];
        let trace = generate_prf_trace(&pad, 7);
        let masks = claimed_masks(&trace);

        // (a) mutate ONLY input_lanes[5] (bits stale): recompose must fire.
        let mut t = trace.clone();
        t.input_lanes[5] = fq_add(t.input_lanes[5], 1);
        let res = all_residuals(&params, &t, &masks);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::BitRecompose { lane: 5 })),
            "stale-bits lane mutation not caught by recompose"
        );

        // (b) mutate the absorb-link input cell directly (perm 0 input lane 5):
        //     the absorb link OR the per-permutation AIR boundary fires.
        let mut t2 = trace.clone();
        t2.subs[0].rows[0][5] = fq_add(t2.subs[0].rows[0][5], 1);
        let res2 = all_residuals(&params, &t2, &masks);
        assert!(
            res2.iter().any(|r| r.value != 0),
            "absorb-link input cell mutation not caught"
        );
    }

    /// MUTATION: mutating an intermediate permutation cell fires the reused
    /// per-permutation AIR.
    #[test]
    fn poseidon2_prf_air_mutate_intermediate_perm_cell_breaks() {
        let params = Poseidon2Params::new();
        let pad = [11u8; 32];
        let trace = generate_prf_trace(&pad, 99);
        let masks = claimed_masks(&trace);

        let mut t = trace.clone();
        // a mid-permutation, mid-trace cell: perm 4, row 30, lane 9.
        t.subs[4].rows[30][9] = fq_add(t.subs[4].rows[30][9], 1);
        let res = all_residuals(&params, &t, &masks);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::Perm { perm: 4, .. })),
            "intermediate permutation cell mutation not caught by per-perm AIR"
        );
    }

    /// MUTATION: mutating an absorb-link cell (the running state at a later
    /// permutation's input) without a matching previous output breaks the link.
    #[test]
    fn poseidon2_prf_air_mutate_absorb_link_breaks() {
        let params = Poseidon2Params::new();
        let pad = [13u8; 32];
        let trace = generate_prf_trace(&pad, 5);
        let masks = claimed_masks(&trace);

        // mutate perm 3's INPUT row lane 2 (an absorb permutation): the absorb
        // link for perm 3 must fire (prev_out + block no longer matches).
        let mut t = trace.clone();
        t.subs[3].rows[0][2] = fq_add(t.subs[3].rows[0][2], 1);
        let res = all_residuals(&params, &t, &masks);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::AbsorbLink { perm: 3, lane: 2 })),
            "absorb-link cell mutation not caught by the link constraint"
        );

        // mutate the SQUEEZE permutation's input (perm 9) lane 1: squeeze link fires.
        let mut t2 = trace.clone();
        t2.subs[9].rows[0][1] = fq_add(t2.subs[9].rows[0][1], 1);
        let res2 = all_residuals(&params, &t2, &masks);
        assert!(
            res2.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::SqueezeLink { perm: 9, lane: 1 })),
            "squeeze-link cell mutation not caught"
        );
    }

    /// MUTATION: mutating a claimed mask output breaks the mask boundary.
    #[test]
    fn poseidon2_prf_air_mutate_claimed_mask_breaks() {
        let params = Poseidon2Params::new();
        let pad = [17u8; 32];
        let trace = generate_prf_trace(&pad, 3);
        let mut masks = claimed_masks(&trace);
        // first squeeze lane
        masks[2] = fq_add(masks[2], 1);
        let res = all_residuals(&params, &trace, &masks);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::MaskOutput { out: 2 })),
            "claimed mask mutation (first squeeze) not caught"
        );
        // second-squeeze lane (mask 5)
        let mut masks2 = claimed_masks(&trace);
        masks2[5] = fq_add(masks2[5], 1);
        let res2 = all_residuals(&params, &trace, &masks2);
        assert!(
            res2.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::MaskOutput { out: 5 })),
            "claimed mask mutation (second squeeze) not caught"
        );
    }

    /// BYTE RANGE: an out-of-range byte (>= 256) must be rejected by the
    /// byte-range constraint. We set an input lane to 256 and give it the
    /// consistent 8-bit decomposition of its low byte (all zero); recompose
    /// then mismatches (256 != 0). We ALSO test a bit set to a non-boolean.
    #[test]
    fn poseidon2_prf_air_out_of_range_byte_rejected() {
        let params = Poseidon2Params::new();
        let pad = [21u8; 32];
        let trace = generate_prf_trace(&pad, 1);
        let masks = claimed_masks(&trace);

        // (a) lane value 256 with 8 zero bits: recompose (sum=0) != 256.
        let mut t = trace.clone();
        t.input_lanes[0] = 256;
        t.input_bits[0] = [0u32; BITS_PER_LANE];
        let res = all_residuals(&params, &t, &masks);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::BitRecompose { lane: 0 })),
            "out-of-range byte (256, zero bits) not rejected by recompose"
        );

        // (b) a 9th-bit-style cheat: try to represent 256 by making a bit = 2
        //     (non-boolean). bit-boolean must fire.
        let mut t2 = trace.clone();
        t2.input_lanes[0] = 256;
        // value 256 has no 8-bit rep; attacker sets bit 0 = 256 to satisfy
        // recompose, which the boolean check must reject.
        t2.input_bits[0] = [0u32; BITS_PER_LANE];
        t2.input_bits[0][0] = 256;
        let res2 = all_residuals(&params, &t2, &masks);
        assert!(
            res2.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::BitBoolean { lane: 0, bit: 0 })),
            "non-boolean bit (256) not rejected by bit-boolean constraint"
        );
    }
}
