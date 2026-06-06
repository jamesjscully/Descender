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

variable {Outcome Break : Type*}
variable [Fintype Outcome] [Fintype Break] [DecidableEq Break]

/--
Finite accepted-spend numeric game.

`fullSignatureSuccess` is the event whose probability the final theorem wants
to bound. `acceptedSpendForgery` is the reduced event. `namedBreak` indexes the
finite set of separately priced bad events.  The soundness field is exactly the
probabilistic version of the full-spend `or break` theorem.
-/
structure AcceptedSpendNumericGame (Outcome Break : Type*)
    [Fintype Outcome] [Fintype Break] where
  traceDist : FiniteProbability Outcome
  fullSignatureSuccess : Outcome -> Bool
  acceptedSpendForgery : Outcome -> Bool
  namedBreak : Break -> Outcome -> Bool
  success_implies_accepted_or_break :
    forall outcome,
      fullSignatureSuccess outcome = true ->
        acceptedSpendForgery outcome = true ∨
          exists breakEvent, namedBreak breakEvent outcome = true

namespace AcceptedSpendNumericGame

/-- Option-indexed event family: `none` is accepted-spend, `some b` is break `b`. -/
def acceptedOrBreakEvent
    (G : AcceptedSpendNumericGame Outcome Break) :
    Option Break -> Outcome -> Bool
  | none => G.acceptedSpendForgery
  | some breakEvent => G.namedBreak breakEvent

/-- Probability of the full-signature success event in the numeric game. -/
def successProb
    (G : AcceptedSpendNumericGame Outcome Break) : ℝ :=
  G.traceDist.acceptProb G.fullSignatureSuccess

/-- Probability of the reduced accepted-spend event. -/
def acceptedSpendProb
    (G : AcceptedSpendNumericGame Outcome Break) : ℝ :=
  G.traceDist.acceptProb G.acceptedSpendForgery

/-- Probability of one named break event. -/
def breakProb
    (G : AcceptedSpendNumericGame Outcome Break) (breakEvent : Break) : ℝ :=
  G.traceDist.acceptProb (G.namedBreak breakEvent)

/--
Finite union-bound expression over the accepted-spend event and every named
break event.
-/
def acceptedSpendOrBreakUnionBound
    (G : AcceptedSpendNumericGame Outcome Break) : ℝ :=
  Finset.univ.sum fun event =>
    G.traceDist.acceptProb (G.acceptedOrBreakEvent event)

/--
The core numeric handoff: proposition-level `accepted spend or named break`
soundness implies a probability bound by the accepted-spend/break union.
-/
theorem successProb_le_acceptedSpendOrBreakUnionBound
    (G : AcceptedSpendNumericGame Outcome Break) :
    G.successProb <= G.acceptedSpendOrBreakUnionBound := by
  unfold successProb acceptedSpendOrBreakUnionBound
  exact
    SignatureForking.acceptProb_le_sum_of_exists
      G.traceDist G.fullSignatureSuccess G.acceptedOrBreakEvent
      (by
        intro outcome hSuccess
        rcases G.success_implies_accepted_or_break outcome hSuccess with
          hAccepted | hBreak
        · exact ⟨none, hAccepted⟩
        · rcases hBreak with ⟨breakEvent, hBreakEvent⟩
          exact ⟨some breakEvent, hBreakEvent⟩)

/--
If deployment pricing supplies a concrete upper bound for the union expression,
then the full-signature success probability is below that bound.
-/
theorem successProb_le_pricedBound
    (G : AcceptedSpendNumericGame Outcome Break)
    {pricedBound : ℝ}
    (hPriced :
      G.acceptedSpendOrBreakUnionBound <= pricedBound) :
    G.successProb <= pricedBound :=
  le_trans G.successProb_le_acceptedSpendOrBreakUnionBound hPriced

end AcceptedSpendNumericGame

/--
Existential package for a production-priced accepted-spend numeric game.

This is the object a selected implementation can eventually provide to the
native certificate layer: a concrete finite outcome space, a finite named-break
index, the accepted-spend numeric game over that space, and a proof that the
finite union-bound expression is below the deployment-priced bound.
-/
structure PricedAcceptedSpendNumericCertificate where
  Outcome : Type
  Break : Type
  outcomeFintype : Fintype Outcome
  breakFintype : Fintype Break
  breakDecidableEq : DecidableEq Break
  activeBreakFamilyPinned : Break = Path2FullSpendGame.BreakEvent
  game : @AcceptedSpendNumericGame Outcome Break outcomeFintype breakFintype
  pricedBound : ℝ
  unionBound_le_priced :
    @AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
      Outcome Break outcomeFintype breakFintype game <=
        pricedBound

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
    C.Outcome C.Break C.outcomeFintype C.breakFintype C.game

/-- The reduced accepted-spend event probability of the packaged game. -/
def acceptedSpendProb (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.acceptedSpendProb
    C.Outcome C.Break C.outcomeFintype C.breakFintype C.game

/-- The accepted-spend plus named-break union expression of the packaged game. -/
def unionBound (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
    C.Outcome C.Break C.outcomeFintype C.breakFintype C.game

/-- The packaged accepted-spend event probability is below the union bound. -/
theorem acceptedSpendProb_le_unionBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.acceptedSpendProb <= C.unionBound := by
  letI : Fintype C.Outcome := C.outcomeFintype
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

/-- The packaged full-signature success probability is below the priced bound. -/
theorem successProb_le_pricedBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.successProb <= C.pricedBound := by
  exact
    @AcceptedSpendNumericGame.successProb_le_pricedBound
      C.Outcome C.Break C.outcomeFintype C.breakFintype C.breakDecidableEq
      C.game C.pricedBound C.unionBound_le_priced

end PricedAcceptedSpendNumericCertificate

end

end Path2AcceptedSpendProbability
end Descender
