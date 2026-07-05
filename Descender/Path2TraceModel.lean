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

end HardPricingFullSignatureProjection

end

end Path2TraceModel

end Descender
