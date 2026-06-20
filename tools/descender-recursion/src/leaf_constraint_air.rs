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
//!     DEFERRED for C1: the per-lane BYTE-RANGE decomposition of the 48 input lanes
//!     (air.rs:1278-1289) — a degree-2 algebraic gadget, not the sponge; rides with the byte/reduce
//!     gadgets in a later chunk.
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
//!     (`leaf_wired_tampered_c6_sponge_cell_rejects_end_to_end`). DEFERRED for C6 (same as C1): the
//!     per-lane BYTE-RANGE decomposition of the 80 byte-lanes (air.rs:1278-1289).
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
//! ## DEFERRED (the rest of leaf `C(zeta)` — STILL committed-not-constrained)
//!
//!   * **C2 — mask PRF sponge** (air.rs:728-737): 40-lane absorb (pad_seed[32] ‖ slot_id[8]),
//!     30 squeezed lanes, PLUS the WIDE-THEN-REDUCE `eval_reduce_gadget` (30 wide lanes → 6 masks)
//!     and the carrier mod-q reduction. Largest remaining sponge.
//!   * **batch digest chain** (air.rs:1112-1262): the per-row running Poseidon2 sponge over every
//!     row's pubio (the CHAIN_PERMS_PER_ROW absorbs + the cross-row transition).
//!   * the C2/C6 sponge-OUTPUT commitment equalities (`c*_out[j] == pubio[j]`) ride with their perms.
//!   * the per-lane BYTE-RANGE decompositions (C1/C2/C6 input lanes), CLASS-C packed-witness anchors,
//!     context bindings (slot/lifecycle/account_id).
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
    Challenge, Val, N_COMMIT_LANES, N_LANES, RELATION_ID, RELATION_ID_ELEMS, ROLE_ROOT_ELEMS, WIDTH,
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

/// Total residuals batched into `C_partial`.
pub const N_CONSTRAINTS: usize = C4_LANES * C4_N_PER_LANE
    + N_C4_ACC
    + N_SEAM_PAD
    + N_SEAM_ACC
    + N_RELATION_ID
    + N_C1
    + N_C6
    + N_ROLE;

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
    + ROLE_CELLS; // role-root sponge (masked[6] ‖ perm blocks ‖ role_root[8]); inputs 0..20 CHAINED

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
    // role-root sponge residuals (CLASS A, CHAINED to C1/C6 outputs).
    debug_assert!(r < N_ROLE);
    role_residual(cells, r)
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

/// Decode the committed `C_partial` cell of a packed row.
pub fn row_c_partial(row: &[Val]) -> Challenge {
    Challenge::from_basis_coefficients_slice(&row[L_CPARTIAL..L_CPARTIAL + D])
        .expect("C_partial decode")
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

    // (6) C1 sponge (pad_source_commitment squeeze) — the first mirrored leaf sponge. Order MUST
    // match the native `c1_residual`: input/capacity bindings (WIDTH/perm), then per-perm Poseidon2
    // round residuals (inlined_perm_octic), then squeeze-output equalities.
    {
        // adapter exposing the octic opened cells to inlined_perm_octic::perm_residuals.
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

        let perm_input = |p: usize, i: usize| cell(c1_perm_base(p) + C_INPUTS + i);
        let perm_output = |p: usize, i: usize| cell(c1_perm_base(p) + C_OUTPUT + i);
        let absorb = |pos: usize| -> [AB::Expr; D] {
            match c1_absorb_source(pos) {
                C1Absorb::Cell(c) => cell(c),
                C1Absorb::One => octic_scalar::<AB>(1),
                C1Absorb::Zero => octic_scalar::<AB>(0),
            }
        };

        // (a) input/capacity bindings.
        for p in 0..C1_N_PERMS {
            let is_absorb = p < C1_N_ABSORB;
            for lane in 0..WIDTH {
                let inp = perm_input(p, lane);
                let expected = if p == 0 {
                    if lane < C1_RATE {
                        absorb(lane)
                    } else {
                        octic_scalar::<AB>(0)
                    }
                } else {
                    let prev = perm_output(p - 1, lane);
                    if lane < C1_RATE {
                        if is_absorb {
                            let pos = p * C1_RATE + lane;
                            let elem = if pos < C1_PADDED_LEN {
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
        for p in 0..C1_N_PERMS {
            let mut pr = inlined_perm_octic::perm_residuals::<AB, _>(&adapter, c1_perm_base(p));
            residuals.append(&mut pr);
        }

        // (c) squeeze-output equalities: pad_source_commitment[j] − squeezed_output[j].
        // squeezed_output[j] is the OPENED committed-output cell of the source perm (the leaf's
        // `pcols.ending_full_rounds[..].post[lane]` = the C_OUTPUT cell), so the residual reads two
        // opened cells. Soundness of the squeeze value vs the absorbed data is enforced by the
        // per-perm round residuals (b), which bind that output cell to the recomputed permutation.
        for j in 0..N_C1_OUT {
            let (sp, lane) = c1_output_source(j);
            let out_cell = cell(c1_perm_base(sp) + C_OUTPUT + lane);
            let committed = cell(c1_out_cell(j));
            residuals.push(octic_sub::<AB>(&committed, &out_cell));
        }
    }

    // (7) C6 sponge (opening_tuple_commitment squeeze) — the second mirrored leaf sponge. Order
    // MUST match the native `c6_residual`: input/capacity bindings (WIDTH/perm), then per-perm
    // Poseidon2 round residuals (inlined_perm_octic), then squeeze-output equalities. Structurally
    // identical to the C1 block above with the C6 constants / absorb source / output target.
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

        let perm_input = |p: usize, i: usize| cell(c6_perm_base(p) + C_INPUTS + i);
        let perm_output = |p: usize, i: usize| cell(c6_perm_base(p) + C_OUTPUT + i);
        let absorb = |pos: usize| -> [AB::Expr; D] {
            match c6_absorb_source(pos) {
                C1Absorb::Cell(c) => cell(c),
                C1Absorb::One => octic_scalar::<AB>(1),
                C1Absorb::Zero => octic_scalar::<AB>(0),
            }
        };

        // (a) input/capacity bindings.
        for p in 0..C6_N_PERMS {
            let is_absorb = p < C6_N_ABSORB;
            for lane in 0..WIDTH {
                let inp = perm_input(p, lane);
                let expected = if p == 0 {
                    if lane < C6_RATE {
                        absorb(lane)
                    } else {
                        octic_scalar::<AB>(0)
                    }
                } else {
                    let prev = perm_output(p - 1, lane);
                    if lane < C6_RATE {
                        if is_absorb {
                            let pos = p * C6_RATE + lane;
                            let elem = if pos < C6_PADDED_LEN {
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
        for p in 0..C6_N_PERMS {
            let mut pr = inlined_perm_octic::perm_residuals::<AB, _>(&adapter, c6_perm_base(p));
            residuals.append(&mut pr);
        }

        // (c) squeeze-output equalities: opening_tuple_commitment[j] − squeezed_output[j].
        for j in 0..N_C6_OUT {
            let (sp, lane) = c6_output_source(j);
            let out_cell = cell(c6_perm_base(sp) + C_OUTPUT + lane);
            let committed = cell(c6_out_cell(j));
            residuals.push(octic_sub::<AB>(&committed, &out_cell));
        }
    }

    // (8) role-root sponge (CLASS A) — the third mirrored leaf sponge, CHAINED to the C1/C6 squeeze
    // outputs. Order MUST match the native `role_residual`: input/capacity bindings (WIDTH/perm),
    // then per-perm Poseidon2 round residuals (inlined_perm_octic), then squeeze-output equalities.
    // Structurally identical to the C1/C6 blocks; the ONLY differences are the role constants and
    // that `role_absorb_source` reads the ALREADY-MIRRORED `c1_out_cell`/`c6_out_cell` cells for
    // input lanes 0..20 (the chaining; air.rs:919-931) + the masked_projection cells for 20..26.
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

        let perm_input = |p: usize, i: usize| cell(role_perm_base(p) + C_INPUTS + i);
        let perm_output = |p: usize, i: usize| cell(role_perm_base(p) + C_OUTPUT + i);
        let absorb = |pos: usize| -> [AB::Expr; D] {
            match role_absorb_source(pos) {
                C1Absorb::Cell(c) => cell(c),
                C1Absorb::One => octic_scalar::<AB>(1),
                C1Absorb::Zero => octic_scalar::<AB>(0),
            }
        };

        // (a) input/capacity bindings.
        for p in 0..ROLE_N_PERMS {
            let is_absorb = p < ROLE_N_ABSORB;
            for lane in 0..WIDTH {
                let inp = perm_input(p, lane);
                let expected = if p == 0 {
                    if lane < ROLE_RATE {
                        absorb(lane)
                    } else {
                        octic_scalar::<AB>(0)
                    }
                } else {
                    let prev = perm_output(p - 1, lane);
                    if lane < ROLE_RATE {
                        if is_absorb {
                            let pos = p * ROLE_RATE + lane;
                            let elem = if pos < ROLE_PADDED_LEN {
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
        for p in 0..ROLE_N_PERMS {
            let mut pr = inlined_perm_octic::perm_residuals::<AB, _>(&adapter, role_perm_base(p));
            residuals.append(&mut pr);
        }

        // (c) squeeze-output equalities: role_root[j] − squeezed_output[j].
        for j in 0..N_ROLE_OUT {
            let (sp, lane) = role_output_source(j);
            let out_cell = cell(role_perm_base(sp) + C_OUTPUT + lane);
            let committed = cell(role_out_cell(j));
            residuals.push(octic_sub::<AB>(&committed, &out_cell));
        }
    }

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
        cells
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
}
