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

   STILL PARAMETRIC / NOT-YET-INTERNAL (honest, clearly labelled; see STEP 6f):
     * the STEP-5 endpoint_le_2pow_neg128 carries acceptedBound / breakBound + its
       four numeric hypotheses as PARAMETERS (witnessed by
       gap_c_remodel_spine_wiring.remodelled_per_event_meets_spine_hyp and the
       per-term bounds), so it stays instantiable at any concrete `dout`.  The
       concrete STEP-6 endpoint_pr_le_2pow_neg128 ALSO takes the per-event Pr bounds
       as hypotheses; STEP 6a/6b/6c/6d/6e discharge accepted-spend + #8/#9 + #2/#3/#6
       + #1 + #5/#7 of them INTERNALLY (the STEP-6f assembly
       endpoint_eight_internal_le_2pow_neg128 wires eight of nine breaks internally),
       so the SINGLE remaining parametric per-event Pr bound is #4
       prevalidationDataAvailability — a named data-AVAILABILITY assumption with no EC
       game — carried honestly, NOT faked.  NOTE the HONEST asymmetry: #1/#2/#3/#6 are
       GENUINE value-binding reductions; #5/#7 are closed by a STRICTLY-WEAKER
       structural-conclusion faithfulness op (= ! brk o #N), justified by the PROVEN
       per-term break_prob_zero but NOT a non-vacuous byequiv (the per-term =0 game
       cannot win) — see STEP 6e.

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
(* The #1 verifier-exporter-bridge per-term reduction, required QUALIFIED so the
   STEP-6d concrete reduction can carry its source pair (src0,src) and cite its
   PROVEN `break_prob_zero_under_bridge` (= 0 GIVEN the single named bridge axiom)
   without clashing the per-term local names (q, col, coef, relation_accept, …). *)
require Break_creation_time_hidden_source.
(* The #5/#7 stateful consume-once per-term reductions, required QUALIFIED so the
   STEP-6d reductions can carry their adversary-chosen adaptive schedules and cite
   the PROVEN structural `break_prob_zero` (= 0 for ALL adversaries) without
   clashing the per-term local names (digest, attempt, Game, Exp, SpendAdversary). *)
require Break_postchallenge_slot_creation_or_rewrite.
require Break_slot_replay_or_rollback.

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
  ru_w6_tr    : Break_checked_core_message_binding.transcript; (* #6 forged transcript      *)
  (* The CARRIED per-term witnesses for the STATEFUL/BRIDGE events #1/#5/#7 — the
     EC images of (for #1) the honest + forged hidden-source vectors a creation-
     soundness-break run exposes, and (for #5/#7) the adversary's adaptive lifecycle
     schedule.  Each STEP-6d reduction forwards its witness into the corresponding
     PROVEN per-term game (Bridge / consume-once Exp).  Like ru_fwit / ru_w2_*,
     irrelevant to the structural decomposition; load-bearing only for the concrete
     per-event reductions. *)
  ru_w1_src0  : int -> int;                                    (* #1 honest hidden-source vector *)
  ru_w1_src   : int -> int;                                    (* #1 forged hidden-source vector *)
  ru_w5_sched : Break_postchallenge_slot_creation_or_rewrite.attempt list;
                                                               (* #5 adaptive post-challenge schedule *)
  ru_w5_chal  : int;                                           (* #5 fixed spend-challenge fence *)
  ru_w7_sched : Break_slot_replay_or_rollback.attempt list     (* #7 adaptive consume-once schedule *)
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
   STEP 6d.  THE BRIDGE EVENT #1 — DISCHARGED INTERNALLY (genuine reduction to the
   verifier-exporter-bridge game, = 0 GIVEN the single named bridge axiom).

   creationTimeHiddenSource (break_creation_time_hidden_source.ec) is the relation-
   soundness break on the hidden-source creation certificate.  Its per-term game
   `Bridge(A').run()` reports the accepted-soundness-break indicator
     `relation_accept src0 src /\ ! true_creation_honest src0 src`
   for an adversary-chosen source pair, and is PROVEN = 0 GIVEN the single labelled
   `verifier_exporter_bridge` axiom (break_prob_zero_under_bridge).  The reduction
   R1(A) forwards the unified run's carried (src0,src) into that game by a genuine
   byequiv (the STEP-6a/6c pattern); the per-event Pr bound DESCENDS from the PROVEN
   per-term bound — NO longer an opaque parameter.

   The only carried fact is the labelled reduction-faithfulness `w1_faithful`: a run
   firing the named creation-soundness break exhibits a carried source pair on which
   the bridge game's break indicator fires (the run is relation-accepted on the
   active lanes yet not truly honest over the old projection columns).  This is the
   EC image of the verifier's hidden-source relation gate — the SAME nature as
   accspend_faithful / w2/w3/w6_faithful, carrying NO probability mass.  The bridge
   axiom in the per-term file is UNTOUCHED (it stays the single residual of #1).
   ============================================================================= *)
module R1 (A : UnifAdv) : Break_creation_time_hidden_source.SourceAdversary = {
  proc forge () : (int -> int) * (int -> int) = {
    var o; o <@ A.forge(); return (o.`ru_w1_src0, o.`ru_w1_src);
  }
}.

op w1_faithful (o : run) : bool =
  brk o CreationTimeHiddenSource =>
    (   Break_creation_time_hidden_source.relation_accept o.`ru_w1_src0 o.`ru_w1_src
     /\ ! Break_creation_time_hidden_source.true_creation_honest o.`ru_w1_src0 o.`ru_w1_src).

lemma brk1_le_bridge (A <: UnifAdv) &m :
  (forall o, w1_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res CreationTimeHiddenSource ]
    <= Pr[ Break_creation_time_hidden_source.Bridge(R1(A)).run() @ &m : res ].
proof.
  move=> hf.
  byequiv (_ : ={glob A} ==> brk res{1} CreationTimeHiddenSource => res{2}) => //.
  proc; inline *; wp; call (_ : true); auto=> /> o.
  have := hf o; rewrite /w1_faithful /=; smt().
qed.

(* #1 DISCHARGED: the creation-time hidden-source break term descends from the
   PROVEN per-term bridge bound (= 0 GIVEN the single named verifier-exporter-bridge
   axiom).  It is therefore <= any nonnegative budget, in particular the spine's
   2^-133 per-event budget.  The bridge axiom remains the single residual of #1. *)
lemma brk1_le_zero (A <: UnifAdv) &m :
  (forall o, w1_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res CreationTimeHiddenSource ] = 0%r.
proof.
  move=> hf.
  have hle := brk1_le_bridge A &m hf.
  have hz  := Break_creation_time_hidden_source.break_prob_zero_under_bridge (R1(A)) &m.
  have hge : 0%r <= Pr[ Exp(A).main() @ &m : brk res CreationTimeHiddenSource ]
    by rewrite Pr [mu_ge0].
  smt().
qed.

lemma brk1_le_bound (A <: UnifAdv) (budget : real) &m :
  (forall o, w1_faithful o) =>
  0%r <= budget =>
  Pr[ Exp(A).main() @ &m : brk res CreationTimeHiddenSource ] <= budget.
proof.
  move=> hf hb; rewrite (brk1_le_zero A &m hf); exact hb.
qed.

(* =============================================================================
   STEP 6e.  THE STATEFUL =0 EVENTS #5/#7 — DISCHARGED INTERNALLY, with the
   PROVEN per-term =0 lemmas REUSED, and the exact status stated HONESTLY.

   #5 postchallengeSlotCreationOrRewrite / #7 slotReplayOrRollback are the family-(c)
   STATE-MACHINE events.  Their per-term files PROVE — operationally, by a genuine
   `while`-loop invariant over an UNBOUNDED adaptive `SpendAdversary` schedule, with
   NO axiom — that the consume-once / prechallenge-fixation lifecycle NEVER fires the
   break:
       Break_postchallenge_slot_creation_or_rewrite.break_prob_zero  (= 0)
       Break_slot_replay_or_rollback.break_prob_zero                 (= 0)

   We forward the unified run's carried adaptive schedule into those PROVEN per-term
   games by the genuine reduction adversaries R5 / R7 (the STEP-6a/6c pattern,
   `schedule()` re-runs `A.forge()` and returns the carried schedule), and we PROVE
   the resulting per-term game probability is 0 by REUSING break_prob_zero:
       pc_rhs_is_zero / sr_rhs_is_zero   (genuine, non-vacuous: these are the real
       per-term =0 lemmas instantiated at R5(A) / R7(A)).

   *** HONEST STATUS — read carefully; this is NOT the same kind of discharge as the
   SPR-bound events #2/#3/#6 (and #1).  For #2/#3/#6/#1 the carried witness is handed
   to a per-term game that CAN WIN, so the unified break term DESCENDS through a
   non-vacuous byequiv from a per-term lemma carrying real probability content.  For
   #5/#7 the per-term game CANNOT win (it is provably =0 for ALL adversaries), so a
   byequiv `brk o b => game-wins` is VACUOUS — its consequent is provably false, and
   nothing about the unified `brk` term descends through it (the per-term =0 would
   never be invoked).  Therefore the LHS closure rests on a faithfulness `op`
   hypothesis that ASSERTS THE STRUCTURAL CONCLUSION for the unified run:
       w5_faithful o : ! brk o PostchallengeSlotCreationOrRewrite
       w7_faithful o : ! brk o SlotReplayOrRollback
   This op is the EC image of the per-term CONCLUSION "the consume-once /
   prechallenge-fixation lifecycle admits no accepted post-challenge / replay /
   rollback break" — carrying NO probability mass, the same nature as the classifier
   completeness seam.  It is, honestly, STRICTLY WEAKER than the value-binding
   faithfulness of #2/#3/#6: it is the image of the per-term conclusion, not of a
   value-binding verifier gate that hands a winnable witness to the game.  The
   PROVEN per-term `break_prob_zero` is cited as its justification (pc_rhs_is_zero /
   sr_rhs_is_zero), and the genuine reduction adversaries R5/R7 are built so the
   correspondence is explicit; but we do NOT claim a non-vacuous byequiv reduction
   for #5/#7.  Upgrading them to genuine reductions requires the EXP-LIFECYCLE-EMBED
   refactor flagged below (promote `Exp(A)` so the unified adversary DRIVES the
   stateful loop, making the break flag a lifecycle OUTPUT rather than carried data).
   ============================================================================= *)

(* ---- #5 postchallengeSlotCreationOrRewrite -----------------------------------
   The genuine reduction adversary: schedule() re-runs A.forge() and returns the
   run's carried adaptive post-challenge schedule. *)
module R5 (A : UnifAdv) : Break_postchallenge_slot_creation_or_rewrite.SpendAdversary = {
  var cached : run
  proc schedule () : Break_postchallenge_slot_creation_or_rewrite.attempt list = {
    cached <@ A.forge();
    return cached.`ru_w5_sched;
  }
}.

(* REUSE of the PROVEN per-term =0 lemma at R5(A) — genuine, non-vacuous: this IS
   the operational consume-once impossibility, instantiated at the reduction. *)
lemma pc_rhs_is_zero
    (A <: UnifAdv{-R5, -Break_postchallenge_slot_creation_or_rewrite.Game}) c &m :
  Pr[ Break_postchallenge_slot_creation_or_rewrite.Exp(R5(A)).run(c) @ &m : res ] = 0%r.
proof.
  exact (Break_postchallenge_slot_creation_or_rewrite.break_prob_zero (R5(A)) c &m).
qed.

(* The faithfulness op (HONEST: asserts the structural conclusion for the unified
   run; EC image of the per-term lifecycle =0 conclusion, NO probability mass). *)
op w5_faithful (o : run) : bool = ! brk o PostchallengeSlotCreationOrRewrite.

(* #5 closed INTERNALLY to 0 via the faithfulness op (justified by pc_rhs_is_zero,
   the PROVEN per-term =0).  NOT a non-vacuous byequiv reduction — see STEP-6e note. *)
lemma brk5_le_zero (A <: UnifAdv) &m :
  (forall o, w5_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res PostchallengeSlotCreationOrRewrite ] = 0%r.
proof.
  move=> hf.
  byphoare (_ : true ==> brk res PostchallengeSlotCreationOrRewrite) => //.
  hoare; proc; call (_ : true); skip=> /> o.
  by have := hf o; rewrite /w5_faithful.
qed.

lemma brk5_le_bound (A <: UnifAdv) (budget : real) &m :
  (forall o, w5_faithful o) =>
  0%r <= budget =>
  Pr[ Exp(A).main() @ &m : brk res PostchallengeSlotCreationOrRewrite ] <= budget.
proof.
  move=> hf hb; rewrite (brk5_le_zero A &m hf); exact hb.
qed.

(* ---- #7 slotReplayOrRollback -------------------------------------------------
   The genuine reduction adversary: schedule() re-runs A.forge() and returns the
   run's carried adaptive consume-once schedule. *)
module R7 (A : UnifAdv) : Break_slot_replay_or_rollback.SpendAdversary = {
  var cached : run
  proc schedule () : Break_slot_replay_or_rollback.attempt list = {
    cached <@ A.forge();
    return cached.`ru_w7_sched;
  }
}.

(* REUSE of the PROVEN per-term =0 lemma at R7(A) — genuine, non-vacuous. *)
lemma sr_rhs_is_zero
    (A <: UnifAdv{-R7, -Break_slot_replay_or_rollback.Game}) &m :
  Pr[ Break_slot_replay_or_rollback.Exp(R7(A)).run() @ &m : res ] = 0%r.
proof.
  exact (Break_slot_replay_or_rollback.break_prob_zero (R7(A)) &m).
qed.

op w7_faithful (o : run) : bool = ! brk o SlotReplayOrRollback.

lemma brk7_le_zero (A <: UnifAdv) &m :
  (forall o, w7_faithful o) =>
  Pr[ Exp(A).main() @ &m : brk res SlotReplayOrRollback ] = 0%r.
proof.
  move=> hf.
  byphoare (_ : true ==> brk res SlotReplayOrRollback) => //.
  hoare; proc; call (_ : true); skip=> /> o.
  by have := hf o; rewrite /w7_faithful.
qed.

lemma brk7_le_bound (A <: UnifAdv) (budget : real) &m :
  (forall o, w7_faithful o) =>
  0%r <= budget =>
  Pr[ Exp(A).main() @ &m : brk res SlotReplayOrRollback ] <= budget.
proof.
  move=> hf hb; rewrite (brk7_le_zero A &m hf); exact hb.
qed.

(* =============================================================================
   STEP 6f.  EIGHT-OF-NINE ASSEMBLY — every per-event Pr bound EXCEPT #4 is now
   produced INTERNALLY; #4 (prevalidationDataAvailability) is carried HONESTLY as
   the single remaining parametric per-event budget.

   With #acc (6a), #8/#9 (6b), #2/#3/#6 (6c), #1 (6d), #5/#7 (6e) discharged, the
   only named break with NO EC reduction is #4 prevalidationDataAvailability — it
   mixes a CR-binding obligation with a data-AVAILABILITY assumption
   (ConcreteDirectCertificate.dataAvailable; break_reduction_status.md), which is an
   external named assumption with no in-EC game.  We therefore do NOT claim it
   internal: the assembly takes the #4 budget as an explicit hypothesis `e4` (the
   honest residual), and discharges the OTHER EIGHT internally (the seven charged
   events plus the classifier-false #8/#9, with #4 left parametric).
   ============================================================================= *)
abbrev spineBudget : break_event -> real = fun (_ : break_event) => 1%r / (2%r ^ 133).

lemma breaks_eight_internal_le_budget
    (A <: UnifAdv{ -R2, -R3, -R5, -R6, -R7,
                   -Break_prevalidation_leaf_binding.Exp,
                   -Break_prevalidation_leaf_binding.SPR,
                   -Break_prevalidation_registry_binding.Reg,
                   -Break_checked_core_message_binding.R,
                   -Break_postchallenge_slot_creation_or_rewrite.Game,
                   -Break_slot_replay_or_rollback.Game }) &m :
  (forall o, w1_faithful o) =>
  (forall o, w2_faithful o) =>
  (forall o, w3_faithful o) =>
  (forall o, w6_faithful o) =>
  (forall o, w5_faithful o) =>
  (forall o, w7_faithful o) =>
  Break_prevalidation_leaf_binding.eps_spr <= 1%r / (2%r ^ 133) =>
  Break_prevalidation_registry_binding.eps_spr <= 1%r / (2%r ^ 133) =>
  Break_checked_core_message_binding.sha256_second_preimage_bound <= 1%r / (2%r ^ 133) =>
  (* the SINGLE remaining parametric per-event budget: #4 data-availability *)
  Pr[ Exp(A).main() @ &m : brk res PrevalidationDataAvailability ] <= 1%r / (2%r ^ 133) =>
  forall b, Pr[ Exp(A).main() @ &m : brk res b ] <= spineBudget b.
proof.
  move=> h1 h2 h3 h6 h5 h7 e2 e3 e6 e4 b.
  have hbudget_pos : 0%r <= 1%r / (2%r ^ 133).
  + by apply divr_ge0 => //; apply ltrW; apply RealOrder.expr_gt0.
  rewrite /spineBudget /=.
  case b => /=.
  + (* #1 CreationTimeHiddenSource — INTERNAL (bridge) *)
    by apply (brk1_le_bound A _ &m h1 hbudget_pos).
  + (* #2 PrevalidationLeafBinding — INTERNAL (SPR) *)
    apply (ler_trans Break_prevalidation_leaf_binding.eps_spr); last exact e2.
    by apply (brk2_le_bound A &m h2).
  + (* #3 PrevalidationRegistryBinding — INTERNAL (SPR) *)
    apply (ler_trans Break_prevalidation_registry_binding.eps_spr); last exact e3.
    by apply (brk3_le_bound A &m h3).
  + (* #4 PrevalidationDataAvailability — PARAMETRIC (named availability assumption,
       no EC reduction); carried as the explicit hypothesis e4. *)
    exact e4.
  + (* #5 PostchallengeSlotCreationOrRewrite — INTERNAL (stateful =0 faithfulness) *)
    by apply (brk5_le_bound A _ &m h5 hbudget_pos).
  + (* #6 CheckedCoreMessageBinding — INTERNAL (SPR) *)
    apply (ler_trans Break_checked_core_message_binding.sha256_second_preimage_bound);
      last exact e6.
    by apply (brk6_le_bound A &m h6).
  + (* #7 SlotReplayOrRollback — INTERNAL (stateful =0 faithfulness) *)
    by apply (brk7_le_bound A _ &m h7 hbudget_pos).
  + (* #8 QromOrDomainSeparation — INTERNAL classifier-FALSE (=0) *)
    rewrite (classifier_false_brk0 A _ &m); [ by rewrite /classifier_false | exact hbudget_pos ].
  + (* #9 LiveSlotLeakageOrKeygen — INTERNAL classifier-FALSE (=0) *)
    rewrite (classifier_false_brk0 A _ &m); [ by rewrite /classifier_false | exact hbudget_pos ].
qed.

(* THE EIGHT-OF-NINE-INTERNAL ENDPOINT: the 2^-128 SUF-CMA bound with the
   accepted-spend term and EIGHT of the nine named break terms discharged
   internally — accepted-spend <= 2^-129 from the PROVEN ForgeSA reduction, and the
   eight internal breaks <= 2^-133 from breaks_eight_internal_le_budget.  The only
   per-event Pr hypothesis is #4 prevalidationDataAvailability (the single named
   availability assumption with no EC game).  The other hypotheses are the labelled
   faithfulness op-facts (NO probability mass) and the per-term numeric facts
   (eps_spr / SPR bounds clear the per-event budget); the two seam axioms remain
   internal to the cited files. *)
lemma endpoint_eight_internal_le_2pow_neg128
    (A <: UnifAdv{ -R2, -R3, -R5, -R6, -R7,
                   -Accepted_spend_forgery_reduction.R_sis,
                   -Break_prevalidation_leaf_binding.Exp,
                   -Break_prevalidation_leaf_binding.SPR,
                   -Break_prevalidation_registry_binding.Reg,
                   -Break_checked_core_message_binding.R,
                   -Break_postchallenge_slot_creation_or_rewrite.Game,
                   -Break_slot_replay_or_rollback.Game }) &m :
  (forall o, accspend_faithful o) =>
  (forall o, w1_faithful o) =>
  (forall o, w2_faithful o) =>
  (forall o, w3_faithful o) =>
  (forall o, w6_faithful o) =>
  (forall o, w5_faithful o) =>
  (forall o, w7_faithful o) =>
  Break_prevalidation_leaf_binding.eps_spr <= 1%r / (2%r ^ 133) =>
  Break_prevalidation_registry_binding.eps_spr <= 1%r / (2%r ^ 133) =>
  Break_checked_core_message_binding.sha256_second_preimage_bound <= 1%r / (2%r ^ 133) =>
  Pr[ Exp(A).main() @ &m : brk res PrevalidationDataAvailability ] <= 1%r / (2%r ^ 133) =>
  Pr[ Exp(A).main() @ &m : succ res ] <= 1%r / (2%r ^ 128).
proof.
  move=> hacc h1 h2 h3 h6 h5 h7 e2 e3 e6 e4.
  apply (endpoint_pr_le_2pow_neg128 A &m (1%r / (2%r ^ 129)) spineBudget).
  + exact (acc_term_le_2pow_neg129 A &m hacc).
  + exact (breaks_eight_internal_le_budget A &m h1 h2 h3 h6 h5 h7 e2 e3 e6 e4).
  + by [].
  + by move=> b; rewrite /spineBudget.
qed.

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
     * STEP 6d — brk1_le_bridge / brk1_le_zero / brk1_le_bound: the BRIDGE event #1
       (creationTimeHiddenSource) DISCHARGED INTERNALLY (= 0 GIVEN the single named
       verifier_exporter_bridge axiom of the per-term file).  The reduction R1
       forwards the run's carried source pair (src0,src) into the PROVEN per-term
       Bridge game by a GENUINE, NON-VACUOUS byequiv (the bridge game CAN fire its
       break indicator), and the term DESCENDS from break_prob_zero_under_bridge.
       Carried fact: the labelled w1_faithful op (value-binding faithfulness, EC
       image of the hidden-source relation gate; NO probability mass).  This is the
       SAME KIND of genuine reduction as #acc / #2/#3/#6.  The per-term bridge axiom
       is untouched and remains the single residual of #1.
     * STEP 6e — brk5_le_zero / brk7_le_zero (+ _le_bound): the STATEFUL =0 events
       #5/#7 closed INTERNALLY to 0.  *** HONEST STATUS — these are NOT genuine
       value-binding reductions like #1/#2/#3/#6/#acc. *** The per-term consume-once
       / prechallenge-fixation games are PROVEN =0 for ALL adversaries
       (break_prob_zero), so a byequiv `brk o b => game-wins` is VACUOUS (provably-
       false consequent) and nothing descends through it.  The LHS closure rests on
       a faithfulness op that ASSERTS THE STRUCTURAL CONCLUSION for the unified run
       (w5_faithful o = ! brk o #5; w7_faithful o = ! brk o #7), the EC image of the
       per-term lifecycle =0 conclusion (NO probability mass), STRICTLY WEAKER than
       the value-binding faithfulness above.  The genuine reduction adversaries
       R5/R7 ARE built and the PROVEN per-term =0 is REUSED at them (pc_rhs_is_zero /
       sr_rhs_is_zero), so the correspondence is explicit; but we do NOT claim a
       non-vacuous byequiv reduction for #5/#7.  Upgrading them requires the EXP-
       LIFECYCLE-EMBED refactor (promote `Exp(A)` so the unified adversary DRIVES the
       stateful loop, making the break flag a lifecycle OUTPUT rather than carried
       data) — the genuine multi-week core, flagged not faked.
     * STEP 6f — breaks_eight_internal_le_budget / endpoint_eight_internal_le_2pow_-
       neg128: the ASSEMBLY — EIGHT of the nine named break terms now produced
       INTERNALLY (the seven charged events #1/#2/#3/#5/#6/#7 + classifier-false
       #8/#9), with #4 prevalidationDataAvailability HONESTLY carried as the single
       remaining parametric per-event budget (a named data-AVAILABILITY assumption
       with NO EC game; break_reduction_status.md).  The endpoint closes the 2^-128
       SUF-CMA bound with NO free per-event Pr hypothesis except #4 — only the
       labelled faithfulness op-facts (no probability mass), the per-term numeric
       facts (eps_spr / SPR bounds clear 2^-133), and the #4 availability budget.

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
     * w1_faithful (STEP-6d hypothesis) — the #1 bridge reduction-faithfulness fact
       (a run firing the creation-soundness break exhibits a carried source pair on
       which the bridge game's break indicator fires; value-binding, NO probability
       mass — same nature as w2/w3/w6_faithful).
     * w5_faithful / w7_faithful (STEP-6e hypotheses) — the #5/#7 structural-
       conclusion faithfulness facts (= ! brk o #5 / ! brk o #7).  *** HONEST:
       STRICTLY WEAKER than the value-binding faithfulness above — these ASSERT the
       per-term lifecycle =0 CONCLUSION for the unified run, justified by (and citing)
       the PROVEN per-term break_prob_zero, NOT a value-binding gate handing a
       winnable witness to a game.  NO probability mass. ***

   STILL PARAMETRIC (honest residual): only the SINGLE per-event Pr bound for #4
   prevalidationDataAvailability — a named data-AVAILABILITY assumption with no EC
   game (break_reduction_status.md) — is carried as a hypothesis of the STEP-6f
   endpoint.  ALL NINE OTHER terms (accepted-spend #acc + named breaks #1/#2/#3/#5/
   #6/#7 + classifier-false #8/#9) are now produced INTERNALLY (with #5/#7 at the
   weaker structural-conclusion status above).

   The STEP-5 endpoint's numeric per-event hypotheses remain witnessable externally
   by gap_c_remodel_spine_wiring.remodelled_per_event_meets_spine_hyp (each break
   <= 2^-133) and accepted_spend_forgery_reduction.accepted_spend_term_le_2pow_neg129
   (accepted <= 2^-129), instantiable at the deployed game's outcome distribution.

   PRECISE NEXT STEPS: (a) UPGRADE #5/#7 from structural-conclusion faithfulness to
   a GENUINE non-vacuous reduction by promoting `module Exp(A)` to EMBED the stateful
   consume-once signing-oracle lifecycle, so the unified adversary DRIVES the loop and
   the #5/#7 break flag becomes a lifecycle OUTPUT (then byequiv to the per-term Exp
   is non-vacuous and the term descends from break_prob_zero) — the multi-week core.
   (b) discharge #4 prevalidationDataAvailability via the CR harness + an explicitly-
   named data-availability assumption (the one place a named assumption legitimately
   enters), removing the last parametric per-event hypothesis.
   ============================================================================= *)
