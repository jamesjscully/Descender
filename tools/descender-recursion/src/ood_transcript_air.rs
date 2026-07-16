//! Vertical Fiat–Shamir accumulator for the production OOD opened-values stream.
//!
//! A production account proof observes roughly 145k base-field elements between the `zeta`
//! sample and the FRI batch-combination challenge.  [`crate::recursive_air`] can inline a bounded
//! number of these rate-7 duplex rounds horizontally, but materializing the production stream that
//! way would make a member window tens of thousands of Poseidon2 blocks wide.  This AIR puts one
//! absorb-permute round on each trace row instead.  Its width is constant; the first pre-absorb
//! state and final post-permutation state are public, and every active row is chained to the next.
//! Each row carries distinct transcript-side and PCS-side value columns. Their exact active-lane
//! equality is the ordered bus that prevents the challenger from observing a different sequence
//! than the PCS reduced-opening consumer.

use crate::challenger::RATE;
use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_INPUTS, C_OUTPUT, PERM_COLS,
};
use descender_account_stark::{Val, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub const C_ACTIVE: usize = 0;
pub const C_LAST: usize = 1;
pub const C_POS: usize = 2;
pub const C_STATE: usize = 3;
pub const C_BLOCK: usize = C_STATE + WIDTH;
pub const C_PCS_BLOCK: usize = C_BLOCK + RATE;
pub const C_PERM: usize = C_PCS_BLOCK + RATE;
pub const N_COLS: usize = C_PERM + PERM_COLS;

pub const PV_START: usize = 0;
pub const PV_END: usize = PV_START + WIDTH;
pub const N_PUBLIC: usize = PV_END + WIDTH;

/// One vertical transcript instance. `rounds = ceil(stream_len / RATE)` and `final_n` is the
/// meaningful lane count of the last round (`RATE` for an aligned, non-empty stream).
#[derive(Clone, Copy, Debug)]
pub struct OodTranscriptAir {
    pub rounds: usize,
    pub final_n: usize,
}

impl OodTranscriptAir {
    pub fn new(stream_len: usize) -> Self {
        assert!(stream_len > 0, "OOD transcript stream must be non-empty");
        let rounds = stream_len.div_ceil(RATE);
        let rem = stream_len % RATE;
        Self {
            rounds,
            final_n: if rem == 0 { RATE } else { rem },
        }
    }
}

impl<F: Sync> BaseAir<F> for OodTranscriptAir {
    fn width(&self) -> usize {
        N_COLS
    }

    fn num_public_values(&self) -> usize {
        N_PUBLIC
    }
}

struct RowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}

impl<AB: AirBuilder<F = Val>> PermCells<AB> for RowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for OodTranscriptAir {
    fn eval(&self, builder: &mut AB) {
        assert!(self.rounds > 0);
        assert!((1..=RATE).contains(&self.final_n));

        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let public: Vec<AB::Expr> = builder
            .public_values()
            .iter()
            .map(|v| (*v).into())
            .collect();
        let one = AB::Expr::ONE;
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let n = |i: usize| -> AB::Expr { next[i].into() };
        let active = l(C_ACTIVE);
        let last = l(C_LAST);

        builder.assert_zero(active.clone() * (active.clone() - one.clone()));
        builder.assert_zero(last.clone() * (last.clone() - one.clone()));
        builder.assert_zero(last.clone() * (one.clone() - active.clone()));

        // Inactive padding is canonical and cannot smuggle transcript data.
        let inactive = one.clone() - active.clone();
        builder.assert_zero(inactive.clone() * l(C_POS));
        for i in 0..WIDTH {
            builder.assert_zero(inactive.clone() * l(C_STATE + i));
        }
        for i in 0..RATE {
            builder.assert_zero(inactive.clone() * l(C_BLOCK + i));
            builder.assert_zero(inactive.clone() * l(C_PCS_BLOCK + i));
        }

        // The permutation input is the exact DuplexChallenger absorb state. Rate lanes are
        // overwritten by the observed block (unused tail lanes are zero); capacity lane RATE gets
        // the length tag, and the remaining capacity is carried from the pre-state.
        for i in 0..RATE {
            let used_on_last = if i < self.final_n {
                one.clone()
            } else {
                one.clone() - last.clone()
            };
            let expected = active.clone() * used_on_last.clone() * l(C_BLOCK + i);
            builder.assert_zero(l(C_PERM + C_INPUTS + i) - expected);
            builder
                .assert_zero(active.clone() * used_on_last * (l(C_BLOCK + i) - l(C_PCS_BLOCK + i)));
            if i >= self.final_n {
                builder.assert_zero(last.clone() * l(C_BLOCK + i));
                builder.assert_zero(last.clone() * l(C_PCS_BLOCK + i));
            }
        }
        let round_n = AB::Expr::from(Val::from_usize(RATE))
            + last.clone()
                * (AB::Expr::from(Val::from_usize(self.final_n))
                    - AB::Expr::from(Val::from_usize(RATE)));
        builder.assert_zero(
            l(C_PERM + C_INPUTS + RATE) - active.clone() * (l(C_STATE + RATE) + round_n),
        );
        for i in (RATE + 1)..WIDTH {
            builder.assert_zero(l(C_PERM + C_INPUTS + i) - active.clone() * l(C_STATE + i));
        }

        let cells = RowCells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();
        eval_perm(builder, &cells, C_PERM, &consts);

        // Exact active prefix and state carry. Position makes it impossible to terminate early.
        {
            let mut first = builder.when_first_row();
            first.assert_zero(active.clone() - one.clone());
            first.assert_zero(l(C_POS));
            if public.len() == N_PUBLIC {
                for i in 0..WIDTH {
                    first.assert_zero(l(C_STATE + i) - public[PV_START + i].clone());
                }
            }
        }
        {
            let mut tr = builder.when_transition();
            tr.assert_zero(n(C_ACTIVE) - (active.clone() - last.clone()));
            tr.assert_zero(n(C_POS) - n(C_ACTIVE) * (l(C_POS) + one.clone()));
            for i in 0..WIDTH {
                let out = l(C_PERM + C_OUTPUT + i);
                tr.assert_zero(n(C_ACTIVE) * (n(C_STATE + i) - out));
            }
        }
        {
            let mut end = builder.when_last_row();
            end.assert_zero(last.clone() - active.clone());
        }
        builder.assert_zero(
            last.clone() * (l(C_POS) - AB::Expr::from(Val::from_usize(self.rounds - 1))),
        );
        if public.len() == N_PUBLIC {
            for i in 0..WIDTH {
                builder.assert_zero(
                    last.clone() * (l(C_PERM + C_OUTPUT + i) - public[PV_END + i].clone()),
                );
            }
        }
    }
}

fn absorb(state: [Val; WIDTH], block: [Val; RATE], n: usize) -> [Val; WIDTH] {
    let mut input = state;
    input[..n].copy_from_slice(&block[..n]);
    input[n..RATE].fill(Val::ZERO);
    input[RATE] += Val::from_usize(n);
    input
}

/// Build the canonical power-of-two trace and return it with the final sponge state.
pub fn build_trace(start: [Val; WIDTH], stream: &[Val]) -> (RowMajorMatrix<Val>, [Val; WIDTH]) {
    build_bound_trace(start, stream, stream)
}

/// Build a trace from independently supplied transcript and PCS opened-value streams. Shape is
/// checked here; value equality and ordering are enforced by the AIR.
pub fn build_bound_trace(
    start: [Val; WIDTH],
    transcript_stream: &[Val],
    pcs_stream: &[Val],
) -> (RowMajorMatrix<Val>, [Val; WIDTH]) {
    assert_eq!(
        transcript_stream.len(),
        pcs_stream.len(),
        "transcript and PCS streams must have the same declared shape"
    );
    let cfg = OodTranscriptAir::new(transcript_stream.len());
    let height = cfg.rounds.next_power_of_two().max(2);
    let consts = inlined_perm::Constants::pinned();
    let mut flat = vec![Val::ZERO; height * N_COLS];
    let mut state = start;
    for r in 0..cfg.rounds {
        let row = &mut flat[r * N_COLS..(r + 1) * N_COLS];
        row[C_ACTIVE] = Val::ONE;
        row[C_LAST] = Val::from_bool(r + 1 == cfg.rounds);
        row[C_POS] = Val::from_usize(r);
        row[C_STATE..C_STATE + WIDTH].copy_from_slice(&state);
        let begin = r * RATE;
        let n = (transcript_stream.len() - begin).min(RATE);
        row[C_BLOCK..C_BLOCK + n].copy_from_slice(&transcript_stream[begin..begin + n]);
        row[C_PCS_BLOCK..C_PCS_BLOCK + n].copy_from_slice(&pcs_stream[begin..begin + n]);
        let block: [Val; RATE] = row[C_BLOCK..C_BLOCK + RATE].try_into().unwrap();
        let perm = inlined_perm::generate_perm_cols(absorb(state, block, n), &consts);
        row[C_PERM..C_PERM + PERM_COLS].copy_from_slice(&perm);
        state = inlined_perm::perm_output(&perm);
    }
    // Permutation constraints are unconditional, so inactive padding uses a valid zero-input perm.
    let pad_perm = inlined_perm::generate_perm_cols([Val::ZERO; WIDTH], &consts);
    for r in cfg.rounds..height {
        flat[r * N_COLS + C_PERM..r * N_COLS + C_PERM + PERM_COLS].copy_from_slice(&pad_perm);
    }
    (RowMajorMatrix::new(flat, N_COLS), state)
}

pub fn public_values(start: [Val; WIDTH], end: [Val; WIDTH]) -> Vec<Val> {
    start.into_iter().chain(end).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_koala_bear::default_koalabear_poseidon2_16;
    use p3_symmetric::Permutation;

    #[test]
    fn vertical_ood_accumulator_matches_native_and_rejects_tamper() {
        let start = core::array::from_fn(|i| Val::from_usize(1000 + i));
        let stream: Vec<Val> = (0..(RATE * 5 + 3))
            .map(|i| Val::from_usize(0x5000 + i))
            .collect();
        let (matrix, end) = build_trace(start, &stream);
        let cfg = OodTranscriptAir::new(stream.len());
        let pv = public_values(start, end);
        assert!(check_all_constraints(&cfg, &matrix, &pv, None).is_ok());

        let mut native_state = start;
        let perm = default_koalabear_poseidon2_16();
        for chunk in stream.chunks(RATE) {
            let mut block = [Val::ZERO; RATE];
            block[..chunk.len()].copy_from_slice(chunk);
            native_state = absorb(native_state, block, chunk.len());
            perm.permute_mut(&mut native_state);
        }
        assert_eq!(end, native_state);

        let mut bad = matrix.values.clone();
        bad[C_BLOCK] += Val::ONE;
        let bad = RowMajorMatrix::new(bad, N_COLS);
        assert!(!check_all_constraints(&cfg, &bad, &pv, None).is_ok());

        // A coherently regenerated trace for a different observed stream still cannot retain the
        // honest terminal-state public value.
        let mut changed_stream = stream.clone();
        changed_stream[0] += Val::ONE;
        let (changed, changed_end) = build_trace(start, &changed_stream);
        assert_ne!(changed_end, end);
        assert!(!check_all_constraints(&cfg, &changed, &pv, None).is_ok());

        let (substituted, substituted_end) = build_bound_trace(start, &stream, &changed_stream);
        assert_eq!(substituted_end, end, "transcript side stayed honest");
        assert!(
            !check_all_constraints(&cfg, &substituted, &pv, None).is_ok(),
            "PCS-side value substitution escaped the ordered equality bus"
        );

        let mut reordered = stream.clone();
        reordered.swap(1, 2);
        let (reordered, reordered_end) = build_bound_trace(start, &stream, &reordered);
        assert_eq!(reordered_end, end, "transcript side stayed honest");
        assert!(
            !check_all_constraints(&cfg, &reordered, &pv, None).is_ok(),
            "PCS-side reordering escaped the ordered equality bus"
        );
    }

    #[test]
    fn vertical_ood_accumulator_proves_at_production_q128_knobs() {
        use p3_uni_stark::{prove, verify};

        let start = core::array::from_fn(|i| Val::from_usize(3000 + i));
        let stream: Vec<Val> = (0..(RATE * 4 + 5))
            .map(|i| Val::from_usize(0x7000 + i * 3))
            .collect();
        let (trace, end) = build_trace(start, &stream);
        let pv = public_values(start, end);
        let air = OodTranscriptAir::new(stream.len());
        let config = crate::settlement::prod_config();
        let proof = prove(&config, &air, trace, &pv);
        verify(&config, &air, &proof, &pv)
            .expect("vertical OOD accumulator proof must verify at q128 knobs");
    }
}
