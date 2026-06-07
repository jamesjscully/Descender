import Descender.SignatureForking
import Descender.Path2FullSpendGame

namespace Descender

/-!
# Path 2 Accepted-Spend Probability Layer

This module supplies the missing numeric shape between the proposition-level
full-spend theorem and the final native full-signature boundary.  It does not
choose the production bound.  Instead, it proves the finite union-bound step:
if full-signature success implies either an accepted-spend event or one of a
finite family of named break events, then the success probability is bounded by
the sum of those event probabilities.
-/

namespace Path2AcceptedSpendProbability

open Advantage

noncomputable section

variable {Trace Outcome Break : Type*}
variable [Fintype Trace] [Fintype Break] [DecidableEq Break]

/--
Finite accepted-spend numeric game.

`Trace` is the finite support sampled by the bounded experiment. `Outcome` is
the observed full-forgery run surface; it need not itself be finite.
`fullSignatureSuccess` is the event whose probability the final theorem wants
to bound. `acceptedSpendForgery` is the reduced event. `namedBreak` indexes the
finite set of separately priced bad events. The soundness field is exactly the
probabilistic version of the full-spend `or break` theorem.
-/
structure AcceptedSpendNumericGame (Trace Outcome Break : Type*)
    [Fintype Trace] [Fintype Break] where
  traceDist : FiniteProbability Trace
  observe : Trace -> Outcome
  fullSignatureSuccess : Outcome -> Bool
  acceptedSpendForgery : Outcome -> Bool
  namedBreak : Break -> Outcome -> Bool
  success_implies_accepted_or_break :
    forall trace,
      fullSignatureSuccess (observe trace) = true ->
        acceptedSpendForgery (observe trace) = true ∨
          exists breakEvent, namedBreak breakEvent (observe trace) = true

namespace AcceptedSpendNumericGame

/-- Option-indexed event family: `none` is accepted-spend, `some b` is break `b`. -/
def acceptedOrBreakEvent
    (G : AcceptedSpendNumericGame Trace Outcome Break) :
    Option Break -> Trace -> Bool
  | none => fun trace => G.acceptedSpendForgery (G.observe trace)
  | some breakEvent => fun trace => G.namedBreak breakEvent (G.observe trace)

/-- Probability of the full-signature success event in the numeric game. -/
def successProb
    (G : AcceptedSpendNumericGame Trace Outcome Break) : ℝ :=
  G.traceDist.acceptProb fun trace => G.fullSignatureSuccess (G.observe trace)

/-- Probability of the reduced accepted-spend event. -/
def acceptedSpendProb
    (G : AcceptedSpendNumericGame Trace Outcome Break) : ℝ :=
  G.traceDist.acceptProb fun trace => G.acceptedSpendForgery (G.observe trace)

/-- Probability of one named break event. -/
def breakProb
    (G : AcceptedSpendNumericGame Trace Outcome Break) (breakEvent : Break) : ℝ :=
  G.traceDist.acceptProb fun trace => G.namedBreak breakEvent (G.observe trace)

/--
Finite union-bound expression over the accepted-spend event and every named
break event.
-/
def acceptedSpendOrBreakUnionBound
    (G : AcceptedSpendNumericGame Trace Outcome Break) : ℝ :=
  Finset.univ.sum fun event =>
    G.traceDist.acceptProb (G.acceptedOrBreakEvent event)

/--
The core numeric handoff: proposition-level `accepted spend or named break`
soundness implies a probability bound by the accepted-spend/break union.
-/
theorem successProb_le_acceptedSpendOrBreakUnionBound
    (G : AcceptedSpendNumericGame Trace Outcome Break) :
    G.successProb <= G.acceptedSpendOrBreakUnionBound := by
  unfold successProb acceptedSpendOrBreakUnionBound
  exact
    SignatureForking.acceptProb_le_sum_of_exists
      G.traceDist (fun trace => G.fullSignatureSuccess (G.observe trace))
      G.acceptedOrBreakEvent
      (by
        intro trace hSuccess
        rcases G.success_implies_accepted_or_break trace hSuccess with
          hAccepted | hBreak
        · exact ⟨none, hAccepted⟩
        · rcases hBreak with ⟨breakEvent, hBreakEvent⟩
          exact ⟨some breakEvent, hBreakEvent⟩)

/--
If deployment pricing supplies a concrete upper bound for the union expression,
then the full-signature success probability is below that bound.
-/
theorem successProb_le_pricedBound
    (G : AcceptedSpendNumericGame Trace Outcome Break)
    {pricedBound : ℝ}
    (hPriced :
      G.acceptedSpendOrBreakUnionBound <= pricedBound) :
    G.successProb <= pricedBound :=
  le_trans G.successProb_le_acceptedSpendOrBreakUnionBound hPriced

end AcceptedSpendNumericGame

/--
Constructive pricing certificate for accepted-spend plus named breaks.

Instead of treating the union-bound pricing as one opaque real inequality, this
records a bound for the accepted-spend event, a bound for each active named
break, and the final budget inequality that the sum fits below the deployment
priced bound.
-/
structure AcceptedSpendBreakPricingCertificate
    (G : AcceptedSpendNumericGame Trace Outcome Break) where
  acceptedSpendBound : ℝ
  breakBound : Break -> ℝ
  pricedBound : ℝ
  acceptedSpendProb_le_bound :
    G.acceptedSpendProb <= acceptedSpendBound
  breakProb_le_bound :
    forall breakEvent, G.breakProb breakEvent <= breakBound breakEvent
  breakBudget_le_priced :
    acceptedSpendBound + Finset.univ.sum breakBound <= pricedBound

namespace AcceptedSpendBreakPricingCertificate

set_option linter.unusedSectionVars false in
/-- Accepted-spend plus per-break pricing bounds the full union expression. -/
theorem unionBound_le_priced
    {G : AcceptedSpendNumericGame Trace Outcome Break}
    (P : AcceptedSpendBreakPricingCertificate G) :
    G.acceptedSpendOrBreakUnionBound <= P.pricedBound := by
  have hUnionBudget :
      G.acceptedSpendOrBreakUnionBound <=
        P.acceptedSpendBound + Finset.univ.sum P.breakBound := by
    unfold AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
    rw [Fintype.sum_option]
    dsimp [AcceptedSpendNumericGame.acceptedOrBreakEvent]
    have hSome :
        (Finset.univ.sum fun breakEvent : Break =>
            G.traceDist.acceptProb fun trace =>
              G.namedBreak breakEvent (G.observe trace)) <=
          Finset.univ.sum P.breakBound := by
      exact Finset.sum_le_sum (by
        intro breakEvent _hBreakEvent
        simpa [AcceptedSpendNumericGame.breakProb] using
          P.breakProb_le_bound breakEvent)
    have hNone :
        G.traceDist.acceptProb
            (fun trace => G.acceptedSpendForgery (G.observe trace)) <=
          P.acceptedSpendBound := by
      simpa [AcceptedSpendNumericGame.acceptedSpendProb] using
        P.acceptedSpendProb_le_bound
    exact add_le_add hNone hSome
  exact le_trans hUnionBudget P.breakBudget_le_priced

end AcceptedSpendBreakPricingCertificate

/--
Existential package for a production-priced accepted-spend numeric game.

This is the object a selected implementation can eventually provide to the
native certificate layer: a concrete finite trace support, an observed
full-forgery outcome surface, a finite named-break index, the accepted-spend
numeric game over that trace support, and a proof that the finite union-bound
expression is below the deployment-priced bound.
-/
structure PricedAcceptedSpendNumericCertificate where
  Trace : Type
  Outcome : Type
  Break : Type
  traceFintype : Fintype Trace
  breakFintype : Fintype Break
  breakDecidableEq : DecidableEq Break
  activeBreakFamilyPinned : Break = Path2FullSpendGame.BreakEvent
  game : @AcceptedSpendNumericGame Trace Outcome Break traceFintype breakFintype
  breakPricing : AcceptedSpendBreakPricingCertificate game
  pricedBound : ℝ
  pricedBound_eq_breakPricing :
    pricedBound = breakPricing.pricedBound

namespace PricedAcceptedSpendNumericCertificate

/--
The priced numeric certificate must price the active Path 2 full-spend break
family, not an implementation-chosen or proof-convenient break index.
-/
theorem breakFamily_eq_activeFullSpendBreakEvent
    (C : PricedAcceptedSpendNumericCertificate) :
    C.Break = Path2FullSpendGame.BreakEvent :=
  C.activeBreakFamilyPinned

/-- The full-signature success probability of the packaged game. -/
def successProb (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.successProb
    C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.game

/-- Observe the concrete outcome associated with a finite trace. -/
def observe (C : PricedAcceptedSpendNumericCertificate)
    (trace : C.Trace) : C.Outcome :=
  letI : Fintype C.Trace := C.traceFintype
  letI : Fintype C.Break := C.breakFintype
  C.game.observe trace

/-- The accepted-spend event of the packaged game at one trace. -/
def acceptedSpendEvent (C : PricedAcceptedSpendNumericCertificate)
    (trace : C.Trace) : Bool :=
  letI : Fintype C.Trace := C.traceFintype
  letI : Fintype C.Break := C.breakFintype
  C.game.acceptedSpendForgery (C.observe trace)

/-- One named break event of the packaged game at one trace. -/
def namedBreakEvent (C : PricedAcceptedSpendNumericCertificate)
    (trace : C.Trace) (breakEvent : C.Break) : Bool :=
  letI : Fintype C.Trace := C.traceFintype
  letI : Fintype C.Break := C.breakFintype
  C.game.namedBreak breakEvent (C.observe trace)

/-- The reduced accepted-spend event probability of the packaged game. -/
def acceptedSpendProb (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.acceptedSpendProb
    C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.game

/-- The accepted-spend plus named-break union expression of the packaged game. -/
def unionBound (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
    C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.game

/-- The priced numeric certificate's union bound is derived from its break-pricing certificate. -/
theorem unionBound_le_priced
    (C : PricedAcceptedSpendNumericCertificate) :
    C.unionBound <= C.pricedBound := by
  letI : Fintype C.Trace := C.traceFintype
  letI : Fintype C.Break := C.breakFintype
  letI : DecidableEq C.Break := C.breakDecidableEq
  calc
    C.unionBound <= C.breakPricing.pricedBound :=
      C.breakPricing.unionBound_le_priced
    _ = C.pricedBound :=
      C.pricedBound_eq_breakPricing.symm

/-- The packaged accepted-spend event probability is below the union bound. -/
theorem acceptedSpendProb_le_unionBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.acceptedSpendProb <= C.unionBound := by
  letI : Fintype C.Trace := C.traceFintype
  letI : Fintype C.Break := C.breakFintype
  letI : DecidableEq C.Break := C.breakDecidableEq
  unfold acceptedSpendProb unionBound AcceptedSpendNumericGame.acceptedSpendProb
    AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
  exact
    Finset.single_le_sum
      (by
        intro event _hEvent
        exact C.game.traceDist.acceptProb_nonneg (C.game.acceptedOrBreakEvent event))
      (Finset.mem_univ (none : Option C.Break))

/-- The packaged accepted-spend event probability is below the priced bound. -/
theorem acceptedSpendProb_le_pricedBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.acceptedSpendProb <= C.pricedBound :=
  le_trans C.acceptedSpendProb_le_unionBound C.unionBound_le_priced

/-- The packaged full-signature success probability is below the union bound. -/
theorem successProb_le_unionBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.successProb <= C.unionBound := by
  exact
    @AcceptedSpendNumericGame.successProb_le_acceptedSpendOrBreakUnionBound
      C.Trace C.Outcome C.Break C.traceFintype C.breakFintype
      C.breakDecidableEq C.game

/-- The packaged full-signature success probability is below the priced bound. -/
theorem successProb_le_pricedBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.successProb <= C.pricedBound := by
  exact
    @AcceptedSpendNumericGame.successProb_le_pricedBound
      C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.breakDecidableEq
      C.game C.pricedBound C.unionBound_le_priced

end PricedAcceptedSpendNumericCertificate

/--
Proof-bearing source package for a selected accepted-spend numeric certificate.

The generic probability theorem cannot know the downstream concrete run type,
but the selected deployment route still needs to prove that the priced
certificate came from a checked outcome-carrier source and the active full-spend
break family.  Concrete modules instantiate `outcomeCarrierPinned` with their
exact carrier equality.
-/
structure SelectedAcceptedSpendNumericSource where
  pricedCertificate : PricedAcceptedSpendNumericCertificate
  outcomeCarrierPinned : Prop
  outcomeCarrierPinnedProof : outcomeCarrierPinned
  acceptedSpendEventPinned : Prop
  acceptedSpendEventPinnedProof : acceptedSpendEventPinned
  namedBreakEventsPinned : Prop
  namedBreakEventsPinnedProof : namedBreakEventsPinned
  breakFamilyPinnedProof :
    pricedCertificate.Break = Path2FullSpendGame.BreakEvent

namespace SelectedAcceptedSpendNumericSource

/-- The selected numeric source exposes the priced certificate it validates. -/
def toPricedAcceptedSpendNumericCertificate
    (Source : SelectedAcceptedSpendNumericSource) :
    PricedAcceptedSpendNumericCertificate :=
  Source.pricedCertificate

/-- The selected numeric source is pinned to the active full-spend break family. -/
theorem breakFamily_eq_activeFullSpendBreakEvent
    (Source : SelectedAcceptedSpendNumericSource) :
    Source.pricedCertificate.Break = Path2FullSpendGame.BreakEvent :=
  Source.breakFamilyPinnedProof

/-- The selected source carries a priced accepted-spend probability bound. -/
theorem acceptedSpendProb_le_pricedBound
    (Source : SelectedAcceptedSpendNumericSource) :
    Source.pricedCertificate.acceptedSpendProb <=
      Source.pricedCertificate.pricedBound :=
  Source.pricedCertificate.acceptedSpendProb_le_pricedBound

/-- The selected source carries the full-forgery success union-bound reduction. -/
theorem successProb_le_unionBound
    (Source : SelectedAcceptedSpendNumericSource) :
    Source.pricedCertificate.successProb <=
      Source.pricedCertificate.unionBound :=
  Source.pricedCertificate.successProb_le_unionBound

/-- The selected source carries a priced full-forgery success bound. -/
theorem successProb_le_pricedBound
    (Source : SelectedAcceptedSpendNumericSource) :
    Source.pricedCertificate.successProb <=
      Source.pricedCertificate.pricedBound :=
  Source.pricedCertificate.successProb_le_pricedBound

/-- The selected source carries the finite union-bound pricing proof. -/
theorem unionBound_le_priced
    (Source : SelectedAcceptedSpendNumericSource) :
    Source.pricedCertificate.unionBound <=
      Source.pricedCertificate.pricedBound :=
  Source.pricedCertificate.unionBound_le_priced

end SelectedAcceptedSpendNumericSource

end

end Path2AcceptedSpendProbability
end Descender
