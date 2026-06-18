//! The FULL account-creation relation (C1 ∧ C2 ∧ C4 ∧ C6 + cross-component
//! seams) as a SINGLE field-generic Plonky3 AIR over a 31-bit STARK field.
//!
//! Moved verbatim from the re-host benchmark's `full_air.rs`; the only change is
//! that the shared WIDE-THEN-REDUCE mask gadget + small AIR helpers
//! (`eval_reduce_gadget`, `fill_reduce_gadget`, `wide_reduce_masks`,
//! `perm_num_cols`, `ce`, and the `NUM_SQUEEZE`/`RATE`/`REDUCE_CELLS` consts)
//! now come from the local `reduce_gadget` module instead of the benchmark's
//! standalone `c2_air` (the C2-only standalone AIR is not part of the production
//! artifact).
//!
//! # The relation
//!
//! * C1 — `pad_source_commitment = Poseidon2-commit(pad_seed[32] ‖ nonce_c1[16])`
//!   absorbed as byte-lanes, squeezed to `N_COMMIT_LANES` 31-bit outputs (public).
//! * C2 — `masks[6] = Poseidon2-PRF(pad_seed[32] ‖ slot[8])` reduced mod q12289
//!   via the WIDE-THEN-REDUCE gadget (the 6 masks feed the carrier, NOT public).
//! * C6 — `opening_tuple_commitment =
//!   Poseidon2-commit(account_projection[6] field-lanes ‖ lifecycle_anchor[32] ‖
//!   nonce_c6[16])`, squeezed to `N_COMMIT_LANES` outputs (public).
//! * C4 — `masked_projection[6] = account_projection[6] + masks[6]` (carrier,
//!   mod q12289) with `account_projection` constrained to a CANONICAL base-128
//!   representation (`q = 96*128+1` canonicity gadget).
//!
//! # Seams (the bindings) — all SAME-ROW column equalities
//!   * `SeamPadSeed`   : C1's 32 pad_seed lanes == C2's 32 pad_seed lanes.
//!   * `SeamOpeningAcc`: C4's 6 account_projection values == C6's 6 field lanes.
//!   * `SeamMask`      : C2's 6 reduced masks == the 6 masks used in the carrier.

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::marker::PhantomData;
use core::mem::MaybeUninit;

use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField, PrimeField32, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_poseidon2::GenericPoseidon2LinearLayers;
use p3_poseidon2_air::{generate_trace_rows_for_perm, Poseidon2Cols, RoundConstants};

use crate::reduce_gadget::{
    ce, eval_reduce_gadget, fill_reduce_gadget, perm_num_cols, wide_reduce_masks, NUM_SQUEEZE, RATE,
    REDUCE_CELLS,
};

// ===========================================================================
// Relation-wide constants
// ===========================================================================

/// The Poseidon2 permutation width used throughout (KoalaBear-16). The digest
/// chain carries a state of this width across rows.
pub const PERM_WIDTH: usize = 16;

/// Projection lanes (masks / account_projection / masked_projection).
pub const N_LANES: usize = 6;
/// Commitment output lanes (C1 / C6). 10 lanes * 31 bits ≈ 310 bits ⇒ ≥128-bit
/// collision margin.
pub const N_COMMIT_LANES: usize = 10;

/// C1 input byte-lanes: pad_seed[32] ‖ blinding_nonce_c1[16].
pub const C1_INPUT_LANES: usize = 32 + 16;
/// C2 input byte-lanes: pad_seed[32] ‖ slot_id[8].
pub const C2_INPUT_LANES: usize = 32 + 8;
/// C6 input lanes: account_projection[6] (FIELD lanes) ‖ lifecycle_anchor[32] ‖
/// blinding_nonce_c6[16] ‖ account_id[32] (byte-lanes). The trailing
/// `account_id` block is the opening-tuple-account-binding (added 2026-06-18 to
/// kill the context-free universal-admission-token finding).
pub const C6_FIELD_LANES: usize = N_LANES;
pub const C6_BYTE_LANES: usize = 32 + 16 + 32;
#[allow(dead_code)]
pub const C6_INPUT_LANES: usize = C6_FIELD_LANES + C6_BYTE_LANES;

// --- Public-context byte-lengths (the public statement the verifier binds) ---
/// `account_id` byte length (embedded as 32 byte-valued field elements).
pub const ACCOUNT_ID_BYTES: usize = 32;
/// `slot_id` little-endian byte length (8 bytes, matches the C2 PRF slot lanes).
pub const SLOT_ID_BYTES: usize = 8;
/// `verifier_version` little-endian byte length (u32 -> 4 byte-valued elements).
pub const VERIFIER_VERSION_BYTES: usize = 4;
/// `account_state_root` byte length (embedded as 32 byte-valued field elements).
pub const STATE_ROOT_BYTES: usize = 32;
/// `lifecycle_anchor` byte length (embedded as 32 byte-valued field elements).
pub const LIFECYCLE_BYTES: usize = 32;

/// Number of PUBLIC-CONTEXT field elements appended to the public statement.
/// Layout (in order): account_id[32] ‖ slot_id[8] ‖ verifier_version[4] ‖
/// account_state_root[32] ‖ lifecycle_anchor[32].
pub const CONTEXT_ELEMS: usize =
    ACCOUNT_ID_BYTES + SLOT_ID_BYTES + VERIFIER_VERSION_BYTES + STATE_ROOT_BYTES + LIFECYCLE_BYTES;

/// Offsets of each context block WITHIN the context region (relative).
pub const CTX_ACCOUNT_ID_OFF: usize = 0;
pub const CTX_SLOT_OFF: usize = CTX_ACCOUNT_ID_OFF + ACCOUNT_ID_BYTES;
pub const CTX_VERIFIER_VERSION_OFF: usize = CTX_SLOT_OFF + SLOT_ID_BYTES;
pub const CTX_STATE_ROOT_OFF: usize = CTX_VERIFIER_VERSION_OFF + VERIFIER_VERSION_BYTES;
pub const CTX_LIFECYCLE_OFF: usize = CTX_STATE_ROOT_OFF + STATE_ROOT_BYTES;

// ---------------------------------------------------------------------------
// CLASS A — ROLE-RESTRICTION (role-root anchor).
//
// The proof's role commitments are folded, IN-AIR via the pinned Poseidon2, into
// a single `role_root` digest and exposed as a PUBLIC (transcript-bound) value:
//
//   role_root = Poseidon2-sponge( pad_source_commitment[10]
//                               ‖ opening_tuple_commitment[10]
//                               ‖ masked_projection[6] )   -> ROLE_ROOT_ELEMS lanes
//
// `masked_projection` is the packed-witness handle present in the trace (the
// only public projection handle), so the root binds C1's source commitment,
// C6's opening-tuple commitment AND the masked-projection handle under ONE root.
// The sponge INPUT lanes are constrained EQUAL to the actual commitment columns
// (so the root cannot be computed over anything but the proven commitments), and
// the squeezed root is constrained EQUAL to the public `role_root`.
// ---------------------------------------------------------------------------
/// Number of role-handle input lanes folded into `role_root`:
/// pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖ masked_projection[6].
pub const ROLE_HANDLE_LANES: usize = N_COMMIT_LANES + N_COMMIT_LANES + N_LANES; // 26
/// Number of squeezed `role_root` field elements (one Poseidon2 digest width).
pub const ROLE_ROOT_ELEMS: usize = 8;

// ---------------------------------------------------------------------------
// CLASS B — PROOF-SYSTEM-BOOKKEEPING (relation_id / proof_system_id).
//
// A pinned constant identifying THIS relation version + column profile + query
// schedule. Exposed as a PUBLIC value, constrained IN-AIR to equal the pinned
// constant, and re-checked by the verifier. A proof for this relation cannot be
// confused with a proof for a different relation / version: the value is part of
// the public statement (FS-transcript-bound), is pinned by an AIR equality, and
// is checked field-wise by the library verifier.
//
// The constant digits encode: relation tag "DESCv1" + the structural profile
// (trace width family, the two new classes, the pinned FRI schedule 56q/blowup4).
// Treat as an OPAQUE pinned id; bump it whenever the relation/profile changes.
// ---------------------------------------------------------------------------
/// Number of pinned `relation_id` field elements.
pub const RELATION_ID_ELEMS: usize = 4;
/// The pinned `relation_id` (proof-system-bookkeeping tag). Bump on any change to
/// the relation version / column profile / query schedule.
pub const RELATION_ID: [u32; RELATION_ID_ELEMS] = [
    0x4445_5343, // "DESC"
    0x7631_0000 | (PUBIO_ELEMS as u32 & 0xFFFF), // "v1" ‖ pubio-profile width
    (ROLE_ROOT_ELEMS as u32) << 16 | (RELATION_ID_ELEMS as u32), // class-A/B sizes
    (56u32 << 8) | 4u32, // pinned FRI schedule: 56 queries, log_blowup 4
];

const BYTE_BITS: usize = 8;

// ---------------------------------------------------------------------------
// CLASS C — CHECKED-CORE-RESIDUE-COMPATIBILITY (the 8th / final mandatory class).
//
// Binds the relation's `masked_projection` to the spend's canonical 18-residue
// `packed_witness_commitment` frame, so the creation proof commits to EXACTLY the
// packed witness the spend verifier consumes.
//
// The 18-residue layout MIRRORS the deployed
// `pack_reviewed_fq12_packed_witness_state18_from_parts`
// (tools/parameter-search/src/production_reference.rs):
//   * residues 0..11  = checked-core SYNDROME lanes (12 lanes; PUBLIC statement
//     inputs here — they come from the spend's checked-core frame).
//   * residues 12..17 = the 6 `masked_projection` lanes (ALREADY in the relation
//     from the C4 carrier mod-q reduction; REUSED, not recomputed).
// Each residue is a canonical q12289 value packed as 14-bit LITTLE-ENDIAN, residue
// `idx` occupying bit positions `[idx*14, idx*14+14)` of a 252-bit stream
// (18*14 = 252), which is byte-packed LE into the `packed_witness_commitment`
// (32-byte / 256-bit; the top 4 bits 252..256 are zero). This is byte-for-bit
// identical to the deployed `write_bits_le(out, idx*FIELD_BITS, FIELD_BITS, value)`
// with `FIELD_BITS = 14`, `DIGEST_BYTES = 32`, so the in-AIR anchor matches the
// spend leaf frame.
//
// ADDED to the AIR (single mode: public inputs; batch mode: per-row pubio cols so
// the digest chain covers them):
//   * 12 `syndrome` PUBLIC residues, each range-checked `< q12289` (14-bit decomp
//     + canonical-slack gadget, so a non-canonical syndrome `>= q` rejects).
//   * 32 `packed_witness_commitment` PUBLIC byte anchors, each constrained in-AIR
//     to equal the corresponding 8-bit LE slice of the 252-bit residue stream
//     (syndrome bits ‖ masked_projection bits). The masked_projection lanes are
//     already canonical (carrier mod-q), so only their 14-bit decomposition is
//     bound into the stream (no extra `< q` check needed).
// ---------------------------------------------------------------------------
/// Checked-core syndrome lanes (residues 0..11 of the packed witness frame).
/// Mirrors the deployed `REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS = 12`.
pub const SYNDROME_LANES: usize = 12;
/// Total packed-witness residues: syndrome[12] ‖ masked_projection[6].
/// Mirrors the deployed `REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS = 18`.
pub const PACKED_WITNESS_RESIDUES: usize = SYNDROME_LANES + N_LANES; // 18
/// Bits per residue lane (14-bit LE). Mirrors the deployed `FIELD_BITS = 14`.
pub const PACKED_LANE_BITS: usize = MODQ_BITS; // 14
/// Total residue-stream bits (18 * 14 = 252).
pub const PACKED_STREAM_BITS: usize = PACKED_WITNESS_RESIDUES * PACKED_LANE_BITS;
/// `packed_witness_commitment` byte anchors. Mirrors the deployed
/// `DIGEST_BYTES = 32` (252 packed bits + 4 zero high bits).
pub const PACKED_WITNESS_COMMITMENT_BYTES: usize = 32;
/// q12289 modulus the syndrome/projection residues are canonical against.
const Q12289_PACK: u32 = Q12289_CARRIER;

/// CLASS-C public field elements: syndrome[12] ‖ packed_witness_commitment[32].
pub const CHECKED_CORE_ELEMS: usize = SYNDROME_LANES + PACKED_WITNESS_COMMITMENT_BYTES;

// --- C4 canonical base-128 (mirrors base128_range_air_q12289) ---
const C4_BASE: u32 = 128;
const C4_DIGIT_BITS: usize = 7;
const C4_MAX_HI: u32 = 96; // q = 96*128 + 1 ⇒ canonical d_hi in [0,96]

// --- C4 carrier mod-q reduction (masked_projection = (acc + mask) mod q12289) ---
/// q12289 used by the masked-projection carrier (matches the spend's q12289 domain).
const Q12289_CARRIER: u32 = 12289;
/// Bits to range-check a canonical [0,q) value (14 bits cover [0,16384) ⊇ [0,q)).
const MODQ_BITS: usize = 14;
/// Cells per lane in the carrier mod-q reduction block:
/// masked_canonical[1] ‖ masked_bits[14] ‖ slack[1] ‖ slack_bits[14] ‖ wrap[1].
const MODQ_CELLS: usize = 1 + MODQ_BITS + 1 + MODQ_BITS + 1; // 31

/// CLASS C — cells per checked-core syndrome lane in the witness block:
/// value[1] ‖ bits[14] ‖ slack[1] ‖ slack_bits[14]. The value+bits give the
/// 14-bit decomposition fed into the packed-witness stream; value+slack enforce
/// the canonical `< q12289` range (value + slack == q-1, slack ∈ [0,q)).
const SYNDROME_CELLS: usize = 1 + MODQ_BITS + 1 + MODQ_BITS; // 30

/// A static description of one sponge: how many input lanes it absorbs, whether
/// each lane is byte-ranged, and how many outputs it squeezes.
#[derive(Clone)]
pub struct SpongePlan {
    pub n_input: usize,
    pub byte_ranged: Vec<bool>,
    pub n_out: usize,
    pub n_absorb: usize,
    pub n_perms: usize,
}

impl SpongePlan {
    pub fn new(byte_ranged: Vec<bool>, n_out: usize) -> Self {
        let n_input = byte_ranged.len();
        let padded = {
            let mut len = n_input + 1;
            while len % RATE != 0 {
                len += 1;
            }
            len
        };
        let n_absorb = padded / RATE;
        let mut remaining = n_out;
        let mut n_squeeze = 0usize;
        loop {
            let take = remaining.min(RATE);
            remaining -= take;
            if remaining == 0 {
                break;
            }
            n_squeeze += 1;
        }
        SpongePlan {
            n_input,
            byte_ranged,
            n_out,
            n_absorb,
            n_perms: n_absorb + n_squeeze,
        }
    }

    fn padded_len(&self) -> usize {
        self.n_absorb * RATE
    }

    fn output_source(&self, out: usize) -> (usize, usize) {
        let mut filled = 0usize;
        let mut read_perm = self.n_absorb - 1;
        let mut remaining = self.n_out;
        loop {
            let take = remaining.min(RATE);
            for i in 0..take {
                if filled == out {
                    return (read_perm, i);
                }
                filled += 1;
            }
            remaining -= take;
            if remaining == 0 {
                break;
            }
            read_perm += 1;
        }
        unreachable!("output index out of range")
    }
}

// ===========================================================================
// Column layout
// ===========================================================================

const C4_CELLS: usize = 3 + 3 * C4_DIGIT_BITS + 2; // 26

#[allow(dead_code)]
struct SpongeLayout {
    lanes: usize,
    lane_bits: usize,
    reduce: usize,
    with_reduction: bool,
    perms: usize,
    width: usize,
}

impl SpongeLayout {
    fn new(plan: &SpongePlan, with_reduction: bool, perm_cols: usize) -> Self {
        let n_byte = plan.byte_ranged.iter().filter(|&&b| b).count();
        let lanes = 0;
        let lane_bits = lanes + plan.n_input;
        let reduce = lane_bits + n_byte * BYTE_BITS;
        let perms = if with_reduction {
            reduce + REDUCE_CELLS
        } else {
            reduce
        };
        let width = perms + plan.n_perms * perm_cols;
        SpongeLayout {
            lanes,
            lane_bits,
            reduce,
            with_reduction,
            perms,
            width,
        }
    }
}

/// Full per-row layout: block offsets + total width.
pub struct FullLayout {
    pub c1_plan: SpongePlan,
    pub c2_plan: SpongePlan,
    pub c6_plan: SpongePlan,
    /// CLASS A — role-root sponge over the role-handle commitment lanes.
    pub role_plan: SpongePlan,
    pub perm_cols: usize,
    c1: SpongeLayout,
    c2: SpongeLayout,
    c6: SpongeLayout,
    role: SpongeLayout,
    pub c1_off: usize,
    pub c2_off: usize,
    pub c6_off: usize,
    /// CLASS A — offset of the role-root sponge block.
    pub role_off: usize,
    pub c4_off: usize,
    pub carry_masked_off: usize,
    pub carry_acc_off: usize,
    /// Carrier mod-q reduction witness block (per lane: bits/slack/wrap).
    pub carry_modq_off: usize,
    /// CLASS C — checked-core syndrome witness block (per lane:
    /// value[1] ‖ bits[14] ‖ slack[1] ‖ slack_bits[14] for the `< q` canonicity).
    pub syndrome_off: usize,
    /// Offset of the per-row PUBLIC-CONTEXT column block (batch mode):
    /// account_id[32] ‖ slot[8] ‖ verifier_version[4] ‖ state_root[32] ‖ lifecycle[32].
    pub ctx_off: usize,
    /// Offset of the per-row PUBLIC-IO column block (batch mode):
    /// pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖ masked[6] ‖ context.
    pub pubio_off: usize,
    // --- BATCH DIGEST CHAIN columns (batch mode only; present but unused/zero
    //     in single mode so the width is identical across modes) ---
    /// Incoming 16-wide running sponge state for this row's chain step.
    pub chain_in_off: usize,
    /// `CHAIN_PERMS_PER_ROW` Poseidon2 perm blocks absorbing this row's pubio.
    pub chain_perms_off: usize,
    /// Outgoing 16-wide running sponge state after this row's absorbs.
    pub chain_out_off: usize,
    /// Per-row enable selector (1 = real account, 0 = padding no-op).
    pub enable_off: usize,
    /// Running count accumulator (sum of enables up to and including this row).
    pub count_acc_off: usize,
    /// The CHAIN_RATE-wide chain-state width (= permutation WIDTH).
    pub chain_width: usize,
    pub width: usize,
}

/// Number of per-row public-IO field elements (the per-instance "statement"),
/// INCLUDING the public-context binding region, the CLASS-A `role_root` and the
/// CLASS-B `relation_id`.
pub const PUBIO_ELEMS: usize = N_COMMIT_LANES
    + N_COMMIT_LANES
    + N_LANES
    + CONTEXT_ELEMS
    + ROLE_ROOT_ELEMS
    + RELATION_ID_ELEMS
    + CHECKED_CORE_ELEMS;

/// Offset (within one row's pubio block) of the `role_root` lanes.
pub const PUBIO_ROLE_ROOT_OFF: usize = 2 * N_COMMIT_LANES + N_LANES + CONTEXT_ELEMS;
/// Offset (within one row's pubio block) of the `relation_id` lanes.
pub const PUBIO_RELATION_ID_OFF: usize = PUBIO_ROLE_ROOT_OFF + ROLE_ROOT_ELEMS;
/// CLASS C — offset (within one row's pubio block) of the `syndrome` lanes.
pub const PUBIO_SYNDROME_OFF: usize = PUBIO_RELATION_ID_OFF + RELATION_ID_ELEMS;
/// CLASS C — offset (within one row's pubio block) of the
/// `packed_witness_commitment` byte anchors.
pub const PUBIO_PACKED_WITNESS_OFF: usize = PUBIO_SYNDROME_OFF + SYNDROME_LANES;

// ---------------------------------------------------------------------------
// BATCH DIGEST CHAIN (the per-row public-IO binding commitment).
//
// THE BREAK this closes: previously the batch verifier ran `p3_verify(proof,
// &[])` (EMPTY public values) and separately checked `digest == hash(tuples)`
// off-circuit — but NOTHING tied the digest to the proof. A proof attested
// only that "N valid rows exist", not "THESE N specific public IOs".
//
// THE FIX: an in-AIR running Poseidon2 *sponge* chain over every row's pubio.
// Each batch row ABSORBS its full PUBIO_ELEMS into a 16-wide state carried
// across rows via the two-row transition window; the last row's state[0..8]
// is EXPOSED as a STARK public value (the digest), so it is observed into the
// Fiat–Shamir transcript AND checked by the verifier, who recomputes the
// identical chain over the supplied tuples. Padding rows are no-ops (an
// `enable` selector) and the real count N is bound (absorbed as a length
// prefix + an in-AIR accumulator constrained to a public N).
//
// The chain is a SELF-CONSISTENT construction (NOT PaddingFreeSponge): the
// verifier's recompute (`crate::chain_digest`) mirrors this AIR exactly, so
// the in-circuit digest and the off-circuit recompute agree by construction.
// ---------------------------------------------------------------------------

/// Sponge rate for the digest chain (RATE elements absorbed per permutation).
pub const CHAIN_RATE: usize = RATE; // 8
/// Number of Poseidon2 permutations to absorb one row's pubio + the per-row
/// length/enable prefix slot. Each row absorbs `1 + PUBIO_ELEMS` field elems
/// (slot 0 carries the running count prefix on the chain), padded to RATE.
pub const CHAIN_ABSORB_LEN: usize = 1 + PUBIO_ELEMS;
/// Permutations per row to absorb `CHAIN_ABSORB_LEN` elems at `CHAIN_RATE`.
pub const CHAIN_PERMS_PER_ROW: usize = CHAIN_ABSORB_LEN.div_ceil(CHAIN_RATE);
/// Number of digest field elements exposed publicly (= Poseidon2 digest width).
pub const CHAIN_DIGEST_ELEMS: usize = 8;

/// Batch public-value layout: `digest[8] ‖ count_n[1]`.
pub const BATCH_PUBLIC_VALUES: usize = CHAIN_DIGEST_ELEMS + 1;

impl FullLayout {
    pub fn new(perm_cols: usize) -> Self {
        let c1_plan = SpongePlan::new(vec![true; C1_INPUT_LANES], N_COMMIT_LANES);
        let c2_plan = SpongePlan::new(vec![true; C2_INPUT_LANES], NUM_SQUEEZE);
        let mut c6_br = vec![false; C6_FIELD_LANES];
        c6_br.extend(vec![true; C6_BYTE_LANES]);
        let c6_plan = SpongePlan::new(c6_br, N_COMMIT_LANES);
        // CLASS A: role-root sponge over ROLE_HANDLE_LANES field lanes (no byte
        // range — the inputs are 31-bit commitment outputs, not bytes).
        let role_plan = SpongePlan::new(vec![false; ROLE_HANDLE_LANES], ROLE_ROOT_ELEMS);

        let c1 = SpongeLayout::new(&c1_plan, false, perm_cols);
        let c2 = SpongeLayout::new(&c2_plan, true, perm_cols);
        let c6 = SpongeLayout::new(&c6_plan, false, perm_cols);
        let role = SpongeLayout::new(&role_plan, false, perm_cols);

        let c1_off = 0;
        let c2_off = c1_off + c1.width;
        let c6_off = c2_off + c2.width;
        let role_off = c6_off + c6.width;
        let c4_off = role_off + role.width;
        let carry_masked_off = c4_off + N_LANES * C4_CELLS;
        let carry_acc_off = carry_masked_off + N_LANES;
        let carry_modq_off = carry_acc_off + N_LANES;
        let syndrome_off = carry_modq_off + N_LANES * MODQ_CELLS;
        let ctx_off = syndrome_off + SYNDROME_LANES * SYNDROME_CELLS;
        let pubio_off = ctx_off + CONTEXT_ELEMS;

        // Batch digest-chain columns (always allocated so the trace width is
        // identical in single and batch modes).
        let chain_width = PERM_WIDTH;
        let chain_in_off = pubio_off + PUBIO_ELEMS;
        let chain_perms_off = chain_in_off + chain_width;
        let chain_out_off = chain_perms_off + CHAIN_PERMS_PER_ROW * perm_cols;
        let enable_off = chain_out_off + chain_width;
        let count_acc_off = enable_off + 1;
        let width = count_acc_off + 1;

        FullLayout {
            c1_plan,
            c2_plan,
            c6_plan,
            role_plan,
            perm_cols,
            c1,
            c2,
            c6,
            role,
            c1_off,
            c2_off,
            c6_off,
            role_off,
            c4_off,
            carry_masked_off,
            carry_acc_off,
            carry_modq_off,
            syndrome_off,
            ctx_off,
            pubio_off,
            chain_in_off,
            chain_perms_off,
            chain_out_off,
            enable_off,
            count_acc_off,
            chain_width,
            width,
        }
    }

    #[allow(dead_code)]
    pub fn total_perms(&self) -> usize {
        self.c1_plan.n_perms + self.c2_plan.n_perms + self.c6_plan.n_perms
    }

    pub fn c1_lanes_off(&self) -> usize {
        self.c1.lanes
    }
    #[allow(dead_code)]
    pub fn c2_lanes_off(&self) -> usize {
        self.c2.lanes
    }
    #[allow(dead_code)]
    pub fn c6_lanes_off(&self) -> usize {
        self.c6.lanes
    }
}

// ===========================================================================
// The field-generic AIR
// ===========================================================================

pub struct FullRelationAir<
    F: PrimeCharacteristicRing,
    LinearLayers,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
> {
    constants: RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    partial_rc: [F; PARTIAL_ROUNDS],
    ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    layout: FullLayout,
    batch: bool,
    _phantom: PhantomData<LinearLayers>,
}

impl<
        F: PrimeField32,
        LinearLayers,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    >
    FullRelationAir<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
{
    #[allow(dead_code)]
    pub fn from_arrays(
        beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        partial_rc: [F; PARTIAL_ROUNDS],
        ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
    ) -> Self {
        Self::from_arrays_mode(beginning_full_rc, partial_rc, ending_full_rc, false)
    }

    pub fn from_arrays_mode(
        beginning_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        partial_rc: [F; PARTIAL_ROUNDS],
        ending_full_rc: [[F; WIDTH]; HALF_FULL_ROUNDS],
        batch: bool,
    ) -> Self {
        let constants = RoundConstants::new(beginning_full_rc, partial_rc, ending_full_rc);
        let perm_cols =
            perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
        Self {
            constants,
            beginning_full_rc,
            partial_rc,
            ending_full_rc,
            layout: FullLayout::new(perm_cols),
            batch,
            _phantom: PhantomData,
        }
    }

    pub fn constants(&self) -> &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS> {
        &self.constants
    }
    pub fn layout(&self) -> &FullLayout {
        &self.layout
    }
}

impl<
        F: PrimeCharacteristicRing + Sync,
        LinearLayers: Sync,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    > BaseAir<F>
    for FullRelationAir<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
{
    fn width(&self) -> usize {
        self.layout.width
    }

    fn main_next_row_columns(&self) -> Vec<usize> {
        if self.batch {
            // Batch mode reads the next row for the digest-chain carry
            // (next.chain_in == cur.chain_out, next.count_acc, next.enable).
            (0..self.width()).collect()
        } else {
            Vec::new()
        }
    }

    fn num_public_values(&self) -> usize {
        if self.batch {
            // digest[8] ‖ count_n[1] — the transcript-bound chain commitment.
            BATCH_PUBLIC_VALUES
        } else {
            // pad[10] ‖ opening[10] ‖ masked[6] ‖ context ‖ role_root ‖
            // relation_id ‖ syndrome[12] ‖ packed_witness_commitment[32].
            N_COMMIT_LANES
                + N_COMMIT_LANES
                + N_LANES
                + CONTEXT_ELEMS
                + ROLE_ROOT_ELEMS
                + RELATION_ID_ELEMS
                + CHECKED_CORE_ELEMS
        }
    }
}

fn eval_sbox_expr<AB: AirBuilder, const DEGREE: u64, const REGISTERS: usize>(
    builder: &mut AB,
    registers: &[AB::Var],
    x: AB::Expr,
) -> AB::Expr {
    match (DEGREE, REGISTERS) {
        (3, 0) => x.cube(),
        (5, 0) => x.exp_const_u64::<5>(),
        (7, 0) => x.exp_const_u64::<7>(),
        (5, 1) => {
            let committed_x3: AB::Expr = registers[0].into();
            let x2 = x.square();
            builder.assert_eq(committed_x3.clone(), x2.clone() * x.clone());
            committed_x3 * x2
        }
        (7, 1) => {
            let committed_x3: AB::Expr = registers[0].into();
            builder.assert_eq(committed_x3.clone(), x.cube());
            committed_x3.square() * x
        }
        (11, 2) => {
            let committed_x3: AB::Expr = registers[0].into();
            let committed_x9: AB::Expr = registers[1].into();
            let x2 = x.square();
            builder.assert_eq(committed_x3.clone(), x2.clone() * x.clone());
            builder.assert_eq(committed_x9.clone(), committed_x3.cube());
            committed_x9 * x2
        }
        _ => panic!("Unsupported (DEGREE, REGISTERS) = ({DEGREE}, {REGISTERS})"),
    }
}

impl<
        AB: AirBuilder,
        LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    > Air<AB>
    for FullRelationAir<
        AB::F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
where
    AB::F: PrimeField64,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.current_slice().to_vec();
        let next = main.next_slice().to_vec();
        let public = builder.public_values().to_vec();
        let is_first_row = builder.is_first_row();
        let is_last_row = builder.is_last_row();
        let is_transition = builder.is_transition();
        let lay = &self.layout;
        let batch = self.batch;

        // ----- C1 sponge -> pad_source_commitment[10] -----
        let c1_out = self.eval_sponge::<AB, LinearLayers>(
            builder,
            &local,
            lay.c1_off,
            &lay.c1,
            &lay.c1_plan,
        );
        for j in 0..N_COMMIT_LANES {
            if batch {
                builder.assert_eq(c1_out[j].clone(), local[lay.pubio_off + j].into());
            } else {
                builder.assert_eq(c1_out[j].clone(), public[j]);
            }
        }

        // ----- C2 sponge -> 30 squeezed lanes -> WIDE-THEN-REDUCE masks[6] -----
        let c2_out = self.eval_sponge::<AB, LinearLayers>(
            builder,
            &local,
            lay.c2_off,
            &lay.c2,
            &lay.c2_plan,
        );
        let c2_masks =
            eval_reduce_gadget::<AB>(builder, lay.c2_off + lay.c2.reduce, &local, &c2_out);

        // ----- C6 sponge -> opening_tuple_commitment[10] -----
        let c6_out = self.eval_sponge::<AB, LinearLayers>(
            builder,
            &local,
            lay.c6_off,
            &lay.c6,
            &lay.c6_plan,
        );
        for j in 0..N_COMMIT_LANES {
            if batch {
                builder.assert_eq(
                    c6_out[j].clone(),
                    local[lay.pubio_off + N_COMMIT_LANES + j].into(),
                );
            } else {
                builder.assert_eq(c6_out[j].clone(), public[N_COMMIT_LANES + j]);
            }
        }

        // ----- C4 canonical base-128 range, reconstructed acc into carry_acc -----
        for j in 0..N_LANES {
            let base = lay.c4_off + j * C4_CELLS;
            let v = self.eval_c4_residue::<AB>(builder, &local, base);
            builder.assert_eq(local[lay.carry_acc_off + j].into(), v.clone());
        }

        // ----- SEAM: pad_seed C1 == C2 (32 same-row lane equalities) -----
        for b in 0..32 {
            builder.assert_eq(
                local[lay.c1_off + lay.c1.lanes + b].into(),
                local[lay.c2_off + lay.c2.lanes + b].into(),
            );
        }

        // ----- SEAM: account_projection C4 == C6 (6 same-row equalities) -----
        for i in 0..N_LANES {
            builder.assert_eq(
                local[lay.carry_acc_off + i].into(),
                local[lay.c6_off + lay.c6.lanes + i].into(),
            );
        }

        // ===================================================================
        // CONTEXT BINDING (kills the context-free universal-admission-token).
        // The public-context region (in `public[..]` single, or the per-row
        // ctx columns in batch) is constrained EQUAL to the in-circuit witness
        // lanes that actually feed C2 / C6, so one proof is bound to exactly
        // one (account, slot, state, verifier, lifecycle).
        // ===================================================================
        let ctx_pub = |i: usize| -> AB::Expr {
            if batch {
                local[lay.ctx_off + i].into()
            } else {
                public[2 * N_COMMIT_LANES + N_LANES + i].clone().into()
            }
        };
        // In batch mode the ctx columns are also exposed as the per-row pubio
        // context block (so the digest covers them); bind ctx == pubio.
        if batch {
            for i in 0..CONTEXT_ELEMS {
                builder.assert_eq(
                    local[lay.ctx_off + i].into(),
                    local[lay.pubio_off + 2 * N_COMMIT_LANES + N_LANES + i].into(),
                );
            }
        }

        // (a) slot_id absorbed into the C2 PRF == public slot_id (in-circuit).
        //     The C2 slot byte-lanes are lane indices [32, 40) (after pad_seed[32]).
        for b in 0..SLOT_ID_BYTES {
            builder.assert_eq(
                local[lay.c2_off + lay.c2.lanes + 32 + b].into(),
                ctx_pub(CTX_SLOT_OFF + b),
            );
        }
        // (b) lifecycle_anchor absorbed into the C6 opening tuple == public (in-circuit).
        //     C6 lifecycle byte-lanes are lane indices [N_LANES, N_LANES+32).
        for b in 0..LIFECYCLE_BYTES {
            builder.assert_eq(
                local[lay.c6_off + lay.c6.lanes + C6_FIELD_LANES + b].into(),
                ctx_pub(CTX_LIFECYCLE_OFF + b),
            );
        }
        // (c) account_id folded into the C6 opening-tuple preimage == public
        //     (opening-tuple-account-binding). account_id byte-lanes sit AFTER
        //     account_projection[6] ‖ lifecycle[32] ‖ nonce_c6[16].
        let c6_account_id_lane0 = C6_FIELD_LANES + LIFECYCLE_BYTES + 16;
        for b in 0..ACCOUNT_ID_BYTES {
            builder.assert_eq(
                local[lay.c6_off + lay.c6.lanes + c6_account_id_lane0 + b].into(),
                ctx_pub(CTX_ACCOUNT_ID_OFF + b),
            );
        }
        // (d) account_state_root + verifier_version: pure-context values (no
        //     witness relation in C1..C6). They ARE part of the public statement
        //     and observed into the Fiat–Shamir transcript at BOTH prover and
        //     verifier. MED-1 hardening (2026-06-18 soundness audit): additionally
        //     pin the ctx columns to the public statement IN-CIRCUIT in single
        //     mode (batch already binds ctx==pubio via the digest chain), so the
        //     binding is enforced by the CONSTRAINT system too — defense-in-depth
        //     against a downstream verifier that mis-supplies public values (the
        //     shape of the prior empty-public-values break).
        if !batch {
            for b in 0..VERIFIER_VERSION_BYTES {
                builder.assert_eq(
                    local[lay.ctx_off + CTX_VERIFIER_VERSION_OFF + b].into(),
                    ctx_pub(CTX_VERIFIER_VERSION_OFF + b),
                );
            }
            for b in 0..STATE_ROOT_BYTES {
                builder.assert_eq(
                    local[lay.ctx_off + CTX_STATE_ROOT_OFF + b].into(),
                    ctx_pub(CTX_STATE_ROOT_OFF + b),
                );
            }
        }

        // ----- Carrier (+ SeamMask): masked = (mask(C2) + acc(C4)) mod q12289 -----
        // Canonical q-residue reduction so masked_projection ∈ [0,q), matching
        // the spend's q12289 domain (was native KoalaBear, no mod-q reduction).
        for j in 0..N_LANES {
            let mask = c2_masks[j].clone();
            let acc = local[lay.carry_acc_off + j];
            let masked = local[lay.carry_masked_off + j];
            let mb = lay.carry_modq_off + j * MODQ_CELLS;

            // masked_canonical value lives at carry_masked_off (also the public).
            // mod-q witness cells: [bits14] [slack] [slack_bits14] [wrap]
            let masked_bits_off = mb + 1;
            let slack_off = mb + 1 + MODQ_BITS;
            let slack_bits_off = slack_off + 1;
            let wrap_off = slack_bits_off + MODQ_BITS;

            let recompose = |builder: &mut AB, start: usize| -> AB::Expr {
                let mut a = AB::Expr::ZERO;
                for b in 0..MODQ_BITS {
                    let bit = local[start + b];
                    builder.assert_bool(bit);
                    a += bit.into() * ce::<AB>(1u64 << b);
                }
                a
            };
            // masked ∈ [0,q): masked == bits and masked + slack == q-1 (slack ∈ [0,q)).
            let masked_rc = recompose(builder, masked_bits_off);
            builder.assert_eq(masked.into(), masked_rc);
            let slack_rc = recompose(builder, slack_bits_off);
            builder.assert_eq(local[slack_off].into(), slack_rc);
            builder.assert_eq(
                masked.into() + local[slack_off].into(),
                ce::<AB>((Q12289_CARRIER - 1) as u64),
            );
            // wrap ∈ {0,1}; acc + mask = masked + wrap*q (acc,mask ∈ [0,q) ⇒ sum < 2q).
            let wrap = local[wrap_off];
            builder.assert_bool(wrap);
            builder.assert_eq(
                acc.into() + mask,
                masked.into() + wrap.into() * ce::<AB>(Q12289_CARRIER as u64),
            );

            if batch {
                builder.assert_eq(
                    masked.into(),
                    local[lay.pubio_off + 2 * N_COMMIT_LANES + j].into(),
                );
            } else {
                builder.assert_eq(masked.into(), public[2 * N_COMMIT_LANES + j]);
            }
        }

        // ===================================================================
        // CLASS A — ROLE-RESTRICTION (role-root anchor).
        // role_root = Poseidon2-sponge(pad_source_commitment ‖
        //   opening_tuple_commitment ‖ masked_projection). The sponge INPUT
        // lanes are constrained EQUAL to the actual commitment columns (so the
        // root is computed over exactly the proven role handles), and the
        // squeezed root is constrained EQUAL to the public `role_root`. This
        // ties the proof to a single named set of role handles under one root.
        // ===================================================================
        {
            // (i) bind the role sponge's input lanes to the real role handles.
            let role_lane = |i: usize| local[lay.role_off + lay.role.lanes + i];
            for j in 0..N_COMMIT_LANES {
                builder.assert_eq(role_lane(j).into(), c1_out[j].clone());
            }
            for j in 0..N_COMMIT_LANES {
                builder.assert_eq(role_lane(N_COMMIT_LANES + j).into(), c6_out[j].clone());
            }
            for j in 0..N_LANES {
                builder.assert_eq(
                    role_lane(2 * N_COMMIT_LANES + j).into(),
                    local[lay.carry_masked_off + j].into(),
                );
            }
            // (ii) run the role-root sponge over those (now-bound) input lanes.
            let role_out = self.eval_sponge::<AB, LinearLayers>(
                builder,
                &local,
                lay.role_off,
                &lay.role,
                &lay.role_plan,
            );
            // (iii) constrain the squeezed root == the public role_root.
            for j in 0..ROLE_ROOT_ELEMS {
                if batch {
                    builder.assert_eq(
                        role_out[j].clone(),
                        local[lay.pubio_off + PUBIO_ROLE_ROOT_OFF + j].into(),
                    );
                } else {
                    builder.assert_eq(
                        role_out[j].clone(),
                        public[2 * N_COMMIT_LANES + N_LANES + CONTEXT_ELEMS + j],
                    );
                }
            }
        }

        // ===================================================================
        // CLASS B — PROOF-SYSTEM-BOOKKEEPING (relation_id / proof_system_id).
        // The public relation_id is pinned IN-AIR to the constant RELATION_ID so
        // a proof for this relation/version/profile cannot be reinterpreted as a
        // proof for another. (The library verifier ALSO checks it field-wise.)
        // ===================================================================
        for j in 0..RELATION_ID_ELEMS {
            let pinned = ce::<AB>(RELATION_ID[j] as u64);
            if batch {
                builder.assert_eq(
                    local[lay.pubio_off + PUBIO_RELATION_ID_OFF + j].into(),
                    pinned,
                );
            } else {
                builder.assert_eq(
                    public[2 * N_COMMIT_LANES + N_LANES + CONTEXT_ELEMS + ROLE_ROOT_ELEMS + j]
                        .clone()
                        .into(),
                    pinned,
                );
            }
        }

        // ===================================================================
        // CLASS C — CHECKED-CORE-RESIDUE-COMPATIBILITY (8th / final class).
        //
        // Bind `masked_projection` (residues 12..17) together with the PUBLIC
        // checked-core `syndrome` (residues 0..11) to the PUBLIC
        // `packed_witness_commitment` anchor, using the SAME 14-bit-LE per-lane
        // packing as the deployed
        // `pack_reviewed_fq12_packed_witness_state18_from_parts`. So the creation
        // proof commits to EXACTLY the packed witness the spend verifier consumes.
        // ===================================================================
        {
            // Single-mode public-value offsets for the CLASS-C region.
            let cc_base =
                2 * N_COMMIT_LANES + N_LANES + CONTEXT_ELEMS + ROLE_ROOT_ELEMS + RELATION_ID_ELEMS;
            // Accessor for a CLASS-C public element (syndrome or commitment byte).
            let cc_pub = |i: usize| -> AB::Expr {
                if batch {
                    // syndrome[12] then packed_witness_commitment[32] in the pubio.
                    local[lay.pubio_off + PUBIO_SYNDROME_OFF + i].into()
                } else {
                    public[cc_base + i].clone().into()
                }
            };

            // The 252-bit residue stream as per-bit exprs. Residue idx occupies
            // bit positions [idx*14, idx*14+14); residues 0..11 = syndrome,
            // residues 12..17 = masked_projection (REUSE the carrier's bits).
            let mut stream: Vec<AB::Expr> = Vec::with_capacity(PACKED_STREAM_BITS);

            // ----- syndrome residues 0..11: PUBLIC, range-checked < q -----
            for lane in 0..SYNDROME_LANES {
                let sb = lay.syndrome_off + lane * SYNDROME_CELLS;
                let val_off = sb;
                let bits_off = sb + 1;
                let slack_off = sb + 1 + MODQ_BITS;
                let slack_bits_off = slack_off + 1;

                // value == public syndrome[lane].
                builder.assert_eq(local[val_off].into(), cc_pub(lane));

                // value == recompose(bits[14]); each bit boolean.
                let mut val_rc = AB::Expr::ZERO;
                for b in 0..MODQ_BITS {
                    let bit = local[bits_off + b];
                    builder.assert_bool(bit);
                    val_rc += bit.into() * ce::<AB>(1u64 << b);
                    stream.push(bit.into());
                }
                builder.assert_eq(local[val_off].into(), val_rc);

                // Canonical `< q12289`: value + slack == q-1, slack ∈ [0,q).
                let mut slack_rc = AB::Expr::ZERO;
                for b in 0..MODQ_BITS {
                    let bit = local[slack_bits_off + b];
                    builder.assert_bool(bit);
                    slack_rc += bit.into() * ce::<AB>(1u64 << b);
                }
                builder.assert_eq(local[slack_off].into(), slack_rc);
                builder.assert_eq(
                    local[val_off].into() + local[slack_off].into(),
                    ce::<AB>((Q12289_PACK - 1) as u64),
                );
            }

            // ----- masked_projection residues 12..17: REUSE carrier bits -----
            // The carrier already proved masked == recompose(masked_bits[14]) and
            // masked ∈ [0,q); we only need its 14 bits in the stream.
            for j in 0..N_LANES {
                let mb = lay.carry_modq_off + j * MODQ_CELLS;
                let masked_bits_off = mb + 1;
                for b in 0..MODQ_BITS {
                    // (already asserted boolean + recomposed in the carrier block.)
                    stream.push(local[masked_bits_off + b].into());
                }
            }
            debug_assert_eq!(stream.len(), PACKED_STREAM_BITS);

            // ----- packed_witness_commitment[32] byte anchors -----
            // Byte k == sum_{b<8} stream[8k+b] * 2^b (LE bit packing == the
            // deployed write_bits_le); high bits 252..256 are zero.
            for k in 0..PACKED_WITNESS_COMMITMENT_BYTES {
                let mut byte = AB::Expr::ZERO;
                for b in 0..BYTE_BITS {
                    let pos = k * BYTE_BITS + b;
                    if pos < PACKED_STREAM_BITS {
                        byte += stream[pos].clone() * ce::<AB>(1u64 << b);
                    }
                    // pos >= 252: contributes 0 (top 4 bits of byte 31).
                }
                builder.assert_eq(cc_pub(SYNDROME_LANES + k), byte);
            }
        }

        // ===================================================================
        // BATCH DIGEST CHAIN — the per-row public-IO binding (closes the
        // batch-aggregation break). See the module-level note on the chain.
        // ===================================================================
        if batch {
            self.eval_digest_chain::<AB, LinearLayers>(
                builder,
                &local,
                &next,
                is_first_row,
                is_last_row,
                is_transition,
                &public,
            );
        }
    }
}

impl<
        F: PrimeField,
        LinearLayers,
        const WIDTH: usize,
        const SBOX_DEGREE: u64,
        const SBOX_REGISTERS: usize,
        const HALF_FULL_ROUNDS: usize,
        const PARTIAL_ROUNDS: usize,
    >
    FullRelationAir<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >
{
    /// In-AIR batch digest chain over the per-row pubio (the binding that
    /// closes the batch-aggregation break).
    ///
    /// Per row r the absorbed message is `m = [enable, enable*pubio[0..]]`
    /// (length `CHAIN_ABSORB_LEN`, zero-padded to a multiple of CHAIN_RATE).
    /// The chain step computes `absorbed = Poseidon2-sponge(chain_in, m)` over
    /// `CHAIN_PERMS_PER_ROW` permutations, then sets
    /// `chain_out = enable ? absorbed : chain_in` (padding rows are TRUE
    /// no-ops, so the final state equals the state after the last real row).
    ///
    /// Boundary/transition bindings:
    ///   * first row: `chain_in == 0` (clean sponge seed).
    ///   * transition: `next.chain_in == cur.chain_out` (carry the state).
    ///   * `enable ∈ {0,1}`; enable is monotone non-increasing
    ///     (`next.enable*(1-cur.enable) == 0`) so the real rows are a prefix.
    ///   * count accumulator: first row `count_acc == enable`; transition
    ///     `next.count_acc == cur.count_acc + next.enable`.
    ///   * last row: `count_acc == public N` AND
    ///     `chain_out[0..8] == public digest[0..8]`.
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    fn eval_digest_chain<AB, LL>(
        &self,
        builder: &mut AB,
        local: &[AB::Var],
        next: &[AB::Var],
        is_first_row: AB::Expr,
        is_last_row: AB::Expr,
        is_transition: AB::Expr,
        public: &[AB::PublicVar],
    ) where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
        LL: GenericPoseidon2LinearLayers<WIDTH>,
    {
        let lay = &self.layout;
        let w = lay.chain_width; // PERM_WIDTH (16)
        let rate = CHAIN_RATE;

        let enable = local[lay.enable_off];
        builder.assert_bool(enable);

        // Build the per-row message exprs: m[0] = enable, m[1+i] = enable*pubio[i].
        let mut msg: Vec<AB::Expr> = Vec::with_capacity(CHAIN_ABSORB_LEN);
        msg.push(enable.into());
        for i in 0..PUBIO_ELEMS {
            msg.push(enable.into() * local[lay.pubio_off + i].into());
        }
        // Zero-pad to a whole number of RATE-sized blocks.
        while msg.len() < CHAIN_PERMS_PER_ROW * rate {
            msg.push(AB::Expr::ZERO);
        }

        // chain_in state exprs.
        let chain_in: Vec<AB::Expr> =
            (0..w).map(|c| local[lay.chain_in_off + c].into()).collect();

        // Absorb across CHAIN_PERMS_PER_ROW permutations seeded by chain_in.
        let mut prev_post: Option<[AB::Expr; WIDTH]> = None;
        for p in 0..CHAIN_PERMS_PER_ROW {
            let base = lay.chain_perms_off + p * lay.perm_cols;
            let block: Vec<AB::Var> = local[base..base + lay.perm_cols].to_vec();
            let pcols: &Poseidon2Cols<
                AB::Var,
                WIDTH,
                SBOX_DEGREE,
                SBOX_REGISTERS,
                HALF_FULL_ROUNDS,
                PARTIAL_ROUNDS,
            > = (block.as_slice()).borrow();

            // Constrain this permutation's inputs.
            match &prev_post {
                None => {
                    // First absorb: state = chain_in, add message chunk to rate lanes.
                    for r in 0..rate {
                        builder.assert_eq(
                            pcols.inputs[r].into(),
                            chain_in[r].clone() + msg[r].clone(),
                        );
                    }
                    for c in rate..w {
                        builder.assert_eq(pcols.inputs[c].into(), chain_in[c].clone());
                    }
                }
                Some(prev) => {
                    for r in 0..rate {
                        builder.assert_eq(
                            pcols.inputs[r].into(),
                            prev[r].clone() + msg[p * rate + r].clone(),
                        );
                    }
                    for c in rate..w {
                        builder.assert_eq(pcols.inputs[c].into(), prev[c].clone());
                    }
                }
            }

            let post = self.eval_perm_block::<AB, LL>(builder, pcols);
            prev_post = Some(post);
        }
        let absorbed = prev_post.expect("CHAIN_PERMS_PER_ROW >= 1");

        // chain_out = enable ? absorbed : chain_in  (padding rows are no-ops).
        for c in 0..w {
            let out = local[lay.chain_out_off + c];
            builder.assert_eq(
                out.into(),
                enable.into() * absorbed[c].clone()
                    + (AB::Expr::ONE - enable.into()) * chain_in[c].clone(),
            );
        }

        // First row: clean sponge seed (chain_in == 0) and count_acc == enable.
        {
            let mut wf = builder.when(is_first_row.clone());
            for c in 0..w {
                wf.assert_zero(local[lay.chain_in_off + c]);
            }
            wf.assert_eq(local[lay.count_acc_off].into(), enable.into());
        }

        // Transition: carry the state, accumulate the count, enforce monotone
        // enable (real rows form a prefix; padding cannot re-enable).
        {
            let next_enable = next[lay.enable_off];
            let mut wt = builder.when(is_transition.clone());
            for c in 0..w {
                wt.assert_eq(next[lay.chain_in_off + c].into(), local[lay.chain_out_off + c].into());
            }
            wt.assert_eq(
                next[lay.count_acc_off].into(),
                local[lay.count_acc_off].into() + next_enable.into(),
            );
            // next.enable * (1 - cur.enable) == 0  ⇒ no 0->1 transition.
            wt.assert_zero(next_enable.into() * (AB::Expr::ONE - enable.into()));
        }

        // Last row: the running count equals public N, and the running digest
        // (first 8 state lanes) equals the public digest. These are the only
        // public values for the batch AIR ⇒ both are observed into the FS
        // transcript and re-checked by the verifier.
        {
            let mut wl = builder.when(is_last_row.clone());
            for c in 0..CHAIN_DIGEST_ELEMS {
                wl.assert_eq(local[lay.chain_out_off + c].into(), public[c].clone().into());
            }
            wl.assert_eq(
                local[lay.count_acc_off].into(),
                public[CHAIN_DIGEST_ELEMS].clone().into(),
            );
        }
    }

    #[allow(clippy::type_complexity)]
    fn eval_sponge<AB, LL>(
        &self,
        builder: &mut AB,
        local: &[AB::Var],
        off: usize,
        slay: &SpongeLayout,
        plan: &SpongePlan,
    ) -> Vec<AB::Expr>
    where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
        LL: GenericPoseidon2LinearLayers<WIDTH>,
    {
        let lane = |i: usize| local[off + slay.lanes + i];
        let mut bit_cursor = 0usize;
        for i in 0..plan.n_input {
            if plan.byte_ranged[i] {
                let mut acc = AB::Expr::ZERO;
                for b in 0..BYTE_BITS {
                    let bit = local[off + slay.lane_bits + bit_cursor * BYTE_BITS + b];
                    builder.assert_bool(bit);
                    acc += bit.into() * ce::<AB>(1u64 << b);
                }
                builder.assert_eq(lane(i).into(), acc);
                bit_cursor += 1;
            }
        }

        let padded = plan.padded_len();
        let mut prev_post: Option<[AB::Expr; WIDTH]> = None;
        for p in 0..plan.n_perms {
            let base = off + slay.perms + p * self.layout.perm_cols;
            let block: Vec<AB::Var> = local[base..base + self.layout.perm_cols].to_vec();
            let pcols: &Poseidon2Cols<
                AB::Var,
                WIDTH,
                SBOX_DEGREE,
                SBOX_REGISTERS,
                HALF_FULL_ROUNDS,
                PARTIAL_ROUNDS,
            > = (block.as_slice()).borrow();

            let inputs: [AB::Expr; WIDTH] = core::array::from_fn(|c| pcols.inputs[c].into());
            let is_absorb = p < plan.n_absorb;
            match &prev_post {
                None => {
                    for r in 0..RATE {
                        let pos = r;
                        let elem = self.absorb_elem_expr::<AB>(local, off, slay, plan, pos);
                        builder.assert_eq(inputs[r].clone(), elem);
                    }
                    for c in RATE..WIDTH {
                        builder.assert_eq(inputs[c].clone(), AB::Expr::ZERO);
                    }
                }
                Some(prev) => {
                    for r in 0..RATE {
                        if is_absorb {
                            let pos = p * RATE + r;
                            let elem = if pos < padded {
                                self.absorb_elem_expr::<AB>(local, off, slay, plan, pos)
                            } else {
                                AB::Expr::ZERO
                            };
                            builder.assert_eq(inputs[r].clone(), prev[r].clone() + elem);
                        } else {
                            builder.assert_eq(inputs[r].clone(), prev[r].clone());
                        }
                    }
                    for c in RATE..WIDTH {
                        builder.assert_eq(inputs[c].clone(), prev[c].clone());
                    }
                }
            }

            let post = self.eval_perm_block::<AB, LL>(builder, pcols);
            prev_post = Some(post);
        }

        let mut outs = vec![AB::Expr::ZERO; plan.n_out];
        for o in 0..plan.n_out {
            let (sp, lane_idx) = plan.output_source(o);
            let base = off + slay.perms + sp * self.layout.perm_cols;
            let block: Vec<AB::Var> = local[base..base + self.layout.perm_cols].to_vec();
            let pcols: &Poseidon2Cols<
                AB::Var,
                WIDTH,
                SBOX_DEGREE,
                SBOX_REGISTERS,
                HALF_FULL_ROUNDS,
                PARTIAL_ROUNDS,
            > = (block.as_slice()).borrow();
            outs[o] = pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post[lane_idx].into();
        }
        outs
    }

    fn absorb_elem_expr<AB>(
        &self,
        local: &[AB::Var],
        off: usize,
        slay: &SpongeLayout,
        plan: &SpongePlan,
        pos: usize,
    ) -> AB::Expr
    where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
    {
        if pos < plan.n_input {
            local[off + slay.lanes + pos].into()
        } else if pos == plan.n_input {
            AB::Expr::ONE
        } else {
            AB::Expr::ZERO
        }
    }

    fn eval_c4_residue<AB>(&self, builder: &mut AB, local: &[AB::Var], base: usize) -> AB::Expr
    where
        AB: AirBuilder<F = F>,
        AB::F: PrimeField,
    {
        let v = local[base];
        let d_lo = local[base + 1];
        let d_hi = local[base + 2];
        let lo_bits: Vec<AB::Var> = (0..C4_DIGIT_BITS).map(|k| local[base + 3 + k]).collect();
        let hi_bits: Vec<AB::Var> = (0..C4_DIGIT_BITS).map(|k| local[base + 10 + k]).collect();
        let slack_bits: Vec<AB::Var> = (0..C4_DIGIT_BITS).map(|k| local[base + 17 + k]).collect();
        let is96 = local[base + 24];
        let inv = local[base + 25];

        let recompose = |bits: &[AB::Var]| -> AB::Expr {
            let mut acc = AB::Expr::ZERO;
            for (b, bit) in bits.iter().enumerate() {
                acc += (*bit).into() * ce::<AB>(1u64 << b);
            }
            acc
        };
        for bit in lo_bits.iter().chain(hi_bits.iter()).chain(slack_bits.iter()) {
            builder.assert_bool(*bit);
        }
        builder.assert_eq(d_lo.into(), recompose(&lo_bits));
        builder.assert_eq(d_hi.into(), recompose(&hi_bits));
        builder.assert_eq(
            ce::<AB>(C4_MAX_HI as u64) - d_hi.into(),
            recompose(&slack_bits),
        );
        builder.assert_bool(is96);
        builder.assert_eq(
            is96.into() * (d_hi.into() - ce::<AB>(C4_MAX_HI as u64)),
            AB::Expr::ZERO,
        );
        builder.assert_eq(
            (d_hi.into() - ce::<AB>(C4_MAX_HI as u64)) * inv.into(),
            AB::Expr::ONE - is96.into(),
        );
        builder.assert_eq(is96.into() * d_lo.into(), AB::Expr::ZERO);
        builder.assert_eq(
            v.into(),
            d_lo.into() + ce::<AB>(C4_BASE as u64) * d_hi.into(),
        );
        v.into()
    }

    fn eval_perm_block<AB, LL>(
        &self,
        builder: &mut AB,
        pcols: &Poseidon2Cols<
            AB::Var,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >,
    ) -> [AB::Expr; WIDTH]
    where
        AB: AirBuilder<F = F>,
        LL: GenericPoseidon2LinearLayers<WIDTH>,
    {
        let mut state: [AB::Expr; WIDTH] = core::array::from_fn(|i| pcols.inputs[i].into());
        LL::external_linear_layer(&mut state);

        for round in 0..HALF_FULL_ROUNDS {
            let fr = &pcols.beginning_full_rounds[round];
            for i in 0..WIDTH {
                let after_rc = state[i].clone()
                    + Into::<AB::Expr>::into(self.beginning_full_rc[round][i].dup());
                let sb = eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(
                    builder,
                    &fr.sbox[i].0,
                    after_rc,
                );
                state[i] = sb;
            }
            LL::external_linear_layer(&mut state);
            for i in 0..WIDTH {
                builder.assert_eq(state[i].clone(), fr.post[i]);
                state[i] = fr.post[i].into();
            }
        }

        for round in 0..PARTIAL_ROUNDS {
            let pr = &pcols.partial_rounds[round];
            let after_rc =
                state[0].clone() + Into::<AB::Expr>::into(self.partial_rc[round].dup());
            let sb =
                eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(builder, &pr.sbox.0, after_rc);
            builder.assert_eq(sb, pr.post_sbox);
            state[0] = pr.post_sbox.into();
            LL::internal_linear_layer(&mut state);
        }

        for round in 0..HALF_FULL_ROUNDS {
            let fr = &pcols.ending_full_rounds[round];
            for i in 0..WIDTH {
                let after_rc =
                    state[i].clone() + Into::<AB::Expr>::into(self.ending_full_rc[round][i].dup());
                let sb = eval_sbox_expr::<AB, SBOX_DEGREE, SBOX_REGISTERS>(
                    builder,
                    &fr.sbox[i].0,
                    after_rc,
                );
                state[i] = sb;
            }
            LL::external_linear_layer(&mut state);
            for i in 0..WIDTH {
                builder.assert_eq(state[i].clone(), fr.post[i]);
                state[i] = fr.post[i].into();
            }
        }

        state
    }
}

// ===========================================================================
// Witness / trace generation
// ===========================================================================

/// Honest secrets for one account-creation instance (internal trace-gen form).
#[derive(Clone)]
pub struct FullSecrets {
    pub pad_seed: [u8; 32],
    pub slot: u64,
    pub account_projection: [u16; N_LANES],
    pub nonce_c1: [u8; 16],
    pub lifecycle_anchor: [u8; 32],
    pub nonce_c6: [u8; 16],
    /// PUBLIC context: 32-byte account identity (folded into the C6 preimage).
    pub account_id: [u8; 32],
    /// PUBLIC context: verifier version (transcript-bound).
    pub verifier_version: u32,
    /// PUBLIC context: account state root (transcript-bound).
    pub account_state_root: [u8; 32],
    /// CLASS C — PUBLIC checked-core syndrome lanes (residues 0..11 of the packed
    /// witness frame; each a canonical q12289 residue from the spend's checked-core
    /// frame). The 6 masked_projection residues are derived in-circuit.
    pub syndrome: [u16; SYNDROME_LANES],
}

/// Computed public outputs + intermediate values from the honest sponge run.
#[derive(Clone)]
pub struct FullInstance<F> {
    pub pad_source_commitment: [F; N_COMMIT_LANES],
    pub opening_tuple_commitment: [F; N_COMMIT_LANES],
    pub masked_projection: [F; N_LANES],
    /// Public-context field elements: account_id[32] ‖ slot[8] ‖
    /// verifier_version[4] ‖ state_root[32] ‖ lifecycle[32].
    pub context: [F; CONTEXT_ELEMS],
    /// CLASS A: role_root = Poseidon2-sponge over the role-handle commitments.
    pub role_root: [F; ROLE_ROOT_ELEMS],
    /// CLASS C: the PUBLIC checked-core syndrome lanes (residues 0..11), echoed
    /// as field elements.
    pub syndrome: [F; SYNDROME_LANES],
    /// CLASS C: the PUBLIC `packed_witness_commitment` 32 byte anchors, packing
    /// `[syndrome[0..12] ‖ masked_projection[0..6]]` as 14-bit LE — IDENTICAL to
    /// the deployed `pack_reviewed_fq12_packed_witness_state18_from_parts`.
    pub packed_witness_commitment: [F; PACKED_WITNESS_COMMITMENT_BYTES],
    #[allow(dead_code)]
    pub masks: [u32; N_LANES],
    pub _phantom: PhantomData<F>,
}

#[allow(clippy::type_complexity)]
fn run_sponge<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    elems: &[F],
    plan: &SpongePlan,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
) -> (Vec<Vec<F>>, [F; WIDTH], Vec<F>) {
    let mut perm_blocks: Vec<Vec<F>> = Vec::with_capacity(plan.n_perms);
    let mut posts: Vec<[F; WIDTH]> = Vec::with_capacity(plan.n_perms);
    let mut prev_post = [F::ZERO; WIDTH];
    for p in 0..plan.n_perms {
        let mut state = [F::ZERO; WIDTH];
        if p == 0 {
            for r in 0..RATE {
                state[r] = elems[r];
            }
        } else if p < plan.n_absorb {
            for r in 0..RATE {
                let pos = p * RATE + r;
                let add = if pos < plan.padded_len() { elems[pos] } else { F::ZERO };
                state[r] = prev_post[r] + add;
            }
            for c in RATE..WIDTH {
                state[c] = prev_post[c];
            }
        } else {
            state = prev_post;
        }

        let mut block: Vec<MaybeUninit<F>> = Vec::with_capacity(perm_cols);
        block.resize_with(perm_cols, MaybeUninit::uninit);
        let pcols: &mut Poseidon2Cols<
            MaybeUninit<F>,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        > = block.as_mut_slice().borrow_mut();
        generate_trace_rows_for_perm::<
            F,
            LinearLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(pcols, state, constants);
        let post = unsafe {
            let pp = &pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post;
            core::array::from_fn::<F, WIDTH, _>(|c| pp[c].assume_init())
        };
        let packed: Vec<F> = unsafe { block.iter().map(|m| m.assume_init()).collect() };
        perm_blocks.push(packed);
        posts.push(post);
        prev_post = post;
    }

    let mut outputs = vec![F::ZERO; plan.n_out];
    for o in 0..plan.n_out {
        let (sp, lane) = plan.output_source(o);
        outputs[o] = posts[sp][lane];
    }
    (perm_blocks, prev_post, outputs)
}

/// REFERENCE C2 projection-mask PRF: the canonical in-proof mask derivation,
/// exposed so the standalone `descender-mask-prf` crate can be proven
/// byte-identical to it (drift-prevention KAT).
///
/// This reproduces EXACTLY what `generate_full_trace` does for the C2 sponge — it
/// reuses the SAME private primitives (`SpongePlan::new`, `build_elems`,
/// `run_sponge`, `wide_reduce_masks`), so it CANNOT drift from the prover's trace
/// fill. It returns both the 30 squeezed lanes and the 6 reduced q12289 masks.
///
/// Input lane order matches the trace generator: `pad_seed[32] ‖ slot_id LE[8]`.
#[doc(hidden)]
pub fn reference_c2_mask_prf<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    pad_seed: &[u8; 32],
    slot_id: u64,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
) -> ([u32; NUM_SQUEEZE], [u32; N_LANES]) {
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();

    // Same C2 data layout as `generate_instance_row` (pad_seed bytes ‖ slot LE).
    let mut c2_data: Vec<u32> = pad_seed.iter().map(|&b| b as u32).collect();
    c2_data.extend(slot_id.to_le_bytes().iter().map(|&b| b as u32));
    debug_assert_eq!(c2_data.len(), C2_INPUT_LANES);

    let plan = SpongePlan::new(vec![true; C2_INPUT_LANES], NUM_SQUEEZE);
    let elems = build_elems::<F>(&c2_data, &plan);
    let (_blocks, _post, outputs) = run_sponge::<
        F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    >(&elems, &plan, constants, perm_cols);

    let mut squeezed = [0u32; NUM_SQUEEZE];
    for (i, s) in squeezed.iter_mut().enumerate() {
        *s = outputs[i].as_canonical_u32();
    }
    let masks = wide_reduce_masks::<F>(&squeezed);
    (squeezed, masks)
}

/// One batch-digest-chain absorb step: seed the 16-wide sponge with `state_in`,
/// absorb `msg` (length must be a multiple of RATE) in RATE-sized blocks, and
/// return the per-permutation filled column blocks and the resulting state.
/// Mirrors `eval_digest_chain` EXACTLY (used for trace fill).
#[allow(clippy::type_complexity)]
fn run_chain_step<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    state_in: [F; WIDTH],
    msg: &[F],
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
) -> (Vec<Vec<F>>, [F; WIDTH]) {
    let n_perms = msg.len() / RATE;
    debug_assert_eq!(msg.len() % RATE, 0);
    let mut blocks: Vec<Vec<F>> = Vec::with_capacity(n_perms);
    let mut prev_post = state_in;
    for p in 0..n_perms {
        let mut state = prev_post;
        for r in 0..RATE {
            state[r] = prev_post[r] + msg[p * RATE + r];
        }
        // capacity lanes carry unchanged (prev_post already in `state`).

        let mut block: Vec<MaybeUninit<F>> = Vec::with_capacity(perm_cols);
        block.resize_with(perm_cols, MaybeUninit::uninit);
        let pcols: &mut Poseidon2Cols<
            MaybeUninit<F>,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        > = block.as_mut_slice().borrow_mut();
        generate_trace_rows_for_perm::<
            F,
            LinearLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(pcols, state, constants);
        let post = unsafe {
            let pp = &pcols.ending_full_rounds[HALF_FULL_ROUNDS - 1].post;
            core::array::from_fn::<F, WIDTH, _>(|c| pp[c].assume_init())
        };
        let packed: Vec<F> = unsafe { block.iter().map(|m| m.assume_init()).collect() };
        blocks.push(packed);
        prev_post = post;
    }
    (blocks, prev_post)
}

fn build_elems<F: PrimeField32>(data: &[u32], plan: &SpongePlan) -> Vec<F> {
    let mut elems = vec![F::ZERO; plan.padded_len()];
    for (i, &v) in data.iter().enumerate() {
        elems[i] = F::from_u32(v);
    }
    elems[plan.n_input] = F::ONE;
    elems
}

/// CLASS C — reference packer: pack `[syndrome[12] ‖ masked_projection[6]]` (each
/// a canonical q12289 residue) as 14-bit LITTLE-ENDIAN into a 32-byte commitment.
/// BYTE-FOR-BIT IDENTICAL to the deployed
/// `pack_reviewed_fq12_packed_witness_state18_from_parts` ->
/// `pack_reviewed_fq12_packed_witness_state18` ->
/// `write_bits_le(out, idx*14, 14, value)`.
fn pack_packed_witness_commitment(
    syndrome: &[u16; SYNDROME_LANES],
    masked_projection: &[u16; N_LANES],
) -> [u8; PACKED_WITNESS_COMMITMENT_BYTES] {
    let mut residues = [0u16; PACKED_WITNESS_RESIDUES];
    residues[..SYNDROME_LANES].copy_from_slice(syndrome);
    residues[SYNDROME_LANES..].copy_from_slice(masked_projection);
    let mut out = [0u8; PACKED_WITNESS_COMMITMENT_BYTES];
    for (idx, &value) in residues.iter().enumerate() {
        let bit_off = idx * PACKED_LANE_BITS;
        for i in 0..PACKED_LANE_BITS {
            let bit = ((value >> i) & 1) as u8;
            if bit != 0 {
                let pos = bit_off + i;
                out[pos / 8] |= 1u8 << (pos % 8);
            }
        }
    }
    out
}

fn c4_witness<F: PrimeField32>(v: u32) -> ([F; C4_CELLS], F) {
    let d_lo = v % C4_BASE;
    let d_hi = v / C4_BASE;
    let slack = C4_MAX_HI - d_hi;
    let is96 = d_hi == C4_MAX_HI;
    let diff = F::from_u32(d_hi) - F::from_u32(C4_MAX_HI);
    let inv = if is96 { F::ZERO } else { diff.inverse() };
    let bits = |x: u32| -> [F; C4_DIGIT_BITS] {
        core::array::from_fn(|k| F::from_u32((x >> k) & 1))
    };
    let mut cells = [F::ZERO; C4_CELLS];
    cells[0] = F::from_u32(v);
    cells[1] = F::from_u32(d_lo);
    cells[2] = F::from_u32(d_hi);
    cells[3..10].copy_from_slice(&bits(d_lo));
    cells[10..17].copy_from_slice(&bits(d_hi));
    cells[17..24].copy_from_slice(&bits(slack));
    cells[24] = if is96 { F::ONE } else { F::ZERO };
    cells[25] = inv;
    (cells, F::from_u32(v))
}

/// Reference (off-circuit) computation of the batch digest chain, mirroring
/// `eval_digest_chain`. Folds each real account's pubio tuple into the running
/// sponge state (msg = `[1, pubio...]` zero-padded to a multiple of RATE),
/// starting from the all-zero state. Returns the 8-element digest. The verifier
/// recomputes this over the supplied tuples; the prover exposes it as the STARK
/// public digest, so the two agree by construction.
#[allow(clippy::type_complexity)]
pub fn chain_digest<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    pubios: &[[F; PUBIO_ELEMS]],
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
) -> [F; CHAIN_DIGEST_ELEMS] {
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let mut state = [F::ZERO; WIDTH];
    for pubio in pubios {
        let mut msg = vec![F::ZERO; CHAIN_PERMS_PER_ROW * RATE];
        msg[0] = F::ONE; // enable
        for (i, &e) in pubio.iter().enumerate() {
            msg[1 + i] = e;
        }
        let (_blocks, post) = run_chain_step::<
            F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(state, &msg, constants, perm_cols);
        state = post;
    }
    core::array::from_fn(|c| state[c])
}

/// Generate the full trace: `num_rows` identical honest rows + the instance.
#[allow(clippy::type_complexity)]
pub fn generate_full_trace<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    secrets: &FullSecrets,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    num_rows: usize,
) -> (RowMajorMatrix<F>, FullInstance<F>) {
    assert!(num_rows.is_power_of_two());
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let lay = FullLayout::new(perm_cols);
    let width = lay.width;

    let (row, inst) = build_full_row::<
        F,
        LinearLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >(secrets, &lay, constants, perm_cols);

    let mut values = Vec::with_capacity(width * num_rows);
    for _ in 0..num_rows {
        values.extend_from_slice(&row);
    }
    (RowMajorMatrix::new(values, width), inst)
}

/// Generate a BATCHED trace: one DISTINCT account-creation instance per row.
#[allow(clippy::type_complexity)]
pub fn generate_batch_trace<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    secrets: &[FullSecrets],
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
) -> (RowMajorMatrix<F>, Vec<FullInstance<F>>, [F; CHAIN_DIGEST_ELEMS], usize) {
    assert!(!secrets.is_empty());
    let perm_cols =
        perm_num_cols::<WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>();
    let lay = FullLayout::new(perm_cols);
    let width = lay.width;

    let n_real = secrets.len();
    let num_rows = n_real.next_power_of_two();

    let mut values = vec![F::ZERO; width * num_rows];
    let mut insts = Vec::with_capacity(n_real);

    let mut last_row: Vec<F> = Vec::new();
    for (r, s) in secrets.iter().enumerate() {
        let (row, inst) = build_full_row::<
            F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(s, &lay, constants, perm_cols);
        values[r * width..(r + 1) * width].copy_from_slice(&row);
        last_row = row;
        insts.push(inst);
    }
    for r in n_real..num_rows {
        values[r * width..(r + 1) * width].copy_from_slice(&last_row);
    }

    // ----- Fill the BATCH DIGEST CHAIN columns (sequential across rows) -----
    let mut state = [F::ZERO; WIDTH];
    let mut count_acc: u64 = 0;
    for r in 0..num_rows {
        let base = r * width;
        let enable = r < n_real;
        count_acc += enable as u64;

        // chain_in = incoming running state.
        for c in 0..WIDTH {
            values[base + lay.chain_in_off + c] = state[c];
        }
        // Build the absorb message: enable ? [1, pubio...] : all-zero.
        let mut msg = vec![F::ZERO; CHAIN_PERMS_PER_ROW * RATE];
        if enable {
            msg[0] = F::ONE;
            for i in 0..PUBIO_ELEMS {
                msg[1 + i] = values[base + lay.pubio_off + i];
            }
        }
        let (blocks, post) = run_chain_step::<
            F, LinearLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(state, &msg, constants, perm_cols);
        for (p, b) in blocks.iter().enumerate() {
            let off = base + lay.chain_perms_off + p * perm_cols;
            values[off..off + perm_cols].copy_from_slice(b);
        }
        // chain_out = enable ? post : chain_in (no-op for padding rows).
        let out = if enable { post } else { state };
        for c in 0..WIDTH {
            values[base + lay.chain_out_off + c] = out[c];
        }
        values[base + lay.enable_off] = if enable { F::ONE } else { F::ZERO };
        values[base + lay.count_acc_off] = F::from_u64(count_acc);

        state = out;
    }
    let digest: [F; CHAIN_DIGEST_ELEMS] = core::array::from_fn(|c| state[c]);

    (RowMajorMatrix::new(values, width), insts, digest, n_real)
}

#[allow(clippy::type_complexity)]
fn build_full_row<
    F: PrimeField32,
    LinearLayers: GenericPoseidon2LinearLayers<WIDTH>,
    const WIDTH: usize,
    const SBOX_DEGREE: u64,
    const SBOX_REGISTERS: usize,
    const HALF_FULL_ROUNDS: usize,
    const PARTIAL_ROUNDS: usize,
>(
    secrets: &FullSecrets,
    lay: &FullLayout,
    constants: &RoundConstants<F, WIDTH, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>,
    perm_cols: usize,
) -> (Vec<F>, FullInstance<F>) {
    let mut row = vec![F::ZERO; lay.width];

    let mut c1_data: Vec<u32> = secrets.pad_seed.iter().map(|&b| b as u32).collect();
    c1_data.extend(secrets.nonce_c1.iter().map(|&b| b as u32));
    let mut c2_data: Vec<u32> = secrets.pad_seed.iter().map(|&b| b as u32).collect();
    c2_data.extend(secrets.slot.to_le_bytes().iter().map(|&b| b as u32));
    let mut c6_data: Vec<u32> = secrets.account_projection.iter().map(|&v| v as u32).collect();
    c6_data.extend(secrets.lifecycle_anchor.iter().map(|&b| b as u32));
    c6_data.extend(secrets.nonce_c6.iter().map(|&b| b as u32));
    c6_data.extend(secrets.account_id.iter().map(|&b| b as u32));

    let fill_sponge = |row: &mut [F], off: usize, slay: &SpongeLayout, plan: &SpongePlan, data: &[u32]| -> Vec<F> {
        for (i, &v) in data.iter().enumerate() {
            row[off + slay.lanes + i] = F::from_u32(v);
        }
        let mut bit_cursor = 0usize;
        for (i, &v) in data.iter().enumerate() {
            if plan.byte_ranged[i] {
                for b in 0..BYTE_BITS {
                    row[off + slay.lane_bits + bit_cursor * BYTE_BITS + b] = F::from_u32((v >> b) & 1);
                }
                bit_cursor += 1;
            }
        }
        let elems = build_elems::<F>(data, plan);
        let (blocks, _post, outputs) = run_sponge::<
            F,
            LinearLayers,
            WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
        >(&elems, plan, constants, perm_cols);
        for (p, b) in blocks.iter().enumerate() {
            let base = off + slay.perms + p * perm_cols;
            row[base..base + perm_cols].copy_from_slice(b);
        }
        outputs
    };

    let c1_out = fill_sponge(&mut row, lay.c1_off, &lay.c1, &lay.c1_plan, &c1_data);
    let c2_out = fill_sponge(&mut row, lay.c2_off, &lay.c2, &lay.c2_plan, &c2_data);
    let c6_out = fill_sponge(&mut row, lay.c6_off, &lay.c6, &lay.c6_plan, &c6_data);

    let mut squeezed = [0u32; NUM_SQUEEZE];
    for (i, s) in squeezed.iter_mut().enumerate() {
        *s = c2_out[i].as_canonical_u32();
    }
    let masks = fill_reduce_gadget::<F>(&mut row, lay.c2_off + lay.c2.reduce, &squeezed);
    debug_assert_eq!(masks, wide_reduce_masks::<F>(&squeezed));

    let mut masked_projection = [F::ZERO; N_LANES];
    let mut masked_projection_u16 = [0u16; N_LANES];
    for j in 0..N_LANES {
        let v = secrets.account_projection[j] as u32;
        let (cells, _vv) = c4_witness::<F>(v);
        let base = lay.c4_off + j * C4_CELLS;
        row[base..base + C4_CELLS].copy_from_slice(&cells);
        row[lay.carry_acc_off + j] = F::from_u32(v);

        // Carrier mod-q: masked_canonical = (acc + mask) mod q12289, wrap ∈ {0,1}.
        let sum = v + masks[j];
        let wrap = if sum >= Q12289_CARRIER { 1u32 } else { 0u32 };
        let masked_canonical = sum - wrap * Q12289_CARRIER;
        let slack = (Q12289_CARRIER - 1) - masked_canonical;

        row[lay.carry_masked_off + j] = F::from_u32(masked_canonical);
        let mb = lay.carry_modq_off + j * MODQ_CELLS;
        let masked_bits_off = mb + 1;
        let slack_off = mb + 1 + MODQ_BITS;
        let slack_bits_off = slack_off + 1;
        let wrap_off = slack_bits_off + MODQ_BITS;
        for b in 0..MODQ_BITS {
            row[masked_bits_off + b] = F::from_u32((masked_canonical >> b) & 1);
            row[slack_bits_off + b] = F::from_u32((slack >> b) & 1);
        }
        row[slack_off] = F::from_u32(slack);
        row[wrap_off] = F::from_u32(wrap);

        masked_projection[j] = F::from_u32(masked_canonical);
        masked_projection_u16[j] = masked_canonical as u16;
    }

    // ----- CLASS C: checked-core syndrome witness (value/bits/slack per lane) -----
    let mut syndrome_f = [F::ZERO; SYNDROME_LANES];
    for lane in 0..SYNDROME_LANES {
        let v = secrets.syndrome[lane] as u32;
        let slack = (Q12289_CARRIER - 1).wrapping_sub(v); // honest syndrome < q ⇒ no wrap
        let sb = lay.syndrome_off + lane * SYNDROME_CELLS;
        let val_off = sb;
        let bits_off = sb + 1;
        let slack_off = sb + 1 + MODQ_BITS;
        let slack_bits_off = slack_off + 1;
        row[val_off] = F::from_u32(v);
        row[slack_off] = F::from_u32(slack);
        for b in 0..MODQ_BITS {
            row[bits_off + b] = F::from_u32((v >> b) & 1);
            row[slack_bits_off + b] = F::from_u32((slack >> b) & 1);
        }
        syndrome_f[lane] = F::from_u32(v);
    }

    // ----- CLASS C: packed_witness_commitment[32] (14-bit-LE residue packing) -----
    let packed_bytes = pack_packed_witness_commitment(&secrets.syndrome, &masked_projection_u16);
    let mut packed_witness_commitment = [F::ZERO; PACKED_WITNESS_COMMITMENT_BYTES];
    for k in 0..PACKED_WITNESS_COMMITMENT_BYTES {
        packed_witness_commitment[k] = F::from_u32(packed_bytes[k] as u32);
    }

    // ----- CLASS A: role-root sponge over the role-handle commitments -----
    // role_data = pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖
    //             masked_projection[6] (canonical u32 reps; all < field modulus).
    let mut role_data: Vec<u32> = Vec::with_capacity(ROLE_HANDLE_LANES);
    role_data.extend(c1_out.iter().map(|f| f.as_canonical_u32()));
    role_data.extend(c6_out.iter().map(|f| f.as_canonical_u32()));
    role_data.extend(masked_projection.iter().map(|f| f.as_canonical_u32()));
    let role_out = fill_sponge(&mut row, lay.role_off, &lay.role, &lay.role_plan, &role_data);
    let mut role_root = [F::ZERO; ROLE_ROOT_ELEMS];
    for j in 0..ROLE_ROOT_ELEMS {
        role_root[j] = role_out[j];
    }

    // ----- Public-context columns: account_id[32] ‖ slot[8] ‖ ver[4] ‖
    //       state_root[32] ‖ lifecycle[32] (byte-valued field elements) -----
    let mut context = [F::ZERO; CONTEXT_ELEMS];
    {
        let mut put = |off: usize, bytes: &[u8]| {
            for (b, &byte) in bytes.iter().enumerate() {
                context[off + b] = F::from_u32(byte as u32);
            }
        };
        put(CTX_ACCOUNT_ID_OFF, &secrets.account_id);
        put(CTX_SLOT_OFF, &secrets.slot.to_le_bytes());
        put(CTX_VERIFIER_VERSION_OFF, &secrets.verifier_version.to_le_bytes());
        put(CTX_STATE_ROOT_OFF, &secrets.account_state_root);
        put(CTX_LIFECYCLE_OFF, &secrets.lifecycle_anchor);
    }
    for i in 0..CONTEXT_ELEMS {
        row[lay.ctx_off + i] = context[i];
    }

    let mut pad_source_commitment = [F::ZERO; N_COMMIT_LANES];
    for j in 0..N_COMMIT_LANES {
        pad_source_commitment[j] = c1_out[j];
    }
    let mut opening_tuple_commitment = [F::ZERO; N_COMMIT_LANES];
    for j in 0..N_COMMIT_LANES {
        opening_tuple_commitment[j] = c6_out[j];
    }

    for j in 0..N_COMMIT_LANES {
        row[lay.pubio_off + j] = pad_source_commitment[j];
        row[lay.pubio_off + N_COMMIT_LANES + j] = opening_tuple_commitment[j];
    }
    for j in 0..N_LANES {
        row[lay.pubio_off + 2 * N_COMMIT_LANES + j] = masked_projection[j];
    }
    for i in 0..CONTEXT_ELEMS {
        row[lay.pubio_off + 2 * N_COMMIT_LANES + N_LANES + i] = context[i];
    }
    // CLASS A: role_root into the pubio block (so the batch digest covers it).
    for j in 0..ROLE_ROOT_ELEMS {
        row[lay.pubio_off + PUBIO_ROLE_ROOT_OFF + j] = role_root[j];
    }
    // CLASS B: relation_id (pinned constant) into the pubio block.
    for j in 0..RELATION_ID_ELEMS {
        row[lay.pubio_off + PUBIO_RELATION_ID_OFF + j] = F::from_u32(RELATION_ID[j]);
    }
    // CLASS C: syndrome[12] ‖ packed_witness_commitment[32] into the pubio block
    // (so the batch digest chain covers them).
    for lane in 0..SYNDROME_LANES {
        row[lay.pubio_off + PUBIO_SYNDROME_OFF + lane] = syndrome_f[lane];
    }
    for k in 0..PACKED_WITNESS_COMMITMENT_BYTES {
        row[lay.pubio_off + PUBIO_PACKED_WITNESS_OFF + k] = packed_witness_commitment[k];
    }

    let inst = FullInstance {
        pad_source_commitment,
        opening_tuple_commitment,
        masked_projection,
        context,
        role_root,
        syndrome: syndrome_f,
        packed_witness_commitment,
        masks,
        _phantom: PhantomData,
    };
    (row, inst)
}

/// Public-values vector: pad_source_commitment[10] ‖ opening_tuple[10] ‖
/// masked[6] ‖ context ‖ role_root ‖ relation_id ‖ syndrome[12] ‖
/// packed_witness_commitment[32].
pub fn full_public_values<F: PrimeField32>(inst: &FullInstance<F>) -> Vec<F> {
    let mut v = Vec::with_capacity(
        2 * N_COMMIT_LANES
            + N_LANES
            + CONTEXT_ELEMS
            + ROLE_ROOT_ELEMS
            + RELATION_ID_ELEMS
            + CHECKED_CORE_ELEMS,
    );
    v.extend_from_slice(&inst.pad_source_commitment);
    v.extend_from_slice(&inst.opening_tuple_commitment);
    v.extend_from_slice(&inst.masked_projection);
    v.extend_from_slice(&inst.context);
    v.extend_from_slice(&inst.role_root);
    v.extend(RELATION_ID.iter().map(|&id| F::from_u32(id)));
    v.extend_from_slice(&inst.syndrome);
    v.extend_from_slice(&inst.packed_witness_commitment);
    v
}

// ===========================================================================
// In-circuit constraint-isolation tests (DEBUG-only constraint checker).
//
// These prove that the context-binding equalities are enforced by the AIR
// itself — independent of the Fiat–Shamir transcript. We build an honest trace,
// corrupt ONLY the in-circuit witness lane that absorbs a context field (leaving
// the public/ctx columns and the public values untouched), and assert the AIR
// constraint checker REJECTS. (Runs under `cargo test` debug; the prover's
// debug-build `check_constraints` is the same gate.)
// ===========================================================================
#[cfg(test)]
mod context_constraint_tests {
    use super::*;
    use crate::config::{
        HALF_FULL_ROUNDS, LinLayers, PARTIAL_ROUNDS, SBOX_DEGREE, SBOX_REGISTERS, Val, WIDTH,
    };
    use p3_air::check_constraints;
    use p3_koala_bear::{
        default_koalabear_poseidon2_16, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL, KOALABEAR_POSEIDON2_RC_16_INTERNAL,
    };

    type Air = FullRelationAir<
        Val,
        LinLayers,
        WIDTH,
        SBOX_DEGREE,
        SBOX_REGISTERS,
        HALF_FULL_ROUNDS,
        PARTIAL_ROUNDS,
    >;

    fn secrets() -> FullSecrets {
        FullSecrets {
            pad_seed: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
            slot: 0xDEAD_BEEF_0000_0042,
            account_projection: [11, 2222, 333, 4444, 555, 12288],
            nonce_c1: core::array::from_fn(|i| (i as u8).wrapping_mul(3).wrapping_add(5)),
            lifecycle_anchor: core::array::from_fn(|i| (i as u8).wrapping_mul(5).wrapping_add(2)),
            nonce_c6: core::array::from_fn(|i| (i as u8).wrapping_mul(9).wrapping_add(1)),
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            verifier_version: 0x0102_0304,
            account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
            // CLASS C: 12 canonical (< q12289) checked-core syndrome lanes.
            syndrome: core::array::from_fn(|i| ((i as u32 * 1009 + 17) % Q12289_CARRIER) as u16),
        }
    }

    fn build() -> (Air, p3_matrix::dense::RowMajorMatrix<Val>, Vec<Val>, FullLayout) {
        let _ = default_koalabear_poseidon2_16(); // pin the same instance
        let air: Air = FullRelationAir::from_arrays_mode(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
            false,
        );
        let (trace, inst) = generate_full_trace::<
            Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(&secrets(), air.constants(), 2);
        let pv = full_public_values(&inst);
        let lay = FullLayout::new(air.layout().perm_cols);
        (air, trace, pv, lay)
    }

    #[test]
    fn honest_trace_satisfies_constraints() {
        let (air, trace, pv, _lay) = build();
        check_constraints(&air, &trace, &pv); // panics on violation
    }

    // Corrupt the C2-absorbed slot lane (byte 0) in BOTH rows, leaving the public
    // ctx slot byte untouched -> the slot==public AIR equality must fail.
    #[test]
    #[should_panic]
    fn c2_slot_lane_mismatch_violates_constraints() {
        let (air, mut trace, pv, lay) = build();
        let col = lay.c2_off + lay.c2_lanes_off() + 32; // first slot byte-lane
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            trace.values[r * w + col] += Val::ONE;
        }
        check_constraints(&air, &trace, &pv);
    }

    // Corrupt the C6-absorbed lifecycle lane -> lifecycle==public equality fails.
    #[test]
    #[should_panic]
    fn c6_lifecycle_lane_mismatch_violates_constraints() {
        let (air, mut trace, pv, lay) = build();
        let col = lay.c6_off + lay.c6_lanes_off() + C6_FIELD_LANES; // first lifecycle byte-lane
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            trace.values[r * w + col] += Val::ONE;
        }
        check_constraints(&air, &trace, &pv);
    }

    // Corrupt the C6-absorbed account_id lane -> account_id==public equality fails.
    #[test]
    #[should_panic]
    fn c6_account_id_lane_mismatch_violates_constraints() {
        let (air, mut trace, pv, lay) = build();
        let col = lay.c6_off + lay.c6_lanes_off() + C6_FIELD_LANES + LIFECYCLE_BYTES + 16;
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            trace.values[r * w + col] += Val::ONE;
        }
        check_constraints(&air, &trace, &pv);
    }

    // --- CLASS A index helpers (single-mode public-value layout). ---
    fn role_root_pv_off() -> usize {
        2 * N_COMMIT_LANES + N_LANES + CONTEXT_ELEMS
    }
    fn relation_id_pv_off() -> usize {
        role_root_pv_off() + ROLE_ROOT_ELEMS
    }

    // CLASS A completeness: the honest instance's role_root equals the Poseidon2
    // sponge over (pad_source_commitment ‖ opening_tuple_commitment ‖
    // masked_projection), and the public-value vector carries it.
    #[test]
    fn role_root_matches_recompute_over_commitments() {
        let _ = default_koalabear_poseidon2_16();
        let air: Air = FullRelationAir::from_arrays_mode(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
            false,
        );
        let (_trace, inst) = generate_full_trace::<
            Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(&secrets(), air.constants(), 2);

        // Independent recompute of the role-root sponge over the role handles.
        let perm_cols = air.layout().perm_cols;
        let role_plan = SpongePlan::new(vec![false; ROLE_HANDLE_LANES], ROLE_ROOT_ELEMS);
        let mut data: Vec<u32> = Vec::new();
        data.extend(inst.pad_source_commitment.iter().map(|f| f.as_canonical_u32()));
        data.extend(inst.opening_tuple_commitment.iter().map(|f| f.as_canonical_u32()));
        data.extend(inst.masked_projection.iter().map(|f| f.as_canonical_u32()));
        let elems = build_elems::<Val>(&data, &role_plan);
        let (_b, _post, outputs) = run_sponge::<
            Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(&elems, &role_plan, air.constants(), perm_cols);
        for j in 0..ROLE_ROOT_ELEMS {
            assert_eq!(inst.role_root[j], outputs[j], "role_root[{j}] != recompute");
        }
        // And the public-value vector carries the role_root + pinned relation_id.
        let pv = full_public_values(&inst);
        for j in 0..ROLE_ROOT_ELEMS {
            assert_eq!(pv[role_root_pv_off() + j], inst.role_root[j]);
        }
        for j in 0..RELATION_ID_ELEMS {
            assert_eq!(pv[relation_id_pv_off() + j], Val::from_u32(RELATION_ID[j]));
        }
    }

    // CLASS A soundness: forge the public role_root (leave the commitment columns
    // honest) -> the in-AIR `role_out == public role_root` equality fails.
    #[test]
    #[should_panic]
    fn forged_role_root_violates_constraints() {
        let (air, trace, mut pv, _lay) = build();
        pv[role_root_pv_off()] += Val::ONE; // tamper the claimed role_root
        check_constraints(&air, &trace, &pv);
    }

    // CLASS A soundness: corrupt a role-sponge INPUT lane (so the root is computed
    // over something other than the proven commitments) -> the input-binding
    // equality (role_lane == c1_out) fails.
    #[test]
    #[should_panic]
    fn role_input_lane_mismatch_violates_constraints() {
        let (air, mut trace, pv, lay) = build();
        let col = lay.role_off; // first role-handle input lane (== c1_out[0])
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            trace.values[r * w + col] += Val::ONE;
        }
        check_constraints(&air, &trace, &pv);
    }

    // CLASS B soundness: claim a wrong relation_id -> the in-AIR pin
    // (relation_id == RELATION_ID constant) fails.
    #[test]
    #[should_panic]
    fn wrong_relation_id_violates_constraints() {
        let (air, trace, mut pv, _lay) = build();
        pv[relation_id_pv_off()] += Val::ONE; // not the pinned constant anymore
        check_constraints(&air, &trace, &pv);
    }

    // --- CLASS C index helpers (single-mode public-value layout). ---
    fn syndrome_pv_off() -> usize {
        relation_id_pv_off() + RELATION_ID_ELEMS
    }
    fn packed_witness_pv_off() -> usize {
        syndrome_pv_off() + SYNDROME_LANES
    }

    // CLASS C completeness: the honest (syndrome, masked_projection) packs to the
    // claimed packed_witness_commitment, and ALL constraints are satisfied. (The
    // honest-trace satisfaction is already covered by
    // `honest_trace_satisfies_constraints`; here we also assert the in-instance
    // packed bytes equal the independent reference packer = the SAME layout as the
    // deployed pack_reviewed_fq12_packed_witness_state18_from_parts.)
    #[test]
    fn checked_core_packing_matches_reference_packer() {
        let _ = default_koalabear_poseidon2_16();
        let air: Air = FullRelationAir::from_arrays_mode(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
            false,
        );
        let (_trace, inst) = generate_full_trace::<
            Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(&secrets(), air.constants(), 2);

        // masked_projection (canonical u16) + syndrome -> reference packer.
        let masked_u16: [u16; N_LANES] =
            core::array::from_fn(|j| inst.masked_projection[j].as_canonical_u32() as u16);
        let expect = pack_packed_witness_commitment(&secrets().syndrome, &masked_u16);
        for k in 0..PACKED_WITNESS_COMMITMENT_BYTES {
            assert_eq!(
                inst.packed_witness_commitment[k].as_canonical_u32() as u8,
                expect[k],
                "packed_witness_commitment[{k}] != reference 14-bit-LE pack"
            );
        }
        // The public-value vector carries syndrome + packed_witness_commitment.
        let pv = full_public_values(&inst);
        for j in 0..SYNDROME_LANES {
            assert_eq!(pv[syndrome_pv_off() + j], inst.syndrome[j]);
        }
        for k in 0..PACKED_WITNESS_COMMITMENT_BYTES {
            assert_eq!(pv[packed_witness_pv_off() + k], inst.packed_witness_commitment[k]);
        }
    }

    // CLASS C soundness: a TAMPERED syndrome witness lane (so the packed stream no
    // longer equals the claimed commitment bytes) -> the byte-anchor equality
    // and/or the value==recompose equality fails.
    #[test]
    #[should_panic]
    fn tampered_syndrome_lane_violates_constraints() {
        let (air, mut trace, pv, lay) = build();
        // Bump syndrome lane 0's value column (and leave its bits/public alone):
        // value==public passes? No — value column is bumped, public is not -> the
        // value==public equality breaks; also value==recompose breaks.
        let col = lay.syndrome_off; // value cell of syndrome lane 0
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            trace.values[r * w + col] += Val::ONE;
        }
        check_constraints(&air, &trace, &pv);
    }

    // CLASS C soundness: a NON-CANONICAL syndrome (value >= q12289) must reject.
    // We set lane 0 to q (=12289) consistently across value + bits + public; the
    // canonical-slack gadget (value + slack == q-1 with slack a 14-bit value)
    // cannot be satisfied -> reject.
    #[test]
    #[should_panic]
    fn noncanonical_syndrome_violates_constraints() {
        let (air, mut trace, mut pv, lay) = build();
        let w = lay.width;
        let q = Q12289_PACK; // 12289
        let sb = lay.syndrome_off; // lane 0 value
        let bits_off = sb + 1;
        for r in 0..trace.values.len() / w {
            let base = r * w;
            trace.values[base + sb] = Val::from_u32(q);
            for b in 0..MODQ_BITS {
                trace.values[base + bits_off + b] = Val::from_u32((q >> b) & 1);
            }
            // The slack cells stay honest-for-the-old-value; even if the prover
            // re-derived slack = (q-1) - q (underflow), it cannot be a 14-bit
            // value, so the slack recompose / range fails. We leave them as-is to
            // trigger value+slack != q-1.
        }
        // Update the public syndrome[0] to match (so value==public still holds and
        // the rejection is specifically the non-canonical range, not a mismatch).
        pv[syndrome_pv_off()] = Val::from_u32(q);
        check_constraints(&air, &trace, &pv);
    }

    // CLASS C soundness: a FORGED packed_witness_commitment (public byte tampered,
    // trace honest) -> the in-AIR byte-anchor equality fails.
    #[test]
    #[should_panic]
    fn forged_packed_witness_commitment_violates_constraints() {
        let (air, trace, mut pv, _lay) = build();
        pv[packed_witness_pv_off()] += Val::ONE; // tamper commitment byte 0
        check_constraints(&air, &trace, &pv);
    }

    // CLASS C soundness: a TAMPERED masked_projection (carrier value column) must
    // change the packed stream and break the commitment byte anchor (the
    // masked_projection residues 12..17 are part of the packed witness).
    #[test]
    #[should_panic]
    fn tampered_masked_projection_breaks_packing() {
        let (air, mut trace, pv, lay) = build();
        // Bump the masked-bits of carrier lane 0 (residue 12) WITHOUT changing the
        // public commitment -> the stream byte differs from the anchor.
        let mb = lay.carry_modq_off; // lane 0 mod-q block
        let masked_bits_off = mb + 1; // first masked bit
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            let cell = r * w + masked_bits_off;
            // flip bit 0: honest masked LSB ^ 1 (keeps it boolean).
            trace.values[cell] = Val::ONE - trace.values[cell];
        }
        check_constraints(&air, &trace, &pv);
    }

    // MED-3 (2026-06-18 soundness audit): regression guard on the C4 canonicity
    // inverse-witness corner. Lane 5 of the build() projection is 12288 = q-1, i.e.
    // d_hi = 96, d_lo = 0, is96 = 1. Flipping ONLY is96 -> 0 isolates the
    // `(d_hi-96)*inv == 1-is96` constraint (air.rs ~1416): with d_hi == 96 the LHS
    // is 0, so 1-is96 must be 0 (is96 == 1); is96 = 0 forces 0 == 1 -> reject.
    // Every other constraint stays honest, so this fails for exactly the right
    // reason. If a future edit drops the inverse line, a prover could set is96 = 0
    // with d_hi = 96 and d_lo != 0 to admit a non-canonical lane (v = 12288 + d_lo
    // >= q); this test would then stop panicking and catch the regression.
    #[test]
    #[should_panic]
    fn c4_is96_inverse_corner_violates_constraints() {
        let (air, mut trace, pv, lay) = build();
        // lane 5: account_projection[5] == 12288 (q-1) -> is96 honest value is 1.
        let is96_cell = lay.c4_off + 5 * C4_CELLS + 24;
        let w = lay.width;
        for r in 0..trace.values.len() / w {
            trace.values[r * w + is96_cell] = Val::ZERO;
        }
        check_constraints(&air, &trace, &pv);
    }
}
