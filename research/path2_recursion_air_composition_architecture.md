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

## Decision

**Adopt Option A (monolithic, inlined Poseidon2 constraints).** It matches the validated
gadget-1 pattern (custom columns + `check_constraints` + tamper tests), needs no p3-internals
adapter, and is the standard shape for a hand-written verifier-AIR on uni-stark. The inlined
Poseidon2 constraint routine is written ONCE and reused across all perm instances in the AIR.
`perm.rs::CompressAir` remains the byte-reference the inlined constraints are tested against
(honest perm output must match a `default_koalabear_poseidon2_16` evaluation; tamper rejected).

## Impact on the remaining-work estimate

This is why the recursion build is genuinely multi-week, not "transcribe the references":
`recursive_air` is one wide monolithic AIR that must inline (a) the gadget-1 challenger
constraints (done as a standalone AIR — fold into the monolith), (b) the Poseidon2 round
constraints (new inline routine), (c) the merkle fold wiring, (d) the FRI fold + DEEP +
public-IO constraints, all over a shared column layout, plus the deferred mirror pieces (leaf
`C(zeta)`, chunk recomposition, pinned constants). Each lands with honest-accept + tamper-reject
tests. No new PROOF risk — but real, careful engineering. Production stays fail-closed; the flip
remains external-review-gated.
