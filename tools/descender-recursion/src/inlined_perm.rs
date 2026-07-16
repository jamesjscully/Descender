//! Gadget 2 — INLINED Poseidon2-16 permutation constraints (Option A building block).
//!
//! Per the composition decision (`research/path2_recursion_air_composition_architecture.md`,
//! Option A): the monolithic `recursive_air` cannot reuse `p3-poseidon2-air`'s `Poseidon2Air`
//! as a sub-window (its round-eval helpers are private and its `eval` does a size-checked
//! `borrow` that breaks when fold columns widen the row). So the Poseidon2 round constraints
//! are RE-IMPLEMENTED here over a column layout this crate controls, written ONCE and applied
//! uniformly. This module is the standalone, byte-pinned building block: it constrains a single
//! width-16 Poseidon2 permutation per row, BYTE-IDENTICAL to `default_koalabear_poseidon2_16`
//! (the leaf's `Perm`), and the fold wiring (`merkle.rs` -> in-AIR) is composed on top in the
//! gadget-2 path-fold AIR (`merkle_air.rs`).
//!
//! ## Anti-divergence discipline
//!
//! The linear layers (`external_linear_layer`, `internal_linear_layer`) and the round structure
//! are written GENERICALLY over `T: PrimeCharacteristicRing` in [`layers`], so the honest-trace
//! GENERATOR (`T = Val`) and the constraint EVAL (`T = AB::Expr`) execute the *same* code — a
//! wrong constant or a wrong layer cannot pass one and fail the other. The pinned round
//! constants + diagonal are lifted verbatim from `descender-mask-prf/src/poseidon2.rs`, itself
//! COPIED VERBATIM from `p3-koala-bear-0.6.1`. The honest trace's output is additionally
//! cross-checked against a real `default_koalabear_poseidon2_16()` evaluation in the tests, so a
//! divergence from the deployed permutation is caught by attempted equality, not by inspection.
//!
//! Column layout for one permutation (SBOX_DEGREE=3, SBOX_REGISTERS=0 — the leaf config):
//! `inputs[16]`, then per beginning full round `post[16]`, per partial round `post_sbox[1]`,
//! per ending full round `post[16]`. The S-box (`x^3`) is degree-3 inlined (no register
//! columns, matching SBOX_REGISTERS=0); the committed `post`/`post_sbox` cells keep every
//! constraint at degree ≤ 3 (same shape as `p3-poseidon2-air`'s `Poseidon2Cols`).

use descender_account_stark::{Val, WIDTH};
use p3_field::{Field, PrimeCharacteristicRing};

/// Half the full rounds (KoalaBear-16: 4 ⇒ 8 full rounds total).
pub const HALF_FULL_ROUNDS: usize = 4;
/// Partial (internal) rounds (KoalaBear-16: 20).
pub const PARTIAL_ROUNDS: usize = 20;

// ---------------------------------------------------------------------------
// PINNED ROUND CONSTANTS — copied verbatim from p3-koala-bear-0.6.1 (and from
// descender-mask-prf/src/poseidon2.rs, which is itself the verbatim copy). The
// `byte_pinned_constants_match_deployed_permutation` test re-derives the whole
// permutation from these and asserts equality with default_koalabear_poseidon2_16,
// so a single transposed digit is caught by attempted equality.
// ---------------------------------------------------------------------------

/// `KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL` (4 rounds × 16).
pub const RC16_EXTERNAL_INITIAL: [[u32; WIDTH]; HALF_FULL_ROUNDS] = [
    [
        0x7ee56a48, 0x11367045, 0x12e41941, 0x7ebbc12b, 0x1970b7d5, 0x662b60e8, 0x3e4990c6,
        0x679f91f5, 0x350813bb, 0x00874ad4, 0x28a0081a, 0x18fa5872, 0x5f25b071, 0x5e5d5998,
        0x5e6fd3e7, 0x5b2e2660,
    ],
    [
        0x6f1837bf, 0x3fe6182b, 0x1edd7ac5, 0x57470d00, 0x43d486d5, 0x1982c70f, 0x0ea53af9,
        0x61d6165b, 0x51639c00, 0x2dec352c, 0x2950e531, 0x2d2cb947, 0x08256cef, 0x1a0109f6,
        0x1f51faf3, 0x5cef1c62,
    ],
    [
        0x3d65e50e, 0x33d91626, 0x133d5a1e, 0x0ff49b0d, 0x38900cd1, 0x2c22cc3f, 0x28852bb2,
        0x06c65a02, 0x7b2cf7bc, 0x68016e1a, 0x15e16bc0, 0x5248149a, 0x6dd212a0, 0x18d6830a,
        0x5001be82, 0x64dac34e,
    ],
    [
        0x5902b287, 0x426583a0, 0x0c921632, 0x3fe028a5, 0x245f8e49, 0x43bb297e, 0x7873dbd9,
        0x3cc987df, 0x286bb4ce, 0x640a8dcd, 0x512a8e36, 0x03a4cf55, 0x481837a2, 0x03d6da84,
        0x73726ac7, 0x760e7fdf,
    ],
];

/// `KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL` (4 rounds × 16).
pub const RC16_EXTERNAL_FINAL: [[u32; WIDTH]; HALF_FULL_ROUNDS] = [
    [
        0x43e7dc24, 0x259a5d61, 0x27e85a3b, 0x1b9133fa, 0x343e5628, 0x485cd4c2, 0x16e269f5,
        0x165b60c6, 0x25f683d9, 0x124f81f9, 0x174331f9, 0x77344dc5, 0x5a821dba, 0x5fc4177f,
        0x54153bf5, 0x5e3f1194,
    ],
    [
        0x3bdbf191, 0x088c84a3, 0x68256c9b, 0x3c90bbc6, 0x6846166a, 0x03f4238d, 0x463335fb,
        0x5e3d3551, 0x6e59ae6f, 0x32d06cc0, 0x596293f3, 0x6c87edb2, 0x08fc60b5, 0x34bcca80,
        0x24f007f3, 0x62731c6f,
    ],
    [
        0x1e1db6c6, 0x0ca409bb, 0x585c1e78, 0x56e94edc, 0x16d22734, 0x18e11467, 0x7b2c3730,
        0x770075e4, 0x35d1b18c, 0x22be3db5, 0x4fb1fbb7, 0x477cb3ed, 0x7d5311c6, 0x5b62ae7d,
        0x559c5fa8, 0x77f15048,
    ],
    [
        0x3211570b, 0x490fef6a, 0x77ec311f, 0x2247171b, 0x4e0ac711, 0x2edf69c9, 0x3b5a8850,
        0x65809421, 0x5619b4aa, 0x362019a7, 0x6bf9d4ed, 0x5b413dff, 0x617e181e, 0x5e7ab57b,
        0x33ad7833, 0x3466c7ca,
    ],
];

/// `KOALABEAR_POSEIDON2_RC_16_INTERNAL` (20 scalar constants, one per partial round).
pub const RC16_INTERNAL: [u32; PARTIAL_ROUNDS] = [
    0x54dfeb5d, 0x7d40afd6, 0x722cb316, 0x106a4573, 0x45a7ccdb, 0x44061375, 0x154077a5, 0x45744faa,
    0x4eb5e5ee, 0x3794e83f, 0x47c7093c, 0x5694903c, 0x69cb6299, 0x373df84c, 0x46a0df58, 0x46b8758a,
    0x3241ebcb, 0x0b09d233, 0x1af42357, 0x1e66cec2,
];

/// The pinned width-16 internal diagonal `V` (the internal linear layer is `s -> (1+Diag(V))s`).
/// Mirrors `descender-mask-prf/src/poseidon2.rs::internal_diag` ==
/// `p3-koala-bear-0.6.1` `InternalLayerBaseParameters<.,16>`.
pub fn internal_diag() -> [Val; WIDTH] {
    let inv_2exp = |k: u64| Val::from_u32(2).exp_u64(k).inverse();
    let half = Val::from_u32(2).inverse(); // 1/2
    [
        -Val::from_u32(2), // -2
        Val::ONE,          // 1
        Val::from_u32(2),  // 2
        half,              // 1/2
        Val::from_u32(3),  // 3
        Val::from_u32(4),  // 4
        -half,             // -1/2
        -Val::from_u32(3), // -3
        -Val::from_u32(4), // -4
        inv_2exp(8),       // 1/2^8
        inv_2exp(3),       // 1/8
        inv_2exp(24),      // 1/2^24
        -inv_2exp(8),      // -1/2^8
        -inv_2exp(3),      // -1/8
        -inv_2exp(4),      // -1/16
        -inv_2exp(24),     // -1/2^24
    ]
}

/// The Poseidon2 linear layers, generic over any `PrimeCharacteristicRing` `T`. This is the
/// SHARED logic used by BOTH the honest-trace generator (`T = Val`) and the in-AIR constraint
/// eval (`T = AB::Expr`), so the two cannot diverge. Mirrors `mds_light_permutation` (WIDTH=16)
/// and `generic_internal_linear_layer` from p3 exactly. `diag` is pre-lifted to `T`.
pub mod layers {
    use super::WIDTH;
    use p3_field::PrimeCharacteristicRing;

    /// `apply_mat4`: multiply a 4-vector by `MDSMat4 = [2 3 1 1; 1 2 3 1; 1 1 2 3; 3 1 1 2]`.
    /// Mirrors `p3-poseidon2-0.6.1/src/external.rs::apply_mat4`.
    #[inline]
    fn apply_mat4<T: PrimeCharacteristicRing>(x: &mut [T; 4]) {
        let t01 = x[0].clone() + x[1].clone();
        let t23 = x[2].clone() + x[3].clone();
        let t0123 = t01.clone() + t23.clone();
        let t01123 = t0123.clone() + x[1].clone();
        let t01233 = t0123 + x[3].clone();
        x[3] = t01233.clone() + x[0].double(); // 3*x0 + x1 + x2 + 2*x3
        x[1] = t01123.clone() + x[2].double(); // x0 + 2*x1 + 3*x2 + x3
        x[0] = t01123 + t01; // 2*x0 + 3*x1 + x2 + x3
        x[2] = t01233 + t23; // x0 + x1 + 2*x2 + 3*x3
    }

    /// External light MDS permutation (`mds_light_permutation` for WIDTH=16): M_4 on each
    /// 4-chunk, then the outer circulant `state[i] += sums[i % 4]`.
    pub fn external_linear_layer<T: PrimeCharacteristicRing>(state: &mut [T; WIDTH]) {
        for chunk_start in (0..WIDTH).step_by(4) {
            let mut chunk = [
                state[chunk_start].clone(),
                state[chunk_start + 1].clone(),
                state[chunk_start + 2].clone(),
                state[chunk_start + 3].clone(),
            ];
            apply_mat4(&mut chunk);
            for (i, c) in chunk.into_iter().enumerate() {
                state[chunk_start + i] = c;
            }
        }
        let sums: [T; 4] = core::array::from_fn(|k| {
            let mut s = T::ZERO;
            let mut j = 0;
            while j < WIDTH {
                s += state[j + k].clone();
                j += 4;
            }
            s
        });
        for i in 0..WIDTH {
            state[i] += sums[i % 4].clone();
        }
    }

    /// Internal linear layer: `s -> (1 + Diag(V))s`, i.e. `state[i] = diag[i]*state[i] + sum`.
    pub fn internal_linear_layer<T: PrimeCharacteristicRing>(
        state: &mut [T; WIDTH],
        diag: &[T; WIDTH],
    ) {
        let mut sum = T::ZERO;
        for s in state.iter() {
            sum += s.clone();
        }
        for i in 0..WIDTH {
            state[i] = diag[i].clone() * state[i].clone() + sum.clone();
        }
    }
}

/// Pinned round constants as `Val`, the single source shared by trace-gen and constraints.
#[derive(Clone)]
pub struct Constants {
    pub external_initial: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    pub external_final: [[Val; WIDTH]; HALF_FULL_ROUNDS],
    pub internal: [Val; PARTIAL_ROUNDS],
    pub diag: [Val; WIDTH],
}

impl Constants {
    /// Build from the pinned `RC16_*` tables (byte-identical to `default_koalabear_poseidon2_16`).
    pub fn pinned() -> Self {
        Self {
            external_initial: RC16_EXTERNAL_INITIAL.map(|r| r.map(Val::from_u32)),
            external_final: RC16_EXTERNAL_FINAL.map(|r| r.map(Val::from_u32)),
            internal: RC16_INTERNAL.map(Val::from_u32),
            diag: internal_diag(),
        }
    }
}

/// Committed columns for ONE permutation: inputs[16] + (full posts) + (partial post_sbox) +
/// (full posts).
pub const PERM_COLS: usize = WIDTH + (2 * HALF_FULL_ROUNDS) * WIDTH + PARTIAL_ROUNDS;

// Column offsets within a single permutation block.
pub const C_INPUTS: usize = 0;
pub const C_BEGIN_POST: usize = C_INPUTS + WIDTH; // HALF_FULL_ROUNDS * WIDTH
pub const C_PARTIAL_POST_SBOX: usize = C_BEGIN_POST + HALF_FULL_ROUNDS * WIDTH; // PARTIAL_ROUNDS
pub const C_END_POST: usize = C_PARTIAL_POST_SBOX + PARTIAL_ROUNDS; // HALF_FULL_ROUNDS * WIDTH
const _: () = assert!(C_END_POST + HALF_FULL_ROUNDS * WIDTH == PERM_COLS);

/// The OUTPUT of the permutation = the last ending full round's `post`, lanes `0..WIDTH`.
/// (Merkle compression uses only the first `DIGEST_ELEMS = 8` lanes.)
pub const C_OUTPUT: usize = C_END_POST + (HALF_FULL_ROUNDS - 1) * WIDTH;

/// Generate the committed cells for one permutation from a 16-lane input, recording every
/// `post` / `post_sbox`. Uses the SHARED [`layers`] code so it stays byte-identical to the
/// constraint eval.
pub fn generate_perm_cols(input: [Val; WIDTH], c: &Constants) -> [Val; PERM_COLS] {
    let mut cols = [Val::ZERO; PERM_COLS];
    cols[C_INPUTS..C_INPUTS + WIDTH].copy_from_slice(&input);

    let mut state = input;
    layers::external_linear_layer(&mut state);

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            state[i] = (state[i] + c.external_initial[r][i]).cube();
        }
        layers::external_linear_layer(&mut state);
        let base = C_BEGIN_POST + r * WIDTH;
        cols[base..base + WIDTH].copy_from_slice(&state);
    }

    for r in 0..PARTIAL_ROUNDS {
        state[0] = (state[0] + c.internal[r]).cube();
        cols[C_PARTIAL_POST_SBOX + r] = state[0];
        layers::internal_linear_layer(&mut state, &c.diag);
    }

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            state[i] = (state[i] + c.external_final[r][i]).cube();
        }
        layers::external_linear_layer(&mut state);
        let base = C_END_POST + r * WIDTH;
        cols[base..base + WIDTH].copy_from_slice(&state);
    }

    cols
}

/// Convenience: the 16-lane permutation output (last ending-full-round post).
pub fn perm_output(cols: &[Val; PERM_COLS]) -> [Val; WIDTH] {
    let mut out = [Val::ZERO; WIDTH];
    out.copy_from_slice(&cols[C_OUTPUT..C_OUTPUT + WIDTH]);
    out
}

// ---------------------------------------------------------------------------
// Constraint eval over a permutation block, generic so it can be called from the
// standalone `PermAir` AND inlined into the wider gadget-2 / recursive_air over a
// column OFFSET (the monolithic-AIR reuse). Pinned to AB::F = Val (the leaf field).
// ---------------------------------------------------------------------------

use p3_air::AirBuilder;

/// Round constants pre-lifted to `AB::Expr` (built once per `eval`). `AB::F = Val`.
pub struct ConstantsExpr<AB: AirBuilder<F = Val>> {
    pub external_initial: [[AB::Expr; WIDTH]; HALF_FULL_ROUNDS],
    pub external_final: [[AB::Expr; WIDTH]; HALF_FULL_ROUNDS],
    pub internal: [AB::Expr; PARTIAL_ROUNDS],
    pub diag: [AB::Expr; WIDTH],
}

impl<AB: AirBuilder<F = Val>> ConstantsExpr<AB> {
    pub fn pinned() -> Self {
        let c = Constants::pinned();
        Self {
            external_initial: c.external_initial.map(|row| row.map(AB::Expr::from)),
            external_final: c.external_final.map(|row| row.map(AB::Expr::from)),
            internal: c.internal.map(AB::Expr::from),
            diag: c.diag.map(AB::Expr::from),
        }
    }
}

/// Reader of a permutation block's cells, abstracting whether the block is a standalone row or
/// a sub-window of the monolithic AIR.
pub trait PermCells<AB: AirBuilder<F = Val>> {
    /// The cell at absolute column index `idx`, as an expression.
    fn at(&self, idx: usize) -> AB::Expr;
}

/// Apply the full Poseidon2 round constraints for ONE permutation block whose first column is
/// `offset`, reading inputs / committed posts via `cells`. Asserts each committed `post` /
/// `post_sbox` equals the recomputed value — the SAME [`layers`] code as the generator, so an
/// honest [`generate_perm_cols`] block satisfies it and any tampered cell breaks it.
pub fn eval_perm<AB: AirBuilder<F = Val>, C: PermCells<AB>>(
    builder: &mut AB,
    cells: &C,
    offset: usize,
    consts: &ConstantsExpr<AB>,
) {
    let mut state: [AB::Expr; WIDTH] = core::array::from_fn(|i| cells.at(offset + C_INPUTS + i));
    layers::external_linear_layer(&mut state);

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            let t = state[i].clone() + consts.external_initial[r][i].clone();
            state[i] = t.cube();
        }
        layers::external_linear_layer(&mut state);
        let base = offset + C_BEGIN_POST + r * WIDTH;
        for i in 0..WIDTH {
            let post = cells.at(base + i);
            builder.assert_eq(state[i].clone(), post.clone());
            state[i] = post;
        }
    }

    for r in 0..PARTIAL_ROUNDS {
        let t = state[0].clone() + consts.internal[r].clone();
        state[0] = t.cube();
        let post = cells.at(offset + C_PARTIAL_POST_SBOX + r);
        builder.assert_eq(state[0].clone(), post.clone());
        state[0] = post;
        layers::internal_linear_layer(&mut state, &consts.diag);
    }

    for r in 0..HALF_FULL_ROUNDS {
        for i in 0..WIDTH {
            let t = state[i].clone() + consts.external_final[r][i].clone();
            state[i] = t.cube();
        }
        layers::external_linear_layer(&mut state);
        let base = offset + C_END_POST + r * WIDTH;
        for i in 0..WIDTH {
            let post = cells.at(base + i);
            builder.assert_eq(state[i].clone(), post.clone());
            state[i] = post;
        }
    }
}

// ---------------------------------------------------------------------------
// Standalone AIR: one permutation per row (the byte-reference, p3 check_all_constraints).
// ---------------------------------------------------------------------------

use p3_air::{Air, BaseAir, WindowAccess};

/// Reads cells directly from the current-row slice (standalone, offset 0).
pub struct RowCells<AB: AirBuilder<F = Val>> {
    pub row: alloc::vec::Vec<AB::Var>,
}

impl<AB: AirBuilder<F = Val>> PermCells<AB> for RowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

/// Standalone inlined-Poseidon2 AIR — one width-16 permutation per row, pinned to
/// `default_koalabear_poseidon2_16`. The byte-reference building block for the monolithic AIR.
pub struct PermAir;

impl<F: Sync> BaseAir<F> for PermAir {
    fn width(&self) -> usize {
        PERM_COLS
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for PermAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let row: alloc::vec::Vec<AB::Var> = main.current_slice().to_vec();
        let cells = RowCells::<AB> { row };
        let consts = ConstantsExpr::<AB>::pinned();
        eval_perm::<AB, _>(builder, &cells, 0, &consts);
    }
}

extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::perm::CompressAir;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_koala_bear::default_koalabear_poseidon2_16;
    use p3_matrix::dense::RowMajorMatrix;
    use p3_symmetric::Permutation;

    fn sample_input(seed: u32) -> [Val; WIDTH] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503))
        })
    }

    /// BYTE-PIN (positive): the inlined permutation (driven by the pinned constants + the
    /// shared `layers` code) must equal `default_koalabear_poseidon2_16().permute(input)` for
    /// EVERY lane, on many inputs. A wrong constant / transposed digit / wrong layer is caught
    /// by attempted equality — NOT by inspection.
    #[test]
    fn byte_pinned_constants_match_deployed_permutation() {
        let c = Constants::pinned();
        let perm = default_koalabear_poseidon2_16();
        for seed in [
            0u32,
            1,
            7,
            0xDEAD_BEEF,
            0x0123_4567,
            0xFFFF_FFFF,
            12289,
            0x7eff_ffff,
        ] {
            let input = sample_input(seed);
            let cols = generate_perm_cols(input, &c);
            let got = perm_output(&cols);
            let mut expected = input;
            perm.permute_mut(&mut expected);
            assert_eq!(
                got, expected,
                "inlined perm diverges from default_koalabear_poseidon2_16 (seed {seed:#x})"
            );
        }
    }

    /// The standalone inlined-perm AIR ACCEPTS an honest trace (real `check_all_constraints`).
    #[test]
    fn air_accepts_honest_perm_trace() {
        let c = Constants::pinned();
        let rows: alloc::vec::Vec<[Val; PERM_COLS]> = (0..4u32)
            .map(|s| generate_perm_cols(sample_input(s), &c))
            .collect();
        let flat: alloc::vec::Vec<Val> = rows.iter().flat_map(|r| r.iter().copied()).collect();
        let m = RowMajorMatrix::new(flat, PERM_COLS);
        let report = check_all_constraints(&PermAir, &m, &[], None);
        assert!(
            report.is_ok(),
            "honest inlined perm trace failed: {} failures",
            report.failures.len()
        );
    }

    /// Tamper an input lane ⇒ rejected (the first beginning-full-round post no longer matches).
    #[test]
    fn air_rejects_tampered_input() {
        let c = Constants::pinned();
        let mut row = generate_perm_cols(sample_input(3), &c);
        row[C_INPUTS + 5] += Val::ONE; // flip an input lane, leave posts as-is
        let m = RowMajorMatrix::new(row.to_vec(), PERM_COLS);
        assert!(check_all_constraints(&PermAir, &m, &[], None).is_ok() == false);
    }

    /// Tamper a committed beginning-full-round post cell ⇒ rejected.
    #[test]
    fn air_rejects_tampered_begin_post() {
        let c = Constants::pinned();
        let mut row = generate_perm_cols(sample_input(2), &c);
        row[C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row.to_vec(), PERM_COLS);
        assert!(check_all_constraints(&PermAir, &m, &[], None).is_ok() == false);
    }

    /// Tamper a committed partial-round post_sbox cell ⇒ rejected.
    #[test]
    fn air_rejects_tampered_partial_post_sbox() {
        let c = Constants::pinned();
        let mut row = generate_perm_cols(sample_input(9), &c);
        row[C_PARTIAL_POST_SBOX + 10] += Val::ONE;
        let m = RowMajorMatrix::new(row.to_vec(), PERM_COLS);
        assert!(check_all_constraints(&PermAir, &m, &[], None).is_ok() == false);
    }

    /// Tamper the OUTPUT (last ending-full-round post) cell ⇒ rejected. This is the cell the
    /// Merkle fold reads, so an underconstrained output would be a real forgery vector.
    #[test]
    fn air_rejects_tampered_output() {
        let c = Constants::pinned();
        let mut row = generate_perm_cols(sample_input(11), &c);
        row[C_OUTPUT] += Val::ONE;
        let m = RowMajorMatrix::new(row.to_vec(), PERM_COLS);
        assert!(check_all_constraints(&PermAir, &m, &[], None).is_ok() == false);
    }

    /// A self-consistent trace built under a DIFFERENT constant set must fail under `PermAir`
    /// (whose constants are pinned) — guards against an undetected constant-set swap. (A wrong
    /// constant alone yields a self-consistent honest trace, so self-consistency is not enough.)
    #[test]
    fn air_rejects_trace_built_with_wrong_constants() {
        let mut wrong = Constants::pinned();
        wrong.internal[0] += Val::ONE; // perturb one internal round constant
        let row = generate_perm_cols(sample_input(4), &wrong);
        let m = RowMajorMatrix::new(row.to_vec(), PERM_COLS);
        assert!(check_all_constraints(&PermAir, &m, &[], None).is_ok() == false);
    }

    /// Cross-check (q128): the inlined output's SQUEEZE (first `DIGEST_ELEMS` lanes) equals the
    /// deployed permutation's first `DIGEST_ELEMS` lanes — the truncation the sponge-mode Merkle
    /// node / leaf-hash squeezes out as the digest. (The old digest-8 form cross-checked a 2-to-1
    /// truncated permutation; at q128 a 13-element digest no longer fits `2·d ≤ WIDTH`, so the node
    /// is sponge-mode — see `merkle_air::tests::air_node_matches_real_sponge_compression`.)
    #[test]
    fn inlined_output_squeeze_matches_deployed_permutation() {
        use descender_account_stark::DIGEST_ELEMS;
        let _ = CompressAir::new; // keep the p3 byte-reference type wired
        let c = Constants::pinned();
        let perm = default_koalabear_poseidon2_16();
        let input: [Val; WIDTH] = core::array::from_fn(|i| Val::from_u32(100 + i as u32));
        let cols = generate_perm_cols(input, &c);
        let out = perm_output(&cols);
        let mut expected = input;
        perm.permute_mut(&mut expected);
        assert_eq!(&out[..DIGEST_ELEMS], &expected[..DIGEST_ELEMS]);
    }
}
