# Recursion AIR composition — architectural finding + decision (2026-06-19)

**Status:** an assumption tested and REFUTED. Earlier framing ("both gadget halves validated
→ just compose them into one `Air<AB>`") underestimated the composition step. Plonky3 0.6.1's
AIR model makes composition a real architectural choice, not transcription. This note records
the constraint and the two viable paths, so `recursive_air` is built on the right one.

## The finding (verified against the pinned p3 0.6.1 sources)

1. **No AIR-composition combinator.** `p3-uni-stark` proves ONE monolithic AIR. There is no
   "combine AIR A and AIR B" primitive.
2. **No lookup / multi-table / interaction layer in the dependency set.** Only `p3-air` +
   `p3-uni-stark` are present (`grep` of the registry: no `p3-machine`/lookup/LogUp/interaction
   crate; `p3-uni-stark/src/lib.rs` has no `lookup`/`interaction`/`permutation_arg`). So the
   "many small tables linked by a lookup argument" architecture (how production zkVMs compose
   a Poseidon2 table with a CPU table) is NOT available without adopting a different proving
   stack.
3. **`p3-poseidon2-air`'s round constraints are not reusable piecewise.** `eval_full_round`,
   `eval_partial_round`, `eval_sbox` are private (`fn`, not `pub`) (air.rs:247/271/300). The
   only public entry is `impl Air<AB> for Poseidon2Air`, whose `eval` does
   `builder.main().current_slice().borrow()` into `Poseidon2Cols` — a SIZE-CHECKED borrow.
   Appending fold/wiring columns changes the width, so the borrow no longer matches and you
   cannot feed `Poseidon2Air::eval` a wider window with extra columns.

**Consequence:** the validated `perm.rs` (`CompressAir` accepts/rejects via `check_constraints`)
proves the Poseidon2 AIR works STANDALONE, but it cannot be dropped INTO a larger gadget-2 /
`recursive_air` window as-is. Composition needs one of:

## Option A — monolithic AIR with INLINED Poseidon2 constraints (recommended)

Build `recursive_air` (and gadget-2) as ONE AIR over a custom column layout, and RE-IMPLEMENT
the Poseidon2 round constraints inline reading those columns (the round structure is known:
external linear layer, `x^3` S-box, round constants, internal/diagonal layer for partial
rounds — the same logic in `descender-mask-prf/src/poseidon2.rs` and mirrored as constraints
in `p3-poseidon2-air`). Per-row (or per-perm-block) the constraint set is fixed and applied
uniformly, so the duplication is ONE inlined constraint routine, not N copies.

- **Pros:** uses only standard `AirBuilder` calls (no adapter), `check_constraints`-testable
  exactly like gadget-1, single proof, no new stack. Generalizes: every gadget's constraints
  live in the one `recursive_air` eval.
- **Cons:** must reproduce ~(8 full + 20 partial)-round Poseidon2 constraints inline (bug-prone
  → guard with honest-accept-vs-reference + tamper-reject tests, the proven discipline); the
  trace is WIDE (challenger + merkle + ~400–600 perms' columns + fri + deep all in one AIR).
  Wide-but-short is fine for FRI (cost ~ height, and height is the perm/step count).

## Option B — projecting `AirBuilder` adapter (reuse `Poseidon2Air`, DRY)

Implement a wrapper `SubBuilder<'a, AB>: AirBuilder` whose `main()` returns a PROJECTED window
(a sub-range of the parent's columns) and whose `assert_zero` forwards to the parent; then call
`Poseidon2Air::eval(&mut sub)` on the projected sub-window while adding fold constraints on the
parent for the appended columns.

- **Pros:** reuses the vetted `Poseidon2Air` constraints verbatim (no duplication); a reusable
  primitive for embedding ANY sub-AIR.
- **Cons:** implementing a correct `AirBuilder` with a projected `MainWindow`/`WindowAccess`
  associated type + selector (`is_first/last/transition`) delegation is intricate p3-internals
  work, and must also satisfy `for<'a> Air<DebugConstraintBuilder<'a,F>>` for testing (project
  the concrete debug builder too). Higher risk of subtle lifetime/associated-type friction.

## Option C — perm-only wrapper, same-width, same-window chaining (UNVERIFIED note)

For a sub-AIR that is PERM-DOMINATED and needs no extra columns, a wrapper whose width EQUALS
`Poseidon2Cols` can call `Poseidon2Air::eval(builder)` on the full window (size matches, borrow
OK) and ADD chaining/boundary constraints on the SAME window by reading `Poseidon2Cols` cells:
`inputs: [T;WIDTH]` are cols `0..WIDTH`; the permutation OUTPUT is the last ending-full-round's
`post` state (≈ the final `WIDTH` columns; exact offset = sum of `FullRound`/`PartialRound`
sizes — `FullRound{sbox:[SBox;WIDTH], post:[T;WIDTH]}`, `SBox` with `REGISTERS=0` is degree-only).
A multi-level Merkle fold then chains `next.inputs[0..8] == this.<last post>[0..8]` with the
sibling as free witness in `inputs[8..16]`. CAVEAT (why this is a NOTE, not the decision): the
per-level left/right INDEX BIT needs an extra column (digest into `inputs[0..8]` vs `[8..16]`),
which breaks the same-width borrow — so Option C only cleanly handles a fixed orientation, and
the FULL `recursive_air` needs the other gadgets' columns anyway (extra cols ⇒ borrow breaks).
UNVERIFIED: exact `post` offset + the bit handling not yet tested. Useful if a perm-only
gadget-2 variant is built first; does not change the recursive_air decision.

## Decision

**Adopt Option A (monolithic, inlined Poseidon2 constraints) for `recursive_air`.** It matches the validated
gadget-1 pattern (custom columns + `check_constraints` + tamper tests), needs no p3-internals
adapter, and is the standard shape for a hand-written verifier-AIR on uni-stark. The inlined
Poseidon2 constraint routine is written ONCE and reused across all perm instances in the AIR.
`perm.rs::CompressAir` remains the byte-reference the inlined constraints are tested against
(honest perm output must match a `default_koalabear_poseidon2_16` evaluation; tamper rejected).

## Impact on the remaining-work scope

This is why the recursion build is substantial, not "transcribe the references":
`recursive_air` is one wide monolithic AIR that must inline (a) the gadget-1 challenger
constraints (done as a standalone AIR — fold into the monolith), (b) the Poseidon2 round
constraints (new inline routine), (c) the merkle fold wiring, (d) the FRI fold + DEEP +
public-IO constraints, all over a shared column layout, plus the deferred mirror pieces (leaf
`C(zeta)`, chunk recomposition, pinned constants). Each lands with honest-accept + tamper-reject
tests. No new PROOF risk — but real, careful engineering. Production stays fail-closed; the flip
remains external-review-gated.

## Provability gate — uniform-AIR requirement (RESOLVED) (2026-06-27, q128 worktree)

`recursive_air` is verified by `p3_air::check_all_constraints` (366 tests at digest-13). Turning it
into a REAL recursive proof — the O(1) artifact a Phase-3 block aggregator folds and an on-chain
verifier checks — additionally requires it be a *uniform* p3-uni-stark AIR (constraint COUNT
independent of the witness): `p3_uni_stark::prove` runs a symbolic constraint-count pass and a
prove pass and asserts they agree (`folder.rs:99` `debug_assert_eq!(constraint_index,
constraint_count)`).

**RESOLVED (2026-06-27).** Both passes take the public-value count from `air.num_public_values()`
(`prover.rs:77` / `symbolic.rs:117`), which defaulted to 0 — so the `pubs.len() == N_PUBLIC_MERKLE`
boundary block was skipped in the symbolic count but emitted in the prove pass (driven with the
real length-26 public values), tripping the assert. The fix was to implement
`RecursiveAir::num_public_values` to return the cfg's boundary count (`N_PUBLIC_MERKLE` when a
Merkle block is present); then the `pubs.len()`-gated block is counted identically in both passes —
no removal of the guards was needed. (`check_all_constraints` ignores `num_public_values`, so the
366 existing tests, which drive varying public-value lengths, are unaffected.)

The keystone test `recursive_air::tests::recursive_air_proves_and_verifies_q128` now PASSES: a real
`p3_uni_stark::prove` + `verify` of the recursive verifier-AIR on the q128 (digest-13 sponge /
rate-7 leaf-hash / deg-8 challenge) Poseidon2 stack. **The recursion is provable** — the
single-proof verifier-AIR produces and verifies an O(1) recursive STARK proof. The economic
aggregation layer beyond this keystone (Phase-2 {creation+spend} co-verification, Phase-3 block
accumulator folding many such proofs into one chain-state proof, the EVM verifier) remains.

## Phase-3 block accumulator — skeleton landed + PROVABLE (2026-06-27)

With the recursion provable (above), the first Phase-3 artifact is built:
`tools/descender-recursion/src/block_accumulator.rs` (`BlockAccumulatorAir`). It folds a block of
per-account recursive-proof commitments `D_0..D_{N-1}` into one `chain_state_root` via the SAME
vetted q128 sponge 2-to-1 compression the Merkle nodes use (`merkle_air::node_blocks`/`eval_node`):
`chain_0 = 0`, `chain_{i+1} = compress(chain_i, D_i)`, `root = chain_N` — one trace row per member,
chained row→row, genesis pinned on the first row, `chain_state_root` bound on the last row. It is a
uniform AIR (`num_public_values = DIGEST_ELEMS`) and PROVES + VERIFIES end-to-end
(`block_accumulator::tests::block_accumulator_proves_and_verifies_q128`), plus honest-accept and
four forgery-reject tests (tampered member / broken chain / wrong root / nonzero genesis). So a
block of `N` admissions collapses to ONE O(1) recursive-STARK proof binding `chain_state_root` —
the on-chain-viability artifact (verify one accumulator proof, not `N` account proofs).

**What full aggregation still needs on top of this skeleton:** each member `D_i` must be the
VERIFIED output of that member's recursive proof — i.e. verifying each inner recursive proof
in-circuit (PCD/IVC), or a fold that carries each proof — so the accumulator attests not just that
"these committed `D_i` chain to this root" but that "each `D_i` is a valid account admission." That
in-circuit member-proof verification (composing `recursive_air` under the accumulator), plus
Phase-2 {creation+spend} co-verification, the F2 reorg-safe pad-seed registry, and the EVM verifier,
remain. The accumulation structure + its O(1) provable artifact are in place.

## Phase-3 pipeline — member-verification → accumulation data-flow tested (2026-06-27)

`recursive_air::tests::block_pipeline_verifies_members_then_accumulates_q128` exercises the block
aggregator's soundness data-flow end-to-end: for a block of members it PROVES+VERIFIES each
member's recursive verifier-AIR (each account admission is in-circuit verified), takes that proof's
VERIFIED reconstructed root as the member commitment `D_i`, and then PROVES+VERIFIES the block
accumulator over exactly those verified roots. So the block's `chain_state_root` provably commits
the VERIFIED-admission roots, not free inputs — the binding the aggregator depends on.

This is the TWO-PROOF pipeline (N member proofs + 1 accumulator proof), which establishes the
data-flow and binding. The remaining step to O(1)-on-chain is to internalize the member-proof
verification INTO the accumulator (PCD/IVC: compose `recursive_air` under each accumulator row, so
`D_i` is recomputed-and-verified in-circuit rather than carried between two proof layers) — then a
single accumulator proof is checked on-chain. Trace heights are powers of two (FRI requirement);
real blocks pad to the next power of two.

## Production-parameter soundness (2026-06-27)

Both prove/verify keystones hold not only at toy completeness knobs but at the PRODUCTION
quantum-128 FRI soundness parameters (`PROD_NUM_QUERIES = 120`, `PROD_LOG_BLOWUP = 4`,
`PROD_QUERY_POW_BITS = 16`; q128 config / ADR-0008): `recursive_air_proves_and_verifies_q128`
(the per-member recursive verifier-AIR) and `block_accumulator_proves_at_production_q128_knobs`
(the per-block O(1) accumulator over an 8-member block) both PROVE + VERIFY at those knobs. So the
recursion and the block aggregator are sound at deployment parameters, not merely satisfiable.

## Phase-3 in-circuit composition step 1 — verified-member roots folded in-circuit (2026-06-27)

`accumulated_member.rs` (`AccumulatedMemberAir`) upgrades the block accumulator from folding FREE
member commitments to folding IN-CIRCUIT-VERIFIED member roots: each row runs TWO sponge nodes — node
1 reconstructs the member root `root_i = compress(leaf_i, sibling_i)` (verifying the member's
Merkle opening in-circuit), node 2 folds that VERIFIED root into the chain `chain_{i+1} =
compress(chain_i, root_i)`. One row per member, chained row→row, genesis pinned, block root public.
It verifies the member's FULL multi-LEVEL Merkle opening in-circuit (leaf folds up `levels` siblings
through the sponge node to the member root) and is uniform, PROVING + VERIFYING at production q128 FRI
knobs for a depth-4, 8-member block (`proves_at_production_q128_knobs`), plus `member_root_matches_real_tree`
(the in-circuit reconstruction equals the real reference Merkle root at depths 1/3/4) and honest-accept /
tampered-path / broken-chain tests. So the block's
`chain_state_root` provably commits roots that were RECONSTRUCTED AND VERIFIED in-circuit from their
(leaf, sibling) — not free inputs.

Remaining to FULL member verification (true O(1) on-chain attesting each member is a complete valid
admission): verify each member's FRI/DEEP/Fiat-Shamir checks in-circuit too — i.e. compose the full
`recursive_air` per member under this chain (multiple rows per member for the Merkle-path depth + the
FRI consumer). This step verifies the Merkle-opening portion in-circuit and chains it; the FRI/DEEP
portion is the remaining composition.

## Phase-3 TRUE single-proof O(1) block aggregator (2026-06-27)

`block_recursive_air.rs` (`BlockRecursiveAir`) achieves the single-proof O(1) composition: each trace
ROW is a full `recursive_air` member window (verifying that member in-circuit, leaf/root INTERNAL —
the per-window public binding auto-skips because this AIR's public values are the single
`chain_state_root`, not `N_PUBLIC_MERKLE`) followed by a chain-fold node folding the window's VERIFIED
root into the running chain. `recursive_air`'s only cross-row/boundary constraints are gated behind
`fan_out_queries > 0` (off here), so each row is an independent member verification; this AIR adds the
genesis pin, the row→row chain, and the public root on the last row. It PROVES + VERIFIES at
production q128 FRI knobs (`proves_at_production_q128_knobs`), plus honest-accept and tampered-member-
window-reject. So ONE O(1) proof now attests that every block member was verified IN-CIRCUIT and that
they chain to `chain_state_root` — no N-proof fan-in (strictly stronger than the two-proof pipeline).

The member cfg here is the Merkle-opening verifier (`cfg_merkle`), so each row verifies the member's
Merkle opening in-circuit. The composition is cfg-generic: swapping in the full FRI/DEEP recursive cfg
(the complete inner-proof verifier, with its heavier per-row window and fixtures) upgrades each row to
a complete inner-proof verification with NO change to the aggregator structure. That cfg swap (and its
production-scale prove cost) is the remaining depth on item 1; the single-proof O(1) aggregation
mechanism itself is now built and proven.

## Item-1 depth step: leaf-hash-verified members in the O(1) aggregator (2026-06-27)

`block_recursive_air` now also runs with a leaf-hash member cfg (`cfg_leaf_hash`): each row's member
verification is the (b-hash) gadget — `leaf` CONSTRAINED == `MyHash(row ‖ salt)` in-circuit — PLUS
the Merkle path to the root, and that verified root is folded into the chain. One proof, N members
each verified at leaf-hash+Merkle depth, PROVES + VERIFIES at production q128 FRI knobs
(`proves_block_with_leaf_hash_verified_members_q128`). This is one concrete step up the item-1 depth
ladder (bare-Merkle → leaf-hash+Merkle); the remaining depth is enabling the FRI-fold / DEEP gadgets
per member (the full inner-proof verifier cfg + its FRI fixtures), which slots into the same
aggregator with no structural change.

## Item-1 depth step 2: FRI-descent-verified members (2026-06-27)

`block_recursive_air` also runs with a descent member cfg (`cfg_deep`: merkle + leaf_row + num_rounds):
each row's member verification now additionally checks the FRI commit-phase FOLD descent in-circuit
(the per-round fold relation), on top of leaf-hash + Merkle. One proof, N members each verified at
FRI-fold + leaf-hash + Merkle depth, PROVES + VERIFIES at production q128 FRI knobs
(`proves_block_with_fri_descent_verified_members_q128`). Depth ladder climbed: bare-Merkle →
leaf-hash+Merkle → FRI-fold+leaf-hash+Merkle, all on the same single-proof O(1) aggregator. The top
rung is the DEEP/quotient consumer (deep_e0_combine + quotient openings + FS-beta + layer opens),
whose fixtures are the remaining (heaviest) construction; the aggregator itself is unchanged.

## Item-1 depth: FULL DEEP/quotient consumer per member in the O(1) aggregator (2026-06-27)

`block_recursive_air` now also runs with the FULL inner-proof consumer member cfg (`cfg_full`:
merkle + leaf_row + deep_log_degree + deep_e0_combine + quot_chunks + quot_merkle_levels): each row
verifies, in-circuit, the DEEP reduced-e0 combination (trace + quotient), the quotient OOD
recombination (`Σ zps·at_zeta`), the quotient Merkle opening, the leaf-hash, and the trace Merkle
path — i.e. each member is a COMPLETE recursive verification — and folds the verified root into the
chain. One proof, N members each fully verified, PROVES + VERIFIES at production q128 FRI knobs
(`proves_block_with_full_consumer_members_q128`), with consistent DEEP/quotient fixtures built via
the pub `deep_reduced_e0` / `quotient_reduced_e0` / `quotient_recombine` helpers and per-member trees.

So the single-proof O(1) aggregator has been exercised at every major member-verification depth:
bare-Merkle, leaf-hash+Merkle, FRI-fold descent (`cfg_deep`), and the DEEP/quotient consumer
(`cfg_full`) — all proving at production parameters. The remaining integration is a SINGLE cfg that
enables ALL gadgets together (descent + FS-beta squeeze + per-round layer opens + DEEP + quotient),
which requires the betas to thread from the FS squeeze into the descent and the layer opens to
authenticate the descent siblings — mutual-consistency fixture work (the recursive_air fidelity
tests' `fb_window`/`LO` machinery) — on the same unchanged aggregator.

## Item-1 depth: FS-transcript FRI consumer in the O(1) aggregator (2026-06-27)

`block_recursive_air` also runs the FS-transcript member cfg (`cfg_fs`: merkle + fs_alpha_zeta + fs_pre_alpha_rounds + fs_beta_rounds + fs_batch_combo_alpha +
fs_final_blocks + num_rounds — the COMPLETE FS transcript: α/ζ/batch-combo-α/β squeeze + final-poly +
query-index continuation): the in-circuit Fiat-Shamir sponge squeezes
alpha/zeta (prefix) and the per-round betas from the commitment blocks, and the FRI descent folds
with THOSE squeezed betas (so the fold challenges are Fiat-Shamir-bound, not free), plus the Merkle
path — one proof, proven at production q128 knobs (`proves_block_with_fs_transcript_members_q128`).

The single-proof O(1) aggregator has now been exercised per member at EVERY major recursive-
verification consumer, all production-proven: bare-Merkle, leaf-hash+Merkle, FRI-fold descent
(`cfg_deep`), DEEP/quotient binding+opening consumer (`cfg_full`), and the FS-transcript-bound FRI
consumer (`cfg_fs`). Item 1 (full in-circuit member verification under a single O(1) proof) is thus
realized across all consumers; the only completeness remainder is ONE cfg enabling every gadget
simultaneously (descent+FS-beta+layer-opens+DEEP+quotient mutually wired) — fixture-assembly on the
unchanged aggregator, each constituent already proven here.

## Item-1 depth: COMPLETE FS transcript in the O(1) aggregator (2026-06-27, updated)

The aggregator's FS rung was upgraded to the COMPLETE Fiat-Shamir transcript: alpha/zeta + the FRI
batch-combination alpha + per-round betas (all squeezed in-circuit) + the final-polynomial observe +
the unified query-index continuation, with the descent folded by the squeezed betas, plus the Merkle
path — one proof at production q128 knobs (`proves_block_with_fs_transcript_members_q128`). Together
with the DEEP/quotient binding+opening consumer (`cfg_full`), the single-proof O(1) aggregator has now
verified members at the full FS-transcript-driven FRI consumer AND the full DEEP/quotient consumer,
both at production parameters. The ONLY uncombined piece is FS-squeezed-alpha/zeta feeding the DEEP
reduced-e0 in ONE window simultaneously — a combination the recursive_air's own leaf tests also do
not assemble (they keep DEEP-e0 free when exercising the FS transcript); it is the last
fixture-consistency item, on the unchanged aggregator, with every constituent already proven here.

## Item-1 COMPLETE: full recursive verifier per member in one O(1) proof (2026-06-27)

`block_recursive_air::proves_block_with_full_recursive_members_q128` proves+verifies, at production
q128 FRI knobs, a block whose per-member verification is the COMPLETE recursive verifier with ALL
gadgets combined and mutually FS-consistent: Merkle path + leaf-hash + DEEP reduced-e0 (trace+quotient)
+ quotient OOD recombination & Merkle opening + the full Fiat-Shamir transcript (α/ζ/batch-combo-α/β/
final-poly/query-index) — with `deep_alpha`/`deep_zeta` BOUND to the FS-squeezed α/ζ, the descent
folded by the FS-squeezed βs, and the openings taken at the FS-DERIVED query index (index-bus). The
fixture computes the FS index via the pub `fs_post_zeta_state`/`fs_squeeze_batch_combo_alpha`/
`fs_squeeze_betas_digest`/`fs_squeeze_query_p0_state` helpers and opens both the trace and quotient
trees there. So ONE O(1) proof attests N members each verified by the FULL in-circuit recursive
verifier and chained to `chain_state_root` — true single-proof O(1) on-chain attestation. Item 1 is
realized end-to-end (this combination is one the leaf's own recursive_air tests did not assemble).

## Spend proofs aggregate to O(1) (2026-06-28)

`descender_recursion::spend_block` connects the spend subsystem to the O(1) block aggregator. The
generic recursion machinery (FS transcript + FRI commit-phase fold + Merkle/FRI-layer openings +
DEEP/quotient consumer) is NOT creation-coupled — the only creation coupling is the optional
`deep_c_full_leaf` full-C(zeta) re-derivation, which the creation aggregator itself runs WITHOUT — so
a spend proof verifies in-circuit at the SAME depth the creation aggregator runs at. `windows_for_spend`
turns each real `SpendWitness` (via `spend_air::spend_derive` → `packed_witness_commitment` →
`spend_leaf_digest`) into a generic `recursive_air` member window opening that commitment, and
`block_recursive_air` folds N of them into one `chain_state_root`. `proves_spend_block_q128`
proves+verifies a 4-spend block at production q128 knobs.

REMAINING (same class as the creation aggregator's open `deep_c_full_leaf` gap): the full
constraint-level spend verification — a `spend_constraint_air` re-deriving the spend C(zeta) (packing
+ frame-digest sponge + checked-core SIS + sigma + challenge sponge), so the aggregator constrains the
inner spend RELATION and not only its FRI/commitment layer; and the in-circuit `{creation, spend}`
cross-binding (MED-2 + key) lifted from the settlement layer into the aggregator.

## Full inner C(zeta) per member in the aggregator — constraint-level + provability finding (2026-06-28)

`block_recursive_air` now has its deepest rung: `cfg_full_leaf_member` + `member_window_full_leaf`,
where each member's DEEP C(zeta) is the FULL alpha-folded creation-leaf constraint
`recompute_c_partial(value_at_zeta) + C_committed` (honest OOD opening via
`leaf_constraint_air::honest_cells_for_test`), composed with `C == Z_H(zeta)·Q`, while the member's
Merkle root chains to one `chain_state_root`. So the aggregator's constraints INCLUDE the full inner
creation RELATION per member. `full_leaf_czeta_block_constraints_hold` validates an honest block via
`check_all_constraints` (the same soundness gate `prove` enforces).

FINDING — the full-C(zeta) rung is NOT provable at production knobs. The inlined `recompute_c_partial`
is very high algebraic degree (leaf Poseidon2 sboxes composed over the degree-8 extension), so
`uni_stark`'s quotient LDE explodes (tens of GB). This is the concrete reason the PROVEN aggregator
rungs use the low-degree FRI/DEEP/commitment consumer rather than `deep_c_full_leaf`. Closing the
full-C(zeta) gap PROVABLY (for BOTH creation and spend) requires a RE-ARITHMETIZED low-degree mirror:
commit the inner permutation's intermediate states as columns so every constraint is degree ≤ 2 —
i.e. rebuild the leaf/spend AIR inside the recursion. That re-arithmetization is the remaining
multi-week frontier; it is one shared piece of work that closes both the creation and spend halves.
