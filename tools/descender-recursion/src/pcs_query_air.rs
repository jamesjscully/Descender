//! Production-shaped PCS query link.
//!
//! This AIR closes the host-only seam between the vertical input-MMCS openings and
//! [`crate::pcs_opening_reduction_air`].  Each row carries up to seven ordered
//! `p_at_x` values.  Those exact cells are simultaneously:
//!
//! 1. absorbed into the deployed salted input-MMCS leaf hash (with the matrix salt
//!    cells inserted at the source-pinned matrix boundaries), and
//! 2. consumed by Plonky3's ordered `open_input` reduction against the corresponding
//!    `p_at_z` extension values.
//!
//! In parallel, the `p_at_z` limbs are duplexed in the exact rate-7 challenger
//! order from the public post-zeta state.  The terminal state is public-bound and
//! the FRI batch-combination challenge is derived from that state with the exact
//! seven-pop-plus-refill degree-eight sample.  Thus the reducer cannot choose a
//! claim stream independent of the external OOD transcript.
//!
//! The public statement exposes the three salted leaf digests, the query index
//! bits, and the terminal reduced opening.  The leaf digests are the bridge to
//! [`crate::vertical_mmcs_air`]; the reduced opening is the bridge to the FRI query
//! chain.  A settlement envelope must verify those companion proofs against this
//! same statement.  This module deliberately does not call that constant-count
//! envelope complete until the FRI-side bridge is installed.

use crate::alpha_fold::{octic_add, octic_mul, octic_sub};
use crate::challenger::RATE;
use crate::fri_fold_air::{limbs, D};
use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_INPUTS, C_OUTPUT, PERM_COLS,
};
use crate::leaf_hash::LEAF_HASH_RATE;
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, SALT_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, Field, PrimeCharacteristicRing, TwoAdicField};
use p3_matrix::dense::RowMajorMatrix;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

const LANES: usize = LEAF_HASH_RATE;
const MAX_LEAF_PERMS: usize = 2;
const MAX_OOD_PERMS: usize = 9;
const OOD_PERM_BASE: usize = MAX_LEAF_PERMS;
const BCA_PERM_INDEX: usize = OOD_PERM_BASE + MAX_OOD_PERMS;
const N_PERMS: usize = BCA_PERM_INDEX + 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PcsBatchShape {
    /// One width per matrix, in the exact MMCS batch order.  Widths already
    /// include the hiding random-codeword columns consumed by `open_input`.
    pub matrix_widths: Vec<usize>,
}

impl PcsBatchShape {
    pub fn new(matrix_widths: Vec<usize>) -> Self {
        assert!(!matrix_widths.is_empty());
        assert!(matrix_widths.iter().all(|&w| w > 0));
        Self { matrix_widths }
    }
}

#[derive(Clone, Debug)]
struct RowPlan {
    batch: usize,
    terms: usize,
    salts: usize,
    batch_end: bool,
    repeat: usize,
    leaf_buffer_in: usize,
    leaf_buffer_out: usize,
    leaf_perms: usize,
    leaf_final_n: usize,
    ood_buffer_in: usize,
    ood_buffer_out: usize,
    ood_perms: usize,
    ood_final_n: usize,
}

impl RowPlan {
    fn same_for_rle(&self, other: &Self) -> bool {
        self.batch == other.batch
            && self.terms == other.terms
            && self.salts == other.salts
            && self.batch_end == other.batch_end
            && self.leaf_buffer_in == other.leaf_buffer_in
            && self.leaf_buffer_out == other.leaf_buffer_out
            && self.leaf_perms == other.leaf_perms
            && self.leaf_final_n == other.leaf_final_n
            && self.ood_buffer_in == other.ood_buffer_in
            && self.ood_buffer_out == other.ood_buffer_out
            && self.ood_perms == other.ood_perms
            && self.ood_final_n == other.ood_final_n
    }
}

#[derive(Clone, Debug)]
pub struct PcsQueryAir {
    pub batches: Vec<PcsBatchShape>,
    pub log_height: usize,
    plans: Vec<RowPlan>,
    active_rows: usize,
    terms: usize,
}

impl PcsQueryAir {
    pub fn new(batches: Vec<PcsBatchShape>, log_height: usize) -> Self {
        assert!(!batches.is_empty());
        assert!(log_height > 0 && log_height <= Val::TWO_ADICITY);

        let mut raw = Vec::<RowPlan>::new();
        let mut ood_buffer = 0usize;
        let mut terms = 0usize;
        for (batch, shape) in batches.iter().enumerate() {
            let mut leaf_buffer = 0usize;
            for (matrix, &width) in shape.matrix_widths.iter().enumerate() {
                let chunks = width.div_ceil(LANES);
                for chunk in 0..chunks {
                    let n_terms = core::cmp::min(LANES, width - chunk * LANES);
                    let matrix_end = chunk + 1 == chunks;
                    let salts = if matrix_end { SALT_ELEMS } else { 0 };
                    let batch_end = matrix_end && matrix + 1 == shape.matrix_widths.len();
                    let leaf_total = leaf_buffer + n_terms + salts;
                    let leaf_perms = if batch_end {
                        leaf_total.div_ceil(RATE)
                    } else {
                        leaf_total / RATE
                    };
                    let leaf_out = if batch_end { 0 } else { leaf_total % RATE };
                    let leaf_final_n = if batch_end && leaf_total % RATE != 0 {
                        leaf_total % RATE
                    } else {
                        RATE
                    };

                    let ood_total = ood_buffer + n_terms * D;
                    let stream_end = batch_end && batch + 1 == batches.len();
                    let ood_perms = if stream_end {
                        ood_total.div_ceil(RATE)
                    } else {
                        ood_total / RATE
                    };
                    let ood_out = if stream_end { 0 } else { ood_total % RATE };
                    let ood_final_n = if stream_end && ood_total % RATE != 0 {
                        ood_total % RATE
                    } else {
                        RATE
                    };
                    assert!(leaf_perms <= MAX_LEAF_PERMS);
                    assert!(ood_perms <= MAX_OOD_PERMS);
                    raw.push(RowPlan {
                        batch,
                        terms: n_terms,
                        salts,
                        batch_end,
                        repeat: 1,
                        leaf_buffer_in: leaf_buffer,
                        leaf_buffer_out: leaf_out,
                        leaf_perms,
                        leaf_final_n,
                        ood_buffer_in: ood_buffer,
                        ood_buffer_out: ood_out,
                        ood_perms,
                        ood_final_n,
                    });
                    leaf_buffer = leaf_out;
                    ood_buffer = ood_out;
                    terms += n_terms;
                }
            }
            assert_eq!(leaf_buffer, 0, "batch-final partial leaf must be permuted");
        }
        assert_eq!(
            ood_buffer, 0,
            "final OOD partial must be duplexed before sampling"
        );

        let active_rows = raw.len();
        let mut plans = Vec::<RowPlan>::new();
        for plan in raw {
            match plans.last_mut() {
                Some(last) if last.same_for_rle(&plan) => last.repeat += 1,
                _ => plans.push(plan),
            }
        }
        Self {
            batches,
            log_height,
            plans,
            active_rows,
            terms,
        }
    }

    pub fn production(log_height: usize) -> Self {
        use crate::pcs_opening_shape::{
            QUOTIENT_MATRIX_WIDTH_V1, RANDOM_MATRIX_WIDTH_V1, TRACE_MATRIX_WIDTH_V1,
        };
        Self::new(
            vec![
                PcsBatchShape::new(vec![RANDOM_MATRIX_WIDTH_V1]),
                PcsBatchShape::new(vec![TRACE_MATRIX_WIDTH_V1]),
                PcsBatchShape::new(vec![
                    QUOTIENT_MATRIX_WIDTH_V1;
                    crate::pcs_opening_shape::PRODUCTION_QUOTIENT_MATRICES_V1
                ]),
            ],
            log_height,
        )
    }

    pub fn terms(&self) -> usize {
        self.terms
    }

    pub fn active_rows(&self) -> usize {
        self.active_rows
    }

    pub fn trace_height(&self) -> usize {
        (self.active_rows + 1).next_power_of_two().max(2)
    }

    pub fn width(&self) -> usize {
        Layout::new(self).n_cols
    }

    pub fn public_width(&self) -> usize {
        PublicLayout::new(self).n_public
    }

    pub fn base_witness_bytes(&self) -> usize {
        self.trace_height() * self.width() * core::mem::size_of::<Val>()
    }
}

#[derive(Clone, Copy)]
struct Layout {
    n_sel: usize,
    seg_last: usize,
    seg_pos: usize,
    seg_inv: usize,
    leaf_state: usize,
    leaf_buffer: usize,
    ood_state: usize,
    ood_buffer: usize,
    terms: usize,
    salts: usize,
    p_at_z: usize,
    alpha: usize,
    z: usize,
    x: usize,
    inv: usize,
    alpha_pow: usize,
    ro: usize,
    x_tower: usize,
    perm: usize,
    n_cols: usize,
}

impl Layout {
    fn new(air: &PcsQueryAir) -> Self {
        let n_sel = air.plans.len() + 1;
        let seg_last = n_sel;
        let seg_pos = seg_last + 1;
        let seg_inv = seg_pos + 1;
        let leaf_state = seg_inv + 1;
        let leaf_buffer = leaf_state + WIDTH;
        let ood_state = leaf_buffer + RATE;
        let ood_buffer = ood_state + WIDTH;
        let terms = ood_buffer + RATE;
        let salts = terms + LANES;
        let p_at_z = salts + SALT_ELEMS;
        let alpha = p_at_z + LANES * D;
        let z = alpha + D;
        let x = z + D;
        let inv = x + 1;
        let alpha_pow = inv + D;
        let ro = alpha_pow + (LANES + 1) * D;
        let x_tower = ro + (LANES + 1) * D;
        let perm = x_tower + air.log_height + 1;
        let n_cols = perm + N_PERMS * PERM_COLS;
        Self {
            n_sel,
            seg_last,
            seg_pos,
            seg_inv,
            leaf_state,
            leaf_buffer,
            ood_state,
            ood_buffer,
            terms,
            salts,
            p_at_z,
            alpha,
            z,
            x,
            inv,
            alpha_pow,
            ro,
            x_tower,
            perm,
            n_cols,
        }
    }

    fn pad(self) -> usize {
        self.n_sel - 1
    }

    fn alpha_pow_step(self, step: usize) -> usize {
        self.alpha_pow + step * D
    }

    fn ro_step(self, step: usize) -> usize {
        self.ro + step * D
    }

    fn perm_block(self, block: usize) -> usize {
        self.perm + block * PERM_COLS
    }
}

#[derive(Clone, Copy)]
struct PublicLayout {
    alpha: usize,
    z: usize,
    x: usize,
    start: usize,
    end: usize,
    leaf_digests: usize,
    index_bits: usize,
    reduced_opening: usize,
    n_public: usize,
}

impl PublicLayout {
    fn new(air: &PcsQueryAir) -> Self {
        let alpha = 0;
        let z = alpha + D;
        let x = z + D;
        let start = x + 1;
        let end = start + WIDTH;
        let leaf_digests = end + WIDTH;
        let index_bits = leaf_digests + air.batches.len() * DIGEST_ELEMS;
        let reduced_opening = index_bits + air.log_height;
        let n_public = reduced_opening + D;
        Self {
            alpha,
            z,
            x,
            start,
            end,
            leaf_digests,
            index_bits,
            reduced_opening,
            n_public,
        }
    }
}

impl<F: Sync> BaseAir<F> for PcsQueryAir {
    fn width(&self) -> usize {
        self.width()
    }

    fn num_public_values(&self) -> usize {
        self.public_width()
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

fn ext_expr<AB: AirBuilder>(local: &[AB::Var], base: usize) -> [AB::Expr; D] {
    core::array::from_fn(|i| local[base + i].into())
}

fn ext_scalar_expr<AB: AirBuilder>(value: AB::Expr) -> [AB::Expr; D] {
    core::array::from_fn(|i| {
        if i == 0 {
            value.clone()
        } else {
            AB::Expr::ZERO
        }
    })
}

fn ext_one_expr<AB: AirBuilder>() -> [AB::Expr; D] {
    core::array::from_fn(|i| {
        if i == 0 {
            AB::Expr::ONE
        } else {
            AB::Expr::ZERO
        }
    })
}

fn leaf_source_expr<AB: AirBuilder<F = Val>>(
    l: &impl Fn(usize) -> AB::Expr,
    lay: Layout,
    plan: &RowPlan,
    index: usize,
) -> AB::Expr {
    if index < plan.leaf_buffer_in {
        l(lay.leaf_buffer + index)
    } else {
        let index = index - plan.leaf_buffer_in;
        if index < plan.terms {
            l(lay.terms + index)
        } else {
            l(lay.salts + index - plan.terms)
        }
    }
}

fn ood_source_expr<AB: AirBuilder<F = Val>>(
    l: &impl Fn(usize) -> AB::Expr,
    lay: Layout,
    plan: &RowPlan,
    index: usize,
) -> AB::Expr {
    if index < plan.ood_buffer_in {
        l(lay.ood_buffer + index)
    } else {
        let index = index - plan.ood_buffer_in;
        l(lay.p_at_z + index)
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for PcsQueryAir {
    fn eval(&self, builder: &mut AB) {
        let lay = Layout::new(self);
        let pv = PublicLayout::new(self);
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
        let sel = |s: usize| -> AB::Expr { l(s) };
        let one = AB::Expr::ONE;

        let mut sel_sum = AB::Expr::ZERO;
        for s in 0..lay.n_sel {
            builder.assert_bool(sel(s));
            sel_sum += sel(s);
        }
        builder.assert_one(sel_sum);
        let active = one.clone() - sel(lay.pad());
        let seg_last = l(lay.seg_last);
        builder.assert_bool(seg_last.clone());
        builder.assert_zero(sel(lay.pad()) * seg_last.clone());

        for (s, plan) in self.plans.iter().enumerate() {
            let gate = sel(s);
            let diff = l(lay.seg_pos) - AB::Expr::from(Val::from_usize(plan.repeat - 1));
            builder.assert_zero(gate.clone() * diff.clone() * seg_last.clone());
            builder.assert_zero(gate * (diff * l(lay.seg_inv) - (one.clone() - seg_last.clone())));
        }
        builder.assert_zero(sel(lay.pad()) * l(lay.seg_pos));
        builder.assert_zero(sel(lay.pad()) * l(lay.seg_inv));

        // Every permutation block is fully constrained.  Phase gates below make
        // the blocks load-bearing only when their row plan consumes them.
        let cells = Cells::<AB> { row: local.clone() };
        let constants = ConstantsExpr::<AB>::pinned();
        for k in 0..N_PERMS {
            eval_perm::<AB, _>(builder, &cells, lay.perm_block(k), &constants);
        }

        // Matrix/batch salted leaf hashing.  The exact plan fixes which cells are
        // polynomial values and where the four MMCS salts enter.
        for (s, plan) in self.plans.iter().enumerate() {
            let gate = sel(s);
            for j in plan.terms..LANES {
                builder.assert_zero(gate.clone() * l(lay.terms + j));
                for limb in 0..D {
                    builder.assert_zero(gate.clone() * l(lay.p_at_z + j * D + limb));
                }
            }
            for j in plan.salts..SALT_ELEMS {
                builder.assert_zero(gate.clone() * l(lay.salts + j));
            }

            let total = plan.leaf_buffer_in + plan.terms + plan.salts;
            for k in 0..plan.leaf_perms {
                let off = lay.perm_block(k);
                let prev = |i: usize| -> AB::Expr {
                    if k == 0 {
                        l(lay.leaf_state + i)
                    } else {
                        l(lay.perm_block(k - 1) + C_OUTPUT + i)
                    }
                };
                let begin = k * RATE;
                let available = total.saturating_sub(begin);
                let absorb = core::cmp::min(RATE, available);
                for j in 0..WIDTH {
                    let expected = if j < absorb {
                        leaf_source_expr::<AB>(&l, lay, plan, begin + j)
                    } else {
                        prev(j)
                    };
                    builder.assert_zero(gate.clone() * (l(off + C_INPUTS + j) - expected));
                }
            }

            if plan.batch_end && pubs.len() == pv.n_public {
                let final_gate = gate.clone() * seg_last.clone();
                let out = lay.perm_block(plan.leaf_perms - 1) + C_OUTPUT;
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(
                        final_gate.clone()
                            * (l(out + i)
                                - pubs[pv.leaf_digests + plan.batch * DIGEST_ELEMS + i].clone()),
                    );
                }
            }

            // OOD stream: prior buffered limbs followed by term-major p_at_z limbs,
            // packed into full rate-7 challenger duplexes.
            for k in 0..plan.ood_perms {
                let off = lay.perm_block(OOD_PERM_BASE + k);
                let prev = |i: usize| -> AB::Expr {
                    if k == 0 {
                        l(lay.ood_state + i)
                    } else {
                        l(lay.perm_block(OOD_PERM_BASE + k - 1) + C_OUTPUT + i)
                    }
                };
                let is_last_partial = k + 1 == plan.ood_perms && plan.ood_final_n < RATE;
                let absorb = if is_last_partial {
                    plan.ood_final_n
                } else {
                    RATE
                };
                for j in 0..WIDTH {
                    let expected = if j < absorb {
                        ood_source_expr::<AB>(&l, lay, plan, k * RATE + j)
                    } else if j < RATE {
                        AB::Expr::ZERO
                    } else if j == RATE {
                        prev(j) + AB::Expr::from(Val::from_usize(absorb))
                    } else {
                        prev(j)
                    };
                    builder.assert_zero(gate.clone() * (l(off + C_INPUTS + j) - expected));
                }
            }
        }

        // Opening reduction.  Seven committed intermediate states keep the
        // extension multiplication degree bounded even on a seven-term row.
        let alpha = ext_expr::<AB>(&local, lay.alpha);
        let z = ext_expr::<AB>(&local, lay.z);
        let inv = ext_expr::<AB>(&local, lay.inv);
        let denominator = octic_sub::<AB>(&z, &ext_scalar_expr::<AB>(l(lay.x)));
        let inverse_check = octic_mul::<AB>(&denominator, &inv);
        let ext_one = ext_one_expr::<AB>();
        for i in 0..D {
            builder.assert_zero(active.clone() * (inverse_check[i].clone() - ext_one[i].clone()));
        }
        for (s, plan) in self.plans.iter().enumerate() {
            let gate = sel(s);
            for j in 0..LANES {
                let ap = ext_expr::<AB>(&local, lay.alpha_pow_step(j));
                let ro = ext_expr::<AB>(&local, lay.ro_step(j));
                let ap_out = ext_expr::<AB>(&local, lay.alpha_pow_step(j + 1));
                let ro_out = ext_expr::<AB>(&local, lay.ro_step(j + 1));
                let term_active = j < plan.terms;
                let (want_ap, want_ro) = if term_active {
                    let pz = ext_expr::<AB>(&local, lay.p_at_z + j * D);
                    let px = ext_scalar_expr::<AB>(l(lay.terms + j));
                    let quotient = octic_mul::<AB>(&octic_sub::<AB>(&pz, &px), &inv);
                    (
                        octic_mul::<AB>(&ap, &alpha),
                        octic_add::<AB>(&ro, &octic_mul::<AB>(&ap, &quotient)),
                    )
                } else {
                    (ap, ro)
                };
                for i in 0..D {
                    builder.assert_zero(gate.clone() * (ap_out[i].clone() - want_ap[i].clone()));
                    builder.assert_zero(gate.clone() * (ro_out[i].clone() - want_ro[i].clone()));
                }
            }
        }
        for j in 0..LANES {
            for i in 0..D {
                builder.assert_zero(
                    sel(lay.pad())
                        * (l(lay.alpha_pow_step(j + 1) + i) - l(lay.alpha_pow_step(j) + i)),
                );
                builder.assert_zero(
                    sel(lay.pad()) * (l(lay.ro_step(j + 1) + i) - l(lay.ro_step(j) + i)),
                );
            }
        }

        // The query point x is verifier-derived from the same public index bits
        // used by each vertical MMCS proof.
        if pubs.len() == pv.n_public {
            builder.assert_eq(l(lay.x_tower), AB::Expr::from(Val::GENERATOR));
            let omega = Val::two_adic_generator(self.log_height);
            for k in 0..self.log_height {
                let bit = pubs[pv.index_bits + k].clone();
                builder.assert_bool(bit.clone());
                let exponent = 1u64 << (self.log_height - 1 - k);
                let factor = AB::Expr::from(omega.exp_u64(exponent));
                let multiplier = one.clone() + bit * (factor - one.clone());
                builder.assert_eq(l(lay.x_tower + k + 1), l(lay.x_tower + k) * multiplier);
            }
            builder.assert_eq(l(lay.x), l(lay.x_tower + self.log_height));
        }

        let final_gate = sel(self.plans.len() - 1) * seg_last.clone();
        let final_plan = self.plans.last().expect("nonempty plans");
        let end_base = lay.perm_block(OOD_PERM_BASE + final_plan.ood_perms - 1) + C_OUTPUT;
        if pubs.len() == pv.n_public {
            for i in 0..WIDTH {
                builder
                    .assert_zero(final_gate.clone() * (l(end_base + i) - pubs[pv.end + i].clone()));
            }
        }
        // sample_algebra_element from a rate-aligned OOD terminal state: seven
        // LIFO pops, then one squeeze-only permutation for limb seven.
        let bca = lay.perm_block(BCA_PERM_INDEX);
        for i in 0..WIDTH {
            // Empty-buffer duplexing is a pure squeeze: it permutes the
            // current state without clearing or tagging the rate lanes.
            let expected = l(end_base + i);
            builder.assert_zero(final_gate.clone() * (l(bca + C_INPUTS + i) - expected));
        }
        for i in 0..D {
            let sampled = if i < RATE {
                l(end_base + RATE - 1 - i)
            } else {
                l(bca + C_OUTPUT + RATE - 1)
            };
            builder.assert_zero(final_gate.clone() * (l(lay.alpha + i) - sampled));
        }

        // Boundaries and transitions.
        {
            let mut first = builder.when_first_row();
            first.assert_one(sel(0));
            first.assert_zero(l(lay.seg_pos));
            for i in 0..WIDTH {
                first.assert_zero(l(lay.leaf_state + i));
                if pubs.len() == pv.n_public {
                    first.assert_eq(l(lay.ood_state + i), pubs[pv.start + i].clone());
                }
            }
            for i in 0..RATE {
                first.assert_zero(l(lay.leaf_buffer + i));
                first.assert_zero(l(lay.ood_buffer + i));
            }
            for i in 0..D {
                first.assert_eq(l(lay.alpha_pow_step(0) + i), ext_one[i].clone());
                first.assert_zero(l(lay.ro_step(0) + i));
                if pubs.len() == pv.n_public {
                    first.assert_eq(l(lay.alpha + i), pubs[pv.alpha + i].clone());
                    first.assert_eq(l(lay.z + i), pubs[pv.z + i].clone());
                }
            }
            if pubs.len() == pv.n_public {
                first.assert_eq(l(lay.x), pubs[pv.x].clone());
            }
        }

        let mut tr = builder.when_transition();
        for s in 0..self.plans.len() {
            let from_prev = if s == 0 {
                AB::Expr::ZERO
            } else {
                sel(s - 1) * seg_last.clone()
            };
            tr.assert_eq(n(s), sel(s) * (one.clone() - seg_last.clone()) + from_prev);
        }
        tr.assert_eq(
            n(lay.pad()),
            sel(lay.pad()) + sel(self.plans.len() - 1) * seg_last.clone(),
        );
        tr.assert_eq(
            n(lay.seg_pos),
            active.clone() * (one.clone() - seg_last.clone()) * (l(lay.seg_pos) + one.clone()),
        );

        for i in 0..D {
            tr.assert_eq(n(lay.alpha + i), l(lay.alpha + i));
            tr.assert_eq(n(lay.z + i), l(lay.z + i));
            tr.assert_eq(n(lay.inv + i), l(lay.inv + i));
            tr.assert_eq(
                n(lay.alpha_pow_step(0) + i),
                l(lay.alpha_pow_step(LANES) + i),
            );
            tr.assert_eq(n(lay.ro_step(0) + i), l(lay.ro_step(LANES) + i));
        }
        tr.assert_eq(n(lay.x), l(lay.x));
        for k in 0..=self.log_height {
            tr.assert_eq(n(lay.x_tower + k), l(lay.x_tower + k));
        }

        // State/buffer transitions are a selector-weighted fixed schedule.
        for i in 0..WIDTH {
            let mut leaf_next = AB::Expr::ZERO;
            let mut ood_next = sel(lay.pad()) * l(lay.ood_state + i);
            for (s, plan) in self.plans.iter().enumerate() {
                let gate = sel(s);
                if !plan.batch_end {
                    leaf_next +=
                        gate.clone() * l(lay.perm_block(plan.leaf_perms - 1) + C_OUTPUT + i);
                }
                ood_next +=
                    gate * l(lay.perm_block(OOD_PERM_BASE + plan.ood_perms - 1) + C_OUTPUT + i);
            }
            tr.assert_eq(n(lay.leaf_state + i), leaf_next);
            tr.assert_eq(n(lay.ood_state + i), ood_next);
        }
        for i in 0..RATE {
            let mut leaf_next = AB::Expr::ZERO;
            let mut ood_next = AB::Expr::ZERO;
            for (s, plan) in self.plans.iter().enumerate() {
                let gate = sel(s);
                if !plan.batch_end && i < plan.leaf_buffer_out {
                    let total = plan.leaf_buffer_in + plan.terms + plan.salts;
                    leaf_next += gate.clone()
                        * leaf_source_expr::<AB>(&l, lay, plan, total - plan.leaf_buffer_out + i);
                }
                if i < plan.ood_buffer_out {
                    let total = plan.ood_buffer_in + plan.terms * D;
                    ood_next += gate
                        * ood_source_expr::<AB>(&l, lay, plan, total - plan.ood_buffer_out + i);
                }
            }
            tr.assert_eq(n(lay.leaf_buffer + i), leaf_next);
            tr.assert_eq(n(lay.ood_buffer + i), ood_next);
        }

        {
            let mut last = builder.when_last_row();
            last.assert_one(sel(lay.pad()));
            if pubs.len() == pv.n_public {
                for i in 0..D {
                    last.assert_eq(l(lay.ro_step(0) + i), pubs[pv.reduced_opening + i].clone());
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PcsQueryBatchInput {
    /// One authenticated opened row per committed matrix.
    pub opened_values: Vec<Vec<Val>>,
    /// One four-element MMCS salt per matrix.
    pub salts: Vec<[Val; SALT_ELEMS]>,
}

#[derive(Clone, Debug)]
pub struct PcsQueryInput {
    pub batches: Vec<PcsQueryBatchInput>,
    /// Extension claims in exact batch/matrix/column order.
    pub p_at_z: Vec<Challenge>,
    pub z: Challenge,
    pub index: usize,
    pub ood_start: [Val; WIDTH],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PcsQueryOutputs {
    pub alpha: Challenge,
    pub x: Val,
    pub ood_end: [Val; WIDTH],
    pub leaf_digests: Vec<[Val; DIGEST_ELEMS]>,
    pub reduced_opening: Challenge,
}

fn duplex_input(mut state: [Val; WIDTH], block: &[Val]) -> [Val; WIDTH] {
    debug_assert!(block.len() <= RATE);
    state[..block.len()].copy_from_slice(block);
    state[block.len()..RATE].fill(Val::ZERO);
    state[RATE] += Val::from_usize(block.len());
    state
}

fn squeeze_input(state: [Val; WIDTH]) -> [Val; WIDTH] {
    state
}

pub fn public_values(
    air: &PcsQueryAir,
    input: &PcsQueryInput,
    output: &PcsQueryOutputs,
) -> Vec<Val> {
    let mut out = Vec::with_capacity(air.public_width());
    out.extend_from_slice(&limbs(output.alpha));
    out.extend_from_slice(&limbs(input.z));
    out.push(output.x);
    out.extend_from_slice(&input.ood_start);
    out.extend_from_slice(&output.ood_end);
    for digest in &output.leaf_digests {
        out.extend_from_slice(digest);
    }
    out.extend((0..air.log_height).map(|k| Val::from_bool((input.index >> k) & 1 == 1)));
    out.extend_from_slice(&limbs(output.reduced_opening));
    debug_assert_eq!(out.len(), air.public_width());
    out
}

pub fn build_trace(
    air: &PcsQueryAir,
    input: &PcsQueryInput,
) -> (RowMajorMatrix<Val>, PcsQueryOutputs) {
    assert_eq!(input.batches.len(), air.batches.len());
    assert!(input.index < (1usize << air.log_height));
    let mut flattened_terms = Vec::<Val>::new();
    let mut flattened_salts = Vec::<[Val; SALT_ELEMS]>::new();
    for (shape, batch) in air.batches.iter().zip(&input.batches) {
        assert_eq!(batch.opened_values.len(), shape.matrix_widths.len());
        assert_eq!(batch.salts.len(), shape.matrix_widths.len());
        for ((row, &width), salt) in batch
            .opened_values
            .iter()
            .zip(&shape.matrix_widths)
            .zip(&batch.salts)
        {
            assert_eq!(row.len(), width);
            flattened_terms.extend_from_slice(row);
            flattened_salts.push(*salt);
        }
    }
    assert_eq!(flattened_terms.len(), air.terms);
    assert_eq!(input.p_at_z.len(), air.terms);

    let lay = Layout::new(air);
    let height = air.trace_height();
    let mut flat = vec![Val::ZERO; height * lay.n_cols];
    let constants = inlined_perm::Constants::pinned();
    let dummy = inlined_perm::generate_perm_cols([Val::ZERO; WIDTH], &constants);

    let omega = Val::two_adic_generator(air.log_height);
    let mut x_tower = vec![Val::ZERO; air.log_height + 1];
    x_tower[0] = Val::GENERATOR;
    for k in 0..air.log_height {
        let factor = omega.exp_u64(1u64 << (air.log_height - 1 - k));
        x_tower[k + 1] = if (input.index >> k) & 1 == 1 {
            x_tower[k] * factor
        } else {
            x_tower[k]
        };
    }
    let x = x_tower[air.log_height];
    let inv = (input.z - Challenge::from(x))
        .try_inverse()
        .expect("opening point must differ from query point");

    let mut leaf_state = [Val::ZERO; WIDTH];
    let mut leaf_buffer = Vec::<Val>::new();
    let mut ood_state = input.ood_start;
    let mut ood_buffer = Vec::<Val>::new();
    let mut leaf_digests = Vec::with_capacity(air.batches.len());
    let mut term_cursor = 0usize;
    let mut matrix_cursor = 0usize;
    let mut row_index = 0usize;

    for (segment, plan) in air.plans.iter().enumerate() {
        for pos in 0..plan.repeat {
            let row = &mut flat[row_index * lay.n_cols..(row_index + 1) * lay.n_cols];
            row[segment] = Val::ONE;
            let last = pos + 1 == plan.repeat;
            row[lay.seg_last] = Val::from_bool(last);
            row[lay.seg_pos] = Val::from_usize(pos);
            let diff = Val::from_usize(pos) - Val::from_usize(plan.repeat - 1);
            row[lay.seg_inv] = if last { Val::ZERO } else { diff.inverse() };
            row[lay.leaf_state..lay.leaf_state + WIDTH].copy_from_slice(&leaf_state);
            row[lay.leaf_buffer..lay.leaf_buffer + leaf_buffer.len()].copy_from_slice(&leaf_buffer);
            row[lay.ood_state..lay.ood_state + WIDTH].copy_from_slice(&ood_state);
            row[lay.ood_buffer..lay.ood_buffer + ood_buffer.len()].copy_from_slice(&ood_buffer);
            row[lay.alpha..lay.alpha + D].copy_from_slice(&[Val::ZERO; D]);
            row[lay.z..lay.z + D].copy_from_slice(&limbs(input.z));
            row[lay.x] = x;
            row[lay.inv..lay.inv + D].copy_from_slice(&limbs(inv));
            row[lay.x_tower..lay.x_tower + x_tower.len()].copy_from_slice(&x_tower);
            for k in 0..N_PERMS {
                let off = lay.perm_block(k);
                row[off..off + PERM_COLS].copy_from_slice(&dummy);
            }

            let terms = &flattened_terms[term_cursor..term_cursor + plan.terms];
            let pz = &input.p_at_z[term_cursor..term_cursor + plan.terms];
            row[lay.terms..lay.terms + plan.terms].copy_from_slice(terms);
            for (j, value) in pz.iter().enumerate() {
                row[lay.p_at_z + j * D..lay.p_at_z + (j + 1) * D].copy_from_slice(&limbs(*value));
            }
            if plan.salts > 0 {
                row[lay.salts..lay.salts + SALT_ELEMS]
                    .copy_from_slice(&flattened_salts[matrix_cursor]);
                matrix_cursor += 1;
            }

            let mut leaf_values = leaf_buffer.clone();
            leaf_values.extend_from_slice(terms);
            if plan.salts > 0 {
                leaf_values.extend_from_slice(&row[lay.salts..lay.salts + plan.salts]);
            }
            let leaf_consumed = if plan.batch_end {
                leaf_values.len()
            } else {
                leaf_values.len() / RATE * RATE
            };
            for (k, chunk) in leaf_values[..leaf_consumed].chunks(RATE).enumerate() {
                let mut perm_input = leaf_state;
                perm_input[..chunk.len()].copy_from_slice(chunk);
                let cols = inlined_perm::generate_perm_cols(perm_input, &constants);
                let off = lay.perm_block(k);
                row[off..off + PERM_COLS].copy_from_slice(&cols);
                leaf_state = inlined_perm::perm_output(&cols);
            }
            leaf_buffer = leaf_values[leaf_consumed..].to_vec();
            if plan.batch_end {
                assert!(leaf_buffer.is_empty());
                leaf_digests.push(core::array::from_fn(|i| leaf_state[i]));
                leaf_state = [Val::ZERO; WIDTH];
            }

            let mut ood_values = ood_buffer.clone();
            for value in pz {
                ood_values.extend_from_slice(value.as_basis_coefficients_slice());
            }
            let stream_end = segment + 1 == air.plans.len() && pos + 1 == plan.repeat;
            let ood_consumed = if stream_end {
                ood_values.len()
            } else {
                ood_values.len() / RATE * RATE
            };
            for (k, chunk) in ood_values[..ood_consumed].chunks(RATE).enumerate() {
                let cols =
                    inlined_perm::generate_perm_cols(duplex_input(ood_state, chunk), &constants);
                let off = lay.perm_block(OOD_PERM_BASE + k);
                row[off..off + PERM_COLS].copy_from_slice(&cols);
                ood_state = inlined_perm::perm_output(&cols);
            }
            ood_buffer = ood_values[ood_consumed..].to_vec();

            term_cursor += plan.terms;
            row_index += 1;
        }
    }

    // The reduction challenge is sampled only after the OOD stream, while the
    // recurrence needs it from row zero. Rebuild the reduction columns now that
    // the exact challenge is known.
    assert!(ood_buffer.is_empty());
    let first_seven: [Val; RATE] = core::array::from_fn(|i| ood_state[RATE - 1 - i]);
    let bca_cols = inlined_perm::generate_perm_cols(squeeze_input(ood_state), &constants);
    let bca_state = inlined_perm::perm_output(&bca_cols);
    let alpha = Challenge::from_basis_coefficients_fn(|i| {
        if i < RATE {
            first_seven[i]
        } else {
            bca_state[RATE - 1]
        }
    });

    let mut alpha_pow = Challenge::ONE;
    let mut ro = Challenge::ZERO;
    term_cursor = 0;
    row_index = 0;
    for (segment, plan) in air.plans.iter().enumerate() {
        for pos in 0..plan.repeat {
            let row = &mut flat[row_index * lay.n_cols..(row_index + 1) * lay.n_cols];
            row[lay.alpha..lay.alpha + D].copy_from_slice(&limbs(alpha));
            row[lay.alpha_pow_step(0)..lay.alpha_pow_step(0) + D]
                .copy_from_slice(&limbs(alpha_pow));
            row[lay.ro_step(0)..lay.ro_step(0) + D].copy_from_slice(&limbs(ro));
            for j in 0..LANES {
                if j < plan.terms {
                    ro += alpha_pow
                        * (input.p_at_z[term_cursor + j]
                            - Challenge::from(flattened_terms[term_cursor + j]))
                        * inv;
                    alpha_pow *= alpha;
                }
                row[lay.alpha_pow_step(j + 1)..lay.alpha_pow_step(j + 1) + D]
                    .copy_from_slice(&limbs(alpha_pow));
                row[lay.ro_step(j + 1)..lay.ro_step(j + 1) + D].copy_from_slice(&limbs(ro));
            }
            if segment + 1 == air.plans.len() && pos + 1 == plan.repeat {
                let off = lay.perm_block(BCA_PERM_INDEX);
                row[off..off + PERM_COLS].copy_from_slice(&bca_cols);
            }
            term_cursor += plan.terms;
            row_index += 1;
        }
    }

    for r in air.active_rows..height {
        let row = &mut flat[r * lay.n_cols..(r + 1) * lay.n_cols];
        row[lay.pad()] = Val::ONE;
        row[lay.alpha..lay.alpha + D].copy_from_slice(&limbs(alpha));
        row[lay.z..lay.z + D].copy_from_slice(&limbs(input.z));
        row[lay.x] = x;
        row[lay.inv..lay.inv + D].copy_from_slice(&limbs(inv));
        row[lay.x_tower..lay.x_tower + x_tower.len()].copy_from_slice(&x_tower);
        for j in 0..=LANES {
            row[lay.alpha_pow_step(j)..lay.alpha_pow_step(j) + D]
                .copy_from_slice(&limbs(alpha_pow));
            row[lay.ro_step(j)..lay.ro_step(j) + D].copy_from_slice(&limbs(ro));
        }
        row[lay.ood_state..lay.ood_state + WIDTH].copy_from_slice(&ood_state);
        for k in 0..N_PERMS {
            let off = lay.perm_block(k);
            row[off..off + PERM_COLS].copy_from_slice(&dummy);
        }
    }

    let outputs = PcsQueryOutputs {
        alpha,
        x,
        ood_end: ood_state,
        leaf_digests,
        reduced_opening: ro,
    };
    (RowMajorMatrix::new(flat, lay.n_cols), outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leaf_hash::leaf_hash_digest;
    use crate::pcs_opening_reduction_air::{reduce_opening_terms, OpeningReductionTerm};
    use p3_air::check_all_constraints;
    use p3_field::BasedVectorSpace;

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(65_537).wrapping_add(i as u32 + 1))
        })
    }

    fn fixture() -> (PcsQueryAir, PcsQueryInput) {
        let air = PcsQueryAir::new(
            vec![
                PcsBatchShape::new(vec![12]),
                PcsBatchShape::new(vec![18]),
                PcsBatchShape::new(vec![12, 12]),
            ],
            6,
        );
        let mut next = 100u32;
        let mut make_batch = |widths: &[usize]| {
            let opened_values = widths
                .iter()
                .map(|&w| {
                    (0..w)
                        .map(|_| {
                            next += 1;
                            Val::from_u32(next)
                        })
                        .collect()
                })
                .collect::<Vec<Vec<Val>>>();
            let salts = widths
                .iter()
                .map(|_| {
                    core::array::from_fn(|_| {
                        next += 1;
                        Val::from_u32(next)
                    })
                })
                .collect();
            PcsQueryBatchInput {
                opened_values,
                salts,
            }
        };
        let batches = vec![make_batch(&[12]), make_batch(&[18]), make_batch(&[12, 12])];
        let p_at_z = (0..air.terms()).map(|i| ext(1000 + i as u32)).collect();
        let ood_start = core::array::from_fn(|i| Val::from_u32(9000 + i as u32));
        (
            air,
            PcsQueryInput {
                batches,
                p_at_z,
                z: ext(77),
                index: 37,
                ood_start,
            },
        )
    }

    #[test]
    fn query_link_binds_leaf_hash_ood_and_reduction() {
        let (air, input) = fixture();
        let (trace, output) = build_trace(&air, &input);
        let pubs = public_values(&air, &input, &output);
        let report = check_all_constraints(&air, &trace, &pubs, None);
        assert!(
            report.is_ok(),
            "honest query link rejected: {} failures",
            report.failures.len()
        );

        let expected_leaves: Vec<[Val; DIGEST_ELEMS]> = input
            .batches
            .iter()
            .map(|batch| {
                let mut salted = Vec::new();
                for (row, salt) in batch.opened_values.iter().zip(&batch.salts) {
                    salted.extend_from_slice(row);
                    salted.extend_from_slice(salt);
                }
                leaf_hash_digest(&salted)
            })
            .collect();
        assert_eq!(output.leaf_digests, expected_leaves);

        let flattened: Vec<Val> = input
            .batches
            .iter()
            .flat_map(|b| b.opened_values.iter().flatten().copied())
            .collect();
        let terms: Vec<OpeningReductionTerm> = flattened
            .into_iter()
            .zip(input.p_at_z.iter().copied())
            .map(|(p_at_x, p_at_z)| OpeningReductionTerm {
                z: input.z,
                x: output.x,
                p_at_x,
                p_at_z,
            })
            .collect();
        assert_eq!(
            output.reduced_opening,
            reduce_opening_terms(output.alpha, &terms)
        );
    }

    #[test]
    fn query_link_rejects_each_cross_binding_mutation() {
        let (air, input) = fixture();
        let (trace, output) = build_trace(&air, &input);
        let pubs = public_values(&air, &input, &output);
        let lay = Layout::new(&air);
        for (label, row_index, column) in [
            ("p_at_x", 0usize, lay.terms + 2),
            ("p_at_z", 1, lay.p_at_z + 3),
            ("leaf permutation", 0, lay.perm_block(0) + C_OUTPUT + 1),
            (
                "ood permutation",
                0,
                lay.perm_block(OOD_PERM_BASE) + C_OUTPUT + 2,
            ),
            ("reduction", 2, lay.ro_step(3) + 4),
        ] {
            let mut bad = trace.values.clone();
            bad[row_index * lay.n_cols + column] += Val::ONE;
            let report =
                check_all_constraints(&air, &RowMajorMatrix::new(bad, lay.n_cols), &pubs, None);
            assert!(!report.is_ok(), "tampered {label} accepted");
        }

        for (label, offset) in [
            ("leaf digest", PublicLayout::new(&air).leaf_digests),
            ("OOD end", PublicLayout::new(&air).end),
            ("reduced opening", PublicLayout::new(&air).reduced_opening),
            ("query index", PublicLayout::new(&air).index_bits),
        ] {
            let mut bad = pubs.clone();
            bad[offset] += Val::ONE;
            assert!(
                !check_all_constraints(&air, &trace, &bad, None).is_ok(),
                "tampered public {label} accepted"
            );
        }
    }

    #[test]
    fn production_shape_is_source_pinned_and_constant_height() {
        let air = PcsQueryAir::production(
            descender_account_stark::SINGLE_PROOF_TRACE_ROWS.ilog2() as usize
                + 1
                + descender_account_stark::PROD_LOG_BLOWUP,
        );
        assert_eq!(air.terms(), 23_254);
        assert_eq!(air.active_rows(), 3_325);
        assert_eq!(air.trace_height(), 4_096);
        assert_eq!(air.width(), 2_265);
        assert_eq!(air.public_width(), 102);
        assert_eq!(air.base_witness_bytes(), 37_109_760);
    }

    #[test]
    fn query_link_proves_at_production_q128_knobs() {
        use p3_uni_stark::{prove, verify};

        let (air, input) = fixture();
        let (trace, output) = build_trace(&air, &input);
        let pubs = public_values(&air, &input, &output);
        let config = crate::settlement::prod_config();
        let proof = prove(&config, &air, trace, &pubs);
        verify(&config, &air, &proof, &pubs)
            .expect("PCS query link must prove and verify at q128 knobs");
    }
}
