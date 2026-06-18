//! AIR-agnostic interface consumed by the generic DEEP-ALI STARK
//! (`stark_q12289`).
//!
//! An `Air` describes an execution-trace relation to the STARK *without* knowing
//! anything about FRI, commitments, the OOD point, or the field extension. It
//! exposes exactly what the standard DEEP-ALI composition needs:
//!
//!   * `n_cols` / `n_rows`        — trace shape (n_rows is padded to a power of two
//!                                   by the caller; the AIR reports its REAL row
//!                                   count and the STARK pads with zero rows).
//!   * `max_degree`               — the largest polynomial degree (in the trace
//!                                   cells) of any single constraint. Selects the
//!                                   FRI blowup so the composition stays below the
//!                                   FRI rate.
//!   * `generate_trace`           — the honest witness: n_rows x n_cols base-field
//!                                   cells.
//!   * `boundary_constraints`     — point constraints `trace[row][col] == value`.
//!                                   The STARK groups them by `row`, builds one
//!                                   vanishing factor `(x - g^row)` per distinct
//!                                   row, and batches the columns at that row with
//!                                   Fiat-Shamir lane combiners.
//!   * `transition_residuals`     — for the consecutive pair (row, row+1), the
//!                                   residual vector that an honest trace zeroes.
//!                                   Evaluated over the field EXTENSION so the
//!                                   STARK can compose on the FRI coset / at the
//!                                   OOD point. The set of `row` indices for which
//!                                   the constraint must hold is `transition_rows`.
//!
//! The STARK turns these into the composition
//!     H(x) = alpha_T * C_T(x)/Z_T(x)
//!          + sum_b alpha_b * C_b(x)/(x - g^row_b)
//! where C_T is the per-row-selector transition numerator over `transition_rows`
//! and C_b is the FS-batched boundary numerator at distinct row `row_b`. A trace
//! satisfies the AIR iff every quotient is a polynomial iff H is low-degree.
//!
//! DEGREE NOTE. With trace size n (= n_rows padded) and max constraint degree d,
//! a transition residual has degree <= d*(n-1); the per-row selector numerator
//! C_T has degree <= d*(n-1) + (|transition_rows| - 1), and C_T/Z_T has degree
//! <= d*(n-1) - 1. The STARK chooses the FRI blowup so that this is below the FRI
//! rate budget (FRI_N/4). See `stark_q12289` for the concrete sizing.

use crate::fq12_q12289::Fq12;

/// A single boundary (point) constraint: `trace[row][col] == value`, where
/// `value` is a base-field public constant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundaryConstraint {
    pub row: usize,
    pub col: usize,
    /// Expected base-field value (public). Compared as a canonical Fq element.
    pub value: u32,
}

/// An AIR: a trace-relation the generic STARK can prove and verify.
///
/// All transition arithmetic is over the field extension `Fq12` so the STARK can
/// evaluate it on the FRI coset and at the out-of-domain point. The trace itself
/// is base-field (`u32` canonical Fq).
pub trait Air {
    /// Number of trace columns.
    fn n_cols(&self) -> usize;

    /// Number of REAL trace rows (the STARK pads to the next power of two with
    /// zero rows; padding rows are referenced by no constraint).
    fn n_rows(&self) -> usize;

    /// Maximum polynomial degree of any single constraint, in the trace cells.
    fn max_degree(&self) -> usize;

    /// Honest trace: `n_rows` rows, each `n_cols` canonical base-field cells.
    fn generate_trace(&self) -> Vec<Vec<u32>>;

    /// All boundary (point) constraints.
    fn boundary_constraints(&self) -> Vec<BoundaryConstraint>;

    /// The rows `i` for which the transition (row i -> row i+1) is constrained.
    /// Must be a subset of `0..n_rows-1`. The STARK's transition vanishing poly
    /// is `Z_T(x) = prod_{i in transition_rows} (x - g^i)`.
    fn transition_rows(&self) -> Vec<usize>;

    /// The transition residual vector for the consecutive pair (row `i`,
    /// row `i+1`), given the trace column values at the two rows lifted to the
    /// extension field. An honest trace yields the all-zero vector for every
    /// `i in transition_rows`. The returned length must be the same for every
    /// `i` (the STARK batches lane residuals with a fixed number of combiners),
    /// padding unused slots with `Fq12::zero()` if a row uses fewer.
    ///
    /// `cur[c]` and `nxt[c]` are column `c` at rows i and i+1 respectively.
    fn transition_residuals(&self, i: usize, cur: &[Fq12], nxt: &[Fq12]) -> Vec<Fq12>;

    /// The number of residual lanes `transition_residuals` always returns
    /// (the width of the transition constraint group). Must equal the length of
    /// every `transition_residuals` output.
    fn n_transition_residuals(&self) -> usize;
}
