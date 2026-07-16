//! C2 relation component as a generic [`Air`]: the projection-mask sponge PRF
//! `projection_masks_poseidon2(pad_seed, slot_id)`, proven end-to-end by the
//! AIR-agnostic DEEP-ALI STARK (`crate::stark_q12289`).
//!
//! This is the FIRST REAL relation-component STARK in the crate. It proves, over
//! a committed execution trace, that the 6 committed mask lanes ARE the Poseidon2
//! sponge-PRF outputs of a HIDDEN `pad_seed` (32 bytes) and `slot_id` (u64 LE),
//! with the hidden input byte-range-checked. It is obligation **C2** of the
//! account-creation relation.
//!
//! It REUSES the arithmetic of `crate::poseidon2_q12289` (matrices, round
//! constants, S-box) and the sponge block-plan of
//! `crate::poseidon2_prf_air_q12289` (n_absorb / n_squeeze / block layout). The
//! S-box is DEGREE-REDUCED from x^5 to degree 3 with one aux "square" column per
//! lane, so the composition degree of the 10-permutation chain (700 rows padded
//! to 1024) is 3*1023 - 1 = 3068 < 16384/4 = 4096 and the proof fits a
//! 16384-point Fq12 FRI domain. (At the native degree 5 the composition is
//! 5*1023 - 1 = 5114 > 4096 and would NOT fit the field's 2-adicity.)
//!
//! =============================================================================
//! TRACE LAYOUT  (n_rows = 1024, n_cols = 93)
//! =============================================================================
//!
//! Rows 0..700 hold the 10 permutation sub-traces stacked: permutation p occupies
//! global rows `p*70 .. p*70 + 70` (row 0 = perm input, row 69 = perm output).
//! Rows 700..1024 are PADDING (zero); no constraint references them.
//!
//! Columns:
//!   cols   0.. 24 : the t = 24 state lanes `x[k]`.
//!   cols  24.. 48 : S-box square aux `sq[k] = a[k]^2`, where `a[k]` is the value
//!                   fed to the k-th S-box of the transition LEAVING this row
//!                   (full round: a[k] = x[k]+rc[k] all k; partial: only a[0],
//!                   sq[1..]=0; non-S-boxing source rows: sq=0). Lets x^5 = sq^2*a
//!                   be degree 3, with the degree-2 binding sq[k] - a[k]^2 = 0.
//!   cols  48.. 53 : `lane_val[i]` (i in 0..RATE) — the field value ABSORBED into
//!                   rate lane i of THIS row, when this row is a permutation INPUT
//!                   row. For a hidden input byte-lane it is the (byte) witness;
//!                   for a sponge constant (pad marker 1 / 0 padding) it is pinned
//!                   to that public constant by a boundary constraint. Zero on
//!                   non-input rows.
//!   cols  53.. 93 : `bits[i][b]` (i in 0..RATE, b in 0..8) — the 8-bit LE
//!                   decomposition of `lane_val[i]` on permutation INPUT rows that
//!                   absorb a HIDDEN input byte-lane at rate position i. Zero
//!                   elsewhere.
//!
//! =============================================================================
//! CONSTRAINTS  (through the generic Air interface; max degree 3)
//! =============================================================================
//!
//! Encoded as BOUNDARY point constraints (`trace[row][col] == const`) or per-row
//! TRANSITION residuals keyed by the SOURCE row i (a residual reads `cur` only
//! for single-row constraints, or `cur` and `nxt` for row-pair constraints):
//!
//!  1. PER-PERMUTATION ROUND (transition; source row strictly inside a block):
//!     degree-reduced Poseidon2 round residual cur.state -> nxt.state, plus the
//!     aux-square binding sq[k] - a[k]^2 for the S-boxed lanes. Degree 3.
//!  2. INITIAL-EXTERNAL (transition; block row 0 -> row 1): nxt.state = M_E*cur.
//!     Degree 1.
//!  3. ABSORB / SQUEEZE LINK (transition; perm p output row -> perm p+1 input):
//!       absorb : nxt.state[i] = cur.state[i] + nxt.lane_val[i]  (i < RATE)
//!                nxt.state[i] = cur.state[i]                    (i >= RATE)
//!       squeeze: nxt.state[i] = cur.state[i]  for all i
//!     Degree 1. The absorbed value comes from the DEST row's own lane_val cells.
//!  4. CAPACITY INIT (boundary): perm 0's input-row capacity lanes (RATE..T) equal
//!     the public domain constant; and perm 0's input-row state RATE lanes equal
//!     lane_val[i] (the initial running state has 0 in the rate lanes, so the
//!     first absorb just deposits the block) — pinned via the p=0 absorb link with
//!     cur = the public initial state. Degree 1.
//!  5. BYTE RANGE (transition, single-row, on absorb input rows carrying hidden
//!     lanes): bit boolean bits[i][b]*(bits[i][b]-1)=0 (degree 2) and recompose
//!     sum_b bits[i][b]*2^b - lane_val[i] = 0 (degree 1).
//!  6. LANE-CONST PIN (boundary): rate positions holding a sponge CONSTANT pin
//!     lane_val[i] to that constant. Degree 1 (point).
//!  7. MASK OUTPUT (boundary): the 6 committed masks equal the squeezed state
//!     cells (first squeeze: perm (n_absorb-1) output lanes 0..5; second squeeze:
//!     perm n_absorb output lane 0). Degree 1 (point).
//!
//! MAX DEGREE = 3.

use crate::air::{Air, BoundaryConstraint};
use crate::fq12_q12289::Fq12;
use crate::poseidon2_air_q12289::{N_ROWS as PERM_ROWS, TransitionKind, transition_kind};
use crate::poseidon2_prf_air_q12289::{
    BITS_PER_LANE, BlockElem, N_INPUT_LANES, N_MASK_OUTPUTS, PermRole, build_plan, n_absorb_perms,
    n_perms,
};
use crate::poseidon2_q12289::{
    POSEIDON2_PROJECTION_MASK_DOMAIN, Poseidon2Params, R_F_HALF, R_P, RATE, T, apply_internal,
    apply_matrix, fq_add, fq_from_u32, fq_mul, init_capacity, projection_masks_poseidon2, sbox,
};

// Column layout.
const STATE: usize = 0; // 0..T
const SQ: usize = T; // T..2T
const LANE_VAL: usize = 2 * T; // 2T .. 2T+RATE
const BITS: usize = 2 * T + RATE; // .. +RATE*8
/// Total trace columns.
pub const N_COLS: usize = 2 * T + RATE + RATE * BITS_PER_LANE; // 48 + 5 + 40 = 93

const fn bit_col(i: usize, b: usize) -> usize {
    BITS + i * BITS_PER_LANE + b
}

// Transition residual slot map (all groups share one fixed-width vector; each
// SOURCE row is exactly one group, so disjoint groups may reuse slot ranges as
// long as the WIDTH covers the widest group).
//   [0 .. T)                        : state residuals (every group)
//   [T .. 2T)                       : Full/Partial aux-square sq bindings
//   [BR_DEPOSIT .. +RATE)           : perm-0 input rate-deposit
//   [BR_RECOMPOSE .. +RATE)         : byte-range recompose (per rate input-lane)
//   [BR_BOOL .. +RATE*8)            : byte-range bit-boolean
const BR_DEPOSIT: usize = T; // T .. T+RATE
const BR_RECOMPOSE: usize = T + RATE; // .. +RATE
const BR_BOOL: usize = T + 2 * RATE; // .. +RATE*8
/// Transition residual vector width.
const RESID_WIDTH: usize = T + 2 * RATE + RATE * BITS_PER_LANE; // 24+10+40 = 74
// Sanity: the round-Full group (2T sq bindings) also fits.
const _: () = assert!(RESID_WIDTH >= 2 * T);

/// Padded trace rows (10 perms * 70 = 700, padded to next power of two = 1024).
pub fn n_rows_padded() -> usize {
    let real = n_perms() * PERM_ROWS;
    let mut n = 1usize;
    while n < real {
        n <<= 1;
    }
    n
}

fn perm_input_row(p: usize) -> usize {
    p * PERM_ROWS
}
fn perm_output_row(p: usize) -> usize {
    p * PERM_ROWS + (PERM_ROWS - 1)
}

/// The C2 PRF chain AIR over a fixed (pad_seed, slot_id).
pub struct C2PrfAir {
    params: Poseidon2Params,
    masks: [u32; N_MASK_OUTPUTS],
    trace: Vec<Vec<u32>>,
}

impl C2PrfAir {
    pub fn new(pad_seed: &[u8; 32], slot_id: u64) -> Self {
        let params = Poseidon2Params::new();
        let ref_masks = projection_masks_poseidon2(pad_seed, slot_id);
        let mut masks = [0u32; N_MASK_OUTPUTS];
        for j in 0..N_MASK_OUTPUTS {
            masks[j] = ref_masks[j] as u32;
        }
        let trace = build_trace(&params, pad_seed, slot_id);
        C2PrfAir {
            params,
            masks,
            trace,
        }
    }

    pub fn masks(&self) -> [u32; N_MASK_OUTPUTS] {
        self.masks
    }
}

// -----------------------------------------------------------------------------
// Honest trace generation
// -----------------------------------------------------------------------------

fn encode_input_lanes(pad_seed: &[u8; 32], slot_id: u64) -> [u32; N_INPUT_LANES] {
    let mut lanes = [0u32; N_INPUT_LANES];
    for (i, &b) in pad_seed.iter().enumerate() {
        lanes[i] = b as u32;
    }
    for (i, &b) in slot_id.to_le_bytes().iter().enumerate() {
        lanes[32 + i] = b as u32;
    }
    lanes
}

/// The aux squares for the S-box of the transition LEAVING local row `local`.
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

/// Generate one permutation's 70 state rows (state + sq columns), from
/// `input_state`. Mirrors `permute` layer by layer.
fn gen_perm_state_rows(params: &Poseidon2Params, input_state: &[u32; T]) -> Vec<[u32; T]> {
    let mut states: Vec<[u32; T]> = Vec::with_capacity(PERM_ROWS);
    let mut state = *input_state;
    for s in state.iter_mut() {
        *s = fq_from_u32(*s);
    }
    states.push(state); // row 0 input
    apply_matrix(&params.m_e, &mut state);
    states.push(state); // row 1 after initial M_E
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

fn build_trace(params: &Poseidon2Params, pad_seed: &[u8; 32], slot_id: u64) -> Vec<Vec<u32>> {
    let plan = build_plan();
    let input_lanes = encode_input_lanes(pad_seed, slot_id);
    let target = n_rows_padded();
    let mut grid: Vec<[u32; N_COLS]> = vec![[0u32; N_COLS]; target];

    // running sponge state.
    let mut run = init_capacity(POSEIDON2_PROJECTION_MASK_DOMAIN);

    for (p, role) in plan.roles.iter().enumerate() {
        // Determine the lane_val deposited at this perm's input (rate lanes).
        let mut lane_val = [0u32; RATE];
        if let PermRole::Absorb { block } = role {
            for i in 0..RATE {
                lane_val[i] = match block[i] {
                    BlockElem::InputLane(idx) => fq_from_u32(input_lanes[idx]),
                    BlockElem::Const(c) => fq_from_u32(c),
                };
            }
        }
        // Apply the linking op to produce this perm's input state.
        match role {
            PermRole::Absorb { .. } => {
                for i in 0..RATE {
                    run[i] = fq_add(run[i], lane_val[i]);
                }
            }
            PermRole::Squeeze => {}
        }

        let states = gen_perm_state_rows(params, &run);
        let base = perm_input_row(p);
        for (local, st) in states.iter().enumerate() {
            let row = &mut grid[base + local];
            row[STATE..STATE + T].copy_from_slice(st);
            let sq = sq_for_leaving(params, local, st);
            row[SQ..SQ + T].copy_from_slice(&sq);
        }
        // Input-row aux: lane_val + bits for hidden byte-lanes.
        {
            let in_row = &mut grid[base];
            for i in 0..RATE {
                in_row[LANE_VAL + i] = lane_val[i];
            }
            if let PermRole::Absorb { block } = role {
                for i in 0..RATE {
                    if let BlockElem::InputLane(_) = block[i] {
                        let v = lane_val[i];
                        for b in 0..BITS_PER_LANE {
                            in_row[bit_col(i, b)] = (v >> b) & 1;
                        }
                    }
                }
            }
        }
        run = states[PERM_ROWS - 1];
        let _ = p;
    }

    grid.into_iter().map(|r| r.to_vec()).collect()
}

// -----------------------------------------------------------------------------
// Transition residual evaluator (over Fq12)
// -----------------------------------------------------------------------------

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

/// What kind of transition the SOURCE global row `i` governs.
enum RowKind {
    /// In-block round transition at local row `local` of permutation `p`.
    Round { p: usize, local: usize },
    /// Link out of permutation `p` (its output row) into perm p+1's input.
    Link { p: usize },
    /// Padding / unconstrained.
    None,
}

fn classify_row(i: usize) -> RowKind {
    let np = n_perms();
    if i >= np * PERM_ROWS {
        return RowKind::None;
    }
    let p = i / PERM_ROWS;
    let local = i % PERM_ROWS;
    if local < PERM_ROWS - 1 {
        RowKind::Round { p, local }
    } else {
        // local == PERM_ROWS - 1 (output row): a link, unless this is the last perm.
        if p + 1 < np {
            RowKind::Link { p }
        } else {
            RowKind::None
        }
    }
}

impl Air for C2PrfAir {
    fn n_cols(&self) -> usize {
        N_COLS
    }
    fn n_rows(&self) -> usize {
        n_rows_padded()
    }
    fn max_degree(&self) -> usize {
        3
    }

    fn generate_trace(&self) -> Vec<Vec<u32>> {
        self.trace.clone()
    }

    fn boundary_constraints(&self) -> Vec<BoundaryConstraint> {
        let mut out = Vec::new();
        let plan = build_plan();

        // 4. capacity-init: perm 0 input row capacity lanes == public domain const.
        let init_state = init_capacity(POSEIDON2_PROJECTION_MASK_DOMAIN);
        let r0 = perm_input_row(0);
        for lane in RATE..T {
            out.push(BoundaryConstraint {
                row: r0,
                col: STATE + lane,
                value: init_state[lane],
            });
        }
        // perm 0 input rate lanes == lane_val[i] is enforced by the p=0 link with
        // cur = the public init state (rate lanes 0). To make the link's cur fully
        // public for p=0 we ALSO pin perm 0 input state rate lanes directly to
        // lane_val via the absorb link (handled in transition_residuals).

        // 6. lane-const pin: rate positions carrying a sponge constant.
        for (p, role) in plan.roles.iter().enumerate() {
            if let PermRole::Absorb { block } = role {
                let in_row = perm_input_row(p);
                for i in 0..RATE {
                    if let BlockElem::Const(c) = block[i] {
                        out.push(BoundaryConstraint {
                            row: in_row,
                            col: LANE_VAL + i,
                            value: fq_from_u32(c),
                        });
                    }
                }
            } else {
                // squeeze perms have no absorbed block: pin lane_val to 0.
                let in_row = perm_input_row(p);
                for i in 0..RATE {
                    out.push(BoundaryConstraint {
                        row: in_row,
                        col: LANE_VAL + i,
                        value: 0,
                    });
                }
            }
        }

        // 7. mask output: committed masks == squeezed cells.
        let n_absorb = n_absorb_perms();
        let mut filled = 0usize;
        let mut read_perm = n_absorb - 1;
        let mut remaining = N_MASK_OUTPUTS;
        loop {
            let take = remaining.min(RATE);
            let out_row = perm_output_row(read_perm);
            for i in 0..take {
                out.push(BoundaryConstraint {
                    row: out_row,
                    col: STATE + i,
                    value: self.masks[filled],
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

    fn transition_rows(&self) -> Vec<usize> {
        let np = n_perms();
        let mut rows = Vec::new();
        for i in 0..(np * PERM_ROWS - 1) {
            if !matches!(classify_row(i), RowKind::None) {
                rows.push(i);
            }
        }
        rows
    }

    fn n_transition_residuals(&self) -> usize {
        // Round transition: T state residuals + T aux-square bindings = 2T = 48.
        // Link transition: T state residuals + RATE recompose + RATE*8 boolean
        //   byte-range = 24 + 5 + 40 = 69. Width = max = 69; unused slots = 0.
        RESID_WIDTH
    }

    fn transition_residuals(&self, i: usize, cur: &[Fq12], nxt: &[Fq12]) -> Vec<Fq12> {
        let mut res = vec![Fq12::zero(); RESID_WIDTH];
        match classify_row(i) {
            RowKind::None => {}
            RowKind::Round { p, local } => {
                // state lifts.
                let mut c = [Fq12::zero(); T];
                let mut n = [Fq12::zero(); T];
                for k in 0..T {
                    c[k] = cur[STATE + k];
                    n[k] = nxt[STATE + k];
                }
                let kind = transition_kind(local);
                match kind {
                    TransitionKind::InitialExternal => {
                        let exp = apply_me_fq12(&self.params, &c);
                        for k in 0..T {
                            res[k] = n[k].sub(&exp[k]);
                        }
                        // no sq binding (no S-box on this transition).
                        // PERM 0 INPUT DEPOSIT (single-row, on `cur` = row 0):
                        // the first absorb deposits lane_val into the rate lanes of
                        // the bare initial state (rate lanes 0), so
                        //   cur.state[i] == cur.lane_val[i]  for i < RATE.
                        // (For p>=1 the deposit is the absorb-LINK; perm 0 has no
                        // predecessor link, so we pin it here.) Spare slots T..T+RATE
                        // are unused by InitialExternal otherwise.
                        if p == 0 {
                            let plan = build_plan();
                            if let PermRole::Absorb { .. } = &plan.roles[0] {
                                for i in 0..RATE {
                                    res[BR_DEPOSIT + i] = cur[STATE + i].sub(&cur[LANE_VAL + i]);
                                }
                            }
                        }
                        // BYTE-RANGE for THIS absorb perm's input row (= cur):
                        // covers every absorb perm uniformly (incl. perm 0).
                        let plan = build_plan();
                        byte_range_residuals(&plan.roles[p], cur, &mut res);
                    }
                    TransitionKind::Full { rc_index } => {
                        let rc = &self.params.rc_full[rc_index];
                        // a[k] = c[k] + rc[k]; sq[k] = cur.sq[k]; sbox = sq^2 * a.
                        let mut y = [Fq12::zero(); T];
                        for k in 0..T {
                            let a = c[k].add(&from_base(rc[k]));
                            let sq = cur[SQ + k];
                            // binding: sq - a^2 = 0.
                            res[T + k] = sq.sub(&a.mul(&a));
                            let sq2 = sq.mul(&sq);
                            y[k] = sq2.mul(&a); // = a^5 if binding holds
                        }
                        let exp = apply_me_fq12(&self.params, &y);
                        for k in 0..T {
                            res[k] = n[k].sub(&exp[k]);
                        }
                    }
                    TransitionKind::Partial { rc_index } => {
                        let mut y = c;
                        let a = c[0].add(&from_base(self.params.rc_partial[rc_index]));
                        let sq = cur[SQ + 0];
                        res[T] = sq.sub(&a.mul(&a)); // binding for lane 0
                        y[0] = sq.mul(&sq).mul(&a); // a^5
                        let exp = apply_mi_fq12(&self.params, &y);
                        for k in 0..T {
                            res[k] = n[k].sub(&exp[k]);
                        }
                    }
                }
            }
            RowKind::Link { p } => {
                // cur = perm p output state, nxt = perm p+1 input row.
                // dest perm role determines absorb vs squeeze.
                let plan = build_plan();
                let dest = p + 1;
                match &plan.roles[dest] {
                    PermRole::Absorb { .. } => {
                        for k in 0..T {
                            let cur_s = cur[STATE + k];
                            let nxt_s = nxt[STATE + k];
                            let expected = if k < RATE {
                                cur_s.add(&nxt[LANE_VAL + k])
                            } else {
                                cur_s
                            };
                            res[k] = nxt_s.sub(&expected);
                        }
                    }
                    PermRole::Squeeze => {
                        for k in 0..T {
                            res[k] = nxt[STATE + k].sub(&cur[STATE + k]);
                        }
                    }
                }
                // (byte-range for the dest input row is emitted on that row's own
                // InitialExternal leaving-transition, not here.)
                let _ = dest;
            }
        }
        res
    }
}

/// Add byte-range residuals for an absorb input row into reserved slots:
///   res[BR_RECOMPOSE + i]            : sum_b bits[i][b]*2^b - lane_val[i]
///   res[BR_BOOL + i*8 + b]           : bits[i][b]*(bits[i][b]-1)
/// for each rate position `i` carrying a HIDDEN input byte-lane. Const positions
/// emit nothing here (their lane_val is boundary-pinned). All cells are on the
/// DEST input row, so this residual reads `nxt`.
fn byte_range_residuals(role: &PermRole, nxt: &[Fq12], res: &mut [Fq12]) {
    if let PermRole::Absorb { block } = role {
        let two = Fq12::from_base(2);
        for i in 0..RATE {
            if let BlockElem::InputLane(_) = block[i] {
                let mut acc = Fq12::zero();
                let mut w = Fq12::one();
                for b in 0..BITS_PER_LANE {
                    let bit = nxt[bit_col(i, b)];
                    acc = acc.add(&bit.mul(&w));
                    w = w.mul(&two);
                    res[BR_BOOL + i * BITS_PER_LANE + b] = bit.mul(&bit.sub(&Fq12::one()));
                }
                res[BR_RECOMPOSE + i] = acc.sub(&nxt[LANE_VAL + i]);
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stark_q12289::{choose_domains, prove_with_queries, verify};

    /// Lift base-field trace cells at one row into an Fq12 row vector.
    fn lift_row(row: &[u32]) -> Vec<Fq12> {
        row.iter().map(|&v| Fq12::from_base(v)).collect()
    }

    /// CHEAP correctness check (no FRI): every transition residual at every real
    /// transition row is zero, and every boundary point constraint holds, when the
    /// trace cells are lifted to Fq12. This validates the AIR constraint logic.
    fn air_satisfied(air: &C2PrfAir) -> Result<(), String> {
        let trace = air.generate_trace();
        // transitions
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
        // boundaries
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

    /// Layout / domain report.
    #[test]
    fn c2_air_layout_and_domain_report() {
        let air = C2PrfAir::new(&[3u8; 32], 42);
        let d = choose_domains(&air);
        assert_eq!(air.n_cols(), N_COLS);
        assert_eq!(air.n_rows(), 1024, "700 rows padded to 1024");
        assert_eq!(air.max_degree(), 3);
        assert_eq!(d.trace_n, 1024);
        assert_eq!(
            d.fri_n, 16384,
            "degree-3 @ n=1024 needs N/4 > 3068 -> 16384"
        );
        // rate <= 1/4.
        assert!(d.log2_fri - d.log2_trace >= 2);
        println!(
            "C2 AIR: n_rows={} n_cols={} max_deg={} trace_n={} fri_n={} blowup=2^{} \
             boundary={} transition_rows={} resid_width={}",
            air.n_rows(),
            air.n_cols(),
            air.max_degree(),
            d.trace_n,
            d.fri_n,
            d.log2_fri - d.log2_trace,
            air.boundary_constraints().len(),
            air.transition_rows().len(),
            air.n_transition_residuals(),
        );
    }

    /// The AIR's constraint logic is satisfied by the honest trace AND the AIR's
    /// committed masks equal the reference PRF (cheap, no FRI). Several secrets.
    #[test]
    fn c2_air_honest_trace_satisfies_and_masks_match_reference() {
        use crate::poseidon2_q12289::projection_masks_poseidon2;
        let mut seed = 0xC0FF_EE00_1234_5678u64;
        for _ in 0..8 {
            let pad = random_pad(&mut seed);
            let slot = xorshift(&mut seed);
            let air = C2PrfAir::new(&pad, slot);
            air_satisfied(&air).expect("honest C2 trace must satisfy the AIR");
            let reference = projection_masks_poseidon2(&pad, slot);
            let masks = air.masks();
            for j in 0..N_MASK_OUTPUTS {
                assert_eq!(
                    masks[j] as u16, reference[j],
                    "mask lane {j} != reference PRF"
                );
            }
        }
    }

    /// C2 COMPLETENESS: an honest C2 trace -> STARK proof VERIFIES, and the
    /// proof's claimed masks equal the reference PRF. (Reduced query count to keep
    /// the 16384-domain proof tractable in CI; >= 1 trial, several below.)
    #[test]
    fn c2_stark_completeness_verifies() {
        use crate::poseidon2_q12289::projection_masks_poseidon2;
        let pad = [7u8; 32];
        let slot = 123456789u64;
        let air = C2PrfAir::new(&pad, slot);
        let reference = projection_masks_poseidon2(&pad, slot);
        for j in 0..N_MASK_OUTPUTS {
            assert_eq!(
                air.masks()[j] as u16,
                reference[j],
                "committed mask != reference"
            );
        }
        let proof = prove_with_queries(&air, None, 24);
        assert!(verify(&air, &proof), "honest C2 STARK proof must verify");
    }

    /// Measure prove/verify time and a proof-size estimate for the C2 STARK.
    /// `#[ignore]` so default CI does not pay it; run with `--ignored`.
    #[test]
    #[ignore]
    fn c2_stark_size_and_timing_report() {
        use std::time::Instant;
        let air = C2PrfAir::new(&[5u8; 32], 7);
        let t0 = Instant::now();
        let proof = prove_with_queries(&air, None, 24);
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
            "C2 STARK timing/size: prove={prove_ms}ms verify={verify_ms}ms \
             fq12_vals={n_fq12} path_nodes={n_pathnode} ~bytes={bytes} \
             (~{} KiB) num_queries=24 fri_n=16384",
            bytes / 1024
        );
    }

    /// C2 SOUNDNESS: tampered traces (breaking a round, an absorb link, a squeeze
    /// link, the byte-range, or claiming a wrong mask) -> STARK verify FAILS.
    #[test]
    fn c2_stark_soundness_tampers_rejected() {
        use crate::poseidon2_q12289::fq_add;
        let pad = [9u8; 32];
        let slot = 99u64;
        let air = C2PrfAir::new(&pad, slot);
        let honest = air.generate_trace();
        assert!(
            verify(&air, &prove_with_queries(&air, None, 24)),
            "baseline honest proof must verify"
        );

        // (row, col) tampers across the regions.
        let np = n_perms();
        let tampers: &[(usize, usize, &str)] = &[
            (
                perm_input_row(0) + 3,
                STATE + 7,
                "perm0 initial full-round cell",
            ),
            (
                perm_input_row(4) + 30,
                STATE + 9,
                "perm4 partial-round cell",
            ),
            (
                perm_output_row(2),
                STATE + 1,
                "perm2 output (feeds absorb link)",
            ),
            (
                perm_input_row(np - 1),
                STATE + 1,
                "squeeze perm input (squeeze link)",
            ),
        ];
        for &(row, col, what) in tampers {
            let mut t = honest.clone();
            t[row][col] = fq_add(t[row][col], 1);
            let proof = prove_with_queries(&air, Some(t), 24);
            assert!(
                !verify(&air, &proof),
                "tamper [{what}] at ({row},{col}) not rejected"
            );
        }

        // tamper a byte-range bit cell (break boolean / recompose).
        {
            let mut t = honest.clone();
            let in_row = perm_input_row(0);
            t[in_row][bit_col(0, 0)] = fq_add(t[in_row][bit_col(0, 0)], 1);
            let proof = prove_with_queries(&air, Some(t), 24);
            assert!(!verify(&air, &proof), "byte-range bit tamper not rejected");
        }

        // claim WRONG masks: build a fresh AIR with the honest trace but a mutated
        // committed mask -> mask-output boundary must fail.
        {
            let mut air2 = C2PrfAir::new(&pad, slot);
            air2.masks[2] = fq_add(air2.masks[2], 1);
            let proof = prove_with_queries(&air2, None, 24);
            assert!(!verify(&air2, &proof), "wrong claimed mask not rejected");
        }
    }
}
