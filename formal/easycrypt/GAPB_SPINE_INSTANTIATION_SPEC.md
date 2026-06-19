# GAP-B EC spine instantiation — executable build spec (2026-06-19)

**Purpose.** Turn the one remaining INTERNAL deployable-critical formal item (GAP-B: the
2⁻¹²⁸ SUF-CMA endpoint is composed ON PAPER across the EC files) into a turn-key build plan,
so the multi-week effort is executable without re-deriving the architecture. This is a SPEC,
not a proof — written to avoid rushing soundness-critical mechanization at unsafe depth.

## What is already mechanized (do NOT rebuild)

- **`spike_union_bound.ec`** proves, PARAMETRICALLY, `success_le_2pow_neg128`:
  given a distribution `dout` over `outcome`, predicates `success_pred / accepted_pred /
  break_pred`, the `structural_split` decomposition (success ⇒ accepted ∨ ∃b∈break_univ.
  break b), `break_card9 : size break_univ = 9`, and the per-event hypotheses
  `mu dout accepted_pred ≤ acceptedSpendBound ≤ 2⁻¹²⁹` and
  `∀b, mu dout (break_pred·b) ≤ breakBound b ≤ 2⁻¹³³`, concludes
  `mu dout success_pred ≤ 2⁻¹²⁸`. The union bound itself (`success_le_union`, via
  `mu_or`/`mu_has_le`) is DERIVED, not axiomatized (the old `success_implies_split` axiom was
  discharged 2026-06-10).
- **`gap_c_remodel_spine_wiring.ec`** proves each per-event budget ≤ 2⁻¹³³ from the
  multi-target arithmetic (`eps_* = tQ·eps_spr n`, `multi_target_le_2pow_neg133`), and the
  accepted term ≤ 2⁻¹²⁹.
- **Per-term reductions** (all PROVEN at their own bound, own adversary type — dossier §2):

  | break_t (b) | EC file | proven bound | key lemma |
  |---|---|---|---|
  | accepted-spend | accepted_spend_forgery_reduction.ec | 2⁻¹²⁹ | accepted_spend_term_le_2pow_neg129 |
  | 1 creationTimeHiddenSource | break_creation_time_hidden_source.ec | 0 (given bridge) | break_prob_zero_under_bridge |
  | 2 prevalidationLeafBinding | break_prevalidation_leaf_binding.ec | 2⁻¹⁹² | break_le_spr_bound |
  | 3 prevalidationRegistryBinding | break_prevalidation_registry_binding.ec | 2⁻¹⁹² | registry_break_bound |
  | 4 prevalidationDataAvailability | **NO EC FILE** | n/a (fail-closed) | **build-item — see Gap** |
  | 5 postchallengeSlotCreationOrRewrite | break_postchallenge_slot_creation_or_rewrite.ec | 0 | break_prob_zero |
  | 6 checkedCoreMessageBinding | break_checked_core_message_binding.ec | 2⁻¹⁵⁰ | multi_target_break_le_bound |
  | 7 slotReplayOrRollback | break_slot_replay_or_rollback.ec | 0 | break_prob_zero |
  | 8 qromOrDomainSeparation | classifier False | 0 | break_zero_of_never |
  | 9 liveSlotLeakageOrKeygen | classifier False | 0 | break_zero_of_never |

## The seam (what GAP-B closure must build)

`spike_union_bound`'s `dout / outcome / break_pred / accepted_pred / structural_split` are
ABSTRACT. The term files each prove `Pr[Game_b(A_b) @ &m : res] ≤ bound_b` for their OWN game
`Game_b` and OWN adversary type `A_b`. Closure = make these the SAME experiment:

1. **Unified experiment `Exp(A)`** (new file `spine_endpoint.ec`): model the deployed SUF-CMA
   game — `keygen; A`-driven signing oracle; `A` outputs a forgery; run `verify`; emit an
   `outcome` (the trace/record `spike_union_bound.observe` reads). `dout = ` distribution of
   `Exp(A).main()`. `success_pred = ` "fresh valid forgery accepted". This is the foundational,
   high-stakes piece — model `verify` faithfully (the dossier's verifier).
2. **`structural_split` (THE central theorem):** prove `success o ⇒ accepted_pred o ∨ ∃b.
   break_pred·b o` — an accepting fresh forgery violates the accepted-spend bound OR one of the
   9 named binding properties. This is the soundness decomposition currently carried on paper.
   Proven from `verify`'s checks (each check ↔ a break event; failing none ⇒ not fresh/invalid).
3. **9 reductions `R_b : A ↦ A_b`:** for each b, a reduction turning the unified adversary `A`
   into the term-file adversary `A_b`, proving `mu dout (break_pred·b) ≤ Pr[Game_b(R_b(A))] ≤
   bound_b` (the term lemma). For the `=0` events (1 given bridge, 5, 7, 8, 9) this is a
   structural argument; for 2,3,6 + accepted it's the reduction wiring to the proven bound.
4. **Discharge:** `apply spike_union_bound.success_le_2pow_neg128` with the bounds from (3),
   each ≤ 2⁻¹³³ (and accepted ≤ 2⁻¹²⁹) via `gap_c_remodel_spine_wiring`. Endpoint ≤ 2⁻¹²⁸.

## Build prerequisite found (2026-06-19, tested empirically)

`spike_union_bound.ec` is a FLAT declaration file (abstract `type break_t / outcome`, `op`s,
`axiom`s at top level), NOT a clone-able `abstract theory`. A direct `clone Spike_union_bound
with type break_t <- break_event …` FAILS to parse — you cannot instantiate the abstract types
by cloning a flat file. So step 0 of the build is: **refactor `spike_union_bound.ec` to wrap its
abstractions in `abstract theory Spine. type break_t. … end Spine.`** (re-proving the lemmas
inside, keeping the file 10/10-typechecking), so the concrete experiment file can
`clone import Spine with type break_t <- break_event, op break_univ <- break_event_univ` and
discharge the finiteness axioms with `spine_break_events.ec`'s lemmas. This refactor touches a
SOUNDNESS-CRITICAL working file → do it carefully with a typecheck gate, NOT at unsafe depth.
`spine_break_events.ec` (the concrete 9-event datatype + finiteness, EC-verified) is ready to
plug into that clone.

## Build order (each step a committable EC file/lemma, honest-checked)

0. **Refactor `spike_union_bound.ec` → `abstract theory Spine`** (prerequisite, above).
1. `spine_endpoint.ec` skeleton: `outcome` type + `Exp(A)` experiment + predicate defs +
   `clone import Spine with type break_t <- break_event …` (discharge finiteness via
   spine_break_events) + `require import gap_c_remodel_spine_wiring` + the term files. Typecheck.
2. `structural_split` for the trivially-`=0` events (5,7,8,9 + 1-given-bridge) — structural,
   no probability. Lowest-risk; do first.
3. Reductions R_b + bound for the PROVEN-bound events (2,3,6, accepted) — the substantive
   wiring; each discharges one hypothesis of `success_le_2pow_neg128`.
4. Final `apply` → `endpoint_le_2pow_neg128`. The 2⁻¹²⁸ composed IN EC, not on paper.

## Gaps / risks to surface during the build

- **#4 prevalidationDataAvailability has NO EC reduction** (held fail-closed, dossier §8). The
  endpoint either excludes it (DA is a liveness/availability property, arguably not an SUF-CMA
  break) or needs a reduction built. DECIDE during step 2.
- **`verifier_exporter_bridge` axiom** (#1) and **`sha256_multi_target_spr` axiom** (#6) remain
  named-standard axioms — the endpoint is "PROVEN modulo these named axioms", same status the
  per-term files carry. Not new debt.
- **Faithful `verify` model** (step 1) is the high-stakes foundation — a wrong model poisons
  `structural_split`. This is why GAP-B is multi-week, not bounded: step 1+2 must be done with
  care and cross-checked against the Rust verifier, NOT rushed.
- Estimated effort: the `=0` events + skeleton are ~days; `structural_split` + the 3 nontrivial
  reductions are the multi-week core. No bounded shortcut (the parametric spine is already done;
  this is the instantiation).

## What this does NOT change

Production stays fail-closed / external-review-gated. This spec mechanizes the SUF-CMA
COMPOSITION (GAP-B); the IRREDUCIBLY EXTERNAL residuals (third-party coded-SIS cryptanalysis;
QROM modeling boundary) are unchanged and remain the external gate.
