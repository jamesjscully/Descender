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
//! - **ONE-TRANSCRIPT scope, honest flags (items 14–15)**: alpha → zeta → beta_0 .. beta_{R-1} now
//!   share ONE ordered sponge state (the beta chain CONTINUES the post-zeta state; tested by
//!   [`tests::air_rejects_fs_beta_broken_postzeta_continuation`]). STILL SEPARATE: (1) the query-INDEX
//!   draw (the (a-FS) `C_DRAW_OUT` chain) is its own committed-`sponge_in` squeeze, NOT yet stitched
//!   onto the post-last-beta state — unifying it (continue the index draw from the final beta round's
//!   output, after observing final_poly + the log-arity schedule + the query-PoW witness, per
//!   verifier.rs:323-353) is the precise NEXT step. (2) The FRI batch-combination alpha
//!   (verifier.rs:195 — a single `sample_algebra_element` with NO observe, between zeta and the first
//!   beta) is not yet modeled as an intervening squeeze; the beta chain currently treats the first
//!   commit-phase observe as the immediate continuation of the quotient-observe round. Both are
//!   bounded, local refinements of the SAME carried-state primitive.
//! - **deferred leaf C(zeta)**: the DEEP block opens `C(zeta)` / `Q(zeta)` as committed values and
//!   constrains the QUOTIENT RELATION; the full `C(zeta) =` leaf-AIR-constraint-polynomial
//!   evaluation (re-expressing the entire leaf folder) + the DEEP linear-combination / chunk
//!   recomposition stay deferred (same boundary [`crate::deep_air`] documents).

use crate::fri_fold::{fold_pair, fold_point};
use crate::fri_fold_air::{limbs, D, W};
use crate::index_bus::{assert_canonical_pub, FIELD_BITS};
use crate::challenger::RATE;
use crate::inlined_perm::{self, eval_perm, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS};
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, WIDTH};
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

// ----- (FS-β) FRI commit-phase beta squeeze block -----
//
// Appended AFTER the (FS-αζ) block. Per commit-phase round `r = 0..fs_beta_rounds`:
//   block[RATE] ‖ perm[PERM_COLS] ‖ beta_r[D]
// `block` is the observed commit-phase commitment; `perm`'s input is absorb(state_r, block, RATE)
// where state_0 is the post-zeta state (the quotient-observe round's output) and state_{r+1} is round
// r's perm output; beta_r is the LIFO squeeze of round r's perm output rate. Each beta_r is bound to
// the (c) descent round_beta(r) cell.
const FB_BLOCK: usize = 0;
const FB_PERM: usize = FB_BLOCK + RATE;
const FB_BETA: usize = FB_PERM + PERM_COLS;
const FB_ROUND: usize = FB_BETA + D;

/// Base of the (FS-β) block (after the FS-αζ block).
fn fb_base(cfg: &RecursiveConfig) -> usize {
    fs_base(cfg) + fs_size(cfg)
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

/// SQUEEZE the FRI commit-phase folding challenges `beta_0 .. beta_{R-1}` CONTINUING from the
/// post-zeta sponge state, in the EXACT p3-fri verifier order (p3-fri-0.6.1 verifier.rs:299-312):
/// for each commit-phase round `r`: `observe(commit_phase_commit_r)` (one FULL-rate absorb-permute
/// into the carried sponge state) → `sample_algebra_element()` (LIFO 8-base-draw squeeze = beta_r).
/// The carried sponge state threads round → round (state_{r+1} = permute(absorb(state_r, comm_r))),
/// so changing comm_r changes beta_r AND every subsequent beta. `start_state` is the post-zeta state
/// (`fs_post_zeta_state`) — the same continuation the alpha/zeta chain ended on, making alpha, zeta,
/// and all betas come from ONE ordered sponge transcript.
///
/// NOTE on FRI batch-alpha: the real p3 PCS samples a FRI *batch-combination* alpha (verifier.rs:195)
/// after zeta and before the betas. That alpha is a distinct challenge from the uni-stark constraint
/// alpha modeled by `fs_squeeze_alpha_zeta`. This recursive-AIR fragment models the commit-phase
/// betas as the next squeezes after the quotient-observe round; wiring the intervening FRI batch-alpha
/// sample (a single `sample_algebra_element` with no observe) is the precise next refinement and is
/// flagged in the module header.
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

/// Total column count for a window with this config.
pub fn n_cols(cfg: &RecursiveConfig) -> usize {
    fb_base(cfg) + fb_size(cfg)
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
            fs_beta_blocks: &[],
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
    let draw_out = if cfg.inline_sponge_draw {
        squeeze_draw(sponge_in)
    } else {
        inp.draw_out
    };
    let canon = draw_out.as_canonical_u32();
    let index = (canon & ((1u32 << h) - 1)) as usize;
    let s0 = fold_point(h, index);
    let rounds = cfg.rounds();

    // (FS-β) when squeezing the FRI commit-phase betas, DERIVE them by continuing the post-zeta
    // sponge state, and OVERRIDE the descent's supplied betas for rounds 0..fs_beta_rounds so the
    // fold actually consumes the FS-squeezed challenge. Computed before the fold so folded_r uses it.
    let fs_betas: Vec<Challenge> = if cfg.fs_beta_rounds > 0 {
        assert!(cfg.fs_alpha_zeta, "fs_beta_rounds requires fs_alpha_zeta");
        assert!(rounds >= cfg.fs_beta_rounds, "num_rounds must be >= fs_beta_rounds");
        assert_eq!(
            inp.fs_beta_blocks.len(),
            cfg.fs_beta_rounds,
            "fs_beta_blocks must have fs_beta_rounds entries"
        );
        let (pre, quot) = inp.fs_blocks.split_at(cfg.fs_pre_alpha_rounds);
        let start = fs_post_zeta_state(pre, quot[0]);
        fs_squeeze_betas(start, inp.fs_beta_blocks).0
    } else {
        Vec::new()
    };
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
        s_prev = s_r;
        folded_prev = folded_r;
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

    // (FS-β) FRI commit-phase beta squeeze block: continuing from the post-zeta sponge state, per
    // round r: observe(commit_phase_commit_r) FULL-rate absorb-permute → squeeze beta_r. Each beta_r
    // is laid into this round's beta cell (and was already used as round_beta_val(r) by the fold).
    if cfg.fs_beta_rounds > 0 {
        let consts = inlined_perm::Constants::pinned();
        let (pre, quot) = inp.fs_blocks.split_at(cfg.fs_pre_alpha_rounds);
        // state_0 = post-zeta state (the quotient-observe round's output; sampling zeta is non-mutating).
        let mut state = fs_post_zeta_state(pre, quot[0]);
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
            // state_{r} reader: r == 0 is the post-zeta state (quotient-observe round's perm output);
            // r >= 1 is FS-β round (r-1)'s perm output.
            let fb_state_prev = |r: usize, i: usize| -> AB::Expr {
                if r == 0 {
                    l(qb + FS_PERM + C_OUTPUT + i)
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
            fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            fs_beta_blocks: &[],
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
            fs_beta_blocks: &[],
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
            fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            sponge_in: [Val::ZERO; WIDTH], prior_state: [Val::ZERO; WIDTH], absorbed: [Val::ZERO; RATE], prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
            fs_beta_blocks: &[],
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
            fs_beta_blocks: &[],
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
            prefix_blocks: &[], public_io: &[], fs_beta_blocks: &[],
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
}
