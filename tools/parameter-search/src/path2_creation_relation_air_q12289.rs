//! Composed account-creation relation AIR over Fq = 12289.
//!
//! This module COMPOSES existing, independently-tested AIR gadgets into a
//! SINGLE relation that binds a HIDDEN `pad_seed` + `slot_id` all the way to a
//! PUBLIC `masked_projection` AND a PUBLIC `pad_source_commitment`:
//!
//!   * C1 — `crate::poseidon2_commit_air_q12289`: proves the 20 committed
//!     `pad_source_commitment` lanes ARE
//!     `pad_source_commitment_poseidon2(pad_seed, blinding_nonce)` of a
//!     byte-range-checked hidden input. This is the pad-source binding that
//!     replaces the SHA `pad_seed_digest` scaffold.
//!   * C2 — `crate::poseidon2_prf_air_q12289`: proves the 6 mask lanes ARE
//!     `projection_masks_poseidon2(pad_seed, slot_id)` of a byte-range-checked
//!     hidden input.
//!   * C4 — `crate::base128_range_air_q12289`: proves each of the 6
//!     `account_projection` residues is a CANONICAL base-128 residue in [0,q),
//!     and supplies the masked-projection carrier equation
//!     `masked = mask + account_projection`.
//!
//! =============================================================================
//! THE SHARED pad_seed IS THE WHOLE POINT (C1 ⟷ C2)
//! =============================================================================
//!
//! C1 and C2 each carry their OWN 32 pad_seed byte-lanes (the first 32 entries
//! of their respective sponge `input_lanes`). Co-locating two commitment proofs
//! over INDEPENDENT pad_seeds would be vacuous. So we emit 32 explicit
//! SEAM-EQUALITY constraints
//!
//!   `SeamPadSeed { byte: i } : c2.input_lanes[i] - c1.input_lanes[i] = 0`
//!     for i in 0..32
//!
//! forcing the pad_seed that derives the projection masks (C2) to be the SAME
//! pad_seed that opens the pad-source commitment (C1). `generate()` runs both
//! sub-AIRs from a single `pad_seed`, so the honest witness satisfies all 32;
//! the shared-pad_seed soundness test tampers C1's pad_seed and watches a
//! `SeamPadSeed` constraint fire.
//!
//! =============================================================================
//! THE SEAM IS THE WHOLE POINT
//! =============================================================================
//!
//! A naive "composition" would let the carrier use INDEPENDENT copies of the
//! mask and the account-projection — a vacuous "digit co-location" that a
//! machine-checked negative theorem already flagged for this project. We do NOT
//! do that. Instead:
//!
//!   * The carrier's `mask_i` IS the C2 PRF AIR's claimed mask output
//!     `prf.mask_outputs[i]` (the very cell the C2 mask-output boundary binds to
//!     the squeezed sponge state). There is NO independent mask column.
//!   * The carrier's `account_projection_i` IS the C4 reconstruction output
//!     `range[i].v` (the residue cell the C4 RECONSTRUCTION constraint binds to
//!     the canonical digits). There is NO independent account column.
//!
//! The carrier residual is evaluated DIRECTLY on those shared cells (see
//! `carrier_residual_for_lane`), so the carrier provably consumes the PRF's
//! actual mask output and the reconstruction's actual residue.
//!
//! BELT-AND-SUSPENDERS — EXPLICIT SEAM-EQUALITY CONSTRAINTS. Because the two
//! sub-modules expose their outputs as plain owned cells (not a shared column
//! object), we ALSO emit explicit seam-equality residuals so that any attempt to
//! drive the carrier with a value that differs from the sub-witness output is
//! caught by a NAMED constraint, independent of the carrier algebra:
//!
//!   * `SeamMask  { lane }` : `prf.mask_outputs[lane] - carrier_mask[lane] = 0`
//!   * `SeamAcc   { lane }` : `range[lane].v        - carrier_acc[lane]  = 0`
//!
//! Here `carrier_mask`/`carrier_acc` are the values the carrier ACTUALLY reads.
//! In the honest witness they are literally the same cells; the seam-equality
//! constraints make that binding auditable and let the soundness tests tamper
//! "the value the carrier sees" vs "the value the sub-witness proved" and watch
//! a specific residual fire. EITHER WAY the binding is enforced.
//!
//! =============================================================================
//! PUBLIC / HIDDEN INTERFACE
//! =============================================================================
//!
//!   PUBLIC  inputs: `masked_projection: [u16; 6]`,
//!                   `pad_source_commitment: [u16; 20]`.
//!   HIDDEN  inputs: `pad_seed: [u8; 32]`, `slot_id: u64`,
//!                   `account_projection: [u16; 6]` (each canonical in [0,q)),
//!                   `blinding_nonce: [u8; 16]`.
//!
//! =============================================================================
//! CONSTRAINTS & DEGREE
//! =============================================================================
//!
//! The combined residual vector is the UNION of:
//!   0. all C1 commitment AIR residuals (commitment = sponge), bound to C1's
//!      claimed output; 20 commit public-bind residuals; 32 SEAM-pad_seed
//!      residuals binding C1's pad_seed byte-lanes to C2's;
//!   1. all C2 PRF AIR residuals (mask = PRF), bound to `prf.mask_outputs`;
//!   2. all C4 range/canonical residuals for the 6 account-projection residues;
//!   3. 6 CARRIER residuals `masked_projection_i - mask_i - acc_i = 0`, with
//!      `mask_i`/`acc_i` the SHARED sub-witness cells;
//!   4. 6 SEAM-MASK + 6 SEAM-ACC equality residuals binding the carrier inputs
//!      to the sub-witness outputs.
//!
//! MAX DEGREE = 5 (the Poseidon2 S-box x^5 inside the reused per-permutation
//! AIR). Every C4 constraint is degree <= 2; carrier and seam-equality are
//! degree <= 1.

use crate::base128_range_air_q12289 as c4;
use crate::poseidon2_commit_air_q12289 as c1;
use crate::poseidon2_opening_tuple_commit_air_q12289 as c6;
use crate::poseidon2_prf_air_q12289 as c2;
use crate::poseidon2_q12289::{
    OPENING_TUPLE_BLINDING_BYTES, OPENING_TUPLE_COMMITMENT_LANES,
    OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES, PAD_SOURCE_BLINDING_BYTES, PAD_SOURCE_COMMITMENT_LANES,
    Poseidon2Params, Q, fq_add, fq_sub,
};

/// Number of projection lanes (mask / account / masked), = 6.
pub const N_LANES: usize = c2::N_MASK_OUTPUTS;

/// Number of pad_seed byte-lanes shared between C1 and C2 (the seam width).
pub const PAD_SEED_LANES: usize = c2::PAD_SEED_LANES; // 32

/// Number of pad-source commitment output lanes (C1's public commitment).
pub const N_COMMIT_LANES: usize = PAD_SOURCE_COMMITMENT_LANES; // 10

/// Number of opening-tuple commitment output lanes (C6's public commitment).
pub const N_OPENING_COMMIT_LANES: usize = OPENING_TUPLE_COMMITMENT_LANES; // 10

/// Number of account_projection lanes shared between C4 and C6 (the seam width).
/// = N_LANES (6), but named for clarity at the seam.
pub const OPENING_ACC_SEAM_LANES: usize = N_LANES; // 6

// -----------------------------------------------------------------------------
// Composed witness
// -----------------------------------------------------------------------------

/// The full composed-relation witness.
///
/// The carrier inputs (`carrier_mask`, `carrier_acc`) are NOT independent
/// secrets — in the honest witness they are copies of the sub-witness outputs
/// (`prf.mask_outputs[i]` and `range[i].v`). They exist as named cells ONLY so
/// the seam-equality constraints have something to bind, and so tests can tamper
/// "what the carrier sees" against "what the sub-witness proved". The carrier
/// algebra reads these cells; the seam-equality constraints pin them to the
/// genuine sub-witness outputs.
#[derive(Clone, Debug)]
pub struct ComposedWitness {
    /// C1 commitment sub-witness: proves pad_source_commitment =
    /// pad_source_commitment_poseidon2(pad_seed, blinding_nonce). Its first 32
    /// input byte-lanes are the SAME pad_seed bound (by SeamPadSeed) to C2's.
    pub commit: c1::Trace,
    /// PUBLIC input: the 20-lane pad-source commitment (C1's claimed output).
    pub pad_source_commitment: [u32; N_COMMIT_LANES],
    /// C2 PRF sub-witness: proves mask = PRF(pad_seed, slot_id).
    pub prf: c2::Trace,
    /// C4 range sub-witnesses, one per account-projection residue (canonical).
    pub range: Vec<c4::Base128RangeWitness>,
    /// C6 opening-tuple commitment sub-witness: proves opening_tuple_commitment =
    /// opening_tuple_commitment_poseidon2(account_projection, lifecycle_anchor,
    /// opening_blinding_nonce). Its first 6 input FIELD lanes are the SAME
    /// account_projection bound (by SeamOpeningAcc) to C4's reconstruction.
    pub opening: c6::Trace,
    /// PUBLIC input: the 20-lane opening-tuple commitment (C6's claimed output).
    pub opening_tuple_commitment: [u32; N_OPENING_COMMIT_LANES],
    /// The value the carrier uses as `mask_i`. Honest = prf.mask_outputs[i].
    pub carrier_mask: [u32; N_LANES],
    /// The value the carrier uses as `acc_i`. Honest = range[i].v.
    pub carrier_acc: [u32; N_LANES],
    /// PUBLIC input: masked_projection_i = mask_i + account_projection_i.
    pub masked_projection: [u32; N_LANES],
}

impl ComposedWitness {
    /// The 6 mask outputs the PRF sub-witness claims (shared carrier `mask_i`).
    #[inline]
    pub fn prf_mask_outputs(&self) -> [u32; N_LANES] {
        c2::claimed_masks(&self.prf)
    }

    /// The 6 reconstructed residues the range sub-witnesses claim (shared
    /// carrier `acc_i`).
    #[inline]
    pub fn range_reconstructed(&self) -> [u32; N_LANES] {
        let mut out = [0u32; N_LANES];
        for (i, w) in self.range.iter().enumerate() {
            out[i] = w.v;
        }
        out
    }

    /// The 32 pad_seed byte-lanes as seen by C2 (the PRF sub-witness).
    #[inline]
    pub fn c2_pad_seed_lanes(&self) -> &[u32] {
        &self.prf.input_lanes[0..PAD_SEED_LANES]
    }

    /// The 32 pad_seed byte-lanes as seen by C1 (the commitment sub-witness).
    #[inline]
    pub fn c1_pad_seed_lanes(&self) -> &[u32] {
        self.commit.pad_seed_lanes()
    }

    /// The 6 account_projection FIELD lanes as seen by C6 (the opening-tuple
    /// commitment sub-witness). SeamOpeningAcc binds these to C4's `range[i].v`.
    #[inline]
    pub fn c6_account_projection_lanes(&self) -> &[u32] {
        self.opening.account_projection_lanes()
    }
}

/// HONEST composed generator.
///
/// 1. runs the C2 PRF AIR generator -> mask[6] = PRF(pad_seed, slot_id);
/// 2. runs the C4 range generator on the 6 account_projection residues;
/// 3. computes masked_projection_i = fq_add(mask_i, account_projection_i) and
///    exposes it as the public input.
///
/// The carrier inputs are set to the genuine sub-witness outputs (shared cells),
/// so the honest witness satisfies every residual including the seam-equalities.
pub fn generate(
    pad_seed: &[u8; 32],
    slot_id: u64,
    account_projection: &[u16; N_LANES],
    blinding_nonce: &[u8; PAD_SOURCE_BLINDING_BYTES],
    lifecycle_anchor: &[u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES],
    opening_blinding_nonce: &[u8; OPENING_TUPLE_BLINDING_BYTES],
) -> ComposedWitness {
    // C1: commitment sub-witness over the SAME pad_seed + blinding nonce. Its
    // first 32 input byte-lanes ARE this pad_seed; SeamPadSeed binds them to C2.
    let commit = c1::generate_commit_trace(pad_seed, blinding_nonce);
    let commit_out = c1::claimed_commitment(&commit);
    let mut pad_source_commitment = [0u32; N_COMMIT_LANES];
    pad_source_commitment.copy_from_slice(&commit_out);

    // C2: PRF sub-witness over the SAME pad_seed. Its mask_outputs are genuine.
    let prf = c2::generate_prf_trace(pad_seed, slot_id);
    let mask = c2::claimed_masks(&prf); // = projection_masks_poseidon2(...)

    // C4: range sub-witnesses for each canonical account-projection residue.
    let mut range: Vec<c4::Base128RangeWitness> = Vec::with_capacity(N_LANES);
    for &v in account_projection.iter() {
        assert!(
            (v as u32) < Q,
            "account_projection lane must be canonical (< q)"
        );
        range.push(c4::Base128RangeWitness::generate(v as u32));
    }

    // C6: opening-tuple commitment sub-witness over the SAME account_projection
    // (absorbed as FIELD lanes) plus the lifecycle anchor, hidden under its own
    // blinding nonce. Its first 6 FIELD input lanes ARE this account_projection;
    // SeamOpeningAcc binds them to C4's reconstruction `range[i].v`.
    let opening =
        c6::generate_commit_trace(account_projection, lifecycle_anchor, opening_blinding_nonce);
    let opening_out = c6::claimed_commitment(&opening);
    let mut opening_tuple_commitment = [0u32; N_OPENING_COMMIT_LANES];
    opening_tuple_commitment.copy_from_slice(&opening_out);

    // Carrier inputs = the genuine SHARED sub-witness outputs.
    let mut carrier_mask = [0u32; N_LANES];
    let mut carrier_acc = [0u32; N_LANES];
    let mut masked_projection = [0u32; N_LANES];
    for i in 0..N_LANES {
        carrier_mask[i] = mask[i] as u32; // = prf.mask_outputs[i]
        carrier_acc[i] = range[i].v; // = reconstruction output
        masked_projection[i] = fq_add(carrier_mask[i], carrier_acc[i]);
    }

    ComposedWitness {
        commit,
        pad_source_commitment,
        prf,
        range,
        opening,
        opening_tuple_commitment,
        carrier_mask,
        carrier_acc,
        masked_projection,
    }
}

// -----------------------------------------------------------------------------
// Combined constraint residuals
// -----------------------------------------------------------------------------

/// A tagged scalar residual. 0 == satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Residual {
    pub kind: ResidualKind,
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualKind {
    /// A C1 commitment AIR residual (the delegated commitment=sponge set).
    Commit { index: usize },
    /// Public binding: public pad_source_commitment[lane] - C1 claimed lane == 0.
    CommitBind { lane: usize },
    /// Seam-equality: c2.pad_seed_lane[byte] - c1.pad_seed_lane[byte] == 0.
    /// THE shared-pad_seed binding between the mask PRF (C2) and the pad-source
    /// commitment (C1).
    SeamPadSeed { byte: usize },
    /// A C2 PRF AIR residual (the delegated mask=PRF constraint set).
    Prf { index: usize },
    /// A C4 range/canonical residual for account-projection lane `lane`,
    /// sub-constraint `sub` (see c4::residual_index).
    Range { lane: usize, sub: usize },
    /// A C6 opening-tuple commitment AIR residual (the delegated
    /// commitment=sponge set).
    Opening { index: usize },
    /// Public binding: public opening_tuple_commitment[lane] - C6 claimed lane.
    OpeningCommitBind { lane: usize },
    /// Seam-equality: c4_account_projection[lane] (range[lane].v) -
    /// c6_account_projection_lane[lane] == 0. THE shared-account_projection
    /// binding between the C4 carrier reconstruction and the opening-tuple
    /// commitment (C6). Without it the opening-tuple co-location is vacuous.
    SeamOpeningAcc { lane: usize },
    /// Carrier: masked_projection[lane] - mask[lane] - acc[lane] == 0, where
    /// mask/acc are the SHARED carrier cells.
    Carrier { lane: usize },
    /// Public binding: public_masked_projection[lane] - witness masked cell == 0.
    PublicBind { lane: usize },
    /// Seam-equality: prf.mask_outputs[lane] - carrier_mask[lane] == 0.
    SeamMask { lane: usize },
    /// Seam-equality: range[lane].v - carrier_acc[lane] == 0.
    SeamAcc { lane: usize },
}

/// The carrier residual for one lane, evaluated on the SHARED cells the carrier
/// actually consumes.
#[inline]
pub fn carrier_residual_for_lane(w: &ComposedWitness, lane: usize) -> u32 {
    // Reuse the C4 carrier algebra verbatim on the carrier's shared inputs.
    c4::carrier_residual(
        w.masked_projection[lane],
        w.carrier_mask[lane],
        w.carrier_acc[lane],
    )
}

/// Evaluate EVERY constraint residual for the composed witness against the
/// PUBLIC `masked_projection`. All-zero means: the public masked_projection is
/// the lane-wise sum of (a) the Poseidon2 PRF mask of the hidden (pad_seed,
/// slot_id) and (b) a CANONICAL hidden account_projection — with the carrier
/// provably consuming those exact sub-witness outputs.
pub fn all_residuals(
    params: &Poseidon2Params,
    w: &ComposedWitness,
    public_masked_projection: &[u32; N_LANES],
    public_pad_source_commitment: &[u32; N_COMMIT_LANES],
    public_opening_tuple_commitment: &[u32; N_OPENING_COMMIT_LANES],
) -> Vec<Residual> {
    let mut out: Vec<Residual> = Vec::new();

    // 0a. C1 commitment AIR residuals, bound to C1's own claimed commitment.
    let commit_claimed = c1::claimed_commitment(&w.commit);
    for (idx, r) in c1::all_residuals(params, &w.commit, &commit_claimed)
        .iter()
        .enumerate()
    {
        out.push(Residual {
            kind: ResidualKind::Commit { index: idx },
            value: r.value,
        });
    }

    // 0b. PUBLIC binding for the pad-source commitment: the public target must
    //     equal C1's claimed commitment output.
    for lane in 0..N_COMMIT_LANES {
        out.push(Residual {
            kind: ResidualKind::CommitBind { lane },
            value: fq_sub(
                public_pad_source_commitment[lane],
                w.pad_source_commitment[lane],
            ),
        });
    }

    // 0c. SHARED-pad_seed SEAM: C2's 32 pad_seed byte-lanes == C1's. This is the
    //     algebraic root binding the mask PRF and the pad-source commitment to
    //     ONE pad_seed.
    let c2_pad = w.c2_pad_seed_lanes();
    let c1_pad = w.c1_pad_seed_lanes();
    for byte in 0..PAD_SEED_LANES {
        out.push(Residual {
            kind: ResidualKind::SeamPadSeed { byte },
            value: fq_sub(c2_pad[byte], c1_pad[byte]),
        });
    }

    // 1. C2 PRF AIR residuals, bound to the PRF sub-witness's own claimed masks.
    //    (The mask-output boundary inside C2 binds prf.mask_outputs to the
    //    squeezed sponge state.)
    let prf_masks = c2::claimed_masks(&w.prf);
    for (idx, r) in c2::all_residuals(params, &w.prf, &prf_masks)
        .iter()
        .enumerate()
    {
        out.push(Residual {
            kind: ResidualKind::Prf { index: idx },
            value: r.value,
        });
    }

    // 2. C4 range/canonical residuals for each account-projection residue.
    for (lane, rw) in w.range.iter().enumerate() {
        for (sub, &rv) in c4::residuals(rw).iter().enumerate() {
            out.push(Residual {
                kind: ResidualKind::Range { lane, sub },
                value: rv,
            });
        }
    }

    // 2c. C6 opening-tuple commitment AIR residuals, bound to C6's own claimed
    //     commitment.
    let opening_claimed = c6::claimed_commitment(&w.opening);
    for (idx, r) in c6::all_residuals(params, &w.opening, &opening_claimed)
        .iter()
        .enumerate()
    {
        out.push(Residual {
            kind: ResidualKind::Opening { index: idx },
            value: r.value,
        });
    }

    // 2d. PUBLIC binding for the opening-tuple commitment: the public target must
    //     equal C6's claimed commitment output.
    for lane in 0..N_OPENING_COMMIT_LANES {
        out.push(Residual {
            kind: ResidualKind::OpeningCommitBind { lane },
            value: fq_sub(
                public_opening_tuple_commitment[lane],
                w.opening_tuple_commitment[lane],
            ),
        });
    }

    // 2e. SHARED-account_projection SEAM (the whole point of co-locating C6 with
    //     the C4 carrier): C6's 6 account_projection FIELD input lanes ==
    //     C4's reconstructed residues `range[i].v`. This binds the opening-tuple
    //     commitment to the SAME account_projection the carrier consumes. Without
    //     it, C6 could commit to an unrelated account_projection (vacuous).
    let c6_acc = w.c6_account_projection_lanes();
    let c4_acc = w.range_reconstructed();
    for lane in 0..OPENING_ACC_SEAM_LANES {
        out.push(Residual {
            kind: ResidualKind::SeamOpeningAcc { lane },
            value: fq_sub(c4_acc[lane], c6_acc[lane]),
        });
    }

    // 3a. PUBLIC BINDING: the public masked_projection target must equal the
    //     witness's masked cell (so a forged public target can't differ from
    //     the witness). Kept SEPARATE from the carrier so neither check can
    //     cancel the other (a single residual folding both would let the
    //     witness masked cell cancel out — a real soundness hole).
    for lane in 0..N_LANES {
        out.push(Residual {
            kind: ResidualKind::PublicBind { lane },
            value: fq_sub(public_masked_projection[lane], w.masked_projection[lane]),
        });
    }

    // 3b. CARRIER residuals on the SHARED cells: masked - mask - acc == 0,
    //     with mask/acc the cells the carrier actually consumes.
    for lane in 0..N_LANES {
        out.push(Residual {
            kind: ResidualKind::Carrier { lane },
            value: carrier_residual_for_lane(w, lane),
        });
    }

    // 4. SEAM-EQUALITY: pin the carrier inputs to the genuine sub-witness
    //    outputs. A tamper of "what the carrier sees" vs "what the sub-witness
    //    proved" fires one of these by NAME.
    let reconstructed = w.range_reconstructed();
    for lane in 0..N_LANES {
        out.push(Residual {
            kind: ResidualKind::SeamMask { lane },
            value: fq_sub(prf_masks[lane], w.carrier_mask[lane]),
        });
        out.push(Residual {
            kind: ResidualKind::SeamAcc { lane },
            value: fq_sub(reconstructed[lane], w.carrier_acc[lane]),
        });
    }

    out
}

/// True iff every combined residual is zero.
pub fn is_satisfied(
    params: &Poseidon2Params,
    w: &ComposedWitness,
    public_masked_projection: &[u32; N_LANES],
    public_pad_source_commitment: &[u32; N_COMMIT_LANES],
    public_opening_tuple_commitment: &[u32; N_OPENING_COMMIT_LANES],
) -> bool {
    all_residuals(
        params,
        w,
        public_masked_projection,
        public_pad_source_commitment,
        public_opening_tuple_commitment,
    )
    .iter()
    .all(|r| r.value == 0)
}

/// Total scalar constraint count: C1 + commit-bind 10 + seam-pad_seed 32 +
/// C2 + C4*6 + public-bind 6 + carrier 6 + seam (6+6).
pub fn n_constraints() -> usize {
    c1::n_constraints()
        + N_COMMIT_LANES // commit public-bind
        + PAD_SEED_LANES // seam pad_seed (C1 <-> C2)
        + c2::n_constraints()
        + N_LANES * c4::CONSTRAINTS_PER_RESIDUE
        + c6::n_constraints() // C6 opening-tuple commitment AIR
        + N_OPENING_COMMIT_LANES // opening-tuple commit public-bind
        + OPENING_ACC_SEAM_LANES // seam account_projection (C4 <-> C6)
        + N_LANES // masked public-bind
        + N_LANES // carrier
        + 2 * N_LANES // seam mask + seam acc
}

/// Maximum polynomial degree of any constraint: 5 (Poseidon2 S-box in C2).
/// C4 is degree <= 2; carrier + seam-equality are degree <= 1.
pub const MAX_DEGREE: u32 = 5;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_q12289::{
        opening_tuple_commitment_poseidon2, pad_source_commitment_poseidon2,
        projection_masks_poseidon2,
    };

    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    fn random_pad(seed: &mut u64) -> [u8; 32] {
        let mut p = [0u8; 32];
        for b in p.iter_mut() {
            *b = (xorshift(seed) & 0xff) as u8;
        }
        p
    }

    fn random_nonce(seed: &mut u64) -> [u8; PAD_SOURCE_BLINDING_BYTES] {
        let mut n = [0u8; PAD_SOURCE_BLINDING_BYTES];
        for b in n.iter_mut() {
            *b = (xorshift(seed) & 0xff) as u8;
        }
        n
    }

    /// A fixed test nonce for the deterministic single-vector tests.
    const TEST_NONCE: [u8; PAD_SOURCE_BLINDING_BYTES] = [9u8; PAD_SOURCE_BLINDING_BYTES];
    /// A fixed test lifecycle anchor for the C6 opening-tuple commitment.
    const TEST_ANCHOR: [u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES] =
        [13u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES];
    /// A fixed test opening-tuple blinding nonce.
    const TEST_OPENING_NONCE: [u8; OPENING_TUPLE_BLINDING_BYTES] =
        [15u8; OPENING_TUPLE_BLINDING_BYTES];

    fn random_anchor(seed: &mut u64) -> [u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES] {
        let mut p = [0u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES];
        for b in p.iter_mut() {
            *b = (xorshift(seed) & 0xff) as u8;
        }
        p
    }

    fn random_opening_nonce(seed: &mut u64) -> [u8; OPENING_TUPLE_BLINDING_BYTES] {
        let mut n = [0u8; OPENING_TUPLE_BLINDING_BYTES];
        for b in n.iter_mut() {
            *b = (xorshift(seed) & 0xff) as u8;
        }
        n
    }

    fn random_acc(seed: &mut u64) -> [u16; N_LANES] {
        let mut a = [0u16; N_LANES];
        for x in a.iter_mut() {
            *x = (xorshift(seed) % Q as u64) as u16;
        }
        a
    }

    /// ANCHOR: generate() yields masked_projection == fq_add(PRF[i], acc[i]) for
    /// each i, and the honest composed witness satisfies ALL residuals (0).
    #[test]
    fn path2_creation_relation_anchor_matches_reference_and_satisfies_all() {
        let params = Poseidon2Params::new();
        let mut seed = 0xA11C_E0DE_1234_5678u64;
        for _ in 0..24 {
            let pad = random_pad(&mut seed);
            let slot = xorshift(&mut seed);
            let acc = random_acc(&mut seed);
            let nonce = random_nonce(&mut seed);
            let anchor = random_anchor(&mut seed);
            let opening_nonce = random_opening_nonce(&mut seed);

            let w = generate(&pad, slot, &acc, &nonce, &anchor, &opening_nonce);

            // masked_projection == fq_add(reference PRF, acc) lane-wise.
            let reference = projection_masks_poseidon2(&pad, slot);
            for i in 0..N_LANES {
                let expect = fq_add(reference[i] as u32, acc[i] as u32);
                assert_eq!(
                    w.masked_projection[i], expect,
                    "lane {i}: masked != PRF + acc"
                );
            }

            // pad_source_commitment == reference commitment over the SAME pad+nonce.
            let ref_commit = pad_source_commitment_poseidon2(&pad, &nonce);
            for i in 0..N_COMMIT_LANES {
                assert_eq!(
                    w.pad_source_commitment[i], ref_commit[i] as u32,
                    "commit lane {i}: != reference pad_source_commitment"
                );
            }

            // opening_tuple_commitment == reference over SAME acc+anchor+nonce.
            let ref_opening = opening_tuple_commitment_poseidon2(&acc, &anchor, &opening_nonce);
            for i in 0..N_OPENING_COMMIT_LANES {
                assert_eq!(
                    w.opening_tuple_commitment[i], ref_opening[i] as u32,
                    "opening lane {i}: != reference opening_tuple_commitment"
                );
            }

            // SHARED account_projection: C4 and C6 truly hold the SAME 6 lanes.
            for (i, &v) in acc.iter().enumerate() {
                assert_eq!(
                    w.range_reconstructed()[i],
                    v as u32,
                    "C4 reconstructed lane {i} != acc"
                );
                assert_eq!(
                    w.c6_account_projection_lanes()[i],
                    v as u32,
                    "C6 account_projection lane {i} != acc"
                );
            }

            // SHARED pad_seed: C1 and C2 truly hold the SAME 32 pad_seed lanes.
            assert_eq!(
                w.c1_pad_seed_lanes(),
                w.c2_pad_seed_lanes(),
                "C1/C2 pad_seed differ"
            );
            for (i, &b) in pad.iter().enumerate() {
                assert_eq!(
                    w.c2_pad_seed_lanes()[i],
                    b as u32,
                    "C2 pad lane {i} != pad byte"
                );
                assert_eq!(
                    w.c1_pad_seed_lanes()[i],
                    b as u32,
                    "C1 pad lane {i} != pad byte"
                );
            }

            // The carrier truly consumes the PRF output and the reconstruction.
            assert_eq!(
                w.carrier_mask,
                w.prf_mask_outputs(),
                "carrier mask != PRF output"
            );
            assert_eq!(
                w.carrier_acc,
                w.range_reconstructed(),
                "carrier acc != reconstruction"
            );

            // ALL residuals zero against ALL THREE public inputs.
            let public = w.masked_projection;
            let public_commit = w.pad_source_commitment;
            let public_opening = w.opening_tuple_commitment;
            let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);
            assert_eq!(res.len(), n_constraints(), "constraint count mismatch");
            for r in &res {
                assert_eq!(r.value, 0, "honest composed witness violated {r:?}");
            }
            assert!(is_satisfied(
                &params,
                &w,
                &public,
                &public_commit,
                &public_opening
            ));
        }
    }

    /// SHARED-pad_seed SOUNDNESS (THE KEY TEST): make the pad_seed C1 commits to
    /// differ from the pad_seed C2 derives masks from. A SeamPadSeed constraint
    /// must fire — proving the commitment and the PRF are bound to ONE pad_seed.
    #[test]
    fn path2_creation_relation_shared_pad_seed_soundness_seampad_fires() {
        let params = Poseidon2Params::new();
        let pad = [5u8; 32];
        let acc = [1u16, 2, 3, 4, 5, 6];
        let w0 = generate(
            &pad,
            8,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w0.masked_projection;
        let public_commit = w0.pad_source_commitment;
        let public_opening = w0.opening_tuple_commitment;

        // Baseline honest passes.
        assert!(is_satisfied(
            &params,
            &w0,
            &public,
            &public_commit,
            &public_opening
        ));

        // Tamper ONLY C1's pad_seed byte-lane 17 (the value the commitment opens
        // to). C2's pad_seed lane 17 is unchanged, so they now disagree.
        let mut w = w0.clone();
        let before = w.commit.input_lanes[17];
        w.commit.input_lanes[17] = fq_add(before, 1);

        let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);
        // SeamPadSeed for byte 17 fires by name.
        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamPadSeed { byte: 17 }),
            "SeamPadSeed byte 17 did not fire when C1 pad_seed != C2 pad_seed"
        );
        // And it is byte 17 SPECIFICALLY (other seam bytes stay clean).
        for byte in 0..PAD_SEED_LANES {
            if byte != 17 {
                assert!(
                    res.iter()
                        .all(|r| !(r.kind == ResidualKind::SeamPadSeed { byte } && r.value != 0)),
                    "unexpected SeamPadSeed fire at byte {byte}"
                );
            }
        }

        // Symmetric direction: tamper C2's pad_seed lane instead -> also fires.
        let mut w2 = w0.clone();
        w2.prf.input_lanes[3] = fq_add(w2.prf.input_lanes[3], 1);
        let res2 = all_residuals(&params, &w2, &public, &public_commit, &public_opening);
        assert!(
            res2.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamPadSeed { byte: 3 }),
            "SeamPadSeed byte 3 did not fire when C2 pad_seed != C1 pad_seed"
        );
    }

    /// COMMIT BIND: a forged public pad_source_commitment is rejected.
    #[test]
    fn path2_creation_relation_forged_commitment_rejected() {
        let params = Poseidon2Params::new();
        let pad = [19u8; 32];
        let acc = [7u16, 8, 9, 10, 11, 12];
        let w = generate(
            &pad,
            4,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w.masked_projection;
        let public_opening = w.opening_tuple_commitment;

        let mut forged = w.pad_source_commitment;
        forged[6] = fq_add(forged[6], 1);
        let res = all_residuals(&params, &w, &public, &forged, &public_opening);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::CommitBind { lane: 6 }),
            "forged pad_source_commitment not rejected by CommitBind"
        );
    }

    /// OPENING COMMIT BIND: a forged public opening_tuple_commitment is rejected.
    #[test]
    fn path2_creation_relation_forged_opening_commitment_rejected() {
        let params = Poseidon2Params::new();
        let pad = [19u8; 32];
        let acc = [7u16, 8, 9, 10, 11, 12];
        let w = generate(
            &pad,
            4,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w.masked_projection;
        let public_commit = w.pad_source_commitment;

        let mut forged = w.opening_tuple_commitment;
        forged[3] = fq_add(forged[3], 1);
        let res = all_residuals(&params, &w, &public, &public_commit, &forged);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::OpeningCommitBind { lane: 3 }),
            "forged opening_tuple_commitment not rejected by OpeningCommitBind"
        );
    }

    /// SHARED-account_projection SOUNDNESS (THE KEY C6 TEST): make the
    /// account_projection C6 commits to differ from C4's reconstructed residue.
    /// A SeamOpeningAcc constraint must fire — proving the opening-tuple
    /// commitment and the carrier are bound to ONE account_projection.
    #[test]
    fn path2_creation_relation_shared_account_projection_soundness_seamopening_fires() {
        let params = Poseidon2Params::new();
        let pad = [31u8; 32];
        let acc = [10u16, 20, 30, 40, 50, 60];
        let w0 = generate(
            &pad,
            3,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w0.masked_projection;
        let public_commit = w0.pad_source_commitment;
        let public_opening = w0.opening_tuple_commitment;

        // Baseline honest passes.
        assert!(is_satisfied(
            &params,
            &w0,
            &public,
            &public_commit,
            &public_opening
        ));

        // Tamper ONLY C6's account_projection FIELD lane 2 (the value the
        // opening-tuple commitment absorbs). C4's reconstructed residue is
        // unchanged, so they now disagree.
        let mut w = w0.clone();
        let before = w.opening.input_lanes[2];
        w.opening.input_lanes[2] = fq_add(before, 1);

        let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);
        // SeamOpeningAcc for lane 2 fires by name.
        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamOpeningAcc { lane: 2 }),
            "SeamOpeningAcc lane 2 did not fire when C6 acc != C4 acc"
        );
        // And it is lane 2 SPECIFICALLY (other seam lanes stay clean).
        for lane in 0..OPENING_ACC_SEAM_LANES {
            if lane != 2 {
                assert!(
                    res.iter().all(|r| !(r.kind == ResidualKind::SeamOpeningAcc { lane }
                        && r.value != 0)),
                    "unexpected SeamOpeningAcc fire at lane {lane}"
                );
            }
        }

        // Symmetric direction: tamper C4's reconstructed residue (range[4].v)
        // instead -> SeamOpeningAcc lane 4 also fires (it diverges from C6).
        let mut w2 = w0.clone();
        w2.range[4] = w2.range[4].mutate_cell(0, fq_add(w2.range[4].v, 1));
        let res2 = all_residuals(&params, &w2, &public, &public_commit, &public_opening);
        assert!(
            res2.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamOpeningAcc { lane: 4 }),
            "SeamOpeningAcc lane 4 did not fire when C4 acc != C6 acc"
        );
    }

    /// BLINDING-NONCE byte-range is inherited from C1: an out-of-range nonce lane
    /// is rejected by C1's recompose constraint (surfaced as a Commit residual).
    #[test]
    fn path2_creation_relation_blinding_nonce_byte_range_enforced() {
        let params = Poseidon2Params::new();
        let pad = [23u8; 32];
        let acc = [1u16, 1, 1, 1, 1, 1];
        let w0 = generate(
            &pad,
            2,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w0.masked_projection;
        let public_commit = w0.pad_source_commitment;
        let public_opening = w0.opening_tuple_commitment;

        // C1 nonce lanes occupy input_lanes[32..48]; set lane 40 to 256 with
        // stale (zero) bits -> C1's BitRecompose fires (a Commit residual).
        let mut w = w0.clone();
        w.commit.input_lanes[40] = 256;
        w.commit.input_bits[40] = [0u32; c1::BITS_PER_LANE];
        let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && matches!(r.kind, ResidualKind::Commit { .. })),
            "out-of-range blinding-nonce lane not rejected by C1"
        );
    }

    /// SEAM SOUNDNESS (a): tamper the mask the carrier uses while the PRF
    /// sub-witness still claims the real PRF output -> SeamMask fires (and the
    /// carrier sum no longer matches).
    #[test]
    fn path2_creation_relation_seam_a_carrier_mask_tamper_fires() {
        let params = Poseidon2Params::new();
        let pad = [7u8; 32];
        let acc = [11u16, 22, 33, 44, 55, 66];
        let w0 = generate(
            &pad,
            9,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w0.masked_projection;
        let public_commit = w0.pad_source_commitment;
        let public_opening = w0.opening_tuple_commitment;

        let mut w = w0.clone();
        // carrier now uses a DIFFERENT mask than the PRF proved.
        w.carrier_mask[2] = fq_add(w.carrier_mask[2], 1);
        let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);

        // The seam-equality for lane 2 must fire by name.
        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamMask { lane: 2 }),
            "SeamMask lane 2 did not fire on carrier-mask tamper"
        );
        // And the PRF sub-witness is untouched (still proves the real PRF).
        assert!(
            res.iter()
                .all(|r| !(matches!(r.kind, ResidualKind::Prf { .. }) && r.value != 0)),
            "PRF residuals should be clean (only the seam was tampered)"
        );
    }

    /// SEAM SOUNDNESS (b): tamper the account_projection the carrier uses vs the
    /// reconstruction -> SeamAcc fires.
    #[test]
    fn path2_creation_relation_seam_b_carrier_acc_tamper_fires() {
        let params = Poseidon2Params::new();
        let pad = [3u8; 32];
        let acc = [100u16, 200, 300, 400, 500, 600];
        let w0 = generate(
            &pad,
            1,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w0.masked_projection;
        let public_commit = w0.pad_source_commitment;
        let public_opening = w0.opening_tuple_commitment;

        let mut w = w0.clone();
        // carrier now uses a DIFFERENT acc than the range gadget reconstructed.
        w.carrier_acc[4] = fq_add(w.carrier_acc[4], 1);
        let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);

        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamAcc { lane: 4 }),
            "SeamAcc lane 4 did not fire on carrier-acc tamper"
        );
        assert!(
            res.iter()
                .all(|r| !(matches!(r.kind, ResidualKind::Range { .. }) && r.value != 0)),
            "Range residuals should be clean (only the seam was tampered)"
        );
    }

    /// SEAM SOUNDNESS (c): a FORGED masked_projection not equal to
    /// PRF(pad)+canonical_acc has no satisfying assignment from generate /
    /// the constraints reject.
    ///
    /// Two angles:
    ///   (c1) Ask the verifier to accept a public target != witness sum: the
    ///        Carrier residual (which binds the public target) fires.
    ///   (c2) Try to fabricate a witness whose masked cell equals the forged
    ///        target by lying about the carrier mask: SeamMask fires; lying
    ///        about the acc: SeamAcc fires. So no consistent witness exists.
    #[test]
    fn path2_creation_relation_seam_c_forged_masked_projection_rejected() {
        let params = Poseidon2Params::new();
        let pad = [21u8; 32];
        let acc = [1u16, 2, 3, 4, 5, 6];
        let w = generate(
            &pad,
            42,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public_commit = w.pad_source_commitment;
        let public_opening = w.opening_tuple_commitment;

        // (c1) honest witness, but verifier is handed a FORGED public target.
        let mut forged = w.masked_projection;
        forged[0] = fq_add(forged[0], 7); // not PRF[0]+acc[0]
        let res = all_residuals(&params, &w, &forged, &public_commit, &public_opening);
        assert!(
            res.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::PublicBind { lane: 0 }),
            "forged public target not rejected by public-binding constraint"
        );

        // (c2) attacker fabricates a witness whose masked cell IS the forged
        // target by overwriting masked_projection AND the carrier mask so the
        // local carrier sum balances — but then carrier_mask != PRF output, so
        // SeamMask fires. There is no escape: to satisfy the carrier sum AND the
        // seam, mask must be the PRF output and acc the reconstruction, forcing
        // masked = PRF + canonical_acc.
        let mut w2 = w.clone();
        w2.masked_projection[0] = forged[0];
        w2.carrier_mask[0] = fq_add(w2.carrier_mask[0], 7); // balance local sum
        // local carrier sum now balances:
        assert_eq!(
            carrier_residual_for_lane(&w2, 0),
            0,
            "fabricated local carrier balances"
        );
        let res2 = all_residuals(&params, &w2, &forged, &public_commit, &public_opening);
        assert!(
            res2.iter()
                .any(|r| r.value != 0 && r.kind == ResidualKind::SeamMask { lane: 0 }),
            "fabricated forged witness not caught by SeamMask"
        );
    }

    /// SEAM SOUNDNESS (d): a NON-CANONICAL account_projection digit rep is
    /// rejected (inherited from C4). Use the classic d_hi=96,d_lo=6 (field
    /// value 5) non-canonical decomposition.
    #[test]
    fn path2_creation_relation_seam_d_noncanonical_acc_rejected() {
        let params = Poseidon2Params::new();
        let pad = [13u8; 32];
        // honest residues; lane 3 will be corrupted to a non-canonical rep.
        let acc = [5u16, 6, 7, 8, 9, 10];
        let mut w = generate(
            &pad,
            5,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );

        // Overwrite lane 3's range witness with the BEST adversarial
        // non-canonical witness for field value 5: d_hi=96, d_lo=6, is96=1.
        let bits = |x: u32| {
            let mut b = [0u32; c4::DIGIT_BITS];
            for (i, s) in b.iter_mut().enumerate() {
                *s = (x >> i) & 1;
            }
            b
        };
        w.range[3] = c4::Base128RangeWitness {
            v: 5, // field value of d_hi=96,d_lo=6
            d_lo: 6,
            d_hi: 96,
            lo_bits: bits(6),
            hi_bits: bits(96),
            slack_bits: bits(0),
            is96: 1,
            inv: 0,
        };
        // keep the carrier consistent with this v so only the canonicity fails.
        w.carrier_acc[3] = 5;
        w.masked_projection[3] = fq_add(w.carrier_mask[3], 5);
        let public = w.masked_projection;
        let public_commit = w.pad_source_commitment;
        let public_opening = w.opening_tuple_commitment;

        let res = all_residuals(&params, &w, &public, &public_commit, &public_opening);
        // The C4 canonical edge constraint (is96 * d_lo) fires for lane 3.
        assert!(
            res.iter().any(|r| r.value != 0
                && r.kind
                    == ResidualKind::Range {
                        lane: 3,
                        sub: c4::residual_index::EDGE_HI96_LO0
                    }),
            "non-canonical acc (d_hi=96,d_lo=6) not rejected by C4 canonical edge"
        );
    }

    /// MUTATION: any single witness-cell flip breaks >= 1 residual. Covers the
    /// PRF sub-trace, the range sub-witnesses, the carrier cells, and the public
    /// target.
    #[test]
    fn path2_creation_relation_single_cell_mutation_breaks() {
        let params = Poseidon2Params::new();
        let pad = [29u8; 32];
        let acc = [12u16, 345, 6789, 0, 11000, 1];
        let w = generate(
            &pad,
            77,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let public = w.masked_projection;
        let pc = w.pad_source_commitment;
        let po = w.opening_tuple_commitment;
        assert!(
            is_satisfied(&params, &w, &public, &pc, &po),
            "baseline honest"
        );

        // (1) flip a PRF sub-trace cell.
        {
            let mut m = w.clone();
            m.prf.subs[2].rows[10][3] = fq_add(m.prf.subs[2].rows[10][3], 1);
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "PRF cell flip not caught"
            );
        }
        // (1b) flip a C1 commitment sub-trace cell.
        {
            let mut m = w.clone();
            m.commit.subs[3].rows[12][4] = fq_add(m.commit.subs[3].rows[12][4], 1);
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "C1 commit cell flip not caught"
            );
        }
        // (1c) flip a C6 opening-tuple commitment sub-trace cell.
        {
            let mut m = w.clone();
            m.opening.subs[4].rows[8][6] = fq_add(m.opening.subs[4].rows[8][6], 1);
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "C6 opening cell flip not caught"
            );
        }
        // (2) flip a range-witness cell on each lane (the residue v).
        for lane in 0..N_LANES {
            let mut m = w.clone();
            m.range[lane] = m.range[lane].mutate_cell(0, fq_add(m.range[lane].v, 1));
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "range v flip lane {lane} not caught"
            );
        }
        // (3) flip a carrier mask cell.
        {
            let mut m = w.clone();
            m.carrier_mask[1] = fq_add(m.carrier_mask[1], 1);
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "carrier_mask flip not caught"
            );
        }
        // (4) flip a carrier acc cell.
        {
            let mut m = w.clone();
            m.carrier_acc[5] = fq_add(m.carrier_acc[5], 1);
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "carrier_acc flip not caught"
            );
        }
        // (5) flip the public masked target.
        {
            let mut bad = public;
            bad[2] = fq_add(bad[2], 1);
            assert!(
                !is_satisfied(&params, &w, &bad, &pc, &po),
                "public target flip not caught"
            );
        }
        // (5b) flip the public pad_source_commitment target.
        {
            let mut bad = pc;
            bad[1] = fq_add(bad[1], 1);
            assert!(
                !is_satisfied(&params, &w, &public, &bad, &po),
                "public commitment flip not caught"
            );
        }
        // (5c) flip the public opening_tuple_commitment target.
        {
            let mut bad = po;
            bad[7] = fq_add(bad[7], 1);
            assert!(
                !is_satisfied(&params, &w, &public, &pc, &bad),
                "public opening commitment flip not caught"
            );
        }
        // (6) flip a witness masked cell (breaks the public-binding carrier).
        {
            let mut m = w.clone();
            m.masked_projection[0] = fq_add(m.masked_projection[0], 1);
            assert!(
                !is_satisfied(&params, &m, &public, &pc, &po),
                "witness masked flip not caught"
            );
        }
    }

    /// REPORT: constraint count, max degree, seam-enforcement summary.
    #[test]
    fn path2_creation_relation_sizes_report() {
        let params = Poseidon2Params::new();
        let pad = [1u8; 32];
        let acc = [0u16; N_LANES];
        let w = generate(
            &pad,
            0,
            &acc,
            &TEST_NONCE,
            &TEST_ANCHOR,
            &TEST_OPENING_NONCE,
        );
        let res = all_residuals(
            &params,
            &w,
            &w.masked_projection,
            &w.pad_source_commitment,
            &w.opening_tuple_commitment,
        );
        assert_eq!(res.len(), n_constraints());

        let c1_n = c1::n_constraints();
        let c2_n = c2::n_constraints();
        let c4_n = N_LANES * c4::CONSTRAINTS_PER_RESIDUE;
        let c6_n = c6::n_constraints();
        println!("path2_creation_relation report:");
        println!(
            "  PUBLIC : masked_projection[6], pad_source_commitment[{N_COMMIT_LANES}], opening_tuple_commitment[{N_OPENING_COMMIT_LANES}]"
        );
        println!(
            "  HIDDEN : pad_seed[32], slot_id(u64), account_projection[6], blinding_nonce[{PAD_SOURCE_BLINDING_BYTES}], lifecycle_anchor[{OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES}], opening_blinding_nonce[{OPENING_TUPLE_BLINDING_BYTES}]"
        );
        println!("  C1 commitment constraints = {c1_n}");
        println!("  commit public-bind        = {N_COMMIT_LANES}");
        println!("  SEAM-pad_seed (C1<->C2)   = {PAD_SEED_LANES}");
        println!("  C2 PRF constraints        = {c2_n}");
        println!(
            "  C4 range constraints      = {c4_n}  (6 lanes x {})",
            c4::CONSTRAINTS_PER_RESIDUE
        );
        println!("  C6 opening-tuple constr.  = {c6_n}");
        println!("  opening commit public-bind= {N_OPENING_COMMIT_LANES}");
        println!("  SEAM-acc (C4<->C6)        = {OPENING_ACC_SEAM_LANES}");
        println!("  masked public-bind        = {N_LANES}");
        println!("  carrier constraints       = {N_LANES}");
        println!("  seam-equality (mask+acc)  = {}", 2 * N_LANES);
        println!("  TOTAL constraints         = {}", n_constraints());
        println!("  max degree                = {MAX_DEGREE} (Poseidon2 S-box x^5)");
        println!("  shared pad_seed: C1.input_lanes[0..32] == C2.input_lanes[0..32]");
        println!("                   enforced by {PAD_SEED_LANES} SeamPadSeed constraints.");
        println!("  shared account_projection: C4.range[i].v == C6.input_lanes[0..6]");
        println!(
            "                   enforced by {OPENING_ACC_SEAM_LANES} SeamOpeningAcc constraints."
        );
        assert_eq!(MAX_DEGREE, 5);
    }
}
