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

    /// Spec §1.2 `observe(value)`.
    pub fn observe(&mut self, value: Val) {
        self.out_buf.clear();
        self.push_row(Op::Observe, value, Val::ZERO, 0);
        self.in_buf.push(value);
        if self.in_buf.len() == RATE {
            self.duplexing();
        }
    }

    /// One base-field draw (spec §1.3 inner closure of `sample`).
    pub fn sample(&mut self) -> Val {
        if !self.in_buf.is_empty() || self.out_buf.is_empty() {
            self.duplexing();
        }
        let v = self.out_buf.pop().expect("output buffer nonempty after duplexing");
        self.push_row(Op::SampleDraw, Val::ZERO, v, 0);
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
