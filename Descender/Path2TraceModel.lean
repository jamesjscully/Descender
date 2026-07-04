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

end

end Path2TraceModel

end Descender
