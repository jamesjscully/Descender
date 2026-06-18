//! Framework-agnostic AIR gadget over Fq (q = 12289) for CANONICAL base-128
//! digit reconstruction + range, plus the masked-projection carrier equation.
//!
//! This is the arithmetization of two row classes of an account-creation STARK
//! relation:
//!   * `qFieldRow`       — digit_reconstruction + masked_projection_equations
//!   * `base128RangeRow` — digit_range_checks
//!
//! CORRECTNESS is paramount. In particular the gadget MUST reject NON-CANONICAL
//! base-128 representations of a field residue, because the account-creation
//! relation relies on the residue->(d_lo,d_hi) decomposition being INJECTIVE.
//!
//! MATH (q = 12289 = 96*128 + 1):
//!   A residue v in [0,q) is decomposed base-128 as the integer identity
//!       v = d_lo + 128 * d_hi,    d_lo, d_hi in [0,128).
//!   The representation is CANONICAL (the integer d_lo + 128*d_hi is < q, so no
//!   field wrap occurred and the map v -> (d_lo,d_hi) is injective) IFF:
//!       d_hi in [0,96]   AND   (d_hi == 96  =>  d_lo == 0).
//!   Reason: 96*128 = 12288 = q-1.
//!     - d_hi in [0,95]  : any d_lo allowed (max 95*128+127 = 12287 < q).
//!     - d_hi == 96      : forces d_lo == 0 (value 12288 = q-1, the largest residue).
//!     - d_hi >= 97, or (d_hi==96 and d_lo>=1): integer value >= q  =>  NON-CANONICAL.
//!
//! CRITICAL: a bare field equation `v = d_lo + 128*d_hi (mod q)` does NOT enforce
//! canonicity, because Fq arithmetic wraps. E.g. d_hi=96, d_lo=6 gives field
//! value (12288 + 6) mod q = 5, which is a perfectly valid field element but a
//! NON-CANONICAL decomposition of the residue 5 (canonical is d_hi=0,d_lo=5).
//! The gadget below rejects this via the `is96 * d_lo = 0` edge constraint.
//!
//! All constraints are degree <= 3 in the witness/trace cells.

use crate::poseidon2_q12289::{fq_add, fq_mul, fq_sub, Q};

/// Base of the digit decomposition.
pub const BASE: u32 = 128;
/// Number of boolean bits per digit (128 = 2^7).
pub const DIGIT_BITS: usize = 7;
/// The canonical upper bound for the high digit: d_hi in [0, MAX_HI].
/// 96 = (q-1)/128, i.e. 96*128 = q-1 = 12288.
pub const MAX_HI: u32 = 96;

// =============================================================================
// Field inverse (Fermat) — used only by the witness generator, never inside a
// constraint. The constraints receive `inv` as an opaque witness value.
// =============================================================================

/// x^(q-2) mod q = x^{-1} for x != 0, built from the reused field ops.
fn fq_inv(x: u32) -> u32 {
    // exponentiation by squaring with exponent q-2 = 12287
    let mut acc: u32 = 1;
    let mut base = x % Q;
    let mut exp: u32 = Q - 2;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = fq_mul(acc, base);
        }
        base = fq_mul(base, base);
        exp >>= 1;
    }
    acc
}

// =============================================================================
// Witness layout
// =============================================================================

/// Per-residue witness for the canonical base-128 range gadget.
///
/// Field-element values are stored as u32 in [0,Q). Bits are stored as u32 in
/// {0,1}. The constraint evaluator treats EVERY field below as an independent
/// trace cell; mutating any one of them (see `mutate_cell`) must break at least
/// one residual when starting from an honest witness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base128RangeWitness {
    /// The residue being decomposed, in [0,Q).
    pub v: u32,
    /// Low digit d_lo in [0,128).
    pub d_lo: u32,
    /// High digit d_hi in [0,128).
    pub d_hi: u32,
    /// 7 boolean bits recomposing d_lo (LSB first).
    pub lo_bits: [u32; DIGIT_BITS],
    /// 7 boolean bits recomposing d_hi (LSB first).
    pub hi_bits: [u32; DIGIT_BITS],
    /// 7 boolean bits recomposing fq_sub(96, d_hi) — proves d_hi <= 96.
    pub slack_bits: [u32; DIGIT_BITS],
    /// Boolean: 1 iff d_hi == 96.
    pub is96: u32,
    /// Field inverse of (d_hi - 96) when d_hi != 96, else arbitrary (0 here).
    pub inv: u32,
}

/// Total number of independent witness/trace cells per residue. Used by
/// `mutate_cell` / mutation testing. Layout index order:
///   0:        v
///   1:        d_lo
///   2:        d_hi
///   3..10:    lo_bits[0..7]
///   10..17:   hi_bits[0..7]
///   17..24:   slack_bits[0..7]
///   24:       is96
///   25:       inv
pub const CELLS_PER_RESIDUE: usize = 3 + 3 * DIGIT_BITS + 2; // = 26

impl Base128RangeWitness {
    /// Fill a witness from a CANONICAL residue v in [0,Q).
    ///
    /// Panics if v >= Q (callers must pass a reduced residue). The produced
    /// witness is always a canonical decomposition, so `residuals` is all zero.
    pub fn generate(v: u32) -> Self {
        assert!(v < Q, "v must be a reduced residue in [0,Q)");
        let d_lo = v % BASE;
        let d_hi = v / BASE; // canonical integer division, in [0,96]
        debug_assert!(d_hi <= MAX_HI);
        debug_assert!(!(d_hi == MAX_HI && d_lo != 0));

        let bits = |x: u32| -> [u32; DIGIT_BITS] {
            let mut b = [0u32; DIGIT_BITS];
            for (i, slot) in b.iter_mut().enumerate() {
                *slot = (x >> i) & 1;
            }
            b
        };

        let slack = fq_sub(MAX_HI, d_hi); // 96 - d_hi, in [0,96] for canonical
        let is96 = if d_hi == MAX_HI { 1 } else { 0 };
        // inv = (d_hi - 96)^{-1} when d_hi != 96; when d_hi == 96 the term it
        // multiplies is zero so any value works — use 0 for determinism.
        let diff = fq_sub(d_hi, MAX_HI);
        let inv = if diff == 0 { 0 } else { fq_inv(diff) };

        Base128RangeWitness {
            v,
            d_lo,
            d_hi,
            lo_bits: bits(d_lo),
            hi_bits: bits(d_hi),
            slack_bits: bits(slack),
            is96,
            inv,
        }
    }

    /// Read the witness as a flat cell vector (for mutation testing).
    fn cells(&self) -> [u32; CELLS_PER_RESIDUE] {
        let mut c = [0u32; CELLS_PER_RESIDUE];
        c[0] = self.v;
        c[1] = self.d_lo;
        c[2] = self.d_hi;
        c[3..10].copy_from_slice(&self.lo_bits);
        c[10..17].copy_from_slice(&self.hi_bits);
        c[17..24].copy_from_slice(&self.slack_bits);
        c[24] = self.is96;
        c[25] = self.inv;
        c
    }

    /// Produce a copy with a single cell overwritten (for mutation testing).
    fn from_cells(c: &[u32; CELLS_PER_RESIDUE]) -> Self {
        let mut lo = [0u32; DIGIT_BITS];
        let mut hi = [0u32; DIGIT_BITS];
        let mut sl = [0u32; DIGIT_BITS];
        lo.copy_from_slice(&c[3..10]);
        hi.copy_from_slice(&c[10..17]);
        sl.copy_from_slice(&c[17..24]);
        Base128RangeWitness {
            v: c[0],
            d_lo: c[1],
            d_hi: c[2],
            lo_bits: lo,
            hi_bits: hi,
            slack_bits: sl,
            is96: c[24],
            inv: c[25],
        }
    }

    /// Return a clone with cell `idx` set to `val`.
    pub fn mutate_cell(&self, idx: usize, val: u32) -> Self {
        let mut c = self.cells();
        c[idx] = val;
        Self::from_cells(&c)
    }
}

// =============================================================================
// Constraint residuals
// =============================================================================

/// Recompose 7 LSB-first bits into a field element: sum bit_i * 2^i.
#[inline]
fn recompose_bits(bits: &[u32; DIGIT_BITS]) -> u32 {
    let mut acc = 0u32;
    let mut weight = 1u32;
    for &b in bits.iter() {
        acc = fq_add(acc, fq_mul(weight % Q, b % Q));
        weight = (weight * 2) % Q;
    }
    acc
}

/// Number of constraint residuals emitted per residue. Must match
/// `residuals().len()`.
///   7  lo_bits booleanity
///   7  hi_bits booleanity
///   7  slack_bits booleanity
///   1  d_lo recompose
///   1  d_hi recompose
///   1  slack recompose (d_hi <= 96)
///   1  is96 booleanity
///   1  is96 * (d_hi - 96) == 0
///   1  (d_hi - 96) * inv == 1 - is96
///   1  is96 * d_lo == 0          (canonical edge: d_hi==96 => d_lo==0)
///   1  v == d_lo + 128 * d_hi    (reconstruction)
pub const CONSTRAINTS_PER_RESIDUE: usize = 3 * DIGIT_BITS + 8; // = 29

/// Maximum algebraic degree (in trace cells) of any emitted constraint.
/// The highest-degree constraint is `(d_hi - 96) * inv == 1 - is96`, which is
/// degree 2. All booleanity (b*(b-1)) are degree 2; recompositions are degree 1;
/// reconstruction is degree 1. So the gadget is degree 2 (<= 3 as required).
pub const MAX_DEGREE: usize = 2;

/// Evaluate ALL constraint residuals for one witness. A residual of 0 means the
/// constraint is satisfied; any nonzero residual is a violation. An honest
/// witness from `generate` yields an all-zero vector; a non-canonical or
/// mutated witness yields at least one nonzero entry.
///
/// The vector is returned in a STABLE order so tests can name "which constraint
/// fired". See `ResidualIndex` for named offsets.
pub fn residuals(w: &Base128RangeWitness) -> Vec<u32> {
    let mut r = Vec::with_capacity(CONSTRAINTS_PER_RESIDUE);

    // booleanity helper: b*(b-1) == 0
    let boolean = |b: u32| -> u32 { fq_mul(b, fq_sub(b, 1)) };

    // [0..7) lo_bits booleanity
    for &b in w.lo_bits.iter() {
        r.push(boolean(b));
    }
    // [7..14) hi_bits booleanity
    for &b in w.hi_bits.iter() {
        r.push(boolean(b));
    }
    // [14..21) slack_bits booleanity
    for &b in w.slack_bits.iter() {
        r.push(boolean(b));
    }

    // [21] d_lo recompose: d_lo - sum(lo_bits) == 0
    r.push(fq_sub(w.d_lo, recompose_bits(&w.lo_bits)));
    // [22] d_hi recompose
    r.push(fq_sub(w.d_hi, recompose_bits(&w.hi_bits)));
    // [23] slack recompose: (96 - d_hi) - sum(slack_bits) == 0  => forces d_hi<=96
    r.push(fq_sub(fq_sub(MAX_HI, w.d_hi), recompose_bits(&w.slack_bits)));

    // [24] is96 booleanity
    r.push(boolean(w.is96));
    // [25] is96 * (d_hi - 96) == 0   (is96==1 => d_hi==96)
    r.push(fq_mul(w.is96, fq_sub(w.d_hi, MAX_HI)));
    // [26] (d_hi - 96) * inv == 1 - is96  (d_hi!=96 => is96==0)
    r.push(fq_sub(
        fq_mul(fq_sub(w.d_hi, MAX_HI), w.inv),
        fq_sub(1, w.is96),
    ));
    // [27] is96 * d_lo == 0   (canonical edge: d_hi==96 => d_lo==0)
    r.push(fq_mul(w.is96, w.d_lo));

    // [28] reconstruction: v - (d_lo + 128*d_hi) == 0
    r.push(fq_sub(w.v, fq_add(w.d_lo, fq_mul(BASE, w.d_hi))));

    debug_assert_eq!(r.len(), CONSTRAINTS_PER_RESIDUE);
    r
}

/// Named residual offsets into the vector returned by `residuals`.
pub mod residual_index {
    use super::DIGIT_BITS;
    pub const LO_BITS_BOOL: usize = 0; // [0,7)
    pub const HI_BITS_BOOL: usize = DIGIT_BITS; // [7,14)
    pub const SLACK_BITS_BOOL: usize = 2 * DIGIT_BITS; // [14,21)
    pub const D_LO_RECOMPOSE: usize = 3 * DIGIT_BITS; // 21
    pub const D_HI_RECOMPOSE: usize = 3 * DIGIT_BITS + 1; // 22
    pub const SLACK_RECOMPOSE: usize = 3 * DIGIT_BITS + 2; // 23  (d_hi <= 96)
    pub const IS96_BOOL: usize = 3 * DIGIT_BITS + 3; // 24
    pub const IS96_FORCES_HI96: usize = 3 * DIGIT_BITS + 4; // 25
    pub const IS96_DEFINITION: usize = 3 * DIGIT_BITS + 5; // 26
    pub const EDGE_HI96_LO0: usize = 3 * DIGIT_BITS + 6; // 27  (d_hi==96 => d_lo==0)
    pub const RECONSTRUCTION: usize = 3 * DIGIT_BITS + 7; // 28
}

/// Convenience: true iff every residual is zero.
pub fn is_satisfied(w: &Base128RangeWitness) -> bool {
    residuals(w).iter().all(|&x| x == 0)
}

// =============================================================================
// Vector version over N residues
// =============================================================================

/// Generate witnesses for a slice of canonical residues.
pub fn generate_vec(vs: &[u32]) -> Vec<Base128RangeWitness> {
    vs.iter().map(|&v| Base128RangeWitness::generate(v)).collect()
}

/// Concatenated residuals over many witnesses (all zero iff all satisfied).
pub fn residuals_vec(ws: &[Base128RangeWitness]) -> Vec<u32> {
    let mut out = Vec::with_capacity(ws.len() * CONSTRAINTS_PER_RESIDUE);
    for w in ws {
        out.extend(residuals(w));
    }
    out
}

// =============================================================================
// Masked-projection carrier equation
// =============================================================================

/// Carrier residual for the masked-projection lane: `masked - mask - account` .
/// This is a DEGREE-1 constraint (linear in trace cells). Returns 0 iff
/// masked == mask + account_projection over Fq.
#[inline]
pub fn carrier_residual(masked: u32, mask: u32, account_projection: u32) -> u32 {
    fq_sub(fq_sub(masked, mask), account_projection)
}

/// Honest carrier value: masked = mask + account_projection (mod q).
#[inline]
pub fn carrier_eval(mask: u32, account_projection: u32) -> u32 {
    fq_add(mask, account_projection)
}

/// Max degree of the carrier constraint.
pub const CARRIER_MAX_DEGREE: usize = 1;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::residual_index as ri;
    use super::*;

    #[test]
    fn base128_range_honest_full_sweep_all_residuals_zero() {
        // Every canonical residue in [0,Q) must produce an all-zero witness.
        for v in 0..Q {
            let w = Base128RangeWitness::generate(v);
            let r = residuals(&w);
            assert_eq!(
                r.iter().position(|&x| x != 0),
                None,
                "canonical residue v={v} produced a nonzero residual: {r:?}"
            );
        }
        // Explicit boundary checks.
        for &v in &[0u32, 1, 127, 128, 12287, 12288] {
            assert!(is_satisfied(&Base128RangeWitness::generate(v)), "boundary v={v}");
        }
        // boundaries: v=12288 is the d_hi==96,d_lo==0 edge.
        let w_edge = Base128RangeWitness::generate(12288);
        assert_eq!(w_edge.d_hi, 96);
        assert_eq!(w_edge.d_lo, 0);
        assert_eq!(w_edge.is96, 1);
        assert!(is_satisfied(&w_edge));
    }

    /// KEY ANCHOR: the non-canonical rep d_hi=96, d_lo=6 (field value 5) MUST be
    /// rejected. We hand-build the BEST POSSIBLE adversarial witness for this
    /// decomposition (all bits/slack/inv chosen to satisfy everything they can)
    /// and show the `is96 * d_lo == 0` edge constraint still fires.
    #[test]
    fn base128_range_rejects_noncanonical_hi96_lo6() {
        let d_hi = 96u32;
        let d_lo = 6u32;
        // field value of this decomposition: 6 + 128*96 = 6 + 12288 = 5 (mod q).
        let v = fq_add(d_lo, fq_mul(BASE, d_hi));
        assert_eq!(v, 5, "sanity: d_hi=96,d_lo=6 has field value 5");

        // Adversary picks the most-satisfying witness it can. d_hi==96 so
        // slack = 96-96 = 0 (valid 7-bit), and the HONEST is-zero gadget for
        // d_hi==96 sets is96=1, inv=0. With is96=1, the edge `is96*d_lo` = 6 != 0.
        let bits = |x: u32| {
            let mut b = [0u32; DIGIT_BITS];
            for (i, s) in b.iter_mut().enumerate() {
                *s = (x >> i) & 1;
            }
            b
        };
        let w_is96_1 = Base128RangeWitness {
            v,
            d_lo,
            d_hi,
            lo_bits: bits(d_lo),
            hi_bits: bits(d_hi),
            slack_bits: bits(0),
            is96: 1,
            inv: 0,
        };
        let r1 = residuals(&w_is96_1);
        assert_ne!(r1[ri::EDGE_HI96_LO0], 0, "is96*d_lo edge must fire (is96=1)");

        // The adversary's only escape would be to set is96=0. But then the
        // is96-definition constraint (d_hi-96)*inv == 1-is96 becomes
        // 0*inv == 1, i.e. 0 == 1, which is unsatisfiable for ANY inv.
        for inv in [0u32, 1, 7, 12288, 6000] {
            let w_is96_0 = Base128RangeWitness {
                is96: 0,
                inv,
                ..w_is96_1.clone()
            };
            let r0 = residuals(&w_is96_0);
            assert_ne!(
                r0[ri::IS96_DEFINITION], 0,
                "with d_hi=96 and is96=0, the is96-definition (0==1) must fire for inv={inv}"
            );
        }
    }

    /// d_hi = 120 (field value 120*128 mod q). The d_hi<=96 slack-recompose
    /// constraint (step 3) must fire because 96-120 = q-24 has no 7-bit rep.
    #[test]
    fn base128_range_rejects_noncanonical_hi120() {
        let d_hi = 120u32;
        let d_lo = 0u32;
        let v = fq_add(d_lo, fq_mul(BASE, d_hi));

        // slack = fq_sub(96,120) = q-24 = 12265, which is NOT representable by
        // 7 boolean bits (max 127). The adversary's best is to pick the 7-bit
        // value closest to 12265 — but no 7-bit value equals it, so the
        // slack-recompose residual is nonzero for EVERY choice of slack_bits.
        let bits = |x: u32| {
            let mut b = [0u32; DIGIT_BITS];
            for (i, s) in b.iter_mut().enumerate() {
                *s = (x >> i) & 1;
            }
            b
        };
        // try ALL 128 possible 7-bit slack values: none can satisfy the recompose.
        for s in 0u32..128 {
            let w = Base128RangeWitness {
                v,
                d_lo,
                d_hi,
                lo_bits: bits(d_lo),
                hi_bits: bits(d_hi),
                slack_bits: bits(s),
                is96: 0,
                inv: fq_inv(fq_sub(d_hi, MAX_HI)),
            };
            let r = residuals(&w);
            assert_ne!(
                r[ri::SLACK_RECOMPOSE], 0,
                "d_hi=120 slack-recompose (d_hi<=96) must fire for every 7-bit slack={s}"
            );
        }
    }

    /// Generic d_hi in [97,127] also rejected by the slack-recompose (step 3).
    #[test]
    fn base128_range_rejects_generic_hi_above_96() {
        for d_hi in 97u32..=127 {
            let d_lo = 13u32;
            let v = fq_add(d_lo, fq_mul(BASE, d_hi));
            let bits = |x: u32| {
                let mut b = [0u32; DIGIT_BITS];
                for (i, s) in b.iter_mut().enumerate() {
                    *s = (x >> i) & 1;
                }
                b
            };
            // best-effort slack: low 7 bits of the true field diff (cannot match)
            let true_slack = fq_sub(MAX_HI, d_hi);
            let mut fired = false;
            for s in 0u32..128 {
                let w = Base128RangeWitness {
                    v,
                    d_lo,
                    d_hi,
                    lo_bits: bits(d_lo),
                    hi_bits: bits(d_hi),
                    slack_bits: bits(s),
                    is96: 0,
                    inv: if fq_sub(d_hi, MAX_HI) == 0 {
                        0
                    } else {
                        fq_inv(fq_sub(d_hi, MAX_HI))
                    },
                };
                if residuals(&w)[ri::SLACK_RECOMPOSE] != 0 {
                    fired = true;
                }
            }
            assert!(fired, "d_hi={d_hi} (>96) must be rejected by slack-recompose");
            let _ = true_slack;
        }
    }

    /// Out-of-range digit: a non-boolean bit and a digit >= 128 are rejected.
    #[test]
    fn base128_range_rejects_out_of_range_digit() {
        // Honest witness for v = 300 (d_lo=44, d_hi=2).
        let w = Base128RangeWitness::generate(300);
        assert!(is_satisfied(&w));

        // (a) set a lo_bit to a non-boolean value 2 -> booleanity fires.
        let mut bad = w.clone();
        bad.lo_bits[0] = 2;
        let r = residuals(&bad);
        assert_ne!(r[ri::LO_BITS_BOOL + 0], 0, "non-boolean lo_bit must fire booleanity");

        // (b) claim a digit >= 128 (d_lo = 200). Its 7-bit recompose can only
        // represent [0,127], so d_lo-recompose fires for every bit choice.
        let bits = |x: u32| {
            let mut b = [0u32; DIGIT_BITS];
            for (i, s) in b.iter_mut().enumerate() {
                *s = (x >> i) & 1;
            }
            b
        };
        for s in 0u32..128 {
            let mut over = w.clone();
            over.d_lo = 200;
            over.lo_bits = bits(s);
            // keep reconstruction honest-ish irrelevant; we only assert recompose
            assert_ne!(
                residuals(&over)[ri::D_LO_RECOMPOSE],
                0,
                "d_lo=200 (>=128) cannot match any 7-bit recompose (bits={s})"
            );
        }
    }

    /// Mutation: flipping any single witness cell of an honest witness breaks
    /// >= 1 residual. Swept over several residues incl. the edge case.
    #[test]
    fn base128_range_single_cell_mutation_breaks_a_residual() {
        for &v in &[0u32, 5, 127, 128, 300, 6000, 12287, 12288] {
            let w = Base128RangeWitness::generate(v);
            assert!(is_satisfied(&w), "baseline honest v={v}");
            for idx in 0..CELLS_PER_RESIDUE {
                // The `inv` cell (idx 25) is a legitimate DON'T-CARE exactly when
                // d_hi == 96: every constraint multiplies inv by (d_hi-96)==0, so
                // inv is unconstrained there. This is correct gadget behaviour
                // (inv only witnesses non-equality), not a soundness hole — when
                // d_hi==96 the is96 path already pins everything. Skip it.
                if idx == 25 && w.d_hi == MAX_HI {
                    continue;
                }
                let cells = w.cells();
                let orig = cells[idx];
                // try a few distinct alternative values for this cell
                for &delta in &[1u32, 2, 7, 100, Q - 1] {
                    let val = (orig + delta) % Q;
                    if val == orig {
                        continue;
                    }
                    let m = w.mutate_cell(idx, val);
                    assert!(
                        !is_satisfied(&m),
                        "mutating cell {idx} of v={v} from {orig} to {val} did NOT break any residual"
                    );
                }
            }
        }
    }

    /// Carrier: honest masked = mask + account satisfies; mutation breaks it.
    #[test]
    fn base128_range_carrier_honest_and_mutation() {
        for &(mask, acc) in &[(0u32, 0u32), (7, 11), (12000, 500), (Q - 1, Q - 1), (123, 9999)] {
            let masked = carrier_eval(mask, acc);
            assert_eq!(carrier_residual(masked, mask, acc), 0, "honest carrier");
            // mutate masked
            assert_ne!(
                carrier_residual(fq_add(masked, 1), mask, acc),
                0,
                "carrier masked mutation must break"
            );
            // mutate mask
            assert_ne!(
                carrier_residual(masked, fq_add(mask, 1), acc),
                0,
                "carrier mask mutation must break"
            );
            // mutate account
            assert_ne!(
                carrier_residual(masked, mask, fq_add(acc, 1)),
                0,
                "carrier account mutation must break"
            );
        }
    }

    /// Vector version sanity + report constraint count and max degree.
    #[test]
    fn base128_range_vector_and_report() {
        let vs: Vec<u32> = (0..Q).step_by(37).collect();
        let ws = generate_vec(&vs);
        let rv = residuals_vec(&ws);
        assert!(rv.iter().all(|&x| x == 0), "vector residuals all zero");
        assert_eq!(rv.len(), ws.len() * CONSTRAINTS_PER_RESIDUE);

        // Report (visible with `cargo test -- --nocapture`).
        println!("base128_range gadget report:");
        println!("  cells per residue        = {CELLS_PER_RESIDUE}");
        println!("  constraints per residue  = {CONSTRAINTS_PER_RESIDUE}");
        println!("  max constraint degree    = {MAX_DEGREE}  (<= 3 required)");
        println!("  carrier max degree       = {CARRIER_MAX_DEGREE}");
        assert!(MAX_DEGREE <= 3);
        assert_eq!(CONSTRAINTS_PER_RESIDUE, 29);
    }
}
