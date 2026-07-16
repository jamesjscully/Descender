//! Phase-2 SPEND recursive-verifier constraint mirror (`spend_constraint_air`) — re-derives the FULL
//! spend relation `C(zeta)` over opened-at-zeta cells, folded PROVABLY via the shared
//! re-arithmetization primitive [`crate::alpha_fold::assert_committed_fold`].
//!
//! This is the spend analogue of [`crate::leaf_constraint_air`]: it transcribes every constraint
//! [`descender_account_stark::spend_air::SpendAir::eval`] emits (`assert_eq` / `assert_bool`) into an
//! opened-cell octic RESIDUAL `lhs(zeta) − rhs(zeta)` that vanishes iff the original constraint holds.
//! All residuals are degree ≤ 3 (the spend AIR already proves), so they fold at degree ≤ 4 through the
//! committed-accumulator primitive — provable regardless of count.
//!
//! ## Constraint classes (mirrored, in `eval` order)
//!
//!  1. **packed-witness consumption** — 18 canonical residues (value ‖ 14 bits ‖ slack ‖ 14 bits) +
//!     the 32-byte 14-bit-LE packing bound to the public `packed_witness_commitment`.
//!  2. **frame-digest sponge** — domain pin + statement-lane bindings + the full Poseidon2 sponge
//!     (byte-decompositions, absorb/capacity wiring, per-block octic perm residuals via
//!     [`crate::inlined_perm_octic`]) + the squeezed digest bound to the public `frame_digest`.
//!  3. **checked-core SIS** — 22 canonical response lanes + per-row two-chunk matrix-vector
//!     reductions (`syndrome[row] == Σ H[row][c]·r[c] mod q`).
//!  3b. **sigma identity** — c/w/t canonical + public-bound, the 23 `c⊛w` product reductions, and the
//!     12 fold+final reductions binding `syndrome == t + c⊛w` (Fq12).
//!  3c. **sigma commitments** — `w == unpack(witness_commitment)`, `t == unpack(proof_commitment)`.
//!  3d. **rehosted challenge sponge** — domain pin + statement/commitment bindings + the full
//!     Poseidon2 sponge + the per-lane squeeze reduction binding `sigma_c == challenge mod q`.
//!  4. **relation-id** — the four pinned `SPEND_RELATION_ID` public elements.
//!
//! ## Fidelity discipline
//!
//! Two native authorities pin the octic exprs: [`spend_residual_values`] recomputes the SAME residual
//! list over `Challenge` arithmetic (the committed-fold witness fill), and the honest-cells test lifts
//! a REAL [`descender_account_stark::spend_air::generate_spend_trace`] row into octic cells and asserts
//! every residual vanishes — a wrong octic transcription is caught by attempted equality, never by
//! inspection.

use crate::alpha_fold::{
    committed_fold_values, native_alpha_fold, octic_add, octic_mul, octic_scalar, octic_sub,
};
use crate::fri_fold_air::D;
use crate::inlined_perm::{
    C_BEGIN_POST, C_END_POST, C_INPUTS, C_OUTPUT, C_PARTIAL_POST_SBOX, HALF_FULL_ROUNDS,
    PARTIAL_ROUNDS,
};
use crate::inlined_perm_octic::{self, OcticPermCells, PERM_RESIDUALS};
use descender_account_stark::checked_core::{
    checked_core_matrix, CC_COLS, CC_DIGEST_BYTES, CC_ROWS, CC_SYNDROME_DIGEST_BITS,
};
use descender_account_stark::poseidon2_sponge::SpongeSpec;
use descender_account_stark::reduce_gadget::RATE;
use descender_account_stark::spend_air::{
    cc_response_base, cc_row_base, challenge_spec, frame_spec, residue_base, sigma_c_base,
    sigma_final_base, sigma_fold_base, sigma_prod_base, sigma_t_base, sigma_w_base, SpendAir,
    CHALLENGE_K_BITS, CHAL_ACCT_LANE, CHAL_DOMAIN_LANE, CHAL_MSG_LANE, CHAL_PC_LANE,
    CHAL_SLOT_LANE, CHAL_STATE_LANE, CHAL_WC_LANE, CHUNK0, CHUNK_REDUCE_CELLS, FOLD_K_BITS,
    FRAME_ACCT_LANE, FRAME_DIGEST_ELEMS, FRAME_MSG_LANE, FRAME_PACKED_LANE, FRAME_SLOT_LANE,
    FRAME_STATE_LANE, K_BITS, PV_ACCOUNT_ID_OFF, PV_FRAME_DIGEST_OFF, PV_MESSAGE_OFF,
    PV_PACKED_OFF, PV_PROOF_COMMITMENT_OFF, PV_RELATION_ID_OFF, PV_SIGMA_C_OFF, PV_SIGMA_T_OFF,
    PV_SIGMA_W_OFF, PV_SLOT_OFF, PV_STATE_ROOT_OFF, PV_WITNESS_COMMITMENT_OFF, SIGMA_PROD_COEFFS,
    SPEND_CHALLENGE_DOMAIN, SPEND_COMMIT_BYTES, SPEND_FRAME_DOMAIN, SPEND_N_PUBLIC, SPEND_Q,
    SPEND_RELATION_ID, SPEND_RELATION_ID_ELEMS, SPEND_RESIDUES, SPEND_RESIDUE_BITS,
    SPEND_STREAM_BITS,
};
use descender_account_stark::{Challenge, Val, WIDTH};
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;
use p3_koala_bear::{
    KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
    KOALABEAR_POSEIDON2_RC_16_INTERNAL,
};

extern crate alloc;
use alloc::vec::Vec;

const BYTE_BITS: usize = 8;

/// The production q128 spend AIR. This is the single constructor the recursion layer uses when it
/// embeds the spend C(zeta) mirror into a recursive window.
pub fn production_spend_air() -> SpendAir {
    SpendAir::from_arrays(
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
        KOALABEAR_POSEIDON2_RC_16_INTERNAL,
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
    )
}

/// Production spend trace width, in base-field cells.
pub fn production_trace_width() -> usize {
    production_spend_air().trace_width()
}

/// Production spend public-value width, in base-field cells.
pub const fn production_public_count() -> usize {
    SPEND_N_PUBLIC
}

/// Number of full-spend residuals emitted by the production spend C(zeta) mirror.
pub fn production_residual_count() -> usize {
    let air = production_spend_air();
    let cells = alloc::vec![Challenge::ZERO; air.trace_width()];
    let pubvals = alloc::vec![Challenge::ZERO; SPEND_N_PUBLIC];
    spend_residual_values(&air, &cells, &pubvals).len()
}

/// Native production spend C_partial fold over opened trace cells and public values.
pub fn production_c_partial(
    cells: &[Challenge],
    pubvals: &[Challenge],
    alpha: Challenge,
) -> Challenge {
    let air = production_spend_air();
    native_alpha_fold(alpha, &spend_residual_values(&air, cells, pubvals))
}

/// Native witness fill for the production spend committed fold.
pub fn production_c_partial_accumulator_values(
    cells: &[Challenge],
    pubvals: &[Challenge],
    alpha: Challenge,
) -> (Vec<Challenge>, Vec<Challenge>) {
    let air = production_spend_air();
    committed_fold_values(alpha, &spend_residual_values(&air, cells, pubvals))
}

// ============================================================================================
// Octic in-AIR residual builders (the constrained `C(zeta)` residual list, in `eval` order).
// ============================================================================================

/// `Σ_b cell(off+b)·2^b` over `nbits` bit cells (NO residual emitted — the recomposed value expr).
fn sum_bits<AB, FCell>(cell: &FCell, off: usize, nbits: usize) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let mut acc = octic_scalar::<AB>(0);
    for b in 0..nbits {
        acc = octic_add::<AB>(
            &acc,
            &octic_mul::<AB>(&cell(off + b), &octic_scalar::<AB>(1u64 << b)),
        );
    }
    acc
}

/// Push `nbits` booleanity residuals `bit·(bit−1)` for the bit cells at `off..off+nbits`.
fn push_bool_bits<AB, FCell>(cell: &FCell, off: usize, nbits: usize, out: &mut Vec<[AB::Expr; D]>)
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let one = octic_scalar::<AB>(1);
    for b in 0..nbits {
        let bit = cell(off + b);
        out.push(octic_mul::<AB>(&bit, &octic_sub::<AB>(&bit, &one)));
    }
}

/// Mirror `assert_canonical_residue`: push the 31 residuals (14 value-bit bools, value-recompose, 14
/// slack-bit bools, slack-recompose, `value+slack=q-1`) for the residue cell at `base`; return the
/// value expr `cell(base)`.
fn push_canon<AB, FCell>(cell: &FCell, base: usize, out: &mut Vec<[AB::Expr; D]>) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let qm1 = octic_scalar::<AB>((SPEND_Q - 1) as u64);
    let val = cell(base);
    push_bool_bits::<AB, _>(cell, base + 1, SPEND_RESIDUE_BITS, out);
    out.push(octic_sub::<AB>(
        &val,
        &sum_bits::<AB, _>(cell, base + 1, SPEND_RESIDUE_BITS),
    ));
    let slack = cell(base + 1 + SPEND_RESIDUE_BITS);
    push_bool_bits::<AB, _>(cell, base + 2 + SPEND_RESIDUE_BITS, SPEND_RESIDUE_BITS, out);
    out.push(octic_sub::<AB>(
        &slack,
        &sum_bits::<AB, _>(cell, base + 2 + SPEND_RESIDUE_BITS, SPEND_RESIDUE_BITS),
    ));
    out.push(octic_sub::<AB>(&octic_add::<AB>(&val, &slack), &qm1));
    val
}

/// Mirror `recompose_bits`: push `nbits` booleanity residuals and return the recomposed value expr.
fn push_recompose<AB, FCell>(
    cell: &FCell,
    off: usize,
    nbits: usize,
    out: &mut Vec<[AB::Expr; D]>,
) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    push_bool_bits::<AB, _>(cell, off, nbits, out);
    sum_bits::<AB, _>(cell, off, nbits)
}

/// `OcticPermCells` adapter so [`inlined_perm_octic::perm_residuals`] can read from a flat cell
/// closure at absolute cell indices.
struct PermCellView<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> {
    cell: &'a FCell,
    _p: core::marker::PhantomData<AB>,
}
impl<AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> OcticPermCells<AB>
    for PermCellView<'_, AB, FCell>
{
    fn at(&self, idx: usize) -> [AB::Expr; D] {
        (self.cell)(idx)
    }
}

/// Mirror [`descender_account_stark::poseidon2_sponge::eval_sponge`] as octic residuals: byte-lane
/// decompositions, per-perm absorb/capacity input wiring, and the per-block Poseidon2 perm residuals.
/// `soff` is the sponge block's first cell; `perm_cols` the per-perm column count (= inlined_perm
/// PERM_COLS, since SBOX_REGISTERS=0).
fn push_sponge<AB, FCell>(
    cell: &FCell,
    soff: usize,
    spec: &SpongeSpec,
    perm_cols: usize,
    out: &mut Vec<[AB::Expr; D]>,
) where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    // Byte-decompose each byte-ranged input lane and bind it to its value.
    let lane_bits_off = spec.n_input;
    let mut bit_cursor = 0usize;
    for i in 0..spec.n_input {
        if spec.byte_ranged[i] {
            let off = soff + lane_bits_off + bit_cursor * BYTE_BITS;
            push_bool_bits::<AB, _>(cell, off, BYTE_BITS, out);
            out.push(octic_sub::<AB>(
                &cell(soff + i),
                &sum_bits::<AB, _>(cell, off, BYTE_BITS),
            ));
            bit_cursor += 1;
        }
    }

    let absorb_elem = |pos: usize| -> [AB::Expr; D] {
        if pos < spec.n_input {
            cell(soff + pos)
        } else if pos == spec.n_input {
            octic_scalar::<AB>(1)
        } else {
            octic_scalar::<AB>(0)
        }
    };
    let perms_off = spec.perms_off();
    let padded = spec.padded_len();
    for p in 0..spec.n_perms {
        let permbase = soff + perms_off + p * perm_cols;
        let input = |c: usize| -> [AB::Expr; D] { cell(permbase + C_INPUTS + c) };
        if p == 0 {
            for r in 0..RATE {
                out.push(octic_sub::<AB>(&input(r), &absorb_elem(r)));
            }
            for c in RATE..WIDTH {
                out.push(input(c)); // capacity == 0
            }
        } else {
            let prevbase = soff + perms_off + (p - 1) * perm_cols;
            let prev = |c: usize| -> [AB::Expr; D] { cell(prevbase + C_OUTPUT + c) };
            let is_absorb = p < spec.n_absorb;
            for r in 0..RATE {
                if is_absorb {
                    let pos = p * RATE + r;
                    let elem = if pos < padded {
                        absorb_elem(pos)
                    } else {
                        octic_scalar::<AB>(0)
                    };
                    out.push(octic_sub::<AB>(
                        &input(r),
                        &octic_add::<AB>(&prev(r), &elem),
                    ));
                } else {
                    out.push(octic_sub::<AB>(&input(r), &prev(r)));
                }
            }
            for c in RATE..WIDTH {
                out.push(octic_sub::<AB>(&input(c), &prev(c)));
            }
        }
        let view = PermCellView::<AB, FCell> {
            cell,
            _p: core::marker::PhantomData,
        };
        out.extend(inlined_perm_octic::perm_residuals::<AB, _>(&view, permbase));
    }
}

/// Squeezed output expr `o` of the sponge at `soff` (the perm-output cell for `spec.output_source(o)`).
fn sponge_output<AB, FCell>(
    cell: &FCell,
    soff: usize,
    spec: &SpongeSpec,
    perm_cols: usize,
    o: usize,
) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let (sp, lane) = spec.output_source(o);
    cell(soff + spec.perms_off() + sp * perm_cols + C_OUTPUT + lane)
}

/// The FULL spend `C(zeta)` residual list over opened cells. `cell(c)` returns opened spend trace
/// column `c` (octic); `pubv(p)` returns opened public value `p` (octic). Order matches
/// [`spend_residual_values`] exactly. `air` supplies the region offsets + perm column count.
pub fn spend_residuals<AB, FCell, FPub>(
    air: &SpendAir,
    cell: &FCell,
    pubv: &FPub,
) -> Vec<[AB::Expr; D]>
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
    FPub: Fn(usize) -> [AB::Expr; D],
{
    let matrix = checked_core_matrix();
    let perm_cols = air.perm_cols();
    let q = octic_scalar::<AB>(SPEND_Q as u64);
    let one = octic_scalar::<AB>(1);
    let mut out: Vec<[AB::Expr; D]> = Vec::new();

    // ---- (1) packed-witness consumption: 18 canonical residues + 32-byte packing.
    let mut stream: Vec<[AB::Expr; D]> = Vec::with_capacity(SPEND_STREAM_BITS);
    for r in 0..SPEND_RESIDUES {
        let base = residue_base(r);
        let _ = push_canon::<AB, _>(cell, base, &mut out);
        for b in 0..SPEND_RESIDUE_BITS {
            stream.push(cell(base + 1 + b));
        }
    }
    for k in 0..SPEND_COMMIT_BYTES {
        let mut byte = octic_scalar::<AB>(0);
        for b in 0..BYTE_BITS {
            let pos = k * BYTE_BITS + b;
            if pos < SPEND_STREAM_BITS {
                byte = octic_add::<AB>(
                    &byte,
                    &octic_mul::<AB>(&stream[pos], &octic_scalar::<AB>(1u64 << b)),
                );
            }
        }
        out.push(octic_sub::<AB>(&byte, &pubv(PV_PACKED_OFF + k)));
    }

    // ---- (2) frame digest: domain pin + statement bindings + sponge + digest binding.
    let foff = air.frame_off();
    let fspec = frame_spec();
    out.push(octic_sub::<AB>(
        &cell(foff),
        &octic_scalar::<AB>(SPEND_FRAME_DOMAIN as u64),
    ));
    for (lane0, pv0, n) in [
        (FRAME_MSG_LANE, PV_MESSAGE_OFF, 32usize),
        (FRAME_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32),
        (FRAME_SLOT_LANE, PV_SLOT_OFF, 8),
        (FRAME_STATE_LANE, PV_STATE_ROOT_OFF, 32),
        (FRAME_PACKED_LANE, PV_PACKED_OFF, 32),
    ] {
        for i in 0..n {
            out.push(octic_sub::<AB>(&cell(foff + lane0 + i), &pubv(pv0 + i)));
        }
    }
    push_sponge::<AB, _>(cell, foff, &fspec, perm_cols, &mut out);
    for j in 0..FRAME_DIGEST_ELEMS {
        let o = sponge_output::<AB, _>(cell, foff, &fspec, perm_cols, j);
        out.push(octic_sub::<AB>(&o, &pubv(PV_FRAME_DIGEST_OFF + j)));
    }

    // ---- (3) checked-core SIS: 22 canonical responses + per-row chunked reductions.
    let cc = air.cc_off();
    let mut r_val: Vec<[AB::Expr; D]> = Vec::with_capacity(CC_COLS);
    for c in 0..CC_COLS {
        r_val.push(push_canon::<AB, _>(
            cell,
            cc + cc_response_base(c),
            &mut out,
        ));
    }
    for row in 0..CC_ROWS {
        let rb = cc + cc_row_base(row);
        let mut x0 = octic_scalar::<AB>(0);
        for c in 0..CHUNK0 {
            x0 = octic_add::<AB>(
                &x0,
                &octic_mul::<AB>(&r_val[c], &octic_scalar::<AB>(matrix[row][c] as u64)),
            );
        }
        let k0 = push_recompose::<AB, _>(cell, rb + 1, K_BITS, &mut out);
        let cr0 = push_canon::<AB, _>(cell, rb + 1 + K_BITS, &mut out);
        out.push(octic_sub::<AB>(
            &x0,
            &octic_add::<AB>(&octic_mul::<AB>(&k0, &q), &cr0),
        ));
        let mut x1 = octic_scalar::<AB>(0);
        for c in CHUNK0..CC_COLS {
            x1 = octic_add::<AB>(
                &x1,
                &octic_mul::<AB>(&r_val[c], &octic_scalar::<AB>(matrix[row][c] as u64)),
            );
        }
        let k1 = push_recompose::<AB, _>(cell, rb + CHUNK_REDUCE_CELLS + 1, K_BITS, &mut out);
        let cr1 = push_canon::<AB, _>(cell, rb + CHUNK_REDUCE_CELLS + 1 + K_BITS, &mut out);
        out.push(octic_sub::<AB>(
            &x1,
            &octic_add::<AB>(&octic_mul::<AB>(&k1, &q), &cr1),
        ));
        let kf = cell(rb + 2 * CHUNK_REDUCE_CELLS);
        out.push(octic_mul::<AB>(&kf, &octic_sub::<AB>(&kf, &one))); // kf boolean
        let syndrome_row = cell(residue_base(row));
        out.push(octic_sub::<AB>(
            &octic_add::<AB>(&cr0, &cr1),
            &octic_add::<AB>(&octic_mul::<AB>(&kf, &q), &syndrome_row),
        ));
    }

    // ---- (3b) sigma identity: c/w/t canonical + public-bound, products, folds + finals.
    let sg = air.sigma_off();
    let mut cl: Vec<[AB::Expr; D]> = Vec::with_capacity(CC_ROWS);
    let mut wl: Vec<[AB::Expr; D]> = Vec::with_capacity(CC_ROWS);
    let mut tl: Vec<[AB::Expr; D]> = Vec::with_capacity(CC_ROWS);
    for i in 0..CC_ROWS {
        let cv = push_canon::<AB, _>(cell, sg + sigma_c_base(i), &mut out);
        out.push(octic_sub::<AB>(&cv, &pubv(PV_SIGMA_C_OFF + i)));
        cl.push(cv);
        let wv = push_canon::<AB, _>(cell, sg + sigma_w_base(i), &mut out);
        out.push(octic_sub::<AB>(&wv, &pubv(PV_SIGMA_W_OFF + i)));
        wl.push(wv);
        let tv = push_canon::<AB, _>(cell, sg + sigma_t_base(i), &mut out);
        out.push(octic_sub::<AB>(&tv, &pubv(PV_SIGMA_T_OFF + i)));
        tl.push(tv);
    }
    let mut p_red: Vec<[AB::Expr; D]> = Vec::with_capacity(SIGMA_PROD_COEFFS);
    for k in 0..SIGMA_PROD_COEFFS {
        let mut x = octic_scalar::<AB>(0);
        for i in 0..CC_ROWS {
            if k >= i && k - i < CC_ROWS {
                x = octic_add::<AB>(&x, &octic_mul::<AB>(&cl[i], &wl[k - i]));
            }
        }
        let base = sg + sigma_prod_base(k);
        let kq = push_recompose::<AB, _>(cell, base, K_BITS, &mut out);
        let r = push_canon::<AB, _>(cell, base + K_BITS, &mut out);
        out.push(octic_sub::<AB>(
            &x,
            &octic_add::<AB>(&octic_mul::<AB>(&kq, &q), &r),
        ));
        p_red.push(r);
    }
    let q26 = octic_scalar::<AB>(26 * SPEND_Q as u64);
    let c25 = octic_scalar::<AB>(25);
    for d in 0..CC_ROWS {
        let mut y = octic_add::<AB>(&p_red[d], &q26);
        if d <= CC_ROWS - 2 {
            y = octic_sub::<AB>(&y, &octic_mul::<AB>(&c25, &p_red[CC_ROWS + d]));
        }
        if d >= 1 {
            y = octic_sub::<AB>(&y, &p_red[CC_ROWS - 1 + d]);
        }
        let fbase = sg + sigma_fold_base(d);
        let kfold = push_recompose::<AB, _>(cell, fbase, FOLD_K_BITS, &mut out);
        let reduced = push_canon::<AB, _>(cell, fbase + FOLD_K_BITS, &mut out);
        out.push(octic_sub::<AB>(
            &y,
            &octic_add::<AB>(&octic_mul::<AB>(&kfold, &q), &reduced),
        ));
        let kfin = cell(sg + sigma_final_base(d));
        out.push(octic_mul::<AB>(&kfin, &octic_sub::<AB>(&kfin, &one))); // kfin boolean
        let syndrome_d = cell(residue_base(d));
        out.push(octic_sub::<AB>(
            &octic_add::<AB>(&reduced, &tl[d]),
            &octic_add::<AB>(&octic_mul::<AB>(&kfin, &q), &syndrome_d),
        ));
    }

    // ---- (3c) sigma input commitments: w/t lanes pack to the public commitments.
    for (lane_base, commit_off) in [
        (
            sigma_w_base as fn(usize) -> usize,
            PV_WITNESS_COMMITMENT_OFF,
        ),
        (sigma_t_base as fn(usize) -> usize, PV_PROOF_COMMITMENT_OFF),
    ] {
        for k in 0..CC_DIGEST_BYTES {
            let mut byte = octic_scalar::<AB>(0);
            for b in 0..BYTE_BITS {
                let pos = k * BYTE_BITS + b;
                if pos < CC_SYNDROME_DIGEST_BITS {
                    let lane = pos / SPEND_RESIDUE_BITS;
                    let bit = pos % SPEND_RESIDUE_BITS;
                    byte = octic_add::<AB>(
                        &byte,
                        &octic_mul::<AB>(
                            &cell(sg + lane_base(lane) + 1 + bit),
                            &octic_scalar::<AB>(1u64 << b),
                        ),
                    );
                }
            }
            out.push(octic_sub::<AB>(&byte, &pubv(commit_off + k)));
        }
    }

    // ---- (3d) rehosted challenge: sponge over (statement ‖ wc ‖ pc) + squeeze reduction to sigma_c.
    let ch = air.challenge_off();
    let cspec = challenge_spec();
    out.push(octic_sub::<AB>(
        &cell(ch + CHAL_DOMAIN_LANE),
        &octic_scalar::<AB>(SPEND_CHALLENGE_DOMAIN as u64),
    ));
    for (lane0, pv0, n) in [
        (CHAL_MSG_LANE, PV_MESSAGE_OFF, 32usize),
        (CHAL_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32),
        (CHAL_SLOT_LANE, PV_SLOT_OFF, 8),
        (CHAL_STATE_LANE, PV_STATE_ROOT_OFF, 32),
        (CHAL_WC_LANE, PV_WITNESS_COMMITMENT_OFF, CC_DIGEST_BYTES),
        (CHAL_PC_LANE, PV_PROOF_COMMITMENT_OFF, CC_DIGEST_BYTES),
    ] {
        for i in 0..n {
            out.push(octic_sub::<AB>(&cell(ch + lane0 + i), &pubv(pv0 + i)));
        }
    }
    push_sponge::<AB, _>(cell, ch, &cspec, perm_cols, &mut out);
    let ck = air.challenge_k_off();
    for i in 0..CC_ROWS {
        let k =
            push_recompose::<AB, _>(cell, ck + i * CHALLENGE_K_BITS, CHALLENGE_K_BITS, &mut out);
        let chal_out = sponge_output::<AB, _>(cell, ch, &cspec, perm_cols, i);
        let sigma_c_i = cell(sg + sigma_c_base(i));
        out.push(octic_sub::<AB>(
            &chal_out,
            &octic_add::<AB>(&octic_mul::<AB>(&k, &q), &sigma_c_i),
        ));
    }

    // ---- (4) relation-id pin.
    for j in 0..SPEND_RELATION_ID_ELEMS {
        out.push(octic_sub::<AB>(
            &pubv(PV_RELATION_ID_OFF + j),
            &octic_scalar::<AB>(SPEND_RELATION_ID[j] as u64),
        ));
    }

    out
}

// ============================================================================================
// Native authority (over `Challenge`) — the SAME residual list, for the committed-fold witness fill.
// ============================================================================================

fn nsum_bits(cells: &[Challenge], off: usize, nbits: usize) -> Challenge {
    let mut acc = Challenge::ZERO;
    for b in 0..nbits {
        acc += cells[off + b] * Challenge::from_u64(1u64 << b);
    }
    acc
}

fn npush_bool_bits(cells: &[Challenge], off: usize, nbits: usize, out: &mut Vec<Challenge>) {
    let one = Challenge::ONE;
    for b in 0..nbits {
        let bit = cells[off + b];
        out.push(bit * (bit - one));
    }
}

fn npush_canon(cells: &[Challenge], base: usize, out: &mut Vec<Challenge>) -> Challenge {
    let qm1 = Challenge::from_u32(SPEND_Q - 1);
    let val = cells[base];
    npush_bool_bits(cells, base + 1, SPEND_RESIDUE_BITS, out);
    out.push(val - nsum_bits(cells, base + 1, SPEND_RESIDUE_BITS));
    let slack = cells[base + 1 + SPEND_RESIDUE_BITS];
    npush_bool_bits(
        cells,
        base + 2 + SPEND_RESIDUE_BITS,
        SPEND_RESIDUE_BITS,
        out,
    );
    out.push(slack - nsum_bits(cells, base + 2 + SPEND_RESIDUE_BITS, SPEND_RESIDUE_BITS));
    out.push((val + slack) - qm1);
    val
}

fn npush_recompose(
    cells: &[Challenge],
    off: usize,
    nbits: usize,
    out: &mut Vec<Challenge>,
) -> Challenge {
    npush_bool_bits(cells, off, nbits, out);
    nsum_bits(cells, off, nbits)
}

fn npush_sponge(
    cells: &[Challenge],
    soff: usize,
    spec: &SpongeSpec,
    perm_cols: usize,
    out: &mut Vec<Challenge>,
) {
    let lane_bits_off = spec.n_input;
    let mut bit_cursor = 0usize;
    for i in 0..spec.n_input {
        if spec.byte_ranged[i] {
            let off = soff + lane_bits_off + bit_cursor * BYTE_BITS;
            npush_bool_bits(cells, off, BYTE_BITS, out);
            out.push(cells[soff + i] - nsum_bits(cells, off, BYTE_BITS));
            bit_cursor += 1;
        }
    }
    let absorb_elem = |pos: usize| -> Challenge {
        if pos < spec.n_input {
            cells[soff + pos]
        } else if pos == spec.n_input {
            Challenge::ONE
        } else {
            Challenge::ZERO
        }
    };
    let perms_off = spec.perms_off();
    let padded = spec.padded_len();
    for p in 0..spec.n_perms {
        let permbase = soff + perms_off + p * perm_cols;
        if p == 0 {
            for r in 0..RATE {
                out.push(cells[permbase + C_INPUTS + r] - absorb_elem(r));
            }
            for c in RATE..WIDTH {
                out.push(cells[permbase + C_INPUTS + c]);
            }
        } else {
            let prevbase = soff + perms_off + (p - 1) * perm_cols;
            let prev = |c: usize| cells[prevbase + C_OUTPUT + c];
            let is_absorb = p < spec.n_absorb;
            for r in 0..RATE {
                if is_absorb {
                    let pos = p * RATE + r;
                    let elem = if pos < padded {
                        absorb_elem(pos)
                    } else {
                        Challenge::ZERO
                    };
                    out.push(cells[permbase + C_INPUTS + r] - (prev(r) + elem));
                } else {
                    out.push(cells[permbase + C_INPUTS + r] - prev(r));
                }
            }
            for c in RATE..WIDTH {
                out.push(cells[permbase + C_INPUTS + c] - prev(c));
            }
        }
        let inputs: [Challenge; WIDTH] = core::array::from_fn(|i| cells[permbase + C_INPUTS + i]);
        let mut committed = Vec::with_capacity(PERM_RESIDUALS);
        for r in 0..HALF_FULL_ROUNDS {
            for i in 0..WIDTH {
                committed.push(cells[permbase + C_BEGIN_POST + r * WIDTH + i]);
            }
        }
        for r in 0..PARTIAL_ROUNDS {
            committed.push(cells[permbase + C_PARTIAL_POST_SBOX + r]);
        }
        for r in 0..HALF_FULL_ROUNDS {
            for i in 0..WIDTH {
                committed.push(cells[permbase + C_END_POST + r * WIDTH + i]);
            }
        }
        out.extend(inlined_perm_octic::native_perm_residuals(
            inputs, &committed,
        ));
    }
}

fn nsponge_output(
    cells: &[Challenge],
    soff: usize,
    spec: &SpongeSpec,
    perm_cols: usize,
    o: usize,
) -> Challenge {
    let (sp, lane) = spec.output_source(o);
    cells[soff + spec.perms_off() + sp * perm_cols + C_OUTPUT + lane]
}

/// Native FULL spend residual values (same order/formula as [`spend_residuals`]). `cells` indexes the
/// spend trace columns (octic values), `pubvals` the public values (octic).
pub fn spend_residual_values(
    air: &SpendAir,
    cells: &[Challenge],
    pubvals: &[Challenge],
) -> Vec<Challenge> {
    let matrix = checked_core_matrix();
    let perm_cols = air.perm_cols();
    let q = Challenge::from_u32(SPEND_Q);
    let one = Challenge::ONE;
    let mut out: Vec<Challenge> = Vec::new();

    // (1) packing.
    let mut stream: Vec<Challenge> = Vec::with_capacity(SPEND_STREAM_BITS);
    for r in 0..SPEND_RESIDUES {
        let base = residue_base(r);
        let _ = npush_canon(cells, base, &mut out);
        for b in 0..SPEND_RESIDUE_BITS {
            stream.push(cells[base + 1 + b]);
        }
    }
    for k in 0..SPEND_COMMIT_BYTES {
        let mut byte = Challenge::ZERO;
        for b in 0..BYTE_BITS {
            let pos = k * BYTE_BITS + b;
            if pos < SPEND_STREAM_BITS {
                byte += stream[pos] * Challenge::from_u64(1u64 << b);
            }
        }
        out.push(byte - pubvals[PV_PACKED_OFF + k]);
    }

    // (2) frame.
    let foff = air.frame_off();
    let fspec = frame_spec();
    out.push(cells[foff] - Challenge::from_u32(SPEND_FRAME_DOMAIN));
    for (lane0, pv0, n) in [
        (FRAME_MSG_LANE, PV_MESSAGE_OFF, 32usize),
        (FRAME_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32),
        (FRAME_SLOT_LANE, PV_SLOT_OFF, 8),
        (FRAME_STATE_LANE, PV_STATE_ROOT_OFF, 32),
        (FRAME_PACKED_LANE, PV_PACKED_OFF, 32),
    ] {
        for i in 0..n {
            out.push(cells[foff + lane0 + i] - pubvals[pv0 + i]);
        }
    }
    npush_sponge(cells, foff, &fspec, perm_cols, &mut out);
    for j in 0..FRAME_DIGEST_ELEMS {
        out.push(
            nsponge_output(cells, foff, &fspec, perm_cols, j) - pubvals[PV_FRAME_DIGEST_OFF + j],
        );
    }

    // (3) SIS.
    let cc = air.cc_off();
    let mut r_val: Vec<Challenge> = Vec::with_capacity(CC_COLS);
    for c in 0..CC_COLS {
        r_val.push(npush_canon(cells, cc + cc_response_base(c), &mut out));
    }
    for row in 0..CC_ROWS {
        let rb = cc + cc_row_base(row);
        let mut x0 = Challenge::ZERO;
        for c in 0..CHUNK0 {
            x0 += r_val[c] * Challenge::from_u32(matrix[row][c] as u32);
        }
        let k0 = npush_recompose(cells, rb + 1, K_BITS, &mut out);
        let cr0 = npush_canon(cells, rb + 1 + K_BITS, &mut out);
        out.push(x0 - (k0 * q + cr0));
        let mut x1 = Challenge::ZERO;
        for c in CHUNK0..CC_COLS {
            x1 += r_val[c] * Challenge::from_u32(matrix[row][c] as u32);
        }
        let k1 = npush_recompose(cells, rb + CHUNK_REDUCE_CELLS + 1, K_BITS, &mut out);
        let cr1 = npush_canon(cells, rb + CHUNK_REDUCE_CELLS + 1 + K_BITS, &mut out);
        out.push(x1 - (k1 * q + cr1));
        let kf = cells[rb + 2 * CHUNK_REDUCE_CELLS];
        out.push(kf * (kf - one));
        let syndrome_row = cells[residue_base(row)];
        out.push((cr0 + cr1) - (kf * q + syndrome_row));
    }

    // (3b) sigma identity.
    let sg = air.sigma_off();
    let mut cl: Vec<Challenge> = Vec::with_capacity(CC_ROWS);
    let mut wl: Vec<Challenge> = Vec::with_capacity(CC_ROWS);
    let mut tl: Vec<Challenge> = Vec::with_capacity(CC_ROWS);
    for i in 0..CC_ROWS {
        let cv = npush_canon(cells, sg + sigma_c_base(i), &mut out);
        out.push(cv - pubvals[PV_SIGMA_C_OFF + i]);
        cl.push(cv);
        let wv = npush_canon(cells, sg + sigma_w_base(i), &mut out);
        out.push(wv - pubvals[PV_SIGMA_W_OFF + i]);
        wl.push(wv);
        let tv = npush_canon(cells, sg + sigma_t_base(i), &mut out);
        out.push(tv - pubvals[PV_SIGMA_T_OFF + i]);
        tl.push(tv);
    }
    let mut p_red: Vec<Challenge> = Vec::with_capacity(SIGMA_PROD_COEFFS);
    for k in 0..SIGMA_PROD_COEFFS {
        let mut x = Challenge::ZERO;
        for i in 0..CC_ROWS {
            if k >= i && k - i < CC_ROWS {
                x += cl[i] * wl[k - i];
            }
        }
        let base = sg + sigma_prod_base(k);
        let kq = npush_recompose(cells, base, K_BITS, &mut out);
        let r = npush_canon(cells, base + K_BITS, &mut out);
        out.push(x - (kq * q + r));
        p_red.push(r);
    }
    let q26 = Challenge::from_u64(26 * SPEND_Q as u64);
    let c25 = Challenge::from_u32(25);
    for d in 0..CC_ROWS {
        let mut y = p_red[d] + q26;
        if d <= CC_ROWS - 2 {
            y -= c25 * p_red[CC_ROWS + d];
        }
        if d >= 1 {
            y -= p_red[CC_ROWS - 1 + d];
        }
        let fbase = sg + sigma_fold_base(d);
        let kfold = npush_recompose(cells, fbase, FOLD_K_BITS, &mut out);
        let reduced = npush_canon(cells, fbase + FOLD_K_BITS, &mut out);
        out.push(y - (kfold * q + reduced));
        let kfin = cells[sg + sigma_final_base(d)];
        out.push(kfin * (kfin - one));
        let syndrome_d = cells[residue_base(d)];
        out.push((reduced + tl[d]) - (kfin * q + syndrome_d));
    }

    // (3c) sigma commitments.
    for (lane_base, commit_off) in [
        (
            sigma_w_base as fn(usize) -> usize,
            PV_WITNESS_COMMITMENT_OFF,
        ),
        (sigma_t_base as fn(usize) -> usize, PV_PROOF_COMMITMENT_OFF),
    ] {
        for k in 0..CC_DIGEST_BYTES {
            let mut byte = Challenge::ZERO;
            for b in 0..BYTE_BITS {
                let pos = k * BYTE_BITS + b;
                if pos < CC_SYNDROME_DIGEST_BITS {
                    let lane = pos / SPEND_RESIDUE_BITS;
                    let bit = pos % SPEND_RESIDUE_BITS;
                    byte += cells[sg + lane_base(lane) + 1 + bit] * Challenge::from_u64(1u64 << b);
                }
            }
            out.push(byte - pubvals[commit_off + k]);
        }
    }

    // (3d) challenge sponge.
    let ch = air.challenge_off();
    let cspec = challenge_spec();
    out.push(cells[ch + CHAL_DOMAIN_LANE] - Challenge::from_u32(SPEND_CHALLENGE_DOMAIN));
    for (lane0, pv0, n) in [
        (CHAL_MSG_LANE, PV_MESSAGE_OFF, 32usize),
        (CHAL_ACCT_LANE, PV_ACCOUNT_ID_OFF, 32),
        (CHAL_SLOT_LANE, PV_SLOT_OFF, 8),
        (CHAL_STATE_LANE, PV_STATE_ROOT_OFF, 32),
        (CHAL_WC_LANE, PV_WITNESS_COMMITMENT_OFF, CC_DIGEST_BYTES),
        (CHAL_PC_LANE, PV_PROOF_COMMITMENT_OFF, CC_DIGEST_BYTES),
    ] {
        for i in 0..n {
            out.push(cells[ch + lane0 + i] - pubvals[pv0 + i]);
        }
    }
    npush_sponge(cells, ch, &cspec, perm_cols, &mut out);
    let ck = air.challenge_k_off();
    for i in 0..CC_ROWS {
        let k = npush_recompose(cells, ck + i * CHALLENGE_K_BITS, CHALLENGE_K_BITS, &mut out);
        let chal_out = nsponge_output(cells, ch, &cspec, perm_cols, i);
        let sigma_c_i = cells[sg + sigma_c_base(i)];
        out.push(chal_out - (k * q + sigma_c_i));
    }

    // (4) relation-id.
    for j in 0..SPEND_RELATION_ID_ELEMS {
        out.push(pubvals[PV_RELATION_ID_OFF + j] - Challenge::from_u32(SPEND_RELATION_ID[j]));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alpha_fold::{assert_committed_fold, committed_fold_values, native_alpha_fold};
    use crate::fri_fold_air::limbs;
    use descender_account_stark::checked_core::CC_Q;
    use descender_account_stark::spend_air::{generate_spend_trace, SpendWitness, SPEND_N_PUBLIC};
    use p3_air::{Air, BaseAir, WindowAccess};
    use p3_field::BasedVectorSpace;
    use p3_koala_bear::{
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
        KOALABEAR_POSEIDON2_RC_16_INTERNAL,
    };
    use p3_matrix::dense::RowMajorMatrix;

    fn air() -> SpendAir {
        SpendAir::from_arrays(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
        )
    }

    fn sample_witness() -> SpendWitness {
        SpendWitness {
            sigma_w: core::array::from_fn(|i| ((i as u32 * 149 + 29) % CC_Q as u32) as u16),
            sigma_t: core::array::from_fn(|i| ((i as u32 * 71 + 5) % CC_Q as u32) as u16),
            projection: core::array::from_fn(|i| ((i as u32 * 211 + 5) % CC_Q as u32) as u16),
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            slot_id: 0xDEAD_BEEF_0000_0042,
            account_state_root: core::array::from_fn(|i| {
                (i as u8).wrapping_mul(17).wrapping_add(7)
            }),
            message_digest: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
        }
    }

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1))
        })
    }

    /// Lift a real spend trace (row 0) + public values into octic cells — the HONEST opened cells an
    /// out-of-domain opening of a valid spend proof would carry (every residual must vanish).
    fn honest_cells() -> (SpendAir, Vec<Challenge>, Vec<Challenge>) {
        let a = air();
        let (trace, pv, _fd) = generate_spend_trace(&a, &sample_witness());
        let w = a.trace_width();
        let cells: Vec<Challenge> = (0..w).map(|c| Challenge::from(trace.values[c])).collect();
        let pubvals: Vec<Challenge> = pv.iter().map(|&v| Challenge::from(v)).collect();
        (a, cells, pubvals)
    }

    /// FIDELITY (honest): every residual of the FULL spend mirror vanishes on a real lifted trace —
    /// proving the octic transcription re-derives the entire spend relation correctly.
    #[test]
    fn full_spend_residuals_vanish_on_honest_trace() {
        let (a, cells, pubvals) = honest_cells();
        let res = spend_residual_values(&a, &cells, &pubvals);
        let nonzero = res.iter().filter(|&&r| r != Challenge::ZERO).count();
        assert_eq!(
            nonzero,
            0,
            "{nonzero} / {} spend residuals nonzero on honest trace",
            res.len()
        );
    }

    const ALPHA_SEED: u32 = 0xC0FFEE;

    /// A test AIR folding the FULL spend `C(zeta)` residuals via the shared committed-accumulator
    /// primitive, exposing the fold result `Σ alpha^r·res_r` as public values. Columns:
    ///   trace cells (W·D) ‖ pub cells (P·D) ‖ alpha(D) ‖ apow(K·D) ‖ acc(K·D).
    struct SpendFoldAir {
        air: SpendAir,
        w: usize, // trace width (cells)
        p: usize, // public count (cells)
        k: usize, // residual count
    }
    impl SpendFoldAir {
        fn new() -> Self {
            let air = air();
            let w = air.trace_width();
            let p = SPEND_N_PUBLIC;
            let cells: Vec<Challenge> = (0..w as u32).map(|c| ext(c + 1)).collect();
            let pubvals: Vec<Challenge> = (0..p as u32).map(|x| ext(x + 5_000_000)).collect();
            let k = spend_residual_values(&air, &cells, &pubvals).len();
            Self { air, w, p, k }
        }
        fn pub_off(&self) -> usize {
            self.w * D
        }
        fn alpha_off(&self) -> usize {
            self.pub_off() + self.p * D
        }
        fn apow_off(&self) -> usize {
            self.alpha_off() + D
        }
        fn acc_off(&self) -> usize {
            self.apow_off() + self.k * D
        }
        fn width_cols(&self) -> usize {
            self.acc_off() + self.k * D
        }
    }
    impl<F: Sync> BaseAir<F> for SpendFoldAir {
        fn width(&self) -> usize {
            self.width_cols()
        }
        fn num_public_values(&self) -> usize {
            D
        }
    }
    impl<AB: AirBuilder<F = Val>> Air<AB> for SpendFoldAir {
        fn eval(&self, builder: &mut AB) {
            let main = builder.main();
            let local = main.current_slice().to_vec();
            let public = builder.public_values().to_vec();
            let pub_off = self.pub_off();
            let alpha_off = self.alpha_off();
            let apow_off = self.apow_off();
            let acc_off = self.acc_off();
            let cell =
                |c: usize| -> [AB::Expr; D] { core::array::from_fn(|i| local[c * D + i].into()) };
            let pubv = |p: usize| -> [AB::Expr; D] {
                core::array::from_fn(|i| local[pub_off + p * D + i].into())
            };
            let apow = |i: usize| -> [AB::Expr; D] {
                core::array::from_fn(|j| local[apow_off + i * D + j].into())
            };
            let acc = |i: usize| -> [AB::Expr; D] {
                core::array::from_fn(|j| local[acc_off + i * D + j].into())
            };
            let alpha: [AB::Expr; D] = core::array::from_fn(|j| local[alpha_off + j].into());
            let residuals = spend_residuals::<AB, _, _>(&self.air, &cell, &pubv);
            let result =
                assert_committed_fold::<AB, _, _>(builder, &residuals, &apow, &acc, &alpha);
            for j in 0..D {
                builder.assert_eq(result[j].clone(), public[j].clone().into());
            }
        }
    }

    /// Build the fold trace (single row, padded to 2) from opened cells + public values + alpha.
    fn build(
        fa: &SpendFoldAir,
        cells: &[Challenge],
        pubvals: &[Challenge],
        alpha: Challenge,
    ) -> (RowMajorMatrix<Val>, Vec<Val>) {
        let resv = spend_residual_values(&fa.air, cells, pubvals);
        assert_eq!(resv.len(), fa.k, "residual count drift");
        let (apow, accv) = committed_fold_values(alpha, &resv);
        let width = fa.width_cols();
        let mut row = alloc::vec![Val::ZERO; width];
        for (c, v) in cells.iter().enumerate() {
            row[c * D..c * D + D].copy_from_slice(&limbs(*v));
        }
        for (p, v) in pubvals.iter().enumerate() {
            row[fa.pub_off() + p * D..fa.pub_off() + p * D + D].copy_from_slice(&limbs(*v));
        }
        row[fa.alpha_off()..fa.alpha_off() + D].copy_from_slice(&limbs(alpha));
        for i in 0..fa.k {
            row[fa.apow_off() + i * D..fa.apow_off() + i * D + D].copy_from_slice(&limbs(apow[i]));
            row[fa.acc_off() + i * D..fa.acc_off() + i * D + D].copy_from_slice(&limbs(accv[i]));
        }
        let result = *accv.last().unwrap();
        // FIDELITY: committed fold result == native Σ alpha^r·res_r.
        assert_eq!(result, native_alpha_fold(alpha, &resv));
        let pv = limbs(result).to_vec();
        let mut values = Vec::with_capacity(2 * width);
        values.extend_from_slice(&row);
        values.extend_from_slice(&row);
        (RowMajorMatrix::new(values, width), pv)
    }

    /// CONSTRAINT-LEVEL: the FULL spend `C(zeta)` mirror + committed fold hold on a real honest trace
    /// (every residual zero → fold result zero) — the spend relation is re-derived constraint-for-
    /// constraint over opened cells and folds via the re-arithmetization primitive.
    #[test]
    fn full_spend_czeta_fold_constraints_hold_honest() {
        use p3_air::check_all_constraints;
        let fa = SpendFoldAir::new();
        let (_a, cells, pubvals) = honest_cells();
        let alpha = ext(ALPHA_SEED);
        let (trace, pv) = build(&fa, &cells, &pubvals, alpha);
        assert_eq!(
            pv,
            limbs(Challenge::ZERO).to_vec(),
            "honest C(zeta) must be 0"
        );
        let report = check_all_constraints(&fa, &trace, &pv, None);
        assert!(
            report.is_ok(),
            "full spend fold rejected: {} failures",
            report.failures.len()
        );
    }

    /// CONSTRAINT-LEVEL (arbitrary cells): the in-AIR residuals match native on arbitrary opened
    /// cells — the fold reconstructs `Σ alpha^r·res_r` and `check_all_constraints` passes (pins the
    /// octic transcription to the native authority limb-for-limb under nonzero residuals).
    #[test]
    fn full_spend_czeta_fold_constraints_hold_arbitrary() {
        use p3_air::check_all_constraints;
        let fa = SpendFoldAir::new();
        let cells: Vec<Challenge> = (0..fa.w as u32).map(|c| ext(c + 1)).collect();
        let pubvals: Vec<Challenge> = (0..fa.p as u32).map(|x| ext(x + 5_000_000)).collect();
        let alpha = ext(ALPHA_SEED);
        let (trace, pv) = build(&fa, &cells, &pubvals, alpha);
        let report = check_all_constraints(&fa, &trace, &pv, None);
        assert!(
            report.is_ok(),
            "full spend fold (arbitrary) rejected: {} failures",
            report.failures.len()
        );
    }

    /// THE POINT: the FULL spend `C(zeta)` — every constraint class — folds PROVABLY at PRODUCTION
    /// q128 knobs via the shared re-arithmetization primitive. Closes the spend full-C(zeta) gap.
    #[test]
    #[ignore = "slow: full spend C(zeta) fold prove at production q128 knobs (~minutes)"]
    fn full_spend_czeta_folds_provably_q128() {
        use descender_account_stark::{
            Challenge as Ch, MyCompress, MyHash, Perm, DIGEST_ELEMS, PROD_LOG_BLOWUP,
            PROD_NUM_QUERIES, PROD_QUERY_POW_BITS,
        };
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            MyHash,
            MyCompress,
            2,
            DIGEST_ELEMS,
        >;
        type ChallengeMmcs = ExtensionMmcs<Val, Ch, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 7>;
        type OuterConfig = StarkConfig<Pcs, Ch, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(
            MyHash::new(perm.clone()),
            MyCompress::new(MyHash::new(perm.clone())),
            0,
        );
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES,
            commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS,
            mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));

        let fa = SpendFoldAir::new();
        // Honest opened cells: a REAL spend C(zeta) (all residuals zero) proves the relation; we use
        // arbitrary cells here to exercise the fold over NONZERO residuals (the harder prove).
        let cells: Vec<Challenge> = (0..fa.w as u32).map(|c| ext(c + 1)).collect();
        let pubvals: Vec<Challenge> = (0..fa.p as u32).map(|x| ext(x + 5_000_000)).collect();
        let alpha = ext(ALPHA_SEED);
        let (trace, pv) = build(&fa, &cells, &pubvals, alpha);
        let proof = prove(&config, &fa, trace, &pv);
        verify(&config, &fa, &proof, &pv)
            .expect("full spend C(zeta) must prove+verify at production q128");
    }
}
