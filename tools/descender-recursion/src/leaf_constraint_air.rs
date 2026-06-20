//! Gadget 4 (proper, PART 2) — the leaf `C(zeta)` MIRROR: evaluating a SUBSET of the leaf
//! [`descender_account_stark::air::FullRelationAir`] constraint polynomials at the DEEP
//! out-of-domain point `zeta`, as a real `Air<AB>`.
//!
//! ## Why this exists
//!
//! [`crate::deep_air`] checks the DEEP-ALI quotient relation `C(zeta) == Z_H(zeta)·Q(zeta)`
//! but treats `C(zeta)` (the leaf AIR constraint-polynomial evaluation at `zeta`) as an
//! OPENED / COMMITTED value — i.e. it trusts the prover's claimed `C(zeta)`. The full DEEP
//! soundness requires `C(zeta)` to be RE-COMPUTED in-circuit from the opened trace cells
//! `T_i(zeta)`, so the prover cannot smuggle a favorable `C(zeta)`. That re-computation is the
//! entire leaf folder (all of `air.rs`: C1/C2/C4/C6 + Poseidon2 perms + reduce + cross-seams),
//! the single largest remaining recursion piece.
//!
//! ## Scope of THIS module (the Poseidon2-FREE leaf constraints, alpha-batched)
//!
//! This mirrors EXACTLY the algebraic, permutation-free leaf gadgets (degree ≤ 2, same-row):
//!
//!   1. **C4 canonical base-128 residue gadget** (`FullRelationAir::eval_c4_residue`,
//!      air.rs:1381) for ALL `C4_LANES = 6` lane blocks (29 residuals each ⇒ 174).
//!   2. **C4-acc carrier binding** (air.rs:762): `carry_acc[j] == v_j` for each lane (6).
//!   3. **SeamPadSeed** (air.rs:765-771): C1's 32 `pad_seed` lanes == C2's 32 `pad_seed`
//!      lanes (32 same-row equalities).
//!   4. **SeamOpeningAcc** (air.rs:773-779): C4's 6 `carry_acc` values == C6's 6 field lanes
//!      (6 same-row equalities).
//!   5. **relation_id pinning** (CLASS B, air.rs:957-967): the 4 public `relation_id` lanes
//!      pinned to the constant [`RELATION_ID`] (4 public pinnings).
//!
//! Each constraint is one extension-valued zero-residual `c_i(T(zeta))`, computed over the
//! OPENED cells (each an extension element `Challenge`, opened at `zeta`). We then form the
//! **alpha-batched** sum mirroring `p3-uni-stark`'s `SymbolicAirBuilder`/`accumulate`:
//!
//!   `C_partial(zeta) = Σ_i alpha^i · c_i(T(zeta))`
//!
//! over the SAME octic (`X^8 = W`) extension arithmetic as [`crate::deep_air`] /
//! [`crate::fri_fold_air`]. The AIR binds the committed `C_partial` cell to this recomputed sum.
//!
//! ## Fidelity (the mirror MUST equal air.rs EXACTLY)
//!
//! A WRONG mirror is a soundness hole, so [`tests::mirror_matches_leaf_air_on_real_row`] and the
//! per-class tamper tests pin the honest recomputed residuals against the leaf AIR's own
//! `eval_c4_residue` / seam / relation_id semantics evaluated on REAL packed leaf rows, and the
//! alpha-batched honest `C_partial` against an independent native reference
//! [`partial_constraint_eval`]. Every opened-cell tamper that violates a mirrored leaf
//! constraint changes `C_partial` ⇒ rejects.
//!
//! ## MIRRORED Poseidon2 sponge (the FIRST sponge in-circuit)
//!
//!   * **C1 — `pad_source_commitment` squeeze** (air.rs:712-726, `eval_sponge` C1 plan):
//!     RE-COMPUTED in-circuit via [`crate::inlined_perm_octic`] over the absorbed leaf cells,
//!     matching the leaf `SpongePlan` (rate-8 absorb with the ONE padding marker at stream pos
//!     `n_input` + zero-fill, capacity-preserving across 7 absorb perms + 1 squeeze perm), with the
//!     committed `pad_source_commitment[10]` digest constrained == the squeezed sponge output. Added
//!     as the `N_C1` residual class (input/capacity bindings ‖ 8 Poseidon2 perm blocks ‖
//!     squeeze-output equalities). Fidelity-PINned against the deployed byte-pinned Val permutation
//!     (`c1_honest_sponge_all_residuals_zero`, `c1_squeeze_matches_deployed_perm_output`) and tamper-
//!     rejected end-to-end through DEEP (`leaf_wired_tampered_c1_sponge_cell_rejects_end_to_end`).
//!     The per-lane BYTE-RANGE decomposition of the 48 C1 input lanes (air.rs:1278-1289) is now
//!     MIRRORED in the `N_BYTERANGE` class (see below), binding the SAME `c1_input_cell` lanes.
//!
//!   * **C6 — `opening_tuple_commitment` squeeze** (air.rs:740-756, `eval_sponge` C6 plan):
//!     RE-COMPUTED in-circuit via [`crate::inlined_perm_octic`] over the absorbed leaf cells,
//!     matching the leaf `SpongePlan` (rate-8 absorb of account_projection[6] FIELD lanes ‖
//!     lifecycle[32] ‖ nonce_c6[16] ‖ account_id[32] byte-lanes = 86 input lanes, ONE padding marker
//!     at stream pos `n_input`(=86) + zero-fill, capacity-preserving across 11 absorb perms + 1
//!     squeeze perm = 12 perms), with the committed `opening_tuple_commitment[10]` digest constrained
//!     == the squeezed sponge output. Added as the `N_C6` residual class. The 6 FIELD absorb lanes
//!     are the EXISTING `O_C6_FIELD` SeamOpeningAcc cells (reused). Fidelity-PINned against the
//!     deployed byte-pinned Val permutation (`c6_honest_sponge_all_residuals_zero`,
//!     `c6_squeeze_matches_deployed_perm_output`) and tamper-rejected end-to-end through DEEP
//!     (`leaf_wired_tampered_c6_sponge_cell_rejects_end_to_end`). The per-lane BYTE-RANGE
//!     decomposition of the 80 C6 byte-lanes (air.rs:1278-1289) is now MIRRORED in the `N_BYTERANGE`
//!     class (the 6 FIELD lanes are NOT byte-ranged), binding the SAME `c6_byte_cell` lanes.
//!
//!   * **role-root sponge — `role_root` squeeze** (CLASS A, air.rs:908-954, plan air.rs:441):
//!     RE-COMPUTED in-circuit via [`crate::inlined_perm_octic`] over the 26 ALL-FIELD role-handle
//!     absorb lanes (NO byte-range): pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖
//!     masked_projection[6], ONE padding marker at stream pos `n_input`(=26) + zero-fill,
//!     capacity-preserving across 4 absorb perms (n_out=8 ≤ RATE ⇒ 0 squeeze perms), with the
//!     committed `role_root[8]` digest constrained == the squeezed sponge output. Added as the
//!     `N_ROLE` residual class. CHAINED: the 20 commitment input lanes are bound to the ALREADY-
//!     MIRRORED C1/C6 squeeze-output cells (`c1_out_cell`/`c6_out_cell`), NOT fresh columns — so the
//!     role input IS the genuine C1/C6 output (air.rs:919-931). Fidelity-PINned against the deployed
//!     byte-pinned Val permutation (`role_honest_sponge_all_residuals_zero`,
//!     `role_squeeze_matches_deployed_perm_output_over_chained_inputs`) and tamper-rejected end-to-end
//!     through DEEP (`leaf_wired_tampered_role_root_sponge_cell_rejects_end_to_end`), INCLUDING a
//!     tampered C1/C6 output that breaks the role-root via the chaining
//!     (`role_air_rejects_tampered_chained_c1_c6_output`).
//!
//!   * **C2 — mask-PRF sponge + WIDE-THEN-REDUCE + carrier** (air.rs:728-737, 856-906;
//!     reduce_gadget.rs): the LARGEST mirrored sponge. RE-COMPUTED in-circuit via
//!     [`crate::inlined_perm_octic`] over the 40 absorb lanes (pad_seed[32] REUSED from the C2
//!     SeamPadSeed cells ‖ slot_id[8] NEW), ONE padding marker at stream pos n_input(=40) + zero-fill,
//!     capacity-preserving across 6 absorb perms + 3 squeeze perms = 9 perms; the 30 squeezed lanes
//!     are read from the squeeze-source perms' OUTPUT cells (NO public digest, so no squeeze-output
//!     equality sub-class). Added as the `N_C2` residual class. The 30 squeezed lanes then feed the
//!     genuinely-NEW WIDE-THEN-REDUCE gadget (mirror of `reduce_gadget::eval_reduce_gadget`: per-mask
//!     base-2^16 limb/carry wrap-free Horner reduction `mask_j = (Σ_i lane_{j,i}·p^i) mod q`) + the
//!     C4 carrier mod-q reduction (`masked_projection[j] == account_projection[j] + mask[j] mod q`),
//!     added as the `N_REDUCE` residual class. Fidelity-PINned against the deployed reduce gadget
//!     (`reduce_masks_match_leaf_reduce_and_carrier`, `c2_honest_sponge_all_residuals_zero`,
//!     `reduce_honest_all_residuals_zero`) and tamper-rejected end-to-end through DEEP
//!     (`leaf_wired_tampered_c2_reduce_cell_rejects_end_to_end`): forged squeezed lane / forged mask /
//!     tampered limb-carry / forged masked_projection / tampered carrier mod-q witness all reject.
//!     The per-lane BYTE-RANGE decomposition of the 40 C2 input lanes (air.rs:1278-1289) is now
//!     MIRRORED in the `N_BYTERANGE` class, binding the SAME `c2_pad_cell`/`c2_slot_cell` lanes.
//!
//!   * **PER-LANE BYTE-RANGE / canonicality** (air.rs:1278-1289): for EVERY byte-ranged input lane
//!     of the C1/C2/C6 sponges (C1 all 48 ‖ C2 all 40 ‖ C6 the 80 byte-lanes; role has none), the
//!     degree-2 gadget `BYTE_BITS bit-bools bit·(bit−1) + recompose eq lane − Σ bit·2^b`. The
//!     decomposed lane is the ACTUAL already-absorbed sponge input cell
//!     (`byterange_absorbed_cell`), NOT a fresh column — so this is the canonicality (each absorbed
//!     preimage lane is a valid byte) of the genuine sponge input, and a vacuous range check is ruled
//!     out by `byterange_recompose_binds_real_absorbed_lane`. Added as the `N_BYTERANGE` class (the
//!     LAST class). Fidelity-PINned (`byterange_honest_all_residuals_zero`,
//!     `byterange_shape_matches_leaf`) and tamper-rejected (`byterange_air_rejects_tampers`:
//!     non-boolean bit, recompose mismatch, boolean-but-wrong-sum) + at the mirror level
//!     (`byterange_rejects_nonboolean_bit`). This FINISHES the PER-ROW algebraic leaf C(zeta) mirror.
//!
//! ## DEFERRED (the rest of leaf `C(zeta)` — STILL committed-not-constrained)
//!
//!   With the byte-range class added, ALL per-row leaf gadgets — C1/C6/role/C2 Poseidon2 sponges,
//!   the C2 wide-then-reduce + carrier, the C4 canonical base-128 residue gadget, the seams /
//!   relation_id pinning, the batch digest chain, AND the per-lane byte-range/canonicality of the
//!   sponge inputs — are now RE-COMPUTED in `recompute_c_partial`. What remains is NOT a per-row
//!   algebraic class but the ARCHITECTURAL multi-row scale-up:
//!
//!   * the SINGLE-ROW harness → a REAL opened multi-row trace (each class evaluated per opened row);
//!   * the boundary selectors (first-row `chain_in==0`, last-row `chain_out==public digest`,
//!     `count_acc==public N`) + the monotone-enable prefix constraint — they need
//!     `is_first_row`/`is_last_row` selectors + STARK public values the single-row mirror does not
//!     carry (see the batch-digest-chain HONESTLY-FLAGGED SCOPE LIMIT note);
//!   * the beta/zeta Fiat-Shamir binding of `alpha`/`zeta`/the public digest into the recursion FS.
//!   * the C2/C6 sponge-OUTPUT commitment equalities (`c*_out[j] == pubio[j]`) ride with their perms.
//!
//! Each remaining sponge reuses the SAME [`crate::inlined_perm_octic`] octic-perm primitive + the
//! absorb/capacity/squeeze pattern established here for C1.
//!
//! ## Wiring into DEEP
//!
//! [`crate::deep_air`] now consumes this `C_partial` as a CONSTRAINED summand of the DEEP
//! `C(zeta)`: `C = C_partial + C_committed`, with `C == Z_H·Q`. A tampered opened leaf cell that
//! violates a mirrored constraint changes `C_partial`, breaking the OOD identity ⇒ rejected
//! END-TO-END. The `C_committed` summand is the deferred Poseidon2-perm bulk (still committed).

use crate::fri_fold_air::{limbs, D, W};
use crate::inlined_perm::{C_INPUTS, C_OUTPUT, PERM_COLS};
use crate::inlined_perm_octic::{self, OcticPermCells};
use descender_account_stark::{
    Challenge, Val, N_COMMIT_LANES, N_LANES, PUBIO_ROLE_ROOT_OFF, RELATION_ID, RELATION_ID_ELEMS,
    ROLE_ROOT_ELEMS, WIDTH,
};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

// ----- leaf C4 gadget constants (mirror air.rs:201-204, 292) -----

/// Base-128 radix (`d = d_lo + 128·d_hi`). Mirrors air.rs `C4_BASE`.
const C4_BASE: u32 = 128;
/// Bits per base-128 digit. Mirrors air.rs `C4_DIGIT_BITS`.
const C4_DIGIT_BITS: usize = 7;
/// Canonical max high digit: `q12289 = 96·128 + 1 ⇒ d_hi ∈ [0,96]`. Mirrors air.rs `C4_MAX_HI`.
const C4_MAX_HI: u32 = 96;
/// Cells in one leaf C4 lane block: `v ‖ d_lo ‖ d_hi ‖ lo_bits[7] ‖ hi_bits[7] ‖ slack_bits[7]
/// ‖ is96 ‖ inv`. Mirrors air.rs `C4_CELLS = 3 + 3*7 + 2 = 26`.
pub const C4_CELLS: usize = 3 + 3 * C4_DIGIT_BITS + 2; // 26

/// Number of C4 lane blocks (= projection lanes). Mirrors air.rs `N_LANES`.
pub const C4_LANES: usize = N_LANES; // 6

/// Number of distinct C4 constraint residuals batched PER LANE: 21 bit-booleans (lo[7] ‖ hi[7] ‖
/// slack[7]) + 8 algebraic (lo-recompose, hi-recompose, slack-recompose, is96-bool,
/// is96·(d_hi−96), inverse-corner, is96·d_lo, v-recompose).
pub const C4_N_PER_LANE: usize = 21 + 8; // 29

// Within-block cell indices (mirror air.rs `eval_c4_residue` local offsets).
const I_V: usize = 0;
const I_DLO: usize = 1;
const I_DHI: usize = 2;
const I_LO_BITS: usize = 3;
const I_HI_BITS: usize = 10;
const I_SLACK_BITS: usize = 17;
const I_IS96: usize = 24;
const I_INV: usize = 25;

// ----- the other algebraic (Poseidon2-free) leaf constraints -----

/// SeamPadSeed: C1's 32 `pad_seed` lanes == C2's 32 `pad_seed` lanes (air.rs:765-771).
pub const PAD_SEED_BYTES: usize = 32;
/// Number of constraint residuals contributed by each mirrored class.
pub const N_C4_ACC: usize = C4_LANES; // carry_acc[j] == v_j (air.rs:762)
pub const N_SEAM_PAD: usize = PAD_SEED_BYTES; // SeamPadSeed
pub const N_SEAM_ACC: usize = N_LANES; // SeamOpeningAcc (air.rs:773-779)
pub const N_RELATION_ID: usize = RELATION_ID_ELEMS; // relation_id pinning (air.rs:957-967)

// ----- C1 sponge (pad_source_commitment squeeze) — the FIRST mirrored leaf sponge -----
//
// Mirrors `descender_account_stark::air::FullRelationAir::eval_sponge` for the C1 SpongePlan:
//   * C1 input lanes: pad_seed[32] ‖ blinding_nonce_c1[16]  ⇒ C1_INPUT_LANES = 48.
//   * RATE-8 absorb with the ONE padding marker at stream position `n_input` (=48) and zero-fill;
//     capacity-preserving (slots RATE..WIDTH carried from the previous perm's output).
//   * 7 absorb perms + 1 squeeze perm = 8 Poseidon2 perms.
//   * squeeze: `pad_source_commitment[0..7]` from the LAST absorb perm's output, `[8..9]` from the
//     squeeze perm's output (matches `SpongePlan::output_source`).
// The committed `pad_source_commitment[10]` digest is the leaf's PUBLIC C1 output (air.rs:712-726);
// the mirror constrains it == the squeezed sponge output.
//
// DEFERRED for the C1 sponge: the per-lane BYTE-RANGE decomposition of the 48 input lanes
// (air.rs:1278-1289 — `assert_bool` over 8 bits/lane + lane==Σ bit·2^b). That is a degree-2
// algebraic gadget (not the Poseidon2 sponge per se); it rides with the other byte-range / reduce
// gadgets in a later chunk. The C1 sponge mirror here constrains the absorb/capacity wiring, the
// Poseidon2 perm rounds, and the squeeze-output equality.

/// C1 input byte-lanes count: pad_seed[32] ‖ blinding_nonce_c1[16]. Mirrors air.rs `C1_INPUT_LANES`.
pub const C1_INPUT_LANES: usize = 32 + 16; // 48
/// Sponge rate (RATE elements absorbed per permutation). Mirrors air.rs `RATE`.
pub const C1_RATE: usize = 8;
/// Number of absorb permutations for the C1 sponge: `ceil((n_input + 1)/RATE)` = `ceil(49/8)` = 7.
pub const C1_N_ABSORB: usize = 7;
/// Total C1 sponge permutations (absorb + squeeze). `n_out = 10` ⇒ 1 squeeze perm ⇒ 8.
pub const C1_N_PERMS: usize = 8;
/// Padded absorb-stream length (`C1_N_ABSORB · RATE`).
pub const C1_PADDED_LEN: usize = C1_N_ABSORB * C1_RATE; // 56
/// pad_source_commitment output lanes. Mirrors air.rs `N_COMMIT_LANES`.
pub const C1_N_OUT: usize = N_COMMIT_LANES; // 10

/// C1 residual count: per perm [RATE+ (WIDTH-RATE) input/capacity bindings] + perm round residuals,
/// then the squeeze-output equalities.
pub const N_C1_INPUT_BIND: usize = C1_N_PERMS * WIDTH; // 16 input/cap bindings per perm
pub const N_C1_PERM: usize = C1_N_PERMS * inlined_perm_octic::PERM_RESIDUALS;
pub const N_C1_OUT: usize = C1_N_OUT;
pub const N_C1: usize = N_C1_INPUT_BIND + N_C1_PERM + N_C1_OUT;

// ----- C6 sponge (opening_tuple_commitment squeeze) — the SECOND mirrored leaf sponge -----
//
// Mirrors `descender_account_stark::air::FullRelationAir::eval_sponge` for the C6 SpongePlan
// (air.rs:740-756, plan built at air.rs:436-438). C6 has the SAME absorb/capacity/squeeze shape
// as C1 (rate-8 absorb with the ONE padding marker at stream position `n_input` + zero-fill,
// capacity-preserving across the absorb perms + 1 squeeze perm), only with a LARGER input:
//   * C6 input lanes: account_projection[6] (FIELD lanes) ‖ lifecycle_anchor[32] ‖
//     blinding_nonce_c6[16] ‖ account_id[32] (byte-lanes)  ⇒ C6_INPUT_LANES = 6 + 80 = 86.
//   * RATE-8 absorb, padding marker at pos n_input(=86), zero-fill ⇒ padded = ceil(87/8)·8 = 88
//     ⇒ C6_N_ABSORB = 11.  n_out = 10 ⇒ 1 squeeze perm ⇒ C6_N_PERMS = 12.
//   * squeeze: `opening_tuple_commitment[0..7]` from the LAST absorb perm's output, `[8..9]` from
//     the squeeze perm's output (same `SpongePlan::output_source` mapping as C1, since n_out=10).
// The committed `opening_tuple_commitment[10]` digest is the leaf's PUBLIC C6 output (air.rs:747-756);
// the mirror constrains it == the squeezed sponge output.
//
// The FIRST 6 C6 absorb lanes are the `account_projection` FIELD lanes, which are ALREADY present in
// the mirror as the `O_C6_FIELD` cells (the SeamOpeningAcc lanes — `local[c6_off + c6.lanes + i]`,
// i<6, air.rs:777). They are REUSED as absorb positions 0..6. The remaining 80 byte-lanes
// (lifecycle ‖ nonce ‖ account_id) are added as new opened cells here.
//
// DEFERRED for the C6 sponge (same as C1): the per-lane BYTE-RANGE decomposition of the 80
// byte-lanes (air.rs:1278-1289) — a degree-2 algebraic gadget, not the sponge; rides with the
// byte/reduce gadgets in a later chunk. (The 6 FIELD lanes are NOT byte-ranged in air.rs.)

/// C6 input lanes count: account_projection[6] ‖ lifecycle[32] ‖ nonce_c6[16] ‖ account_id[32].
/// Mirrors air.rs `C6_INPUT_LANES = C6_FIELD_LANES + C6_BYTE_LANES`.
pub const C6_FIELD_LANES: usize = N_LANES; // 6 (the account_projection field lanes)
pub const C6_BYTE_LANES: usize = 32 + 16 + 32; // 80 (lifecycle ‖ nonce_c6 ‖ account_id)
pub const C6_INPUT_LANES: usize = C6_FIELD_LANES + C6_BYTE_LANES; // 86
/// Sponge rate (shared with C1). Mirrors air.rs `RATE`.
pub const C6_RATE: usize = 8;
/// Number of absorb permutations: `ceil((n_input + 1)/RATE)` = `ceil(87/8)` = 11.
pub const C6_N_ABSORB: usize = 11;
/// Total C6 sponge permutations (absorb + squeeze). `n_out = 10` ⇒ 1 squeeze perm ⇒ 12.
pub const C6_N_PERMS: usize = 12;
/// Padded absorb-stream length (`C6_N_ABSORB · RATE`).
pub const C6_PADDED_LEN: usize = C6_N_ABSORB * C6_RATE; // 88
/// opening_tuple_commitment output lanes. Mirrors air.rs `N_COMMIT_LANES`.
pub const C6_N_OUT: usize = N_COMMIT_LANES; // 10

/// C6 residual count: input/capacity bindings (WIDTH/perm) + perm round residuals + squeeze-output
/// equalities — same structure as C1, with C6's perm count.
pub const N_C6_INPUT_BIND: usize = C6_N_PERMS * WIDTH;
pub const N_C6_PERM: usize = C6_N_PERMS * inlined_perm_octic::PERM_RESIDUALS;
pub const N_C6_OUT: usize = C6_N_OUT;
pub const N_C6: usize = N_C6_INPUT_BIND + N_C6_PERM + N_C6_OUT;

// ----- role-root sponge (CLASS A) — the THIRD mirrored leaf sponge, CHAINED to C1/C6 -----
//
// Mirrors `descender_account_stark::air::FullRelationAir`'s CLASS-A role-root sponge
// (air.rs:908-954, plan built at air.rs:441 `role_plan = SpongePlan::new(vec![false; 26], 8)`).
// The role-root sponge is the cleanest sponge: ALL-FIELD 26 input lanes (NO byte-range), a single
// 8-lane squeeze, and its INPUTS are the C1/C6 commitment squeeze outputs — so it CHAINS the two
// already-mirrored sponges:
//   * role input lanes 0..10  == `pad_source_commitment[10]`  ≡ the C1 squeeze output cells.
//   * role input lanes 10..20 == `opening_tuple_commitment[10]` ≡ the C6 squeeze output cells.
//   * role input lanes 20..26 == `masked_projection[6]`         (NEW opened cells).
// We do NOT introduce fresh committed columns for the first 20 input lanes; the role absorb reads
// the ALREADY-MIRRORED `c1_out_cell(j)` / `c6_out_cell(j)` cells directly (air.rs:919-931 binds the
// role sponge's input lanes EQUAL to the real commitment columns, which here ARE the mirrored C1/C6
// outputs). Tampering a C1/C6 output thus ALSO breaks the role-root — the chaining.
//
//   * RATE-8 absorb, padding marker at pos n_input(=26), zero-fill ⇒ padded = ceil(27/8)·8 = 32
//     ⇒ ROLE_N_ABSORB = 4.  n_out = 8 ≤ RATE ⇒ 0 squeeze perms ⇒ ROLE_N_PERMS = 4.
//   * squeeze: `role_root[0..8]` all from the LAST absorb perm (perm 3), lanes 0..8
//     (`SpongePlan::output_source` with n_out=8 ≤ RATE).
// The committed `role_root[8]` digest is the leaf's PUBLIC CLASS-A output (air.rs:941-953); the
// mirror constrains it == the squeezed sponge output.

/// Role-handle input lanes: pad_source_commitment[10] ‖ opening_tuple_commitment[10] ‖
/// masked_projection[6]. Mirrors air.rs `ROLE_HANDLE_LANES`.
pub const ROLE_INPUT_LANES: usize = N_COMMIT_LANES + N_COMMIT_LANES + N_LANES; // 26
/// Role masked_projection input lanes (the only NEW role input cells; the first 20 are the chained
/// C1/C6 squeeze outputs).
pub const ROLE_MASKED_LANES: usize = N_LANES; // 6
/// Sponge rate (shared with C1/C6). Mirrors air.rs `RATE`.
pub const ROLE_RATE: usize = 8;
/// Number of absorb permutations: `ceil((n_input + 1)/RATE)` = `ceil(27/8)` = 4.
pub const ROLE_N_ABSORB: usize = 4;
/// Total role-root sponge permutations (absorb + squeeze). `n_out = 8 ≤ RATE` ⇒ 0 squeeze ⇒ 4.
pub const ROLE_N_PERMS: usize = 4;
/// Padded absorb-stream length (`ROLE_N_ABSORB · RATE`).
pub const ROLE_PADDED_LEN: usize = ROLE_N_ABSORB * ROLE_RATE; // 32
/// Squeezed role_root output lanes. Mirrors air.rs `ROLE_ROOT_ELEMS`.
pub const ROLE_N_OUT: usize = ROLE_ROOT_ELEMS; // 8

/// Role-root residual count: input/capacity bindings (WIDTH/perm) + perm round residuals +
/// squeeze-output equalities — same structure as C1/C6, with the role perm count.
pub const N_ROLE_INPUT_BIND: usize = ROLE_N_PERMS * WIDTH;
pub const N_ROLE_PERM: usize = ROLE_N_PERMS * inlined_perm_octic::PERM_RESIDUALS;
pub const N_ROLE_OUT: usize = ROLE_N_OUT;
pub const N_ROLE: usize = N_ROLE_INPUT_BIND + N_ROLE_PERM + N_ROLE_OUT;
// ----- C2 sponge (mask-PRF squeeze) — the FOURTH mirrored leaf sponge, the LARGEST -----
//
// Mirrors `descender_account_stark::air::FullRelationAir::eval_sponge` for the C2 SpongePlan
// (air.rs:728-737, plan built at air.rs:435 `c2_plan = SpongePlan::new(vec![true; C2_INPUT_LANES],
// NUM_SQUEEZE)`). C2 has the SAME absorb/capacity/squeeze shape as C1/C6 (rate-8 absorb with the ONE
// padding marker at stream position `n_input` + zero-fill, capacity-preserving across the absorb
// perms + the squeeze perms), but it SQUEEZES 30 lanes (not 10) and its output feeds the
// WIDE-THEN-REDUCE gadget + carrier (not a public commitment):
//   * C2 input lanes: pad_seed[32] ‖ slot_id[8]  ⇒ C2_INPUT_LANES = 40. The 32 pad_seed lanes are
//     the EXISTING `O_C2_PAD` cells (the SeamPadSeed C2 lanes, reused as absorb positions 0..32);
//     the 8 slot_id lanes are NEW opened cells.
//   * RATE-8 absorb, padding marker at pos n_input(=40), zero-fill ⇒ padded = ceil(41/8)·8 = 48
//     ⇒ C2_N_ABSORB = 6.  n_out = 30 ⇒ ceil over RATE: outputs 0..8 from the LAST absorb perm
//     (perm 5), 8..16 from squeeze perm 6, 16..24 from squeeze perm 7, 24..30 from squeeze perm 8
//     ⇒ 3 squeeze perms ⇒ C2_N_PERMS = 9 (matches `SpongePlan::output_source` for n_out=30).
// The 30 squeezed lanes are then reduced to 6 masks by the wide-then-reduce gadget (the
// `N_REDUCE` class below); they are NOT a public digest, so the C2 sponge class has NO
// squeeze-output equality sub-class (unlike C1/C6/role). Instead the reduce gadget reads the
// squeezed OUTPUT cells directly (`c2_squeeze_output_cell`), and the squeeze soundness is enforced
// by the per-perm round residuals binding those output cells to the recomputed permutation.

/// C2 input lanes count: pad_seed[32] ‖ slot_id[8]. Mirrors air.rs `C2_INPUT_LANES`.
pub const C2_PAD_LANES: usize = PAD_SEED_BYTES; // 32 (the SeamPadSeed C2 pad_seed cells, reused)
pub const C2_SLOT_LANES: usize = 8; // slot_id[8] (NEW absorb cells)
pub const C2_INPUT_LANES: usize = C2_PAD_LANES + C2_SLOT_LANES; // 40
/// Sponge rate (shared with C1/C6/role). Mirrors air.rs `RATE`.
pub const C2_RATE: usize = 8;
/// Number of absorb permutations: `ceil((n_input + 1)/RATE)` = `ceil(41/8)` = 6.
pub const C2_N_ABSORB: usize = 6;
/// Squeezed output lanes. Mirrors `reduce_gadget::NUM_SQUEEZE`.
pub const C2_N_OUT: usize = 30;
/// Total C2 sponge permutations (absorb + squeeze). n_out=30 ⇒ 3 squeeze perms ⇒ 9.
pub const C2_N_PERMS: usize = 9;
/// Padded absorb-stream length (`C2_N_ABSORB · RATE`).
pub const C2_PADDED_LEN: usize = C2_N_ABSORB * C2_RATE; // 48

/// C2 sponge residual count: input/capacity bindings (WIDTH/perm) + per-perm Poseidon2 round
/// residuals. NO squeeze-output equality sub-class (the squeezed lanes feed the reduce gadget, not a
/// public digest; their soundness is the per-perm round residuals binding the OUTPUT cells).
pub const N_C2_INPUT_BIND: usize = C2_N_PERMS * WIDTH;
pub const N_C2_PERM: usize = C2_N_PERMS * inlined_perm_octic::PERM_RESIDUALS;
pub const N_C2: usize = N_C2_INPUT_BIND + N_C2_PERM;

// ----- WIDE-THEN-REDUCE + carrier gadget (the genuinely-NEW constraint shape) -----
//
// Mirrors `descender_account_stark::reduce_gadget::eval_reduce_gadget` EXACTLY (the 30 squeezed
// lanes → 6 q12289 masks via a base-2^16 limb/carry wrap-free Horner reduction `mask_j =
// (Σ_i lane_{j,i}·p^i) mod q`), PLUS the C4 carrier mod-q reduction (air.rs:856-906,
// `masked_projection[j] == account_projection[j] + mask[j] mod q12289`). The reduce gadget is the
// soundness gadget: a wrong limb/carry / mod-q reduction is a hole, so it is fidelity-PINned against
// `reduce_gadget::fill_reduce_gadget`/`wide_reduce_masks` on a real honest row.
//
// All `reduce_gadget` shape constants are MIRRORED here (so this file is self-contained and the
// layout is pinned by `reduce_shape_matches_leaf`):
pub use crate::reduce_consts::*;

// ----- BATCH DIGEST CHAIN (the per-row running Poseidon2 sponge + the CROSS-ROW transition) -----
//
// Mirrors `descender_account_stark::air::FullRelationAir::eval_digest_chain` (air.rs:1109-1260) +
// the off-circuit reference `descender_account_stark::air::chain_digest` / the trace-fill in
// `generate_batch_trace` (lib.rs:1899-1936). This is the LAST large committed-not-constrained piece
// of the leaf `C(zeta)` mirror, and it differs from the per-row sponges (C1/C2/C6/role) because the
// running digest carries ACROSS ROWS: row r's `chain_in` is row (r-1)'s `chain_out`.
//
// Per row the absorbed message is `m = [enable, enable·pubio[0..PUBIO_ELEMS]]`, zero-padded to
// `CHAIN_PERMS_PER_ROW · CHAIN_RATE`. The chain step absorbs `m` into the 16-wide running state
// `chain_in` across `CHAIN_PERMS_PER_ROW` Poseidon2 perms (rate-8 absorb, capacity carried), giving
// `absorbed = last-perm output`; then `chain_out = enable ? absorbed : chain_in` (padding rows are
// TRUE no-ops). The cross-row binding is `chain_in == prev_chain_out` and the count accumulator is
// `count_acc == prev_count_acc + enable`.
//
// ## How the cross-row PREV value is bound (the NEW shape) — READ THIS
//
// The recursion `C(zeta)` mirror is a SINGLE-ROW harness (one opened OOD row's cells). A genuine
// multi-row STARK transition (`next.chain_in == cur.chain_out`, accessed via the two-row window)
// is NOT expressible in this single-row folder. We therefore model the transition the way air.rs
// relates the two rows, but with the PREVIOUS row's `chain_out` carried as its OWN opened cell
// block `prev_chain_out[16]` (and `prev_count_acc`):
//   * `chain_in[c] - prev_chain_out[c]`        (the carry; mirror of `next.chain_in==cur.chain_out`)
//   * `count_acc   - (prev_count_acc + enable)` (mirror of `next.count_acc==cur.count_acc+next.enable`)
// `prev_chain_out`/`prev_count_acc` ARE real bound cells (their own columns), and these residuals
// bind `chain_in`/`count_acc` to them — so within the harness the carry is NOT free: a tampered
// `prev_chain_out` (a swapped/altered prior-row digest) breaks the transition residual and rejects.
// In a real opened multi-row trace these cells WOULD be the opened previous-row `chain_out`/
// `count_acc` (which a real DEEP opening exposes), so the residual is the genuine cross-row binding.
//
// HONESTLY-FLAGGED SCOPE LIMIT: because this is a single-row harness, the only thing NOT modeled
// here (vs the full multi-row AIR) is (i) the boundary selectors — first-row `chain_in==0` /
// `count_acc==enable`, last-row `chain_out[0..8]==public digest` / `count_acc==public N` — and
// (ii) the monotone-enable prefix constraint `next.enable·(1-cur.enable)==0`. Those depend on
// `is_first_row`/`is_last_row` selectors + STARK public values that the single-row recursion mirror
// does not carry; they ride with the eventual scaling to the REAL opened multi-row trace + the
// beta/zeta FS public-digest binding. The per-row absorb, the running fold, the no-op padding
// branch, the cross-row carry, and the count accumulator ARE modeled and fidelity-PINned here.

/// Chain running-state width (= PERM_WIDTH = 16, the Poseidon2 perm WIDTH). Mirrors `chain_width`.
pub const CHAIN_W: usize = WIDTH; // 16
/// Number of pubio field elements absorbed per row (the per-instance statement). Mirrors air.rs.
pub const CHAIN_PUBIO: usize = descender_account_stark::PUBIO_ELEMS;
/// Sponge rate for the chain (= CHAIN_RATE = RATE = 8). Mirrors air.rs.
pub const CHAIN_RT: usize = descender_account_stark::CHAIN_RATE; // 8
/// Permutations per row to absorb `1 + PUBIO_ELEMS` elems at CHAIN_RATE. Mirrors air.rs.
pub const CHAIN_NPERMS: usize = descender_account_stark::CHAIN_PERMS_PER_ROW;
/// Padded per-row absorb-stream length (`CHAIN_NPERMS · CHAIN_RT`).
pub const CHAIN_PADDED_LEN: usize = CHAIN_NPERMS * CHAIN_RT;

/// Chain residual count:
///   * 1 enable-boolean
///   * input/capacity bindings (CHAIN_W per perm)
///   * per-perm Poseidon2 round residuals
///   * chain_out binding (CHAIN_W)
///   * cross-row carry: chain_in == prev_chain_out (CHAIN_W)
///   * count accumulator: count_acc == prev_count_acc + enable (1)
pub const N_CHAIN_ENABLE: usize = 1;
pub const N_CHAIN_INPUT_BIND: usize = CHAIN_NPERMS * CHAIN_W;
pub const N_CHAIN_PERM: usize = CHAIN_NPERMS * inlined_perm_octic::PERM_RESIDUALS;
pub const N_CHAIN_OUT_BIND: usize = CHAIN_W;
pub const N_CHAIN_CARRY: usize = CHAIN_W;
pub const N_CHAIN_COUNT: usize = 1;
pub const N_CHAIN: usize = N_CHAIN_ENABLE
    + N_CHAIN_INPUT_BIND
    + N_CHAIN_PERM
    + N_CHAIN_OUT_BIND
    + N_CHAIN_CARRY
    + N_CHAIN_COUNT;

/// C2 sponge cells: ONLY the 8 NEW slot_id input lanes (pad_seed reused from `O_C2_PAD`) + perm
/// blocks. NO output digest cells (the 30 squeezed lanes are read from the perm OUTPUT cells).
pub const C2_CELLS: usize = C2_SLOT_LANES + C2_N_PERMS * PERM_COLS;

/// Reduce-gadget + carrier cells: the gadget's own witness columns (`REDUCE_CELLS`) + the 6
/// masked_projection carrier values + the 6 carrier mod-q witness blocks (`MODQ_CELLS` each).
/// account_projection (the carrier `acc`) is the EXISTING `O_ACC` carry_acc cells (reused).
pub const CARRIER_CELLS: usize = N_LANES + N_LANES * MODQ_CELLS;
pub const REDUCE_BLOCK_CELLS: usize = REDUCE_CELLS + CARRIER_CELLS;

/// Batch-digest-chain opened cells: prev_chain_out[16] ‖ chain_in[16] ‖ CHAIN_NPERMS perm blocks ‖
/// chain_out[16] ‖ enable[1] ‖ prev_count_acc[1] ‖ count_acc[1] ‖ pubio[PUBIO_ELEMS].
pub const CHAIN_CELLS: usize = CHAIN_W // prev_chain_out
    + CHAIN_W // chain_in
    + CHAIN_NPERMS * PERM_COLS // per-row absorb perms
    + CHAIN_W // chain_out
    + 1 // enable
    + 1 // prev_count_acc
    + 1 // count_acc
    + CHAIN_PUBIO; // absorbed pubio lanes

/// N_REDUCE residual count. Mirrors `eval_reduce_gadget` assertion count per mask + the carrier:
///   per mask: 5 lane-bind + per step [11 value-recompose + 11·(bit-bool over their widths) +
///   r_t-recompose + slack-recompose + (r+slack==q-1) + digit-split + 6 limb/carry eqs] ; then the
///   carrier per lane [masked-recompose + slack-recompose + (m+slack==q-1) + wrap-bool + carrier eq].
/// Encoded structurally in `reduce_residual` (we count it there); exposed as a const via the same
/// arithmetic so `N_CONSTRAINTS` is a const.
pub const N_REDUCE: usize = NUM_OUTPUTS * N_REDUCE_PER_MASK + N_LANES * N_CARRIER_PER_LANE;

// ----- PER-LANE BYTE-RANGE / CANONICALITY decomposition of the sponge INPUT lanes -----
//
// Mirrors `descender_account_stark::air::FullRelationAir::eval_sponge`'s per-lane byte-range gadget
// (air.rs:1278-1289) for EVERY byte-ranged input lane of the C1/C2/C6 sponges. For each byte-ranged
// lane the leaf AIR emits (in this EXACT order, riding inside `eval_sponge` before the perm loop):
//   * BYTE_BITS bit-boolean residuals `bit·(bit−1)` over the lane's 8 bit witnesses, then
//   * 1 recompose equality `lane − Σ_b bit_b·2^b == 0`.
// The decomposed bits constrain the ACTUAL absorbed lane cells the sponges already consume (NOT
// fresh free columns): C1 reads `c1_input_cell(i)` (all 48 lanes byte-ranged), C2 reads
// `c2_pad_cell(i)`/`c2_slot_cell(i−32)` (all 40 lanes), C6 reads `c6_byte_cell(b)` (the 80 byte-
// lanes; the 6 leading FIELD lanes are NOT byte-ranged, matching air.rs `c6_br`). So this class is
// the canonicality (each absorbed input lane is a valid byte `< 256`) of the genuine sponge preimage.
//
// air.rs byte-range counts (from `FullLayout::new`):
//   * C1: `vec![true; C1_INPUT_LANES]` ⇒ ALL 48 lanes (pad_seed[32] ‖ nonce_c1[16]).
//   * C2: `vec![true; C2_INPUT_LANES]` ⇒ ALL 40 lanes (pad_seed[32] ‖ slot_id[8]).
//   * C6: `false`×C6_FIELD_LANES(6) ‖ `true`×C6_BYTE_LANES(80) ⇒ the 80 byte-lanes only.
// The role-root sponge has `vec![false; 26]` (no byte-range — already noted CLASS A), so it
// contributes ZERO byte-range residuals.

/// Bits per byte-ranged input lane. Mirrors air.rs `BYTE_BITS`.
pub const BYTE_BITS: usize = 8;
/// C1 byte-ranged input lanes (ALL 48). Mirrors `vec![true; C1_INPUT_LANES]`.
pub const C1_BR_LANES: usize = C1_INPUT_LANES; // 48
/// C2 byte-ranged input lanes (ALL 40). Mirrors `vec![true; C2_INPUT_LANES]`.
pub const C2_BR_LANES: usize = C2_INPUT_LANES; // 40
/// C6 byte-ranged input lanes (the 80 byte-lanes; the 6 FIELD lanes are NOT byte-ranged).
pub const C6_BR_LANES: usize = C6_BYTE_LANES; // 80
/// Total byte-ranged lanes across C1 ‖ C2 ‖ C6 (the global byte-range index space).
pub const N_BR_LANES: usize = C1_BR_LANES + C2_BR_LANES + C6_BR_LANES; // 168
/// Byte-range residuals PER lane: BYTE_BITS bit-bools + 1 recompose eq. Mirrors air.rs:1278-1289.
pub const N_BR_PER_LANE: usize = BYTE_BITS + 1; // 9
/// Total byte-range residuals batched into `C_partial`.
pub const N_BYTERANGE: usize = N_BR_LANES * N_BR_PER_LANE; // 1512

// ----- SPONGE-OUTPUT → PUBLIC-VALUE binding (the per-instance commitment equalities) -----
//
// Mirrors `descender_account_stark::air::FullRelationAir::eval` (BATCH mode) where each mirrored
// sponge OUTPUT is constrained EQUAL to the leaf's PUBLIC statement value, which in batch mode is
// the per-row `pubio` block `local[lay.pubio_off + off]` (air.rs:712-756, 898-906, 941-953). The
// recursion already models that per-row pubio block as the `chain_pubio_cell(i)` cells — the SAME
// cells the batch digest chain ABSORBS (`chain_residual` reads `chain_pubio_cell(i)` for the running
// fold). So binding the mirrored outputs to `chain_pubio_cell(off)` ties them to the ONE source of
// truth for the public statement (a forger cannot expose a public commitment inconsistent with the
// in-circuit sponge, and a tampered public lane changes both the chain digest AND this equality).
//
// air.rs batch bindings (pubio offsets within one row's pubio block):
//   * pad_source_commitment[0..10]   == pubio[0..10]                       (= C1 squeeze output)
//   * opening_tuple_commitment[0..10]== pubio[N_COMMIT_LANES..2·N_COMMIT_LANES] (= C6 squeeze output)
//   * masked_projection[0..6]        == pubio[2·N_COMMIT_LANES..+N_LANES]  (= carrier masked output)
//   * role_root[0..8]                == pubio[PUBIO_ROLE_ROOT_OFF..+8]     (= role-root squeeze output)
//
// The mirrored outputs are the ALREADY-CONSTRAINED `c1_out_cell` / `c6_out_cell` /
// `carrier_masked_cell` / `role_out_cell` cells (bound to their recomputed sponges/gadget by the
// earlier classes), so this class CHAINS them to the public-IO model — it does NOT introduce fresh
// columns on either side.
//
// SCOPE NOTE (HONESTLY FLAGGED): the relation_id pubio binding (air.rs:962-968,
// `pubio[PUBIO_RELATION_ID_OFF + j] == RELATION_ID[j]`) is ALREADY covered by the existing
// `N_RELATION_ID` class (relation_id pinning over the SAME constant); the recursion pins relation_id
// directly. The context==pubio, syndrome, and packed_witness pubio bindings are NOT yet mirrored as
// in-circuit sponges in this harness (their producing gadgets are deferred), so they are not bound
// here; they ride with their gadgets' future mirror. The four bindings here are exactly the four
// per-instance COMMITMENT outputs whose sponges/gadgets ARE already mirrored.
pub const N_PUBIO_C1: usize = N_COMMIT_LANES; // pad_source_commitment[10]
pub const N_PUBIO_C6: usize = N_COMMIT_LANES; // opening_tuple_commitment[10]
pub const N_PUBIO_MASKED: usize = N_LANES; // masked_projection[6]
pub const N_PUBIO_ROLE: usize = ROLE_ROOT_ELEMS; // role_root[8]
pub const N_PUBIO_BIND: usize = N_PUBIO_C1 + N_PUBIO_C6 + N_PUBIO_MASKED + N_PUBIO_ROLE; // 34

/// Total residuals batched into `C_partial`.
pub const N_CONSTRAINTS: usize = C4_LANES * C4_N_PER_LANE
    + N_C4_ACC
    + N_SEAM_PAD
    + N_SEAM_ACC
    + N_RELATION_ID
    + N_C1
    + N_C6
    + N_ROLE
    + N_C2
    + N_REDUCE
    + N_CHAIN
    + N_BYTERANGE
    + N_PUBIO_BIND;

// ===========================================================================
// Column layout (one leaf-constraint-mirror check per row)
// ===========================================================================
//
// The opened OOD cells are extension elements (D=8 `Val` limbs each), laid out as contiguous
// blocks of `D` limbs in the following order:
//   * `C4_LANES` C4 lane blocks, each `C4_CELLS` cells           [C4_LANES*C4_CELLS cells]
//   * `carry_acc[N_LANES]`                                       [N_LANES cells]
//   * C1 `pad_seed[32]`                                          [PAD_SEED_BYTES cells]
//   * C2 `pad_seed[32]`                                          [PAD_SEED_BYTES cells]
//   * C6 `field_lanes[N_LANES]`                                  [N_LANES cells]
//   * `relation_id[RELATION_ID_ELEMS]`                           [RELATION_ID_ELEMS cells]
// then `alpha` (1 extension elem) and the committed `C_partial` (1 extension elem).

/// Number of opened OOD cells in the C1 sponge block: input lanes + perm blocks + output digest.
pub const C1_CELLS: usize = C1_INPUT_LANES + C1_N_PERMS * PERM_COLS + C1_N_OUT;

/// Number of opened OOD cells in the C6 sponge block: byte-lanes (the 6 FIELD lanes are REUSED from
/// `O_C6_FIELD`) + perm blocks + output digest.
pub const C6_CELLS: usize = C6_BYTE_LANES + C6_N_PERMS * PERM_COLS + C6_N_OUT;

/// Number of opened OOD cells in the role-root sponge block: ONLY the masked_projection[6] input
/// lanes (the other 20 input lanes are the CHAINED C1/C6 squeeze-output cells, not fresh) + perm
/// blocks + role_root[8] output digest.
pub const ROLE_CELLS: usize = ROLE_MASKED_LANES + ROLE_N_PERMS * PERM_COLS + ROLE_N_OUT;

/// Number of opened OOD cells fed to the mirror.
pub const N_OPENED_CELLS: usize = C4_LANES * C4_CELLS // C4 lanes
    + N_LANES // carry_acc
    + PAD_SEED_BYTES // C1 pad_seed
    + PAD_SEED_BYTES // C2 pad_seed
    + N_LANES // C6 field lanes (also the C6 sponge absorb positions 0..6)
    + RELATION_ID_ELEMS // relation_id
    + C1_CELLS // C1 sponge (input lanes ‖ perm blocks ‖ pad_source_commitment)
    + C6_CELLS // C6 sponge (byte-lanes ‖ perm blocks ‖ opening_tuple_commitment)
    + ROLE_CELLS // role-root sponge (masked[6] ‖ perm blocks ‖ role_root[8]); inputs 0..20 CHAINED
    + C2_CELLS // C2 mask-PRF sponge (slot_id[8] ‖ perm blocks); pad_seed[32] reused from O_C2_PAD
    + REDUCE_BLOCK_CELLS // wide-then-reduce gadget cells + carrier (masked[6] ‖ mod-q blocks)
    + CHAIN_CELLS // batch digest chain (prev_state ‖ chain_in ‖ perms ‖ chain_out ‖ acc ‖ pubio)
    + N_BYTERANGE_BITS; // per-lane byte-range bit witnesses (C1[48]‖C2[40]‖C6[80] × BYTE_BITS)

/// Byte-range bit witness cells: BYTE_BITS bits per byte-ranged lane (C1 ‖ C2 ‖ C6). These are the
/// ONLY NEW cells the byte-range class introduces; the decomposed LANE values are the EXISTING
/// already-absorbed sponge input cells (`c1_input_cell`/`c2_pad_cell`/`c2_slot_cell`/`c6_byte_cell`).
pub const N_BYTERANGE_BITS: usize = N_BR_LANES * BYTE_BITS;

// Cell-block offsets (in units of cells, each `D` limbs).
const O_C4: usize = 0;
const O_ACC: usize = O_C4 + C4_LANES * C4_CELLS;
const O_C1_PAD: usize = O_ACC + N_LANES;
const O_C2_PAD: usize = O_C1_PAD + PAD_SEED_BYTES;
const O_C6_FIELD: usize = O_C2_PAD + PAD_SEED_BYTES;
const O_RELATION_ID: usize = O_C6_FIELD + N_LANES;
// C1 sponge block: input lanes, then 8 perm blocks, then the 10-lane pad_source_commitment.
const O_C1_INPUT: usize = O_RELATION_ID + RELATION_ID_ELEMS;
const O_C1_PERMS: usize = O_C1_INPUT + C1_INPUT_LANES;
const O_C1_OUT: usize = O_C1_PERMS + C1_N_PERMS * PERM_COLS;
// C6 sponge block: byte-lanes, then 12 perm blocks, then the 10-lane opening_tuple_commitment.
// (The 6 FIELD absorb lanes live at `O_C6_FIELD`, reused.)
const O_C6_BYTE: usize = O_C1_OUT + C1_N_OUT;
const O_C6_PERMS: usize = O_C6_BYTE + C6_BYTE_LANES;
const O_C6_OUT: usize = O_C6_PERMS + C6_N_PERMS * PERM_COLS;
// Role-root sponge block: masked_projection[6] input lanes (the only NEW role inputs), then 4 perm
// blocks, then the 8-lane role_root. Input lanes 0..20 are the CHAINED `c1_out_cell`/`c6_out_cell`.
const O_ROLE_MASKED: usize = O_C6_OUT + C6_N_OUT;
const O_ROLE_PERMS: usize = O_ROLE_MASKED + ROLE_MASKED_LANES;
const O_ROLE_OUT: usize = O_ROLE_PERMS + ROLE_N_PERMS * PERM_COLS;
// C2 mask-PRF sponge block: slot_id[8] input lanes (pad_seed[32] reused from O_C2_PAD), then 9 perm
// blocks. NO output digest cells — the 30 squeezed lanes are read from the perm OUTPUT cells of the
// squeeze source perms (`c2_squeeze_output_cell`), and feed the reduce gadget.
const O_C2_SLOT: usize = O_ROLE_OUT + ROLE_N_OUT;
const O_C2_PERMS: usize = O_C2_SLOT + C2_SLOT_LANES;
// Reduce-gadget block: REDUCE_CELLS gadget witness cells, then the carrier (6 masked_projection
// values, then 6 mod-q witness blocks of MODQ_CELLS each). account_projection (carrier `acc`) is the
// EXISTING O_ACC carry_acc cells (reused).
const O_REDUCE: usize = O_C2_PERMS + C2_N_PERMS * PERM_COLS;
const O_CARRIER_MASKED: usize = O_REDUCE + REDUCE_CELLS;
const O_CARRIER_MODQ: usize = O_CARRIER_MASKED + N_LANES;
// Batch-digest-chain block: prev_chain_out[16], chain_in[16], CHAIN_NPERMS perm blocks,
// chain_out[16], enable[1], prev_count_acc[1], count_acc[1], pubio[PUBIO_ELEMS].
const O_CHAIN_PREV_OUT: usize = O_CARRIER_MODQ + N_LANES * MODQ_CELLS;
const O_CHAIN_IN: usize = O_CHAIN_PREV_OUT + CHAIN_W;
const O_CHAIN_PERMS: usize = O_CHAIN_IN + CHAIN_W;
const O_CHAIN_OUT: usize = O_CHAIN_PERMS + CHAIN_NPERMS * PERM_COLS;
const O_CHAIN_ENABLE: usize = O_CHAIN_OUT + CHAIN_W;
const O_CHAIN_PREV_COUNT: usize = O_CHAIN_ENABLE + 1;
const O_CHAIN_COUNT: usize = O_CHAIN_PREV_COUNT + 1;
const O_CHAIN_PUBIO: usize = O_CHAIN_COUNT + 1;
// Byte-range bit-witness block: BYTE_BITS bits per byte-ranged lane, global byte-range index order
// (C1 48 lanes ‖ C2 40 lanes ‖ C6 80 lanes). Placed LAST so it does not perturb prior cell offsets.
const O_BYTERANGE_BITS: usize = O_CHAIN_PUBIO + CHAIN_PUBIO;

/// First column: the opened OOD cells, each `D` limbs.
pub const L_CELLS: usize = 0;
/// The constraint-batching challenge `alpha` (`D` limbs).
pub const L_ALPHA: usize = L_CELLS + N_OPENED_CELLS * D;
/// The committed alpha-batched partial constraint eval `C_partial(zeta)` (`D` limbs).
pub const L_CPARTIAL: usize = L_ALPHA + D;

/// Total column count.
pub const fn n_cols() -> usize {
    L_CPARTIAL + D
}

/// Limb base of opened cell `c` (`c < N_OPENED_CELLS`).
const fn cell_base(c: usize) -> usize {
    L_CELLS + c * D
}

// Cell accessors (each returns a cell INDEX in `[0, N_OPENED_CELLS)`).
const fn c4_cell(lane: usize, within: usize) -> usize {
    O_C4 + lane * C4_CELLS + within
}
const fn acc_cell(j: usize) -> usize {
    O_ACC + j
}
const fn c1_pad_cell(b: usize) -> usize {
    O_C1_PAD + b
}
const fn c2_pad_cell(b: usize) -> usize {
    O_C2_PAD + b
}
const fn c6_field_cell(i: usize) -> usize {
    O_C6_FIELD + i
}
const fn relation_id_cell(j: usize) -> usize {
    O_RELATION_ID + j
}
/// C1 input lane `i` (`i < C1_INPUT_LANES`) — the absorbed pad_seed/nonce field element.
const fn c1_input_cell(i: usize) -> usize {
    O_C1_INPUT + i
}
/// First cell of C1 perm block `p` (`p < C1_N_PERMS`); the block spans `PERM_COLS` cells.
const fn c1_perm_base(p: usize) -> usize {
    O_C1_PERMS + p * PERM_COLS
}
/// C1 pad_source_commitment output lane `j` (`j < C1_N_OUT`) — the squeezed digest cell.
const fn c1_out_cell(j: usize) -> usize {
    O_C1_OUT + j
}
/// C6 byte-lane `b` (`b < C6_BYTE_LANES`) — the absorbed lifecycle/nonce/account_id field element.
/// (Absorb positions 6..86; positions 0..6 are the FIELD lanes at `O_C6_FIELD`.)
const fn c6_byte_cell(b: usize) -> usize {
    O_C6_BYTE + b
}
/// First cell of C6 perm block `p` (`p < C6_N_PERMS`); the block spans `PERM_COLS` cells.
const fn c6_perm_base(p: usize) -> usize {
    O_C6_PERMS + p * PERM_COLS
}
/// C6 opening_tuple_commitment output lane `j` (`j < C6_N_OUT`) — the squeezed digest cell.
const fn c6_out_cell(j: usize) -> usize {
    O_C6_OUT + j
}
/// Role masked_projection input lane `i` (`i < ROLE_MASKED_LANES`) — absorb positions 20..26.
const fn role_masked_cell(i: usize) -> usize {
    O_ROLE_MASKED + i
}
/// First cell of role perm block `p` (`p < ROLE_N_PERMS`); the block spans `PERM_COLS` cells.
const fn role_perm_base(p: usize) -> usize {
    O_ROLE_PERMS + p * PERM_COLS
}
/// Role_root output lane `j` (`j < ROLE_N_OUT`) — the squeezed digest cell.
const fn role_out_cell(j: usize) -> usize {
    O_ROLE_OUT + j
}
/// C2 slot_id input lane `i` (`i < C2_SLOT_LANES`) — absorb positions 32..40 (NEW cells).
const fn c2_slot_cell(i: usize) -> usize {
    O_C2_SLOT + i
}
/// First cell of C2 perm block `p` (`p < C2_N_PERMS`); the block spans `PERM_COLS` cells.
const fn c2_perm_base(p: usize) -> usize {
    O_C2_PERMS + p * PERM_COLS
}
/// The OOD cell holding squeezed C2 output lane `o` (`o < C2_N_OUT`): the OUTPUT (C_OUTPUT) cell of
/// the squeeze source perm. Mirrors `SpongePlan::output_source` (n_out=30): the source perm's
/// `pcols.post[lane]` cell. This is the value fed to the reduce gadget (`squeezed[o]`).
const fn c2_squeeze_output_cell(o: usize) -> usize {
    let (sp, lane) = c2_output_source(o);
    c2_perm_base(sp) + C_OUTPUT + lane
}
/// Reduce-gadget witness cell `i` (`i < REDUCE_CELLS`), relative to the gadget base `O_REDUCE`.
const fn reduce_cell(i: usize) -> usize {
    O_REDUCE + i
}
/// Carrier masked_projection value cell `j` (`j < N_LANES`).
const fn carrier_masked_cell(j: usize) -> usize {
    O_CARRIER_MASKED + j
}
/// First cell of carrier mod-q witness block for lane `j`; the block spans `MODQ_CELLS` cells.
const fn carrier_modq_base(j: usize) -> usize {
    O_CARRIER_MODQ + j * MODQ_CELLS
}

// ----- batch-digest-chain cell accessors -----
/// Previous-row chain_out lane `c` (`c < CHAIN_W`) — the cross-row carry source (a BOUND cell; in a
/// real opened multi-row trace this is the opened previous row's `chain_out`).
const fn chain_prev_out_cell(c: usize) -> usize {
    O_CHAIN_PREV_OUT + c
}
/// This row's incoming running state lane `c` (`c < CHAIN_W`).
const fn chain_in_cell(c: usize) -> usize {
    O_CHAIN_IN + c
}
/// First cell of chain absorb-perm block `p` (`p < CHAIN_NPERMS`); spans `PERM_COLS` cells.
const fn chain_perm_base(p: usize) -> usize {
    O_CHAIN_PERMS + p * PERM_COLS
}
/// This row's outgoing running state lane `c` (`c < CHAIN_W`).
const fn chain_out_cell(c: usize) -> usize {
    O_CHAIN_OUT + c
}
/// This row's enable selector (1 = real account, 0 = padding no-op).
const fn chain_enable_cell() -> usize {
    O_CHAIN_ENABLE
}
/// Previous-row running count accumulator (a BOUND cell; the prior row's `count_acc`).
const fn chain_prev_count_cell() -> usize {
    O_CHAIN_PREV_COUNT
}
/// This row's running count accumulator.
const fn chain_count_cell() -> usize {
    O_CHAIN_COUNT
}
/// This row's absorbed pubio field-element lane `i` (`i < CHAIN_PUBIO`).
const fn chain_pubio_cell(i: usize) -> usize {
    O_CHAIN_PUBIO + i
}

// ----- byte-range cell accessors -----
/// The ALREADY-ABSORBED sponge input lane cell that global byte-range index `g` (`g < N_BR_LANES`)
/// decomposes. The decomposition binds the REAL consumed lane (NOT a fresh column), so it is not
/// vacuous: C1 indices 0..48 → `c1_input_cell`; C2 indices 48..88 → `c2_pad_cell`(<32) /
/// `c2_slot_cell`(≥32); C6 indices 88..168 → `c6_byte_cell`. Mirrors air.rs `lane(i)` =
/// `local[off + slay.lanes + i]` for the byte-ranged positions of each sponge's `byte_ranged` mask.
const fn byterange_absorbed_cell(g: usize) -> usize {
    if g < C1_BR_LANES {
        c1_input_cell(g)
    } else if g < C1_BR_LANES + C2_BR_LANES {
        let i = g - C1_BR_LANES;
        if i < C2_PAD_LANES {
            c2_pad_cell(i)
        } else {
            c2_slot_cell(i - C2_PAD_LANES)
        }
    } else {
        // C6: only the 80 byte-lanes are byte-ranged (the 6 FIELD lanes are skipped, as in air.rs).
        let b = g - C1_BR_LANES - C2_BR_LANES;
        c6_byte_cell(b)
    }
}
/// Bit cell `b` (`b < BYTE_BITS`) of global byte-range index `g` (`g < N_BR_LANES`).
const fn byterange_bit_cell(g: usize, b: usize) -> usize {
    O_BYTERANGE_BITS + g * BYTE_BITS + b
}

/// The absorbed chain stream element at position `pos` (`pos < CHAIN_PADDED_LEN`). Mirrors the
/// per-row message `m = [enable, enable·pubio[0..PUBIO_ELEMS]]` zero-padded: `pos == 0` → enable;
/// `pos ∈ 1..=PUBIO_ELEMS` → enable·pubio[pos-1]; else → ZERO. The `enable·pubio` product makes
/// padding rows (enable=0) absorb the all-zero message, exactly as air.rs builds `msg`.
enum ChainAbsorb {
    Enable,
    EnablePubio(usize),
    Zero,
}
fn chain_absorb_source(pos: usize) -> ChainAbsorb {
    if pos == 0 {
        ChainAbsorb::Enable
    } else if pos <= CHAIN_PUBIO {
        ChainAbsorb::EnablePubio(pos - 1)
    } else {
        ChainAbsorb::Zero
    }
}

/// The absorbed C2 stream element at position `pos` (`pos < C2_PADDED_LEN`), as a cell index OR a
/// constant marker. Mirrors `absorb_elem_expr`: `pos < 32` → pad_seed cell (REUSED `O_C2_PAD`);
/// `pos ∈ 32..40` → slot_id cell; `pos == n_input(=40)` → the ONE padding marker; else → ZERO.
fn c2_absorb_source(pos: usize) -> C1Absorb {
    if pos < C2_PAD_LANES {
        C1Absorb::Cell(c2_pad_cell(pos))
    } else if pos < C2_INPUT_LANES {
        C1Absorb::Cell(c2_slot_cell(pos - C2_PAD_LANES))
    } else if pos == C2_INPUT_LANES {
        C1Absorb::One
    } else {
        C1Absorb::Zero
    }
}

/// `(perm_index, lane)` the C2 squeeze reads output `o` from (`o < C2_N_OUT`). Mirrors
/// `SpongePlan::output_source` for n_out=30: outputs 0..8 from the LAST absorb perm
/// (`C2_N_ABSORB-1`), 8..16 from squeeze perm `C2_N_ABSORB`, 16..24 from `C2_N_ABSORB+1`, 24..30
/// from `C2_N_ABSORB+2`.
const fn c2_output_source(o: usize) -> (usize, usize) {
    let blk = o / C2_RATE; // 0,1,2,3
    let lane = o % C2_RATE;
    if blk == 0 {
        (C2_N_ABSORB - 1, lane)
    } else {
        (C2_N_ABSORB - 1 + blk, lane)
    }
}

/// The absorbed role-root stream element at position `pos` (`pos < ROLE_PADDED_LEN`), as a cell
/// index OR a constant marker. Mirrors `absorb_elem_expr` (air.rs:1372) over the role input lanes —
/// the CHAINED layout (air.rs:919-931 binds role input lanes EQUAL to the commitment columns):
///   * `pos ∈ 0..10`  → C1 squeeze output cell `c1_out_cell(pos)`   (pad_source_commitment).
///   * `pos ∈ 10..20` → C6 squeeze output cell `c6_out_cell(pos-10)` (opening_tuple_commitment).
///   * `pos ∈ 20..26` → masked_projection cell `role_masked_cell(pos-20)`.
///   * `pos == 26`    → the ONE padding marker; else → ZERO.
fn role_absorb_source(pos: usize) -> C1Absorb {
    if pos < N_COMMIT_LANES {
        C1Absorb::Cell(c1_out_cell(pos))
    } else if pos < 2 * N_COMMIT_LANES {
        C1Absorb::Cell(c6_out_cell(pos - N_COMMIT_LANES))
    } else if pos < ROLE_INPUT_LANES {
        C1Absorb::Cell(role_masked_cell(pos - 2 * N_COMMIT_LANES))
    } else if pos == ROLE_INPUT_LANES {
        C1Absorb::One
    } else {
        C1Absorb::Zero
    }
}

/// `(perm_index, lane)` the role-root squeeze reads output `o` from. With `n_out = 8 ≤ RATE`, ALL 8
/// outputs come from the LAST absorb perm (`ROLE_N_ABSORB-1`), lanes `0..8`. Mirrors
/// `SpongePlan::output_source`.
const fn role_output_source(o: usize) -> (usize, usize) {
    (ROLE_N_ABSORB - 1, o)
}

/// The absorbed stream element at position `pos` (`pos < C1_PADDED_LEN`), as a cell index OR a
/// constant marker. Mirrors `absorb_elem_expr` (air.rs:1360): `pos < n_input` → input lane cell;
/// `pos == n_input` → the ONE padding marker; else → ZERO.
enum C1Absorb {
    Cell(usize),
    One,
    Zero,
}
fn c1_absorb_source(pos: usize) -> C1Absorb {
    if pos < C1_INPUT_LANES {
        C1Absorb::Cell(c1_input_cell(pos))
    } else if pos == C1_INPUT_LANES {
        C1Absorb::One
    } else {
        C1Absorb::Zero
    }
}

/// `(perm_index, lane)` the C1 squeeze reads output `o` from. Mirrors `SpongePlan::output_source`:
/// outputs `0..RATE` come from the LAST absorb perm (`C1_N_ABSORB-1`); the remainder from the
/// squeeze perm (`C1_N_ABSORB`).
const fn c1_output_source(o: usize) -> (usize, usize) {
    if o < C1_RATE {
        (C1_N_ABSORB - 1, o)
    } else {
        (C1_N_ABSORB, o - C1_RATE)
    }
}

/// The absorbed C6 stream element at position `pos` (`pos < C6_PADDED_LEN`), as a cell index OR a
/// constant marker. Mirrors `absorb_elem_expr` (air.rs:1360): `pos < n_input(=86)` → input lane cell
/// (the 6 FIELD lanes for pos<6, else a byte-lane); `pos == n_input` → the ONE padding marker; else
/// → ZERO. The FIELD lanes are the `O_C6_FIELD` cells, the byte-lanes the `O_C6_BYTE` cells.
fn c6_absorb_source(pos: usize) -> C1Absorb {
    if pos < C6_FIELD_LANES {
        C1Absorb::Cell(c6_field_cell(pos))
    } else if pos < C6_INPUT_LANES {
        C1Absorb::Cell(c6_byte_cell(pos - C6_FIELD_LANES))
    } else if pos == C6_INPUT_LANES {
        C1Absorb::One
    } else {
        C1Absorb::Zero
    }
}

/// `(perm_index, lane)` the C6 squeeze reads output `o` from. Same mapping as C1 (`n_out = 10`):
/// outputs `0..RATE` from the LAST absorb perm (`C6_N_ABSORB-1`), the remainder from the squeeze
/// perm (`C6_N_ABSORB`). Mirrors `SpongePlan::output_source`.
const fn c6_output_source(o: usize) -> (usize, usize) {
    if o < C6_RATE {
        (C6_N_ABSORB - 1, o)
    } else {
        (C6_N_ABSORB, o - C6_RATE)
    }
}

// ===========================================================================
// Octic (deg-8 extension) arithmetic over `X^D = W` — same as deep_air / fri_fold_air.
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

/// Limbwise add of two extension exprs.
fn octic_add<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() + b[i].clone())
}

/// Limbwise subtract.
fn octic_sub<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    core::array::from_fn(|i| a[i].clone() - b[i].clone())
}

/// Embed a base-field scalar `s` as an extension expr (limb 0 = s, rest 0).
fn octic_scalar<AB: AirBuilder>(s: u64) -> [AB::Expr; D] {
    core::array::from_fn(|i| {
        if i == 0 {
            AB::Expr::from(AB::F::from_u64(s))
        } else {
            AB::Expr::ZERO
        }
    })
}

/// Scale an extension expr by a base-field constant `s` (limbwise; no reduction since `s` is base).
fn scale<AB: AirBuilder>(a: &[AB::Expr; D], s: u64) -> [AB::Expr; D] {
    let sc = AB::Expr::from(AB::F::from_u64(s));
    core::array::from_fn(|i| a[i].clone() * sc.clone())
}

// ===========================================================================
// Native reference (the AUTHORITY the AIR is checked against)
// ===========================================================================

/// Native recompose of 7 bit cells starting at `start` into `Σ bit_b · 2^b`.
fn native_recompose(cells: &[Challenge; N_OPENED_CELLS], start_cell: usize) -> Challenge {
    let mut acc = Challenge::ZERO;
    for b in 0..C4_DIGIT_BITS {
        acc += cells[start_cell + b] * Challenge::from(Val::from_u64(1u64 << b));
    }
    acc
}

/// One leaf C4 constraint residual for `lane`, `i ∈ [0, C4_N_PER_LANE)`:
///   * `0..21`  : bit-boolean residuals `bit·(bit−1)` for lo[7] ‖ hi[7] ‖ slack[7].
///   * `21`     : `d_lo − recompose(lo_bits)`
///   * `22`     : `d_hi − recompose(hi_bits)`
///   * `23`     : `(96 − d_hi) − recompose(slack_bits)`
///   * `24`     : `is96·(is96−1)`
///   * `25`     : `is96·(d_hi − 96)`
///   * `26`     : `(d_hi − 96)·inv − (1 − is96)`
///   * `27`     : `is96·d_lo`
///   * `28`     : `v − (d_lo + 128·d_hi)`
fn c4_lane_residual(cells: &[Challenge; N_OPENED_CELLS], lane: usize, i: usize) -> Challenge {
    let v = cells[c4_cell(lane, I_V)];
    let d_lo = cells[c4_cell(lane, I_DLO)];
    let d_hi = cells[c4_cell(lane, I_DHI)];
    let is96 = cells[c4_cell(lane, I_IS96)];
    let inv = cells[c4_cell(lane, I_INV)];
    let max_hi = Challenge::from(Val::from_u32(C4_MAX_HI));
    let base = Challenge::from(Val::from_u32(C4_BASE));

    if i < 21 {
        let bit = cells[c4_cell(lane, I_LO_BITS + i)];
        return bit * (bit - Challenge::ONE);
    }
    match i - 21 {
        0 => d_lo - native_recompose(cells, c4_cell(lane, I_LO_BITS)),
        1 => d_hi - native_recompose(cells, c4_cell(lane, I_HI_BITS)),
        2 => (max_hi - d_hi) - native_recompose(cells, c4_cell(lane, I_SLACK_BITS)),
        3 => is96 * (is96 - Challenge::ONE),
        4 => is96 * (d_hi - max_hi),
        5 => (d_hi - max_hi) * inv - (Challenge::ONE - is96),
        6 => is96 * d_lo,
        7 => v - (d_lo + base * d_hi),
        _ => unreachable!("c4 constraint index out of range"),
    }
}

/// The `k`-th NATIVE C1-sponge residual over the opened cells, `k ∈ [0, N_C1)`. Mirrors the leaf
/// `eval_sponge` order EXACTLY (over `Challenge` arithmetic):
///   * `0 .. N_C1_INPUT_BIND`           : per-perm input/capacity bindings (WIDTH per perm).
///         perm 0 absorb:  inputs[r] − absorb[r]  (r<RATE) ; inputs[c] − 0 (c≥RATE).
///         perm p>0 absorb: inputs[r] − (prev_out[r] + absorb[p·RATE+r]) ; inputs[c] − prev_out[c].
///         squeeze perm:    inputs[r] − prev_out[r] ; inputs[c] − prev_out[c].
///   * next `N_C1_PERM`                 : per-perm Poseidon2 round residuals (inlined_perm_octic).
///   * next `N_C1_OUT`                  : pad_source_commitment[j] − squeezed_output[j].
fn c1_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    // perm inputs / committed posts / outputs from the opened cells.
    let perm_input = |p: usize, i: usize| cells[c1_perm_base(p) + C_INPUTS + i];
    let perm_output = |p: usize, i: usize| cells[c1_perm_base(p) + C_OUTPUT + i];
    let absorb = |pos: usize| -> Challenge {
        match c1_absorb_source(pos) {
            C1Absorb::Cell(c) => cells[c],
            C1Absorb::One => Challenge::ONE,
            C1Absorb::Zero => Challenge::ZERO,
        }
    };

    // (a) input/capacity bindings (WIDTH per perm).
    if k < N_C1_INPUT_BIND {
        let p = k / WIDTH;
        let lane = k % WIDTH;
        let is_absorb = p < C1_N_ABSORB;
        if p == 0 {
            // first perm: inputs[r] == absorb[r] (r<RATE), inputs[c] == 0 (capacity).
            if lane < C1_RATE {
                return perm_input(0, lane) - absorb(lane);
            }
            return perm_input(0, lane);
        }
        let prev = perm_output(p - 1, lane);
        if lane < C1_RATE {
            if is_absorb {
                let pos = p * C1_RATE + lane;
                let elem = if pos < C1_PADDED_LEN { absorb(pos) } else { Challenge::ZERO };
                return perm_input(p, lane) - (prev + elem);
            }
            // squeeze perm: rate lanes carried unchanged.
            return perm_input(p, lane) - prev;
        }
        // capacity carried unchanged from the previous output.
        return perm_input(p, lane) - prev;
    }
    let mut k = k - N_C1_INPUT_BIND;

    // (b) per-perm Poseidon2 round residuals (post − recomputed). Computed by the native octic perm
    // reference over THIS perm block's opened input cells, in inlined_perm_octic post order.
    if k < N_C1_PERM {
        let per = inlined_perm_octic::PERM_RESIDUALS;
        let p = k / per;
        let ri = k % per;
        let inputs: [Challenge; WIDTH] = core::array::from_fn(|i| perm_input(p, i));
        // committed posts in perm_residuals order (begin-full, partial, end-full), read as OPENED
        // cells so the reset semantics of `native_perm_residuals` matches the in-AIR
        // `perm_residuals` limb-for-limb (on arbitrary off-domain cells, not just honest ones).
        let committed: Vec<Challenge> = (0..per)
            .map(|i| cells[c1_perm_base(p) + c1_post_cell_offset(i)])
            .collect();
        let res = inlined_perm_octic::native_perm_residuals(inputs, &committed);
        return res[ri];
    }
    k -= N_C1_PERM;

    // (c) squeeze-output equality: pad_source_commitment[j] − opened_output_cell[j].
    // The opened output cell is the source perm's committed C_OUTPUT post; the per-perm round
    // residuals (b) bind that cell to the recomputed permutation, so binding the digest to the
    // opened cell here suffices (and matches the leaf, which equates the public commitment to the
    // perm's committed `post` cell, air.rs:1355 + 720-726).
    debug_assert!(k < N_C1_OUT);
    let j = k;
    let (sp, lane) = c1_output_source(j);
    cells[c1_out_cell(j)] - perm_output(sp, lane)
}

/// The `k`-th NATIVE C6-sponge residual over the opened cells, `k ∈ [0, N_C6)`. STRUCTURALLY
/// identical to [`c1_residual`] (the absorb/capacity/squeeze pattern is shared by `eval_sponge`),
/// differing only in the C6 constants (`C6_*`), the absorb source (6 FIELD lanes ‖ 80 byte-lanes),
/// the perm/output cells, and the squeezed digest target (`opening_tuple_commitment`).
fn c6_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let perm_input = |p: usize, i: usize| cells[c6_perm_base(p) + C_INPUTS + i];
    let perm_output = |p: usize, i: usize| cells[c6_perm_base(p) + C_OUTPUT + i];
    let absorb = |pos: usize| -> Challenge {
        match c6_absorb_source(pos) {
            C1Absorb::Cell(c) => cells[c],
            C1Absorb::One => Challenge::ONE,
            C1Absorb::Zero => Challenge::ZERO,
        }
    };

    // (a) input/capacity bindings (WIDTH per perm).
    if k < N_C6_INPUT_BIND {
        let p = k / WIDTH;
        let lane = k % WIDTH;
        let is_absorb = p < C6_N_ABSORB;
        if p == 0 {
            if lane < C6_RATE {
                return perm_input(0, lane) - absorb(lane);
            }
            return perm_input(0, lane);
        }
        let prev = perm_output(p - 1, lane);
        if lane < C6_RATE {
            if is_absorb {
                let pos = p * C6_RATE + lane;
                let elem = if pos < C6_PADDED_LEN { absorb(pos) } else { Challenge::ZERO };
                return perm_input(p, lane) - (prev + elem);
            }
            return perm_input(p, lane) - prev;
        }
        return perm_input(p, lane) - prev;
    }
    let mut k = k - N_C6_INPUT_BIND;

    // (b) per-perm Poseidon2 round residuals.
    if k < N_C6_PERM {
        let per = inlined_perm_octic::PERM_RESIDUALS;
        let p = k / per;
        let ri = k % per;
        let inputs: [Challenge; WIDTH] = core::array::from_fn(|i| perm_input(p, i));
        let committed: Vec<Challenge> = (0..per)
            .map(|i| cells[c6_perm_base(p) + c1_post_cell_offset(i)])
            .collect();
        let res = inlined_perm_octic::native_perm_residuals(inputs, &committed);
        return res[ri];
    }
    k -= N_C6_PERM;

    // (c) squeeze-output equality: opening_tuple_commitment[j] − opened_output_cell[j].
    debug_assert!(k < N_C6_OUT);
    let j = k;
    let (sp, lane) = c6_output_source(j);
    cells[c6_out_cell(j)] - perm_output(sp, lane)
}

/// The `k`-th NATIVE role-root-sponge residual over the opened cells, `k ∈ [0, N_ROLE)`. The
/// role-root sponge (CLASS A, air.rs:908-954) is STRUCTURALLY identical to [`c1_residual`] /
/// [`c6_residual`] (the absorb/capacity/squeeze pattern is shared by `eval_sponge`), differing only
/// in the role constants (`ROLE_*`), the absorb source — which CHAINS the C1/C6 squeeze outputs
/// (`role_absorb_source` reads `c1_out_cell`/`c6_out_cell` for input lanes 0..20, the masked lanes
/// for 20..26) — the perm/output cells, and the squeezed digest target (`role_root`).
fn role_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let perm_input = |p: usize, i: usize| cells[role_perm_base(p) + C_INPUTS + i];
    let perm_output = |p: usize, i: usize| cells[role_perm_base(p) + C_OUTPUT + i];
    let absorb = |pos: usize| -> Challenge {
        match role_absorb_source(pos) {
            C1Absorb::Cell(c) => cells[c],
            C1Absorb::One => Challenge::ONE,
            C1Absorb::Zero => Challenge::ZERO,
        }
    };

    // (a) input/capacity bindings (WIDTH per perm).
    if k < N_ROLE_INPUT_BIND {
        let p = k / WIDTH;
        let lane = k % WIDTH;
        let is_absorb = p < ROLE_N_ABSORB;
        if p == 0 {
            if lane < ROLE_RATE {
                return perm_input(0, lane) - absorb(lane);
            }
            return perm_input(0, lane);
        }
        let prev = perm_output(p - 1, lane);
        if lane < ROLE_RATE {
            if is_absorb {
                let pos = p * ROLE_RATE + lane;
                let elem = if pos < ROLE_PADDED_LEN { absorb(pos) } else { Challenge::ZERO };
                return perm_input(p, lane) - (prev + elem);
            }
            return perm_input(p, lane) - prev;
        }
        return perm_input(p, lane) - prev;
    }
    let mut k = k - N_ROLE_INPUT_BIND;

    // (b) per-perm Poseidon2 round residuals.
    if k < N_ROLE_PERM {
        let per = inlined_perm_octic::PERM_RESIDUALS;
        let p = k / per;
        let ri = k % per;
        let inputs: [Challenge; WIDTH] = core::array::from_fn(|i| perm_input(p, i));
        let committed: Vec<Challenge> = (0..per)
            .map(|i| cells[role_perm_base(p) + c1_post_cell_offset(i)])
            .collect();
        let res = inlined_perm_octic::native_perm_residuals(inputs, &committed);
        return res[ri];
    }
    k -= N_ROLE_PERM;

    // (c) squeeze-output equality: role_root[j] − opened_output_cell[j].
    debug_assert!(k < N_ROLE_OUT);
    let j = k;
    let (sp, lane) = role_output_source(j);
    cells[role_out_cell(j)] - perm_output(sp, lane)
}

/// The `k`-th NATIVE C2-sponge residual over the opened cells, `k ∈ [0, N_C2)`. STRUCTURALLY
/// identical to [`c1_residual`] / [`c6_residual`] (the absorb/capacity/squeeze pattern is shared by
/// `eval_sponge`), differing only in the C2 constants (`C2_*`), the absorb source (32 pad_seed lanes
/// REUSED from `O_C2_PAD` ‖ 8 slot_id lanes), and the perm cells. The C2 sponge has NO squeeze-output
/// equality sub-class: the 30 squeezed lanes are NOT a public digest; they feed the reduce gadget,
/// which reads the perm OUTPUT cells directly, and their soundness is the per-perm round residuals.
fn c2_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let perm_input = |p: usize, i: usize| cells[c2_perm_base(p) + C_INPUTS + i];
    let perm_output = |p: usize, i: usize| cells[c2_perm_base(p) + C_OUTPUT + i];
    let absorb = |pos: usize| -> Challenge {
        match c2_absorb_source(pos) {
            C1Absorb::Cell(c) => cells[c],
            C1Absorb::One => Challenge::ONE,
            C1Absorb::Zero => Challenge::ZERO,
        }
    };

    // (a) input/capacity bindings (WIDTH per perm).
    if k < N_C2_INPUT_BIND {
        let p = k / WIDTH;
        let lane = k % WIDTH;
        let is_absorb = p < C2_N_ABSORB;
        if p == 0 {
            if lane < C2_RATE {
                return perm_input(0, lane) - absorb(lane);
            }
            return perm_input(0, lane);
        }
        let prev = perm_output(p - 1, lane);
        if lane < C2_RATE {
            if is_absorb {
                let pos = p * C2_RATE + lane;
                let elem = if pos < C2_PADDED_LEN { absorb(pos) } else { Challenge::ZERO };
                return perm_input(p, lane) - (prev + elem);
            }
            return perm_input(p, lane) - prev;
        }
        return perm_input(p, lane) - prev;
    }
    let k = k - N_C2_INPUT_BIND;

    // (b) per-perm Poseidon2 round residuals.
    debug_assert!(k < N_C2_PERM);
    let per = inlined_perm_octic::PERM_RESIDUALS;
    let p = k / per;
    let ri = k % per;
    let inputs: [Challenge; WIDTH] = core::array::from_fn(|i| perm_input(p, i));
    let committed: Vec<Challenge> = (0..per)
        .map(|i| cells[c2_perm_base(p) + c1_post_cell_offset(i)])
        .collect();
    let res = inlined_perm_octic::native_perm_residuals(inputs, &committed);
    res[ri]
}

/// Native recompose of `nbits` bit cells starting at cell `start` into `Σ bit_b · 2^b` (over
/// `Challenge`). Mirrors `reduce_gadget::eval_reduce_gadget`'s `recompose` closure (the ACCUMULATION;
/// the bit-boolean residuals are emitted separately by [`reduce_residual`]).
fn native_recompose_n(cells: &[Challenge; N_OPENED_CELLS], start: usize, nbits: usize) -> Challenge {
    let mut acc = Challenge::ZERO;
    for b in 0..nbits {
        acc += cells[start + b] * Challenge::from(Val::from_u64(1u64 << b));
    }
    acc
}

/// q12289 split base-2^16 constants over `Val` (KoalaBear), MIRRORING `reduce_gadget::p_lo_hi` +
/// the carrier constants. `(p_lo, p_hi)` are the field modulus' base-2^16 limbs; `q` is q12289;
/// `two16 = 2^16`.
fn reduce_field_consts() -> (u64, u64, u64, u64) {
    let p = <Val as p3_field::PrimeField64>::ORDER_U64;
    let p_lo = p & ((1u64 << LIMB_BASE_BITS) - 1);
    let p_hi = p >> LIMB_BASE_BITS;
    (p_lo, p_hi, Q12289 as u64, 1u64 << LIMB_BASE_BITS)
}

/// The 6 reduced masks over `Challenge` from the opened reduce-gadget LANE cells, computed by the
/// in-gadget Horner recurrence over the OPENED `r_t` cells (so it matches what the in-AIR mirror
/// returns: `masks[j] = r_t` of the LAST Horner step, read from the opened cell). Reads the opened
/// cells, NOT a recomputation — the per-step residuals bind those cells to the honest reduction.
fn reduce_masks_from_cells(cells: &[Challenge; N_OPENED_CELLS]) -> [Challenge; NUM_OUTPUTS] {
    core::array::from_fn(|j| {
        let mask_base = j * PER_MASK_CELLS;
        let step0 = mask_base + LANES_PER_MASK;
        let last_sb = step0 + (HORNER_STEPS - 1) * STEP_CELLS;
        cells[reduce_cell(last_sb + StepOff::R)]
    })
}

/// The `k`-th NATIVE wide-then-reduce + carrier residual over the opened cells, `k ∈ [0, N_REDUCE)`.
/// Mirrors `descender_account_stark::reduce_gadget::eval_reduce_gadget` (per-mask Horner steps) +
/// the C4 carrier mod-q reduction (air.rs:856-906) EXACTLY, in the SAME assertion ORDER. Each leaf
/// `assert_bool(b)` becomes `b·(b-1)`; each `assert_eq(a, c)` becomes `a - c`.
///
/// Per-mask order (`N_REDUCE_PER_MASK` each): 5 lane-bind eqs, then per Horner step
/// (`N_STEP_RESIDUALS`): the 11 value-recompose blocks (each = its bit-bools then the value eq, in
/// the StepOff order qt_lo, qt_hi, lane_lo, lane_hi, s0, s1, s2, c0, c1, d0, d1), then r_t (bits then
/// eq), slack (bits then eq), (r_t+slack==q-1), (digit==lane_lo+lane_hi·2^16), and the 6 limb/carry
/// eqs. Then the carrier (`N_CARRIER_PER_LANE` each): masked-recompose (bits then eq), slack-recompose
/// (bits then eq), (masked+slack==q-1), wrap-bool, (acc+mask==masked+wrap·q).
fn reduce_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let (p_lo, p_hi, q, two16) = reduce_field_consts();
    let one = Challenge::ONE;
    let cv = |s: u64| Challenge::from(Val::from_u64(s));
    // squeezed lane `s` (s < NUM_SQUEEZE) is the C2 squeeze OUTPUT cell.
    let squeezed = |s: usize| cells[c2_squeeze_output_cell(s)];

    let reduce_total = NUM_OUTPUTS * N_REDUCE_PER_MASK;
    if k < reduce_total {
        let j = k / N_REDUCE_PER_MASK;
        let mut kk = k % N_REDUCE_PER_MASK;
        let mask_base = j * PER_MASK_CELLS;
        // (i) 5 lane-bind eqs: local[lane_i] - squeezed[j*5 + i].
        if kk < LANES_PER_MASK {
            let i = kk;
            return cells[reduce_cell(mask_base + i)] - squeezed(j * LANES_PER_MASK + i);
        }
        kk -= LANES_PER_MASK;
        // (ii) Horner steps.
        let t = kk / N_STEP_RESIDUALS;
        let mut si = kk % N_STEP_RESIDUALS;
        let step0 = mask_base + LANES_PER_MASK;
        let sb = step0 + t * STEP_CELLS; // reduce-relative step base (cell offset within gadget)
        let lane_idx = LANES_PER_MASK - 1 - t;
        let digit = squeezed(j * LANES_PER_MASK + lane_idx);

        // Each value block: `nbits` bit-bools, then 1 value-recompose eq. Order matches StepOff.
        // (val_cell_off, bits_off, nbits)
        let blocks: [(usize, usize, usize); 11] = [
            (StepOff::QT_LO, StepOff::QT_LO_BITS, LO_BITS),
            (StepOff::QT_HI, StepOff::QT_HI_BITS, HI_BITS),
            (StepOff::LANE_LO, StepOff::LANE_LO_BITS, LO_BITS),
            (StepOff::LANE_HI, StepOff::LANE_HI_BITS, HI_BITS),
            (StepOff::S0, StepOff::S0_BITS, S_LO_BITS),
            (StepOff::S1, StepOff::S1_BITS, S_LO_BITS),
            (StepOff::S2, StepOff::S2_BITS, S_TOP_BITS),
            (StepOff::C0, StepOff::C0_BITS, S_TOP_BITS),
            (StepOff::C1, StepOff::C1_BITS, S_TOP_BITS),
            (StepOff::D0, StepOff::D0_BITS, S_TOP_BITS),
            (StepOff::D1, StepOff::D1_BITS, S_TOP_BITS),
        ];
        for (val_off, bits_off, nbits) in blocks.iter().copied() {
            // bit-bools for this block.
            if si < nbits {
                let bit = cells[reduce_cell(sb + bits_off + si)];
                return bit * (bit - one);
            }
            si -= nbits;
            // value-recompose eq.
            if si == 0 {
                let rc = native_recompose_n(cells, reduce_cell(sb + bits_off), nbits);
                return cells[reduce_cell(sb + val_off)] - rc;
            }
            si -= 1;
        }
        // r_t block: bits then eq.
        if si < MASK_BITS {
            let bit = cells[reduce_cell(sb + StepOff::R_BITS + si)];
            return bit * (bit - one);
        }
        si -= MASK_BITS;
        if si == 0 {
            let rc = native_recompose_n(cells, reduce_cell(sb + StepOff::R_BITS), MASK_BITS);
            return cells[reduce_cell(sb + StepOff::R)] - rc;
        }
        si -= 1;
        // slack block: bits then eq.
        if si < MASK_BITS {
            let bit = cells[reduce_cell(sb + StepOff::SLACK_BITS + si)];
            return bit * (bit - one);
        }
        si -= MASK_BITS;
        if si == 0 {
            let rc = native_recompose_n(cells, reduce_cell(sb + StepOff::SLACK_BITS), MASK_BITS);
            return cells[reduce_cell(sb + StepOff::SLACK)] - rc;
        }
        si -= 1;

        // value handles (read the opened value cells, bound to their recompose by the eqs above).
        let qt_lo = cells[reduce_cell(sb + StepOff::QT_LO)];
        let qt_hi = cells[reduce_cell(sb + StepOff::QT_HI)];
        let lane_lo = cells[reduce_cell(sb + StepOff::LANE_LO)];
        let lane_hi = cells[reduce_cell(sb + StepOff::LANE_HI)];
        let s0 = cells[reduce_cell(sb + StepOff::S0)];
        let s1 = cells[reduce_cell(sb + StepOff::S1)];
        let s2 = cells[reduce_cell(sb + StepOff::S2)];
        let c0 = cells[reduce_cell(sb + StepOff::C0)];
        let c1 = cells[reduce_cell(sb + StepOff::C1)];
        let d0 = cells[reduce_cell(sb + StepOff::D0)];
        let d1 = cells[reduce_cell(sb + StepOff::D1)];
        let r_t = cells[reduce_cell(sb + StepOff::R)];
        let slack = cells[reduce_cell(sb + StepOff::SLACK)];
        // r_prev = previous step's r_t (step 0 ⇒ 0); the in-gadget recurrence.
        let r_prev = if t == 0 {
            Challenge::ZERO
        } else {
            let psb = step0 + (t - 1) * STEP_CELLS;
            cells[reduce_cell(psb + StepOff::R)]
        };

        // 0 : r_t + slack == q-1.
        if si == 0 {
            return (r_t + slack) - cv(q - 1);
        }
        si -= 1;
        // 1 : digit == lane_lo + lane_hi·2^16.
        if si == 0 {
            return digit - (lane_lo + lane_hi * cv(two16));
        }
        si -= 1;
        // 2..8 : 6 limb/carry eqs.
        let l0 = r_prev * cv(p_lo) + lane_lo;
        let l1 = r_prev * cv(p_hi) + lane_hi;
        let r0 = qt_lo * cv(q) + r_t;
        let r1 = qt_hi * cv(q);
        return match si {
            0 => l0 - (s0 + c0 * cv(two16)),
            1 => (l1 + c0) - (s1 + c1 * cv(two16)),
            2 => c1 - s2,
            3 => r0 - (s0 + d0 * cv(two16)),
            4 => (r1 + d0) - (s1 + d1 * cv(two16)),
            5 => d1 - s2,
            _ => unreachable!("reduce step alg residual index out of range"),
        };
    }
    // ---- carrier: masked_projection[j] == account_projection[j] + mask[j] mod q12289 ----
    let mut kk = k - reduce_total;
    let j = kk / N_CARRIER_PER_LANE;
    kk %= N_CARRIER_PER_LANE;
    let mb = carrier_modq_base(j);
    let masked_bits_off = mb + 1;
    let slack_off = mb + 1 + MODQ_BITS;
    let slack_bits_off = slack_off + 1;
    let wrap_off = slack_bits_off + MODQ_BITS;
    let masked = cells[carrier_masked_cell(j)];
    let acc = cells[acc_cell(j)]; // account_projection lane (carry_acc, reused)
    let mask = reduce_masks_from_cells(cells)[j];

    // masked-recompose: MASK_BITS bools then eq.
    if kk < MODQ_BITS {
        let bit = cells[masked_bits_off + kk];
        return bit * (bit - one);
    }
    kk -= MODQ_BITS;
    if kk == 0 {
        let rc = native_recompose_n(cells, masked_bits_off, MODQ_BITS);
        return masked - rc;
    }
    kk -= 1;
    // slack-recompose: MASK_BITS bools then eq.
    if kk < MODQ_BITS {
        let bit = cells[slack_bits_off + kk];
        return bit * (bit - one);
    }
    kk -= MODQ_BITS;
    if kk == 0 {
        let rc = native_recompose_n(cells, slack_bits_off, MODQ_BITS);
        return cells[slack_off] - rc;
    }
    kk -= 1;
    // masked + slack == q-1.
    if kk == 0 {
        return (masked + cells[slack_off]) - cv(q - 1);
    }
    kk -= 1;
    // wrap-bool.
    if kk == 0 {
        let wrap = cells[wrap_off];
        return wrap * (wrap - one);
    }
    kk -= 1;
    // carrier: acc + mask == masked + wrap·q.
    debug_assert_eq!(kk, 0);
    let wrap = cells[wrap_off];
    (acc + mask) - (masked + wrap * cv(q))
}

/// The `k`-th NATIVE batch-digest-chain residual over the opened cells, `k ∈ [0, N_CHAIN)`. Mirrors
/// `descender_account_stark::air::FullRelationAir::eval_digest_chain` (air.rs:1109-1260) + the
/// off-circuit reference (`chain_digest` / the `generate_batch_trace` fill) EXACTLY, over `Challenge`
/// arithmetic, in the SAME assertion order as the in-AIR mirror in `recompute_c_partial`:
///   * `0`                                : enable boolean `enable·(enable−1)`.
///   * next `N_CHAIN_INPUT_BIND`          : per-perm input/capacity bindings (CHAIN_W per perm).
///         perm 0:  inputs[r] − (chain_in[r] + msg[r])  (r<RATE) ; inputs[c] − chain_in[c] (c≥RATE).
///         perm p>0: inputs[r] − (prev_out[r] + msg[p·RATE+r]) ; inputs[c] − prev_out[c].
///       where `msg[0]=enable`, `msg[1+i]=enable·pubio[i]`, else 0.
///   * next `N_CHAIN_PERM`                : per-perm Poseidon2 round residuals (inlined_perm_octic).
///   * next `N_CHAIN_OUT_BIND`            : chain_out[c] − (enable·absorbed[c] + (1−enable)·chain_in[c]).
///   * next `N_CHAIN_CARRY`               : chain_in[c] − prev_chain_out[c] (the CROSS-ROW carry).
///   * next `N_CHAIN_COUNT`               : count_acc − (prev_count_acc + enable).
fn chain_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let perm_input = |p: usize, i: usize| cells[chain_perm_base(p) + C_INPUTS + i];
    let perm_output = |p: usize, i: usize| cells[chain_perm_base(p) + C_OUTPUT + i];
    let enable = cells[chain_enable_cell()];
    // msg element at stream position `pos` (with the enable·pubio products folded in).
    let msg = |pos: usize| -> Challenge {
        match chain_absorb_source(pos) {
            ChainAbsorb::Enable => enable,
            ChainAbsorb::EnablePubio(i) => enable * cells[chain_pubio_cell(i)],
            ChainAbsorb::Zero => Challenge::ZERO,
        }
    };

    // (a) enable boolean.
    if k < N_CHAIN_ENABLE {
        return enable * (enable - Challenge::ONE);
    }
    let mut k = k - N_CHAIN_ENABLE;

    // (b) per-perm input/capacity bindings.
    if k < N_CHAIN_INPUT_BIND {
        let p = k / CHAIN_W;
        let lane = k % CHAIN_W;
        if p == 0 {
            // first absorb: state = chain_in, add message to rate lanes.
            if lane < CHAIN_RT {
                return perm_input(0, lane) - (cells[chain_in_cell(lane)] + msg(lane));
            }
            return perm_input(0, lane) - cells[chain_in_cell(lane)];
        }
        let prev = perm_output(p - 1, lane);
        if lane < CHAIN_RT {
            return perm_input(p, lane) - (prev + msg(p * CHAIN_RT + lane));
        }
        return perm_input(p, lane) - prev;
    }
    k -= N_CHAIN_INPUT_BIND;

    // (c) per-perm Poseidon2 round residuals.
    if k < N_CHAIN_PERM {
        let per = inlined_perm_octic::PERM_RESIDUALS;
        let p = k / per;
        let ri = k % per;
        let inputs: [Challenge; WIDTH] = core::array::from_fn(|i| perm_input(p, i));
        let committed: Vec<Challenge> = (0..per)
            .map(|i| cells[chain_perm_base(p) + c1_post_cell_offset(i)])
            .collect();
        let res = inlined_perm_octic::native_perm_residuals(inputs, &committed);
        return res[ri];
    }
    k -= N_CHAIN_PERM;

    // (d) chain_out binding: chain_out[c] == enable·absorbed[c] + (1−enable)·chain_in[c].
    // absorbed[c] = the LAST absorb perm's output cell.
    if k < N_CHAIN_OUT_BIND {
        let c = k;
        let absorbed = perm_output(CHAIN_NPERMS - 1, c);
        let chain_in = cells[chain_in_cell(c)];
        let expected = enable * absorbed + (Challenge::ONE - enable) * chain_in;
        return cells[chain_out_cell(c)] - expected;
    }
    k -= N_CHAIN_OUT_BIND;

    // (e) cross-row carry: chain_in[c] == prev_chain_out[c]. `prev_chain_out` is a BOUND opened cell
    // (the previous row's chain_out); a swapped/altered prior digest breaks this residual.
    if k < N_CHAIN_CARRY {
        let c = k;
        return cells[chain_in_cell(c)] - cells[chain_prev_out_cell(c)];
    }
    k -= N_CHAIN_CARRY;

    // (f) count accumulator: count_acc == prev_count_acc + enable.
    debug_assert!(k < N_CHAIN_COUNT);
    cells[chain_count_cell()] - (cells[chain_prev_count_cell()] + enable)
}

/// The `k`-th NATIVE byte-range residual over the opened cells, `k ∈ [0, N_BYTERANGE)`. Mirrors
/// `eval_sponge`'s per-lane byte-range gadget (air.rs:1278-1289) EXACTLY, lane-major over the global
/// byte-range index `g = k / N_BR_PER_LANE` (C1 48 ‖ C2 40 ‖ C6 80):
///   * within-lane index `0..BYTE_BITS` : bit-boolean residual `bit·(bit−1)`.
///   * within-lane index `BYTE_BITS`    : recompose eq `lane − Σ_b bit_b·2^b`, where `lane` is the
///     ACTUAL already-absorbed sponge input cell (`byterange_absorbed_cell(g)`) — NOT a free column.
fn byterange_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let g = k / N_BR_PER_LANE;
    let within = k % N_BR_PER_LANE;
    debug_assert!(g < N_BR_LANES);
    if within < BYTE_BITS {
        // assert_bool: bit·(bit−1).
        let bit = cells[byterange_bit_cell(g, within)];
        return bit * (bit - Challenge::ONE);
    }
    // recompose eq: lane − Σ_b bit_b·2^b (the lane is the genuine absorbed input cell).
    let lane = cells[byterange_absorbed_cell(g)];
    let mut acc = Challenge::ZERO;
    for b in 0..BYTE_BITS {
        acc += cells[byterange_bit_cell(g, b)] * Challenge::from(Val::from_u64(1u64 << b));
    }
    lane - acc
}

/// Map the `k`-th sponge-output→public-value binding (`k ∈ [0, N_PUBIO_BIND)`) to the
/// `(output_cell, pubio_lane)` pair it equates. The `output_cell` is the ALREADY-MIRRORED sponge /
/// gadget output cell; the `pubio_lane` is the offset within the per-row pubio block (= the
/// `chain_pubio_cell` index — the SAME cell the digest chain absorbs). Mirrors the air.rs BATCH
/// bindings EXACTLY:
///   * `0..10`  : `c1_out_cell(j)`        == `chain_pubio_cell(j)`                       (C1).
///   * `10..20` : `c6_out_cell(j)`        == `chain_pubio_cell(N_COMMIT_LANES + j)`      (C6).
///   * `20..26` : `carrier_masked_cell(j)`== `chain_pubio_cell(2·N_COMMIT_LANES + j)`    (masked).
///   * `26..34` : `role_out_cell(j)`      == `chain_pubio_cell(PUBIO_ROLE_ROOT_OFF + j)` (role).
const fn pubio_bind_pair(k: usize) -> (usize, usize) {
    if k < N_PUBIO_C1 {
        let j = k;
        (c1_out_cell(j), chain_pubio_cell(j))
    } else if k < N_PUBIO_C1 + N_PUBIO_C6 {
        let j = k - N_PUBIO_C1;
        (c6_out_cell(j), chain_pubio_cell(N_COMMIT_LANES + j))
    } else if k < N_PUBIO_C1 + N_PUBIO_C6 + N_PUBIO_MASKED {
        let j = k - N_PUBIO_C1 - N_PUBIO_C6;
        (carrier_masked_cell(j), chain_pubio_cell(2 * N_COMMIT_LANES + j))
    } else {
        let j = k - N_PUBIO_C1 - N_PUBIO_C6 - N_PUBIO_MASKED;
        (role_out_cell(j), chain_pubio_cell(PUBIO_ROLE_ROOT_OFF + j))
    }
}

/// The `k`-th NATIVE sponge-output→public-value binding residual over the opened cells,
/// `k ∈ [0, N_PUBIO_BIND)`: `output_cell − pubio_cell` for the `(output, pubio)` pair from
/// [`pubio_bind_pair`]. On an honest row the mirrored sponge output already equals the public-IO
/// lane the chain absorbs, so the residual vanishes; a forged public commitment (≠ mirrored output)
/// or a tampered mirrored output makes it non-zero ⇒ rejects.
fn pubio_bind_residual(cells: &[Challenge; N_OPENED_CELLS], k: usize) -> Challenge {
    let (out_cell, pubio_cell) = pubio_bind_pair(k);
    cells[out_cell] - cells[pubio_cell]
}

/// Map the `ri`-th committed post (in `inlined_perm_octic::perm_residuals` order: begin-full posts,
/// then partial post_sbox, then end-full posts) to its CELL offset within a `PERM_COLS` block (the
/// `inlined_perm` column layout). Used to read the OPENED committed-post cell for the C1 residual.
fn c1_post_cell_offset(ri: usize) -> usize {
    use crate::inlined_perm::{
        C_BEGIN_POST, C_END_POST, C_PARTIAL_POST_SBOX, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    };
    let begin = HALF_FULL_ROUNDS * WIDTH;
    let partial = PARTIAL_ROUNDS;
    if ri < begin {
        C_BEGIN_POST + ri
    } else if ri < begin + partial {
        C_PARTIAL_POST_SBOX + (ri - begin)
    } else {
        C_END_POST + (ri - begin - partial)
    }
}

/// The `r`-th leaf constraint residual over the opened cells, `r ∈ [0, N_CONSTRAINTS)`. Ordering:
///   * `0 .. C4_LANES*C4_N_PER_LANE`            : C4 lane residuals (lane-major).
///   * next `N_C4_ACC`                          : `carry_acc[j] − v_j` (C4-acc binding).
///   * next `N_SEAM_PAD`                        : `c1_pad_seed[b] − c2_pad_seed[b]` (SeamPadSeed).
///   * next `N_SEAM_ACC`                        : `carry_acc[i] − c6_field[i]` (SeamOpeningAcc).
///   * next `N_RELATION_ID`                     : `relation_id[j] − RELATION_ID[j]` (pinning).
///   * next `N_C1`                              : C1 sponge residuals (pad_source_commitment).
///   * next `N_C6`                              : C6 sponge residuals (opening_tuple_commitment).
///   * next `N_ROLE`                            : role-root sponge residuals (CLASS A, chained).
pub fn constraint_residual(cells: &[Challenge; N_OPENED_CELLS], r: usize) -> Challenge {
    let mut r = r;
    let c4_total = C4_LANES * C4_N_PER_LANE;
    if r < c4_total {
        let lane = r / C4_N_PER_LANE;
        let i = r % C4_N_PER_LANE;
        return c4_lane_residual(cells, lane, i);
    }
    r -= c4_total;
    if r < N_C4_ACC {
        // carry_acc[j] − v_j (air.rs:762).
        let j = r;
        return cells[acc_cell(j)] - cells[c4_cell(j, I_V)];
    }
    r -= N_C4_ACC;
    if r < N_SEAM_PAD {
        // SeamPadSeed: c1_pad_seed[b] − c2_pad_seed[b].
        let b = r;
        return cells[c1_pad_cell(b)] - cells[c2_pad_cell(b)];
    }
    r -= N_SEAM_PAD;
    if r < N_SEAM_ACC {
        // SeamOpeningAcc: carry_acc[i] − c6_field[i].
        let i = r;
        return cells[acc_cell(i)] - cells[c6_field_cell(i)];
    }
    r -= N_SEAM_ACC;
    if r < N_RELATION_ID {
        // relation_id pinning: relation_id[j] − RELATION_ID[j].
        let j = r;
        return cells[relation_id_cell(j)] - Challenge::from(Val::from_u32(RELATION_ID[j]));
    }
    r -= N_RELATION_ID;
    if r < N_C1 {
        // C1 sponge residuals (pad_source_commitment squeeze).
        return c1_residual(cells, r);
    }
    r -= N_C1;
    if r < N_C6 {
        // C6 sponge residuals (opening_tuple_commitment squeeze).
        return c6_residual(cells, r);
    }
    r -= N_C6;
    if r < N_ROLE {
        // role-root sponge residuals (CLASS A, CHAINED to C1/C6 outputs).
        return role_residual(cells, r);
    }
    r -= N_ROLE;
    if r < N_C2 {
        // C2 mask-PRF sponge residuals (the largest sponge; 30 squeezed lanes feed the reduce gadget).
        return c2_residual(cells, r);
    }
    r -= N_C2;
    if r < N_REDUCE {
        // wide-then-reduce + carrier residuals (30 squeezed lanes -> 6 masks -> masked_projection mod q).
        return reduce_residual(cells, r);
    }
    r -= N_REDUCE;
    if r < N_CHAIN {
        // batch digest chain residuals (per-row absorb + chain_out + CROSS-ROW carry + count acc).
        return chain_residual(cells, r);
    }
    r -= N_CHAIN;
    if r < N_BYTERANGE {
        // PER-LANE BYTE-RANGE / canonicality of the C1/C2/C6 sponge INPUT lanes (air.rs:1278-1289).
        return byterange_residual(cells, r);
    }
    r -= N_BYTERANGE;
    // SPONGE-OUTPUT → PUBLIC-VALUE binding (the per-instance commitment equalities, air.rs batch
    // bindings): pad_source_commitment / opening_tuple_commitment / masked_projection / role_root
    // == the SAME per-row pubio cells the digest chain absorbs.
    debug_assert!(r < N_PUBIO_BIND);
    pubio_bind_residual(cells, r)
}

/// Native alpha-batched leaf partial constraint eval over opened cells:
/// `C_partial = Σ_r alpha^r · c_r(cells)`. This is the AUTHORITY; the AIR re-derives it in-circuit.
pub fn partial_constraint_eval(cells: &[Challenge; N_OPENED_CELLS], alpha: Challenge) -> Challenge {
    let mut acc = Challenge::ZERO;
    let mut a_pow = Challenge::ONE;
    for r in 0..N_CONSTRAINTS {
        acc += a_pow * constraint_residual(cells, r);
        a_pow *= alpha;
    }
    acc
}

/// Build one honest leaf-constraint-mirror row from opened cells + `alpha`, computing `C_partial`
/// via the native reference. (The builder can only emit honest rows; tamper tests mutate the
/// matrix afterwards.)
pub fn pack_row(cells: &[Challenge; N_OPENED_CELLS], alpha: Challenge) -> Vec<Val> {
    let c_partial = partial_constraint_eval(cells, alpha);
    let mut row = vec![Val::ZERO; n_cols()];
    for c in 0..N_OPENED_CELLS {
        row[cell_base(c)..cell_base(c) + D].copy_from_slice(&limbs(cells[c]));
    }
    row[L_ALPHA..L_ALPHA + D].copy_from_slice(&limbs(alpha));
    row[L_CPARTIAL..L_CPARTIAL + D].copy_from_slice(&limbs(c_partial));
    row
}

/// TEST-ONLY: a fully-honest opened-cell vector (canonical C4 lanes + matching seams/relation_id +
/// an HONEST C1 sponge built from the deployed byte-pinned Val permutation). All mirrored residuals
/// vanish. Exposed for cross-module end-to-end tests (e.g. `deep_air`).
#[cfg(test)]
pub fn honest_cells_for_test(
    vals: [u32; C4_LANES],
    pad_seed_seed: u32,
) -> [Challenge; N_OPENED_CELLS] {
    tests::honest_cells(vals, pad_seed_seed)
}

/// TEST-ONLY: the C1 sponge input-lane / perm / output cell accessors, for cross-module tamper
/// tests. Returns the cell INDEX (multiply by `D` for the limb base).
#[cfg(test)]
pub fn c1_input_cell_idx(i: usize) -> usize {
    c1_input_cell(i)
}
#[cfg(test)]
pub fn c1_out_cell_idx(j: usize) -> usize {
    c1_out_cell(j)
}
#[cfg(test)]
pub fn c1_perm_cell_idx(p: usize, within: usize) -> usize {
    c1_perm_base(p) + within
}
#[cfg(test)]
pub fn c6_byte_cell_idx(b: usize) -> usize {
    c6_byte_cell(b)
}
#[cfg(test)]
pub fn c6_field_cell_idx(i: usize) -> usize {
    c6_field_cell(i)
}
#[cfg(test)]
pub fn c6_out_cell_idx(j: usize) -> usize {
    c6_out_cell(j)
}
#[cfg(test)]
pub fn c6_perm_cell_idx(p: usize, within: usize) -> usize {
    c6_perm_base(p) + within
}
#[cfg(test)]
pub fn role_masked_cell_idx(i: usize) -> usize {
    role_masked_cell(i)
}
#[cfg(test)]
pub fn role_out_cell_idx(j: usize) -> usize {
    role_out_cell(j)
}
#[cfg(test)]
pub fn role_perm_cell_idx(p: usize, within: usize) -> usize {
    role_perm_base(p) + within
}
/// TEST-ONLY: cross-module C1/C6 output cell accessors for the CHAINING tamper (a tampered C1/C6
/// squeeze output must ALSO break the role-root since the role absorb reads those cells).
#[cfg(test)]
pub fn c1_out_chain_cell_idx(j: usize) -> usize {
    c1_out_cell(j)
}
/// TEST-ONLY: C2 sponge + reduce-gadget + carrier cell accessors for cross-module tamper tests.
#[cfg(test)]
pub fn c2_slot_cell_idx(i: usize) -> usize {
    c2_slot_cell(i)
}
#[cfg(test)]
pub fn c2_perm_cell_idx(p: usize, within: usize) -> usize {
    c2_perm_base(p) + within
}
/// The OOD cell holding squeezed C2 output lane `o` (the reduce gadget's `squeezed[o]` source).
#[cfg(test)]
pub fn c2_squeeze_output_cell_idx(o: usize) -> usize {
    c2_squeeze_output_cell(o)
}
/// A reduce-gadget witness cell `i` (`i < REDUCE_CELLS`).
#[cfg(test)]
pub fn reduce_cell_idx(i: usize) -> usize {
    reduce_cell(i)
}
/// The carrier masked_projection value cell for lane `j`.
#[cfg(test)]
pub fn carrier_masked_cell_idx(j: usize) -> usize {
    carrier_masked_cell(j)
}
/// The `within`-th cell of carrier mod-q block for lane `j` (`within < MODQ_CELLS`).
#[cfg(test)]
pub fn carrier_modq_cell_idx(j: usize, within: usize) -> usize {
    carrier_modq_base(j) + within
}
/// TEST-ONLY: batch-digest-chain cell accessors for cross-module tamper tests.
#[cfg(test)]
pub fn chain_prev_out_cell_idx(c: usize) -> usize {
    chain_prev_out_cell(c)
}
#[cfg(test)]
pub fn chain_in_cell_idx(c: usize) -> usize {
    chain_in_cell(c)
}
#[cfg(test)]
pub fn chain_perm_cell_idx(p: usize, within: usize) -> usize {
    chain_perm_base(p) + within
}
#[cfg(test)]
pub fn chain_out_cell_idx(c: usize) -> usize {
    chain_out_cell(c)
}
#[cfg(test)]
pub fn chain_enable_cell_idx() -> usize {
    chain_enable_cell()
}
#[cfg(test)]
pub fn chain_prev_count_cell_idx() -> usize {
    chain_prev_count_cell()
}
#[cfg(test)]
pub fn chain_count_cell_idx() -> usize {
    chain_count_cell()
}
#[cfg(test)]
pub fn chain_pubio_cell_idx(i: usize) -> usize {
    chain_pubio_cell(i)
}
/// TEST-ONLY: the `(mirrored-output cell, public/pubio cell)` pair the `k`-th sponge-output→
/// public-value binding equates (`k < N_PUBIO_BIND`). Used by cross-module end-to-end tamper tests.
#[cfg(test)]
pub fn pubio_bind_pair_idx(k: usize) -> (usize, usize) {
    pubio_bind_pair(k)
}
/// TEST-ONLY: number of sponge-output→public-value bindings.
#[cfg(test)]
pub fn n_pubio_bind() -> usize {
    N_PUBIO_BIND
}
/// TEST-ONLY: byte-range bit-witness cell accessor (global byte-range index `g`, bit `b`).
#[cfg(test)]
pub fn byterange_bit_cell_idx(g: usize, b: usize) -> usize {
    byterange_bit_cell(g, b)
}
/// TEST-ONLY: the ALREADY-ABSORBED sponge input cell that byte-range index `g` decomposes.
#[cfg(test)]
pub fn byterange_absorbed_cell_idx(g: usize) -> usize {
    byterange_absorbed_cell(g)
}

/// Decode the committed `C_partial` cell of a packed row.
pub fn row_c_partial(row: &[Val]) -> Challenge {
    Challenge::from_basis_coefficients_slice(&row[L_CPARTIAL..L_CPARTIAL + D])
        .expect("C_partial decode")
}

/// Push the PER-LANE BYTE-RANGE / canonicality residuals (air.rs:1278-1289) onto `residuals`, in the
/// SAME order as the native [`byterange_residual`]: lane-major over the global byte-range index
/// (C1 48 ‖ C2 40 ‖ C6 80), each lane emitting BYTE_BITS bit-bool residuals `bit·(bit−1)` then 1
/// recompose eq `lane − Σ_b bit_b·2^b` binding the ACTUAL absorbed sponge input cell (NOT a free
/// column). EXTRACTED into its own (`#[inline(never)]`) frame to keep `recompute_c_partial`'s frame
/// small (1512 octic residuals), matching the existing per-class extraction pattern.
#[inline(never)]
fn push_byterange_residuals<AB, FCell>(cell: &FCell, residuals: &mut Vec<[AB::Expr; D]>)
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let one = octic_scalar::<AB>(1);
    for g in 0..N_BR_LANES {
        // BYTE_BITS bit-bool residuals bit·(bit−1).
        for b in 0..BYTE_BITS {
            let bit = cell(byterange_bit_cell(g, b));
            let bit_minus_one = octic_sub::<AB>(&bit, &one);
            residuals.push(octic_mul::<AB>(&bit, &bit_minus_one));
        }
        // recompose eq: lane − Σ_b bit_b·2^b (lane = the genuine absorbed sponge input cell).
        let lane = cell(byterange_absorbed_cell(g));
        let mut acc = octic_scalar::<AB>(0);
        for b in 0..BYTE_BITS {
            let bit = cell(byterange_bit_cell(g, b));
            let scaled = scale::<AB>(&bit, 1u64 << b);
            acc = octic_add::<AB>(&acc, &scaled);
        }
        residuals.push(octic_sub::<AB>(&lane, &acc));
    }
}

/// Push the BATCH DIGEST CHAIN residuals (the per-row running Poseidon2 sponge + the CROSS-ROW
/// transition) onto `residuals`, in the SAME order as the native [`chain_residual`]. EXTRACTED from
/// [`recompute_c_partial`] into its own (`#[inline(never)]`) frame because the chain folds 24
/// Poseidon2 perms — keeping its large per-perm locals out of the already-huge `recompute_c_partial`
/// frame is what prevents a stack overflow when the mirror is evaluated row-wise. Mirrors
/// `descender_account_stark::air::FullRelationAir::eval_digest_chain` (air.rs:1109-1260).
#[inline(never)]
fn push_chain_residuals<AB, FCell>(cell: &FCell, residuals: &mut Vec<[AB::Expr; D]>)
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    struct CellAdapter<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> {
        cell: &'a FCell,
        _ab: core::marker::PhantomData<AB>,
    }
    impl<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> OcticPermCells<AB>
        for CellAdapter<'a, AB, FCell>
    {
        fn at(&self, idx: usize) -> [AB::Expr; D] {
            (self.cell)(idx)
        }
    }
    let adapter = CellAdapter::<AB, FCell> { cell, _ab: core::marker::PhantomData };
    let one = octic_scalar::<AB>(1);

    let perm_input = |p: usize, i: usize| cell(chain_perm_base(p) + C_INPUTS + i);
    let perm_output = |p: usize, i: usize| cell(chain_perm_base(p) + C_OUTPUT + i);
    let enable = cell(chain_enable_cell());
    // msg element at stream position `pos` (enable·pubio products folded in).
    let msg = |pos: usize| -> [AB::Expr; D] {
        match chain_absorb_source(pos) {
            ChainAbsorb::Enable => enable.clone(),
            ChainAbsorb::EnablePubio(i) => octic_mul::<AB>(&enable, &cell(chain_pubio_cell(i))),
            ChainAbsorb::Zero => octic_scalar::<AB>(0),
        }
    };

    // (a) enable boolean.
    {
        let em1 = octic_sub::<AB>(&enable, &one);
        residuals.push(octic_mul::<AB>(&enable, &em1));
    }

    // (b) input/capacity bindings.
    for p in 0..CHAIN_NPERMS {
        for lane in 0..CHAIN_W {
            let inp = perm_input(p, lane);
            let expected = if p == 0 {
                let chain_in = cell(chain_in_cell(lane));
                if lane < CHAIN_RT {
                    octic_add::<AB>(&chain_in, &msg(lane))
                } else {
                    chain_in
                }
            } else {
                let prev = perm_output(p - 1, lane);
                if lane < CHAIN_RT {
                    octic_add::<AB>(&prev, &msg(p * CHAIN_RT + lane))
                } else {
                    prev
                }
            };
            residuals.push(octic_sub::<AB>(&inp, &expected));
        }
    }

    // (c) per-perm Poseidon2 round residuals.
    for p in 0..CHAIN_NPERMS {
        let mut pr = inlined_perm_octic::perm_residuals::<AB, _>(&adapter, chain_perm_base(p));
        residuals.append(&mut pr);
    }

    // (d) chain_out binding: chain_out[c] == enable·absorbed[c] + (1−enable)·chain_in[c].
    for c in 0..CHAIN_W {
        let absorbed = perm_output(CHAIN_NPERMS - 1, c);
        let chain_in = cell(chain_in_cell(c));
        let one_minus_en = octic_sub::<AB>(&one, &enable);
        let expected = octic_add::<AB>(
            &octic_mul::<AB>(&enable, &absorbed),
            &octic_mul::<AB>(&one_minus_en, &chain_in),
        );
        residuals.push(octic_sub::<AB>(&cell(chain_out_cell(c)), &expected));
    }

    // (e) cross-row carry: chain_in[c] == prev_chain_out[c].
    for c in 0..CHAIN_W {
        residuals.push(octic_sub::<AB>(
            &cell(chain_in_cell(c)),
            &cell(chain_prev_out_cell(c)),
        ));
    }

    // (f) count accumulator: count_acc == prev_count_acc + enable.
    {
        let expected = octic_add::<AB>(&cell(chain_prev_count_cell()), &enable);
        residuals.push(octic_sub::<AB>(&cell(chain_count_cell()), &expected));
    }
}

/// Push the SPONGE-OUTPUT → PUBLIC-VALUE binding residuals (`output_cell − pubio_cell` for each
/// per-instance commitment output) onto `residuals`, in the SAME order as the native
/// [`pubio_bind_residual`]. Mirrors the air.rs BATCH bindings (air.rs:712-756, 898-906, 941-953):
/// the mirrored C1/C6/masked/role outputs are constrained EQUAL to the per-row pubio cells the
/// digest chain absorbs (`chain_pubio_cell` — the SAME public-value cells, one source of truth).
/// EXTRACTED into its own (`#[inline(never)]`) frame to keep `recompute_c_partial`'s frame small.
#[inline(never)]
fn push_pubio_bind_residuals<AB, FCell>(cell: &FCell, residuals: &mut Vec<[AB::Expr; D]>)
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    for k in 0..N_PUBIO_BIND {
        let (out_cell, pubio_cell) = pubio_bind_pair(k);
        residuals.push(octic_sub::<AB>(&cell(out_cell), &cell(pubio_cell)));
    }
}

// ===========================================================================
// The AIR
// ===========================================================================

/// REUSABLE: recompute the alpha-batched Poseidon2-free leaf `C_partial(zeta)` over opened OOD
/// cells, as in-AIR extension expressions. `cell(c)` returns opened cell `c ∈ [0, N_OPENED_CELLS)`
/// as a `[AB::Expr; D]` octic element; `alpha` is the batching challenge block. The returned
/// `[AB::Expr; D]` is `Σ_r alpha^r · c_r(cells)` — the SAME quantity [`partial_constraint_eval`]
/// computes natively and the SAME residual ORDER. [`LeafConstraintMirrorAir`] binds this to the
/// committed `C_partial`; [`crate::deep_air`] wires it as a constrained summand of the DEEP
/// `C(zeta)`. Soundness lives ENTIRELY in this function's faithfulness to air.rs (fidelity-PINned).
pub fn recompute_c_partial<AB, FCell>(cell: &FCell, alpha: &[AB::Expr; D]) -> [AB::Expr; D]
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    let one = octic_scalar::<AB>(1);
    let max_hi = octic_scalar::<AB>(C4_MAX_HI as u64);
    let base_radix = octic_scalar::<AB>(C4_BASE as u64);

    // recompose 7 bit cells starting at cell index `start` into Σ bit_b · 2^b.
    let recompose = |start_cell: usize| -> [AB::Expr; D] {
        let mut acc = octic_scalar::<AB>(0);
        for b in 0..C4_DIGIT_BITS {
            let bit = cell(start_cell + b);
            let scaled = scale::<AB>(&bit, 1u64 << b);
            acc = octic_add::<AB>(&acc, &scaled);
        }
        acc
    };

    // ----- build the residuals (each an extension element), SAME order as the native ref -----
    let mut residuals: Vec<[AB::Expr; D]> = Vec::with_capacity(N_CONSTRAINTS);

    // (1) C4 lanes (lane-major).
    for lane in 0..C4_LANES {
        let v = cell(c4_cell(lane, I_V));
        let d_lo = cell(c4_cell(lane, I_DLO));
        let d_hi = cell(c4_cell(lane, I_DHI));
        let is96 = cell(c4_cell(lane, I_IS96));
        let inv = cell(c4_cell(lane, I_INV));

        // 0..21 : bit-boolean residuals bit·(bit−1) over lo[7] ‖ hi[7] ‖ slack[7].
        for i in 0..21 {
            let bit = cell(c4_cell(lane, I_LO_BITS + i));
            let bit_minus_one = octic_sub::<AB>(&bit, &one);
            residuals.push(octic_mul::<AB>(&bit, &bit_minus_one));
        }
        // 21 : d_lo − recompose(lo_bits)
        residuals.push(octic_sub::<AB>(&d_lo, &recompose(c4_cell(lane, I_LO_BITS))));
        // 22 : d_hi − recompose(hi_bits)
        residuals.push(octic_sub::<AB>(&d_hi, &recompose(c4_cell(lane, I_HI_BITS))));
        // 23 : (96 − d_hi) − recompose(slack_bits)
        let max_minus_dhi = octic_sub::<AB>(&max_hi, &d_hi);
        residuals.push(octic_sub::<AB>(
            &max_minus_dhi,
            &recompose(c4_cell(lane, I_SLACK_BITS)),
        ));
        // 24 : is96·(is96−1)
        let is96_minus_one = octic_sub::<AB>(&is96, &one);
        residuals.push(octic_mul::<AB>(&is96, &is96_minus_one));
        // 25 : is96·(d_hi − 96)
        let dhi_minus_max = octic_sub::<AB>(&d_hi, &max_hi);
        residuals.push(octic_mul::<AB>(&is96, &dhi_minus_max));
        // 26 : (d_hi − 96)·inv − (1 − is96)
        let prod = octic_mul::<AB>(&dhi_minus_max, &inv);
        let one_minus_is96 = octic_sub::<AB>(&one, &is96);
        residuals.push(octic_sub::<AB>(&prod, &one_minus_is96));
        // 27 : is96·d_lo
        residuals.push(octic_mul::<AB>(&is96, &d_lo));
        // 28 : v − (d_lo + 128·d_hi)
        let base_dhi = octic_mul::<AB>(&base_radix, &d_hi);
        let recomposed_v = octic_add::<AB>(&d_lo, &base_dhi);
        residuals.push(octic_sub::<AB>(&v, &recomposed_v));
    }

    // (2) C4-acc carrier binding: carry_acc[j] − v_j.
    for j in 0..N_C4_ACC {
        let acc = cell(acc_cell(j));
        let v = cell(c4_cell(j, I_V));
        residuals.push(octic_sub::<AB>(&acc, &v));
    }

    // (3) SeamPadSeed: c1_pad_seed[b] − c2_pad_seed[b].
    for b in 0..N_SEAM_PAD {
        let c1b = cell(c1_pad_cell(b));
        let c2b = cell(c2_pad_cell(b));
        residuals.push(octic_sub::<AB>(&c1b, &c2b));
    }

    // (4) SeamOpeningAcc: carry_acc[i] − c6_field[i].
    for i in 0..N_SEAM_ACC {
        let acc = cell(acc_cell(i));
        let c6 = cell(c6_field_cell(i));
        residuals.push(octic_sub::<AB>(&acc, &c6));
    }

    // (5) relation_id pinning: relation_id[j] − RELATION_ID[j].
    for j in 0..N_RELATION_ID {
        let rid = cell(relation_id_cell(j));
        let pinned = octic_scalar::<AB>(RELATION_ID[j] as u64);
        residuals.push(octic_sub::<AB>(&rid, &pinned));
    }

    // (6) C1 / (7) C6 / (8) role-root squeeze sponges — EXTRACTED into the generic
    // `push_squeeze_sponge_residuals` (its own stack frame per call), to keep this function's frame
    // small enough to evaluate row-wise without a stack overflow. Each call mirrors the native
    // `c1_residual` / `c6_residual` / `role_residual` EXACTLY (input/capacity bindings, per-perm
    // Poseidon2 round residuals, squeeze-output equalities) — same order, same constants. The role
    // sponge's `role_absorb_source` reads the ALREADY-MIRRORED C1/C6 output cells (the chaining).
    push_squeeze_sponge_residuals::<AB, FCell>(
        cell, &mut residuals, C1_N_PERMS, C1_N_ABSORB, C1_RATE, C1_PADDED_LEN, N_C1_OUT,
        c1_perm_base, c1_absorb_source, c1_output_source, c1_out_cell,
    );
    push_squeeze_sponge_residuals::<AB, FCell>(
        cell, &mut residuals, C6_N_PERMS, C6_N_ABSORB, C6_RATE, C6_PADDED_LEN, N_C6_OUT,
        c6_perm_base, c6_absorb_source, c6_output_source, c6_out_cell,
    );
    push_squeeze_sponge_residuals::<AB, FCell>(
        cell, &mut residuals, ROLE_N_PERMS, ROLE_N_ABSORB, ROLE_RATE, ROLE_PADDED_LEN, N_ROLE_OUT,
        role_perm_base, role_absorb_source, role_output_source, role_out_cell,
    );

    // (9) C2 mask-PRF sponge — the LARGEST mirrored leaf sponge. EXTRACTED into
    // `push_c2_residuals` (its own stack frame) alongside the reduce/chain extractions, to keep this
    // function's frame small enough to evaluate row-wise without a stack overflow. Order MUST match
    // the native `c2_residual`: input/capacity bindings (WIDTH/perm), then per-perm Poseidon2 round
    // residuals. NO squeeze-output equality sub-class.
    push_c2_residuals::<AB, FCell>(cell, &mut residuals);

    // (10) WIDE-THEN-REDUCE + carrier — the genuinely-NEW constraint shape. EXTRACTED into
    // `push_reduce_residuals` (its own stack frame) alongside the chain extraction, to keep this
    // function's frame small enough to evaluate row-wise without a stack overflow. Mirrors
    // `reduce_gadget::eval_reduce_gadget` + the C4 carrier mod-q reduction, SAME order as the native
    // `reduce_residual`.
    push_reduce_residuals::<AB, FCell>(cell, &mut residuals);

    // (11) BATCH DIGEST CHAIN — EXTRACTED into `push_chain_residuals` (its own stack frame). The
    // chain folds 24 Poseidon2 perms; keeping its locals out of this frame avoids a stack overflow
    // when the whole mirror is evaluated row-wise. Order MUST match the native `chain_residual`.
    push_chain_residuals::<AB, FCell>(cell, &mut residuals);

    // (12) PER-LANE BYTE-RANGE / canonicality of the C1/C2/C6 sponge INPUT lanes (air.rs:1278-1289).
    // EXTRACTED into `push_byterange_residuals` (its own `#[inline(never)]` frame) to keep this
    // folder frame small. Order MUST match the native `byterange_residual`: lane-major over the
    // global byte-range index (C1 48 ‖ C2 40 ‖ C6 80), each lane = BYTE_BITS bit-bools then 1
    // recompose eq binding the ACTUAL absorbed sponge input cell.
    push_byterange_residuals::<AB, FCell>(cell, &mut residuals);

    // (13) SPONGE-OUTPUT → PUBLIC-VALUE binding — EXTRACTED into `push_pubio_bind_residuals` (its own
    // `#[inline(never)]` frame). Binds the mirrored C1/C6/masked/role outputs EQUAL to the per-row
    // pubio cells the digest chain absorbs (the SAME public-value cells — one source of truth). Order
    // MUST match the native `pubio_bind_residual`. This is the LAST class.
    push_pubio_bind_residuals::<AB, FCell>(cell, &mut residuals);

    debug_assert_eq!(residuals.len(), N_CONSTRAINTS);

    // ----- alpha-batch: acc = Σ_r alpha^r · residual_r -----
    let mut acc = octic_scalar::<AB>(0);
    let mut a_pow = octic_scalar::<AB>(1);
    for res in residuals.iter() {
        let term = octic_mul::<AB>(&a_pow, res);
        acc = octic_add::<AB>(&acc, &term);
        a_pow = octic_mul::<AB>(&a_pow, alpha);
    }
    acc
}

/// Push a SQUEEZE sponge's residuals (input/capacity bindings ‖ per-perm Poseidon2 round residuals ‖
/// squeeze-output equalities) onto `residuals`, in the SAME order as the per-sponge native reference
/// (`c1_residual` / `c6_residual` / `role_residual`). GENERIC over the sponge constants + the
/// absorb-source / perm-base / output-source / output-cell mappings, so C1/C6/role share ONE body.
/// EXTRACTED from [`recompute_c_partial`] into its own (`#[inline(never)]`) frame per call to keep
/// the main folder frame small. Mirrors `eval_sponge`.
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn push_squeeze_sponge_residuals<AB, FCell>(
    cell: &FCell,
    residuals: &mut Vec<[AB::Expr; D]>,
    n_perms: usize,
    n_absorb: usize,
    rate: usize,
    padded_len: usize,
    n_out: usize,
    perm_base: fn(usize) -> usize,
    absorb_source: fn(usize) -> C1Absorb,
    output_source: fn(usize) -> (usize, usize),
    out_cell: fn(usize) -> usize,
) where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    struct CellAdapter<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> {
        cell: &'a FCell,
        _ab: core::marker::PhantomData<AB>,
    }
    impl<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> OcticPermCells<AB>
        for CellAdapter<'a, AB, FCell>
    {
        fn at(&self, idx: usize) -> [AB::Expr; D] {
            (self.cell)(idx)
        }
    }
    let adapter = CellAdapter::<AB, FCell> { cell, _ab: core::marker::PhantomData };

    let perm_input = |p: usize, i: usize| cell(perm_base(p) + C_INPUTS + i);
    let perm_output = |p: usize, i: usize| cell(perm_base(p) + C_OUTPUT + i);
    let absorb = |pos: usize| -> [AB::Expr; D] {
        match absorb_source(pos) {
            C1Absorb::Cell(c) => cell(c),
            C1Absorb::One => octic_scalar::<AB>(1),
            C1Absorb::Zero => octic_scalar::<AB>(0),
        }
    };

    // (a) input/capacity bindings.
    for p in 0..n_perms {
        let is_absorb = p < n_absorb;
        for lane in 0..WIDTH {
            let inp = perm_input(p, lane);
            let expected = if p == 0 {
                if lane < rate {
                    absorb(lane)
                } else {
                    octic_scalar::<AB>(0)
                }
            } else {
                let prev = perm_output(p - 1, lane);
                if lane < rate {
                    if is_absorb {
                        let pos = p * rate + lane;
                        let elem = if pos < padded_len { absorb(pos) } else { octic_scalar::<AB>(0) };
                        octic_add::<AB>(&prev, &elem)
                    } else {
                        prev
                    }
                } else {
                    prev
                }
            };
            residuals.push(octic_sub::<AB>(&inp, &expected));
        }
    }

    // (b) per-perm Poseidon2 round residuals.
    for p in 0..n_perms {
        let mut pr = inlined_perm_octic::perm_residuals::<AB, _>(&adapter, perm_base(p));
        residuals.append(&mut pr);
    }

    // (c) squeeze-output equalities: committed_digest[j] − squeezed_output[j] (the source perm's
    // C_OUTPUT cell; bound to the recomputed permutation by the round residuals in (b)).
    for j in 0..n_out {
        let (sp, lane) = output_source(j);
        let src = cell(perm_base(sp) + C_OUTPUT + lane);
        let committed = cell(out_cell(j));
        residuals.push(octic_sub::<AB>(&committed, &src));
    }
}

/// Push the C2 mask-PRF sponge residuals onto `residuals`, in the SAME order as the native
/// [`c2_residual`]. EXTRACTED from [`recompute_c_partial`] into its own (`#[inline(never)]`) frame
/// to keep the main folder frame small. Mirrors `eval_sponge` for the C2 SpongePlan.
#[inline(never)]
fn push_c2_residuals<AB, FCell>(cell: &FCell, residuals: &mut Vec<[AB::Expr; D]>)
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    struct CellAdapter<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> {
        cell: &'a FCell,
        _ab: core::marker::PhantomData<AB>,
    }
    impl<'a, AB: AirBuilder<F = Val>, FCell: Fn(usize) -> [AB::Expr; D]> OcticPermCells<AB>
        for CellAdapter<'a, AB, FCell>
    {
        fn at(&self, idx: usize) -> [AB::Expr; D] {
            (self.cell)(idx)
        }
    }
    let adapter = CellAdapter::<AB, FCell> { cell, _ab: core::marker::PhantomData };

    let perm_input = |p: usize, i: usize| cell(c2_perm_base(p) + C_INPUTS + i);
    let perm_output = |p: usize, i: usize| cell(c2_perm_base(p) + C_OUTPUT + i);
    let absorb = |pos: usize| -> [AB::Expr; D] {
        match c2_absorb_source(pos) {
            C1Absorb::Cell(c) => cell(c),
            C1Absorb::One => octic_scalar::<AB>(1),
            C1Absorb::Zero => octic_scalar::<AB>(0),
        }
    };

    // (a) input/capacity bindings.
    for p in 0..C2_N_PERMS {
        let is_absorb = p < C2_N_ABSORB;
        for lane in 0..WIDTH {
            let inp = perm_input(p, lane);
            let expected = if p == 0 {
                if lane < C2_RATE {
                    absorb(lane)
                } else {
                    octic_scalar::<AB>(0)
                }
            } else {
                let prev = perm_output(p - 1, lane);
                if lane < C2_RATE {
                    if is_absorb {
                        let pos = p * C2_RATE + lane;
                        let elem = if pos < C2_PADDED_LEN {
                            absorb(pos)
                        } else {
                            octic_scalar::<AB>(0)
                        };
                        octic_add::<AB>(&prev, &elem)
                    } else {
                        prev
                    }
                } else {
                    prev
                }
            };
            residuals.push(octic_sub::<AB>(&inp, &expected));
        }
    }

    // (b) per-perm Poseidon2 round residuals.
    for p in 0..C2_N_PERMS {
        let mut pr = inlined_perm_octic::perm_residuals::<AB, _>(&adapter, c2_perm_base(p));
        residuals.append(&mut pr);
    }
}

/// Push the WIDE-THEN-REDUCE gadget + carrier residuals onto `residuals`, in the SAME order as the
/// native [`reduce_residual`]. EXTRACTED from [`recompute_c_partial`] into its own
/// (`#[inline(never)]`) frame to keep the main folder frame small. Mirrors
/// `descender_account_stark::reduce_gadget::eval_reduce_gadget` + the C4 carrier (air.rs:856-906).
#[inline(never)]
fn push_reduce_residuals<AB, FCell>(cell: &FCell, residuals: &mut Vec<[AB::Expr; D]>)
where
    AB: AirBuilder<F = Val>,
    FCell: Fn(usize) -> [AB::Expr; D],
{
    {
        let (p_lo, p_hi, q, two16) = reduce_field_consts();
        let one = octic_scalar::<AB>(1);
        // squeezed lane `s` is the C2 squeeze OUTPUT cell.
        let squeezed = |s: usize| cell(c2_squeeze_output_cell(s));
        let rcell = |i: usize| cell(reduce_cell(i));
        // octic recompose of `nbits` bit cells (gadget-relative `start`) into Σ bit_b·2^b, pushing the
        // per-bit boolean residuals in order, then returning the accumulated value.
        let push_recompose =
            |residuals: &mut Vec<[AB::Expr; D]>, start: usize, nbits: usize| -> [AB::Expr; D] {
                let mut acc = octic_scalar::<AB>(0);
                for b in 0..nbits {
                    let bit = rcell(start + b);
                    let bm1 = octic_sub::<AB>(&bit, &one);
                    residuals.push(octic_mul::<AB>(&bit, &bm1));
                    let scaled = scale::<AB>(&bit, 1u64 << b);
                    acc = octic_add::<AB>(&acc, &scaled);
                }
                acc
            };

        for j in 0..NUM_OUTPUTS {
            let mask_base = j * PER_MASK_CELLS;
            // (i) lane-bind eqs.
            for i in 0..LANES_PER_MASK {
                let bound = octic_sub::<AB>(&rcell(mask_base + i), &squeezed(j * LANES_PER_MASK + i));
                residuals.push(bound);
            }
            let step0 = mask_base + LANES_PER_MASK;
            for t in 0..HORNER_STEPS {
                let sb = step0 + t * STEP_CELLS;
                let lane_idx = LANES_PER_MASK - 1 - t;
                let digit = squeezed(j * LANES_PER_MASK + lane_idx);

                // value blocks: bit-bools then value-recompose eq (StepOff order).
                let blocks: [(usize, usize, usize); 11] = [
                    (StepOff::QT_LO, StepOff::QT_LO_BITS, LO_BITS),
                    (StepOff::QT_HI, StepOff::QT_HI_BITS, HI_BITS),
                    (StepOff::LANE_LO, StepOff::LANE_LO_BITS, LO_BITS),
                    (StepOff::LANE_HI, StepOff::LANE_HI_BITS, HI_BITS),
                    (StepOff::S0, StepOff::S0_BITS, S_LO_BITS),
                    (StepOff::S1, StepOff::S1_BITS, S_LO_BITS),
                    (StepOff::S2, StepOff::S2_BITS, S_TOP_BITS),
                    (StepOff::C0, StepOff::C0_BITS, S_TOP_BITS),
                    (StepOff::C1, StepOff::C1_BITS, S_TOP_BITS),
                    (StepOff::D0, StepOff::D0_BITS, S_TOP_BITS),
                    (StepOff::D1, StepOff::D1_BITS, S_TOP_BITS),
                ];
                for (val_off, bits_off, nbits) in blocks.iter().copied() {
                    let rc = push_recompose(&mut *residuals, sb + bits_off, nbits);
                    residuals.push(octic_sub::<AB>(&rcell(sb + val_off), &rc));
                }
                // r_t (bits then eq).
                let r_rc = push_recompose(&mut *residuals, sb + StepOff::R_BITS, MASK_BITS);
                residuals.push(octic_sub::<AB>(&rcell(sb + StepOff::R), &r_rc));
                // slack (bits then eq).
                let slack_rc = push_recompose(&mut *residuals, sb + StepOff::SLACK_BITS, MASK_BITS);
                residuals.push(octic_sub::<AB>(&rcell(sb + StepOff::SLACK), &slack_rc));

                // value handles.
                let qt_lo = rcell(sb + StepOff::QT_LO);
                let qt_hi = rcell(sb + StepOff::QT_HI);
                let lane_lo = rcell(sb + StepOff::LANE_LO);
                let lane_hi = rcell(sb + StepOff::LANE_HI);
                let s0 = rcell(sb + StepOff::S0);
                let s1 = rcell(sb + StepOff::S1);
                let s2 = rcell(sb + StepOff::S2);
                let c0 = rcell(sb + StepOff::C0);
                let c1 = rcell(sb + StepOff::C1);
                let d0 = rcell(sb + StepOff::D0);
                let d1 = rcell(sb + StepOff::D1);
                let r_t = rcell(sb + StepOff::R);
                let slack = rcell(sb + StepOff::SLACK);
                let r_prev = if t == 0 {
                    octic_scalar::<AB>(0)
                } else {
                    rcell(step0 + (t - 1) * STEP_CELLS + StepOff::R)
                };

                // r_t + slack == q-1.
                let r_plus_slack = octic_add::<AB>(&r_t, &slack);
                residuals.push(octic_sub::<AB>(&r_plus_slack, &octic_scalar::<AB>(q - 1)));
                // digit == lane_lo + lane_hi·2^16.
                let lane_hi_sh = scale::<AB>(&lane_hi, two16);
                let recomp_digit = octic_add::<AB>(&lane_lo, &lane_hi_sh);
                residuals.push(octic_sub::<AB>(&digit, &recomp_digit));
                // 6 limb/carry eqs.
                let l0 = octic_add::<AB>(&scale::<AB>(&r_prev, p_lo), &lane_lo);
                let l1 = octic_add::<AB>(&scale::<AB>(&r_prev, p_hi), &lane_hi);
                let r0 = octic_add::<AB>(&scale::<AB>(&qt_lo, q), &r_t);
                let r1 = scale::<AB>(&qt_hi, q);
                // l0 == s0 + c0·2^16
                let rhs = octic_add::<AB>(&s0, &scale::<AB>(&c0, two16));
                residuals.push(octic_sub::<AB>(&l0, &rhs));
                // l1 + c0 == s1 + c1·2^16
                let lhs = octic_add::<AB>(&l1, &c0);
                let rhs = octic_add::<AB>(&s1, &scale::<AB>(&c1, two16));
                residuals.push(octic_sub::<AB>(&lhs, &rhs));
                // c1 == s2
                residuals.push(octic_sub::<AB>(&c1, &s2));
                // r0 == s0 + d0·2^16
                let rhs = octic_add::<AB>(&s0, &scale::<AB>(&d0, two16));
                residuals.push(octic_sub::<AB>(&r0, &rhs));
                // r1 + d0 == s1 + d1·2^16
                let lhs = octic_add::<AB>(&r1, &d0);
                let rhs = octic_add::<AB>(&s1, &scale::<AB>(&d1, two16));
                residuals.push(octic_sub::<AB>(&lhs, &rhs));
                // d1 == s2
                residuals.push(octic_sub::<AB>(&d1, &s2));
            }
        }

        // carrier: masked_projection[j] == account_projection[j] + mask[j] mod q12289.
        for j in 0..N_LANES {
            let mb = carrier_modq_base(j);
            let masked_bits_off = mb + 1;
            let slack_off = mb + 1 + MODQ_BITS;
            let slack_bits_off = slack_off + 1;
            let wrap_off = slack_bits_off + MODQ_BITS;
            let masked = cell(carrier_masked_cell(j));
            let acc = cell(acc_cell(j));
            // mask[j] = r_t of the LAST Horner step (the opened cell the reduce binds).
            let mask = {
                let mask_base = j * PER_MASK_CELLS;
                let step0 = mask_base + LANES_PER_MASK;
                let last_sb = step0 + (HORNER_STEPS - 1) * STEP_CELLS;
                cell(reduce_cell(last_sb + StepOff::R))
            };
            // masked-recompose (bits then eq).
            let masked_rc = {
                let mut acc = octic_scalar::<AB>(0);
                for b in 0..MODQ_BITS {
                    let bit = cell(masked_bits_off + b);
                    let bm1 = octic_sub::<AB>(&bit, &one);
                    residuals.push(octic_mul::<AB>(&bit, &bm1));
                    acc = octic_add::<AB>(&acc, &scale::<AB>(&bit, 1u64 << b));
                }
                acc
            };
            residuals.push(octic_sub::<AB>(&masked, &masked_rc));
            // slack-recompose (bits then eq).
            let slack_rc = {
                let mut acc = octic_scalar::<AB>(0);
                for b in 0..MODQ_BITS {
                    let bit = cell(slack_bits_off + b);
                    let bm1 = octic_sub::<AB>(&bit, &one);
                    residuals.push(octic_mul::<AB>(&bit, &bm1));
                    acc = octic_add::<AB>(&acc, &scale::<AB>(&bit, 1u64 << b));
                }
                acc
            };
            residuals.push(octic_sub::<AB>(&cell(slack_off), &slack_rc));
            // masked + slack == q-1.
            let m_plus_s = octic_add::<AB>(&masked, &cell(slack_off));
            residuals.push(octic_sub::<AB>(&m_plus_s, &octic_scalar::<AB>(q - 1)));
            // wrap-bool.
            let wrap = cell(wrap_off);
            let wm1 = octic_sub::<AB>(&wrap, &one);
            residuals.push(octic_mul::<AB>(&wrap, &wm1));
            // acc + mask == masked + wrap·q.
            let lhs = octic_add::<AB>(&acc, &mask);
            let rhs = octic_add::<AB>(&masked, &scale::<AB>(&wrap, q));
            residuals.push(octic_sub::<AB>(&lhs, &rhs));
        }
    }
}

/// Leaf-constraint-mirror AIR: recomputes the alpha-batched Poseidon2-free leaf constraint
/// residual `C_partial(zeta)` over the opened OOD cells (extension elements) and binds it to the
/// committed `C_partial`.
pub struct LeafConstraintMirrorAir;

/// Back-compat alias (this module historically exported the single-lane C4 mirror as
/// `LeafC4MirrorAir`; it is now the full Poseidon2-free constraint mirror).
pub type LeafC4MirrorAir = LeafConstraintMirrorAir;

impl<F: Sync> BaseAir<F> for LeafConstraintMirrorAir {
    fn width(&self) -> usize {
        n_cols()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for LeafConstraintMirrorAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let block = |base: usize| -> [AB::Expr; D] { core::array::from_fn(|i| l(base + i)) };

        let cell = |c: usize| -> [AB::Expr; D] { block(cell_base(c)) };
        let alpha = block(L_ALPHA);

        // recompute the alpha-batched leaf C_partial over the opened cells (the reusable folder).
        let acc = recompute_c_partial::<AB, _>(&cell, &alpha);

        // ----- bind acc == committed C_partial (limb-for-limb) -----
        let c_partial = block(L_CPARTIAL);
        for i in 0..D {
            builder.assert_zero(c_partial[i].clone() - acc[i].clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_air::check_all_constraints;
    use p3_field::{Field, PrimeCharacteristicRing};
    use p3_matrix::dense::RowMajorMatrix;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }

    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    /// Honest C4 lane cells for a canonical value `v ∈ [0, q12289)`: mirrors air.rs `c4_witness`
    /// but lifts each base-field cell into the extension.
    fn c4_lane_cells(v: u32) -> [Challenge; C4_CELLS] {
        let d_lo = v % C4_BASE;
        let d_hi = v / C4_BASE;
        let slack = C4_MAX_HI - d_hi;
        let is96 = d_hi == C4_MAX_HI;
        let inv = if is96 {
            Val::ZERO
        } else {
            (Val::from_u32(d_hi) - Val::from_u32(C4_MAX_HI)).inverse()
        };
        let bits = |x: u32| -> [Challenge; C4_DIGIT_BITS] {
            core::array::from_fn(|k| ext((x >> k) & 1))
        };
        let mut cells = [Challenge::ZERO; C4_CELLS];
        cells[I_V] = ext(v);
        cells[I_DLO] = ext(d_lo);
        cells[I_DHI] = ext(d_hi);
        cells[I_LO_BITS..I_LO_BITS + 7].copy_from_slice(&bits(d_lo));
        cells[I_HI_BITS..I_HI_BITS + 7].copy_from_slice(&bits(d_hi));
        cells[I_SLACK_BITS..I_SLACK_BITS + 7].copy_from_slice(&bits(slack));
        cells[I_IS96] = if is96 { Challenge::ONE } else { Challenge::ZERO };
        cells[I_INV] = Challenge::from(inv);
        cells
    }

    /// A fully-honest opened-cell vector: 6 canonical C4 lanes, carry_acc = each lane's v,
    /// C6 field = carry_acc, C1 pad_seed == C2 pad_seed, relation_id = RELATION_ID. All residuals
    /// vanish (matches an on-domain leaf row); the OOD off-domain case is exercised separately.
    pub(crate) fn honest_cells(vals: [u32; C4_LANES], pad_seed_seed: u32) -> [Challenge; N_OPENED_CELLS] {
        let mut cells = [Challenge::ZERO; N_OPENED_CELLS];
        for lane in 0..C4_LANES {
            let lc = c4_lane_cells(vals[lane]);
            for w in 0..C4_CELLS {
                cells[c4_cell(lane, w)] = lc[w];
            }
            // carry_acc[lane] == v_lane (C4-acc binding).
            cells[acc_cell(lane)] = ext(vals[lane]);
            // SeamOpeningAcc: c6_field == carry_acc.
            cells[c6_field_cell(lane)] = ext(vals[lane]);
        }
        // SeamPadSeed: c1 == c2 (arbitrary byte values, equal).
        for b in 0..PAD_SEED_BYTES {
            let pb = ext(pad_seed_seed.wrapping_add(b as u32) % 256);
            cells[c1_pad_cell(b)] = pb;
            cells[c2_pad_cell(b)] = pb;
        }
        // relation_id == RELATION_ID.
        for j in 0..RELATION_ID_ELEMS {
            cells[relation_id_cell(j)] = ext(RELATION_ID[j]);
        }
        // C1 sponge: honest absorb/perm/squeeze over byte-valued input lanes (built from the real
        // byte-pinned Val sponge, lifted to the extension — the fidelity authority).
        fill_honest_c1(&mut cells, pad_seed_seed);
        // C6 sponge: honest absorb/perm/squeeze over the 6 FIELD lanes (already = vals[..], set
        // above) ‖ 80 byte-valued lanes (built from the real byte-pinned Val sponge).
        fill_honest_c6(&mut cells, pad_seed_seed);
        // role-root sponge: honest absorb/perm/squeeze over the CHAINED C1/C6 squeeze outputs (set
        // by the two fills above) ‖ 6 masked_projection lanes. MUST run after fill_honest_c1/c6.
        fill_honest_role(&mut cells, pad_seed_seed);
        // C2 mask-PRF sponge: honest absorb/perm/squeeze over pad_seed[32] (the c2_pad cells set
        // above) ‖ slot_id[8] (fresh byte-valued). Then the wide-then-reduce gadget over the 30
        // squeezed lanes -> 6 masks, then the carrier masked = (acc + mask) mod q (acc = vals[..],
        // in [0,q)). MUST run after the pad_seed cells (c2_pad) + acc cells are set.
        fill_honest_c2_and_reduce(&mut cells, pad_seed_seed, vals);
        // batch digest chain: honest per-row absorb of an enabled (enable=1) pubio over a chosen
        // prev_chain_out running state, then chain_out = absorbed, count_acc = prev_count_acc + 1.
        fill_honest_chain(&mut cells, pad_seed_seed, true);
        // PER-LANE BYTE-RANGE bits: split each ALREADY-SET absorbed sponge input lane (C1/C2/C6) into
        // its 8 little-endian bits. MUST run AFTER fill_honest_c1/c6/c2 (which set the input lanes).
        fill_honest_byterange(&mut cells);
        cells
    }

    /// Fill the byte-range bit-witness cells: for each byte-ranged sponge input lane (C1 48 ‖ C2 40 ‖
    /// C6 80), read the ALREADY-SET absorbed lane cell (a byte-valued base element) and write its 8
    /// little-endian bits. This is the byte-range fidelity AUTHORITY (honest decomposition of the
    /// genuine consumed lanes). PRECONDITION: the C1/C2/C6 sponge fills have run, so the absorbed
    /// input cells hold byte-valued base elements.
    fn fill_honest_byterange(cells: &mut [Challenge; N_OPENED_CELLS]) {
        use p3_field::PrimeField32;
        for g in 0..N_BR_LANES {
            let lane: Val =
                Challenge::as_basis_coefficients_slice(&cells[byterange_absorbed_cell(g)])[0];
            let v = lane.as_canonical_u32();
            for b in 0..BYTE_BITS {
                cells[byterange_bit_cell(g, b)] = Challenge::from(Val::from_u64(((v >> b) & 1) as u64));
            }
        }
    }

    /// Fill the batch-digest-chain cells (prev_chain_out[16] ‖ chain_in[16] ‖ CHAIN_NPERMS perm
    /// blocks ‖ chain_out[16] ‖ enable ‖ prev_count_acc ‖ count_acc ‖ pubio[PUBIO_ELEMS]) with an
    /// HONEST chain step over a chosen prev-row state + pubio, running the deployed Val permutation
    /// for each absorb perm, exactly mirroring `eval_digest_chain` / `chain_digest` / the
    /// `generate_batch_trace` fill, then lifting every cell into the extension. This is the chain
    /// fidelity AUTHORITY. We build an ENABLED (enable=1) real row (the soundness-relevant case;
    /// padding rows enable=0 are a TRUE no-op and exercised separately by a fidelity test).
    fn fill_honest_chain(cells: &mut [Challenge; N_OPENED_CELLS], seed: u32, enabled: bool) {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT, PERM_COLS};
        let consts = inlined_perm::Constants::pinned();

        let enable = if enabled { 1u64 } else { 0u64 };
        cells[chain_enable_cell()] = Challenge::from(Val::from_u64(enable));

        // prev-row running state (the cross-row carry source) + prev count. Field-valued.
        let mut prev_state = [Val::ZERO; CHAIN_W];
        for c in 0..CHAIN_W {
            let v = seed.wrapping_mul(1009).wrapping_add(c as u32 * 17 + 3);
            prev_state[c] = Val::from_u32(v);
            cells[chain_prev_out_cell(c)] = Challenge::from(prev_state[c]);
        }
        let prev_count = 5u64;
        cells[chain_prev_count_cell()] = Challenge::from(Val::from_u64(prev_count));
        cells[chain_count_cell()] = Challenge::from(Val::from_u64(prev_count + enable));

        // chain_in == prev_chain_out (the carry).
        for c in 0..CHAIN_W {
            cells[chain_in_cell(c)] = cells[chain_prev_out_cell(c)];
        }

        // pubio lanes: field-valued (the per-instance statement), lifted to one-limb base elements.
        // The COMMITMENT lanes (C1/C6/masked/role) are set EQUAL to the already-filled mirrored
        // sponge / gadget OUTPUT cells — so the sponge-output→public-value binding class
        // (`pubio_bind_residual`) vanishes on the honest row, exactly as the batch leaf AIR binds the
        // squeezed output to the per-row pubio block. The OTHER pubio lanes (context, relation_id,
        // syndrome, packed_witness) are arbitrary field values (the chain absorbs them regardless);
        // they are not bound by this class (their producing gadgets are not yet mirrored). MUST run
        // after fill_honest_c1/c6/role/c2_and_reduce.
        let mut pubio = [Val::ZERO; CHAIN_PUBIO];
        let out_val = |cells: &[Challenge; N_OPENED_CELLS], c: usize| -> Val {
            Challenge::as_basis_coefficients_slice(&cells[c])[0]
        };
        for i in 0..CHAIN_PUBIO {
            let v = seed.wrapping_mul(131).wrapping_add(i as u32 * 7 + 1);
            pubio[i] = Val::from_u32(v);
        }
        // C1 squeeze output → pubio[0..10].
        for j in 0..N_COMMIT_LANES {
            pubio[j] = out_val(cells, c1_out_cell(j));
        }
        // C6 squeeze output → pubio[N_COMMIT_LANES..2·N_COMMIT_LANES].
        for j in 0..N_COMMIT_LANES {
            pubio[N_COMMIT_LANES + j] = out_val(cells, c6_out_cell(j));
        }
        // carrier masked_projection → pubio[2·N_COMMIT_LANES..+N_LANES].
        for j in 0..N_LANES {
            pubio[2 * N_COMMIT_LANES + j] = out_val(cells, carrier_masked_cell(j));
        }
        // role-root squeeze output → pubio[PUBIO_ROLE_ROOT_OFF..+ROLE_ROOT_ELEMS].
        for j in 0..ROLE_ROOT_ELEMS {
            pubio[PUBIO_ROLE_ROOT_OFF + j] = out_val(cells, role_out_cell(j));
        }
        for i in 0..CHAIN_PUBIO {
            cells[chain_pubio_cell(i)] = Challenge::from(pubio[i]);
        }

        // message: m[0]=enable, m[1+i]=enable*pubio[i], zero-padded to CHAIN_PADDED_LEN. For a
        // padding row (enable=0) the message is all-zero — a TRUE no-op absorb, as in air.rs.
        let enable_v = Val::from_u64(enable);
        let msg = |pos: usize| -> Val {
            if pos == 0 {
                enable_v
            } else if pos <= CHAIN_PUBIO {
                enable_v * pubio[pos - 1]
            } else {
                Val::ZERO
            }
        };

        // absorb across CHAIN_NPERMS perms seeded by chain_in (= prev_state).
        let mut prev_out = prev_state;
        let mut absorbed = prev_state;
        for p in 0..CHAIN_NPERMS {
            let mut perm_in = [Val::ZERO; WIDTH];
            if p == 0 {
                for r in 0..CHAIN_RT {
                    perm_in[r] = prev_state[r] + msg(r);
                }
                for c in CHAIN_RT..WIDTH {
                    perm_in[c] = prev_state[c];
                }
            } else {
                for r in 0..CHAIN_RT {
                    perm_in[r] = prev_out[r] + msg(p * CHAIN_RT + r);
                }
                for c in CHAIN_RT..WIDTH {
                    perm_in[c] = prev_out[c];
                }
            }
            let block = generate_perm_cols(perm_in, &consts);
            let base = chain_perm_base(p);
            for c in 0..PERM_COLS {
                cells[base + c] = Challenge::from(block[c]);
            }
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            prev_out = out;
            absorbed = out;
        }

        // chain_out = enable ? absorbed : chain_in (the no-op fold for padding rows).
        for c in 0..CHAIN_W {
            let out = if enabled { absorbed[c] } else { prev_state[c] };
            cells[chain_out_cell(c)] = Challenge::from(out);
        }
    }

    /// Fill the C1 sponge cells (input lanes ‖ 8 perm blocks ‖ pad_source_commitment) with an
    /// HONEST sponge over byte-valued input lanes derived from `seed`. Runs the deployed Val
    /// permutation (`inlined_perm::generate_perm_cols`, byte-pinned to default_koalabear_poseidon2_16)
    /// for each absorb/squeeze perm, exactly mirroring the leaf `eval_sponge` absorb/capacity wiring,
    /// then lifts every cell into the extension. This is the C1 fidelity AUTHORITY.
    fn fill_honest_c1(cells: &mut [Challenge; N_OPENED_CELLS], seed: u32) {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT, PERM_COLS};
        let consts = inlined_perm::Constants::pinned();

        // input lanes: byte-valued (0..255), as the leaf's byte-ranged C1 lanes.
        let mut input_lane = [Val::ZERO; C1_INPUT_LANES];
        for i in 0..C1_INPUT_LANES {
            let b = (seed.wrapping_mul(31).wrapping_add(i as u32)) % 256;
            input_lane[i] = Val::from_u32(b);
            cells[c1_input_cell(i)] = Challenge::from(input_lane[i]);
        }

        // absorbed stream (padded): lanes, ONE marker at n_input, zero-fill.
        let absorb = |pos: usize| -> Val {
            if pos < C1_INPUT_LANES {
                input_lane[pos]
            } else if pos == C1_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };

        let mut prev_out: Option<[Val; WIDTH]> = None;
        for p in 0..C1_N_PERMS {
            let is_absorb = p < C1_N_ABSORB;
            let mut perm_in = [Val::ZERO; WIDTH];
            match prev_out {
                None => {
                    for r in 0..C1_RATE {
                        perm_in[r] = absorb(r);
                    }
                    // capacity stays zero.
                }
                Some(prev) => {
                    for r in 0..C1_RATE {
                        if is_absorb {
                            let pos = p * C1_RATE + r;
                            let elem = if pos < C1_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                            perm_in[r] = prev[r] + elem;
                        } else {
                            perm_in[r] = prev[r];
                        }
                    }
                    for c in C1_RATE..WIDTH {
                        perm_in[c] = prev[c];
                    }
                }
            }
            let block = generate_perm_cols(perm_in, &consts);
            let base = c1_perm_base(p);
            for c in 0..PERM_COLS {
                cells[base + c] = Challenge::from(block[c]);
            }
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            prev_out = Some(out);
        }

        // squeeze: pad_source_commitment[j] = source-perm output lane.
        for j in 0..C1_N_OUT {
            let (sp, lane) = c1_output_source(j);
            let out_cell = cells[c1_perm_base(sp) + IP_OUTPUT + lane];
            cells[c1_out_cell(j)] = out_cell;
        }
    }

    /// Fill the C6 sponge cells (byte-lanes ‖ 12 perm blocks ‖ opening_tuple_commitment) with an
    /// HONEST sponge over: the 6 FIELD lanes already set in `cells[c6_field_cell(..)]` (the
    /// account_projection lanes, NOT byte-ranged) ‖ 80 byte-valued lanes derived from `seed`. Runs
    /// the deployed Val permutation for each absorb/squeeze perm, exactly mirroring the leaf
    /// `eval_sponge` absorb/capacity wiring, then lifts every cell into the extension. This is the
    /// C6 fidelity AUTHORITY. PRECONDITION: the 6 FIELD lanes must already be base-field elements
    /// (one limb) so they round-trip to `Val`; `honest_cells` sets them to `ext(vals[..])`.
    fn fill_honest_c6(cells: &mut [Challenge; N_OPENED_CELLS], seed: u32) {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT, PERM_COLS};
        let consts = inlined_perm::Constants::pinned();

        // input lanes (as Val): FIELD lanes from the existing C6 field cells, byte-lanes fresh.
        let mut input_lane = [Val::ZERO; C6_INPUT_LANES];
        for i in 0..C6_FIELD_LANES {
            input_lane[i] = Challenge::as_basis_coefficients_slice(&cells[c6_field_cell(i)])[0];
        }
        for b in 0..C6_BYTE_LANES {
            let v = (seed.wrapping_mul(53).wrapping_add(b as u32 + 1)) % 256;
            input_lane[C6_FIELD_LANES + b] = Val::from_u32(v);
            cells[c6_byte_cell(b)] = Challenge::from(input_lane[C6_FIELD_LANES + b]);
        }

        let absorb = |pos: usize| -> Val {
            if pos < C6_INPUT_LANES {
                input_lane[pos]
            } else if pos == C6_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };

        let mut prev_out: Option<[Val; WIDTH]> = None;
        for p in 0..C6_N_PERMS {
            let is_absorb = p < C6_N_ABSORB;
            let mut perm_in = [Val::ZERO; WIDTH];
            match prev_out {
                None => {
                    for r in 0..C6_RATE {
                        perm_in[r] = absorb(r);
                    }
                }
                Some(prev) => {
                    for r in 0..C6_RATE {
                        if is_absorb {
                            let pos = p * C6_RATE + r;
                            let elem = if pos < C6_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                            perm_in[r] = prev[r] + elem;
                        } else {
                            perm_in[r] = prev[r];
                        }
                    }
                    for c in C6_RATE..WIDTH {
                        perm_in[c] = prev[c];
                    }
                }
            }
            let block = generate_perm_cols(perm_in, &consts);
            let base = c6_perm_base(p);
            for c in 0..PERM_COLS {
                cells[base + c] = Challenge::from(block[c]);
            }
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            prev_out = Some(out);
        }

        // squeeze: opening_tuple_commitment[j] = source-perm output lane.
        for j in 0..C6_N_OUT {
            let (sp, lane) = c6_output_source(j);
            let out_cell = cells[c6_perm_base(sp) + IP_OUTPUT + lane];
            cells[c6_out_cell(j)] = out_cell;
        }
    }

    /// Fill the role-root sponge cells (masked[6] ‖ 4 perm blocks ‖ role_root[8]) with an HONEST
    /// sponge over the CHAINED input lanes: pad_source_commitment[10] (= the C1 squeeze output cells
    /// `c1_out_cell`) ‖ opening_tuple_commitment[10] (= the C6 squeeze output cells `c6_out_cell`) ‖
    /// masked_projection[6] (fresh field-valued lanes). PRECONDITION: `fill_honest_c1`/`fill_honest_c6`
    /// must have run first so the chained output cells are set. The role inputs are FIELD elements
    /// (no byte-range), so they round-trip to `Val` (the C1/C6 squeeze outputs are deployed-perm
    /// outputs, single-limb base elements). This is the role-root fidelity AUTHORITY.
    fn fill_honest_role(cells: &mut [Challenge; N_OPENED_CELLS], seed: u32) {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT, PERM_COLS};
        let consts = inlined_perm::Constants::pinned();

        // masked_projection lanes: fresh field-valued (one limb), as the leaf's masked carrier lanes.
        let mut masked = [Val::ZERO; ROLE_MASKED_LANES];
        for i in 0..ROLE_MASKED_LANES {
            let v = seed.wrapping_mul(97).wrapping_add(i as u32 * 13 + 1);
            masked[i] = Val::from_u32(v);
            cells[role_masked_cell(i)] = Challenge::from(masked[i]);
        }

        // input lanes (as Val): CHAINED C1 out ‖ C6 out ‖ masked — snapshot now (the chained C1/C6
        // outputs are already set by fill_honest_c1/c6) so the absorb closure does not borrow `cells`
        // across the mutable perm-cell writes below.
        let mut input_lane = [Val::ZERO; ROLE_INPUT_LANES];
        for pos in 0..ROLE_INPUT_LANES {
            input_lane[pos] = match role_absorb_source(pos) {
                C1Absorb::Cell(c) => Challenge::as_basis_coefficients_slice(&cells[c])[0],
                _ => unreachable!("role input lanes 0..26 are all cells"),
            };
        }
        let absorb = |pos: usize| -> Val {
            if pos < ROLE_INPUT_LANES {
                input_lane[pos]
            } else if pos == ROLE_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };

        let mut prev_out: Option<[Val; WIDTH]> = None;
        for p in 0..ROLE_N_PERMS {
            let is_absorb = p < ROLE_N_ABSORB;
            let mut perm_in = [Val::ZERO; WIDTH];
            match prev_out {
                None => {
                    for r in 0..ROLE_RATE {
                        perm_in[r] = absorb(r);
                    }
                }
                Some(prev) => {
                    for r in 0..ROLE_RATE {
                        if is_absorb {
                            let pos = p * ROLE_RATE + r;
                            let elem = if pos < ROLE_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                            perm_in[r] = prev[r] + elem;
                        } else {
                            perm_in[r] = prev[r];
                        }
                    }
                    for c in ROLE_RATE..WIDTH {
                        perm_in[c] = prev[c];
                    }
                }
            }
            let block = generate_perm_cols(perm_in, &consts);
            let base = role_perm_base(p);
            for c in 0..PERM_COLS {
                cells[base + c] = Challenge::from(block[c]);
            }
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            prev_out = Some(out);
        }

        // squeeze: role_root[j] = source-perm output lane.
        for j in 0..ROLE_N_OUT {
            let (sp, lane) = role_output_source(j);
            let out_cell = cells[role_perm_base(sp) + IP_OUTPUT + lane];
            cells[role_out_cell(j)] = out_cell;
        }
    }

    /// Fill the C2 mask-PRF sponge cells (slot_id[8] ‖ 9 perm blocks) with an HONEST sponge over the
    /// pad_seed[32] cells (the `c2_pad` cells, already set) ‖ 8 fresh byte-valued slot_id lanes, then
    /// the WIDE-THEN-REDUCE gadget (the 30 squeezed lanes -> 6 masks via the leaf's `fill_reduce_gadget`)
    /// + the carrier mod-q reduction (masked = (acc + mask) mod q12289). Runs the deployed Val
    /// permutation for each absorb/squeeze perm, then lifts every cell into the extension. This is the
    /// C2-sponge + reduce-gadget + carrier fidelity AUTHORITY. PRECONDITION: the `c2_pad` cells and the
    /// `acc_cell` cells (= `vals[..]`, in [0,q)) must already be set; `honest_cells` sets them.
    fn fill_honest_c2_and_reduce(
        cells: &mut [Challenge; N_OPENED_CELLS],
        seed: u32,
        vals: [u32; C4_LANES],
    ) {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT, PERM_COLS};
        use descender_account_stark::reduce_gadget::{fill_reduce_gadget, NUM_SQUEEZE as RG_NSQ};
        use p3_field::PrimeField32;
        let consts = inlined_perm::Constants::pinned();

        // input lanes (as Val): pad_seed[32] from the existing c2_pad cells, slot_id[8] fresh bytes.
        let mut input_lane = [Val::ZERO; C2_INPUT_LANES];
        for b in 0..C2_PAD_LANES {
            input_lane[b] = Challenge::as_basis_coefficients_slice(&cells[c2_pad_cell(b)])[0];
        }
        for i in 0..C2_SLOT_LANES {
            let v = (seed.wrapping_mul(71).wrapping_add(i as u32 + 1)) % 256;
            input_lane[C2_PAD_LANES + i] = Val::from_u32(v);
            cells[c2_slot_cell(i)] = Challenge::from(input_lane[C2_PAD_LANES + i]);
        }

        let absorb = |pos: usize| -> Val {
            if pos < C2_INPUT_LANES {
                input_lane[pos]
            } else if pos == C2_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };

        let mut prev_out: Option<[Val; WIDTH]> = None;
        let mut perm_outs: Vec<[Val; WIDTH]> = Vec::new();
        for p in 0..C2_N_PERMS {
            let is_absorb = p < C2_N_ABSORB;
            let mut perm_in = [Val::ZERO; WIDTH];
            match prev_out {
                None => {
                    for r in 0..C2_RATE {
                        perm_in[r] = absorb(r);
                    }
                }
                Some(prev) => {
                    for r in 0..C2_RATE {
                        if is_absorb {
                            let pos = p * C2_RATE + r;
                            let elem = if pos < C2_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                            perm_in[r] = prev[r] + elem;
                        } else {
                            perm_in[r] = prev[r];
                        }
                    }
                    for c in C2_RATE..WIDTH {
                        perm_in[c] = prev[c];
                    }
                }
            }
            let block = generate_perm_cols(perm_in, &consts);
            let base = c2_perm_base(p);
            for c in 0..PERM_COLS {
                cells[base + c] = Challenge::from(block[c]);
            }
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            perm_outs.push(out);
            prev_out = Some(out);
        }

        // 30 squeezed lanes (as Val) from the squeeze source perms (= what the reduce gadget reads).
        let mut lanes = [0u32; RG_NSQ];
        for o in 0..C2_N_OUT {
            let (sp, lane) = c2_output_source(o);
            lanes[o] = perm_outs[sp][lane].as_canonical_u32();
        }

        // run the LEAF's authoritative reduce-gadget fill into a scratch Val row, then lift the
        // gadget columns into the opened reduce cells (same StepOff layout, gadget-relative).
        let mut scratch = vec![Val::ZERO; REDUCE_CELLS];
        let masks = fill_reduce_gadget::<Val>(&mut scratch, 0, &lanes);
        for i in 0..REDUCE_CELLS {
            cells[reduce_cell(i)] = Challenge::from(scratch[i]);
        }

        // carrier: masked = (acc + mask) mod q12289, with the mod-q witness block per lane.
        let q = Q12289 as u64;
        for j in 0..N_LANES {
            let acc = vals[j] as u64;
            let mask = masks[j] as u64;
            let sum = acc + mask;
            let wrap = if sum >= q { 1u64 } else { 0u64 };
            let masked = sum - wrap * q;
            let slack = (q - 1) - masked;

            cells[carrier_masked_cell(j)] = Challenge::from(Val::from_u64(masked));
            let mb = carrier_modq_base(j);
            let masked_bits_off = mb + 1;
            let slack_off = mb + 1 + MODQ_BITS;
            let slack_bits_off = slack_off + 1;
            let wrap_off = slack_bits_off + MODQ_BITS;
            for b in 0..MODQ_BITS {
                cells[masked_bits_off + b] = Challenge::from(Val::from_u64((masked >> b) & 1));
                cells[slack_bits_off + b] = Challenge::from(Val::from_u64((slack >> b) & 1));
            }
            cells[slack_off] = Challenge::from(Val::from_u64(slack));
            cells[wrap_off] = Challenge::from(Val::from_u64(wrap));
        }
    }

    /// Arbitrary OFF-domain opened cells (every cell a multi-limb extension element). The mirror
    /// recomputes C_partial != 0 and binds it — the DEEP-relevant shape.
    fn offdomain_cells(seed: u32) -> [Challenge; N_OPENED_CELLS] {
        core::array::from_fn(|c| ext_multi(seed.wrapping_mul(31).wrapping_add(c as u32)))
    }

    fn matrix(cells: &[Challenge; N_OPENED_CELLS], alpha: Challenge) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(pack_row(cells, alpha), n_cols())
    }

    /// On a fully-honest (on-domain) leaf row ALL mirrored leaf constraints hold ⇒ every residual
    /// is ZERO and honest `C_partial` is ZERO.
    #[test]
    fn honest_on_domain_has_zero_partial() {
        let cells = honest_cells([0, 1, 127, 128, 12288, 5000], 17);
        for r in 0..N_CONSTRAINTS {
            assert_eq!(
                constraint_residual(&cells, r),
                Challenge::ZERO,
                "honest on-domain row violates mirrored residual {r}"
            );
        }
        let cp = partial_constraint_eval(&cells, ext_multi(9));
        assert_eq!(cp, Challenge::ZERO, "honest on-domain row should give zero residual sum");
    }

    /// GOLDEN (positive): honest mirror rows ACCEPT via the real `check_all_constraints` — both
    /// on-domain honest cells (C_partial = 0) and OFF-domain arbitrary extension cells
    /// (C_partial != 0, the DEEP-relevant case).
    #[test]
    fn air_accepts_honest_rows() {
        // on-domain honest
        let cells = honest_cells([0, 127, 12288, 9999, 4096, 1], 42);
        let m = matrix(&cells, ext(31));
        assert!(
            check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
            "honest on-domain row rejected"
        );
        // off-domain arbitrary extension cells
        for seed in [1u32, 7, 42, 100] {
            let cells = offdomain_cells(seed);
            let m = matrix(&cells, ext_multi(seed + 1000));
            assert!(
                check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "honest off-domain row seed={seed} rejected"
            );
        }
    }

    /// FORGERY — a forged `C_partial` (≠ the recomputed alpha-batched residual) is rejected.
    #[test]
    fn air_rejects_forged_c_partial() {
        let cells = offdomain_cells(3);
        let mut m = matrix(&cells, ext_multi(77));
        m.values[L_CPARTIAL + 2] += Val::ONE; // tamper a C_partial limb
        assert!(!check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok());
    }

    /// FORGERY — tampering ANY opened cell that feeds SOME mirrored leaf constraint changes the
    /// recomputed `C_partial` ⇒ rejected. We tamper one representative cell of EACH mirrored class,
    /// across lanes, and require rejection. This is the soundness link: a trace cell that violates
    /// a mirrored leaf constraint is caught at the OOD point.
    #[test]
    fn air_rejects_tampered_opened_cell() {
        let cells = offdomain_cells(50);
        let alpha = ext_multi(123);
        let mut targets: Vec<usize> = Vec::new();
        // C4: a class of cell per lane (v, d_lo, d_hi, a bit, is96, inv).
        for lane in 0..C4_LANES {
            targets.push(c4_cell(lane, I_V));
            targets.push(c4_cell(lane, I_DLO));
            targets.push(c4_cell(lane, I_DHI));
            targets.push(c4_cell(lane, I_LO_BITS + (lane % 7)));
            targets.push(c4_cell(lane, I_IS96));
            targets.push(c4_cell(lane, I_INV));
        }
        // acc / seam / relation_id cells.
        for j in 0..N_LANES {
            targets.push(acc_cell(j));
            targets.push(c6_field_cell(j));
        }
        for b in 0..PAD_SEED_BYTES {
            targets.push(c1_pad_cell(b));
            targets.push(c2_pad_cell(b));
        }
        for j in 0..RELATION_ID_ELEMS {
            targets.push(relation_id_cell(j));
        }
        for &cell_idx in &targets {
            let mut m = matrix(&cells, alpha);
            m.values[cell_base(cell_idx)] += Val::ONE; // tamper limb 0 of the cell
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampering opened cell {cell_idx} was NOT rejected — that residual is underconstrained"
            );
        }
    }

    /// FORGERY — tampering `alpha` (the batching challenge) with `C_partial` honest ⇒ rejected.
    #[test]
    fn air_rejects_tampered_alpha() {
        let cells = offdomain_cells(200);
        let mut m = matrix(&cells, ext_multi(9));
        m.values[L_ALPHA + 1] += Val::ONE; // tamper an alpha limb (C_partial unchanged)
        assert!(!check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok());
    }

    /// FIDELITY PIN (C4) — the per-constraint C4 residuals from THIS module's native reference
    /// MUST match the leaf AIR's own `eval_c4_residue` semantics (air.rs:1381): on a canonical
    /// lane all 29 residuals are zero, and single deliberate corruptions make EXACTLY the expected
    /// residual nonzero.
    #[test]
    fn mirror_matches_leaf_air_c4() {
        let cells = honest_cells([7000, 0, 12288, 128, 5000, 96], 1);
        // canonical ⇒ all C4 residuals zero (per lane).
        for lane in 0..C4_LANES {
            for i in 0..C4_N_PER_LANE {
                assert_eq!(
                    c4_lane_residual(&cells, lane, i),
                    Challenge::ZERO,
                    "canonical lane {lane} violates mirrored C4 residual {i}"
                );
            }
        }
        // Corrupt v on lane 2 ⇒ v-recompose residual (28) nonzero; lo-recompose (21) still zero.
        let mut bad = cells;
        bad[c4_cell(2, I_V)] += Challenge::ONE;
        assert_ne!(c4_lane_residual(&bad, 2, 28), Challenge::ZERO, "v-recompose blind to v");
        assert_eq!(c4_lane_residual(&bad, 2, 21), Challenge::ZERO, "lo-recompose should not read v");
        // and OTHER lanes unaffected.
        assert_eq!(c4_lane_residual(&bad, 0, 28), Challenge::ZERO, "lane bleed");

        // is96 inverse-corner (air.rs:1416): lane with d_hi=96 (v=12288), flip is96 -> residual 26.
        let mut corner = cells;
        // lane 2 has v=12288 ⇒ d_hi=96, is96=1.
        corner[c4_cell(2, I_IS96)] = Challenge::ZERO;
        assert_ne!(
            c4_lane_residual(&corner, 2, 26),
            Challenge::ZERO,
            "is96 inverse-corner residual blind to is96 flip — soundness hole vs air.rs:1416"
        );
    }

    /// FIDELITY PIN (seams + relation_id) — the algebraic residuals match air.rs:
    ///   * SeamPadSeed (765-771): zero iff c1_pad == c2_pad; nonzero on a single mismatch.
    ///   * SeamOpeningAcc (773-779): zero iff carry_acc == c6_field; nonzero on a single mismatch.
    ///   * C4-acc (762): zero iff carry_acc == v; nonzero on a single mismatch.
    ///   * relation_id (957-967): zero iff relation_id == RELATION_ID; nonzero on any deviation.
    #[test]
    fn mirror_matches_leaf_air_seams_and_relation_id() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 99);
        let c4_total = C4_LANES * C4_N_PER_LANE;
        // honest: all non-C4 residuals zero.
        for r in c4_total..N_CONSTRAINTS {
            assert_eq!(constraint_residual(&cells, r), Challenge::ZERO, "honest residual {r} nonzero");
        }
        // C4-acc: corrupt carry_acc[3] ⇒ both the acc-binding residual AND SeamOpeningAcc[3] react.
        let mut bad_acc = cells;
        bad_acc[acc_cell(3)] += Challenge::ONE;
        assert_ne!(
            constraint_residual(&bad_acc, c4_total + 3),
            Challenge::ZERO,
            "C4-acc binding (air.rs:762) blind to carry_acc"
        );
        // SeamPadSeed: corrupt c2_pad_seed[10] ⇒ residual c4_total+N_C4_ACC+10 nonzero.
        let mut bad_pad = cells;
        bad_pad[c2_pad_cell(10)] += Challenge::ONE;
        assert_ne!(
            constraint_residual(&bad_pad, c4_total + N_C4_ACC + 10),
            Challenge::ZERO,
            "SeamPadSeed (air.rs:765) blind to c2 pad_seed"
        );
        // SeamOpeningAcc: corrupt c6_field[4] ⇒ residual c4_total+N_C4_ACC+N_SEAM_PAD+4 nonzero.
        let mut bad_c6 = cells;
        bad_c6[c6_field_cell(4)] += Challenge::ONE;
        assert_ne!(
            constraint_residual(&bad_c6, c4_total + N_C4_ACC + N_SEAM_PAD + 4),
            Challenge::ZERO,
            "SeamOpeningAcc (air.rs:773) blind to c6 field lane"
        );
        // relation_id: corrupt relation_id[1] ⇒ residual at the relation-id base nonzero.
        let mut bad_rid = cells;
        bad_rid[relation_id_cell(1)] += Challenge::ONE;
        let rid_base = c4_total + N_C4_ACC + N_SEAM_PAD + N_SEAM_ACC;
        assert_ne!(
            constraint_residual(&bad_rid, rid_base + 1),
            Challenge::ZERO,
            "relation_id pinning (air.rs:957) blind to relation_id"
        );
    }

    /// The AIR's in-circuit recomputation agrees with the native `partial_constraint_eval`.
    #[test]
    fn air_agrees_with_native_partial_eval() {
        for seed in [11u32, 22, 33] {
            let cells = offdomain_cells(seed);
            let alpha = ext_multi(seed.wrapping_add(500));
            let m = matrix(&cells, alpha);
            assert!(
                check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "AIR recomputation disagrees with native partial eval (seed={seed})"
            );
            assert_eq!(row_c_partial(m.values.as_slice()), partial_constraint_eval(&cells, alpha));
        }
    }

    /// A multi-row trace of independent honest mirror checks (one per row) accepts.
    #[test]
    fn air_accepts_multirow() {
        let mut flat = Vec::new();
        for r in 0..6u32 {
            let cells = offdomain_cells(900 + r * 31);
            flat.extend_from_slice(&pack_row(&cells, ext_multi(r + 1)));
        }
        let m = RowMajorMatrix::new(flat, n_cols());
        assert!(check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok());
    }

    // =======================================================================
    // C1 SPONGE (pad_source_commitment squeeze) — the FIRST mirrored leaf sponge.
    // =======================================================================

    /// Base residual index of the C1 sponge class within `constraint_residual`.
    const C1_RESIDUAL_BASE: usize =
        C4_LANES * C4_N_PER_LANE + N_C4_ACC + N_SEAM_PAD + N_SEAM_ACC + N_RELATION_ID;

    /// FIDELITY PIN (C1) — an HONEST C1 sponge (built from the deployed byte-pinned Val permutation
    /// via `fill_honest_c1`, lifted to the extension) makes EVERY C1 residual zero: the absorb /
    /// capacity wiring, all 8 Poseidon2 perm blocks, and the squeeze-output equality. A wrong octic
    /// perm / absorb wiring / output mapping would make some honest residual nonzero — caught here
    /// by attempted equality, not inspection.
    #[test]
    fn c1_honest_sponge_all_residuals_zero() {
        for seed in [1u32, 7, 42, 255] {
            let cells = honest_cells([0, 1, 127, 128, 12288, 5000], seed);
            for k in 0..N_C1 {
                assert_eq!(
                    constraint_residual(&cells, C1_RESIDUAL_BASE + k),
                    Challenge::ZERO,
                    "honest C1 residual {k} nonzero (seed={seed})"
                );
            }
        }
    }

    /// FIDELITY PIN (C1) — the squeezed pad_source_commitment cells equal the source-perm OUTPUT
    /// cells (the leaf's `pcols.post[lane]`), AND those outputs are the deployed permutation applied
    /// to the absorbed stream. We re-derive the last absorb perm's output independently from the
    /// real Val perm and check the committed digest matches.
    #[test]
    fn c1_squeeze_matches_deployed_perm_output() {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT};
        let cells = honest_cells([7000, 0, 12288, 128, 5000, 96], 9);
        // independently recompute the source perms' inputs/outputs from the absorbed stream.
        let consts = inlined_perm::Constants::pinned();
        // input lanes as Val.
        let mut input_lane = [Val::ZERO; C1_INPUT_LANES];
        for i in 0..C1_INPUT_LANES {
            input_lane[i] =
                Challenge::as_basis_coefficients_slice(&cells[c1_input_cell(i)])[0];
        }
        let absorb = |pos: usize| -> Val {
            if pos < C1_INPUT_LANES {
                input_lane[pos]
            } else if pos == C1_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };
        let mut prev: Option<[Val; WIDTH]> = None;
        let mut outs: Vec<[Val; WIDTH]> = Vec::new();
        for p in 0..C1_N_PERMS {
            let is_absorb = p < C1_N_ABSORB;
            let mut pin = [Val::ZERO; WIDTH];
            match prev {
                None => {
                    for r in 0..C1_RATE {
                        pin[r] = absorb(r);
                    }
                }
                Some(pv) => {
                    for r in 0..C1_RATE {
                        if is_absorb {
                            let pos = p * C1_RATE + r;
                            pin[r] = pv[r] + if pos < C1_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                        } else {
                            pin[r] = pv[r];
                        }
                    }
                    for c in C1_RATE..WIDTH {
                        pin[c] = pv[c];
                    }
                }
            }
            let block = generate_perm_cols(pin, &consts);
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            outs.push(out);
            prev = Some(out);
        }
        for j in 0..C1_N_OUT {
            let (sp, lane) = c1_output_source(j);
            let committed = cells[c1_out_cell(j)];
            assert_eq!(
                committed,
                Challenge::from(outs[sp][lane]),
                "C1 squeeze lane {j} != deployed perm output (perm {sp} lane {lane})"
            );
        }
    }

    /// FORGERY (C1) — a forged pad_source_commitment digest cell ⇒ the squeeze-output equality
    /// residual is nonzero ⇒ the AIR rejects.
    #[test]
    fn c1_air_rejects_forged_pad_source_commitment() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        for j in 0..C1_N_OUT {
            let mut m = matrix(&cells, ext_multi(13));
            m.values[cell_base(c1_out_cell(j))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged pad_source_commitment lane {j} NOT rejected"
            );
        }
    }

    /// FORGERY (C1) — a tampered absorbed input lane (e.g. a pad_seed byte) propagates into the
    /// first absorb perm's input binding ⇒ the input-binding residual + downstream perm residuals
    /// shift ⇒ the AIR rejects.
    #[test]
    fn c1_air_rejects_tampered_absorbed_input_lane() {
        let cells = honest_cells([10, 20, 30, 40, 50, 60], 3);
        // tamper a few representative input lanes (a pad_seed byte and a nonce byte).
        for &i in &[0usize, 5, 31, 47] {
            let mut m = matrix(&cells, ext_multi(21));
            m.values[cell_base(c1_input_cell(i))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered C1 absorbed input lane {i} NOT rejected"
            );
        }
    }

    /// FORGERY (C1) — a tampered INTERIOR perm cell (a committed round post / a perm input) of some
    /// C1 perm block ⇒ the inlined octic Poseidon2 round residuals reject. Confirms the perm
    /// sub-blocks are fully constrained, not just the squeeze output.
    #[test]
    fn c1_air_rejects_tampered_interior_perm_cell() {
        use crate::inlined_perm::{C_BEGIN_POST, C_INPUTS, C_PARTIAL_POST_SBOX};
        let cells = honest_cells([100, 200, 300, 400, 500, 600], 5);
        // representative interior cells across perm blocks: an input lane, a begin-full post, a
        // partial post_sbox, and the OUTPUT lane (which the squeeze reads).
        let targets = [
            c1_perm_base(0) + C_INPUTS + 3,
            c1_perm_base(2) + C_BEGIN_POST + 7,
            c1_perm_base(6) + C_PARTIAL_POST_SBOX + 10,
            c1_perm_base(C1_N_ABSORB - 1) + C_OUTPUT + 1, // last-absorb output lane (squeeze source)
            c1_perm_base(C1_N_ABSORB) + C_OUTPUT + 0,     // squeeze perm output lane
        ];
        for &cell_idx in &targets {
            let mut m = matrix(&cells, ext_multi(33));
            m.values[cell_base(cell_idx)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered C1 interior perm cell {cell_idx} NOT rejected"
            );
        }
    }

    // =======================================================================
    // C6 SPONGE (opening_tuple_commitment squeeze) — the SECOND mirrored leaf sponge.
    // =======================================================================

    /// Base residual index of the C6 sponge class within `constraint_residual`.
    const C6_RESIDUAL_BASE: usize =
        C4_LANES * C4_N_PER_LANE + N_C4_ACC + N_SEAM_PAD + N_SEAM_ACC + N_RELATION_ID + N_C1;

    /// FIDELITY PIN (C6) — an HONEST C6 sponge (built from the deployed byte-pinned Val permutation
    /// via `fill_honest_c6`, over 6 FIELD lanes ‖ 80 byte-lanes, lifted to the extension) makes EVERY
    /// C6 residual zero: the absorb / capacity wiring across all 12 perm blocks and the squeeze-output
    /// equality. A wrong octic perm / absorb wiring / output mapping would make some honest residual
    /// nonzero — caught here by attempted equality, not inspection.
    #[test]
    fn c6_honest_sponge_all_residuals_zero() {
        for seed in [1u32, 7, 42, 255] {
            let cells = honest_cells([0, 1, 127, 128, 12288, 5000], seed);
            for k in 0..N_C6 {
                assert_eq!(
                    constraint_residual(&cells, C6_RESIDUAL_BASE + k),
                    Challenge::ZERO,
                    "honest C6 residual {k} nonzero (seed={seed})"
                );
            }
        }
    }

    /// FIDELITY PIN (C6) — the squeezed opening_tuple_commitment cells equal the source-perm OUTPUT
    /// cells (the leaf's `pcols.post[lane]`, air.rs:1355), AND those outputs are the deployed
    /// permutation applied to the absorbed stream (6 FIELD lanes ‖ 80 byte-lanes ‖ ONE-pad ‖
    /// zero-fill). We re-derive the source perms' outputs independently from the real Val perm and
    /// check the committed digest matches — pins the C6 sponge against the leaf AIR's own sponge.
    #[test]
    fn c6_squeeze_matches_deployed_perm_output() {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT};
        let cells = honest_cells([7000, 0, 12288, 128, 5000, 96], 9);
        let consts = inlined_perm::Constants::pinned();
        // input lanes as Val: 6 FIELD lanes ‖ 80 byte-lanes.
        let mut input_lane = [Val::ZERO; C6_INPUT_LANES];
        for i in 0..C6_FIELD_LANES {
            input_lane[i] = Challenge::as_basis_coefficients_slice(&cells[c6_field_cell(i)])[0];
        }
        for b in 0..C6_BYTE_LANES {
            input_lane[C6_FIELD_LANES + b] =
                Challenge::as_basis_coefficients_slice(&cells[c6_byte_cell(b)])[0];
        }
        let absorb = |pos: usize| -> Val {
            if pos < C6_INPUT_LANES {
                input_lane[pos]
            } else if pos == C6_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };
        let mut prev: Option<[Val; WIDTH]> = None;
        let mut outs: Vec<[Val; WIDTH]> = Vec::new();
        for p in 0..C6_N_PERMS {
            let is_absorb = p < C6_N_ABSORB;
            let mut pin = [Val::ZERO; WIDTH];
            match prev {
                None => {
                    for r in 0..C6_RATE {
                        pin[r] = absorb(r);
                    }
                }
                Some(pv) => {
                    for r in 0..C6_RATE {
                        if is_absorb {
                            let pos = p * C6_RATE + r;
                            pin[r] = pv[r] + if pos < C6_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                        } else {
                            pin[r] = pv[r];
                        }
                    }
                    for c in C6_RATE..WIDTH {
                        pin[c] = pv[c];
                    }
                }
            }
            let block = generate_perm_cols(pin, &consts);
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            outs.push(out);
            prev = Some(out);
        }
        for j in 0..C6_N_OUT {
            let (sp, lane) = c6_output_source(j);
            let committed = cells[c6_out_cell(j)];
            assert_eq!(
                committed,
                Challenge::from(outs[sp][lane]),
                "C6 squeeze lane {j} != deployed perm output (perm {sp} lane {lane})"
            );
        }
    }

    /// FORGERY (C6) — a forged opening_tuple_commitment digest cell ⇒ the squeeze-output equality
    /// residual is nonzero ⇒ the AIR rejects.
    #[test]
    fn c6_air_rejects_forged_opening_tuple_commitment() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        for j in 0..C6_N_OUT {
            let mut m = matrix(&cells, ext_multi(13));
            m.values[cell_base(c6_out_cell(j))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged opening_tuple_commitment lane {j} NOT rejected"
            );
        }
    }

    /// FORGERY (C6) — a tampered absorbed lane (a FIELD account_projection lane OR a byte-lane such
    /// as a lifecycle / account_id byte) propagates into the absorb-perm input bindings ⇒ the
    /// input-binding residual + downstream perm residuals shift ⇒ the AIR rejects. Note the FIELD
    /// lane is ALSO bound by SeamOpeningAcc, so it rejects via two classes — both are soundness.
    #[test]
    fn c6_air_rejects_tampered_absorbed_lane() {
        let cells = honest_cells([10, 20, 30, 40, 50, 60], 3);
        // a representative byte-lane in each block: lifecycle[0], nonce[0]=byte 32, account_id[0]=byte 48.
        for &b in &[0usize, 32, 48, C6_BYTE_LANES - 1] {
            let mut m = matrix(&cells, ext_multi(21));
            m.values[cell_base(c6_byte_cell(b))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered C6 absorbed byte-lane {b} NOT rejected"
            );
        }
        // a FIELD absorb lane (account_projection lane 3).
        let mut m = matrix(&cells, ext_multi(22));
        m.values[cell_base(c6_field_cell(3))] += Val::ONE;
        assert!(
            !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
            "tampered C6 FIELD absorb lane NOT rejected"
        );
    }

    /// FORGERY (C6) — a tampered INTERIOR perm cell (a committed round post / a perm input / the
    /// output lane the squeeze reads) of some C6 perm block ⇒ the inlined octic Poseidon2 round
    /// residuals reject. Confirms the C6 perm sub-blocks are fully constrained, not just the squeeze.
    #[test]
    fn c6_air_rejects_tampered_interior_perm_cell() {
        use crate::inlined_perm::{C_BEGIN_POST, C_INPUTS, C_PARTIAL_POST_SBOX};
        let cells = honest_cells([100, 200, 300, 400, 500, 600], 5);
        let targets = [
            c6_perm_base(0) + C_INPUTS + 3,
            c6_perm_base(5) + C_BEGIN_POST + 7,
            c6_perm_base(9) + C_PARTIAL_POST_SBOX + 10,
            c6_perm_base(C6_N_ABSORB - 1) + C_OUTPUT + 1, // last-absorb output lane (squeeze source)
            c6_perm_base(C6_N_ABSORB) + C_OUTPUT + 0,     // squeeze perm output lane
        ];
        for &cell_idx in &targets {
            let mut m = matrix(&cells, ext_multi(33));
            m.values[cell_base(cell_idx)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered C6 interior perm cell {cell_idx} NOT rejected"
            );
        }
    }

    // =======================================================================
    // ROLE-ROOT SPONGE (CLASS A) — the THIRD mirrored leaf sponge, CHAINED to
    // the C1/C6 squeeze outputs (role input lanes 0..20 ARE the C1/C6 outputs).
    // =======================================================================

    /// Base residual index of the role-root sponge class within `constraint_residual`.
    const ROLE_RESIDUAL_BASE: usize = C4_LANES * C4_N_PER_LANE
        + N_C4_ACC
        + N_SEAM_PAD
        + N_SEAM_ACC
        + N_RELATION_ID
        + N_C1
        + N_C6;

    /// FIDELITY PIN (role-root) — an HONEST role-root sponge (built from the deployed byte-pinned Val
    /// permutation via `fill_honest_role`, over the CHAINED C1/C6 squeeze outputs ‖ 6 masked lanes,
    /// lifted to the extension) makes EVERY role residual zero: the absorb / capacity wiring across
    /// all 4 perm blocks and the 8-lane squeeze-output equality. A wrong octic perm / absorb wiring /
    /// output mapping would make some honest residual nonzero — caught here by attempted equality.
    #[test]
    fn role_honest_sponge_all_residuals_zero() {
        for seed in [1u32, 7, 42, 255] {
            let cells = honest_cells([0, 1, 127, 128, 12288, 5000], seed);
            for k in 0..N_ROLE {
                assert_eq!(
                    constraint_residual(&cells, ROLE_RESIDUAL_BASE + k),
                    Challenge::ZERO,
                    "honest role-root residual {k} nonzero (seed={seed})"
                );
            }
        }
    }

    /// FIDELITY PIN (role-root) — the squeezed role_root cells equal the source-perm OUTPUT cells
    /// (the leaf's `pcols.ending_full_rounds[..].post[lane]`, air.rs:1355), AND those outputs are the
    /// deployed permutation applied to the absorbed stream: pad_source_commitment[10] ‖
    /// opening_tuple_commitment[10] ‖ masked_projection[6] ‖ ONE-pad ‖ zero-fill — where the first 20
    /// lanes are the CHAINED C1/C6 squeeze outputs. We re-derive the source perm's output
    /// independently from the real Val perm and check the committed role_root matches. This pins both
    /// the role-root sponge AND the chaining (the role input IS the C1/C6 mirrored output).
    #[test]
    fn role_squeeze_matches_deployed_perm_output_over_chained_inputs() {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT};
        let cells = honest_cells([7000, 0, 12288, 128, 5000, 96], 9);
        let consts = inlined_perm::Constants::pinned();
        // input lanes as Val: the CHAINED C1 out ‖ C6 out ‖ masked lanes (read from the cells).
        let mut input_lane = [Val::ZERO; ROLE_INPUT_LANES];
        for pos in 0..ROLE_INPUT_LANES {
            let c = match role_absorb_source(pos) {
                C1Absorb::Cell(c) => c,
                _ => unreachable!(),
            };
            input_lane[pos] = Challenge::as_basis_coefficients_slice(&cells[c])[0];
        }
        // sanity: the first 20 chained inputs ARE the C1/C6 commitment digests.
        for j in 0..N_COMMIT_LANES {
            assert_eq!(
                Challenge::from(input_lane[j]),
                cells[c1_out_cell(j)],
                "role input lane {j} != C1 squeeze output (chaining broken)"
            );
            assert_eq!(
                Challenge::from(input_lane[N_COMMIT_LANES + j]),
                cells[c6_out_cell(j)],
                "role input lane {} != C6 squeeze output (chaining broken)",
                N_COMMIT_LANES + j
            );
        }
        let absorb = |pos: usize| -> Val {
            if pos < ROLE_INPUT_LANES {
                input_lane[pos]
            } else if pos == ROLE_INPUT_LANES {
                Val::ONE
            } else {
                Val::ZERO
            }
        };
        let mut prev: Option<[Val; WIDTH]> = None;
        let mut outs: Vec<[Val; WIDTH]> = Vec::new();
        for p in 0..ROLE_N_PERMS {
            let is_absorb = p < ROLE_N_ABSORB;
            let mut pin = [Val::ZERO; WIDTH];
            match prev {
                None => {
                    for r in 0..ROLE_RATE {
                        pin[r] = absorb(r);
                    }
                }
                Some(pv) => {
                    for r in 0..ROLE_RATE {
                        if is_absorb {
                            let pos = p * ROLE_RATE + r;
                            pin[r] = pv[r] + if pos < ROLE_PADDED_LEN { absorb(pos) } else { Val::ZERO };
                        } else {
                            pin[r] = pv[r];
                        }
                    }
                    for c in ROLE_RATE..WIDTH {
                        pin[c] = pv[c];
                    }
                }
            }
            let block = generate_perm_cols(pin, &consts);
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            outs.push(out);
            prev = Some(out);
        }
        for j in 0..ROLE_N_OUT {
            let (sp, lane) = role_output_source(j);
            let committed = cells[role_out_cell(j)];
            assert_eq!(
                committed,
                Challenge::from(outs[sp][lane]),
                "role_root lane {j} != deployed perm output (perm {sp} lane {lane})"
            );
        }
    }

    /// FORGERY (role-root) — a forged role_root digest cell ⇒ the squeeze-output equality residual is
    /// nonzero ⇒ the AIR rejects.
    #[test]
    fn role_air_rejects_forged_role_root() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        for j in 0..ROLE_N_OUT {
            let mut m = matrix(&cells, ext_multi(13));
            m.values[cell_base(role_out_cell(j))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged role_root lane {j} NOT rejected"
            );
        }
    }

    /// FORGERY (role-root) — a tampered masked_projection input lane (the only NEW role input cells)
    /// propagates into the absorb-perm input bindings ⇒ the input-binding residual + downstream perm
    /// residuals shift ⇒ the AIR rejects.
    #[test]
    fn role_air_rejects_tampered_masked_input_lane() {
        let cells = honest_cells([10, 20, 30, 40, 50, 60], 3);
        for i in 0..ROLE_MASKED_LANES {
            let mut m = matrix(&cells, ext_multi(21));
            m.values[cell_base(role_masked_cell(i))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered role masked_projection lane {i} NOT rejected"
            );
        }
    }

    /// FORGERY (role-root) — a tampered INTERIOR perm cell (a committed round post / a perm input /
    /// the output lane the squeeze reads) of some role perm block ⇒ the inlined octic Poseidon2 round
    /// residuals reject. Confirms the role perm sub-blocks are fully constrained, not just the squeeze.
    /// NOTE: the role sponge has NO separate squeeze perm (n_out=8 ≤ RATE); perms 0..4 are absorbs and
    /// perm 3 is the squeeze SOURCE.
    #[test]
    fn role_air_rejects_tampered_interior_perm_cell() {
        use crate::inlined_perm::{C_BEGIN_POST, C_INPUTS, C_PARTIAL_POST_SBOX};
        let cells = honest_cells([100, 200, 300, 400, 500, 600], 5);
        let targets = [
            role_perm_base(0) + C_INPUTS + 3,
            role_perm_base(1) + C_BEGIN_POST + 7,
            role_perm_base(2) + C_PARTIAL_POST_SBOX + 10,
            role_perm_base(ROLE_N_ABSORB - 1) + C_OUTPUT + 1, // last-absorb output (squeeze source)
            role_perm_base(ROLE_N_ABSORB - 1) + C_OUTPUT + 7, // another squeezed output lane
        ];
        for &cell_idx in &targets {
            let mut m = matrix(&cells, ext_multi(33));
            m.values[cell_base(cell_idx)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered role interior perm cell {cell_idx} NOT rejected"
            );
        }
    }

    /// CHAINING SOUNDNESS (role-root) — a tampered C1 OR C6 squeeze output cell must ALSO break the
    /// role-root, because the role absorb reads those cells as input lanes 0..20. Tampering the C1/C6
    /// output (without touching the role perm cells) shifts the role absorb-input binding ⇒ the role
    /// residuals reject. (The C1/C6 output is ALSO bound by its own squeeze-output equality, so it
    /// rejects via two classes — both are soundness; here we confirm the role-root chaining link.)
    #[test]
    fn role_air_rejects_tampered_chained_c1_c6_output() {
        let cells = honest_cells([3, 14, 159, 26, 53, 58], 7);
        // a C1 output lane and a C6 output lane (both feed the role absorb input lanes 0..20).
        let targets = [c1_out_cell(0), c1_out_cell(9), c6_out_cell(0), c6_out_cell(9)];
        for &cell_idx in &targets {
            let mut m = matrix(&cells, ext_multi(44));
            m.values[cell_base(cell_idx)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered chained C1/C6 output cell {cell_idx} NOT rejected at role-root mirror"
            );
        }
    }

    /// CHAINING ISOLATION (role-root) — verify the role-root residual class SPECIFICALLY reacts to a
    /// tampered C1/C6 output, not merely the C1/C6 own squeeze class. We tamper a C1 output cell and
    /// assert at least one ROLE residual (in the role residual range) becomes nonzero — proving the
    /// chaining is a genuine in-role binding, not an artifact of the C1 class alone.
    #[test]
    fn role_residual_class_reacts_to_chained_output_tamper() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 17);
        // honest: all role residuals zero.
        for k in 0..N_ROLE {
            assert_eq!(constraint_residual(&cells, ROLE_RESIDUAL_BASE + k), Challenge::ZERO);
        }
        // tamper C1 output lane 0 (a role absorb input at pos 0).
        let mut bad = cells;
        bad[c1_out_cell(0)] += Challenge::ONE;
        let any_role_nonzero =
            (0..N_ROLE).any(|k| constraint_residual(&bad, ROLE_RESIDUAL_BASE + k) != Challenge::ZERO);
        assert!(
            any_role_nonzero,
            "role-root residual class is BLIND to a tampered chained C1 output — chaining underconstrained"
        );
    }

    // =======================================================================
    // C2 MASK-PRF SPONGE (the LARGEST sponge) + WIDE-THEN-REDUCE + CARRIER.
    // =======================================================================

    /// Base residual index of the C2 sponge class within `constraint_residual`.
    const C2_RESIDUAL_BASE: usize = C4_LANES * C4_N_PER_LANE
        + N_C4_ACC
        + N_SEAM_PAD
        + N_SEAM_ACC
        + N_RELATION_ID
        + N_C1
        + N_C6
        + N_ROLE;
    /// Base residual index of the reduce-gadget + carrier class.
    const REDUCE_RESIDUAL_BASE: usize = C2_RESIDUAL_BASE + N_C2;

    /// SHAPE SANITY — the residual counts agree with the structural constants (so the dispatcher and
    /// the in-AIR loop cover EXACTLY the same residuals).
    #[test]
    fn c2_reduce_residual_counts_consistent() {
        // C2 sponge: input-binds + perm residuals, NO output sub-class.
        assert_eq!(N_C2, C2_N_PERMS * WIDTH + C2_N_PERMS * inlined_perm_octic::PERM_RESIDUALS);
        // reduce + carrier total.
        assert_eq!(N_REDUCE, NUM_OUTPUTS * N_REDUCE_PER_MASK + N_LANES * N_CARRIER_PER_LANE);
        // the chain class follows reduce; reduce+chain+byterange+pubio-bind cover up to N_CONSTRAINTS.
        assert_eq!(
            REDUCE_RESIDUAL_BASE + N_REDUCE + N_CHAIN + N_BYTERANGE + N_PUBIO_BIND,
            N_CONSTRAINTS
        );
    }

    /// FIDELITY PIN (C2) — an HONEST C2 sponge (built from the deployed byte-pinned Val permutation
    /// via `fill_honest_c2_and_reduce`, over pad_seed[32] ‖ slot_id[8], lifted to the extension) makes
    /// EVERY C2 residual zero: the absorb / capacity wiring across all 9 perm blocks. A wrong octic
    /// perm / absorb wiring would make some honest residual nonzero — caught by attempted equality.
    #[test]
    fn c2_honest_sponge_all_residuals_zero() {
        for seed in [1u32, 7, 42, 255] {
            let cells = honest_cells([0, 1, 127, 128, 12288, 5000], seed);
            for k in 0..N_C2 {
                assert_eq!(
                    constraint_residual(&cells, C2_RESIDUAL_BASE + k),
                    Challenge::ZERO,
                    "honest C2 residual {k} nonzero (seed={seed})"
                );
            }
        }
    }

    /// FIDELITY PIN (reduce + carrier) — an HONEST reduce gadget + carrier makes EVERY reduce/carrier
    /// residual zero: the 6 masks via the base-2^16 limb/carry Horner reduction, AND the carrier
    /// masked = (acc + mask) mod q. A wrong limb/carry / mod-q reduction (a soundness hole) would make
    /// some honest residual nonzero.
    #[test]
    fn reduce_honest_all_residuals_zero() {
        for seed in [1u32, 7, 42, 255] {
            let cells = honest_cells([0, 1, 127, 128, 12288, 5000], seed);
            for k in 0..N_REDUCE {
                assert_eq!(
                    constraint_residual(&cells, REDUCE_RESIDUAL_BASE + k),
                    Challenge::ZERO,
                    "honest reduce/carrier residual {k} nonzero (seed={seed})"
                );
            }
        }
    }

    /// FIDELITY PIN (reduce) — the 6 masks read from the opened reduce cells equal the LEAF's own
    /// `wide_reduce_masks` reference over the SAME 30 squeezed lanes, AND the carrier masked equals
    /// `(account_projection + mask) mod q12289`. This pins the WHOLE gadget against the deployed leaf
    /// reduce gadget + carrier semantics on a real honest row — a wrong gadget is caught by equality.
    #[test]
    fn reduce_masks_match_leaf_reduce_and_carrier() {
        use descender_account_stark::reduce_gadget::{wide_reduce_masks, NUM_SQUEEZE as RG_NSQ};
        use p3_field::PrimeField32;
        let vals = [7000u32, 0, 12288, 128, 5000, 96];
        let cells = honest_cells(vals, 9);
        // recover the 30 squeezed lanes from the opened C2 squeeze output cells.
        let mut lanes = [0u32; RG_NSQ];
        for o in 0..C2_N_OUT {
            let limb0: Val =
                Challenge::as_basis_coefficients_slice(&cells[c2_squeeze_output_cell(o)])[0];
            lanes[o] = limb0.as_canonical_u32();
        }
        let leaf_masks = wide_reduce_masks::<Val>(&lanes);
        let q = Q12289 as u64;
        for j in 0..NUM_OUTPUTS {
            // mask read from the opened gadget cells (last Horner step's r_t).
            let mask = reduce_masks_from_cells(&cells)[j];
            assert_eq!(
                mask,
                Challenge::from(Val::from_u32(leaf_masks[j])),
                "reduce mask lane {j} != leaf wide_reduce_masks"
            );
            // carrier: masked == (acc + mask) mod q.
            let masked_limb0: Val =
                Challenge::as_basis_coefficients_slice(&cells[carrier_masked_cell(j)])[0];
            let masked = masked_limb0.as_canonical_u32() as u64;
            let expected = (vals[j] as u64 + leaf_masks[j] as u64) % q;
            assert_eq!(masked, expected, "carrier masked_projection lane {j} != (acc+mask) mod q");
        }
    }

    /// FORGERY (C2) — a tampered absorbed lane (a pad_seed byte, reused from C2_PAD, OR a slot_id
    /// byte) propagates into the absorb-perm input bindings ⇒ rejected. NOTE: tampering a pad_seed
    /// cell ALSO breaks SeamPadSeed and the C1 sponge — all soundness; here we use the slot_id lanes
    /// (UNIQUE to C2) to isolate the C2 sponge binding, and one shared pad_seed lane for completeness.
    #[test]
    fn c2_air_rejects_tampered_absorbed_lane() {
        let cells = honest_cells([10, 20, 30, 40, 50, 60], 3);
        // slot_id lanes (unique to C2).
        for i in 0..C2_SLOT_LANES {
            let mut m = matrix(&cells, ext_multi(21));
            m.values[cell_base(c2_slot_cell(i))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered C2 slot_id lane {i} NOT rejected"
            );
        }
    }

    /// FORGERY (C2) — a tampered INTERIOR perm cell (a committed round post / a perm input / a
    /// squeeze-source output lane) of some C2 perm block ⇒ the inlined octic Poseidon2 round
    /// residuals reject. Confirms the 9 C2 perm sub-blocks are fully constrained.
    #[test]
    fn c2_air_rejects_tampered_interior_perm_cell() {
        use crate::inlined_perm::{C_BEGIN_POST, C_INPUTS, C_PARTIAL_POST_SBOX};
        let cells = honest_cells([100, 200, 300, 400, 500, 600], 5);
        let targets = [
            c2_perm_base(0) + C_INPUTS + 3,
            c2_perm_base(3) + C_BEGIN_POST + 7,
            c2_perm_base(5) + C_PARTIAL_POST_SBOX + 10,
            c2_perm_base(C2_N_ABSORB - 1) + C_OUTPUT + 1, // last-absorb output (squeeze source, lane 1)
            c2_perm_base(C2_N_PERMS - 1) + C_OUTPUT + 5,  // last squeeze perm output (lane 5)
        ];
        for &cell_idx in &targets {
            let mut m = matrix(&cells, ext_multi(33));
            m.values[cell_base(cell_idx)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered C2 interior perm cell {cell_idx} NOT rejected"
            );
        }
    }

    /// FORGERY (reduce) — a forged SQUEEZED lane (the C2 output the reduce gadget reads) shifts the
    /// reduce gadget's lane-bind eq AND the C2 perm output residual ⇒ rejected. This is the link from
    /// the C2 sponge into the reduce gadget: a forged squeezed value cannot pass.
    #[test]
    fn reduce_air_rejects_forged_squeezed_lane() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        for &o in &[0usize, 7, 15, 29] {
            let mut m = matrix(&cells, ext_multi(13));
            m.values[cell_base(c2_squeeze_output_cell(o))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged C2 squeezed lane {o} NOT rejected at reduce mirror"
            );
        }
    }

    /// FORGERY (reduce) — a forged MASK (the last Horner step's r_t cell) shifts both the reduce
    /// gadget's mod-q identities for that step AND the carrier `acc + mask == masked + wrap·q` ⇒
    /// rejected. This pins the mask -> carrier soundness link.
    #[test]
    fn reduce_air_rejects_forged_mask() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        let step0 = LANES_PER_MASK;
        let last_sb = step0 + (HORNER_STEPS - 1) * STEP_CELLS;
        for j in 0..NUM_OUTPUTS {
            let mask_base = j * PER_MASK_CELLS;
            let mut m = matrix(&cells, ext_multi(13));
            // tamper the last step's r_t value cell (the mask) for lane j.
            m.values[cell_base(reduce_cell(mask_base + last_sb + StepOff::R))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged reduce mask lane {j} NOT rejected"
            );
        }
    }

    /// FORGERY (carrier) — a forged masked_projection value (the carrier output, also the public
    /// masked_projection) ⇒ the carrier mod-q identities reject (masked-recompose / masked+slack==q-1
    /// / acc+mask==masked+wrap·q). This is the carrier soundness: a forged masked cannot pass.
    #[test]
    fn carrier_air_rejects_forged_masked_projection() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        for j in 0..N_LANES {
            let mut m = matrix(&cells, ext_multi(13));
            m.values[cell_base(carrier_masked_cell(j))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged masked_projection lane {j} NOT rejected at carrier mirror"
            );
        }
    }

    /// FORGERY (reduce) — a tampered LIMB/CARRY cell (qt_lo, a carry c0, an s-limb, or a bit of one of
    /// these) of some Horner step ⇒ the limb/carry mod-q identities reject. This is the core of the
    /// reduce gadget's soundness (the wrap-free base-2^16 reduction); a tampered limb/carry is caught.
    #[test]
    fn reduce_air_rejects_tampered_limb_carry() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        // representative limb/carry value + bit cells across masks/steps.
        let targets: [(usize, usize, usize); 6] = [
            (0, 0, StepOff::QT_LO),       // mask0 step0 qt_lo value
            (1, 2, StepOff::C0),          // mask1 step2 c0 carry value
            (2, 4, StepOff::S0),          // mask2 step4 s0 limb value
            (3, 1, StepOff::D1),          // mask3 step1 d1 carry value
            (4, 3, StepOff::C0_BITS),     // mask4 step3 c0 bit 0
            (5, 0, StepOff::LANE_HI_BITS), // mask5 step0 lane_hi bit 0
        ];
        for &(j, t, off) in &targets {
            let mask_base = j * PER_MASK_CELLS;
            let step0 = mask_base + LANES_PER_MASK;
            let sb = step0 + t * STEP_CELLS;
            let mut m = matrix(&cells, ext_multi(33));
            m.values[cell_base(reduce_cell(sb + off))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered reduce limb/carry (mask {j} step {t} off {off}) NOT rejected"
            );
        }
    }

    /// FORGERY (carrier) — a tampered wrap bit or a mod-q witness bit ⇒ the carrier / range identities
    /// reject. Confirms the carrier mod-q witness block is fully constrained.
    #[test]
    fn carrier_air_rejects_tampered_modq_witness() {
        let cells = honest_cells([1, 2, 3, 4, 5, 6], 11);
        // mod-q block layout (cell offsets within the block, mirroring air.rs): cell 0 unused,
        // [1..1+14] masked_bits, [1+14] slack value, [2+14..2+28] slack_bits, [last] wrap.
        let slack_off = 1 + MODQ_BITS;
        let wrap_off = slack_off + 1 + MODQ_BITS;
        for j in 0..N_LANES {
            for &within in &[1usize /*masked bit0*/, slack_off /*slack value*/, wrap_off /*wrap*/] {
                let mut m = matrix(&cells, ext_multi(13));
                m.values[cell_base(carrier_modq_base(j) + within)] += Val::ONE;
                assert!(
                    !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                    "tampered carrier mod-q cell (lane {j} within {within}) NOT rejected"
                );
            }
        }
    }

    // =======================================================================
    // BATCH DIGEST CHAIN — the per-row running Poseidon2 sponge + CROSS-ROW transition.
    // =======================================================================

    /// Base residual index of the batch-digest-chain class within `constraint_residual`.
    const CHAIN_RESIDUAL_BASE: usize = REDUCE_RESIDUAL_BASE + N_REDUCE;

    /// Base of the byte-range class (follows chain).
    const BYTERANGE_RESIDUAL_BASE: usize = CHAIN_RESIDUAL_BASE + N_CHAIN;

    /// Base of the sponge-output→public-value binding class (the LAST class; follows byte-range).
    const PUBIO_BIND_RESIDUAL_BASE: usize = BYTERANGE_RESIDUAL_BASE + N_BYTERANGE;

    /// SHAPE SANITY — the chain residual counts agree with the structural constants, and the chain
    /// class is followed by the byte-range class (which now covers up to N_CONSTRAINTS).
    #[test]
    fn chain_residual_counts_consistent() {
        assert_eq!(
            N_CHAIN,
            1 + CHAIN_NPERMS * CHAIN_W
                + CHAIN_NPERMS * inlined_perm_octic::PERM_RESIDUALS
                + CHAIN_W
                + CHAIN_W
                + 1
        );
        assert_eq!(CHAIN_RESIDUAL_BASE + N_CHAIN + N_BYTERANGE + N_PUBIO_BIND, N_CONSTRAINTS);
        // sanity: the chain uses the full 16-wide perm and rate-8 absorb (mirrors air.rs).
        assert_eq!(CHAIN_W, WIDTH);
        assert_eq!(CHAIN_RT, 8);
        assert_eq!(CHAIN_PADDED_LEN, CHAIN_NPERMS * CHAIN_RT);
    }

    /// FIDELITY PIN (chain) — an HONEST chain step (built from the deployed byte-pinned Val
    /// permutation via `fill_honest_chain`, lifted to the extension) makes EVERY chain residual zero:
    /// the enable-bool, the per-row absorb input/capacity wiring across all CHAIN_NPERMS perm blocks,
    /// the chain_out fold, the cross-row carry (chain_in == prev_chain_out), and the count
    /// accumulator. A wrong octic perm / absorb wiring / fold / carry would make some honest residual
    /// nonzero — caught by attempted equality.
    #[test]
    fn chain_honest_all_residuals_zero() {
        for seed in [1u32, 7, 42, 255] {
            let cells = honest_cells([0, 1, 127, 128, 12288, 5000], seed);
            for k in 0..N_CHAIN {
                assert_eq!(
                    constraint_residual(&cells, CHAIN_RESIDUAL_BASE + k),
                    Challenge::ZERO,
                    "honest chain residual {k} nonzero (seed={seed})"
                );
            }
        }
    }

    /// FIDELITY PIN (chain) — the in-AIR per-row absorb + fold equals the LEAF's own reference
    /// `chain_digest` over a single enabled pubio tuple, starting from the SAME prev state. I.e. the
    /// opened `chain_out` lanes must equal `Poseidon2-sponge(chain_in, [1, pubio...])` exactly — the
    /// authority is the deployed `run_chain_step` semantics (mirrored by `fill_honest_chain`). We
    /// independently re-run the absorb over the recovered prev-state + pubio and require equality.
    #[test]
    fn chain_out_matches_deployed_perm_fold() {
        use crate::inlined_perm::{self, generate_perm_cols, C_OUTPUT as IP_OUTPUT};
        let consts = inlined_perm::Constants::pinned();
        let cells = honest_cells([7000, 0, 12288, 128, 5000, 96], 9);

        // recover prev-state, pubio, enable from the opened cells.
        let prev_state: [Val; CHAIN_W] = core::array::from_fn(|c| {
            let s: &[Val] = Challenge::as_basis_coefficients_slice(&cells[chain_prev_out_cell(c)]);
            s[0]
        });
        let pubio: Vec<Val> = (0..CHAIN_PUBIO)
            .map(|i| {
                let s: &[Val] = Challenge::as_basis_coefficients_slice(&cells[chain_pubio_cell(i)]);
                s[0]
            })
            .collect();
        let enable: Val = Challenge::as_basis_coefficients_slice(&cells[chain_enable_cell()])[0];
        assert_eq!(enable, Val::ONE, "fidelity uses an enabled row");

        // re-run the per-row absorb independently (msg = [1, pubio...] zero-padded).
        let msg = |pos: usize| -> Val {
            if pos == 0 {
                Val::ONE
            } else if pos <= CHAIN_PUBIO {
                pubio[pos - 1]
            } else {
                Val::ZERO
            }
        };
        let mut prev_out = prev_state;
        for p in 0..CHAIN_NPERMS {
            let mut perm_in = [Val::ZERO; WIDTH];
            let base_state = if p == 0 { prev_state } else { prev_out };
            for r in 0..CHAIN_RT {
                perm_in[r] = base_state[r] + msg(p * CHAIN_RT + r);
            }
            for c in CHAIN_RT..WIDTH {
                perm_in[c] = base_state[c];
            }
            let block = generate_perm_cols(perm_in, &consts);
            let mut out = [Val::ZERO; WIDTH];
            out.copy_from_slice(&block[IP_OUTPUT..IP_OUTPUT + WIDTH]);
            prev_out = out;
        }
        // chain_out (enable=1) must equal the re-derived absorbed state lane-for-lane.
        for c in 0..CHAIN_W {
            let opened: Val = Challenge::as_basis_coefficients_slice(&cells[chain_out_cell(c)])[0];
            assert_eq!(opened, prev_out[c], "chain_out lane {c} != deployed-perm fold");
        }
    }

    /// FIDELITY PIN (chain) — a PADDING row (enable=0) is a TRUE no-op: chain_out == chain_in
    /// regardless of the absorb perms, and count_acc == prev_count_acc. We build a padding row by
    /// hand (enable=0, chain_in carried, chain_out=chain_in, count_acc=prev_count) and require all
    /// chain residuals to vanish — mirroring `generate_batch_trace`'s padding branch.
    #[test]
    fn chain_padding_row_is_noop() {
        // build a proper padding row: enable=0 ⇒ all-zero absorb message, perms run over the zero
        // message, chain_out == chain_in, count_acc == prev_count_acc. ALL chain residuals vanish.
        let mut cells = honest_cells([1, 2, 3, 4, 5, 6], 17);
        fill_honest_chain(&mut cells, 17, false);
        // chain_out must equal chain_in (the no-op fold).
        for c in 0..CHAIN_W {
            assert_eq!(
                cells[chain_out_cell(c)],
                cells[chain_in_cell(c)],
                "padding row chain_out lane {c} != chain_in"
            );
        }
        for k in 0..N_CHAIN {
            assert_eq!(
                constraint_residual(&cells, CHAIN_RESIDUAL_BASE + k),
                Challenge::ZERO,
                "padding (enable=0) chain residual {k} nonzero — no-op branch broken"
            );
        }
    }

    /// FORGERY (chain) — tampering ANY chain cell that feeds a mirrored chain constraint shifts the
    /// recomputed C_partial ⇒ rejected at the mirror. Covers: enable, an absorbed pubio lane, a
    /// chain_in lane, an interior perm post, the squeeze-source (last-perm) output, a chain_out lane,
    /// the count accumulator, and the prev_count. (The CROSS-ROW prev_chain_out tamper is its own
    /// test below.)
    #[test]
    fn chain_air_rejects_tampered_cell() {
        use crate::inlined_perm::{C_BEGIN_POST, C_OUTPUT as IP_OUTPUT};
        let cells = honest_cells([10, 20, 30, 40, 50, 60], 3);
        let targets = [
            chain_enable_cell(),                       // enable (boolean + fold + count)
            chain_pubio_cell(0),                       // absorbed pubio lane 0
            chain_pubio_cell(CHAIN_PUBIO - 1),         // absorbed pubio last lane
            chain_in_cell(0),                          // chain_in lane 0 (absorb + carry)
            chain_in_cell(CHAIN_W - 1),                // chain_in capacity lane (carry + cap bind)
            chain_perm_base(0) + C_BEGIN_POST,         // interior perm post (perm 0)
            chain_perm_base(CHAIN_NPERMS - 1) + IP_OUTPUT + 1, // last-perm output (the absorbed fold src)
            chain_out_cell(0),                         // chain_out lane 0 (the running digest)
            chain_count_cell(),                        // count accumulator
            chain_prev_count_cell(),                   // prev count (cross-row count carry)
        ];
        for &cell_idx in &targets {
            let mut m = matrix(&cells, ext_multi(21));
            m.values[cell_base(cell_idx)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered chain cell {cell_idx} NOT rejected — that residual is underconstrained"
            );
        }
    }

    /// FORGERY (chain) — the CROSS-ROW tamper: altering a `prev_chain_out` lane (a swapped/forged
    /// prior-row digest) breaks the cross-row carry `chain_in == prev_chain_out` ⇒ rejected. This is
    /// the running-accumulator soundness: a forger who tries to swap the prior row's state cannot keep
    /// the chain consistent. (prev_chain_out is a BOUND cell — see the module note on how the
    /// single-row harness binds the prev value.)
    #[test]
    fn chain_air_rejects_cross_row_tamper() {
        let cells = honest_cells([100, 200, 300, 400, 500, 600], 5);
        for c in [0usize, 7, CHAIN_W - 1] {
            let mut m = matrix(&cells, ext_multi(33));
            m.values[cell_base(chain_prev_out_cell(c))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "CROSS-ROW tamper of prev_chain_out lane {c} NOT rejected — the carry is underconstrained"
            );
        }
    }

    /// FIDELITY PIN (chain) — the enable boolean is enforced: a non-boolean enable (e.g. 2) makes the
    /// enable-bool residual nonzero. Mirrors `assert_bool(enable)` (air.rs:1148).
    #[test]
    fn chain_enable_must_be_boolean() {
        let mut cells = honest_cells([1, 2, 3, 4, 5, 6], 7);
        cells[chain_enable_cell()] = Challenge::from(Val::from_u32(2));
        // the enable-bool residual is the FIRST chain residual.
        assert_ne!(
            constraint_residual(&cells, CHAIN_RESIDUAL_BASE),
            Challenge::ZERO,
            "enable-boolean residual blind to a non-boolean enable"
        );
    }

    // ----- PER-LANE BYTE-RANGE / canonicality (air.rs:1278-1289) -----

    /// SHAPE SANITY — byte-range counts mirror air.rs `FullLayout::new` byte_ranged masks EXACTLY,
    /// and the byte-range class is followed by the pubio-bind class (together up to N_CONSTRAINTS).
    #[test]
    fn byterange_shape_matches_leaf() {
        // C1: all 48 lanes; C2: all 40; C6: only the 80 byte-lanes (6 FIELD lanes excluded).
        assert_eq!(C1_BR_LANES, 48);
        assert_eq!(C2_BR_LANES, 40);
        assert_eq!(C6_BR_LANES, 80);
        assert_eq!(N_BR_LANES, 168);
        assert_eq!(N_BR_PER_LANE, BYTE_BITS + 1);
        assert_eq!(N_BYTERANGE, N_BR_LANES * N_BR_PER_LANE);
        assert_eq!(N_BYTERANGE_BITS, N_BR_LANES * BYTE_BITS);
        assert_eq!(BYTERANGE_RESIDUAL_BASE + N_BYTERANGE + N_PUBIO_BIND, N_CONSTRAINTS);
        // the recompose eq for EVERY lane targets the REAL already-absorbed sponge input cell,
        // matching the sponge's own absorb source — NOT a fresh free column (the vacuity guard).
        for g in 0..C1_BR_LANES {
            assert_eq!(byterange_absorbed_cell(g), c1_input_cell(g));
        }
        for i in 0..C2_PAD_LANES {
            assert_eq!(byterange_absorbed_cell(C1_BR_LANES + i), c2_pad_cell(i));
        }
        for i in 0..C2_SLOT_LANES {
            assert_eq!(byterange_absorbed_cell(C1_BR_LANES + C2_PAD_LANES + i), c2_slot_cell(i));
        }
        for b in 0..C6_BR_LANES {
            assert_eq!(byterange_absorbed_cell(C1_BR_LANES + C2_BR_LANES + b), c6_byte_cell(b));
        }
    }

    /// FIDELITY PIN (byte-range) — on an honest row EVERY byte-range residual is zero (8 bit-bools +
    /// recompose eq per lane), and the decomposed bits genuinely reconstruct the absorbed lane value.
    #[test]
    fn byterange_honest_all_residuals_zero() {
        use p3_field::PrimeField32;
        let cells = honest_cells([0, 1, 127, 12288, 5000, 96], 17);
        for k in 0..N_BYTERANGE {
            assert_eq!(
                constraint_residual(&cells, BYTERANGE_RESIDUAL_BASE + k),
                Challenge::ZERO,
                "honest byte-range residual {k} nonzero"
            );
        }
        // independent reconstruction: bits sum back to the absorbed lane byte.
        for g in 0..N_BR_LANES {
            let lane_v: Val =
                Challenge::as_basis_coefficients_slice(&cells[byterange_absorbed_cell(g)])[0];
            let lane = lane_v.as_canonical_u32();
            let mut acc = 0u32;
            for b in 0..BYTE_BITS {
                let bit_v: Val =
                    Challenge::as_basis_coefficients_slice(&cells[byterange_bit_cell(g, b)])[0];
                let bit = bit_v.as_canonical_u32();
                assert!(bit <= 1, "byte-range bit g={g} b={b} not boolean in honest fill");
                acc |= bit << b;
            }
            assert_eq!(acc, lane, "byte-range bits don't reconstruct absorbed lane g={g}");
            assert!(lane < 256, "absorbed lane g={g} is not a byte ({lane})");
        }
    }

    /// FIDELITY PIN (byte-range, NOT vacuous) — tampering the ACTUAL absorbed sponge input lane (with
    /// its bits unchanged) breaks the recompose eq for THAT lane. Proves the decomposition constrains
    /// the genuine consumed lane, not a free column. We probe a lane from each sponge (C1, C2, C6).
    #[test]
    fn byterange_recompose_binds_real_absorbed_lane() {
        let probes = [
            0usize,                                   // C1 lane 0 (pad_seed)
            C1_BR_LANES + 35,                         // C2 lane 35 (a slot_id lane)
            C1_BR_LANES + C2_BR_LANES + 40,           // C6 lane 40 (an account_id byte-lane)
        ];
        for &g in &probes {
            let mut bad = honest_cells([3, 14, 159, 26, 53, 58], 7);
            // tamper the absorbed lane cell (limb 0) WITHOUT touching its bits.
            bad[byterange_absorbed_cell(g)] += Challenge::ONE;
            // the recompose eq is the LAST within-lane residual for index g.
            let rk = BYTERANGE_RESIDUAL_BASE + g * N_BR_PER_LANE + BYTE_BITS;
            assert_ne!(
                constraint_residual(&bad, rk),
                Challenge::ZERO,
                "recompose eq for byte-range lane {g} blind to its absorbed cell — VACUOUS check"
            );
        }
    }

    /// FORGERY (byte-range) — a non-boolean bit makes its bit-bool residual nonzero AND (since the
    /// lane no longer equals Σ bit·2^b unless the bits were also adjusted) shifts the recompose eq.
    /// Mirrors `assert_bool(bit)` (air.rs:1283).
    #[test]
    fn byterange_rejects_nonboolean_bit() {
        let g = C1_BR_LANES + C2_BR_LANES + 5; // a C6 byte-lane
        let mut bad = honest_cells([1, 2, 3, 4, 5, 6], 9);
        bad[byterange_bit_cell(g, 3)] += Challenge::ONE; // bit 3: 0->1 or 1->2 (non-boolean if 1)
        // the bit-bool residual for bit 3 of lane g:
        let bk = BYTERANGE_RESIDUAL_BASE + g * N_BR_PER_LANE + 3;
        // EITHER the bit became non-boolean (bit-bool residual nonzero) OR it flipped a valid bit
        // (recompose eq nonzero). At least one residual of this lane must react.
        let any_react = (0..N_BR_PER_LANE).any(|i| {
            constraint_residual(&bad, BYTERANGE_RESIDUAL_BASE + g * N_BR_PER_LANE + i)
                != Challenge::ZERO
        });
        assert!(any_react, "tampered byte-range bit did not break any residual of its lane");
        // and specifically: if the bit was 1, +1 makes it 2 ⇒ bit-bool 2·1=2 ≠ 0.
        let _ = bk;
    }

    /// FORGERY (byte-range, end-to-end through the mirror AIR) — three distinct tampers, each rejected
    /// by `check_all_constraints`:
    ///   (1) out-of-range bit (non-boolean limb),
    ///   (2) recomposition mismatch (absorbed lane changed, bits not),
    ///   (3) a value that passes the bit-bools but whose bits don't sum to the lane (limb ≥ byte).
    #[test]
    fn byterange_air_rejects_tampers() {
        let cells = honest_cells([0, 1, 127, 128, 12288, 5000], 23);
        // honest accepts.
        let m_ok = matrix(&cells, ext(31));
        assert!(
            check_all_constraints(&LeafConstraintMirrorAir, &m_ok, &[], None).is_ok(),
            "honest byte-range row rejected"
        );

        // (1) out-of-range / non-boolean bit on a C1 lane.
        {
            let mut m = matrix(&cells, ext(31));
            let g = 0;
            m.values[cell_base(byterange_bit_cell(g, 2))] += Val::from_u32(2); // bit -> non-boolean
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "non-boolean byte-range bit NOT rejected"
            );
        }
        // (2) recomposition mismatch on a C2 slot lane (absorbed cell shifted, bits unchanged).
        {
            let mut m = matrix(&cells, ext(31));
            let g = C1_BR_LANES + C2_PAD_LANES + 2;
            m.values[cell_base(byterange_absorbed_cell(g))] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "byte-range recompose mismatch NOT rejected"
            );
        }
        // (3) bits stay boolean but no longer sum to the absorbed lane (clear a set bit) on a C6 lane.
        {
            // pick a C6 byte-lane whose absorbed value has bit 0 set (odd) so clearing it is a real
            // change; fall back to forcing a known mismatch regardless.
            let g = C1_BR_LANES + C2_BR_LANES + 1;
            let mut m = matrix(&cells, ext(31));
            // set bit 0 to 0 (boolean) — if it was 1 the sum drops by 1 ≠ lane; if it was already 0,
            // instead set bit 7 high to perturb the sum while staying boolean.
            let cur: Val =
                Challenge::as_basis_coefficients_slice(&cells[byterange_bit_cell(g, 0)])[0];
            if cur == Val::ONE {
                m.values[cell_base(byterange_bit_cell(g, 0))] -= Val::ONE; // 1 -> 0, boolean, breaks sum
            } else {
                m.values[cell_base(byterange_bit_cell(g, 0))] += Val::ONE; // 0 -> 1, boolean, breaks sum
            }
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "byte-range sum-mismatch (boolean bits, wrong total) NOT rejected"
            );
        }
    }

    // ----- SPONGE-OUTPUT → PUBLIC-VALUE binding (the per-instance commitment equalities) -----

    /// SHAPE SANITY — the pubio-bind class is the LAST class (covers exactly up to N_CONSTRAINTS),
    /// its count is the four per-instance commitment outputs, and EVERY binding maps the
    /// already-mirrored sponge/gadget OUTPUT cell to the SAME per-row pubio cell the digest chain
    /// absorbs (`chain_pubio_cell`) at the air.rs batch offset — one source of truth, not a fresh
    /// column on either side.
    #[test]
    fn pubio_bind_shape_matches_leaf() {
        assert_eq!(N_PUBIO_C1, N_COMMIT_LANES);
        assert_eq!(N_PUBIO_C6, N_COMMIT_LANES);
        assert_eq!(N_PUBIO_MASKED, N_LANES);
        assert_eq!(N_PUBIO_ROLE, ROLE_ROOT_ELEMS);
        assert_eq!(N_PUBIO_BIND, 2 * N_COMMIT_LANES + N_LANES + ROLE_ROOT_ELEMS);
        assert_eq!(PUBIO_BIND_RESIDUAL_BASE + N_PUBIO_BIND, N_CONSTRAINTS);
        // C1: c1_out_cell(j) == chain_pubio_cell(j).
        for j in 0..N_COMMIT_LANES {
            assert_eq!(pubio_bind_pair(j), (c1_out_cell(j), chain_pubio_cell(j)));
        }
        // C6: c6_out_cell(j) == chain_pubio_cell(N_COMMIT_LANES + j).
        for j in 0..N_COMMIT_LANES {
            assert_eq!(
                pubio_bind_pair(N_COMMIT_LANES + j),
                (c6_out_cell(j), chain_pubio_cell(N_COMMIT_LANES + j))
            );
        }
        // masked: carrier_masked_cell(j) == chain_pubio_cell(2·N_COMMIT_LANES + j).
        for j in 0..N_LANES {
            assert_eq!(
                pubio_bind_pair(2 * N_COMMIT_LANES + j),
                (carrier_masked_cell(j), chain_pubio_cell(2 * N_COMMIT_LANES + j))
            );
        }
        // role: role_out_cell(j) == chain_pubio_cell(PUBIO_ROLE_ROOT_OFF + j).
        for j in 0..ROLE_ROOT_ELEMS {
            assert_eq!(
                pubio_bind_pair(2 * N_COMMIT_LANES + N_LANES + j),
                (role_out_cell(j), chain_pubio_cell(PUBIO_ROLE_ROOT_OFF + j))
            );
        }
    }

    /// FIDELITY PIN (pubio-bind) — on an honest row EVERY pubio-bind residual is zero: the
    /// fixture sets the absorbed pubio commitment lanes EQUAL to the actual mirrored sponge/gadget
    /// outputs (exactly as the batch leaf AIR binds the squeezed output to the per-row pubio block),
    /// so `output − pubio == 0`. ALSO checks the bound pubio cells genuinely carry the sponge-output
    /// values (NOT zero / NOT the arbitrary non-commitment fill) — the non-vacuity of the fixture.
    #[test]
    fn pubio_bind_honest_all_residuals_zero() {
        let cells = honest_cells([0, 1, 127, 12288, 5000, 96], 17);
        for k in 0..N_PUBIO_BIND {
            assert_eq!(
                constraint_residual(&cells, PUBIO_BIND_RESIDUAL_BASE + k),
                Challenge::ZERO,
                "honest pubio-bind residual {k} nonzero"
            );
        }
        // non-vacuity: the bound pubio cell really holds the mirrored output value (so the equality
        // is meaningful, not "both sides happen to be zero"). At least one output is nonzero.
        let mut any_nonzero = false;
        for k in 0..N_PUBIO_BIND {
            let (out_cell, pubio_cell) = pubio_bind_pair(k);
            assert_eq!(cells[out_cell], cells[pubio_cell], "pubio cell != mirrored output at {k}");
            if cells[out_cell] != Challenge::ZERO {
                any_nonzero = true;
            }
        }
        assert!(any_nonzero, "all mirrored outputs zero — pubio-bind fixture is vacuous");
    }

    /// FORGERY (pubio-bind, FORGED PUBLIC COMMITMENT) — a public commitment lane (the per-row pubio
    /// cell the chain absorbs) set to a value ≠ the mirrored sponge output breaks the binding residual
    /// for that lane. THIS is the core soundness property: a forger cannot expose a public commitment
    /// inconsistent with the in-circuit sponge. We probe one lane of each of the four commitment
    /// outputs (C1, C6, masked, role).
    #[test]
    fn pubio_bind_rejects_forged_public_commitment() {
        let probes = [
            0usize,                                   // C1 pad_source_commitment lane 0
            N_COMMIT_LANES + 3,                       // C6 opening_tuple_commitment lane 3
            2 * N_COMMIT_LANES + 2,                   // masked_projection lane 2
            2 * N_COMMIT_LANES + N_LANES + 5,         // role_root lane 5
        ];
        for &k in &probes {
            let mut bad = honest_cells([3, 14, 159, 26, 53, 58], 7);
            let (_out_cell, pubio_cell) = pubio_bind_pair(k);
            bad[pubio_cell] += Challenge::ONE; // forge the public commitment lane
            assert_ne!(
                constraint_residual(&bad, PUBIO_BIND_RESIDUAL_BASE + k),
                Challenge::ZERO,
                "forged public commitment lane {k} not caught by the binding"
            );
        }
    }

    /// FORGERY (pubio-bind, TAMPERED SPONGE OUTPUT) — conversely, a mirrored sponge OUTPUT cell ≠ the
    /// public commitment breaks the binding residual. (The output is ALSO bound to its recomputed
    /// sponge by the earlier class, so a real forger can't move it freely; here we confirm the binding
    /// reacts to a mismatch from the output side too.)
    #[test]
    fn pubio_bind_rejects_tampered_sponge_output() {
        let probes = [0usize, N_COMMIT_LANES, 2 * N_COMMIT_LANES, 2 * N_COMMIT_LANES + N_LANES];
        for &k in &probes {
            let mut bad = honest_cells([1, 2, 3, 4, 5, 6], 9);
            let (out_cell, _pubio_cell) = pubio_bind_pair(k);
            bad[out_cell] += Challenge::ONE; // tamper the mirrored output
            assert_ne!(
                constraint_residual(&bad, PUBIO_BIND_RESIDUAL_BASE + k),
                Challenge::ZERO,
                "tampered sponge output lane {k} not caught by the binding"
            );
        }
    }

    /// FORGERY (pubio-bind, end-to-end through the mirror AIR) — a forged public commitment lane and a
    /// tampered mirrored output are each rejected by the real `check_all_constraints` on the
    /// `LeafConstraintMirrorAir`. Soundness by attempted forgery, never by inspection.
    #[test]
    fn pubio_bind_air_rejects_tampers() {
        let cells = honest_cells([0, 1, 127, 128, 12288, 5000], 23);
        let m_ok = matrix(&cells, ext(31));
        assert!(
            check_all_constraints(&LeafConstraintMirrorAir, &m_ok, &[], None).is_ok(),
            "honest pubio-bind row rejected"
        );
        // (a) forge a public commitment lane (C6 lane 0 = the pubio cell the chain absorbs).
        {
            let mut m = matrix(&cells, ext(31));
            let (_oc, pubio_cell) = pubio_bind_pair(N_COMMIT_LANES);
            m.values[cell_base(pubio_cell)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "forged public commitment NOT rejected end-to-end at the mirror"
            );
        }
        // (b) tamper a mirrored output lane (role_root lane 0).
        {
            let mut m = matrix(&cells, ext(31));
            let (out_cell, _pc) = pubio_bind_pair(2 * N_COMMIT_LANES + N_LANES);
            m.values[cell_base(out_cell)] += Val::ONE;
            assert!(
                !check_all_constraints(&LeafConstraintMirrorAir, &m, &[], None).is_ok(),
                "tampered mirrored output NOT rejected end-to-end at the mirror"
            );
        }
    }
}
