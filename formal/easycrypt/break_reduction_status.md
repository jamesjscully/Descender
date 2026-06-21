# Break-event reduction status — first real Phase-2 EC increment

> PACKAGE CONTEXT (2026-06-20): This package-scoped historical note is covered
> by `research/ARCHIVE_INDEX.md`. Current authority starts at `spec/README.md`,
> ADRs in `decisions/`, and `TRACKER.md`.

**Date:** 2026-06-09
**Tooling:** EasyCrypt 2026.05, Z3 4.16.0 (why3 auto-config).
**Artifact:** `formal/easycrypt/break_slot_replay_or_rollback.ec` — `easycrypt` **exit 0**.
**Gating follow-up:** the EC-SPIKE verdict's named next step — "one real
break-event reduction before committing all of Phase 2."

---

## Event attempted: `slotReplayOrRollback` (BreakEvent #7)

### Why this event (triage)

The 7 cryptographically-charged breaks split, per `spec §5` and
`research/cryptanalysis/ws7_reduction_scoping.md`, into reduction families:

| Family | Events | Best first proof |
|---|---|---|
| **(c) state-machine / structural** | `slotReplayOrRollback`, `postchallengeSlotCreationOrRewrite` | **=0 by a discharged invariant** — no hardness assumption |
| (b) digest/Merkle binding | `checkedCoreMessageBinding`, `prevalidationLeafBinding`, `prevalidationRegistryBinding`, `creationTimeHiddenSource` | reduction to a CR game (CR bound stays a named assumption) |
| data-availability | `prevalidationDataAvailability` | mixed (availability + binding) |

`spec §5` explicitly classifies the replay/rollback family as "a **state-machine**
violation (replay/rollback — **proven impossible by the consume-once
lifecycle**)", and `ws7_reduction_scoping.md` marks `slotReplayOrRollback` as
**PROVE** (not NAME): "consume-once slot state machine (D2: must be proven, not
assumed)". This is the unique event that admits a **fully discharged real proof
now**, with **zero** residual hardness assumption — the strongest possible
outcome for a first increment, and the one that most directly answers the
honesty mandate (a structural =0 reduction cannot be an "axiom dressed as a
proof").

### What was achieved: a REAL DISCHARGED REDUCTION (=0), not a faithful-game-with-obligation

The result is the stronger of the two acceptable outcomes. The break probability
is **proven equal to 0** by a genuine EasyCrypt program-logic argument over an
operational consume-once state machine driven by an **unbounded adaptive
adversary** — not asserted, not axiomatized.

#### The gap this closes vs. the existing Lean

`Path2TraceModel.lean` already discharges this break to probability 0 via
`breakProb_eq_zero_of_never` — but only after the concrete game *wires* the break
event to `false` **by construction** (`ConcreteBreakFor ... = False`, closed by
`decide_false`). That wiring is an assertion about the classifier, not a theorem
derived from the slot lifecycle's operational semantics. The spike's
`break_zero_of_never` likewise **took** the never-firing postcondition as a
hypothesis.

`break_slot_replay_or_rollback.ec` derives the never-firing postcondition itself:

- A stateful `Game` module models the consume-once lifecycle faithfully to
  `Path2FullSpendConcrete.ConcreteConsumeState` (a `consumed : digest fset`
  ledger + a monotone `barrier` height + a `break_fired` flag).
- `Game.spend (d, h)` is the **guarded** one-use transition: it accepts iff the
  digest is not already consumed (anti-replay) **and** the height is not below the
  monotone barrier (anti-rollback); on accept it records the digest and advances
  the barrier.
- The break flag is set by the **operational semantics**:
  `break_fired <- break_fired \/ (accept /\ (is_replay \/ is_rollback))` — a real
  bad-event read of live state, **not** a constant `false` (this respects the
  WS7 "MUST NOT inhabit by `True`/`False` fields" rule). Two `hoare` witness
  lemmas (`replay_rejected`, `rollback_rejected`) prove the guard genuinely
  rejects a replay attempt and a rollback attempt — the non-tautology evidence.

#### PROVEN (exit-0 EC, no `admit`, no `sorry`, no `axiom`; 6 `qed`s)

| Lemma | Statement | Discharged by |
|---|---|---|
| `replay_rejected` | a spend of an already-consumed digest is not accepted | `proc; auto; smt()` |
| `rollback_rejected` | a spend below the barrier is not accepted | `proc; auto; smt()` |
| `spend_preserves_nobreak` | one `spend` step preserves `! break_fired` | `proc; auto; smt()` (boolean contradiction: `accept ⇒ ¬is_replay ∧ ¬is_rollback`) |
| `spend_loop_never_breaks` | `hoare [ Exp(A).run : true ==> ! res ]` for any unbounded adaptive `A` | `while (! Game.break_fired)` invariant + `inline; auto; smt()` |
| `break_prob_zero` | `Pr[ Exp(A).run() @ &m : res ] = 0%r` | `byphoare` from the loop invariant |
| `break_prob_le_any` | `0 ≤ budget ⇒ Pr[…] ≤ budget` | rewrite by `break_prob_zero` |

#### ASSUMED (named hardness): **NONE**

There is **no** cryptographic hardness assumption and **no** axiom carrying the
bound. The only imports are the EasyCrypt standard library
(`AllCore Int Real Distr List FSet`). `grep -E '^\s*(admit|axiom )|sorry'` over
the file returns nothing. This is the correct boundary for a family-(c) event:
the consume-once lifecycle makes replay/rollback a **combinatorial
impossibility**, so the honest bound is `=0` with no standing assumption — and
that is exactly what is proven.

---

## Why this is the standard/correct boundary

A game-based proof discharges what the operational semantics forces and names
only genuine hardness. For `slotReplayOrRollback` the semantics force the bound
entirely (a guarded consume-once monotone state machine cannot exhibit
replay/rollback at all), so there is nothing left to assume. This is the EC
analogue — now *operationally derived* — of the Lean `breakProb_eq_zero_of_never`
discharge, and it matches `spec §5` family (c). The remaining binding/SIS/QROM
events are where named assumptions correctly enter; this one is not.

---

## Recommended order for the remaining 6

1. ~~**`postchallengeSlotCreationOrRewrite`**~~ — **DONE** (see "Event #2" section
   below). Discharged `=0`, zero assumptions, exit 0.
2. **`checkedCoreMessageBinding`** — first real *cryptographic* reduction: a clean
   reduction to a digest collision-resistance game (`theories/crypto/assumptions/CRHash.ec`
   is the template; `Path2FullSpendConcrete` already gives injective frame
   encoders). The reduction is provable; the CR bound stays a single named axiom.
3. **`prevalidationLeafBinding`**, then **`prevalidationRegistryBinding`** — both
   reduce to the same digest/Merkle CR game; do them after #2 reuses its harness.
4. **`creationTimeHiddenSource`** — also reduces to digest binding (C2), but
   couples a timing predicate; cheaper once #2/#3 fix the CR-game idiom.
5. **`prevalidationDataAvailability`** — last: it mixes a CR-binding obligation
   with a data-availability assumption (`ConcreteDirectCertificate.dataAvailable`),
   so it needs both the CR harness and an explicitly-named availability assumption.

Rationale: clear the two remaining structural `=0` events first (cheap, no
assumptions, reuse this template), then build the CR-game harness once on the
cleanest binding event and amortize it across the four binding/availability
events.

---

## Bottom line

- **Event chosen:** `slotReplayOrRollback`.
- **EC exit code:** 0.
- **PROVEN vs assumed:** 6 lemmas proven by EC program logic + Z3; **zero** named
  hardness assumptions, **zero** axioms, **zero** `admit`/`sorry`.
- **Outcome:** a **real discharged reduction** (probability **=0** from an
  operationally-derived consume-once invariant against an unbounded adaptive
  adversary) — the stronger outcome, not a faithful-game-with-obligation.

---

## Event #2 attempted: `postchallengeSlotCreationOrRewrite` (BreakEvent #5)

**Date:** 2026-06-10
**Artifact:** `formal/easycrypt/break_postchallenge_slot_creation_or_rewrite.ec`
— `easycrypt` **exit 0**.

### Why this event / what kind of proof

Second member of family (c) (state-machine / structural), the recommended next
step after `slotReplayOrRollback`. The event is adaptive post-challenge slot
manipulation: an adversary creating or rewriting a slot AFTER the spend challenge
is fixed, for the same spend. In the Lean concrete classifier
(`Path2FullSpendConcrete.ConcreteBreakFor`) this break is

```
postchallengeSlotCreationOrRewriteBreak := ¬ LeafFixedBeforeChallenge spend.leaf spend.spendChallengeTime
LeafFixedBeforeChallenge leaf t          :=  leaf.frozenAt ≤ t
```

i.e. the slot's creation/prevalidation freeze height `frozenAt` is post-challenge.

It admits a fully discharged `=0` because acceptance requires the prevalidation
handle to be **committed under the prechallenge account-state root**: the
abstract game's `AcceptedSpendConclusion` bundles `AdmittedPrechallengeLeaf`,
whose conjuncts include `directAdmissionContains … accountStateRootBefore …` and
`leafFixedBeforeChallenge leaf t` (= `frozenAt ≤ t`). So acceptance already
**entails** `frozenAt ≤ challengeTime`; the break fires only on
`accept ∧ frozenAt > challengeTime`, an unsatisfiable conjunction. The reason is
structural (the verifier admits the slot against the prechallenge root snapshot;
a post-challenge creation/rewrite lands in a later snapshot, never in the
committed prechallenge admission set), not cryptographic.

### Operational model (faithful, derived — not asserted)

`Game` holds a **fixed** challenge fence `challenge`, a **committed prechallenge
admission set** `admitted : (digest * int) fset`, and a `break_fired` flag. The
guarded `spend(d, frozenAt)` step:
- admits the handle into the committed set **only if** `frozenAt ≤ challenge`
  (prechallenge-fixation: a post-challenge freeze height never enters the
  prechallenge root snapshot);
- accepts iff `(d, frozenAt) ∈ admitted` (acceptance reads the committed set);
- sets `break_fired ⟵ break_fired ∨ (accept ∧ challenge < frozenAt)` — a live
  bad-event read, not a constant `false`.

`Exp(A).run(c)` drives an UNBOUNDED adaptive schedule through the lifecycle.

### PROVEN (exit-0 EC, no `admit`, no `sorry`, no `axiom`; 7 `qed`s)

| Lemma | Statement | Discharged by |
|---|---|---|
| `postcreate_rejected` | a not-yet-admitted post-challenge handle is not accepted on its own spend | `proc; auto; smt(in_fsetU in_fset1)` |
| `accept_implies_prechallenge` | under the committed-set wf invariant, a post-challenge spend is not accepted | `proc; auto; smt(in_fsetU in_fset1)` |
| `spend_preserves_admitted_wf` | one `spend` preserves "every admitted handle is prechallenge" | `proc; auto` + fset case split |
| `spend_preserves_nobreak` | one `spend` preserves `! break_fired ∧ wf` (only `accept ∧ postcreate` sets the flag; `accept` under wf entails `! postcreate`) | `proc; auto` + fset case split |
| `spend_loop_never_breaks` | `hoare [ Exp(A).run : true ==> ! res ]` for any unbounded adaptive `A` | `while (! break_fired ∧ wf)` invariant; `init` gives empty admission set |
| `break_prob_zero` | `Pr[ Exp(A).run(c) @ &m : res ] = 0%r` | `byphoare` from the loop invariant |
| `break_prob_le_any` | `0 ≤ budget ⇒ Pr[…] ≤ budget` | rewrite by `break_prob_zero` |

Note the small enrichment over the replay/rollback template: the no-break
invariant is **not stable alone** — it requires the auxiliary committed-set
well-formedness invariant (every admitted handle is prechallenge), which `init`
establishes (empty set) and `spend` maintains. The loop carries the conjunction.
This faithfully reflects that acceptance reading the prechallenge admission set is
what forces the bound.

### ASSUMED (named hardness): **NONE**

No cryptographic hardness assumption, no `axiom`. Only the EC standard library
(`AllCore Int Real Distr List FSet`) is imported.
`grep -E '^\s*axiom |(^|[^a-zA-Z_])(admit|sorry)\b'` over the file returns only
prose-comment hits (the word "admit" used as a verb), no real tactics/decls.

### Bottom line (Event #2)

- **Event:** `postchallengeSlotCreationOrRewrite`.
- **EC exit code:** 0.
- **PROVEN vs assumed:** 7 lemmas proven by EC program logic + Z3; **zero** named
  hardness assumptions, **zero** axioms, **zero** `admit`/`sorry`. A clean
  structural `=0`, not a reduction-to-named-assumption.

### Break events now remaining (5)

Both structural family-(c) events are now discharged. The remaining 5 are the
binding/availability/QROM family:
`checkedCoreMessageBinding`, `prevalidationLeafBinding`,
`prevalidationRegistryBinding`, `creationTimeHiddenSource`,
`prevalidationDataAvailability` — these are where named hardness assumptions
(digest/Merkle CR, data-availability) correctly enter, per the order above.
(`qromOrDomainSeparation` and `liveSlotLeakageOrKeygen` are wired to `False` in
the concrete classifier and are not part of the 7 cryptographically-charged set.)
