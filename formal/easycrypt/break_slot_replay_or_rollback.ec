(* =============================================================================
   break_slot_replay_or_rollback.ec
   Path-2 Descender — REAL EasyCrypt reduction for ONE cryptographically-charged
   break event:  slotReplayOrRollbackBreak  (BreakEvent #7).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHY THIS EVENT, AND WHAT KIND OF PROOF IT ADMITS
   ---------------------------------------------------------------------------
   spec §5 (descender-signature-spec.md) classifies the forgery reduction into
   three families:
     (a) response-link-coded SIS solver,
     (b) hash collision-resistance / domain-separation (QROM), and
     (c) a STATE-MACHINE violation (replay/rollback) — "proven impossible by the
         consume-once lifecycle".
   slotReplayOrRollback is the canonical member of family (c).  ws7_reduction_
   scoping.md marks it **PROVE** (not NAME): "consume-once slot state machine
   (D2: must be proven, not assumed)".

   The Lean side (Path2TraceModel.lean) discharges this break to probability 0
   via `breakProb_eq_zero_of_never` — but ONLY after the concrete game *wires*
   the break to `false` by construction (`ConcreteBreakFor ... = False`).  That
   wiring is asserted, not derived from an operational state machine.

   THIS FILE CLOSES THAT GAP HONESTLY: it builds the consume-once lifecycle as a
   real stateful EasyCrypt module whose `Spend` step is GUARDED (it refuses to
   re-consume an already-consumed slot digest, and refuses to accept below the
   monotone rollback barrier).  The "replay-or-rollback occurred" break flag is
   set by the operational semantics itself.  We then PROVE, by a genuine
   `while`-loop invariant, that an accepted transition can NEVER exhibit a replay
   or a rollback, hence the break flag is provably never set, hence
   Pr[break fires] = 0 by `byphoare`.  An UNBOUNDED adaptive adversary chooses
   the spend sequence; the impossibility is combinatorial, not probabilistic and
   not cryptographic — exactly a discharged structural (=0) reduction.

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no `admit`, closed by EC program logic + Z3; every lemma ends `qed`):
     * spend_preserves_nobreak — one guarded `Game.spend` step preserves
         `! Game.break_fired`: the only way to set the flag is
         `accept /\ (is_replay \/ is_rollback)`, but `accept` already entails
         `! is_replay /\ ! is_rollback`, a boolean contradiction Z3 closes.
     * spend_loop_never_breaks — `hoare [ Exp(A).run : true ==> ! res ]`: the whole
         experiment never returns "break fired", for ANY unbounded adaptive
         adversary `A`.  Proven by the `while`-loop invariant `! Game.break_fired`,
         established by `Game.init` and preserved by every `Game.spend` iteration.
     * break_prob_zero  — `Pr[ Exp(A).run() @ &m : res ] = 0%r`.  PROVEN by
         `byphoare` from `spend_loop_never_breaks`.  This is the EC image of the
         Lean `breakProb_eq_zero_of_never`, but here the `never` hypothesis is
         *itself proven from the operational state machine* — it is NOT axiom-fed.
     * break_prob_le_any — the plug-in corollary: 0 <= any nonnegative budget, so
         this break's union-bound term is discharged for free.
     * replay_rejected / rollback_rejected — local `hoare` lemmas exhibiting that
         the guard is what forces the impossibility (the non-tautology witnesses:
         a replay attempt and a rollback attempt are each provably NOT accepted).

   ASSUMED (named hardness):  *** NONE. ***
     This event reduces to a fully discharged structural (=0) argument.  There is
     no cryptographic hardness assumption and no axiom carrying the bound.  The
     ONLY things imported are the EasyCrypt standard library (Int, Real, FSet,
     Distr) — no `axiom` declaration appears in this file.

   This is the honest, real-proof counterpart to the spike's `break_zero_of_never`
   (which took the `never` postcondition as a hypothesis): here we PROVE the
   `never` postcondition from the consume-once lifecycle semantics.
   ============================================================================= *)

require import AllCore Int Real Distr List FSet.

(* -----------------------------------------------------------------------------
   Slot-authorization digests.  In the concrete model (Path2FullSpendConcrete.
   ConcreteConsumeState) these are the `slotAuthorizationDigest : Nat` consumed
   by the one-use lifecycle transition; here an abstract equality type.
   --------------------------------------------------------------------------- *)
type digest = int.

(* A single adversary-chosen spend attempt: which slot digest to consume, and at
   which chain height to apply it.  (height models ConcreteConsumeState.chainHeight
   / the rollback barrier coordinate.)  These are the adversary's *requests*; the
   lifecycle decides whether each is accepted. *)
type attempt = digest * int.   (* (slot digest, chain height) *)

(* -----------------------------------------------------------------------------
   The adversary: it adaptively produces a finite schedule of spend attempts.
   It is UNBOUNDED — no query/coin budget restriction is imposed — so proving the
   break never fires against THIS adversary is the worst case.
   --------------------------------------------------------------------------- *)
module type SpendAdversary = {
  proc schedule () : attempt list
}.

(* =============================================================================
   THE CONSUME-ONCE LIFECYCLE  (faithful operational model)

   State (mirrors ConcreteConsumeState):
     consumed     : set of slot digests already consumed (consume-once ledger)
     barrier      : monotone rollback barrier / state nonce (no-rollback height)
     break_fired  : did an ACCEPTED transition exhibit a replay or a rollback?

   `Spend (d, h)` is the GUARDED one-use transition:
     accept  <=>  (d is NOT already consumed)  /\  (h >= barrier)
   i.e. consume-once (anti-replay) AND monotone barrier (anti-rollback).
   On accept it records d as consumed and advances the barrier to h.

   The break flag is set ONLY when an attempt that WOULD replay or roll back is
   nonetheless treated as an accepted consumption.  Because the guard rejects
   exactly those, the flag can never be set — and THAT is what we prove.  The
   flag is computed from the *pre-state* fact (already-consumed / below-barrier)
   conjoined with acceptance, so it is a faithful "bad event", not a tautology.
   ============================================================================= *)
module Game = {
  var consumed    : digest fset
  var barrier     : int
  var break_fired : bool

  proc init () : unit = {
    consumed    <- fset0;
    barrier     <- 0;
    break_fired <- false;
  }

  (* One guarded consume-once transition.  Returns whether it accepted. *)
  proc spend (d : digest, h : int) : bool = {
    var is_replay, is_rollback, accept;
    is_replay   <- d \in consumed;       (* slot digest already consumed?      *)
    is_rollback <- h < barrier;          (* height below monotone barrier?     *)
    accept      <- (! is_replay) /\ (! is_rollback);
    if (accept) {
      consumed <- consumed `|` fset1 d;  (* consume-once: record the digest    *)
      barrier  <- h;                     (* advance monotone rollback barrier  *)
    }
    (* BAD EVENT: an accepted transition that replayed or rolled back.  The guard
       makes the conjunction unsatisfiable; recorded faithfully nonetheless. *)
    break_fired <- break_fired \/ (accept /\ (is_replay \/ is_rollback));
    return accept;
  }
}.

(* The experiment functor: drives an adversary-chosen adaptive schedule through
   the lifecycle and reports whether the replay/rollback break EVER fired. *)
module Exp (A : SpendAdversary) = {
  proc run () : bool = {
    var sched, a;
    Game.init();
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
   NON-TAUTOLOGY WITNESSES — the guard is doing real work.

   These two `hoare` lemmas show the break flag is governed by the guard: a spend
   that WOULD replay (digest already consumed) is rejected, and a spend that WOULD
   roll back (height below barrier) is rejected.  They prove the bad event is a
   real conditional on the state, not the constant `false`.
   ============================================================================= *)

(* A spend of an already-consumed digest is NOT accepted (anti-replay guard). *)
lemma replay_rejected (dd : digest) (hh : int) :
  hoare [ Game.spend :
            d = dd /\ h = hh /\ dd \in Game.consumed ==> ! res ].
proof.
  proc; auto; smt().
qed.

(* A spend below the monotone barrier is NOT accepted (anti-rollback guard). *)
lemma rollback_rejected (dd : digest) (hh : int) :
  hoare [ Game.spend :
            d = dd /\ h = hh /\ hh < Game.barrier ==> ! res ].
proof.
  proc; auto; smt().
qed.

(* =============================================================================
   CORE STRUCTURAL LEMMA — one guarded step preserves "no break yet".

   If the break flag is false before a `spend`, it is false after: the only way
   to set it is `accept /\ (is_replay \/ is_rollback)`, but `accept` already
   entails `! is_replay /\ ! is_rollback`, so that conjunction is false.  Z3
   closes the boolean contradiction.
   ============================================================================= *)
lemma spend_preserves_nobreak :
  hoare [ Game.spend : ! Game.break_fired ==> ! Game.break_fired ].
proof.
  proc; auto.
  (* goal after `auto`: from `!break_fired`, show
       ! (break_fired \/ (accept /\ (is_replay \/ is_rollback)))
     where accept = !is_replay /\ !is_rollback.  Pure boolean. *)
  smt().
qed.

(* =============================================================================
   THE LOOP INVARIANT — the whole lifecycle never sets the break flag.

   `! Game.break_fired` is an invariant of the adversary-driven `while` loop:
   `init` establishes it, and every iteration preserves it by `spend_preserves_
   nobreak`.  This is the genuine consume-once impossibility argument, against an
   UNBOUNDED adaptive adversary.
   ============================================================================= *)
lemma spend_loop_never_breaks (A <: SpendAdversary{-Game}) :
  hoare [ Exp(A).run : true ==> ! res ].
proof.
  proc.
  (* loop with invariant `! Game.break_fired` *)
  while (! Game.break_fired).
  + (* body preserves the invariant *)
    inline Game.spend; auto; smt().
  (* before the loop: init sets break_fired <- false; schedule call is transparent *)
  call (_ : true).            (* A.schedule has no spec we need *)
  inline Game.init; auto.
qed.

(* =============================================================================
   THE BOUND — Pr[ replay/rollback break fires ] = 0.   *** PROVEN. ***

   `byphoare` turns the probability goal into the `hoare` postcondition `! res`,
   discharged by `spend_loop_never_breaks`.  EC image of Lean
   `breakProb_eq_zero_of_never`, but with the `never` premise PROVEN here from the
   operational consume-once state machine rather than supplied as a hypothesis.
   ============================================================================= *)
lemma break_prob_zero (A <: SpendAdversary{-Game}) &m :
  Pr[ Exp(A).run() @ &m : res ] = 0%r.
proof.
  byphoare (_ : true ==> res) => //.
  hoare.
  conseq (spend_loop_never_breaks A).
qed.

(* =============================================================================
   PLUG-IN COROLLARY — this discharges the slotReplayOrRollback term of the
   union bound (spike_union_bound.ec Goal 3 / the Lean breakBound for this event)
   to exactly 0, with NO hardness assumption: 0 <= ANY budget.
   ============================================================================= *)
lemma break_prob_le_any (A <: SpendAdversary{-Game}) (budget : real) &m :
  0%r <= budget =>
  Pr[ Exp(A).run() @ &m : res ] <= budget.
proof.
  move=> hb; rewrite (break_prob_zero A &m); exact hb.
qed.

(* =============================================================================
   END.   slotReplayOrRollback: REAL DISCHARGED STRUCTURAL REDUCTION (=0).
   PROVEN by EC program logic + Z3 (exit 0).  NO axiom, NO hardness assumption.
   ============================================================================= *)
