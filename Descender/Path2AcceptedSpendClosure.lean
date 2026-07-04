import Descender.Path2AcceptedSpendProbability

namespace Descender
namespace Path2AcceptedSpendProbability

/-!
# Path 2 Accepted-Spend Closure (fail-closed → closed-modulo-assumptions)

This module drives the accepted-spend numeric obligation from its fail-closed
(uninhabited certificate) state toward an honest *closed* shape. "Closed" here
means what a real EUF-CMA reduction means: the deployment bound is *derived* from
explicitly-named standard cryptographic per-event bounds, with every other step
(the union bound, the numeric budget arithmetic, and the zero-probability of the
state-machine break events) **proven** rather than assumed.

What is proven here, with no `sorry` and no vacuous `True`/empty-type cheat:
* the state-machine break events that the consume-once model makes impossible
  contribute probability `0` (`breakProb_eq_zero_of_never`);
* an explicit per-event bound budget that sums below a target yields
  `successProb ≤ target` (`successProb_le_target_of_bounds`), reusing the proven
  union-bound and pricing lemmas;
* the numeric budget actually closes at the deployed 128-bit target
  (`activeBudget_closes`, `successProb_le_2pow_neg128_of_proportional_bounds`).

What remains an *explicit hypothesis* (correctly — these are never proven
unconditionally): the accepted-spend forgery bound (SIS / response-link-coded
hardness) and the cryptographic break bounds (QROM / hash collision / domain
separation). Discharging those, and supplying a faithful concrete trace model
`G`, is the remaining WS7 work; this module is the closure skeleton they plug
into.
-/

namespace AcceptedSpendNumericGame

open Advantage

noncomputable section

variable {Trace Outcome Break : Type*}
variable [Fintype Trace] [Fintype Break] [DecidableEq Break]

omit [DecidableEq Break] in
/--
A named break that never fires under the game's trace model contributes zero
probability.  This is how the state-machine break events (consume-once replay /
rollback, post-challenge slot rewrite) are discharged: the model makes them
impossible, so they cost nothing in the budget.
-/
theorem breakProb_eq_zero_of_never
    (G : AcceptedSpendNumericGame Trace Outcome Break) (b : Break)
    (h : ∀ trace, G.namedBreak b (G.observe trace) = false) :
    G.breakProb b = 0 := by
  simp only [breakProb, FiniteProbability.acceptProb]
  apply Finset.sum_eq_zero
  intro trace _
  rw [h trace, boolIndicator_false, mul_zero]

/--
**Conditional closure.**  Given an explicit numeric bound for the accepted-spend
event and for each named break, plus the budget inequality that they sum below a
target, the full-signature success probability is below that target.

This is the honest endpoint shape: the accepted-spend / cryptographic-break
bounds are the standard cryptographic assumptions; this theorem composes them
into the deployment bound with no further assumption (it just repackages the
already-proven union-bound + pricing lemmas).
-/
theorem successProb_le_target_of_bounds
    (G : AcceptedSpendNumericGame Trace Outcome Break)
    (acceptedSpendBound : ℝ) (breakBound : Break → ℝ) (target : ℝ)
    (hAccept : G.acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak : ∀ b, G.breakProb b ≤ breakBound b)
    (hBudget : acceptedSpendBound + Finset.univ.sum breakBound ≤ target) :
    G.successProb ≤ target := by
  have hsplit :
      G.acceptedSpendOrBreakUnionBound =
        G.acceptedSpendProb + Finset.univ.sum G.breakProb :=
    G.acceptedSpendOrBreakUnionBound_eq_acceptedSpendProb_add_breakProb_sum
  have hbreaks : Finset.univ.sum G.breakProb ≤ Finset.univ.sum breakBound :=
    Finset.sum_le_sum (fun b _ => hBreak b)
  calc
    G.successProb ≤ G.acceptedSpendOrBreakUnionBound :=
        G.successProb_le_acceptedSpendOrBreakUnionBound
    _ = G.acceptedSpendProb + Finset.univ.sum G.breakProb := hsplit
    _ ≤ acceptedSpendBound + Finset.univ.sum breakBound := add_le_add hAccept hbreaks
    _ ≤ target := hBudget

end

end AcceptedSpendNumericGame

noncomputable section

variable {Trace Outcome : Type*}
variable [Fintype Trace]

open AcceptedSpendNumericGame

omit [Fintype Trace] in
/--
**Structural → numeric bridge.**  The numeric game's key hypothesis
`success_implies_accepted_or_break` is *derived* (not assumed) from the
already-proven structural soundness `DirectAcceptedSpendSoundness`, given only
the modeling-faithfulness conditions that the game's boolean events reflect the
structural predicates.  This moves the central reduction step from the assumed
column to the proven column: what remains assumed is the structural
soundness package and the per-event numeric bounds, not this implication.
-/
theorem success_implies_accepted_or_break_of_structural
    {T : Path2FullSpendGame.Types}
    {P : Path2FullSpendGame.Predicates T}
    (S : Path2FullSpendGame.DirectAcceptedSpendSoundness P)
    (observe : Trace → Outcome)
    (spendOf : Outcome → Path2FullSpendGame.SpendTuple T)
    (fullSignatureSuccess acceptedSpendForgery : Outcome → Bool)
    (namedBreak : Path2FullSpendGame.BreakEvent → Outcome → Bool)
    (hSuccess : ∀ o, fullSignatureSuccess o = true →
      P.verifyPath2SpendAuthorizationAccepts (spendOf o))
    (hAccepted : ∀ o,
      Path2FullSpendGame.AcceptedSpendConclusion P (spendOf o) →
        acceptedSpendForgery o = true)
    (hBreak : ∀ o b, P.breakFor (spendOf o) b → namedBreak b o = true) :
    ∀ trace, fullSignatureSuccess (observe trace) = true →
      acceptedSpendForgery (observe trace) = true ∨
        ∃ b, namedBreak b (observe trace) = true := by
  intro trace hsucc
  have haccept := hSuccess _ hsucc
  rcases Path2FullSpendGame.account_creation_prevalidated_slot_authorization_or_break
      S haccept with hconc | hbreak
  · exact Or.inl (hAccepted _ hconc)
  · rcases hbreak with ⟨b, hb⟩
    exact Or.inr ⟨b, hBreak _ b hb⟩

/--
**The numeric budget closes for the active 9-event break family, at any target.**
With the accepted-spend bound at half the target and each of the nine named
breaks at `1/32` of the target, the budget sums to `25/32` of the target, which
is below the target whenever it is nonnegative.  This discharges the final
numeric step for the active Path 2 break family with no remaining arithmetic
obligation.
-/
theorem activeBudget_closes
    (B : ℝ) (hB : 0 ≤ B)
    (acceptedSpendBound : ℝ)
    (breakBound : Path2FullSpendGame.BreakEvent → ℝ)
    (hAccept : acceptedSpendBound ≤ B / 2)
    (hBreak : ∀ b, breakBound b ≤ B / 32) :
    acceptedSpendBound + Finset.univ.sum breakBound ≤ B := by
  have hcard : (Finset.univ : Finset Path2FullSpendGame.BreakEvent).card = 9 := by
    decide
  have hsum :
      Finset.univ.sum breakBound ≤ 9 * (B / 32) := by
    calc
      Finset.univ.sum breakBound
          ≤ Finset.univ.sum (fun _ : Path2FullSpendGame.BreakEvent => B / 32) :=
            Finset.sum_le_sum (fun b _ => hBreak b)
      _ = (9 : ℝ) * (B / 32) := by
            rw [Finset.sum_const, hcard, nsmul_eq_mul]
            norm_num
  linarith

/--
Convenience closure: per-event bounds in the active proportions give
`successProb ≤ B` for the active break family.
-/
theorem successProb_le_target_of_proportional_bounds
    (G : AcceptedSpendNumericGame Trace Outcome Path2FullSpendGame.BreakEvent)
    (B : ℝ) (hB : 0 ≤ B)
    (acceptedSpendBound : ℝ)
    (breakBound : Path2FullSpendGame.BreakEvent → ℝ)
    (hAccept : G.acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak : ∀ b, G.breakProb b ≤ breakBound b)
    (hAcceptBudget : acceptedSpendBound ≤ B / 2)
    (hBreakBudget : ∀ b, breakBound b ≤ B / 32) :
    G.successProb ≤ B :=
  successProb_le_target_of_bounds G acceptedSpendBound breakBound B hAccept hBreak
    (activeBudget_closes B hB acceptedSpendBound breakBound hAcceptBudget hBreakBudget)

/--
**Concrete 128-bit closure.**  At the deployed target `2^-128`, if the
accepted-spend forgery probability is `≤ 2^-129` and each of the nine named
breaks is `≤ 2^-133`, then the full-signature success probability is `≤ 2^-128`.

Every step here is proven; the two numeric hypotheses are exactly the standard
cryptographic per-event bounds (accepted-spend SIS forgery, and the
QROM/hash/domain-separation breaks) that the remaining WS7 reductions must
supply.  This is the closed shape the fail-closed certificate is being driven
toward.
-/
theorem successProb_le_2pow_neg128_of_proportional_bounds
    (G : AcceptedSpendNumericGame Trace Outcome Path2FullSpendGame.BreakEvent)
    (acceptedSpendBound : ℝ)
    (breakBound : Path2FullSpendGame.BreakEvent → ℝ)
    (hAccept : G.acceptedSpendProb ≤ acceptedSpendBound)
    (hBreak : ∀ b, G.breakProb b ≤ breakBound b)
    (hAcceptBudget : acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget : ∀ b, breakBound b ≤ (1 : ℝ) / 2 ^ 133) :
    G.successProb ≤ (1 : ℝ) / 2 ^ 128 := by
  refine successProb_le_target_of_proportional_bounds G ((1 : ℝ) / 2 ^ 128)
    (by positivity) acceptedSpendBound breakBound hAccept hBreak ?_ ?_
  · -- 2^-129 = (2^-128)/2
    have : (1 : ℝ) / 2 ^ 128 / 2 = 1 / 2 ^ 129 := by norm_num
    rw [this]; exact hAcceptBudget
  · intro b
    have : (1 : ℝ) / 2 ^ 128 / 32 = 1 / 2 ^ 133 := by norm_num
    rw [this]; exact hBreakBudget b

end

namespace PricedAcceptedSpendNumericCertificate

/--
Packaged component-budget closure for the active selected break family.

This is the certificate-level form of the 128-bit arithmetic check: once the
priced certificate is pinned to the active nine-event break family, an
accepted-spend component at `2^-129` and every named break at `2^-133` compose
to a total component budget below the deployed `2^-128` target.
-/
theorem componentBudget_le_2pow_neg128_of_proportional_bounds
    (C : PricedAcceptedSpendNumericCertificate)
    (hAcceptBudget :
      C.acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : C.Break,
        C.breakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    C.acceptedSpendBound +
        (@Finset.univ C.Break C.breakFintype).sum C.breakBound ≤
      (1 : ℝ) / 2 ^ 128 := by
  cases C with
  | mk Trace Outcome Break traceFintype breakFintype breakDecidableEq
      activeBreakFamilyPinned game breakPricing pricedBound pricedBound_eq_breakPricing =>
    dsimp [acceptedSpendBound, breakBound] at hAcceptBudget hBreakBudget ⊢
    cases activeBreakFamilyPinned
    letI : Fintype Trace := traceFintype
    letI : Fintype Path2FullSpendGame.BreakEvent := breakFintype
    have hUniv :
        (@Finset.univ Path2FullSpendGame.BreakEvent breakFintype) =
          (@Finset.univ Path2FullSpendGame.BreakEvent
            Path2FullSpendGame.instFintypeBreakEvent) := by
      ext breakEvent
      simp
    rw [hUniv]
    refine activeBudget_closes ((1 : ℝ) / 2 ^ 128) (by positivity)
      breakPricing.acceptedSpendBound breakPricing.breakBound ?_ ?_
    · have : (1 : ℝ) / 2 ^ 128 / 2 = 1 / 2 ^ 129 := by norm_num
      rw [this]
      exact hAcceptBudget
    · intro breakEvent
      have : (1 : ℝ) / 2 ^ 128 / 32 = 1 / 2 ^ 133 := by norm_num
      rw [this]
      exact hBreakBudget breakEvent

/--
Packaged 128-bit closure for a priced accepted-spend numeric certificate.

This exposes the proved arithmetic to downstream selected/concrete evidence:
the certificate's component probabilities plus active break-family pin imply
the deployed success bound without assuming a monolithic priced inequality.
-/
theorem successProb_le_2pow_neg128_of_proportional_bounds
    (C : PricedAcceptedSpendNumericCertificate)
    (hAcceptBudget :
      C.acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : C.Break,
        C.breakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    C.successProb ≤ (1 : ℝ) / 2 ^ 128 :=
  le_trans C.successProb_le_componentBudget
    (C.componentBudget_le_2pow_neg128_of_proportional_bounds
      hAcceptBudget hBreakBudget)

end PricedAcceptedSpendNumericCertificate

namespace SelectedAcceptedSpendNumericSource

/--
Selected-source component-budget closure for the active Path 2 break family.

The concrete route can consume this theorem after it supplies the per-event
cryptographic bounds; the theorem itself only composes the already-packaged
certificate facts and the active nine-event arithmetic.
-/
theorem componentBudget_le_2pow_neg128_of_proportional_bounds
    (Source : SelectedAcceptedSpendNumericSource)
    (hAcceptBudget :
      Source.pricedCertificate.acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : Source.pricedCertificate.Break,
        Source.pricedCertificate.breakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    Source.pricedCertificate.acceptedSpendBound +
        (@Finset.univ Source.pricedCertificate.Break
          Source.pricedCertificate.breakFintype).sum
          Source.pricedCertificate.breakBound ≤
      (1 : ℝ) / 2 ^ 128 :=
  Source.pricedCertificate.componentBudget_le_2pow_neg128_of_proportional_bounds
    hAcceptBudget hBreakBudget

/--
Selected-source 128-bit success closure from proportional component bounds.
-/
theorem successProb_le_2pow_neg128_of_proportional_bounds
    (Source : SelectedAcceptedSpendNumericSource)
    (hAcceptBudget :
      Source.pricedCertificate.acceptedSpendBound ≤ (1 : ℝ) / 2 ^ 129)
    (hBreakBudget :
      ∀ breakEvent : Source.pricedCertificate.Break,
        Source.pricedCertificate.breakBound breakEvent ≤ (1 : ℝ) / 2 ^ 133) :
    Source.pricedCertificate.successProb ≤ (1 : ℝ) / 2 ^ 128 :=
  Source.pricedCertificate.successProb_le_2pow_neg128_of_proportional_bounds
    hAcceptBudget hBreakBudget

end SelectedAcceptedSpendNumericSource

end Path2AcceptedSpendProbability
end Descender
