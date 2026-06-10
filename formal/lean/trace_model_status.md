# Path 2 Concrete Trace Model — Status

File: `Descender/Path2TraceModel.lean` (sorry-free, axiom-clean: only
`propext`, `Classical.choice`, `Quot.sound`).

## What this increment delivers

The closure skeleton in `Descender/Path2AcceptedSpendClosure.lean` proves
`successProb ≤ 2^-128` **given** an `AcceptedSpendNumericGame Trace Outcome
BreakEvent` whose per-event probabilities meet the standard cryptographic
budgets. That closure was abstract over the game `G`. This file supplies a
*faithful* `G` and discharges the part of the budget that is real-and-now.

### Proven (not faked)

1. **Faithful outcome surface.** `ConcreteOutcome :=
   Path2FullSpendGame.FullForgeryRun concreteTypes` — exactly the run carrier
   already used by `Path2FullSpendConcrete`, not a proof-convenient finite
   stand-in.

2. **Events pinned to the real verifier predicates.**
   - `concreteFullSignatureSuccess` = `decide (ConcreteSpendVerificationAccepts C …)`
   - `concreteAcceptedSpendForgery` = `decide (AcceptedSpendConclusion …)`
   - `concreteNamedBreak b` = `decide (ConcreteBreakFor C … b)`
   These are the actual structural predicates, not toy booleans.

3. **Soundness field derived, not assumed.**
   `concrete_success_implies_accepted_or_break` is obtained from the proven
   structural theorem
   `concrete_account_creation_prevalidated_slot_authorization_or_break` via the
   closure bridge `success_implies_accepted_or_break_of_structural`. No new
   assumption enters here.

4. **The game constructor does not rig probabilities.**
   `concreteNumericGame` takes the trace support `[Fintype Trace]`,
   `traceDist`, and `observe` as **inputs**. It chooses none of them, so it
   cannot make `successProb ≤ 2^-128` vacuously true. Whatever sampler a
   deployment supplies, the events stay pinned to the real verifier.

5. **Two per-event break bounds genuinely discharged to `0`.**
   `ConcreteBreakFor` classifies `qromOrDomainSeparationBreak` and
   `liveSlotLeakageOrKeygenBreak` as `False` *at this layer* (they are charged
   in the separate hash/QROM sub-game). The boolean events therefore never fire
   (`qromBreak_never`, `liveSlotLeakageBreak_never`), so their probability is
   provably `0` (`qromBreakProb_eq_zero`, `liveSlotLeakageBreakProb_eq_zero`) via
   `breakProb_eq_zero_of_never`. This is a real `never`-proof of a real boolean
   event, not a vacuous `True`/empty-type cheat.

## Remaining obligations (named, NOT `sorry`, NOT faked)

`concreteNumericClosure_of_cryptographic_bounds` states the closed `2^-128`
conclusion for the faithful game with the following as **explicit hypotheses**:

| Obligation | Hypothesis in the theorem | Why still open |
|---|---|---|
| Accepted-spend forgery bound `≤ 2^-129` | `hAccept` + `hAcceptBudget` | This is the core SIS / response-link-coded hardness reduction. It is the multi-week WS7 forgery reduction and is never proven unconditionally — it is a standard cryptographic assumption that a reduction must price. |
| The 7 cryptographically-charged named breaks `≤ 2^-133` each | `hBreak` + `hBreakBudget` | `creationTimeHiddenSourceSoundnessBreak`, `prevalidationLeafBindingBreak`, `prevalidationRegistryBindingBreak`, `prevalidationDataAvailabilityBreak`, `postchallengeSlotCreationOrRewriteBreak`, `checkedCoreMessageBindingBreak`, `slotReplayOrRollbackBreak`. Each requires its own QROM / hash-collision / domain-separation / consensus-rollback bound. These are the per-event reductions WS7 owns. |
| `qromOrDomainSeparationBreak`, `liveSlotLeakageOrKeygenBreak` | satisfied for free | Already proven `= 0 ≤ 2^-133` here. |

### Why the open items cannot be discharged now

- **Tooling.** The forgery reduction (accepted-spend bound) is the candidate for
  migration to EasyCrypt (the EC-SPIKE decision); EasyCrypt is **not installed**
  in this environment, so the SIS reduction cannot be machine-checked here even
  if written.
- **Mathematical depth.** Each cryptographic break bound is a genuine
  QROM/collision argument, not a Mathlib arithmetic fact. Producing a sound
  `breakProb b ≤ 2^-133` for the seven active breaks is the multi-week reduction
  work, not a one-file increment.
- **Honesty constraint.** Discharging them with a toy distribution that makes the
  events impossible would be exactly the vacuous `G` the task forbids. The
  faithful game keeps the events tied to the real verifier, so the bounds are
  necessarily the real cryptographic ones.

## Next steps for WS7

1. Supply a SIS / forking reduction bounding
   `(concreteNumericGame …).acceptedSpendProb ≤ 2^-129` (EasyCrypt or Lean).
2. Supply per-break QROM/hash/rollback reductions bounding the seven active
   `breakProb b ≤ 2^-133`.
3. Feed both into `concreteNumericClosure_of_cryptographic_bounds` to close the
   faithful concrete game at `2^-128` with no remaining hypothesis.
