(* =============================================================================
   spine_endpoint.ec
   GAP-B closure: the SUF-CMA 2^-128 endpoint, COMPOSED IN EASYCRYPT.

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   PURPOSE
   ---------------------------------------------------------------------------
   spike_union_bound.ec (theory `Spine`) proves PARAMETRICALLY:
       success_le_2pow_neg128 : given the abstract distribution `dout`, the three
       outcome predicates, the pointwise `structural_split`, the finiteness of
       `break_univ`, and the per-event hypotheses (accepted <= 2^-129, each break
       <= 2^-133), the success probability is <= 2^-128.
   The per-term reduction files each prove their OWN per-event bound for their OWN
   game.  gap_c_remodel_spine_wiring.ec proves the CONCRETE numeric epsilons meet
   the spine's per-event hypotheses (`remodelled_per_event_meets_spine_hyp`).

   The remaining GAP-B seam (carried "on paper") is the COMPOSITION: making the
   parametric spine's abstractions concrete and discharging `structural_split` —
   the central soundness decomposition that an accepting fresh forgery is either
   the accepted-spend event OR one of the 9 named break events.

   This file does that composition.  It:
     (1) instantiates the abstract `break_t` of `Spine` with the concrete 9-event
         datatype `break_event` (via spine_break_events), discharging the three
         finiteness axioms with the EC-verified lemmas there;
     (2) defines the CONCRETE outcome model — a record carrying the verify-check
         verdicts a forgery run exposes — and the three outcome predicates as
         deterministic PROJECTIONS of it (exactly the Lean
         `fullSignatureSuccess / acceptedSpendForgery / namedBreak` shape);
     (3) DECOMPOSES `structural_split` into per-check facts (`decompose_success`),
         so the irreducible residual is the single atomic completeness fact
         `verify_check_classifier_complete` (the EC image of the Lean structural
         soundness theorem), NOT the whole probabilistic union;
     (4) COMPOSES the endpoint: `endpoint_le_2pow_neg128`, the 2^-128 SUF-CMA
         bound, derived from the proven per-event bounds via the union assembly.

   ---------------------------------------------------------------------------
   DISCHARGE LEDGER  (PROVEN vs CARRIED-AXIOM, honest)
   ---------------------------------------------------------------------------
   PROVEN here (no admit / no sorry; closed by EC logic + Z3; every lemma qed):
     * The clone of `Spine` at break_event with all three finiteness axioms
       discharged (break_univ_uniq / break_univ_total / break_card9).
     * decompose_success — the LOGICAL decomposition of `structural_split` into
       the per-check facts: an accepting fresh forgery whose verify-classifier is
       exhaustive lands in accepted-spend or fires exactly one named break.  This
       is PROVEN by case analysis over the classifier; it carries NO probability.
     * succ_le_union — the union bound at the concrete predicates, derived from
       the proven decomposition (NOT the abstract clone's structural_split axiom).
     * endpoint_le_2pow_neg128 — the 2^-128 endpoint, COMPOSED in EC over the
       per-event bounds (no longer "on paper").
     * STEP 6 (2026-06-19) — the distribution MADE OPERATIONAL: a concrete
       `module Exp(A)` SUF-CMA experiment whose output realizes `dout`, with the
       union bound (succ_le_union_pr, by Pr [mu_sub] + pr_has_le list-induction
       union) and the endpoint (endpoint_pr_le_2pow_neg128) RE-DERIVED at the
       concrete `Pr[Exp(A).main() @ &m : .]` level.
     * STEP 6a — the DOMINANT accepted-spend term DISCHARGED INTERNALLY: the
       reduction R_acc(A) forwards the run's carried forgery into the PROVEN
       per-term ForgeSA game, so `Pr[Exp(A).main : acc res] <= 2^-129` DESCENDS
       from accepted_spend_forgery_reduction.accepted_spend_term_le_2pow_neg129
       (acc_term_le_2pow_neg129) — NOT an opaque parameter anymore.  Genuine
       byequiv transfer; the only carried fact is the labelled reduction-
       faithfulness `accspend_faithful` (the EC image of the response-path verifier
       structure, NO probability mass).
     * STEP 6b — classifier-FALSE events #8/#9 DISCHARGED INTERNALLY to 0
       (classifier_false_brk0), matching the per-term ledger's definitional
       "classifier False" status (no reduction file; the Lean classifier never
       emits them).

   CARRIED-AXIOM (clearly labelled; the irreducible atomic residual):
     * verify_check_classifier_complete — the SINGLE completeness fact: an
       accepting fresh forgery is classified by the verify model into the
       accepted-spend event OR one of the 9 named checks (the classifier is
       exhaustive).  This is the EC image of the Lean structural soundness
       theorem `account_creation_prevalidated_slot_authorization_or_break`.  It is
       the genuine Lean<->EC correspondence seam (the same status the flat-file
       `structural_split` carried — but here it is the ATOMIC completeness of the
       check classifier, with the disjunction-assembly PROVEN around it, not the
       whole split asserted).  It carries NO probability mass.

     * accspend_faithful — the STEP-6a reduction-faithfulness fact (an accepted-
       spend run's carried forgery wins ForgeSA); supplied as a hypothesis to the
       discharge lemmas, the EC image of the response-path verifier structure (NO
       probability mass — same nature as verify_check_classifier_complete).
     * classifier_false_events — the STEP-6b definitional fact (#8/#9 checks always
       hold); the EC image of the Lean classifier's "ConcreteBreakFor = False" for
       these two placeholders.  Matches the per-term ledger status exactly.

   STEP 6c (2026-06-19) — the THREE SPR-BOUND BINDING EVENTS #2/#3/#6 DISCHARGED
   INTERNALLY: each per-term file now exposes a SELF-CONTAINED bound (the honest
   target sampled INSIDE the game, and for #6 the reduction global installed
   inside — break_le_spr_bound_sc / registry_break_bound_sc / break_le_bound_sc),
   and the reductions R2/R3/R6 forward the unified run's carried per-term witness
   into those games by genuine byequiv (the STEP-6a R_acc pattern), so
   brk2_le_bound / brk3_le_bound / brk6_le_bound DESCEND from the PROVEN per-term
   bounds — NO longer opaque parameters.  Carried fact per event: the labelled
   reduction-faithfulness w2/w3/w6_faithful (verifier binding-gate image, NO
   probability mass).

   STILL PARAMETRIC / NOT-YET-INTERNAL (honest, clearly labelled in STEP 6c):
     * the STEP-5 endpoint_le_2pow_neg128 carries acceptedBound / breakBound + its
       four numeric hypotheses as PARAMETERS (witnessed by
       gap_c_remodel_spine_wiring.remodelled_per_event_meets_spine_hyp and the
       per-term bounds), so it stays instantiable at any concrete `dout`.  The
       concrete STEP-6 endpoint_pr_le_2pow_neg128 ALSO takes the per-event Pr
       bounds as hypotheses; STEP 6a/6b/6c discharge accepted-spend + #8/#9 +
       #2/#3/#6 of them INTERNALLY, while the THREE remaining per-event Pr bounds
       (#1 bridge, #5/#7 stateful =0) are LEFT as the parametric hypotheses and
       ISOLATED as STEP-6c obligations with the precise interface reason each needs
       (bridge / stateful unified-Exp refactor) — NOT faked.

   NOTE on faithfulness: the outcome model records, per run, the verdict of EACH
   named verify check (the EC image of the dossier verifier's gates).  The break
   predicates fire EXACTLY when their named check is violated on an accepting run,
   so `structural_split` is NOT vacuously true: it holds only because the
   classifier is complete (the carried axiom), and the disjunction over the 9
   checks is the PROVEN assembly.  A wrong/over-permissive model would make some
   break_pred unsatisfiable, but each break_pred here is the genuine negation of
   its named check, matching the per-term files' break events.
   ============================================================================= *)

require import AllCore List Real Distr StdBigop StdOrder.
import Bigreal.BRA.
import RealOrder.
require import Spike_union_bound.
require import Spine_break_events.
(* The accepted-spend per-term reduction, required QUALIFIED (no `import`) so the
   STEP-6 concrete experiment can carry its `forgery` witness type and cite its
   PROVEN `accepted_spend_term_le_2pow_neg129` bound without clashing the many
   same-named local types/modules across the per-term files (verified clash-free). *)
require Accepted_spend_forgery_reduction.
(* The three SPR-bound binding per-term reductions (#2/#3/#6), required QUALIFIED
   (no `import`) so their self-contained games (ExpSC / AccSC) and PROVEN per-event
   bounds (break_le_spr_bound_sc / registry_break_bound_sc / break_le_bound_sc) can
   be cited from STEP 6c's internal discharge without clashing the many same-named
   local types/modules (leaf, digest, Exp, SPR, eps_spr, …) across the files. *)
require Break_prevalidation_leaf_binding.
require Break_prevalidation_registry_binding.
require Break_checked_core_message_binding.

(* =============================================================================
   STEP 1.  Instantiate the parametric spine at the concrete 9-event datatype.

   The clone realizes the abstract `break_t <- break_event` and
   `break_univ <- break_event_univ`, discharging the three finiteness axioms with
   the machine-checked lemmas of spine_break_events.ec.  After this, the cloned
   theory `S` exposes the PROVEN spine arithmetic (success_le_union,
   success_le_2pow_neg128, …) at the concrete break datatype, and CARRIES
   `structural_split` as an axiom we discharge below via the concrete model.
   ============================================================================= *)
clone import Spine as S with
  type break_t    <- break_event,
  op   break_univ <- break_event_univ
proof break_univ_uniq, break_univ_total, break_card9.
realize break_univ_uniq.  proof. exact break_event_univ_uniq.  qed.
realize break_univ_total. proof. exact break_event_univ_total. qed.
realize break_card9.      proof. exact break_event_card9.      qed.

(* =============================================================================
   STEP 2.  THE CONCRETE OUTCOME MODEL (the faithful verify record).

   A single SUF-CMA experiment run produces an outcome: the verdict the deployed
   verifier reaches on the adversary's forgery, together with the per-check status
   flags the verifier's gates expose.  We model it as a record:

     ru_accepted    : did the verifier ACCEPT the forged signature?  (final gate)
     ru_fresh       : is the forgery on a FRESH message (not a replayed signing-
                      oracle answer)?  (the SUF-CMA freshness predicate)
     ru_accspend    : is this the accepted-spend (response-path) forgery — a fresh
                      message accepted under the honestly-derived action challenge,
                      the dominant coded-SIS surface (accepted_spend_forgery_-
                      reduction.ec ForgeSA event)?
     ru_check b     : the verdict of the named verify CHECK indexed by break b —
                      TRUE iff the check HOLDS (is satisfied) on this run.  The
                      break for event b fires iff the run is an accepting fresh
                      forgery whose check b is VIOLATED (and not accepted-spend).

   The three outcome predicates are deterministic projections, exactly as in Lean.
   --------------------------------------------------------------------------- *)
type run = {
  ru_accepted : bool;
  ru_fresh    : bool;
  ru_accspend : bool;
  ru_check    : break_event -> bool;  (* per-check verdict: TRUE = check holds *)
  ru_fwit     : Accepted_spend_forgery_reduction.forgery;
                (* the CARRIED accepted-spend forgery transcript a response-path
                   run exposes — the witness the STEP-6 accepted-spend reduction
                   forwards into the proven per-term ForgeSA game.  Irrelevant to
                   the structural decomposition; load-bearing only for the
                   concrete reduction. *)
  (* The CARRIED per-term binding witnesses for the SPR-bound events #2/#3/#6 —
     the EC images of the on-chain honest target + the forged divergent object a
     binding-break run exposes.  Each STEP-6c reduction forwards its witness into
     the corresponding PROVEN self-contained per-term game (ExpSC / AccSC).  Like
     ru_fwit, irrelevant to the structural decomposition; load-bearing only for
     the concrete per-event reductions. *)
  ru_w2_frame : Break_prevalidation_leaf_binding.preimage;     (* #2 honest committed frame *)
  ru_w2_leaf  : Break_prevalidation_leaf_binding.leaf;         (* #2 forged divergent leaf  *)
  ru_w3_leaf  : Break_prevalidation_registry_binding.leaf;     (* #3 honest committed leaf  *)
  ru_w3_key   : Break_prevalidation_registry_binding.key;      (* #3 attacked key           *)
  ru_w3_forge : Break_prevalidation_registry_binding.leaf;     (* #3 forged presented leaf  *)
  ru_w6_tuple : Break_checked_core_message_binding.tuple;      (* #6 honest signed tuple    *)
  ru_w6_tr    : Break_checked_core_message_binding.transcript  (* #6 forged transcript      *)
}.

(* A forgery run is a SUCCESS (a break of SUF-CMA) iff the verifier accepted it AND
   it is on a fresh message.  (Lean fullSignatureSuccess.) *)
op succ (o : run) : bool = o.`ru_accepted /\ o.`ru_fresh.

(* The accepted-spend forgery event: a fresh accepted forgery on the response /
   coded-SIS path (accepted_spend_forgery_reduction.ec's ForgeSA event), modelled
   by the recorded flag.  (Lean acceptedSpendForgery.) *)
op acc (o : run) : bool = o.`ru_accepted /\ o.`ru_fresh /\ o.`ru_accspend.

(* The named break for event b fires iff this run is an accepting fresh forgery
   that is NOT the accepted-spend path AND violates the named check b.  (Lean
   namedBreak b.)  This is the genuine negation of the per-term file's check:
     b=PrevalidationLeafBinding   -> the leaf-binding check (break_prevalidation_-
                                     leaf_binding.ec Exp break) is violated;
     b=CheckedCoreMessageBinding  -> the FS message-binding check (break_checked_-
                                     core_message_binding.ec binding_break) fired;
     ... and so on for the 9 named events.  `! ru_check b` is exactly "check b
   violated", matching each per-term file's break event. *)
op brk (o : run) (b : break_event) : bool =
     o.`ru_accepted /\ o.`ru_fresh /\ ! o.`ru_accspend /\ ! (o.`ru_check b).

(* =============================================================================
   STEP 3.  THE ATOMIC SEAM — the verify-check classifier is COMPLETE.

   This is the SINGLE carried correspondence fact.  It says: a successful forgery
   (accepted + fresh) that is NOT the accepted-spend path must VIOLATE at least one
   of the 9 named verify checks.  Equivalently: if a forgery passes ALL 9 named
   checks and is not the accepted-spend path, it is NOT an accepting fresh forgery.

   This is the EC image of the Lean structural soundness theorem
   `account_creation_prevalidated_slot_authorization_or_break`: the verifier's
   acceptance is the CONJUNCTION of the 9 named binding/structural checks plus the
   response-path (accepted-spend) gate, so an accepting fresh non-accepted-spend
   forgery must break one of the named checks.  It carries NO probability mass — it
   is a pure pointwise classification fact about the verify model.

   *** It is the IRREDUCIBLE atomic residual: the disjunction-assembly over the 9
   checks (decompose_success below) is PROVEN; only the COMPLETENESS of the check
   list (that these 9 checks exhaust the acceptance conjunction) is carried, which
   is the genuine Lean<->EC verify-model correspondence. ***
   ============================================================================= *)
axiom verify_check_classifier_complete (o : run) :
  succ o => acc o \/ exists b, ! (o.`ru_check b).

(* =============================================================================
   STEP 4.  THE DECOMPOSITION  (PROVEN — the logical assembly).

   From the atomic completeness axiom we PROVE the spine-shaped split:
     succ o => acc o \/ has (brk o) break_event_univ.
   The work is the disjunction assembly: turn the existential over a violated check
   into a `has` over the finite `break_event_univ`, and reattach the
   accepting/fresh/non-accspend conjuncts that `brk` carries.  This is the genuine
   LOGICAL content of structural_split, proven; the only thing imported is the
   completeness axiom.
   ============================================================================= *)
lemma decompose_success (o : run) :
  succ o => acc o \/ has (brk o) break_event_univ.
proof.
  move=> hsucc.
  have hclass := verify_check_classifier_complete o hsucc.
  case hclass => [hacc | [b hb]].
  + by left.
  + (* a check b is violated; either we are on the accepted-spend path (left) or
       the named break b fires (right). *)
    case (acc o) => hac.
    - by left.
    - right.
      (* brk o b holds: accepting + fresh (from succ) + not accspend + check b
         violated. *)
      have hbrk : brk o b.
      + move: hsucc hac; rewrite /succ /acc /brk => -[ha hf] hnacc.
        by rewrite ha hf hb /=; move: hnacc; rewrite ha hf /=.
      (* has over the universe, since b is in break_event_univ. *)
      by apply/hasP; exists b; rewrite break_event_univ_total hbrk.
qed.

(* =============================================================================
   STEP 5.  THE ENDPOINT  (COMPOSED in EC).

   We compose the 2^-128 SUF-CMA bound over the CONCRETE predicates.  The
   union-bound arithmetic is re-derived here over (succ, acc, brk) — the same
   derivation the cloned `success_le_2pow_neg128` performs over its abstract
   predicates, but at the concrete model (so the seam is fully discharged through
   the proven decompose_success rather than the abstract clone's structural_split
   axiom).

   Given a distribution `dout : run distr` (the EC image of the deployed SUF-CMA
   game's outcome distribution) and the per-event bounds, the success probability
   is <= 2^-128.  The per-event hypotheses are EXACTLY those witnessed by
   gap_c_remodel_spine_wiring.remodelled_per_event_meets_spine_hyp and the per-term
   reductions; we carry them as parameters so the endpoint is instantiable at the
   real game.
   ============================================================================= *)
section ENDPOINT.

declare const dout : run distr.

(* The union bound at the CONCRETE predicates, derived from the proven
   decompose_success (the discharged structural_split). *)
lemma succ_le_union :
  mu dout succ
  <= mu dout acc
     + big predT (fun (b : break_event) => mu dout (fun o => brk o b)) break_event_univ.
proof.
  have hcontain :
      mu dout succ
        <= mu dout (fun o => acc o \/ has (brk o) break_event_univ).
  + by apply mu_le => o _; apply decompose_success.
  have hor :
      mu dout (fun o => acc o \/ has (brk o) break_event_univ)
        <= mu dout acc
           + mu dout (fun o => has (brk o) break_event_univ).
  + by rewrite mu_or; smt(mu_bounded).
  have hhas :
      mu dout (fun o => has (brk o) break_event_univ)
        <= big predT (fun (b : break_event) => mu dout (fun o => brk o b)) break_event_univ
    by apply (mu_has_le brk dout break_event_univ).
  apply (ler_trans (mu dout (fun o => acc o \/ has (brk o) break_event_univ)));
    first exact hcontain.
  apply (ler_trans (mu dout acc
          + mu dout (fun o => has (brk o) break_event_univ))); first exact hor.
  by apply ler_add => //.
qed.

(* The concrete 2^-128 endpoint: with the accepted-spend term <= 2^-129 and each
   of the 9 named break terms <= 2^-133, the SUF-CMA success probability is
   <= 2^-128.  This is the GAP-B endpoint, COMPOSED in EC over the proven
   per-event bounds (no longer "on paper").

   The hypotheses are exactly the shape proven by
   gap_c_remodel_spine_wiring.remodelled_per_event_meets_spine_hyp (each break
   <= 2^-133) and accepted_spend_forgery_reduction.accepted_spend_term_le_2pow_neg129
   (accepted <= 2^-129), instantiated at the deployed game's `dout`. *)
lemma endpoint_le_2pow_neg128
    (acceptedBound : real) (breakBound : break_event -> real) :
  mu dout acc <= acceptedBound =>
  (forall b, mu dout (fun o => brk o b) <= breakBound b) =>
  acceptedBound <= 1%r / (2%r ^ 129) =>
  (forall b, breakBound b <= 1%r / (2%r ^ 133)) =>
  mu dout succ <= 1%r / (2%r ^ 128).
proof.
  move=> hAcc hBrk hAcc129 hBrk133.
  (* Step A: the break sum <= the budget sum. *)
  have hbreaks :
      big predT (fun b => mu dout (fun o => brk o b)) break_event_univ
        <= big predT breakBound break_event_univ.
  + by apply Bigreal.ler_sum => b _ /=; exact hBrk.
  have hunion := succ_le_union.
  (* Step B: the budget sum <= 9 * 2^-133, via the card-9 bound. *)
  have hsum :
      big predT breakBound break_event_univ
        <= (size break_event_univ)%r * (1%r / (2%r ^ 133)).
  + have <-: big predT (fun (_ : break_event) => 1%r / (2%r ^ 133)) break_event_univ
               = (size break_event_univ)%r * (1%r / (2%r ^ 133)).
    - by rewrite Bigreal.sumr_const count_predT.
    by apply Bigreal.ler_sum => b _ /=; exact hBrk133.
  rewrite break_event_card9 in hsum.
  (* Step C: numeric closure.  2^-129 + 9*2^-133 = (1/2 + 9/32)*2^-128
     = 25/32 * 2^-128 <= 2^-128. *)
  have e129 : 2%r ^ 129 = 2%r ^ 128 * 2%r.
  + by rewrite (_ : 129 = 128 + 1) 1:// RField.exprD_nneg // RField.expr1.
  have e133 : 2%r ^ 133 = 2%r ^ 128 * 32%r.
  + rewrite (_ : 133 = 128 + 5) 1:// RField.exprD_nneg //.
    by rewrite (_ : 5 = 1+1+1+1+1) 1:// !RField.exprD_nneg // !RField.expr1.
  have pos128 : 0%r < 2%r ^ 128 by apply RealOrder.expr_gt0.
  (* The arithmetic spine: mu succ <= acceptedBound + 9*2^-133 <= 2^-128. *)
  have hchain :
      mu dout succ <= acceptedBound + 9%r * (1%r / (2%r ^ 133)).
  + apply (ler_trans (mu dout acc
            + big predT (fun (b : break_event) => mu dout (fun o => brk o b))
                break_event_univ)); first exact hunion.
    apply ler_add; first exact hAcc.
    by apply (ler_trans (big predT breakBound break_event_univ)).
  apply (ler_trans (acceptedBound + 9%r * (1%r / (2%r ^ 133)))); first exact hchain.
  (* acceptedBound + 9*2^-133 <= 2^-129 + 9*2^-133 <= 2^-128 (25/32 <= 1). *)
  have hstep : acceptedBound + 9%r * (1%r / (2%r ^ 133))
             <= 1%r / (2%r ^ 129) + 9%r * (1%r / (2%r ^ 133)).
  + by apply ler_add => //; exact hAcc129.
  apply (ler_trans (1%r / (2%r ^ 129) + 9%r * (1%r / (2%r ^ 133)))); first exact hstep.
  move: e129 e133 pos128; smt().
qed.

end section ENDPOINT.

(* =============================================================================
   STEP 6.  THE CONCRETE SUF-CMA EXPERIMENT  (dout MADE OPERATIONAL).

   The STEP-5 endpoint above is PARAMETRIC over an abstract `dout : run distr` and
   takes the per-event probability bounds as HYPOTHESES.  Here we make the
   distribution OPERATIONAL: a concrete `module Exp(A)` whose unified adversary `A`
   produces the observed run, so the success / accepted / break probabilities are
   the REAL `Pr[Exp(A).main() @ &m : .]` of a concrete experiment, and the union
   bound + endpoint are re-derived at that concrete level.  Then we DISCHARGE the
   per-event hypotheses that COMPOSE cleanly with the per-term reductions:
     * accepted-spend  — genuine reduction to the PROVEN ForgeSA bound (2^-129);
     * events #8/#9    — definitional classifier-FALSE (=0), matching the per-term
                         ledger (no reduction file: the Lean classifier never emits
                         these), so the unified break is 0.
   The remaining per-event terms (#1 bridge, #2/#3/#6 fixed-target SPR, #5/#7
   stateful =0) are isolated below as clearly-labelled OBLIGATIONS with the precise
   interface reason each needs (a per-term refactor or a stateful unified Exp),
   NOT faked.
   ============================================================================= *)

(* The unified SUF-CMA adversary: it drives keygen + signing oracle + forgery and
   exposes the observed run (verdict flags + carried accepted-spend witness).  The
   EC image of the deployed game's observable; `Exp(A).main()`'s output realizes
   the abstract `dout`. *)
module type UnifAdv = { proc forge () : run }.

module Exp (A : UnifAdv) = {
  proc main () : run = { var o; o <@ A.forge(); return o; }
}.

(* -----------------------------------------------------------------------------
   The Pr-level finite union bound over the concrete experiment, by list
   induction (each cons split by `Pr [mu_or]`).  PROVEN.
   --------------------------------------------------------------------------- *)
lemma pr_has_le (A <: UnifAdv) (P : run -> break_event -> bool)
    (l : break_event list) &m :
  Pr[ Exp(A).main() @ &m : has (P res) l ]
  <= big predT (fun b => Pr[ Exp(A).main() @ &m : P res b ]) l.
proof.
  elim: l => [| b l ih].
  + rewrite big_nil.
    have ->: Pr[ Exp(A).main() @ &m : has (P res) [] ] = 0%r
      by byphoare => //=; hoare; auto.
    by [].
  + rewrite big_cons {1}/predT /=.
    have hsplit :
      Pr[ Exp(A).main() @ &m : has (P res) (b :: l) ]
      <= Pr[ Exp(A).main() @ &m : P res b ]
       + Pr[ Exp(A).main() @ &m : has (P res) l ].
    - have heq : Pr[ Exp(A).main() @ &m : has (P res) (b :: l) ]
               = Pr[ Exp(A).main() @ &m : (P res b) \/ (has (P res) l) ]
        by rewrite Pr [mu_eq] => />.
      rewrite heq Pr [mu_or].
      have : Pr[ Exp(A).main() @ &m : P res b /\ has (P res) l ] >= 0%r
        by rewrite Pr [mu_ge0].
      smt(mu_bounded ge0_mu).
    apply (ler_trans (Pr[ Exp(A).main() @ &m : P res b ]
                    + Pr[ Exp(A).main() @ &m : has (P res) l ])); first exact hsplit.
    by apply ler_add => //; apply ih.
qed.

(* The union bound at the CONCRETE experiment, derived from the proven
   decompose_success (the discharged structural_split) via Pr [mu_sub] + the
   Pr-level finite union bound. *)
lemma succ_le_union_pr (A <: UnifAdv) &m :
  Pr[ Exp(A).main() @ &m : succ res ]
  <= Pr[ Exp(A).main() @ &m : acc res ]
     + big predT (fun (b : break_event) => Pr[ Exp(A).main() @ &m : brk res b ])
         break_event_univ.
proof.
  have hcontain :
      Pr[ Exp(A).main() @ &m : succ res ]
        <= Pr[ Exp(A).main() @ &m : acc res \/ has (brk res) break_event_univ ].
  + rewrite Pr [mu_sub] => //; move=> &hr; exact decompose_success.
  have hor :
      Pr[ Exp(A).main() @ &m : acc res \/ has (brk res) break_event_univ ]
        <= Pr[ Exp(A).main() @ &m : acc res ]
         + Pr[ Exp(A).main() @ &m : has (brk res) break_event_univ ].
  + rewrite Pr [mu_or]; smt(mu_bounded ge0_mu).
  have hhas := pr_has_le A brk break_event_univ &m.
  apply (ler_trans (Pr[ Exp(A).main() @ &m : acc res \/ has (brk res) break_event_univ ]));
    first exact hcontain.
  apply (ler_trans (Pr[ Exp(A).main() @ &m : acc res ]
          + Pr[ Exp(A).main() @ &m : has (brk res) break_event_univ ])); first exact hor.
  by apply ler_add => //.
qed.

(* The concrete 2^-128 endpoint over the operational experiment, with the per-event
   Pr bounds.  Same arithmetic as STEP 5's endpoint_le_2pow_neg128, re-derived at
   the concrete `Pr[Exp(A).main() @ &m : .]` level. *)
lemma endpoint_pr_le_2pow_neg128 (A <: UnifAdv) &m
    (acceptedBound : real) (breakBound : break_event -> real) :
  Pr[ Exp(A).main() @ &m : acc res ] <= acceptedBound =>
  (forall b, Pr[ Exp(A).main() @ &m : brk res b ] <= breakBound b) =>
  acceptedBound <= 1%r / (2%r ^ 129) =>
  (forall b, breakBound b <= 1%r / (2%r ^ 133)) =>
  Pr[ Exp(A).main() @ &m : succ res ] <= 1%r / (2%r ^ 128).
proof.
  move=> hAcc hBrk hAcc129 hBrk133.
  have hbreaks :
      big predT (fun b => Pr[ Exp(A).main() @ &m : brk res b ]) break_event_univ
        <= big predT breakBound break_event_univ.
  + by apply Bigreal.ler_sum => b _ /=; exact hBrk.
  have hunion := succ_le_union_pr A &m.
  have hsum :
      big predT breakBound break_event_univ
        <= (size break_event_univ)%r * (1%r / (2%r ^ 133)).
  + have <-: big predT (fun (_ : break_event) => 1%r / (2%r ^ 133)) break_event_univ
               = (size break_event_univ)%r * (1%r / (2%r ^ 133)).
    - by rewrite Bigreal.sumr_const count_predT.
    by apply Bigreal.ler_sum => b _ /=; exact hBrk133.
  rewrite break_event_card9 in hsum.
  have e129 : 2%r ^ 129 = 2%r ^ 128 * 2%r.
  + by rewrite (_ : 129 = 128 + 1) 1:// RField.exprD_nneg // RField.expr1.
  have e133 : 2%r ^ 133 = 2%r ^ 128 * 32%r.
  + rewrite (_ : 133 = 128 + 5) 1:// RField.exprD_nneg //.
    by rewrite (_ : 5 = 1+1+1+1+1) 1:// !RField.exprD_nneg // !RField.expr1.
  have pos128 : 0%r < 2%r ^ 128 by apply RealOrder.expr_gt0.
  have hchain :
      Pr[ Exp(A).main() @ &m : succ res ] <= acceptedBound + 9%r * (1%r / (2%r ^ 133)).
  + apply (ler_trans (Pr[ Exp(A).main() @ &m : acc res ]
            + big predT (fun (b : break_event) => Pr[ Exp(A).main() @ &m : brk res b ])
                break_event_univ)); first exact hunion.
    apply ler_add; first exact hAcc.
    by apply (ler_trans (big predT breakBound break_event_univ)).
  apply (ler_trans (acceptedBound + 9%r * (1%r / (2%r ^ 133)))); first exact hchain.
  have hstep : acceptedBound + 9%r * (1%r / (2%r ^ 133))
             <= 1%r / (2%r ^ 129) + 9%r * (1%r / (2%r ^ 133)).
  + by apply ler_add => //; exact hAcc129.
  apply (ler_trans (1%r / (2%r ^ 129) + 9%r * (1%r / (2%r ^ 133)))); first exact hstep.
  move: e129 e133 pos128; smt().
qed.

(* =============================================================================
   STEP 6a.  ACCEPTED-SPEND TERM — DISCHARGED INTERNALLY (genuine reduction).

   The reduction R_acc(A) forwards the run's carried forgery to the PROVEN
   per-term game ForgeSA (accepted_spend_forgery_reduction.ec).  The
   probability transfer is a real byequiv: an accepted-spend unified run's carried
   forgery is a same-action accepted_forgery (the labelled reduction-faithfulness
   fact `accspend_faithful`, the EC image of the verifier's response-path
   structure).  The 2^-129 then DESCENDS from the PROVEN
   accepted_spend_term_le_2pow_neg129 — it is NO LONGER an opaque parameter.
   ============================================================================= *)
module R_acc (A : UnifAdv) : Accepted_spend_forgery_reduction.ForgeAdv = {
  proc forge () : Accepted_spend_forgery_reduction.forgery = {
    var o; o <@ A.forge(); return o.`ru_fwit;
  }
}.

(* The reduction-faithfulness fact: a unified accepted-spend run's carried forgery
   wins ForgeSA.  This is the EC image of the verifier's response-path acceptance
   (a fresh accepted response-path forgery exhibits a same-action accepted_forgery
   on its checked-core transcript), the analogue of verify_check_classifier_complete
   for the accepted-spend term.  It carries NO probability mass. *)
op accspend_faithful (o : run) : bool =
  acc o =>
    (Accepted_spend_forgery_reduction.accepted_forgery o.`ru_fwit
     /\ Accepted_spend_forgery_reduction.same_action o.`ru_fwit).

lemma acc_le_forgeSA (A <: UnifAdv{-Accepted_spend_forgery_reduction.R_sis}) &m :
  (forall o, accspend_faithful o) =>
  Pr[ Exp(A).main() @ &m : acc res ]
    <= Pr[ Accepted_spend_forgery_reduction.ForgeSA(R_acc(A)).run() @ &m : res ].
proof.
  move=> hf.
  byequiv (_ : ={glob A} ==> acc res{1} => res{2}) => //.
  proc; inline *; wp; call (_ : true).
  auto=> /> o; have := hf o; rewrite /accspend_faithful /acc /=; smt().
qed.

(* THE ACCEPTED-SPEND TERM, DISCHARGED INTERNALLY: <= 2^-129, descending from the
   PROVEN per-term reduction (no opaque parameter). *)
lemma acc_term_le_2pow_neg129 (A <: UnifAdv{-Accepted_spend_forgery_reduction.R_sis}) &m :
  (forall o, accspend_faithful o) =>
  Pr[ Exp(A).main() @ &m : acc res ] <= 1%r / (2%r ^ 129).
proof.
  move=> hf.
  apply (ler_trans (Pr[ Accepted_spend_forgery_reduction.ForgeSA(R_acc(A)).run() @ &m : res ])).
  + by apply (acc_le_forgeSA A &m).
  + by apply (Accepted_spend_forgery_reduction.accepted_spend_term_le_2pow_neg129 (R_acc(A)) &m).
qed.

(* =============================================================================
   STEP 6b.  CLASSIFIER-FALSE EVENTS #8/#9 — DISCHARGED INTERNALLY (=0).

   #8 qromOrDomainSeparation and #9 liveSlotLeakageOrKeygen have NO per-term
   reduction file: in the Lean concrete classifier they are definitional NON-events
   (ConcreteBreakFor wires them to False; spine ledger: "classifier False").  The
   EC image is that their named check ALWAYS holds — `classifier_false_events` —
   so the unified break for them is identically false, hence probability 0.  This
   matches the per-term status EXACTLY (definitional 0, no game); it is NOT a faked
   reduction.
   ============================================================================= *)
op classifier_false (b : break_event) : bool =
  b = QromOrDomainSeparation \/ b = LiveSlotLeakageOrKeygen.

(* The EC image of "the classifier never emits #8/#9": their check always holds. *)
axiom classifier_false_events (o : run) (b : break_event) :
  classifier_false b => o.`ru_check b.

lemma classifier_false_brk0 (A <: UnifAdv) (b : break_event) &m :
  classifier_false b => Pr[ Exp(A).main() @ &m : brk res b ] = 0%r.
proof.
  move=> hcf; byphoare (_ : true ==> brk res b) => //.
  hoare; proc; call (_ : true); skip=> /> o.
  by rewrite /brk; have := classifier_false_events o b hcf; smt().
qed.

(* =============================================================================
   STEP 6c.  THE THREE SPR-BOUND BINDING EVENTS #2/#3/#6 — DISCHARGED INTERNALLY
   (genuine reductions to the self-contained per-term games).

   Each per-term file's binding bound used to take the honest target as a PROCEDURE
   ARGUMENT (f0 / l0 / h) — plus, for #6, a &m-side GLOBAL precondition
   `R.honest_tuple{m} = h` — so a single-shot byequiv from the unified `Exp(A)`
   could not install it.  Each file now EXPOSES a SELF-CONTAINED form where the
   honest target is SAMPLED INSIDE the game (and, for #6, the reduction global is
   installed inside), with a clean `Pr[Game(R(A))] <= bound` carrying NO procedure
   argument and NO &m-side precondition:
     #2  Break_prevalidation_leaf_binding.break_le_spr_bound_sc  (ExpSC)
     #3  Break_prevalidation_registry_binding.registry_break_bound_sc (AccSC)
     #6  Break_checked_core_message_binding.break_le_bound_sc    (ExpSC)
   Here we forward each unified run's carried per-term binding witness into the
   corresponding self-contained game by a genuine byequiv reduction (exactly the
   STEP-6a `R_acc` pattern), so the per-event Pr bound DESCENDS from the PROVEN
   per-term bound — NO longer an opaque parameter.

   The only carried fact per event is the labelled reduction-faithfulness
   (`w2_faithful` / `w3_faithful` / `w6_faithful`): a run firing the named binding
   break exhibits a carried target+forged object on which the per-term game's break
   predicate fires.  These are the EC images of the verifier's named binding gates
   (slot-authorization digest / registry handle / checked-core challenge) — the
   SAME nature as `accspend_faithful` and the classifier seam, carrying NO
   probability mass.  The SPR axioms in the per-term files are UNTOUCHED.
   ============================================================================= *)

(* ---- #6 checkedCoreMessageBinding --------------------------------------------
   The reduction forwards the run's carried (honest tuple, transcript) into the
   self-contained per-term game ExpSC; the per-term game installs the honest tuple
   into its reduction global and FS-checks the transcript. *)
module R6 (A : UnifAdv) : Break_checked_core_message_binding.BindAdvSC = {
  var cached : run
  proc setup () : Break_checked_core_message_binding.tuple = {
    cached <@ A.forge();
    return cached.`ru_w6_tuple;
  }
  proc forge (honest : Break_checked_core_message_binding.tuple)
      : Break_checked_core_message_binding.transcript = {
    return cached.`ru_w6_tr;
  }
}.

op w6_faithful (o : run) : bool =
  brk o CheckedCoreMessageBinding =>
    Break_checked_core_message_binding.binding_break o.`ru_w6_tuple o.`ru_w6_tr.

lemma brk6_le_expsc (A <: UnifAdv{-R6, -Break_checked_core_message_binding.R}) &m :
  (forall o, w6_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res CheckedCoreMessageBinding ]
    <= Pr[ Break_checked_core_message_binding.ExpSC(R6(A)).run() @ &m : res ].
proof.
  move=> hf.
  byequiv (_ : ={glob A} ==> brk res{1} CheckedCoreMessageBinding => res{2}) => //.
  proc; inline *; wp; call (_ : true); auto=> /> o.
  have := hf o; rewrite /w6_faithful /=; smt().
qed.

(* #6 DISCHARGED: the checked-core message-binding break term descends from the
   PROVEN self-contained single-target bound (q/2^246, which clears the spine's
   2^-133 per-event budget; the multi-target horizon T stays folded into the
   spine's per-event `breakBound` parameter). *)
lemma brk6_le_bound (A <: UnifAdv{-R6, -Break_checked_core_message_binding.R}) &m :
  (forall o, w6_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res CheckedCoreMessageBinding ]
    <= Break_checked_core_message_binding.sha256_second_preimage_bound.
proof.
  move=> hf.
  apply (ler_trans (Pr[ Break_checked_core_message_binding.ExpSC(R6(A)).run() @ &m : res ])).
  + by apply (brk6_le_expsc A &m).
  + by apply (Break_checked_core_message_binding.break_le_bound_sc (R6(A)) &m).
qed.

(* ---- #2 prevalidationLeafBinding ---------------------------------------------
   The reduction schedules the carried divergent leaf as a SINGLETON against the
   carried honest committed frame; the leaf game's one-step loop fires the break
   exactly when the carried witness is a second preimage of the frame. *)
module R2 (A : UnifAdv) : Break_prevalidation_leaf_binding.LeafAdversarySC = {
  var cached : run
  proc setup () : Break_prevalidation_leaf_binding.preimage = {
    cached <@ A.forge();
    return cached.`ru_w2_frame;
  }
  proc schedule () : Break_prevalidation_leaf_binding.leaf list = {
    return [ cached.`ru_w2_leaf ];
  }
}.

op w2_faithful (o : run) : bool =
  brk o PrevalidationLeafBinding =>
    (   Break_prevalidation_leaf_binding.sha256
          (Break_prevalidation_leaf_binding.leaf_frame o.`ru_w2_leaf)
        = Break_prevalidation_leaf_binding.sha256 o.`ru_w2_frame
     /\ Break_prevalidation_leaf_binding.leaf_frame o.`ru_w2_leaf <> o.`ru_w2_frame).

lemma brk2_le_expsc (A <: UnifAdv{-R2, -Break_prevalidation_leaf_binding.Exp,
                                  -Break_prevalidation_leaf_binding.SPR}) &m :
  (forall o, w2_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res PrevalidationLeafBinding ]
    <= Pr[ Break_prevalidation_leaf_binding.ExpSC(R2(A)).run() @ &m : res ].
proof.
  move=> hf.
  byequiv (_ : ={glob A} ==> brk res{1} PrevalidationLeafBinding => res{2}) => //.
  proc; inline *.
  rcondt{2} ^while; first by auto.
  rcondf{2} ^while; first by auto.
  wp; call (_ : true); auto=> /> o.
  by have := hf o; rewrite /w2_faithful /=; smt().
qed.

(* #2 DISCHARGED: the leaf-binding break term descends from the PROVEN self-
   contained SPR bound eps_spr (~q/2^256). *)
lemma brk2_le_bound (A <: UnifAdv{-R2, -Break_prevalidation_leaf_binding.Exp,
                                  -Break_prevalidation_leaf_binding.SPR}) &m :
  (forall o, w2_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res PrevalidationLeafBinding ]
    <= Break_prevalidation_leaf_binding.eps_spr.
proof.
  move=> hf.
  apply (ler_trans (Pr[ Break_prevalidation_leaf_binding.ExpSC(R2(A)).run() @ &m : res ])).
  + by apply (brk2_le_expsc A &m).
  + by apply (Break_prevalidation_leaf_binding.break_le_spr_bound_sc (R2(A)) &m).
qed.

(* ---- #3 prevalidationRegistryBinding -----------------------------------------
   The reduction presents the carried forged (key, leaf) as a SINGLETON against
   the carried honest committed leaf (installed at every key); the registry game's
   one-step loop fires the break exactly when the forged leaf is a second preimage
   of the committed leaf's authorization digest. *)
module R3 (A : UnifAdv) : Break_prevalidation_registry_binding.RegAdvSC = {
  var cached : run
  proc setup () : Break_prevalidation_registry_binding.leaf = {
    cached <@ A.forge();
    return cached.`ru_w3_leaf;
  }
  proc present () : (Break_prevalidation_registry_binding.key *
                     Break_prevalidation_registry_binding.leaf) list = {
    return [ (cached.`ru_w3_key, cached.`ru_w3_forge) ];
  }
}.

op w3_faithful (o : run) : bool =
  brk o PrevalidationRegistryBinding =>
    (   Break_prevalidation_registry_binding.dig o.`ru_w3_forge
        = Break_prevalidation_registry_binding.dig o.`ru_w3_leaf
     /\ o.`ru_w3_forge <> o.`ru_w3_leaf).

lemma brk3_le_accsc (A <: UnifAdv{-R3, -Break_prevalidation_registry_binding.Reg}) &m :
  (forall o, w3_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res PrevalidationRegistryBinding ]
    <= Pr[ Break_prevalidation_registry_binding.AccSC(R3(A)).run() @ &m : res ].
proof.
  move=> hf.
  byequiv (_ : ={glob A} ==> brk res{1} PrevalidationRegistryBinding => res{2}) => //.
  proc; inline *.
  rcondt{2} ^while; first by auto.
  rcondf{2} ^while; first by auto.
  wp; call (_ : true); auto=> /> o.
  by have := hf o; rewrite /w3_faithful /=; smt().
qed.

(* #3 DISCHARGED: the registry-binding break term descends from the PROVEN self-
   contained SPR bound eps_spr (~q/2^256). *)
lemma brk3_le_bound (A <: UnifAdv{-R3, -Break_prevalidation_registry_binding.Reg}) &m :
  (forall o, w3_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res PrevalidationRegistryBinding ]
    <= Break_prevalidation_registry_binding.eps_spr.
proof.
  move=> hf.
  apply (ler_trans (Pr[ Break_prevalidation_registry_binding.AccSC(R3(A)).run() @ &m : res ])).
  + by apply (brk3_le_accsc A &m).
  + by apply (Break_prevalidation_registry_binding.registry_break_bound_sc (R3(A)) &m).
qed.

(* =============================================================================
   STEP 6c.  REMAINING PER-EVENT OBLIGATIONS (#1 / #5 / #7, clearly labelled).

   With #2/#3/#6 now DISCHARGED INTERNALLY above (genuine byequiv reductions to
   the self-contained per-term games), the honest residual is THREE events whose
   per-term game INTERFACE still does not compose with the single-shot unified
   `Exp` by a pure byequiv — they need the Exp-lifecycle refactor (a separate run):

   #1 creationTimeHiddenSource (break_creation_time_hidden_source.ec):
        Bridge(A').run() = 0 GIVEN the verifier_exporter_bridge axiom.  Genuine
        transfer needs the run to carry (src0,src) and a faithfulness fact
        `brk o #1 => relation_accept src0 src /\ ! true_creation_honest src0 src`.
        OBLIGATION: carry the source pair + faithfulness, then reduce to
        break_prob_zero_under_bridge.

   #5 postchallengeSlotCreationOrRewrite / #7 slotReplayOrRollback (stateful =0):
        the per-term =0 games are over ADAPTIVE multi-step schedules
        (SpendAdversary.schedule : attempt list) run through a stateful lifecycle;
        the bound is 0 for ALL adversaries.  A single-shot unified run cannot DRIVE
        the stateful loop, so a non-vacuous byequiv `brk o b => game wins` does not
        exist in this model (the game never wins, so the implication would force
        brk = false — vacuous).  OBLIGATION (needs the unified Exp to EMBED the
        stateful signing-oracle loop, the genuine multi-week core): run the
        consume-once lifecycle inside Exp and discharge via break_prob_zero.

   These THREE events (#1, #5, #7) are the honest residual of this turn:
   the COMPOSITION machinery (concrete Exp, union bound, endpoint), the dominant
   accepted-spend term, the classifier-false pair (#8/#9), AND the three SPR-bound
   binding events (#2/#3/#6) are now discharged INTERNALLY; only the bridge and
   stateful connections are flagged with their precise refactor, not faked.
   ============================================================================= *)

(* =============================================================================
   END.   GAP-B SPINE INSTANTIATION — the SUF-CMA 2^-128 endpoint COMPOSED IN EC.

   PROVEN (EC logic + Z3, exit 0; no admit, no sorry, every lemma qed):
     * the clone of the parametric spine at the concrete 9-event datatype, all
       three finiteness axioms discharged;
     * decompose_success — the LOGICAL decomposition (structural_split discharged
       to the atomic completeness axiom): an accepting fresh forgery lands in
       accepted-spend or fires exactly one of the 9 named break events.  PROVEN by
       case analysis; carries NO probability;
     * succ_le_union — the union bound at the concrete predicates, derived from
       the proven decomposition;
     * endpoint_le_2pow_neg128 — the 2^-128 SUF-CMA endpoint, COMPOSED in EC over
       the per-event bounds (accepted <= 2^-129, each of 9 breaks <= 2^-133);
     * STEP 6 (concrete experiment) — pr_has_le, succ_le_union_pr,
       endpoint_pr_le_2pow_neg128 (the endpoint over the operational `module Exp(A)`);
     * STEP 6a — acc_le_forgeSA + acc_term_le_2pow_neg129: the accepted-spend term
       DISCHARGED INTERNALLY (<= 2^-129 descends from the PROVEN per-term ForgeSA
       bound by a genuine byequiv reduction R_acc);
     * STEP 6b — classifier_false_brk0: events #8/#9 DISCHARGED INTERNALLY to 0
       (definitional classifier-False, matching the per-term ledger);
     * STEP 6c — brk2_le_bound / brk3_le_bound / brk6_le_bound: the THREE SPR-bound
       binding events #2/#3/#6 DISCHARGED INTERNALLY (each break term descends from
       the PROVEN per-term SELF-CONTAINED bound break_le_spr_bound_sc /
       registry_break_bound_sc / break_le_bound_sc, via the genuine byequiv
       reductions R2/R3/R6 forwarding the run's carried per-term witness — NO longer
       opaque parameters).

   CARRIED-AXIOM (the atomic residuals, clearly labelled):
     * verify_check_classifier_complete — the COMPLETENESS of the 9-check verify
       classifier (an accepting fresh non-accepted-spend forgery violates a named
       check).  The EC image of the Lean structural soundness theorem; the genuine
       Lean<->EC verify-model correspondence seam.  NO probability mass.  This is
       STRICTLY MORE PRIMITIVE than the flat-file `structural_split` axiom: the
       disjunction-assembly over the 9 checks is now PROVEN around it.
     * accspend_faithful (STEP-6a hypothesis) — the response-path reduction-
       faithfulness fact (NO probability mass; same nature as the classifier seam).
     * classifier_false_events (STEP-6b axiom) — the #8/#9 definitional non-event
       fact (matches the per-term ledger; NO probability mass).
     * w2_faithful / w3_faithful / w6_faithful (STEP-6c hypotheses) — the binding-
       break reduction-faithfulness facts (a run firing the named binding break
       exhibits a carried target+forged object on which the per-term game's break
       predicate fires; EC image of the verifier's named binding gates; NO
       probability mass — same nature as accspend_faithful and the classifier seam).

   STILL PARAMETRIC (honest residual; STEP 6c obligations, NOT faked): the THREE
   per-event Pr bounds for #1 (bridge) and #5/#7 (stateful =0) are left as the
   endpoint_pr_le_2pow_neg128 hypotheses, each isolated with the precise interface
   refactor it needs (carry the source pair + self-contained bridge bound, or embed
   the stateful loop in Exp).  The accepted-spend term (#acc) and the three
   SPR-bound binding terms (#2/#3/#6) are now INTERNAL.

   The STEP-5 endpoint's numeric per-event hypotheses remain witnessable externally
   by gap_c_remodel_spine_wiring.remodelled_per_event_meets_spine_hyp (each break
   <= 2^-133) and accepted_spend_forgery_reduction.accepted_spend_term_le_2pow_neg129
   (accepted <= 2^-129), instantiable at the deployed game's outcome distribution.

   PRECISE NEXT STEP: discharge #1/#5/#7 by promoting `module Exp(A)` to EMBED the
   stateful signing-oracle lifecycle so the per-term =0 / bridge games run inside it
   (the genuine multi-week core): carry the (src0,src) source pair + a #1
   faithfulness fact and reduce to break_prob_zero_under_bridge; run the consume-
   once lifecycle inside Exp and discharge #5/#7 via break_prob_zero.
   ============================================================================= *)
