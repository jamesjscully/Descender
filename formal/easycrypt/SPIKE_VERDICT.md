# EC-SPIKE — go/no-go verdict (Phase 1 decision gate)

**Date:** 2026-06-09
**Tooling:** EasyCrypt 2026.05, Z3 4.16.0 (3 variants), why3 auto-detected.
**Artifact:** `formal/easycrypt/spike_union_bound.ec` — `easycrypt` **exit 0**.
**Method:** This verdict is written from the *actual* experience of driving the
one game hop (union bound + a single break-event reduction) to typecheck under an
installed EasyCrypt, against the bar fixed beforehand in
`spike_success_criteria.md`. It is not post-hoc.

---

## VERDICT: **GO (qualified)** — adopt EasyCrypt for the union-bound / closure spine.

The structural union-bound assembly is **markedly cleaner in EasyCrypt** than the
Lean closure skeleton, and it **typechecks as a real proof discharged by Z3**, not
by `admit`. The qualification: the *cryptographic seam* (the single-experiment
event coupling, OB-1/OB-2) is an explicit assumption on **both** sides and EC does
not make it cheaper — it is identical work. So the GO is specifically for the
*probability-assembly layer*, which is the part this spike measured.

---

## What actually typechecked (Z3-discharged) vs. what remains assumed

**Zero `admit.` tactics remain in the file.** Every lemma closes with `qed`. The
honest boundary is carried by named *axioms* (exactly the explicit hypotheses the
Lean file also leaves open), not by abandoned proof obligations.

| EC lemma | Status | Discharged by |
|---|---|---|
| `break_zero_of_never` (Goal 2 / OB-4) | **PROVEN** | `byphoare` + `hoare` + `conseq` — full Pr[..:res]=0 reduction |
| `success_le_target` (Goal 3) | **PROVEN** | `Bigreal.ler_sum` + `ler_add`/`ler_trans`, real arith closed by **Z3 (smt)** |
| `sum_break_le_card` (helper) | **PROVEN** | `Bigreal.sumr_const` + `count_predT` + `Bigreal.ler_sum` |
| `success_le_2pow_neg128` (Goal 3b) | **PROVEN** | explicit `2^129=2^128·2`, `2^133=2^128·32` expansions, then **Z3** closes 25/32 ≤ 1 |
| `success_le_union` (Goal 1) | proven *by `exact`* from the OB-2 import axiom | — |

**Z3-discharged real-arithmetic steps:** 3 (the two closure `smt` budget steps +
the `t3`-style `ler_add`/`ler_trans` combination inside `success_le_target`).
**Genuine `byphoare`/program-logic reduction:** 1 (`break_zero_of_never`).
**Remaining axioms (the cryptographic/Fintype seam, assumed on both sides):**
`success_implies_split` (OB-1/OB-2 coupling — the Lean↔EC spec seam), and the
`Fintype Break`/`card = 9` facts (`break_univ_uniq`, `break_univ_total`,
`break_card9`). In Lean these card facts are `decide`d; here they are primitive
axioms (a minor EC ergonomic loss, not a blocker).

---

## Head-to-head (EC vs Lean), measured

| | EasyCrypt spike | Lean (`Path2AcceptedSpendClosure.lean`) |
|---|---|---|
| Code lines (comments stripped) | **~92** | ~118 |
| Theorems/lemmas in the spine | 5 | 6 |
| Real-arithmetic budget step (25/32≤1) | `smt` / Z3, **1 tactic** after 3 power-expansion `have`s | `linarith` + `norm_num` |
| Finite-sum monotonicity (Σ Pr[b] ≤ Σ bound) | `Bigreal.ler_sum` (1 line) | `Finset.sum_le_sum` (1 line) |
| Constant-sum collapse (Σ c = n·c) | `Bigreal.sumr_const` + `count_predT` | `Finset.sum_const` + `nsmul_eq_mul` |
| `Pr[never] = 0` | `byphoare`+`hoare` (3 lines), **native to EC** | `Finset.sum_eq_zero` over a probability *definition* |

Both ecosystems have the finite-sum-over-events machinery; **neither side had to
hand-roll** finite-sum probability lemmas. That was the single biggest open
question in `spike_success_criteria.md` GO-criterion 2, and EC passes it:
`StdBigop.Bigreal.BRA` provides `big`, `ler_sum`, `sumr_const` ergonomically.

---

## Lemma drift I had to chase (the real cost of the spike)

The fix-up was dominated by **library-name lookup**, exactly as predicted. Count:

1. `op union_bound (&m : unit)` — **invalid**: `&m` memory cannot be an operator
   parameter (the original parse error at line 116). Removed; the decomposition
   lives in `success_implies_split` instead.
2. `ler_sum` is **not** top-level — it is `Bigreal.ler_sum` (cloned from
   `Bigalg.BigOrder` with `BAdd→BRA`), and its pointwise hypothesis form is
   `=> b _` (predicate side condition), not `=> b _ _`.
3. `ler_sum_const` (guessed in the skeleton) **does not exist**; the constant
   bound is `Bigreal.sumr_const` + `count_predT` (giving `(count predT s)%r * c`).
4. `smt(hLocalHypothesis)` **does not accept local hypotheses by name** in
   2026.05 — had to `move: h1 h2; smt()` to push them into the goal first. This
   bit twice and is a genuine ergonomic papercut.
5. `2%r ^ 129` etc. are symbolic `RField.exp`; **Z3 will not expand them**. Had to
   feed explicit `exprD_nneg`-based exponent relations (`2^129=2^128·2`,
   `2^133=2^128·32`) plus `RealOrder.expr_gt0` positivity. Lean's `norm_num` does
   this for free — a clear point **in Lean's favour**.
6. `expr_gt0` lives under `RealOrder` (StdOrder), needs qualification.

Net: ~6 name/idiom corrections + 1 syntax fix to reach exit 0. No missing
mathematics — every lemma I needed existed; the cost was *finding* it.

---

## Where EC's automation helped / hurt / vs Lean

- **Helped:** `byphoare`→`hoare` makes `Pr[never]=0` more *direct* than Lean,
  where the same fact goes through unfolding the `breakProb`/`acceptProb`
  *definition* and `Finset.sum_eq_zero`. The probability layer is first-class in
  EC; in Lean it is a derived `FiniteProbability` construction.
- **Hurt:** symbolic-power arithmetic (papercut 5) and `smt`-hypothesis passing
  (papercut 4). Both are recoverable with a one-time idiom, but they are exactly
  the friction Lean's `norm_num`/`linarith`/named-hypothesis `simp` avoid.
- **Wash:** finite-sum monotonicity and constant-sum — equally ergonomic both
  sides once the right name is known.

---

## Decision rationale

GO-criteria from `spike_success_criteria.md`:
1. OB-3/OB-4 discharge with effort ≤ Lean — **met** (fewer LoC, `byphoare` is
   cleaner than the Lean definition-unfold).
2. `big`/`Distr` finite machinery exists and is ergonomic — **met**.
3. Closure lemmas close by `smt`/Z3 — **met** (with the power-expansion caveat).
4. Someone owns the `.ec` — organizational, out of scope here.

NO-GO triggers:
1. Awkward math support — **not triggered**; nothing was missing.
2. QROM break terms need research-level scaffolding — **NOT EXERCISED by this
   spike.** The single break-event reduction we proved (`break_zero_of_never`) is
   the *consume-once state-machine* break, which needs **no** QROM model — it is a
   plain `hoare` postcondition. The QROM-modelled breaks remain the hard,
   unmeasured part and are the reason this GO is *qualified*.
3. Coupling (OB-1) can't be stated cleanly — **partially triggered.** We did not
   build the single-experiment coupling; we axiomatized it. The spike used 3
   separate procedures, so OB-1 is deferred. This is the one place the Lean↔EC
   seam is *not* shown to be narrow.

---

## THE SINGLE MOST IMPORTANT FACTOR

**The union-bound assembly — the part that is pure recurring proof labor — is
cleaner and shorter in EasyCrypt and Z3 closes its real-arithmetic spine, while
the hard part (the structural/QROM coupling) is an explicit assumption on both
sides.** EasyCrypt does not make the cryptographic seam cheaper, but it makes the
*surrounding probability bookkeeping* cheaper. Since Phase 2's recurring cost is
the bookkeeping (the seam is paid once), **GO for the EC reduction is the right
call — provided the team is willing to absorb the QROM-modelling risk, which this
spike did not exercise.**

## Recommendation

- **Adopt EasyCrypt for the union-bound + numeric-closure layer** (Goals 1–3b).
  Port `successProb_le_target_of_bounds` / `…_2pow_neg128` and
  `breakProb_eq_zero_of_never` to `.ec` as the canonical reduction skeleton.
- **Before committing Phase 2 fully to EC, run a second, narrower spike on OB-1**
  (collapse the 3 procedures into one experiment and prove the event implication
  by `byequiv`) and on **one QROM-modelled break**. Those are the two unmeasured
  risks; if either is a slog, keep *those* events in Lean and use EC only for the
  assembly. The probability-assembly verdict here is solid; the crypto-seam
  verdict is still open.
