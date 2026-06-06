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
  pricedBound : ℝ
  unionBound_le_priced :
    @AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
      Trace Outcome Break traceFintype breakFintype game <=
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
    C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.game

/-- The reduced accepted-spend event probability of the packaged game. -/
def acceptedSpendProb (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.acceptedSpendProb
    C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.game

/-- The accepted-spend plus named-break union expression of the packaged game. -/
def unionBound (C : PricedAcceptedSpendNumericCertificate) : ℝ :=
  @AcceptedSpendNumericGame.acceptedSpendOrBreakUnionBound
    C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.game

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

/-- The packaged full-signature success probability is below the priced bound. -/
theorem successProb_le_pricedBound
    (C : PricedAcceptedSpendNumericCertificate) :
    C.successProb <= C.pricedBound := by
  exact
    @AcceptedSpendNumericGame.successProb_le_pricedBound
      C.Trace C.Outcome C.Break C.traceFintype C.breakFintype C.breakDecidableEq
      C.game C.pricedBound C.unionBound_le_priced

end PricedAcceptedSpendNumericCertificate

end

end Path2AcceptedSpendProbability
end Descender
