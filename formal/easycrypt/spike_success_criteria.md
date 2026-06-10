# EC-SPIKE — success criteria (written BEFORE the machine run)

Per `../VERIFICATION_PLAN.md` Phase 1, the spike's job is a **go/no-go signal**
on "EasyCrypt vs Lean for the reduction", from ONE game hop. This file fixes the
bar *before* an installed EasyCrypt run, so the verdict is not post-hoc.

## The one game hop (scope)

The hop is the union-bound step + a single break-event reduction, exactly
mirroring the proven Lean theorems:

| Lean (proven, no `sorry`)                                  | EC image (`spike_union_bound.ec`) |
|------------------------------------------------------------|-----------------------------------|
| `successProb_le_acceptedSpendOrBreakUnionBound`            | `success_le_union`                |
| `breakProb_eq_zero_of_never`                               | `break_zero_of_never`             |
| `successProb_le_target_of_bounds`                          | `success_le_target`               |
| `successProb_le_2pow_neg128_of_proportional_bounds`        | `success_le_2pow_neg128`          |
| `acceptedSpendOrBreakUnionBound_eq_...add_breakProb_sum`   | `op union_bound` decomposition    |

## Obligations the installed run must discharge (the `admit`s/axioms)

- **OB-1** Single-experiment coupling: collapse `success/accepted/break_` into
  one experiment and prove the event implication
  `success => accepted \/ \big_or_b break b` (`byequiv`/pRHL).
- **OB-2** `success_implies_split` for a concrete `G` — the structural-soundness
  import. In Lean this is *derived*
  (`success_implies_accepted_or_break_of_structural` from
  `DirectAcceptedSpendSoundness`); in EC it must be re-derived against the
  experiment. **This is Seam 1 (Lean↔EC spec correspondence)** named in
  `README.md` — it stays a named assumption either way.
- **OB-3** `success_le_union`: finite union bound over `break_univ`
  (`Distr.mu_or` iterated via `StdBigop.Bigreal.BRA.big_split`).
- **OB-4** `break_zero_of_never`: `byphoare` from the consume-once hoare
  invariant (the state-machine replay/rollback discharge).

The two closure lemmas are pure real arithmetic over OB-1..OB-3 and are expected
to close via z3 immediately (they are the EC image of the Lean
`calc`/`linarith`).

## GO criteria (adopt EasyCrypt for the reduction)

1. OB-1, OB-3, OB-4 discharge in EC with effort **comparable to or below** the
   Lean equivalents (the Lean union-bound + closure is ~210 lines, fully proven).
2. The finite `big`/`Distr` machinery (`BRA.big_split`, `mu_or`) exists and is
   ergonomic — no need to hand-roll finite-sum probability lemmas.
3. The two closure lemmas close by `smt` with z3 (real-arithmetic budget step).
4. Someone on the team will own/maintain `.ec`.

## NO-GO criteria (keep the reduction in Lean)

1. Missing/awkward math support (field/lattice/real-analysis) makes OB-1/OB-3 a
   slog — i.e. EC's probability-over-finite-families story is *worse* than the
   `FiniteProbability` + `Finset.sum` layer already proven in Lean.
2. The QROM break terms (the hardest, per `README.md`) have no tractable EC
   model — note EC QROM support is research-level; if even the *single*
   break-event reduction (OB-4) needs QROM scaffolding, that is a strong NO-GO
   signal for now.
3. The coupling (OB-1) cannot be stated cleanly against the existing reference
   semantics → the Lean↔EC seam is wider than the Lean structural proof.

## Current honest status

**PREPARED, BLOCKED ON TOOL INSTALL.** The `.ec` skeleton mirrors the Lean
theorem structure line-for-line (table above), but EasyCrypt/opam are absent
(`../easycrypt/INSTALL.md`), so **nothing here is machine-checked**. The go/no-go
verdict cannot be issued until OB-1..OB-4 are attempted in an installed EC.
