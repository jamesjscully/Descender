//! Framework-agnostic AIR (Algebraic Intermediate Representation) for ONE
//! Poseidon2 permutation over Fq = 12289, t = 24.
//!
//! This module expresses the EXACT permutation implemented in
//! `crate::poseidon2_q12289::permute` as a polynomial constraint system over an
//! execution trace, together with an honest trace/witness generator. It is the
//! AIR *relation* R only — there is NO FRI / commitment / proof layer here, and
//! NO dependency on any external STARK library. Everything is pure
//! constraint-evaluation over Fq.
//!
//! CORRECTNESS and HONESTY are the priorities, not speed.
//!
//! =============================================================================
//! TRACE LAYOUT
//! =============================================================================
//!
//! The permutation (see `poseidon2_q12289::permute`) is the composition of these
//! layers, in order:
//!
//!   L0.  state := M_E * state                          (initial external layer)
//!   For r in 0..4   (initial FULL rounds):
//!        state := M_E * SBOX_all( state + rc_full[r] )
//!   For r in 0..60  (PARTIAL rounds):
//!        state := M_I * SBOX_lane0( state + rc_partial[r]·e0 )
//!   For r in 0..4   (terminal FULL rounds):
//!        state := M_E * SBOX_all( state + rc_full[4+r] )
//!
//! where SBOX_all applies x^5 to every lane, SBOX_lane0 applies x^5 to lane 0
//! only (other lanes pass through), and `·e0` means the round constant is added
//! to lane 0 only.
//!
//! We choose ONE TRACE ROW PER STATE between layers. A row is the full t = 24
//! lane state. The number of rows is therefore:
//!
//!   row 0                : the RAW input state (before ANY layer)
//!   row 1                : state after the initial M_E layer (L0)
//!   rows 2 .. 2+4        : state after each of the 4 initial full rounds
//!   rows 6 .. 6+60       : state after each of the 60 partial rounds
//!   rows 66 .. 66+4      : state after each of the 4 terminal full rounds
//!   ---------------------------------------------------------------------------
//!   total ROWS = 1 (input) + 1 (initial M_E) + 4 + 60 + 4 = 70 rows
//!
//! There are NO auxiliary columns. The S-box value `pre^5` is degree 5 in the
//! trace cells, which is exactly our degree budget, so we constrain it directly
//! (no witnessed intermediate column is needed and none is used).
//!
//!   COLUMNS  = T = 24   (one per state lane)
//!   ROWS     = N_ROWS = 70
//!
//! Indexing helpers below name the row of each layer boundary. Each consecutive
//! row pair (row i, row i+1) is governed by exactly ONE transition rule.
//!
//! =============================================================================
//! CONSTRAINTS
//! =============================================================================
//!
//! BOUNDARY (2 vector constraints, one per checked endpoint):
//!   * row 0    == the claimed input state                    (T scalar eqns)
//!   * row N-1  == the claimed output state                   (T scalar eqns)
//!
//! TRANSITION (one rule per consecutive row pair); for each pair we emit, for
//! every lane k, the residual  next[k] - (expected next[k]):
//!
//!   T0  (row0 -> row1)  : pure linear, next = M_E * cur.
//!                         residual_k = next[k] - sum_j M_E[k][j]*cur[j]
//!                         DEGREE 1.
//!
//!   FULL round (the 8 full-round transitions):
//!                         next = M_E * SBOX_all(cur + rc).
//!                         Let a_j = cur[j] + rc[j]; SBOX is a_j^5.
//!                         residual_k = next[k] - sum_j M_E[k][j] * a_j^5
//!                         DEGREE 5  (a_j^5 is degree 5 in cur[j]; the outer
//!                         M_E mix is linear).
//!
//!   PARTIAL round (the 60 partial-round transitions):
//!                         next = M_I * y   where
//!                            y[0] = (cur[0] + rc_partial)^5     (S-box lane 0)
//!                            y[j] = cur[j]               for j != 0
//!                         residual_k = next[k] - sum_j M_I[k][j] * y[j]
//!                         DEGREE 5  (only y[0] is degree 5; M_I mix is linear).
//!
//! MAX CONSTRAINT DEGREE = 5  (the S-box). No constraint exceeds 5.
//!
//! A residual of 0 for every emitted scalar equation == the trace satisfies the
//! AIR. `max_residual` returns the maximum absolute residual (each residual is a
//! canonical Fq element; "0" means satisfied, any nonzero element means a
//! violated constraint). `all_residuals` returns them all for auditing.

use crate::poseidon2_q12289::{
    Poseidon2Params, R_F_HALF, R_P, T, apply_internal, apply_matrix, fq_add, fq_from_u32, fq_sub,
    sbox,
};

#[cfg(test)]
use crate::poseidon2_q12289::R_F;

// -----------------------------------------------------------------------------
// Row layout constants
// -----------------------------------------------------------------------------

/// Row 0 = raw input.
pub const ROW_INPUT: usize = 0;
/// Row 1 = state after the initial external (M_E) layer.
pub const ROW_AFTER_INIT_ME: usize = 1;
/// First row that is the output of an initial full round.
pub const ROW_FIRST_FULL_INIT: usize = ROW_AFTER_INIT_ME + 1; // 2
/// First row that is the output of a partial round.
pub const ROW_FIRST_PARTIAL: usize = ROW_FIRST_FULL_INIT + R_F_HALF; // 6
/// First row that is the output of a terminal full round.
pub const ROW_FIRST_FULL_TERM: usize = ROW_FIRST_PARTIAL + R_P; // 66
/// Total number of rows in the trace.
pub const N_ROWS: usize = ROW_FIRST_FULL_TERM + R_F_HALF; // 70

/// Number of trace columns (state lanes).
pub const N_COLS: usize = T;

/// The kind of transition between row `i` and row `i+1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionKind {
    /// Pure linear M_E (the initial external layer): next = M_E * cur.
    InitialExternal,
    /// Full round: next = M_E * SBOX_all(cur + rc_full[idx]).
    Full {
        /// Index into `rc_full` (0..R_F).
        rc_index: usize,
    },
    /// Partial round: next = M_I * SBOX_lane0(cur + rc_partial[idx]·e0).
    Partial {
        /// Index into `rc_partial` (0..R_P).
        rc_index: usize,
    },
}

/// Classify the transition that governs the pair (row `i`, row `i+1`).
///
/// `i` must be in 0..N_ROWS-1.
pub fn transition_kind(i: usize) -> TransitionKind {
    assert!(i + 1 < N_ROWS, "transition row out of range: {i}");
    // The transition (row i -> row i+1) is the LAYER that PRODUCES row i+1, so
    // classify by the produced row `i + 1`.
    let produced = i + 1;
    if produced == ROW_AFTER_INIT_ME {
        // row 1 = initial external M_E layer.
        TransitionKind::InitialExternal
    } else if produced < ROW_FIRST_PARTIAL {
        // rows 2..6 are the 4 initial full-round outputs: row 2 uses rc 0, ...
        TransitionKind::Full {
            rc_index: produced - ROW_FIRST_FULL_INIT,
        }
    } else if produced < ROW_FIRST_FULL_TERM {
        // rows 6..66 are the 60 partial-round outputs: row 6 uses rc 0, ...
        TransitionKind::Partial {
            rc_index: produced - ROW_FIRST_PARTIAL,
        }
    } else {
        // rows 66..70 are the 4 terminal full-round outputs: rc_full[R_F_HALF+k].
        TransitionKind::Full {
            rc_index: R_F_HALF + (produced - ROW_FIRST_FULL_TERM),
        }
    }
}

// -----------------------------------------------------------------------------
// Trace
// -----------------------------------------------------------------------------

/// An execution trace: `N_ROWS` rows, each a `T`-lane Fq state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    /// rows[i][k] is lane k of the state at row i. Every entry is a canonical
    /// Fq element (< Q).
    pub rows: Vec<[u32; T]>,
}

impl Trace {
    /// The claimed input is row 0.
    pub fn input(&self) -> [u32; T] {
        self.rows[ROW_INPUT]
    }
    /// The claimed permutation output is the last row.
    pub fn output(&self) -> [u32; T] {
        self.rows[N_ROWS - 1]
    }
}

/// Honestly generate the trace by running the exact permutation layer by layer,
/// recording the full state after each layer. The recorded computation IS the
/// reference permutation decomposed into the layers above — by construction the
/// last row equals `permute(input)`.
pub fn generate_trace(params: &Poseidon2Params, input_state: &[u32; T]) -> Trace {
    let mut rows: Vec<[u32; T]> = Vec::with_capacity(N_ROWS);

    // Row 0: raw input (reduced into Fq, matching permute's first step).
    let mut state = *input_state;
    for s in state.iter_mut() {
        *s = fq_from_u32(*s);
    }
    rows.push(state);

    // Row 1: initial external M_E layer.
    apply_matrix(&params.m_e, &mut state);
    rows.push(state);

    // Rows 2..6: 4 initial full rounds.
    for r in 0..R_F_HALF {
        let rc = &params.rc_full[r];
        for k in 0..T {
            state[k] = fq_add(state[k], rc[k]);
        }
        for k in 0..T {
            state[k] = sbox(state[k]);
        }
        apply_matrix(&params.m_e, &mut state);
        rows.push(state);
    }

    // Rows 6..66: 60 partial rounds.
    for r in 0..R_P {
        state[0] = fq_add(state[0], params.rc_partial[r]);
        state[0] = sbox(state[0]);
        apply_internal(&params.internal_diag, &mut state);
        rows.push(state);
    }

    // Rows 66..70: 4 terminal full rounds.
    for r in 0..R_F_HALF {
        let rc = &params.rc_full[R_F_HALF + r];
        for k in 0..T {
            state[k] = fq_add(state[k], rc[k]);
        }
        for k in 0..T {
            state[k] = sbox(state[k]);
        }
        apply_matrix(&params.m_e, &mut state);
        rows.push(state);
    }

    debug_assert_eq!(rows.len(), N_ROWS);
    Trace { rows }
}

// -----------------------------------------------------------------------------
// Expected-next-row evaluators (the heart of the transition constraints)
// -----------------------------------------------------------------------------

/// Compute the EXPECTED next-row state from `cur` under the transition rule for
/// row pair starting at row `i`. This is the polynomial the constraints pin:
/// the transition residual is `next - expected_next(cur)`.
///
/// The arithmetic here is identical (same field ops, same matrices, same S-box)
/// to the per-layer computation in `generate_trace` / `permute`, so an honest
/// trace yields residual 0 by construction; any deviation in `next` shows up as
/// a nonzero residual.
fn expected_next(params: &Poseidon2Params, kind: TransitionKind, cur: &[u32; T]) -> [u32; T] {
    match kind {
        TransitionKind::InitialExternal => {
            let mut s = *cur;
            apply_matrix(&params.m_e, &mut s);
            s
        }
        TransitionKind::Full { rc_index } => {
            let rc = &params.rc_full[rc_index];
            let mut s = *cur;
            for k in 0..T {
                s[k] = fq_add(s[k], rc[k]);
            }
            for k in 0..T {
                s[k] = sbox(s[k]);
            }
            apply_matrix(&params.m_e, &mut s);
            s
        }
        TransitionKind::Partial { rc_index } => {
            let mut s = *cur;
            s[0] = fq_add(s[0], params.rc_partial[rc_index]);
            s[0] = sbox(s[0]);
            apply_internal(&params.internal_diag, &mut s);
            s
        }
    }
}

// -----------------------------------------------------------------------------
// Constraint residuals
// -----------------------------------------------------------------------------

/// One scalar constraint residual, tagged for auditing/debugging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Residual {
    pub kind: ResidualKind,
    /// The residual value as a canonical Fq element. 0 == satisfied.
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualKind {
    /// Boundary: row 0 lane `lane` must equal claimed input lane.
    BoundaryInput { lane: usize },
    /// Boundary: last row lane `lane` must equal claimed output lane.
    BoundaryOutput { lane: usize },
    /// Transition residual for the pair (row, row+1), lane `lane`.
    Transition { row: usize, lane: usize },
}

/// Evaluate EVERY constraint residual for `trace` against the given claimed
/// `input` and `output` boundary values. Returns all scalar residuals.
///
/// An all-zero result == the trace satisfies the entire AIR relation R for
/// these boundary values.
pub fn all_residuals(
    params: &Poseidon2Params,
    trace: &Trace,
    input: &[u32; T],
    output: &[u32; T],
) -> Vec<Residual> {
    let mut out: Vec<Residual> = Vec::with_capacity(2 * T + (N_ROWS - 1) * T);

    // Boundary: row 0 == input (reduce input the same way permute does so a
    // non-canonical claimed input is compared canonically).
    for lane in 0..T {
        let claimed = fq_from_u32(input[lane]);
        let residual = fq_sub(trace.rows[ROW_INPUT][lane], claimed);
        out.push(Residual {
            kind: ResidualKind::BoundaryInput { lane },
            value: residual,
        });
    }

    // Transitions.
    for i in 0..(N_ROWS - 1) {
        let kind = transition_kind(i);
        let expected = expected_next(params, kind, &trace.rows[i]);
        for lane in 0..T {
            let residual = fq_sub(trace.rows[i + 1][lane], expected[lane]);
            out.push(Residual {
                kind: ResidualKind::Transition { row: i, lane },
                value: residual,
            });
        }
    }

    // Boundary: last row == output.
    for lane in 0..T {
        let claimed = fq_from_u32(output[lane]);
        let residual = fq_sub(trace.rows[N_ROWS - 1][lane], claimed);
        out.push(Residual {
            kind: ResidualKind::BoundaryOutput { lane },
            value: residual,
        });
    }

    out
}

/// The maximum residual magnitude over all constraints (each residual is a
/// canonical Fq element; we take the numeric max as a single "is it satisfied?"
/// scalar). 0 == ALL constraints satisfied.
pub fn max_residual(
    params: &Poseidon2Params,
    trace: &Trace,
    input: &[u32; T],
    output: &[u32; T],
) -> u32 {
    all_residuals(params, trace, input, output)
        .iter()
        .map(|r| r.value)
        .max()
        .unwrap_or(0)
}

/// Total number of scalar constraints emitted by `all_residuals`.
pub const N_CONSTRAINTS: usize = 2 * T + (N_ROWS - 1) * T;

/// Maximum polynomial degree of any single transition/boundary constraint, in
/// terms of the trace cells. The S-box `x^5` is the only nonlinearity, so the
/// bound is 5. Boundary and the initial-external transition are degree 1.
pub const MAX_DEGREE: u32 = 5;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_q12289::{Q, permute};

    /// Small reproducible xorshift PRNG so tests are deterministic.
    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    fn random_state(seed: &mut u64) -> [u32; T] {
        let mut st = [0u32; T];
        for k in 0..T {
            st[k] = (xorshift(seed) % Q as u64) as u32;
        }
        st
    }

    /// Layout / sizing sanity: documents the row/column/constraint counts.
    #[test]
    fn poseidon2_air_layout_sizes() {
        assert_eq!(N_ROWS, 70, "rows = 1 input + 1 init-M_E + 4 + 60 + 4");
        assert_eq!(N_COLS, 24);
        // 2 boundary vectors (T each) + (N_ROWS-1) transition vectors (T each).
        assert_eq!(N_CONSTRAINTS, 2 * 24 + 69 * 24);
        assert_eq!(N_CONSTRAINTS, 1704);
        assert_eq!(MAX_DEGREE, 5);
        // exactly one transition kind per consecutive pair, all classifiable.
        let mut n_init = 0;
        let mut n_full = 0;
        let mut n_partial = 0;
        for i in 0..(N_ROWS - 1) {
            match transition_kind(i) {
                TransitionKind::InitialExternal => n_init += 1,
                TransitionKind::Full { .. } => n_full += 1,
                TransitionKind::Partial { .. } => n_partial += 1,
            }
        }
        assert_eq!(n_init, 1);
        assert_eq!(n_full, R_F); // 8 full rounds total
        assert_eq!(n_partial, R_P); // 60 partial rounds
    }

    /// KEY CORRECTNESS ANCHOR: the honestly generated trace's claimed output
    /// equals the reference `permute()` on the same input, for many inputs.
    #[test]
    fn poseidon2_air_trace_output_matches_permute() {
        let params = Poseidon2Params::new();
        let mut seed = 0xDEAD_BEEF_1234_5678u64;
        for _ in 0..64 {
            let input = random_state(&mut seed);
            let trace = generate_trace(&params, &input);

            let mut reference = input;
            permute(&params, &mut reference);

            assert_eq!(
                trace.output(),
                reference,
                "AIR trace output must equal reference permute()"
            );
            // and the trace's row 0 is the (reduced) input
            let mut reduced = input;
            for s in reduced.iter_mut() {
                *s = fq_from_u32(*s);
            }
            assert_eq!(trace.input(), reduced);
        }
    }

    /// An honestly generated trace satisfies ALL constraints (every residual 0).
    #[test]
    fn poseidon2_air_honest_trace_satisfies_all_constraints() {
        let params = Poseidon2Params::new();
        let mut seed = 0x0BAD_F00D_CAFE_BABEu64;
        for _ in 0..64 {
            let input = random_state(&mut seed);
            let trace = generate_trace(&params, &input);
            let output = trace.output();

            let residuals = all_residuals(&params, &trace, &input, &output);
            assert_eq!(residuals.len(), N_CONSTRAINTS);
            for r in &residuals {
                assert_eq!(r.value, 0, "honest trace violated a constraint: {r:?}");
            }
            assert_eq!(max_residual(&params, &trace, &input, &output), 0);
        }
    }

    /// SOUNDNESS-FLAVOR TEST: mutating ANY single trace cell makes at least one
    /// constraint residual nonzero. We hit one cell in each region:
    ///   * a lane in an initial full round
    ///   * lane 0 of a partial round (the S-boxed lane)
    ///   * a NON-zero lane of a partial round (passes through S-box but is mixed
    ///     by M_I and bound by the next transition)
    ///   * a lane in a terminal full round / the output row
    /// plus an exhaustive sweep over EVERY cell to be thorough.
    #[test]
    fn poseidon2_air_mutations_break_constraints() {
        let params = Poseidon2Params::new();
        let input = {
            let mut seed = 0x1111_2222_3333_4444u64;
            random_state(&mut seed)
        };
        let honest = generate_trace(&params, &input);
        let honest_output = honest.output();

        // helper: does mutating (row, lane) by +delta break some constraint?
        // We keep the claimed boundary input/output FIXED at the honest values,
        // because the AIR relation is "this trace computes permute(input)=output"
        // for the given public boundary. (For mutations of the output ROW we
        // also check the boundary-output residual catches it.)
        let breaks = |row: usize, lane: usize, delta: u32| -> bool {
            let mut t = honest.clone();
            t.rows[row][lane] = fq_add(t.rows[row][lane], delta);
            let residuals = all_residuals(&params, &t, &input, &honest_output);
            residuals.iter().any(|r| r.value != 0)
        };

        // Named spot checks.
        // initial full round output is row 2..6; mutate row 3 lane 7.
        assert!(
            breaks(3, 7, 1),
            "mutating an initial full-round lane not caught"
        );
        // partial round output rows are 6..66; lane 0 of row 10.
        assert!(breaks(10, 0, 1), "mutating partial-round lane 0 not caught");
        // partial round NON-zero lane: row 10 lane 13.
        assert!(
            breaks(10, 13, 1),
            "mutating partial-round non-zero lane not caught"
        );
        // terminal full round output rows 66..70; mutate row 67 lane 5.
        assert!(
            breaks(67, 5, 1),
            "mutating a terminal full-round lane not caught"
        );
        // the OUTPUT row (last) lane 0.
        assert!(
            breaks(N_ROWS - 1, 0, 1),
            "mutating the output row not caught"
        );
        // the INPUT row (row 0) lane 4.
        assert!(breaks(0, 4, 1), "mutating the input row not caught");

        // Exhaustive sweep: EVERY cell, several deltas, must break something.
        for row in 0..N_ROWS {
            for lane in 0..T {
                for &delta in &[1u32, 2, 7, Q - 1] {
                    assert!(
                        breaks(row, lane, delta),
                        "mutation at row {row} lane {lane} delta {delta} not caught by any constraint"
                    );
                }
            }
        }
    }

    /// Cross-check: a mutation to lane 0 of a partial-round row is caught by the
    /// SAME transition that S-boxes it OR by the transition consuming that row,
    /// and a mutation to a non-zero partial lane is caught by the consuming
    /// transition (M_I mixes it). This asserts the partial-round constraints are
    /// not vacuous on the pass-through lanes.
    #[test]
    fn poseidon2_air_partial_passthrough_lane_is_constrained() {
        let params = Poseidon2Params::new();
        let mut seed = 0x9999_8888_7777_6666u64;
        let input = random_state(&mut seed);
        let honest = generate_trace(&params, &input);
        let output = honest.output();

        // pick a partial row in the middle and a non-zero lane.
        let row = ROW_FIRST_PARTIAL + 20; // some partial output row
        let lane = 11;
        let mut t = honest.clone();
        t.rows[row][lane] = fq_add(t.rows[row][lane], 3);
        let residuals = all_residuals(&params, &t, &input, &output);
        // the transition CONSUMING `row` (pair row->row+1) must fire, because
        // M_I mixes every lane into every next lane.
        let consuming_fired = residuals.iter().any(|r| {
            matches!(r.kind, ResidualKind::Transition { row: rr, .. } if rr == row) && r.value != 0
        });
        assert!(
            consuming_fired,
            "partial-round non-zero lane mutation not caught by the consuming M_I transition"
        );
    }
}
