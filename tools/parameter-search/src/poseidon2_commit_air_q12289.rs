//! Framework-agnostic AIR for the pad-source commitment sponge
//! `pad_source_commitment_poseidon2(pad_seed, blinding_nonce)` over Fq = 12289.
//!
//! This proves, in-circuit, that 20 committed commitment lanes ARE the
//! Poseidon2-sponge outputs of a HIDDEN `pad_seed` (32 bytes) and a HIDDEN
//! `blinding_nonce` (16 bytes). It is obligation **C1** (pad-source binding) of
//! an account-creation STARK relation, replacing the SHA `pad_seed_digest`
//! scaffold.
//!
//! CORRECTNESS and HONEST labeling are the priorities, not speed.
//!
//! This module does NOT reimplement the permutation or its constraints. Exactly
//! like the C2 PRF AIR (`crate::poseidon2_prf_air_q12289`) it CHAINS the
//! single-permutation AIR (`crate::poseidon2_air_q12289`) — one sub-trace per
//! sponge permutation — and adds the sponge LINKING constraints (capacity init,
//! absorb-add, squeeze read-off) plus BYTE-RANGE constraints on every input
//! lane. The per-permutation transition/boundary logic is REUSED verbatim.
//!
//! =============================================================================
//! PERMUTATION COUNT (derived from the reference sponge, NOT guessed)
//! =============================================================================
//!
//! `pad_source_commitment_poseidon2` builds 48 input byte-lanes (32 pad_seed +
//! 16 blinding_nonce) and calls `poseidon2_sponge(domain, inputs, n_out=20)`:
//!
//!   * 10* padding: 48 inputs + 1 pad marker = 49, padded up to 50 (next
//!     multiple of RATE = 5)  =>  50 / 5 = 10 absorb permutations.
//!   * Squeeze n_out = 20: read 5 (remaining 15), permute; read 5 (remaining 10),
//!     permute; read 5 (remaining 5), permute; read 5 (remaining 0)  =>  3
//!     squeeze permutations (a permutation after each non-final read of RATE).
//!
//!   TOTAL PERMUTATIONS = 10 absorb + 3 squeeze = 13.
//!
//! These are derived from the reference sponge shape at module load
//! (`derive_block_plan`) and asserted, so a change to RATE / input length /
//! n_out re-derives the count rather than relying on the literal 13.
//!
//! =============================================================================
//! TRACE LAYOUT  (identical shape to the C2 PRF AIR)
//! =============================================================================
//!
//!   * `subs[p]` for p in 0..N_PERMS (=13): the full single-permutation AIR
//!     sub-trace (70 rows x 24 cols) for the p-th permutation.
//!   * `input_lanes[48]`: the byte-lane hidden witness (pad_seed[0..32] then
//!     blinding_nonce[0..16]). Each is a field element in [0, 256).
//!   * `input_bits[48][8]`: 8-bit little-endian decomposition for byte-range.
//!   * `commit_outputs[20]`: the COMMITTED commitment lanes, boundary-bound to
//!     the squeezed state cells.
//!
//! CONSTRAINTS are the same FAMILY as C2 (per-permutation degree<=5; all
//! linking/range constraints degree <= 2). MAX DEGREE = 5 (the S-box).

use crate::poseidon2_air_q12289::{
    self as perm_air, Trace as PermTrace, generate_trace as gen_perm_trace,
};
use crate::poseidon2_q12289::{
    CAPACITY, PAD_SOURCE_BLINDING_BYTES, PAD_SOURCE_COMMITMENT_LANES,
    POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN, Poseidon2Params, RATE, T, fq_add, fq_from_u32, fq_mul,
    fq_sub, init_capacity,
};

// -----------------------------------------------------------------------------
// Static shape constants (derived, not guessed)
// -----------------------------------------------------------------------------

/// Number of pad_seed byte-lanes.
pub const PAD_SEED_LANES: usize = 32;
/// Number of blinding-nonce byte-lanes.
pub const NONCE_LANES: usize = PAD_SOURCE_BLINDING_BYTES; // 16
/// Total hidden input byte-lanes.
pub const N_INPUT_LANES: usize = PAD_SEED_LANES + NONCE_LANES; // 48
/// Bits per byte-lane decomposition.
pub const BITS_PER_LANE: usize = 8;
/// Number of committed commitment output lanes (= 10).
pub const N_COMMIT_OUTPUTS: usize = PAD_SOURCE_COMMITMENT_LANES;

/// A single absorb block, padded into RATE lanes. Each entry is either a
/// reference to an `input_lanes` witness (`InputLane(idx)`) or a PUBLIC constant
/// (the 10* pad marker `1` or a `0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockElem {
    InputLane(usize),
    Const(u32),
}

/// What a permutation does in the sponge: absorb a padded block, or squeeze.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermRole {
    Absorb { block: [BlockElem; RATE] },
    Squeeze,
}

/// The full, deterministic permutation plan for the pad-source commitment.
pub struct CommitPlan {
    pub roles: Vec<PermRole>,
}

pub fn n_absorb_perms() -> usize {
    derive_block_plan().0
}
pub fn n_squeeze_perms() -> usize {
    derive_block_plan().1
}
pub fn n_perms() -> usize {
    let (a, s) = derive_block_plan();
    a + s
}

/// Derive (n_absorb, n_squeeze) EXACTLY as the reference sponge would, from
/// N_INPUT_LANES, RATE and N_COMMIT_OUTPUTS. Mirrors
/// `poseidon2_q12289::poseidon2_sponge` step for step (10* padding + squeeze).
fn derive_block_plan() -> (usize, usize) {
    let mut n_elems = N_INPUT_LANES + 1; // 10* padding: inputs + 1 pad marker
    while n_elems % RATE != 0 {
        n_elems += 1;
    }
    let n_absorb = n_elems / RATE;

    let mut remaining = N_COMMIT_OUTPUTS;
    let mut n_squeeze = 0usize;
    loop {
        let take = remaining.min(RATE);
        remaining -= take;
        if remaining == 0 {
            break;
        }
        n_squeeze += 1;
    }
    (n_absorb, n_squeeze)
}

/// Build the deterministic plan, mirroring the reference's padded field-element
/// stream chunked by RATE.
pub fn build_plan() -> CommitPlan {
    let (n_absorb, n_squeeze) = derive_block_plan();

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
    CommitPlan { roles }
}

// -----------------------------------------------------------------------------
// Trace
// -----------------------------------------------------------------------------

/// The pad-source commitment execution trace.
#[derive(Clone, Debug)]
pub struct Trace {
    /// One single-permutation AIR sub-trace per permutation (length = n_perms()).
    pub subs: Vec<PermTrace>,
    /// The 48 hidden byte-lane inputs (pad_seed[0..32] then nonce[0..16]).
    pub input_lanes: [u32; N_INPUT_LANES],
    /// 8-bit little-endian decomposition of each input lane.
    pub input_bits: [[u32; BITS_PER_LANE]; N_INPUT_LANES],
    /// The 20 committed commitment outputs (claimed = squeezed state cells).
    pub commit_outputs: [u32; N_COMMIT_OUTPUTS],
}

impl Trace {
    fn perm_output(&self, p: usize) -> [u32; T] {
        self.subs[p].output()
    }
    fn perm_input(&self, p: usize) -> [u32; T] {
        self.subs[p].input()
    }

    /// The 32 pad_seed byte-lane cells (the SEAM cells C2 shares with C1).
    /// These ARE the first 32 entries of `input_lanes`.
    pub fn pad_seed_lanes(&self) -> &[u32] {
        &self.input_lanes[0..PAD_SEED_LANES]
    }
}

/// Encode the 48 input byte-lanes exactly as the reference commitment does.
fn encode_input_lanes(pad_seed: &[u8; 32], nonce: &[u8; NONCE_LANES]) -> [u32; N_INPUT_LANES] {
    let mut lanes = [0u32; N_INPUT_LANES];
    for (i, &b) in pad_seed.iter().enumerate() {
        lanes[i] = b as u32;
    }
    for (i, &b) in nonce.iter().enumerate() {
        lanes[PAD_SEED_LANES + i] = b as u32;
    }
    lanes
}

fn byte_bits(v: u32) -> [u32; BITS_PER_LANE] {
    let mut bits = [0u32; BITS_PER_LANE];
    for (b, slot) in bits.iter_mut().enumerate() {
        *slot = (v >> b) & 1;
    }
    bits
}

fn block_elem_value(elem: BlockElem, input_lanes: &[u32; N_INPUT_LANES]) -> u32 {
    match elem {
        BlockElem::InputLane(idx) => fq_from_u32(input_lanes[idx]),
        BlockElem::Const(c) => fq_from_u32(c),
    }
}

/// HONEST trace generator: runs the REAL sponge state evolution for the
/// commitment and records each permutation as a single-permutation AIR
/// sub-trace, plus all linking witnesses. By construction the recorded states
/// are exactly those the reference sponge visits.
pub fn generate_commit_trace(pad_seed: &[u8; 32], nonce: &[u8; NONCE_LANES]) -> Trace {
    let params = Poseidon2Params::new();
    let plan = build_plan();
    let input_lanes = encode_input_lanes(pad_seed, nonce);

    let mut input_bits = [[0u32; BITS_PER_LANE]; N_INPUT_LANES];
    for i in 0..N_INPUT_LANES {
        input_bits[i] = byte_bits(input_lanes[i]);
    }

    let mut state = init_capacity(POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN);

    let mut subs: Vec<PermTrace> = Vec::with_capacity(plan.roles.len());
    for role in &plan.roles {
        match role {
            PermRole::Absorb { block } => {
                for i in 0..RATE {
                    let v = block_elem_value(block[i], &input_lanes);
                    state[i] = fq_add(state[i], v);
                }
            }
            PermRole::Squeeze => { /* state passes through unchanged */ }
        }
        let sub = gen_perm_trace(&params, &state);
        state = sub.output();
        subs.push(sub);
    }

    // Squeeze read-off: mirror the reference squeeze loop exactly to fill 20.
    let (n_absorb, _n_squeeze) = derive_block_plan();
    let mut commit_outputs = [0u32; N_COMMIT_OUTPUTS];
    let mut filled = 0usize;
    let mut read_perm = n_absorb - 1;
    let mut remaining = N_COMMIT_OUTPUTS;
    loop {
        let take = remaining.min(RATE);
        let src = subs[read_perm].output();
        for i in 0..take {
            commit_outputs[filled] = src[i];
            filled += 1;
        }
        remaining -= take;
        if remaining == 0 {
            break;
        }
        read_perm += 1;
    }

    Trace {
        subs,
        input_lanes,
        input_bits,
        commit_outputs,
    }
}

// -----------------------------------------------------------------------------
// Constraint residuals
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Residual {
    pub kind: ResidualKind,
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualKind {
    Perm { perm: usize, index: usize },
    CapacityInit { lane: usize },
    AbsorbLink { perm: usize, lane: usize },
    SqueezeLink { perm: usize, lane: usize },
    BitBoolean { lane: usize, bit: usize },
    BitRecompose { lane: usize },
    CommitOutput { out: usize },
}

/// Evaluate EVERY constraint residual for `trace` against the committed
/// `commit_outputs`. All-zero == the 10 committed lanes ARE the commitment of
/// the byte-range-checked hidden (pad_seed, blinding_nonce).
pub fn all_residuals(
    params: &Poseidon2Params,
    trace: &Trace,
    committed: &[u32; N_COMMIT_OUTPUTS],
) -> Vec<Residual> {
    let plan = build_plan();
    let n_perms = plan.roles.len();
    let (n_absorb, _) = derive_block_plan();
    let mut out: Vec<Residual> = Vec::new();

    // 1. Per-permutation AIR residuals (reused verbatim).
    for (p, sub) in trace.subs.iter().enumerate() {
        let inp = sub.input();
        let outp = sub.output();
        for (idx, r) in perm_air::all_residuals(params, sub, &inp, &outp)
            .iter()
            .enumerate()
        {
            out.push(Residual {
                kind: ResidualKind::Perm {
                    perm: p,
                    index: idx,
                },
                value: r.value,
            });
        }
    }

    // 2. Capacity init (boundary).
    let init_state = init_capacity(POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN);
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
        for (bit, &b) in bits.iter().enumerate() {
            let residual = fq_mul(b, fq_sub(b, 1));
            out.push(Residual {
                kind: ResidualKind::BitBoolean { lane, bit },
                value: residual,
            });
        }
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

    // 6. Commitment output boundary: committed lanes == squeezed cells.
    let mut filled = 0usize;
    let mut read_perm = n_absorb - 1;
    let mut remaining = N_COMMIT_OUTPUTS;
    loop {
        let take = remaining.min(RATE);
        let src = trace.perm_output(read_perm);
        for i in 0..take {
            let residual = fq_sub(committed[filled], src[i]);
            out.push(Residual {
                kind: ResidualKind::CommitOutput { out: filled },
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

pub fn max_residual(
    params: &Poseidon2Params,
    trace: &Trace,
    committed: &[u32; N_COMMIT_OUTPUTS],
) -> u32 {
    all_residuals(params, trace, committed)
        .iter()
        .map(|r| r.value)
        .max()
        .unwrap_or(0)
}

/// The claimed 10 commitment outputs of the AIR (what the relation commits to).
pub fn claimed_commitment(trace: &Trace) -> [u32; N_COMMIT_OUTPUTS] {
    trace.commit_outputs
}

/// Total scalar constraint count emitted by `all_residuals`.
pub fn n_constraints() -> usize {
    let (n_absorb, n_squeeze) = derive_block_plan();
    let n_perms = n_absorb + n_squeeze;
    let per_perm = perm_air::N_CONSTRAINTS;
    let cap_init = CAPACITY;
    let links = n_perms * T;
    let byte_range = N_INPUT_LANES * (BITS_PER_LANE + 1);
    let commit = N_COMMIT_OUTPUTS;
    n_perms * per_perm + cap_init + links + byte_range + commit
}

/// Maximum polynomial degree of any constraint. The S-box `x^5` inside the
/// reused per-permutation AIR dominates; all linking/range constraints are
/// degree <= 2.
pub const MAX_DEGREE: u32 = 5;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_q12289::{Q, pad_source_commitment_poseidon2};

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

    fn random_nonce(seed: &mut u64) -> [u8; NONCE_LANES] {
        let mut n = [0u8; NONCE_LANES];
        for b in n.iter_mut() {
            *b = (xorshift(seed) & 0xff) as u8;
        }
        n
    }

    /// Document the derived permutation count.
    #[test]
    fn poseidon2_commit_air_permutation_count_is_derived() {
        let (n_absorb, n_squeeze) = derive_block_plan();
        // 48 inputs + 1 pad marker = 49 -> padded to 50 -> 10 absorb blocks.
        assert_eq!(n_absorb, 10, "expected 10 absorb permutations");
        // n_out = 20: 4 reads of 5 with a permute after each non-final read ->
        // 3 squeeze permutations.
        assert_eq!(n_squeeze, 3, "expected 3 squeeze permutations");
        assert_eq!(n_perms(), 13);
        let plan = build_plan();
        assert_eq!(plan.roles.len(), 13);
        for (p, role) in plan.roles.iter().enumerate() {
            if p < 10 {
                assert!(
                    matches!(role, PermRole::Absorb { .. }),
                    "perm {p} not absorb"
                );
            } else {
                assert!(matches!(role, PermRole::Squeeze), "perm {p} not squeeze");
            }
        }
    }

    /// Report sizes.
    #[test]
    fn poseidon2_commit_air_sizes_report() {
        let pad = [3u8; 32];
        let nonce = [5u8; NONCE_LANES];
        let trace = generate_commit_trace(&pad, &nonce);
        assert_eq!(trace.subs.len(), 13);
        for sub in &trace.subs {
            assert_eq!(sub.rows.len(), perm_air::N_ROWS);
        }
        let perm_cells = 13 * perm_air::N_ROWS * T;
        let aux_cells = N_INPUT_LANES + N_INPUT_LANES * BITS_PER_LANE + N_COMMIT_OUTPUTS;
        println!(
            "C1 COMMIT AIR: perms=13 perm_cells={perm_cells} aux_cells={aux_cells} \
             constraints={} max_degree={}",
            n_constraints(),
            MAX_DEGREE
        );
        let params = Poseidon2Params::new();
        let committed = claimed_commitment(&trace);
        let residuals = all_residuals(&params, &trace, &committed);
        assert_eq!(
            residuals.len(),
            n_constraints(),
            "constraint count mismatch"
        );
        assert_eq!(MAX_DEGREE, 5);
    }

    /// KEY CORRECTNESS ANCHOR: AIR commitment output == reference function over
    /// many random (pad_seed, nonce); and the honest trace has residual 0.
    #[test]
    fn poseidon2_commit_air_output_matches_reference() {
        let params = Poseidon2Params::new();
        let mut seed = 0xC0FF_EE11_2233_4455u64;
        for _ in 0..32 {
            let pad = random_pad(&mut seed);
            let nonce = random_nonce(&mut seed);
            let trace = generate_commit_trace(&pad, &nonce);
            let reference = pad_source_commitment_poseidon2(&pad, &nonce);
            let claimed = claimed_commitment(&trace);
            for j in 0..N_COMMIT_OUTPUTS {
                assert_eq!(
                    claimed[j] as u16, reference[j],
                    "AIR commit lane {j} != reference (pad first byte {})",
                    pad[0]
                );
                assert!(claimed[j] < Q, "commit not canonical");
            }
            // honest trace residual 0
            let res = all_residuals(&params, &trace, &claimed);
            assert_eq!(res.len(), n_constraints());
            for r in &res {
                assert_eq!(r.value, 0, "honest C1 trace violated {r:?}");
            }
            assert_eq!(max_residual(&params, &trace, &claimed), 0);
        }
    }

    /// MUTATION: a stale-bits input-lane mutation fires the recompose; a direct
    /// absorb-input cell mutation fires the absorb-link or per-perm AIR.
    #[test]
    fn poseidon2_commit_air_mutate_input_lane_breaks() {
        let params = Poseidon2Params::new();
        let pad = [9u8; 32];
        let nonce = [2u8; NONCE_LANES];
        let trace = generate_commit_trace(&pad, &nonce);
        let committed = claimed_commitment(&trace);

        let mut t = trace.clone();
        t.input_lanes[40] = fq_add(t.input_lanes[40], 1); // a nonce lane
        let res = all_residuals(&params, &t, &committed);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::BitRecompose { lane: 40 })),
            "stale-bits nonce lane mutation not caught"
        );

        let mut t2 = trace.clone();
        t2.subs[0].rows[0][3] = fq_add(t2.subs[0].rows[0][3], 1);
        let res2 = all_residuals(&params, &t2, &committed);
        assert!(
            res2.iter().any(|r| r.value != 0),
            "absorb input cell mutation not caught"
        );
    }

    /// MUTATION: an intermediate permutation cell fires the reused per-perm AIR.
    #[test]
    fn poseidon2_commit_air_mutate_intermediate_perm_cell_breaks() {
        let params = Poseidon2Params::new();
        let pad = [11u8; 32];
        let nonce = [4u8; NONCE_LANES];
        let trace = generate_commit_trace(&pad, &nonce);
        let committed = claimed_commitment(&trace);

        let mut t = trace.clone();
        t.subs[5].rows[30][9] = fq_add(t.subs[5].rows[30][9], 1);
        let res = all_residuals(&params, &t, &committed);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::Perm { perm: 5, .. })),
            "intermediate perm cell mutation not caught"
        );
    }

    /// MUTATION: a claimed commitment output mutation fires the commit boundary,
    /// on both the first squeeze (lanes 0..5) and the second squeeze (lanes 5..10).
    #[test]
    fn poseidon2_commit_air_mutate_claimed_commitment_breaks() {
        let params = Poseidon2Params::new();
        let pad = [17u8; 32];
        let nonce = [6u8; NONCE_LANES];
        let trace = generate_commit_trace(&pad, &nonce);

        let mut c1 = claimed_commitment(&trace);
        c1[2] = fq_add(c1[2], 1); // first squeeze lane
        let res = all_residuals(&params, &trace, &c1);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::CommitOutput { out: 2 })),
            "first-squeeze commitment mutation not caught"
        );

        let mut c2 = claimed_commitment(&trace);
        c2[8] = fq_add(c2[8], 1); // second squeeze lane
        let res2 = all_residuals(&params, &trace, &c2);
        assert!(
            res2.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::CommitOutput { out: 8 })),
            "second-squeeze commitment mutation not caught"
        );
    }

    /// BYTE RANGE: an out-of-range byte (>= 256) on a blinding lane is rejected.
    #[test]
    fn poseidon2_commit_air_out_of_range_byte_rejected() {
        let params = Poseidon2Params::new();
        let pad = [21u8; 32];
        let nonce = [1u8; NONCE_LANES];
        let trace = generate_commit_trace(&pad, &nonce);
        let committed = claimed_commitment(&trace);

        // (a) value 256 with 8 zero bits: recompose mismatches.
        let mut t = trace.clone();
        t.input_lanes[45] = 256;
        t.input_bits[45] = [0u32; BITS_PER_LANE];
        let res = all_residuals(&params, &t, &committed);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::BitRecompose { lane: 45 })),
            "out-of-range byte not rejected by recompose"
        );

        // (b) a non-boolean bit (256) on a pad_seed lane: bit-boolean fires.
        let mut t2 = trace.clone();
        t2.input_lanes[0] = 256;
        t2.input_bits[0] = [0u32; BITS_PER_LANE];
        t2.input_bits[0][0] = 256;
        let res2 = all_residuals(&params, &t2, &committed);
        assert!(
            res2.iter()
                .any(|r| r.value != 0
                    && matches!(r.kind, ResidualKind::BitBoolean { lane: 0, bit: 0 })),
            "non-boolean bit not rejected"
        );
    }
}
