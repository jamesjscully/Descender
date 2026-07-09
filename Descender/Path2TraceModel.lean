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

/--
Concrete 128-bit closure with the two structurally-impossible breaks discharged
inside the trace model.

The caller supplies cryptographic probability bounds only for the remaining
named breaks.  The QROM/domain-separation and live-slot/keygen events are
handled here by their proven zero-probability facts, plus nonnegative component
budgets for their slots.
-/
theorem concreteNumericClosure_of_remaining_cryptographic_bounds
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
      ∀ b,
        b ≠ Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak →
        b ≠ Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak →
          (concreteNumericGame C traceDist observe).breakProb b ≤
            breakBound b)
    (hQromBreakBound_nonneg :
      0 ≤ breakBound
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak)
    (hLiveSlotBreakBound_nonneg :
      0 ≤ breakBound
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget : ∀ b, breakBound b ≤ (1 : ℝ) / 2 ^ 133) :
    (concreteNumericGame C traceDist observe).successProb ≤ (1 : ℝ) / 2 ^ 128 := by
  refine
    concreteNumericClosure_of_cryptographic_bounds
      C traceDist observe acceptedSpendBound breakBound hAccept ?_
      hAcceptBudget hBreakBudget
  intro b
  by_cases hQrom :
      b = Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak
  · subst b
    rw [qromBreakProb_eq_zero]
    exact hQromBreakBound_nonneg
  · by_cases hLive :
        b = Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak
    · subst b
      rw [liveSlotLeakageBreakProb_eq_zero]
      exact hLiveSlotBreakBound_nonneg
    · exact hBreak b hQrom hLive

/-- Break events that remain cryptographically priced by the caller. -/
def IsRemainingCryptographicBreak
    (breakEvent : Path2FullSpendGame.BreakEvent) : Prop :=
  breakEvent ≠
      Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak ∧
    breakEvent ≠
      Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak

/--
Subtype of named breaks whose probability still needs an external
cryptographic bound after the trace model discharges the two impossible
structural events.
-/
abbrev RemainingCryptographicBreak : Type :=
  { breakEvent : Path2FullSpendGame.BreakEvent //
    IsRemainingCryptographicBreak breakEvent }

/-- Exact finite set of named breaks still priced by cryptographic evidence. -/
def remainingCryptographicBreakEvents :
    Finset Path2FullSpendGame.BreakEvent :=
  { Path2FullSpendGame.BreakEvent.creationTimeHiddenSourceSoundnessBreak,
    Path2FullSpendGame.BreakEvent.prevalidationLeafBindingBreak,
    Path2FullSpendGame.BreakEvent.prevalidationRegistryBindingBreak,
    Path2FullSpendGame.BreakEvent.prevalidationDataAvailabilityBreak,
    Path2FullSpendGame.BreakEvent.postchallengeSlotCreationOrRewriteBreak,
    Path2FullSpendGame.BreakEvent.checkedCoreMessageBindingBreak,
    Path2FullSpendGame.BreakEvent.slotReplayOrRollbackBreak }

/--
The explicit remaining-break set is exactly the subtype predicate used by the
pricing package: the only excluded finite events are the two structurally-zero
slots discharged in this trace model.
-/
theorem mem_remainingCryptographicBreakEvents_iff
    (breakEvent : Path2FullSpendGame.BreakEvent) :
    breakEvent ∈ remainingCryptographicBreakEvents ↔
      IsRemainingCryptographicBreak breakEvent := by
  cases breakEvent <;>
    simp [remainingCryptographicBreakEvents, IsRemainingCryptographicBreak]

/-- The active remaining cryptographic break surface has seven events. -/
theorem remainingCryptographicBreakEvents_card :
    remainingCryptographicBreakEvents.card = 7 := by
  native_decide

/-- Every subtype witness projects into the explicit remaining-break set. -/
theorem remainingCryptographicBreak_mem_remainingCryptographicBreakEvents
    (breakEvent : RemainingCryptographicBreak) :
    breakEvent.1 ∈ remainingCryptographicBreakEvents :=
  (mem_remainingCryptographicBreakEvents_iff breakEvent.1).2 breakEvent.2

/-- The remaining cryptographic break subtype has seven witnesses. -/
theorem remainingCryptographicBreak_card :
    Fintype.card RemainingCryptographicBreak = 7 := by
  rw [Fintype.card_subtype]
  have hEvents :
      ({ breakEvent | IsRemainingCryptographicBreak breakEvent } :
          Finset Path2FullSpendGame.BreakEvent) =
        remainingCryptographicBreakEvents := by
    ext breakEvent
    simpa using (mem_remainingCryptographicBreakEvents_iff breakEvent).symm
  rw [hEvents]
  exact remainingCryptographicBreakEvents_card

/--
Finite WS7 closure obligations left after the trace model discharges the two
structurally impossible break events: the accepted-spend bound itself, plus one
bound for each remaining cryptographic break.
-/
inductive WS7RemainingClosureObligation
  | acceptedSpendForgeryBound
  | remainingBreakBound (breakEvent : RemainingCryptographicBreak)
  deriving DecidableEq, Fintype

/-- The accepted-spend component of the WS7 remaining-obligation surface. -/
def ws7AcceptedSpendClosureObligations :
    Finset WS7RemainingClosureObligation :=
  { WS7RemainingClosureObligation.acceptedSpendForgeryBound }

/-- The seven per-break components of the WS7 remaining-obligation surface. -/
def ws7RemainingBreakBoundObligations :
    Finset WS7RemainingClosureObligation :=
  Finset.univ.image WS7RemainingClosureObligation.remainingBreakBound

/-- All WS7 remaining closure obligations. -/
def ws7RemainingClosureObligations :
    Finset WS7RemainingClosureObligation :=
  ws7AcceptedSpendClosureObligations ∪
    ws7RemainingBreakBoundObligations

/-- Predicate for the accepted-spend WS7 closure obligation. -/
def IsWS7AcceptedSpendClosureObligation
    (obligation : WS7RemainingClosureObligation) : Prop :=
  obligation = WS7RemainingClosureObligation.acceptedSpendForgeryBound

/-- Predicate for per-break WS7 closure obligations. -/
def IsWS7RemainingBreakClosureObligation
    (obligation : WS7RemainingClosureObligation) : Prop :=
  ∃ breakEvent : RemainingCryptographicBreak,
    obligation =
      WS7RemainingClosureObligation.remainingBreakBound breakEvent

/-- Predicate for the complete WS7 remaining-obligation surface. -/
def IsWS7RemainingClosureObligation
    (obligation : WS7RemainingClosureObligation) : Prop :=
  IsWS7AcceptedSpendClosureObligation obligation ∨
    IsWS7RemainingBreakClosureObligation obligation

/-- The singleton accepted-spend obligation set matches its predicate. -/
theorem mem_ws7AcceptedSpendClosureObligations_iff
    (obligation : WS7RemainingClosureObligation) :
    obligation ∈ ws7AcceptedSpendClosureObligations ↔
      IsWS7AcceptedSpendClosureObligation obligation := by
  cases obligation <;>
    simp [ws7AcceptedSpendClosureObligations,
      IsWS7AcceptedSpendClosureObligation]

/-- The remaining-break obligation set matches the seven-break subtype. -/
theorem mem_ws7RemainingBreakBoundObligations_iff
    (obligation : WS7RemainingClosureObligation) :
    obligation ∈ ws7RemainingBreakBoundObligations ↔
      IsWS7RemainingBreakClosureObligation obligation := by
  cases obligation <;>
    simp [ws7RemainingBreakBoundObligations,
      IsWS7RemainingBreakClosureObligation]

/-- The accepted-spend part contributes one WS7 closure obligation. -/
theorem ws7AcceptedSpendClosureObligations_card :
    ws7AcceptedSpendClosureObligations.card = 1 := by
  native_decide

/-- The remaining-break part contributes seven WS7 closure obligations. -/
theorem ws7RemainingBreakBoundObligations_card :
    ws7RemainingBreakBoundObligations.card = 7 := by
  have hInjective :
      Function.Injective
        WS7RemainingClosureObligation.remainingBreakBound := by
    intro left right h
    cases h
    rfl
  rw [ws7RemainingBreakBoundObligations,
    Finset.card_image_of_injective Finset.univ hInjective,
    Finset.card_univ, remainingCryptographicBreak_card]

/-- The complete WS7 remaining-obligation set matches its predicate. -/
theorem mem_ws7RemainingClosureObligations_iff
    (obligation : WS7RemainingClosureObligation) :
    obligation ∈ ws7RemainingClosureObligations ↔
      IsWS7RemainingClosureObligation obligation := by
  cases obligation <;>
    simp [ws7RemainingClosureObligations,
      mem_ws7AcceptedSpendClosureObligations_iff,
      mem_ws7RemainingBreakBoundObligations_iff,
      IsWS7RemainingClosureObligation]

/-- The complete WS7 remaining-obligation surface has eight rows. -/
theorem ws7RemainingClosureObligations_card :
    ws7RemainingClosureObligations.card = 8 := by
  have hDisjoint :
      Disjoint ws7AcceptedSpendClosureObligations
        ws7RemainingBreakBoundObligations := by
    rw [Finset.disjoint_left]
    intro obligation hAccepted hRemaining
    have hAcceptedPred :=
      (mem_ws7AcceptedSpendClosureObligations_iff obligation).1
        hAccepted
    have hRemainingPred :=
      (mem_ws7RemainingBreakBoundObligations_iff obligation).1
        hRemaining
    rcases hRemainingPred with ⟨breakEvent, hBreak⟩
    cases hAcceptedPred
    cases hBreak
  rw [ws7RemainingClosureObligations,
    Finset.card_union_of_disjoint hDisjoint,
    ws7AcceptedSpendClosureObligations_card,
    ws7RemainingBreakBoundObligations_card]

/-- No WS7 remaining closure obligation is both accepted-spend and per-break. -/
theorem no_ws7RemainingClosureObligation_accepted_and_remaining
    (obligation : WS7RemainingClosureObligation) :
    ¬ (obligation ∈ ws7AcceptedSpendClosureObligations ∧
        obligation ∈ ws7RemainingBreakBoundObligations) := by
  intro h
  have hAccepted :=
    (mem_ws7AcceptedSpendClosureObligations_iff obligation).1 h.1
  have hRemaining :=
    (mem_ws7RemainingBreakBoundObligations_iff obligation).1 h.2
  rcases hRemaining with ⟨breakEvent, hBreak⟩
  cases hAccepted
  cases hBreak

/--
Selected WS7 obligation semantics for the faithful trace model.  The accepted
spend row carries its probability bound and 2^-129 component budget; each
remaining-break row carries its break-probability bound and 2^-133 component
budget.
-/
def SelectedRustWS7RemainingClosureObligationSatisfied
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ) :
    WS7RemainingClosureObligation → Prop
  | WS7RemainingClosureObligation.acceptedSpendForgeryBound =>
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound ∧
        acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129
  | WS7RemainingClosureObligation.remainingBreakBound breakEvent =>
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent ∧
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133

/--
One selected WS7 evidence object supplies exactly the eight remaining closure
rows: one accepted-spend row and one row for each remaining cryptographic break.
-/
structure SelectedRustWS7RemainingClosureEvidence
    (Trace : Type) [traceFintype : Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome) where
  acceptedSpendBound : ℝ
  remainingBreakBound : RemainingCryptographicBreak → ℝ
  obligationSatisfied :
    ∀ obligation : WS7RemainingClosureObligation,
      obligation ∈ ws7RemainingClosureObligations →
        SelectedRustWS7RemainingClosureObligationSatisfied
          traceDist observe acceptedSpendBound remainingBreakBound obligation

namespace SelectedRustWS7RemainingClosureEvidence

/-- The WS7 accepted-spend row gives the accepted-spend probability bound. -/
theorem acceptedSpendProb_le_bound
    {Trace : Type} [Fintype Trace]
    {traceDist : FiniteProbability Trace}
    {observe : Trace → ConcreteOutcome}
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    (concreteNumericGame selectedRustConcreteGameConfig traceDist
        observe).acceptedSpendProb ≤ E.acceptedSpendBound := by
  have hMem :
      WS7RemainingClosureObligation.acceptedSpendForgeryBound ∈
        ws7RemainingClosureObligations :=
    (mem_ws7RemainingClosureObligations_iff _).2 (Or.inl rfl)
  exact
    (E.obligationSatisfied
      WS7RemainingClosureObligation.acceptedSpendForgeryBound hMem).1

/-- The WS7 accepted-spend row gives the 2^-129 component budget. -/
theorem acceptedSpendBound_le_2pow_neg129
    {Trace : Type} [Fintype Trace]
    {traceDist : FiniteProbability Trace}
    {observe : Trace → ConcreteOutcome}
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    E.acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129 := by
  have hMem :
      WS7RemainingClosureObligation.acceptedSpendForgeryBound ∈
        ws7RemainingClosureObligations :=
    (mem_ws7RemainingClosureObligations_iff _).2 (Or.inl rfl)
  exact
    (E.obligationSatisfied
      WS7RemainingClosureObligation.acceptedSpendForgeryBound hMem).2

/-- Each WS7 remaining-break row gives the corresponding break-probability bound. -/
theorem remainingBreakProb_le_bound
    {Trace : Type} [Fintype Trace]
    {traceDist : FiniteProbability Trace}
    {observe : Trace → ConcreteOutcome}
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (breakEvent : RemainingCryptographicBreak) :
    (concreteNumericGame selectedRustConcreteGameConfig traceDist
        observe).breakProb breakEvent.1 ≤
      E.remainingBreakBound breakEvent := by
  have hMem :
      WS7RemainingClosureObligation.remainingBreakBound breakEvent ∈
        ws7RemainingClosureObligations :=
    (mem_ws7RemainingClosureObligations_iff _).2
      (Or.inr ⟨breakEvent, rfl⟩)
  exact
    (E.obligationSatisfied
      (WS7RemainingClosureObligation.remainingBreakBound breakEvent) hMem).1

/-- Each WS7 remaining-break row gives the corresponding 2^-133 component budget. -/
theorem remainingBreakBound_le_2pow_neg133
    {Trace : Type} [Fintype Trace]
    {traceDist : FiniteProbability Trace}
    {observe : Trace → ConcreteOutcome}
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (breakEvent : RemainingCryptographicBreak) :
    E.remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133 := by
  have hMem :
      WS7RemainingClosureObligation.remainingBreakBound breakEvent ∈
        ws7RemainingClosureObligations :=
    (mem_ws7RemainingClosureObligations_iff _).2
      (Or.inr ⟨breakEvent, rfl⟩)
  exact
    (E.obligationSatisfied
      (WS7RemainingClosureObligation.remainingBreakBound breakEvent) hMem).2

end SelectedRustWS7RemainingClosureEvidence

/--
Extend caller-supplied remaining-break bounds to the full active break family
by assigning zero budget to the structurally impossible events.
-/
def remainingCryptographicBreakBound
    (remainingBreakBound : RemainingCryptographicBreak → ℝ) :
    Path2FullSpendGame.BreakEvent → ℝ :=
  fun breakEvent =>
    if hQrom :
        breakEvent =
          Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak then
      0
    else if hLiveSlot :
        breakEvent =
          Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak then
      0
    else
      remainingBreakBound
        ⟨breakEvent, ⟨hQrom, hLiveSlot⟩⟩

/--
The synthesized full-family break-bound map is below the active per-break
target whenever every remaining cryptographic break is below that target.
-/
theorem remainingCryptographicBreakBound_le_2pow_neg133
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    ∀ breakEvent : Path2FullSpendGame.BreakEvent,
      remainingCryptographicBreakBound remainingBreakBound breakEvent ≤
        (1 : ℝ) / 2 ^ 133 := by
  intro breakEvent
  by_cases hQrom :
      breakEvent =
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak
  · subst breakEvent
    simp [remainingCryptographicBreakBound]
  · by_cases hLiveSlot :
        breakEvent =
          Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak
    · subst breakEvent
      simp [remainingCryptographicBreakBound, hQrom]
    · simpa [remainingCryptographicBreakBound, hQrom, hLiveSlot] using
        hBreakBudget ⟨breakEvent, ⟨hQrom, hLiveSlot⟩⟩

/--
The faithful concrete game satisfies the synthesized full-family break-bound
map when the caller supplies bounds for only the remaining cryptographic
breaks.  The two structural events use their zero-probability proofs.
-/
theorem concreteBreakProb_le_remainingCryptographicBreakBound
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame C traceDist observe).breakProb
            breakEvent.1 ≤
          remainingBreakBound breakEvent) :
    ∀ breakEvent : Path2FullSpendGame.BreakEvent,
      (concreteNumericGame C traceDist observe).breakProb breakEvent ≤
        remainingCryptographicBreakBound remainingBreakBound breakEvent := by
  intro breakEvent
  by_cases hQrom :
      breakEvent =
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak
  · subst breakEvent
    rw [qromBreakProb_eq_zero]
    simp [remainingCryptographicBreakBound]
  · by_cases hLiveSlot :
        breakEvent =
          Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak
    · subst breakEvent
      rw [liveSlotLeakageBreakProb_eq_zero]
      simp [remainingCryptographicBreakBound, hQrom]
    · simpa [remainingCryptographicBreakBound, hQrom, hLiveSlot] using
        hBreak ⟨breakEvent, ⟨hQrom, hLiveSlot⟩⟩

/--
Concrete 128-bit closure where the caller supplies bounds only for the seven
remaining cryptographic named breaks.

The full nine-event budget is synthesized here: the QROM/domain-separation and
live-slot/keygen slots receive zero budget, matching the already-proven
zero-probability facts, while every other break consumes the caller's
cryptographic bound.
-/
theorem concreteNumericClosure_of_remaining_break_bounds
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame C traceDist observe).acceptedSpendProb ≤
        acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame C traceDist observe).breakProb
            breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    (concreteNumericGame C traceDist observe).successProb ≤ (1 : ℝ) / 2 ^ 128 := by
  exact concreteNumericClosure_of_cryptographic_bounds
    C traceDist observe acceptedSpendBound
    (remainingCryptographicBreakBound remainingBreakBound)
    hAccept
    (concreteBreakProb_le_remainingCryptographicBreakBound
      C traceDist observe remainingBreakBound hBreak)
    hAcceptBudget
    (remainingCryptographicBreakBound_le_2pow_neg133
      remainingBreakBound hBreakBudget)

/--
The first-class WS7 remaining-closure evidence is enough to close the concrete
selected numeric game at the active 128-bit success target.
-/
theorem concreteNumericClosure_of_ws7_remaining_closure_evidence
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    (concreteNumericGame selectedRustConcreteGameConfig traceDist observe).successProb ≤
      (1 : ℝ) / 2 ^ 128 :=
  concreteNumericClosure_of_remaining_break_bounds
    selectedRustConcreteGameConfig
    traceDist
    observe
    E.acceptedSpendBound
    E.remainingBreakBound
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)

/--
The WS7 evidence success bound is the active 128-bit inverse-power target used
by the selected deployment boundary.
-/
theorem concreteNumericClosure_of_ws7_remaining_closure_evidence_le_active128BitTarget
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    (concreteNumericGame selectedRustConcreteGameConfig traceDist observe).successProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [one_div] using
    (concreteNumericClosure_of_ws7_remaining_closure_evidence
      traceDist observe E)

/--
Constructive break-pricing certificate for the faithful concrete game from
accepted-spend pricing plus the seven remaining cryptographic break bounds.
-/
def concreteBreakPricingCertificate_of_remaining_break_bounds
    {Trace : Type*} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame C traceDist observe).acceptedSpendProb ≤
        acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame C traceDist observe).breakProb
            breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    AcceptedSpendBreakPricingCertificate
      (concreteNumericGame C traceDist observe) where
  acceptedSpendBound := acceptedSpendBound
  breakBound := remainingCryptographicBreakBound remainingBreakBound
  pricedBound := (1 : ℝ) / 2 ^ 128
  acceptedSpendProb_le_bound := hAccept
  breakProb_le_bound :=
    concreteBreakProb_le_remainingCryptographicBreakBound
      C traceDist observe remainingBreakBound hBreak
  breakBudget_le_priced := by
    refine activeBudget_closes ((1 : ℝ) / 2 ^ 128) (by positivity)
      acceptedSpendBound
      (remainingCryptographicBreakBound remainingBreakBound) ?_ ?_
    · have : (1 : ℝ) / 2 ^ 128 / 2 = 1 / 2 ^ 129 := by norm_num
      rw [this]
      exact hAcceptBudget
    · intro breakEvent
      have : (1 : ℝ) / 2 ^ 128 / 32 = 1 / 2 ^ 133 := by norm_num
      rw [this]
      exact
        remainingCryptographicBreakBound_le_2pow_neg133
          remainingBreakBound hBreakBudget breakEvent

/--
Packaged priced numeric certificate for the faithful concrete game from the
accepted-spend bound and seven remaining cryptographic break bounds.
-/
def concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (C : ConcreteGameConfig)
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame C traceDist observe).acceptedSpendProb ≤
        acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame C traceDist observe).breakProb
            breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    PricedAcceptedSpendNumericCertificate where
  Trace := Trace
  Outcome := ConcreteOutcome
  Break := Path2FullSpendGame.BreakEvent
  traceFintype := inferInstance
  breakFintype := inferInstance
  breakDecidableEq := inferInstance
  activeBreakFamilyPinned := rfl
  game := concreteNumericGame C traceDist observe
  breakPricing :=
    concreteBreakPricingCertificate_of_remaining_break_bounds
      C traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget
  pricedBound := (1 : ℝ) / 2 ^ 128
  pricedBound_eq_breakPricing := rfl

/--
Constructive break-pricing certificate directly from first-class WS7
remaining-closure evidence.
-/
def concreteBreakPricingCertificate_of_ws7_remaining_closure_evidence
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    AcceptedSpendBreakPricingCertificate
      (concreteNumericGame selectedRustConcreteGameConfig traceDist observe) :=
  concreteBreakPricingCertificate_of_remaining_break_bounds
    selectedRustConcreteGameConfig
    traceDist
    observe
    E.acceptedSpendBound
    E.remainingBreakBound
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)

/--
Packaged priced numeric certificate directly from first-class WS7
remaining-closure evidence.
-/
def concretePricedAcceptedSpendNumericCertificate_of_ws7_remaining_closure_evidence
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    PricedAcceptedSpendNumericCertificate :=
  concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds
    selectedRustConcreteGameConfig
    traceDist
    observe
    E.acceptedSpendBound
    E.remainingBreakBound
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)

/--
Selected-Rust concrete numeric certificate wrapper built from the faithful
trace model and the seven remaining cryptographic break bounds.

The trace support and distribution are still caller supplied; this constructor
only removes the proof-convenience step of first building an arbitrary
full-family pricing certificate before wrapping it for AP2-N003.
-/
def selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (finiteTraceSupportDigest : ByteArray)
    (activeOutcomeCarrierDigest : ByteArray)
    (acceptedSpendEventSemanticsDigest : ByteArray)
    (activeBreakFamilyDigest : ByteArray)
    (namedBreakEventsDigest : ByteArray)
    (acceptedSpendUnionBoundPricingDigest : ByteArray)
    (acceptedSpendPricedBoundDigest : ByteArray)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    SelectedRustConcreteAcceptedSpendNumericCertificate where
  pricedCertificate :=
    concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds
      selectedRustConcreteGameConfig traceDist observe acceptedSpendBound
      remainingBreakBound hAccept hBreak hAcceptBudget hBreakBudget
  finiteTraceSupportDigest := finiteTraceSupportDigest
  activeOutcomeCarrierDigest := activeOutcomeCarrierDigest
  acceptedSpendEventSemanticsDigest := acceptedSpendEventSemanticsDigest
  activeBreakFamilyDigest := activeBreakFamilyDigest
  namedBreakEventsDigest := namedBreakEventsDigest
  acceptedSpendUnionBoundPricingDigest := acceptedSpendUnionBoundPricingDigest
  acceptedSpendPricedBoundDigest := acceptedSpendPricedBoundDigest
  outcomeCarrierPinned := rfl
  acceptedSpendEventPinned := by
    intro trace hAccepted
    have hEvent :
        concreteAcceptedSpendForgery selectedRustConcreteGameConfig
            (observe trace) = true := by
      simpa [concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concreteBreakPricingCertificate_of_remaining_break_bounds,
        concreteNumericGame, PricedAcceptedSpendNumericCertificate.acceptedSpendEvent,
        PricedAcceptedSpendNumericCertificate.observe] using hAccepted
    simpa [concreteAcceptedSpendForgery] using of_decide_eq_true hEvent
  breakFamilyPinned := rfl
  namedBreakEventsPinned := by
    intro trace breakEvent hBreakEvent
    have hEvent :
        concreteNamedBreak selectedRustConcreteGameConfig breakEvent
            (observe trace) = true := by
      simpa [concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concreteBreakPricingCertificate_of_remaining_break_bounds,
        concreteNumericGame, PricedAcceptedSpendNumericCertificate.namedBreakEvent,
        PricedAcceptedSpendNumericCertificate.observe] using hBreakEvent
    simpa [concreteNamedBreak] using of_decide_eq_true hEvent

/--
Selected-Rust concrete numeric certificate wrapper built directly from the
first-class WS7 remaining-closure evidence.
-/
def selectedRustConcreteAcceptedSpendNumericCertificate_of_ws7_remaining_closure_evidence
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    SelectedRustConcreteAcceptedSpendNumericCertificate :=
  selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds
    traceDist
    observe
    E.acceptedSpendBound
    E.remainingBreakBound
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).finiteTraceSupportDigest
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).activeOutcomeCarrierDigest
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendEventSemanticsDigest
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).activeBreakFamilyDigest
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).namedBreakEventsDigest
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendUnionBoundPricingDigest
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundDigest
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)

/--
The selected concrete numeric certificate built from WS7 evidence closes its
success probability at the fixed active 128-bit target.
-/
theorem selectedRustConcreteAcceptedSpendNumericCertificate_successProb_le_active128BitTarget_of_ws7_remaining_closure_evidence
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe) :
    let Cert :=
      selectedRustConcreteAcceptedSpendNumericCertificate_of_ws7_remaining_closure_evidence
        traceDist observe E
    Cert.pricedCertificate.successProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendNumericCertificate_of_ws7_remaining_closure_evidence,
    selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    one_div] using
    (concreteNumericClosure_of_ws7_remaining_closure_evidence
      traceDist observe E)

/--
Selected pricing witness built from the faithful trace model and the seven
remaining cryptographic break bounds.

This is the AP2-N003 pricing-witness handoff.  It still requires the real
cryptographic inputs: accepted-spend pricing, remaining-break pricing, the
selected report pricing facts, and the real-valued priced-bound security
inequality.  What it removes is the ability to satisfy the witness through an
arbitrary full-family certificate detached from the concrete trace model.
-/
def selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    SelectedRustConcreteAcceptedSpendPricingWitness :=
  let concreteCertificate :=
    selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).finiteTraceSupportDigest
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).activeOutcomeCarrierDigest
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendEventSemanticsDigest
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).activeBreakFamilyDigest
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).namedBreakEventsDigest
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendUnionBoundPricingDigest
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundDigest
      hAccept hBreak hAcceptBudget hBreakBudget
  { concreteCertificate := concreteCertificate
    acceptedSpendPricedBoundSecurityPriced := by
      simpa [concreteCertificate,
        selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        one_div] using hPriced
    acceptedSpendComponentBoundMeetsProportionalTarget := by
      simpa [concreteCertificate,
        selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concreteBreakPricingCertificate_of_remaining_break_bounds,
        PricedAcceptedSpendNumericCertificate.acceptedSpendBound] using
        hAcceptBudget
    namedBreakComponentBoundsMeetProportionalTarget := by
      intro breakEvent
      simpa [concreteCertificate,
        selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
        concreteBreakPricingCertificate_of_remaining_break_bounds,
        PricedAcceptedSpendNumericCertificate.breakBound] using
        remainingCryptographicBreakBound_le_2pow_neg133
          remainingBreakBound hBreakBudget breakEvent
    acceptedSpendNumericBoundPricedProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_numericBoundPriced
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    acceptedSpendEventBoundPricedProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_acceptedSpendEventBoundPriced
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    namedBreakEventBoundsPricedProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_namedBreakEventBoundsPriced
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    breakPricingBudgetMeetsPricedBoundProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_breakPricingBudgetMeetsPricedBound
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    acceptedSpendNumericBudgetMeetsTargetProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_numericBudgetMeetsTarget
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    acceptedSpendPricedBoundSecurityMeetsTargetProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_pricedBoundSecurityMeetsTarget
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    acceptedSpendPricedBoundSecurityWithinTranscriptProof :=
      Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_pricedBoundSecurityWithinTranscript
        Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport hPricing
    finiteTraceSupportDigestBoundProof := rfl
    activeOutcomeCarrierDigestBoundProof := rfl
    acceptedSpendEventSemanticsDigestBoundProof := rfl
    activeBreakFamilyDigestBoundProof := rfl
    namedBreakEventsDigestBoundProof := rfl
    acceptedSpendUnionBoundPricingDigestBoundProof := rfl
    acceptedSpendPricedBoundDigestBoundProof := rfl }

/--
Selected pricing witness from the four hard current accepted-spend pricing
facts. The selected report's numeric-budget target and transcript-cap pricing
facts are already proved in `Path2NativeAccountDesign` and are supplied
internally.
-/
def selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    SelectedRustConcreteAcceptedSpendPricingWitness :=
  selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound hAccept hBreak
    hAcceptBudget hBreakBudget hPriced
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
      hHardPricing)

/--
Active-target consequence for the selected trace-model pricing witness.

This theorem gives the future AP2-N003 instantiation a single named endpoint
from the faithful trace model, selected pricing facts, and remaining-break
cryptographic prices to the selected accepted-spend numeric success bound.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_successProb_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.successProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.successProb_le_active128BitTarget_of_proportional_bounds
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)

/--
The trace-model pricing witness exposes the accepted-spend component budget at
the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendBound_le_2pow_neg129_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendBound ≤
      (1 : ℝ) / 2 ^ 129 :=
  SelectedRustConcreteAcceptedSpendPricingWitness.acceptedSpendBound_le_2pow_neg129
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)

/--
The trace-model pricing witness exposes every remaining cryptographic
named-break component budget at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakBound_le_2pow_neg133_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (breakEvent : RemainingCryptographicBreak) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.breakBound breakEvent.1 ≤
      (1 : ℝ) / 2 ^ 133 :=
  SelectedRustConcreteAcceptedSpendPricingWitness.breakBound_le_2pow_neg133
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)
    breakEvent.1

/--
The trace-model pricing witness keeps the concrete accepted-spend probability
under its accepted-spend component at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendProb_le_acceptedSpendBound_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendProb ≤
      W.concreteCertificate.pricedCertificate.acceptedSpendBound :=
  SelectedRustConcreteAcceptedSpendPricingWitness.acceptedSpendProb_le_acceptedSpendBound
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)

/--
The trace-model pricing witness carries the concrete accepted-spend probability
to the fixed active 128-bit target at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendProb_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.acceptedSpendProb_le_active128BitTarget
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)

/--
The trace-model pricing witness carries every remaining cryptographic
named-break probability to its proportional 2^-133 component target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakProb_le_2pow_neg133_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (breakEvent : RemainingCryptographicBreak) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      (1 : ℝ) / 2 ^ 133 :=
  le_trans
    (SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_breakBound
      (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)
      breakEvent.1)
    (SelectedRustConcreteAcceptedSpendPricingWitness.breakBound_le_2pow_neg133
      (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)
      breakEvent.1)

/--
The trace-model pricing witness carries every remaining cryptographic
named-break probability to the fixed active 128-bit target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakProb_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (breakEvent : RemainingCryptographicBreak) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_active128BitTarget
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)
    breakEvent.1

/--
The trace-model pricing witness composes its remaining-break component
budgets into the fixed active 128-bit component budget.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_componentBudget_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ W.concreteCertificate.pricedCertificate.Break
          W.concreteCertificate.pricedCertificate.breakFintype).sum
            W.concreteCertificate.pricedCertificate.breakBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.componentBudget_le_active128BitTarget_of_proportional_bounds
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)

/--
The trace-model pricing witness carries the accepted-spend union expression to
the fixed active 128-bit target at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_unionBound_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    W.concreteCertificate.pricedCertificate.unionBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.unionBound_le_active128BitTarget
    (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)

/--
The trace-model pricing witness exposes that the structural QROM/domain
separation slot is discharged to zero probability at this layer.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_eq_zero_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak = 0 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds,
    selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concreteBreakPricingCertificate_of_remaining_break_bounds] using
    qromBreakProb_eq_zero selectedRustConcreteGameConfig traceDist observe

/--
The trace-model pricing witness prices the structural QROM/domain separation
slot below its active proportional component target because it is zero.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_le_2pow_neg133_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak ≤
      (1 : ℝ) / 2 ^ 133 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds,
    selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concreteBreakPricingCertificate_of_remaining_break_bounds] using
    SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_2pow_neg133_of_never
      (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)
      Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak
      (by
        intro trace
        simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds,
          selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
          concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
          concreteBreakPricingCertificate_of_remaining_break_bounds,
          PricedAcceptedSpendNumericCertificate.namedBreakEvent,
          PricedAcceptedSpendNumericCertificate.observe] using
          qromBreak_never selectedRustConcreteGameConfig observe trace)

/--
The trace-model pricing witness exposes that the structural live-slot/keygen
slot is discharged to zero probability at this layer.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_eq_zero_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak = 0 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds,
    selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concreteBreakPricingCertificate_of_remaining_break_bounds] using
    liveSlotLeakageBreakProb_eq_zero selectedRustConcreteGameConfig traceDist observe

/--
The trace-model pricing witness prices the structural live-slot/keygen slot
below its active proportional component target because it is zero.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_le_2pow_neg133_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak ≤
      (1 : ℝ) / 2 ^ 133 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds,
    selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
    concreteBreakPricingCertificate_of_remaining_break_bounds] using
    SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_2pow_neg133_of_never
      (selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing)
      Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak
      (by
        intro trace
        simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds,
          selectedRustConcreteAcceptedSpendNumericCertificate_of_remaining_break_bounds,
          concretePricedAcceptedSpendNumericCertificate_of_remaining_break_bounds,
          concreteBreakPricingCertificate_of_remaining_break_bounds,
          PricedAcceptedSpendNumericCertificate.namedBreakEvent,
          PricedAcceptedSpendNumericCertificate.observe] using
          liveSlotLeakageBreak_never selectedRustConcreteGameConfig observe trace)

section CurrentHardPricingTraceWitnessProjections

variable {Trace : Type} [Fintype Trace]
variable (traceDist : FiniteProbability Trace)
variable (observe : Trace → ConcreteOutcome)
variable (acceptedSpendBound : ℝ)
variable (remainingBreakBound : RemainingCryptographicBreak → ℝ)
variable (hAccept :
  (concreteNumericGame selectedRustConcreteGameConfig traceDist
      observe).acceptedSpendProb ≤ acceptedSpendBound)
variable (hBreak :
  ∀ breakEvent : RemainingCryptographicBreak,
    (concreteNumericGame selectedRustConcreteGameConfig traceDist
        observe).breakProb breakEvent.1 ≤
      remainingBreakBound breakEvent)
variable (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
variable (hBreakBudget :
  ∀ breakEvent : RemainingCryptographicBreak,
    remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
variable (hPriced :
  (1 : ℝ) / 2 ^ 128 ≤
    ((2 : ℝ) ^
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
variable (hHardPricing :
  Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)

/--
The current-hard-pricing trace witness keeps concrete accepted-spend success on
the fixed active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_successProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.successProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_successProb_le_active128BitTarget_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness exposes the accepted-spend component
budget at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendBound_le_2pow_neg129_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendBound ≤
      (1 : ℝ) / 2 ^ 129 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendBound_le_2pow_neg129_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness exposes every remaining cryptographic
named-break component budget at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakBound_le_2pow_neg133_of_remaining_break_bounds_and_current_hard_pricing_facts
    (breakEvent : RemainingCryptographicBreak) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.breakBound breakEvent.1 ≤
      (1 : ℝ) / 2 ^ 133 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakBound_le_2pow_neg133_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)
        breakEvent

/--
The current-hard-pricing trace witness keeps the concrete accepted-spend
probability under its accepted-spend component at the remaining-break endpoint.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendProb_le_acceptedSpendBound_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendProb ≤
      W.concreteCertificate.pricedCertificate.acceptedSpendBound := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendProb_le_acceptedSpendBound_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness keeps the concrete accepted-spend
probability on the fixed active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_acceptedSpendProb_le_active128BitTarget_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness carries every remaining cryptographic
named-break probability to its proportional 2^-133 component target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakProb_le_2pow_neg133_of_remaining_break_bounds_and_current_hard_pricing_facts
    (breakEvent : RemainingCryptographicBreak) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      (1 : ℝ) / 2 ^ 133 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakProb_le_2pow_neg133_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)
        breakEvent

/--
The current-hard-pricing trace witness keeps every remaining cryptographic break
probability on the fixed active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts
    (breakEvent : RemainingCryptographicBreak) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_remainingBreakProb_le_active128BitTarget_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)
        breakEvent

/--
The current-hard-pricing trace witness keeps the component budget on the fixed
active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_componentBudget_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ W.concreteCertificate.pricedCertificate.Break
          W.concreteCertificate.pricedCertificate.breakFintype).sum
            W.concreteCertificate.pricedCertificate.breakBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_componentBudget_le_active128BitTarget_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness keeps the union expression on the fixed
active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_unionBound_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.unionBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_unionBound_le_active128BitTarget_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness exposes the structural QROM/domain
separation slot as zero probability.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_eq_zero_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak = 0 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_eq_zero_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness prices the structural QROM/domain
separation slot below its active proportional component target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_le_2pow_neg133_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak ≤
      (1 : ℝ) / 2 ^ 133 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_le_2pow_neg133_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness exposes the structural live-slot/keygen
slot as zero probability.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_eq_zero_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak = 0 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_eq_zero_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

/--
The current-hard-pricing trace witness prices the structural live-slot/keygen
slot below its active proportional component target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_le_2pow_neg133_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        W.concreteCertificate.pricedCertificate.Trace
        W.concreteCertificate.pricedCertificate.Outcome
        W.concreteCertificate.pricedCertificate.Break
        W.concreteCertificate.pricedCertificate.traceFintype
        W.concreteCertificate.pricedCertificate.breakFintype
        W.concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak ≤
      (1 : ℝ) / 2 ^ 133 := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_le_2pow_neg133_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
          hHardPricing)

end CurrentHardPricingTraceWitnessProjections

/--
Selected trace-model pricing witness built directly from first-class WS7
remaining-closure evidence and the current hard pricing facts.

This is still fail-closed for the current selected report: the hard pricing
facts remain explicit inputs.  The bridge removes only the manual unpacking of
the WS7 accepted-spend and remaining-break rows.
-/
def selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    SelectedRustConcreteAcceptedSpendPricingWitness :=
  selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist
    observe
    E.acceptedSpendBound
    E.remainingBreakBound
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)
    hPriced
    hHardPricing

/--
The direct WS7-to-pricing-witness bridge keeps concrete accepted-spend success
on the fixed active 128-bit target once the current hard pricing facts are
supplied.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_successProb_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.successProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_successProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe E.acceptedSpendBound E.remainingBreakBound
        (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
        (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
        (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
        (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)
        hPriced hHardPricing

/--
The direct WS7-to-pricing-witness bridge composes the accepted-spend and
remaining-break component budgets into the fixed active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_componentBudget_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ W.concreteCertificate.pricedCertificate.Break
          W.concreteCertificate.pricedCertificate.breakFintype).sum
            W.concreteCertificate.pricedCertificate.breakBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_componentBudget_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe E.acceptedSpendBound E.remainingBreakBound
        (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
        (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
        (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
        (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)
        hPriced hHardPricing

/--
The direct WS7-to-pricing-witness bridge carries the union expression to the
fixed active target.
-/
theorem selectedRustConcreteAcceptedSpendPricingWitness_unionBound_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    let W :=
      selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
    W.concreteCertificate.pricedCertificate.unionBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_unionBound_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe E.acceptedSpendBound E.remainingBreakBound
        (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
        (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
        (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
        (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)
        hPriced hHardPricing

/--
Single AP2-N003 hard-half package for selected accepted-spend pricing over the
faithful remaining-break trace model.

This packages the accepted-spend bound, the seven remaining cryptographic break
bounds, their proportional budgets, the selected report pricing bits, and the
real-valued priced-bound security inequality into one object. Constructing this
package is still the hard production pricing work; consuming it is no longer a
long anonymous argument list.
-/
structure SelectedRustAcceptedSpendRemainingBreakPricingPackage where
  Trace : Type
  [traceFintype : Fintype Trace]
  traceDist : FiniteProbability Trace
  observe : Trace → ConcreteOutcome
  acceptedSpendBound : ℝ
  remainingBreakBound : RemainingCryptographicBreak → ℝ
  acceptedSpendProb_le_bound :
    (concreteNumericGame selectedRustConcreteGameConfig traceDist
        observe).acceptedSpendProb ≤ acceptedSpendBound
  remainingBreakProb_le_bound :
    ∀ breakEvent : RemainingCryptographicBreak,
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).breakProb breakEvent.1 ≤
        remainingBreakBound breakEvent
  acceptedSpendBound_le_2pow_neg129 :
    acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129
  remainingBreakBound_le_2pow_neg133 :
    ∀ breakEvent : RemainingCryptographicBreak,
      remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133
  activePricedBound_le_reportedSecurity :
    (1 : ℝ) / 2 ^ 128 ≤
      ((2 : ℝ) ^
        (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹
  pricingFactsComplete :
    Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete

namespace SelectedRustAcceptedSpendRemainingBreakPricingPackage

/--
Construct the remaining-break pricing package from the four hard current
accepted-spend pricing facts.  The current selected report supplies the already
closed numeric-budget target and transcript-cap facts internally.
-/
def ofCurrentHardPricingFacts
    (Trace : Type) [traceFintype : Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (acceptedSpendProb_le_bound :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (remainingBreakProb_le_bound :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (acceptedSpendBound_le_2pow_neg129 :
      acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (remainingBreakBound_le_2pow_neg133 :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (activePricedBound_le_reportedSecurity :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hardPricingFactsComplete :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    SelectedRustAcceptedSpendRemainingBreakPricingPackage where
  Trace := Trace
  traceFintype := traceFintype
  traceDist := traceDist
  observe := observe
  acceptedSpendBound := acceptedSpendBound
  remainingBreakBound := remainingBreakBound
  acceptedSpendProb_le_bound := acceptedSpendProb_le_bound
  remainingBreakProb_le_bound := remainingBreakProb_le_bound
  acceptedSpendBound_le_2pow_neg129 :=
    acceptedSpendBound_le_2pow_neg129
  remainingBreakBound_le_2pow_neg133 :=
    remainingBreakBound_le_2pow_neg133
  activePricedBound_le_reportedSecurity :=
    activePricedBound_le_reportedSecurity
  pricingFactsComplete :=
    Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
      hardPricingFactsComplete

/--
Construct the remaining-break pricing package from the first-class eight-row
WS7 remaining-obligation evidence.
-/
def ofWS7RemainingClosureEvidence
    (Trace : Type) [traceFintype : Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (activePricedBound_le_reportedSecurity :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hardPricingFactsComplete :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    SelectedRustAcceptedSpendRemainingBreakPricingPackage := by
  exact
    ofCurrentHardPricingFacts
      Trace
      traceDist
      observe
      E.acceptedSpendBound
      E.remainingBreakBound
      (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
      (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
      (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
      (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)
      activePricedBound_le_reportedSecurity
      hardPricingFactsComplete

/-- Erase the packaged hard-half input into the selected pricing witness. -/
def toPricingWitness
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    SelectedRustConcreteAcceptedSpendPricingWitness := by
  letI := P.traceFintype
  exact
    selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      P.traceDist
      P.observe
      P.acceptedSpendBound
      P.remainingBreakBound
      P.acceptedSpendProb_le_bound
      P.remainingBreakProb_le_bound
      P.acceptedSpendBound_le_2pow_neg129
      P.remainingBreakBound_le_2pow_neg133
      P.activePricedBound_le_reportedSecurity
      P.pricingFactsComplete

/-- The package supplies the active selected report pricing half. -/
theorem toPricingWitness_pricingFactsComplete
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete :=
  SelectedRustConcreteAcceptedSpendPricingWitness.pricingFactsComplete
    (toPricingWitness P)

/-- The package exposes the four hard accepted-spend pricing facts. -/
theorem toPricingWitness_hardPricingFactsComplete
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_hardPricingFactsComplete
    Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport
    (toPricingWitness_pricingFactsComplete P)

/-- No remaining-break pricing package exists for the current hard-pricing fail-closed report. -/
theorem currentSelectedHardPricingReport_impossible
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) : False :=
  Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_currentHardPricingFactsComplete_impossible
    (toPricingWitness_hardPricingFactsComplete P)

/-- No remaining-break pricing package exists for the current fail-closed report. -/
theorem currentSelectedReport_impossible
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) : False :=
  currentSelectedHardPricingReport_impossible P

/--
The current fail-closed selected report also rejects a remaining-break pricing
package produced from WS7 obligation evidence.
-/
theorem ofWS7RemainingClosureEvidence_currentSelectedHardPricingReport_impossible
    (Trace : Type) [traceFintype : Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (activePricedBound_le_reportedSecurity :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hardPricingFactsComplete :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    False :=
  currentSelectedHardPricingReport_impossible
    (ofWS7RemainingClosureEvidence
      Trace traceDist observe E
      activePricedBound_le_reportedSecurity hardPricingFactsComplete)

/--
The package keeps the concrete accepted-spend probability on the fixed
active-target route.
-/
theorem acceptedSpendProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.acceptedSpendProb_le_active128BitTarget
    (toPricingWitness P)

/--
The package exposes the accepted-spend component bound used by the selected
trace-model pricing witness.
-/
theorem acceptedSpendComponentBound_le_2pow_neg129
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendBound ≤
      (1 : ℝ) / 2 ^ 129 :=
  SelectedRustConcreteAcceptedSpendPricingWitness.acceptedSpendBound_le_2pow_neg129
    (toPricingWitness P)

/--
The package keeps the concrete accepted-spend probability under its component
budget before taking the active-target consequence.
-/
theorem acceptedSpendProb_le_acceptedSpendBound
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendProb ≤
      (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendBound :=
  SelectedRustConcreteAcceptedSpendPricingWitness.acceptedSpendProb_le_acceptedSpendBound
    (toPricingWitness P)

/--
The package exposes every remaining cryptographic break component bound used by
the selected trace-model pricing witness.
-/
theorem remainingBreakComponentBound_le_2pow_neg133
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (breakEvent : RemainingCryptographicBreak) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.breakBound
        breakEvent.1 ≤
      (1 : ℝ) / 2 ^ 133 :=
  SelectedRustConcreteAcceptedSpendPricingWitness.breakBound_le_2pow_neg133
    (toPricingWitness P) breakEvent.1

/--
The package keeps every remaining cryptographic break probability under its
component budget before taking the active-target consequence.
-/
theorem remainingBreakProb_le_breakBound
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (breakEvent : RemainingCryptographicBreak) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      (toPricingWitness P).concreteCertificate.pricedCertificate.breakBound
        breakEvent.1 :=
  SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_breakBound
    (toPricingWitness P) breakEvent.1

/--
The package keeps every remaining cryptographic break probability on its
proportional component route.
-/
theorem remainingBreakProb_le_2pow_neg133
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (breakEvent : RemainingCryptographicBreak) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      (1 : ℝ) / 2 ^ 133 :=
  le_trans
    (SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_breakBound
      (toPricingWitness P) breakEvent.1)
    (SelectedRustConcreteAcceptedSpendPricingWitness.breakBound_le_2pow_neg133
      (toPricingWitness P) breakEvent.1)

/--
The package keeps every remaining cryptographic break probability on the fixed
active-target route.
-/
theorem remainingBreakProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (breakEvent : RemainingCryptographicBreak) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game breakEvent.1 ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.breakProb_le_active128BitTarget
    (toPricingWitness P) breakEvent.1

/--
The package exposes that the structural QROM/domain-separation break slot is
zero probability in the selected trace-model witness.
-/
theorem qromBreakProb_eq_zero
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak = 0 := by
  letI := P.traceFintype
  simpa [toPricingWitness] using
    selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_eq_zero_of_remaining_break_bounds
      P.traceDist P.observe P.acceptedSpendBound P.remainingBreakBound
      P.acceptedSpendProb_le_bound P.remainingBreakProb_le_bound
      P.acceptedSpendBound_le_2pow_neg129
      P.remainingBreakBound_le_2pow_neg133
      P.activePricedBound_le_reportedSecurity
      P.pricingFactsComplete

/--
The package exposes that the structural QROM/domain-separation break slot
meets the active per-break component target because it is zero probability.
-/
theorem qromBreakProb_le_2pow_neg133
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak ≤
      (1 : ℝ) / 2 ^ 133 := by
  letI := P.traceFintype
  simpa [toPricingWitness] using
    selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_le_2pow_neg133_of_remaining_break_bounds
      P.traceDist P.observe P.acceptedSpendBound P.remainingBreakBound
      P.acceptedSpendProb_le_bound P.remainingBreakProb_le_bound
      P.acceptedSpendBound_le_2pow_neg129
      P.remainingBreakBound_le_2pow_neg133
      P.activePricedBound_le_reportedSecurity
      P.pricingFactsComplete

/--
The package exposes that the structural live-slot/keygen break slot is zero
probability in the selected trace-model witness.
-/
theorem liveSlotLeakageBreakProb_eq_zero
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak = 0 := by
  letI := P.traceFintype
  simpa [toPricingWitness] using
    selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_eq_zero_of_remaining_break_bounds
      P.traceDist P.observe P.acceptedSpendBound P.remainingBreakBound
      P.acceptedSpendProb_le_bound P.remainingBreakProb_le_bound
      P.acceptedSpendBound_le_2pow_neg129
      P.remainingBreakBound_le_2pow_neg133
      P.activePricedBound_le_reportedSecurity
      P.pricingFactsComplete

/--
The package exposes that the structural live-slot/keygen break slot meets the
active per-break component target because it is zero probability.
-/
theorem liveSlotLeakageBreakProb_le_2pow_neg133
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    @Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb
        (toPricingWitness P).concreteCertificate.pricedCertificate.Trace
        (toPricingWitness P).concreteCertificate.pricedCertificate.Outcome
        (toPricingWitness P).concreteCertificate.pricedCertificate.Break
        (toPricingWitness P).concreteCertificate.pricedCertificate.traceFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype
        (toPricingWitness P).concreteCertificate.pricedCertificate.game
        Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak ≤
      (1 : ℝ) / 2 ^ 133 := by
  letI := P.traceFintype
  simpa [toPricingWitness] using
    selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_le_2pow_neg133_of_remaining_break_bounds
      P.traceDist P.observe P.acceptedSpendBound P.remainingBreakBound
      P.acceptedSpendProb_le_bound P.remainingBreakProb_le_bound
      P.acceptedSpendBound_le_2pow_neg129
      P.remainingBreakBound_le_2pow_neg133
      P.activePricedBound_le_reportedSecurity
      P.pricingFactsComplete

/-- The package exposes the union-to-component-budget route. -/
theorem unionBound_le_componentBudget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.unionBound ≤
      (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ
          (toPricingWitness P).concreteCertificate.pricedCertificate.Break
          (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype).sum
            (toPricingWitness P).concreteCertificate.pricedCertificate.breakBound :=
  SelectedRustConcreteAcceptedSpendPricingWitness.unionBound_le_componentBudget
    (toPricingWitness P)

/-- The package exposes the success-to-component-budget route. -/
theorem successProb_le_componentBudget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.successProb ≤
      (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ
          (toPricingWitness P).concreteCertificate.pricedCertificate.Break
          (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype).sum
            (toPricingWitness P).concreteCertificate.pricedCertificate.breakBound :=
  SelectedRustConcreteAcceptedSpendPricingWitness.successProb_le_componentBudget
    (toPricingWitness P)

/-- The package exposes the component-budget-to-priced-bound route. -/
theorem componentBudget_le_pricedBound
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ
          (toPricingWitness P).concreteCertificate.pricedCertificate.Break
          (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype).sum
            (toPricingWitness P).concreteCertificate.pricedCertificate.breakBound ≤
      (toPricingWitness P).concreteCertificate.pricedCertificate.pricedBound :=
  SelectedRustConcreteAcceptedSpendPricingWitness.breakBudget_le_pricedBound
    (toPricingWitness P)

/-- The package keeps the component budget on the fixed active target. -/
theorem componentBudget_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.acceptedSpendBound +
        (@Finset.univ
          (toPricingWitness P).concreteCertificate.pricedCertificate.Break
          (toPricingWitness P).concreteCertificate.pricedCertificate.breakFintype).sum
            (toPricingWitness P).concreteCertificate.pricedCertificate.breakBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.componentBudget_le_active128BitTarget
    (toPricingWitness P)

/-- The package keeps the accepted-spend union expression on the active target. -/
theorem unionBound_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.unionBound ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.unionBound_le_active128BitTarget
    (toPricingWitness P)

/-- The package keeps full numeric success on the active target. -/
theorem successProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    (toPricingWitness P).concreteCertificate.pricedCertificate.successProb ≤
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  SelectedRustConcreteAcceptedSpendPricingWitness.successProb_le_active128BitTarget
    (toPricingWitness P)

/-- Erase the package into the selected accepted-spend numeric source. -/
def toSelectedAcceptedSpendNumericSource
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage) :
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource :=
  SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toSelectedAcceptedSpendNumericSource
    (SelectedRustConcreteAcceptedSpendPricingWitness.toConcreteNumericSourceCertificate
      (toPricingWitness P))

/--
Erase the package into the selected accepted-spend soundness certificate while
keeping production probability pins and closure propositions explicit.
-/
def toSelectedRustAcceptedSpendSoundnessCertificate
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate := by
  let W := toPricingWitness P
  let Source := toSelectedAcceptedSpendNumericSource P
  exact
    SelectedRustConcreteAcceptedSpendPricingWitness.toSelectedRustAcceptedSpendSoundnessCertificate
      W
      Source.pricedCertificate.unionBound
      Source.pricedCertificate.pricedBound
      productionAcceptedSpendForgeryProb
      productionAcceptedSpendForgeryBound
      rfl
      rfl
      (by
        simpa [Source] using acceptedSpendForgeryProbPinned)
      (by
        simpa [Source] using acceptedSpendForgeryBoundPinned)
      hiddenSourceProofSemanticsClosed
      checkedCoreCompositionClosed
      lifecycleReplayClosed
      noExportClosed
      hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof
      lifecycleReplayClosedProof
      noExportClosedProof

/-- Erase the package-level accepted-spend certificate into the native boundary. -/
def toAcceptedSpendSoundnessBoundary
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    Path2NativeAccountDesign.NativeAcceptedSpendSoundnessBoundary :=
  (toSelectedRustAcceptedSpendSoundnessCertificate
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof).toAcceptedSpendSoundnessBoundary

/--
The package-level accepted-spend boundary reaches the fixed active
128-bit probability target.
-/
theorem toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    let B :=
      toAcceptedSpendSoundnessBoundary
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget
    (toSelectedRustAcceptedSpendSoundnessCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
The package-level accepted-spend boundary keeps the exported accepted-spend
bound on the fixed active 128-bit target.
-/
theorem toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    let B :=
      toAcceptedSpendSoundnessBoundary
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
    (toSelectedRustAcceptedSpendSoundnessCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
Continue the package-level accepted-spend certificate into the selected
full-signature production witness while keeping production facts explicit.
-/
def toFullSignatureReductionProductionWitness
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  let acceptedSpendCertificate :=
    toSelectedRustAcceptedSpendSoundnessCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.ofAcceptedSpendAndReportBits
    acceptedSpendCertificate
    hashBoundary
    signatureForgerySuccessProb
    productionSignatureForgerySuccessProb
    signatureForgerySuccessProbPinned
    (by
      simpa [acceptedSpendCertificate] using
        signatureForgerySuccessMatchesAcceptedSpendGame)
    keygenDistributionModeled
    keygenDistributionModeledProof
    signingOracleModeled
    signingOracleModeledProof
    completenessModeled
    completenessModeledProof
    verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof
    checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof
    multiTargetLossPricedProof
    productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof
    acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof

/--
Continue the package-level accepted-spend certificate into the selected
full-signature production witness using only the three hard current production
facts.  The already-selected active query-symbol pricing and verifier-game /
production-execution bits are supplied by `Path2NativeAccountDesign`.
-/
def toFullSignatureReductionProductionWitnessWithCurrentHardProductionFacts
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  let acceptedSpendCertificate :=
    toSelectedRustAcceptedSpendSoundnessCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.ofAcceptedSpendAndCurrentHardProductionFacts
    acceptedSpendCertificate
    hashBoundary
    signatureForgerySuccessProb
    productionSignatureForgerySuccessProb
    signatureForgerySuccessProbPinned
    (by
      simpa [acceptedSpendCertificate] using
        signatureForgerySuccessMatchesAcceptedSpendGame)
    keygenDistributionModeled
    keygenDistributionModeledProof
    signingOracleModeled
    signingOracleModeledProof
    completenessModeled
    completenessModeledProof
    verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof
    checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof
    multiTargetLossPricedProof
    hardProductionFactsComplete

/-- Erase the package-level production witness into the selected full-reduction certificate. -/
def toFullSignatureReductionCertificate
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.toFullSignatureReductionCertificate
    (toFullSignatureReductionProductionWitness
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled
      signingOracleModeledProof completenessModeled completenessModeledProof
      verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof
      acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
      verifierGameMatchesProductionExecutionProof)

/-- Erase the package-level full-reduction certificate into the native full boundary. -/
def toNativeFullSignatureSecurityBoundary
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  (toFullSignatureReductionCertificate
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).toNativeFullSignatureSecurityBoundary

/--
The package-level native full-signature boundary exposes the active Path 2
handoff from full-signature success to the accepted-spend bound.
-/
theorem toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundary
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb ≤ B.activePathForgeryBound :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound
    (toFullSignatureReductionCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The package-level native full-signature boundary keeps the accepted-spend
active-path bound on the fixed active target.
-/
theorem toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundary
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.activePathForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (toFullSignatureReductionCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The package-level native full-signature boundary reaches the fixed active
target after the explicit production reduction facts are supplied.
-/
theorem toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundary
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (toFullSignatureReductionCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The package-level native full-signature boundary exposes the full
model/reduction/hash completion conjunction after boundary erasure.
-/
theorem toNativeFullSignatureSecurityBoundary_fullSignatureSecurityClaimComplete
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundary
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.keygenDistributionModeled ∧
      B.signingOracleModeled ∧
      B.completenessModeled ∧
      B.verifierRelationMatchesImplementation ∧
      B.fullForgeryImpliesAcceptedSpendForgery ∧
      B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      B.acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      B.acceptedSpendSoundness.lifecycleReplayClosed ∧
      B.acceptedSpendSoundness.noExportClosed ∧
      B.hashBoundary.checkedCoreFrameDomainSeparated ∧
      B.hashBoundary.hiddenSourceFrameDomainSeparated ∧
      B.hashBoundary.queryBudgetPriced ∧
      B.hashBoundary.multiTargetLossPriced :=
  (toNativeFullSignatureSecurityBoundary
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).fullSignatureSecurityClaimComplete

/--
Build the package-level full-reduction production witness from a concrete
full-signature execution source instead of an abstract full-forgery lift.
-/
def toFullSignatureReductionProductionWitnessWithConcreteExecutionSource
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  toFullSignatureReductionProductionWitness
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    (ConcreteFullExecutionAcceptedSpendLift C Source)
    (concrete_fullExecutionAcceptedSpendLift C Source)
    checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof multiTargetLossPricedProof
    productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof
    acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof

/--
Build the package-level full-reduction production witness from a concrete
execution source and the three hard current production facts.  The already
selected active query-symbol pricing and verifier-game/production-execution
bits are not caller-supplied inputs.
-/
def toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  toFullSignatureReductionProductionWitnessWithCurrentHardProductionFacts
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    (ConcreteFullExecutionAcceptedSpendLift C Source)
    (concrete_fullExecutionAcceptedSpendLift C Source)
    checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete

/--
Erase the concrete-source package-level production witness into the selected
full-reduction certificate.
-/
def toFullSignatureReductionCertificateWithConcreteExecutionSource
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.toFullSignatureReductionCertificate
    (toFullSignatureReductionProductionWitnessWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof
      acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
      verifierGameMatchesProductionExecutionProof)

/--
Erase the concrete-source package-level certificate into the native
full-signature boundary.
-/
def toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  (toFullSignatureReductionCertificateWithConcreteExecutionSource
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof C Source
    checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof multiTargetLossPricedProof
    productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof
    acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).toNativeFullSignatureSecurityBoundary

section ConcreteSourceBoundarySignatureProjections

variable (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
variable (productionAcceptedSpendForgeryProb : ℝ)
variable (productionAcceptedSpendForgeryBound : ℝ)
variable (acceptedSpendForgeryProbPinned :
  (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
    productionAcceptedSpendForgeryProb)
variable (acceptedSpendForgeryBoundPinned :
  (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
    productionAcceptedSpendForgeryBound)
variable (hiddenSourceProofSemanticsClosed : Prop)
variable (checkedCoreCompositionClosed : Prop)
variable (lifecycleReplayClosed : Prop)
variable (noExportClosed : Prop)
variable (hiddenSourceProofSemanticsClosedProof :
  hiddenSourceProofSemanticsClosed)
variable (checkedCoreCompositionClosedProof :
  checkedCoreCompositionClosed)
variable (lifecycleReplayClosedProof :
  lifecycleReplayClosed)
variable (noExportClosedProof :
  noExportClosed)
variable (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
variable (signatureForgerySuccessProb : ℝ)
variable (productionSignatureForgerySuccessProb : ℝ)
variable (signatureForgerySuccessProbPinned :
  signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
variable (signatureForgerySuccessMatchesAcceptedSpendGame :
  signatureForgerySuccessProb =
    (toSelectedRustAcceptedSpendSoundnessCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof).pricedNumericCertificate.successProb)
variable (keygenDistributionModeled : Prop)
variable (keygenDistributionModeledProof : keygenDistributionModeled)
variable (signingOracleModeled : Prop)
variable (signingOracleModeledProof : signingOracleModeled)
variable (completenessModeled : Prop)
variable (completenessModeledProof : completenessModeled)
variable (verifierRelationMatchesImplementation : Prop)
variable (verifierRelationMatchesImplementationProof :
  verifierRelationMatchesImplementation)
variable (C : ConcreteGameConfig)
variable (Source : ConcreteFullSignatureExecutionSource C)
variable (checkedCoreFrameDomainSeparatedProof :
  hashBoundary.checkedCoreFrameDomainSeparated)
variable (hiddenSourceFrameDomainSeparatedProof :
  hashBoundary.hiddenSourceFrameDomainSeparated)
variable (queryBudgetPricedProof :
  hashBoundary.queryBudgetPriced)
variable (multiTargetLossPricedProof :
  hashBoundary.multiTargetLossPriced)
variable (productionSignatureProbabilityPinnedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
    true)
variable (signatureSuccessMatchesAcceptedSpendGameProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
    true)
variable (acceptedSpendProbabilityLinkedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
    true)
variable (allActiveQuerySymbolsPricedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
    true)
variable (verifierGameMatchesProductionExecutionProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
    true)

/--
The concrete-source package-level native boundary preserves the selected
reduction from signature-forgery success to accepted-spend success.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_signatureForgerySuccessProb_le_acceptedSpendForgeryProb :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb ≤
      B.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The concrete-source package-level native boundary keeps signature-forgery
success nonnegative after erasure.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_signatureForgerySuccessProb_nonneg :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    0 ≤ B.signatureForgerySuccessProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The concrete-source package-level native boundary keeps signature-forgery
success at most one after erasure.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_signatureForgerySuccessProb_le_one :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb ≤ 1 :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The concrete-source package-level native boundary keeps signature-forgery
success on the fixed active target.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_signatureForgerySuccessProb_le_active128BitTarget :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

end ConcreteSourceBoundarySignatureProjections

/--
The concrete-source package-level native full-signature boundary exposes the
active Path 2 handoff from full-signature success to the accepted-spend bound.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_fullSignatureSuccessProb_le_activePathForgeryBound
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb ≤ B.activePathForgeryBound :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof
      acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
      verifierGameMatchesProductionExecutionProof)

/--
The concrete-source package-level native full-signature boundary keeps the
accepted-spend active-path bound on the fixed active target.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_activePathForgeryBound_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.activePathForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof
      acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
      verifierGameMatchesProductionExecutionProof)

/--
The concrete-source package-level native full-signature boundary reaches the
fixed active target after the remaining production facts are supplied.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_fullSignatureSuccessProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (toFullSignatureReductionCertificateWithConcreteExecutionSource
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof
      acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
      verifierGameMatchesProductionExecutionProof)

/--
The concrete-source package-level native full-signature boundary exposes the
full model/reduction/hash completion conjunction after boundary erasure.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource_fullSignatureSecurityClaimComplete
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof
        acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
        verifierGameMatchesProductionExecutionProof
    B.keygenDistributionModeled ∧
      B.signingOracleModeled ∧
      B.completenessModeled ∧
      B.verifierRelationMatchesImplementation ∧
      B.fullForgeryImpliesAcceptedSpendForgery ∧
      B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      B.acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      B.acceptedSpendSoundness.lifecycleReplayClosed ∧
      B.acceptedSpendSoundness.noExportClosed ∧
      B.hashBoundary.checkedCoreFrameDomainSeparated ∧
      B.hashBoundary.hiddenSourceFrameDomainSeparated ∧
      B.hashBoundary.queryBudgetPriced ∧
      B.hashBoundary.multiTargetLossPriced :=
  (toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSource
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof C Source
    checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof multiTargetLossPricedProof
    productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof
    acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).fullSignatureSecurityClaimComplete

/--
Erase the concrete-source hard-production package-level witness into the
selected full-reduction certificate.
-/
def toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.toFullSignatureReductionCertificate
    (toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
Erase the concrete-source hard-production package-level certificate into the
native full-signature boundary.
-/
def toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      hardProductionFactsComplete).toNativeFullSignatureSecurityBoundary

section ConcreteSourceHardProductionBoundarySignatureProjections

variable (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
variable (productionAcceptedSpendForgeryProb : ℝ)
variable (productionAcceptedSpendForgeryBound : ℝ)
variable (acceptedSpendForgeryProbPinned :
  (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
    productionAcceptedSpendForgeryProb)
variable (acceptedSpendForgeryBoundPinned :
  (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
    productionAcceptedSpendForgeryBound)
variable (hiddenSourceProofSemanticsClosed : Prop)
variable (checkedCoreCompositionClosed : Prop)
variable (lifecycleReplayClosed : Prop)
variable (noExportClosed : Prop)
variable (hiddenSourceProofSemanticsClosedProof :
  hiddenSourceProofSemanticsClosed)
variable (checkedCoreCompositionClosedProof :
  checkedCoreCompositionClosed)
variable (lifecycleReplayClosedProof :
  lifecycleReplayClosed)
variable (noExportClosedProof :
  noExportClosed)
variable (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
variable (signatureForgerySuccessProb : ℝ)
variable (productionSignatureForgerySuccessProb : ℝ)
variable (signatureForgerySuccessProbPinned :
  signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
variable (signatureForgerySuccessMatchesAcceptedSpendGame :
  signatureForgerySuccessProb =
    (toSelectedRustAcceptedSpendSoundnessCertificate
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof).pricedNumericCertificate.successProb)
variable (keygenDistributionModeled : Prop)
variable (keygenDistributionModeledProof : keygenDistributionModeled)
variable (signingOracleModeled : Prop)
variable (signingOracleModeledProof : signingOracleModeled)
variable (completenessModeled : Prop)
variable (completenessModeledProof : completenessModeled)
variable (verifierRelationMatchesImplementation : Prop)
variable (verifierRelationMatchesImplementationProof :
  verifierRelationMatchesImplementation)
variable (C : ConcreteGameConfig)
variable (Source : ConcreteFullSignatureExecutionSource C)
variable (checkedCoreFrameDomainSeparatedProof :
  hashBoundary.checkedCoreFrameDomainSeparated)
variable (hiddenSourceFrameDomainSeparatedProof :
  hashBoundary.hiddenSourceFrameDomainSeparated)
variable (queryBudgetPricedProof :
  hashBoundary.queryBudgetPriced)
variable (multiTargetLossPricedProof :
  hashBoundary.multiTargetLossPriced)
variable (hardProductionFactsComplete :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete)

/--
The concrete-source hard-production package-level production witness supplies
the selected report production facts before certificate erasure.
-/
theorem toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts_productionFactsComplete :
    let _w :=
      toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete
    Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.productionFactsComplete
    (toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level production witness exposes
the three hard current production facts before certificate erasure.
-/
theorem toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts_hardProductionFactsComplete :
    let _w :=
      toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete
    Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.hardProductionFactsComplete
    (toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level production witness remains
fail-closed for the current selected full-reduction report.
-/
theorem toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts_currentSelectedHardProductionReport_impossible :
    let _w :=
      toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete
    False :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.currentSelectedHardProductionReport_impossible
    (toFullSignatureReductionProductionWitnessWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level certificate remains
fail-closed for the current selected hard-production report after witness
erasure.
-/
theorem toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts_currentSelectedHardProductionReport_impossible :
    let _cert :=
      toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete
    False :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.currentSelectedHardProductionReport_impossible
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level certificate remains
fail-closed for the current selected full-reduction report after witness erasure.
-/
theorem toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts_currentSelectedReport_impossible :
    let _cert :=
      toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete
    False :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.currentSelectedReport_impossible
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary preserves the
selected reduction from signature-forgery success to accepted-spend success.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_signatureForgerySuccessProb_le_acceptedSpendForgeryProb :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    B.signatureForgerySuccessProb ≤
      B.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary keeps
signature-forgery success nonnegative after erasure.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_signatureForgerySuccessProb_nonneg :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    0 ≤ B.signatureForgerySuccessProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary keeps
signature-forgery success at most one after erasure.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_signatureForgerySuccessProb_le_one :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    B.signatureForgerySuccessProb ≤ 1 :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary keeps
signature-forgery success on the fixed active target.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_signatureForgerySuccessProb_le_active128BitTarget :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    B.signatureForgerySuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

end ConcreteSourceHardProductionBoundarySignatureProjections

/--
The concrete-source hard-production package-level native boundary exposes the
active Path 2 handoff from full-signature success to the accepted-spend bound.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_fullSignatureSuccessProb_le_activePathForgeryBound
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    B.fullSignatureSuccessProb ≤ B.activePathForgeryBound :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary keeps the
accepted-spend active-path bound on the fixed active target.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_activePathForgeryBound_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    B.activePathForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary reaches the
fixed active target for full-signature success.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_fullSignatureSuccessProb_le_active128BitTarget
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    (toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof
      hardProductionFactsComplete).fullSignatureSuccessProb <=
    ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (toFullSignatureReductionCertificateWithConcreteExecutionSourceAndCurrentHardProductionFacts
      P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof C Source
      checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
      queryBudgetPricedProof multiTargetLossPricedProof hardProductionFactsComplete)

/--
The concrete-source hard-production package-level native boundary exposes the
full model/reduction/hash completion conjunction after boundary erasure.
-/
theorem toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts_fullSignatureSecurityClaimComplete
    (P : SelectedRustAcceptedSpendRemainingBreakPricingPackage)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (toSelectedAcceptedSpendNumericSource P).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (toSelectedRustAcceptedSpendSoundnessCertificate
          P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    let B :=
      toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
        P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof C Source
        checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
        queryBudgetPricedProof multiTargetLossPricedProof
        hardProductionFactsComplete
    B.keygenDistributionModeled ∧
      B.signingOracleModeled ∧
      B.completenessModeled ∧
      B.verifierRelationMatchesImplementation ∧
      B.fullForgeryImpliesAcceptedSpendForgery ∧
      B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      B.acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      B.acceptedSpendSoundness.lifecycleReplayClosed ∧
      B.acceptedSpendSoundness.noExportClosed ∧
      B.hashBoundary.checkedCoreFrameDomainSeparated ∧
      B.hashBoundary.hiddenSourceFrameDomainSeparated ∧
      B.hashBoundary.queryBudgetPriced ∧
      B.hashBoundary.multiTargetLossPriced :=
  (toNativeFullSignatureSecurityBoundaryWithConcreteExecutionSourceAndCurrentHardProductionFacts
    P productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof C Source
    checkedCoreFrameDomainSeparatedProof hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof multiTargetLossPricedProof
    hardProductionFactsComplete).fullSignatureSecurityClaimComplete

end SelectedRustAcceptedSpendRemainingBreakPricingPackage

/--
Selected accepted-spend numeric source obtained by erasing the trace-model
pricing witness.

This keeps the final AP2-N003 route on the faithful trace model while exposing
the generic source expected by the selected accepted-spend boundary.
-/
def selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource :=
  let W :=
    selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
  SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toSelectedAcceptedSpendNumericSource
    (SelectedRustConcreteAcceptedSpendPricingWitness.toConcreteNumericSourceCertificate W)

/--
Selected accepted-spend numeric source obtained from the faithful trace model
and the four hard current accepted-spend pricing facts.
-/
def selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource :=
  selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
      hHardPricing)

/--
Selected accepted-spend numeric source built directly from first-class WS7
remaining-closure evidence and the current hard pricing facts.
-/
def selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource :=
  selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist
    observe
    E.acceptedSpendBound
    E.remainingBreakBound
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakProb_le_bound E)
    (SelectedRustWS7RemainingClosureEvidence.acceptedSpendBound_le_2pow_neg129 E)
    (SelectedRustWS7RemainingClosureEvidence.remainingBreakBound_le_2pow_neg133 E)
    hPriced
    hHardPricing

/--
The direct WS7 numeric source keeps concrete numeric success on the fixed
active 128-bit target once the current hard pricing facts are supplied.
-/
theorem selectedRustAcceptedSpendNumericSource_successProb_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    let Source :=
      selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
    Source.pricedCertificate.successProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts,
    selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts,
    selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds,
    selectedRustConcreteAcceptedSpendPricingWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts]
    using
      selectedRustConcreteAcceptedSpendPricingWitness_successProb_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing

/--
The direct WS7 numeric source exposes the finite union-bound-to-component-budget
step used before the active-target consequence.
-/
theorem selectedRustAcceptedSpendNumericSource_unionBound_le_componentBudget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    let Source :=
      selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
    Source.pricedCertificate.unionBound ≤
      Source.pricedCertificate.acceptedSpendBound +
        (@Finset.univ Source.pricedCertificate.Break
          Source.pricedCertificate.breakFintype).sum
            Source.pricedCertificate.breakBound := by
  exact
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource.unionBound_le_componentBudget
      (selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing)

/--
The direct WS7 numeric source exposes the composed component-budget-to-priced
bound step.
-/
theorem selectedRustAcceptedSpendNumericSource_componentBudget_le_pricedBound_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    let Source :=
      selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
    Source.pricedCertificate.acceptedSpendBound +
        (@Finset.univ Source.pricedCertificate.Break
          Source.pricedCertificate.breakFintype).sum
            Source.pricedCertificate.breakBound ≤
      Source.pricedCertificate.pricedBound := by
  exact
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource.breakBudget_le_pricedBound
      (selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing)

/--
Selected accepted-spend certificate built from the faithful trace model and the
seven remaining cryptographic break bounds.

The only inputs beyond the pricing witness are the production probability pins
and the four closure propositions that the full-signature boundary consumes.
-/
def selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate :=
  let W :=
    selectedRustConcreteAcceptedSpendPricingWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
  let Source :=
    SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toSelectedAcceptedSpendNumericSource
      (SelectedRustConcreteAcceptedSpendPricingWitness.toConcreteNumericSourceCertificate W)
  SelectedRustConcreteAcceptedSpendPricingWitness.toSelectedRustAcceptedSpendSoundnessCertificate
    W
    Source.pricedCertificate.unionBound
    Source.pricedCertificate.pricedBound
    productionAcceptedSpendForgeryProb
    productionAcceptedSpendForgeryBound
    rfl
    rfl
    (by
      simpa [selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds,
        Source, W] using acceptedSpendForgeryProbPinned)
    (by
      simpa [selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds,
        Source, W] using acceptedSpendForgeryBoundPinned)
    hiddenSourceProofSemanticsClosed
    checkedCoreCompositionClosed
    lifecycleReplayClosed
    noExportClosed
    hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof
    lifecycleReplayClosedProof
    noExportClosedProof

/--
Selected accepted-spend certificate built from the faithful trace model and the
four hard current accepted-spend pricing facts.
-/
def selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate :=
  selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
      hHardPricing)
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    (by
      simpa [selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts]
        using acceptedSpendForgeryProbPinned)
    (by
      simpa [selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts]
        using acceptedSpendForgeryBoundPinned)
    hiddenSourceProofSemanticsClosed
    checkedCoreCompositionClosed
    lifecycleReplayClosed
    noExportClosed
    hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof
    lifecycleReplayClosedProof
    noExportClosedProof

/--
Erase the trace-model selected accepted-spend certificate into the active
accepted-spend soundness boundary.
-/
def selectedRustAcceptedSpendSoundnessBoundary_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    Path2NativeAccountDesign.NativeAcceptedSpendSoundnessBoundary :=
  (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof).toAcceptedSpendSoundnessBoundary

/--
Erase the trace-model selected accepted-spend certificate into the active
accepted-spend soundness boundary from the four hard current pricing facts.
-/
def selectedRustAcceptedSpendSoundnessBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    Path2NativeAccountDesign.NativeAcceptedSpendSoundnessBoundary :=
  (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof).toAcceptedSpendSoundnessBoundary

/--
The accepted-spend boundary erased from the trace-model constructor reaches the
fixed active 128-bit target.
-/
theorem selectedRustAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    let B :=
      selectedRustAcceptedSpendSoundnessBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget
    (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
The accepted-spend boundary erased from the trace-model constructor keeps the
exported accepted-spend bound at the fixed active 128-bit target.
-/
theorem selectedRustAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    let B :=
      selectedRustAcceptedSpendSoundnessBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
    (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
The hard-pricing accepted-spend boundary erased from the trace-model
constructor reaches the fixed active 128-bit target.
-/
theorem selectedRustAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    let B :=
      selectedRustAcceptedSpendSoundnessBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget
    (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
The hard-pricing accepted-spend boundary keeps the exported accepted-spend bound
at the fixed active 128-bit target.
-/
theorem selectedRustAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    let B :=
      selectedRustAcceptedSpendSoundnessBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
    (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
Selected full-reduction production witness built from the faithful trace-model
accepted-spend certificate.

This is the AP2-N003 handoff from accepted-spend numeric closure into the
full-signature reduction layer.  The selected full-reduction production facts,
hash-budget facts, production probability pin, and verifier/model obligations
remain explicit inputs.
-/
def selectedRustFullSignatureReductionProductionWitness_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  let acceptedSpendCertificate :=
    selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.ofAcceptedSpendAndReportBits
    acceptedSpendCertificate
    hashBoundary
    signatureForgerySuccessProb
    productionSignatureForgerySuccessProb
    signatureForgerySuccessProbPinned
    (by
      simpa [acceptedSpendCertificate] using
        signatureForgerySuccessMatchesAcceptedSpendGame)
    keygenDistributionModeled
    keygenDistributionModeledProof
    signingOracleModeled
    signingOracleModeledProof
    completenessModeled
    completenessModeledProof
    verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof
    checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof
    multiTargetLossPricedProof
    productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof
    acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof

/--
Selected full-signature reduction certificate built from the trace-model
accepted-spend certificate and the explicit production witness facts.
-/
def selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.toFullSignatureReductionCertificate
    (selectedRustFullSignatureReductionProductionWitness_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled
      signingOracleModeledProof completenessModeled completenessModeledProof
      verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof
      acceptedSpendProbabilityLinkedProof allActiveQuerySymbolsPricedProof
      verifierGameMatchesProductionExecutionProof)

/--
Erase the trace-model full-reduction certificate into the native
full-signature security boundary.
-/
def selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).toNativeFullSignatureSecurityBoundary

/--
Package the selected WS7 remaining-obligation evidence into the current
hard-pricing remaining-break package used by the full-signature route.
-/
def selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    SelectedRustAcceptedSpendRemainingBreakPricingPackage :=
  SelectedRustAcceptedSpendRemainingBreakPricingPackage.ofWS7RemainingClosureEvidence
    Trace traceDist observe E hPriced hHardPricing

/--
The direct WS7 numeric-source route exposes the same union-bound component as
the WS7 remaining-break pricing package route.
-/
theorem selectedRustAcceptedSpendNumericSource_unionBound_eq_package_unionBound_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    (selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing).pricedCertificate.unionBound =
    (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
      (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing)).pricedCertificate.unionBound := by
  rfl

/--
The direct WS7 numeric-source route exposes the same priced-bound component as
the WS7 remaining-break pricing package route.
-/
theorem selectedRustAcceptedSpendNumericSource_pricedBound_eq_package_pricedBound_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete) :
    (selectedRustAcceptedSpendNumericSource_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing).pricedCertificate.pricedBound =
    (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
      (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing)).pricedCertificate.pricedBound := by
  rfl

/--
Selected full-reduction production witness built from the first-class WS7
remaining-obligation evidence and the current hard pricing/production facts.
-/
def selectedRustFullSignatureReductionProductionWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
        (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
          traceDist observe E hPriced hHardPricing)).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
        (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
          traceDist observe E hPriced hHardPricing)).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedRustAcceptedSpendSoundnessCertificate
          (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
            traceDist observe E hPriced hHardPricing)
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (hardProductionFactsComplete :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  let P :=
    selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
  SelectedRustAcceptedSpendRemainingBreakPricingPackage.toFullSignatureReductionProductionWitnessWithCurrentHardProductionFacts
    P
    productionAcceptedSpendForgeryProb
    productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned
    acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed
    checkedCoreCompositionClosed
    lifecycleReplayClosed
    noExportClosed
    hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof
    lifecycleReplayClosedProof
    noExportClosedProof
    hashBoundary
    signatureForgerySuccessProb
    productionSignatureForgerySuccessProb
    signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame
    keygenDistributionModeled
    keygenDistributionModeledProof
    signingOracleModeled
    signingOracleModeledProof
    completenessModeled
    completenessModeledProof
    verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof
    checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof
    multiTargetLossPricedProof
    hardProductionFactsComplete

section WS7RemainingClosureAcceptedSpendBoundaryProjection

variable {Trace : Type} [Fintype Trace]
variable (traceDist : FiniteProbability Trace)
variable (observe : Trace → ConcreteOutcome)
variable (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
variable (hPriced :
  (1 : ℝ) / 2 ^ 128 ≤
    ((2 : ℝ) ^
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
variable (hHardPricing :
  Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
variable (productionAcceptedSpendForgeryProb : ℝ)
variable (productionAcceptedSpendForgeryBound : ℝ)
variable (acceptedSpendForgeryProbPinned :
  (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
    (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing)).pricedCertificate.unionBound =
    productionAcceptedSpendForgeryProb)
variable (acceptedSpendForgeryBoundPinned :
  (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
    (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing)).pricedCertificate.pricedBound =
    productionAcceptedSpendForgeryBound)
variable (hiddenSourceProofSemanticsClosed : Prop)
variable (checkedCoreCompositionClosed : Prop)
variable (lifecycleReplayClosed : Prop)
variable (noExportClosed : Prop)
variable (hiddenSourceProofSemanticsClosedProof :
  hiddenSourceProofSemanticsClosed)
variable (checkedCoreCompositionClosedProof :
  checkedCoreCompositionClosed)
variable (lifecycleReplayClosedProof :
  lifecycleReplayClosed)
variable (noExportClosedProof :
  noExportClosed)

/--
Erase the WS7 remaining-closure accepted-spend package into the selected
accepted-spend soundness certificate.
-/
def selectedRustAcceptedSpendSoundnessCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate :=
  SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedRustAcceptedSpendSoundnessCertificate
    (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing)
    productionAcceptedSpendForgeryProb
    productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned
    acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed
    checkedCoreCompositionClosed
    lifecycleReplayClosed
    noExportClosed
    hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof
    lifecycleReplayClosedProof
    noExportClosedProof

/--
Erase the WS7 remaining-closure accepted-spend certificate into the native
accepted-spend soundness boundary.
-/
def selectedRustAcceptedSpendSoundnessBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    Path2NativeAccountDesign.NativeAcceptedSpendSoundnessBoundary :=
  (selectedRustAcceptedSpendSoundnessCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    traceDist observe E hPriced hHardPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof).toAcceptedSpendSoundnessBoundary

/--
The WS7 remaining-closure accepted-spend boundary reaches the fixed active
128-bit probability target.
-/
theorem selectedRustAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustAcceptedSpendSoundnessBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget
    (selectedRustAcceptedSpendSoundnessCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

/--
The WS7 remaining-closure accepted-spend boundary keeps the exported
accepted-spend bound on the fixed active 128-bit target.
-/
theorem selectedRustAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustAcceptedSpendSoundnessBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof
    B.acceptedSpendForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
    (selectedRustAcceptedSpendSoundnessCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof)

end WS7RemainingClosureAcceptedSpendBoundaryProjection

section WS7RemainingClosureFullSignatureBoundaryProjection

variable {Trace : Type} [Fintype Trace]
variable (traceDist : FiniteProbability Trace)
variable (observe : Trace → ConcreteOutcome)
variable (E : SelectedRustWS7RemainingClosureEvidence Trace traceDist observe)
variable (hPriced :
  (1 : ℝ) / 2 ^ 128 ≤
    ((2 : ℝ) ^
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
variable (hHardPricing :
  Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
variable (productionAcceptedSpendForgeryProb : ℝ)
variable (productionAcceptedSpendForgeryBound : ℝ)
variable (acceptedSpendForgeryProbPinned :
  (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
    (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing)).pricedCertificate.unionBound =
    productionAcceptedSpendForgeryProb)
variable (acceptedSpendForgeryBoundPinned :
  (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedAcceptedSpendNumericSource
    (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing)).pricedCertificate.pricedBound =
    productionAcceptedSpendForgeryBound)
variable (hiddenSourceProofSemanticsClosed : Prop)
variable (checkedCoreCompositionClosed : Prop)
variable (lifecycleReplayClosed : Prop)
variable (noExportClosed : Prop)
variable (hiddenSourceProofSemanticsClosedProof :
  hiddenSourceProofSemanticsClosed)
variable (checkedCoreCompositionClosedProof :
  checkedCoreCompositionClosed)
variable (lifecycleReplayClosedProof :
  lifecycleReplayClosed)
variable (noExportClosedProof :
  noExportClosed)
variable (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
variable (signatureForgerySuccessProb : ℝ)
variable (productionSignatureForgerySuccessProb : ℝ)
variable (signatureForgerySuccessProbPinned :
  signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
variable (signatureForgerySuccessMatchesAcceptedSpendGame :
  signatureForgerySuccessProb =
    (SelectedRustAcceptedSpendRemainingBreakPricingPackage.toSelectedRustAcceptedSpendSoundnessCertificate
      (selectedRustAcceptedSpendRemainingBreakPricingPackage_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing)
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof).pricedNumericCertificate.successProb)
variable (keygenDistributionModeled : Prop)
variable (keygenDistributionModeledProof : keygenDistributionModeled)
variable (signingOracleModeled : Prop)
variable (signingOracleModeledProof : signingOracleModeled)
variable (completenessModeled : Prop)
variable (completenessModeledProof : completenessModeled)
variable (verifierRelationMatchesImplementation : Prop)
variable (verifierRelationMatchesImplementationProof :
  verifierRelationMatchesImplementation)
variable (fullForgeryImpliesAcceptedSpendForgery : Prop)
variable (fullForgeryImpliesAcceptedSpendForgeryProof :
  fullForgeryImpliesAcceptedSpendForgery)
variable (checkedCoreFrameDomainSeparatedProof :
  hashBoundary.checkedCoreFrameDomainSeparated)
variable (hiddenSourceFrameDomainSeparatedProof :
  hashBoundary.hiddenSourceFrameDomainSeparated)
variable (queryBudgetPricedProof :
  hashBoundary.queryBudgetPriced)
variable (multiTargetLossPricedProof :
  hashBoundary.multiTargetLossPriced)
variable (hardProductionFactsComplete :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.HardProductionFactsComplete)

/--
Erase the WS7 remaining-closure production witness into the selected
full-signature reduction certificate.
-/
def selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.toFullSignatureReductionCertificate
    (selectedRustFullSignatureReductionProductionWitness_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
Erase the WS7 remaining-closure full-reduction certificate into the native
full-signature security boundary.
-/
def selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  (selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    traceDist observe E hPriced hHardPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof hardProductionFactsComplete).toNativeFullSignatureSecurityBoundary

/--
The WS7 remaining-closure route preserves the accepted-spend reduction after
native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.signatureForgerySuccessProb ≤
      B.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route keeps signature forgery success nonnegative
after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    0 ≤ B.signatureForgerySuccessProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route keeps signature forgery success at most one
after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.signatureForgerySuccessProb ≤ 1 :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route reaches the fixed active 128-bit target for
signature forgery success after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.signatureForgerySuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route preserves the composed full-signature bound by
the active Path 2 forgery bound.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.fullSignatureSuccessProb ≤ B.activePathForgeryBound :=
  Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    (selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route keeps its active Path 2 bound handoff at the
fixed active 128-bit target after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.activePathForgeryBound ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route reaches the fixed active 128-bit target for
the composed full-signature success probability.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.fullSignatureSuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
      traceDist observe E hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof hardProductionFactsComplete)

/--
The WS7 remaining-closure route exposes the full model/reduction/hash
completion conjunction after erasing into the native full-signature boundary.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSecurityClaimComplete_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
        traceDist observe E hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof hardProductionFactsComplete
    B.keygenDistributionModeled ∧
      B.signingOracleModeled ∧
      B.completenessModeled ∧
      B.verifierRelationMatchesImplementation ∧
      B.fullForgeryImpliesAcceptedSpendForgery ∧
      B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      B.acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      B.acceptedSpendSoundness.lifecycleReplayClosed ∧
      B.acceptedSpendSoundness.noExportClosed ∧
      B.hashBoundary.checkedCoreFrameDomainSeparated ∧
      B.hashBoundary.hiddenSourceFrameDomainSeparated ∧
      B.hashBoundary.queryBudgetPriced ∧
      B.hashBoundary.multiTargetLossPriced :=
  (selectedRustNativeFullSignatureSecurityBoundary_of_ws7_remaining_closure_evidence_and_current_hard_pricing_facts
    traceDist observe E hPriced hHardPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof hardProductionFactsComplete).fullSignatureSecurityClaimComplete

end WS7RemainingClosureFullSignatureBoundaryProjection

/--
Selected full-reduction production witness built from the faithful trace-model
accepted-spend certificate and the current four hard accepted-spend pricing
facts.
-/
def selectedRustFullSignatureReductionProductionWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness :=
  selectedRustFullSignatureReductionProductionWitness_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced
    (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport_pricingFactsComplete_of_hardPricingFactsComplete
      hHardPricing)
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    (by
      simpa [selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts]
        using acceptedSpendForgeryProbPinned)
    (by
      simpa [selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts]
        using acceptedSpendForgeryBoundPinned)
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    (by
      simpa [selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts]
        using signatureForgerySuccessMatchesAcceptedSpendGame)
    keygenDistributionModeled keygenDistributionModeledProof
    signingOracleModeled signingOracleModeledProof completenessModeled
    completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof

/--
Selected full-signature reduction certificate built from the trace-model
accepted-spend certificate and the current four hard accepted-spend pricing
facts.
-/
def selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionProductionWitness.toFullSignatureReductionCertificate
    (selectedRustFullSignatureReductionProductionWitness_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
Erase the hard-pricing trace-model full-reduction certificate into the native
full-signature security boundary.
-/
def selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hHardPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).toNativeFullSignatureSecurityBoundary

/--
The erased full-signature boundary from the trace-model route preserves the
signature-success reduction to the accepted-spend probability.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb <=
      B.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The erased full-signature boundary from the trace-model route keeps signature
success nonnegative after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    0 <= B.signatureForgerySuccessProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The erased full-signature boundary from the trace-model route keeps signature
success at most one after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb <= 1 :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The erased full-signature boundary from the trace-model route reaches the fixed
active 128-bit target.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The erased full-signature boundary from the trace-model route exposes the
active Path 2 bound handoff through accepted-spend soundness.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb <= B.activePathForgeryBound :=
  Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    (selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The erased full-signature boundary from the trace-model route keeps its active
Path 2 bound handoff at the fixed active 128-bit target.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.activePathForgeryBound <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The erased full-signature boundary from the trace-model route reaches the fixed
active 128-bit target after composing the active Path 2 handoff.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget_of_remaining_break_bounds
    {Trace : Type} [Fintype Trace]
    (traceDist : FiniteProbability Trace)
    (observe : Trace → ConcreteOutcome)
    (acceptedSpendBound : ℝ)
    (remainingBreakBound : RemainingCryptographicBreak → ℝ)
    (hAccept :
      (concreteNumericGame selectedRustConcreteGameConfig traceDist
          observe).acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak :
      ∀ breakEvent : RemainingCryptographicBreak,
        (concreteNumericGame selectedRustConcreteGameConfig traceDist
            observe).breakProb breakEvent.1 ≤
          remainingBreakBound breakEvent)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : RemainingCryptographicBreak,
        remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
    (hPriced :
      (1 : ℝ) / 2 ^ 128 ≤
        ((2 : ℝ) ^
          (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
    (hPricing :
      Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
        productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
        productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed)
    (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
          traceDist observe acceptedSpendBound remainingBreakBound
          hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
          productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
          acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
          hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
          lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
          checkedCoreCompositionClosedProof lifecycleReplayClosedProof
          noExportClosedProof).pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
        true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
        true)
    (acceptedSpendProbabilityLinkedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
        true)
    (allActiveQuerySymbolsPricedProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
        true)
    (verifierGameMatchesProductionExecutionProof :
      Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
        true) :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

section FullSignatureCompletionProjection

variable {Trace : Type} [Fintype Trace]
variable (traceDist : FiniteProbability Trace)
variable (observe : Trace → ConcreteOutcome)
variable (acceptedSpendBound : ℝ)
variable (remainingBreakBound : RemainingCryptographicBreak → ℝ)
variable (hAccept :
  (concreteNumericGame selectedRustConcreteGameConfig traceDist
      observe).acceptedSpendProb ≤ acceptedSpendBound)
variable (hBreak :
  ∀ breakEvent : RemainingCryptographicBreak,
    (concreteNumericGame selectedRustConcreteGameConfig traceDist
        observe).breakProb breakEvent.1 ≤
      remainingBreakBound breakEvent)
variable (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
variable (hBreakBudget :
  ∀ breakEvent : RemainingCryptographicBreak,
    remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
variable (hPriced :
  (1 : ℝ) / 2 ^ 128 ≤
    ((2 : ℝ) ^
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
variable (hPricing :
  Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
variable (productionAcceptedSpendForgeryProb : ℝ)
variable (productionAcceptedSpendForgeryBound : ℝ)
variable (acceptedSpendForgeryProbPinned :
  (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.unionBound =
    productionAcceptedSpendForgeryProb)
variable (acceptedSpendForgeryBoundPinned :
  (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing).pricedCertificate.pricedBound =
    productionAcceptedSpendForgeryBound)
variable (hiddenSourceProofSemanticsClosed : Prop)
variable (checkedCoreCompositionClosed : Prop)
variable (lifecycleReplayClosed : Prop)
variable (noExportClosed : Prop)
variable (hiddenSourceProofSemanticsClosedProof :
  hiddenSourceProofSemanticsClosed)
variable (checkedCoreCompositionClosedProof :
  checkedCoreCompositionClosed)
variable (lifecycleReplayClosedProof :
  lifecycleReplayClosed)
variable (noExportClosedProof :
  noExportClosed)
variable (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
variable (signatureForgerySuccessProb : ℝ)
variable (productionSignatureForgerySuccessProb : ℝ)
variable (signatureForgerySuccessProbPinned :
  signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
variable (signatureForgerySuccessMatchesAcceptedSpendGame :
  signatureForgerySuccessProb =
    (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof).pricedNumericCertificate.successProb)
variable (keygenDistributionModeled : Prop)
variable (keygenDistributionModeledProof : keygenDistributionModeled)
variable (signingOracleModeled : Prop)
variable (signingOracleModeledProof : signingOracleModeled)
variable (completenessModeled : Prop)
variable (completenessModeledProof : completenessModeled)
variable (verifierRelationMatchesImplementation : Prop)
variable (verifierRelationMatchesImplementationProof :
  verifierRelationMatchesImplementation)
variable (fullForgeryImpliesAcceptedSpendForgery : Prop)
variable (fullForgeryImpliesAcceptedSpendForgeryProof :
  fullForgeryImpliesAcceptedSpendForgery)
variable (checkedCoreFrameDomainSeparatedProof :
  hashBoundary.checkedCoreFrameDomainSeparated)
variable (hiddenSourceFrameDomainSeparatedProof :
  hashBoundary.hiddenSourceFrameDomainSeparated)
variable (queryBudgetPricedProof :
  hashBoundary.queryBudgetPriced)
variable (multiTargetLossPricedProof :
  hashBoundary.multiTargetLossPriced)
variable (productionSignatureProbabilityPinnedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
    true)
variable (signatureSuccessMatchesAcceptedSpendGameProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
    true)
variable (acceptedSpendProbabilityLinkedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
    true)
variable (allActiveQuerySymbolsPricedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
    true)
variable (verifierGameMatchesProductionExecutionProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
    true)

/--
The trace-model route exposes the full model/reduction/hash completion
conjunction after erasing into the native full-signature boundary.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSecurityClaimComplete_of_remaining_break_bounds :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.keygenDistributionModeled ∧
      B.signingOracleModeled ∧
      B.completenessModeled ∧
      B.verifierRelationMatchesImplementation ∧
      B.fullForgeryImpliesAcceptedSpendForgery ∧
      B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      B.acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      B.acceptedSpendSoundness.lifecycleReplayClosed ∧
      B.acceptedSpendSoundness.noExportClosed ∧
      B.hashBoundary.checkedCoreFrameDomainSeparated ∧
      B.hashBoundary.hiddenSourceFrameDomainSeparated ∧
      B.hashBoundary.queryBudgetPriced ∧
      B.hashBoundary.multiTargetLossPriced :=
  (selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).fullSignatureSecurityClaimComplete

end FullSignatureCompletionProjection

section HardPricingFullSignatureProjection

variable {Trace : Type} [Fintype Trace]
variable (traceDist : FiniteProbability Trace)
variable (observe : Trace → ConcreteOutcome)
variable (acceptedSpendBound : ℝ)
variable (remainingBreakBound : RemainingCryptographicBreak → ℝ)
variable (hAccept :
  (concreteNumericGame selectedRustConcreteGameConfig traceDist
      observe).acceptedSpendProb ≤ acceptedSpendBound)
variable (hBreak :
  ∀ breakEvent : RemainingCryptographicBreak,
    (concreteNumericGame selectedRustConcreteGameConfig traceDist
        observe).breakProb breakEvent.1 ≤
      remainingBreakBound breakEvent)
variable (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
variable (hBreakBudget :
  ∀ breakEvent : RemainingCryptographicBreak,
    remainingBreakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133)
variable (hPriced :
  (1 : ℝ) / 2 ^ 128 ≤
    ((2 : ℝ) ^
      (Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport).acceptedSpendPricedBoundSecurityBits)⁻¹)
variable (hHardPricing :
  Path2NativeAccountDesign.selectedRustAcceptedSpendEvidenceReport.HardPricingFactsComplete)
variable (productionAcceptedSpendForgeryProb : ℝ)
variable (productionAcceptedSpendForgeryBound : ℝ)
variable (acceptedSpendForgeryProbPinned :
  (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.unionBound =
    productionAcceptedSpendForgeryProb)
variable (acceptedSpendForgeryBoundPinned :
  (selectedRustAcceptedSpendNumericSource_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing).pricedCertificate.pricedBound =
    productionAcceptedSpendForgeryBound)
variable (hiddenSourceProofSemanticsClosed : Prop)
variable (checkedCoreCompositionClosed : Prop)
variable (lifecycleReplayClosed : Prop)
variable (noExportClosed : Prop)
variable (hiddenSourceProofSemanticsClosedProof :
  hiddenSourceProofSemanticsClosed)
variable (checkedCoreCompositionClosedProof :
  checkedCoreCompositionClosed)
variable (lifecycleReplayClosedProof :
  lifecycleReplayClosed)
variable (noExportClosedProof :
  noExportClosed)
variable (hashBoundary : Path2NativeAccountDesign.NativeHashBudgetBoundary)
variable (signatureForgerySuccessProb : ℝ)
variable (productionSignatureForgerySuccessProb : ℝ)
variable (signatureForgerySuccessProbPinned :
  signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
variable (signatureForgerySuccessMatchesAcceptedSpendGame :
  signatureForgerySuccessProb =
    (selectedRustAcceptedSpendSoundnessCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof).pricedNumericCertificate.successProb)
variable (keygenDistributionModeled : Prop)
variable (keygenDistributionModeledProof : keygenDistributionModeled)
variable (signingOracleModeled : Prop)
variable (signingOracleModeledProof : signingOracleModeled)
variable (completenessModeled : Prop)
variable (completenessModeledProof : completenessModeled)
variable (verifierRelationMatchesImplementation : Prop)
variable (verifierRelationMatchesImplementationProof :
  verifierRelationMatchesImplementation)
variable (fullForgeryImpliesAcceptedSpendForgery : Prop)
variable (fullForgeryImpliesAcceptedSpendForgeryProof :
  fullForgeryImpliesAcceptedSpendForgery)
variable (checkedCoreFrameDomainSeparatedProof :
  hashBoundary.checkedCoreFrameDomainSeparated)
variable (hiddenSourceFrameDomainSeparatedProof :
  hashBoundary.hiddenSourceFrameDomainSeparated)
variable (queryBudgetPricedProof :
  hashBoundary.queryBudgetPriced)
variable (multiTargetLossPricedProof :
  hashBoundary.multiTargetLossPriced)
variable (productionSignatureProbabilityPinnedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned =
    true)
variable (signatureSuccessMatchesAcceptedSpendGameProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame =
    true)
variable (acceptedSpendProbabilityLinkedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked =
    true)
variable (allActiveQuerySymbolsPricedProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced =
    true)
variable (verifierGameMatchesProductionExecutionProof :
  Path2NativeAccountDesign.selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution =
    true)

/--
The hard-pricing trace-model route preserves the accepted-spend reduction after
native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb <=
      B.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route keeps signature forgery success nonnegative
after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    0 <= B.signatureForgerySuccessProb :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route keeps signature forgery success at most one
after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb <= 1 :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route reaches the fixed active 128-bit target for
signature forgery success after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.signatureForgerySuccessProb ≤ ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route preserves the composed full-signature bound
by the active Path 2 forgery bound.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb <= B.activePathForgeryBound :=
  Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    (selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route keeps its active Path 2 bound handoff at the
fixed active 128-bit target after native boundary erasure.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.activePathForgeryBound <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route reaches the fixed active 128-bit target for
the composed full-signature success probability.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.fullSignatureSuccessProb <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Path2NativeAccountDesign.SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (selectedRustFullSignatureReductionCertificate_of_remaining_break_bounds_and_current_hard_pricing_facts
      traceDist observe acceptedSpendBound remainingBreakBound
      hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
      productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
      acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
      hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
      lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
      checkedCoreCompositionClosedProof lifecycleReplayClosedProof
      noExportClosedProof hashBoundary signatureForgerySuccessProb
      productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
      signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
      keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
      completenessModeled completenessModeledProof verifierRelationMatchesImplementation
      verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
      fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
      hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
      multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
      signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
      allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof)

/--
The hard-pricing trace-model route exposes the full model/reduction/hash
completion conjunction after erasing into the native full-signature boundary.
-/
theorem selectedRustNativeFullSignatureSecurityBoundary_fullSignatureSecurityClaimComplete_of_remaining_break_bounds_and_current_hard_pricing_facts :
    let B :=
      selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
        traceDist observe acceptedSpendBound remainingBreakBound
        hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
        productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
        acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
        hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
        lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
        checkedCoreCompositionClosedProof lifecycleReplayClosedProof
        noExportClosedProof hashBoundary signatureForgerySuccessProb
        productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
        signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
        keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
        completenessModeled completenessModeledProof verifierRelationMatchesImplementation
        verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
        fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
        hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
        multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
        signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
        allActiveQuerySymbolsPricedProof verifierGameMatchesProductionExecutionProof
    B.keygenDistributionModeled ∧
      B.signingOracleModeled ∧
      B.completenessModeled ∧
      B.verifierRelationMatchesImplementation ∧
      B.fullForgeryImpliesAcceptedSpendForgery ∧
      B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      B.acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      B.acceptedSpendSoundness.lifecycleReplayClosed ∧
      B.acceptedSpendSoundness.noExportClosed ∧
      B.hashBoundary.checkedCoreFrameDomainSeparated ∧
      B.hashBoundary.hiddenSourceFrameDomainSeparated ∧
      B.hashBoundary.queryBudgetPriced ∧
      B.hashBoundary.multiTargetLossPriced :=
  (selectedRustNativeFullSignatureSecurityBoundary_of_remaining_break_bounds_and_current_hard_pricing_facts
    traceDist observe acceptedSpendBound remainingBreakBound
    hAccept hBreak hAcceptBudget hBreakBudget hPriced hHardPricing
    productionAcceptedSpendForgeryProb productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbPinned acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed checkedCoreCompositionClosed
    lifecycleReplayClosed noExportClosed hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof lifecycleReplayClosedProof
    noExportClosedProof hashBoundary signatureForgerySuccessProb
    productionSignatureForgerySuccessProb signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame keygenDistributionModeled
    keygenDistributionModeledProof signingOracleModeled signingOracleModeledProof
    completenessModeled completenessModeledProof verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof queryBudgetPricedProof
    multiTargetLossPricedProof productionSignatureProbabilityPinnedProof
    signatureSuccessMatchesAcceptedSpendGameProof acceptedSpendProbabilityLinkedProof
    allActiveQuerySymbolsPricedProof
    verifierGameMatchesProductionExecutionProof).fullSignatureSecurityClaimComplete

end HardPricingFullSignatureProjection

end

end Path2TraceModel

end Descender
