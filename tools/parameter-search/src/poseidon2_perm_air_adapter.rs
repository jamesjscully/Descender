//! Adapter exposing the single Poseidon2 permutation AIR
//! (`crate::poseidon2_air_q12289`) through the generic [`Air`] interface.
//!
//! This re-expresses the EXACT same relation (row 0 = input, last row = output,
//! 69 per-row-pair transitions, max degree 5) so the generic STARK can prove it
//! UNCHANGED. The transition residual evaluator lifts
//! `poseidon2_air_q12289::expected_next` to Fq12 (the same field ops, same
//! matrices, same S-box), and the boundary point constraints pin row 0 to the
//! claimed input and the last row to the trace's output.

use crate::air::{Air, BoundaryConstraint};
use crate::fq12_q12289::Fq12;
use crate::poseidon2_air_q12289::{
    self as perm_air, generate_trace, transition_kind, TransitionKind, N_COLS, N_ROWS,
};
use crate::poseidon2_q12289::{Poseidon2Params, T};

/// The single-permutation AIR over a fixed (params, input). The output is taken
/// from the honestly generated trace and pinned as the boundary-output public
/// value, so the STARK proves "this trace computes permute(input) = output".
pub struct Poseidon2PermAir {
    params: Poseidon2Params,
    input: [u32; T],
    output: [u32; T],
}

impl Poseidon2PermAir {
    pub fn new(params: &Poseidon2Params, input: [u32; T]) -> Self {
        // Rebuild params locally (Poseidon2Params::new is deterministic) so the
        // AIR owns its data and is Send-free / lifetime-free for &dyn Air.
        let p = Poseidon2Params::new();
        let _ = params;
        let trace = generate_trace(&p, &input);
        let output = trace.output();
        Poseidon2PermAir {
            params: p,
            input,
            output,
        }
    }

    pub fn output(&self) -> [u32; T] {
        self.output
    }

    fn lift(v: &[Fq12], out: &mut [Fq12; T]) {
        for (k, slot) in out.iter_mut().enumerate() {
            *slot = v[k];
        }
    }
}

// Fq12 lifts of the permutation layer arithmetic (mirror expected_next).
fn apply_me_fq12(params: &Poseidon2Params, v: &[Fq12; T]) -> [Fq12; T] {
    let mut out = [Fq12::zero(); T];
    for i in 0..T {
        let mut acc = Fq12::zero();
        for j in 0..T {
            acc = acc.add(&Fq12::from_base(params.m_e[i][j]).mul(&v[j]));
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
        let mu_minus_1 = Fq12::from_base(params.internal_diag[k]).sub(&Fq12::one());
        out[k] = sum.add(&mu_minus_1.mul(&v[k]));
    }
    out
}

fn sbox_fq12(x: &Fq12) -> Fq12 {
    let x2 = x.mul(x);
    let x4 = x2.mul(&x2);
    x4.mul(x)
}

fn expected_next_fq12(params: &Poseidon2Params, kind: TransitionKind, cur: &[Fq12; T]) -> [Fq12; T] {
    match kind {
        TransitionKind::InitialExternal => apply_me_fq12(params, cur),
        TransitionKind::Full { rc_index } => {
            let rc = &params.rc_full[rc_index];
            let mut s = *cur;
            for k in 0..T {
                s[k] = s[k].add(&Fq12::from_base(rc[k]));
            }
            for k in 0..T {
                s[k] = sbox_fq12(&s[k]);
            }
            apply_me_fq12(params, &s)
        }
        TransitionKind::Partial { rc_index } => {
            let mut s = *cur;
            s[0] = s[0].add(&Fq12::from_base(params.rc_partial[rc_index]));
            s[0] = sbox_fq12(&s[0]);
            apply_mi_fq12(params, &s)
        }
    }
}

impl Air for Poseidon2PermAir {
    fn n_cols(&self) -> usize {
        N_COLS
    }
    fn n_rows(&self) -> usize {
        N_ROWS
    }
    fn max_degree(&self) -> usize {
        perm_air::MAX_DEGREE as usize
    }

    fn generate_trace(&self) -> Vec<Vec<u32>> {
        let trace = generate_trace(&self.params, &self.input);
        trace.rows.iter().map(|r| r.to_vec()).collect()
    }

    fn boundary_constraints(&self) -> Vec<BoundaryConstraint> {
        let mut out = Vec::with_capacity(2 * T);
        for col in 0..T {
            out.push(BoundaryConstraint {
                row: perm_air::ROW_INPUT,
                col,
                value: self.input[col],
            });
        }
        for col in 0..T {
            out.push(BoundaryConstraint {
                row: N_ROWS - 1,
                col,
                value: self.output[col],
            });
        }
        out
    }

    fn transition_rows(&self) -> Vec<usize> {
        (0..(N_ROWS - 1)).collect()
    }

    fn n_transition_residuals(&self) -> usize {
        T
    }

    fn transition_residuals(&self, i: usize, cur: &[Fq12], nxt: &[Fq12]) -> Vec<Fq12> {
        let mut c = [Fq12::zero(); T];
        let mut n = [Fq12::zero(); T];
        Self::lift(cur, &mut c);
        Self::lift(nxt, &mut n);
        let exp = expected_next_fq12(&self.params, transition_kind(i), &c);
        (0..T).map(|k| n[k].sub(&exp[k])).collect()
    }
}
