import Descender.SignatureForking

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

end

end Path2AcceptedSpendProbability
end Descender
