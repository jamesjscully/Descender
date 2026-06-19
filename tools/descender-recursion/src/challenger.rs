//! Gadget 1 — in-AIR `DuplexChallenger` (Fiat–Shamir duplex sponge), reference layer.
//!
//! This is the ROW-BASED reference that the verifier-AIR's challenger columns must
//! enforce constant-for-constant. It mirrors `p3_challenger::DuplexChallenger<Val,
//! Perm, 16, 8>` (the leaf's `descender_account_stark::Challenger`) EXACTLY — the
//! semantics pinned in `PHASE1_GADGET1_CHALLENGER_SPEC.md` §1: overwrite-not-add on
//! absorb, the `+= num_absorbed` length tag into capacity slot `RATE`, LIFO
//! `output_buffer.pop()`, and the `sample_bits` low-bits draw.
//!
//! The Poseidon2 permutation is used here as a BLACK BOX (the real `Perm`): gadget 2
//! (`p3-poseidon2-air`) is what constrains the permutation in-circuit; gadget 1 only
//! fixes the sponge wiring + the per-row op-program. The reference emits one [`Row`]
//! per atomic challenger event (observe / field-draw / permute), which is the trace
//! the AIR commits, and exposes the sampled values so a golden test can assert
//! element-for-element equality against the native challenger (spec §7.1).

use descender_account_stark::{Perm, Val, WIDTH};
use p3_field::{PrimeCharacteristicRing, PrimeField32};
use p3_koala_bear::default_koalabear_poseidon2_16;
use p3_symmetric::Permutation;

/// Sponge rate (must match `Challenger = DuplexChallenger<Val, Perm, 16, 8>`).
pub const RATE: usize = 8;
/// Sponge capacity = WIDTH − RATE = 8 (the c=8 floor accepted at 2⁻¹²⁴; see
/// `field-bound-soundness-ceiling` decision — width-16 Poseidon2 is kept).
pub const CAPACITY: usize = WIDTH - RATE;

/// Which atomic challenger transition a [`Row`] performs (spec §3.1 selectors;
/// mutually exclusive).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    /// Absorb one field element into the input buffer.
    Observe,
    /// Pop one field element from the output buffer (one base-field draw; a component
    /// of `sample` / `sample_bits`).
    SampleDraw,
    /// Perform a duplexing permutation (absorb the buffered rate, or squeeze).
    Permute,
}

/// One challenger-trace row — the AIR substrate. State columns are snapshotted at the
/// START of the row (spec §3.1).
#[derive(Clone, Debug)]
pub struct Row {
    pub op: Op,
    /// Full sponge state at the start of the row.
    pub sponge: [Val; WIDTH],
    /// Input-buffer contents (valid lanes `< in_len`) at start of row.
    pub in_buf: [Val; RATE],
    pub in_len: usize,
    /// Output-buffer contents (valid lanes `< out_len`) at start of row.
    pub out_buf: [Val; RATE],
    pub out_len: usize,
    /// `Observe`: the absorbed element. Else `ZERO`.
    pub observed_val: Val,
    /// `SampleDraw`: the popped element. Else `ZERO`.
    pub draw_out: Val,
    /// `Permute`: `n = num_absorbed` (the length tag; `0` on a squeeze-permute).
    pub absorb_len_tag: usize,
    /// `Permute`: `1` iff this permute had `n == 0` (squeeze, no rate overwrite / tag).
    pub is_squeeze_permute: bool,
}

/// Row-based reference duplex challenger. Mirrors `DuplexChallenger` field-for-field
/// and records a [`Row`] per atomic event.
pub struct RowChallenger {
    perm: Perm,
    sponge: [Val; WIDTH],
    in_buf: Vec<Val>,
    out_buf: Vec<Val>,
    rows: Vec<Row>,
}

impl Default for RowChallenger {
    fn default() -> Self {
        Self::new()
    }
}

impl RowChallenger {
    /// `new()` ⇒ all-zero sponge, empty buffers (spec §1, `DuplexChallenger::new`).
    pub fn new() -> Self {
        Self {
            perm: default_koalabear_poseidon2_16(),
            sponge: [Val::ZERO; WIDTH],
            in_buf: Vec::with_capacity(RATE),
            out_buf: Vec::with_capacity(RATE),
            rows: Vec::new(),
        }
    }

    fn snapshot_buf(buf: &[Val]) -> ([Val; RATE], usize) {
        let mut a = [Val::ZERO; RATE];
        a[..buf.len()].copy_from_slice(buf);
        (a, buf.len())
    }

    fn push_row(&mut self, op: Op, observed_val: Val, draw_out: Val, absorb_len_tag: usize) {
        let (in_buf, in_len) = Self::snapshot_buf(&self.in_buf);
        let (out_buf, out_len) = Self::snapshot_buf(&self.out_buf);
        self.rows.push(Row {
            op,
            sponge: self.sponge,
            in_buf,
            in_len,
            out_buf,
            out_len,
            observed_val,
            draw_out,
            absorb_len_tag,
            is_squeeze_permute: op == Op::Permute && absorb_len_tag == 0,
        });
    }

    /// Spec §1.1 `duplexing()` — emits one `Permute` row (snapshot BEFORE the permute).
    fn duplexing(&mut self) {
        let n = self.in_buf.len();
        debug_assert!(n <= RATE);
        self.push_row(Op::Permute, Val::ZERO, Val::ZERO, n);
        // overwrite rate lanes with the drained input buffer
        for (i, v) in self.in_buf.drain(..).enumerate() {
            self.sponge[i] = v;
        }
        if n > 0 {
            // absorb: zero the unfilled rate lanes, add the length tag into slot RATE
            self.sponge[n..RATE].fill(Val::ZERO);
            self.sponge[RATE] += Val::from_u8(n as u8);
        }
        self.perm.permute_mut(&mut self.sponge);
        self.out_buf.clear();
        self.out_buf.extend_from_slice(&self.sponge[..RATE]);
    }

    /// Spec §1.2 `observe(value)`. The row snapshots state at the START of the op (before
    /// the clear/push), so the transition constraints relate this snapshot to the next.
    pub fn observe(&mut self, value: Val) {
        self.push_row(Op::Observe, value, Val::ZERO, 0);
        self.out_buf.clear();
        self.in_buf.push(value);
        if self.in_buf.len() == RATE {
            self.duplexing();
        }
    }

    /// One base-field draw (spec §1.3 inner closure of `sample`). The row snapshots the
    /// PRE-pop state (out_len = m, draw_out = out_buf[m-1]); the pop's effect shows in the
    /// next row (out_len = m-1) — matching `constraints::transition_ok`'s §4.3 rule.
    pub fn sample(&mut self) -> Val {
        if !self.in_buf.is_empty() || self.out_buf.is_empty() {
            self.duplexing();
        }
        let m = self.out_buf.len();
        debug_assert!(m >= 1, "output buffer nonempty after duplexing");
        let v = self.out_buf[m - 1];
        self.push_row(Op::SampleDraw, Val::ZERO, v, 0);
        self.out_buf.pop();
        v
    }

    /// Spec §1.4 `sample_bits(bits)` — low `bits` bits of the canonical draw.
    pub fn sample_bits(&mut self, bits: usize) -> usize {
        debug_assert!(bits < usize::BITS as usize);
        let r = self.sample();
        (r.as_canonical_u32() as usize) & ((1usize << bits) - 1)
    }

    /// Spec §1.5 `check_witness(bits, w)` = `observe(w); sample_bits(bits) == 0`.
    pub fn check_witness(&mut self, bits: usize, w: Val) -> bool {
        self.observe(w);
        self.sample_bits(bits) == 0
    }

    /// The recorded trace rows (the AIR substrate).
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }
}

/// # Lifting [`constraints`] into a Plonky3 `Air<AB>` (next step — NOT mechanical)
///
/// The [`constraints`] checker below is the validated reference logic, but it indexes by
/// witness values (`in_buf[..in_len]`, `out_buf[out_len-1]`). An AIR constraint must be a
/// FIXED polynomial — it cannot index a row by a witness-valued length. So the `Air<AB>`
/// lift requires re-expressing each buffer op with POSITION MASKING, not value-indexing:
///
/// * Carry a one-hot encoding of `in_len` (`in_sel[0..=RATE]`, exactly one set) and of
///   `out_len`. Booleanity + `Σ = 1` per row.
/// * §4.1 observe append "`in_buf'[in_len] = observed_val`" becomes, for every lane `j`:
///   `in_sel[j] * (in_buf'[j] − observed_val) = 0` and `(1 − Σ_{i≤j} in_sel[i]) * (...)`
///   carry/copy masks — i.e. assert the relation at EVERY lane gated by the one-hot, so no
///   variable index appears.
/// * §4.3 LIFO pop "`draw_out = out_buf[out_len−1]`" becomes
///   `Σ_j out_sel[j] * (out_buf[j−1] − draw_out) = 0` (a fixed linear combination).
/// * §4.2 permute refill `out_buf'[j] = sponge'[j]` for `j<RATE` is already lane-fixed.
///   The permuted state `sponge' = permute(absorb(sponge, in_buf, n))` is the GADGET-2 bus.
///
/// Column layout (flat, ~42 cols): `is_observe,is_sample_draw,is_permute,is_pad`,
/// `sponge[16]`, `in_buf[RATE]`, `in_sel[RATE+1]`, `out_buf[RATE]`, `out_sel[RATE+1]`,
/// `observed_val`, `draw_out`, `absorb_len_tag`, `is_squeeze_permute`. Serialize the [`Row`]
/// trace (padded to a power of two with `is_pad` rows that freeze state) into a
/// `RowMajorMatrix<Val>`. TEST via `p3_air::check_constraints` /
/// `check_all_constraints(&air, &trace, &[]).is_ok()` for the honest trace, and
/// `has_failures()` for each tamper — the same forgery-rejection discipline as the
/// [`constraints`] tests below, but exercised through the real AIR builder. Degree stays ≤ 2
/// (one-hot masks are linear; the single product `in_sel[j]*(...)` is degree 2).
///
/// Gadget-1 transition + boundary CONSTRAINTS (spec §4), as concrete checkers over the
/// row trace. This is the constraint logic the eventual `Air<AB>` impl mirrors one-for-one
/// (the same relationship the row reference has to the native challenger). Everything here
/// is degree ≤ 2 in the row registers and is the SPONGE-WIRING half of gadget 1.
///
/// DEFERRED TO GADGET 2: the nonlinear permutation relation `next.sponge ==
/// permute(absorb(local.sponge, local.in_buf, n))` on a `Permute` row. Gadget 1 constrains
/// everything AROUND the permutation (buffer drain, length-tag presence via `absorb_len_tag`,
/// output refill from the post-permute rate) but treats the permuted state as supplied by
/// the Poseidon2 sub-AIR. Marked at each use with `// GADGET-2`.
pub mod constraints {
    use super::{Op, Row, Val, RATE, WIDTH};
    use p3_field::PrimeCharacteristicRing;

    /// Boundary (spec §4.6): the first active row starts from `new()` — zero sponge,
    /// empty buffers.
    pub fn boundary_ok(first: &Row) -> bool {
        first.sponge == [Val::ZERO; WIDTH] && first.in_len == 0 && first.out_len == 0
    }

    /// One transition constraint check over consecutive rows `(local, next)`. `next`'s
    /// snapshot is the state AFTER `local`'s op completes, so each rule relates `local`'s
    /// recorded op to `next`'s starting state. Returns `true` iff every wiring constraint
    /// for `local.op` holds.
    pub fn transition_ok(local: &Row, next: &Row) -> bool {
        match local.op {
            // §4.1 observe: append observed_val at in_buf[in_len]; clear output; sponge
            // unchanged; in_len += 1.
            Op::Observe => {
                let k = local.in_len;
                k < RATE
                    && next.in_len == k + 1
                    && next.in_buf[..k] == local.in_buf[..k]
                    && next.in_buf[k] == local.observed_val
                    && next.out_len == 0
                    && next.sponge == local.sponge
            }
            // §4.3 sample-draw: pop from the BACK (LIFO); out_len -= 1; sponge + input
            // untouched. (Spec §7.4 — front-pop would reorder the challenge limbs.)
            Op::SampleDraw => {
                let m = local.out_len;
                m >= 1
                    && local.draw_out == local.out_buf[m - 1]
                    && next.out_len == m - 1
                    && next.out_buf[..m - 1] == local.out_buf[..m - 1]
                    && next.sponge == local.sponge
                    && next.in_len == local.in_len
                    && next.in_buf[..local.in_len] == local.in_buf[..local.in_len]
            }
            // §4.2 permute (duplexing): buffer drains (in_len -> 0); output refills from the
            // post-permute rate (out_buf == sponge[..RATE], out_len == RATE); squeeze flag
            // agrees with n==0. The permuted-state correctness itself is GADGET-2.
            Op::Permute => {
                let n = local.absorb_len_tag;
                n <= RATE
                    && (local.is_squeeze_permute == (n == 0))
                    && next.in_len == 0
                    && next.out_len == RATE
                    && next.out_buf[..RATE] == next.sponge[..RATE] // refill wiring (GADGET-2 supplies sponge)
            }
        }
    }

    /// Whole-trace check: boundary on row 0 + every consecutive transition. (The real AIR
    /// additionally pads to a power of two with `is_pad` rows that freeze state; the row
    /// reference emits no pad rows, so we check the active prefix.)
    pub fn trace_ok(rows: &[Row]) -> bool {
        if rows.is_empty() {
            return false;
        }
        boundary_ok(&rows[0]) && rows.windows(2).all(|w| transition_ok(&w[0], &w[1]))
    }
}

/// Flat AIR column layout for gadget 1 + the trace serializer (the data half of the
/// `Air<AB>` lift). Columns carry the one-hot `in_len`/`out_len` encodings the
/// position-masked constraints (see the module doc "Lifting … into a Plonky3 Air<AB>")
/// will read. This module fixes + validates the ENCODING; the polynomial `eval` over these
/// columns is the next step and will reproduce [`constraints`] using these one-hots.
pub mod air {
    use super::{Op, Row, Val, RATE, WIDTH};
    use p3_field::{PrimeCharacteristicRing, PrimeField32};

    // Column offsets (flat row).
    pub const C_IS_OBSERVE: usize = 0;
    pub const C_IS_SAMPLE_DRAW: usize = 1;
    pub const C_IS_PERMUTE: usize = 2;
    pub const C_IS_PAD: usize = 3;
    pub const C_SPONGE: usize = 4; // ..C_SPONGE+WIDTH
    pub const C_IN_BUF: usize = C_SPONGE + WIDTH; // ..+RATE
    pub const C_IN_SEL: usize = C_IN_BUF + RATE; // one-hot of in_len in 0..=RATE  (RATE+1 cols)
    pub const C_OUT_BUF: usize = C_IN_SEL + (RATE + 1); // ..+RATE
    pub const C_OUT_SEL: usize = C_OUT_BUF + RATE; // one-hot of out_len in 0..=RATE
    pub const C_OBSERVED_VAL: usize = C_OUT_SEL + (RATE + 1);
    pub const C_DRAW_OUT: usize = C_OBSERVED_VAL + 1;
    pub const C_ABSORB_LEN_TAG: usize = C_DRAW_OUT + 1;
    pub const C_IS_SQUEEZE: usize = C_ABSORB_LEN_TAG + 1;
    /// Total AIR width for gadget 1.
    pub const N_COLS: usize = C_IS_SQUEEZE + 1;

    /// Serialize one [`Row`] into its flat column vector, including the one-hot
    /// `in_len`/`out_len` selectors (the position-masking substrate).
    pub fn pack_row(row: &Row) -> [Val; N_COLS] {
        let mut c = [Val::ZERO; N_COLS];
        match row.op {
            Op::Observe => c[C_IS_OBSERVE] = Val::ONE,
            Op::SampleDraw => c[C_IS_SAMPLE_DRAW] = Val::ONE,
            Op::Permute => c[C_IS_PERMUTE] = Val::ONE,
        }
        c[C_SPONGE..C_SPONGE + WIDTH].copy_from_slice(&row.sponge);
        c[C_IN_BUF..C_IN_BUF + RATE].copy_from_slice(&row.in_buf);
        c[C_IN_SEL + row.in_len] = Val::ONE; // one-hot in_len
        c[C_OUT_BUF..C_OUT_BUF + RATE].copy_from_slice(&row.out_buf);
        c[C_OUT_SEL + row.out_len] = Val::ONE; // one-hot out_len
        c[C_OBSERVED_VAL] = row.observed_val;
        c[C_DRAW_OUT] = row.draw_out;
        c[C_ABSORB_LEN_TAG] = Val::from_u8(row.absorb_len_tag as u8);
        c[C_IS_SQUEEZE] = if row.is_squeeze_permute { Val::ONE } else { Val::ZERO };
        c
    }

    /// A padding row (spec §4: `is_pad` tail rows freeze state). Used to pad the trace to a
    /// power of two for FRI.
    pub fn pad_row() -> [Val; N_COLS] {
        let mut c = [Val::ZERO; N_COLS];
        c[C_IS_PAD] = Val::ONE;
        c[C_IN_SEL] = Val::ONE; // in_len = 0
        c[C_OUT_SEL] = Val::ONE; // out_len = 0
        c
    }

    /// Decode the one-hot length selector at `base` back to a length in `0..=RATE`
    /// (returns `None` if not a valid one-hot — an encoding-integrity check).
    fn decode_onehot(cols: &[Val], base: usize) -> Option<usize> {
        let mut found = None;
        for k in 0..=RATE {
            if cols[base + k] == Val::ONE {
                if found.is_some() {
                    return None; // not one-hot
                }
                found = Some(k);
            } else if cols[base + k] != Val::ZERO {
                return None; // non-boolean
            }
        }
        found
    }

    /// Decode a packed active (non-pad) row back to a [`Row`], validating the one-hot
    /// selectors. Returns `None` on an ill-formed encoding.
    pub fn unpack_row(cols: &[Val]) -> Option<Row> {
        let op = match (
            cols[C_IS_OBSERVE],
            cols[C_IS_SAMPLE_DRAW],
            cols[C_IS_PERMUTE],
        ) {
            (o, s, p) if o == Val::ONE && s == Val::ZERO && p == Val::ZERO => Op::Observe,
            (o, s, p) if o == Val::ZERO && s == Val::ONE && p == Val::ZERO => Op::SampleDraw,
            (o, s, p) if o == Val::ZERO && s == Val::ZERO && p == Val::ONE => Op::Permute,
            _ => return None,
        };
        let in_len = decode_onehot(cols, C_IN_SEL)?;
        let out_len = decode_onehot(cols, C_OUT_SEL)?;
        let mut sponge = [Val::ZERO; WIDTH];
        sponge.copy_from_slice(&cols[C_SPONGE..C_SPONGE + WIDTH]);
        let mut in_buf = [Val::ZERO; RATE];
        in_buf.copy_from_slice(&cols[C_IN_BUF..C_IN_BUF + RATE]);
        let mut out_buf = [Val::ZERO; RATE];
        out_buf.copy_from_slice(&cols[C_OUT_BUF..C_OUT_BUF + RATE]);
        Some(Row {
            op,
            sponge,
            in_buf,
            in_len,
            out_buf,
            out_len,
            observed_val: cols[C_OBSERVED_VAL],
            draw_out: cols[C_DRAW_OUT],
            absorb_len_tag: cols[C_ABSORB_LEN_TAG].as_canonical_u32() as usize,
            is_squeeze_permute: cols[C_IS_SQUEEZE] == Val::ONE,
        })
    }

    /// Serialize a full row trace to a flat matrix (row-major), padded to a power of two
    /// with `is_pad` rows. Returns `(flat, n_cols)`.
    pub fn trace_to_matrix(rows: &[Row]) -> (Vec<Val>, usize) {
        let height = rows.len().next_power_of_two().max(2);
        let mut flat = Vec::with_capacity(height * N_COLS);
        for row in rows {
            flat.extend_from_slice(&pack_row(row));
        }
        for _ in rows.len()..height {
            flat.extend_from_slice(&pad_row());
        }
        (flat, N_COLS)
    }

    use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};

    /// Gadget-1 verifier-AIR (sponge-wiring half). Encodes [`super::constraints`] as
    /// position-masked polynomial constraints over [`pack_row`]'s columns (degree ≤ 2). The
    /// Poseidon2 permutation relation on a `Permute` row (`next.sponge ==
    /// permute(absorb(...))`) is DEFERRED to gadget 2; everything around it is constrained.
    pub struct ChallengerAir;

    impl<F: Sync> BaseAir<F> for ChallengerAir {
        fn width(&self) -> usize {
            N_COLS
        }
    }

    impl<AB: AirBuilder> Air<AB> for ChallengerAir {
        fn eval(&self, builder: &mut AB) {
            let main = builder.main();
            let lo: Vec<AB::Var> = main.current_slice().to_vec();
            let nx: Vec<AB::Var> = main.next_slice().to_vec();
            let is_tr = builder.is_transition();
            let one = AB::Expr::ONE;

            let l = |i: usize| -> AB::Expr { lo[i].into() };
            let n = |i: usize| -> AB::Expr { nx[i].into() };

            // ---- per-row: op-selector booleanity + exactly-one (incl. is_pad) ----
            let sels = [C_IS_OBSERVE, C_IS_SAMPLE_DRAW, C_IS_PERMUTE, C_IS_PAD];
            let mut sel_sum = AB::Expr::ZERO;
            for &s in &sels {
                builder.assert_zero(l(s) * (l(s) - one.clone())); // boolean
                sel_sum += l(s);
            }
            builder.assert_zero(sel_sum - one.clone()); // exactly one op

            // ---- per-row: in_sel / out_sel one-hot (the in_len/out_len encoding) ----
            let mut in_sum = AB::Expr::ZERO;
            let mut out_sum = AB::Expr::ZERO;
            for k in 0..=RATE {
                builder.assert_zero(l(C_IN_SEL + k) * (l(C_IN_SEL + k) - one.clone()));
                builder.assert_zero(l(C_OUT_SEL + k) * (l(C_OUT_SEL + k) - one.clone()));
                in_sum += l(C_IN_SEL + k);
                out_sum += l(C_OUT_SEL + k);
            }
            builder.assert_zero(in_sum - one.clone());
            builder.assert_zero(out_sum - one.clone());

            // ---- boundary (first row): new() zero sponge + empty buffers ----
            {
                let mut fb = builder.when_first_row();
                for j in 0..WIDTH {
                    fb.assert_zero(l(C_SPONGE + j));
                }
                fb.assert_zero(l(C_IN_SEL) - one.clone()); // in_len == 0
                fb.assert_zero(l(C_OUT_SEL) - one.clone()); // out_len == 0
            }

            // strict "less-than" masks from the one-hot: lt_in[j] = 1 iff in_len > j.
            let lt_in = |j: usize| -> AB::Expr {
                let mut acc = AB::Expr::ZERO;
                for i in (j + 1)..=RATE {
                    acc += l(C_IN_SEL + i);
                }
                acc
            };

            // ---- §4.1 OBSERVE transition ----
            let g_obs = is_tr.clone() * l(C_IS_OBSERVE);
            {
                let mut b = builder.when(g_obs);
                b.assert_zero(n(C_OUT_SEL) - one.clone()); // output cleared (out_len 0)
                for j in 0..WIDTH {
                    b.assert_zero(n(C_SPONGE + j) - l(C_SPONGE + j)); // sponge unchanged
                }
                // append observed_val at index in_len: next.in_buf[j] = lt*cur + onehot*observed
                for j in 0..RATE {
                    b.assert_zero(
                        n(C_IN_BUF + j) - lt_in(j) * l(C_IN_BUF + j) - l(C_IN_SEL + j) * l(C_OBSERVED_VAL),
                    );
                }
                // in_len += 1: next.in_sel shifts up by one
                b.assert_zero(n(C_IN_SEL)); // next in_len != 0
                for k in 1..=RATE {
                    b.assert_zero(n(C_IN_SEL + k) - l(C_IN_SEL + (k - 1)));
                }
            }

            // ---- §4.3 SAMPLE-DRAW transition (LIFO pop) ----
            let g_draw = is_tr.clone() * l(C_IS_SAMPLE_DRAW);
            {
                let mut b = builder.when(g_draw);
                // draw_out == out_buf[out_len-1] (back), via one-hot: Σ out_sel[k]*out_buf[k-1]
                let mut popped = AB::Expr::ZERO;
                for k in 1..=RATE {
                    popped += l(C_OUT_SEL + k) * l(C_OUT_BUF + (k - 1));
                }
                b.assert_zero(l(C_DRAW_OUT) - popped);
                // out_len -= 1: next.out_sel shifts down by one
                for k in 0..RATE {
                    b.assert_zero(n(C_OUT_SEL + k) - l(C_OUT_SEL + (k + 1)));
                }
                b.assert_zero(n(C_OUT_SEL + RATE)); // top can't stay set
                // kept prefix unchanged: kept[j]=1 iff out_len > j+1
                for j in 0..RATE {
                    let mut kept = AB::Expr::ZERO;
                    for i in (j + 2)..=RATE {
                        kept += l(C_OUT_SEL + i);
                    }
                    b.assert_zero(kept * (n(C_OUT_BUF + j) - l(C_OUT_BUF + j)));
                }
                // sponge + input untouched by a draw
                for j in 0..WIDTH {
                    b.assert_zero(n(C_SPONGE + j) - l(C_SPONGE + j));
                }
                for k in 0..=RATE {
                    b.assert_zero(n(C_IN_SEL + k) - l(C_IN_SEL + k));
                }
                for j in 0..RATE {
                    b.assert_zero(n(C_IN_BUF + j) - l(C_IN_BUF + j));
                }
            }

            // ---- §4.2 PERMUTE transition (perm relation itself deferred to gadget 2) ----
            let g_perm = is_tr.clone() * l(C_IS_PERMUTE);
            {
                let mut b = builder.when(g_perm);
                b.assert_zero(n(C_IN_SEL) - one.clone()); // input drained (in_len 0)
                b.assert_zero(n(C_OUT_SEL + RATE) - one.clone()); // out refilled (out_len RATE)
                for j in 0..RATE {
                    b.assert_zero(n(C_OUT_BUF + j) - n(C_SPONGE + j)); // refill from post-perm rate
                }
                // squeeze flag == (in_len == 0): is_squeeze must equal in_sel[0]
                b.assert_zero(l(C_IS_SQUEEZE) - l(C_IN_SEL));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_challenger::{CanObserve, CanSample, CanSampleBits};
    use descender_account_stark::Challenger;

    fn native() -> Challenger {
        Challenger::new(default_koalabear_poseidon2_16())
    }

    fn v(x: u32) -> Val {
        Val::from_u32(x)
    }

    /// A representative op-stream resembling the leaf transcript shape (spec §2):
    /// observe a commitment cap + public IO, sample α/ζ, a few β's with grind, then
    /// query indices.
    fn drive_native(c: &mut Challenger) -> (Vec<Val>, Vec<usize>) {
        let mut samples = Vec::new();
        let mut bits = Vec::new();
        for i in 0..8 {
            c.observe(v(0x1000 + i)); // a commitment "cap" (8 elems)
        }
        for i in 0..19 {
            c.observe(v(0xAB00 + i)); // public IO (odd count -> non-aligned buffer)
        }
        for _ in 0..4 {
            samples.push(c.sample()); // alpha (4 base draws)
        }
        for i in 0..8 {
            c.observe(v(0x2000 + i)); // quotient cap
        }
        for _ in 0..4 {
            samples.push(c.sample()); // zeta
        }
        for round in 0..3 {
            for i in 0..8 {
                c.observe(v(0x3000 + round * 16 + i)); // commit-phase cap
            }
            samples.push(c.sample()); // beta_round (one base draw, to keep it simple)
        }
        // query grind: check_witness(16, w) -> observe(w); sample_bits(16)
        c.observe(v(0x9999));
        bits.push(c.sample_bits(16));
        for _ in 0..PROD_NUM_QUERIES_SMALL {
            bits.push(c.sample_bits(20)); // query indices
        }
        (samples, bits)
    }

    fn drive_row(c: &mut RowChallenger) -> (Vec<Val>, Vec<usize>) {
        let mut samples = Vec::new();
        let mut bits = Vec::new();
        for i in 0..8 {
            c.observe(v(0x1000 + i));
        }
        for i in 0..19 {
            c.observe(v(0xAB00 + i));
        }
        for _ in 0..4 {
            samples.push(c.sample());
        }
        for i in 0..8 {
            c.observe(v(0x2000 + i));
        }
        for _ in 0..4 {
            samples.push(c.sample());
        }
        for round in 0..3 {
            for i in 0..8 {
                c.observe(v(0x3000 + round * 16 + i));
            }
            samples.push(c.sample());
        }
        c.observe(v(0x9999));
        bits.push(c.sample_bits(16));
        for _ in 0..PROD_NUM_QUERIES_SMALL {
            bits.push(c.sample_bits(20));
        }
        (samples, bits)
    }

    const PROD_NUM_QUERIES_SMALL: usize = 8;

    /// Spec §7.1 GOLDEN EQUIVALENCE (positive): the row-based reference must produce
    /// the SAME sampled challenges + query bits as the native `DuplexChallenger`,
    /// element-for-element, on the same op-stream. This is the constant-for-constant
    /// anchor the AIR will inherit.
    #[test]
    fn row_challenger_matches_native_duplex_challenger() {
        let mut n = native();
        let (ns, nb): (Vec<Val>, Vec<usize>) = drive_native(&mut n);

        let mut r = RowChallenger::new();
        let (rs, rb) = drive_row(&mut r);

        assert_eq!(rs, ns, "sampled challenge values diverge from native challenger");
        assert_eq!(rb, nb, "sampled query bits diverge from native challenger");
        assert!(!r.rows().is_empty());
    }

    /// Spec §7.5 (forged statement ⇒ different challenge): flipping ONE observed public
    /// element must change a downstream sampled challenge. Tested by attempted recovery
    /// of identical challenges (divergence), NEVER by substring scan.
    #[test]
    fn wrong_public_io_changes_sampled_challenge() {
        let mut a = RowChallenger::new();
        a.observe(v(0xAB00));
        a.observe(v(0xAB01));
        let sa = a.sample();

        let mut b = RowChallenger::new();
        b.observe(v(0xAB00));
        b.observe(v(0xAB99)); // one element flipped
        let sb = b.sample();

        assert_ne!(sa, sb, "a flipped public element did not change the FS challenge");
    }

    /// Spec §7.3 length-tag binding: `[5,7]` vs `[5,7,0,0]` (distinct absorb lengths n)
    /// MUST yield different challenges — the `+= n` capacity tag is what makes the
    /// absorb prefix-free. A gadget omitting the tag would make these collide.
    #[test]
    fn absorb_length_tag_is_binding() {
        let mut a = RowChallenger::new();
        a.observe(v(5));
        a.observe(v(7));
        let sa = a.sample(); // n=2 absorb (forced by the sample's duplexing)

        let mut b = RowChallenger::new();
        b.observe(v(5));
        b.observe(v(7));
        b.observe(v(0));
        b.observe(v(0));
        let sb = b.sample(); // n=4 absorb

        assert_ne!(sa, sb, "length-tag not binding: [5,7] and [5,7,0,0] collided");
    }

    /// Spec §4 CONSTRAINTS — honest trace satisfies every transition + boundary.
    #[test]
    fn honest_trace_satisfies_gadget1_constraints() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        assert!(constraints::trace_ok(r.rows()), "honest challenger trace violated gadget-1 constraints");
    }

    /// Forged trace rejected: flipping a popped draw value breaks the LIFO-pop relation
    /// (spec §4.3). Tested by the constraint REJECTING the tamper, not by substring scan.
    #[test]
    fn tampered_draw_value_is_rejected() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let mut rows = r.rows().to_vec();
        let i = rows.iter().position(|w| w.op == Op::SampleDraw).expect("a draw row exists");
        rows[i].draw_out += Val::ONE; // claim a different popped element
        assert!(!constraints::transition_ok(&rows[i], &rows[i + 1]));
        assert!(!constraints::trace_ok(&rows));
    }

    /// Forged trace rejected: front-pop instead of back-pop (spec §7.4 — reorders the
    /// challenge limbs). Set draw_out to the FRONT of the output buffer.
    #[test]
    fn front_pop_instead_of_lifo_is_rejected() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let rows = r.rows().to_vec();
        let i = rows
            .iter()
            .position(|w| w.op == Op::SampleDraw && w.out_len >= 2)
            .expect("a draw row with >=2 buffered exists");
        let mut row = rows[i].clone();
        row.draw_out = row.out_buf[0]; // front element (LIFO requires out_buf[out_len-1])
        assert!(!constraints::transition_ok(&row, &rows[i + 1]));
    }

    /// Forged trace rejected: mutating the sponge across a sample-draw (a draw must NOT
    /// touch sponge state, spec §4.3).
    #[test]
    fn sponge_mutation_during_draw_is_rejected() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let mut rows = r.rows().to_vec();
        let i = rows.iter().position(|w| w.op == Op::SampleDraw).expect("a draw row exists");
        rows[i + 1].sponge[0] += Val::ONE; // sponge changed across a pure draw
        assert!(!constraints::transition_ok(&rows[i], &rows[i + 1]));
    }

    /// Forged trace rejected: squeeze-flag / length-tag inconsistency on a permute row
    /// (spec §4.2 / §6 #2 — the `+= n` tag is what makes the absorb prefix-free).
    #[test]
    fn permute_squeeze_flag_inconsistency_is_rejected() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let mut rows = r.rows().to_vec();
        let i = rows
            .iter()
            .position(|w| w.op == Op::Permute && w.absorb_len_tag > 0)
            .expect("an absorb-permute row exists");
        rows[i].is_squeeze_permute = true; // claim squeeze while n>0
        assert!(!constraints::transition_ok(&rows[i], &rows[i + 1]));
    }

    /// Boundary rejected: a nonzero starting sponge fails `new()` boundary (spec §4.6).
    #[test]
    fn nonzero_boundary_is_rejected() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let mut rows = r.rows().to_vec();
        rows[0].sponge[3] += Val::ONE;
        assert!(!constraints::boundary_ok(&rows[0]));
        assert!(!constraints::trace_ok(&rows));
    }

    /// AIR column encoding round-trips: pack each honest row to columns and decode back to
    /// the identical Row, and the decoded trace still satisfies the validated §4 constraints.
    /// Validates the one-hot in_len/out_len encoding (the position-masking substrate) faithfully
    /// carries the trace — the data half of the Air<AB> lift.
    #[test]
    fn air_column_encoding_round_trips_and_preserves_constraints() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let rows = r.rows();

        // pack -> unpack is identity on every active row.
        let decoded: Vec<Row> = rows
            .iter()
            .map(|row| {
                let cols = air::pack_row(row);
                let back = air::unpack_row(&cols).expect("packed row decodes");
                assert_eq!(back.op, row.op);
                assert_eq!(back.sponge, row.sponge);
                assert_eq!(back.in_buf, row.in_buf);
                assert_eq!(back.in_len, row.in_len, "one-hot in_len must round-trip");
                assert_eq!(back.out_buf, row.out_buf);
                assert_eq!(back.out_len, row.out_len, "one-hot out_len must round-trip");
                assert_eq!(back.observed_val, row.observed_val);
                assert_eq!(back.draw_out, row.draw_out);
                assert_eq!(back.absorb_len_tag, row.absorb_len_tag);
                assert_eq!(back.is_squeeze_permute, row.is_squeeze_permute);
                back
            })
            .collect();
        // The column-decoded trace satisfies the same validated constraints.
        assert!(constraints::trace_ok(&decoded));
    }

    /// The serialized matrix has power-of-two height and `N_COLS` width; pad rows decode to
    /// the `is_pad` selector with zero lengths.
    #[test]
    fn trace_matrix_is_padded_power_of_two() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let n_active = r.rows().len();
        let (flat, ncols) = air::trace_to_matrix(r.rows());
        assert_eq!(ncols, air::N_COLS);
        let height = flat.len() / ncols;
        assert!(height.is_power_of_two() && height >= n_active);
        // A pad row (last) is NOT a valid active op (is_pad set, op selectors all zero).
        let last = &flat[(height - 1) * ncols..];
        assert_eq!(last[air::C_IS_PAD], Val::ONE);
        assert!(air::unpack_row(last).is_none(), "pad row is not a valid active op");
    }

    /// Encoding-integrity: a corrupted (non-one-hot) length selector is rejected by the
    /// decoder — the AIR will enforce one-hotness; the encoding check mirrors it.
    #[test]
    fn malformed_onehot_selector_is_rejected() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let mut cols = air::pack_row(&r.rows()[0]);
        // set a second in_sel bit -> not one-hot
        cols[air::C_IN_SEL + 3] = Val::ONE;
        cols[air::C_IN_SEL + 4] = Val::ONE;
        assert!(air::unpack_row(&cols).is_none());
    }

    /// The gadget-1 `Air<AB>` ACCEPTS the honest trace and REJECTS each tamper, via the real
    /// Plonky3 `check_all_constraints` (DebugConstraintBuilder) over the packed column matrix.
    /// This is the position-masked constraint eval validated end-to-end as an AIR — the tamper
    /// rejections guard against an underconstrained AIR. (Unpadded matrix: the last row's
    /// `is_transition = 0` disables its transition, so no active→pad edge to special-case here.)
    mod air_eval {
        use super::super::air::{self, ChallengerAir};
        use super::{drive_row, Op, RowChallenger};
        use descender_account_stark::Val;
        use p3_air::check_all_constraints;
        use p3_field::PrimeCharacteristicRing;
        use p3_matrix::dense::RowMajorMatrix;

        fn matrix(rows: &[super::super::Row]) -> RowMajorMatrix<Val> {
            let flat: Vec<Val> = rows.iter().flat_map(|r| air::pack_row(r)).collect();
            RowMajorMatrix::new(flat, air::N_COLS)
        }

        #[test]
        fn air_accepts_honest_trace() {
            let mut r = RowChallenger::new();
            let _ = drive_row(&mut r);
            let m = matrix(r.rows());
            let report = check_all_constraints(&ChallengerAir, &m, &[], None);
            assert!(report.is_ok(), "honest trace failed gadget-1 AIR: {} failures", report.failures.len());
        }

        #[test]
        fn air_rejects_tampered_draw_value() {
            let mut r = RowChallenger::new();
            let _ = drive_row(&mut r);
            let mut rows = r.rows().to_vec();
            let i = rows.iter().position(|w| w.op == Op::SampleDraw).unwrap();
            rows[i].draw_out += Val::ONE;
            assert!(check_all_constraints(&ChallengerAir, &matrix(&rows), &[], None).is_ok() == false);
        }

        #[test]
        fn air_rejects_sponge_mutation_during_draw() {
            let mut r = RowChallenger::new();
            let _ = drive_row(&mut r);
            let mut rows = r.rows().to_vec();
            let i = rows.iter().position(|w| w.op == Op::SampleDraw).unwrap();
            rows[i + 1].sponge[0] += Val::ONE; // sponge changed across a pure draw
            assert!(check_all_constraints(&ChallengerAir, &matrix(&rows), &[], None).is_ok() == false);
        }

        #[test]
        fn air_rejects_broken_observe_append() {
            let mut r = RowChallenger::new();
            let _ = drive_row(&mut r);
            let mut rows = r.rows().to_vec();
            // find an observe row that is NOT the last (so its transition fires) and whose
            // next row should carry the appended value; corrupt the appended cell.
            let i = rows.iter().position(|w| w.op == Op::Observe).unwrap();
            let in_len = rows[i].in_len;
            rows[i + 1].in_buf[in_len] += Val::ONE; // next.in_buf[in_len] != observed_val
            assert!(check_all_constraints(&ChallengerAir, &matrix(&rows), &[], None).is_ok() == false);
        }

        #[test]
        fn air_rejects_nonzero_boundary() {
            let mut r = RowChallenger::new();
            let _ = drive_row(&mut r);
            let mut rows = r.rows().to_vec();
            rows[0].sponge[3] += Val::ONE; // first row must start from zero sponge
            assert!(check_all_constraints(&ChallengerAir, &matrix(&rows), &[], None).is_ok() == false);
        }

        #[test]
        fn air_rejects_broken_onehot_selector() {
            let mut r = RowChallenger::new();
            let _ = drive_row(&mut r);
            let mut rows = r.rows().to_vec();
            // force in_len to a different value than the (untouched) buffer expects by also
            // corrupting nothing else — a non-matching in_sel breaks the one-hot vs next shift.
            rows[2].in_len = if rows[2].in_len == 0 { 1 } else { 0 };
            // pack_row uses in_len for the one-hot, so this desyncs the append/shift relation.
            assert!(check_all_constraints(&ChallengerAir, &matrix(&rows), &[], None).is_ok() == false);
        }
    }

    /// Boundary + one-hot structural checks on the emitted trace (spec §4.6 / §3.1):
    /// first row starts from the zero sponge and empty buffers; every row has exactly
    /// one op selector (enum guarantees this); a `Permute` row's `is_squeeze_permute`
    /// agrees with `absorb_len_tag == 0`.
    #[test]
    fn trace_boundary_and_selector_invariants() {
        let mut r = RowChallenger::new();
        let _ = drive_row(&mut r);
        let rows = r.rows();
        let first = &rows[0];
        assert_eq!(first.sponge, [Val::ZERO; WIDTH], "first row sponge != 0 (new())");
        assert_eq!(first.in_len, 0);
        assert_eq!(first.out_len, 0);
        for row in rows {
            if row.op == Op::Permute {
                assert_eq!(row.is_squeeze_permute, row.absorb_len_tag == 0);
                assert!(row.absorb_len_tag <= RATE);
            }
        }
    }
}
