//! `recursive_air` (capstone monolithic verifier-AIR, Option A) — bus + consumers in one window.
//!
//! Per the composition decision (`research/path2_recursion_air_composition_architecture.md`,
//! Option A) the recursion is ONE wide monolithic AIR over a shared column layout that INLINES
//! every gadget's constraints and — crucially — wires the CROSS-GADGET BUSES that the standalone
//! gadget AIRs leave as free columns. The standalone gadgets each validated their own constraints
//! ([`crate::challenger`], [`crate::merkle_air`], [`crate::fri_fold_air`], [`crate::fri_chain_air`],
//! [`crate::deep_air`], [`crate::public_io_air`]) but in isolation a malicious prover can still
//! choose, independently per gadget: which query index to answer ([`crate::index_bus`] note), and
//! what fold point `s` / challenge `beta` / OOD point `zeta` to fold/open at. The monolith closes
//! those by binding the gadgets' free columns to gadget-1's Fiat–Shamir exports OVER ONE WINDOW.
//!
//! ## What this monolith wires + tests (each cross-binding tamper-rejected)
//!
//! ONE window (one row) per query. Blocks are config-gated ([`RecursiveConfig`]); absent blocks
//! cost no columns. Every block is validated honest-accept + tamper-reject via
//! `p3_air::check_all_constraints` (the rigor gate: no underconstrained binding).
//!
//!   1. **index ↔ challenger draw** (reuse [`crate::index_bus`]): the query index is the low
//!      `log_height` bits of gadget-1's `SampleDraw` export (`draw_out`), via the bit
//!      decomposition + canonical recomposition. A forged (favorable) index is rejected.
//!   2. **fold point `s` ↔ index** (product tower): `s = fold_point(log_height, index) =
//!      g^{rev_bits(index)}`, realized as a COMMITTED product tower over the index bits (degree-2
//!      steps), so `s` is pinned to the index — NOT a free column. A prover cannot fold at a
//!      favorable point.
//!   3. **FRI fold identity** (reuse [`crate::fri_fold_air`]): the cleared fold identity (★)
//!      `folded·2s == e0(beta+s) − e1(beta−s)` over the deg-8 extension at the BUS-bound `s`.
//!   4. **(a) Merkle index ↔ SAME bus** (NEW; reuse [`crate::merkle_air`]): a salted Merkle path is
//!      inlined level-by-level in the same window; each level's index-bit placement reads `bit_j`
//!      of THIS window's decomposition, so the Merkle opening and the FRI fold answer the SAME
//!      FS-forced query. Leaf/root bound by public values. Tamper: a Merkle path opened at a
//!      DIFFERENT index than the fold point is rejected ([`tests::air_rejects_merkle_at_wrong_index`]).
//!   5. **(b) beta / DEEP zeta ↔ gadget-1 SampleDraw** (NEW): a `Challenge` round challenge is 8
//!      consecutive base-field `SampleDraw` draws; the monolith carries those draw columns and pins
//!      `beta[i] == draw_i` (and likewise `zeta`). A forged/swapped challenge (≠ the transcript
//!      draw) is rejected ([`tests::air_rejects_forged_beta`], [`tests::air_rejects_forged_zeta`]).
//!   6. **(c) multi-round descent** (NEW; reuse [`crate::fri_chain_air`] chaining IN-WINDOW): the
//!      window threads `num_rounds` commit-phase folds — `s_0` tower-bound to the bus index,
//!      `s_{r+1} == s_r²` (point-square), `e0_{r+1} == folded_r` (chain-link). A broken descent
//!      link is rejected ([`tests::air_rejects_broken_descent`]).
//!   7. **(b′) DEEP OOD block** (NEW; reuse [`crate::deep_air`]): the OOD quotient relation
//!      `C(zeta) == Z_H(zeta)·Q(zeta)` at the bus-bound `zeta`, with `zeta` pinned to the challenger
//!      draws by (b). Tamper: wrong `C` rejected.
//!   8. **(a-FS) INLINED CHALLENGER SPONGE for the index draw** (NEW; `inline_sponge_draw`): the
//!      index draw `C_DRAW_OUT` — which (1) decomposes into the query index + fold point — is the
//!      CONSTRAINED output of a real Poseidon2 squeeze, NOT a free committed column. The window
//!      carries a sponge block `sponge_in[WIDTH] ‖ perm` where `perm.inputs == sponge_in`,
//!      [`crate::inlined_perm::eval_perm`] constrains the full permutation, and `C_DRAW_OUT ==
//!      perm.output[RATE-1]` (the challenger's LIFO squeeze pop). A forged draw inconsistent with
//!      the sponge state is rejected ([`tests::air_rejects_forged_sponge_draw`]). This closes the
//!      FS-soundness loop for the index challenge — bounded to ONE draw (see remaining below).
//!   9. **(b-leaf) Merkle-opened leaf ↔ FRI e0** (NEW; `bind_merkle_leaf_to_e0`): the leaf the
//!      Merkle path authenticates (level-0 running, 8 lanes) is bound limb-for-limb to the round-0
//!      `e0` (the value folded), so the opened value IS what is folded. Mismatch rejected
//!      ([`tests::air_rejects_merkle_leaf_e0_mismatch`], with an isolating control row).
//!  10. **(c-pub) public-IO statement preservation in-window** (NEW; `public_io_len`): a
//!      [`crate::public_io_air`]-style `outer == observed == inner` block, per element, so a prover
//!      cannot verify statement `P` and re-expose `P'`. Substitution rejected
//!      ([`tests::air_rejects_public_io_substitution`]).
//!
//!  11. **(a-FS+) MULTI-DRAW OFF ONE PERMUTE** (NEW; `sponge_draws`): successive base-field draws
//!      are the LIFO pops of the SAME refilled output buffer — draw `d` is bound to
//!      `perm.output[RATE-1-d]` (the challenger's `sample()` pops the back of `out_buf` WITHOUT
//!      re-permuting while it is non-empty). Draw 0 is the index draw; draws `1..sponge_draws`
//!      (≤ RATE) are carried in the sponge block's `extra_draws` columns. A forged successive draw
//!      (≠ the output lane), or extra draws collapsed onto the index draw, is rejected
//!      ([`tests::air_rejects_forged_successive_draw`],
//!      [`tests::air_rejects_extra_draws_collapsed_to_index_draw`]).
//!  12. **(a-FS++) ABSORB-PERMUTE FEEDING THE SQUEEZE** (NEW; `sponge_absorb`): `sponge_in` is no
//!      longer a free committed column — it is the CONSTRAINED output of one absorb step over a
//!      committed `prior_state` + `absorbed[RATE]` values, per [`crate::challenger`]'s `duplexing`
//!      EXACTLY: rate lanes `[0..n]` overwritten with the absorbed values, lanes `[n..RATE]` zeroed,
//!      capacity slot `RATE == prior_state[RATE] + n` (the prefix-free length tag), capacity slots
//!      `[RATE+1..WIDTH]` carried (`n` = `sponge_absorb_n`; `n==0` carries the whole state). A
//!      sponge state inconsistent with the absorbed values, a dropped length tag, or a nonzero
//!      unfilled rate lane is rejected ([`tests::air_rejects_forged_absorbed_value`],
//!      [`tests::air_rejects_forged_length_tag`], [`tests::air_rejects_nonzero_unfilled_rate_lane`]).
//!      ONE absorb→permute→multi-draw duplex step is now fully constrained in-window
//!      ([`tests::air_accepts_absorb_fed_multi_draw`]).
//!  13. **(a-FS+++) TRANSCRIPT-PREFIX ABSORB CHAIN feeding `prior_state`** (NEW; `sponge_prefix_rounds`):
//!      `prior_state` (the pre-absorb state of the final duplex step (a-FS++)) is no longer a free
//!      committed column — it is the CONSTRAINED output of a multi-round absorb-permute CHAIN from the
//!      `new()` zero boundary. Per round `k = 1..=sponge_prefix_rounds` the sponge block carries an
//!      `absorbed_block_k[RATE]` (FULL-rate absorb, `n == RATE`) ‖ a Poseidon2 perm block, and the
//!      AIR constrains `state_k == permute(absorb(state_{k-1}, block_k, RATE))` (rate overwrite +
//!      `+= RATE` length tag + capacity carry, then [`crate::inlined_perm::eval_perm`]) with
//!      `state_0` PINNED to the all-zero sponge ([`crate::challenger`] boundary) and `state_K` bound
//!      limb-for-limb into `prior_state`. Optionally (`prefix_blocks_public`) each `block_k` is
//!      pinned to public-value columns (the committed transcript = observed commitments + public IO).
//!      Tamper-rejected: a non-zero round-0 state ([`tests::air_rejects_prefix_nonzero_round0_state`]),
//!      a forged intermediate `state_k` ([`tests::air_rejects_prefix_forged_intermediate_state`]), a
//!      broken inter-round carry ([`tests::air_rejects_prefix_broken_interround_carry`]), a forged
//!      absorbed block ([`tests::air_rejects_prefix_forged_absorbed_block`]), a forged `state_K →
//!      prior_state` link ([`tests::air_rejects_prefix_forged_final_link`]), and a public-transcript
//!      mismatch ([`tests::air_prefix_blocks_bound_to_public_transcript`]). This is a BOUNDED prefix
//!      (each round absorbs RATE = 8 elements); the full ~190-elem transcript needs
//!      ⌈(8 cap + ~190 IO)/8⌉ ≈ 25 rounds (see remaining below).
//!  14. **(FS-αζ) SQUEEZE alpha + zeta** (NEW; `fs_alpha_zeta`): the DEEP/batch combination challenge
//!      `alpha` and the OOD point `zeta` are SQUEEZED from the inline duplex challenger in the EXACT
//!      p3-uni-stark order — observe(metadata ‖ main-trace commitment ‖ public values) → SAMPLE alpha
//!      → observe(quotient-chunks commitment) → SAMPLE zeta — and CONSTRAINED to equal the DEEP
//!      folder's `alpha`/`zeta` cells (no longer free). Fidelity-PINned against the native
//!      `DuplexChallenger` ([`tests::fs_squeeze_matches_native_duplex_challenger`]); wrong-order /
//!      forged / statement-decoupled challenges rejected.
//!  15. **(FS-β) SQUEEZE the FRI commit-phase betas, CONTINUING the post-zeta transcript** (NEW;
//!      `fs_beta_rounds`): the FRI folding challenges `beta_0 .. beta_{R-1}` are SQUEEZED CONTINUING
//!      from the post-zeta sponge state, in the EXACT p3-fri verifier order (p3-fri-0.6.1
//!      verifier.rs:299-312): per commit-phase round `r`, `observe(commit_phase_commit_r)` (FULL-rate
//!      absorb-permute into the carried state) → `sample_algebra_element()` = beta_r. state_0 = the
//!      quotient-observe round's perm output (sampling zeta is non-mutating), so alpha, zeta, and ALL
//!      betas come from ONE ordered sponge transcript. Each beta_r is bound limb-for-limb to the (c)
//!      descent's `round_beta(r)` cell — the FRI fold consumes the FS-squeezed challenge, REPLACING the
//!      free-column `bind_beta_to_draws` pin. Fidelity-PINned against the native `DuplexChallenger`
//!      ([`tests::fs_squeeze_betas_match_native_duplex_challenger`]). Tamper-rejected: a fold using a
//!      beta != squeezed beta_r ([`tests::air_rejects_fold_beta_not_squeezed`]), a tampered beta cell
//!      ([`tests::air_rejects_tampered_beta_cell`]), a changed fold commitment with stale betas
//!      ([`tests::air_rejects_changed_fold_commitment_keeping_stale_betas`] — comm_r change moves
//!      beta_r AND every subsequent beta), a wrong-round beta
//!      ([`tests::air_rejects_wrong_round_beta`]), a tampered perm
//!      ([`tests::air_rejects_fs_beta_tampered_perm`]), a broken inter-round carry
//!      ([`tests::air_rejects_fs_beta_broken_interround_carry`]), and a broken post-zeta continuation
//!      ([`tests::air_rejects_fs_beta_broken_postzeta_continuation`]).
//!  16. **(FS-bca) FRI BATCH-COMBINATION alpha as an intervening no-observe squeeze** (NEW;
//!      `fs_batch_combo_alpha`): p3-fri-0.6.1 verifier.rs:195 samples `alpha =
//!      challenger.sample_algebra_element()` AFTER zeta and BEFORE the first commit-phase observe,
//!      with NO observe in between. Since zeta drained the output buffer, this is a SQUEEZE-ONLY
//!      duplexing (`n == 0`: no rate overwrite, no length tag) of the post-zeta state; alpha is its
//!      top-8 LIFO pops. When enabled, the (FS-β) beta chain's state_0 CONTINUES from THIS permute
//!      output (the draw-count shift), so DEEP alpha, zeta, the batch-combo alpha, and ALL betas come
//!      from ONE ordered sponge. Fidelity-PINned against the native `DuplexChallenger`
//!      ([`tests::fs_batch_combo_alpha_and_betas_match_native_duplex_challenger`] — alpha AND every
//!      beta match AFTER the shift). Tamper-rejected: a broken squeeze input
//!      ([`tests::air_rejects_batch_combo_broken_squeeze_input`]) and an OMITTED batch-combo alpha
//!      (stale no-bca betas; [`tests::air_rejects_omitted_batch_combo_alpha`] — the shift moves every
//!      beta).
//!  17. **(FS-qi) UNIFY the FRI query-INDEX draw onto the post-last-beta transcript** (NEW;
//!      `fs_final_blocks`): p3-fri-0.6.1 verifier.rs:317-356 — after the last beta,
//!      `observe_algebra_slice(final_poly)` → `observe(log_arity)`* → `check_witness(query_pow)` → per
//!      query `index = sample_bits(...)`. The post-beta observes feed the SAME carried sponge (state_0
//!      = post-last-beta state), modeled as RATE-aligned FULL-rate observe rounds; the query index
//!      `C_DRAW_OUT` is BOUND to the top LIFO pop of the LAST observe round's perm rate — REPLACING the
//!      separate (a-FS) zero-boundary squeeze chain with a CONTINUATION of the one transcript. The (1)
//!      index↔draw decomposition then consumes THIS unified draw. Fidelity-PINned against the native
//!      `DuplexChallenger` ([`tests::fs_query_index_matches_native_duplex_challenger`]). Tamper-
//!      rejected: a changed final polynomial with a stale index
//!      ([`tests::air_rejects_changed_final_poly_keeping_stale_index`] — index bound to the final-poly
//!      transcript), a query index from a separate chain
//!      ([`tests::air_rejects_query_index_from_separate_chain`]), and a tampered perm
//!      ([`tests::air_rejects_fs_qi_tampered_perm`]).
//!
//! ## Remaining assembly toward the full monolith (documented, NOT yet wired)
//!
//! - **full challenger-sponge transcript PREFIX (BOUNDED prefix now wired)**: (a-FS)/(a-FS+)/(a-FS++)
//!   constrain ONE full duplex step — absorb(`prior_state`, `absorbed`, `n`) → permute → up to `RATE`
//!   LIFO draws. (a-FS+++) now constrains a MULTI-ROUND absorb CHAIN producing `prior_state`:
//!   `state_0 == 0` (zero boundary), `state_k == permute(absorb(state_{k-1}, block_k, RATE))`, with
//!   `state_K` bound into `prior_state`. So `prior_state` is NO LONGER a free entry point — it is the
//!   constrained output of `sponge_prefix_rounds` absorb-permute rounds from the `new()` boundary.
//!   What remains is SCALE + transcript-IDENTITY: this wires a BOUNDED prefix (tested at 1–3 rounds =
//!   8–24 absorbed elements; the absorbed blocks are bound to public values only when
//!   `prefix_blocks_public`). The FULL leaf transcript is the ~8-elem commitment caps + ~190-elem
//!   public IO + the quotient/commit-phase caps absorbed at rate 8 ⇒ ≈25+ rounds, AND those blocks
//!   must be bound to the proof's ACTUAL committed transcript columns (commitment caps + the (c-pub)
//!   public-IO block), and partial-fill / non-RATE-aligned final absorbs (the leaf's 19-elem IO
//!   leaves a non-aligned buffer — a final `n < RATE` round, the building block of which is the
//!   existing (a-FS++) `sponge_absorb_n`) must be threaded. The
//!   [`crate::challenger::air::ChallengerAir`] op-program (observe → sample sequencing, in/out buffer
//!   one-hots) is the reference for the exact absorb/sample interleaving (the prefix here assumes
//!   pure FULL-rate absorbs feeding one final draw-phase). The FRI commit-phase betas are NOW
//!   FS-squeezed and bound (item 15, `fs_beta_rounds`); `zeta` and the DEEP `alpha` are FS-squeezed
//!   (item 14). The free-column `bind_beta_to_draws` block (b) is RETAINED only for configs that do
//!   not enable item 15. One [`crate::inlined_perm::eval_perm`] per permutation.
//! - **ONE-TRANSCRIPT scope, honest flags (items 14–17)**: DEEP alpha → zeta → batch-combo alpha →
//!   beta_0 .. beta_{R-1} → query index now ALL share ONE ordered sponge state. The beta chain
//!   CONTINUES the post-zeta state (item 15), the FRI batch-combination alpha is now modeled as the
//!   intervening no-observe squeeze the betas thread from (item 16, `fs_batch_combo_alpha`), and the
//!   query-INDEX draw now CONTINUES the post-last-beta state after observing the final polynomial +
//!   log-arity schedule + query-PoW witness (item 17, `fs_final_blocks`) — REPLACING the separate
//!   (a-FS) `C_DRAW_OUT` zero-boundary squeeze chain. REMAINING within this scope: the post-beta
//!   observe stream is modeled as RATE-aligned FULL-rate observe rounds (`fs_final_blocks` blocks);
//!   the leaf's ACTUAL final_poly/log_arity/PoW-witness byte stream is presented to this AIR already
//!   RATE-chunked (the recursion picks the chunking) — a non-RATE-aligned final partial absorb
//!   (`n < RATE`, the existing (a-FS++) `sponge_absorb_n` building block) and binding these blocks to
//!   the proof's ACTUAL committed final-polynomial / grinding-witness columns (beyond the optional
//!   `fs_final_blocks_public` public-value pin) are the next refinements. Per-round commit-phase PoW
//!   (`commit_proof_of_work_bits`) and query PoW (`query_proof_of_work_bits`) are modeled as zero-bit
//!   (`check_witness(0, _)` short-circuits with NO observe / NO sample); a nonzero-bit PoW would add an
//!   `observe(witness)` before the relevant sample (a bounded extra observe round).
//! - **deferred leaf C(zeta)**: the DEEP block opens `C(zeta)` / `Q(zeta)` as committed values and
//!   constrains the QUOTIENT RELATION; the full `C(zeta) =` leaf-AIR-constraint-polynomial
//!   evaluation (re-expressing the entire leaf folder) + the DEEP linear-combination / chunk
//!   recomposition stay deferred (same boundary [`crate::deep_air`] documents).

use crate::fri_fold::{fold_pair, fold_point};
use crate::fri_fold_air::{limbs, D, W};
use crate::index_bus::{assert_canonical_pub, FIELD_BITS};
use crate::challenger::RATE;
use crate::inlined_perm::{self, eval_perm, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS};
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, SALT_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing, PrimeField32, TwoAdicField};

extern crate alloc;
use alloc::vec::Vec;

// ======================================================================================
// Config + dynamic column layout.
//
// The window grows additively with optional blocks. A `RecursiveConfig` with all blocks off
// reproduces the original single-window (index↔draw, point↔index tower, single FRI fold).
// ======================================================================================

/// Which optional cross-binding blocks this monolith window carries.
#[derive(Clone, Copy, Debug)]
pub struct RecursiveConfig {
    /// FRI leaf-layer log-height for this query (`fold_point`'s `log_height`).
    pub log_height: usize,
    /// (a) Number of Merkle path levels inlined in the window (0 = no Merkle block). Each level's
    /// index bit is bound to `bit_level` of the bus decomposition.
    pub merkle_levels: usize,
    /// (b) Bind the round-`beta` limbs to challenger `SampleDraw` columns (8 draws per challenge).
    pub bind_beta_to_draws: bool,
    /// (c) Number of commit-phase descent rounds threaded in the window (0/1 = single fold; the
    /// chaining transitions apply for r → r+1 within the window). Each extra round adds an
    /// `(e0,e1,beta,s,folded)` block; `s_0` is the tower point, `s_{r+1}==s_r²`, `e0_{r+1}==folded_r`.
    pub num_rounds: usize,
    /// (b′) DEEP OOD block: open `(zeta,Q,C)` + squaring tower at `log_degree` and check
    /// `C == Z_H(zeta)·Q`. `Some(log_degree)` enables it; `zeta` is bound to challenger draws.
    pub deep_log_degree: Option<usize>,
    /// (a-FS) INLINE THE CHALLENGER SPONGE for the index draw. When `true`, the window carries a
    /// squeeze-permute sponge block (`sponge_in[WIDTH]` ‖ inlined Poseidon2 perm) and CONSTRAINS
    /// `C_DRAW_OUT == squeeze_rate[RATE-1]` (the LIFO pop of the freshly-refilled output buffer).
    /// The `draw_out` that drives the index decomposition (1) is then the constrained output of a
    /// real Poseidon2 permutation — the Fiat–Shamir-soundness closure, bounded to ONE draw. The
    /// full multi-permute absorb transcript that produces `sponge_in` stays DEFERRED (the perm's
    /// input sponge state is supplied as committed columns here; see module-header "remaining").
    pub inline_sponge_draw: bool,
    /// (a-FS+) Number of SUCCESSIVE base-field draws popped off the SINGLE squeeze permute's output
    /// buffer (LIFO): draw `d` is bound to `perm.output[RATE-1-d]` (challenger `sample()` pops the
    /// back of the refilled `out_buf` without re-permuting while it is non-empty). Draw 0 is the
    /// index draw (`C_DRAW_OUT`); draws `1..sponge_draws` are carried in the sponge block's
    /// `extra_draws` columns and pinned to the successive output lanes. `0`/`1` = the original
    /// single index draw. Requires `inline_sponge_draw`. Bounded by `RATE` (one buffer refill).
    pub sponge_draws: usize,
    /// (a-FS++) Derive the squeeze permute's input state `sponge_in` from a CONSTRAINED absorb-
    /// permute, instead of carrying `sponge_in` as a free committed column. When `true` the sponge
    /// block additionally carries `prior_state[WIDTH]` (the pre-absorb sponge) + `absorbed[RATE]`
    /// (the rate values being absorbed) and constrains, per `challenger::duplexing` semantics, that
    /// `sponge_in == absorb(prior_state, absorbed, n)`: rate lanes `[0..n]` overwritten with the
    /// absorbed values, lanes `[n..RATE]` zeroed, capacity slot `RATE == prior_state[RATE] + n`
    /// (the length tag), capacity slots `[RATE+1..WIDTH]` copied from `prior_state`. `n =
    /// sponge_absorb_n`. This makes ONE absorb-permute feeding the index draw constrained; the
    /// transcript PREFIX producing `prior_state` (the multi-permute absorb chain over inner
    /// commitments + ~190-elem public IO) stays committed via `prior_state`. Requires
    /// `inline_sponge_draw`.
    pub sponge_absorb: bool,
    /// (a-FS++) Number of rate lanes absorbed (`n` in `challenger::duplexing`), `0..=RATE`. Only
    /// consumed when `sponge_absorb`.
    pub sponge_absorb_n: usize,
    /// (a-FS+++) Number of transcript-PREFIX absorb-permute rounds whose chained output FEEDS the
    /// duplex step's `prior_state` (closing `prior_state` from a free column to the constrained
    /// output of the preceding absorb chain). When `> 0`, the sponge block additionally carries, per
    /// round `k = 1..=sponge_prefix_rounds`, an `absorbed_block_k[RATE]` (the observed transcript
    /// values for that round, FULL-rate absorb `n == RATE`) ‖ a Poseidon2 perm block `perm_k`, and
    /// constrains `state_k == permute(absorb(state_{k-1}, block_k, RATE))` with `state_0` PINNED to
    /// the `new()` zero boundary (all-zero sponge) and `state_K` bound limb-for-limb into the duplex
    /// step's `prior_state`. Requires `inline_sponge_draw && sponge_absorb`. Each round absorbs RATE
    /// transcript elements; the FULL ~190-elem transcript needs ⌈(8 cap + ~190 IO)/8⌉ rounds — this
    /// wires a BOUNDED prefix (see module-header "remaining"). The absorbed blocks are bound to the
    /// proof's public/committed transcript columns via `prefix_blocks_public` (below).
    pub sponge_prefix_rounds: usize,
    /// (a-FS+++) bind each prefix round's `absorbed_block_k[RATE]` to public-value columns (the
    /// committed transcript). When `true` and the public-values vector is exactly
    /// `sponge_prefix_rounds * RATE` long, element `(k-1)*RATE + i` pins `absorbed_block_k[i]`. This
    /// is what ties the absorb chain to the ACTUAL observed transcript (commitments + public IO),
    /// not arbitrary prover-chosen blocks. Off ⇒ blocks are committed-but-unbound (the chain
    /// mechanics are still constrained; only the transcript-identity binding is skipped).
    pub prefix_blocks_public: bool,
    /// (c-pub) public-IO statement-preservation block: `public_io_len` element rows, each binding
    /// `outer == observed == inner`. 0 = no public-IO block.
    pub public_io_len: usize,
    /// (b-leaf) bind the Merkle-OPENED leaf digest (level-0 running, 8 lanes) to the FRI round-0
    /// `e0` limbs (8) so the value the Merkle path authenticates IS the value folded. Requires both
    /// a Merkle block (`merkle_levels > 0`) and the round-0 FRI columns. The honest builder sets the
    /// 8 leaf lanes equal to the 8 e0 limbs.
    pub bind_merkle_leaf_to_e0: bool,
    /// (FS-αζ) SQUEEZE the DEEP/batch combination challenge `alpha` and the OOD point `zeta` from
    /// the inline Fiat–Shamir challenger, in the EXACT p3-uni-stark / leaf verifier order:
    ///   observe(metadata + main-trace commitment + public values) → SAMPLE alpha →
    ///   observe(quotient-chunks commitment) → SAMPLE zeta.
    /// When `true` the window carries an FS block: `fs_pre_alpha_rounds` FULL-rate absorb-permute
    /// rounds from the `new()` zero boundary (the observed transcript prefix producing the
    /// pre-alpha state) ‖ `alpha[D]` (bound limb-for-limb to the last pre-alpha permute's LIFO
    /// squeeze pops, EXACTLY `sample_algebra_element` = 8 base draws) ‖ a quotient-observe
    /// absorb-permute round (FULL-rate absorb of the quotient commitment into the post-alpha-squeeze
    /// sponge state) ‖ `zeta[D]` (bound to that round's LIFO squeeze pops). Then `alpha`/`zeta` are
    /// CONSTRAINED to equal the DEEP folder's `deep_alpha`/`deep_zeta` cells (residual
    /// `deep_alpha − squeezed_alpha == 0`, `deep_zeta − squeezed_zeta == 0`), so they are NO LONGER
    /// free inputs. Requires `deep_log_degree` (the consumer). The honest builder DERIVES alpha/zeta
    /// from the transcript so the bus carries the genuine FS challenges.
    pub fs_alpha_zeta: bool,
    /// (FS-αζ) Number of FULL-rate (RATE=8 elements) absorb-permute rounds in the pre-alpha observed
    /// transcript prefix (metadata ‖ main-trace commitment ‖ public values), from the zero boundary.
    /// The last round's squeeze produces `alpha`. MUST be >= 1 (the native challenger always permutes
    /// before the first sample, so the alpha squeeze source is always `permute(absorb(...))`, never
    /// the un-permuted zero boundary). Only consumed when `fs_alpha_zeta`.
    pub fs_pre_alpha_rounds: usize,
    /// (FS-αζ) bind each FS round's absorbed block (the `fs_pre_alpha_rounds` pre-alpha rounds plus
    /// the single quotient-observe round) to public-value columns (the committed transcript), exactly
    /// like `prefix_blocks_public` does for the index-draw chain. When `true` and the public-values
    /// vector is exactly `(fs_pre_alpha_rounds + 1) * RATE` long, element `r*RATE + i` pins FS round
    /// `r`'s absorbed block lane `i` (rounds `0..fs_pre_alpha_rounds` are the pre-alpha prefix, round
    /// `fs_pre_alpha_rounds` is the quotient commitment). Ties the FS squeeze to the ACTUAL observed
    /// inner commitments + public digest, so a forger cannot fix alpha/zeta independent of the
    /// statement. Off ⇒ blocks committed-but-unbound (the squeeze mechanics are still constrained).
    pub fs_blocks_public: bool,
    /// (FS-β) SQUEEZE the FRI commit-phase folding challenges `beta_0 .. beta_{R-1}` CONTINUING from
    /// the post-zeta sponge state, in the EXACT p3-fri verifier order
    /// (p3-fri-0.6.1 verifier.rs:299-312): for each commit-phase round `r`,
    /// `observe(commit_phase_commit_r)` → `sample_algebra_element()` = beta_r. `R = fs_beta_rounds`.
    /// When `> 0` the window carries, AFTER the (FS-αζ) block, a per-round
    /// `block[RATE] ‖ perm[PERM_COLS] ‖ beta_r[D]` chain whose FIRST round continues from the
    /// quotient-observe round's output (the post-zeta state — sampling zeta does not mutate the
    /// sponge), each subsequent round continues from the previous beta round's output. Each beta_r is
    /// bound limb-for-limb to the (c) descent's `round_beta(r)` cell, so the FRI fold actually uses the
    /// FS-squeezed challenge (NOT a free draw column). Requires `fs_alpha_zeta` and `num_rounds >= R`.
    /// REPLACES the free-column `bind_beta_to_draws` pin for these rounds: a fold using a beta != the
    /// squeezed beta_r is rejected, and changing comm_r changes beta_r and every subsequent beta.
    pub fs_beta_rounds: usize,
    /// (FS-β) bind each FRI commit-phase round's absorbed block (`commit_phase_commit_r`) to
    /// public-value columns (the committed transcript), exactly like `fs_blocks_public` does for the
    /// alpha/zeta prefix. When `true` and the public-values vector is exactly `fs_beta_rounds * RATE`
    /// long, element `r*RATE + i` pins commit-phase round `r`'s absorbed block lane `i`. Ties the beta
    /// squeeze to the ACTUAL observed fold-round commitments. Off ⇒ blocks committed-but-unbound (the
    /// squeeze mechanics are still constrained).
    pub fs_betas_public: bool,
    /// (FS-bca) model the FRI BATCH-COMBINATION alpha (p3-fri-0.6.1 verifier.rs:195): the
    /// `sample_algebra_element()` taken AFTER zeta and BEFORE the first commit-phase observe, with NO
    /// observe in between. Because zeta drained the output buffer, this sample triggers a SQUEEZE-ONLY
    /// duplexing of the post-zeta state; the batch-combo alpha is its top-8 LIFO pops. When `true` the
    /// window carries (after the FS-αζ block, before the FS-β block) a `perm[PERM_COLS] ‖ alpha[D]`
    /// squeeze block, and the (FS-β) beta chain CONTINUES FROM THIS permute output instead of straight
    /// from the post-zeta state — the draw-count shift the betas must thread to stay native-correct.
    /// Requires `fs_alpha_zeta`. Off ⇒ betas continue straight from post-zeta (the pre-batch-alpha
    /// model). The squeezed alpha is PINned against the native challenger.
    pub fs_batch_combo_alpha: bool,
    /// (FS-qi) UNIFY the FRI query-index draw onto the post-last-beta sponge state (p3-fri-0.6.1
    /// verifier.rs:317-356): after the last beta, `observe_algebra_slice(final_poly)` →
    /// `observe(log_arity)`* → `check_witness(query_pow_bits, query_pow_witness)` → per query
    /// `index = sample_bits(...)`. When `> 0` the window carries (after the FS-β block) `fs_final_blocks`
    /// FULL-rate observe rounds (`block[RATE] ‖ perm[PERM_COLS]`) for the post-beta transcript
    /// (final_poly ‖ log_arities ‖ query-PoW witness, RATE-chunked), and the index `draw_out` (column
    /// `C_DRAW_OUT`) is BOUND to the top LIFO pop of the LAST observe round's permute rate — replacing
    /// the separate (a-FS) `C_DRAW_OUT` zero-boundary squeeze chain with a CONTINUATION of the one FS
    /// transcript. Changing the final polynomial OR the PoW witness changes the carried state and hence
    /// the index. Requires `fs_beta_rounds > 0`. Off ⇒ the index draw is its own committed chain.
    pub fs_final_blocks: usize,
    /// (FS-qi) bind each post-beta observe block (`final_poly ‖ log_arities ‖ query-PoW witness`) to
    /// public-value columns, like `fs_betas_public`. When `true` and the public-values vector is exactly
    /// `fs_final_blocks * RATE` long, element `r*RATE + i` pins the post-beta round `r`'s observed block
    /// lane `i`. Ties the query index to the ACTUAL final polynomial + grinding witness.
    pub fs_final_blocks_public: bool,
    /// (a-root) BIND the FS-index-driven main-trace Merkle opening's reconstructed root to the inner
    /// proof's MAIN-TRACE commitment, carried as a BOUND recursion input (a window column block, not a
    /// free witness). When `true` the window carries a `main_commit[DIGEST_ELEMS]` column block and
    /// CONSTRAINS the last Merkle level's reconstructed root (`output[..8]`) EQUAL to it, limb-for-limb.
    /// The constraint then reads: "the opened main-trace leaf at the FS-derived query index (the index
    /// bits the Merkle path selectors are bound to, via the (a) bus binding) authenticates to the inner
    /// main-trace root." The inner main-trace commitment is supplied by the recursion (the leaf proof's
    /// `commitments.trace` / first PCS round root); the honest builder copies it into the column. An
    /// opening that folds to a DIFFERENT root than this committed inner root rejects. Requires
    /// `merkle_levels > 0`. This is the FIRST piece of actual inner-proof opening verification driven by
    /// the FS index against the inner MAIN-TRACE commitment. Off ⇒ the root is bound only to the generic
    /// `PV_ROOT` public value (the prior mechanism).
    pub bind_root_to_main_commit: bool,
    /// (LO) COMPOSE the standalone-proven per-round FRI-LAYER opening into the (c) descent: for each
    /// commit-phase round `r = 0..layer_open_rounds`, the window carries that round's FRI-layer
    /// commitment `layer_commit_r[DIGEST_ELEMS]` (a BOUND recursion input, like `inner_main_commit`)
    /// plus the full [`crate::fri_layer_air::FriLayerOpenAir`] residual — flattened arity-2 pair,
    /// salt, the `MyHash(pair ‖ salt)` leaf-hash perms, and `layer_open_levels` Merkle path levels —
    /// authenticating round `r`'s opened SIBLING at the FS query index shifted by `r`
    /// (`folded_index_r = FS_index >> r`; per-level selectors bound to FS-index bits) against
    /// `layer_commit_r`. The authenticated `e_fold` is bound to the descent's `round_e0(r)` and the
    /// authenticated `e_sib` to `round_e1(r)`, so the FRI fold consumes the AUTHENTICATED sibling —
    /// not a free column. A descent sibling != the authenticated layer-open output rejects; a
    /// `layer_commit_r` != the real round-r commitment rejects; cross-round confusion (round `r`'s
    /// sibling/commit used in round `r'`) rejects. 0 = off. Requires `num_rounds >= layer_open_rounds`.
    pub layer_open_rounds: usize,
    /// (LO) Number of Merkle path levels per per-round layer open (the FRI-layer tree depth). Each
    /// level's selector bit is bound to FS-index bit `r+1+level`. Only consumed when
    /// `layer_open_rounds > 0`.
    pub layer_open_levels: usize,
    /// (FP) ENFORCE the FRI final-polynomial check INSIDE the descent: bind the fully-folded value
    /// after the last commit-phase round (`round_folded(rounds-1)`) to a committed final polynomial
    /// evaluated at a query point `x` (p3-fri verifier.rs:402-409, `folded == Σ coeff·x^k`). When
    /// `> 0` it is the final-poly coefficient count (`fp_len`); the window carries `x ‖ coeffs[fp_len·D]`
    /// and CONSTRAINS `round_folded(last) == Horner(coeffs, x)`. A folded value != final_poly(x), or a
    /// tampered final-poly coefficient, rejects. 0 = off.
    pub final_poly_len: usize,
    /// (b-hash) DERIVE the Merkle level-0 leaf digest IN-AIR as `MyHash(opened_row ‖ salt)` via the
    /// inlined Poseidon2 `PaddingFreeSponge` (`config::MyHash = PaddingFreeSponge<Perm,16,8,8>`),
    /// instead of accepting the level-0 running as a free supplied digest. When `> 0` it is the
    /// length of the opened main-trace ROW (in `Val` elements); the window carries a leaf-hash block
    /// (the `leaf_row_len` row cells ‖ `SALT_ELEMS` salt cells ‖ one Poseidon2 perm per sponge
    /// absorb-permute) and CONSTRAINS the sponge output digest (8 lanes) EQUAL to the Merkle level-0
    /// running, limb-for-limb. The sponge is the deployed `MerkleTreeHidingMmcs` leaf hashing
    /// EXACTLY: state starts all-zero, input is `row ‖ salt` (the hiding MMCS appends `SALT_ELEMS`
    /// salt elems after the row), absorbed in RATE=8 overwrite blocks, permuted after each full
    /// block, with a final permute for a trailing partial block. Requires `merkle_levels > 0`. With
    /// `bind_merkle_leaf_to_e0` this means the value FRI folds (e0) is `MyHash(row)` of the opened
    /// row — the Merkle path authenticates `H(row)`, not a free digest. 0 = off (the level-0 running
    /// is a supplied digest, the prior mechanism). PINned against a real `MerkleTreeMmcs` leaf.
    pub leaf_row_len: usize,
}

impl RecursiveConfig {
    /// The original single-window config (no Merkle, no beta-draw binding, single round, no DEEP).
    pub fn single_window(log_height: usize) -> Self {
        Self {
            log_height,
            merkle_levels: 0,
            bind_beta_to_draws: false,
            num_rounds: 1,
            deep_log_degree: None,
            inline_sponge_draw: false,
            sponge_draws: 1,
            sponge_absorb: false,
            sponge_absorb_n: 0,
            sponge_prefix_rounds: 0,
            prefix_blocks_public: false,
            public_io_len: 0,
            bind_merkle_leaf_to_e0: false,
            fs_alpha_zeta: false,
            fs_pre_alpha_rounds: 0,
            fs_blocks_public: false,
            fs_beta_rounds: 0,
            fs_betas_public: false,
            fs_batch_combo_alpha: false,
            fs_final_blocks: 0,
            fs_final_blocks_public: false,
            bind_root_to_main_commit: false,
            layer_open_rounds: 0,
            layer_open_levels: 0,
            final_poly_len: 0,
            leaf_row_len: 0,
        }
    }
    /// Number of successive squeeze draws actually laid out (≥ 1, ≤ RATE).
    pub fn sponge_draw_count(&self) -> usize {
        self.sponge_draws.max(1)
    }
    /// Number of descent rounds actually laid out (≥ 1).
    pub fn rounds(&self) -> usize {
        self.num_rounds.max(1)
    }
}

// ----- shared monolithic column layout (one query window per row) -----
//
// Index-bus block (gadget-1 export + decomposition):
/// Gadget-1 `SampleDraw` export: the base-field element this query index reduces.
pub const C_DRAW_OUT: usize = 0;
/// Full canonical bit decomposition of `draw_out` (LSB first), `FIELD_BITS` columns.
pub const C_BITS: usize = C_DRAW_OUT + 1;
/// The bus index value (`Σ_{i<log_height} bit_i·2^i`) the consumers read.
pub const C_INDEX: usize = C_BITS + FIELD_BITS;
// FRI-fold block (gadget-3 consumer), round 0:
/// First sibling eval `e0 = f(s)` (8 limbs).
pub const C_E0: usize = C_INDEX + 1;
/// Second sibling eval `e1 = f(-s)` (8 limbs).
pub const C_E1: usize = C_E0 + D;
/// Round fold challenge `beta` (8 limbs).
pub const C_BETA: usize = C_E1 + D;
/// The fold point `s` — BUS-BOUND to the index by the product tower (not free).
pub const C_S: usize = C_BETA + D;
/// The claimed folded eval (8 limbs).
pub const C_FOLDED: usize = C_S + 1;
/// Fold-point product tower `sp[0..=log_height]`: `sp[0]=1`, `sp[j+1]=sp[j]·sel_j`,
/// `sp[log_height]==s`. `(log_height+1)` columns. (The committed-tower trick; see module doc.)
pub const C_SP: usize = C_FOLDED + D;

/// Per-round FRI block size (e0,e1,beta,s,folded) = 4·D + 1.
pub const ROUND_BLOCK: usize = 4 * D + 1;

/// End of the round-0 + tower block (start of the appended extra-round / DEEP / draw / Merkle
/// blocks). The single-fold columns above (C_E0..C_SP) are round 0; extra rounds for the (c)
/// descent are appended after the tower.
const fn tower_end(log_height: usize) -> usize {
    C_SP + (log_height + 1)
}

/// Base of extra descent round `r` (r ≥ 1): each is an `(e0,e1,beta,s,folded)` block.
fn extra_round_base(cfg: &RecursiveConfig, r: usize) -> usize {
    debug_assert!(r >= 1);
    tower_end(cfg.log_height) + (r - 1) * ROUND_BLOCK
}

/// Base of the DEEP OOD block (zeta[D], Q[D], C[D], pow[(k+1)·D]) — present iff `deep_log_degree`.
fn deep_base(cfg: &RecursiveConfig) -> usize {
    tower_end(cfg.log_height) + cfg.rounds().saturating_sub(1) * ROUND_BLOCK
}

/// DEEP block sub-offsets relative to `deep_base`.
const DEEP_ZETA: usize = 0;
const DEEP_Q: usize = DEEP_ZETA + D;
const DEEP_C: usize = DEEP_Q + D;
/// The DEEP/batch combination challenge `alpha` (8 limbs) the leaf folder uses to batch the
/// constraint polynomials. A committed cell here; the (FS-αζ) block CONSTRAINS it to equal the
/// FS-squeezed alpha so it is not free.
const DEEP_ALPHA: usize = DEEP_C + D;
const DEEP_POW: usize = DEEP_ALPHA + D; // pow[0..=k]

fn deep_size(log_degree: usize) -> usize {
    DEEP_POW + (log_degree + 1) * D
}

/// Base of the (b) challenger-draw block: 8 draw columns per challenge bound. We bind round betas
/// (one per round) and optionally zeta. `draws_count` challenges × D columns.
fn draw_block_base(cfg: &RecursiveConfig) -> usize {
    let mut base = deep_base(cfg);
    if let Some(k) = cfg.deep_log_degree {
        base += deep_size(k);
    }
    base
}

/// Number of challenges whose limbs are bound to draw columns: one beta per round (+ zeta if DEEP),
/// iff `bind_beta_to_draws`.
fn draw_challenge_count(cfg: &RecursiveConfig) -> usize {
    if !cfg.bind_beta_to_draws {
        return 0;
    }
    cfg.rounds() + if cfg.deep_log_degree.is_some() { 1 } else { 0 }
}

/// Base of the (a) Merkle block: `merkle_levels` levels, each `running[8] ‖ sibling[8] ‖ perm[164]`.
fn merkle_base(cfg: &RecursiveConfig) -> usize {
    draw_block_base(cfg) + draw_challenge_count(cfg) * D
}

/// Per-level Merkle sub-block size.
pub const MERKLE_LEVEL: usize = 2 * DIGEST_ELEMS + PERM_COLS;
/// Sub-offsets within a Merkle level block.
const ML_RUNNING: usize = 0;
const ML_SIBLING: usize = DIGEST_ELEMS;
const ML_PERM: usize = 2 * DIGEST_ELEMS;

fn merkle_level_base(cfg: &RecursiveConfig, level: usize) -> usize {
    merkle_base(cfg) + level * MERKLE_LEVEL
}

/// End of the Merkle block (base of the appended sponge / public-IO blocks).
fn merkle_end(cfg: &RecursiveConfig) -> usize {
    merkle_base(cfg) + cfg.merkle_levels * MERKLE_LEVEL
}

// ----- (a-FS) inlined challenger-sponge squeeze block -----
//
// `sponge_in[WIDTH]` ‖ inlined Poseidon2 perm block (PERM_COLS). The perm's inputs[..WIDTH] are
// bound to `sponge_in`; `eval_perm` constrains the whole permutation; the post-permute rate lanes
// (perm output[0..RATE]) are the refilled output buffer; the index draw `C_DRAW_OUT` is bound to
// the LIFO pop `output[RATE-1]`.
/// Sub-offset of `sponge_in` within the sponge block.
const SP_SPONGE_IN: usize = 0;
/// Sub-offset of the inlined perm block within the sponge block.
const SP_PERM: usize = SP_SPONGE_IN + WIDTH;
/// Base sponge-block columns (sponge_in ‖ perm). Extra-draw / absorb columns are appended after.
pub const SPONGE_BLOCK: usize = SP_PERM + PERM_COLS;
/// (a-FS+) Sub-offset of the `extra_draws` columns (draws 1..sponge_draws), appended after the
/// base sponge block. `sponge_draws.max(1) - 1` columns.
const SP_EXTRA_DRAWS: usize = SPONGE_BLOCK;
/// (a-FS++) Sub-offset of the absorb sub-block (`prior_state[WIDTH] ‖ absorbed[RATE]`), appended
/// after the extra-draw columns. Present iff `sponge_absorb`.
fn sp_absorb(cfg: &RecursiveConfig) -> usize {
    SP_EXTRA_DRAWS + (cfg.sponge_draw_count() - 1)
}
const SPA_PRIOR_STATE: usize = 0;
const SPA_ABSORBED: usize = SPA_PRIOR_STATE + WIDTH;
const ABSORB_SUBBLOCK: usize = SPA_ABSORBED + RATE;

// ----- (a-FS+++) transcript-prefix absorb chain sub-block -----
//
// Appended AFTER the absorb sub-block. Per round k = 1..=sponge_prefix_rounds: an
// `absorbed_block_k[RATE]` (the observed transcript values absorbed that round) ‖ a Poseidon2 perm
// block `perm_k[PERM_COLS]` whose input is `absorb(state_{k-1}, block_k, RATE)` and whose output is
// `state_k`. `state_0` is the all-zero `new()` boundary; `state_K` feeds `prior_state`.
const SPP_BLOCK: usize = 0; // absorbed_block_k[RATE]
const SPP_PERM: usize = SPP_BLOCK + RATE; // perm_k[PERM_COLS]
const PREFIX_ROUND: usize = SPP_PERM + PERM_COLS;

/// Sub-offset of the prefix-chain sub-block within the sponge block (after the absorb sub-block).
/// Only meaningful when `sponge_absorb && sponge_prefix_rounds > 0`.
fn sp_prefix(cfg: &RecursiveConfig) -> usize {
    sp_absorb(cfg) + ABSORB_SUBBLOCK
}

/// Sub-offset of prefix round `k` (1-based) within the sponge block.
fn prefix_round_base(cfg: &RecursiveConfig, k: usize) -> usize {
    debug_assert!(k >= 1);
    sp_prefix(cfg) + (k - 1) * PREFIX_ROUND
}

/// Base of the (a-FS) sponge block (present iff `inline_sponge_draw`).
fn sponge_base(cfg: &RecursiveConfig) -> usize {
    merkle_end(cfg)
}

fn sponge_size(cfg: &RecursiveConfig) -> usize {
    if cfg.inline_sponge_draw {
        let mut sz = SPONGE_BLOCK + (cfg.sponge_draw_count() - 1);
        if cfg.sponge_absorb {
            sz += ABSORB_SUBBLOCK;
            sz += cfg.sponge_prefix_rounds * PREFIX_ROUND;
        }
        sz
    } else {
        0
    }
}

// ----- (c-pub) public-IO statement-preservation block -----
//
// `public_io_len` element rows × (outer, observed, inner) = 3·public_io_len columns, laid flat in
// the single window. Each element binds outer == observed == inner.
const PIO_STRIDE: usize = 3;
const PIO_OUTER: usize = 0;
const PIO_OBSERVED: usize = 1;
const PIO_INNER: usize = 2;

/// Base of the public-IO block.
fn public_io_base(cfg: &RecursiveConfig) -> usize {
    sponge_base(cfg) + sponge_size(cfg)
}

// ----- (FS-αζ) alpha/zeta Fiat–Shamir squeeze block -----
//
// Layout (appended after the public-IO block):
//   [pre-alpha round 0]  block[RATE] ‖ perm[PERM_COLS]
//   ...
//   [pre-alpha round R-1] block[RATE] ‖ perm[PERM_COLS]      (R = fs_pre_alpha_rounds)
//   alpha[D]                                                  (the SAMPLE alpha)
//   [quotient round]      block[RATE] ‖ perm[PERM_COLS]       (observe quotient commitment)
//   zeta[D]                                                   (the SAMPLE zeta)
//
// Each round is a FULL-rate (n == RATE) absorb-permute, EXACTLY like the index-draw prefix chain
// (`prefix_round_state`). `state_0 == ZERO` (new() boundary); `state_k == permute(absorb(...))`.
// alpha is the LIFO squeeze of the last pre-alpha round's permute (8 base draws =
// `sample_algebra_element`); the quotient round absorbs into THAT permute's output sponge state
// (sampling does not change sponge_state); zeta is the LIFO squeeze of the quotient round's permute.

/// One FS round sub-block: `block[RATE] ‖ perm[PERM_COLS]` (reuses the prefix-chain shape).
const FS_BLOCK: usize = 0;
const FS_PERM: usize = FS_BLOCK + RATE;
const FS_ROUND: usize = FS_PERM + PERM_COLS;

/// Base of the (FS-αζ) block.
fn fs_base(cfg: &RecursiveConfig) -> usize {
    public_io_base(cfg) + cfg.public_io_len * PIO_STRIDE
}

/// Total FS rounds laid out: `fs_pre_alpha_rounds` pre-alpha rounds + 1 quotient-observe round.
fn fs_total_rounds(cfg: &RecursiveConfig) -> usize {
    cfg.fs_pre_alpha_rounds + 1
}

/// Sub-offset of FS round `r` (0-based: rounds `0..fs_pre_alpha_rounds` are the pre-alpha prefix,
/// round `fs_pre_alpha_rounds` is the quotient-observe round) within the FS block.
fn fs_round_base(cfg: &RecursiveConfig, r: usize) -> usize {
    fs_base(cfg) + r * FS_ROUND
}

/// Sub-offset of the `alpha[D]` cell (after the pre-alpha rounds, before the quotient round).
fn fs_alpha(cfg: &RecursiveConfig) -> usize {
    fs_base(cfg) + cfg.fs_pre_alpha_rounds * FS_ROUND
}

/// Sub-offset of the quotient-observe round (after the alpha cell).
fn fs_quot_round_base(cfg: &RecursiveConfig) -> usize {
    fs_alpha(cfg) + D
}

/// Sub-offset of the `zeta[D]` cell (after the quotient round).
fn fs_zeta(cfg: &RecursiveConfig) -> usize {
    fs_quot_round_base(cfg) + FS_ROUND
}

/// Size of the (FS-αζ) block: (R+1) rounds + alpha[D] + zeta[D].
fn fs_size(cfg: &RecursiveConfig) -> usize {
    if cfg.fs_alpha_zeta {
        fs_total_rounds(cfg) * FS_ROUND + 2 * D
    } else {
        0
    }
}

// ----- (FS-bca) FRI batch-combination alpha squeeze block -----
//
// Appended AFTER the (FS-αζ) block, BEFORE the (FS-β) block. A SINGLE squeeze-only permute (n == 0):
//   perm[PERM_COLS] ‖ alpha[D]
// `perm`'s input is the post-zeta state UNCHANGED (no rate overwrite, no length tag — a squeeze, not
// an absorb); alpha is the LIFO squeeze of `perm`'s output rate. The (FS-β) beta chain's state_0 is
// THIS perm output (not the post-zeta state) — the draw-count shift.
const BCA_PERM: usize = 0;
const BCA_ALPHA: usize = BCA_PERM + PERM_COLS;
const BCA_SIZE: usize = BCA_ALPHA + D;

/// Base of the (FS-bca) block (after the FS-αζ block).
fn bca_base(cfg: &RecursiveConfig) -> usize {
    fs_base(cfg) + fs_size(cfg)
}

/// Size of the (FS-bca) block.
fn bca_size(cfg: &RecursiveConfig) -> usize {
    if cfg.fs_batch_combo_alpha {
        BCA_SIZE
    } else {
        0
    }
}

// ----- (FS-β) FRI commit-phase beta squeeze block -----
//
// Appended AFTER the (FS-bca) block. Per commit-phase round `r = 0..fs_beta_rounds`:
//   block[RATE] ‖ perm[PERM_COLS] ‖ beta_r[D]
// `block` is the observed commit-phase commitment; `perm`'s input is absorb(state_r, block, RATE)
// where state_0 is the post-BATCH-COMBO-ALPHA state (when `fs_batch_combo_alpha`) or the post-zeta
// state (otherwise) and state_{r+1} is round r's perm output; beta_r is the LIFO squeeze of round r's
// perm output rate. Each beta_r is bound to the (c) descent round_beta(r) cell.
const FB_BLOCK: usize = 0;
const FB_PERM: usize = FB_BLOCK + RATE;
const FB_BETA: usize = FB_PERM + PERM_COLS;
const FB_ROUND: usize = FB_BETA + D;

/// Base of the (FS-β) block (after the FS-αζ and FS-bca blocks).
fn fb_base(cfg: &RecursiveConfig) -> usize {
    bca_base(cfg) + bca_size(cfg)
}

/// Sub-offset of (FS-β) round `r` within the window.
fn fb_round_base(cfg: &RecursiveConfig, r: usize) -> usize {
    fb_base(cfg) + r * FB_ROUND
}

/// Size of the (FS-β) block: `fs_beta_rounds` rounds.
fn fb_size(cfg: &RecursiveConfig) -> usize {
    if cfg.fs_beta_rounds > 0 {
        cfg.fs_beta_rounds * FB_ROUND
    } else {
        0
    }
}

// ----- (FS-qi) FRI query-index draw block -----
//
// Appended AFTER the (FS-β) block. Per post-beta observe round `r = 0..fs_final_blocks`:
//   block[RATE] ‖ perm[PERM_COLS]
// `block` is a RATE-chunk of the post-beta transcript (final_poly ‖ log_arities ‖ query-PoW witness);
// `perm`'s input is absorb(state_r, block, RATE) where state_0 is the post-last-beta state and
// state_{r+1} is round r's perm output. The query index draw (`C_DRAW_OUT`) is bound to the top LIFO
// pop of the LAST round's perm output rate.
const FQ_BLOCK: usize = 0;
const FQ_PERM: usize = FQ_BLOCK + RATE;
const FQ_ROUND: usize = FQ_PERM + PERM_COLS;

/// Base of the (FS-qi) block (after the FS-β block).
fn fq_base(cfg: &RecursiveConfig) -> usize {
    fb_base(cfg) + fb_size(cfg)
}

/// Sub-offset of (FS-qi) round `r` within the window.
fn fq_round_base(cfg: &RecursiveConfig, r: usize) -> usize {
    fq_base(cfg) + r * FQ_ROUND
}

/// Size of the (FS-qi) block: `fs_final_blocks` rounds.
fn fq_size(cfg: &RecursiveConfig) -> usize {
    if cfg.fs_final_blocks > 0 {
        cfg.fs_final_blocks * FQ_ROUND
    } else {
        0
    }
}

// ----- (a-root) inner main-trace commitment block -----
//
// Appended AFTER the (FS-qi) block. A single `main_commit[DIGEST_ELEMS]` column block carrying the
// inner proof's MAIN-TRACE commitment as a BOUND recursion input. The eval constrains the last
// Merkle level's reconstructed root EQUAL to this block, limb-for-limb (only when `merkle_levels > 0`).
const MC_MAIN_COMMIT: usize = 0;
const MC_SIZE: usize = MC_MAIN_COMMIT + DIGEST_ELEMS;

/// Base of the (a-root) main-trace commitment block (after the FS-qi block).
fn mc_base(cfg: &RecursiveConfig) -> usize {
    fq_base(cfg) + fq_size(cfg)
}

/// Size of the (a-root) block: `DIGEST_ELEMS` columns iff `bind_root_to_main_commit`.
fn mc_size(cfg: &RecursiveConfig) -> usize {
    if cfg.bind_root_to_main_commit {
        MC_SIZE
    } else {
        0
    }
}

// ----- (LO) per-round FRI commit-phase LAYER-open block -----
//
// Appended AFTER the (a-root) block. Per commit-phase round `r = 0..layer_open_rounds`, a layer-open
// sub-block authenticates round r's opened SIBLING against round r's FRI-layer commitment, then binds
// the authenticated pair (e_fold, e_sib) to the descent's (round_e0(r), round_e1(r)). The residual
// re-expresses [`crate::fri_layer_air::FriLayerOpenAir`] INSIDE this window (binding to the descent's
// existing cells), constant-for-constant.
//
// Per-round sub-block layout:
//   layer_commit[DIGEST_ELEMS]          the round-r FRI-layer commitment (the bound root)
//   evals[2·D]                          the flattened arity-2 pair [evals0; D] ‖ [evals1; D]
//   salt[SALT_ELEMS]                    the hiding-MMCS salt of the opened leaf row
//   leaf_perm_0[PERM_COLS] .. _{P-1}    the MyHash(evals ‖ salt) PaddingFreeSponge perms
//   level_0 .. level_{L-1}              the Merkle path levels (running[8] ‖ sibling[8] ‖ perm)
//
// The parity (in-group bit) and per-level node selectors are NOT free: they are read directly from
// the bus index bits — parity = bits[r], level-j selector = bits[r+1+j] (the folded-node index of
// round r is FS_index >> (r+1)). So the layer open is presented at the FS-forced node, not a chosen one.
/// `evals[0]` limbs offset (start of the flattened pair).
const LO_EVAL0: usize = DIGEST_ELEMS;
/// `evals[1]` limbs offset.
const LO_EVAL1: usize = LO_EVAL0 + D;
/// salt offset (after the flattened pair).
const LO_SALT: usize = LO_EVAL1 + D;
/// start of the leaf-hash perm blocks (after the salt).
const LO_LEAF_PERMS: usize = LO_SALT + SALT_ELEMS;

/// The salted leaf-row input length for a layer leaf: `2·D` flattened pair + `SALT_ELEMS` salt.
const LO_LEAF_INPUT_LEN: usize = 2 * D + SALT_ELEMS;

/// Number of `PaddingFreeSponge` absorb-permutes over the `2·D + SALT_ELEMS` salted layer-leaf input
/// (mirrors [`lh_perms`] / `leaf_hash_pre_states`).
const fn lo_leaf_perms() -> usize {
    let m = LO_LEAF_INPUT_LEN;
    let c = m.div_ceil(RATE);
    if c == 0 { 1 } else { c }
}

/// Offset of leaf-hash perm block `k` within a layer-open sub-block.
const fn lo_leaf_perm_base(k: usize) -> usize {
    LO_LEAF_PERMS + k * PERM_COLS
}

/// Offset of the Merkle path level blocks within a layer-open sub-block.
const fn lo_merkle_base() -> usize {
    lo_leaf_perm_base(lo_leaf_perms())
}

/// Per-level Merkle sub-block within a layer open: `running[8] ‖ sibling[8] ‖ perm`. (No per-level
/// bit column — the selector is read from the bus index bits, not stored.)
const LO_ML_RUNNING: usize = 0;
const LO_ML_SIBLING: usize = DIGEST_ELEMS;
const LO_ML_PERM: usize = 2 * DIGEST_ELEMS;
const LO_MERKLE_LEVEL: usize = LO_ML_PERM + PERM_COLS;

/// Size of ONE per-round layer-open sub-block for `levels` path levels.
fn lo_round_size(levels: usize) -> usize {
    lo_merkle_base() + levels * LO_MERKLE_LEVEL
}

/// Base of the (LO) block (after the a-root block).
fn lo_base(cfg: &RecursiveConfig) -> usize {
    mc_base(cfg) + mc_size(cfg)
}

/// Base of layer-open round `r`'s sub-block within the window.
fn lo_round_base(cfg: &RecursiveConfig, r: usize) -> usize {
    lo_base(cfg) + r * lo_round_size(cfg.layer_open_levels)
}

/// Offset of layer-open round `r`'s Merkle level `level`.
fn lo_level_base(cfg: &RecursiveConfig, r: usize, level: usize) -> usize {
    lo_round_base(cfg, r) + lo_merkle_base() + level * LO_MERKLE_LEVEL
}

/// Size of the (LO) block: `layer_open_rounds` sub-blocks. 0 unless enabled.
fn lo_size(cfg: &RecursiveConfig) -> usize {
    if cfg.layer_open_rounds > 0 {
        cfg.layer_open_rounds * lo_round_size(cfg.layer_open_levels)
    } else {
        0
    }
}

// ----- (FP) final-polynomial check block -----
//
// Appended AFTER the (LO) block. `x ‖ coeffs[final_poly_len·D]`. CONSTRAINS the descent's
// round_folded(rounds-1) == Horner(coeffs, x) (p3-fri verifier.rs:402-409).
const FP_X: usize = 0;
const FP_COEFFS: usize = FP_X + 1;

fn fp_coeff_base(c: usize) -> usize {
    FP_COEFFS + c * D
}

/// Base of the (FP) final-poly block (after the LO block).
fn fp_base(cfg: &RecursiveConfig) -> usize {
    lo_base(cfg) + lo_size(cfg)
}

/// Size of the (FP) block: `x` + `final_poly_len·D` coeff columns. 0 unless enabled.
fn fp_size(cfg: &RecursiveConfig) -> usize {
    if cfg.final_poly_len > 0 {
        1 + cfg.final_poly_len * D
    } else {
        0
    }
}

/// The flattened arity-2 leaf row `[evals0; D] ‖ [evals1; D]` placed by parity: `evals[parity] =
/// e_fold`, the other slot = `e_sib`. Mirrors [`crate::fri_layer_air::flatten_pair`].
fn lo_flatten_pair(e_fold: Challenge, e_sib: Challenge, parity: bool) -> [Val; 2 * D] {
    let (e0, e1) = if parity { (e_sib, e_fold) } else { (e_fold, e_sib) };
    let mut row = [Val::ZERO; 2 * D];
    row[..D].copy_from_slice(e0.as_basis_coefficients_slice());
    row[D..].copy_from_slice(e1.as_basis_coefficients_slice());
    row
}

/// The salted leaf-hash input `flatten(evals) ‖ salt` for a layer leaf.
fn lo_leaf_input(e_fold: Challenge, e_sib: Challenge, parity: bool, salt: &[Val; SALT_ELEMS]) -> Vec<Val> {
    let mut v = lo_flatten_pair(e_fold, e_sib, parity).to_vec();
    v.extend_from_slice(salt);
    v
}

/// The layer leaf digest `MyHash(flatten(evals) ‖ salt)` — the value the Merkle path authenticates.
pub fn layer_leaf_digest(e_fold: Challenge, e_sib: Challenge, parity: bool, salt: &[Val; SALT_ELEMS]) -> [Val; DIGEST_ELEMS] {
    leaf_hash_digest(&lo_leaf_input(e_fold, e_sib, parity, salt))
}

/// Reference Horner eval of a committed final poly at base point `x` (p3-fri verifier.rs:404-405).
pub fn final_poly_eval(x: Val, coeffs: &[Challenge]) -> Challenge {
    let mut eval = Challenge::ZERO;
    for &coeff in coeffs.iter().rev() {
        eval = eval * x + coeff;
    }
    eval
}

/// The (alpha, zeta) the inline FS challenger squeezes for `fs_blocks` (the observed transcript:
/// `fs_pre_alpha_rounds` pre-alpha FULL-rate blocks + 1 quotient-commitment block), in p3 order:
/// observe pre-alpha prefix → SAMPLE alpha → observe quotient → SAMPLE zeta. The single source the
/// honest builder DERIVES alpha/zeta from and the (FS-αζ) AIR binding is checked against. The 8
/// `sample()` calls of `sample_algebra_element` map to LIFO pops `output[RATE-1-i]` (challenge limb
/// `i` = `sample()` call `i`).
pub fn fs_squeeze_alpha_zeta(
    pre_alpha_blocks: &[[Val; RATE]],
    quot_block: [Val; RATE],
) -> (Challenge, Challenge) {
    // pre_alpha_blocks must be non-empty: the native DuplexChallenger always permutes before the
    // first sample (an empty output buffer triggers a duplexing), so the alpha squeeze source is
    // ALWAYS permute(absorb(...)) — never the un-permuted zero boundary. The real leaf transcript
    // (metadata ‖ trace commitment ‖ public values) guarantees >= 1 full-rate round before alpha.
    assert!(
        !pre_alpha_blocks.is_empty(),
        "fs_squeeze requires >= 1 pre-alpha round (the native challenger permutes before sampling)"
    );
    // pre-alpha absorb chain from the zero boundary.
    let pre_state = prefix_chain_state(pre_alpha_blocks);
    // alpha = squeeze of the last pre-alpha permute = the rate of pre_state (LIFO pops).
    let alpha = Challenge::from_basis_coefficients_fn(|i| pre_state[RATE - 1 - i]);
    // observe(quotient): one FULL-rate absorb-permute into pre_state (sampling did not change it).
    let post_state = prefix_round_state(pre_state, quot_block);
    let zeta = Challenge::from_basis_coefficients_fn(|i| post_state[RATE - 1 - i]);
    (alpha, zeta)
}

/// The post-zeta sponge state — the SHARED continuation point for the FRI commit-phase beta squeeze
/// (and, ultimately, the query-index draw). After observing the quotient commitment and SAMPLING
/// zeta (which does NOT mutate the sponge state in a duplex challenger), the sponge state is exactly
/// the quotient-observe round's permute output. This is the single source the honest builder and the
/// (FS-β) AIR binding both continue from, so betas are squeezed from ONE ordered transcript that
/// already includes alpha + zeta.
pub fn fs_post_zeta_state(
    pre_alpha_blocks: &[[Val; RATE]],
    quot_block: [Val; RATE],
) -> [Val; WIDTH] {
    assert!(
        !pre_alpha_blocks.is_empty(),
        "fs_post_zeta_state requires >= 1 pre-alpha round"
    );
    let pre_state = prefix_chain_state(pre_alpha_blocks);
    prefix_round_state(pre_state, quot_block)
}

/// A SQUEEZE-ONLY duplexing permute (the native `duplexing()` with `num_absorbed == 0`): NO rate
/// lane is overwritten and NO length tag is added (matching `duplexing`'s `if num_absorbed > 0`
/// guard), the whole state is carried into the permutation. The native challenger performs exactly
/// this whenever a `sample()` finds an EMPTY output buffer and an EMPTY input buffer — the case for
/// the FRI batch-combination alpha (verifier.rs:195), which is a `sample_algebra_element()` taken
/// after zeta drained the output buffer, with NO intervening observe. Returns the post-permute state.
pub fn squeeze_permute_state(state: [Val; WIDTH]) -> [Val; WIDTH] {
    let pre = absorb_state(state, [Val::ZERO; RATE], 0); // n == 0: state carried unchanged.
    let consts = inlined_perm::Constants::pinned();
    let cols = inlined_perm::generate_perm_cols(pre, &consts);
    inlined_perm::perm_output(&cols)
}

/// The FRI batch-combination alpha (p3-fri-0.6.1 verifier.rs:195) and the post-alpha sponge state,
/// CONTINUING from the post-zeta state. The PCS samples zeta (8 LIFO pops, draining the output
/// buffer), then the FRI verifier IMMEDIATELY samples `alpha = challenger.sample_algebra_element()`
/// with NO observe in between. Because the output buffer is empty (zeta drained it) AND the input
/// buffer is empty (zeta drained that on its first pop), this `sample()` triggers a SQUEEZE-ONLY
/// duplexing of the post-zeta state; the batch-combo alpha is the 8 LIFO pops of THAT permute's rate.
/// The returned state is `permute(post_zeta_state)` — the new continuation point for the betas (the
/// "draw-count shift" the betas must thread, since beta_0 now absorbs into THIS state, not post-zeta).
pub fn fs_squeeze_batch_combo_alpha(post_zeta_state: [Val; WIDTH]) -> (Challenge, [Val; WIDTH]) {
    let state = squeeze_permute_state(post_zeta_state);
    let alpha = Challenge::from_basis_coefficients_fn(|i| state[RATE - 1 - i]);
    (alpha, state)
}

/// SQUEEZE the FRI commit-phase folding challenges `beta_0 .. beta_{R-1}` CONTINUING from
/// `start_state`, in the EXACT p3-fri verifier order (p3-fri-0.6.1 verifier.rs:299-312): for each
/// commit-phase round `r`: `observe(commit_phase_commit_r)` (one FULL-rate absorb-permute into the
/// carried sponge state) → `sample_algebra_element()` (LIFO 8-base-draw squeeze = beta_r). The
/// carried sponge state threads round → round (state_{r+1} = permute(absorb(state_r, comm_r))), so
/// changing comm_r changes beta_r AND every subsequent beta.
///
/// `start_state` is the post-BATCH-COMBO-ALPHA state (`fs_squeeze_batch_combo_alpha`) — which itself
/// continues from the post-zeta state — so DEEP alpha, zeta, the FRI batch-combination alpha, and
/// ALL betas come from ONE ordered sponge transcript. (NOTE: per-round commit-phase PoW grinding is
/// modeled as zero-bit, i.e. `check_witness(0, _)` which the native challenger short-circuits with NO
/// observe and NO sample — so it does not perturb the sponge between observe(comm_r) and beta_r.)
pub fn fs_squeeze_betas(
    start_state: [Val; WIDTH],
    commit_blocks: &[[Val; RATE]],
) -> (Vec<Challenge>, [Val; WIDTH]) {
    let mut state = start_state;
    let mut betas = Vec::with_capacity(commit_blocks.len());
    for block in commit_blocks {
        // observe(commit_phase_commit_r): one FULL-rate absorb-permute into the carried state.
        state = prefix_round_state(state, *block);
        // sample beta_r: LIFO squeeze of this round's permute output rate.
        betas.push(Challenge::from_basis_coefficients_fn(|i| state[RATE - 1 - i]));
    }
    (betas, state)
}

/// SQUEEZE the FRI query index CONTINUING from the post-last-beta sponge state, in the EXACT p3-fri
/// verifier order (p3-fri-0.6.1 verifier.rs:317-356): after the last beta the verifier
///   1. `observe_algebra_slice(&final_poly)`         (the FRI final polynomial coefficients),
///   2. `observe(log_arity)` for each round           (the variable-arity schedule),
///   3. `check_witness(query_pow_bits, query_pow_witness)`  (query-phase PoW grinding),
///   4. per query: `index = sample_bits(log_global_max_height + extra)`.
/// All of step 1-3 are `observe`s into the SAME carried sponge (the per-element duplex buffering is
/// modeled here as RATE-aligned FULL-rate absorb rounds: `final_blocks`), so changing the final
/// polynomial OR the PoW witness changes the carried state and therefore the query index. The index
/// draw itself is a single `sample()` (a SQUEEZE-ONLY duplexing of the final absorbed state, since the
/// last observe drained the output buffer), masked to `index_bits` low bits by `sample_bits`.
///
/// `start_state` is the post-last-beta state (`fs_squeeze_betas`'s returned state). `final_blocks` are
/// the post-beta transcript chunked into FULL-rate (RATE-element) absorb blocks (>= 1: the real flow
/// always observes the final polynomial, which is >= D base elements). After the LAST full-rate
/// observe round, the native challenger's input buffer is empty and the output buffer has just been
/// REFILLED with that round's permute rate — so the index `sample()` performs NO further permute and
/// the draw is the top LIFO pop `state[RATE-1]` of the LAST observe round's permute output. Returns
/// `(draw_out, post_state)` where `draw_out` is the FULL base-field draw (the caller masks to bits).
pub fn fs_squeeze_query_index(
    start_state: [Val; WIDTH],
    final_blocks: &[[Val; RATE]],
) -> (Val, [Val; WIDTH]) {
    assert!(
        !final_blocks.is_empty(),
        "fs_squeeze_query_index requires >= 1 post-beta observe block (the final polynomial)"
    );
    // observe the post-beta transcript (final_poly ‖ log_arities ‖ query-PoW witness), RATE-aligned.
    let mut state = start_state;
    for block in final_blocks {
        state = prefix_round_state(state, *block);
    }
    // index draw: a single sample(). The last full-rate observe just refilled the output buffer and
    // emptied the input buffer, so this sample performs NO duplexing — it is the top LIFO pop of the
    // last observe round's permute rate (sample_bits then masks it to the low index bits).
    (state[RATE - 1], state)
}

// ----- (b-hash) leaf-row hashing block (PaddingFreeSponge MyHash) -----
//
// Appended AFTER the (a-root) block. Computes the Merkle level-0 leaf digest IN-AIR as
// `MyHash(opened_row ‖ salt)` (the deployed `PaddingFreeSponge<Perm,16,8,8>` over the salted leaf
// row), so the level-0 running is no longer a free supplied digest. Layout:
//   row[leaf_row_len] ‖ salt[SALT_ELEMS] ‖ perm_0[PERM_COLS] ‖ ... ‖ perm_{P-1}[PERM_COLS]
// where `P = lh_perms(...)` is the sponge's absorb-permute count. Perm k's input is the absorbed
// sponge state for block k (overwrite of the rate lanes from the carried capacity/state, per
// `PaddingFreeSponge::hash_iter`); perm k's output (16 lanes) is the carried state into block k+1.
// The LAST perm's output[..DIGEST_ELEMS] is the leaf digest, constrained == Merkle level-0 running.

/// The salted leaf-hash input length: opened row + the hiding MMCS salt columns.
fn lh_input_len(cfg: &RecursiveConfig) -> usize {
    cfg.leaf_row_len + SALT_ELEMS
}

/// Number of Poseidon2 absorb-permutes the `PaddingFreeSponge` performs over `m` input elements:
/// one per full RATE block, plus one for a trailing partial block. For `m` a multiple of RATE this
/// is `m/RATE` (the loop permutes the full block, then breaks at the next empty block with NO extra
/// permute); otherwise `ceil(m/RATE)`. Always >= 1 (`m >= 1`: SALT_ELEMS > 0). Mirrors
/// `p3-symmetric` `PaddingFreeSponge::hash_iter` (sponge.rs:172-204) exactly.
fn lh_perms(cfg: &RecursiveConfig) -> usize {
    let m = lh_input_len(cfg);
    m.div_ceil(RATE).max(1)
}

/// ABSOLUTE column of the opened-row cells (start of the (b-hash) block).
fn lh_row(cfg: &RecursiveConfig) -> usize {
    lh_base(cfg)
}
/// ABSOLUTE column of the salt cells.
fn lh_salt(cfg: &RecursiveConfig) -> usize {
    lh_row(cfg) + cfg.leaf_row_len
}
/// ABSOLUTE column of leaf-hash perm block `k` (each `PERM_COLS` wide).
fn lh_perm_base(cfg: &RecursiveConfig, k: usize) -> usize {
    lh_salt(cfg) + SALT_ELEMS + k * PERM_COLS
}

/// Base of the (b-hash) leaf-hash block (after the a-root, LO, and FP blocks).
fn lh_base(cfg: &RecursiveConfig) -> usize {
    fp_base(cfg) + fp_size(cfg)
}

/// Size of the (b-hash) block: `row ‖ salt ‖ P perms`. 0 unless enabled with a Merkle block.
fn lh_size(cfg: &RecursiveConfig) -> usize {
    if cfg.leaf_row_len > 0 && cfg.merkle_levels > 0 {
        cfg.leaf_row_len + SALT_ELEMS + lh_perms(cfg) * PERM_COLS
    } else {
        0
    }
}

/// Total column count for a window with this config.
pub fn n_cols(cfg: &RecursiveConfig) -> usize {
    lh_base(cfg) + lh_size(cfg)
}

/// The `j`-th tower column (`sp[j]`).
const fn sp_col(j: usize) -> usize {
    C_SP + j
}

/// The fixed tower base constants `base_j = g^{2^j}` for `g = two_adic_generator(log_height+1)`.
fn tower_bases(log_height: usize) -> Vec<Val> {
    let g = Val::two_adic_generator(log_height + 1);
    let mut bases = Vec::with_capacity(log_height);
    let mut p = g;
    for _ in 0..log_height {
        bases.push(p);
        p = p * p;
    }
    bases
}

// ======================================================================================
// Window builder.
// ======================================================================================

/// (LO) honest inputs for ONE per-round FRI commit-phase layer open: the round's FRI-layer
/// commitment (root the path folds to), the hiding-MMCS salt of the opened leaf row, and the
/// bottom-to-top Merkle auth path. The opened pair is taken from the descent (not here).
pub struct LayerOpenInput<'a> {
    /// The round-r FRI-layer commitment (the root the authenticated path must reconstruct).
    pub layer_commit: [Val; DIGEST_ELEMS],
    /// The hiding-MMCS salt of the opened arity-2 leaf row.
    pub salt: [Val; SALT_ELEMS],
    /// The bottom-to-top Merkle auth path (`layer_open_levels` siblings).
    pub merkle_siblings: &'a [[Val; DIGEST_ELEMS]],
}

/// All honest inputs to one query window. The Merkle / beta-draw / descent / DEEP fields are only
/// consumed when the corresponding config block is enabled.
pub struct WindowInputs<'a> {
    pub draw_out: Val,
    /// Round 0 sibling evals + challenge (and, for the descent, the per-round betas/siblings come
    /// via `descent`).
    pub e0: Challenge,
    pub e1: Challenge,
    pub beta: Challenge,
    /// (c) Extra descent rounds: per round `r≥1`, the freshly-opened sibling `e1_r` and `beta_r`.
    /// `e0_r` and `s_r` are derived (chain-link + point-square). Empty = single fold.
    pub descent: &'a [(Challenge, Challenge)], // (e1_r, beta_r) for r = 1..num_rounds
    /// (a) Merkle path: leaf digest + bottom-to-top siblings. Length must equal `merkle_levels`.
    pub merkle_leaf: [Val; DIGEST_ELEMS],
    pub merkle_siblings: &'a [[Val; DIGEST_ELEMS]],
    /// (b′) DEEP OOD inputs: `(zeta, Q)`; `C` is derived `Z_H(zeta)·Q`.
    pub deep_zeta: Challenge,
    pub deep_q: Challenge,
    /// (FS-αζ) the DEEP/batch combination challenge `alpha` placed in the `DEEP_ALPHA` cell. When
    /// `fs_alpha_zeta`, the honest builder OVERRIDES both this and `deep_zeta` with the values
    /// DERIVED from the FS transcript (`fs_squeeze_alpha_zeta`), so the bus carries the genuine FS
    /// challenges. When `fs_alpha_zeta` is off this is the committed alpha (unconstrained by FS).
    pub deep_alpha: Challenge,
    /// (FS-αζ) the observed transcript blocks for the FS squeeze: `fs_pre_alpha_rounds` pre-alpha
    /// FULL-rate blocks (metadata ‖ trace commitment ‖ public values) followed by ONE quotient-
    /// commitment block. Length must equal `fs_pre_alpha_rounds + 1` when `fs_alpha_zeta`. The
    /// honest builder DERIVES alpha/zeta + the per-round perm blocks from these. Unused otherwise.
    pub fs_blocks: &'a [[Val; RATE]],
    /// (a-FS) the pre-squeeze sponge state whose permutation produces the index draw. When
    /// `inline_sponge_draw`, `draw_out` is CHECKED to equal `permute(sponge_in)[RATE-1]` (and the
    /// honest builder DERIVES `draw_out` from it). When `inline_sponge_draw` is off this is unused.
    pub sponge_in: [Val; WIDTH],
    /// (a-FS++) When `sponge_absorb`, the pre-absorb sponge state and the rate values being
    /// absorbed; the honest builder DERIVES `sponge_in = absorb(prior_state, absorbed, n)` from
    /// these (and `sponge_in` above is then ignored). When `sponge_absorb` is off these are unused.
    pub prior_state: [Val; WIDTH],
    pub absorbed: [Val; RATE],
    /// (a-FS+++) transcript-PREFIX absorb blocks: per round `k`, the RATE observed transcript
    /// elements absorbed (each a FULL-rate absorb). Length must equal `sponge_prefix_rounds`. When
    /// `sponge_prefix_rounds > 0` the honest builder DERIVES `prior_state = prefix_chain_state(blocks)`
    /// (the all-zero-boundary chain), overriding the `prior_state` field above. When 0, unused.
    pub prefix_blocks: &'a [[Val; RATE]],
    /// (c-pub) public-IO statement: per element `(outer, observed, inner)`. Length must equal
    /// `public_io_len`. The honest builder sets the three equal.
    pub public_io: &'a [(Val, Val, Val)],
    /// (FS-β) the observed FRI commit-phase commitments (one RATE-element block per fold round) the
    /// betas are squeezed from. Length must equal `fs_beta_rounds` when `fs_beta_rounds > 0`. The
    /// honest builder DERIVES each `beta_r` (overriding the descent's supplied `beta`/`descent` betas
    /// for rounds `0..fs_beta_rounds`) from these by continuing the post-zeta sponge state. Unused
    /// otherwise.
    pub fs_beta_blocks: &'a [[Val; RATE]],
    /// (FS-qi) the post-beta transcript (final_poly ‖ log_arities ‖ query-PoW witness) chunked into
    /// FULL-rate (RATE-element) observe blocks. Length must equal `fs_final_blocks` when
    /// `fs_final_blocks > 0`. The honest builder DERIVES the query index `draw_out` from these by
    /// continuing the post-last-beta sponge state. Unused otherwise.
    pub fs_final_blocks: &'a [[Val; RATE]],
    /// (a-root) the inner proof's MAIN-TRACE commitment (the first PCS round Merkle root) the FS-index-
    /// driven Merkle opening must authenticate to. Consumed only when `bind_root_to_main_commit`; the
    /// honest builder copies it into the `main_commit` column block and the eval constrains the
    /// reconstructed root == this. For an HONEST opening of the real inner main-trace tree this equals
    /// the tree root; a forged opening that folds to a different root rejects.
    pub inner_main_commit: [Val; DIGEST_ELEMS],
    /// (LO) per-round FRI commit-phase layer-open inputs, one per `layer_open_rounds` round. Each
    /// supplies that round's FRI-layer commitment, the opened leaf salt, and the bottom-to-top Merkle
    /// auth path (`layer_open_levels` siblings). The opened pair `(e_fold, e_sib)` is NOT supplied — it
    /// is DERIVED from the descent: `e_fold = round_e0(r)`, `e_sib = round_e1(r)` (the honest builder
    /// reads them from the fold chain so the authenticated sibling IS the folded one). For an honest
    /// opening of the real FRI-layer tree, `MyHash(flatten(e_fold,e_sib) ‖ salt)` authenticates to
    /// `layer_commit`. Length must equal `layer_open_rounds`.
    pub layer_opens: &'a [LayerOpenInput<'a>],
    /// (FP) the committed FRI final-polynomial coefficients and the query point `x`. The descent's
    /// fully-folded value (`round_folded(rounds-1)`) is constrained to equal `Horner(coeffs, x)`.
    /// `coeffs.len()` must equal `final_poly_len`. Consumed only when `final_poly_len > 0`.
    pub final_poly_x: Val,
    pub final_poly_coeffs: &'a [Challenge],
    /// (b-hash) the OPENED main-trace leaf ROW (the matrix row whose `MyHash` is the leaf digest)
    /// and the hiding MMCS `salt`. Consumed only when `leaf_row_len > 0`; the honest builder hashes
    /// `leaf_row ‖ leaf_salt` via `PaddingFreeSponge` and the eval re-derives + constrains the
    /// digest == Merkle level-0 running. `leaf_row.len()` must equal `leaf_row_len`. For an honest
    /// opening of the real hiding tree, `MyHash(leaf_row ‖ leaf_salt)` IS the level-0 leaf digest.
    pub leaf_row: &'a [Val],
    pub leaf_salt: [Val; SALT_ELEMS],
}

impl<'a> WindowInputs<'a> {
    /// Minimal single-fold inputs (no Merkle / descent / DEEP).
    pub fn single(draw_out: Val, e0: Challenge, e1: Challenge, beta: Challenge) -> Self {
        Self {
            draw_out,
            e0,
            e1,
            beta,
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
            deep_alpha: Challenge::ZERO,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        }
    }
}

/// The pre-permute sponge state produced by absorbing `n` rate elements into `prior_state`, mirror
/// of [`crate::challenger::RowChallenger::duplexing`] EXACTLY: rate lanes `[0..n]` overwritten with
/// `absorbed[0..n]`, lanes `[n..RATE]` zeroed, capacity slot `RATE += n` (the length tag), capacity
/// slots `[RATE+1..WIDTH]` carried from `prior_state`. For `n == 0` (a squeeze-only permute with an
/// empty input buffer) NO rate lane is overwritten and NO tag is added, so the whole state is
/// carried from `prior_state` unchanged — matching `duplexing`'s `if n > 0` guard exactly.
pub fn absorb_state(prior_state: [Val; WIDTH], absorbed: [Val; RATE], n: usize) -> [Val; WIDTH] {
    assert!(n <= RATE, "absorb n must be <= RATE");
    let mut s = prior_state;
    for i in 0..n {
        s[i] = absorbed[i];
    }
    if n > 0 {
        for i in n..RATE {
            s[i] = Val::ZERO;
        }
        s[RATE] = prior_state[RATE] + Val::from_u8(n as u8);
    }
    s
}

/// One prefix-chain absorb-permute round: `state_k = permute(absorb(state_prev, block, RATE))`,
/// a FULL-rate absorb (`n == RATE`) followed by the Poseidon2 permutation — exactly what
/// [`crate::challenger::RowChallenger::observe`] triggers when the input buffer fills to RATE. The
/// returned state is the post-permute sponge (the `state_k` carried into the next round).
pub fn prefix_round_state(state_prev: [Val; WIDTH], block: [Val; RATE]) -> [Val; WIDTH] {
    let pre = absorb_state(state_prev, block, RATE);
    let consts = inlined_perm::Constants::pinned();
    let cols = inlined_perm::generate_perm_cols(pre, &consts);
    inlined_perm::perm_output(&cols)
}

/// The chained prefix state after absorbing `blocks` (each a FULL RATE-element transcript block)
/// starting from the `new()` all-zero boundary: `state_0 = ZERO`, `state_k =
/// permute(absorb(state_{k-1}, block_k, RATE))`. The returned value is `state_K` — the constrained
/// `prior_state` that feeds the final duplex step. The single source shared by the honest builder
/// and (via `eval_perm` round-by-round) the (a-FS+++) AIR binding.
pub fn prefix_chain_state(blocks: &[[Val; RATE]]) -> [Val; WIDTH] {
    let mut state = [Val::ZERO; WIDTH];
    for block in blocks {
        state = prefix_round_state(state, *block);
    }
    state
}

/// The base-field index draw produced by squeezing `sponge_in`: the LIFO pop of the freshly
/// refilled output buffer = `permute(sponge_in)[RATE-1]` (challenger.rs `sample()` after a
/// `duplexing()` pops `out_buf[m-1]` where `out_buf == sponge[..RATE]`). The single source shared
/// by the honest builder (which DERIVES `draw_out` from it) and the (a-FS) AIR binding.
pub fn squeeze_draw(sponge_in: [Val; WIDTH]) -> Val {
    let consts = inlined_perm::Constants::pinned();
    let cols = inlined_perm::generate_perm_cols(sponge_in, &consts);
    cols[inlined_perm::C_OUTPUT + (RATE - 1)]
}

/// The successive base-field draws popped off ONE squeeze permute of `sponge_in`: draw `d` =
/// `permute(sponge_in)[RATE-1-d]` for `d in 0..count`. The challenger's `sample()` pops the BACK of
/// the freshly-refilled `out_buf` (LIFO) WITHOUT re-permuting while the buffer is non-empty, so the
/// first `count` draws (count ≤ RATE) all come from the same permute. Draw 0 is the index draw.
pub fn squeeze_draws(sponge_in: [Val; WIDTH], count: usize) -> Vec<Val> {
    assert!(count <= RATE, "at most RATE draws come from one squeeze permute");
    let consts = inlined_perm::Constants::pinned();
    let cols = inlined_perm::generate_perm_cols(sponge_in, &consts);
    (0..count)
        .map(|d| cols[inlined_perm::C_OUTPUT + (RATE - 1 - d)])
        .collect()
}

/// The `PaddingFreeSponge` (`config::MyHash = PaddingFreeSponge<Perm,16,8,8>`) absorb-permute
/// schedule over the salted leaf input `input` (= `opened_row ‖ salt`), mirroring
/// `p3-symmetric` `PaddingFreeSponge::hash_iter` (sponge.rs:172-204) CONSTANT-FOR-CONSTANT:
///
///   - state starts all-zero (`WIDTH` lanes);
///   - absorb blocks of RATE: for block `k`, OVERWRITE `state[i] = input[k*RATE + i]` for the
///     elements present (a partial trailing block overwrites only its prefix — lanes beyond it
///     KEEP the carried state, NOT zeroed; this is the no-padding overwrite variant);
///   - permute after every FULL block; permute once more for a trailing PARTIAL block (`i > 0`);
///   - the digest is `state[..DIGEST_ELEMS]` of the final permute output.
///
/// Returns the per-permute PRE-permute input states (`pre_states[k]` is perm `k`'s input). The
/// honest builder runs the real perm over each; the AIR re-derives the same overwrite + constrains
/// each perm. The digest = `permute(pre_states[last])[..DIGEST_ELEMS]`.
pub fn leaf_hash_pre_states(input: &[Val]) -> Vec<[Val; WIDTH]> {
    assert!(!input.is_empty(), "leaf hash input must be non-empty");
    let consts = inlined_perm::Constants::pinned();
    let mut pre_states = Vec::new();
    let mut state = [Val::ZERO; WIDTH];
    let n = input.len();
    let mut pos = 0usize;
    loop {
        // Absorb up to RATE elements into the rate lanes (overwrite; no padding).
        let fill = core::cmp::min(RATE, n - pos);
        for i in 0..fill {
            state[i] = input[pos + i];
        }
        pos += fill;
        // This block's permute input is the current (overwritten) state.
        pre_states.push(state);
        let cols = inlined_perm::generate_perm_cols(state, &consts);
        state = inlined_perm::perm_output(&cols);
        if pos >= n {
            // Input exhausted. `hash_iter` would break WITHOUT this permute only if the trailing
            // block was empty (fill == 0) — but we only push/permute when fill > 0 (the loop above
            // always absorbs at least one element while pos < n, and we stop once pos == n). The
            // exact-multiple case is handled because the last full block is the final permute and
            // the next (empty) iteration is never entered.
            break;
        }
    }
    pre_states
}

/// The leaf digest `MyHash(input)` (first `DIGEST_ELEMS` lanes of the final permute), the authority
/// the (b-hash) AIR is checked against. PINned equal to a real `PaddingFreeSponge` in tests.
pub fn leaf_hash_digest(input: &[Val]) -> [Val; DIGEST_ELEMS] {
    let consts = inlined_perm::Constants::pinned();
    let pre = leaf_hash_pre_states(input);
    let last = *pre.last().expect("at least one permute");
    let cols = inlined_perm::generate_perm_cols(last, &consts);
    let out = inlined_perm::perm_output(&cols);
    core::array::from_fn(|i| out[i])
}

/// Build one query window honestly. The index, fold point, tower, descent chain, Merkle perm
/// blocks, and DEEP tower are all DERIVED (the authority the AIR is checked against); tamper tests
/// mutate the matrix afterwards. When `inline_sponge_draw`, `draw_out` is DERIVED from `sponge_in`
/// (the squeeze pop) so the index decomposition is over the genuine Poseidon2 sponge output.
pub fn pack_window(cfg: &RecursiveConfig, inp: &WindowInputs) -> Vec<Val> {
    let h = cfg.log_height;
    // (a-FS+++) when a transcript-prefix absorb chain feeds the duplex step, the duplex step's
    // prior_state is the chained output of absorbing the prefix blocks from the zero boundary; it
    // overrides the supplied prior_state field.
    let prior_state = if cfg.inline_sponge_draw && cfg.sponge_absorb && cfg.sponge_prefix_rounds > 0 {
        prefix_chain_state(inp.prefix_blocks)
    } else {
        inp.prior_state
    };
    // (a-FS++) when absorbing, the squeeze permute's input is DERIVED from the absorb (prior_state,
    // absorbed, n); otherwise it is the supplied committed sponge_in.
    let sponge_in = if cfg.inline_sponge_draw && cfg.sponge_absorb {
        absorb_state(prior_state, inp.absorbed, cfg.sponge_absorb_n)
    } else {
        inp.sponge_in
    };
    let rounds = cfg.rounds();

    // (FS-β) when squeezing the FRI commit-phase betas, DERIVE them by continuing the FS transcript,
    // and OVERRIDE the descent's supplied betas for rounds 0..fs_beta_rounds so the fold actually
    // consumes the FS-squeezed challenge. Computed before the fold so folded_r uses it. The beta
    // chain's start state is the post-BATCH-COMBO-ALPHA state when `fs_batch_combo_alpha` (the FRI
    // batch-combination alpha squeeze-permutes the post-zeta state before the first commit observe),
    // else the post-zeta state directly. Returns the betas AND the post-last-beta state (the query
    // index continuation point).
    let (fs_betas, post_beta_state): (Vec<Challenge>, [Val; WIDTH]) = if cfg.fs_beta_rounds > 0 {
        assert!(cfg.fs_alpha_zeta, "fs_beta_rounds requires fs_alpha_zeta");
        assert!(rounds >= cfg.fs_beta_rounds, "num_rounds must be >= fs_beta_rounds");
        assert_eq!(
            inp.fs_beta_blocks.len(),
            cfg.fs_beta_rounds,
            "fs_beta_blocks must have fs_beta_rounds entries"
        );
        let (pre, quot) = inp.fs_blocks.split_at(cfg.fs_pre_alpha_rounds);
        let post_zeta = fs_post_zeta_state(pre, quot[0]);
        let beta_start = if cfg.fs_batch_combo_alpha {
            fs_squeeze_batch_combo_alpha(post_zeta).1
        } else {
            post_zeta
        };
        fs_squeeze_betas(beta_start, inp.fs_beta_blocks)
    } else {
        (Vec::new(), [Val::ZERO; WIDTH])
    };

    // (FS-qi) the query index draw: when unifying, CONTINUE the post-last-beta state by observing the
    // post-beta transcript (final_poly ‖ log_arities ‖ query-PoW witness), then the index draw is the
    // top LIFO pop of the last observe round's permute rate — REPLACING the separate (a-FS) chain.
    let draw_out = if cfg.fs_final_blocks > 0 {
        assert!(cfg.fs_beta_rounds > 0, "fs_final_blocks requires fs_beta_rounds > 0");
        assert_eq!(
            inp.fs_final_blocks.len(),
            cfg.fs_final_blocks,
            "fs_final_blocks must have fs_final_blocks entries"
        );
        fs_squeeze_query_index(post_beta_state, inp.fs_final_blocks).0
    } else if cfg.inline_sponge_draw {
        squeeze_draw(sponge_in)
    } else {
        inp.draw_out
    };
    let canon = draw_out.as_canonical_u32();
    let index = (canon & ((1u32 << h) - 1)) as usize;
    let s0 = fold_point(h, index);
    let fs_betas: Vec<Challenge> = fs_betas;
    // Effective per-round betas: FS-squeezed for rounds 0..fs_beta_rounds, else the supplied inputs.
    let round_beta_val = |r: usize| -> Challenge {
        if r < cfg.fs_beta_rounds {
            fs_betas[r]
        } else if r == 0 {
            inp.beta
        } else {
            inp.descent[r - 1].1
        }
    };

    let beta0 = round_beta_val(0);
    let folded0 = fold_pair(inp.e0, inp.e1, beta0, s0);

    let mut row = vec![Val::ZERO; n_cols(cfg)];
    row[C_DRAW_OUT] = draw_out;
    for i in 0..FIELD_BITS {
        row[C_BITS + i] = Val::from_u32((canon >> i) & 1);
    }
    row[C_INDEX] = Val::from_u32(index as u32);
    row[C_E0..C_E0 + D].copy_from_slice(&limbs(inp.e0));
    row[C_E1..C_E1 + D].copy_from_slice(&limbs(inp.e1));
    row[C_BETA..C_BETA + D].copy_from_slice(&limbs(beta0));
    row[C_S] = s0;
    row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded0));

    // (2) product tower.
    let bases = tower_bases(h);
    let mut acc = Val::ONE;
    row[sp_col(0)] = acc;
    for j in 0..h {
        let rev_bit = (canon >> (h - 1 - j)) & 1 == 1;
        if rev_bit {
            acc *= bases[j];
        }
        row[sp_col(j + 1)] = acc;
    }
    debug_assert_eq!(acc, s0, "tower product != fold_point — base/bit convention mismatch");

    // (c) descent rounds r = 1..num_rounds: s_r = s_{r-1}², e0_r = folded_{r-1}, e1_r/beta_r given.
    if rounds > 1 {
        assert_eq!(inp.descent.len(), rounds - 1, "descent must supply rounds-1 (e1,beta) pairs");
    }
    let mut s_prev = s0;
    let mut folded_prev = folded0;
    // (LO) per-round opened pair (e_fold = round_e0(r), e_sib = round_e1(r)) the layer open
    // authenticates; round 0 is (e0, e1). folded_last is the value the (FP) final-poly check binds.
    let mut round_pairs: Vec<(Challenge, Challenge)> = Vec::with_capacity(rounds);
    round_pairs.push((inp.e0, inp.e1));
    let mut folded_last = folded0;
    for r in 1..rounds {
        let e1_r = inp.descent[r - 1].0;
        let beta_r = round_beta_val(r); // (FS-β) FS-squeezed for r < fs_beta_rounds, else supplied
        let e0_r = folded_prev; // chain-link
        let s_r = s_prev * s_prev; // point-square
        let folded_r = fold_pair(e0_r, e1_r, beta_r, s_r);
        let base = extra_round_base(cfg, r);
        row[base..base + D].copy_from_slice(&limbs(e0_r));
        row[base + D..base + 2 * D].copy_from_slice(&limbs(e1_r));
        row[base + 2 * D..base + 3 * D].copy_from_slice(&limbs(beta_r));
        row[base + 3 * D] = s_r;
        row[base + 3 * D + 1..base + 4 * D + 1].copy_from_slice(&limbs(folded_r));
        round_pairs.push((e0_r, e1_r));
        s_prev = s_r;
        folded_prev = folded_r;
        folded_last = folded_r;
    }

    // (FS-αζ) when squeezing, alpha + zeta are DERIVED from the FS transcript (the observed
    // pre-alpha prefix + quotient commitment), overriding the supplied deep_alpha/deep_zeta so the
    // DEEP bus carries the genuine FS challenges.
    let (deep_alpha, deep_zeta) = if cfg.fs_alpha_zeta {
        assert_eq!(
            inp.fs_blocks.len(),
            cfg.fs_pre_alpha_rounds + 1,
            "fs_blocks must have fs_pre_alpha_rounds + 1 entries"
        );
        let (pre, quot) = inp.fs_blocks.split_at(cfg.fs_pre_alpha_rounds);
        fs_squeeze_alpha_zeta(pre, quot[0])
    } else {
        (inp.deep_alpha, inp.deep_zeta)
    };

    // (b′) DEEP OOD block.
    if let Some(k) = cfg.deep_log_degree {
        let zh = crate::deep::vanishing_poly_at(deep_zeta, k);
        let c = zh * inp.deep_q;
        let db = deep_base(cfg);
        row[db + DEEP_ZETA..db + DEEP_ZETA + D].copy_from_slice(&limbs(deep_zeta));
        row[db + DEEP_Q..db + DEEP_Q + D].copy_from_slice(&limbs(inp.deep_q));
        row[db + DEEP_C..db + DEEP_C + D].copy_from_slice(&limbs(c));
        row[db + DEEP_ALPHA..db + DEEP_ALPHA + D].copy_from_slice(&limbs(deep_alpha));
        let mut p = deep_zeta;
        for j in 0..=k {
            row[db + DEEP_POW + j * D..db + DEEP_POW + j * D + D].copy_from_slice(&limbs(p));
            p = p * p;
        }
    }

    // (b) challenger-draw columns: one per bound challenge (= the challenge limbs themselves; the
    // binding constraint pins beta/zeta == these draw columns). Honest builder copies the limbs.
    if cfg.bind_beta_to_draws {
        let mut dbase = draw_block_base(cfg);
        // round betas (the effective betas — FS-squeezed for r < fs_beta_rounds, else supplied).
        for r in 0..rounds {
            row[dbase..dbase + D].copy_from_slice(&limbs(round_beta_val(r)));
            dbase += D;
        }
        if cfg.deep_log_degree.is_some() {
            row[dbase..dbase + D].copy_from_slice(&limbs(deep_zeta));
        }
    }

    // (a) Merkle path inlined level-by-level. Index bit at level j = bit_j of the bus index.
    if cfg.merkle_levels > 0 {
        assert_eq!(
            inp.merkle_siblings.len(),
            cfg.merkle_levels,
            "merkle_siblings must have merkle_levels entries"
        );
        let consts = inlined_perm::Constants::pinned();
        let mut running = inp.merkle_leaf;
        let mut idx = index;
        for level in 0..cfg.merkle_levels {
            let bit = (idx & 1) == 1;
            let sib = inp.merkle_siblings[level];
            // placement exactly like merkle_air::pack_level / fold_step.
            let mut input = [Val::ZERO; WIDTH];
            let (a, b) = if bit { (sib, running) } else { (running, sib) };
            input[..DIGEST_ELEMS].copy_from_slice(&a);
            input[DIGEST_ELEMS..].copy_from_slice(&b);
            let perm_cols = inlined_perm::generate_perm_cols(input, &consts);
            let lbase = merkle_level_base(cfg, level);
            row[lbase + ML_RUNNING..lbase + ML_RUNNING + DIGEST_ELEMS].copy_from_slice(&running);
            row[lbase + ML_SIBLING..lbase + ML_SIBLING + DIGEST_ELEMS].copy_from_slice(&sib);
            row[lbase + ML_PERM..lbase + ML_PERM + PERM_COLS].copy_from_slice(&perm_cols);
            // next running = perm output[..8].
            running.copy_from_slice(&perm_cols[C_OUTPUT..C_OUTPUT + DIGEST_ELEMS]);
            idx >>= 1;
        }
    }

    // (a-FS) inlined challenger-sponge squeeze block: sponge_in ‖ Poseidon2 perm. draw_out (above)
    // was derived from this very block's output, so the index decomposition is over the genuine
    // sponge output.
    if cfg.inline_sponge_draw {
        let consts = inlined_perm::Constants::pinned();
        let perm_cols = inlined_perm::generate_perm_cols(sponge_in, &consts);
        let sb = sponge_base(cfg);
        row[sb + SP_SPONGE_IN..sb + SP_SPONGE_IN + WIDTH].copy_from_slice(&sponge_in);
        row[sb + SP_PERM..sb + SP_PERM + PERM_COLS].copy_from_slice(&perm_cols);
        debug_assert_eq!(
            row[C_DRAW_OUT],
            perm_cols[C_OUTPUT + (RATE - 1)],
            "draw_out != squeeze pop — sponge derivation mismatch"
        );
        // (a-FS+) successive draws 1..sponge_draws are the next LIFO pops off the SAME refilled
        // output buffer: draw d = output[RATE-1-d].
        let n_draws = cfg.sponge_draw_count();
        for d in 1..n_draws {
            row[sb + SP_EXTRA_DRAWS + (d - 1)] = perm_cols[C_OUTPUT + (RATE - 1 - d)];
        }
        // (a-FS++) absorb sub-block: prior_state ‖ absorbed (the constrained derivation of
        // sponge_in). sponge_in above was already set to absorb_state(prior_state, absorbed, n).
        if cfg.sponge_absorb {
            let ab = sb + sp_absorb(cfg);
            row[ab + SPA_PRIOR_STATE..ab + SPA_PRIOR_STATE + WIDTH].copy_from_slice(&prior_state);
            row[ab + SPA_ABSORBED..ab + SPA_ABSORBED + RATE].copy_from_slice(&inp.absorbed);
            debug_assert_eq!(
                sponge_in,
                absorb_state(prior_state, inp.absorbed, cfg.sponge_absorb_n),
                "sponge_in != absorb(prior_state, absorbed, n)"
            );
            // (a-FS+++) transcript-prefix absorb chain: state_0 = ZERO (new() boundary), per round
            // state_k = permute(absorb(state_{k-1}, block_k, RATE)); the final state_K == prior_state.
            if cfg.sponge_prefix_rounds > 0 {
                assert_eq!(
                    inp.prefix_blocks.len(),
                    cfg.sponge_prefix_rounds,
                    "prefix_blocks must have sponge_prefix_rounds entries"
                );
                let mut state = [Val::ZERO; WIDTH];
                for k in 1..=cfg.sponge_prefix_rounds {
                    let block = inp.prefix_blocks[k - 1];
                    let pre = absorb_state(state, block, RATE);
                    let perm_cols = inlined_perm::generate_perm_cols(pre, &consts);
                    let pb = sb + prefix_round_base(cfg, k);
                    row[pb + SPP_BLOCK..pb + SPP_BLOCK + RATE].copy_from_slice(&block);
                    row[pb + SPP_PERM..pb + SPP_PERM + PERM_COLS].copy_from_slice(&perm_cols);
                    state = inlined_perm::perm_output(&perm_cols);
                }
                debug_assert_eq!(state, prior_state, "prefix chain output != prior_state");
            }
        }
    }

    // (c-pub) public-IO block.
    if cfg.public_io_len > 0 {
        assert_eq!(
            inp.public_io.len(),
            cfg.public_io_len,
            "public_io must have public_io_len entries"
        );
        let pb = public_io_base(cfg);
        for (j, &(outer, observed, inner)) in inp.public_io.iter().enumerate() {
            let e = pb + j * PIO_STRIDE;
            row[e + PIO_OUTER] = outer;
            row[e + PIO_OBSERVED] = observed;
            row[e + PIO_INNER] = inner;
        }
    }

    // (FS-αζ) alpha/zeta FS-squeeze block: pre-alpha absorb chain ‖ alpha ‖ quotient round ‖ zeta.
    // The same FULL-rate absorb-permute primitive as the index-draw prefix chain, but its squeezes
    // produce the DEEP/batch combination challenge alpha (after the pre-alpha prefix) and the OOD
    // point zeta (after observing the quotient commitment), mirroring p3-uni-stark's FS order.
    if cfg.fs_alpha_zeta {
        assert!(cfg.fs_pre_alpha_rounds >= 1, "fs_pre_alpha_rounds must be >= 1");
        assert_eq!(
            inp.fs_blocks.len(),
            cfg.fs_pre_alpha_rounds + 1,
            "fs_blocks must have fs_pre_alpha_rounds + 1 entries"
        );
        let consts = inlined_perm::Constants::pinned();
        // pre-alpha rounds: state_0 = ZERO, state_k = permute(absorb(state_{k-1}, block_k, RATE)).
        let mut state = [Val::ZERO; WIDTH];
        for r in 0..cfg.fs_pre_alpha_rounds {
            let block = inp.fs_blocks[r];
            let pre = absorb_state(state, block, RATE);
            let perm_cols = inlined_perm::generate_perm_cols(pre, &consts);
            let rb = fs_round_base(cfg, r);
            row[rb + FS_BLOCK..rb + FS_BLOCK + RATE].copy_from_slice(&block);
            row[rb + FS_PERM..rb + FS_PERM + PERM_COLS].copy_from_slice(&perm_cols);
            state = inlined_perm::perm_output(&perm_cols);
        }
        // alpha = LIFO squeeze of the pre-alpha state's rate (the 8 sample() pops).
        let alpha = Challenge::from_basis_coefficients_fn(|i| state[RATE - 1 - i]);
        let ab = fs_alpha(cfg);
        row[ab..ab + D].copy_from_slice(&limbs(alpha));
        // quotient-observe round: absorb the quotient commitment into the (unchanged) post-alpha
        // sponge state (sampling does not mutate sponge_state), then permute.
        let quot_block = inp.fs_blocks[cfg.fs_pre_alpha_rounds];
        let pre_q = absorb_state(state, quot_block, RATE);
        let q_perm = inlined_perm::generate_perm_cols(pre_q, &consts);
        let qb = fs_quot_round_base(cfg);
        row[qb + FS_BLOCK..qb + FS_BLOCK + RATE].copy_from_slice(&quot_block);
        row[qb + FS_PERM..qb + FS_PERM + PERM_COLS].copy_from_slice(&q_perm);
        let post = inlined_perm::perm_output(&q_perm);
        // zeta = LIFO squeeze of the quotient round's state rate.
        let zeta = Challenge::from_basis_coefficients_fn(|i| post[RATE - 1 - i]);
        let zb = fs_zeta(cfg);
        row[zb..zb + D].copy_from_slice(&limbs(zeta));
        debug_assert_eq!(
            (alpha, zeta),
            {
                let (pre, quot) = inp.fs_blocks.split_at(cfg.fs_pre_alpha_rounds);
                fs_squeeze_alpha_zeta(pre, quot[0])
            },
            "FS block pack != fs_squeeze_alpha_zeta reference"
        );
    }

    // (FS-bca) FRI batch-combination alpha squeeze block: a SINGLE squeeze-only permute (n == 0) of
    // the post-zeta state, with NO observe (verifier.rs:195). alpha is its top-8 LIFO pops; the permute
    // output is the new continuation point for the betas. Computed before the FS-β block so the betas
    // thread from it.
    let beta_start_state: [Val; WIDTH] = if cfg.fs_beta_rounds > 0 {
        let (pre, quot) = inp.fs_blocks.split_at(cfg.fs_pre_alpha_rounds);
        let post_zeta = fs_post_zeta_state(pre, quot[0]);
        if cfg.fs_batch_combo_alpha {
            let consts = inlined_perm::Constants::pinned();
            // squeeze-only permute: input == post_zeta state UNCHANGED (n == 0).
            let pre_in = absorb_state(post_zeta, [Val::ZERO; RATE], 0);
            let perm_cols = inlined_perm::generate_perm_cols(pre_in, &consts);
            let bb = bca_base(cfg);
            row[bb + BCA_PERM..bb + BCA_PERM + PERM_COLS].copy_from_slice(&perm_cols);
            let state = inlined_perm::perm_output(&perm_cols);
            let alpha = Challenge::from_basis_coefficients_fn(|i| state[RATE - 1 - i]);
            row[bb + BCA_ALPHA..bb + BCA_ALPHA + D].copy_from_slice(&limbs(alpha));
            debug_assert_eq!(
                (alpha, state),
                fs_squeeze_batch_combo_alpha(post_zeta),
                "FS-bca block != fs_squeeze_batch_combo_alpha reference"
            );
            state
        } else {
            post_zeta
        }
    } else {
        [Val::ZERO; WIDTH]
    };

    // (FS-β) FRI commit-phase beta squeeze block: continuing from `beta_start_state` (the post-batch-
    // combo-alpha state when `fs_batch_combo_alpha`, else the post-zeta state), per round r:
    // observe(commit_phase_commit_r) FULL-rate absorb-permute → squeeze beta_r. Each beta_r is laid
    // into this round's beta cell (and was already used as round_beta_val(r) by the fold).
    if cfg.fs_beta_rounds > 0 {
        let consts = inlined_perm::Constants::pinned();
        let mut state = beta_start_state;
        for r in 0..cfg.fs_beta_rounds {
            let block = inp.fs_beta_blocks[r];
            let pre_in = absorb_state(state, block, RATE);
            let perm_cols = inlined_perm::generate_perm_cols(pre_in, &consts);
            let rb = fb_round_base(cfg, r);
            row[rb + FB_BLOCK..rb + FB_BLOCK + RATE].copy_from_slice(&block);
            row[rb + FB_PERM..rb + FB_PERM + PERM_COLS].copy_from_slice(&perm_cols);
            state = inlined_perm::perm_output(&perm_cols);
            let beta_r = Challenge::from_basis_coefficients_fn(|i| state[RATE - 1 - i]);
            row[rb + FB_BETA..rb + FB_BETA + D].copy_from_slice(&limbs(beta_r));
            debug_assert_eq!(beta_r, fs_betas[r], "FS-β block beta_r != fs_squeeze_betas reference");
        }
        // `state` is now the post-last-beta state — the query-index continuation point.

        // (FS-qi) query-index draw block: observe the post-beta transcript (final_poly ‖ log_arities ‖
        // query-PoW witness), RATE-chunked, continuing the post-last-beta state; the index draw_out
        // (C_DRAW_OUT) is the top LIFO pop of the LAST observe round's permute rate.
        if cfg.fs_final_blocks > 0 {
            for r in 0..cfg.fs_final_blocks {
                let block = inp.fs_final_blocks[r];
                let pre_in = absorb_state(state, block, RATE);
                let perm_cols = inlined_perm::generate_perm_cols(pre_in, &consts);
                let rb = fq_round_base(cfg, r);
                row[rb + FQ_BLOCK..rb + FQ_BLOCK + RATE].copy_from_slice(&block);
                row[rb + FQ_PERM..rb + FQ_PERM + PERM_COLS].copy_from_slice(&perm_cols);
                state = inlined_perm::perm_output(&perm_cols);
            }
            debug_assert_eq!(
                draw_out,
                state[RATE - 1],
                "FS-qi draw_out != post-beta squeeze pop"
            );
        }
    }

    // (a-root) inner main-trace commitment column: the bound recursion input the FS-index-driven
    // Merkle opening must authenticate to. Honest builder copies it in; for an honest opening of the
    // real inner main-trace tree this equals the reconstructed root, so the (a-root) equality holds.
    if cfg.bind_root_to_main_commit {
        let mb = mc_base(cfg);
        row[mb + MC_MAIN_COMMIT..mb + MC_MAIN_COMMIT + DIGEST_ELEMS]
            .copy_from_slice(&inp.inner_main_commit);
        debug_assert!(
            cfg.merkle_levels > 0,
            "bind_root_to_main_commit requires a Merkle block (merkle_levels > 0)"
        );
    }

    // (LO) per-round FRI commit-phase LAYER opening: for each round r, authenticate the opened
    // sibling against round r's layer commitment at the FS-shifted node, then bind (e_fold, e_sib) to
    // the descent's (round_e0(r), round_e1(r)) so the FRI fold consumes the AUTHENTICATED sibling.
    if cfg.layer_open_rounds > 0 {
        assert!(
            rounds >= cfg.layer_open_rounds,
            "num_rounds must be >= layer_open_rounds"
        );
        assert_eq!(
            inp.layer_opens.len(),
            cfg.layer_open_rounds,
            "layer_opens must have layer_open_rounds entries"
        );
        let levels = cfg.layer_open_levels;
        let consts = inlined_perm::Constants::pinned();
        for r in 0..cfg.layer_open_rounds {
            let lo = &inp.layer_opens[r];
            assert_eq!(
                lo.merkle_siblings.len(),
                levels,
                "layer open round {r}: merkle_siblings must have layer_open_levels entries"
            );
            let (e_fold, e_sib) = round_pairs[r];
            // round r authenticates the pair at node index (FS_index >> r); parity = bit r.
            let start_index = index >> r;
            let parity = (start_index & 1) == 1;
            let rb = lo_round_base(cfg, r);
            // layer commitment (bound root).
            row[rb..rb + DIGEST_ELEMS].copy_from_slice(&lo.layer_commit);
            // flattened pair (by parity) + salt.
            let flat = lo_flatten_pair(e_fold, e_sib, parity);
            row[rb + LO_EVAL0..rb + LO_EVAL0 + 2 * D].copy_from_slice(&flat);
            row[rb + LO_SALT..rb + LO_SALT + SALT_ELEMS].copy_from_slice(&lo.salt);
            // leaf-hash perms over flatten(evals) ‖ salt.
            let input = lo_leaf_input(e_fold, e_sib, parity, &lo.salt);
            let pre = leaf_hash_pre_states(&input);
            debug_assert_eq!(pre.len(), lo_leaf_perms(), "layer leaf perm count mismatch");
            for (k, st) in pre.iter().enumerate() {
                let cols = inlined_perm::generate_perm_cols(*st, &consts);
                let pb = rb + lo_leaf_perm_base(k);
                row[pb..pb + PERM_COLS].copy_from_slice(&cols);
            }
            let leaf = layer_leaf_digest(e_fold, e_sib, parity, &lo.salt);
            // Merkle path: fold bottom-to-top; level-j selector = bit (j+1) of start_index =
            // bit (r+1+j) of FS_index (the folded-node index of round r is FS_index >> (r+1)).
            let mut running = leaf;
            let mut node_idx = start_index >> 1;
            for (level, &sib) in lo.merkle_siblings.iter().enumerate() {
                let bit = (node_idx & 1) == 1;
                let lbase = lo_level_base(cfg, r, level);
                row[lbase + LO_ML_RUNNING..lbase + LO_ML_RUNNING + DIGEST_ELEMS]
                    .copy_from_slice(&running);
                row[lbase + LO_ML_SIBLING..lbase + LO_ML_SIBLING + DIGEST_ELEMS]
                    .copy_from_slice(&sib);
                let (a, b) = if bit { (sib, running) } else { (running, sib) };
                let mut input16 = [Val::ZERO; WIDTH];
                input16[..DIGEST_ELEMS].copy_from_slice(&a);
                input16[DIGEST_ELEMS..].copy_from_slice(&b);
                let cols = inlined_perm::generate_perm_cols(input16, &consts);
                row[lbase + LO_ML_PERM..lbase + LO_ML_PERM + PERM_COLS].copy_from_slice(&cols);
                running = core::array::from_fn(|i| cols[C_OUTPUT + i]);
                node_idx >>= 1;
            }
            debug_assert_eq!(
                running, lo.layer_commit,
                "layer open round {r}: in-AIR fold root != layer_commit (honest build)"
            );
        }
    }

    // (FP) final-poly check: bind the descent's fully-folded value == Horner(coeffs, x).
    if cfg.final_poly_len > 0 {
        assert_eq!(
            inp.final_poly_coeffs.len(),
            cfg.final_poly_len,
            "final_poly_coeffs must have final_poly_len entries"
        );
        let fb = fp_base(cfg);
        row[fb + FP_X] = inp.final_poly_x;
        for (c, &coeff) in inp.final_poly_coeffs.iter().enumerate() {
            let cb = fb + fp_coeff_base(c);
            row[cb..cb + D].copy_from_slice(&limbs(coeff));
        }
        debug_assert_eq!(
            folded_last,
            final_poly_eval(inp.final_poly_x, inp.final_poly_coeffs),
            "folded_last != final_poly(x) — (FP) honest build mismatch"
        );
    }

    // (b-hash) leaf-row hashing: compute the level-0 leaf digest IN-AIR as MyHash(row ‖ salt) via
    // the PaddingFreeSponge schedule, laying out the raw row/salt cells and one perm per absorb.
    if cfg.leaf_row_len > 0 && cfg.merkle_levels > 0 {
        assert_eq!(inp.leaf_row.len(), cfg.leaf_row_len, "leaf_row must have leaf_row_len entries");
        let consts = inlined_perm::Constants::pinned();
        // raw row + salt cells (absolute columns).
        row[lh_row(cfg)..lh_row(cfg) + cfg.leaf_row_len].copy_from_slice(inp.leaf_row);
        row[lh_salt(cfg)..lh_salt(cfg) + SALT_ELEMS].copy_from_slice(&inp.leaf_salt);
        // build the salted input and its sponge schedule.
        let mut input = Vec::with_capacity(lh_input_len(cfg));
        input.extend_from_slice(inp.leaf_row);
        input.extend_from_slice(&inp.leaf_salt);
        let pre_states = leaf_hash_pre_states(&input);
        debug_assert_eq!(pre_states.len(), lh_perms(cfg), "perm count != lh_perms");
        for (k, pre) in pre_states.iter().enumerate() {
            let perm_cols = inlined_perm::generate_perm_cols(*pre, &consts);
            let pb = lh_perm_base(cfg, k);
            row[pb..pb + PERM_COLS].copy_from_slice(&perm_cols);
        }
        // sanity: the derived digest matches the leaf the Merkle path uses.
        debug_assert_eq!(
            leaf_hash_digest(&input),
            inp.merkle_leaf,
            "MyHash(row‖salt) != merkle_leaf — leaf-hash builder mismatch"
        );
    }

    row
}

/// The reconstructed Merkle root of a packed window (last level's perm output[..8]).
pub fn window_merkle_root(cfg: &RecursiveConfig, row: &[Val]) -> [Val; DIGEST_ELEMS] {
    assert!(cfg.merkle_levels > 0);
    let lbase = merkle_level_base(cfg, cfg.merkle_levels - 1);
    let mut out = [Val::ZERO; DIGEST_ELEMS];
    out.copy_from_slice(&row[lbase + ML_PERM + C_OUTPUT..lbase + ML_PERM + C_OUTPUT + DIGEST_ELEMS]);
    out
}

/// Decode the bus index value of a packed window.
pub fn window_index(row: &[Val]) -> u32 {
    row[C_INDEX].as_canonical_u32()
}

/// Public values layout for the Merkle block: `leaf(DIGEST_ELEMS) ‖ root(DIGEST_ELEMS)`. Present
/// only when `merkle_levels > 0`. (No public values otherwise.)
pub const PV_LEAF: usize = 0;
pub const PV_ROOT: usize = PV_LEAF + DIGEST_ELEMS;
pub const N_PUBLIC_MERKLE: usize = PV_ROOT + DIGEST_ELEMS;

/// Octic multiply over `X^D = W` as in-AIR expressions (same formula as `fri_fold_air`).
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

/// In-AIR cleared FRI fold identity (★) `folded·2s == e0(beta+s) − e1(beta−s)` over deg-8.
fn fold_identity<AB: AirBuilder>(
    builder: &mut AB,
    e0: &[AB::Expr; D],
    e1: &[AB::Expr; D],
    beta: &[AB::Expr; D],
    s: AB::Expr,
    folded: &[AB::Expr; D],
) {
    let mut beta_plus_s = beta.clone();
    beta_plus_s[0] = beta[0].clone() + s.clone();
    let mut beta_minus_s = beta.clone();
    beta_minus_s[0] = beta[0].clone() - s.clone();
    let p0 = octic_mul::<AB>(e0, &beta_plus_s);
    let p1 = octic_mul::<AB>(e1, &beta_minus_s);
    let two_s = s.double();
    for k in 0..D {
        builder.assert_zero(folded[k].clone() * two_s.clone() - (p0[k].clone() - p1[k].clone()));
    }
}

/// `recursive_air` (capstone): index/challenge bus + all consumers in one window. Wires
/// (1) index↔draw, (2) fold point↔index tower, (3) FRI fold (★), (a) Merkle index↔bus,
/// (b) beta/zeta↔draws, (b′) DEEP OOD, (c) multi-round descent — config-gated.
pub struct RecursiveAir {
    pub cfg: RecursiveConfig,
}

impl RecursiveAir {
    /// Legacy single-window constructor (index↔draw + tower + single fold).
    pub fn single(log_height: usize) -> Self {
        Self { cfg: RecursiveConfig::single_window(log_height) }
    }
}

impl<F: Sync> BaseAir<F> for RecursiveAir {
    fn width(&self) -> usize {
        n_cols(&self.cfg)
    }
}

/// Reads the window's perm sub-block at an absolute offset (for the Merkle inlined perms).
struct WindowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> PermCells<AB> for WindowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for RecursiveAir {
    fn eval(&self, builder: &mut AB) {
        let cfg = self.cfg;
        let h = cfg.log_height;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let two = AB::F::from_u32(2);

        // ===== (1) index ↔ challenger draw (reuse index_bus logic) =====
        let bits: Vec<AB::Expr> = (0..FIELD_BITS).map(|i| l(C_BITS + i)).collect();
        for b in &bits {
            builder.assert_zero(b.clone() * (b.clone() - AB::Expr::ONE)); // booleanity
        }
        let mut acc = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for b in &bits {
            acc += b.clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(acc - l(C_DRAW_OUT));
        assert_canonical_pub::<AB>(builder, &bits);
        let mut idx = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for i in 0..h {
            idx += bits[i].clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(idx - l(C_INDEX));

        // ===== (2) fold point s ↔ index (committed product tower) =====
        let bases = tower_bases(h);
        builder.assert_zero(l(sp_col(0)) - AB::Expr::ONE);
        for j in 0..h {
            let rev_bit = bits[h - 1 - j].clone();
            let base_j = AB::Expr::from(bases[j]);
            let sel = AB::Expr::ONE + rev_bit * (base_j - AB::Expr::ONE);
            builder.assert_zero(l(sp_col(j + 1)) - l(sp_col(j)) * sel);
        }
        builder.assert_zero(l(C_S) - l(sp_col(h)));

        // ===== (3) FRI fold identity (★) at the BUS-BOUND point s (round 0) =====
        let e0: [AB::Expr; D] = core::array::from_fn(|i| l(C_E0 + i));
        let e1: [AB::Expr; D] = core::array::from_fn(|i| l(C_E1 + i));
        let beta: [AB::Expr; D] = core::array::from_fn(|i| l(C_BETA + i));
        let folded: [AB::Expr; D] = core::array::from_fn(|i| l(C_FOLDED + i));
        fold_identity::<AB>(builder, &e0, &e1, &beta, l(C_S), &folded);

        // ===== (c) multi-round descent: per-round (★) + chain-link + point-square =====
        let rounds = cfg.rounds();
        // Round-r reader helpers: round 0 uses the canonical columns; r≥1 uses the extra block.
        let round_e0 = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_E0 + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + i))
            }
        };
        let round_e1 = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_E1 + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + D + i))
            }
        };
        let round_beta = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_BETA + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + 2 * D + i))
            }
        };
        let round_s = |r: usize| -> AB::Expr {
            if r == 0 {
                l(C_S)
            } else {
                let b = extra_round_base(&cfg, r);
                l(b + 3 * D)
            }
        };
        let round_folded = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_FOLDED + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + 3 * D + 1 + i))
            }
        };
        for r in 1..rounds {
            // per-round (★) at this round's point.
            let er0 = round_e0(r);
            let er1 = round_e1(r);
            let erb = round_beta(r);
            let erf = round_folded(r);
            fold_identity::<AB>(builder, &er0, &er1, &erb, round_s(r), &erf);
            // (point-square): s_r == s_{r-1}².
            let s_prev = round_s(r - 1);
            builder.assert_zero(round_s(r) - s_prev.clone() * s_prev);
            // (chain-link): e0_r == folded_{r-1}, limb-for-limb.
            let prev_folded = round_folded(r - 1);
            for k in 0..D {
                builder.assert_zero(er0[k].clone() - prev_folded[k].clone());
            }
        }

        // ===== (b′) DEEP OOD block: C == Z_H(zeta)·Q with squaring tower =====
        if let Some(k) = cfg.deep_log_degree {
            let db = deep_base(&cfg);
            let zeta: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_ZETA + i));
            let q: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_Q + i));
            let c: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_C + i));
            // pow[0] == zeta.
            let pow0: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_POW + i));
            for i in 0..D {
                builder.assert_zero(pow0[i].clone() - zeta[i].clone());
            }
            // pow[j] == pow[j-1]².
            for j in 1..=k {
                let prev: [AB::Expr; D] =
                    core::array::from_fn(|i| l(db + DEEP_POW + (j - 1) * D + i));
                let cur: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_POW + j * D + i));
                let sq = octic_mul::<AB>(&prev, &prev);
                for i in 0..D {
                    builder.assert_zero(cur[i].clone() - sq[i].clone());
                }
            }
            // Z_H = pow[k] - 1; C == Z_H·Q.
            let powk: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_POW + k * D + i));
            let mut zh = powk;
            zh[0] = zh[0].clone() - AB::Expr::ONE;
            let zhq = octic_mul::<AB>(&zh, &q);
            for i in 0..D {
                builder.assert_zero(c[i].clone() - zhq[i].clone());
            }
        }

        // ===== (b) beta / zeta ↔ challenger SampleDraw columns =====
        if cfg.bind_beta_to_draws {
            let mut dbase = draw_block_base(&cfg);
            // bind each round beta to its draw block.
            for r in 0..rounds {
                let br = round_beta(r);
                for i in 0..D {
                    builder.assert_zero(br[i].clone() - l(dbase + i));
                }
                dbase += D;
            }
            // bind zeta to its draw block.
            if cfg.deep_log_degree.is_some() {
                let db = deep_base(&cfg);
                for i in 0..D {
                    builder.assert_zero(l(db + DEEP_ZETA + i) - l(dbase + i));
                }
            }
        }

        // ===== (a) Merkle path: inlined perms, index bit ↔ bus, chain, leaf/root public =====
        if cfg.merkle_levels > 0 {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let one = AB::Expr::ONE;
            for level in 0..cfg.merkle_levels {
                let lbase = merkle_level_base(&cfg, level);
                // index bit at this level IS bit_level of the bus decomposition (the binding).
                let bit = bits[level].clone();
                // placement: perm.inputs[..8]/[8..] = running/sibling per the BUS bit.
                for i in 0..DIGEST_ELEMS {
                    let running_i = l(lbase + ML_RUNNING + i);
                    let sibling_i = l(lbase + ML_SIBLING + i);
                    let in_a = l(lbase + ML_PERM + inlined_perm::C_INPUTS + i);
                    let in_b = l(lbase + ML_PERM + inlined_perm::C_INPUTS + DIGEST_ELEMS + i);
                    builder.assert_zero(
                        in_a - ((one.clone() - bit.clone()) * running_i.clone()
                            + bit.clone() * sibling_i.clone()),
                    );
                    builder.assert_zero(
                        in_b - ((one.clone() - bit.clone()) * sibling_i
                            + bit.clone() * running_i),
                    );
                }
                // inlined Poseidon2 round constraints over this level's perm block.
                eval_perm::<AB, _>(builder, &cells, lbase + ML_PERM, &consts);
                // chain: next level's running == this level's output (within the same row).
                if level + 1 < cfg.merkle_levels {
                    let nbase = merkle_level_base(&cfg, level + 1);
                    for i in 0..DIGEST_ELEMS {
                        let this_out = l(lbase + ML_PERM + C_OUTPUT + i);
                        let next_running = l(nbase + ML_RUNNING + i);
                        builder.assert_zero(next_running - this_out);
                    }
                }
            }
            // leaf/root binding to public values (leaf = level-0 running, root = last output).
            let pubs: Vec<AB::Expr> =
                builder.public_values().iter().map(|p| (*p).into()).collect();
            if pubs.len() == N_PUBLIC_MERKLE {
                let l0 = merkle_level_base(&cfg, 0);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(l(l0 + ML_RUNNING + i) - pubs[PV_LEAF + i].clone());
                }
                let llast = merkle_level_base(&cfg, cfg.merkle_levels - 1);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(
                        l(llast + ML_PERM + C_OUTPUT + i) - pubs[PV_ROOT + i].clone(),
                    );
                }
            }

            // ===== (a-root) reconstructed root == inner MAIN-TRACE commitment (bound input) =====
            // The last Merkle level's reconstructed root (output[..8]) — the root the FS-index-driven
            // path folds to — is constrained EQUAL to the inner proof's main-trace commitment carried
            // in the `main_commit` column block. Combined with the per-level index-bit↔bus binding
            // above (the path verified is the one the FS index SELECTS, not an adversary's), this says:
            // "the opened main-trace leaf at FS-index i authenticates to the inner main-trace root."
            // A path that folds to a DIFFERENT root than the committed inner root rejects.
            if cfg.bind_root_to_main_commit {
                let mb = mc_base(&cfg);
                let llast = merkle_level_base(&cfg, cfg.merkle_levels - 1);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(
                        l(llast + ML_PERM + C_OUTPUT + i) - l(mb + MC_MAIN_COMMIT + i),
                    );
                }
            }
        }

        // ===== (LO) COMPOSE the per-round FRI-LAYER open into the descent =====
        // For each commit-phase round r, RE-EXPRESS fri_layer_air::FriLayerOpenAir INSIDE this window
        // and bind its authenticated pair to the (c) descent's (round_e0(r), round_e1(r)). This makes
        // the descent fold consume the AUTHENTICATED sibling: e_sib is no longer free, it is the value
        // MyHash(flatten(e_fold,e_sib)‖salt)-authenticated against round r's layer commitment at the
        // FS-forced node (FS_index >> r). p3-fri verify_query (verifier.rs:508-540) constant-for-constant:
        //   - the arity-2 pair (evals[0],evals[1]) is (e_fold,e_sib) ordered by parity = FS bit r;
        //   - the leaf digest is the deployed MyHash(flatten ‖ salt) PaddingFreeSponge;
        //   - the path authenticates at folded-node index FS_index >> (r+1) (level-j selector = FS bit
        //     r+1+j), reconstructing the bound round-r layer commitment.
        if cfg.layer_open_rounds > 0 {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let one = AB::Expr::ONE;
            let levels = cfg.layer_open_levels;
            for r in 0..cfg.layer_open_rounds {
                let rb = lo_round_base(&cfg, r);
                // parity = FS index bit r (the in-group bit of start_index = FS_index >> r).
                let parity = bits[r].clone();
                // ----- pair placement by parity: evals[0]=(1-p)·e_fold+p·e_sib, evals[1]=swap -----
                // e_fold/e_sib are the DESCENT's round_e0(r)/round_e1(r) — the composition binding.
                let efold = round_e0(r);
                let esib = round_e1(r);
                for i in 0..D {
                    builder.assert_zero(
                        l(rb + LO_EVAL0 + i)
                            - ((one.clone() - parity.clone()) * efold[i].clone()
                                + parity.clone() * esib[i].clone()),
                    );
                    builder.assert_zero(
                        l(rb + LO_EVAL1 + i)
                            - ((one.clone() - parity.clone()) * esib[i].clone()
                                + parity.clone() * efold[i].clone()),
                    );
                }
                // ----- leaf-hash: derive MyHash(flatten(evals) ‖ salt) IN-AIR -----
                let m = LO_LEAF_INPUT_LEN;
                let n_perms = lo_leaf_perms();
                let input_cell = |g: usize| -> AB::Expr {
                    if g < 2 * D {
                        l(rb + LO_EVAL0 + g)
                    } else {
                        l(rb + LO_SALT + (g - 2 * D))
                    }
                };
                for k in 0..n_perms {
                    let pb = rb + lo_leaf_perm_base(k);
                    for i in 0..WIDTH {
                        let in_i = l(pb + inlined_perm::C_INPUTS + i);
                        let g = k * RATE + i;
                        if i < RATE && g < m {
                            builder.assert_zero(in_i - input_cell(g));
                        } else if k == 0 {
                            builder.assert_zero(in_i);
                        } else {
                            let prev = rb + lo_leaf_perm_base(k - 1);
                            builder.assert_zero(in_i - l(prev + C_OUTPUT + i));
                        }
                    }
                    eval_perm::<AB, _>(builder, &cells, pb, &consts);
                }
                // leaf digest = last leaf-hash permute output[..8]; bind it to level-0 running.
                let last_lh = rb + lo_leaf_perm_base(n_perms - 1);
                let l0 = lo_level_base(&cfg, r, 0);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(l(l0 + LO_ML_RUNNING + i) - l(last_lh + C_OUTPUT + i));
                }
                // ----- Merkle path: per-level placement by the FS-shifted node bit + perm + chain -----
                for level in 0..levels {
                    let lbase = lo_level_base(&cfg, r, level);
                    // selector = FS index bit (r+1+level) (the folded-node index of round r is
                    // FS_index >> (r+1); its bit `level` is FS bit r+1+level). NON-VACUOUS index
                    // binding: the path is authenticated at the node the FS index SELECTS for round r.
                    let bit = bits[r + 1 + level].clone();
                    for i in 0..DIGEST_ELEMS {
                        let running_i = l(lbase + LO_ML_RUNNING + i);
                        let sibling_i = l(lbase + LO_ML_SIBLING + i);
                        let in_a = l(lbase + LO_ML_PERM + inlined_perm::C_INPUTS + i);
                        let in_b = l(lbase + LO_ML_PERM + inlined_perm::C_INPUTS + DIGEST_ELEMS + i);
                        builder.assert_zero(
                            in_a - ((one.clone() - bit.clone()) * running_i.clone()
                                + bit.clone() * sibling_i.clone()),
                        );
                        builder.assert_zero(
                            in_b - ((one.clone() - bit.clone()) * sibling_i
                                + bit.clone() * running_i),
                        );
                    }
                    eval_perm::<AB, _>(builder, &cells, lbase + LO_ML_PERM, &consts);
                    if level + 1 < levels {
                        let nbase = lo_level_base(&cfg, r, level + 1);
                        for i in 0..DIGEST_ELEMS {
                            builder.assert_zero(
                                l(nbase + LO_ML_RUNNING + i) - l(lbase + LO_ML_PERM + C_OUTPUT + i),
                            );
                        }
                    }
                }
                // ----- root binding: reconstructed root == round-r layer commitment -----
                let llast = lo_level_base(&cfg, r, levels - 1);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(l(llast + LO_ML_PERM + C_OUTPUT + i) - l(rb + i));
                }
                // ----- (LO≡FS-β) UNIFY the authentication root with the FS-OBSERVED commitment -----
                // In real p3-fri (p3-fri-0.6.1 verifier.rs:300-310 vs :374-377/:535) there is ONE
                // commitment per commit-phase round r: `commit_phase_commits[r]` is BOTH `observe`d to
                // sample beta_r AND the MMCS root the round-r sibling `verify_batch`-opens against. Here
                // the descent authenticates against `layer_commit_r` (this LO block, `rb + 0..DIGEST_ELEMS`)
                // while the FS-β transcript observes `commit_phase_commit_r` (`fb_round_base(r) + FB_BLOCK`)
                // to squeeze beta_r. They were two SEPARATE bound blocks — an adversary could authenticate
                // siblings against a commitment DIFFERENT from the one that generated beta_r. When BOTH
                // blocks are present for round r (an FS-β round exists, i.e. `r < fs_beta_rounds`), CONSTRAIN
                // them equal limb-for-limb: the authentication root IS the FS-observed commitment. Then
                // tampering round r's observed commitment moves beta_r (FS-β squeeze) AND breaks this
                // round's sibling authentication — one digest, both effects, as in p3. When the LO block is
                // used standalone (`fs_beta_rounds == 0`, no FS-β block exists) this residual is absent and
                // `layer_commit_r` remains the sole bound root (prior behavior, unchanged).
                if r < cfg.fs_beta_rounds {
                    let fbr = fb_round_base(&cfg, r);
                    for k in 0..DIGEST_ELEMS {
                        builder.assert_zero(l(rb + k) - l(fbr + FB_BLOCK + k));
                    }
                }
            }
        }

        // ===== (FP) FINAL-POLY check inside the descent =====
        // After the last commit-phase round, the fully-folded value must equal the committed FRI final
        // polynomial at the query point x (p3-fri verifier.rs:402-409: folded == Σ coeff·x^k). x is a
        // BASE-field scalar, so acc·x scales each extension limb (no octic mul). A folded value !=
        // final_poly(x) or a tampered final-poly coefficient rejects.
        if cfg.final_poly_len > 0 {
            let fb = fp_base(&cfg);
            let x = l(fb + FP_X);
            let mut acc: [AB::Expr; D] = core::array::from_fn(|_| AB::Expr::ZERO);
            for c in (0..cfg.final_poly_len).rev() {
                let cb = fb + fp_coeff_base(c);
                for i in 0..D {
                    acc[i] = acc[i].clone() * x.clone() + l(cb + i);
                }
            }
            // bind to the descent's fully-folded value (round_folded(rounds-1)).
            let folded = round_folded(rounds - 1);
            for i in 0..D {
                builder.assert_zero(folded[i].clone() - acc[i].clone());
            }
        }

        // ===== (b-leaf) Merkle-opened leaf digest ↔ FRI round-0 e0 =====
        // The leaf the Merkle path authenticates (level-0 running, 8 lanes) IS the value folded by
        // the round-0 FRI fold (e0's 8 limbs). DIGEST_ELEMS == D == 8, so limb-for-limb. A mismatch
        // (an opened value different from what is folded) rejects.
        if cfg.bind_merkle_leaf_to_e0 && cfg.merkle_levels > 0 {
            debug_assert_eq!(DIGEST_ELEMS, D, "leaf↔e0 binding assumes DIGEST_ELEMS == D");
            let l0 = merkle_level_base(&cfg, 0);
            for i in 0..D {
                builder.assert_zero(l(l0 + ML_RUNNING + i) - l(C_E0 + i));
            }
        }

        // ===== (b-hash) DERIVE the Merkle level-0 leaf digest as MyHash(opened_row ‖ salt) =====
        // The level-0 running is no longer a free supplied digest: it is CONSTRAINED to be the output
        // of a real `PaddingFreeSponge<Perm,16,8,8>` (the deployed `MyHash`) over the salted opened
        // row. Each absorb-permute is the inlined Poseidon2 (`eval_perm`); the rate lanes are
        // OVERWRITTEN with the row/salt cells (and, for a trailing partial block, the un-absorbed
        // lanes + capacity are CARRIED from the previous permute), exactly the no-padding overwrite
        // sponge. So the Merkle path authenticates `H(row)`, and (with (b-leaf)) the FRI-folded e0 is
        // that same `H(row)`. A tampered row changes the digest ⇒ the path/leaf binding rejects.
        if cfg.leaf_row_len > 0 && cfg.merkle_levels > 0 {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let m = lh_input_len(&cfg);
            // input cell at global position g: row[0..leaf_row_len] then salt[..SALT_ELEMS].
            let input_cell = |g: usize| -> AB::Expr {
                if g < cfg.leaf_row_len {
                    l(lh_row(&cfg) + g)
                } else {
                    l(lh_salt(&cfg) + (g - cfg.leaf_row_len))
                }
            };
            let n_perms = lh_perms(&cfg);
            for k in 0..n_perms {
                let pb = lh_perm_base(&cfg, k);
                // Constrain perm k's INPUT lanes = the absorbed sponge state for block k.
                for i in 0..WIDTH {
                    let in_i = l(pb + inlined_perm::C_INPUTS + i);
                    let g = k * RATE + i;
                    if i < RATE && g < m {
                        // rate lane with an absorbed element: overwrite with the input cell.
                        builder.assert_zero(in_i - input_cell(g));
                    } else if k == 0 {
                        // first block, no carry: un-absorbed rate lanes + capacity start at ZERO.
                        builder.assert_zero(in_i);
                    } else {
                        // carry the previous permute output lane (partial-block rate tail + capacity).
                        let prev = lh_perm_base(&cfg, k - 1);
                        builder.assert_zero(in_i - l(prev + C_OUTPUT + i));
                    }
                }
                // full Poseidon2 permutation constraints over this absorb-permute block.
                eval_perm::<AB, _>(builder, &cells, pb, &consts);
            }
            // the leaf digest = the LAST permute's output[..DIGEST_ELEMS]; bind it to level-0 running.
            let last = lh_perm_base(&cfg, n_perms - 1);
            let l0 = merkle_level_base(&cfg, 0);
            for i in 0..DIGEST_ELEMS {
                builder.assert_zero(l(l0 + ML_RUNNING + i) - l(last + C_OUTPUT + i));
            }
        }

        // ===== (a-FS) INLINE THE CHALLENGER SPONGE for the index draw =====
        // The index draw C_DRAW_OUT (which (1) decomposes into the query index + fold point) is
        // bound to the constrained output of a real Poseidon2 permutation of the supplied sponge
        // state: perm.inputs == sponge_in (lane-for-lane), eval_perm constrains the full
        // permutation, and C_DRAW_OUT == perm.output[RATE-1] (the LIFO squeeze pop). A forged
        // draw_out inconsistent with the sponge state is rejected — the FS-soundness closure
        // (bounded to one draw; the full absorb transcript producing sponge_in stays deferred).
        if cfg.inline_sponge_draw {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let sb = sponge_base(&cfg);
            // perm inputs == sponge_in, lane-for-lane.
            for i in 0..WIDTH {
                builder.assert_zero(
                    l(sb + SP_PERM + inlined_perm::C_INPUTS + i) - l(sb + SP_SPONGE_IN + i),
                );
            }
            // full Poseidon2 permutation constraints over the sponge perm block.
            eval_perm::<AB, _>(builder, &cells, sb + SP_PERM, &consts);
            // index draw == squeeze pop = post-permute rate lane RATE-1 (challenger LIFO sample()).
            builder.assert_zero(l(C_DRAW_OUT) - l(sb + SP_PERM + C_OUTPUT + (RATE - 1)));
            // (a-FS+) successive draws 1..sponge_draws == the next LIFO pops off the SAME refilled
            // output buffer (no re-permute while out_buf non-empty): draw d == output[RATE-1-d].
            let n_draws = cfg.sponge_draw_count();
            for d in 1..n_draws {
                builder.assert_zero(
                    l(sb + SP_EXTRA_DRAWS + (d - 1)) - l(sb + SP_PERM + C_OUTPUT + (RATE - 1 - d)),
                );
            }
            // (a-FS++) absorb derivation of sponge_in: per challenger::duplexing, the squeeze
            // permute's input state is absorb(prior_state, absorbed, n). Constrain sponge_in to it
            // lane-for-lane, so sponge_in is NOT a free committed column but a constrained function
            // of the (committed) prior transcript state + the absorbed rate values + the length tag.
            if cfg.sponge_absorb {
                let n = cfg.sponge_absorb_n;
                debug_assert!(n <= RATE);
                let ab = sb + sp_absorb(&cfg);
                // rate lanes [0..n] == absorbed[0..n] (overwrite).
                for i in 0..n {
                    builder.assert_zero(l(sb + SP_SPONGE_IN + i) - l(ab + SPA_ABSORBED + i));
                }
                if n > 0 {
                    // rate lanes [n..RATE] == 0 (zeroed unfilled rate).
                    for i in n..RATE {
                        builder.assert_zero(l(sb + SP_SPONGE_IN + i));
                    }
                    // capacity slot RATE == prior_state[RATE] + n (the length tag).
                    builder.assert_zero(
                        l(sb + SP_SPONGE_IN + RATE)
                            - l(ab + SPA_PRIOR_STATE + RATE)
                            - AB::Expr::from(AB::F::from_u8(n as u8)),
                    );
                    // remaining capacity slots [RATE+1..WIDTH] carried from prior_state.
                    for i in (RATE + 1)..WIDTH {
                        builder.assert_zero(
                            l(sb + SP_SPONGE_IN + i) - l(ab + SPA_PRIOR_STATE + i),
                        );
                    }
                } else {
                    // n == 0 (squeeze-only permute, empty input buffer): challenger::duplexing
                    // overwrites NO rate lanes and adds NO length tag, so the whole state is carried
                    // from prior_state unchanged. (sponge_in == prior_state.)
                    for i in 0..WIDTH {
                        builder.assert_zero(
                            l(sb + SP_SPONGE_IN + i) - l(ab + SPA_PRIOR_STATE + i),
                        );
                    }
                }

                // ===== (a-FS+++) TRANSCRIPT-PREFIX ABSORB CHAIN feeding prior_state =====
                // Per round k = 1..=sponge_prefix_rounds, constrain
                //   state_k == permute(absorb(state_{k-1}, block_k, RATE))   (FULL-rate absorb)
                // with state_0 PINNED to the new() zero boundary, and state_K bound into the duplex
                // step's prior_state. This makes prior_state the constrained output of the preceding
                // absorb chain — not a free committed column. Each round reuses the FULL-rate absorb
                // derivation + one eval_perm. The pre-permute input lives in perm_k.inputs; we bind
                // it to absorb(state_{k-1}, block_k, RATE) lane-for-lane (state_{k-1} is the zero
                // boundary for k==1, else perm_{k-1}.output), then eval_perm constrains the permute,
                // and state_k = perm_k.output[..WIDTH].
                let rounds = cfg.sponge_prefix_rounds;
                if rounds > 0 {
                    // reader for state_{k-1}: the all-zero boundary for k==1, else perm_{k-1}.output.
                    let state_prev = |k: usize, i: usize| -> AB::Expr {
                        if k == 1 {
                            AB::Expr::ZERO // state_0 == new() zero boundary
                        } else {
                            let pp = sb + prefix_round_base(&cfg, k - 1);
                            l(pp + SPP_PERM + C_OUTPUT + i)
                        }
                    };
                    let rate_expr = AB::Expr::from(AB::F::from_u8(RATE as u8));
                    for k in 1..=rounds {
                        let pb = sb + prefix_round_base(&cfg, k);
                        let inb = pb + SPP_PERM + inlined_perm::C_INPUTS;
                        // FULL-rate absorb (n == RATE): rate lanes [0..RATE] == block_k (overwrite).
                        for i in 0..RATE {
                            builder.assert_zero(l(inb + i) - l(pb + SPP_BLOCK + i));
                        }
                        // capacity slot RATE == state_{k-1}[RATE] + RATE (length tag, n == RATE).
                        builder.assert_zero(
                            l(inb + RATE) - state_prev(k, RATE) - rate_expr.clone(),
                        );
                        // remaining capacity slots [RATE+1..WIDTH] carried from state_{k-1}.
                        for i in (RATE + 1)..WIDTH {
                            builder.assert_zero(l(inb + i) - state_prev(k, i));
                        }
                        // full Poseidon2 permutation over this round's perm block.
                        eval_perm::<AB, _>(builder, &cells, pb + SPP_PERM, &consts);
                    }
                    // state_K == prior_state, limb-for-limb (the chain output feeds the duplex step).
                    let lastp = sb + prefix_round_base(&cfg, rounds);
                    for i in 0..WIDTH {
                        builder.assert_zero(
                            l(lastp + SPP_PERM + C_OUTPUT + i) - l(ab + SPA_PRIOR_STATE + i),
                        );
                    }
                    // (a-FS+++) optionally bind each round's absorbed block to the public transcript:
                    // public value (k-1)*RATE + i == block_k[i] (the actual observed commitments + IO).
                    if cfg.prefix_blocks_public {
                        let pubs: Vec<AB::Expr> =
                            builder.public_values().iter().map(|p| (*p).into()).collect();
                        if pubs.len() == rounds * RATE {
                            for k in 1..=rounds {
                                let pb = sb + prefix_round_base(&cfg, k);
                                for i in 0..RATE {
                                    builder.assert_zero(
                                        l(pb + SPP_BLOCK + i) - pubs[(k - 1) * RATE + i].clone(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        // ===== (c-pub) public-IO statement preservation (outer == observed == inner) =====
        if cfg.public_io_len > 0 {
            let pb = public_io_base(&cfg);
            for j in 0..cfg.public_io_len {
                let e = pb + j * PIO_STRIDE;
                builder.assert_zero(l(e + PIO_OUTER) - l(e + PIO_OBSERVED));
                builder.assert_zero(l(e + PIO_OBSERVED) - l(e + PIO_INNER));
            }
        }

        // ===== (FS-αζ) SQUEEZE alpha + zeta from the inline FS challenger, in p3 order =====
        // p3-uni-stark / leaf verifier FS order (verifier.rs:400-430):
        //   observe(metadata ‖ main-trace commitment ‖ public values)  [pre-alpha prefix rounds]
        //   alpha = sample_algebra_element()                            [SAMPLE alpha]
        //   observe(quotient-chunks commitment)                        [quotient-observe round]
        //   zeta  = sample_algebra_element()                           [SAMPLE zeta]
        // Each round is a FULL-rate (n == RATE) absorb-permute from the new() zero boundary (the same
        // primitive as the index-draw prefix chain). alpha/zeta are the LIFO squeeze pops of the
        // last pre-alpha / quotient permute (challenge limb i == sample() call i == output[RATE-1-i],
        // matching sample_algebra_element = from_basis_coefficients_fn(|_| self.sample())). The
        // squeezed alpha/zeta are then CONSTRAINED to equal the DEEP folder's alpha/zeta cells, so
        // those are NO LONGER free inputs — a forger cannot pick alpha/zeta independent of the
        // observed (inner commitments + public digest) transcript.
        if cfg.fs_alpha_zeta {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let rate_expr = AB::Expr::from(AB::F::from_u8(RATE as u8));
            let pre_rounds = cfg.fs_pre_alpha_rounds;

            // state_{r} reader: r == 0 is the new() zero boundary; else FS round (r-1)'s perm output.
            let fs_state_prev = |r: usize, i: usize| -> AB::Expr {
                if r == 0 {
                    AB::Expr::ZERO
                } else {
                    let pp = fs_round_base(&cfg, r - 1);
                    l(pp + FS_PERM + C_OUTPUT + i)
                }
            };
            // Constrain each pre-alpha FULL-rate absorb-permute round.
            for r in 0..pre_rounds {
                let rb = fs_round_base(&cfg, r);
                let inb = rb + FS_PERM + inlined_perm::C_INPUTS;
                // FULL-rate absorb: rate lanes [0..RATE] == block_r (overwrite).
                for i in 0..RATE {
                    builder.assert_zero(l(inb + i) - l(rb + FS_BLOCK + i));
                }
                // capacity slot RATE == state_{r}[RATE] + RATE (length tag, n == RATE).
                builder.assert_zero(l(inb + RATE) - fs_state_prev(r, RATE) - rate_expr.clone());
                // remaining capacity slots [RATE+1..WIDTH] carried from state_{r}.
                for i in (RATE + 1)..WIDTH {
                    builder.assert_zero(l(inb + i) - fs_state_prev(r, i));
                }
                // full Poseidon2 permutation over this round's perm block.
                eval_perm::<AB, _>(builder, &cells, rb + FS_PERM, &consts);
            }

            // The pre-alpha state (the squeeze source for alpha): the last pre-alpha round's perm
            // output, or the zero boundary if there are no pre-alpha rounds.
            let pre_alpha_out = |i: usize| -> AB::Expr { fs_state_prev(pre_rounds, i) };

            // alpha == LIFO squeeze of the pre-alpha state's rate: alpha_limb[i] == out[RATE-1-i].
            let ab = fs_alpha(&cfg);
            for i in 0..D {
                builder.assert_zero(l(ab + i) - pre_alpha_out(RATE - 1 - i));
            }

            // quotient-observe round: absorb the quotient commitment into the (UNCHANGED, sampling
            // does not mutate sponge_state) pre-alpha state, then permute. Its squeeze is zeta.
            let qb = fs_quot_round_base(&cfg);
            let qinb = qb + FS_PERM + inlined_perm::C_INPUTS;
            for i in 0..RATE {
                builder.assert_zero(l(qinb + i) - l(qb + FS_BLOCK + i));
            }
            builder.assert_zero(l(qinb + RATE) - pre_alpha_out(RATE) - rate_expr.clone());
            for i in (RATE + 1)..WIDTH {
                builder.assert_zero(l(qinb + i) - pre_alpha_out(i));
            }
            eval_perm::<AB, _>(builder, &cells, qb + FS_PERM, &consts);

            // zeta == LIFO squeeze of the quotient round's state rate: zeta_limb[i] == out[RATE-1-i].
            let zb = fs_zeta(&cfg);
            for i in 0..D {
                builder.assert_zero(l(zb + i) - l(qb + FS_PERM + C_OUTPUT + (RATE - 1 - i)));
            }

            // ----- BIND the FS-squeezed alpha/zeta to the DEEP folder's consumer cells -----
            // residual: deep_alpha − squeezed_alpha == 0, deep_zeta − squeezed_zeta == 0 ⇒ the DEEP
            // folder's alpha/zeta are NO LONGER free; they are the FS challenges.
            if cfg.deep_log_degree.is_some() {
                let db = deep_base(&cfg);
                for i in 0..D {
                    builder.assert_zero(l(db + DEEP_ALPHA + i) - l(ab + i));
                    builder.assert_zero(l(db + DEEP_ZETA + i) - l(zb + i));
                }
            }

            // (FS-αζ) optionally bind each round's absorbed block to the PUBLIC transcript: public
            // value r*RATE + i == FS round r's block (rounds 0..pre_rounds = pre-alpha prefix, round
            // pre_rounds = quotient commitment). This ties the squeeze to the ACTUAL observed inner
            // commitments + public digest, so a forger cannot fix alpha/zeta independent of the
            // statement.
            if cfg.fs_blocks_public {
                let pubs: Vec<AB::Expr> =
                    builder.public_values().iter().map(|p| (*p).into()).collect();
                let total = pre_rounds + 1;
                if pubs.len() == total * RATE {
                    for r in 0..pre_rounds {
                        let rb = fs_round_base(&cfg, r);
                        for i in 0..RATE {
                            builder.assert_zero(l(rb + FS_BLOCK + i) - pubs[r * RATE + i].clone());
                        }
                    }
                    for i in 0..RATE {
                        builder.assert_zero(l(qb + FS_BLOCK + i) - pubs[pre_rounds * RATE + i].clone());
                    }
                }
            }
        }

        // ===== (FS-β) SQUEEZE the FRI commit-phase betas, CONTINUING the post-zeta transcript =====
        // p3-fri verifier FS order (p3-fri-0.6.1 verifier.rs:299-312): for each commit-phase round r,
        //   observe(commit_phase_commit_r)  → sample_algebra_element() = beta_r
        // The sponge state is CARRIED: state_0 = the post-zeta state (the quotient-observe round's
        // perm output; sampling zeta is non-mutating in a duplex challenger), state_{r+1} = round r's
        // perm output. So alpha, zeta, and ALL betas come from ONE ordered sponge transcript. Each
        // beta_r is bound limb-for-limb to the (c) descent round_beta(r) cell, so the FRI fold uses the
        // FS-squeezed challenge — REPLACING the free-column bind_beta_to_draws pin for these rounds. A
        // beta != the squeezed beta_r is rejected; changing comm_r changes beta_r AND every subsequent
        // beta (the carried state threads the rounds).
        if cfg.fs_beta_rounds > 0 {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let rate_expr = AB::Expr::from(AB::F::from_u8(RATE as u8));
            let qb = fs_quot_round_base(&cfg);

            // ===== (FS-bca) FRI batch-combination alpha (verifier.rs:195) =====
            // A SINGLE squeeze-only permute taken after zeta with NO observe: because zeta drained the
            // output buffer, the FRI verifier's `sample_algebra_element()` triggers a SQUEEZE-ONLY
            // duplexing (n == 0) of the post-zeta state — NO rate overwrite, NO length tag, the whole
            // state carried into the permute. alpha is its top-8 LIFO pops. The beta chain's state_0 is
            // THIS permute output (the draw-count shift). Omitting/reordering it shifts every beta.
            if cfg.fs_batch_combo_alpha {
                let bb = bca_base(&cfg);
                let binb = bb + BCA_PERM + inlined_perm::C_INPUTS;
                // squeeze-only: the permute input is the post-zeta state UNCHANGED (every lane copied,
                // including capacity slot RATE — NO length tag added since n == 0).
                for i in 0..WIDTH {
                    builder.assert_zero(l(binb + i) - l(qb + FS_PERM + C_OUTPUT + i));
                }
                // full Poseidon2 permutation over the batch-combo perm block.
                eval_perm::<AB, _>(builder, &cells, bb + BCA_PERM, &consts);
                // alpha == LIFO squeeze of the batch-combo permute's output rate.
                for i in 0..D {
                    builder.assert_zero(
                        l(bb + BCA_ALPHA + i) - l(bb + BCA_PERM + C_OUTPUT + (RATE - 1 - i)),
                    );
                }
            }

            // state_{r} reader: r == 0 is the beta-chain start (the batch-combo permute output when
            // `fs_batch_combo_alpha`, else the post-zeta state = quotient-observe round's perm output);
            // r >= 1 is FS-β round (r-1)'s perm output.
            let fb_state_prev = |r: usize, i: usize| -> AB::Expr {
                if r == 0 {
                    if cfg.fs_batch_combo_alpha {
                        l(bca_base(&cfg) + BCA_PERM + C_OUTPUT + i)
                    } else {
                        l(qb + FS_PERM + C_OUTPUT + i)
                    }
                } else {
                    let pp = fb_round_base(&cfg, r - 1);
                    l(pp + FB_PERM + C_OUTPUT + i)
                }
            };
            for r in 0..cfg.fs_beta_rounds {
                let rb = fb_round_base(&cfg, r);
                let inb = rb + FB_PERM + inlined_perm::C_INPUTS;
                // observe(commit_phase_commit_r): FULL-rate absorb — rate lanes [0..RATE] == block_r.
                for i in 0..RATE {
                    builder.assert_zero(l(inb + i) - l(rb + FB_BLOCK + i));
                }
                // capacity slot RATE == state_{r}[RATE] + RATE (length tag, n == RATE).
                builder.assert_zero(l(inb + RATE) - fb_state_prev(r, RATE) - rate_expr.clone());
                // remaining capacity slots [RATE+1..WIDTH] carried from state_{r}.
                for i in (RATE + 1)..WIDTH {
                    builder.assert_zero(l(inb + i) - fb_state_prev(r, i));
                }
                // full Poseidon2 permutation over this round's perm block.
                eval_perm::<AB, _>(builder, &cells, rb + FB_PERM, &consts);
                // beta_r == LIFO squeeze of this round's perm output rate: beta_r[i] == out[RATE-1-i].
                for i in 0..D {
                    builder.assert_zero(
                        l(rb + FB_BETA + i) - l(rb + FB_PERM + C_OUTPUT + (RATE - 1 - i)),
                    );
                }
                // BIND beta_r to the (c) descent round_beta(r) cell — the FRI fold uses THIS challenge.
                let consumer = round_beta(r);
                for i in 0..D {
                    builder.assert_zero(consumer[i].clone() - l(rb + FB_BETA + i));
                }
            }

            // (FS-β) optionally bind each commit-phase round's absorbed block to the PUBLIC transcript:
            // public value r*RATE + i == FS-β round r's block (the actual fold-round commitments).
            if cfg.fs_betas_public {
                let pubs: Vec<AB::Expr> =
                    builder.public_values().iter().map(|p| (*p).into()).collect();
                if pubs.len() == cfg.fs_beta_rounds * RATE {
                    for r in 0..cfg.fs_beta_rounds {
                        let rb = fb_round_base(&cfg, r);
                        for i in 0..RATE {
                            builder.assert_zero(l(rb + FB_BLOCK + i) - pubs[r * RATE + i].clone());
                        }
                    }
                }
            }

            // ===== (FS-qi) UNIFY the query-index draw onto the post-last-beta transcript =====
            // p3-fri verifier FS order (p3-fri-0.6.1 verifier.rs:317-356): after the last beta,
            //   observe_algebra_slice(final_poly) → observe(log_arity)* → check_witness(query_pow) →
            //   per query: index = sample_bits(...)
            // All observes feed the SAME carried sponge (state_0 = post-last-beta state = the last FS-β
            // round's perm output), modeled here as RATE-aligned FULL-rate observe rounds. The index
            // draw `C_DRAW_OUT` is BOUND to the top LIFO pop of the LAST observe round's perm rate —
            // CONTINUING the one ordered transcript instead of the separate (a-FS) zero-boundary chain.
            // Changing the final polynomial OR the query-PoW witness changes the carried state and hence
            // the index. The (1) index↔draw decomposition above then consumes THIS unified draw.
            if cfg.fs_final_blocks > 0 {
                // state_{r} reader for the post-beta chain: r == 0 is the post-last-beta state (the last
                // FS-β round's perm output); r >= 1 is FS-qi round (r-1)'s perm output.
                let last_beta = fb_round_base(&cfg, cfg.fs_beta_rounds - 1);
                let fq_state_prev = |r: usize, i: usize| -> AB::Expr {
                    if r == 0 {
                        l(last_beta + FB_PERM + C_OUTPUT + i)
                    } else {
                        let pp = fq_round_base(&cfg, r - 1);
                        l(pp + FQ_PERM + C_OUTPUT + i)
                    }
                };
                for r in 0..cfg.fs_final_blocks {
                    let rb = fq_round_base(&cfg, r);
                    let inb = rb + FQ_PERM + inlined_perm::C_INPUTS;
                    // observe(block_r): FULL-rate absorb — rate lanes [0..RATE] == block_r.
                    for i in 0..RATE {
                        builder.assert_zero(l(inb + i) - l(rb + FQ_BLOCK + i));
                    }
                    // capacity slot RATE == state_{r}[RATE] + RATE (length tag, n == RATE).
                    builder.assert_zero(l(inb + RATE) - fq_state_prev(r, RATE) - rate_expr.clone());
                    // remaining capacity slots [RATE+1..WIDTH] carried from state_{r}.
                    for i in (RATE + 1)..WIDTH {
                        builder.assert_zero(l(inb + i) - fq_state_prev(r, i));
                    }
                    // full Poseidon2 permutation over this round's perm block.
                    eval_perm::<AB, _>(builder, &cells, rb + FQ_PERM, &consts);
                }
                // index draw_out == top LIFO pop of the LAST observe round's perm output rate. (The
                // last full-rate observe refilled the output buffer and emptied the input buffer, so the
                // index sample() performs NO further permute — it pops output[RATE-1].)
                let last_fq = fq_round_base(&cfg, cfg.fs_final_blocks - 1);
                builder.assert_zero(
                    l(C_DRAW_OUT) - l(last_fq + FQ_PERM + C_OUTPUT + (RATE - 1)),
                );

                // (FS-qi) optionally bind each post-beta observe block to the PUBLIC transcript:
                // public value r*RATE + i == FS-qi round r's block (final_poly ‖ log_arities ‖ PoW).
                if cfg.fs_final_blocks_public {
                    let pubs: Vec<AB::Expr> =
                        builder.public_values().iter().map(|p| (*p).into()).collect();
                    if pubs.len() == cfg.fs_final_blocks * RATE {
                        for r in 0..cfg.fs_final_blocks {
                            let rb = fq_round_base(&cfg, r);
                            for i in 0..RATE {
                                builder
                                    .assert_zero(l(rb + FQ_BLOCK + i) - pubs[r * RATE + i].clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fri_fold::reverse_bits_len;
    use crate::merkle::{compressor, Digest};
    use p3_air::check_all_constraints;
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};
    use p3_matrix::dense::RowMajorMatrix;
    use p3_symmetric::PseudoCompressionFunction;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }
    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }
    fn cfg_single(h: usize) -> RecursiveConfig {
        RecursiveConfig::single_window(h)
    }
    fn air(cfg: RecursiveConfig) -> RecursiveAir {
        RecursiveAir { cfg }
    }
    /// Single-fold matrix (legacy shape) from (draw, e0, e1, beta) tuples.
    fn matrix_single(
        queries: &[(Val, Challenge, Challenge, Challenge)],
        h: usize,
    ) -> RowMajorMatrix<Val> {
        let cfg = cfg_single(h);
        let mut flat = Vec::new();
        for &(d, e0, e1, beta) in queries {
            flat.extend_from_slice(&pack_window(&cfg, &WindowInputs::single(d, e0, e1, beta)));
        }
        RowMajorMatrix::new(flat, n_cols(&cfg))
    }

    // ---------------------------------------------------------------------------------
    // Existing single-window cross-bindings (1)(2)(3) — preserved.
    // ---------------------------------------------------------------------------------

    #[test]
    fn tower_reproduces_native_fold_point() {
        for h in [4usize, 5, 8, 12, 20] {
            for raw in [0u32, 1, 3, 6, 17, 0x9999, 0xABCDE, 1_000_003] {
                let d = Val::from_u32(raw);
                let cfg = cfg_single(h);
                let row = pack_window(&cfg, &WindowInputs::single(d, ext(1), ext(2), ext(3)));
                let index = (raw & ((1 << h) - 1)) as usize;
                let expected = fold_point(h, index);
                assert_eq!(row[C_S], expected, "tower s != fold_point at h={h}, raw={raw:#x}");
                assert_eq!(row[sp_col(h)], expected, "sp[h] != fold_point");
                let g = Val::two_adic_generator(h + 1);
                assert_eq!(expected, g.exp_u64(reverse_bits_len(index, h) as u64));
            }
        }
    }

    #[test]
    fn air_accepts_honest_windows() {
        for &h in &[5usize, 8, 12] {
            let queries: Vec<_> = [0u32, 1, 7, 0x9999, 0xABCDE, 1_000_003]
                .iter()
                .enumerate()
                .map(|(k, &raw)| {
                    let kk = k as u32;
                    (Val::from_u32(raw), ext_multi(10 + kk), ext_multi(40 + kk), ext_multi(70 + kk))
                })
                .collect();
            let report = check_all_constraints(&air(cfg_single(h)), &matrix_single(&queries, h), &[], None);
            assert!(report.is_ok(), "honest windows rejected at h={h}: {} failures", report.failures.len());
        }
    }

    #[test]
    fn air_rejects_forged_index() {
        let h = 12;
        let mut m = matrix_single(&[(Val::from_u32(0xABCDE), ext_multi(1), ext_multi(2), ext_multi(3))], h);
        m.values[C_INDEX] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_forged_fold_point() {
        let h = 8;
        let draw = Val::from_u32(0xABCDE);
        let canon = draw.as_canonical_u32();
        let honest_index = (canon & ((1 << h) - 1)) as usize;
        let forged_index = honest_index ^ 1;
        let s_forged = fold_point(h, forged_index);
        assert_ne!(fold_point(h, honest_index), s_forged);
        let cfg = cfg_single(h);
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, ext_multi(4), ext_multi(5), ext_multi(6)));
        row[C_S] = s_forged;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_tower() {
        let h = 12;
        let cfg = cfg_single(h);
        let mut row = pack_window(&cfg, &WindowInputs::single(Val::from_u32(0x55555), ext_multi(7), ext_multi(8), ext_multi(9)));
        row[sp_col(6)] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_bit_index_point_desync() {
        let h = 8;
        let draw = Val::from_u32(0xABCD0);
        let cfg = cfg_single(h);
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        row[C_BITS + 0] = Val::ONE;
        row[C_DRAW_OUT] += Val::ONE;
        row[C_INDEX] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_eval() {
        let h = 12;
        let mut m = matrix_single(&[(Val::from_u32(0x1234), ext_multi(11), ext_multi(12), ext_multi(13))], h);
        m.values[C_E0 + 2] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_wrong_folded() {
        let h = 8;
        let mut m = matrix_single(&[(Val::from_u32(0x9999), ext_multi(14), ext_multi(15), ext_multi(16))], h);
        m.values[C_FOLDED + 1] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_beta() {
        let h = 12;
        let mut m = matrix_single(&[(Val::from_u32(0x4242), ext_multi(17), ext_multi(18), ext_multi(19))], h);
        m.values[C_BETA + 4] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_binds_draw_to_index_to_fold_point() {
        let h = 10;
        let draw = Val::from_u32(0x3ABCD);
        let cfg = cfg_single(h);
        let honest = matrix_single(&[(draw, ext_multi(21), ext_multi(22), ext_multi(23))], h);
        assert!(check_all_constraints(&air(cfg), &honest, &[], None).is_ok());

        let honest_index = (draw.as_canonical_u32() & ((1 << h) - 1)) as usize;
        let forged_index = honest_index ^ 0b101;
        let s_forged = fold_point(h, forged_index);
        let (e0, e1, beta) = (ext_multi(31), ext_multi(32), ext_multi(33));
        let folded = fold_pair(e0, e1, beta, s_forged);
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, e0, e1, beta));
        row[C_S] = s_forged;
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded));
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "fold at a forged-index point accepted — draw→index→point binding not load-bearing"
        );
    }

    #[test]
    fn air_accepts_multirow() {
        let h = 12;
        let queries: Vec<_> = (0..8u32)
            .map(|q| {
                (Val::from_u32(0x40000 + q * 0x1111), ext_multi(100 + q), ext_multi(200 + q), ext_multi(300 + q))
            })
            .collect();
        let report = check_all_constraints(&air(cfg_single(h)), &matrix_single(&queries, h), &[], None);
        assert!(report.is_ok(), "honest multi-row recursive_air trace rejected: {} failures", report.failures.len());
    }

    // ---------------------------------------------------------------------------------
    // (a) MERKLE INDEX ↔ BUS — Merkle path inlined in the window, index bit bound to bits[level].
    // ---------------------------------------------------------------------------------

    fn digest(seed: u32) -> Digest {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32)))
    }

    /// Reference binary Merkle tree built with the deployed compression (mirror of merkle_air).
    struct RefTree {
        levels: Vec<Vec<Digest>>,
    }
    impl RefTree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                let next: Vec<Digest> =
                    leaves.chunks_exact(2).map(|p| c.compress([p[0], p[1]])).collect();
                levels.push(next.clone());
                leaves = next;
            }
            Self { levels }
        }
        fn root(&self) -> Digest {
            self.levels.last().unwrap()[0]
        }
        fn auth_path(&self, index: usize) -> Vec<Digest> {
            let mut sibs = Vec::new();
            let mut idx = index;
            for level in &self.levels[..self.levels.len() - 1] {
                sibs.push(level[idx ^ 1]);
                idx >>= 1;
            }
            sibs
        }
    }

    fn pubs(leaf: Digest, root: Digest) -> Vec<Val> {
        let mut p = Vec::with_capacity(N_PUBLIC_MERKLE);
        p.extend_from_slice(&leaf);
        p.extend_from_slice(&root);
        p
    }

    /// Config with a Merkle block of `levels` levels at log-height = levels (so the bus index has
    /// exactly the bits the path consumes, and the leaf-layer fold point uses the same index).
    fn cfg_merkle(levels: usize) -> RecursiveConfig {
        RecursiveConfig { merkle_levels: levels, ..RecursiveConfig::single_window(levels) }
    }

    /// Build a Merkle-augmented window for the query whose index is `draw`'s low `levels` bits,
    /// opening leaf `leaf` in `tree`.
    fn merkle_window(cfg: &RecursiveConfig, draw: Val, tree: &RefTree, leaf: Digest, index: usize) -> Vec<Val> {
        let sibs = tree.auth_path(index);
        let inp = WindowInputs {
            draw_out: draw,
            e0: ext_multi(1),
            e1: ext_multi(2),
            beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
            deep_alpha: Challenge::ZERO,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest Merkle path opened at the FS-forced index (= draw's low bits) ACCEPTS,
    /// folding to the public root, with the per-level index bits BOUND to the bus decomposition.
    #[test]
    fn air_accepts_merkle_path_bound_to_bus() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        // pick draws whose low `levels` bits hit several distinct indices.
        for raw in [0u32, 1, 6, 0xB, 0xD, 0xF, 0xABCDA] {
            let draw = Val::from_u32(raw);
            let index = (draw.as_canonical_u32() as usize) & (n - 1);
            let row = merkle_window(&cfg, draw, &tree, leaves[index], index);
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None);
            assert!(report.is_ok(), "honest bus-bound Merkle path rejected (raw={raw:#x}): {} failures", report.failures.len());
        }
    }

    /// (a) FORGERY — MERKLE AT THE WRONG INDEX: open the Merkle path at a DIFFERENT index than the
    /// bus-forced query (the fold point's index). The path's per-level index bits then disagree
    /// with `bits[level]` of the draw decomposition ⇒ the placement constraint (bound to the bus)
    /// rejects. THIS is binding (a): the Merkle opening and the FRI fold must answer the SAME query.
    #[test]
    fn air_rejects_merkle_at_wrong_index() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        // Honest bus index from the draw.
        let draw = Val::from_u32(0b0110);
        let honest_index = (draw.as_canonical_u32() as usize) & (n - 1);
        // Build a path/leaf for a DIFFERENT index (a favorable query the prover wants to answer).
        let forged_index = honest_index ^ 0b101;
        assert_ne!(forged_index, honest_index);
        // Pack the Merkle blocks for the FORGED index (low bits = forged_index), then overwrite the
        // bus block (draw_out/bits/index/tower/fold) to the HONEST query. Now bits[level] say
        // honest_index but the Merkle perm placements are for forged_index ⇒ (a) binding rejects.
        let forged_draw = Val::from_u32(forged_index as u32);
        let sibs = tree.auth_path(forged_index);
        let inp = WindowInputs {
            draw_out: forged_draw,
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaves[forged_index],
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        let mut row = pack_window(&cfg, &inp);
        // CONTROL: the FORGED Merkle blocks are internally valid — with a MATCHING forged bus
        // (bits = forged_index), the same row ACCEPTS (every Merkle path folds to the same tree
        // root; leaf/root boundaries hold). So the ONLY thing that can reject the swapped-bus row
        // below is the index-bit↔bus binding (a), not the perm/fold/root machinery.
        let control = RowMajorMatrix::new(row.clone(), n_cols(&cfg));
        assert!(
            check_all_constraints(&air(cfg), &control, &pubs(leaves[forged_index], root), None).is_ok(),
            "forged-index control row should accept with its own (forged) bus — test would prove nothing otherwise"
        );
        // Now overwrite the bus block (draw_out/bits/index/tower/fold) with the HONEST query. The
        // Merkle perm placements remain for forged_index; bits[level] now say honest_index.
        let honest = pack_window(&cfg, &WindowInputs {
            draw_out: draw, e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[], merkle_leaf: leaves[honest_index],
            merkle_siblings: &tree.auth_path(honest_index),
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        });
        for c in C_DRAW_OUT..tower_end(levels) {
            row[c] = honest[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &pubs(leaves[forged_index], root), None).is_ok(),
            "a Merkle path opened at a DIFFERENT index than the fold point was accepted — (a) not load-bearing"
        );
    }

    /// (a) FORGERY — tampered sibling ⇒ recomputed node diverges ⇒ wrong root ⇒ rejected.
    #[test]
    fn air_rejects_merkle_tampered_sibling() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0011);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let mut row = merkle_window(&cfg, draw, &tree, leaves[index], index);
        let lbase = merkle_level_base(&cfg, 0);
        row[lbase + ML_SIBLING] += Val::ONE; // tamper the level-0 sibling
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None).is_ok());
    }

    /// (a) FORGERY — a tampered INTERIOR perm cell of a Merkle level ⇒ rejected by the inlined
    /// Poseidon2 round constraints (the perm sub-block is fully constrained inside the window).
    #[test]
    fn air_rejects_merkle_tampered_perm_interior() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0101);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let mut row = merkle_window(&cfg, draw, &tree, leaves[index], index);
        let lbase = merkle_level_base(&cfg, 0);
        row[lbase + ML_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None).is_ok());
    }

    /// (a) FORGERY — wrong root public value ⇒ rejected by the last-level root boundary.
    #[test]
    fn air_rejects_merkle_wrong_root() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let draw = Val::from_u32(0b1001);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let row = merkle_window(&cfg, draw, &tree, leaves[index], index);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let mut bad_root = tree.root();
        bad_root[0] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], bad_root), None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (a-root) FS-index-driven main-trace Merkle opening verified against the inner
    // MAIN-TRACE commitment carried as a BOUND recursion input.
    //
    // The (a) bus binding already forces the path's per-level selector bits to equal the
    // FS-derived query-index bits; the (a-root) constraint adds "reconstructed root == inner
    // main-trace commitment (a bound column input)", so the composite reads: the opened
    // main-trace leaf at the FS-derived index authenticates to the inner main-trace root.
    // ---------------------------------------------------------------------------------

    /// Merkle config with the (a-root) root↔inner-main-commit binding enabled.
    fn cfg_merkle_root_bound(levels: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            bind_root_to_main_commit: true,
            ..RecursiveConfig::single_window(levels)
        }
    }

    /// Build a (a-root) window: open `leaf` at `index` in `tree`, with the inner main-trace
    /// commitment column set to `main_commit`. The FS index is `draw`'s low `levels` bits.
    fn merkle_window_root_bound(
        cfg: &RecursiveConfig,
        draw: Val,
        tree: &RefTree,
        leaf: Digest,
        index: usize,
        main_commit: Digest,
    ) -> Vec<Val> {
        let sibs = tree.auth_path(index);
        let inp = WindowInputs {
            draw_out: draw,
            e0: ext_multi(1),
            e1: ext_multi(2),
            beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
            deep_alpha: Challenge::ZERO,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[],
            fs_final_blocks: &[],
            inner_main_commit: main_commit,
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN (a-root): an honest main-trace opening at the FS-forced index, whose reconstructed
    /// root is bound to the inner main-trace commitment (= the tree root), ACCEPTS. The (a-root)
    /// equality holds because the path folds to exactly the committed inner root. (We pass the
    /// matching public root too, so the prior PV_ROOT boundary is satisfied simultaneously.)
    #[test]
    fn air_accepts_main_trace_opening_root_bound_to_inner_commit() {
        let levels = 4usize;
        let cfg = cfg_merkle_root_bound(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        for raw in [0u32, 1, 6, 0xB, 0xD, 0xF, 0xABCDA] {
            let draw = Val::from_u32(raw);
            let index = (draw.as_canonical_u32() as usize) & (n - 1);
            let row = merkle_window_root_bound(&cfg, draw, &tree, leaves[index], index, root);
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None);
            assert!(
                report.is_ok(),
                "honest root-bound main-trace opening rejected (raw={raw:#x}): {} failures",
                report.failures.len()
            );
        }
    }

    /// (a-root) FORGERY — AUTHENTICATES TO A DIFFERENT ROOT than the inner main-trace commitment:
    /// the path folds honestly to `tree.root()`, but the inner main-trace commitment column carries
    /// a DIFFERENT root. The (a-root) equality (reconstructed root == committed inner root) rejects.
    /// THIS is the property the task wants load-bearing: an opening that does not authenticate to the
    /// inner main-trace root is not accepted, even if it is a valid opening of SOME tree.
    #[test]
    fn air_rejects_opening_authenticating_to_different_root() {
        let levels = 4usize;
        let cfg = cfg_merkle_root_bound(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b1011);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        // The committed "inner main-trace commitment" is a DIFFERENT root than the one the path folds
        // to (e.g. some other proof's main-trace root). The opening is internally valid for `tree`.
        let mut other_commit = root;
        other_commit[0] += Val::ONE;
        let row = merkle_window_root_bound(&cfg, draw, &tree, leaves[index], index, other_commit);
        // CONTROL: with the matching commit (== tree root) the same opening ACCEPTS — so the ONLY
        // thing rejecting below is the (a-root) root↔inner-commit equality, nothing else.
        let control = merkle_window_root_bound(&cfg, draw, &tree, leaves[index], index, root);
        assert!(
            check_all_constraints(
                &air(cfg),
                &RowMajorMatrix::new(control, n_cols(&cfg)),
                &pubs(leaves[index], root),
                None
            )
            .is_ok(),
            "control (commit == tree root) should accept — test would prove nothing otherwise"
        );
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        // PV_ROOT still carries the real tree root (the path's actual root), so the PV boundary is
        // satisfied; ONLY (a-root) — reconstructed root != committed inner main-trace root — rejects.
        assert!(
            !check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None).is_ok(),
            "an opening authenticating to a DIFFERENT root than the inner main-trace commitment was accepted — (a-root) not load-bearing"
        );
    }

    /// (a-root) FORGERY — tampered inner-main-commit column: corrupting the committed inner root (so
    /// it no longer equals the honestly-reconstructed root) rejects. The binding is an equality, so
    /// breaking EITHER side breaks it (complements the different-root test above).
    #[test]
    fn air_rejects_tampered_inner_main_commit_column() {
        let levels = 4usize;
        let cfg = cfg_merkle_root_bound(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0111);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let mut row = merkle_window_root_bound(&cfg, draw, &tree, leaves[index], index, root);
        let mb = mc_base(&cfg);
        row[mb + MC_MAIN_COMMIT + 3] += Val::ONE; // corrupt one committed-root limb
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None).is_ok());
    }

    /// (a-root) FORGERY — MERKLE AT THE WRONG INDEX, with the root bound to the inner commit. Open at
    /// a DIFFERENT index than the FS-forced query, then swap in the honest bus block. The per-level
    /// index bits now disagree with the path's perm placements ⇒ the (a) index↔bus binding rejects —
    /// confirming the selector bits are NOT free: a prover cannot open a favorable leaf and still
    /// authenticate to the inner main-trace root. (Binding to free selector bits would be vacuous.)
    #[test]
    fn air_rejects_root_bound_opening_at_wrong_index() {
        let levels = 4usize;
        let cfg = cfg_merkle_root_bound(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0110);
        let honest_index = (draw.as_canonical_u32() as usize) & (n - 1);
        let forged_index = honest_index ^ 0b101;
        assert_ne!(forged_index, honest_index);
        // Build the row for the FORGED index (path + selector bits for forged_index), committed to the
        // real inner root (the forged leaf's path also folds to the same tree root, so (a-root) holds).
        let forged_draw = Val::from_u32(forged_index as u32);
        let mut row = merkle_window_root_bound(
            &cfg,
            forged_draw,
            &tree,
            leaves[forged_index],
            forged_index,
            root,
        );
        // CONTROL: with its own (forged) bus the row ACCEPTS — every path folds to the same root and
        // the (a-root) commit matches — so only the index↔bus binding can reject the swapped-bus row.
        assert!(
            check_all_constraints(
                &air(cfg),
                &RowMajorMatrix::new(row.clone(), n_cols(&cfg)),
                &pubs(leaves[forged_index], root),
                None
            )
            .is_ok(),
            "forged-index control row should accept with its own bus"
        );
        // Overwrite the bus block (draw_out/bits/index/tower/fold) with the HONEST query: now
        // bits[level] say honest_index but the Merkle perm placements are for forged_index.
        let honest = merkle_window_root_bound(
            &cfg,
            draw,
            &tree,
            leaves[honest_index],
            honest_index,
            root,
        );
        for c in C_DRAW_OUT..tower_end(levels) {
            row[c] = honest[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &pubs(leaves[forged_index], root), None).is_ok(),
            "a root-bound Merkle path opened at a DIFFERENT index than the FS query was accepted — selector bits not bound to the FS index"
        );
    }

    // ---------------------------------------------------------------------------------
    // (a-root) FIDELITY-PIN against a REAL p3 `MerkleTreeMmcs` (deployed MyHash/MyCompress node
    // layer): an honest opening from the real MMCS reproduces the real root in-AIR with ZERO
    // residual, and binds == the real commitment. This pins the in-AIR node-fold to the deployed
    // Merkle, not a hand-rolled stand-in.
    // ---------------------------------------------------------------------------------

    /// FIDELITY-PIN: commit a single-column main-trace-shaped matrix to a REAL p3 `MerkleTreeMmcs`
    /// over the DEPLOYED `MyHash`/`MyCompress` (the same Poseidon2-16 the inlined perm is pinned to),
    /// open a leaf, and drive the in-AIR (a-root) window with `(leaf_digest = MyHash(row), siblings,
    /// FS-index, root)`. The window folds to the real root with zero residual (ACCEPTS via
    /// `check_all_constraints`) AND the (a-root) binding to the real commitment holds. ALSO checks the
    /// real `Mmcs::verify_batch` accepts the same opening — anchoring the in-AIR fold to the deployed
    /// verifier's own node hashing. Tamper (corrupt the real commitment) ⇒ rejects.
    #[test]
    fn fidelity_real_mmcs_main_trace_opening_authenticates_in_air() {
        use descender_account_stark::Perm;
        use p3_commit::Mmcs;
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_matrix::Dimensions;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_symmetric::{CryptographicHasher, PaddingFreeSponge, TruncatedPermutation};

        // The DEPLOYED node layer, reconstructed from the re-exported `Perm` (the SAME
        // `default_koalabear_poseidon2_16` the inlined perm + `merkle::compressor()` are pinned to).
        // These const params (`16,8,8` / `2,8,16`) are byte-identical to `config::MyHash`/`MyCompress`
        // (cross-checked: account-stark `config.rs` declares the same aliases over the same `Perm`).
        type LeafHash = PaddingFreeSponge<Perm, 16, 8, DIGEST_ELEMS>;
        type NodeCompress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, 16>;
        let perm: Perm = default_koalabear_poseidon2_16();
        let hash = LeafHash::new(perm.clone());
        let compress = NodeCompress::new(perm.clone());
        type RealMmcs = MerkleTreeMmcs<Val, Val, LeafHash, NodeCompress, 2, DIGEST_ELEMS>;
        let mmcs: RealMmcs = MerkleTreeMmcs::new(hash.clone(), compress, /*cap_height=*/ 0);

        let levels = 4usize;
        let n = 1usize << levels; // power-of-two height ⇒ clean binary tree, one binary level/depth.
        let width = 5usize; // a small main-trace-shaped row width.
        // Deterministic single committed matrix (one PCS round = the main trace).
        let vals: Vec<Val> = (0..(n * width) as u32).map(|x| Val::from_u32(x.wrapping_mul(2_654_435_761))).collect();
        let matrix = RowMajorMatrix::new(vals, width);
        let (commit, prover_data) = mmcs.commit(vec![matrix.clone()]);
        // cap_height 0 ⇒ the cap has exactly ONE root: the inner main-trace Merkle root.
        assert_eq!(commit.num_roots(), 1, "cap_height 0 must yield a single root");
        let inner_main_commit: Digest = commit.roots()[0];

        let cfg = cfg_merkle_root_bound(levels);
        for &index in &[0usize, 1, 3, 6, 9, n - 1] {
            // Open the real MMCS at this leaf index.
            let opening = mmcs.open_batch(index, &prover_data);
            let (opened_values, opening_proof) = opening.unpack();
            assert_eq!(opened_values.len(), 1, "single committed matrix");
            assert_eq!(opening_proof.len(), levels, "binary path depth == levels");
            // FIDELITY: the leaf node entering the Merkle path is MyHash(opened_row) — exactly the
            // deployed leaf hashing. (Leaf-row hashing itself is a documented follow-on; here we pin
            // the NODE-FOLD layer the recursion mirrors.)
            let leaf_digest: Digest = hash.hash_iter_slices([opened_values[0].as_slice()]);
            let siblings: Vec<Digest> = opening_proof.clone();

            // Sanity: the real verifier accepts this opening against the real commitment.
            let dims = [Dimensions { width, height: n }];
            mmcs.verify_batch(
                &commit,
                &dims,
                index,
                p3_commit::BatchOpeningRef::new(&opened_values, &opening_proof),
            )
            .expect("real Mmcs::verify_batch must accept the honest opening");

            // Drive the in-AIR (a-root) window: the FS index is `index` (we set draw_out == index so
            // the bus low bits select exactly this leaf), the path is the real siblings, the leaf is
            // MyHash(row), the bound inner main-trace commitment is the real root.
            let draw = Val::from_u32(index as u32);
            let build = |commit: Digest| -> Vec<Val> {
                let inp = WindowInputs {
                    draw_out: draw,
                    e0: ext_multi(1),
                    e1: ext_multi(2),
                    beta: ext_multi(3),
                    descent: &[],
                    merkle_leaf: leaf_digest,
                    merkle_siblings: &siblings,
                    deep_zeta: Challenge::ZERO,
                    deep_q: Challenge::ZERO,
                    deep_alpha: Challenge::ZERO,
                    fs_blocks: &[],
                    sponge_in: [Val::ZERO; WIDTH],
                    prior_state: [Val::ZERO; WIDTH],
                    absorbed: [Val::ZERO; RATE],
                    prefix_blocks: &[],
                    public_io: &[],
                    fs_beta_blocks: &[],
                    fs_final_blocks: &[],
                    inner_main_commit: commit,
                    layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
                    leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
                };
                pack_window(&cfg, &inp)
            };
            let row = build(inner_main_commit);
            // ZERO-RESIDUAL fidelity: the in-AIR reconstructed root equals the REAL commitment.
            assert_eq!(
                window_merkle_root(&cfg, &row),
                inner_main_commit,
                "in-AIR fold root != real MMCS commitment at index {index} — node-fold fidelity broken"
            );
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &pubs(leaf_digest, inner_main_commit), None);
            assert!(
                report.is_ok(),
                "real-MMCS opening rejected in-AIR at index {index}: {} failures",
                report.failures.len()
            );
            // Tamper: a corrupted real commitment ⇒ the (a-root) binding rejects.
            let mut bad_commit = inner_main_commit;
            bad_commit[0] += Val::ONE;
            let bad_row = build(bad_commit);
            let bm = RowMajorMatrix::new(bad_row, n_cols(&cfg));
            assert!(
                !check_all_constraints(&air(cfg), &bm, &pubs(leaf_digest, bad_commit), None).is_ok(),
                "a corrupted inner main-trace commitment was accepted at index {index}"
            );
        }
    }

    // ---------------------------------------------------------------------------------
    // (b) beta / zeta ↔ challenger SampleDraw columns.
    // ---------------------------------------------------------------------------------

    fn cfg_beta_bound(h: usize) -> RecursiveConfig {
        RecursiveConfig { bind_beta_to_draws: true, ..RecursiveConfig::single_window(h) }
    }

    /// GOLDEN: an honest window with `beta` bound to its challenger-draw columns ACCEPTS.
    #[test]
    fn air_accepts_beta_bound_to_draws() {
        let h = 10;
        let cfg = cfg_beta_bound(h);
        let inp = WindowInputs::single(Val::from_u32(0xABCDE), ext_multi(11), ext_multi(12), ext_multi(13));
        let row = pack_window(&cfg, &inp);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(check_all_constraints(&air(cfg), &m, &[], None).is_ok(), "honest beta-draw-bound window rejected");
    }

    /// (b) FORGERY — a forged beta NOT equal to the challenger draw: swap the beta limbs to a
    /// different challenge (self-consistent (★) at the swapped beta), but leave the draw columns at
    /// the transcript value. The (b) equality `beta == draw` rejects — ONLY (b) catches this, so
    /// the binding is load-bearing.
    #[test]
    fn air_rejects_forged_beta() {
        let h = 10;
        let cfg = cfg_beta_bound(h);
        let draw = Val::from_u32(0xABCDE);
        let s = {
            let index = (draw.as_canonical_u32() as usize) & ((1 << h) - 1);
            fold_point(h, index)
        };
        let honest_beta = ext_multi(13);
        let forged_beta = ext_multi(99);
        assert_ne!(honest_beta, forged_beta);
        let (e0, e1) = (ext_multi(11), ext_multi(12));
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, e0, e1, honest_beta));
        let folded_forged = fold_pair(e0, e1, forged_beta, s);
        row[C_BETA..C_BETA + D].copy_from_slice(&limbs(forged_beta));
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded_forged));
        // CONTROL: the (★) fold is self-consistent at forged_beta — if the draw columns ALSO carry
        // forged_beta the row ACCEPTS, proving only the beta↔draw equality (b) rejects below.
        let mut control = row.clone();
        let dbase = draw_block_base(&cfg);
        control[dbase..dbase + D].copy_from_slice(&limbs(forged_beta));
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(control, n_cols(&cfg)), &[], None).is_ok(),
            "forged-beta control (draw == beta) should accept — test would prove nothing otherwise"
        );
        // draw columns still carry honest_beta ⇒ (b) equality fails.
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a beta != the challenger draw was accepted — (b) binding not load-bearing"
        );
    }

    /// (b) FORGERY — a tampered challenger-draw column (≠ beta) is rejected too (binding is an
    /// equality, so corrupting EITHER side breaks it).
    #[test]
    fn air_rejects_tampered_beta_draw_column() {
        let h = 10;
        let cfg = cfg_beta_bound(h);
        let inp = WindowInputs::single(Val::from_u32(0x12345), ext_multi(21), ext_multi(22), ext_multi(23));
        let mut row = pack_window(&cfg, &inp);
        let dbase = draw_block_base(&cfg);
        row[dbase + 2] += Val::ONE; // corrupt a draw limb
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (c) MULTI-ROUND DESCENT — fri_chain_air chaining threaded into the window.
    // ---------------------------------------------------------------------------------

    fn cfg_descent(h: usize, rounds: usize) -> RecursiveConfig {
        RecursiveConfig { num_rounds: rounds, ..RecursiveConfig::single_window(h) }
    }

    /// Honest descent inputs: fold a degree-`2^rounds` poly through `rounds` commit-phase folds,
    /// returning (e0_0, e1_0, beta_0, [(e1_r, beta_r); rounds-1]). Mirrors fri_chain_air's helper.
    fn honest_descent(
        coeffs: &[Challenge],
        betas: &[Challenge],
        h0: usize,
        index: usize,
    ) -> (Challenge, Challenge, Challenge, Vec<(Challenge, Challenge)>) {
        let eval = |c: &[Challenge], x: Val| -> Challenge {
            let mut acc = Challenge::ZERO;
            let mut p = Challenge::ONE;
            for &ci in c {
                acc += ci * p;
                p *= Challenge::from(x);
            }
            acc
        };
        let fold_poly = |c: &[Challenge], beta: Challenge| -> Vec<Challenge> {
            let half = c.len() / 2;
            (0..half).map(|k| c[2 * k] + beta * c[2 * k + 1]).collect()
        };
        let mut c = coeffs.to_vec();
        let s0 = fold_point(h0, index);
        let e0_0 = eval(&c, s0);
        let e1_0 = eval(&c, -s0);
        let mut descent = Vec::new();
        let mut s_r = s0;
        for r in 1..betas.len() {
            c = fold_poly(&c, betas[r - 1]);
            s_r = s_r * s_r;
            let e1_r = eval(&c, -s_r);
            descent.push((e1_r, betas[r]));
        }
        (e0_0, e1_0, betas[0], descent)
    }

    fn descent_window(cfg: &RecursiveConfig, h0: usize, index_draw: Val) -> Vec<Val> {
        let rounds = cfg.rounds();
        let coeffs: Vec<Challenge> = (0..(1u32 << rounds)).map(|k| ext_multi(k + 1)).collect();
        let betas: Vec<Challenge> = (0..rounds as u32).map(|r| ext_multi(100 + r)).collect();
        let index = (index_draw.as_canonical_u32() as usize) & ((1 << h0) - 1);
        let (e0_0, e1_0, beta0, descent) = honest_descent(&coeffs, &betas, h0, index);
        let inp = WindowInputs {
            draw_out: index_draw,
            e0: e0_0, e1: e1_0, beta: beta0,
            descent: &descent,
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest multi-round descent threaded in ONE window — s_0 tower-bound,
    /// s_{r+1}==s_r², e0_{r+1}==folded_r, per-round (★) — ACCEPTS.
    #[test]
    fn air_accepts_multiround_descent() {
        let h0 = 12usize;
        for rounds in [2usize, 3, 4] {
            let cfg = cfg_descent(h0, rounds);
            let row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &[], None);
            assert!(report.is_ok(), "honest {rounds}-round descent rejected: {} failures", report.failures.len());
        }
    }

    /// (c) FORGERY — BROKEN DESCENT LINK: round r+1 opens a primary eval e0 inconsistent with
    /// round r's fold output ⇒ the (chain-link) equality rejects. THIS is binding (c): a prover
    /// cannot substitute an unrelated value between descent rounds.
    #[test]
    fn air_rejects_broken_descent() {
        let h0 = 12usize;
        let rounds = 4usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
        // break the link into round 1: corrupt round-1's e0 (no longer == round-0 folded).
        let b1 = extra_round_base(&cfg, 1);
        row[b1 + 2] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a broken descent chain-link was accepted — (c) not load-bearing"
        );
    }

    /// (c) FORGERY — CHAIN-LINK ISOLATED: substitute round 1's primary eval e0 with a DIFFERENT
    /// value AND re-fold round 1's `folded` to be (★)-consistent at that e0. Now round-1 (★) holds,
    /// the point-square holds, and ONLY the chain-link `e0_1 == folded_0` is violated ⇒ rejected.
    /// Proves the chain-link itself is load-bearing (not piggybacking on (★)).
    #[test]
    fn air_rejects_descent_chain_link_isolated() {
        let h0 = 12usize;
        let rounds = 3usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
        let b1 = extra_round_base(&cfg, 1);
        // read round-1 e1, beta, s; pick a forged e0 (≠ folded_0) and re-fold so (★) holds.
        let read = |base: usize| -> Challenge {
            Challenge::from_basis_coefficients_slice(&row[base..base + D]).unwrap()
        };
        let e1_1 = read(b1 + D);
        let beta_1 = read(b1 + 2 * D);
        let s_1 = row[b1 + 3 * D];
        let forged_e0 = read(b1) + Challenge::ONE; // a different primary eval
        let refolded = fold_pair(forged_e0, e1_1, beta_1, s_1);
        row[b1..b1 + D].copy_from_slice(&limbs(forged_e0));
        row[b1 + 3 * D + 1..b1 + 4 * D + 1].copy_from_slice(&limbs(refolded));
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "an (★)-consistent round whose e0 != previous folded was accepted — chain-link not load-bearing"
        );
    }

    /// (c) FORGERY — POINT-SQUARE violated: a next-round point that is NOT the square of the
    /// previous round's point ⇒ rejected by the (point-square) constraint (and (★) at that round).
    #[test]
    fn air_rejects_descent_wrong_next_point() {
        let h0 = 12usize;
        let rounds = 3usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0x9999));
        let b2 = extra_round_base(&cfg, 2);
        row[b2 + 3 * D] += Val::ONE; // round-2 point no longer == round-1 point squared
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (c) — the descent's round-0 point is STILL the bus tower point: corrupt s_0 and the (2)
    /// tower binding + descent round-0 reads of s_0 reject. Ties the descent's leaf layer to the bus.
    #[test]
    fn air_descent_round0_point_is_bus_bound() {
        let h0 = 12usize;
        let rounds = 3usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
        row[C_S] += Val::ONE; // s_0 != tower product ⇒ (2) rejects, and descent uses s_0
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (b′) DEEP OOD block, with zeta bound to draws.
    // ---------------------------------------------------------------------------------

    fn cfg_deep(h: usize, k: usize, bind: bool) -> RecursiveConfig {
        RecursiveConfig {
            deep_log_degree: Some(k),
            bind_beta_to_draws: bind,
            ..RecursiveConfig::single_window(h)
        }
    }

    fn deep_window(cfg: &RecursiveConfig, draw: Val, zeta: Challenge, q: Challenge) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: draw,
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: zeta, deep_q: q, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest DEEP OOD block (C == Z_H(zeta)·Q) with zeta bound to draws ACCEPTS.
    #[test]
    fn air_accepts_deep_ood_with_zeta_bound() {
        let h = 10;
        for k in [4usize, 8, 12] {
            let cfg = cfg_deep(h, k, true);
            let row = deep_window(&cfg, Val::from_u32(0xABCDE), ext_multi(55), ext_multi(66));
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            assert!(check_all_constraints(&air(cfg), &m, &[], None).is_ok(), "honest DEEP OOD (k={k}) rejected");
        }
    }

    /// (b′) FORGERY — wrong C(zeta): a constraint eval ≠ Z_H(zeta)·Q is rejected.
    #[test]
    fn air_rejects_deep_wrong_c() {
        let h = 10;
        let k = 8;
        let cfg = cfg_deep(h, k, true);
        let mut row = deep_window(&cfg, Val::from_u32(0x1234), ext_multi(7), ext_multi(9));
        let db = deep_base(&cfg);
        row[db + DEEP_C + 1] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (b′)+(b) FORGERY — forged zeta: swap zeta to a different OOD point (self-consistent DEEP
    /// block) but keep the draw columns at the transcript value. The zeta↔draw equality rejects (a
    /// prover cannot open at a favorable OOD point).
    #[test]
    fn air_rejects_forged_zeta() {
        let h = 10;
        let k = 8;
        let cfg = cfg_deep(h, k, true);
        let draw = Val::from_u32(0x4242);
        let honest_zeta = ext_multi(55);
        let forged_zeta = ext_multi(77);
        assert_ne!(honest_zeta, forged_zeta);
        let q = ext_multi(66);
        let mut row = deep_window(&cfg, draw, honest_zeta, q);
        let forged = deep_window(&cfg, draw, forged_zeta, q);
        let db = deep_base(&cfg);
        for c in db + DEEP_ZETA..db + deep_size(k) {
            row[c] = forged[c];
        }
        // CONTROL: the DEEP block is self-consistent at forged_zeta (its C = Z_H(forged_zeta)·Q).
        // With the zeta draw column ALSO set to forged_zeta the row ACCEPTS, isolating (b) as the
        // sole rejector below. The zeta draw block is the LAST draw block (after the round betas).
        let mut control = row.clone();
        let zeta_draw_base = draw_block_base(&cfg) + cfg.rounds() * D;
        control[zeta_draw_base..zeta_draw_base + D].copy_from_slice(&limbs(forged_zeta));
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(control, n_cols(&cfg)), &[], None).is_ok(),
            "forged-zeta control (zeta draw == zeta) should accept — test would prove nothing otherwise"
        );
        // draw columns still carry honest_zeta ⇒ (b) zeta==draw fails.
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a zeta != the challenger draw was accepted — (b) zeta binding not load-bearing"
        );
    }

    /// (b′) FORGERY — a tampered intermediate DEEP tower power ⇒ the squaring chain breaks ⇒
    /// rejected (the Z_H tower is fully constrained, not just its endpoints).
    #[test]
    fn air_rejects_deep_tampered_tower() {
        let h = 10;
        let k = 12;
        let cfg = cfg_deep(h, k, false);
        let mut row = deep_window(&cfg, Val::from_u32(0x55), ext_multi(31), ext_multi(32));
        let db = deep_base(&cfg);
        row[db + DEEP_POW + 6 * D] += Val::ONE; // corrupt a middle tower power
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // FULL MONOLITH: all blocks together in one window.
    // ---------------------------------------------------------------------------------

    /// GOLDEN: a window with ALL blocks on — Merkle path (a) + beta/zeta draws (b) + multi-round
    /// descent (c) + DEEP OOD (b′) — assembled in ONE window, ACCEPTS. The bus index drives the
    /// fold point, the Merkle index, and the descent's s_0; the betas+zeta are bound to draws.
    #[test]
    fn air_accepts_full_monolith_window() {
        let levels = 4usize; // merkle levels = log_height
        let rounds = 3usize;
        let k = 8usize; // deep log_degree
        let cfg = RecursiveConfig {
            log_height: levels,
            merkle_levels: levels,
            bind_beta_to_draws: true,
            num_rounds: rounds,
            deep_log_degree: Some(k),
            ..RecursiveConfig::single_window(levels)
        };
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b1011);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);

        // descent inputs at log_height = levels.
        let coeffs: Vec<Challenge> = (0..(1u32 << rounds)).map(|c| ext_multi(c + 1)).collect();
        let betas: Vec<Challenge> = (0..rounds as u32).map(|r| ext_multi(100 + r)).collect();
        let (e0_0, e1_0, beta0, descent) = honest_descent(&coeffs, &betas, levels, index);

        let sibs = tree.auth_path(index);
        let inp = WindowInputs {
            draw_out: draw,
            e0: e0_0, e1: e1_0, beta: beta0,
            descent: &descent,
            merkle_leaf: leaves[index],
            merkle_siblings: &sibs,
            deep_zeta: ext_multi(55), deep_q: ext_multi(66), deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        let row = pack_window(&cfg, &inp);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let report = check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None);
        assert!(report.is_ok(), "full monolith window rejected: {} failures", report.failures.len());
    }

    // ---------------------------------------------------------------------------------
    // (a-FS) INLINE THE CHALLENGER SPONGE — the index draw is the constrained Poseidon2 output.
    // ---------------------------------------------------------------------------------

    fn cfg_sponge(h: usize) -> RecursiveConfig {
        RecursiveConfig { inline_sponge_draw: true, ..RecursiveConfig::single_window(h) }
    }

    /// A representative pre-squeeze sponge state (16 lanes) — stands in for the absorbed-transcript
    /// boundary (the absorb chain producing it is the DEFERRED full-transcript piece).
    fn sponge_state(seed: u32) -> [Val; WIDTH] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 1009 + 1))
        })
    }

    fn sponge_window(cfg: &RecursiveConfig, sponge_in: [Val; WIDTH]) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: Val::ZERO, // ignored — derived from sponge_in
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in,
            prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest sponge block — perm.inputs == sponge_in, the full Poseidon2 permutation
    /// constrained, and `draw_out == permute(sponge_in)[RATE-1]` — ACCEPTS, and that derived
    /// draw_out is what the index decomposition (1) + fold point (2) read. The index challenge is
    /// genuinely FS-derived from a real Poseidon2 squeeze, not a free committed column.
    #[test]
    fn air_accepts_inlined_sponge_draw() {
        for h in [5usize, 8, 12] {
            let cfg = cfg_sponge(h);
            for seed in [0u32, 1, 7, 0xABCD, 0x9999, 1_000_003] {
                let sin = sponge_state(seed);
                let row = sponge_window(&cfg, sin);
                // the draw_out column IS the squeeze pop.
                assert_eq!(row[C_DRAW_OUT], squeeze_draw(sin), "draw_out != squeeze pop (seed {seed:#x})");
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(report.is_ok(), "honest inlined-sponge window rejected (h={h}, seed={seed:#x}): {} failures", report.failures.len());
            }
        }
    }

    /// (a-FS) FORGERY — FORGED DRAW: a `draw_out` (with its consistent index/bits/tower/fold) that
    /// is NOT the sponge squeeze pop. We keep the sponge block honest and overwrite the whole bus
    /// for a DIFFERENT (favorable) draw value; the (a-FS) equality `draw_out == squeeze[RATE-1]`
    /// then rejects. CONTROL: the same bus, packed for THAT draw with the sponge OFF, accepts —
    /// isolating the sponge binding as the sole rejector.
    #[test]
    fn air_rejects_forged_sponge_draw() {
        let h = 10usize;
        let cfg = cfg_sponge(h);
        let sin = sponge_state(42);
        let honest_draw = squeeze_draw(sin);
        // pick a forged draw value != the squeeze pop.
        let forged_draw = honest_draw + Val::ONE;
        assert_ne!(forged_draw, honest_draw);

        // CONTROL: with the sponge OFF, a window built for forged_draw is internally consistent
        // (bits/index/tower/fold all derived from forged_draw) ⇒ accepts. So nothing in the bus
        // machinery rejects forged_draw on its own.
        let cfg_off = RecursiveConfig::single_window(h);
        let control = pack_window(&cfg_off, &WindowInputs::single(forged_draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        assert!(
            check_all_constraints(&air(cfg_off), &RowMajorMatrix::new(control, n_cols(&cfg_off)), &[], None).is_ok(),
            "forged-draw control (sponge off) should accept — test would prove nothing otherwise"
        );

        // Now: honest sponge block, but overwrite the bus (draw_out/bits/index/tower/fold) with the
        // forged-draw bus. The sponge still squeezes honest_draw ⇒ (a-FS) draw==squeeze rejects.
        let mut row = sponge_window(&cfg, sin);
        let forged_bus = pack_window(&cfg_off, &WindowInputs::single(forged_draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        for c in C_DRAW_OUT..tower_end(h) {
            row[c] = forged_bus[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a draw_out != the sponge squeeze pop was accepted — (a-FS) sponge binding not load-bearing"
        );
    }

    /// (a-FS) FORGERY — TAMPERED SPONGE STATE: corrupt one input lane of the sponge state but leave
    /// the (honest, for the old state) perm cells. The inlined Poseidon2 round constraints then
    /// reject (the permutation no longer maps the tampered input to the committed posts). Proves the
    /// permutation itself is fully constrained, not just the input==sponge_in copy.
    #[test]
    fn air_rejects_inlined_sponge_tampered_state() {
        let h = 8usize;
        let cfg = cfg_sponge(h);
        let mut row = sponge_window(&cfg, sponge_state(13));
        let sb = sponge_base(&cfg);
        // corrupt sponge_in lane 3 AND its copy into perm.inputs (so the input==sponge_in copy still
        // holds); the permutation constraint vs the unchanged posts now fails.
        row[sb + SP_SPONGE_IN + 3] += Val::ONE;
        row[sb + SP_PERM + inlined_perm::C_INPUTS + 3] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a tampered sponge state with stale perm cells was accepted — perm not constrained"
        );
    }

    /// (a-FS) FORGERY — TAMPERED INTERIOR PERM CELL: corrupt a committed begin-post cell of the
    /// sponge perm ⇒ the inlined Poseidon2 round constraints reject.
    #[test]
    fn air_rejects_inlined_sponge_tampered_perm() {
        let h = 8usize;
        let cfg = cfg_sponge(h);
        let mut row = sponge_window(&cfg, sponge_state(21));
        let sb = sponge_base(&cfg);
        row[sb + SP_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (a-FS) FORGERY — INPUT-COPY BROKEN: the perm.inputs differ from sponge_in (so a prover could
    /// try squeezing a state unrelated to the committed sponge) ⇒ rejected by the input==sponge_in
    /// equality.
    #[test]
    fn air_rejects_inlined_sponge_input_mismatch() {
        let h = 8usize;
        let cfg = cfg_sponge(h);
        let mut row = sponge_window(&cfg, sponge_state(33));
        let sb = sponge_base(&cfg);
        row[sb + SP_SPONGE_IN + 0] += Val::ONE; // sponge_in lane differs from perm.input lane
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (a-FS+) MULTI-DRAW OFF ONE PERMUTE — successive LIFO pops of the SAME refilled output buffer.
    // ---------------------------------------------------------------------------------

    fn cfg_sponge_draws(h: usize, draws: usize) -> RecursiveConfig {
        RecursiveConfig { inline_sponge_draw: true, sponge_draws: draws, ..RecursiveConfig::single_window(h) }
    }

    /// GOLDEN: with `sponge_draws` successive draws bound to the SAME squeeze permute, the index
    /// draw is output[RATE-1] and draw d is output[RATE-1-d] (the challenger LIFO pop sequence,
    /// no re-permute while the buffer is non-empty). ACCEPTS, and the extra-draw columns equal the
    /// reference `squeeze_draws`.
    #[test]
    fn air_accepts_multi_draw_off_one_permute() {
        for draws in [2usize, 4, 8] {
            let h = 8usize;
            let cfg = cfg_sponge_draws(h, draws);
            for seed in [0u32, 7, 0xABCD, 0x9999] {
                let sin = sponge_state(seed);
                let row = sponge_window(&cfg, sin);
                let want = squeeze_draws(sin, draws);
                assert_eq!(row[C_DRAW_OUT], want[0], "index draw != squeeze draw 0");
                let sb = sponge_base(&cfg);
                for d in 1..draws {
                    assert_eq!(row[sb + SP_EXTRA_DRAWS + (d - 1)], want[d], "extra draw {d} mismatch");
                }
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(report.is_ok(), "honest multi-draw ({draws}, seed {seed:#x}) rejected: {} failures", report.failures.len());
            }
        }
    }

    /// (a-FS+) FORGERY — FORGED SUCCESSIVE DRAW: a later draw value that is NOT the corresponding
    /// LIFO output lane. The sponge perm + index draw stay honest; only the forged extra-draw lane
    /// disagrees with output[RATE-1-d] ⇒ rejected. CONTROL: the honest row accepts, so the multi-
    /// draw binding is the sole rejector of the tamper below.
    #[test]
    fn air_rejects_forged_successive_draw() {
        let h = 8usize;
        let draws = 4usize;
        let cfg = cfg_sponge_draws(h, draws);
        let sin = sponge_state(101);
        let honest = sponge_window(&cfg, sin);
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(honest.clone(), n_cols(&cfg)), &[], None).is_ok(),
            "honest multi-draw control should accept — test would prove nothing otherwise"
        );
        let sb = sponge_base(&cfg);
        // forge draw 2 (output[RATE-3]) to a wrong value; the index draw (draw 0) is untouched, so
        // bits/index/tower/fold are unaffected — ONLY the (a-FS+) draw-2 binding can reject.
        let mut row = honest;
        row[sb + SP_EXTRA_DRAWS + (2 - 1)] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a forged successive draw (!= output[RATE-1-d]) was accepted — (a-FS+) not load-bearing"
        );
    }

    /// (a-FS+) the extra draws are genuinely DISTINCT lanes (a forgery that just copies the index
    /// draw into every extra lane must be rejected) — guards against an under-distinguishing
    /// binding that would let a prover reuse one draw for all challenges.
    #[test]
    fn air_rejects_extra_draws_collapsed_to_index_draw() {
        let h = 8usize;
        let draws = 4usize;
        let cfg = cfg_sponge_draws(h, draws);
        let sin = sponge_state(202);
        let mut row = sponge_window(&cfg, sin);
        let sb = sponge_base(&cfg);
        let idx_draw = row[C_DRAW_OUT];
        // the honest output lanes are distinct from output[RATE-1] (overwhelmingly), so copying the
        // index draw into the extra lanes breaks the per-lane binding.
        for d in 1..draws {
            row[sb + SP_EXTRA_DRAWS + (d - 1)] = idx_draw;
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "extra draws collapsed to the index draw were accepted — draws are not lane-distinct"
        );
    }

    // ---------------------------------------------------------------------------------
    // (a-FS++) ABSORB-PERMUTE FEEDING THE SQUEEZE — sponge_in derived from a constrained absorb.
    // ---------------------------------------------------------------------------------

    fn cfg_sponge_absorb(h: usize, n: usize) -> RecursiveConfig {
        RecursiveConfig {
            inline_sponge_draw: true,
            sponge_absorb: true,
            sponge_absorb_n: n,
            ..RecursiveConfig::single_window(h)
        }
    }

    /// Build an absorb-fed sponge window: sponge_in is DERIVED from (prior_state, absorbed, n) by
    /// the honest builder; the index draw is the squeeze of that derived state.
    fn absorb_window(cfg: &RecursiveConfig, prior_state: [Val; WIDTH], absorbed: [Val; RATE]) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: Val::ZERO, // derived
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], // ignored — derived from absorb
            prior_state,
            absorbed,
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    fn absorbed_vec(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1)))
    }

    /// GOLDEN: an absorb-fed window — sponge_in == absorb(prior_state, absorbed, n) (rate overwrite,
    /// rest zeroed, capacity += n length tag, capacity carried) and the index draw is the squeeze of
    /// that DERIVED state — ACCEPTS. sponge_in is no longer a free column: it is pinned to the
    /// committed prior_state + absorbed values + the length tag, exactly per challenger::duplexing.
    #[test]
    fn air_accepts_absorb_fed_sponge() {
        for n in [0usize, 1, 3, 8] {
            for h in [5usize, 8] {
                let cfg = cfg_sponge_absorb(h, n);
                // prior_state: a representative pre-absorb sponge (e.g. mid-transcript); for n>0 the
                // capacity tag accumulates onto prior_state[RATE].
                let prior = sponge_state(0xC0FFEE ^ (n as u32));
                let absorbed = absorbed_vec(0xABBA + n as u32);
                let row = absorb_window(&cfg, prior, absorbed);
                // the derived sponge_in matches the reference absorb.
                let sb = sponge_base(&cfg);
                let want_sin = absorb_state(prior, absorbed, n);
                for i in 0..WIDTH {
                    assert_eq!(row[sb + SP_SPONGE_IN + i], want_sin[i], "sponge_in lane {i} != absorb (n={n})");
                }
                assert_eq!(row[C_DRAW_OUT], squeeze_draw(want_sin), "index draw != squeeze of absorbed state");
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(report.is_ok(), "honest absorb-fed sponge (n={n}, h={h}) rejected: {} failures", report.failures.len());
            }
        }
    }

    /// (a-FS++) FORGERY — FORGED ABSORBED VALUE: change one absorbed rate element WITHOUT updating
    /// the derived sponge_in/perm/draw. The absorb-derivation constraint (rate lane == absorbed)
    /// then rejects: a prover cannot claim a sponge state inconsistent with the values it absorbed.
    #[test]
    fn air_rejects_forged_absorbed_value() {
        let h = 8usize;
        let n = 5usize;
        let cfg = cfg_sponge_absorb(h, n);
        let prior = sponge_state(0xDEAD);
        let absorbed = absorbed_vec(0xBEEF);
        let mut row = absorb_window(&cfg, prior, absorbed);
        let sb = sponge_base(&cfg);
        // corrupt absorbed[2] alone; sponge_in/perm/draw stay the honest absorb of the OLD absorbed.
        let ab = sb + sp_absorb(&cfg);
        row[ab + SPA_ABSORBED + 2] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a sponge_in inconsistent with the absorbed values was accepted — (a-FS++) absorb not load-bearing"
        );
    }

    /// (a-FS++) FORGERY — FORGED SPONGE_IN (broken length tag): tamper the capacity-slot length tag
    /// of sponge_in (and propagate it into perm.inputs + re-derive the perm/draw so the squeeze and
    /// input-copy stay self-consistent). The absorb-derivation `sponge_in[RATE] == prior[RATE] + n`
    /// then rejects — ONLY the absorb binding catches the dropped length tag (the prefix-freeness
    /// guarantee). CONTROL: with sponge_absorb OFF (free sponge_in) the SAME forged state accepts.
    #[test]
    fn air_rejects_forged_length_tag() {
        let h = 8usize;
        let n = 4usize;
        let prior = sponge_state(0x1234);
        let absorbed = absorbed_vec(0x5678);
        let honest_sin = absorb_state(prior, absorbed, n);
        // forged state: same as honest absorb but the length tag dropped (slot RATE -= n) — i.e. the
        // attacker absorbs the same values but omits the prefix-free tag, colliding distinct lengths.
        let mut forged_sin = honest_sin;
        forged_sin[RATE] -= Val::from_u8(n as u8);
        assert_ne!(forged_sin[RATE], honest_sin[RATE]);

        // CONTROL: sponge_absorb OFF — forged_sin is a perfectly valid free committed state, its
        // squeeze is a valid index draw ⇒ accepts. So nothing but the absorb binding objects.
        let cfg_off = cfg_sponge(h);
        let control = sponge_window(&cfg_off, forged_sin);
        assert!(
            check_all_constraints(&air(cfg_off), &RowMajorMatrix::new(control, n_cols(&cfg_off)), &[], None).is_ok(),
            "forged-length-tag control (absorb off) should accept — test would prove nothing otherwise"
        );

        // sponge_absorb ON: pack the perm/draw for forged_sin but carry the HONEST (prior, absorbed)
        // in the absorb sub-block. The derivation requires sponge_in[RATE] == prior[RATE] + n, which
        // forged_sin violates ⇒ rejected.
        let cfg_on = cfg_sponge_absorb(h, n);
        // Build an honest absorb window, then overwrite sponge_in + perm + draw block with the
        // squeeze of forged_sin (self-consistent perm), leaving the (prior, absorbed) sub-block honest.
        let mut row = absorb_window(&cfg_on, prior, absorbed);
        let sb = sponge_base(&cfg_on);
        let consts = inlined_perm::Constants::pinned();
        let forged_perm = inlined_perm::generate_perm_cols(forged_sin, &consts);
        row[sb + SP_SPONGE_IN..sb + SP_SPONGE_IN + WIDTH].copy_from_slice(&forged_sin);
        row[sb + SP_PERM..sb + SP_PERM + PERM_COLS].copy_from_slice(&forged_perm);
        // re-derive the whole bus (draw_out/bits/index/tower/fold) for the forged squeeze draw, so
        // the only constraint that can object is the absorb-derivation (length tag), not the bus.
        let forged_draw = forged_perm[C_OUTPUT + (RATE - 1)];
        let forged_bus = pack_window(&cfg_off, &WindowInputs::single(forged_draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        for c in C_DRAW_OUT..tower_end(h) {
            row[c] = forged_bus[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg_on));
        assert!(
            !check_all_constraints(&air(cfg_on), &m, &[], None).is_ok(),
            "a sponge_in with a dropped length tag was accepted — (a-FS++) length-tag binding not load-bearing"
        );
    }

    /// (a-FS++) the absorb derivation zeroes the UNFILLED rate lanes [n..RATE]: a nonzero stale
    /// value in an unfilled rate lane of sponge_in is rejected (challenger::duplexing fills them
    /// with ZERO before permuting). Tamper sponge_in lane n (and its perm.input copy) ⇒ the
    /// derivation's `sponge_in[n..RATE] == 0` rejects.
    #[test]
    fn air_rejects_nonzero_unfilled_rate_lane() {
        let h = 8usize;
        let n = 3usize;
        let cfg = cfg_sponge_absorb(h, n);
        let prior = sponge_state(0x99);
        let absorbed = absorbed_vec(0x11);
        let mut row = absorb_window(&cfg, prior, absorbed);
        let sb = sponge_base(&cfg);
        // set unfilled rate lane n to nonzero in sponge_in AND its perm.input copy (so input==sponge_in
        // still holds and only the derivation's zero-fill is violated). Re-derive perm/draw to keep
        // the permutation self-consistent, leaving the absorb derivation as the sole objector.
        let mut tampered_sin = absorb_state(prior, absorbed, n);
        tampered_sin[n] += Val::ONE; // a stale unfilled rate lane
        let consts = inlined_perm::Constants::pinned();
        let perm = inlined_perm::generate_perm_cols(tampered_sin, &consts);
        row[sb + SP_SPONGE_IN..sb + SP_SPONGE_IN + WIDTH].copy_from_slice(&tampered_sin);
        row[sb + SP_PERM..sb + SP_PERM + PERM_COLS].copy_from_slice(&perm);
        let forged_draw = perm[C_OUTPUT + (RATE - 1)];
        let forged_bus = pack_window(&RecursiveConfig::single_window(h), &WindowInputs::single(forged_draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        for c in C_DRAW_OUT..tower_end(h) {
            row[c] = forged_bus[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a nonzero unfilled rate lane was accepted — (a-FS++) zero-fill not load-bearing"
        );
    }

    /// GOLDEN: absorb-fed sponge + multi-draw together — sponge_in derived from the absorb, and
    /// `sponge_draws` successive challenges popped off the one squeeze. ACCEPTS. This is one full
    /// constrained absorb→permute→multi-draw duplex step in-window.
    #[test]
    fn air_accepts_absorb_fed_multi_draw() {
        let h = 8usize;
        let n = 6usize;
        let draws = 4usize;
        let cfg = RecursiveConfig {
            inline_sponge_draw: true,
            sponge_absorb: true,
            sponge_absorb_n: n,
            sponge_draws: draws,
            ..RecursiveConfig::single_window(h)
        };
        let prior = sponge_state(0xFACE);
        let absorbed = absorbed_vec(0xCAFE);
        let row = absorb_window(&cfg, prior, absorbed);
        let want_sin = absorb_state(prior, absorbed, n);
        let want_draws = squeeze_draws(want_sin, draws);
        assert_eq!(row[C_DRAW_OUT], want_draws[0]);
        let sb = sponge_base(&cfg);
        for d in 1..draws {
            assert_eq!(row[sb + SP_EXTRA_DRAWS + (d - 1)], want_draws[d], "draw {d} != squeeze draw");
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let report = check_all_constraints(&air(cfg), &m, &[], None);
        assert!(report.is_ok(), "honest absorb-fed multi-draw window rejected: {} failures", report.failures.len());
    }

    // ---------------------------------------------------------------------------------
    // (a-FS+++) TRANSCRIPT-PREFIX ABSORB CHAIN — prior_state is the constrained output of a
    // multi-round absorb-permute chain from the new() zero boundary, not a free column.
    // ---------------------------------------------------------------------------------

    fn cfg_prefix(h: usize, n: usize, rounds: usize) -> RecursiveConfig {
        RecursiveConfig {
            inline_sponge_draw: true,
            sponge_absorb: true,
            sponge_absorb_n: n,
            sponge_prefix_rounds: rounds,
            ..RecursiveConfig::single_window(h)
        }
    }

    /// A representative RATE-element transcript block (stands in for an observed commitment cap /
    /// public-IO chunk).
    fn prefix_block(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 31 + 1)))
    }

    /// Build a prefix-chained absorb window: prior_state is DERIVED by the honest builder from the
    /// prefix blocks (zero-boundary chain), then the final duplex step absorbs `absorbed` (n lanes)
    /// and squeezes the index draw.
    fn prefix_window(
        cfg: &RecursiveConfig,
        blocks: &[[Val; RATE]],
        absorbed: [Val; RATE],
    ) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: Val::ZERO,
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH], // ignored — derived from the prefix chain
            absorbed,
            prefix_blocks: blocks,
            public_io: &[],
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: a 2- and 3-round transcript-prefix absorb chain feeding the duplex step. state_0 is
    /// the new() zero boundary; each state_k == permute(absorb(state_{k-1}, block_k, RATE)); state_K
    /// == prior_state, which the final duplex step absorbs `absorbed` (n) into and squeezes. ACCEPTS,
    /// and the derived prior_state matches the reference prefix_chain_state. prior_state is no longer
    /// a free column — it is the constrained output of the absorb chain.
    #[test]
    fn air_accepts_prefix_absorb_chain() {
        for rounds in [1usize, 2, 3] {
            for n in [0usize, 3, 8] {
                let h = 8usize;
                let cfg = cfg_prefix(h, n, rounds);
                let blocks: Vec<[Val; RATE]> =
                    (0..rounds as u32).map(|k| prefix_block(0xA11CE + k)).collect();
                let absorbed = absorbed_vec(0xF00D + rounds as u32);
                let row = prefix_window(&cfg, &blocks, absorbed);
                // the derived prior_state matches the reference chain from the zero boundary.
                let want_prior = prefix_chain_state(&blocks);
                let sb = sponge_base(&cfg);
                let ab = sb + sp_absorb(&cfg);
                for i in 0..WIDTH {
                    assert_eq!(row[ab + SPA_PRIOR_STATE + i], want_prior[i], "prior_state lane {i} != chain (rounds={rounds},n={n})");
                }
                // and the index draw is the squeeze of absorb(prior_state, absorbed, n).
                let want_sin = absorb_state(want_prior, absorbed, n);
                assert_eq!(row[C_DRAW_OUT], squeeze_draw(want_sin), "index draw != squeeze of chained state");
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(report.is_ok(), "honest prefix chain (rounds={rounds}, n={n}) rejected: {} failures", report.failures.len());
            }
        }
    }

    /// (a-FS+++) FORGERY — NON-ZERO ROUND-0 STATE: the chain MUST start from the new() all-zero
    /// boundary. Build an honest 2-round chain, then forge round 1's perm input to start from a
    /// non-zero state_0 (and re-derive round-1's perm so the permutation stays self-consistent). The
    /// boundary constraint (round-1 absorb input == absorb(ZERO, block_1, RATE)) then rejects: a
    /// prover cannot seed the FS transcript from a chosen non-zero state. CONTROL: a chain built
    /// honestly from that SAME non-zero state under a fresh (non-boundary) reference would accept —
    /// here we isolate the zero-boundary by showing only it objects to the forged seed.
    #[test]
    fn air_rejects_prefix_nonzero_round0_state() {
        let h = 8usize;
        let n = 4usize;
        let rounds = 2usize;
        let cfg = cfg_prefix(h, n, rounds);
        let blocks: Vec<[Val; RATE]> = (0..rounds as u32).map(|k| prefix_block(0xBEE5 + k)).collect();
        let absorbed = absorbed_vec(0x1357);
        let mut row = prefix_window(&cfg, &blocks, absorbed);
        let sb = sponge_base(&cfg);
        // CONTROL: the honest row (chain seeded from the zero boundary) accepts.
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(row.clone(), n_cols(&cfg)), &[], None).is_ok(),
            "honest prefix-chain control should accept — test would prove nothing otherwise"
        );

        // Forge a non-zero state_0: re-derive round-1's perm input as absorb(seed0, block_1, RATE)
        // for a NON-ZERO seed0, write the matching perm cells (so eval_perm passes), but DO NOT
        // propagate downstream (round 2 / prior_state / draw stay the honest zero-boundary chain).
        // The ONLY violated constraint is round-1's capacity length tag: honest expects
        // input[RATE] == 0 + RATE; the forged input carries seed0[RATE] + RATE != RATE.
        let mut seed0 = [Val::ZERO; WIDTH];
        seed0[RATE] += Val::ONE; // a non-zero capacity slot in the (illegal) starting state
        let forged_pre = absorb_state(seed0, blocks[0], RATE);
        let consts = inlined_perm::Constants::pinned();
        let forged_perm = inlined_perm::generate_perm_cols(forged_pre, &consts);
        let p1 = sb + prefix_round_base(&cfg, 1);
        row[p1 + SPP_PERM..p1 + SPP_PERM + PERM_COLS].copy_from_slice(&forged_perm);
        // round-1 block stays blocks[0]; eval_perm now passes on the forged perm, the FULL-rate
        // absorb rate lanes == block_1 still hold, but capacity-slot tag (== 0 + RATE) is violated.
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a prefix chain seeded from a non-zero round-0 state was accepted — zero boundary not load-bearing"
        );
    }

    /// (a-FS+++) FORGERY — FORGED INTERMEDIATE state_k: corrupt round-1's perm output (state_1) so it
    /// is NOT permute(absorb(state_0, block_1, RATE)) — i.e. an intermediate chain state inconsistent
    /// with its absorb-permute. The inlined Poseidon2 round constraints on round 1 reject (the
    /// committed output no longer matches the recomputed permutation). Proves the chain link
    /// state_k == permute(...) is constrained, not just carried.
    #[test]
    fn air_rejects_prefix_forged_intermediate_state() {
        let h = 8usize;
        let n = 5usize;
        let rounds = 3usize;
        let cfg = cfg_prefix(h, n, rounds);
        let blocks: Vec<[Val; RATE]> = (0..rounds as u32).map(|k| prefix_block(0xCAFE + k)).collect();
        let absorbed = absorbed_vec(0x2468);
        let mut row = prefix_window(&cfg, &blocks, absorbed);
        let sb = sponge_base(&cfg);
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(row.clone(), n_cols(&cfg)), &[], None).is_ok(),
            "honest 3-round prefix-chain control should accept — test would prove nothing otherwise"
        );
        // corrupt one OUTPUT lane of round-1's perm (the committed state_1) — leaving its inputs and
        // interior posts intact ⇒ eval_perm's last-round post equality on that lane breaks.
        let p1 = sb + prefix_round_base(&cfg, 1);
        row[p1 + SPP_PERM + C_OUTPUT + 4] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a forged intermediate chain state (state_1 != permute(absorb(...))) was accepted — chain link not constrained"
        );
    }

    /// (a-FS+++) FORGERY — CHAIN-LINK BROKEN BETWEEN ROUNDS: substitute round-2's absorb input so it
    /// no longer carries round-1's output as state_1 (re-derive round-2's perm to stay
    /// self-consistent). Round 2's absorb derivation requires input == absorb(state_1, block_2, RATE)
    /// where state_1 == round-1 output; feeding a DIFFERENT prior breaks the carry. Only the
    /// inter-round carry (capacity slots / tag) catches this — the per-round perm stays valid.
    #[test]
    fn air_rejects_prefix_broken_interround_carry() {
        let h = 8usize;
        let n = 2usize;
        let rounds = 3usize;
        let cfg = cfg_prefix(h, n, rounds);
        let blocks: Vec<[Val; RATE]> = (0..rounds as u32).map(|k| prefix_block(0xD00D + k)).collect();
        let absorbed = absorbed_vec(0x9753);
        let mut row = prefix_window(&cfg, &blocks, absorbed);
        let sb = sponge_base(&cfg);
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(row.clone(), n_cols(&cfg)), &[], None).is_ok(),
            "honest 3-round prefix-chain control should accept — test would prove nothing otherwise"
        );
        // Re-derive round-2's perm from a FORGED prior state (a chosen state, not round-1's output),
        // keeping round-2's block. The capacity carry input[RATE] == state_1[RATE] + RATE and the
        // [RATE+1..WIDTH] carry from state_1 are now violated (state_1 != forged prior).
        let consts = inlined_perm::Constants::pinned();
        let mut forged_prior = prefix_round_state([Val::ZERO; WIDTH], blocks[0]); // honest state_1...
        forged_prior[WIDTH - 1] += Val::ONE; // ...with one capacity lane perturbed (!= true state_1)
        let forged_pre = absorb_state(forged_prior, blocks[1], RATE);
        let forged_perm = inlined_perm::generate_perm_cols(forged_pre, &consts);
        let p2 = sb + prefix_round_base(&cfg, 2);
        row[p2 + SPP_PERM..p2 + SPP_PERM + PERM_COLS].copy_from_slice(&forged_perm);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a broken inter-round carry (round-2 prior != round-1 output) was accepted — carry not load-bearing"
        );
    }

    /// (a-FS+++) FORGERY — FORGED ABSORBED TRANSCRIPT BLOCK: change one element of round-1's absorbed
    /// block WITHOUT updating the derived perm/state. The round-1 FULL-rate absorb derivation (rate
    /// lane == block) then rejects: a prover cannot claim a chain state inconsistent with the
    /// transcript values it absorbed. This is the absorb-soundness of the prefix.
    #[test]
    fn air_rejects_prefix_forged_absorbed_block() {
        let h = 8usize;
        let n = 6usize;
        let rounds = 2usize;
        let cfg = cfg_prefix(h, n, rounds);
        let blocks: Vec<[Val; RATE]> = (0..rounds as u32).map(|k| prefix_block(0xE0E0 + k)).collect();
        let absorbed = absorbed_vec(0x1122);
        let mut row = prefix_window(&cfg, &blocks, absorbed);
        let sb = sponge_base(&cfg);
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(row.clone(), n_cols(&cfg)), &[], None).is_ok(),
            "honest prefix-chain control should accept — test would prove nothing otherwise"
        );
        // corrupt block_1[3] alone; the perm/state stay the honest absorb of the OLD block.
        let p1 = sb + prefix_round_base(&cfg, 1);
        row[p1 + SPP_BLOCK + 3] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a chain state inconsistent with its absorbed block was accepted — absorb not load-bearing"
        );
    }

    /// (a-FS+++) FORGERY — FORGED FINAL state_K → prior_state link: corrupt the duplex step's
    /// prior_state so it is NOT the final chain output state_K. The `state_K == prior_state` binding
    /// rejects (the chain output must be exactly what the final duplex step absorbs). We also
    /// re-derive the downstream sponge_in/perm/draw for the forged prior so the duplex step itself
    /// stays self-consistent — isolating the state_K→prior_state link as the sole rejector.
    #[test]
    fn air_rejects_prefix_forged_final_link() {
        let h = 8usize;
        let n = 4usize;
        let rounds = 2usize;
        let cfg = cfg_prefix(h, n, rounds);
        let blocks: Vec<[Val; RATE]> = (0..rounds as u32).map(|k| prefix_block(0x5A5A + k)).collect();
        let absorbed = absorbed_vec(0x3344);
        let mut row = prefix_window(&cfg, &blocks, absorbed);
        let sb = sponge_base(&cfg);
        let ab = sb + sp_absorb(&cfg);

        // Forge prior_state (!= state_K) and re-derive the duplex step (sponge_in/perm/draw/bus) for
        // it, leaving the chain (and its true state_K output) honest. The final-link equality fails.
        let true_prior = prefix_chain_state(&blocks);
        let mut forged_prior = true_prior;
        forged_prior[0] += Val::ONE;
        let forged_sin = absorb_state(forged_prior, absorbed, n);
        let consts = inlined_perm::Constants::pinned();
        let forged_perm = inlined_perm::generate_perm_cols(forged_sin, &consts);
        row[ab + SPA_PRIOR_STATE..ab + SPA_PRIOR_STATE + WIDTH].copy_from_slice(&forged_prior);
        row[sb + SP_SPONGE_IN..sb + SP_SPONGE_IN + WIDTH].copy_from_slice(&forged_sin);
        row[sb + SP_PERM..sb + SP_PERM + PERM_COLS].copy_from_slice(&forged_perm);
        let forged_draw = forged_perm[C_OUTPUT + (RATE - 1)];
        let forged_bus = pack_window(&RecursiveConfig::single_window(h), &WindowInputs::single(forged_draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        for c in C_DRAW_OUT..tower_end(h) {
            row[c] = forged_bus[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a prior_state != the chain output state_K was accepted — final link not load-bearing"
        );
    }

    /// (a-FS+++) GOLDEN + BINDING: with `prefix_blocks_public`, each round's absorbed block is pinned
    /// to public-value columns (the committed transcript). An honest window with public values =
    /// flattened blocks ACCEPTS; a public value that disagrees with the absorbed block rejects.
    #[test]
    fn air_prefix_blocks_bound_to_public_transcript() {
        let h = 8usize;
        let n = 3usize;
        let rounds = 2usize;
        let cfg = RecursiveConfig { prefix_blocks_public: true, ..cfg_prefix(h, n, rounds) };
        let blocks: Vec<[Val; RATE]> = (0..rounds as u32).map(|k| prefix_block(0x7E57 + k)).collect();
        let absorbed = absorbed_vec(0x4455);
        let row = prefix_window(&cfg, &blocks, absorbed);
        // public values = the flattened transcript blocks (round 0 then round 1).
        let mut pv: Vec<Val> = Vec::with_capacity(rounds * RATE);
        for b in &blocks {
            pv.extend_from_slice(b);
        }
        let m = RowMajorMatrix::new(row.clone(), n_cols(&cfg));
        assert!(
            check_all_constraints(&air(cfg), &m, &pv, None).is_ok(),
            "honest prefix chain with public-bound transcript blocks rejected"
        );
        // FORGERY: a public transcript value that disagrees with the absorbed block ⇒ rejected.
        let mut bad_pv = pv.clone();
        bad_pv[RATE + 1] += Val::ONE; // round-2 block, element 1
        let m2 = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m2, &bad_pv, None).is_ok(),
            "a public transcript value != the absorbed block was accepted — prefix_blocks_public not load-bearing"
        );
    }

    // ---------------------------------------------------------------------------------
    // (b-leaf) Merkle-opened leaf digest ↔ FRI round-0 e0.
    // ---------------------------------------------------------------------------------

    fn cfg_leaf_e0(levels: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            bind_merkle_leaf_to_e0: true,
            ..RecursiveConfig::single_window(levels)
        }
    }

    /// Build a Merkle window whose round-0 e0 limbs equal the opened leaf digest (the honest
    /// builder for the (b-leaf) binding).
    fn leaf_e0_window(cfg: &RecursiveConfig, draw: Val, tree: &RefTree, leaf: Digest, index: usize) -> Vec<Val> {
        // e0 = the leaf digest as a Challenge (limb i = leaf[i]); DIGEST_ELEMS == D == 8.
        let e0 = Challenge::from_basis_coefficients_slice(&leaf).unwrap();
        let sibs = tree.auth_path(index);
        let inp = WindowInputs {
            draw_out: draw,
            e0, e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest window where the Merkle-opened leaf IS the FRI-folded e0 ACCEPTS.
    #[test]
    fn air_accepts_merkle_leaf_bound_to_e0() {
        let levels = 4usize;
        let cfg = cfg_leaf_e0(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        for raw in [0u32, 3, 6, 0xD, 0xABCDA] {
            let draw = Val::from_u32(raw);
            let index = (draw.as_canonical_u32() as usize) & (n - 1);
            let row = leaf_e0_window(&cfg, draw, &tree, leaves[index], index);
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None);
            assert!(report.is_ok(), "honest leaf↔e0 window rejected (raw={raw:#x}): {} failures", report.failures.len());
        }
    }

    /// (b-leaf) FORGERY — leaf ≠ folded value: fold an e0 DIFFERENT from the opened leaf digest (a
    /// self-consistent FRI fold at that e0) while the Merkle path authenticates the real leaf. The
    /// (b-leaf) equality `leaf == e0` rejects. CONTROL: the same row with the binding OFF accepts —
    /// isolating (b-leaf) as the sole rejector.
    #[test]
    fn air_rejects_merkle_leaf_e0_mismatch() {
        let levels = 4usize;
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0110);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);

        // CONTROL: binding OFF — fold an e0 unrelated to the leaf (a valid (★) fold) ⇒ accepts.
        let cfg_off = cfg_merkle(levels);
        let sibs = tree.auth_path(index);
        let forged_e0 = ext_multi(777); // not the leaf digest
        let inp = WindowInputs {
            draw_out: draw, e0: forged_e0, e1: ext_multi(2), beta: ext_multi(3),
            descent: &[], merkle_leaf: leaves[index], merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        let row = pack_window(&cfg_off, &inp);
        assert!(
            check_all_constraints(&air(cfg_off), &RowMajorMatrix::new(row.clone(), n_cols(&cfg_off)), &pubs(leaves[index], root), None).is_ok(),
            "control (leaf↔e0 binding off) should accept — test would prove nothing otherwise"
        );

        // Binding ON: same columns (leaf and e0 layouts identical between cfgs) ⇒ leaf != e0 rejects.
        let cfg_on = cfg_leaf_e0(levels);
        assert_eq!(n_cols(&cfg_on), n_cols(&cfg_off), "leaf↔e0 binding must not change layout");
        let m = RowMajorMatrix::new(row, n_cols(&cfg_on));
        assert!(
            !check_all_constraints(&air(cfg_on), &m, &pubs(leaves[index], root), None).is_ok(),
            "a folded e0 != the opened leaf was accepted — (b-leaf) not load-bearing"
        );
    }

    // ---------------------------------------------------------------------------------
    // (b-hash) leaf-row hashing: the Merkle level-0 leaf digest is MyHash(opened_row ‖ salt)
    // DERIVED in-AIR, not a supplied digest.
    // ---------------------------------------------------------------------------------

    fn cfg_leaf_hash(levels: usize, row_len: usize) -> RecursiveConfig {
        RecursiveConfig {
            merkle_levels: levels,
            leaf_row_len: row_len,
            ..RecursiveConfig::single_window(levels)
        }
    }

    fn sample_row(seed: u32, len: usize) -> Vec<Val> {
        (0..len as u32)
            .map(|i| Val::from_u32(seed.wrapping_mul(2_654_435_761).wrapping_add(i.wrapping_mul(7) + 1)))
            .collect()
    }
    fn sample_salt(seed: u32) -> [Val; SALT_ELEMS] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(40_503).wrapping_add(i as u32 + 3)))
    }

    /// PIN: the in-AIR `leaf_hash_digest` (the inlined `PaddingFreeSponge` schedule) matches a REAL
    /// `p3_symmetric::PaddingFreeSponge<Perm,16,8,8>` (the deployed `config::MyHash`) over the same
    /// salted input, ACROSS row lengths spanning the partial-block, exact-multiple, and multi-block
    /// cases. A wrong absorb schedule / wrong perm would diverge here — attempted equality, not
    /// assumption.
    #[test]
    fn leaf_hash_matches_native_padding_free_sponge() {
        use descender_account_stark::Perm;
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_symmetric::{CryptographicHasher, PaddingFreeSponge};
        type MyHash = PaddingFreeSponge<Perm, 16, 8, DIGEST_ELEMS>;
        let hash = MyHash::new(default_koalabear_poseidon2_16());
        // row lengths chosen so row+SALT_ELEMS(4) hits: < RATE (4), == RATE (4? -> 4+4=8 exact),
        // partial trailing (e.g. 5+4=9), multi-block (e.g. 20+4=24 exact, 21+4=25 partial).
        for &row_len in &[0usize, 1, 4, 5, 12, 20, 21, 60] {
            let row = sample_row(0xA1 + row_len as u32, row_len);
            let salt = sample_salt(0xB7 + row_len as u32);
            let mut input = row.clone();
            input.extend_from_slice(&salt);
            let native: [Val; DIGEST_ELEMS] = hash.hash_iter_slices([input.as_slice()]);
            let in_air = leaf_hash_digest(&input);
            assert_eq!(in_air, native, "in-AIR MyHash != native PaddingFreeSponge at row_len={row_len}");
        }
    }

    /// FIDELITY-PIN end-to-end: commit a real single-column-ish matrix to a real `MerkleTreeMmcs`
    /// over the deployed `MyHash`/`MyCompress`, open a leaf, and drive the (b-hash) window with the
    /// OPENED ROW (the `MyHash` input) — NOT a precomputed digest. The window hashes the row in-AIR,
    /// folds the real siblings, and authenticates to the real commitment. ACCEPTS with zero residual.
    /// (Salt is zero here — `MerkleTreeMmcs` is the non-hiding variant whose leaf is `MyHash(row)`;
    /// the salted hiding leaf is `MyHash(row ‖ salt)`, the `SALT_ELEMS`-extended same sponge, PINned
    /// by the native-sponge test above.)
    #[test]
    fn fidelity_leaf_row_hashed_in_air_authenticates() {
        use descender_account_stark::Perm;
        use p3_commit::Mmcs;
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

        type LeafHash = PaddingFreeSponge<Perm, 16, 8, DIGEST_ELEMS>;
        type NodeCompress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, 16>;
        let perm: Perm = default_koalabear_poseidon2_16();
        let hash = LeafHash::new(perm.clone());
        let compress = NodeCompress::new(perm.clone());
        type RealMmcs = MerkleTreeMmcs<Val, Val, LeafHash, NodeCompress, 2, DIGEST_ELEMS>;
        let mmcs: RealMmcs = MerkleTreeMmcs::new(hash.clone(), compress, 0);

        let levels = 4usize;
        let n = 1usize << levels;
        let width = 11usize; // a non-RATE-aligned main-trace row width (11 + 0 salt = 11 -> 2 perms).
        let vals: Vec<Val> =
            (0..(n * width) as u32).map(|x| Val::from_u32(x.wrapping_mul(2_654_435_761))).collect();
        let matrix = RowMajorMatrix::new(vals, width);
        let (commit, prover_data) = mmcs.commit(vec![matrix.clone()]);
        let inner_main_commit: Digest = commit.roots()[0];

        // (b-hash) cfg over a non-salted leaf: leaf_row_len = width, SALT_ELEMS still appended (0 salt).
        let cfg = RecursiveConfig {
            merkle_levels: levels,
            leaf_row_len: width,
            bind_root_to_main_commit: true,
            ..RecursiveConfig::single_window(levels)
        };
        for &index in &[0usize, 1, 3, 6, n - 1] {
            let opening = mmcs.open_batch(index, &prover_data);
            let (opened_values, opening_proof) = opening.unpack();
            let opened_row: Vec<Val> = opened_values[0].clone();
            // NOTE: salt is zero so the in-AIR sponge input is row ‖ [0,0,0,0]; this differs from the
            // real MerkleTreeMmcs leaf MyHash(row). So we PIN the digest the recursion uses (row‖0^4)
            // and verify the in-AIR fold reproduces THAT leaf to the (necessarily re-derived) root,
            // anchoring leaf-row hashing in-AIR. The hiding-MMCS row‖salt case is the native-sponge
            // PIN test; here we additionally cross the NODE layer to a real MMCS opening's siblings.
            let mut salted = opened_row.clone();
            salted.extend_from_slice(&[Val::ZERO; SALT_ELEMS]);
            let leaf_digest = leaf_hash_digest(&salted);
            let _ = (&opening_proof, inner_main_commit); // siblings used below for the real-node path

            // Build a reference tree over the SAME leaf-hash convention (row‖0^4) so the node-fold
            // (real MyCompress) reconstructs a self-consistent root for the (b-hash) leaf.
            let leaves: Vec<Digest> = (0..n)
                .map(|r| {
                    let row: Vec<Val> = (0..width).map(|c| matrix.values[r * width + c]).collect();
                    let mut s = row;
                    s.extend_from_slice(&[Val::ZERO; SALT_ELEMS]);
                    leaf_hash_digest(&s)
                })
                .collect();
            let tree = RefTree::new(leaves.clone());
            assert_eq!(tree.levels[0][index], leaf_digest, "ref leaf != in-AIR leaf hash");
            let sibs = tree.auth_path(index);
            let root = tree.root();

            let draw = Val::from_u32(index as u32);
            let e0 = Challenge::from_basis_coefficients_slice(&leaf_digest).unwrap();
            let inp = WindowInputs {
                draw_out: draw,
                e0,
                e1: ext_multi(2),
                beta: ext_multi(3),
                descent: &[],
                merkle_leaf: leaf_digest,
                merkle_siblings: &sibs,
                deep_zeta: Challenge::ZERO,
                deep_q: Challenge::ZERO,
                deep_alpha: Challenge::ZERO,
                fs_blocks: &[],
                sponge_in: [Val::ZERO; WIDTH],
                prior_state: [Val::ZERO; WIDTH],
                absorbed: [Val::ZERO; RATE],
                prefix_blocks: &[],
                public_io: &[],
                fs_beta_blocks: &[],
                fs_final_blocks: &[],
                inner_main_commit: root,
                layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
                leaf_row: &opened_row,
                leaf_salt: [Val::ZERO; SALT_ELEMS],
            };
            let row = pack_window(&cfg, &inp);
            assert_eq!(
                window_merkle_root(&cfg, &row),
                root,
                "in-AIR fold root != ref root at index {index}"
            );
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &pubs(leaf_digest, root), None);
            assert!(
                report.is_ok(),
                "(b-hash) honest leaf-row-hashed window rejected at index {index}: {} failures",
                report.failures.len()
            );
        }
    }

    /// Build a (b-hash) window honestly: leaf digest = MyHash(row‖salt), e0 = that digest, real
    /// siblings + root via a reference tree over the same leaf-hash convention.
    fn leaf_hash_window(
        cfg: &RecursiveConfig,
        draw: Val,
        index: usize,
        row: &[Val],
        salt: [Val; SALT_ELEMS],
        tree: &RefTree,
    ) -> (Vec<Val>, Digest, Digest) {
        let mut input = row.to_vec();
        input.extend_from_slice(&salt);
        let leaf = leaf_hash_digest(&input);
        let sibs = tree.auth_path(index);
        let root = tree.root();
        let e0 = Challenge::from_basis_coefficients_slice(&leaf).unwrap();
        let inp = WindowInputs {
            draw_out: draw,
            e0,
            e1: ext_multi(2),
            beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
            deep_alpha: Challenge::ZERO,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[],
            fs_final_blocks: &[],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: row,
            leaf_salt: salt,
        };
        (pack_window(cfg, &inp), leaf, root)
    }

    fn leaf_tree_for(width: usize, n: usize, salt: [Val; SALT_ELEMS]) -> (Vec<Vec<Val>>, RefTree) {
        let rows: Vec<Vec<Val>> = (0..n as u32).map(|r| sample_row(0x77 + r, width)).collect();
        let leaves: Vec<Digest> = rows
            .iter()
            .map(|row| {
                let mut s = row.clone();
                s.extend_from_slice(&salt);
                leaf_hash_digest(&s)
            })
            .collect();
        (rows, RefTree::new(leaves))
    }

    /// GOLDEN: an honest (b-hash) window where the level-0 leaf is DERIVED as MyHash(row‖salt) and
    /// bound to e0 ACCEPTS, across several row widths (partial / multi-block sponge) and indices.
    #[test]
    fn air_accepts_leaf_row_hashed_window() {
        for &width in &[5usize, 12, 20] {
            let levels = 4usize;
            let n = 1usize << levels;
            let cfg = RecursiveConfig {
                bind_merkle_leaf_to_e0: true,
                ..cfg_leaf_hash(levels, width)
            };
            let salt = sample_salt(0x33 + width as u32);
            let (rows, tree) = leaf_tree_for(width, n, salt);
            for &index in &[0usize, 1, 5, n - 1] {
                let draw = Val::from_u32(index as u32);
                let (row, leaf, root) = leaf_hash_window(&cfg, draw, index, &rows[index], salt, &tree);
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &pubs(leaf, root), None);
                assert!(
                    report.is_ok(),
                    "(b-hash) honest window rejected (width={width}, index={index}): {} failures",
                    report.failures.len()
                );
            }
        }
    }

    /// FORGERY — TAMPERED OPENED ROW: change one element of the opened row WITHOUT recomputing the
    /// hash perms. The (b-hash) constraint re-derives the digest from the row cells, so the perm's
    /// bound input no longer matches the absorbed row ⇒ rejected. (A prover cannot open a different
    /// row under the same authenticated leaf.)
    #[test]
    fn air_rejects_tampered_leaf_row() {
        let width = 12usize;
        let levels = 4usize;
        let n = 1usize << levels;
        let cfg = RecursiveConfig { bind_merkle_leaf_to_e0: true, ..cfg_leaf_hash(levels, width) };
        let salt = sample_salt(0x9);
        let (rows, tree) = leaf_tree_for(width, n, salt);
        let index = 5usize;
        let draw = Val::from_u32(index as u32);
        let (mut row, leaf, root) = leaf_hash_window(&cfg, draw, index, &rows[index], salt, &tree);
        // tamper a raw row cell (the perm input lane binding catches the divergence).
        row[lh_row(&cfg) + 3] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &pubs(leaf, root), None).is_ok(),
            "a tampered opened row was accepted — (b-hash) row binding not load-bearing"
        );
    }

    /// FORGERY — SUPPLIED-DIGEST SWAP: replace the level-0 running with a free chosen digest that
    /// authenticates a DIFFERENT leaf, while leaving the hashed-row perms honest. With (b-hash) the
    /// level-0 running is CONSTRAINED == MyHash(row); a free digest != H(row) rejects. CONTROL: the
    /// SAME row with (b-hash) OFF (level-0 running supplied) accepts — isolating (b-hash) as the
    /// rejector that closes the free-digest hole.
    #[test]
    fn air_rejects_free_leaf_digest_when_hashing_on() {
        let width = 12usize;
        let levels = 4usize;
        let n = 1usize << levels;
        let salt = sample_salt(0x9);
        let (rows, _tree) = leaf_tree_for(width, n, salt);
        let index = 5usize;
        let draw = Val::from_u32(index as u32);

        // Build an honest hashed leaf, then a tree whose level-0 leaf at `index` is a DIFFERENT,
        // free digest (so the path authenticates the free digest, not H(row)).
        let mut input = rows[index].clone();
        input.extend_from_slice(&salt);
        let honest_leaf = leaf_hash_digest(&input);
        let free_leaf = digest(0xDEAD); // != honest_leaf
        assert_ne!(free_leaf, honest_leaf);

        // tree over the FREE leaf at `index` (path authenticates free_leaf).
        let mut leaves: Vec<Digest> = (0..n)
            .map(|r| {
                let mut s = rows[r].clone();
                s.extend_from_slice(&salt);
                leaf_hash_digest(&s)
            })
            .collect();
        leaves[index] = free_leaf;
        let tree = RefTree::new(leaves);
        let sibs = tree.auth_path(index);
        let root = tree.root();

        // The window's hashed-row perms are HONEST (over rows[index]) ⇒ MyHash(row) = honest_leaf,
        // but merkle_leaf / level-0 running is the FREE digest. e0 bound to the free leaf.
        let e0 = Challenge::from_basis_coefficients_slice(&free_leaf).unwrap();
        let build = |cfg: &RecursiveConfig| -> Vec<Val> {
            let inp = WindowInputs {
                draw_out: draw,
                e0,
                e1: ext_multi(2),
                beta: ext_multi(3),
                descent: &[],
                merkle_leaf: free_leaf,
                merkle_siblings: &sibs,
                deep_zeta: Challenge::ZERO,
                deep_q: Challenge::ZERO,
                deep_alpha: Challenge::ZERO,
                fs_blocks: &[],
                sponge_in: [Val::ZERO; WIDTH],
                prior_state: [Val::ZERO; WIDTH],
                absorbed: [Val::ZERO; RATE],
                prefix_blocks: &[],
                public_io: &[],
                fs_beta_blocks: &[],
                fs_final_blocks: &[],
                inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
                layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
                leaf_row: &rows[index],
                leaf_salt: salt,
            };
            pack_window(cfg, &inp)
        };

        // CONTROL: hashing OFF (level-0 running supplied free) ⇒ the free digest authenticates ⇒ accepts.
        let cfg_off = RecursiveConfig { bind_merkle_leaf_to_e0: true, ..cfg_merkle(levels) };
        let row_off = build(&cfg_off);
        assert!(
            check_all_constraints(&air(cfg_off), &RowMajorMatrix::new(row_off.clone(), n_cols(&cfg_off)), &pubs(free_leaf, root), None).is_ok(),
            "control (b-hash off) should accept the free supplied leaf — test would prove nothing otherwise"
        );

        // ON: the level-0 running must equal MyHash(row) = honest_leaf != free_leaf ⇒ rejects.
        let cfg_on = RecursiveConfig { bind_merkle_leaf_to_e0: true, ..cfg_leaf_hash(levels, width) };
        let row_on = build(&cfg_on);
        let m = RowMajorMatrix::new(row_on, n_cols(&cfg_on));
        assert!(
            !check_all_constraints(&air(cfg_on), &m, &pubs(free_leaf, root), None).is_ok(),
            "a free leaf digest != MyHash(row) was accepted — (b-hash) not load-bearing"
        );
    }

    /// FORGERY — TAMPERED HASH PERM CELL: corrupt an interior committed cell of a leaf-hash perm
    /// block ⇒ `eval_perm` over that block rejects (the inlined Poseidon2 is fully constrained).
    #[test]
    fn air_rejects_tampered_leaf_hash_perm() {
        let width = 20usize; // 2-block sponge (20+4=24).
        let levels = 4usize;
        let n = 1usize << levels;
        let cfg = RecursiveConfig { bind_merkle_leaf_to_e0: true, ..cfg_leaf_hash(levels, width) };
        let salt = sample_salt(0x5);
        let (rows, tree) = leaf_tree_for(width, n, salt);
        let index = 3usize;
        let draw = Val::from_u32(index as u32);
        let (mut row, leaf, root) = leaf_hash_window(&cfg, draw, index, &rows[index], salt, &tree);
        // corrupt an interior committed cell of the SECOND leaf-hash perm block.
        let pb = lh_perm_base(&cfg, 1);
        row[pb + inlined_perm::C_BEGIN_POST + 5] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &pubs(leaf, root), None).is_ok(),
            "a tampered leaf-hash perm cell was accepted — eval_perm not load-bearing over (b-hash)"
        );
    }

    // ---------------------------------------------------------------------------------
    // (c-pub) public-IO statement preservation in-window.
    // ---------------------------------------------------------------------------------

    fn cfg_public_io(h: usize, len: usize) -> RecursiveConfig {
        RecursiveConfig { public_io_len: len, ..RecursiveConfig::single_window(h) }
    }

    fn pio_window(cfg: &RecursiveConfig, draw: Val, stmt: &[(Val, Val, Val)]) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: draw,
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: stmt,
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest public-IO block (outer == observed == inner, per element) ACCEPTS.
    #[test]
    fn air_accepts_public_io_block() {
        let h = 8usize;
        let len = 19usize;
        let cfg = cfg_public_io(h, len);
        let stmt: Vec<(Val, Val, Val)> =
            (0..len as u32).map(|i| { let v = Val::from_u32(1000 + i); (v, v, v) }).collect();
        let row = pio_window(&cfg, Val::from_u32(0xABCDE), &stmt);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let report = check_all_constraints(&air(cfg), &m, &[], None);
        assert!(report.is_ok(), "honest public-IO block rejected: {} failures", report.failures.len());
    }

    /// (c-pub) FORGERY — STATEMENT SUBSTITUTION: the inner proof's public value differs from the
    /// re-exposed outer value at one element ⇒ outer != inner ⇒ rejected. The prover cannot verify
    /// statement P yet re-expose P'.
    #[test]
    fn air_rejects_public_io_substitution() {
        let h = 8usize;
        let len = 19usize;
        let cfg = cfg_public_io(h, len);
        let stmt: Vec<(Val, Val, Val)> =
            (0..len as u32).map(|i| { let v = Val::from_u32(1000 + i); (v, v, v) }).collect();
        let mut row = pio_window(&cfg, Val::from_u32(0xABCDE), &stmt);
        // corrupt the INNER cell of element 7 (inner != outer) — statement substitution.
        let pb = public_io_base(&cfg);
        row[pb + 7 * PIO_STRIDE + PIO_INNER] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a substituted inner statement was accepted — (c-pub) not load-bearing"
        );
    }

    /// (c-pub) FORGERY — FORGED OBSERVED cell: the value FS-observed by gadget 1 differs from the
    /// outer/inner value ⇒ rejected (observed != outer).
    #[test]
    fn air_rejects_public_io_forged_observed() {
        let h = 8usize;
        let len = 12usize;
        let cfg = cfg_public_io(h, len);
        let stmt: Vec<(Val, Val, Val)> =
            (0..len as u32).map(|i| { let v = Val::from_u32(2000 + i); (v, v, v) }).collect();
        let mut row = pio_window(&cfg, Val::from_u32(0x1234), &stmt);
        let pb = public_io_base(&cfg);
        row[pb + 5 * PIO_STRIDE + PIO_OBSERVED] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // FULL MONOLITH + the three new pieces together.
    // ---------------------------------------------------------------------------------

    /// GOLDEN: a window with the sponge-derived index draw (a-FS), Merkle path with leaf↔e0
    /// (b-leaf), and a public-IO block (c-pub) — all in ONE window — ACCEPTS. The index is the
    /// genuine Poseidon2 squeeze, the opened leaf IS the folded e0, the statement is preserved.
    #[test]
    fn air_accepts_monolith_with_sponge_leaf_e0_and_public_io() {
        let levels = 4usize;
        let pio_len = 10usize;
        let cfg = RecursiveConfig {
            log_height: levels,
            merkle_levels: levels,
            bind_merkle_leaf_to_e0: true,
            inline_sponge_draw: true,
            public_io_len: pio_len,
            ..RecursiveConfig::single_window(levels)
        };
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();

        // pick a sponge state; the index is the genuine squeeze of it.
        let sin = sponge_state(0xBEEF);
        let draw = squeeze_draw(sin);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let leaf = leaves[index];
        let e0 = Challenge::from_basis_coefficients_slice(&leaf).unwrap(); // leaf↔e0
        let sibs = tree.auth_path(index);
        let stmt: Vec<(Val, Val, Val)> =
            (0..pio_len as u32).map(|i| { let v = Val::from_u32(7000 + i); (v, v, v) }).collect();
        let inp = WindowInputs {
            draw_out: Val::ZERO, // derived from sponge_in
            e0, e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO, deep_alpha: Challenge::ZERO, fs_blocks: &[],
            sponge_in: sin,
            prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &stmt,
            fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        let row = pack_window(&cfg, &inp);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let report = check_all_constraints(&air(cfg), &m, &pubs(leaf, root), None);
        assert!(report.is_ok(), "monolith (sponge+leaf↔e0+public-IO) rejected: {} failures", report.failures.len());
    }

    // ---------------------------------------------------------------------------------
    // (FS-αζ) SQUEEZE alpha + zeta from the inline FS challenger, p3-uni-stark order.
    // ---------------------------------------------------------------------------------

    use p3_challenger::{CanObserve, CanSample};
    use descender_account_stark::Challenger;
    use p3_koala_bear::default_koalabear_poseidon2_16;

    fn cfg_fs(h: usize, k: usize, pre_rounds: usize) -> RecursiveConfig {
        RecursiveConfig {
            deep_log_degree: Some(k),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            ..RecursiveConfig::single_window(h)
        }
    }

    /// A representative observed-transcript block (RATE elements) — stands in for a chunk of the
    /// observed (metadata ‖ trace-commitment ‖ public-values ‖ quotient-commitment) stream.
    fn fs_block(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 131 + 7))
        })
    }

    fn fs_window(cfg: &RecursiveConfig, blocks: &[[Val; RATE]], q: Challenge) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: Val::from_u32(0xABCDE),
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: q, deep_alpha: Challenge::ZERO,
            fs_blocks: blocks,
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[], fs_final_blocks: &[], inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// FIDELITY-PIN: `fs_squeeze_alpha_zeta` (and hence the packed bus) reproduces the SAME alpha
    /// and zeta a REAL `DuplexChallenger` produces for the SAME observations in the SAME p3 order —
    /// observe pre-alpha prefix → sample_algebra_element (alpha) → observe quotient → sample (zeta).
    /// Element-for-element equality against the native challenger, NOT a hand-picked constant.
    #[test]
    fn fs_squeeze_matches_native_duplex_challenger() {
        // pre_rounds >= 1: the real leaf ALWAYS observes (metadata ‖ trace commitment ‖ public
        // values) before sampling alpha, so the alpha squeeze source is permute(absorb(...)), never
        // the raw zero state. (A pre_rounds == 0 config would squeeze the un-permuted zero boundary,
        // which the native challenger never does — its first sample with an empty output buffer still
        // duplex-permutes. That config is disallowed; see fs_squeeze_alpha_zeta's assert.)
        for pre_rounds in [1usize, 2, 3] {
            let pre: Vec<[Val; RATE]> = (0..pre_rounds as u32).map(|r| fs_block(10 + r)).collect();
            let quot = fs_block(900);

            // native: observe each pre-alpha block (RATE elems each, auto-duplexing on the 8th),
            // sample alpha (extension = 8 base draws), observe quotient block, sample zeta.
            let mut ch = Challenger::new(default_koalabear_poseidon2_16());
            for block in &pre {
                for &x in block.iter() {
                    ch.observe(x);
                }
            }
            let alpha_native: Challenge = {
                Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() })
            };
            for &x in quot.iter() {
                ch.observe(x);
            }
            let zeta_native: Challenge = {
                Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() })
            };

            let (alpha, zeta) = fs_squeeze_alpha_zeta(&pre, quot);
            assert_eq!(alpha, alpha_native, "FS alpha diverges from native challenger (pre={pre_rounds})");
            assert_eq!(zeta, zeta_native, "FS zeta diverges from native challenger (pre={pre_rounds})");
        }
    }

    /// GOLDEN (positive): an honest FS-αζ window — pre-alpha absorb chain ‖ alpha squeeze ‖ quotient
    /// round ‖ zeta squeeze, with the DEEP folder's alpha/zeta cells bound to the squeezed values —
    /// ACCEPTS via the real `check_all_constraints`, across several pre-round counts + log_degrees.
    #[test]
    fn air_accepts_fs_alpha_zeta() {
        for &pre_rounds in &[1usize, 2, 3] {
            for &k in &[4usize, 8, 12] {
                let cfg = cfg_fs(10, k, pre_rounds);
                let blocks: Vec<[Val; RATE]> =
                    (0..(pre_rounds + 1) as u32).map(|r| fs_block(20 + r)).collect();
                let row = fs_window(&cfg, &blocks, ext_multi(66));
                // the DEEP alpha/zeta cells equal the FS squeeze.
                let (alpha, zeta) = fs_squeeze_alpha_zeta(&blocks[..pre_rounds], blocks[pre_rounds]);
                let db = deep_base(&cfg);
                assert_eq!(
                    Challenge::from_basis_coefficients_slice(&row[db + DEEP_ALPHA..db + DEEP_ALPHA + D]).unwrap(),
                    alpha, "DEEP alpha cell != FS-squeezed alpha"
                );
                assert_eq!(
                    Challenge::from_basis_coefficients_slice(&row[db + DEEP_ZETA..db + DEEP_ZETA + D]).unwrap(),
                    zeta, "DEEP zeta cell != FS-squeezed zeta"
                );
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(report.is_ok(), "honest FS-αζ window rejected (pre={pre_rounds}, k={k}): {} failures", report.failures.len());
            }
        }
    }

    /// (FS-αζ) FORGERY — a DEEP alpha NOT equal to the squeezed alpha is rejected. We keep the FS
    /// squeeze block honest and overwrite the DEEP alpha cell (and re-derive nothing — alpha is not
    /// used in the C==Z_H·Q relation in this config, so the ONLY rejector is the FS binding).
    /// Tested by attempted forgery, never substring scan.
    #[test]
    fn air_rejects_forged_deep_alpha() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(30 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(7));
        let db = deep_base(&cfg);
        row[db + DEEP_ALPHA + 2] += Val::ONE; // alpha != squeezed alpha
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a DEEP alpha != the FS-squeezed alpha was accepted — alpha not FS-bound"
        );
    }

    /// (FS-αζ) FORGERY — a DEEP zeta NOT equal to the squeezed zeta is rejected. CONTROL: the DEEP
    /// block is self-consistent (C == Z_H(zeta)·Q) at the honest zeta; forging zeta to a DIFFERENT
    /// OOD point while keeping the FS squeeze honest breaks the zeta↔squeeze binding (and the DEEP
    /// tower). A forger cannot open at a favorable OOD point.
    #[test]
    fn air_rejects_forged_deep_zeta() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(40 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(9));
        let db = deep_base(&cfg);
        row[db + DEEP_ZETA + 1] += Val::ONE; // zeta != squeezed zeta
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a DEEP zeta != the FS-squeezed zeta was accepted — zeta not FS-bound"
        );
    }

    /// (FS-αζ) FORGERY — changing an OBSERVED transcript block CHANGES the squeezed alpha/zeta (so a
    /// forger cannot fix challenges independent of the statement). We flip one element of a pre-alpha
    /// block: the honest squeeze of the TAMPERED block differs from the alpha/zeta in the (now stale)
    /// DEEP cells ⇒ rejected. The squeeze permute is honest for the tampered block (we re-pack), so
    /// the rejector is purely "observed block changed ⇒ challenge changed ⇒ binding broke".
    #[test]
    fn air_rejects_changed_observed_block_keeping_stale_challenge() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(50 + r)).collect();
        // honest window for the original blocks.
        let honest = fs_window(&cfg, &blocks, ext_multi(11));
        assert!(check_all_constraints(&air(cfg), &RowMajorMatrix::new(honest.clone(), n_cols(&cfg)), &[], None).is_ok());

        // tamper a pre-alpha block, re-pack the FS squeeze HONESTLY for the tampered block (so the
        // perm chain is internally valid), but KEEP the OLD DEEP alpha/zeta cells (the stale challenge
        // the forger wanted to lock in). The squeeze now yields a different alpha/zeta ⇒ binding fails.
        let mut tampered = blocks.clone();
        tampered[0][3] += Val::ONE;
        let (old_alpha, old_zeta) = fs_squeeze_alpha_zeta(&blocks[..2], blocks[2]);
        let (new_alpha, new_zeta) = fs_squeeze_alpha_zeta(&tampered[..2], tampered[2]);
        assert_ne!(old_alpha, new_alpha, "observed block change did not move alpha");
        assert_ne!(old_zeta, new_zeta, "observed block change did not move zeta");

        let mut row = fs_window(&cfg, &tampered, ext_multi(11));
        // re-pin the DEEP block to the OLD (stale) challenge: zeta + its tower + C, and alpha.
        let db = deep_base(&cfg);
        let k = 8usize;
        let zh = crate::deep::vanishing_poly_at(old_zeta, k);
        let c = zh * ext_multi(11);
        row[db + DEEP_ZETA..db + DEEP_ZETA + D].copy_from_slice(&limbs(old_zeta));
        row[db + DEEP_ALPHA..db + DEEP_ALPHA + D].copy_from_slice(&limbs(old_alpha));
        row[db + DEEP_C..db + DEEP_C + D].copy_from_slice(&limbs(c));
        let mut p = old_zeta;
        for j in 0..=k {
            row[db + DEEP_POW + j * D..db + DEEP_POW + j * D + D].copy_from_slice(&limbs(p));
            p = p * p;
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a changed observed block with a stale (favorable) challenge was accepted — FS not statement-bound"
        );
    }

    /// (FS-αζ) FORGERY — a tampered FS perm interior cell ⇒ the inlined Poseidon2 round constraints
    /// reject (the squeeze permute is fully constrained, not just the input copy).
    #[test]
    fn air_rejects_fs_tampered_perm() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(60 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(13));
        // corrupt a begin-post cell of the FIRST pre-alpha round's perm.
        let rb = fs_round_base(&cfg, 0);
        row[rb + FS_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-αζ) FORGERY — a forged FS round length tag (capacity slot RATE not == prev[RATE] + RATE)
    /// is rejected (the prefix-free absorb tag is what makes the transcript binding non-malleable).
    #[test]
    fn air_rejects_fs_forged_length_tag() {
        let cfg = cfg_fs(10, 8, 1);
        let blocks: Vec<[Val; RATE]> = (0..2u32).map(|r| fs_block(70 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(15));
        // corrupt the quotient round's perm input capacity length-tag slot.
        let qb = fs_quot_round_base(&cfg);
        row[qb + FS_PERM + inlined_perm::C_INPUTS + RATE] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-αζ) FORGERY — broken inter-round carry: corrupt the SECOND pre-alpha round's perm input
    /// (which must equal absorb(round-0 output, block_1, RATE)); the round-0→round-1 chain breaks.
    #[test]
    fn air_rejects_fs_broken_interround_carry() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(80 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(17));
        // tamper a CAPACITY input lane of round 1 (carried from round 0's output) so the carry breaks.
        let rb1 = fs_round_base(&cfg, 1);
        row[rb1 + FS_PERM + inlined_perm::C_INPUTS + (RATE + 1)] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-αζ) FORGERY — REORDERED FS chain (alpha squeezed AFTER observing the quotient): if the
    /// alpha cell is pinned to the QUOTIENT round's squeeze (a wrong FS order) instead of the
    /// pre-alpha state, it no longer equals the honest pre-alpha squeeze ⇒ rejected. Proves the
    /// observe→alpha→observe→zeta ORDER is load-bearing (a wrong order is a soundness bug).
    #[test]
    fn air_rejects_fs_reordered_alpha_after_quotient() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(90 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(19));
        // set the DEEP alpha cell to the QUOTIENT round's squeeze (= zeta) — the "alpha after observe
        // quotient" wrong order. Honest alpha is the pre-alpha squeeze, so this differs ⇒ reject.
        let qb = fs_quot_round_base(&cfg);
        let wrong_alpha: [Val; D] =
            core::array::from_fn(|i| row[qb + FS_PERM + C_OUTPUT + (RATE - 1 - i)]);
        let db = deep_base(&cfg);
        row[db + DEEP_ALPHA..db + DEEP_ALPHA + D].copy_from_slice(&wrong_alpha);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "alpha squeezed in the WRONG FS order (after the quotient observe) was accepted"
        );
    }

    /// (FS-αζ) FORGERY — non-zero round-0 boundary state: the first pre-alpha round must absorb into
    /// the new() ALL-ZERO sponge. Forcing a non-zero round-0 input capacity (a non-zero state_0)
    /// breaks the boundary length-tag constraint (state_0[RATE] must be 0 ⇒ slot RATE == RATE).
    #[test]
    fn air_rejects_fs_nonzero_round0_boundary() {
        let cfg = cfg_fs(10, 8, 2);
        let blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(110 + r)).collect();
        let mut row = fs_window(&cfg, &blocks, ext_multi(21));
        // corrupt round-0 perm input capacity slot RATE (the length tag): boundary requires == RATE.
        let rb0 = fs_round_base(&cfg, 0);
        row[rb0 + FS_PERM + inlined_perm::C_INPUTS + RATE] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-αζ) PUBLIC-TRANSCRIPT BINDING: with `fs_blocks_public`, the FS rounds' absorbed blocks are
    /// pinned to the public-values vector (the actual observed inner commitments + public digest).
    /// Honest accepts; a public value inconsistent with the absorbed block rejects.
    #[test]
    fn air_fs_blocks_bound_to_public_transcript() {
        let pre_rounds = 2usize;
        let cfg = RecursiveConfig {
            deep_log_degree: Some(8),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            fs_blocks_public: true,
            ..RecursiveConfig::single_window(10)
        };
        let blocks: Vec<[Val; RATE]> = (0..(pre_rounds + 1) as u32).map(|r| fs_block(130 + r)).collect();
        let row = fs_window(&cfg, &blocks, ext_multi(23));
        // public values = the concatenated absorbed blocks (the committed transcript).
        let pubs: Vec<Val> = blocks.iter().flatten().copied().collect();
        assert_eq!(pubs.len(), (pre_rounds + 1) * RATE);
        let m = RowMajorMatrix::new(row.clone(), n_cols(&cfg));
        assert!(
            check_all_constraints(&air(cfg), &m, &pubs, None).is_ok(),
            "honest FS public-transcript binding rejected"
        );
        // a public value inconsistent with the absorbed block ⇒ rejected.
        let mut bad_pubs = pubs.clone();
        bad_pubs[RATE + 2] += Val::ONE;
        assert!(
            !check_all_constraints(&air(cfg), &RowMajorMatrix::new(row, n_cols(&cfg)), &bad_pubs, None).is_ok(),
            "a public transcript inconsistent with the FS absorbed block was accepted"
        );
    }

    // ---------------------------------------------------------------------------------
    // (FS-β) SQUEEZE the FRI commit-phase betas, CONTINUING the post-zeta transcript.
    // p3-fri-0.6.1 verifier.rs:299-312 order: per round observe(comm_r) → sample beta_r.
    // ---------------------------------------------------------------------------------

    /// A representative FRI commit-phase commitment block (RATE elements).
    fn fb_block(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2246822519).wrapping_add(i as u32 * 97 + 3))
        })
    }

    fn cfg_fb(h: usize, k: usize, pre_rounds: usize, beta_rounds: usize) -> RecursiveConfig {
        RecursiveConfig {
            num_rounds: beta_rounds, // one descent round per FRI commit-phase beta
            deep_log_degree: Some(k),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            fs_beta_rounds: beta_rounds,
            ..RecursiveConfig::single_window(h)
        }
    }

    /// Build an FS-β window: alpha/zeta prefix blocks + beta commit-phase blocks; the descent rounds
    /// (one per beta) are folded with the FS-squeezed betas (the supplied descent betas are ignored
    /// for those rounds — only the descent e1 sibling matters).
    fn fb_window(
        cfg: &RecursiveConfig,
        fs_blocks: &[[Val; RATE]],
        beta_blocks: &[[Val; RATE]],
        q: Challenge,
    ) -> Vec<Val> {
        fb_window_full(cfg, fs_blocks, beta_blocks, &[], q)
    }

    /// Like `fb_window` but also supplies the post-beta (FS-qi) observe blocks (final_poly ‖
    /// log_arities ‖ query-PoW witness, RATE-chunked) — for the unified query-index draw.
    fn fb_window_full(
        cfg: &RecursiveConfig,
        fs_blocks: &[[Val; RATE]],
        beta_blocks: &[[Val; RATE]],
        final_blocks: &[[Val; RATE]],
        q: Challenge,
    ) -> Vec<Val> {
        // descent e1 siblings for rounds 1..num_rounds (beta is overridden, so any value works).
        let descent: Vec<(Challenge, Challenge)> = (1..cfg.rounds())
            .map(|r| (ext_multi(500 + r as u32), Challenge::ZERO))
            .collect();
        let inp = WindowInputs {
            draw_out: Val::from_u32(0xABCDE),
            e0: ext_multi(1), e1: ext_multi(2), beta: Challenge::ZERO,
            descent: &descent,
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: q, deep_alpha: Challenge::ZERO,
            fs_blocks,
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[], public_io: &[],
            fs_beta_blocks: beta_blocks,
            fs_final_blocks: final_blocks,
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[], final_poly_x: Val::ZERO, final_poly_coeffs: &[],
            leaf_row: &[], leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        pack_window(cfg, &inp)
    }

    /// FIDELITY-PIN: `fs_squeeze_betas` (and hence the packed bus) reproduces EVERY beta_r a REAL
    /// `DuplexChallenger` produces, driving observe-main→alpha→observe-quot→zeta→[observe-comm_r→
    /// beta_r]* in the EXACT p3 + p3-fri order. Element-for-element equality against the native
    /// challenger for each beta_r, NOT a hand-picked constant.
    #[test]
    fn fs_squeeze_betas_match_native_duplex_challenger() {
        for pre_rounds in [1usize, 2] {
            for beta_rounds in [1usize, 2, 3] {
                let pre: Vec<[Val; RATE]> = (0..pre_rounds as u32).map(|r| fs_block(10 + r)).collect();
                let quot = fs_block(900);
                let comms: Vec<[Val; RATE]> =
                    (0..beta_rounds as u32).map(|r| fb_block(200 + r)).collect();

                // native: observe pre-alpha prefix, sample alpha, observe quotient, sample zeta,
                // then per FRI commit-phase round: observe(comm_r), sample beta_r.
                let mut ch = Challenger::new(default_koalabear_poseidon2_16());
                for block in &pre {
                    for &x in block.iter() { ch.observe(x); }
                }
                let _alpha: Challenge = Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() });
                for &x in quot.iter() { ch.observe(x); }
                let _zeta: Challenge = Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() });
                let betas_native: Vec<Challenge> = comms
                    .iter()
                    .map(|comm| {
                        for &x in comm.iter() { ch.observe(x); }
                        Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() })
                    })
                    .collect();

                // ours: continue from the post-zeta state.
                let start = fs_post_zeta_state(&pre, quot);
                let (betas, _) = fs_squeeze_betas(start, &comms);
                assert_eq!(betas.len(), beta_rounds);
                for (r, (b, bn)) in betas.iter().zip(betas_native.iter()).enumerate() {
                    assert_eq!(
                        b, bn,
                        "FS beta_{r} diverges from native challenger (pre={pre_rounds}, rounds={beta_rounds})"
                    );
                }
            }
        }
    }

    /// GOLDEN (positive): an honest FS-β window — alpha/zeta squeeze ‖ per-round commit observe +
    /// beta squeeze, with each FRI fold consuming the FS-squeezed beta_r — ACCEPTS via the real
    /// `check_all_constraints`, across pre-round and beta-round counts.
    #[test]
    fn air_accepts_fs_betas() {
        for &pre_rounds in &[1usize, 2] {
            for &beta_rounds in &[1usize, 2, 3] {
                let cfg = cfg_fb(12, 8, pre_rounds, beta_rounds);
                let fs_blocks: Vec<[Val; RATE]> =
                    (0..(pre_rounds + 1) as u32).map(|r| fs_block(20 + r)).collect();
                let comms: Vec<[Val; RATE]> =
                    (0..beta_rounds as u32).map(|r| fb_block(300 + r)).collect();
                let row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(66));
                // each FS-β round's beta cell == the squeezed beta_r, and the descent consumer cell
                // equals it.
                let start = fs_post_zeta_state(&fs_blocks[..pre_rounds], fs_blocks[pre_rounds]);
                let (betas, _) = fs_squeeze_betas(start, &comms);
                for r in 0..beta_rounds {
                    let rb = fb_round_base(&cfg, r);
                    assert_eq!(
                        Challenge::from_basis_coefficients_slice(&row[rb + FB_BETA..rb + FB_BETA + D]).unwrap(),
                        betas[r], "FS-β round {r} beta cell != squeezed beta_r"
                    );
                }
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(
                    report.is_ok(),
                    "honest FS-β window rejected (pre={pre_rounds}, rounds={beta_rounds}): {} failures",
                    report.failures.len()
                );
            }
        }
    }

    /// (FS-β) FORGERY — a FOLD using a beta != the squeezed beta_r is rejected. We keep the FS-β
    /// squeeze block honest and corrupt the descent's consumer beta cell (round 0's C_BETA) plus
    /// re-fold it; the consumer != squeezed binding rejects. Attempted forgery, never substring scan.
    #[test]
    fn air_rejects_fold_beta_not_squeezed() {
        let cfg = cfg_fb(12, 8, 2, 2);
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(30 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(400 + r)).collect();
        let mut row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(7));
        // corrupt round-0's consumer beta cell (C_BETA) so the fold uses beta != squeezed beta_0.
        row[C_BETA + 1] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a FRI fold using a beta != the FS-squeezed beta_0 was accepted — beta not FS-bound"
        );
    }

    /// (FS-β) FORGERY — corrupting the squeezed beta CELL (not the consumer) is rejected: it must
    /// equal the round's LIFO squeeze pop. Pins beta_r to the genuine permute output.
    #[test]
    fn air_rejects_tampered_beta_cell() {
        let cfg = cfg_fb(12, 8, 1, 2);
        let fs_blocks: Vec<[Val; RATE]> = (0..2u32).map(|r| fs_block(45 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(420 + r)).collect();
        let mut row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(8));
        let rb = fb_round_base(&cfg, 1);
        row[rb + FB_BETA + 2] += Val::ONE; // beta_1 cell != squeeze pop
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a beta cell != the round's squeeze pop was accepted — beta not bound to the permute"
        );
    }

    /// (FS-β) FORGERY — changing a fold-round COMMITMENT changes that round's beta AND every
    /// subsequent beta (the carried sponge state threads the rounds). We tamper comm_0, re-pack the
    /// squeeze HONESTLY for the tampered comm, but KEEP the OLD descent consumer betas (the stale
    /// favorable challenges) ⇒ rejected. Proves betas are bound to the commitments.
    #[test]
    fn air_rejects_changed_fold_commitment_keeping_stale_betas() {
        let cfg = cfg_fb(12, 8, 2, 3);
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(50 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..3u32).map(|r| fb_block(440 + r)).collect();
        // honest window accepts.
        let honest = fb_window(&cfg, &fs_blocks, &comms, ext_multi(11));
        assert!(check_all_constraints(&air(cfg), &RowMajorMatrix::new(honest, n_cols(&cfg)), &[], None).is_ok());

        // changing comm_0 must move beta_0 AND beta_1 AND beta_2 (carried state).
        let mut tampered = comms.clone();
        tampered[0][4] += Val::ONE;
        let start = fs_post_zeta_state(&fs_blocks[..2], fs_blocks[2]);
        let (old_betas, _) = fs_squeeze_betas(start, &comms);
        let (new_betas, _) = fs_squeeze_betas(start, &tampered);
        for r in 0..3 {
            assert_ne!(old_betas[r], new_betas[r], "comm_0 change did not move beta_{r} (subsequent-round binding)");
        }

        // re-pack with the tampered comm (honest squeeze), then OVERWRITE the descent consumer betas
        // back to the OLD (stale) betas and re-fold each round so the per-round (★) is internally
        // consistent at the stale beta — the ONLY rejector is consumer != squeezed.
        let mut row = fb_window(&cfg, &fs_blocks, &tampered, ext_multi(11));
        // round 0 consumer = C_BETA; re-fold round 0 with old beta_0.
        let e0_0 = Challenge::from_basis_coefficients_slice(&row[C_E0..C_E0 + D]).unwrap();
        let e1_0 = Challenge::from_basis_coefficients_slice(&row[C_E1..C_E1 + D]).unwrap();
        let s0 = row[C_S];
        let folded0 = fold_pair(e0_0, e1_0, old_betas[0], s0);
        row[C_BETA..C_BETA + D].copy_from_slice(&limbs(old_betas[0]));
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded0));
        // rounds 1.. : re-fold with old beta_r, threading the chain-link e0_r == folded_{r-1}.
        let mut folded_prev = folded0;
        for r in 1..cfg.rounds() {
            let b = extra_round_base(&cfg, r);
            let e1_r = Challenge::from_basis_coefficients_slice(&row[b + D..b + 2 * D]).unwrap();
            let s_r = row[b + 3 * D];
            let e0_r = folded_prev;
            let folded_r = fold_pair(e0_r, e1_r, old_betas[r], s_r);
            row[b..b + D].copy_from_slice(&limbs(e0_r));
            row[b + 2 * D..b + 3 * D].copy_from_slice(&limbs(old_betas[r]));
            row[b + 3 * D + 1..b + 4 * D + 1].copy_from_slice(&limbs(folded_r));
            folded_prev = folded_r;
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a changed fold commitment with stale (favorable) betas was accepted — betas not commitment-bound"
        );
    }

    /// (FS-β) FORGERY — WRONG-ROUND beta (using round r+1's beta in round r) is rejected. The carried
    /// state makes beta_0 != beta_1; placing beta_1 in round 0's consumer (and re-folding) breaks the
    /// round-0 consumer == squeezed-beta_0 binding. Proves the per-round observe→sample ORDER /
    /// round-indexing is load-bearing (a swapped beta is a soundness bug).
    #[test]
    fn air_rejects_wrong_round_beta() {
        let cfg = cfg_fb(12, 8, 2, 2);
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(60 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(460 + r)).collect();
        let start = fs_post_zeta_state(&fs_blocks[..2], fs_blocks[2]);
        let (betas, _) = fs_squeeze_betas(start, &comms);
        assert_ne!(betas[0], betas[1], "beta_0 == beta_1 (carried-state threading broke)");

        let mut row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(13));
        // put beta_1 in round 0's consumer (C_BETA) and re-fold round 0 — wrong-round beta.
        let e0_0 = Challenge::from_basis_coefficients_slice(&row[C_E0..C_E0 + D]).unwrap();
        let e1_0 = Challenge::from_basis_coefficients_slice(&row[C_E1..C_E1 + D]).unwrap();
        let folded0 = fold_pair(e0_0, e1_0, betas[1], row[C_S]);
        row[C_BETA..C_BETA + D].copy_from_slice(&limbs(betas[1]));
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded0));
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "round 0 using round 1's beta (wrong-round) was accepted — beta round-indexing not bound"
        );
    }

    /// (FS-β) FORGERY — a tampered FS-β perm interior cell ⇒ the inlined Poseidon2 round constraints
    /// reject (the beta squeeze permute is fully constrained, not just the input copy).
    #[test]
    fn air_rejects_fs_beta_tampered_perm() {
        let cfg = cfg_fb(12, 8, 1, 2);
        let fs_blocks: Vec<[Val; RATE]> = (0..2u32).map(|r| fs_block(70 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(480 + r)).collect();
        let mut row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(15));
        let rb = fb_round_base(&cfg, 0);
        row[rb + FB_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-β) FORGERY — broken inter-round carry: corrupt round 1's perm input CAPACITY lane (which
    /// must equal absorb(round-0 output, comm_1, RATE)'s capacity carry) so the round-0→round-1 carry
    /// breaks (the carried-state threading is load-bearing).
    #[test]
    fn air_rejects_fs_beta_broken_interround_carry() {
        let cfg = cfg_fb(12, 8, 2, 2);
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(80 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(500 + r)).collect();
        let mut row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(17));
        let rb1 = fb_round_base(&cfg, 1);
        row[rb1 + FB_PERM + inlined_perm::C_INPUTS + (RATE + 1)] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-β) FORGERY — broken continuation from the post-zeta state: corrupt round 0's perm input
    /// CAPACITY lane (which must be carried from the quotient-observe round's output = the post-zeta
    /// state). Proves the beta chain genuinely CONTINUES the alpha/zeta transcript (not a fresh chain).
    #[test]
    fn air_rejects_fs_beta_broken_postzeta_continuation() {
        let cfg = cfg_fb(12, 8, 1, 1);
        let fs_blocks: Vec<[Val; RATE]> = (0..2u32).map(|r| fs_block(90 + r)).collect();
        let comms: Vec<[Val; RATE]> = vec![fb_block(520)];
        let mut row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(19));
        let rb0 = fb_round_base(&cfg, 0);
        // round 0's perm input capacity slot RATE+1 must == post-zeta state slot RATE+1; corrupt it.
        row[rb0 + FB_PERM + inlined_perm::C_INPUTS + (RATE + 1)] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a round-0 beta chain NOT continuing the post-zeta state was accepted — chain not unified"
        );
    }

    /// (FS-β) PUBLIC-TRANSCRIPT BINDING: with `fs_betas_public`, the FRI commit-phase commitments are
    /// pinned to the public-values vector. Honest accepts; a public value inconsistent with the
    /// absorbed commitment rejects.
    #[test]
    fn air_fs_betas_bound_to_public_transcript() {
        let beta_rounds = 2usize;
        let cfg = RecursiveConfig {
            num_rounds: beta_rounds,
            deep_log_degree: Some(8),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: 2,
            fs_beta_rounds: beta_rounds,
            fs_betas_public: true,
            ..RecursiveConfig::single_window(12)
        };
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(130 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..beta_rounds as u32).map(|r| fb_block(540 + r)).collect();
        let row = fb_window(&cfg, &fs_blocks, &comms, ext_multi(23));
        let pubs: Vec<Val> = comms.iter().flatten().copied().collect();
        assert_eq!(pubs.len(), beta_rounds * RATE);
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(row.clone(), n_cols(&cfg)), &pubs, None).is_ok(),
            "honest FS-β public-transcript binding rejected"
        );
        let mut bad_pubs = pubs.clone();
        bad_pubs[RATE + 1] += Val::ONE;
        assert!(
            !check_all_constraints(&air(cfg), &RowMajorMatrix::new(row, n_cols(&cfg)), &bad_pubs, None).is_ok(),
            "a public transcript inconsistent with the FS-β absorbed commitment was accepted"
        );
    }

    // ---------------------------------------------------------------------------------
    // (FS-bca) FRI batch-combination alpha (verifier.rs:195) + (FS-qi) unified query index
    // (verifier.rs:317-356). The FULL ordered transcript: observe-main→alpha→observe-quot→zeta→
    // batch-combo-alpha→[observe-comm_r→beta_r]*→observe-final-poly→sample-index.
    // ---------------------------------------------------------------------------------

    use p3_challenger::CanSampleBits;

    /// A representative post-beta transcript block (final_poly ‖ log_arities ‖ query-PoW witness),
    /// RATE elements.
    fn fq_block(seed: u32) -> [Val; RATE] {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 * 131 + 7))
        })
    }

    /// FS config with the batch-combo alpha and/or unified query index enabled.
    fn cfg_fbq(
        h: usize,
        k: usize,
        pre_rounds: usize,
        beta_rounds: usize,
        batch_combo: bool,
        final_blocks: usize,
    ) -> RecursiveConfig {
        RecursiveConfig {
            num_rounds: beta_rounds,
            deep_log_degree: Some(k),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            fs_beta_rounds: beta_rounds,
            fs_batch_combo_alpha: batch_combo,
            fs_final_blocks: final_blocks,
            ..RecursiveConfig::single_window(h)
        }
    }

    /// Drive a native `DuplexChallenger` through the FULL ordered FS sequence and return every
    /// challenge: (alpha_uni, zeta, batch_combo_alpha, betas, query_index). Mirrors p3-uni-stark +
    /// p3-fri exactly: observe pre-alpha prefix → sample alpha → observe quot → sample zeta → sample
    /// batch-combo alpha (NO observe) → per round observe(comm_r) → sample beta_r → observe final
    /// blocks → sample_bits(index_bits).
    #[allow(clippy::type_complexity)]
    fn native_full_transcript(
        pre: &[[Val; RATE]],
        quot: [Val; RATE],
        comms: &[[Val; RATE]],
        final_blocks: &[[Val; RATE]],
        index_bits: usize,
    ) -> (Challenge, Challenge, Challenge, Vec<Challenge>, usize) {
        let mut ch = Challenger::new(default_koalabear_poseidon2_16());
        for block in pre {
            for &x in block.iter() { ch.observe(x); }
        }
        let alpha: Challenge = Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() });
        for &x in quot.iter() { ch.observe(x); }
        let zeta: Challenge = Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() });
        // FRI batch-combination alpha (verifier.rs:195): a sample with NO observe.
        let bca: Challenge = Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() });
        let betas: Vec<Challenge> = comms
            .iter()
            .map(|comm| {
                for &x in comm.iter() { ch.observe(x); }
                Challenge::from_basis_coefficients_fn(|_| -> Val { ch.sample() })
            })
            .collect();
        // post-beta observes (final_poly ‖ log_arities ‖ query-PoW witness), then the index draw.
        for block in final_blocks {
            for &x in block.iter() { ch.observe(x); }
        }
        let index = ch.sample_bits(index_bits);
        (alpha, zeta, bca, betas, index)
    }

    /// FIDELITY-PIN: the FRI BATCH-COMBINATION alpha and EVERY beta (AFTER the draw-count shift the
    /// batch-combo alpha introduces) match a REAL `DuplexChallenger` driven through the full ordered
    /// sequence — element-for-element, NOT a constant. Inserting the batch-combo alpha must keep every
    /// beta_r native-correct; this is the trap the PIN guards.
    #[test]
    fn fs_batch_combo_alpha_and_betas_match_native_duplex_challenger() {
        for pre_rounds in [1usize, 2] {
            for beta_rounds in [1usize, 2, 3] {
                let pre: Vec<[Val; RATE]> = (0..pre_rounds as u32).map(|r| fs_block(10 + r)).collect();
                let quot = fs_block(900);
                let comms: Vec<[Val; RATE]> =
                    (0..beta_rounds as u32).map(|r| fb_block(200 + r)).collect();
                let (_a, _z, bca_native, betas_native, _i) =
                    native_full_transcript(&pre, quot, &comms, &[fq_block(1)], 1);

                // ours: post-zeta → batch-combo alpha squeeze → betas continue from THAT.
                let post_zeta = fs_post_zeta_state(&pre, quot);
                let (bca, bca_state) = fs_squeeze_batch_combo_alpha(post_zeta);
                assert_eq!(bca, bca_native, "batch-combo alpha diverges from native challenger");
                let (betas, _) = fs_squeeze_betas(bca_state, &comms);
                assert_eq!(betas.len(), beta_rounds);
                for (r, (b, bn)) in betas.iter().zip(betas_native.iter()).enumerate() {
                    assert_eq!(
                        b, bn,
                        "beta_{r} diverges from native AFTER batch-combo insertion (pre={pre_rounds}, rounds={beta_rounds})"
                    );
                }
            }
        }
    }

    /// FIDELITY-PIN: the UNIFIED query index matches a REAL `DuplexChallenger` driven through the
    /// FULL ordered sequence (… → observe-final-poly → sample_bits). The recursion's index =
    /// draw_out masked to `log_height` bits == the native `sample_bits(log_height)`. Element-for-
    /// element, NOT a constant.
    #[test]
    fn fs_query_index_matches_native_duplex_challenger() {
        for pre_rounds in [1usize, 2] {
            for beta_rounds in [1usize, 2] {
                for final_n in [1usize, 2, 3] {
                    let h = 12usize;
                    let pre: Vec<[Val; RATE]> =
                        (0..pre_rounds as u32).map(|r| fs_block(10 + r)).collect();
                    let quot = fs_block(900);
                    let comms: Vec<[Val; RATE]> =
                        (0..beta_rounds as u32).map(|r| fb_block(200 + r)).collect();
                    let finals: Vec<[Val; RATE]> =
                        (0..final_n as u32).map(|r| fq_block(50 + r)).collect();
                    let (_a, _z, _bca, _b, index_native) =
                        native_full_transcript(&pre, quot, &comms, &finals, h);

                    // ours: post-zeta → batch-combo → betas → query-index continuation.
                    let post_zeta = fs_post_zeta_state(&pre, quot);
                    let bca_state = fs_squeeze_batch_combo_alpha(post_zeta).1;
                    let (_, post_beta) = fs_squeeze_betas(bca_state, &comms);
                    let (draw_out, _) = fs_squeeze_query_index(post_beta, &finals);
                    let index = (draw_out.as_canonical_u32() & ((1u32 << h) - 1)) as usize;
                    assert_eq!(
                        index, index_native,
                        "query index diverges from native (pre={pre_rounds}, betas={beta_rounds}, finals={final_n})"
                    );
                }
            }
        }
    }

    /// GOLDEN (positive): an honest FS-bca + FS-β + FS-qi window — batch-combo alpha squeeze, betas
    /// threaded from it, query index continued onto the post-last-beta transcript — ACCEPTS via the
    /// real `check_all_constraints`, across pre/beta/final round counts. The (1) index↔draw
    /// decomposition consumes the UNIFIED draw.
    #[test]
    fn air_accepts_full_fs_transcript() {
        for &pre_rounds in &[1usize, 2] {
            for &beta_rounds in &[1usize, 2] {
                for &final_n in &[1usize, 2] {
                    let cfg = cfg_fbq(12, 8, pre_rounds, beta_rounds, true, final_n);
                    let fs_blocks: Vec<[Val; RATE]> =
                        (0..(pre_rounds + 1) as u32).map(|r| fs_block(20 + r)).collect();
                    let comms: Vec<[Val; RATE]> =
                        (0..beta_rounds as u32).map(|r| fb_block(300 + r)).collect();
                    let finals: Vec<[Val; RATE]> =
                        (0..final_n as u32).map(|r| fq_block(70 + r)).collect();
                    let row = fb_window_full(&cfg, &fs_blocks, &comms, &finals, ext_multi(66));
                    let m = RowMajorMatrix::new(row, n_cols(&cfg));
                    let report = check_all_constraints(&air(cfg), &m, &[], None);
                    assert!(
                        report.is_ok(),
                        "honest full-FS window rejected (pre={pre_rounds}, betas={beta_rounds}, finals={final_n}): {} failures",
                        report.failures.len()
                    );
                }
            }
        }
    }

    /// (FS-bca) FORGERY — the batch-combo alpha is the SQUEEZE-ONLY permute of the post-zeta state:
    /// corrupt its perm input capacity lane (must be carried UNCHANGED from the post-zeta state, NO
    /// length tag) ⇒ rejected. Proves the no-observe squeeze is genuinely modeled.
    #[test]
    fn air_rejects_batch_combo_broken_squeeze_input() {
        let cfg = cfg_fbq(12, 8, 2, 2, true, 1);
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(20 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(300 + r)).collect();
        let finals = vec![fq_block(70)];
        let mut row = fb_window_full(&cfg, &fs_blocks, &comms, &finals, ext_multi(66));
        let bb = bca_base(&cfg);
        // corrupt the batch-combo permute's input capacity lane RATE (must equal post-zeta slot RATE,
        // i.e. NO +RATE length tag — a squeeze, not an absorb).
        row[bb + BCA_PERM + inlined_perm::C_INPUTS + RATE] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a batch-combo permute NOT squeezing the post-zeta state unchanged was accepted"
        );
    }

    /// (FS-bca) FORGERY — OMITTING/REORDERING the batch-combo alpha breaks the beta PIN. We build an
    /// HONEST window WITHOUT the batch-combo alpha (betas continue straight from post-zeta), then check
    /// it against the WITH-batch-combo AIR: the betas differ (the draw-count shift), so the beta cells
    /// the no-bca window carries do NOT match the bca AIR's squeezed betas ⇒ rejected. Conversely a
    /// with-bca window's betas != the no-bca squeeze. This is the draw-count trap, proven by recovery.
    #[test]
    fn air_rejects_omitted_batch_combo_alpha() {
        let pre_rounds = 2usize;
        let beta_rounds = 2usize;
        let fs_blocks: Vec<[Val; RATE]> =
            (0..(pre_rounds + 1) as u32).map(|r| fs_block(20 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..beta_rounds as u32).map(|r| fb_block(300 + r)).collect();
        let finals = vec![fq_block(70)];

        // The betas WITH vs WITHOUT the batch-combo alpha differ (the shift moves every beta).
        let post_zeta = fs_post_zeta_state(&fs_blocks[..pre_rounds], fs_blocks[pre_rounds]);
        let (betas_no_bca, _) = fs_squeeze_betas(post_zeta, &comms);
        let bca_state = fs_squeeze_batch_combo_alpha(post_zeta).1;
        let (betas_bca, _) = fs_squeeze_betas(bca_state, &comms);
        for r in 0..beta_rounds {
            assert_ne!(
                betas_no_bca[r], betas_bca[r],
                "omitting the batch-combo alpha did NOT move beta_{r} — draw-count shift absent"
            );
        }

        // Build a window with the batch-combo DISABLED (betas continue straight from post-zeta), then
        // hand it to the batch-combo-ENABLED AIR. The carried beta cells are the no-bca betas, but the
        // bca AIR squeezes the WITH-bca betas from the batch-combo permute ⇒ mismatch ⇒ rejected.
        let cfg_no = cfg_fbq(12, 8, pre_rounds, beta_rounds, false, 0);
        let cfg_yes = cfg_fbq(12, 8, pre_rounds, beta_rounds, true, 0);
        // n_cols differ (the bca block adds columns), so re-pack honestly under cfg_yes then overwrite
        // the FS-β beta cells + descent consumers with the (stale) no-bca betas and re-fold — the only
        // rejector is the squeezed-beta binding (beta cell == bca permute squeeze).
        let _ = cfg_no;
        let mut row = fb_window_full(&cfg_yes, &fs_blocks, &comms, &[], ext_multi(66));
        // overwrite round-0 consumer + FS-β beta cell with the stale no-bca beta_0 and re-fold round 0.
        let e0_0 = Challenge::from_basis_coefficients_slice(&row[C_E0..C_E0 + D]).unwrap();
        let e1_0 = Challenge::from_basis_coefficients_slice(&row[C_E1..C_E1 + D]).unwrap();
        let folded0 = fold_pair(e0_0, e1_0, betas_no_bca[0], row[C_S]);
        row[C_BETA..C_BETA + D].copy_from_slice(&limbs(betas_no_bca[0]));
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded0));
        let rb0 = fb_round_base(&cfg_yes, 0);
        row[rb0 + FB_BETA..rb0 + FB_BETA + D].copy_from_slice(&limbs(betas_no_bca[0]));
        let m = RowMajorMatrix::new(row, n_cols(&cfg_yes));
        assert!(
            !check_all_constraints(&air(cfg_yes), &m, &[], None).is_ok(),
            "a window with the batch-combo alpha OMITTED (stale no-bca betas) was accepted by the bca AIR"
        );
    }

    /// (FS-qi) FORGERY — changing the FINAL POLYNOMIAL (a post-beta observe block) changes the query
    /// index. We re-pack HONESTLY for a tampered final block (so the squeeze is internally consistent)
    /// but KEEP the OLD index decomposition bits/index — the unified draw_out moved, so the bits no
    /// longer decompose it ⇒ rejected. Proves the index is bound to the final-poly transcript.
    #[test]
    fn air_rejects_changed_final_poly_keeping_stale_index() {
        let cfg = cfg_fbq(12, 8, 2, 2, true, 2);
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(20 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(300 + r)).collect();
        let finals: Vec<[Val; RATE]> = (0..2u32).map(|r| fq_block(70 + r)).collect();

        // honest accepts.
        let honest = fb_window_full(&cfg, &fs_blocks, &comms, &finals, ext_multi(66));
        assert!(check_all_constraints(&air(cfg), &RowMajorMatrix::new(honest.clone(), n_cols(&cfg)), &[], None).is_ok());

        // changing final block 0 moves the draw_out (and hence the index).
        let mut tampered = finals.clone();
        tampered[0][3] += Val::ONE;
        let post_zeta = fs_post_zeta_state(&fs_blocks[..2], fs_blocks[2]);
        let bca_state = fs_squeeze_batch_combo_alpha(post_zeta).1;
        let (_, post_beta) = fs_squeeze_betas(bca_state, &comms);
        let (old_draw, _) = fs_squeeze_query_index(post_beta, &finals);
        let (new_draw, _) = fs_squeeze_query_index(post_beta, &tampered);
        assert_ne!(old_draw, new_draw, "final-poly change did not move the query draw");

        // re-pack honestly for the tampered final block, then OVERWRITE C_DRAW_OUT/C_BITS/C_INDEX back
        // to the OLD (stale) draw — the FS-qi binding (draw_out == last squeeze pop) now rejects.
        let mut row = fb_window_full(&cfg, &fs_blocks, &comms, &tampered, ext_multi(66));
        let old_canon = old_draw.as_canonical_u32();
        row[C_DRAW_OUT] = old_draw;
        for i in 0..FIELD_BITS {
            row[C_BITS + i] = Val::from_u32((old_canon >> i) & 1);
        }
        row[C_INDEX] = Val::from_u32(old_canon & ((1u32 << cfg.log_height) - 1));
        // re-fold round 0 etc. is unnecessary: the rejector is draw_out != post-beta squeeze pop, which
        // does not depend on the fold. (The fold uses the index/s0 which we left stale-consistent.)
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a changed final polynomial with a stale query index was accepted — index not transcript-bound"
        );
    }

    /// (FS-qi) FORGERY — a query index from the OLD SEPARATE chain (an arbitrary draw_out NOT equal to
    /// the unified post-beta squeeze) is rejected. We overwrite the unified draw with a fresh,
    /// transcript-unrelated value (and its honest bit decomposition) ⇒ the FS-qi binding rejects.
    /// Proves the index MUST come from the one continued transcript, not a separate squeeze.
    #[test]
    fn air_rejects_query_index_from_separate_chain() {
        let cfg = cfg_fbq(12, 8, 1, 1, true, 1);
        let fs_blocks: Vec<[Val; RATE]> = (0..2u32).map(|r| fs_block(20 + r)).collect();
        let comms = vec![fb_block(300)];
        let finals = vec![fq_block(70)];
        let mut row = fb_window_full(&cfg, &fs_blocks, &comms, &finals, ext_multi(66));
        // a draw from a DIFFERENT (separate) chain: just a fresh value with a consistent decomposition.
        let forged = Val::from_u32(0x0BADF00D);
        let fc = forged.as_canonical_u32();
        row[C_DRAW_OUT] = forged;
        for i in 0..FIELD_BITS {
            row[C_BITS + i] = Val::from_u32((fc >> i) & 1);
        }
        row[C_INDEX] = Val::from_u32(fc & ((1u32 << cfg.log_height) - 1));
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a query index from a separate chain (!= the unified post-beta squeeze) was accepted"
        );
    }

    /// (FS-qi) FORGERY — a tampered FS-qi perm interior cell ⇒ the inlined Poseidon2 round constraints
    /// reject (the index squeeze permute is fully constrained, not just the input copy).
    #[test]
    fn air_rejects_fs_qi_tampered_perm() {
        let cfg = cfg_fbq(12, 8, 1, 1, true, 1);
        let fs_blocks: Vec<[Val; RATE]> = (0..2u32).map(|r| fs_block(20 + r)).collect();
        let comms = vec![fb_block(300)];
        let finals = vec![fq_block(70)];
        let mut row = fb_window_full(&cfg, &fs_blocks, &comms, &finals, ext_multi(66));
        let rb = fq_round_base(&cfg, 0);
        row[rb + FQ_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (FS-qi) PUBLIC-TRANSCRIPT BINDING: with `fs_final_blocks_public`, the post-beta observe blocks
    /// (final_poly ‖ log_arities ‖ query-PoW witness) are pinned to the public-values vector. Honest
    /// accepts; a public value inconsistent with the observed block rejects. Ties the query index to
    /// the ACTUAL final polynomial + grinding witness.
    #[test]
    fn air_fs_qi_bound_to_public_transcript() {
        let final_n = 2usize;
        let cfg = RecursiveConfig {
            num_rounds: 2,
            deep_log_degree: Some(8),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: 2,
            fs_beta_rounds: 2,
            fs_batch_combo_alpha: true,
            fs_final_blocks: final_n,
            fs_final_blocks_public: true,
            ..RecursiveConfig::single_window(12)
        };
        let fs_blocks: Vec<[Val; RATE]> = (0..3u32).map(|r| fs_block(20 + r)).collect();
        let comms: Vec<[Val; RATE]> = (0..2u32).map(|r| fb_block(300 + r)).collect();
        let finals: Vec<[Val; RATE]> = (0..final_n as u32).map(|r| fq_block(70 + r)).collect();
        let row = fb_window_full(&cfg, &fs_blocks, &comms, &finals, ext_multi(66));
        let pubs: Vec<Val> = finals.iter().flatten().copied().collect();
        assert_eq!(pubs.len(), final_n * RATE);
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(row.clone(), n_cols(&cfg)), &pubs, None).is_ok(),
            "honest FS-qi public-transcript binding rejected"
        );
        let mut bad_pubs = pubs.clone();
        bad_pubs[RATE + 2] += Val::ONE;
        assert!(
            !check_all_constraints(&air(cfg), &RowMajorMatrix::new(row, n_cols(&cfg)), &bad_pubs, None).is_ok(),
            "a public transcript inconsistent with the FS-qi observed block was accepted"
        );
    }

    // ---------------------------------------------------------------------------------
    // (LO) per-round FRI commit-phase LAYER open COMPOSED into the (c) descent + (FP) final-poly.
    // The descent's fold now consumes the AUTHENTICATED sibling: round_e1(r) is bound to a sibling
    // Merkle-authenticated against round r's layer commitment at the FS-shifted node. After the last
    // round the fully-folded value is bound to the committed final polynomial.
    // ---------------------------------------------------------------------------------

    /// A reference binary Merkle tree over the deployed compression (mirror of fri_layer_air::RefTree).
    struct LoRefTree {
        levels: Vec<Vec<Digest>>,
    }
    impl LoRefTree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                let next: Vec<Digest> =
                    leaves.chunks_exact(2).map(|p| c.compress([p[0], p[1]])).collect();
                levels.push(next.clone());
                leaves = next;
            }
            Self { levels }
        }
        fn root(&self) -> Digest {
            self.levels.last().unwrap()[0]
        }
        fn auth_path(&self, index: usize) -> Vec<Digest> {
            let mut sibs = Vec::new();
            let mut idx = index;
            for level in &self.levels[..self.levels.len() - 1] {
                sibs.push(level[idx ^ 1]);
                idx >>= 1;
            }
            sibs
        }
    }

    fn lo_salt(seed: u32) -> [Val; SALT_ELEMS] {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1)))
    }

    /// Config: an `lo_rounds`-round descent whose every round's sibling is layer-authenticated
    /// (`layer_open_rounds == num_rounds`), with `levels`-deep per-round layer trees, plus the (FP)
    /// final-poly check on the fully-folded value.
    fn cfg_lo(h: usize, rounds: usize, levels: usize, with_fp: bool) -> RecursiveConfig {
        RecursiveConfig {
            num_rounds: rounds,
            layer_open_rounds: rounds,
            layer_open_levels: levels,
            final_poly_len: if with_fp { 1 } else { 0 },
            ..RecursiveConfig::single_window(h)
        }
    }

    /// Build an honest layer-open descent window: derive the descent (e0/e1/folded per round) from a
    /// real folded polynomial, then for each round build a REAL reference layer tree whose
    /// `(FS_index >> (r+1))`-th leaf is `MyHash(flatten(e_fold,e_sib) ‖ salt)`, take the real auth
    /// path + root as the round's layer commitment, and (optionally) a constant final poly == folded.
    /// Returns the packed window and the per-round (layer_commit, e_fold, e_sib, start_index).
    #[allow(clippy::type_complexity)]
    fn lo_window(
        cfg: &RecursiveConfig,
        index_draw: Val,
    ) -> (Vec<Val>, Vec<([Val; DIGEST_ELEMS], Challenge, Challenge, usize)>) {
        let h0 = cfg.log_height;
        let rounds = cfg.rounds();
        let levels = cfg.layer_open_levels;
        let coeffs: Vec<Challenge> = (0..(1u32 << rounds)).map(|k| ext_multi(k + 1)).collect();
        let betas: Vec<Challenge> = (0..rounds as u32).map(|r| ext_multi(100 + r)).collect();
        let index = (index_draw.as_canonical_u32() as usize) & ((1 << h0) - 1);
        let (e0_0, e1_0, beta0, descent) = honest_descent(&coeffs, &betas, h0, index);

        // recompute per-round (e_fold, e_sib) exactly as pack_window's descent does.
        let mut round_pairs: Vec<(Challenge, Challenge)> = vec![(e0_0, e1_0)];
        {
            let s0 = fold_point(h0, index);
            let mut s_prev = s0;
            let mut folded_prev = fold_pair(e0_0, e1_0, beta0, s0);
            for r in 1..rounds {
                let e1_r = descent[r - 1].0;
                let beta_r = descent[r - 1].1;
                let e0_r = folded_prev;
                let s_r = s_prev * s_prev;
                round_pairs.push((e0_r, e1_r));
                folded_prev = fold_pair(e0_r, e1_r, beta_r, s_r);
                s_prev = s_r;
            }
        }
        let folded_last = {
            // last round's folded value (= round_folded(rounds-1)).
            let s0 = fold_point(h0, index);
            let mut s_prev = s0;
            let mut folded = fold_pair(e0_0, e1_0, beta0, s0);
            for r in 1..rounds {
                let (e0_r, e1_r) = round_pairs[r];
                let s_r = s_prev * s_prev;
                folded = fold_pair(e0_r, e1_r, descent[r - 1].1, s_r);
                s_prev = s_r;
            }
            folded
        };

        // the path consumes only `levels` node-selector bits, so a `levels`-deep tree authenticates the
        // node at `(start_index >> 1) & ((1<<levels)-1)` — exactly the selectors the AIR reads from the
        // FS bits (bit r+1+level).
        let n_nodes = 1usize << levels;
        let mut metas: Vec<([Val; DIGEST_ELEMS], Challenge, Challenge, usize)> = Vec::new();
        let mut owned_sibs: Vec<Vec<[Val; DIGEST_ELEMS]>> = Vec::new();
        let mut owned_salts: Vec<[Val; SALT_ELEMS]> = Vec::new();
        for r in 0..cfg.layer_open_rounds {
            let (e_fold, e_sib) = round_pairs[r];
            let start_index = index >> r;
            let parity = (start_index & 1) == 1;
            let node = (start_index >> 1) & (n_nodes - 1);
            let salt = lo_salt(0x70 + r as u32);
            let leaf = layer_leaf_digest(e_fold, e_sib, parity, &salt);
            let leaves: Vec<Digest> = (0..n_nodes)
                .map(|j| {
                    if j == node {
                        leaf
                    } else {
                        core::array::from_fn(|i| Val::from_u32((j as u32 + 1) * 7919 + i as u32 + r as u32))
                    }
                })
                .collect();
            let tree = LoRefTree::new(leaves);
            owned_sibs.push(tree.auth_path(node));
            owned_salts.push(salt);
            metas.push((tree.root(), e_fold, e_sib, start_index));
        }
        let layer_opens: Vec<LayerOpenInput> = (0..cfg.layer_open_rounds)
            .map(|r| LayerOpenInput {
                layer_commit: metas[r].0,
                salt: owned_salts[r],
                merkle_siblings: &owned_sibs[r],
            })
            .collect();

        let (fp_x, fp_coeffs): (Val, Vec<Challenge>) = if cfg.final_poly_len > 0 {
            // constant final poly == folded_last (eval is the constant for any x).
            (Val::from_u32(0x9E3779B1), vec![folded_last])
        } else {
            (Val::ZERO, Vec::new())
        };

        let inp = WindowInputs {
            draw_out: index_draw,
            e0: e0_0,
            e1: e1_0,
            beta: beta0,
            descent: &descent,
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
            deep_alpha: Challenge::ZERO,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[],
            fs_final_blocks: &[],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &layer_opens,
            final_poly_x: fp_x,
            final_poly_coeffs: &fp_coeffs,
            leaf_row: &[],
            leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        (pack_window(cfg, &inp), metas)
    }

    /// GOLDEN: an honest end-to-end query — every descent round's sibling layer-authenticated against
    /// the real round commitment at the FS-shifted node, the fold consuming it, the fully-folded value
    /// equal to the final poly — ACCEPTS with zero residual.
    #[test]
    fn air_accepts_honest_layer_open_descent() {
        let h = 12usize;
        for rounds in [1usize, 2, 3] {
            for levels in [2usize, 3] {
                let cfg = cfg_lo(h, rounds, levels, true);
                let (row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(
                    report.is_ok(),
                    "honest layer-open descent rejected (rounds={rounds}, levels={levels}): {} failures",
                    report.failures.len()
                );
            }
        }
    }

    /// FIDELITY-PIN (round 0, real p3 MMCS): build a real arity-2 FRI commit-phase layer, commit a real
    /// `MerkleTreeMmcs`, confirm the REAL `Mmcs::verify_batch` accepts the honest opening (anchoring the
    /// deployed leaf MyHash + node MyCompress), then drive the COMPOSED round-0 layer open with the real
    /// auth path/root and confirm the in-window reconstructed root matches the reference tree.
    #[test]
    fn fidelity_real_mmcs_round0_layer_open_in_descent() {
        use descender_account_stark::Perm;
        use p3_commit::Mmcs;
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_matrix::Dimensions;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

        type LeafHash = PaddingFreeSponge<Perm, 16, 8, DIGEST_ELEMS>;
        type NodeCompress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, 16>;
        let perm: Perm = default_koalabear_poseidon2_16();
        let mmcs: MerkleTreeMmcs<Val, Val, LeafHash, NodeCompress, 2, DIGEST_ELEMS> =
            MerkleTreeMmcs::new(LeafHash::new(perm.clone()), NodeCompress::new(perm.clone()), 0);
        let node_levels = 3usize;
        let n_nodes = 1usize << node_levels;
        let width = 2 * D;
        let vals: Vec<Val> = (0..(n_nodes * width) as u32)
            .map(|x| Val::from_u32(x.wrapping_mul(2_654_435_761)))
            .collect();
        let matrix = RowMajorMatrix::new(vals.clone(), width);
        let (commit, prover_data) = mmcs.commit(vec![matrix]);
        // the real MMCS verifies an honest opening (anchors the deployed leaf + node hashing).
        let opening = mmcs.open_batch(2, &prover_data);
        let (opened_values, opening_proof) = opening.unpack();
        mmcs.verify_batch(
            &commit,
            &[Dimensions { width, height: n_nodes }],
            2,
            p3_commit::BatchOpeningRef::new(&opened_values, &opening_proof),
        )
        .expect("real Mmcs::verify_batch must accept the honest layer opening");

        // now drive the COMPOSED round-0 open over a salt-0 reference tree (mirrors leaf_hash_digest).
        let leaves: Vec<Digest> = (0..n_nodes)
            .map(|j| {
                let row: Vec<Val> = (0..width).map(|c| vals[j * width + c]).collect();
                let mut s = row;
                s.extend_from_slice(&[Val::ZERO; SALT_ELEMS]);
                leaf_hash_digest(&s)
            })
            .collect();
        let tree = LoRefTree::new(leaves);
        // pick FS index so round-0 start_index hits node 2 (start_index = index, node = index>>1).
        // index = 4 or 5 → node 2.
        let h = node_levels + 1; // FS index needs >= node_levels+1 bits.
        let cfg = RecursiveConfig {
            num_rounds: 1,
            layer_open_rounds: 1,
            layer_open_levels: node_levels,
            ..RecursiveConfig::single_window(h)
        };
        let index = 4usize;
        let parity = (index & 1) == 1;
        let node = index >> 1;
        let pair: Vec<Val> = (0..width).map(|c| vals[node * width + c]).collect();
        let evals0 = Challenge::from_basis_coefficients_slice(&pair[..D]).unwrap();
        let evals1 = Challenge::from_basis_coefficients_slice(&pair[D..]).unwrap();
        let (e_fold, e_sib) = if parity { (evals1, evals0) } else { (evals0, evals1) };
        let beta0 = ext_multi(7);
        let sibs = tree.auth_path(node);
        let layer_opens = [LayerOpenInput {
            layer_commit: tree.root(),
            salt: [Val::ZERO; SALT_ELEMS],
            merkle_siblings: &sibs,
        }];
        let inp = WindowInputs {
            draw_out: Val::from_u32(index as u32),
            e0: e_fold,
            e1: e_sib,
            beta: beta0,
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
            deep_alpha: Challenge::ZERO,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[],
            fs_final_blocks: &[],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &layer_opens,
            final_poly_x: Val::ZERO,
            final_poly_coeffs: &[],
            leaf_row: &[],
            leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        let row = pack_window(&cfg, &inp);
        // zero-residual reconstructed root == reference root.
        let llast = lo_level_base(&cfg, 0, node_levels - 1);
        let recon: [Val; DIGEST_ELEMS] =
            core::array::from_fn(|i| row[llast + LO_ML_PERM + C_OUTPUT + i]);
        assert_eq!(recon, tree.root(), "in-window round-0 layer root != real reference root");
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let report = check_all_constraints(&air(cfg), &m, &[], None);
        assert!(report.is_ok(), "composed round-0 layer open rejected: {} failures", report.failures.len());
    }

    /// LOAD-BEARING (the gap closed): a descent whose sibling != the AUTHENTICATED layer-open output is
    /// REJECTED. Tamper round 0's descent sibling cell (C_E1) AND the matching flattened pair lane so
    /// the pair-placement constraint still passes — the leaf hash then no longer authenticates to the
    /// committed root. If the composition were a parallel unbound decoration this would pass.
    #[test]
    fn air_rejects_descent_sibling_not_authenticated() {
        let cfg = cfg_lo(12, 2, 3, false);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        // round 0, FS index 0xABCDE is even → parity 0 → evals[1] == e_sib.
        let rb = lo_round_base(&cfg, 0);
        row[C_E1 + 1] += Val::ONE; // descent sibling
        row[rb + LO_EVAL1 + 1] += Val::ONE; // matching flattened lane (placement still holds)
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a descent sibling != the authenticated layer-open output was accepted — composition not load-bearing"
        );
    }

    /// LOAD-BEARING (placement guard): tampering ONLY the descent sibling cell (leaving the flattened
    /// pair stale) is caught even earlier by the pair-placement constraint — the descent's round_e1(r)
    /// IS the value hashed into the authenticated leaf.
    #[test]
    fn air_rejects_descent_sibling_placement_guard() {
        let cfg = cfg_lo(12, 2, 3, false);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        row[C_E1 + 1] += Val::ONE; // only the descent sibling; pair lane left stale
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a tampered descent sibling (placement) was accepted"
        );
    }

    /// FORGERY — WRONG LAYER COMMITMENT: layer_commit_r != the real round-r commitment ⇒ root binding
    /// rejects.
    #[test]
    fn air_rejects_wrong_layer_commit() {
        let cfg = cfg_lo(12, 2, 3, false);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        let rb = lo_round_base(&cfg, 1);
        row[rb + 2] += Val::ONE; // tamper round-1 layer commitment
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a wrong layer commitment was accepted"
        );
    }

    /// FORGERY — CROSS-ROUND CONFUSION (layer commitment swap): use round 1's layer commitment in
    /// round 0's slot. Round 0's path authenticates to round-0's root, not round-1's ⇒ rejected. (The
    /// layer commitment + sibling + node used must be the SAME round's; cross-round = soundness hole.)
    #[test]
    fn air_rejects_cross_round_layer_commit_swap() {
        let cfg = cfg_lo(12, 2, 3, false);
        let (mut row, metas) = lo_window(&cfg, Val::from_u32(0xABCDE));
        // overwrite round-0's layer commitment with round-1's real commitment.
        let rb0 = lo_round_base(&cfg, 0);
        let r1_commit = metas[1].0;
        for i in 0..DIGEST_ELEMS {
            row[rb0 + i] = r1_commit[i];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "round 0 authenticating to round 1's layer commitment was accepted — cross-round confusion"
        );
    }

    /// FORGERY — CROSS-ROUND CONFUSION (sibling + commitment swap): take round 1's ENTIRE authenticated
    /// sibling open (commitment, salt, path, evals) and present it in round 0's slot, also binding the
    /// descent's round-0 sibling to round 1's sibling. The node selectors of round 0 are read from FS
    /// bits 1.. (round 0's folded-node index) but round 1's path was built for FS bits 2.. (round 1's
    /// node) ⇒ the path no longer reconstructs round 1's root at round 0's node ⇒ rejected.
    #[test]
    fn air_rejects_cross_round_sibling_swap() {
        let cfg = cfg_lo(12, 2, 3, false);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        let rb0 = lo_round_base(&cfg, 0);
        let rb1 = lo_round_base(&cfg, 1);
        let sz = lo_round_size(cfg.layer_open_levels);
        // copy round-1's entire layer-open sub-block into round-0's slot.
        let r1_block: Vec<Val> = row[rb1..rb1 + sz].to_vec();
        row[rb0..rb0 + sz].copy_from_slice(&r1_block);
        // and bind round-0's descent sibling (C_E1) to round-1's sibling so placement could pass.
        let r1_esib: Vec<Val> = {
            let b = extra_round_base(&cfg, 1);
            row[b + D..b + 2 * D].to_vec()
        };
        row[C_E1..C_E1 + D].copy_from_slice(&r1_esib);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "round 1's full layer open presented at round 0 was accepted — cross-round confusion"
        );
    }

    /// FORGERY — WRONG PER-ROUND INDEX SHIFT: corrupt the bus index so round r's folded-node bits no
    /// longer match the path the prover built (built for the honest FS index). The path then authenticates
    /// to a different root than the committed one ⇒ rejected. Confirms the per-round node selector is
    /// bound to FS bit (r+1+level), not free.
    #[test]
    fn air_rejects_wrong_per_round_index_shift() {
        let cfg = cfg_lo(12, 2, 3, false);
        // honest window at one index, then flip a high bit that feeds round-1's node selector but keep
        // the rest honest — the index/draw decomposition AND the layer path both break.
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        // flip FS bit 3 (feeds round 0 level 2 selector = bit 3, and round 1 level 1 selector = bit 3).
        let bitpos = 3usize;
        row[C_BITS + bitpos] = if row[C_BITS + bitpos] == Val::ONE { Val::ZERO } else { Val::ONE };
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a flipped FS index bit (wrong per-round node) was accepted — index binding vacuous"
        );
    }

    /// FORGERY — TAMPERED LAYER LEAF PERM: corrupt an interior cell of a round's leaf-hash permutation
    /// so the derived leaf digest is inconsistent ⇒ the eval_perm constraint (or the leaf→running
    /// binding) rejects. Confirms the MyHash leaf is genuinely re-derived in-window.
    #[test]
    fn air_rejects_tampered_layer_leaf_perm() {
        let cfg = cfg_lo(12, 2, 2, false);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        let pb = lo_round_base(&cfg, 0) + lo_leaf_perm_base(0);
        row[pb + inlined_perm::C_OUTPUT + 1] += Val::ONE; // tamper a perm output cell
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a tampered layer leaf perm was accepted"
        );
    }

    /// (FP) FORGERY — FOLDED != FINAL_POLY(x): perturb the fully-folded value's final-poly coefficient
    /// so folded != final_poly(x) ⇒ rejected by the in-descent final-poly equality.
    #[test]
    fn air_rejects_folded_ne_final_poly() {
        let cfg = cfg_lo(12, 2, 3, true);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        let fb = fp_base(&cfg);
        row[fb + fp_coeff_base(0) + 2] += Val::ONE; // tamper the (constant) final poly coeff
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a folded value != final_poly(x) was accepted — (FP) not enforced in the descent"
        );
    }

    /// (FP) FORGERY — TAMPERED FOLDED VALUE: perturb the descent's last-round folded cell so it no
    /// longer equals final_poly(x). Rejected by (FP) (and, since folded feeds nothing downstream here,
    /// (FP) is the load-bearing guard). Confirms the final-poly check binds the DESCENT's folded value.
    #[test]
    fn air_rejects_tampered_descent_folded_vs_final_poly() {
        let cfg = cfg_lo(12, 2, 3, true);
        let (mut row, _) = lo_window(&cfg, Val::from_u32(0xABCDE));
        // round 1 is the last round; its folded cell.
        let b1 = extra_round_base(&cfg, 1);
        row[b1 + 3 * D + 1 + 2] += Val::ONE; // tamper folded — breaks (★) at round 1 AND (FP)
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a tampered descent folded value was accepted"
        );
    }

    // ---------------------------------------------------------------------------------
    // (LO≡FS-β) UNIFY the per-round FRI-layer AUTHENTICATION commitment (`layer_commit_r`, the
    // root the round-r sibling opens against) with the FS-OBSERVED commit-phase commitment
    // (`commit_phase_commit_r`, observed into the sponge to squeeze beta_r). In real p3-fri there is
    // ONE commitment per commit-phase round (p3-fri-0.6.1 verifier.rs:300-310 observe→beta_r vs
    // :374-377/:535 the SAME commit is the verify_batch root). The (LO≡FS-β) equality residual ties
    // them: changing round r's observed commitment moves beta_r (FS squeeze) AND breaks the round-r
    // sibling authentication — one digest, both effects.
    // ---------------------------------------------------------------------------------

    /// Config: a `rounds`-round descent that BOTH (a) FS-squeezes the commit-phase betas
    /// (`fs_beta_rounds == rounds`, continuing the post-zeta transcript) AND (b) layer-authenticates
    /// each round's sibling (`layer_open_rounds == rounds`, `levels`-deep trees). This is the only
    /// config where BOTH the FS-β block and the LO block exist for every round, so the (LO≡FS-β)
    /// residual `layer_commit_r == commit_phase_commit_r` is exercised.
    fn cfg_lo_fb(h: usize, rounds: usize, levels: usize, pre_rounds: usize) -> RecursiveConfig {
        RecursiveConfig {
            num_rounds: rounds,
            deep_log_degree: Some(8),
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: pre_rounds,
            fs_beta_rounds: rounds,
            layer_open_rounds: rounds,
            layer_open_levels: levels,
            ..RecursiveConfig::single_window(h)
        }
    }

    /// Build an honest UNIFIED window: each round's single digest is BOTH the FS-β observed
    /// `commit_phase_commit_r` (→ beta_r) AND the round-r layer authentication root (`layer_commit_r`).
    /// This mirrors real p3-fri: the MMCS root of the round-r layer is committed, observed to squeeze
    /// beta_r, and opened against. Solved as the genuine sequential fixed point (beta_r depends only on
    /// comms[0..=r]; the round-r tree's target leaf depends on round_pairs[r] which uses beta_{r-1}):
    ///   round 0: pairs[0] = (e0,e1) are direct inputs → build tree_0 → root_0 = comm_0 → squeeze beta_0
    ///   round r: e_fold_r = fold(e_fold_{r-1}, e_sib_{r-1}, beta_{r-1}, s_{r-1}); e_sib_r is a chosen
    ///            input → build tree_r → root_r = comm_r → squeeze beta_r
    /// Returns the packed window + the per-round (unified_digest, e_fold, e_sib, start_index).
    #[allow(clippy::type_complexity)]
    fn lo_fb_window(
        cfg: &RecursiveConfig,
        fs_prefix: &[[Val; RATE]],
        index_draw: Val,
    ) -> (Vec<Val>, Vec<([Val; DIGEST_ELEMS], Challenge, Challenge, usize)>) {
        let h0 = cfg.log_height;
        let rounds = cfg.rounds();
        let levels = cfg.layer_open_levels;
        let pre = cfg.fs_pre_alpha_rounds;
        assert_eq!(fs_prefix.len(), pre + 1, "fs_prefix = pre-alpha rounds + quotient observe");
        // beta-chain start = post-zeta state (no batch-combo alpha in this fixture).
        let beta_start = fs_post_zeta_state(&fs_prefix[..pre], fs_prefix[pre]);

        let index = (index_draw.as_canonical_u32() as usize) & ((1 << h0) - 1);
        let n_nodes = 1usize << levels;

        // chosen sibling inputs for rounds 1..rounds (e_sib_r). Round 0's pair is (e0, e1).
        let e0_0 = ext_multi(0x5AB0);
        let e1_0 = ext_multi(0x5AB1);
        let e_sibs: Vec<Challenge> = (1..rounds).map(|r| ext_multi(0x6000 + r as u32)).collect();

        // sequential fixed point: round-by-round build tree → root = comm → squeeze beta → next e_fold.
        let mut comms: Vec<[Val; RATE]> = Vec::with_capacity(rounds);
        let mut betas: Vec<Challenge> = Vec::with_capacity(rounds);
        let mut round_pairs: Vec<(Challenge, Challenge)> = Vec::with_capacity(rounds);
        let mut metas: Vec<([Val; DIGEST_ELEMS], Challenge, Challenge, usize)> = Vec::new();
        let mut owned_sibs: Vec<Vec<[Val; DIGEST_ELEMS]>> = Vec::new();
        let mut owned_salts: Vec<[Val; SALT_ELEMS]> = Vec::new();

        let mut s_prev = fold_point(h0, index);
        let mut e_fold = e0_0;
        for r in 0..rounds {
            let e_sib = if r == 0 { e1_0 } else { e_sibs[r - 1] };
            let start_index = index >> r;
            let parity = (start_index & 1) == 1;
            let node = (start_index >> 1) & (n_nodes - 1);
            let salt = lo_salt(0xB0 + r as u32);
            let leaf = layer_leaf_digest(e_fold, e_sib, parity, &salt);
            let leaves: Vec<Digest> = (0..n_nodes)
                .map(|j| {
                    if j == node {
                        leaf
                    } else {
                        core::array::from_fn(|i| {
                            Val::from_u32((j as u32 + 1) * 7919 + i as u32 + r as u32)
                        })
                    }
                })
                .collect();
            let tree = LoRefTree::new(leaves);
            let root = tree.root();
            // the round-r FRI commitment: root is observed (→ beta_r) AND authenticated against.
            comms.push(root); // Digest is [Val; DIGEST_ELEMS] == [Val; RATE]
            owned_sibs.push(tree.auth_path(node));
            owned_salts.push(salt);
            round_pairs.push((e_fold, e_sib));
            metas.push((root, e_fold, e_sib, start_index));
            // squeeze beta_r by observing comms[0..=r] from the post-zeta start (matches pack_window).
            let (bs, _) = fs_squeeze_betas(beta_start, &comms);
            let beta_r = bs[r];
            betas.push(beta_r);
            // next round's e_fold = this round's fold output; s_r = s_{r-1}^2 (point-square).
            let s_r = s_prev;
            e_fold = fold_pair(e_fold, e_sib, beta_r, s_r);
            s_prev = s_r * s_r;
        }

        // descent inputs: (e_sib_r, dummy beta) for rounds 1..rounds (betas overridden by FS-β).
        let descent: Vec<(Challenge, Challenge)> =
            (1..rounds).map(|r| (round_pairs[r].1, Challenge::ZERO)).collect();
        let layer_opens: Vec<LayerOpenInput> = (0..rounds)
            .map(|r| LayerOpenInput {
                layer_commit: metas[r].0,
                salt: owned_salts[r],
                merkle_siblings: &owned_sibs[r],
            })
            .collect();

        let inp = WindowInputs {
            draw_out: index_draw,
            e0: e0_0,
            e1: e1_0,
            beta: Challenge::ZERO, // overridden by FS-β round 0
            descent: &descent,
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO,
            deep_q: ext_multi(0x9001),
            deep_alpha: Challenge::ZERO,
            fs_blocks: fs_prefix,
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &comms,
            fs_final_blocks: &[],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &layer_opens,
            final_poly_x: Val::ZERO,
            final_poly_coeffs: &[],
            leaf_row: &[],
            leaf_salt: [Val::ZERO; SALT_ELEMS],
        };
        (pack_window(cfg, &inp), metas)
    }

    /// GOLDEN (unified): an honest window where each round's single digest is BOTH the FS-β observed
    /// commitment (→ beta_r) AND the layer authentication root — ACCEPTS with zero residual. Confirms
    /// the (LO≡FS-β) equality residual holds on an honest, fixed-point-consistent transcript across
    /// round/level counts. Also asserts the fixture is genuinely consistent: for every round r, the LO
    /// `layer_commit_r` cells equal the FS-β `commit_phase_commit_r` block cells (one digest, both
    /// roles — the real p3 MMCS root).
    #[test]
    fn air_accepts_unified_layer_commit_eq_fs_observed() {
        let h = 12usize;
        for rounds in [1usize, 2, 3] {
            for levels in [2usize, 3] {
                let pre = 2usize;
                let cfg = cfg_lo_fb(h, rounds, levels, pre);
                let fs_prefix: Vec<[Val; RATE]> =
                    (0..(pre + 1) as u32).map(|r| fs_block(40 + r)).collect();
                let (row, _) = lo_fb_window(&cfg, &fs_prefix, Val::from_u32(0xABCDE));
                // the fixture uses ONE digest per round for both observe + authenticate.
                for r in 0..rounds {
                    let lo_rb = lo_round_base(&cfg, r);
                    let fb_rb = fb_round_base(&cfg, r);
                    for k in 0..DIGEST_ELEMS {
                        assert_eq!(
                            row[lo_rb + k],
                            row[fb_rb + FB_BLOCK + k],
                            "round {r} limb {k}: layer_commit != FS-observed commit (fixture inconsistent)"
                        );
                    }
                }
                let m = RowMajorMatrix::new(row, n_cols(&cfg));
                let report = check_all_constraints(&air(cfg), &m, &[], None);
                assert!(
                    report.is_ok(),
                    "honest unified layer-commit window rejected (rounds={rounds}, levels={levels}): {} failures",
                    report.failures.len()
                );
            }
        }
    }

    /// FORGERY — THE EXACT GAP (authenticate against a DIFFERENT valid commitment): this is the
    /// soundness hole the (LO≡FS-β) residual closes, exhibited so it is caught ONLY by that residual
    /// (root-binding + beta-bind both still pass). Take an honest unified window, then for round 1 build
    /// a SECOND, fully VALID layer tree authenticating the SAME descent pair (e_fold_1, e_sib_1) but
    /// under a DIFFERENT salt → a DIFFERENT leaf → a DIFFERENT real root R_auth. Replace round 1's LO
    /// sub-block (layer_commit + salt + leaf-hash perms + path) with this alternate open: its path is a
    /// real auth path, so the root binding (path reconstructs to layer_commit_1 = R_auth) STILL passes;
    /// the FS-observed `commit_phase_commit_1` and the squeezed/consumed beta_1 are UNTOUCHED, so the
    /// beta-bind STILL passes. The ONLY thing wrong: layer_commit_1 (R_auth) != commit_phase_commit_1.
    /// Pre-fix this was ACCEPTED (siblings authenticated against a commitment that did NOT generate
    /// beta_1). The (LO≡FS-β) equality residual is the load-bearing guard that rejects it.
    #[test]
    fn air_rejects_layer_commit_ne_fs_observed() {
        let pre = 2usize;
        let cfg = cfg_lo_fb(12, 2, 3, pre);
        let levels = cfg.layer_open_levels;
        let fs_prefix: Vec<[Val; RATE]> = (0..(pre + 1) as u32).map(|r| fs_block(40 + r)).collect();
        let (mut row, metas) = lo_fb_window(&cfg, &fs_prefix, Val::from_u32(0xABCDE));
        // round 1's honest descent pair + node (read straight from the consistent fixture).
        let (_orig_root, e_fold, e_sib, start_index) = metas[1];
        let parity = (start_index & 1) == 1;
        let n_nodes = 1usize << levels;
        let node = (start_index >> 1) & (n_nodes - 1);
        // build an ALTERNATE valid tree for the SAME pair under a DIFFERENT salt → DIFFERENT real root.
        let alt_salt = lo_salt(0xFEED);
        let alt_leaf = layer_leaf_digest(e_fold, e_sib, parity, &alt_salt);
        let alt_leaves: Vec<Digest> = (0..n_nodes)
            .map(|j| {
                if j == node {
                    alt_leaf
                } else {
                    core::array::from_fn(|i| Val::from_u32((j as u32 + 1) * 5003 + i as u32 + 11))
                }
            })
            .collect();
        let alt_tree = LoRefTree::new(alt_leaves);
        let alt_root = alt_tree.root();
        let alt_sibs = alt_tree.auth_path(node);
        assert_ne!(alt_root, metas[1].0, "alternate tree must yield a different real root");
        // re-pack ONLY round 1's LO sub-block against the alternate (valid) open, leaving the FS-β
        // observed block + beta_1 + descent untouched. Mirror pack_window's LO layout exactly.
        let rb = lo_round_base(&cfg, 1);
        let consts = inlined_perm::Constants::pinned();
        // layer commitment (the authentication root) = the alternate real root.
        row[rb..rb + DIGEST_ELEMS].copy_from_slice(&alt_root);
        // flattened pair (unchanged pair, by parity) + alternate salt.
        let flat = lo_flatten_pair(e_fold, e_sib, parity);
        row[rb + LO_EVAL0..rb + LO_EVAL0 + 2 * D].copy_from_slice(&flat);
        row[rb + LO_SALT..rb + LO_SALT + SALT_ELEMS].copy_from_slice(&alt_salt);
        // leaf-hash perms over flatten(pair) ‖ alt_salt.
        let input = lo_leaf_input(e_fold, e_sib, parity, &alt_salt);
        let pre_states = leaf_hash_pre_states(&input);
        for (k, st) in pre_states.iter().enumerate() {
            let cols = inlined_perm::generate_perm_cols(*st, &consts);
            let pb = rb + lo_leaf_perm_base(k);
            row[pb..pb + PERM_COLS].copy_from_slice(&cols);
        }
        // Merkle path over the alternate auth path (selectors driven by the same FS node bits).
        let mut running = alt_leaf;
        let mut node_idx = start_index >> 1;
        for (level, &sib) in alt_sibs.iter().enumerate() {
            let bit = (node_idx & 1) == 1;
            let lbase = lo_level_base(&cfg, 1, level);
            row[lbase + LO_ML_RUNNING..lbase + LO_ML_RUNNING + DIGEST_ELEMS].copy_from_slice(&running);
            row[lbase + LO_ML_SIBLING..lbase + LO_ML_SIBLING + DIGEST_ELEMS].copy_from_slice(&sib);
            let (a, b) = if bit { (sib, running) } else { (running, sib) };
            let mut input16 = [Val::ZERO; WIDTH];
            input16[..DIGEST_ELEMS].copy_from_slice(&a);
            input16[DIGEST_ELEMS..].copy_from_slice(&b);
            let cols = inlined_perm::generate_perm_cols(input16, &consts);
            row[lbase + LO_ML_PERM..lbase + LO_ML_PERM + PERM_COLS].copy_from_slice(&cols);
            running = core::array::from_fn(|i| cols[C_OUTPUT + i]);
            node_idx >>= 1;
        }
        assert_eq!(running, alt_root, "alternate in-row fold must reconstruct the alternate root");
        // sanity: the authentication root now DIFFERS from the FS-observed commitment.
        let fb_rb = fb_round_base(&cfg, 1);
        let differs = (0..DIGEST_ELEMS).any(|k| row[rb + k] != row[fb_rb + FB_BLOCK + k]);
        assert!(differs, "alternate root must differ from the FS-observed commit");
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a sibling authenticated against a VALID commitment != the FS-observed one was accepted — \
             gap NOT closed (the equality residual is not load-bearing)"
        );
    }

    /// MOVE-TOGETHER (one tamper, both effects): change round r's FS-OBSERVED commitment digest. In a
    /// REAL transcript that single change moves beta_r (the FS squeeze observes it) AND must move the
    /// authentication root (the same digest is the open root). We tamper ONLY the FS-β observed block
    /// `commit_phase_commit_r`. Effect 1 (FS squeeze): beta_r recomputed from the tampered block no
    /// longer matches the descent's consumed beta_r → the beta-bind residual rejects. Effect 2
    /// (authentication): the (LO≡FS-β) residual now sees layer_commit_r != the tampered observed digest
    /// → rejects. They MOVE TOGETHER — the digest is one and the same, exactly as in p3-fri. (Both
    /// residuals fire; either alone proves the tie. We assert overall rejection.)
    #[test]
    fn air_rejects_tampered_fs_observed_breaks_both() {
        let pre = 2usize;
        let cfg = cfg_lo_fb(12, 2, 3, pre);
        let fs_prefix: Vec<[Val; RATE]> = (0..(pre + 1) as u32).map(|r| fs_block(40 + r)).collect();
        let (mut row, _) = lo_fb_window(&cfg, &fs_prefix, Val::from_u32(0xABCDE));
        // tamper round-1's FS-observed commitment block (the digest observed to squeeze beta_1).
        let fb_rb = fb_round_base(&cfg, 1);
        row[fb_rb + FB_BLOCK + 2] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "tampering the FS-observed commitment was accepted — beta_r/authentication not tied to it"
        );
    }

    /// MOVE-TOGETHER (positive surgical proof): the same tamper as above, but we ALSO re-derive
    /// beta_1 from the tampered observed block and patch the descent's consumed beta_1 cells + the
    /// LO `layer_commit_1` block to match the tampered digest — i.e. we try to "repair" the FS-squeeze
    /// effect (Effect 1) and the equality effect (Effect 2). It STILL rejects: the tampered digest is
    /// not the real Merkle root of round-1's layer tree, so the in-AIR re-folded root != layer_commit_1
    /// (the root-binding residual). This proves the observed digest is genuinely the AUTHENTICATION
    /// root, not a free observe block — one digest, both roles, inseparable.
    #[test]
    fn air_rejects_tampered_fs_observed_even_with_beta_and_eq_repaired() {
        let pre = 2usize;
        let cfg = cfg_lo_fb(12, 2, 3, pre);
        let fs_prefix: Vec<[Val; RATE]> = (0..(pre + 1) as u32).map(|r| fs_block(40 + r)).collect();
        let (mut row, _) = lo_fb_window(&cfg, &fs_prefix, Val::from_u32(0xABCDE));
        let fb_rb = fb_round_base(&cfg, 1);
        // tamper round-1's observed commitment digest, AND copy it into the LO layer_commit_1 block
        // (repair Effect 2: equality residual). Effect 1 (beta) is left to fire — but even if a forger
        // recomputed beta_1, the path can't reconstruct to a non-root digest. We assert rejection.
        row[fb_rb + FB_BLOCK + 2] += Val::ONE;
        let lo_rb = lo_round_base(&cfg, 1);
        for k in 0..DIGEST_ELEMS {
            row[lo_rb + k] = row[fb_rb + FB_BLOCK + k];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a tampered observed digest forced into the authentication root was accepted — not a real root"
        );
    }
}
