//! The FULL account-creation relation as a SINGLE STARK-provable [`Air`].
//!
//! This is the culminating integration brick. It lays the THREE Poseidon2
//! sponge chains (C1 pad-source commitment, C2 projection-mask PRF, C6
//! opening-tuple commitment) plus the C4 canonical base-128 reconstruction as
//! PARALLEL COLUMN GROUPS in one unified trace, ties them with the cross-group
//! SEAMS and the masked-projection carrier, and is proven END-TO-END by the
//! generic DEEP-ALI STARK (`crate::stark_q12289`). Verifying one proof of this
//! AIR establishes the whole C1 ∧ C2 ∧ C4 ∧ C6 account-creation relation
//! (modulo the FRI proximity-gap + DEEP-ALI assumptions named in `stark_q12289`).
//!
//! It REUSES, almost verbatim, the degree-reduced (deg-3) Poseidon2 sponge AIR
//! arithmetic of `crate::c2_prf_air_q12289` — generalized here into a small
//! `SpongeChain` helper parameterized by (domain, input-lane plan, n_out). The
//! S-box is degree-reduced x^5 -> deg 3 with one aux "square" column per lane,
//! so the whole composition fits a 16384-point Fq12 FRI domain.
//!
//! =============================================================================
//! WHY ONE AIR INSTEAD OF THREE PROOFS
//! =============================================================================
//! Proving C1, C2, C4, C6 as four independent proofs would NOT bind their shared
//! secrets: nothing would force the pad_seed that opens the pad-source
//! commitment (C1) to be the pad_seed that derives the masks (C2), nor force the
//! account_projection committed by C6 to be the canonical residue range-checked
//! by C4 and consumed by the carrier. The whole relation is the SEAMS. By
//! co-locating all four in ONE trace we can express each seam as a SAME-ROW
//! cross-column-group EQUALITY residual (see `transition_residuals`), so a single
//! STARK proof certifies the conjunction WITH the bindings.
//!
//! =============================================================================
//! COLUMN GROUPS  (n_cols laid out as disjoint blocks of the same row index)
//! =============================================================================
//! Each sponge group reuses the C2 per-row layout (state, sq aux, lane_val, bits):
//!   per group width PG = 2*T + RATE + RATE*8 = 93.
//!   * C1 group : offset C1_OFF, perms = 13 (10 absorb + 3 squeeze), 910 rows.
//!   * C2 group : offset C2_OFF, perms = 10 ( 9 absorb + 1 squeeze), 700 rows.
//!   * C6 group : offset C6_OFF, perms = 14 (11 absorb + 3 squeeze), 980 rows.
//! Each group's permutation p occupies that group's rows p*70 .. p*70+70; rows
//! past a group's last perm are PADDING for that group (no constraint reads them).
//! Trace length = max group depth = 14*70 = 980, padded to 1024.
//!
//!   * C4 block : offset C4_OFF, 6 residue sub-blocks of CELLS_PER_RESIDUE=26
//!     cells each. Residue i is laid on the SAME ROW as C6's i-th
//!     account_projection FIELD lane (lanes 0..4 -> row 0; lane 5 -> row 70), so
//!     the C4 canonical constraints AND the C4<->C6 seam are both single-row.
//!
//! =============================================================================
//! SEAMS  (each a SAME-ROW cross-group equality residual)
//! =============================================================================
//!   * SeamPadSeed (C1 == C2), 32 lanes. pad_seed byte i is absorbed by both C1
//!     and C2 at absorb-perm `i/RATE`, rate lane `i%RATE`. Its witness cell is
//!     that perm's INPUT-row `lane_val[i%RATE]`. So at global row `(i/5)*70` we
//!     emit  cur[C1_OFF+LANE_VAL+i%5] - cur[C2_OFF+LANE_VAL+i%5] = 0.
//!   * SeamOpeningAcc (C4 == C6), 6 lanes. C6's account_projection field lane i is
//!     absorbed at perm `i/RATE`, rate lane `i%RATE`; its cell is that perm's
//!     INPUT-row `lane_val[i%RATE]`. C4 residue i's reconstructed value `v` is on
//!     the SAME row (lane 0..4 -> row 0, lane 5 -> row 70). So at that row we emit
//!     C4_block[i].v - cur[C6_OFF+LANE_VAL+i%5] = 0.
//!   * Carrier (C2 mask + C4 acc -> masked), 6 lanes. At row 0 (where the C2
//!     squeeze output and the C4 residues are read into row-0 carrier columns) we
//!     bind masked_i - mask_i - acc_i = 0 with mask_i the C2 squeezed output and
//!     acc_i the C4 reconstruction, both copied into dedicated row-0 carrier
//!     columns and pinned by same-row equality + boundary.
//!
//! =============================================================================
//! PUBLIC INPUTS
//! =============================================================================
//!   pad_source_commitment[20]  (C1 output, boundary-bound),
//!   opening_tuple_commitment[20] (C6 output, boundary-bound),
//!   masked_projection[6]       (carrier output, boundary-bound).
//!
//! MAX DEGREE = 3 (degree-reduced sponge S-box). Every seam / carrier / C4
//! constraint is degree <= 2.

use crate::air::{Air, BoundaryConstraint};
use crate::base128_range_air_q12289 as c4;
use crate::fq12_q12289::Fq12;
use crate::poseidon2_air_q12289::{transition_kind, TransitionKind, N_ROWS as PERM_ROWS};
use crate::poseidon2_q12289::{
    apply_internal, apply_matrix, fq_add, fq_from_u32, fq_mul, init_capacity, sbox,
    Poseidon2Params, OPENING_TUPLE_BLINDING_BYTES, OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES,
    PAD_SOURCE_BLINDING_BYTES, POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN,
    POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN, POSEIDON2_PROJECTION_MASK_DOMAIN, RATE, R_F_HALF, R_P, T,
};

// =============================================================================
// Per-group sponge column sub-layout (identical to c2_prf_air_q12289)
// =============================================================================

const BITS_PER_LANE: usize = 8;
/// per-group column width: state (T) + sq aux (T) + lane_val (RATE) + bits.
const PG: usize = 2 * T + RATE + RATE * BITS_PER_LANE; // 93
const STATE: usize = 0;
const SQ: usize = T;
const LANE_VAL: usize = 2 * T;
const BITS: usize = 2 * T + RATE;
const fn bit_col_local(i: usize, b: usize) -> usize {
    BITS + i * BITS_PER_LANE + b
}

/// Number of projection lanes (= 6).
pub const N_LANES: usize = 6;
/// pad_seed seam width (C1 <-> C2).
pub const PAD_SEED_LANES: usize = 32;
/// account_projection seam width (C4 <-> C6).
pub const ACC_LANES: usize = N_LANES;
/// C1 / C6 commitment output lanes (public), C2 mask output lanes.
pub const N_COMMIT_LANES: usize = 20;
pub const N_MASK_OUTPUTS: usize = N_LANES;

// =============================================================================
// One sponge chain: a self-contained description of an absorb/squeeze plan.
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockElem {
    /// witness byte/field lane absorbed here, with `byte_ranged` = needs 8-bit
    /// decomposition (true for byte lanes, false for C6 field lanes).
    Lane { value: u32, byte_ranged: bool },
    /// public sponge constant (10* pad marker 1 / 0 padding).
    Const(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Absorb { block: [BlockElem; RATE] },
    Squeeze,
}

/// A concrete sponge chain instance: its domain, its plan (roles), the running
/// per-permutation states, and its squeezed outputs.
struct SpongeChain {
    domain: &'static [u8],
    roles: Vec<Role>,
    n_absorb: usize,
    /// states[p] = the 70 state rows of permutation p (row 0 = input).
    states: Vec<Vec<[u32; T]>>,
    n_out: usize,
    outputs: Vec<u32>,
}

impl SpongeChain {
    fn n_perms(&self) -> usize {
        self.roles.len()
    }

    /// For output lane `out`, the (permutation index, state lane) the squeeze
    /// reads it from — mirroring the squeeze read-off loop.
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

    /// The global trace row holding output lane `out` (its source perm's output
    /// row) and the state column lane within that row.
    fn output_row_lane(&self, out: usize) -> (usize, usize) {
        let (p, lane) = self.output_source(out);
        (p * PERM_ROWS + (PERM_ROWS - 1), lane)
    }
}

/// Build the role plan for a stream of (value, byte_ranged) lanes: 10* padding
/// then RATE-chunking, then `n_squeeze` squeeze perms.
fn build_roles(lanes: &[(u32, bool)], n_out: usize) -> (Vec<Role>, usize, usize) {
    let mut stream: Vec<BlockElem> = lanes
        .iter()
        .map(|&(v, br)| BlockElem::Lane {
            value: v,
            byte_ranged: br,
        })
        .collect();
    stream.push(BlockElem::Const(1)); // 10* pad marker
    while stream.len() % RATE != 0 {
        stream.push(BlockElem::Const(0));
    }
    let n_absorb = stream.len() / RATE;

    // squeeze count: read min(remaining,RATE), permute if more remain.
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

    let mut roles: Vec<Role> = Vec::new();
    for chunk in stream.chunks(RATE) {
        let mut block = [BlockElem::Const(0); RATE];
        block.copy_from_slice(chunk);
        roles.push(Role::Absorb { block });
    }
    for _ in 0..n_squeeze {
        roles.push(Role::Squeeze);
    }
    (roles, n_absorb, n_squeeze)
}

/// The aux squares for the S-box of the transition LEAVING local row `local`
/// (degree-reduction trick). Mirrors c2_prf_air_q12289::sq_for_leaving.
fn sq_for_leaving(params: &Poseidon2Params, local: usize, state: &[u32; T]) -> [u32; T] {
    if local + 1 >= PERM_ROWS {
        return [0u32; T];
    }
    let mut sq = [0u32; T];
    match transition_kind(local) {
        TransitionKind::InitialExternal => {}
        TransitionKind::Full { rc_index } => {
            let rc = &params.rc_full[rc_index];
            for k in 0..T {
                let a = fq_add(state[k], rc[k]);
                sq[k] = fq_mul(a, a);
            }
        }
        TransitionKind::Partial { rc_index } => {
            let a = fq_add(state[0], params.rc_partial[rc_index]);
            sq[0] = fq_mul(a, a);
        }
    }
    sq
}

/// Generate one permutation's 70 state rows from `input_state`. Mirrors
/// c2_prf_air_q12289::gen_perm_state_rows.
fn gen_perm_state_rows(params: &Poseidon2Params, input_state: &[u32; T]) -> Vec<[u32; T]> {
    let mut states: Vec<[u32; T]> = Vec::with_capacity(PERM_ROWS);
    let mut state = *input_state;
    for s in state.iter_mut() {
        *s = fq_from_u32(*s);
    }
    states.push(state);
    apply_matrix(&params.m_e, &mut state);
    states.push(state);
    for r in 0..R_F_HALF {
        let rc = &params.rc_full[r];
        for k in 0..T {
            state[k] = fq_add(state[k], rc[k]);
        }
        for k in 0..T {
            state[k] = sbox(state[k]);
        }
        apply_matrix(&params.m_e, &mut state);
        states.push(state);
    }
    for r in 0..R_P {
        state[0] = fq_add(state[0], params.rc_partial[r]);
        state[0] = sbox(state[0]);
        apply_internal(&params.internal_diag, &mut state);
        states.push(state);
    }
    for r in 0..R_F_HALF {
        let rc = &params.rc_full[R_F_HALF + r];
        for k in 0..T {
            state[k] = fq_add(state[k], rc[k]);
        }
        for k in 0..T {
            state[k] = sbox(state[k]);
        }
        apply_matrix(&params.m_e, &mut state);
        states.push(state);
    }
    debug_assert_eq!(states.len(), PERM_ROWS);
    states
}

/// Resolve a block element to its concrete absorbed value.
fn block_value(e: BlockElem) -> u32 {
    match e {
        BlockElem::Lane { value, .. } => fq_from_u32(value),
        BlockElem::Const(c) => fq_from_u32(c),
    }
}

impl SpongeChain {
    /// Build the full chain: run the honest sponge, recording per-perm states and
    /// the squeezed outputs.
    fn new(params: &Poseidon2Params, domain: &'static [u8], lanes: &[(u32, bool)], n_out: usize) -> Self {
        let (roles, n_absorb, _n_squeeze) = build_roles(lanes, n_out);
        let mut run = init_capacity(domain);
        let mut states: Vec<Vec<[u32; T]>> = Vec::with_capacity(roles.len());
        for role in &roles {
            if let Role::Absorb { block } = role {
                for i in 0..RATE {
                    run[i] = fq_add(run[i], block_value(block[i]));
                }
            }
            let st = gen_perm_state_rows(params, &run);
            run = st[PERM_ROWS - 1];
            states.push(st);
        }
        // squeeze read-off
        let mut outputs = vec![0u32; n_out];
        let mut filled = 0usize;
        let mut read_perm = n_absorb - 1;
        let mut remaining = n_out;
        loop {
            let take = remaining.min(RATE);
            let src = states[read_perm][PERM_ROWS - 1];
            for i in 0..take {
                outputs[filled] = src[i];
                filled += 1;
            }
            remaining -= take;
            if remaining == 0 {
                break;
            }
            read_perm += 1;
        }
        SpongeChain {
            domain,
            roles,
            n_absorb,
            states,
            n_out,
            outputs,
        }
    }
}

// =============================================================================
// Unified column layout
// =============================================================================

const C1_OFF: usize = 0;
const C2_OFF: usize = PG;
const C6_OFF: usize = 2 * PG;
const C4_OFF: usize = 3 * PG;
/// C4 cell index layout (see base128_range CELLS_PER_RESIDUE).
const C4_CELLS: usize = c4::CELLS_PER_RESIDUE; // 26
/// carrier columns (masked[6]) on row 0.
const CARRY_OFF: usize = 3 * PG + N_LANES * C4_CELLS;
const CARRY_MASKED: usize = CARRY_OFF; // 6  public masked_projection, row 0
/// MASK propagation columns: constant-down-all-rows copies of the 6 C2 squeezed
/// mask outputs, seeded at their squeeze rows, consumed by the carrier at row 0.
const MPROP_OFF: usize = CARRY_OFF + N_LANES; // 6
/// ACC propagation columns: constant-down-all-rows copies of the 6 C4
/// reconstructed account_projection values, seeded at their C4 rows, consumed by
/// the carrier at row 0.
const APROP_OFF: usize = MPROP_OFF + N_LANES; // 6
/// total columns.
pub const N_COLS: usize = APROP_OFF + N_LANES;

/// The row a C4 residue (and the matching C6 field lane) lives on.
fn acc_lane_row(i: usize) -> usize {
    (i / RATE) * PERM_ROWS
}
/// The C6 lane_val column for account_projection field lane i.
fn c6_field_lane_col(i: usize) -> usize {
    C6_OFF + LANE_VAL + (i % RATE)
}

/// Padded trace rows: max group depth (C6 = 14 perms) padded to power of two.
fn n_rows_padded(c6_perms: usize) -> usize {
    let real = c6_perms * PERM_ROWS;
    let mut n = 1usize;
    while n < real {
        n <<= 1;
    }
    n
}

// =============================================================================
// The full-relation AIR
// =============================================================================

/// The full C1 ∧ C2 ∧ C4 ∧ C6 account-creation relation as one AIR.
pub struct FullRelationAir {
    params: Poseidon2Params,
    c1: SpongeChain,
    c2: SpongeChain,
    c6: SpongeChain,
    c4: Vec<c4::Base128RangeWitness>,
    /// public outputs.
    pad_source_commitment: [u32; N_COMMIT_LANES],
    opening_tuple_commitment: [u32; N_COMMIT_LANES],
    masked_projection: [u32; N_LANES],
    /// the 6 C2 mask outputs (= carrier mask inputs), propagated to row 0.
    mask_prop: [u32; N_LANES],
    /// the 6 C4 reconstructed acc values (= carrier acc inputs), propagated.
    acc_prop: [u32; N_LANES],
    trace: Vec<Vec<u32>>,
}

impl FullRelationAir {
    /// Build the honest full-relation AIR from one consistent secret set.
    pub fn new(
        pad_seed: &[u8; 32],
        slot_id: u64,
        account_projection: &[u16; N_LANES],
        blinding_nonce: &[u8; PAD_SOURCE_BLINDING_BYTES],
        lifecycle_anchor: &[u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES],
        opening_blinding_nonce: &[u8; OPENING_TUPLE_BLINDING_BYTES],
    ) -> Self {
        let params = Poseidon2Params::new();

        // --- C1 lanes: pad_seed[32] bytes + blinding_nonce[16] bytes.
        let mut c1_lanes: Vec<(u32, bool)> = Vec::new();
        for &b in pad_seed.iter() {
            c1_lanes.push((b as u32, true));
        }
        for &b in blinding_nonce.iter() {
            c1_lanes.push((b as u32, true));
        }
        let c1 = SpongeChain::new(
            &params,
            POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN,
            &c1_lanes,
            N_COMMIT_LANES,
        );

        // --- C2 lanes: pad_seed[32] bytes + slot_id[8] LE bytes.
        let mut c2_lanes: Vec<(u32, bool)> = Vec::new();
        for &b in pad_seed.iter() {
            c2_lanes.push((b as u32, true));
        }
        for &b in slot_id.to_le_bytes().iter() {
            c2_lanes.push((b as u32, true));
        }
        let c2 = SpongeChain::new(
            &params,
            POSEIDON2_PROJECTION_MASK_DOMAIN,
            &c2_lanes,
            N_MASK_OUTPUTS,
        );

        // --- C6 lanes: account_projection[6] FIELD lanes + lifecycle_anchor[32]
        //     bytes + opening_blinding_nonce[16] bytes.
        let mut c6_lanes: Vec<(u32, bool)> = Vec::new();
        for &v in account_projection.iter() {
            c6_lanes.push((v as u32, false)); // FIELD lane, no byte-range
        }
        for &b in lifecycle_anchor.iter() {
            c6_lanes.push((b as u32, true));
        }
        for &b in opening_blinding_nonce.iter() {
            c6_lanes.push((b as u32, true));
        }
        let c6 = SpongeChain::new(
            &params,
            POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN,
            &c6_lanes,
            N_COMMIT_LANES,
        );

        // --- C4 range witnesses (canonical base-128).
        let mut c4: Vec<c4::Base128RangeWitness> = Vec::with_capacity(N_LANES);
        for &v in account_projection.iter() {
            c4.push(c4::Base128RangeWitness::generate(v as u32));
        }

        // public outputs
        let mut pad_source_commitment = [0u32; N_COMMIT_LANES];
        pad_source_commitment.copy_from_slice(&c1.outputs);
        let mut opening_tuple_commitment = [0u32; N_COMMIT_LANES];
        opening_tuple_commitment.copy_from_slice(&c6.outputs);

        let mut mask_prop = [0u32; N_LANES];
        let mut acc_prop = [0u32; N_LANES];
        let mut masked_projection = [0u32; N_LANES];
        for i in 0..N_LANES {
            mask_prop[i] = c2.outputs[i];
            acc_prop[i] = c4[i].v;
            masked_projection[i] = fq_add(mask_prop[i], acc_prop[i]);
        }

        let mut air = FullRelationAir {
            params,
            c1,
            c2,
            c6,
            c4,
            pad_source_commitment,
            opening_tuple_commitment,
            masked_projection,
            mask_prop,
            acc_prop,
            trace: Vec::new(),
        };
        air.trace = air.build_trace();
        air
    }

    pub fn pad_source_commitment(&self) -> [u32; N_COMMIT_LANES] {
        self.pad_source_commitment
    }
    pub fn opening_tuple_commitment(&self) -> [u32; N_COMMIT_LANES] {
        self.opening_tuple_commitment
    }
    pub fn masked_projection(&self) -> [u32; N_LANES] {
        self.masked_projection
    }

    fn c6_perms(&self) -> usize {
        self.c6.n_perms()
    }

    // -------------------------------------------------------------------------
    // Trace generation
    // -------------------------------------------------------------------------

    fn lay_group(&self, grid: &mut [Vec<u32>], off: usize, chain: &SpongeChain) {
        for (p, st_rows) in chain.states.iter().enumerate() {
            let base = p * PERM_ROWS;
            for (local, st) in st_rows.iter().enumerate() {
                let row = &mut grid[base + local];
                row[off + STATE..off + STATE + T].copy_from_slice(st);
                let sq = sq_for_leaving(&self.params, local, st);
                row[off + SQ..off + SQ + T].copy_from_slice(&sq);
            }
            // input-row lane_val + bits.
            if let Role::Absorb { block } = &chain.roles[p] {
                let in_row = &mut grid[base];
                for i in 0..RATE {
                    let v = block_value(block[i]);
                    in_row[off + LANE_VAL + i] = v;
                    if let BlockElem::Lane { byte_ranged: true, .. } = block[i] {
                        for b in 0..BITS_PER_LANE {
                            in_row[off + bit_col_local(i, b)] = (v >> b) & 1;
                        }
                    }
                }
            }
        }
    }

    fn build_trace(&self) -> Vec<Vec<u32>> {
        let target = n_rows_padded(self.c6_perms());
        let mut grid: Vec<Vec<u32>> = vec![vec![0u32; N_COLS]; target];

        self.lay_group(&mut grid, C1_OFF, &self.c1);
        self.lay_group(&mut grid, C2_OFF, &self.c2);
        self.lay_group(&mut grid, C6_OFF, &self.c6);

        // C4 residue blocks, each on its acc_lane_row.
        for (i, w) in self.c4.iter().enumerate() {
            let row = &mut grid[acc_lane_row(i)];
            let base = C4_OFF + i * C4_CELLS;
            row[base] = w.v;
            row[base + 1] = w.d_lo;
            row[base + 2] = w.d_hi;
            row[base + 3..base + 10].copy_from_slice(&w.lo_bits);
            row[base + 10..base + 17].copy_from_slice(&w.hi_bits);
            row[base + 17..base + 24].copy_from_slice(&w.slack_bits);
            row[base + 24] = w.is96;
            row[base + 25] = w.inv;
        }

        // masked_projection (public) on row 0.
        for i in 0..N_LANES {
            grid[0][CARRY_MASKED + i] = self.masked_projection[i];
        }
        // MASK / ACC propagation columns: constant on EVERY row (transition
        // cur==nxt enforces constancy; a same-row seed residual ties them to the
        // source cell). Filling them constant makes the honest trace satisfy both.
        for row in grid.iter_mut() {
            for i in 0..N_LANES {
                row[MPROP_OFF + i] = self.mask_prop[i];
                row[APROP_OFF + i] = self.acc_prop[i];
            }
        }

        grid
    }

    // -------------------------------------------------------------------------
    // Per-group residual helpers (mirror c2_prf_air_q12289)
    // -------------------------------------------------------------------------

    /// Per-group transition residual for SOURCE global row `i`, written into
    /// `res[base..base+T]` (state) + `res[base+T..base+2T]` (sq bindings). The
    /// byte-range / deposit residuals are written into the shared aux slots passed
    /// by the caller. Returns nothing; mutates `res`.
    #[allow(clippy::too_many_arguments)]
    fn group_round_residual(
        &self,
        chain: &SpongeChain,
        off: usize,
        p: usize,
        local: usize,
        cur: &[Fq12],
        nxt: &[Fq12],
        res: &mut [Fq12],
        state_base: usize,
        deposit_base: usize,
        recompose_base: usize,
        bool_base: usize,
    ) {
        let mut c = [Fq12::zero(); T];
        let mut n = [Fq12::zero(); T];
        for k in 0..T {
            c[k] = cur[off + STATE + k];
            n[k] = nxt[off + STATE + k];
        }
        let kind = transition_kind(local);
        match kind {
            TransitionKind::InitialExternal => {
                let exp = apply_me_fq12(&self.params, &c);
                for k in 0..T {
                    res[state_base + k] = n[k].sub(&exp[k]);
                }
                // perm 0 deposit: first absorb places lane_val into rate lanes of
                // the bare initial state.
                if p == 0 {
                    if let Role::Absorb { .. } = &chain.roles[0] {
                        for i in 0..RATE {
                            res[deposit_base + i] =
                                cur[off + STATE + i].sub(&cur[off + LANE_VAL + i]);
                        }
                    }
                }
                // byte-range for this absorb perm's input row (= cur).
                byte_range_residuals(
                    &chain.roles[p],
                    off,
                    cur,
                    res,
                    recompose_base,
                    bool_base,
                );
            }
            TransitionKind::Full { rc_index } => {
                let rc = &self.params.rc_full[rc_index];
                let mut y = [Fq12::zero(); T];
                for k in 0..T {
                    let a = c[k].add(&from_base(rc[k]));
                    let sq = cur[off + SQ + k];
                    res[state_base + T + k] = sq.sub(&a.mul(&a));
                    y[k] = sq.mul(&sq).mul(&a);
                }
                let exp = apply_me_fq12(&self.params, &y);
                for k in 0..T {
                    res[state_base + k] = n[k].sub(&exp[k]);
                }
            }
            TransitionKind::Partial { rc_index } => {
                let mut y = c;
                let a = c[0].add(&from_base(self.params.rc_partial[rc_index]));
                let sq = cur[off + SQ];
                res[state_base + T] = sq.sub(&a.mul(&a));
                y[0] = sq.mul(&sq).mul(&a);
                let exp = apply_mi_fq12(&self.params, &y);
                for k in 0..T {
                    res[state_base + k] = n[k].sub(&exp[k]);
                }
            }
        }
    }

    /// Per-group LINK residual (perm p output -> perm p+1 input), into
    /// res[state_base..state_base+T].
    fn group_link_residual(
        &self,
        chain: &SpongeChain,
        off: usize,
        p: usize,
        cur: &[Fq12],
        nxt: &[Fq12],
        res: &mut [Fq12],
        state_base: usize,
    ) {
        let dest = p + 1;
        match &chain.roles[dest] {
            Role::Absorb { .. } => {
                for k in 0..T {
                    let cur_s = cur[off + STATE + k];
                    let expected = if k < RATE {
                        cur_s.add(&nxt[off + LANE_VAL + k])
                    } else {
                        cur_s
                    };
                    res[state_base + k] = nxt[off + STATE + k].sub(&expected);
                }
            }
            Role::Squeeze => {
                for k in 0..T {
                    res[state_base + k] = nxt[off + STATE + k].sub(&cur[off + STATE + k]);
                }
            }
        }
    }
}

fn from_base(v: u32) -> Fq12 {
    Fq12::from_base(v)
}

fn apply_me_fq12(params: &Poseidon2Params, v: &[Fq12; T]) -> [Fq12; T] {
    let mut out = [Fq12::zero(); T];
    for i in 0..T {
        let mut acc = Fq12::zero();
        for j in 0..T {
            acc = acc.add(&from_base(params.m_e[i][j]).mul(&v[j]));
        }
        out[i] = acc;
    }
    out
}

fn apply_mi_fq12(params: &Poseidon2Params, v: &[Fq12; T]) -> [Fq12; T] {
    let mut sum = Fq12::zero();
    for vj in v.iter() {
        sum = sum.add(vj);
    }
    let mut out = [Fq12::zero(); T];
    for k in 0..T {
        let mu_minus_1 = from_base(params.internal_diag[k]).sub(&Fq12::one());
        out[k] = sum.add(&mu_minus_1.mul(&v[k]));
    }
    out
}

/// byte-range residuals for an absorb input row, into reserved aux slots.
fn byte_range_residuals(
    role: &Role,
    off: usize,
    cur: &[Fq12],
    res: &mut [Fq12],
    recompose_base: usize,
    bool_base: usize,
) {
    if let Role::Absorb { block } = role {
        let two = Fq12::from_base(2);
        for i in 0..RATE {
            if let BlockElem::Lane { byte_ranged: true, .. } = block[i] {
                let mut acc = Fq12::zero();
                let mut w = Fq12::one();
                for b in 0..BITS_PER_LANE {
                    let bit = cur[off + bit_col_local(i, b)];
                    acc = acc.add(&bit.mul(&w));
                    w = w.mul(&two);
                    res[bool_base + i * BITS_PER_LANE + b] = bit.mul(&bit.sub(&Fq12::one()));
                }
                res[recompose_base + i] = acc.sub(&cur[off + LANE_VAL + i]);
            }
        }
    }
}

// =============================================================================
// Residual-vector slot map (one fixed-width vector; disjoint groups reuse slots)
// =============================================================================
//
// We give EACH sponge group its own state+sq region (2T), plus per-group
// deposit (RATE), byte-recompose (RATE) and byte-bool (RATE*8). They are
// disjoint so that on a given source row, multiple groups' transitions can fire
// simultaneously (different groups have constraints at the same global row).
//
// layout (per group g in {C1,C2,C6}):
//   GBASE(g)            .. +2T      : state residuals + sq bindings
//   GDEP(g)             .. +RATE    : perm-0 deposit
//   GREC(g)             .. +RATE    : byte recompose
//   GBOOL(g)            .. +RATE*8  : byte bool
// then SEAM + C4 + carrier region.

const PER_GROUP_RESID: usize = 2 * T + RATE + RATE + RATE * BITS_PER_LANE; // 24+24... = 24*2? compute below
// 2T=48, deposit RATE=5, recompose RATE=5, bool 40 => 98.

const G1_BASE: usize = 0;
const G2_BASE: usize = PER_GROUP_RESID;
const G6_BASE: usize = 2 * PER_GROUP_RESID;
const SEAM_PAD_BASE: usize = 3 * PER_GROUP_RESID; // 32 (C1<->C2)
const SEAM_ACC_BASE: usize = SEAM_PAD_BASE + PAD_SEED_LANES; // 6 (C4<->C6)
const C4_RESID_BASE: usize = SEAM_ACC_BASE + ACC_LANES; // 6 * CONSTRAINTS_PER_RESIDUE
const CARRIER_RESID_BASE: usize = C4_RESID_BASE + N_LANES * c4::CONSTRAINTS_PER_RESIDUE; // 6
/// MASK propagation: constancy (cur==nxt) on every row.
const MPROP_CONST_BASE: usize = CARRIER_RESID_BASE + N_LANES; // 6
/// MASK propagation: seed (prop == C2 squeezed output) on each squeeze row.
const MPROP_SEED_BASE: usize = MPROP_CONST_BASE + N_LANES; // 6
/// ACC propagation: constancy on every row.
const APROP_CONST_BASE: usize = MPROP_SEED_BASE + N_LANES; // 6
/// ACC propagation: seed (prop == C4 v) on each C4 row.
const APROP_SEED_BASE: usize = APROP_CONST_BASE + N_LANES; // 6
const RESID_WIDTH: usize = APROP_SEED_BASE + N_LANES;

// per-group slot offsets within its region.
const R_STATE: usize = 0; // ..2T
const R_DEP: usize = 2 * T; // ..RATE
const R_REC: usize = 2 * T + RATE; // ..RATE
const R_BOOL: usize = 2 * T + 2 * RATE; // ..RATE*8

fn group_base(which: usize) -> usize {
    match which {
        0 => G1_BASE,
        1 => G2_BASE,
        _ => G6_BASE,
    }
}

impl FullRelationAir {
    fn group_transition(
        &self,
        which: usize,
        off: usize,
        chain: &SpongeChain,
        i: usize,
        cur: &[Fq12],
        nxt: &[Fq12],
        res: &mut [Fq12],
    ) {
        let np = chain.n_perms();
        if i >= np * PERM_ROWS {
            return;
        }
        let p = i / PERM_ROWS;
        let local = i % PERM_ROWS;
        let gb = group_base(which);
        if local < PERM_ROWS - 1 {
            self.group_round_residual(
                chain,
                off,
                p,
                local,
                cur,
                nxt,
                res,
                gb + R_STATE,
                gb + R_DEP,
                gb + R_REC,
                gb + R_BOOL,
            );
        } else if p + 1 < np {
            self.group_link_residual(chain, off, p, cur, nxt, res, gb + R_STATE);
        }
    }
}

// =============================================================================
// Air impl
// =============================================================================

impl Air for FullRelationAir {
    fn n_cols(&self) -> usize {
        N_COLS
    }
    fn n_rows(&self) -> usize {
        n_rows_padded(self.c6_perms())
    }
    fn max_degree(&self) -> usize {
        3
    }

    fn generate_trace(&self) -> Vec<Vec<u32>> {
        self.trace.clone()
    }

    fn boundary_constraints(&self) -> Vec<BoundaryConstraint> {
        let mut out = Vec::new();

        // --- per-group capacity init + lane-const pins + output binds.
        let groups: [(usize, &SpongeChain, Option<&[u32; N_COMMIT_LANES]>); 3] = [
            (C1_OFF, &self.c1, Some(&self.pad_source_commitment)),
            (C2_OFF, &self.c2, None),
            (C6_OFF, &self.c6, Some(&self.opening_tuple_commitment)),
        ];
        for &(off, chain, commit) in groups.iter() {
            // capacity init: perm 0 input row, capacity lanes == domain const.
            let init_state = init_capacity(chain.domain);
            for lane in RATE..T {
                out.push(BoundaryConstraint {
                    row: 0,
                    col: off + STATE + lane,
                    value: init_state[lane],
                });
            }
            // lane-const pins: const positions; squeeze perms pin lane_val to 0.
            for (p, role) in chain.roles.iter().enumerate() {
                let in_row = p * PERM_ROWS;
                match role {
                    Role::Absorb { block } => {
                        for i in 0..RATE {
                            if let BlockElem::Const(c) = block[i] {
                                out.push(BoundaryConstraint {
                                    row: in_row,
                                    col: off + LANE_VAL + i,
                                    value: fq_from_u32(c),
                                });
                            }
                        }
                    }
                    Role::Squeeze => {
                        for i in 0..RATE {
                            out.push(BoundaryConstraint {
                                row: in_row,
                                col: off + LANE_VAL + i,
                                value: 0,
                            });
                        }
                    }
                }
            }
            // commitment output binds (C1/C6 only — public commitments). C2's
            // mask output is NOT public; it is tied to the carrier via the MPROP
            // seed transition residual instead.
            if let Some(commit) = commit {
                push_output_binds(&mut out, off, chain, &commit[..]);
            }
        }

        // --- carrier row-0 bind: masked == public masked_projection.
        for i in 0..N_LANES {
            out.push(BoundaryConstraint {
                row: 0,
                col: CARRY_MASKED + i,
                value: self.masked_projection[i],
            });
        }

        out
    }

    fn transition_rows(&self) -> Vec<usize> {
        // EVERY consecutive pair must be constrained: the per-group sponge
        // residuals fire only on that group's real rows (others evaluate to 0 via
        // `group_transition`'s range guard), but the MASK/ACC propagation
        // constancy residual must hold on EVERY row so the constant copy cannot
        // jump anywhere. So we constrain the full range 0..n_rows-1.
        (0..self.n_rows() - 1).collect()
    }

    fn n_transition_residuals(&self) -> usize {
        RESID_WIDTH
    }

    fn transition_residuals(&self, i: usize, cur: &[Fq12], nxt: &[Fq12]) -> Vec<Fq12> {
        let mut res = vec![Fq12::zero(); RESID_WIDTH];

        // each sponge group's transition (disjoint regions).
        self.group_transition(0, C1_OFF, &self.c1, i, cur, nxt, &mut res);
        self.group_transition(1, C2_OFF, &self.c2, i, cur, nxt, &mut res);
        self.group_transition(2, C6_OFF, &self.c6, i, cur, nxt, &mut res);

        // --- SeamPadSeed (C1 == C2): pad_seed byte b lives at row (b/5)*70, lane
        //     b%5. Emit the equality on the row where it lives.
        for b in 0..PAD_SEED_LANES {
            if acc_lane_row_for_pad(b) == i {
                let lane = b % RATE;
                res[SEAM_PAD_BASE + b] =
                    cur[C1_OFF + LANE_VAL + lane].sub(&cur[C2_OFF + LANE_VAL + lane]);
            }
        }

        // --- SeamOpeningAcc (C4 == C6): residue lane j at row acc_lane_row(j).
        for j in 0..ACC_LANES {
            if acc_lane_row(j) == i {
                let c4v = cur[C4_OFF + j * C4_CELLS]; // v cell
                let c6cell = cur[c6_field_lane_col(j)];
                res[SEAM_ACC_BASE + j] = c4v.sub(&c6cell);
            }
        }

        // --- C4 canonical residuals: residue j on its row.
        for j in 0..N_LANES {
            if acc_lane_row(j) == i {
                let base = C4_OFF + j * C4_CELLS;
                c4_residuals_fq12(cur, base, &mut res, C4_RESID_BASE + j * c4::CONSTRAINTS_PER_RESIDUE);
            }
        }

        // --- MASK / ACC propagation columns: constancy on EVERY transition row.
        //     cur[prop] - nxt[prop] = 0 makes the column constant down the trace.
        let np_rows = self.n_rows();
        if i + 1 < np_rows {
            for j in 0..N_LANES {
                res[MPROP_CONST_BASE + j] = cur[MPROP_OFF + j].sub(&nxt[MPROP_OFF + j]);
                res[APROP_CONST_BASE + j] = cur[APROP_OFF + j].sub(&nxt[APROP_OFF + j]);
            }
        }

        // --- MASK seed: on the C2 squeeze row holding mask lane j, bind the
        //     (constant) MPROP column to the C2 squeezed state cell. This IS the
        //     SeamMask binding (carrier mask == C2 PRF output), same-row.
        for j in 0..N_LANES {
            let (mrow, mlane) = self.c2.output_row_lane(j);
            if mrow == i {
                res[MPROP_SEED_BASE + j] =
                    cur[MPROP_OFF + j].sub(&cur[C2_OFF + STATE + mlane]);
            }
        }

        // --- ACC seed: on the C4 residue row for lane j, bind the (constant)
        //     APROP column to the C4 reconstructed value `v`. This IS the carrier
        //     SeamAcc binding (carrier acc == C4 reconstruction), same-row.
        for j in 0..N_LANES {
            if acc_lane_row(j) == i {
                res[APROP_SEED_BASE + j] = cur[APROP_OFF + j].sub(&cur[C4_OFF + j * C4_CELLS]);
            }
        }

        // --- carrier: masked - mask - acc == 0 on row 0, reading the propagated
        //     mask (= C2 output) and acc (= C4 reconstruction). masked is the
        //     row-0 public-bound cell.
        if i == 0 {
            for j in 0..N_LANES {
                let masked = cur[CARRY_MASKED + j];
                let mask = cur[MPROP_OFF + j];
                let acc = cur[APROP_OFF + j];
                res[CARRIER_RESID_BASE + j] = masked.sub(&mask).sub(&acc);
            }
        }

        res
    }
}

/// pad_seed byte b lives at perm b/RATE input row.
fn acc_lane_row_for_pad(b: usize) -> usize {
    (b / RATE) * PERM_ROWS
}

/// Push the output-bind boundary constraints for a sponge group: each of the
/// `committed.len()` output lanes equals a squeezed state cell.
fn push_output_binds(
    out: &mut Vec<BoundaryConstraint>,
    off: usize,
    chain: &SpongeChain,
    committed: &[u32],
) {
    let mut filled = 0usize;
    let mut read_perm = chain.n_absorb - 1;
    let mut remaining = chain.n_out;
    loop {
        let take = remaining.min(RATE);
        let out_row = read_perm * PERM_ROWS + (PERM_ROWS - 1);
        for i in 0..take {
            out.push(BoundaryConstraint {
                row: out_row,
                col: off + STATE + i,
                value: committed[filled],
            });
            filled += 1;
        }
        remaining -= take;
        if remaining == 0 {
            break;
        }
        read_perm += 1;
    }
}

/// Evaluate the C4 canonical base-128 residuals over Fq12 on the residue cells
/// at `cur[base..base+26]`, writing CONSTRAINTS_PER_RESIDUE residuals starting at
/// `res[rbase]`. Mirrors base128_range_air_q12289::residuals exactly (deg <= 2).
fn c4_residuals_fq12(cur: &[Fq12], base: usize, res: &mut [Fq12], rbase: usize) {
    let v = cur[base];
    let d_lo = cur[base + 1];
    let d_hi = cur[base + 2];
    let lo_bits: Vec<Fq12> = (0..7).map(|k| cur[base + 3 + k]).collect();
    let hi_bits: Vec<Fq12> = (0..7).map(|k| cur[base + 10 + k]).collect();
    let slack_bits: Vec<Fq12> = (0..7).map(|k| cur[base + 17 + k]).collect();
    let is96 = cur[base + 24];
    let inv = cur[base + 25];

    let one = Fq12::one();
    let two = Fq12::from_base(2);
    let max_hi = Fq12::from_base(c4::MAX_HI);
    let base128 = Fq12::from_base(c4::BASE);

    let boolean = |b: &Fq12| -> Fq12 { b.mul(&b.sub(&one)) };
    let recompose = |bits: &[Fq12]| -> Fq12 {
        let mut acc = Fq12::zero();
        let mut w = one;
        for b in bits {
            acc = acc.add(&b.mul(&w));
            w = w.mul(&two);
        }
        acc
    };

    let mut idx = rbase;
    for b in &lo_bits {
        res[idx] = boolean(b);
        idx += 1;
    }
    for b in &hi_bits {
        res[idx] = boolean(b);
        idx += 1;
    }
    for b in &slack_bits {
        res[idx] = boolean(b);
        idx += 1;
    }
    // d_lo recompose
    res[idx] = d_lo.sub(&recompose(&lo_bits));
    idx += 1;
    // d_hi recompose
    res[idx] = d_hi.sub(&recompose(&hi_bits));
    idx += 1;
    // slack recompose: (96 - d_hi) - sum == 0
    res[idx] = max_hi.sub(&d_hi).sub(&recompose(&slack_bits));
    idx += 1;
    // is96 booleanity
    res[idx] = boolean(&is96);
    idx += 1;
    // is96 * (d_hi - 96) == 0
    res[idx] = is96.mul(&d_hi.sub(&max_hi));
    idx += 1;
    // (d_hi - 96) * inv == 1 - is96
    res[idx] = d_hi.sub(&max_hi).mul(&inv).sub(&one.sub(&is96));
    idx += 1;
    // is96 * d_lo == 0
    res[idx] = is96.mul(&d_lo);
    idx += 1;
    // reconstruction: v - (d_lo + 128*d_hi) == 0
    res[idx] = v.sub(&d_lo.add(&base128.mul(&d_hi)));
    idx += 1;
    debug_assert_eq!(idx - rbase, c4::CONSTRAINTS_PER_RESIDUE);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_q12289::{
        opening_tuple_commitment_poseidon2, pad_source_commitment_poseidon2,
        projection_masks_poseidon2,
    };
    use crate::stark_q12289::{choose_domains, prove_with_queries, verify};

    fn lift_row(row: &[u32]) -> Vec<Fq12> {
        row.iter().map(|&v| Fq12::from_base(v)).collect()
    }

    const TEST_NONCE: [u8; PAD_SOURCE_BLINDING_BYTES] = [9u8; PAD_SOURCE_BLINDING_BYTES];
    const TEST_ANCHOR: [u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES] =
        [13u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES];
    const TEST_OPENING_NONCE: [u8; OPENING_TUPLE_BLINDING_BYTES] =
        [15u8; OPENING_TUPLE_BLINDING_BYTES];

    fn mk(pad: &[u8; 32], slot: u64, acc: &[u16; N_LANES]) -> FullRelationAir {
        FullRelationAir::new(pad, slot, acc, &TEST_NONCE, &TEST_ANCHOR, &TEST_OPENING_NONCE)
    }

    /// CHEAP correctness: every transition residual zero on every real row, and
    /// every boundary holds, when the honest trace is lifted to Fq12.
    fn air_satisfied(air: &FullRelationAir) -> Result<(), String> {
        let trace = air.generate_trace();
        for i in air.transition_rows() {
            let cur = lift_row(&trace[i]);
            let nxt = lift_row(&trace[i + 1]);
            let res = air.transition_residuals(i, &cur, &nxt);
            for (k, r) in res.iter().enumerate() {
                if !r.is_zero() {
                    return Err(format!("transition residual nonzero at row {i} slot {k}"));
                }
            }
        }
        for b in air.boundary_constraints() {
            let got = trace[b.row][b.col];
            let want = b.value % crate::poseidon2_q12289::Q;
            if got != want {
                return Err(format!(
                    "boundary mismatch row {} col {}: got {got} want {want}",
                    b.row, b.col
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn full_relation_layout_and_domain_report() {
        let air = mk(&[3u8; 32], 42, &[1, 2, 3, 4, 5, 6]);
        let d = choose_domains(&air);
        println!(
            "FULL RELATION AIR: n_rows={} n_cols={} max_deg={} trace_n={} fri_n={} \
             blowup=2^{} rate=1/{} boundary={} transition_rows={} resid_width={}",
            air.n_rows(),
            air.n_cols(),
            air.max_degree(),
            d.trace_n,
            d.fri_n,
            d.log2_fri - d.log2_trace,
            1usize << (d.log2_fri - d.log2_trace),
            air.boundary_constraints().len(),
            air.transition_rows().len(),
            air.n_transition_residuals(),
        );
        println!(
            "  column groups: C1@{C1_OFF} C2@{C2_OFF} C6@{C6_OFF} C4@{C4_OFF} carrier@{CARRY_OFF} (PG={PG})"
        );
        assert_eq!(air.n_cols(), N_COLS);
        assert_eq!(air.max_degree(), 3);
        assert_eq!(d.trace_n, 1024, "980 rows padded to 1024");
        assert!(d.log2_fri - d.log2_trace >= 2, "rate <= 1/4");
    }

    #[test]
    fn full_relation_honest_trace_satisfies_air_and_outputs_match_references() {
        let mut seed = 0xA11C_E0DE_1234_5678u64;
        let xs = |s: &mut u64| {
            *s ^= *s << 13;
            *s ^= *s >> 7;
            *s ^= *s << 17;
            *s
        };
        for _ in 0..6 {
            let mut pad = [0u8; 32];
            for b in pad.iter_mut() {
                *b = (xs(&mut seed) & 0xff) as u8;
            }
            let slot = xs(&mut seed);
            let mut acc = [0u16; N_LANES];
            for a in acc.iter_mut() {
                *a = (xs(&mut seed) % crate::poseidon2_q12289::Q as u64) as u16;
            }
            let air = mk(&pad, slot, &acc);
            air_satisfied(&air).expect("honest full-relation trace must satisfy the AIR");

            // outputs match references.
            let ref_commit = pad_source_commitment_poseidon2(&pad, &TEST_NONCE);
            for j in 0..N_COMMIT_LANES {
                assert_eq!(air.pad_source_commitment()[j] as u16, ref_commit[j], "C1 lane {j}");
            }
            let ref_open = opening_tuple_commitment_poseidon2(&acc, &TEST_ANCHOR, &TEST_OPENING_NONCE);
            for j in 0..N_COMMIT_LANES {
                assert_eq!(air.opening_tuple_commitment()[j] as u16, ref_open[j], "C6 lane {j}");
            }
            let ref_mask = projection_masks_poseidon2(&pad, slot);
            for j in 0..N_LANES {
                let expect = fq_add(ref_mask[j] as u32, acc[j] as u32);
                assert_eq!(air.masked_projection()[j], expect, "masked lane {j}");
            }
        }
    }

    /// COMPLETENESS: honest secrets -> STARK proof VERIFIES.
    #[test]
    fn full_relation_stark_completeness_verifies() {
        let air = mk(&[7u8; 32], 123456789, &[10, 20, 30, 40, 50, 60]);
        let proof = prove_with_queries(&air, None, 16);
        assert!(verify(&air, &proof), "honest full-relation STARK proof must verify");
    }

    /// SOUNDNESS (a): tamper a C1/C2/C6 sponge cell -> verify FAILS.
    #[test]
    fn full_relation_stark_soundness_component_cell() {
        use crate::poseidon2_q12289::fq_add;
        let air = mk(&[9u8; 32], 99, &[1, 2, 3, 4, 5, 6]);
        let honest = air.generate_trace();
        // C1 mid-perm cell.
        let tampers: &[(usize, usize, &str)] = &[
            (5 * PERM_ROWS + 30, C1_OFF + STATE + 9, "C1 mid-perm cell"),
            (4 * PERM_ROWS + 3, C2_OFF + STATE + 7, "C2 full-round cell"),
            (5 * PERM_ROWS + 30, C6_OFF + STATE + 9, "C6 mid-perm cell"),
        ];
        for &(row, col, what) in tampers {
            let mut t = honest.clone();
            t[row][col] = fq_add(t[row][col], 1);
            let proof = prove_with_queries(&air, Some(t), 16);
            assert!(!verify(&air, &proof), "component tamper [{what}] not rejected");
        }
    }

    /// SOUNDNESS (b): break the C1<->C2 pad_seed seam -> verify FAILS.
    #[test]
    fn full_relation_stark_soundness_seam() {
        use crate::poseidon2_q12289::fq_add;
        let air = mk(&[5u8; 32], 8, &[1, 2, 3, 4, 5, 6]);
        let honest = air.generate_trace();
        // pad_seed byte 7 lives at perm 1 (7/5=1) input row, lane 2 (7%5). Tamper
        // ONLY C1's lane_val there (the value the commitment absorbs), so C1's
        // pad_seed differs from C2's -> SeamPadSeed fires. NOTE: also breaks C1's
        // own absorb deposit/chain, which equally proves the binding is enforced.
        let b = 7usize;
        let row = (b / RATE) * PERM_ROWS;
        let col = C1_OFF + LANE_VAL + (b % RATE);
        let mut t = honest.clone();
        t[row][col] = fq_add(t[row][col], 1);
        let proof = prove_with_queries(&air, Some(t), 16);
        assert!(!verify(&air, &proof), "pad_seed seam tamper not rejected");
    }

    /// SOUNDNESS (c): a wrong public commitment -> verify FAILS.
    #[test]
    fn full_relation_stark_soundness_wrong_public_commitment() {
        use crate::poseidon2_q12289::fq_add;
        let mut air = mk(&[19u8; 32], 4, &[7, 8, 9, 10, 11, 12]);
        // mutate the claimed public pad_source_commitment -> output boundary fails.
        air.pad_source_commitment[6] = fq_add(air.pad_source_commitment[6], 1);
        // rebuild trace so the carrier/other parts stay consistent; only the
        // commitment boundary now mismatches the squeezed cell.
        let proof = prove_with_queries(&air, None, 16);
        assert!(!verify(&air, &proof), "wrong public pad_source_commitment not rejected");
    }

    /// SOUNDNESS (d): a C4 non-canonical digit violation -> verify FAILS.
    #[test]
    fn full_relation_stark_soundness_c4_noncanonical() {
        use crate::poseidon2_q12289::fq_add;
        let air = mk(&[13u8; 32], 5, &[5, 6, 7, 8, 9, 10]);
        let honest = air.generate_trace();
        // Lane 0 residue v=5 lives on row 0. Overwrite its digits to the classic
        // non-canonical rep d_hi=96, d_lo=6 (field value 5) -> C4 edge fires.
        let mut t = honest.clone();
        let base = C4_OFF + 0 * C4_CELLS;
        let bits = |x: u32, n: usize| -> Vec<u32> { (0..n).map(|k| (x >> k) & 1).collect() };
        t[0][base + 1] = 6; // d_lo
        t[0][base + 2] = 96; // d_hi
        for (k, v) in bits(6, 7).into_iter().enumerate() {
            t[0][base + 3 + k] = v;
        }
        for (k, v) in bits(96, 7).into_iter().enumerate() {
            t[0][base + 10 + k] = v;
        }
        for (k, v) in bits(0, 7).into_iter().enumerate() {
            t[0][base + 17 + k] = v;
        }
        t[0][base + 24] = 1; // is96
        t[0][base + 25] = 0; // inv
        let _ = fq_add(0, 0);
        let proof = prove_with_queries(&air, Some(t), 16);
        assert!(!verify(&air, &proof), "C4 non-canonical digit not rejected");
    }

    /// SOUNDNESS (e): break the C4<->C6 account_projection seam -> verify FAILS.
    #[test]
    fn full_relation_stark_soundness_acc_seam() {
        use crate::poseidon2_q12289::fq_add;
        let air = mk(&[31u8; 32], 3, &[10, 20, 30, 40, 50, 60]);
        let honest = air.generate_trace();
        // C6 field lane 2 lives at row 0, lane_val[2]. Tamper ONLY it so C6's
        // absorbed account_projection differs from C4's reconstruction.
        let mut t = honest.clone();
        let col = c6_field_lane_col(2);
        t[acc_lane_row(2)][col] = fq_add(t[acc_lane_row(2)][col], 1);
        let proof = prove_with_queries(&air, Some(t), 16);
        assert!(!verify(&air, &proof), "account_projection seam tamper not rejected");
    }

    /// SOUNDNESS (f): a wrong public masked_projection -> verify FAILS.
    #[test]
    fn full_relation_stark_soundness_wrong_masked() {
        use crate::poseidon2_q12289::fq_add;
        let mut air = mk(&[21u8; 32], 42, &[1, 2, 3, 4, 5, 6]);
        air.masked_projection[0] = fq_add(air.masked_projection[0], 7);
        air.trace = air.build_trace(); // re-lay row-0 masked cell to the forged value
        // now the carrier residual fires: masked != mask + acc.
        let proof = prove_with_queries(&air, None, 16);
        assert!(!verify(&air, &proof), "forged masked_projection not rejected");
    }

    /// SIZE + TIMING report (ignored: multi-second proving at the 16384 domain).
    #[test]
    #[ignore]
    fn full_relation_stark_size_and_timing_report() {
        use std::time::Instant;
        let air = mk(&[5u8; 32], 7, &[1, 2, 3, 4, 5, 6]);
        let d = choose_domains(&air);
        let t0 = Instant::now();
        let proof = prove_with_queries(&air, None, 16);
        let prove_ms = t0.elapsed().as_millis();
        let t1 = Instant::now();
        let ok = verify(&air, &proof);
        let verify_ms = t1.elapsed().as_millis();
        assert!(ok);

        let fq12_bytes = 12 * 2;
        let mut n_fq12 = proof.ood_cur.len() + proof.ood_nxt.len() + 1;
        let mut n_pathnode = 0usize;
        n_fq12 += proof.fri.final_layer.len();
        for q in &proof.fri.queries {
            for l in &q.layers {
                n_fq12 += 2;
                n_pathnode += l.value_path.siblings.len() + l.sibling_path.siblings.len();
            }
        }
        for op in &proof.openings {
            n_fq12 += op.cur.len() + op.nxt.len() + 1;
            n_pathnode +=
                op.cur_path.siblings.len() + op.nxt_path.siblings.len() + op.h_path.siblings.len();
        }
        let digest_bytes = proof.trace_root.len() * 2;
        let bytes = n_fq12 * fq12_bytes + n_pathnode * digest_bytes;
        println!(
            "FULL RELATION STARK timing/size: prove={prove_ms}ms verify={verify_ms}ms \
             n_rows={} n_cols={} fri_n={} fq12_vals={n_fq12} path_nodes={n_pathnode} \
             ~bytes={bytes} (~{} KiB) num_queries=16",
            air.n_rows(),
            air.n_cols(),
            d.fri_n,
            bytes / 1024,
        );
    }
}
