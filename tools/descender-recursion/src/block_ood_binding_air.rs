//! Constant-width block companion for vertically bound production OOD streams.
//!
//! For each member this AIR (1) duplexes the ordered transcript/PCS-claim-equal OOD stream from the
//! member's post-zeta state, (2) computes `MyHash(start || end)`, (3) augments the independently
//! verified member root with that endpoint digest, and (4) folds the augmented root into the same
//! block chain used by [`crate::block_recursive_air::BlockRecursiveAir`]. A settlement verifier
//! accepts both O(1) proofs against one public chain root, linking the vertical stream to the
//! recursive member without widening a member row by thousands of permutations.
//!
//! The `pcs_stream` columns are an ordered claim-side bus, not by themselves a PCS verifier. Full
//! soundness additionally requires the production opening-reduction consumer to authenticate those
//! claim-side values. Settlement composition and endpoint binding here deliberately do not claim to
//! discharge that remaining proof-family obligation.

use crate::challenger::RATE;
use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_INPUTS, C_OUTPUT, PERM_COLS,
};
use crate::leaf_hash::{leaf_hash_digest, LEAF_HASH_RATE};
use crate::merkle_air::{node_blocks, ABSORB_LEN};
use descender_account_stark::{Val, DIGEST_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;

const N_EH: usize = (2 * WIDTH).div_ceil(LEAF_HASH_RATE);
const N_NODE: usize = (2 * DIGEST_ELEMS).div_ceil(LEAF_HASH_RATE);
const N_SEL: usize = 1 + N_EH + N_NODE + N_NODE + 1;
const S_OOD: usize = 0;
const S_EH: usize = 1;
const S_AUG: usize = S_EH + N_EH;
const S_CHAIN: usize = S_AUG + N_NODE;
const S_PAD: usize = S_CHAIN + N_NODE;

const C_SEL: usize = 0;
const C_OOD_LAST: usize = C_SEL + N_SEL;
const C_OOD_POS: usize = C_OOD_LAST + 1;
const C_OOD_INV: usize = C_OOD_POS + 1;
const C_FINAL: usize = C_OOD_INV + 1;
const C_FINAL_INV: usize = C_FINAL + 1;
const C_MEMBER: usize = C_FINAL_INV + 1;
const C_STATE: usize = C_MEMBER + 1;
const C_START: usize = C_STATE + WIDTH;
const C_END: usize = C_START + WIDTH;
const C_BLOCK: usize = C_END + WIDTH;
const C_PCS_BLOCK: usize = C_BLOCK + RATE;
const C_MEMBER_ROOT: usize = C_PCS_BLOCK + RATE;
const C_CHAIN_IN: usize = C_MEMBER_ROOT + DIGEST_ELEMS;
const C_ENDPOINT_DIGEST: usize = C_CHAIN_IN + DIGEST_ELEMS;
const C_AUG_ROOT: usize = C_ENDPOINT_DIGEST + DIGEST_ELEMS;
const C_PERM: usize = C_AUG_ROOT + DIGEST_ELEMS;
pub const N_COLS: usize = C_PERM + PERM_COLS;
pub const N_PUBLIC: usize = DIGEST_ELEMS;
/// Current relation-v3 production proof's flattened OOD claim stream. The genuine-proof capstone
/// pins this value so relation/config changes force an explicit sizing/KAT update.
pub const PRODUCTION_OOD_STREAM_ELEMS_V1: usize = 186_032;

#[derive(Clone, Copy, Debug)]
pub struct BlockOodBindingAir {
    pub members: usize,
    pub stream_len: usize,
}

impl BlockOodBindingAir {
    pub fn new(members: usize, stream_len: usize) -> Self {
        assert!(members > 0 && stream_len > 0);
        Self {
            members,
            stream_len,
        }
    }
    pub fn ood_rounds(self) -> usize {
        self.stream_len.div_ceil(RATE)
    }
    pub fn final_n(self) -> usize {
        let r = self.stream_len % RATE;
        if r == 0 {
            RATE
        } else {
            r
        }
    }
    pub fn active_rows_per_member(self) -> usize {
        self.ood_rounds() + N_EH + 2 * N_NODE
    }

    pub fn trace_height(self) -> usize {
        (self.active_rows_per_member() * self.members)
            .next_power_of_two()
            .max(2)
    }
}

impl<F: Sync> BaseAir<F> for BlockOodBindingAir {
    fn width(&self) -> usize {
        N_COLS
    }
    fn num_public_values(&self) -> usize {
        N_PUBLIC
    }
}

struct Cells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> PermCells<AB> for Cells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

fn hash_source_expr<AB: AirBuilder<F = Val>>(
    l: &impl Fn(usize) -> AB::Expr,
    selector: usize,
    lane: usize,
) -> Option<AB::Expr> {
    let (phase, k) = if selector >= S_EH && selector < S_AUG {
        (0, selector - S_EH)
    } else if selector >= S_AUG && selector < S_CHAIN {
        (1, selector - S_AUG)
    } else if selector >= S_CHAIN && selector < S_PAD {
        (2, selector - S_CHAIN)
    } else {
        return None;
    };
    let g = k * LEAF_HASH_RATE + lane;
    match phase {
        0 if g < 2 * WIDTH => Some(if g < WIDTH {
            l(C_START + g)
        } else {
            l(C_END + g - WIDTH)
        }),
        1 if g < 2 * DIGEST_ELEMS => Some(if g < DIGEST_ELEMS {
            l(C_MEMBER_ROOT + g)
        } else {
            l(C_ENDPOINT_DIGEST + g - DIGEST_ELEMS)
        }),
        2 if g < 2 * DIGEST_ELEMS => Some(if g < DIGEST_ELEMS {
            l(C_CHAIN_IN + g)
        } else {
            l(C_AUG_ROOT + g - DIGEST_ELEMS)
        }),
        _ => None,
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for BlockOodBindingAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder
            .public_values()
            .iter()
            .map(|p| (*p).into())
            .collect();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let n = |i: usize| -> AB::Expr { next[i].into() };
        let one = AB::Expr::ONE;
        let sel = |s: usize| -> AB::Expr { l(C_SEL + s) };

        let mut sum = AB::Expr::ZERO;
        for s in 0..N_SEL {
            builder.assert_zero(sel(s) * (sel(s) - one.clone()));
            sum += sel(s);
        }
        builder.assert_zero(sum - one.clone());
        let active = one.clone() - sel(S_PAD);
        let ood = sel(S_OOD);
        let ood_last = l(C_OOD_LAST);
        builder.assert_zero(ood_last.clone() * (ood_last.clone() - one.clone()));
        builder.assert_zero(ood_last.clone() * (one.clone() - ood.clone()));
        let delta = l(C_OOD_POS) - AB::Expr::from(Val::from_usize(self.ood_rounds() - 1));
        builder.assert_zero(delta.clone() * ood_last.clone());
        builder.assert_zero(delta * l(C_OOD_INV) - (ood.clone() - ood_last.clone()));

        let final_sel = l(C_FINAL);
        builder.assert_zero(final_sel.clone() * (final_sel.clone() - one.clone()));
        builder.assert_zero(final_sel.clone() * (one.clone() - sel(S_CHAIN + N_NODE - 1)));
        let member_delta = l(C_MEMBER) - AB::Expr::from(Val::from_usize(self.members - 1));
        builder.assert_zero(member_delta.clone() * final_sel.clone());
        builder.assert_zero(
            member_delta * l(C_FINAL_INV) - (sel(S_CHAIN + N_NODE - 1) - final_sel.clone()),
        );

        let inactive = sel(S_PAD);
        builder.assert_zero(inactive.clone() * l(C_OOD_POS));
        builder.assert_zero(inactive.clone() * l(C_MEMBER));
        for i in C_STATE..C_PERM {
            builder.assert_zero(inactive.clone() * l(i));
        }

        // One real Poseidon2 permutation on every row; padding uses the zero permutation.
        let cells = Cells::<AB> { row: local.clone() };
        let consts = ConstantsExpr::<AB>::pinned();
        eval_perm(builder, &cells, C_PERM, &consts);

        // OOD duplex row and transcript/PCS ordered equality.
        let round_n = AB::Expr::from(Val::from_usize(RATE))
            + ood_last.clone()
                * (AB::Expr::from(Val::from_usize(self.final_n()))
                    - AB::Expr::from(Val::from_usize(RATE)));
        for i in 0..RATE {
            let used = if i < self.final_n() {
                one.clone()
            } else {
                one.clone() - ood_last.clone()
            };
            builder.assert_zero(
                ood.clone() * (l(C_PERM + C_INPUTS + i) - used.clone() * l(C_BLOCK + i)),
            );
            builder.assert_zero(ood.clone() * used * (l(C_BLOCK + i) - l(C_PCS_BLOCK + i)));
            if i >= self.final_n() {
                builder.assert_zero(ood_last.clone() * l(C_BLOCK + i));
                builder.assert_zero(ood_last.clone() * l(C_PCS_BLOCK + i));
            }
        }
        builder
            .assert_zero(ood.clone() * (l(C_PERM + C_INPUTS + RATE) - l(C_STATE + RATE) - round_n));
        for i in (RATE + 1)..WIDTH {
            builder.assert_zero(ood.clone() * (l(C_PERM + C_INPUTS + i) - l(C_STATE + i)));
        }
        for i in 0..WIDTH {
            builder.assert_zero(ood_last.clone() * (l(C_END + i) - l(C_PERM + C_OUTPUT + i)));
        }

        // PaddingFreeSponge rows for endpoint, augmented-root, and chain hashes.
        for s in S_EH..S_PAD {
            let gate = sel(s);
            let first = s == S_EH || s == S_AUG || s == S_CHAIN;
            for i in 0..WIDTH {
                let input = l(C_PERM + C_INPUTS + i);
                if i < LEAF_HASH_RATE {
                    if let Some(source) = hash_source_expr::<AB>(&l, s, i) {
                        builder.assert_zero(gate.clone() * (input - source));
                    } else {
                        builder.assert_zero(gate.clone() * (input - l(C_STATE + i)));
                    }
                } else {
                    builder.assert_zero(gate.clone() * (input - l(C_STATE + i)));
                }
                if first {
                    builder.assert_zero(gate.clone() * l(C_STATE + i));
                }
            }
        }
        let eh_last = sel(S_EH + N_EH - 1);
        let aug_last = sel(S_AUG + N_NODE - 1);
        for i in 0..DIGEST_ELEMS {
            builder.assert_zero(
                eh_last.clone() * (l(C_ENDPOINT_DIGEST + i) - l(C_PERM + C_OUTPUT + i)),
            );
            builder.assert_zero(aug_last.clone() * (l(C_AUG_ROOT + i) - l(C_PERM + C_OUTPUT + i)));
        }

        // Boundaries and deterministic phase transitions.
        {
            let mut fb = builder.when_first_row();
            fb.assert_zero(sel(S_OOD) - one.clone());
            fb.assert_zero(l(C_OOD_POS));
            fb.assert_zero(l(C_MEMBER));
            for i in 0..DIGEST_ELEMS {
                fb.assert_zero(l(C_CHAIN_IN + i));
            }
            for i in 0..WIDTH {
                fb.assert_zero(l(C_STATE + i) - l(C_START + i));
            }
        }
        let mut tr = builder.when_transition();
        let not_final = one.clone() - final_sel.clone();
        tr.assert_zero(
            n(C_SEL + S_OOD)
                - not_final.clone()
                    * (ood.clone() * (one.clone() - ood_last.clone()) + sel(S_CHAIN + N_NODE - 1)),
        );
        tr.assert_zero(n(C_SEL + S_EH) - ood_last.clone());
        for k in 1..N_EH {
            tr.assert_zero(n(C_SEL + S_EH + k) - sel(S_EH + k - 1));
        }
        tr.assert_zero(n(C_SEL + S_AUG) - sel(S_EH + N_EH - 1));
        for k in 1..N_NODE {
            tr.assert_zero(n(C_SEL + S_AUG + k) - sel(S_AUG + k - 1));
        }
        tr.assert_zero(n(C_SEL + S_CHAIN) - sel(S_AUG + N_NODE - 1));
        for k in 1..N_NODE {
            tr.assert_zero(n(C_SEL + S_CHAIN + k) - sel(S_CHAIN + k - 1));
        }
        tr.assert_zero(n(C_SEL + S_PAD) - (sel(S_PAD) + final_sel.clone()));
        tr.assert_zero(
            n(C_OOD_POS)
                - ood.clone() * (one.clone() - ood_last.clone()) * (l(C_OOD_POS) + one.clone()),
        );
        tr.assert_zero(n(C_MEMBER) - not_final.clone() * (l(C_MEMBER) + sel(S_CHAIN + N_NODE - 1)));

        // Carry permutation state inside OOD/hash phases; start a new hash phase at zero and a new
        // member at its committed start. Metadata is constant until the chain phase completes.
        let carry_state = ood.clone() * (one.clone() - ood_last.clone())
            + (S_EH..S_EH + N_EH - 1)
                .map(|s| sel(s))
                .fold(AB::Expr::ZERO, |a, b| a + b)
            + (S_AUG..S_AUG + N_NODE - 1)
                .map(|s| sel(s))
                .fold(AB::Expr::ZERO, |a, b| a + b)
            + (S_CHAIN..S_CHAIN + N_NODE - 1)
                .map(|s| sel(s))
                .fold(AB::Expr::ZERO, |a, b| a + b);
        for i in 0..WIDTH {
            tr.assert_zero(carry_state.clone() * (n(C_STATE + i) - l(C_PERM + C_OUTPUT + i)));
            tr.assert_zero(
                sel(S_CHAIN + N_NODE - 1) * not_final.clone() * (n(C_STATE + i) - n(C_START + i)),
            );
        }
        let same_member = active.clone() * (one.clone() - sel(S_CHAIN + N_NODE - 1));
        for i in C_START..C_PERM {
            if (C_BLOCK..C_PCS_BLOCK + RATE).contains(&i) || (C_STATE..C_START).contains(&i) {
                continue;
            }
            tr.assert_zero(same_member.clone() * (n(i) - l(i)));
        }
        for i in 0..DIGEST_ELEMS {
            tr.assert_zero(
                sel(S_CHAIN + N_NODE - 1)
                    * not_final.clone()
                    * (n(C_CHAIN_IN + i) - l(C_PERM + C_OUTPUT + i)),
            );
        }

        if pubs.len() == N_PUBLIC {
            for i in 0..DIGEST_ELEMS {
                builder
                    .assert_zero(final_sel.clone() * (l(C_PERM + C_OUTPUT + i) - pubs[i].clone()));
            }
        }
    }
}

fn absorb(mut state: [Val; WIDTH], block: [Val; RATE], n: usize) -> [Val; WIDTH] {
    state[..n].copy_from_slice(&block[..n]);
    state[n..RATE].fill(Val::ZERO);
    state[RATE] += Val::from_usize(n);
    state
}

/// Native terminal state of the exact rate-7 OOD duplex schedule proven by this AIR.
pub fn terminal_state(start: [Val; WIDTH], stream: &[Val]) -> [Val; WIDTH] {
    assert!(
        !stream.is_empty(),
        "OOD transcript stream must be non-empty"
    );
    let consts = inlined_perm::Constants::pinned();
    let mut state = start;
    for chunk in stream.chunks(RATE) {
        let mut block = [Val::ZERO; RATE];
        block[..chunk.len()].copy_from_slice(chunk);
        let cols = inlined_perm::generate_perm_cols(absorb(state, block, chunk.len()), &consts);
        state = inlined_perm::perm_output(&cols);
    }
    state
}
fn padding_absorb(mut state: [Val; WIDTH], input: &[Val], k: usize) -> [Val; WIDTH] {
    let begin = k * LEAF_HASH_RATE;
    let n = (input.len() - begin).min(LEAF_HASH_RATE);
    state[..n].copy_from_slice(&input[begin..begin + n]);
    state
}

pub struct MemberOodInput<'a> {
    pub member_root: [Val; DIGEST_ELEMS],
    pub start: [Val; WIDTH],
    pub transcript_stream: &'a [Val],
    pub pcs_stream: &'a [Val],
}

/// The member root consumed by block settlement after binding the independently verified member
/// root to the vertical OOD transcript's exact start and terminal states.
pub fn augmented_member_root(
    member_root: [Val; DIGEST_ELEMS],
    start: [Val; WIDTH],
    end: [Val; WIDTH],
) -> [Val; DIGEST_ELEMS] {
    let endpoint_input: Vec<Val> = start.into_iter().chain(end).collect();
    let endpoint_digest = leaf_hash_digest(&endpoint_input);
    let mut combined = [Val::ZERO; ABSORB_LEN];
    combined[..DIGEST_ELEMS].copy_from_slice(&member_root);
    combined[DIGEST_ELEMS..].copy_from_slice(&endpoint_digest);
    node_blocks(&combined, &inlined_perm::Constants::pinned()).1
}

fn fill_common(
    row: &mut [Val],
    member: usize,
    input: &MemberOodInput<'_>,
    end: &[Val; WIDTH],
    endpoint_digest: &[Val; DIGEST_ELEMS],
    aug_root: &[Val; DIGEST_ELEMS],
    chain_in: &[Val; DIGEST_ELEMS],
) {
    row[C_MEMBER] = Val::from_usize(member);
    row[C_START..C_START + WIDTH].copy_from_slice(&input.start);
    row[C_END..C_END + WIDTH].copy_from_slice(end);
    row[C_MEMBER_ROOT..C_MEMBER_ROOT + DIGEST_ELEMS].copy_from_slice(&input.member_root);
    row[C_CHAIN_IN..C_CHAIN_IN + DIGEST_ELEMS].copy_from_slice(chain_in);
    row[C_ENDPOINT_DIGEST..C_ENDPOINT_DIGEST + DIGEST_ELEMS].copy_from_slice(endpoint_digest);
    row[C_AUG_ROOT..C_AUG_ROOT + DIGEST_ELEMS].copy_from_slice(aug_root);
}

pub fn build_trace(inputs: &[MemberOodInput<'_>]) -> (RowMajorMatrix<Val>, [Val; DIGEST_ELEMS]) {
    assert!(!inputs.is_empty());
    let stream_len = inputs[0].transcript_stream.len();
    let cfg = BlockOodBindingAir::new(inputs.len(), stream_len);
    let active_rows = cfg.active_rows_per_member() * inputs.len();
    let height = cfg.trace_height();
    let mut flat = vec![Val::ZERO; height * N_COLS];
    let consts = inlined_perm::Constants::pinned();
    let mut chain = [Val::ZERO; DIGEST_ELEMS];
    let mut row_idx = 0;
    for (member, input) in inputs.iter().enumerate() {
        assert_eq!(input.transcript_stream.len(), stream_len);
        assert_eq!(input.pcs_stream.len(), stream_len);

        // Compute this member's entire committed metadata tuple before writing any row. The AIR
        // then carries the tuple unchanged through all phases and independently re-derives each
        // component at its phase boundary.
        let computed_end = terminal_state(input.start, input.transcript_stream);
        let endpoint_input: Vec<Val> = input.start.into_iter().chain(computed_end).collect();
        let endpoint_digest = leaf_hash_digest(&endpoint_input);
        let mut aug_input = [Val::ZERO; ABSORB_LEN];
        aug_input[..DIGEST_ELEMS].copy_from_slice(&input.member_root);
        aug_input[DIGEST_ELEMS..].copy_from_slice(&endpoint_digest);
        let aug = node_blocks(&aug_input, &consts).1;
        let chain_in = chain;
        let mut chain_input = [Val::ZERO; ABSORB_LEN];
        chain_input[..DIGEST_ELEMS].copy_from_slice(&chain_in);
        chain_input[DIGEST_ELEMS..].copy_from_slice(&aug);
        let chain_out = node_blocks(&chain_input, &consts).1;

        let mut state = input.start;
        for r in 0..cfg.ood_rounds() {
            let row = &mut flat[row_idx * N_COLS..(row_idx + 1) * N_COLS];
            fill_common(
                row,
                member,
                input,
                &computed_end,
                &endpoint_digest,
                &aug,
                &chain_in,
            );
            row[C_SEL + S_OOD] = Val::ONE;
            row[C_OOD_POS] = Val::from_usize(r);
            let last = r + 1 == cfg.ood_rounds();
            row[C_OOD_LAST] = Val::from_bool(last);
            let d = Val::from_usize(r) - Val::from_usize(cfg.ood_rounds() - 1);
            if !last {
                row[C_OOD_INV] = d.inverse();
            }
            row[C_STATE..C_STATE + WIDTH].copy_from_slice(&state);
            let begin = r * RATE;
            let n = (stream_len - begin).min(RATE);
            row[C_BLOCK..C_BLOCK + n].copy_from_slice(&input.transcript_stream[begin..begin + n]);
            row[C_PCS_BLOCK..C_PCS_BLOCK + n].copy_from_slice(&input.pcs_stream[begin..begin + n]);
            let block = row[C_BLOCK..C_BLOCK + RATE].try_into().unwrap();
            let perm = inlined_perm::generate_perm_cols(absorb(state, block, n), &consts);
            row[C_PERM..C_PERM + PERM_COLS].copy_from_slice(&perm);
            state = inlined_perm::perm_output(&perm);
            row_idx += 1;
        }
        let end = state;
        debug_assert_eq!(end, computed_end);
        let endpoint = endpoint_input;
        state = [Val::ZERO; WIDTH];
        for k in 0..N_EH {
            let row = &mut flat[row_idx * N_COLS..(row_idx + 1) * N_COLS];
            fill_common(row, member, input, &end, &endpoint_digest, &aug, &chain_in);
            row[C_SEL + S_EH + k] = Val::ONE;
            row[C_STATE..C_STATE + WIDTH].copy_from_slice(&state);
            let perm =
                inlined_perm::generate_perm_cols(padding_absorb(state, &endpoint, k), &consts);
            row[C_PERM..C_PERM + PERM_COLS].copy_from_slice(&perm);
            state = inlined_perm::perm_output(&perm);
            debug_assert!(k + 1 != N_EH || state[..DIGEST_ELEMS] == endpoint_digest);
            row_idx += 1;
        }
        state = [Val::ZERO; WIDTH];
        for k in 0..N_NODE {
            let row = &mut flat[row_idx * N_COLS..(row_idx + 1) * N_COLS];
            fill_common(row, member, input, &end, &endpoint_digest, &aug, &chain_in);
            row[C_SEL + S_AUG + k] = Val::ONE;
            row[C_STATE..C_STATE + WIDTH].copy_from_slice(&state);
            let perm =
                inlined_perm::generate_perm_cols(padding_absorb(state, &aug_input, k), &consts);
            row[C_PERM..C_PERM + PERM_COLS].copy_from_slice(&perm);
            state = inlined_perm::perm_output(&perm);
            debug_assert!(k + 1 != N_NODE || state[..DIGEST_ELEMS] == aug);
            row_idx += 1;
        }
        state = [Val::ZERO; WIDTH];
        for k in 0..N_NODE {
            let row = &mut flat[row_idx * N_COLS..(row_idx + 1) * N_COLS];
            fill_common(row, member, input, &end, &endpoint_digest, &aug, &chain_in);
            row[C_SEL + S_CHAIN + k] = Val::ONE;
            row[C_STATE..C_STATE + WIDTH].copy_from_slice(&state);
            let perm =
                inlined_perm::generate_perm_cols(padding_absorb(state, &chain_input, k), &consts);
            row[C_PERM..C_PERM + PERM_COLS].copy_from_slice(&perm);
            state = inlined_perm::perm_output(&perm);
            let final_row = member + 1 == inputs.len() && k + 1 == N_NODE;
            row[C_FINAL] = Val::from_bool(final_row);
            let delta = Val::from_usize(member) - Val::from_usize(inputs.len() - 1);
            if k + 1 == N_NODE && !final_row {
                row[C_FINAL_INV] = delta.inverse();
            }
            row_idx += 1;
        }
        debug_assert_eq!(&state[..DIGEST_ELEMS], &chain_out);
        chain = chain_out;
    }
    let pad_perm = inlined_perm::generate_perm_cols([Val::ZERO; WIDTH], &consts);
    for r in active_rows..height {
        flat[r * N_COLS + C_SEL + S_PAD] = Val::ONE;
        flat[r * N_COLS + C_PERM..r * N_COLS + C_PERM + PERM_COLS].copy_from_slice(&pad_perm);
    }
    (RowMajorMatrix::new(flat, N_COLS), chain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_uni_stark::{prove, verify};

    fn root(seed: usize) -> [Val; DIGEST_ELEMS] {
        core::array::from_fn(|i| Val::from_usize(seed * 1000 + i + 1))
    }

    fn start(seed: usize) -> [Val; WIDTH] {
        core::array::from_fn(|i| Val::from_usize(seed * 10_000 + i + 1))
    }

    fn stream(seed: usize, len: usize) -> Vec<Val> {
        (0..len)
            .map(|i| Val::from_usize(seed * 100_000 + i * 17 + 3))
            .collect()
    }

    fn end_state(start: [Val; WIDTH], stream: &[Val]) -> [Val; WIDTH] {
        let consts = inlined_perm::Constants::pinned();
        let mut state = start;
        for chunk in stream.chunks(RATE) {
            let mut block = [Val::ZERO; RATE];
            block[..chunk.len()].copy_from_slice(chunk);
            let cols = inlined_perm::generate_perm_cols(absorb(state, block, chunk.len()), &consts);
            state = inlined_perm::perm_output(&cols);
        }
        state
    }

    fn manual_root(
        roots: &[[Val; DIGEST_ELEMS]],
        starts: &[[Val; WIDTH]],
        streams: &[Vec<Val>],
    ) -> [Val; DIGEST_ELEMS] {
        let consts = inlined_perm::Constants::pinned();
        let mut chain = [Val::ZERO; DIGEST_ELEMS];
        for i in 0..roots.len() {
            let end = end_state(starts[i], &streams[i]);
            let aug = augmented_member_root(roots[i], starts[i], end);
            let mut combined = [Val::ZERO; ABSORB_LEN];
            combined[..DIGEST_ELEMS].copy_from_slice(&chain);
            combined[DIGEST_ELEMS..].copy_from_slice(&aug);
            chain = node_blocks(&combined, &consts).1;
        }
        chain
    }

    #[test]
    fn block_ood_binding_accepts_and_matches_block_chain() {
        let len = RATE * 3 + 2;
        let roots = [root(1), root(2)];
        let starts = [start(1), start(2)];
        let streams = [stream(1, len), stream(2, len)];
        let inputs = [
            MemberOodInput {
                member_root: roots[0],
                start: starts[0],
                transcript_stream: &streams[0],
                pcs_stream: &streams[0],
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &streams[1],
                pcs_stream: &streams[1],
            },
        ];
        let (trace, got_root) = build_trace(&inputs);
        let air = BlockOodBindingAir::new(inputs.len(), len);
        assert!(check_all_constraints(&air, &trace, &got_root, None).is_ok());
        assert_eq!(got_root, manual_root(&roots, &starts, &streams));
    }

    #[test]
    fn block_ood_binding_rejects_bus_order_shape_and_metadata_attacks() {
        let len = RATE * 2 + 3;
        let roots = [root(3), root(4)];
        let starts = [start(3), start(4)];
        let streams = [stream(3, len), stream(4, len)];
        let honest = [
            MemberOodInput {
                member_root: roots[0],
                start: starts[0],
                transcript_stream: &streams[0],
                pcs_stream: &streams[0],
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &streams[1],
                pcs_stream: &streams[1],
            },
        ];
        let (trace, honest_root) = build_trace(&honest);
        let air = BlockOodBindingAir::new(honest.len(), len);
        assert!(check_all_constraints(&air, &trace, &honest_root, None).is_ok());

        let mut substituted = streams[0].clone();
        substituted[0] += Val::ONE;
        let bad_bus = [
            MemberOodInput {
                member_root: roots[0],
                start: starts[0],
                transcript_stream: &streams[0],
                pcs_stream: &substituted,
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &streams[1],
                pcs_stream: &streams[1],
            },
        ];
        let (bad_bus, _) = build_trace(&bad_bus);
        assert!(!check_all_constraints(&air, &bad_bus, &honest_root, None).is_ok());

        let mut reordered = streams[0].clone();
        reordered.swap(1, 2);
        let bad_order = [
            MemberOodInput {
                member_root: roots[0],
                start: starts[0],
                transcript_stream: &streams[0],
                pcs_stream: &reordered,
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &streams[1],
                pcs_stream: &streams[1],
            },
        ];
        let (bad_order, _) = build_trace(&bad_order);
        assert!(!check_all_constraints(&air, &bad_order, &honest_root, None).is_ok());

        // A shorter stream is a different AIR profile even if its power-of-two trace height is
        // unchanged. It cannot verify under the settlement-approved profile/root.
        let short0 = streams[0][..len - 1].to_vec();
        let short1 = streams[1][..len - 1].to_vec();
        let short = [
            MemberOodInput {
                member_root: roots[0],
                start: starts[0],
                transcript_stream: &short0,
                pcs_stream: &short0,
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &short1,
                pcs_stream: &short1,
            },
        ];
        let (short_trace, _) = build_trace(&short);
        assert!(!check_all_constraints(&air, &short_trace, &honest_root, None).is_ok());

        let mut changed_roots = roots;
        changed_roots[0][0] += Val::ONE;
        let changed = [
            MemberOodInput {
                member_root: changed_roots[0],
                start: starts[0],
                transcript_stream: &streams[0],
                pcs_stream: &streams[0],
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &streams[1],
                pcs_stream: &streams[1],
            },
        ];
        let (changed, changed_root) = build_trace(&changed);
        assert_ne!(changed_root, honest_root);
        assert!(!check_all_constraints(&air, &changed, &honest_root, None).is_ok());

        let mut tampered = trace;
        tampered.values[C_END] += Val::ONE;
        assert!(!check_all_constraints(&air, &tampered, &honest_root, None).is_ok());
    }

    #[test]
    fn block_ood_binding_proves_at_production_q128_knobs() {
        let len = RATE + 1;
        let roots = [root(5), root(6)];
        let starts = [start(5), start(6)];
        let streams = [stream(5, len), stream(6, len)];
        let inputs = [
            MemberOodInput {
                member_root: roots[0],
                start: starts[0],
                transcript_stream: &streams[0],
                pcs_stream: &streams[0],
            },
            MemberOodInput {
                member_root: roots[1],
                start: starts[1],
                transcript_stream: &streams[1],
                pcs_stream: &streams[1],
            },
        ];
        let (trace, root) = build_trace(&inputs);
        let air = BlockOodBindingAir::new(inputs.len(), len);
        let config = crate::settlement::prod_config();
        let proof = prove(&config, &air, trace, &root);
        verify(&config, &air, &proof, &root)
            .expect("block OOD binding companion must verify at q128 knobs");
    }

    #[test]
    fn production_companion_shape_is_explicit() {
        let one = BlockOodBindingAir::new(1, PRODUCTION_OOD_STREAM_ELEMS_V1);
        assert_eq!(N_COLS, 299);
        assert_eq!(one.ood_rounds(), 26_576);
        assert_eq!(one.active_rows_per_member(), 26_589);
        assert_eq!(one.trace_height(), 32_768);
        assert_eq!(core::mem::size_of::<Val>(), 4);
        assert_eq!(
            one.trace_height() * N_COLS * core::mem::size_of::<Val>(),
            39_190_528
        );

        let two = BlockOodBindingAir::new(2, PRODUCTION_OOD_STREAM_ELEMS_V1);
        assert_eq!(two.trace_height(), 65_536);
        assert_eq!(
            two.trace_height() * N_COLS * core::mem::size_of::<Val>(),
            78_381_056
        );
    }
}
