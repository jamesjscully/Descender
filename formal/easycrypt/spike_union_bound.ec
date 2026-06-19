(* =============================================================================
   EC-SPIKE: one game hop in EasyCrypt mirroring the proven Lean skeleton.
   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   GAP B DISCHARGE (2026-06-10): the OB-1 event coupling is no longer an axiom.

   PREVIOUS STATE (carried as axiom):
     axiom success_implies_split &m :
       Pr[G.success() @ &m : res]
       <= Pr[G.accepted() @ &m : res] + big ... Pr[G.break_(b)].
   This smuggled the WHOLE union bound (success/accept/break collapse + the
   finite additivity) into one imported probability inequality.

   NEW STATE (derived): the experiment is collapsed into ONE distribution `dout`
   over OUTCOMES, and success/accepted/break are deterministic boolean
   PROJECTIONS of the outcome (exactly as in Lean, where they are
   `fullSignatureSuccess (observe trace)`, `acceptedSpendForgery (observe trace)`,
   `namedBreak b (observe trace)`).  The union bound `success_le_union` is now
   PROVEN by EC's native `mu_le` (event-implication => probability monotonicity)
   composed with `mu_has_le` (the iterated `mu_or` union bound over `break_univ`).
   The ONLY residual import is the SMALLEST structural fact: a POINTWISE,
   probability-free event implication on outcomes
       structural_split o : success_pred o => accepted_pred o \/ has-break o
   which is the verbatim EC image of the Lean structural soundness theorem
   `account_creation_prevalidated_slot_authorization_or_break` pushed through the
   three faithfulness conditions of
   `success_implies_accepted_or_break_of_structural`.  It carries NO probability
   mass and is the irreducible Lean<->EC correspondence (the seam).

   See research/cryptanalysis/lean_ec_seam_discharge.md for the full
   correspondence map + reviewer checklist (GAP D).

   ---------------------------------------------------------------------------
   DISCHARGE LEDGER (what Z3 / the EC logic actually closed vs. what is axiom):

   PROVEN (no admit, closed by EC + Z3):
     * success_le_union     (Goal 1, the UNION BOUND ITSELF) — now DERIVED from
       the pointwise `structural_split` via `mu_le` + `mu_has_le`.  This is the
       GAP-B discharge: the coupling is no longer imported as a probability
       inequality; only the pointwise outcome implication remains.
     * break_zero_of_never  (Goal 2, OB-4 structural part) — full `byphoare`
       reduction Pr[...:res] = 0 from a never-firing `hoare` postcondition (the
       module variant), AND a pointwise `mu`-level variant.
     * success_le_target    (Goal 3) — the union-bound ASSEMBLY: finite-sum
       monotonicity (`Bigreal.ler_sum`) + `ler_add` + `ler_trans`, closed by smt.
     * sum_break_le_card    (helper) — Σ breakBound <= |breaks| * bound.
     * success_le_2pow_neg128 (Goal 3b) — concrete 2^-128 budget, 25/32<=1.

   AXIOM / IMPORTED (the irreducible seam, NOT provable by this spike):
     * structural_split — the POINTWISE event implication on outcomes (NO
       probability).  EC image of Lean `account_creation_prevalidated_slot_
       authorization_or_break` after the three faithfulness conditions.  This is
       strictly MORE PRIMITIVE than the old probability-inequality axiom: it is a
       pure predicate fact, and everything probabilistic above it is now PROVEN.
     * break_univ_uniq / break_univ_total / break_card9 — the `[Fintype Break]`
       + `card = 9` facts (Lean proves card by `decide`; here primitive DATA).
   THERE ARE NO `admit.` TACTICS in this file; every lemma closes with `qed`.
   ============================================================================= *)

require import AllCore List Real Distr StdBigop StdOrder.
import Bigreal.BRA.

(* GAP-B step 0 (2026-06-19): wrapped in an `abstract theory` so the parametric
   union-bound spine can be CLONED with concrete types (break_t <- break_event,
   etc.) by spine_endpoint.ec. Contents + proofs are UNCHANGED — only namespaced
   under `Spine` — so all prior facts (success_le_union, success_le_2pow_neg128, …)
   are preserved and machine-checked. No consumers imported the flat file, so this
   is non-breaking. *)
abstract theory Spine.

(* -----------------------------------------------------------------------------
   Outcome / break carriers.

   `outcome` is the EC image of the Lean `Outcome` reached as `observe trace`.
   In Lean the three event indicators are deterministic functions of the
   outcome; we model them identically here as boolean predicates on `outcome`.
   The randomness lives in the distribution `dout : outcome distr` (the EC image
   of `traceDist`/`FiniteProbability Trace` pushed through `observe`).
   --------------------------------------------------------------------------- *)
type outcome.
type break_t.                         (* Lean: BreakEvent (Fintype, card 9)    *)

(* The finite enumeration of all named breaks (Lean `Finset.univ`). *)
op break_univ : break_t list.
axiom break_univ_uniq  : uniq break_univ.
axiom break_univ_total : forall (b : break_t), b \in break_univ.
axiom break_card9      : size break_univ = 9.   (* theory-level so a clone can realize it *)

(* -----------------------------------------------------------------------------
   The three boolean event PROJECTIONS of the single experiment outcome.
   Lean:
     success_pred  o  <->  fullSignatureSuccess o
     accepted_pred o  <->  acceptedSpendForgery o
     break_pred  o b  <->  namedBreak b o
   --------------------------------------------------------------------------- *)
op success_pred  : outcome -> bool.
op accepted_pred : outcome -> bool.
op break_pred    : outcome -> break_t -> bool.

(* =============================================================================
   THE SEAM (GAP B): the SMALLEST structural fact, imported pointwise.

   This is the verbatim EC image of the Lean theorem
     account_creation_prevalidated_slot_authorization_or_break
   pushed through the three faithfulness conditions hSuccess/hAccepted/hBreak of
     success_implies_accepted_or_break_of_structural
   (Path2AcceptedSpendClosure.lean:109).  In Lean that theorem is DERIVED from
   the `DirectAcceptedSpendSoundness` package by case analysis; here it is the
   single carried fact.  It contains NO probability — it is a pure pointwise
   implication on outcomes.  Everything probabilistic is derived from it below.

   Lean:  ∀ trace, fullSignatureSuccess (observe trace) = true →
            acceptedSpendForgery (observe trace) = true ∨
              ∃ b, namedBreak b (observe trace) = true
   EC  :  ∀ o, success_pred o => accepted_pred o \/ has (break_pred o) break_univ
   (`∃ b ∈ univ` <-> `has (..) break_univ` by break_univ_total).               *)
axiom structural_split (o : outcome) :
  success_pred o => accepted_pred o \/ has (break_pred o) break_univ.

(* =============================================================================
   GOAL 1 — UNION BOUND  (now PROVEN, not imported)
   Lean: AcceptedSpendNumericGame.successProb_le_acceptedSpendOrBreakUnionBound
     G.successProb <= Pr[accepted] + sum_{b in univ} Pr[break b].

   With the single-experiment outcome model the coupling (OB-1) is in scope and
   discharged: the pointwise `structural_split` gives `success_pred` is contained
   in `accepted_pred \/ has break_pred`, so `mu_le` bounds Pr[success] by
   Pr[accepted \/ has-break], then `mu_or` + `mu_has_le` split it into
   Pr[accepted] + Σ Pr[break b].
   ============================================================================= *)
section UNION_BOUND.

declare const dout : outcome distr.

(* Pr[success] over the single experiment. *)
lemma success_le_union :
  mu dout success_pred
  <= mu dout accepted_pred
     + big predT (fun (b : break_t) => mu dout (fun o => break_pred o b)) break_univ.
proof.
  (* Step 1 (OB-1, the event coupling): success_pred is pointwise contained in
     accepted_pred \/ (has break over break_univ), by structural_split. *)
  have hcontain :
      mu dout success_pred
        <= mu dout (fun o => accepted_pred o \/ has (break_pred o) break_univ).
  + by apply mu_le => o _; apply structural_split.
  (* Step 2 (OB-3): split the disjunction by mu_or (<= additivity). *)
  have hor :
      mu dout (fun o => accepted_pred o \/ has (break_pred o) break_univ)
        <= mu dout accepted_pred
           + mu dout (fun o => has (break_pred o) break_univ).
  + by rewrite mu_or; smt(mu_bounded).
  (* Step 3 (OB-3): the finite union bound over break_univ by mu_has_le. *)
  have hhas :
      mu dout (fun o => has (break_pred o) break_univ)
        <= big predT (fun (b : break_t) => mu dout (fun o => break_pred o b)) break_univ
    by apply (mu_has_le break_pred dout break_univ).
  (* compose by transitivity + monotone addition (real arith closed by Z3). *)
  smt().
qed.

end section UNION_BOUND.

(* =============================================================================
   GOAL 2 — A NEVER-FIRING BREAK CONTRIBUTES 0  (single break-event reduction)
   Lean: breakProb_eq_zero_of_never.  Two EC images:
     (a) the pointwise mu-level form (matches the new outcome model);
     (b) the module/byphoare form (the original spike form, retained).
   ============================================================================= *)
section BREAK_ZERO.

declare const dout : outcome distr.

(* (a) mu-level: if the break predicate never holds on the support, its
   probability is 0.  EC image of Lean `breakProb_eq_zero_of_never`. *)
lemma break_zero_of_never_mu (b : break_t) :
  (forall o, break_pred o b = false) =>
  mu dout (fun o => break_pred o b) = 0%r.
proof.
  move=> hnever.
  rewrite (mu_eq dout _ pred0).
  + by move=> o; rewrite /pred0 hnever.
  by rewrite mu0.
qed.

end section BREAK_ZERO.

(* (b) module/byphoare form retained for the consume-once state-machine bridge:
   a procedure that never returns true has Pr 0. *)
module type BREAKGAME = {
  proc break_ (b : break_t) : bool
}.

section BREAK_ZERO_MODULE.
declare module G <: BREAKGAME.
lemma break_zero_of_never (b : break_t) &m :
  hoare [ G.break_ : arg = b ==> !res ] =>
  Pr[G.break_(b) @ &m : res] = 0%r.
proof.
  move=> hnever.
  byphoare (_ : arg = b ==> res) => //.
  hoare. conseq hnever => //.
qed.
end section BREAK_ZERO_MODULE.

(* =============================================================================
   GOAL 3 — CONDITIONAL CLOSURE  (compose into the deployment target)
   Lean: successProb_le_target_of_bounds.  Now stands on the PROVEN
   success_le_union (GAP B discharged), not on an axiom.
   ============================================================================= *)
section CLOSURE.

declare const dout : outcome distr.

(* The union bound is now the PROVEN lemma (instantiated at this dout), not an
   axiom.  We re-expose it locally to keep the closure proof self-contained. *)
lemma success_le_union_c :
  mu dout success_pred
  <= mu dout accepted_pred
     + big predT (fun (b : break_t) => mu dout (fun o => break_pred o b)) break_univ.
proof.
  (* identical derivation as Goal 1; instantiated at this section's dout *)
  have hcontain :
      mu dout success_pred
        <= mu dout (fun o => accepted_pred o \/ has (break_pred o) break_univ).
  + by apply mu_le => o _; apply structural_split.
  have hor :
      mu dout (fun o => accepted_pred o \/ has (break_pred o) break_univ)
        <= mu dout accepted_pred
           + mu dout (fun o => has (break_pred o) break_univ).
  + by rewrite mu_or; smt(mu_bounded).
  have hhas :
      mu dout (fun o => has (break_pred o) break_univ)
        <= big predT (fun (b : break_t) => mu dout (fun o => break_pred o b)) break_univ
    by apply (mu_has_le break_pred dout break_univ).
  smt().
qed.

lemma success_le_target
    (acceptedSpendBound : real) (breakBound : break_t -> real) (target : real) :
  (* hAccept *)  mu dout accepted_pred <= acceptedSpendBound =>
  (* hBreak  *)  (forall b, mu dout (fun o => break_pred o b) <= breakBound b) =>
  (* hBudget *)  acceptedSpendBound + big predT breakBound break_univ <= target =>
  (* concl   *)  mu dout success_pred <= target.
proof.
  move=> hAccept hBreak hBudget.
  have hbreaks :
      big predT (fun b => mu dout (fun o => break_pred o b)) break_univ
        <= big predT breakBound break_univ.
  + apply Bigreal.ler_sum => b _ /=; exact hBreak.
  have hsplit := success_le_union_c.
  move: hAccept hbreaks hsplit hBudget; smt().
qed.

(* -----------------------------------------------------------------------------
   GOAL 3b — concrete 128-bit closure.  (break_card9 is now theory-level, above.)
   --------------------------------------------------------------------------- *)
lemma sum_break_le_card (breakBound : break_t -> real) (c : real) :
  (forall b, breakBound b <= c) =>
  big predT breakBound break_univ <= (size break_univ)%r * c.
proof.
  move=> hBrk.
  have <-: big predT (fun (_ : break_t) => c) break_univ
             = (size break_univ)%r * c.
  + by rewrite Bigreal.sumr_const count_predT.
  by apply Bigreal.ler_sum => b _ /=; exact hBrk.
qed.

lemma success_le_2pow_neg128
    (acceptedSpendBound : real) (breakBound : break_t -> real) :
  mu dout accepted_pred <= acceptedSpendBound =>
  (forall b, mu dout (fun o => break_pred o b) <= breakBound b) =>
  acceptedSpendBound <= 1%r / (2%r ^ 129) =>
  (forall b, breakBound b <= 1%r / (2%r ^ 133)) =>
  mu dout success_pred <= 1%r / (2%r ^ 128).
proof.
  move=> hAccept hBreak hAcc129 hBrk133.
  apply (success_le_target acceptedSpendBound breakBound (1%r / 2%r ^ 128)
           hAccept hBreak).
  have hsum := sum_break_le_card breakBound (1%r / 2%r ^ 133) hBrk133.
  rewrite break_card9 in hsum.
  have e129 : 2%r ^ 129 = 2%r ^ 128 * 2%r.
  + by rewrite (_ : 129 = 128 + 1) 1:// RField.exprD_nneg // RField.expr1.
  have e133 : 2%r ^ 133 = 2%r ^ 128 * 32%r.
  + rewrite (_ : 133 = 128 + 5) 1:// RField.exprD_nneg //.
    by rewrite (_ : 5 = 1+1+1+1+1) 1:// !RField.exprD_nneg // !RField.expr1.
  have pos128 : 0%r < 2%r ^ 128 by apply RealOrder.expr_gt0.
  move: hAcc129 hsum e129 e133 pos128; smt().
qed.

end section CLOSURE.

(* =============================================================================
   WHAT REMAINS (the irreducible seam, after GAP B discharge):
     * structural_split — the POINTWISE outcome-level event implication.  This is
       the EC image of the Lean structural soundness theorem and is the single
       carried correspondence.  It carries NO probability mass; everything
       probabilistic (the union bound, the never-firing =0, the budget) is PROVEN.
     * break_univ_{uniq,total}, break_card9 — Fintype/card DATA.
   MACHINE-CHECKED IN THIS FILE (EasyCrypt 2026.05 + Z3 4.16.0, exit 0):
     * success_le_union     — UNION BOUND, now DERIVED via mu_le + mu_or + mu_has_le.
     * break_zero_of_never{,_mu} — never-firing =0 (Goal 2/OB-4), mu + byphoare.
     * success_le_target    — union-bound ASSEMBLY (Bigreal.ler_sum + smt) (Goal 3).
     * sum_break_le_card     — Σ bound via Bigreal.sumr_const + ler_sum.
     * success_le_2pow_neg128 — concrete 2^-128 budget, 25/32<=1 by Z3.
   See research/cryptanalysis/lean_ec_seam_discharge.md (correspondence + GAP D
   reviewer checklist) and SPIKE_VERDICT.md.
   ============================================================================= *)

end Spine.
