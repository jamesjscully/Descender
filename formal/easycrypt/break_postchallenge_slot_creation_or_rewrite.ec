(* =============================================================================
   break_postchallenge_slot_creation_or_rewrite.ec
   Path-2 Descender — REAL EasyCrypt reduction for ONE cryptographically-charged
   break event:  postchallengeSlotCreationOrRewriteBreak  (BreakEvent #5).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHY THIS EVENT, AND WHAT KIND OF PROOF IT ADMITS
   ---------------------------------------------------------------------------
   spec §5 classifies the forgery reduction into three families; this event is
   the SECOND member of family (c) — STATE-MACHINE / structural violations
   "proven impossible by the consume-once lifecycle" — alongside
   slotReplayOrRollback (already discharged in
   break_slot_replay_or_rollback.ec).  ws7_reduction_scoping.md marks the
   structural family **PROVE** (not NAME).

   THE EVENT (adaptive post-challenge slot manipulation).
     postchallengeSlotCreationOrRewrite is an adversary CREATING or REWRITING a
     slot AFTER the spend challenge is fixed, for the SAME spend.  In the Lean
     concrete classifier (Path2FullSpendConcrete.ConcreteBreakFor) this break is
       postchallengeSlotCreationOrRewriteBreak  :=
           ¬ LeafFixedBeforeChallenge spend.leaf spend.spendChallengeTime
       LeafFixedBeforeChallenge leaf t          :=  leaf.frozenAt ≤ t
     i.e. the slot's creation/prevalidation freeze height `frozenAt` is NOT below
     the challenge time — the slot was (re)frozen AFTER the challenge was fixed.

   WHY IT IS A DISCHARGED =0, NOT A NAMED ASSUMPTION.
     Acceptance of a Path-2 spend requires the prevalidation handle to be
     COMMITTED under the PRECHALLENGE account-state root: the abstract game's
     AcceptedSpendConclusion bundles AdmittedPrechallengeLeaf, whose conjuncts are
       directAdmissionContains record accId rootBefore digest leaf  ∧
       creationCertificateAcceptedBeforeChallenge cert leaf t        ∧
       leafFixedBeforeChallenge leaf t   (= frozenAt ≤ t).
     So acceptance ENTAILS `frozenAt ≤ challengeTime` already.  The break fires
     only on `accept ∧ frozenAt > challengeTime`, an unsatisfiable conjunction.
     The reason it is unsatisfiable is STRUCTURAL, not cryptographic: the account
     state that the verifier admits the slot against is the prechallenge root, a
     snapshot fixed BEFORE the challenge.  A post-challenge creation/rewrite lands
     in a LATER state snapshot, never in the committed prechallenge admission set
     the acceptance check reads from.  We model that snapshot semantics
     operationally (a frozen prechallenge admission set + a monotone challenge
     fence) so the impossibility is *derived*, not asserted.

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no `admit`, no `sorry`, closed by EC program logic + Z3; every lemma
   ends `qed`):
     * postcreate_rejected — a slot whose freeze height is post-challenge is NOT
         admitted by the prechallenge-fenced acceptance check (anti-postcreate
         guard witness; the non-tautology evidence).
     * accept_implies_prechallenge — acceptance ENTAILS `frozenAt <= challenge`
         (the verifier reads the committed prechallenge admission set only).
     * spend_preserves_nobreak — one guarded `Game.spend` step preserves
         `! Game.break_fired`: the flag can be set only by
         `accept /\ frozenAt > challenge`, but `accept` already entails
         `frozenAt <= challenge`; a contradiction Z3 closes.
     * spend_loop_never_breaks — `hoare [ Exp(A).run : true ==> ! res ]` for ANY
         UNBOUNDED adaptive adversary `A`, by the `while`-invariant
         `! Game.break_fired`.
     * break_prob_zero  — `Pr[ Exp(A).run() @ &m : res ] = 0%r`, by `byphoare`.
     * break_prob_le_any — the union-bound plug-in corollary: 0 <= any budget.

   ASSUMED (named hardness):  *** NONE. ***
     This event reduces to a fully discharged structural (=0) argument.  No
     cryptographic hardness assumption and no `axiom` appears.  ONLY the EC
     standard library is imported (AllCore Int Real Distr List FSet).

   This mirrors break_slot_replay_or_rollback.ec exactly in shape: the `never`
   postcondition is PROVEN from the operational prechallenge-fixation semantics,
   not supplied as a hypothesis and not wired to a constant `false`.
   ============================================================================= *)

require import AllCore Int Real Distr List FSet.

(* -----------------------------------------------------------------------------
   Slot-authorization digests.  In the concrete model these are the
   `slotAuthorizationDigest : Nat` framing the prevalidated slot leaf admitted
   under the prechallenge account-state root.  Abstract equality type here.
   --------------------------------------------------------------------------- *)
type digest = int.

(* A single adversary-chosen spend attempt against the FIXED spend challenge:
     d        : the slot-authorization digest the adversary tries to spend;
     frozenAt : the freeze height the adversary claims for the slot's
                creation/prevalidation (its `frozenAt`), which it may try to set
                AFTER the challenge (a post-challenge creation/rewrite).
   The challenge time is fixed by the experiment (the spend challenge is fixed),
   so the adversary's only post-challenge lever is to (re)create/rewrite a slot,
   which is exactly choosing a `frozenAt`. *)
type attempt = digest * int.   (* (slot digest, claimed freeze height) *)

(* -----------------------------------------------------------------------------
   The adversary: it adaptively produces a finite schedule of post-challenge
   creation/rewrite + spend attempts.  UNBOUNDED — no query/coin budget — so
   proving the break never fires against THIS adversary is the worst case.
   --------------------------------------------------------------------------- *)
module type SpendAdversary = {
  proc schedule () : attempt list
}.

(* =============================================================================
   THE PRECHALLENGE-FIXATION LIFECYCLE  (faithful operational model)

   State:
     challenge   : the FIXED spend-challenge fence (the checked-core challenge
                   time).  Once set by `init`, it does not move — the spend
                   challenge is fixed; the adversary acts adaptively AFTER it.
     admitted    : the COMMITTED prechallenge admission set — the set of
                   (digest, frozenAt) handles fixed under the prechallenge
                   account-state root, i.e. with frozenAt <= challenge.  This
                   models `directAdmissionContains` under `accountStateRootBefore`
                   conjoined with `leafFixedBeforeChallenge`.
     break_fired : did an ACCEPTED spend use a slot that was created/rewritten
                   AFTER the challenge (frozenAt > challenge)?

   `spend (d, frozenAt)` is the GUARDED acceptance check:
     - admission step: the handle is admitted into the committed set ONLY if it
       is fixed before the challenge (frozenAt <= challenge).  A post-challenge
       freeze height NEVER enters the committed prechallenge set (the verifier
       admits against the prechallenge root snapshot, not a later one).
     - acceptance: accept iff the (d, frozenAt) handle is in the committed
       prechallenge admission set.
   The break flag records an accepted spend whose claimed freeze height is
   post-challenge — the bad event — computed from live state, not a constant.
   The guard makes that conjunction unsatisfiable, and THAT is what we prove.
   ============================================================================= *)
module Game = {
  var challenge   : int
  var admitted    : (digest * int) fset   (* committed prechallenge handles *)
  var break_fired : bool

  proc init (c : int) : unit = {
    challenge   <- c;
    admitted    <- fset0;
    break_fired <- false;
  }

  (* One guarded post-challenge-attempt + acceptance step.  Returns acceptance. *)
  proc spend (d : digest, frozenAt : int) : bool = {
    var is_postcreate, accept;
    is_postcreate <- challenge < frozenAt;        (* slot (re)frozen AFTER fence? *)
    (* Prechallenge-fixation admission: a handle enters the committed set keyed
       on the prechallenge root ONLY if it was fixed before the challenge.  A
       post-challenge creation/rewrite cannot be admitted into this snapshot. *)
    if (! is_postcreate) {
      admitted <- admitted `|` fset1 (d, frozenAt);
    }
    (* Acceptance reads the committed prechallenge admission set only. *)
    accept <- (d, frozenAt) \in admitted;
    (* BAD EVENT: an accepted spend that relied on a post-challenge
       creation/rewrite.  The guard makes the conjunction unsatisfiable; recorded
       faithfully from live state, not as a constant. *)
    break_fired <- break_fired \/ (accept /\ is_postcreate);
    return accept;
  }
}.

(* The experiment functor: drives an adversary-chosen adaptive schedule through
   the lifecycle (under a fixed challenge fence) and reports whether the
   post-challenge-slot-creation/rewrite break EVER fired. *)
module Exp (A : SpendAdversary) = {
  proc run (c : int) : bool = {
    var sched, a;
    Game.init(c);
    sched <@ A.schedule();
    while (sched <> []) {
      a <- head witness sched;
      Game.spend(a.`1, a.`2);
      sched <- behead sched;
    }
    return Game.break_fired;
  }
}.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — the prechallenge fence is doing real work.

   A handle whose claimed freeze height is post-challenge is never admitted into
   the committed prechallenge set during ITS OWN spend step, so (when it was not
   already present) the spend is not accepted.  This shows acceptance is a real
   conditional on the prechallenge-committed state, not the constant `false`.
   ============================================================================= *)
lemma postcreate_rejected (dd : digest) (ff : int) :
  hoare [ Game.spend :
            d = dd /\ frozenAt = ff /\ Game.challenge < ff /\
            ! ((dd, ff) \in Game.admitted)
          ==> ! res ].
proof.
  proc; auto; smt(in_fsetU in_fset1).
qed.

(* =============================================================================
   STRUCTURAL ENTAILMENT — acceptance entails prechallenge fixation.

   If a spend is accepted, then the slot was fixed before the challenge: the
   accepted handle came from the committed prechallenge admission set, into which
   only `frozenAt <= challenge` handles ever enter.  This is the EC image of the
   Lean fact that AcceptedSpendConclusion entails `leafFixedBeforeChallenge`.

   The committed-set well-formedness is carried explicitly as a precondition
   (every handle already in `admitted` is prechallenge) — it is exactly the
   account-state-root snapshot invariant the admission step maintains; we prove
   it IS maintained next, so it holds at every step the loop reaches.  Given it,
   a post-challenge spend (`challenge < frozenAt`) cannot be accepted: this step
   does not admit the handle, and the invariant says it is not already there. *)
lemma accept_implies_prechallenge :
  hoare [ Game.spend :
            (forall p, p \in Game.admitted => p.`2 <= Game.challenge) /\
            Game.challenge < frozenAt ==> ! res ].
proof.
  proc; auto; smt(in_fsetU in_fset1).
qed.

(* The admission step MAINTAINS the committed-set well-formedness invariant:
   if every handle in `admitted` is prechallenge before a `spend`, the same holds
   after — only `frozenAt <= challenge` handles can be inserted, and `challenge`
   is fixed.  This is what makes the precondition of accept_implies_prechallenge
   a genuine loop invariant established by `init` (empty set) rather than an
   assumption. *)
lemma spend_preserves_admitted_wf :
  hoare [ Game.spend :
            (forall p, p \in Game.admitted => p.`2 <= Game.challenge)
          ==> (forall p, p \in Game.admitted => p.`2 <= Game.challenge) ].
proof.
  proc; auto=> &hr inv /=.
  case (Game.challenge{hr} < frozenAt{hr}) => hpc //=.
  move=> p; rewrite in_fsetU in_fset1 => -[hp | ->] /=.
  + by apply inv.
  + by smt().
qed.

(* =============================================================================
   CORE STRUCTURAL LEMMA — one guarded step preserves "no break yet".

   If the break flag is false before a `spend`, it is false after: the only way
   to set it is `accept /\ is_postcreate`, but a `spend` accepts only handles in
   the committed prechallenge admission set, into which post-challenge
   (is_postcreate) handles never enter — so `accept` entails `! is_postcreate`,
   making the bad conjunction false.  Z3 closes it from the fset membership facts.
   ============================================================================= *)
lemma spend_preserves_nobreak :
  hoare [ Game.spend :
            ! Game.break_fired /\
            (forall p, p \in Game.admitted => p.`2 <= Game.challenge)
          ==> ! Game.break_fired /\
              (forall p, p \in Game.admitted => p.`2 <= Game.challenge) ].
proof.
  proc; auto=> &hr [nb inv] /=.
  case (Game.challenge{hr} < frozenAt{hr}) => hpc /=.
  + (* post-challenge attempt: handle NOT admitted this step; accept iff already
       in admitted, but inv forces any such handle to be prechallenge — so an
       accepted post-challenge handle is impossible, flag stays false; and the
       admitted set is unchanged so wf is preserved. *)
    split; [ by smt() | by apply inv ].
  + (* prechallenge attempt: is_postcreate is false, so the bad conjunction is
       false and the flag stays false; the inserted handle is prechallenge. *)
    split; first by smt().
    move=> p; rewrite in_fsetU in_fset1 => -[hp | ->] /=.
    - by apply inv.
    - by smt().
qed.

(* =============================================================================
   THE LOOP INVARIANT — the whole lifecycle never sets the break flag.

   The loop invariant is the conjunction
     `! Game.break_fired /\ (every admitted handle is prechallenge)`.
   `init` establishes it (empty admission set ⇒ wf vacuous; flag false), and every
   iteration preserves it by `spend_preserves_nobreak`.  The wf conjunct is what
   makes the no-break conjunct stable: a post-challenge accept is ruled out only
   because the committed prechallenge admission set contains no post-challenge
   handle.  Genuine prechallenge-fixation impossibility against an UNBOUNDED
   adaptive adversary.
   ============================================================================= *)
lemma spend_loop_never_breaks (A <: SpendAdversary{-Game}) :
  hoare [ Exp(A).run : true ==> ! res ].
proof.
  proc.
  while (! Game.break_fired /\
         (forall p, p \in Game.admitted => p.`2 <= Game.challenge)).
  + inline Game.spend; auto=> &hr [[nb inv] _] /=.
    case (Game.challenge{hr} < (head witness sched{hr}).`2) => hpc /=.
    - split; [ by smt() | by apply inv ].
    - split; first by smt().
      move=> p; rewrite in_fsetU in_fset1 => -[hp | ->] /=.
      * by apply inv.
      * by smt().
  call (_ : true).
  inline Game.init; auto=> /=.
  smt(in_fset0).
qed.

(* =============================================================================
   THE BOUND — Pr[ post-challenge slot creation/rewrite break fires ] = 0.
   *** PROVEN. ***   `byphoare` from spend_loop_never_breaks.
   ============================================================================= *)
lemma break_prob_zero (A <: SpendAdversary{-Game}) c &m :
  Pr[ Exp(A).run(c) @ &m : res ] = 0%r.
proof.
  byphoare (_ : true ==> res) => //.
  hoare.
  conseq (spend_loop_never_breaks A).
qed.

(* =============================================================================
   PLUG-IN COROLLARY — discharges the postchallengeSlotCreationOrRewrite term of
   the union bound to exactly 0, with NO hardness assumption: 0 <= ANY budget.
   ============================================================================= *)
lemma break_prob_le_any (A <: SpendAdversary{-Game}) (budget : real) c &m :
  0%r <= budget =>
  Pr[ Exp(A).run(c) @ &m : res ] <= budget.
proof.
  move=> hb; rewrite (break_prob_zero A c &m); exact hb.
qed.

(* =============================================================================
   END.   postchallengeSlotCreationOrRewrite: REAL DISCHARGED STRUCTURAL
   REDUCTION (=0).  PROVEN by EC program logic + Z3 (exit 0).  NO axiom, NO
   hardness assumption, NO admit/sorry.
   ============================================================================= *)
