# WS7 — Concrete Reduction Scoping (AP2-N003)

Created: 2026-06-09. **Maintainer-led** (this is the largest remaining proof
piece and the maintainer is driving fail-closed → closed). This doc scopes the
work and applies the D2 (conservative) policy to the per-event sorting so the
reduction has a target; it does NOT itself construct the reduction (that would be
the very scaffolding-without-substance this program exists to avoid).

## The obligation (restated precisely)

Construct a concrete `PricedAcceptedSpendNumericCertificate` so that
```
P[full-signature forgery] ≤ P[accepted-spend] + Σ_18 P[break_i] ≤ pricedBound ≤ 2^-128
```
holds at the deployed query budget `T = 2^32`, replacing the currently
uninhabited / fail-closed certificate. The union-bound layer
(`Path2AcceptedSpendProbability.lean`) is already proven; what is missing is a
**concrete trace distribution** and **real per-event numeric bounds**.

## D2 per-event sorting (18 distinct break events — from the ledger)

Policy (conservative): **prove** structural/replay/domain-separation events;
leave only standard cryptographic primitives as **named-but-standard** assumptions
(QROM hash CR, the SIS/LWE family), each with margin.

| Break event | Recommend | Bound source |
|---|---|---|
| slotReplayOrRollback | **PROVE** | consume-once slot state machine (D2: must be proven, not assumed) |
| postchallengeSlotCreationOrRewrite | **PROVE** | state-transition soundness |
| checkedCoreMessageBinding | **PROVE** | digest binding (collision-resistance, → C2-style) |
| prevalidationLeafBinding / RegistryBinding / DataAvailability | **PROVE** | Merkle/registry binding → digest CR |
| boundaryBinding / innerCommitmentBinding / reviewedFq12PayloadBinding | **PROVE** | digest CR (C2) |
| reviewedFq12HiddenSourceSoundness | reduce → C2 | digest binding (ledger C2) |
| descentSoundness / transparentProofSoundness | reduce → SIS | B1/B2/B3 (ledger) |
| normOrRange | **PROVE** | exact bounded-kernel cert (#1, already proven: 96<114) |
| qromOrDomainSeparation / reviewedFq12DomainSeparation | NAME (standard) | QROM + domain-separation; price via 182-bit transcript term |
| liveSlotLeakageOrKeygen | NAME (standard, AP2-005 open) | keygen security — explicitly out of v1 closed set per AP2-005 |
| creationTimeHiddenSourceSoundness | reduce → C2 | digest binding |
| noExport | **PROVE** | AP2-007 serializer no-export (already instantiated) |
| accepted-spend numeric term | NAME (standard) | action collision soundness term 2^-99 @ T=2^32 (WS3) + transcript term |

Net: the **only** events that should remain *named* standard assumptions are
QROM/domain-separation, the action-collision term, keygen leakage (AP2-005), and
the SIS/LWE family — everything else is provable from binding/state machinery
already largely present.

## Composed-bound arithmetic target (from WS3)

With each term as a fixed rational, the final step is `norm_num`:
```
2^-99 (action, FS-gated @ T=2^32)  +  Σ digest-CR terms  +  Σ SIS terms
   +  QROM/domain-sep term (182-bit transcript)  ≤  2^-128 ?
```
**This does not currently close**: the action term alone is `2^-99 > 2^-128`.
The 182-bit transcript term must dominate/replace it in the accounting, OR the
query budget / action space must change. Resolving this is the crux of AP2-N003
and must be settled numerically before the certificate can be inhabited
honestly. (See WS3 findings, FLAGGED item 1.)

## What WS7 must NOT do

Do not inhabit the certificate by choosing per-event `Prop` fields that are
`True` or by an acceptor definitionally equal to the conclusion (the prior
`ConcreteSpendVerificationAccepts` pattern). Each per-event bound must be a real
inequality with a non-tautological witness, cross-checked against
`audit/lean_theorem_statements.txt`.

## Progress 2026-06-09 — closure skeleton proven (`Descender/Path2AcceptedSpendClosure.lean`)

Driven from fail-closed toward closed-modulo-named-assumptions. Five real,
`sorry`/axiom-free theorems (full build 3359 + axiom audit green):

1. `breakProb_eq_zero_of_never` — a break the model makes impossible contributes
   probability **0**. This is how the state-machine breaks (replay/rollback,
   post-challenge slot rewrite) are discharged once the concrete game wires them
   to `false` — proven, not assumed.
2. `success_implies_accepted_or_break_of_structural` — **derives** the numeric
   game's central hypothesis (`success_implies_accepted_or_break`) from the
   already-proven structural soundness `DirectAcceptedSpendSoundness`, given only
   modeling-faithfulness conditions. The key reduction step moves from assumed to
   proven.
3. `successProb_le_target_of_bounds` — explicit per-event bounds + a budget
   inequality ⟹ `successProb ≤ target` (composes the proven union-bound + pricing
   lemmas).
4. `activeBudget_closes` — the numeric budget **actually closes** at any target:
   accept ≤ B/2, each of 9 breaks ≤ B/32 ⟹ sum ≤ 25/32·B ≤ B. Real `Finset`/`linarith`.
5. `successProb_le_2pow_neg128_of_proportional_bounds` — the concrete **128-bit**
   endpoint: accept ≤ 2⁻¹²⁹ and each break ≤ 2⁻¹³³ ⟹ `successProb ≤ 2⁻¹²⁸`.

**What is now proven vs. what remains assumed.** Proven: the union bound, the
budget arithmetic at the 128-bit target, the zero-cost of impossible breaks, and
the structural→numeric bridge. Still required (the honest residual):

- a **faithful concrete game `G`** — a real finite trace model of the bounded
  EUF-CMA experiment tied to the actual verifier (the byte-for-byte bridge,
  AP2-006). This is the part the closure theorems take as input; building it is
  multi-week and partly external.
- the **per-event cryptographic bounds** as hypotheses to discharge: the
  accepted-spend forgery bound (SIS / response-link-coded hardness) and the
  QROM/hash/domain-separation break bounds. These are *correctly* explicit — a
  reduction never proves SIS/QROM unconditionally; the WS7 task is to prove each
  per-event bound (per the D2 sorting above) against `G`.

So the certificate is no longer just "fail-closed / uninhabited": there is a
proven skeleton with an explicit, named, standard-assumption interface. Closing
fully = supply `G` + discharge the per-event bounds into this skeleton.

## Handoff

This sorting + the WS3 verified per-term numbers are the inputs the maintainer's
reduction work consumes. The open numeric gap (action term vs 2^-128) is the
first thing to resolve, because it determines whether the current parameters can
carry 128-bit security at all or need adjustment.
