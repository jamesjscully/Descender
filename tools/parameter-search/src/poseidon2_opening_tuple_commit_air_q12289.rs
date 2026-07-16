//! Framework-agnostic AIR for the opening-tuple commitment sponge
//! `opening_tuple_commitment_poseidon2(account_projection, lifecycle_anchor,
//! blinding_nonce)` over Fq = 12289.
//!
//! This proves, in-circuit, that 20 committed commitment lanes ARE the
//! Poseidon2-sponge outputs of:
//!   * 6 `account_projection` FIELD lanes (each a canonical residue < q,
//!     absorbed AS field elements — NOT byte-decomposed),
//!   * a HIDDEN 32-byte `lifecycle_anchor`, and
//!   * a HIDDEN 16-byte `blinding_nonce`.
//!
//! It is obligation **C6** (opening-tuple binding) of the account-creation STARK
//! relation. The 6 `account_projection` field lanes are EXPOSED as accessor
//! cells so the composed relation can SEAM them to the C4 base-128 carrier's
//! reconstruction (`range[i].v`), making the opening-tuple commitment bind the
//! SAME account_projection the carrier uses (without that shared seam, the
//! co-location would be vacuous).
//!
//! CORRECTNESS and HONEST labeling are the priorities, not speed.
//!
//! This module is a DIRECT MIRROR of the C1 pad-source commitment AIR
//! (`crate::poseidon2_commit_air_q12289`). Like C1 it does NOT reimplement the
//! permutation or its constraints: it CHAINS the single-permutation AIR
//! (`crate::poseidon2_air_q12289`) — one sub-trace per sponge permutation — and
//! adds the sponge LINKING constraints (capacity init, absorb-add, squeeze
//! read-off) plus BYTE-RANGE constraints on the 32+16 BYTE input lanes. The 6
//! account_projection lanes are FIELD lanes and therefore carry NO byte-range
//! check (they are range-checked elsewhere, by C4's canonical base-128 gadget).
//! The per-permutation transition/boundary logic is REUSED verbatim.
//!
//! =============================================================================
//! PERMUTATION COUNT (derived from the reference sponge, NOT guessed)
//! =============================================================================
//!
//! `opening_tuple_commitment_poseidon2` builds 6 + 32 + 16 = 54 input lanes
//! (6 account_projection field lanes, 32 lifecycle_anchor byte-lanes, 16
//! blinding_nonce byte-lanes) and calls `poseidon2_sponge(domain, inputs,
//! n_out=20)`:
//!
//!   * 10* padding: 54 inputs + 1 pad marker = 55, already a multiple of
//!     RATE = 5  =>  55 / 5 = 11 absorb permutations.
//!   * Squeeze n_out = 20: 4 reads of 5 with a permute after each non-final read
//!     =>  3 squeeze permutations.
//!
//!   TOTAL PERMUTATIONS = 11 absorb + 3 squeeze = 14.
//!
//! These are DERIVED from the reference sponge shape (`derive_block_plan`) from
//! N_INPUT_LANES / RATE / N_COMMIT_OUTPUTS, so a change to any of those
//! re-derives the count rather than relying on the literal 14.
//!
//! =============================================================================
//! TRACE LAYOUT  (identical shape to the C1 commit AIR, but with field lanes)
//! =============================================================================
//!
//!   * `subs[p]` for p in 0..N_PERMS (=14): the full single-permutation AIR
//!     sub-trace for the p-th permutation.
//!   * `input_lanes[54]`: the witness input lanes —
//!       indices  0..6  : account_projection FIELD lanes (canonical residue < q),
//!       indices  6..38 : lifecycle_anchor byte-lanes (each in [0,256)),
//!       indices 38..54 : blinding_nonce byte-lanes (each in [0,256)).
//!   * `byte_bits[N_BYTE_LANES][8]`: 8-bit little-endian decomposition for the
//!     48 BYTE lanes only (the 6 field lanes get NO bit decomposition).
//!   * `commit_outputs[20]`: the COMMITTED commitment lanes, boundary-bound to
//!     the squeezed state cells.
//!
//! CONSTRAINTS are the same FAMILY as C1 (per-permutation degree<=5; all
//! linking/range constraints degree <= 2). MAX DEGREE = 5 (the S-box).

use crate::poseidon2_air_q12289::{
    self as perm_air, Trace as PermTrace, generate_trace as gen_perm_trace,
};
use crate::poseidon2_q12289::{
    CAPACITY, OPENING_TUPLE_ACCOUNT_PROJECTION_LANES, OPENING_TUPLE_BLINDING_BYTES,
    OPENING_TUPLE_COMMITMENT_LANES, OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES,
    POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN, Poseidon2Params, RATE, T, fq_add, fq_from_u32,
    fq_mul, fq_sub, init_capacity,
};

// -----------------------------------------------------------------------------
// Static shape constants (derived, not guessed)
// -----------------------------------------------------------------------------

/// Number of account_projection FIELD lanes (absorbed as field elements).
pub const ACCOUNT_PROJECTION_LANES: usize = OPENING_TUPLE_ACCOUNT_PROJECTION_LANES; // 6
/// Number of lifecycle_anchor byte-lanes.
pub const LIFECYCLE_ANCHOR_LANES: usize = OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES; // 32
/// Number of blinding-nonce byte-lanes.
pub const NONCE_LANES: usize = OPENING_TUPLE_BLINDING_BYTES; // 16
/// Total input lanes absorbed (field + byte).
pub const N_INPUT_LANES: usize = ACCOUNT_PROJECTION_LANES + LIFECYCLE_ANCHOR_LANES + NONCE_LANES; // 54
/// Number of BYTE lanes (those that carry an 8-bit range decomposition). These
/// are exactly the lifecycle_anchor + nonce lanes; the field lanes are excluded.
pub const N_BYTE_LANES: usize = LIFECYCLE_ANCHOR_LANES + NONCE_LANES; // 48
/// Index in `input_lanes` where the byte lanes begin (after the field lanes).
pub const BYTE_LANES_OFFSET: usize = ACCOUNT_PROJECTION_LANES; // 6
/// Bits per byte-lane decomposition.
pub const BITS_PER_LANE: usize = 8;
/// Number of committed commitment output lanes (= 10).
pub const N_COMMIT_OUTPUTS: usize = OPENING_TUPLE_COMMITMENT_LANES;

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

/// The full, deterministic permutation plan for the opening-tuple commitment.
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
/// stream chunked by RATE. Input lane order matches the reference encoding:
/// account_projection field lanes, then lifecycle_anchor bytes, then nonce bytes.
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

/// The opening-tuple commitment execution trace.
#[derive(Clone, Debug)]
pub struct Trace {
    /// One single-permutation AIR sub-trace per permutation (length = n_perms()).
    pub subs: Vec<PermTrace>,
    /// The 54 input lanes:
    ///   [0..6)   account_projection FIELD lanes (canonical residue < q),
    ///   [6..38)  lifecycle_anchor byte-lanes (each in [0,256)),
    ///   [38..54) blinding_nonce byte-lanes (each in [0,256)).
    pub input_lanes: [u32; N_INPUT_LANES],
    /// 8-bit little-endian decomposition of each of the 48 BYTE lanes only,
    /// indexed by byte-lane number 0..48 (NOT by `input_lanes` index).
    pub byte_bits: [[u32; BITS_PER_LANE]; N_BYTE_LANES],
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

    /// The 6 account_projection FIELD lane cells — the SEAM cells the composed
    /// relation binds to the C4 carrier's reconstruction (`range[i].v`). These
    /// ARE the first 6 entries of `input_lanes`.
    pub fn account_projection_lanes(&self) -> &[u32] {
        &self.input_lanes[0..ACCOUNT_PROJECTION_LANES]
    }
}

/// Encode the 54 input lanes exactly as the reference commitment does:
/// account_projection field lanes, lifecycle_anchor bytes, then nonce bytes.
fn encode_input_lanes(
    account_projection: &[u16; ACCOUNT_PROJECTION_LANES],
    lifecycle_anchor: &[u8; LIFECYCLE_ANCHOR_LANES],
    nonce: &[u8; NONCE_LANES],
) -> [u32; N_INPUT_LANES] {
    let mut lanes = [0u32; N_INPUT_LANES];
    for (i, &v) in account_projection.iter().enumerate() {
        lanes[i] = v as u32; // field lane (canonical residue)
    }
    for (i, &b) in lifecycle_anchor.iter().enumerate() {
        lanes[ACCOUNT_PROJECTION_LANES + i] = b as u32;
    }
    for (i, &b) in nonce.iter().enumerate() {
        lanes[ACCOUNT_PROJECTION_LANES + LIFECYCLE_ANCHOR_LANES + i] = b as u32;
    }
    lanes
}

fn byte_bits_of(v: u32) -> [u32; BITS_PER_LANE] {
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
/// opening-tuple commitment and records each permutation as a single-permutation
/// AIR sub-trace, plus all linking witnesses. By construction the recorded
/// states are exactly those the reference sponge visits.
pub fn generate_commit_trace(
    account_projection: &[u16; ACCOUNT_PROJECTION_LANES],
    lifecycle_anchor: &[u8; LIFECYCLE_ANCHOR_LANES],
    nonce: &[u8; NONCE_LANES],
) -> Trace {
    let params = Poseidon2Params::new();
    let plan = build_plan();
    let input_lanes = encode_input_lanes(account_projection, lifecycle_anchor, nonce);

    // Byte-bit decompositions for the 48 BYTE lanes only (skip the 6 field lanes).
    let mut byte_bits = [[0u32; BITS_PER_LANE]; N_BYTE_LANES];
    for j in 0..N_BYTE_LANES {
        byte_bits[j] = byte_bits_of(input_lanes[BYTE_LANES_OFFSET + j]);
    }

    let mut state = init_capacity(POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN);

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
        byte_bits,
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
    Perm {
        perm: usize,
        index: usize,
    },
    CapacityInit {
        lane: usize,
    },
    AbsorbLink {
        perm: usize,
        lane: usize,
    },
    SqueezeLink {
        perm: usize,
        lane: usize,
    },
    /// Booleanity of a bit of a BYTE lane (indexed by byte-lane number 0..48).
    BitBoolean {
        byte_lane: usize,
        bit: usize,
    },
    /// 8-bit recompose of a BYTE lane (indexed by byte-lane number 0..48).
    BitRecompose {
        byte_lane: usize,
    },
    CommitOutput {
        out: usize,
    },
}

/// Evaluate EVERY constraint residual for `trace` against the committed
/// `commit_outputs`. All-zero == the 10 committed lanes ARE the commitment of
/// the 6 account_projection field lanes plus the byte-range-checked hidden
/// (lifecycle_anchor, blinding_nonce).
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
    let init_state = init_capacity(POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN);
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

    // 5. Byte-range constraints on each BYTE lane (lifecycle_anchor + nonce).
    //    The 6 account_projection FIELD lanes carry NO byte-range check; they
    //    are canonical residues range-checked by C4's base-128 gadget.
    for byte_lane in 0..N_BYTE_LANES {
        let bits = &trace.byte_bits[byte_lane];
        for (bit, &b) in bits.iter().enumerate() {
            let residual = fq_mul(b, fq_sub(b, 1));
            out.push(Residual {
                kind: ResidualKind::BitBoolean { byte_lane, bit },
                value: residual,
            });
        }
        let mut acc = 0u32;
        let mut weight = 1u32;
        for &b in bits.iter() {
            acc = fq_add(acc, fq_mul(b, weight));
            weight = fq_mul(weight, 2);
        }
        let residual = fq_sub(acc, trace.input_lanes[BYTE_LANES_OFFSET + byte_lane]);
        out.push(Residual {
            kind: ResidualKind::BitRecompose { byte_lane },
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
    let byte_range = N_BYTE_LANES * (BITS_PER_LANE + 1);
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
    use crate::poseidon2_q12289::{Q, opening_tuple_commitment_poseidon2};

    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    fn random_account_projection(seed: &mut u64) -> [u16; ACCOUNT_PROJECTION_LANES] {
        let mut a = [0u16; ACCOUNT_PROJECTION_LANES];
        for x in a.iter_mut() {
            *x = (xorshift(seed) % Q as u64) as u16; // canonical residue
        }
        a
    }

    fn random_anchor(seed: &mut u64) -> [u8; LIFECYCLE_ANCHOR_LANES] {
        let mut p = [0u8; LIFECYCLE_ANCHOR_LANES];
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
    fn opening_tuple_commit_air_permutation_count_is_derived() {
        let (n_absorb, n_squeeze) = derive_block_plan();
        // 54 inputs + 1 pad marker = 55 (already a multiple of 5) -> 11 absorb.
        assert_eq!(n_absorb, 11, "expected 11 absorb permutations");
        // n_out = 20: 4 reads of 5 with a permute after each non-final read ->
        // 3 squeeze permutations.
        assert_eq!(n_squeeze, 3, "expected 3 squeeze permutations");
        assert_eq!(n_perms(), 14);
        let plan = build_plan();
        assert_eq!(plan.roles.len(), 14);
        for (p, role) in plan.roles.iter().enumerate() {
            if p < 11 {
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
    fn opening_tuple_commit_air_sizes_report() {
        let acc = [1u16, 2, 3, 4, 5, 6];
        let anchor = [3u8; LIFECYCLE_ANCHOR_LANES];
        let nonce = [5u8; NONCE_LANES];
        let trace = generate_commit_trace(&acc, &anchor, &nonce);
        assert_eq!(trace.subs.len(), 14);
        for sub in &trace.subs {
            assert_eq!(sub.rows.len(), perm_air::N_ROWS);
        }
        let perm_cells = 14 * perm_air::N_ROWS * T;
        let aux_cells = N_INPUT_LANES + N_BYTE_LANES * BITS_PER_LANE + N_COMMIT_OUTPUTS;
        println!(
            "C6 OPENING-TUPLE COMMIT AIR: perms=14 perm_cells={perm_cells} aux_cells={aux_cells} \
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
    /// many random (account_projection, anchor, nonce); honest trace residual 0.
    #[test]
    fn opening_tuple_commit_air_output_matches_reference() {
        let params = Poseidon2Params::new();
        let mut seed = 0xC0FF_EE99_8877_6655u64;
        for _ in 0..32 {
            let acc = random_account_projection(&mut seed);
            let anchor = random_anchor(&mut seed);
            let nonce = random_nonce(&mut seed);
            let trace = generate_commit_trace(&acc, &anchor, &nonce);
            let reference = opening_tuple_commitment_poseidon2(&acc, &anchor, &nonce);
            let claimed = claimed_commitment(&trace);
            for j in 0..N_COMMIT_OUTPUTS {
                assert_eq!(
                    claimed[j] as u16, reference[j],
                    "AIR commit lane {j} != reference (acc[0]={})",
                    acc[0]
                );
                assert!(claimed[j] < Q, "commit not canonical");
            }
            // exposed account_projection accessor matches the inputs.
            for (i, &v) in acc.iter().enumerate() {
                assert_eq!(
                    trace.account_projection_lanes()[i],
                    v as u32,
                    "account_projection accessor lane {i} mismatch"
                );
            }
            // honest trace residual 0
            let res = all_residuals(&params, &trace, &claimed);
            assert_eq!(res.len(), n_constraints());
            for r in &res {
                assert_eq!(r.value, 0, "honest C6 trace violated {r:?}");
            }
            assert_eq!(max_residual(&params, &trace, &claimed), 0);
        }
    }

    /// MUTATION: a stale-bits byte-lane mutation fires the recompose; a direct
    /// absorb-input cell mutation fires the absorb-link or per-perm AIR.
    #[test]
    fn opening_tuple_commit_air_mutate_input_lane_breaks() {
        let params = Poseidon2Params::new();
        let acc = [9u16, 8, 7, 6, 5, 4];
        let anchor = [9u8; LIFECYCLE_ANCHOR_LANES];
        let nonce = [2u8; NONCE_LANES];
        let trace = generate_commit_trace(&acc, &anchor, &nonce);
        let committed = claimed_commitment(&trace);

        // mutate a nonce byte lane (byte-lane index 32+5 = 37, input_lanes 6+37).
        let mut t = trace.clone();
        let bl = LIFECYCLE_ANCHOR_LANES + 5; // a nonce byte lane
        t.input_lanes[BYTE_LANES_OFFSET + bl] = fq_add(t.input_lanes[BYTE_LANES_OFFSET + bl], 1);
        let res = all_residuals(&params, &t, &committed);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::BitRecompose { byte_lane } if byte_lane == bl)),
            "stale-bits nonce byte-lane mutation not caught"
        );

        let mut t2 = trace.clone();
        t2.subs[0].rows[0][3] = fq_add(t2.subs[0].rows[0][3], 1);
        let res2 = all_residuals(&params, &t2, &committed);
        assert!(
            res2.iter().any(|r| r.value != 0),
            "absorb input cell mutation not caught"
        );
    }

    /// MUTATION: tampering an account_projection FIELD lane (NO byte decomposition
    /// to keep stale) is caught by the absorb-link constraint, since the absorbed
    /// state must equal prev_out + field-lane value.
    #[test]
    fn opening_tuple_commit_air_mutate_field_lane_breaks() {
        let params = Poseidon2Params::new();
        let acc = [12u16, 345, 6789, 0, 11000, 1];
        let anchor = [4u8; LIFECYCLE_ANCHOR_LANES];
        let nonce = [7u8; NONCE_LANES];
        let trace = generate_commit_trace(&acc, &anchor, &nonce);
        let committed = claimed_commitment(&trace);

        // mutate account_projection field lane 2 (input_lanes[2]) WITHOUT
        // re-running the sponge -> absorb-link for the perm that absorbs it fires.
        let mut t = trace.clone();
        t.input_lanes[2] = fq_add(t.input_lanes[2], 1);
        let res = all_residuals(&params, &t, &committed);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::AbsorbLink { .. })),
            "account_projection field-lane mutation not caught by absorb-link"
        );
    }

    /// MUTATION: an intermediate permutation cell fires the reused per-perm AIR.
    #[test]
    fn opening_tuple_commit_air_mutate_intermediate_perm_cell_breaks() {
        let params = Poseidon2Params::new();
        let acc = [11u16, 22, 33, 44, 55, 66];
        let anchor = [11u8; LIFECYCLE_ANCHOR_LANES];
        let nonce = [4u8; NONCE_LANES];
        let trace = generate_commit_trace(&acc, &anchor, &nonce);
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
    fn opening_tuple_commit_air_mutate_claimed_commitment_breaks() {
        let params = Poseidon2Params::new();
        let acc = [17u16, 18, 19, 20, 21, 22];
        let anchor = [17u8; LIFECYCLE_ANCHOR_LANES];
        let nonce = [6u8; NONCE_LANES];
        let trace = generate_commit_trace(&acc, &anchor, &nonce);

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

    /// BYTE RANGE: an out-of-range byte (>= 256) on a lifecycle/nonce lane is
    /// rejected.
    #[test]
    fn opening_tuple_commit_air_out_of_range_byte_rejected() {
        let params = Poseidon2Params::new();
        let acc = [21u16, 22, 23, 24, 25, 26];
        let anchor = [21u8; LIFECYCLE_ANCHOR_LANES];
        let nonce = [1u8; NONCE_LANES];
        let trace = generate_commit_trace(&acc, &anchor, &nonce);
        let committed = claimed_commitment(&trace);

        // (a) value 256 with 8 zero bits on a nonce byte lane: recompose fails.
        let mut t = trace.clone();
        let bl = LIFECYCLE_ANCHOR_LANES + 7; // a nonce byte lane
        t.input_lanes[BYTE_LANES_OFFSET + bl] = 256;
        t.byte_bits[bl] = [0u32; BITS_PER_LANE];
        let res = all_residuals(&params, &t, &committed);
        assert!(
            res.iter().any(|r| r.value != 0
                && matches!(r.kind, ResidualKind::BitRecompose { byte_lane } if byte_lane == bl)),
            "out-of-range byte not rejected by recompose"
        );

        // (b) a non-boolean bit (256) on a lifecycle_anchor byte lane.
        let mut t2 = trace.clone();
        t2.input_lanes[BYTE_LANES_OFFSET + 0] = 256;
        t2.byte_bits[0] = [0u32; BITS_PER_LANE];
        t2.byte_bits[0][0] = 256;
        let res2 = all_residuals(&params, &t2, &committed);
        assert!(
            res2.iter().any(|r| r.value != 0
                && matches!(
                    r.kind,
                    ResidualKind::BitBoolean {
                        byte_lane: 0,
                        bit: 0
                    }
                )),
            "non-boolean bit not rejected"
        );
    }
}
