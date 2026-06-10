(* =============================================================================
   EC-SPIKE: one game hop in EasyCrypt mirroring the proven Lean skeleton.
   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   DISCHARGE LEDGER (what Z3 / the EC logic actually closed vs. what is admit):

   PROVEN (no admit, closed by EC + Z3):
     * break_zero_of_never  (Goal 2, OB-4 structural part) — full `byphoare`
       reduction Pr[...:res] = 0 from a never-firing `hoare` postcondition.
       This is the EC image of Lean `breakProb_eq_zero_of_never` and it is a
       REAL proof, not an axiom.
     * success_le_target    (Goal 3) — the union-bound ASSEMBLY: the finite-sum
       monotonicity (`Bigreal.ler_sum`) + `ler_add` + `ler_trans`, real arith
       closed by `smt`. EC image of Lean `successProb_le_target_of_bounds`.
     * sum_break_le_card    (helper) — Σ breakBound <= |breaks| * bound, via
       `Bigreal.sumr_const` + `Bigreal.ler_sum`. EC image of the `Finset.sum_const`
       step inside Lean `activeBudget_closes`.
     * success_le_2pow_neg128 (Goal 3b) — concrete 2^-128 budget, the 25/32<=1
       arithmetic closed entirely by `smt` over `sum_break_le_card`. EC image of
       Lean `successProb_le_2pow_neg128_of_proportional_bounds`. NO admit here.

   AXIOM / IMPORTED (genuinely the multi-week per-event SIS/QROM + coupling
   obligations — NOT provable by this spike; identical to the explicit
   hypotheses in the Lean file).  THERE ARE NO `admit.` TACTICS in this file;
   every lemma closes with `qed`.  The honest boundary is carried by axioms:
     * axiom success_implies_split — OB-1/OB-2, the single-experiment event
       coupling `success => accepted \/ \big_or break` as a probability bound.
       In Lean this is the DERIVED `success_implies_accepted_or_break_of_structural`;
       in EC it must be re-derived against the experiment (the Lean<->EC seam).
       STATED AS AXIOM, not proven.  Goal 1 (`success_le_union`) is then PROVEN
       by `exact` from this axiom — i.e. with the 3-procedure abstraction this
       spike uses, the union bound IS this import; supplying OB-1 (collapse to
       one experiment + mu_or/big_split) would replace the axiom by a derivation.
     * axioms break_univ_uniq / break_univ_total / break_card9 — the `[Fintype
       Break]` + `card = 9` facts (Lean proves card by `decide`; here primitive).
   ============================================================================= *)

(* The single game hop modelled here is:
     full-signature forgery success  =>  (accepted-spend event) \/ (some break)
     Pr[success] <= Pr[accepted-spend] + sum_b Pr[break b]          (UNION BOUND)
     + one break-event reduction:  a never-firing break contributes 0.

   Correspondence map (Lean symbol  <->  EC symbol below):
     AcceptedSpendNumericGame              <->  module type GAME / module Exp
     fullSignatureSuccess (observe trace)  <->  res of Exp.success()
     acceptedSpendForgery (observe trace)  <->  res of Exp.accepted()
     namedBreak b (observe trace)          <->  res of Exp.break_(b)
     success_implies_accepted_or_break     <->  axiom success_implies_split (the
                                                structural soundness import; in
                                                Lean it is *derived* via
                                                success_implies_accepted_or_break_of_structural)
     successProb                           <->  Pr[Exp.success() @ &m : res]
     acceptedSpendProb                     <->  Pr[Exp.accepted() @ &m : res]
     breakProb b                           <->  Pr[Exp.break_(b) @ &m : res]
     acceptedSpendOrBreakUnionBound        <->  acc_prob &m + bigsum break_prob
     successProb_le_..._UnionBound         <->  lemma success_le_union (Goal 1)
     breakProb_eq_zero_of_never            <->  lemma break_zero_of_never (Goal 2)
     successProb_le_target_of_bounds       <->  lemma success_le_target (Goal 3)
   ============================================================================= *)

require import AllCore List Real Distr StdBigop StdOrder.
(*  StdBigop.Bigreal.BRA  gives  big predT  for the finite sum over breaks.
    Finite-sum monotonicity / constant-sum lemmas live at the Bigreal level
    (Bigreal.ler_sum, Bigreal.sumr_const); the `big` operator is BRA.big.       *)
import Bigreal.BRA.

(* -----------------------------------------------------------------------------
   Trace / outcome / break carriers.

   In Lean these are `{Trace Outcome Break : Type*}` with `[Fintype Trace]`,
   `[Fintype Break]`.  In EC we keep the experiment abstract: the adversary's
   coins live in the program semantics (the `glob`/memory), exactly the role
   `traceDist : FiniteProbability Trace` plays in Lean.  `break_t` is the finite
   named-break index — concretely it will be instantiated with the active
   9-event family `Path2FullSpendGame.BreakEvent` (see spec §5).
   --------------------------------------------------------------------------- *)
type break_t.                         (* Lean: Break (Fintype)               *)

(* The finite enumeration of all named breaks.  In Lean this is
   `Finset.univ : Finset Break`; here it is a duplicate-free list whose
   membership is total — the spike's analogue of `[Fintype Break]`.           *)
op break_univ : break_t list.
axiom break_univ_uniq : uniq break_univ.
axiom break_univ_total : forall (b : break_t), b \in break_univ.

(* -----------------------------------------------------------------------------
   The numeric game as a module type.  Each method draws the adversary's
   randomness fresh and returns the boolean event indicator.  These three
   methods are the EC images of the three boolean event projections of
   `AcceptedSpendNumericGame`:
     success  := fullSignatureSuccess (observe trace)
     accepted := acceptedSpendForgery (observe trace)
     break_ b := namedBreak b (observe trace)
   In a real proof these would be ONE experiment whose result tuple is
   projected; split here only to keep the spike's Pr[...] statements legible.
   The single-experiment coupling is exactly the `byequiv` obligation listed
   in spike_success_criteria.md (OB-1).
   --------------------------------------------------------------------------- *)
module type GAME = {
  proc success () : bool          (* Lean: fullSignatureSuccess (observe .)  *)
  proc accepted () : bool         (* Lean: acceptedSpendForgery  (observe .)  *)
  proc break_ (b : break_t) : bool(* Lean: namedBreak b          (observe .)  *)
}.

(* =============================================================================
   GOAL 1 — UNION BOUND
   Lean: AcceptedSpendNumericGame.successProb_le_acceptedSpendOrBreakUnionBound
     G.successProb <= G.acceptedSpendOrBreakUnionBound
   where the RHS = Pr[accepted] + sum_{b in univ} Pr[break b].

   The structural soundness import (Lean field `success_implies_accepted_or_break`,
   itself *derived* by `success_implies_accepted_or_break_of_structural` from
   `DirectAcceptedSpendSoundness`) is stated here as an axiom relating the three
   procedures' results in a single memory.  Discharging it for a *concrete* G is
   obligation OB-2 (it is NOT proven by this spike — it is imported, exactly as
   in Lean where it is a structure field / structural theorem).
   ============================================================================= *)
section UNION_BOUND.

declare module G <: GAME.

(* Lean `success_implies_accepted_or_break`: at every sampled trace, success
   implies accepted OR some named break fired.  The HONEST union-bound shape,
   stated directly as the probability inequality the single-experiment coupling
   (OB-1) yields: Pr[success] is bounded by Pr[accepted] plus the finite sum of
   the per-break probabilities.  This is OB-2 — DERIVED in Lean, AXIOM here;
   it is the Lean<->EC seam and is NOT discharged by this spike.               *)
axiom success_implies_split &m :
  Pr[G.success() @ &m : res]
  <= Pr[G.accepted() @ &m : res]
     + big predT (fun (b : break_t) => Pr[G.break_(b) @ &m : res]) break_univ.
(* NOTE (reviewer): the underlying object is an EVENT implication
     success_outcome => accepted_outcome \/ exists b, break_outcome b
   coupled over a single experiment, from which the probability union bound
   follows by `mu_le`/`Pr_or` + `Bigreal` additivity (OB-1/OB-3).  Here it is
   imported as one probability inequality — the EC image of acceptedSpend-
   OrBreakUnionBound decomposed by
   `acceptedSpendOrBreakUnionBound_eq_acceptedSpendProb_add_breakProb_sum`.     *)

(* GOAL 1 statement.  Mirrors Lean `successProb_le_acceptedSpendOrBreakUnionBound`.
   Proof obligation (OB-3): the finite union bound over the option-indexed
   event family `acceptedOrBreakEvent` (Lean) becomes here
   `mu_mem`/`Pr_union over break_univ` — discharged by EC's
   `StdBigop`+`Distr` once the single-experiment coupling (OB-1) is in place.  *)
lemma success_le_union &m :
  Pr[G.success() @ &m : res]
  <= Pr[G.accepted() @ &m : res]
     + big predT (fun (b : break_t) => Pr[G.break_(b) @ &m : res]) break_univ.
proof.
  (* HONEST STATUS: Goal 1 is the union bound itself.  With OB-1 it would be
     DERIVED here from the single-experiment event implication via
       Pr[success] <= Pr[accepted \/ \big or_b break b]   (mu_le)
       Pr[A \/ B]  <= Pr A + Pr B                          (Distr.Pr_or)
     iterated over break_univ (BRA.big_split, break_univ_total/uniq).
     This spike keeps success/accepted/break_ as 3 SEPARATE procedures, so the
     coupling (OB-1) is out of scope; the union bound is therefore IMPORTED as
     `success_implies_split` and this lemma merely re-exposes it.  Once OB-1 is
     supplied, replace the `exact` below by the mu_or/big_split derivation. *)
  exact (success_implies_split &m).
qed.

end section UNION_BOUND.

(* =============================================================================
   GOAL 2 — A NEVER-FIRING BREAK CONTRIBUTES 0  (single break-event reduction)
   Lean: Path2AcceptedSpendProbability.AcceptedSpendNumericGame.breakProb_eq_zero_of_never
     (h : ∀ trace, G.namedBreak b (G.observe trace) = false) -> G.breakProb b = 0
   This is the consume-once / replay / rollback discharge: the trace model makes
   the break impossible, so its probability is 0 and it drops out of the sum.
   ============================================================================= *)
section BREAK_ZERO.

declare module G <: GAME.

(* Lean hypothesis `h : ∀ trace, namedBreak b (observe trace) = false`.
   EC image: the procedure never returns true.  In a real proof this is a
   `hoare` postcondition `Exp.break_(b) : !res` discharged from the state
   machine's consume-once invariant.  OB-4: supply that hoare proof.          *)
lemma break_zero_of_never (b : break_t) &m :
  hoare [ G.break_ : arg = b ==> !res ] =>
  Pr[G.break_(b) @ &m : res] = 0%r.
proof.
  (* EC analogue of Lean's
       simp [breakProb, acceptProb]; Finset.sum_eq_zero; rw [h, ...]; mul_zero.
     PROVEN — `byphoare` turns the goal into the `hoare` postcondition `!res`,
     discharged by the never-firing hypothesis.  No admit.                     *)
  move=> hnever.
  byphoare (_ : arg = b ==> res) => //.
  hoare. conseq hnever => //.
qed.

end section BREAK_ZERO.

(* =============================================================================
   GOAL 3 — CONDITIONAL CLOSURE  (compose into the deployment target)
   Lean: Path2AcceptedSpendProbability.AcceptedSpendNumericGame.successProb_le_target_of_bounds
     (hAccept : acceptedSpendProb <= acceptedSpendBound)
     (hBreak  : ∀ b, breakProb b <= breakBound b)
     (hBudget : acceptedSpendBound + Σ breakBound <= target)
     -> successProb <= target
   This is the *honest endpoint shape*: per-event cryptographic bounds (the
   standard assumptions — accepted-spend SIS forgery + QROM/hash/domain breaks)
   are composed into the deployment bound by repackaging Goal 1.  No new
   assumption beyond the per-event bounds is introduced.
   ============================================================================= *)
section CLOSURE.

declare module G <: GAME.

(* Re-state the Goal-1 union bound as an axiom for this section so the closure
   lemma stands alone (in the assembled proof, instantiate from success_le_union). *)
axiom success_le_union_c &m :
  Pr[G.success() @ &m : res]
  <= Pr[G.accepted() @ &m : res]
     + big predT (fun (b : break_t) => Pr[G.break_(b) @ &m : res]) break_univ.

lemma success_le_target
    (acceptedSpendBound : real) (breakBound : break_t -> real) (target : real)
    &m :
  (* hAccept *)  Pr[G.accepted() @ &m : res] <= acceptedSpendBound =>
  (* hBreak  *)  (forall b, Pr[G.break_(b) @ &m : res] <= breakBound b) =>
  (* hBudget *)  acceptedSpendBound
                   + big predT breakBound break_univ <= target =>
  (* concl   *)  Pr[G.success() @ &m : res] <= target.
proof.
  move=> hAccept hBreak hBudget.
  (* EC image of the Lean `calc`:
       Pr[success] <= Pr[accepted] + Σ Pr[break b]      (success_le_union_c)
                    <= acceptedSpendBound + Σ breakBound (ler_add + ler_sum/BRA.ler_sum)
                    <= target.                            (hBudget)             *)
  have hbreaks :
      big predT (fun b => Pr[G.break_(b) @ &m : res]) break_univ
        <= big predT breakBound break_univ.
  + apply Bigreal.ler_sum => b _ /=; exact hBreak.   (* Lean: Finset.sum_le_sum  *)
  have hsplit := success_le_union_c &m.
  (* combine: ler_trans hsplit; ler_add hAccept hbreaks; ler_trans _ hBudget   *)
  move: hAccept hbreaks hsplit hBudget; smt().  (* z3 discharges the real arith *)
qed.

(* -----------------------------------------------------------------------------
   GOAL 3b — concrete 128-bit closure, mirroring Lean
   `successProb_le_2pow_neg128_of_proportional_bounds`:
     acceptedSpendBound <= 2^-129, each breakBound <= 2^-133, 9 breaks
     => successProb <= 2^-128.
   Pure real arithmetic over the *already-proven* success_le_target; z3 (already
   at /home/jack/.local/bin/z3) closes the `25/32 <= 1` budget step.
   --------------------------------------------------------------------------- *)
axiom break_card9 : size break_univ = 9.

(* Σ_{b in break_univ} breakBound b <= |break_univ| * c, when every breakBound
   is <= c.  PROVEN: `Bigreal.sumr_const` collapses the constant sum to
   (count predT break_univ)%r * c = (size break_univ)%r * c, then
   `Bigreal.ler_sum` lifts the pointwise bound.  EC image of the
   `Finset.sum_const`/`Finset.sum_le_sum` step in Lean `activeBudget_closes`.  *)
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
    (acceptedSpendBound : real) (breakBound : break_t -> real) &m :
  Pr[G.accepted() @ &m : res] <= acceptedSpendBound =>
  (forall b, Pr[G.break_(b) @ &m : res] <= breakBound b) =>
  acceptedSpendBound <= 1%r / (2%r ^ 129) =>
  (forall b, breakBound b <= 1%r / (2%r ^ 133)) =>
  Pr[G.success() @ &m : res] <= 1%r / (2%r ^ 128).
proof.
  move=> hAccept hBreak hAcc129 hBrk133.
  apply (success_le_target acceptedSpendBound breakBound (1%r / 2%r ^ 128) &m
           hAccept hBreak).
  (* budget: acceptedSpendBound + Σ breakBound
             <= 2^-129 + 9 * 2^-133 = 2^-129 + 9/8 * 2^-130 ... <= 2^-128.
     Σ breakBound <= size break_univ * 2^-133 = 9 * 2^-133  (BRA.ler_sum_const).*)
  have hsum := sum_break_le_card breakBound (1%r / 2%r ^ 133) hBrk133.
  rewrite break_card9 in hsum.
  (* Real-arithmetic budget step.  Z3 cannot expand the symbolic powers, so we
     feed it the exponent relations 2^129 = 2^128*2 and 2^133 = 2^128*32 and the
     positivity of 2^128 (EC image of Lean's `norm_num`); Z3 then closes
     2^-129 + 9*2^-133 <= 2^-128  (the 25/32 <= 1 budget).                       *)
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
   WHAT REMAINS FOR A FULL (non-spike) EASYCRYPT REDUCTION (the axioms above):
     OB-1  Collapse the 3 procedures into ONE experiment and prove the event
           implication  success => accepted \/ \big_or break  by `byequiv`.
     OB-2  Derive `success_implies_split` for the concrete G (the structural
           soundness import — in Lean it is DERIVED, here it must be re-derived
           against the EC experiment; this is the Lean<->EC SEAM, named in README).
     OB-3  `success_le_union`: with OB-1, replace the current `exact` by the
           finite union bound over break_univ (Distr.mu_or iterated via
           StdBigop.Bigreal.BRA.big_split).
   MACHINE-CHECKED IN THIS FILE (EasyCrypt 2026.05 + Z3 4.16.0, exit 0):
     * break_zero_of_never  — full byphoare reduction Pr[..:res]=0 (Goal 2/OB-4).
     * success_le_target    — the union-bound ASSEMBLY (Bigreal.ler_sum + ler_add
                              + ler_trans), real arith closed by Z3 (Goal 3).
     * sum_break_le_card     — Σ bound via Bigreal.sumr_const + ler_sum.
     * success_le_2pow_neg128 — concrete 2^-128 budget, 25/32<=1 closed by Z3
                              over explicit 2^129/2^133 exponent expansions.
   See spike_success_criteria.md and SPIKE_VERDICT.md.
   ============================================================================= *)
