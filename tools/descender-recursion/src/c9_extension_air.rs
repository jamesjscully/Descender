//! Low-degree AIR gadget for one ADR-0011 rate-7 extension challenge.
//!
//! A degree-8 challenge spans two seven-element output blocks. This gadget
//! supports both transcript states that occur in Plonky3:
//!
//! - `ReadyOutput`: the final observe filled a rate block and already permuted;
//! - `PendingAbsorb(n)`: `n` trailing observed values must be duplexed first.
//!
//! Two permutation blocks are always committed. In `ReadyOutput`, the first is
//! the squeeze needed for limb 8 and the second is a constrained, unused next
//! squeeze. This fixed-width shape is slightly larger but can be embedded into
//! the monolithic recursive AIR without witness-dependent layout.

use crate::c9_challenger::{C9_CAPACITY, C9_RATE};
use crate::inlined_perm::{
    eval_perm, generate_perm_cols, perm_output, Constants, ConstantsExpr, PermCells, RowCells,
    C_INPUTS, C_OUTPUT, PERM_COLS,
};
use descender_account_stark::{Challenge, Val, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

pub const PRIOR: usize = 0;
pub const ABSORBED: usize = PRIOR + WIDTH;
pub const PERM0: usize = ABSORBED + C9_RATE;
pub const PERM1: usize = PERM0 + PERM_COLS;
pub const CHALLENGE: usize = PERM1 + PERM_COLS;
pub const POST_STATE: usize = CHALLENGE + 8;
pub const C9_EXTENSION_COLS: usize = POST_STATE + WIDTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C9ExtensionSource {
    ReadyOutput,
    PendingAbsorb(usize),
}

impl C9ExtensionSource {
    fn pending_len(self) -> Option<usize> {
        match self {
            Self::ReadyOutput => None,
            Self::PendingAbsorb(n) => {
                assert!((1..C9_RATE).contains(&n));
                Some(n)
            }
        }
    }
}

pub fn generate_c9_extension_cols(
    prior: [Val; WIDTH],
    absorbed: [Val; C9_RATE],
    source: C9ExtensionSource,
) -> [Val; C9_EXTENSION_COLS] {
    let mut row = [Val::ZERO; C9_EXTENSION_COLS];
    row[PRIOR..PRIOR + WIDTH].copy_from_slice(&prior);

    let mut first_input = prior;
    if let Some(n) = source.pending_len() {
        row[ABSORBED..ABSORBED + n].copy_from_slice(&absorbed[..n]);
        first_input[..n].copy_from_slice(&absorbed[..n]);
        first_input[n..C9_RATE].fill(Val::ZERO);
        first_input[C9_RATE] = prior[C9_RATE] + Val::from_usize(n);
    }

    let constants = Constants::pinned();
    let perm0 = generate_perm_cols(first_input, &constants);
    let state0 = perm_output(&perm0);
    let perm1 = generate_perm_cols(state0, &constants);
    let state1 = perm_output(&perm1);
    row[PERM0..PERM0 + PERM_COLS].copy_from_slice(&perm0);
    row[PERM1..PERM1 + PERM_COLS].copy_from_slice(&perm1);

    let (first_source, eighth_source, post) = match source {
        C9ExtensionSource::ReadyOutput => (prior, state0, state0),
        C9ExtensionSource::PendingAbsorb(_) => (state0, state1, state1),
    };
    for limb in 0..C9_RATE {
        row[CHALLENGE + limb] = first_source[C9_RATE - 1 - limb];
    }
    row[CHALLENGE + 7] = eighth_source[C9_RATE - 1];
    row[POST_STATE..POST_STATE + WIDTH].copy_from_slice(&post);
    row
}

pub fn challenge_from_c9_extension_cols(row: &[Val; C9_EXTENSION_COLS]) -> Challenge {
    Challenge::from_basis_coefficients_fn(|limb| row[CHALLENGE + limb])
}

pub fn post_state_from_c9_extension_cols(row: &[Val; C9_EXTENSION_COLS]) -> [Val; WIDTH] {
    core::array::from_fn(|lane| row[POST_STATE + lane])
}

pub struct C9ExtensionAir {
    pub source: C9ExtensionSource,
}

impl<F: Sync> BaseAir<F> for C9ExtensionAir {
    fn width(&self) -> usize {
        C9_EXTENSION_COLS
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for C9ExtensionAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let row: Vec<AB::Var> = main.current_slice().to_vec();
        let cells = RowCells::<AB> { row };
        let c = ConstantsExpr::<AB>::pinned();

        let prior = |lane: usize| cells.at(PRIOR + lane);
        let absorbed = |lane: usize| cells.at(ABSORBED + lane);
        let p0_input = |lane: usize| cells.at(PERM0 + C_INPUTS + lane);
        let p0_output = |lane: usize| cells.at(PERM0 + C_OUTPUT + lane);
        let p1_input = |lane: usize| cells.at(PERM1 + C_INPUTS + lane);
        let p1_output = |lane: usize| cells.at(PERM1 + C_OUTPUT + lane);

        match self.source {
            C9ExtensionSource::ReadyOutput => {
                for lane in 0..C9_RATE {
                    builder.assert_zero(absorbed(lane));
                }
                for lane in 0..WIDTH {
                    builder.assert_eq(p0_input(lane), prior(lane));
                }
            }
            C9ExtensionSource::PendingAbsorb(n) => {
                assert!((1..C9_RATE).contains(&n));
                for lane in 0..n {
                    builder.assert_eq(p0_input(lane), absorbed(lane));
                }
                for lane in n..C9_RATE {
                    builder.assert_zero(p0_input(lane));
                    builder.assert_zero(absorbed(lane));
                }
                builder.assert_eq(
                    p0_input(C9_RATE),
                    prior(C9_RATE) + AB::Expr::from(Val::from_usize(n)),
                );
                for lane in C9_RATE + 1..WIDTH {
                    builder.assert_eq(p0_input(lane), prior(lane));
                }
            }
        }

        eval_perm(builder, &cells, PERM0, &c);
        for lane in 0..WIDTH {
            builder.assert_eq(p1_input(lane), p0_output(lane));
        }
        eval_perm(builder, &cells, PERM1, &c);

        match self.source {
            C9ExtensionSource::ReadyOutput => {
                for limb in 0..C9_RATE {
                    builder.assert_eq(cells.at(CHALLENGE + limb), prior(C9_RATE - 1 - limb));
                }
                builder.assert_eq(cells.at(CHALLENGE + 7), p0_output(C9_RATE - 1));
                for lane in 0..WIDTH {
                    builder.assert_eq(cells.at(POST_STATE + lane), p0_output(lane));
                }
            }
            C9ExtensionSource::PendingAbsorb(_) => {
                for limb in 0..C9_RATE {
                    builder.assert_eq(cells.at(CHALLENGE + limb), p0_output(C9_RATE - 1 - limb));
                }
                builder.assert_eq(cells.at(CHALLENGE + 7), p1_output(C9_RATE - 1));
                for lane in 0..WIDTH {
                    builder.assert_eq(cells.at(POST_STATE + lane), p1_output(lane));
                }
            }
        }

        let _ = C9_CAPACITY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c9_challenger::C9Challenger;
    use crate::inlined_perm;
    use p3_air::check_all_constraints;
    use p3_matrix::dense::RowMajorMatrix;

    fn state(seed: u32) -> [Val; WIDTH] {
        core::array::from_fn(|lane| {
            Val::from_u32(seed.wrapping_mul(0x9e37_79b9).wrapping_add(lane as u32 + 1))
        })
    }

    fn absorbed(seed: u32) -> [Val; C9_RATE] {
        core::array::from_fn(|lane| Val::from_u32(seed + 17 * lane as u32 + 1))
    }

    fn assert_honest(source: C9ExtensionSource) {
        let prior = state(7);
        let values = match source {
            C9ExtensionSource::ReadyOutput => [Val::ZERO; C9_RATE],
            C9ExtensionSource::PendingAbsorb(_) => absorbed(90),
        };
        let row = generate_c9_extension_cols(prior, values, source);
        let matrix = RowMajorMatrix::new(row.to_vec(), C9_EXTENSION_COLS);
        let report = check_all_constraints(&C9ExtensionAir { source }, &matrix, &[], None);
        assert!(report.is_ok(), "honest c9 extension gadget failed");

        let mut native = match source {
            C9ExtensionSource::ReadyOutput => C9Challenger::from_ready_output_state(prior),
            C9ExtensionSource::PendingAbsorb(_) => C9Challenger::from_state(prior),
        };
        if let C9ExtensionSource::PendingAbsorb(n) = source {
            native.observe_slice(&values[..n]);
        }
        assert_eq!(
            challenge_from_c9_extension_cols(&row),
            native.sample_extension()
        );
        assert_eq!(post_state_from_c9_extension_cols(&row), native.state());
    }

    #[test]
    fn air_accepts_ready_output_extension_sample() {
        assert_honest(C9ExtensionSource::ReadyOutput);
    }

    #[test]
    fn air_accepts_pending_absorb_extension_sample() {
        for n in 1..C9_RATE {
            assert_honest(C9ExtensionSource::PendingAbsorb(n));
        }
    }

    #[test]
    fn air_rejects_tampered_eighth_limb() {
        let source = C9ExtensionSource::PendingAbsorb(6);
        let mut row = generate_c9_extension_cols(state(11), absorbed(33), source);
        row[CHALLENGE + 7] += Val::ONE;
        let matrix = RowMajorMatrix::new(row.to_vec(), C9_EXTENSION_COLS);
        assert!(!check_all_constraints(&C9ExtensionAir { source }, &matrix, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_squeeze_permutation() {
        let source = C9ExtensionSource::PendingAbsorb(6);
        let mut row = generate_c9_extension_cols(state(19), absorbed(44), source);
        row[PERM1 + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let matrix = RowMajorMatrix::new(row.to_vec(), C9_EXTENSION_COLS);
        assert!(!check_all_constraints(&C9ExtensionAir { source }, &matrix, &[], None).is_ok());
    }
}
