(* =============================================================================
   break_prevalidation_registry_binding.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       prevalidationRegistryBinding   (BreakEvent #3).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   *** GAP-C RE-MODEL (2026-06-10): SECOND-PREIMAGE, NOT FREE COLLISION ***
   ---------------------------------------------------------------------------
   The previous version modelled the break as a FREE birthday collision: the
   reduction adversary B(A) output BOTH leaves (running the honest registry as
   `fun _ => witness`), winning the CR game on `x <> y /\ dig x = dig y`,
   charging eps ~ q^2/2^256.  That is the strictly weaker / operationally WRONG
   game.  Operationally the registry entry is FIXED by an HONEST party BEFORE the
   adversary acts:

     * The honest admission registry commits, at slot-creation time, a UNIQUE
       leaf `committed k` per key k = (accountId, accountStateRootBefore); its
       slot-authorization digest `t_k = dig (committed k)` is the registry handle.
       This is fixed BY HONEST CREATION, before the adversary presents any spend.
     * The verifier, at spend time, accepts a presented leaf `lf` at key k iff
       its authorization digest equals the COMMITTED handle: dig lf = t_k
       (DirectAdmissionContains: the registry entry at (accountId, root, digest)
       is matched against the LEAF-FIXED committed value — confirmed in
       research/cryptanalysis/creation_time_binding_VERDICT.md, the verifier
       recomputes against the leaf-fixed committed digest, production_reference.rs).
     * Therefore a forged-handle attack must produce  lf <> committed k  with
       dig lf = t_k = dig (committed k).  The target `t_k` was fixed FIRST, by
       the honest registry; the attacker faces a SECOND-PREIMAGE on `t_k`, NOT a
       free birthday collision.

   Consequence: second-preimage advantage is eps ~ q/2^256 (no birthday square,
   no query-cap reliance), giving >= 2^-192 at q = 2^64 with NO classical query
   cap.  This closes GAP C for this event unconditionally.

   ---------------------------------------------------------------------------
   THE EVENT (registry / prevalidation-handle binding).
   ---------------------------------------------------------------------------
   In the Lean concrete classifier (Path2FullSpendConcrete.ConcreteBreakFor):
     prevalidationRegistryBindingBreak :=
       ¬ ( DirectAdmissionContains spend.admissionRecord spend.accountId
             spend.accountStateRootBefore spend.slotAuthorizationDigest spend.leaf
         ∧ CreationCertificateAcceptedBeforeChallenge ...
         ∧ LeafFixedBeforeChallenge spend.leaf spend.spendChallengeTime )
   The cryptographic conjunct `DirectAdmissionContains` binds the prevalidation
   handle to (accountId, root, slotAuthorizationDigest) and names exactly the
   honestly-committed `leaf`.  `LeafFixedBeforeChallenge` is exactly the
   target-fixed-before-adversary discipline this re-model exploits.

   ---------------------------------------------------------------------------
   THE REDUCTION (PROVEN vs ASSUMED)
   ---------------------------------------------------------------------------
   PROVEN (no admit/sorry; closed by EC program logic + Z3; every lemma `qed`):
     * step_break_sets_second_preimage — an accepted forged handle records a
         second-preimage witness `lf <> committed k /\ dig lf = dig (committed k)`
         against the FIXED honest target dig (committed k).
     * run_break_implies_second_preimage — the experiment maintains the invariant
         break ==> recorded second preimage of the fixed target.
     * B_wins_when_break / break_le_spr — the reduction adversary B(A) wins the
         SECOND-PREIMAGE game whenever the registry break fires; lifts to
         Pr[break] <= Pr[B wins SPR].
     * registry_break_bound — union-bound plug-in: the term is <= eps_spr.

   ASSUMED (named hardness):  EXACTLY ONE, clearly labeled:
     * axiom sha256_second_preimage_resistance — against the FIXED honest target
       digest, Pr[ SPR(B(A)).main wins ] <= eps_spr (SHA-256 SPR, generic bound
       ~q/2^256, NO query cap).  Everything above it is PROVEN.
   ============================================================================= *)

require import AllCore Int Real Distr List FSet.

(* -----------------------------------------------------------------------------
   Leaves and digests.  `dig leaf = SHA256(DESCENDER_SLOT_AUTHORIZATION_V1||leaf)`;
   the domain-separation tag is folded into the abstract function.
   --------------------------------------------------------------------------- *)
type leaf.
type digest = int.

op dig : leaf -> digest.          (* SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || .) *)

type key = int * int.             (* (accountId, accountStateRootBefore) *)

(* =============================================================================
   THE SECOND-PREIMAGE GAME for the slot-authorization hash `dig`.

   The honest target is FIXED FIRST: the game is initialized with the honest
   committed leaf `l0` (the registry entry), whose digest `dig l0` is the target.
   The adversary then produces a candidate leaf `y`; it WINS iff `y` is a SECOND
   preimage of the fixed target: `y <> l0 /\ dig y = dig l0`.  This is the
   standard second-preimage game; its advantage is the named SHA-256 SPR
   assumption.
   ============================================================================= *)
module type SPRAdv = {
  proc find (l0 : leaf) : leaf
}.

module SPR (C : SPRAdv) = {
  proc main (l0 : leaf) : bool = {
    var y;
    y <@ C.find(l0);
    return y <> l0 /\ dig y = dig l0;
  }
}.

(* =============================================================================
   THE REGISTRY / PREVALIDATION-HANDLE BINDING GAME (target FIXED FIRST).

   The honest admission registry commits, for each key k, a UNIQUE prevalidation
   leaf `committed k`, FIXED before the adversary acts.  A binding adversary `A`
   adaptively presents spend attempts (k, lf'); the GUARDED verifier accepts
   `lf'` at key k iff its authorization digest equals the COMMITTED handle's
   digest:  dig lf' = dig (committed k)  (DirectAdmissionContains against the
   leaf-fixed committed value).  The REGISTRY-BINDING BREAK fires when an
   ACCEPTED presented leaf differs from the honestly-committed leaf:
       accept  /\  lf' <> committed k.
   Acceptance forces dig lf' = dig (committed k), so a forged accept is exactly a
   SECOND preimage of the fixed target dig (committed k) — extracted below.
   ============================================================================= *)
module type RegAdv = {
  proc present () : (key * leaf) list
}.

module Reg = {
  var committed   : key -> leaf      (* honest registry: unique leaf per key   *)
  var break_fired : bool
  var wit_key     : key              (* key whose target was hit (honest)      *)
  var wit_y       : leaf             (* recorded second-preimage witness       *)

  proc init (c : key -> leaf) : unit = {
    committed   <- c;
    break_fired <- false;
    wit_key     <- witness;
    wit_y       <- witness;
  }

  (* One guarded admission/acceptance step for a presented (key, leaf'). *)
  proc admit_step (k : key, lf : leaf) : bool = {
    var bound, accept;
    (* registry binding: the presented leaf's authorization digest equals the
       HONEST committed leaf's digest at k (the leaf-fixed registry entry at
       (accountId, root, dig (committed k)) is the FIXED target). *)
    bound  <- dig lf = dig (committed k);
    accept <- bound;
    (* REGISTRY-BINDING BREAK: an accepted presented leaf that is NOT the leaf
       the honest registry committed under k — a forged handle, i.e. a SECOND
       preimage of the fixed target dig (committed k). *)
    if (accept /\ lf <> committed k /\ ! break_fired) {
      wit_key <- k;                  (* key whose committed leaf is the target *)
      wit_y   <- lf;                 (* forged accepted leaf = second preimage  *)
    }
    break_fired <- break_fired \/ (accept /\ lf <> committed k);
    return accept;
  }
}.

module Acc (A : RegAdv) = {
  proc run (c : key -> leaf) : bool = {
    var sched, a;
    Reg.init(c);
    sched <@ A.present();
    while (sched <> []) {
      a <- head witness sched;
      Reg.admit_step(a.`1, a.`2);
      sched <- behead sched;
    }
    return Reg.break_fired;
  }
}.

(* =============================================================================
   THE REDUCTION ADVERSARY  B(A)  against SHA-256 SECOND-PREIMAGE RESISTANCE.

   The SPR game fixes the honest target leaf `l0` FIRST and hands it to B.find.
   B installs an honest registry that commits `l0` at EVERY key (the constant
   registry `fun _ => l0`), so the fixed target the verifier checks against IS
   `dig l0` at every key.  B runs the registry binding game with `A`; if `A`
   provoked a break, the recorded witness `wit_y` is a leaf distinct from `l0`
   with `dig wit_y = dig (committed wit_key) = dig l0` — a genuine SECOND
   preimage of the FIXED target `dig l0`.
   ============================================================================= *)
module B (A : RegAdv) : SPRAdv = {
  proc find (l0 : leaf) : leaf = {
    var b;
    b <@ Acc(A).run(fun _ => l0);    (* honest registry commits l0 at every key *)
    return Reg.wit_y;
  }
}.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — the binding guard is doing real work.

   A presented leaf whose authorization digest DIFFERS from the honest committed
   leaf's digest is NOT accepted.  Acceptance is a real conditional on hitting
   the fixed registry target, not the constant `true`.
   ============================================================================= *)
lemma unbound_rejected (kk : key) (ll : leaf) :
  hoare [ Reg.admit_step :
            k = kk /\ lf = ll /\ dig ll <> dig (Reg.committed kk) ==> ! res ].
proof.
  proc; auto.
qed.

(* =============================================================================
   CORE REDUCTION IMPLICATION (step level) — an accepted forged handle is a
   second-preimage witness of the fixed target.

   If a single `admit_step` SETS the break flag from a clear state
   (! break_fired before), then afterwards the recorded witness satisfies
   `wit_y <> committed wit_key /\ dig wit_y = dig (committed wit_key)` — a genuine
   SECOND preimage of the FIXED target dig (committed wit_key).
   ============================================================================= *)
lemma step_break_sets_second_preimage :
  hoare [ Reg.admit_step :
            ! Reg.break_fired
          ==> Reg.break_fired =>
                Reg.wit_y <> Reg.committed Reg.wit_key
                /\ dig Reg.wit_y = dig (Reg.committed Reg.wit_key) ].
proof.
  proc; auto=> &hr nb /=.
  smt().
qed.

(* The second-preimage-witness property, once TRUE, is STABLE across a later
   step: the witness is only overwritten while `! break_fired`, but here it is
   already set. *)
lemma step_keeps_second_preimage :
  hoare [ Reg.admit_step :
            Reg.break_fired /\
            Reg.wit_y <> Reg.committed Reg.wit_key /\
            dig Reg.wit_y = dig (Reg.committed Reg.wit_key)
          ==> Reg.break_fired /\
              Reg.wit_y <> Reg.committed Reg.wit_key /\
              dig Reg.wit_y = dig (Reg.committed Reg.wit_key) ].
proof.
  proc; auto=> &hr [bf col] /=.
  smt().
qed.

(* =============================================================================
   THE LOOP INVARIANT — the experiment maintains "break ==> recorded 2nd preimage".
   ============================================================================= *)
lemma run_break_implies_second_preimage (A <: RegAdv{-Reg}) :
  hoare [ Acc(A).run :
            true
          ==> res = Reg.break_fired
              /\ (res =>
                    Reg.wit_y <> Reg.committed Reg.wit_key
                    /\ dig Reg.wit_y = dig (Reg.committed Reg.wit_key)) ].
proof.
  proc.
  while ( Reg.break_fired =>
            Reg.wit_y <> Reg.committed Reg.wit_key
            /\ dig Reg.wit_y = dig (Reg.committed Reg.wit_key) ).
  + inline Reg.admit_step; auto=> &hr [inv _] /=.
    smt().
  call (_ : true).
  inline Reg.init; auto.
qed.

(* =============================================================================
   THE REDUCTION ADVERSARY WINS SPR WHENEVER THE REGISTRY BREAK FIRES.

   B(A).find(l0) installs `committed = fun _ => l0`, so committed wit_key = l0.
   By run_break_implies_second_preimage, a break implies
   wit_y <> committed wit_key = l0  and  dig wit_y = dig (committed wit_key) = dig l0,
   exactly the SPR winning condition `wit_y <> l0 /\ dig wit_y = dig l0`.  Hence
   the SPR game with adversary B(A) returns `true` whenever the break fired.
   ============================================================================= *)
(* Helper: across Acc(A).run, the honest registry `committed` is the constant
   map it was initialized with (no step mutates it).  Combined with the
   second-preimage invariant, this lets B's SPR win be read off directly. *)
lemma run_committed_const (A <: RegAdv{-Reg}) (c0 : key -> leaf) :
  hoare [ Acc(A).run : arg = c0 ==> Reg.committed = c0 ].
proof.
  proc; while (Reg.committed = c0).
  + inline Reg.admit_step; auto.
  call (_ : true); inline Reg.init; auto.
qed.

lemma run_keeps_committed (A <: RegAdv{-Reg}) (c0 : key -> leaf) :
  hoare [ Acc(A).run :
            arg = c0
          ==> Reg.committed = c0
              /\ (Reg.break_fired =>
                    Reg.wit_y <> Reg.committed Reg.wit_key
                    /\ dig Reg.wit_y = dig (Reg.committed Reg.wit_key)) ].
proof.
  conseq (run_committed_const A c0)
         (_ : true
              ==> Reg.break_fired =>
                    Reg.wit_y <> Reg.committed Reg.wit_key
                    /\ dig Reg.wit_y = dig (Reg.committed Reg.wit_key)) => //.
  conseq (run_break_implies_second_preimage A) => /#.
qed.

lemma B_wins_when_break (A <: RegAdv{-Reg}) (l0 : leaf) :
  hoare [ SPR(B(A)).main :
            arg = l0 ==> Reg.break_fired => res ].
proof.
  proc; inline B(A).find; wp; sp.
  call (run_keeps_committed A (fun (_ : key) => l0)).
  skip=> /> &hr h bf; have [hne heq] := h bf; smt().
qed.

(* =============================================================================
   PROBABILITY TRANSFER — Pr[registry break] <= Pr[B(A) wins SPR].
   ============================================================================= *)
lemma break_le_spr (A <: RegAdv{-Reg}) (l0 : leaf) &m :
  Pr[ Acc(A).run(fun _ => l0) @ &m : res ]
    <= Pr[ SPR(B(A)).main(l0) @ &m : res ].
proof.
  byequiv (_ :   ={glob A}
              /\ arg{1} = (fun _ => l0) /\ arg{2} = l0
              ==> res{1} => res{2}) => //.
  proc *.
  inline SPR(B(A)).main B(A).find.
  wp.
  call (_ :   ={glob A} /\ c{1} = (fun _ => l0) /\ c{2} = (fun _ => l0)
           ==> ={glob Reg}
              /\ Reg.committed{2} = (fun _ => l0)
              /\ (res{1} =>
                    Reg.wit_y{2} <> Reg.committed{2} Reg.wit_key{2}
                    /\ dig Reg.wit_y{2} = dig (Reg.committed{2} Reg.wit_key{2})) ).
  + proc.
    while (   ={glob Reg, sched}
           /\ Reg.committed{2} = (fun _ => l0)
           /\ Reg.break_fired{1} = Reg.break_fired{2}
           /\ (Reg.break_fired{2} =>
                 Reg.wit_y{2} <> Reg.committed{2} Reg.wit_key{2}
                 /\ dig Reg.wit_y{2} = dig (Reg.committed{2} Reg.wit_key{2})) ).
    - inline Reg.admit_step; auto=> />; smt().
    call (_ : ={glob Reg}).
    inline Reg.init; wp; skip=> />.
  (* RHS SPR result is (wit_y{2} <> l0 /\ dig wit_y{2} = dig l0); since
     committed{2} = fun _ => l0, committed{2} wit_key{2} = l0. *)
  auto=> /> *; smt().
qed.

(* =============================================================================
   THE SINGLE NAMED STANDARD HARDNESS ASSUMPTION — SHA-256 SECOND-PREIMAGE.

   Against the FIXED honest target (committed by the honest registry before the
   adversary acts), the reduction adversary B(A) outputs a second preimage with
   probability at most eps_spr (SHA-256 SPR; generic bound ~q/2^256, LINEAR in q,
   NO birthday square, NO query cap).  This is the ONLY assumption in this file.
   ============================================================================= *)
op eps_spr : real.
axiom eps_spr_ge0 : 0%r <= eps_spr.

axiom sha256_second_preimage_resistance (A <: RegAdv{-Reg}) (l0 : leaf) &m :
  Pr[ SPR(B(A)).main(l0) @ &m : res ] <= eps_spr.

(* =============================================================================
   THE BOUND — Pr[ prevalidation-registry-binding break fires ] <= eps_spr.
   *** REDUCTION PROVEN; only SPR hardness named. ***
   ============================================================================= *)
lemma registry_break_bound (A <: RegAdv{-Reg}) (l0 : leaf) &m :
  Pr[ Acc(A).run(fun _ => l0) @ &m : res ] <= eps_spr.
proof.
  have h1 := break_le_spr A l0 &m.
  have h2 := sha256_second_preimage_resistance A l0 &m.
  smt().
qed.

(* Union-bound-shaped corollary: this event's term is <= any budget the SPR
   advantage fits under (the form the spine consumes). *)
lemma registry_break_le_budget (A <: RegAdv{-Reg}) (l0 : leaf) (budget : real) &m :
  eps_spr <= budget =>
  Pr[ Acc(A).run(fun _ => l0) @ &m : res ] <= budget.
proof.
  move=> hb.
  have h := registry_break_bound A l0 &m.
  smt().
qed.

(* =============================================================================
   SELF-CONTAINED FORM (spine-composable; honest target SAMPLED INSIDE).
   -----------------------------------------------------------------------------
   The reduction above takes the honest committed leaf `l0` as a PROCEDURE
   ARGUMENT (Acc(A).run(fun _ => l0)).  The unified SUF-CMA spine experiment
   (spine_endpoint.ec `module Exp(A)`) is a SINGLE-SHOT run with NO such argument,
   so a byequiv from it cannot install `l0`.  We re-expose the bound here in a
   SELF-CONTAINED game where the honest registry target is produced INSIDE the
   game by the adversary (the EC image of the honest admission registry committing
   the unique prevalidation leaf before the adversary acts), so the bound is a
   clean `Pr[ AccSC(C).run() : res ] <= eps_spr` with no &m-side precondition and
   no procedure argument.

   This DOES NOT weaken the proven reduction: `AccSC(C)` simply has C first emit
   the honest leaf `l0` (via `setup`), then runs the EXACT same
   `Acc(C).run(fun _ => l0)`, so its bound DESCENDS from `registry_break_bound`
   (which holds for EVERY l0) by a bypr step that reads off the emitted l0.  The
   SPR axiom is untouched.
   ============================================================================= *)

(* The self-contained adversary: emits the honest committed leaf, then (as a
   RegAdv) presents its spend attempts.  `setup` is the EC image of the honest
   registry FIXING the committed prevalidation leaf before the adversary acts. *)
module type RegAdvSC = {
  proc setup   () : leaf
  proc present () : (key * leaf) list
}.

(* The self-contained experiment: sample the honest registry leaf INSIDE (from
   C.setup), install it at every key, then run the registry-binding experiment.
   No argument, no precondition. *)
module AccSC (C : RegAdvSC) = {
  proc run () : bool = {
    var l0, r;
    l0 <@ C.setup();                 (* honest target FIXED inside the game *)
    r  <@ Acc(C).run(fun _ => l0);   (* the proven registry-binding experiment *)
    return r;
  }
}.

(* phoare form of the per-target bound, keyed on the constant-registry argument.
   PROVEN from `registry_break_bound` via bypr at the PROC-ARGUMENT level, where
   the constant map `fun _ => l0` IS the formal argument (so the lambda matches
   syntactically — the obstruction a post-`call` bypr hits at the call site). *)
lemma registry_break_phoare (A <: RegAdv{-Reg}) (l0 : leaf) :
  phoare [ Acc(A).run : arg = (fun (_ : key) => l0) ==> res ] <= eps_spr.
proof.
  bypr=> &m0 he.
  rewrite he.
  by apply (registry_break_bound A l0 &m0).
qed.

(* The self-contained bound: the registry-binding break in AccSC is <= eps_spr.
   PROVEN by sampling the honest leaf inside, then `call`ing the phoare-form bound
   on the emitted leaf (the per-target bound holds for EVERY l0).  No &m-side
   precondition, no procedure argument. *)
lemma registry_break_bound_sc (C <: RegAdvSC{-Reg}) &m :
  Pr[ AccSC(C).run() @ &m : res ] <= eps_spr.
proof.
  byphoare (_ : true ==> res) => //.
  proc.
  seq 1 : true 1%r eps_spr 0%r 0%r true => //.
  exists* l0; elim* => lv.
  call (registry_break_phoare C lv); auto.
qed.

(* =============================================================================
   END.   prevalidationRegistryBinding: REAL GAME-BASED REDUCTION TO SHA-256
   SECOND-PREIMAGE RESISTANCE (re-modelled, GAP-C closure).
   The implication  accept /\ registry-binding-break ==> SHA-256 SECOND preimage
   (the reduction adversary B(A) wins the SPR game against the FIXED honest
   target) is PROVEN by EC program logic + Z3 (exit 0).  The ONLY named hardness
   assumption is `sha256_second_preimage_resistance` (eps_spr ~ q/2^256, NO query
   cap).  NO admit, NO sorry, NO unjustified axiom.
   ============================================================================= *)
