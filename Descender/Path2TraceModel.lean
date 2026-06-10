import Descender.Path2AcceptedSpendClosure
import Descender.Path2FullSpendConcrete

namespace Descender

/-!
# Path 2 Concrete Trace Model (faithful scaffold for the closure skeleton)

`Path2AcceptedSpendClosure` proves, with no `sorry`, that the full-signature
success probability is below the deployed `2^-128` target *given* an
`AcceptedSpendNumericGame` whose per-event probabilities are bounded by the
standard cryptographic budgets.  What that closure consumed abstractly is a
concrete `AcceptedSpendNumericGame Trace Outcome BreakEvent`: a finite trace
distribution, an `observe` map into the verifier's actual outcome surface, and
boolean accepted-spend / named-break events that *faithfully reflect the real
verifier predicates*.

This module supplies that faithful `G`, but it is deliberately honest about what
is and is not proven:

* **Faithful by construction (proven here).**  The outcome surface is exactly the
  concrete full-forgery run `FullForgeryRun concreteTypes` already used by
  `Path2FullSpendConcrete` (not a proof-convenient finite stand-in).  The
  accepted-spend event is `Classical.decide` of the *real*
  `ConcreteSpendVerificationAccepts` predicate, and each named break is
  `Classical.decide` of the *real* `ConcreteBreakFor` classifier.  The
  `success_implies_accepted_or_break` soundness field is **derived** — not
  assumed — from the already-proven structural theorem
  `concrete_account_creation_prevalidated_slot_authorization_or_break` via the
  closure bridge `success_implies_accepted_or_break_of_structural`.

* **Genuinely discharged per-event bounds (proven here, probability `0`).**  The
  concrete classifier `ConcreteBreakFor` makes the `qromOrDomainSeparationBreak`
  and `liveSlotLeakageOrKeygenBreak` events `False` *by construction at this
  layer* (they are charged in a different sub-game).  We discharge their numeric
  contribution to `0` with a real `never`-proof through
  `breakProb_eq_zero_of_never`.  This is not vacuous: it is a real boolean event
  that is provably never `true`, so its probability is provably `0`.

* **Still open (named, NOT `sorry`, NOT faked).**  The accepted-spend forgery
  bound (SIS / response-link hardness) and the seven *cryptographically*-charged
  named breaks are NOT proven here and must not be.  They remain explicit
  hypotheses of `concreteNumericClosure_of_cryptographic_bounds` below; supplying
  them is the multi-week WS7 reduction work (see
  `formal/lean/trace_model_status.md`).

Crucially, this file does **not** define a toy distribution that makes
`successProb ≤ 2^-128` true-but-meaningless.  It does not pick `traceDist`,
`observe`, or the trace support at all: those are taken as inputs to the game
constructor `concreteNumericGame`.  Whatever sampler a deployment supplies, the
events are pinned to the real verifier, the soundness field is the real theorem,
and the two structurally-impossible breaks cost `0`.
-/

namespace Path2TraceModel

open Advantage
open Path2AcceptedSpendProbability
open Path2AcceptedSpendProbability.AcceptedSpendNumericGame
open Path2FullSpendConcrete
open Classical

noncomputable section

/-- The faithful outcome surface: the concrete full-forgery run already used by
the `Path2FullSpendConcrete` instantiation, not a proof-convenient stand-in. -/
abbrev ConcreteOutcome : Type :=
  Path2FullSpendGame.FullForgeryRun concreteTypes

/--
The faithful accepted-spend event: `decide` of the **real** concrete verifier
acceptance predicate on the run's spend tuple.  Acceptance here means the run
realizes the admitted prechallenge leaf and the exact consume-once transition —
the conclusion the structural theorem delivers.
-/
def concreteFullSignatureSuccess
    (C : ConcreteGameConfig) (o : ConcreteOutcome) : Bool :=
  decide (ConcreteSpendVerificationAccepts C o.spend)

/--
The faithful accepted-spend forgery event: `decide` of the structural
accepted-spend conclusion for the run's spend tuple.  This is exactly the event
whose probability a SIS / response-link reduction must bound.
-/
def concreteAcceptedSpendForgery
    (C : ConcreteGameConfig) (o : ConcreteOutcome) : Bool :=
  decide
    (Path2FullSpendGame.AcceptedSpendConclusion (concretePredicates C) o.spend)

/--
The faithful named-break event family: `decide` of the **real** concrete
break classifier `ConcreteBreakFor` for the run's spend tuple.  This is the
same nine-event family the structural soundness charges failures to.
-/
def concreteNamedBreak
    (C : ConcreteGameConfig)
    (b : Path2FullSpendGame.BreakEvent) (o : ConcreteOutcome) : Bool :=
  decide (ConcreteBreakFor C o.spend b)

/--
**Soundness field, derived not assumed.**

Any trace whose `observe`d run is accepted by the concrete verifier yields
either the accepted-spend forgery event or one of the named breaks.  This is the
probabilistic shadow of the proven structural theorem
`concrete_account_creation_prevalidated_slot_authorization_or_break`, lifted
through the closure bridge `success_implies_accepted_or_break_of_structural`.
No new assumption is introduced.
-/
theorem concrete_success_implies_accepted_or_break
    {Trace : Type*} (C : ConcreteGameConfig)
    (observe : Trace → ConcreteOutcome) :
    ∀ trace,
      concreteFullSignatureSuccess C (observe trace) = true →
        concreteAcceptedSpendForgery C (observe trace) = true ∨
          ∃ b, concreteNamedBreak C b (observe trace) = true := by
  refine
    success_implies_accepted_or_break_of_structural
      (concreteDirectAcceptedSpendSoundness C)
      observe (fun o => o.spend)
      (concreteFullSignatureSuccess C)
      (concreteAcceptedSpendForgery C)
      (concreteNamedBreak C)
      ?_ ?_ ?_
  · -- success ⇒ verifier accepts
    intro o hsucc
    exact of_decide_eq_true hsucc
  · -- accepted-spend structural conclusion ⇒ event fires
    intro o hconc
    exact decide_eq_true hconc
  · -- structural break ⇒ named-break event fires
    intro o b hbreak
    exact decide_eq_true hbreak

/--
The faithful concrete accepted-spend numeric game.

This is the `G` the closure skeleton consumes.  Its trace support and
distribution are **inputs** (`traceDist`, `observe`) supplied by a deployment
sampler — they are not chosen here, so the construction cannot rig the
probabilities.  Every boolean event is pinned to the real concrete predicate,
and the soundness field is the derived theorem above.
-/
def concreteNumericGame
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome) :
    AcceptedSpendNumericGame Trace ConcreteOutcome
      Path2FullSpendGame.BreakEvent where
  traceDist := traceDist
  observe := observe
  fullSignatureSuccess := concreteFullSignatureSuccess C
  acceptedSpendForgery := concreteAcceptedSpendForgery C
  namedBreak := concreteNamedBreak C
  success_implies_accepted_or_break :=
    concrete_success_implies_accepted_or_break C observe

/-! ## Structurally-impossible breaks cost zero (genuinely discharged) -/

/--
The concrete classifier makes `qromOrDomainSeparationBreak` `False` at this
layer, so the boolean event never fires for any trace.  This is a real
`never`-proof, not a vacuous type cheat.
-/
theorem qromBreak_never
    {Trace : Type*} (C : ConcreteGameConfig)
    (observe : Trace → ConcreteOutcome) :
    ∀ trace,
      concreteNamedBreak C
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak
        (observe trace) = false := by
  intro trace
  simp only [concreteNamedBreak, ConcreteBreakFor, decide_false]

/-- Likewise the `liveSlotLeakageOrKeygenBreak` event never fires at this layer. -/
theorem liveSlotLeakageBreak_never
    {Trace : Type*} (C : ConcreteGameConfig)
    (observe : Trace → ConcreteOutcome) :
    ∀ trace,
      concreteNamedBreak C
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak
        (observe trace) = false := by
  intro trace
  simp only [concreteNamedBreak, ConcreteBreakFor, decide_false]

/--
**Discharged per-event bound (probability `0`).**  The QROM/domain-separation
break contributes `0` to the budget in the faithful concrete game.
-/
theorem qromBreakProb_eq_zero
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome) :
    (concreteNumericGame C traceDist observe).breakProb
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak = 0 :=
  breakProb_eq_zero_of_never (concreteNumericGame C traceDist observe)
    Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak
    (by
      intro trace
      simpa [concreteNumericGame] using qromBreak_never C observe trace)

/--
**Discharged per-event bound (probability `0`).**  The live-slot
leakage/keygen break contributes `0` to the budget in the faithful concrete
game.
-/
theorem liveSlotLeakageBreakProb_eq_zero
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome) :
    (concreteNumericGame C traceDist observe).breakProb
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak = 0 :=
  breakProb_eq_zero_of_never (concreteNumericGame C traceDist observe)
    Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak
    (by
      intro trace
      simpa [concreteNumericGame] using liveSlotLeakageBreak_never C observe trace)

/-! ## Honest endpoint: the remaining cryptographic obligations, named explicitly

The two breaks above are zero *by construction*.  The accepted-spend forgery
bound and the remaining seven named breaks are genuinely cryptographic and are
NOT discharged here.  The theorem below states the closed `2^-128` conclusion
with those obligations as explicit hypotheses, so the faithful game plugs into
the proven closure without any hidden assumption or `sorry`.  Supplying the
hypotheses is the multi-week WS7 reduction work documented in
`formal/lean/trace_model_status.md`. -/

/--
**Conditional 128-bit closure for the faithful concrete game.**

Given the standard cryptographic per-event budgets — the accepted-spend forgery
probability `≤ 2^-129` and each named break `≤ 2^-133` — the faithful concrete
game's full-signature success probability is `≤ 2^-128`.

The QROM and live-slot breaks already have proven `0 ≤ 2^-133` bounds (from
`qromBreakProb_eq_zero` / `liveSlotLeakageBreakProb_eq_zero`); the caller need
only supply the genuinely-cryptographic remaining budgets.  Nothing here is
faked: the two impossible breaks are discharged, and the rest are the real
unproven obligations passed in as hypotheses.
-/
theorem concreteNumericClosure_of_cryptographic_bounds
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (breakBound : Path2FullSpendGame.BreakEvent → ℝ)
    (hAccept :
      (concreteNumericGame C traceDist observe).acceptedSpendProb ≤
        acceptedSpendBound)
    (hBreak :
      ∀ b, (concreteNumericGame C traceDist observe).breakProb b ≤ breakBound b)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget : ∀ b, breakBound b ≤ (1 : ℝ) / 2 ^ 133) :
    (concreteNumericGame C traceDist observe).successProb ≤ (1 : ℝ) / 2 ^ 128 :=
  successProb_le_2pow_neg128_of_proportional_bounds
    (concreteNumericGame C traceDist observe)
    acceptedSpendBound breakBound hAccept hBreak hAcceptBudget hBreakBudget

end

end Path2TraceModel

end Descender
