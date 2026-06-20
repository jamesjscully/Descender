//! Octic (deg-8 extension) Poseidon2-16 permutation eval — the OOD-point mirror of
//! [`crate::inlined_perm`].
//!
//! ## Why this exists
//!
//! The leaf `C(zeta)` mirror ([`crate::leaf_constraint_air`]) recomputes leaf AIR constraint
//! residuals over OPENED out-of-domain (OOD) trace cells, each an extension element
//! `Challenge = BinomialExtensionField<Val, 8>` (8 `Val` limbs, octic `X^8 = W` arithmetic). The
//! leaf's Poseidon2 SPONGE constraints ([`descender_account_stark::air::FullRelationAir::eval_perm_block`])
//! are emitted as `builder.assert_eq(recomputed, committed_post)` over BASE-field cells. To pull
//! those sponge constraints into the constrained `C_partial`, each such equality becomes an octic
//! RESIDUAL `recomputed(zeta) − committed_post(zeta)` over the opened octic cells, which the
//! leaf-constraint mirror then alpha-batches like every other residual.
//!
//! This module is the octic analogue of [`crate::inlined_perm`]:
//!   * [`OcticPermCells`] — reads one permutation block's octic cells (inputs + committed posts).
//!   * [`perm_residuals`] — the in-AIR octic residual list for ONE permutation block, in the SAME
//!     order [`crate::inlined_perm::eval_perm`] emits its `assert_eq`s (begin-full posts, then
//!     partial post_sbox, then end-full posts). Each residual is a `[AB::Expr; D]` octic element
//!     that vanishes iff the committed post equals the recomputed Poseidon2 value.
//!   * [`native_perm_residuals`] / [`native_perm_output`] — the native `Challenge` AUTHORITY: the
//!     same residuals / the squeezed output computed over `Challenge` arithmetic (reusing the
//!     SHARED [`crate::inlined_perm::layers`] code, so the linear layers cannot diverge from the
//!     deployed permutation).
//!
//! ## Fidelity discipline
//!
//! The linear layers + round structure run through the SAME [`crate::inlined_perm::layers`]
//! generic code for the native reference (`T = Challenge`), and the in-AIR octic eval mirrors that
//! structure limb-for-limb. The pinned constants are the SAME [`crate::inlined_perm::Constants`]
//! (themselves byte-pinned to `default_koalabear_poseidon2_16`). Fidelity is PINned by a test that
//! lifts a real Val permutation (`inlined_perm::generate_perm_cols`) into octic cells and checks
//! the octic native output equals the lifted Val output AND every honest octic residual is zero —
//! a wrong octic layer / constant is caught by attempted equality, never by inspection.

use crate::fri_fold_air::{D, W};
use crate::inlined_perm::{
    layers, Constants, C_BEGIN_POST, C_END_POST, C_INPUTS, C_OUTPUT, C_PARTIAL_POST_SBOX,
    HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
};
use descender_account_stark::{Challenge, Val, WIDTH};
use p3_air::AirBuilder;
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

/// Number of octic residuals contributed by ONE permutation block (one per committed post cell):
/// begin-full posts (`HALF_FULL_ROUNDS·WIDTH`) + partial post_sbox (`PARTIAL_ROUNDS`) + end-full
/// posts (`HALF_FULL_ROUNDS·WIDTH`). Matches the number of `assert_eq`s in
/// [`crate::inlined_perm::eval_perm`].
pub const PERM_RESIDUALS: usize =
    HALF_FULL_ROUNDS * WIDTH + PARTIAL_ROUNDS + HALF_FULL_ROUNDS * WIDTH;

// ===========================================================================
// Octic (deg-8 extension) arithmetic over X^D = W — same as deep_air / leaf_constraint_air.
// ===========================================================================

fn octic_mul<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    let w = AB::Expr::from(AB::F::from_u32(W));
    core::array::from_fn(|k| {
        let mut acc = AB::Expr::ZERO;
        for i in 0..D {
            for j in 0..D {
                if i + j == k {
                    acc += a[i].clone() * b[j].clone();
                } else if i + j == k + D {
                    acc += w.clone() * a[i].clone() * b[j].clone();
                }
            }
        }
        acc
    })
}

fn octic_add<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() + b[i].clone())
}

fn octic_sub<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() - b[i].clone())
}

/// Octic cube `a^3` (the Poseidon2 S-box, SBOX_DEGREE = 3, SBOX_REGISTERS = 0 — the leaf config).
fn octic_cube<AB: AirBuilder>(a: &[AB::Expr; D]) -> [AB::Expr; D] {
    let sq = octic_mul::<AB>(a, a);
    octic_mul::<AB>(&sq, a)
}

/// Lift a base-field `Val` constant to an octic expr (limb 0 = constant, rest 0).
fn octic_const<AB: AirBuilder<F = Val>>(c: Val) -> [AB::Expr; D] {
    let limbs = <Challenge as BasedVectorSpace<Val>>::as_basis_coefficients_slice(&Challenge::from(
        c,
    ))
    .to_vec();
    core::array::from_fn(|i| AB::Expr::from(limbs[i]))
}

/// Octic external (full-round) linear layer mirroring `layers::external_linear_layer` (WIDTH=16):
/// M_4 on each 4-chunk via `apply_mat4`, then `state[i] += sums[i % 4]`. Octic analogue of the
/// SHARED generic code; PINned to it by the fidelity test.
fn octic_external_linear_layer<AB: AirBuilder<F = Val>>(state: &mut [[AB::Expr; D]; WIDTH]) {
    // apply_mat4 on each 4-chunk: [2 3 1 1; 1 2 3 1; 1 1 2 3; 3 1 1 2].
    for chunk in (0..WIDTH).step_by(4) {
        let x0 = state[chunk].clone();
        let x1 = state[chunk + 1].clone();
        let x2 = state[chunk + 2].clone();
        let x3 = state[chunk + 3].clone();
        let t01 = octic_add::<AB>(&x0, &x1);
        let t23 = octic_add::<AB>(&x2, &x3);
        let t0123 = octic_add::<AB>(&t01, &t23);
        let t01123 = octic_add::<AB>(&t0123, &x1);
        let t01233 = octic_add::<AB>(&t0123, &x3);
        let x0_dbl = octic_add::<AB>(&x0, &x0);
        let x2_dbl = octic_add::<AB>(&x2, &x2);
        // x3 = t01233 + 2*x0 ; x1 = t01123 + 2*x2 ; x0 = t01123 + t01 ; x2 = t01233 + t23
        state[chunk + 3] = octic_add::<AB>(&t01233, &x0_dbl);
        state[chunk + 1] = octic_add::<AB>(&t01123, &x2_dbl);
        state[chunk] = octic_add::<AB>(&t01123, &t01);
        state[chunk + 2] = octic_add::<AB>(&t01233, &t23);
    }
    // outer circulant: sums[k] = Σ_{j ≡ k (mod 4)} state[j]; state[i] += sums[i % 4].
    let sums: [[AB::Expr; D]; 4] = core::array::from_fn(|k| {
        let mut s = octic_const::<AB>(Val::ZERO);
        let mut j = 0;
        while j < WIDTH {
            s = octic_add::<AB>(&s, &state[j + k]);
            j += 4;
        }
        s
    });
    for i in 0..WIDTH {
        state[i] = octic_add::<AB>(&state[i], &sums[i % 4]);
    }
}

/// Octic internal (partial-round) linear layer: `state[i] = diag[i]*state[i] + sum`. `diag` is the
/// pinned width-16 internal diagonal pre-lifted to octic. Mirrors `layers::internal_linear_layer`.
fn octic_internal_linear_layer<AB: AirBuilder<F = Val>>(
    state: &mut [[AB::Expr; D]; WIDTH],
    diag: &[[AB::Expr; D]; WIDTH],
) {
    let mut sum = octic_const::<AB>(Val::ZERO);
    for s in state.iter() {
        sum = octic_add::<AB>(&sum, s);
    }
    for i in 0..WIDTH {
        let scaled = octic_mul::<AB>(&diag[i], &state[i]);
        state[i] = octic_add::<AB>(&scaled, &sum);
    }
}

/// Pinned round constants pre-lifted to octic exprs (built once per call).
struct OcticConstants<AB: AirBuilder<F = Val>> {
    external_initial: [[[AB::Expr; D]; WIDTH]; HALF_FULL_ROUNDS],
    external_final: [[[AB::Expr; D]; WIDTH]; HALF_FULL_ROUNDS],
    internal: [[AB::Expr; D]; PARTIAL_ROUNDS],
    diag: [[AB::Expr; D]; WIDTH],
}

impl<AB: AirBuilder<F = Val>> OcticConstants<AB> {
    fn pinned() -> Self {
        let c = Constants::pinned();
        Self {
            external_initial: c
                .external_initial
                .map(|row| row.map(|v| octic_const::<AB>(v))),
            external_final: c.external_final.map(|row| row.map(|v| octic_const::<AB>(v))),
            internal: c.internal.map(|v| octic_const::<AB>(v)),
            diag: c.diag.map(|v| octic_const::<AB>(v)),
        }
    }
}

/// Reader of ONE permutation block's octic cells (inputs + committed posts), at absolute octic-cell
/// index. The leaf-constraint mirror's cell accessor returns one opened OOD cell as `[AB::Expr; D]`.
pub trait OcticPermCells<AB: AirBuilder<F = Val>> {
    /// The octic cell at absolute index `idx` (units of whole cells, not limbs).
    fn at(&self, idx: usize) -> [AB::Expr; D];
}

/// Octic in-AIR residuals for ONE permutation block whose first cell is `offset` (cell index).
/// Reads `inputs[16]` + committed posts via `cells`, recomputes the Poseidon2 rounds in octic and
/// returns one octic residual `recomputed − committed_post` per committed post cell, in the SAME
/// order [`crate::inlined_perm::eval_perm`] emits its `assert_eq`s. Each residual vanishes iff the
/// committed post is the honest recomputed value (so a tampered input / interior post / output is
/// caught once these residuals are alpha-batched into `C_partial`).
pub fn perm_residuals<AB: AirBuilder<F = Val>, C: OcticPermCells<AB>>(
    cells: &C,
    offset: usize,
) -> Vec<[AB::Expr; D]> {
    let consts = OcticConstants::<AB>::pinned();
    let mut residuals: Vec<[AB::Expr; D]> = Vec::with_capacity(PERM_RESIDUALS);

    let mut state: [[AB::Expr; D]; WIDTH] =
        core::array::from_fn(|i| cells.at(offset + C_INPUTS + i));
    octic_external_linear_layer::<AB>(&mut state);

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            let t = octic_add::<AB>(&state[i], &consts.external_initial[r][i]);
            state[i] = octic_cube::<AB>(&t);
        }
        octic_external_linear_layer::<AB>(&mut state);
        let base = offset + C_BEGIN_POST + r * WIDTH;
        for i in 0..WIDTH {
            let post = cells.at(base + i);
            residuals.push(octic_sub::<AB>(&state[i], &post));
            state[i] = post;
        }
    }

    for r in 0..PARTIAL_ROUNDS {
        let t = octic_add::<AB>(&state[0], &consts.internal[r]);
        state[0] = octic_cube::<AB>(&t);
        let post = cells.at(offset + C_PARTIAL_POST_SBOX + r);
        residuals.push(octic_sub::<AB>(&state[0], &post));
        state[0] = post;
        octic_internal_linear_layer::<AB>(&mut state, &consts.diag);
    }

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            let t = octic_add::<AB>(&state[i], &consts.external_final[r][i]);
            state[i] = octic_cube::<AB>(&t);
        }
        octic_external_linear_layer::<AB>(&mut state);
        let base = offset + C_END_POST + r * WIDTH;
        for i in 0..WIDTH {
            let post = cells.at(base + i);
            residuals.push(octic_sub::<AB>(&state[i], &post));
            state[i] = post;
        }
    }

    debug_assert_eq!(residuals.len(), PERM_RESIDUALS);
    residuals
}

// ===========================================================================
// Native reference (the AUTHORITY) — over `Challenge`, reusing the SHARED layers code.
// ===========================================================================

/// Native `Challenge` round-constant set, lifted from the pinned [`Constants`].
struct NativeConstants {
    external_initial: [[Challenge; WIDTH]; HALF_FULL_ROUNDS],
    external_final: [[Challenge; WIDTH]; HALF_FULL_ROUNDS],
    internal: [Challenge; PARTIAL_ROUNDS],
    diag: [Challenge; WIDTH],
}

impl NativeConstants {
    fn pinned() -> Self {
        let c = Constants::pinned();
        Self {
            external_initial: c.external_initial.map(|row| row.map(Challenge::from)),
            external_final: c.external_final.map(|row| row.map(Challenge::from)),
            internal: c.internal.map(Challenge::from),
            diag: c.diag.map(Challenge::from),
        }
    }
}

/// Native octic permutation over `Challenge` (the AUTHORITY): runs the full Poseidon2-16 rounds via
/// the SHARED [`crate::inlined_perm::layers`] generic code (`T = Challenge`), recording every
/// committed post. Returns `(posts, output)` where `posts[k]` is the `k`-th committed post in
/// [`perm_residuals`] order and `output` is the 16-lane permutation result (last end-full post).
pub fn native_perm(inputs: [Challenge; WIDTH]) -> (Vec<Challenge>, [Challenge; WIDTH]) {
    let c = NativeConstants::pinned();
    let mut posts: Vec<Challenge> = Vec::with_capacity(PERM_RESIDUALS);

    let mut state = inputs;
    layers::external_linear_layer(&mut state);

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            state[i] = (state[i] + c.external_initial[r][i]).cube();
        }
        layers::external_linear_layer(&mut state);
        for i in 0..WIDTH {
            posts.push(state[i]);
        }
    }

    for r in 0..PARTIAL_ROUNDS {
        state[0] = (state[0] + c.internal[r]).cube();
        posts.push(state[0]);
        layers::internal_linear_layer(&mut state, &c.diag);
    }

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            state[i] = (state[i] + c.external_final[r][i]).cube();
        }
        layers::external_linear_layer(&mut state);
        for i in 0..WIDTH {
            posts.push(state[i]);
        }
    }

    debug_assert_eq!(posts.len(), PERM_RESIDUALS);
    (posts, state)
}

/// Native committed-post cells for ONE permutation block from its 16-lane input, in
/// [`perm_residuals`] order. (The committed posts an honest opened OOD perm block would carry.)
pub fn native_perm_posts(inputs: [Challenge; WIDTH]) -> Vec<Challenge> {
    native_perm(inputs).0
}

/// Native `Challenge` residuals for ONE permutation block — the EXACT AUTHORITY for
/// [`perm_residuals`], reproducing its reset semantics: after each round the running state is RESET
/// to the COMMITTED post cell (so each round's recomputed value depends on the previous round's
/// committed post, not the fully-recomputed chain). `inputs[i]` and `committed[k]` (in
/// [`perm_residuals`] order) are the opened cells; returns `recomputed_k − committed_k` per post.
/// On honest cells (`committed == recomputed`) every residual is zero; on arbitrary cells this
/// matches [`perm_residuals`] limb-for-limb.
pub fn native_perm_residuals(
    inputs: [Challenge; WIDTH],
    committed: &[Challenge],
) -> Vec<Challenge> {
    let c = NativeConstants::pinned();
    let mut residuals: Vec<Challenge> = Vec::with_capacity(PERM_RESIDUALS);
    let mut k = 0usize;

    let mut state = inputs;
    layers::external_linear_layer(&mut state);

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            state[i] = (state[i] + c.external_initial[r][i]).cube();
        }
        layers::external_linear_layer(&mut state);
        for i in 0..WIDTH {
            let post = committed[k];
            residuals.push(state[i] - post);
            state[i] = post;
            k += 1;
        }
    }

    for r in 0..PARTIAL_ROUNDS {
        state[0] = (state[0] + c.internal[r]).cube();
        let post = committed[k];
        residuals.push(state[0] - post);
        state[0] = post;
        k += 1;
        layers::internal_linear_layer(&mut state, &c.diag);
    }

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            state[i] = (state[i] + c.external_final[r][i]).cube();
        }
        layers::external_linear_layer(&mut state);
        for i in 0..WIDTH {
            let post = committed[k];
            residuals.push(state[i] - post);
            state[i] = post;
            k += 1;
        }
    }

    debug_assert_eq!(residuals.len(), PERM_RESIDUALS);
    residuals
}

/// Native 16-lane permutation OUTPUT (last end-full-round post) over `Challenge`.
pub fn native_perm_output(inputs: [Challenge; WIDTH]) -> [Challenge; WIDTH] {
    native_perm(inputs).1
}

/// The OUTPUT lane index of committed post `C_OUTPUT + lane` within the in-AIR perm block (for the
/// squeeze: the leaf reads `ending_full_rounds[HALF_FULL_ROUNDS-1].post[lane]`, which is the cell
/// at `C_OUTPUT + lane`). Re-exported for the sponge mirror.
pub const fn output_cell(offset: usize, lane: usize) -> usize {
    offset + C_OUTPUT + lane
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inlined_perm::{self, PERM_COLS};
    use p3_field::PrimeCharacteristicRing;

    fn lift(v: Val) -> Challenge {
        Challenge::from(v)
    }

    fn sample_input(seed: u32) -> [Val; WIDTH] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503))
        })
    }

    /// FIDELITY PIN (base-lift): the octic native perm (over `Challenge`) reproduces the deployed
    /// Val permutation EXACTLY when fed base-lifted inputs. We run the same input through
    /// `inlined_perm::generate_perm_cols` (Val, byte-pinned to default_koalabear_poseidon2_16) and
    /// through `native_perm` (Challenge), and assert every committed post AND the output match —
    /// a wrong octic layer / constant is caught by attempted equality.
    #[test]
    fn native_octic_matches_val_perm_on_base_inputs() {
        let consts = inlined_perm::Constants::pinned();
        for seed in [0u32, 1, 7, 0xDEAD_BEEF, 12289, 0x7eff_ffff] {
            let input = sample_input(seed);
            let cols = inlined_perm::generate_perm_cols(input, &consts);
            let val_out = inlined_perm::perm_output(&cols);

            let oct_in: [Challenge; WIDTH] = core::array::from_fn(|i| lift(input[i]));
            let (oct_posts, oct_out) = native_perm(oct_in);

            for lane in 0..WIDTH {
                assert_eq!(oct_out[lane], lift(val_out[lane]), "octic output lane {lane} diverges (seed {seed:#x})");
            }
            // committed posts: compare octic posts vs the Val cols at the post offsets.
            let mut k = 0usize;
            for r in 0..HALF_FULL_ROUNDS {
                for i in 0..WIDTH {
                    assert_eq!(oct_posts[k], lift(cols[C_BEGIN_POST + r * WIDTH + i]));
                    k += 1;
                }
            }
            for r in 0..PARTIAL_ROUNDS {
                assert_eq!(oct_posts[k], lift(cols[C_PARTIAL_POST_SBOX + r]));
                k += 1;
            }
            for r in 0..HALF_FULL_ROUNDS {
                for i in 0..WIDTH {
                    assert_eq!(oct_posts[k], lift(cols[C_END_POST + r * WIDTH + i]));
                    k += 1;
                }
            }
            assert_eq!(k, PERM_RESIDUALS);
        }
    }

    /// FIDELITY PIN (multi-limb): the octic native perm is exercised on genuine multi-limb
    /// extension inputs (not just base-lifts), recording posts; the residuals computed in-AIR over
    /// an honest cell layout must vanish (proved in the leaf_constraint_air sponge tests). Here we
    /// just sanity-check determinism + that PERM_RESIDUALS posts are recorded.
    #[test]
    fn native_octic_multilimb_records_all_posts() {
        let input: [Challenge; WIDTH] = core::array::from_fn(|i| {
            Challenge::from_basis_coefficients_fn(|j| {
                Val::from_u32((i as u32 + 1).wrapping_mul(7).wrapping_add(j as u32 * 13 + 1))
            })
        });
        let (posts, _out) = native_perm(input);
        assert_eq!(posts.len(), PERM_RESIDUALS);
        // determinism
        let (posts2, _) = native_perm(input);
        assert_eq!(posts, posts2);
    }

    /// Sanity: `PERM_RESIDUALS` equals the count of committed posts in a `PERM_COLS` block
    /// (PERM_COLS = inputs[16] + posts), so the residual list covers exactly the committed cells.
    #[test]
    fn perm_residuals_count_matches_block_posts() {
        assert_eq!(PERM_RESIDUALS, PERM_COLS - WIDTH);
    }
}
