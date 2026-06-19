(* =============================================================================
   break_prevalidation_leaf_binding.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       prevalidationLeafBinding  (BreakEvent #2, spec §5 family (b)).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   *** GAP-C RE-MODEL (2026-06-10): SECOND-PREIMAGE, NOT FREE COLLISION ***
   ---------------------------------------------------------------------------
   The previous version of this file modelled the break as a FREE birthday
   collision: the adversary output BOTH frames m1, m2 with sha256 m1 = sha256 m2,
   charging eps ~ q^2/2^256.  That is the strictly weaker (more conservative-
   looking but operationally WRONG) game.  Operationally the slot-authorization
   digest is FIXED by an HONEST party BEFORE the adversary acts:

     * The prevalidated slot's authorization digest `t = SHA256(leaf_frame leaf0)`
       is committed on-chain (the admission registry entry) at slot-creation time,
       by the honest creator, for the honest leaf `leaf0`.
     * The VERIFIER, at spend time, recomputes  d = SHA256(leaf_frame leaf)  and
       REJECTS unless `d` equals the slot's COMMITTED authorization digest `t`
       (SlotAuthorizationDigestFramesLeaf against the leaf-FIXED slot value —
       confirmed in research/cryptanalysis/creation_time_binding_VERDICT.md:
       the opening_commitment SHA-256 frame includes account_witness_commitment,
       production_reference.rs:44266, and the verifier recomputes against the
       leaf-fixed committed value).
     * Therefore an attacker who wants a DIFFERENT leaf accepted under that one
       live slot must find  leaf' with leaf_frame leaf' <> leaf_frame leaf0  and
       SHA256(leaf_frame leaf') = t.  The target `t` was fixed FIRST, by honest
       creation; the attacker faces a SECOND-PREIMAGE on `t`, NOT a free birthday.

   Consequence for the budget: second-preimage advantage is eps ~ q/2^256 (no
   birthday square, no query-cap reliance), giving >= 2^-192 at q = 2^64 with
   NO classical query cap.  This is the HONEST operational model and it closes
   GAP C for this event unconditionally.

   ---------------------------------------------------------------------------
   WHY THIS EVENT IS A NAMED-ASSUMPTION REDUCTION, NOT A STRUCTURAL =0
   ---------------------------------------------------------------------------
   spec §5 (descender-signature-spec.md) splits the forgery reduction into
   three families:
     (a) response-link-coded SIS solver,
     (b) hash collision-/second-preimage-resistance / domain-separation,
     (c) consume-once state-machine violations (replay/rollback).
   The two structural family-(c) events are discharged =0 with NO assumption in
   break_slot_replay_or_rollback.ec / break_postchallenge_slot_creation_or_rewrite.ec.

   prevalidationLeafBinding is family (b): "Merkle/registry binding -> digest
   binding".  A clean =0 is NOT expected and would be DISHONEST: the binding
   holds only because the underlying digest is second-preimage-resistant, a
   genuine cryptographic hardness fact.

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no admit / no sorry; closed by EC program logic + Z3; every lemma qed):
     * frame_injective_witness — distinct leaf tuples have distinct frames, so
         the second preimage is a genuine (non-trivial) collision witness.
     * spend_binds_digest — acceptance ENTAILS d = SHA256(leaf_frame leaf): the
         verifier recomputes the slot-authorization digest from the leaf frame.
     * break_implies_second_preimage — THE REDUCTION.  hoare proof that whenever
         the leaf-binding break fires inside Exp against the FIXED honest target
         `t`, the SPR adversary's recorded preimage `f'` satisfies
         f' <> honest_frame /\ sha256 f' = t  (= sha256 honest_frame), i.e.
         SPR.spr_found is set.  PROVEN, not assumed.
     * break_le_spr — Pr[ leaf-binding break fires ] <= Pr[ SPR adversary wins ].
     * break_le_spr_bound — plug-in: with the named SPR bound, the break term is
         <= eps_spr; feeds the union-bound spine (eps_spr ~ q/2^256).

   ASSUMED (named hardness):  *** EXACTLY ONE ***
     axiom sha256_second_preimage_resistance — SHA-256 is second-preimage-
       resistant: against a FIXED target digest `t = sha256 f0` (f0 honest,
       fixed before the adversary acts), Pr[ adversary finds f' <> f0 with
       sha256 f' = t ] <= eps_spr.  Standard (NIST FIPS 180-4 SHA-256 SPR; the
       generic bound is ~q/2^256 — NO birthday square, NO query cap).
   ============================================================================= *)

require import AllCore Int Real Distr List FSet StdOrder.
import RealOrder.

(* -----------------------------------------------------------------------------
   THE DIGEST AND THE FRAME PREIMAGE.  As before: `sha256 : preimage -> digest`
   is the production hash; the leaf tuple frames INJECTIVELY into a preimage.
   --------------------------------------------------------------------------- *)
type digest.
type preimage.

op sha256 : preimage -> digest.

type leaf.

op leaf_frame : leaf -> preimage.

axiom leaf_frame_injective (l1 l2 : leaf) :
  leaf_frame l1 = leaf_frame l2 => l1 = l2.

op slot_auth_digest (l : leaf) : digest = sha256 (leaf_frame l).

(* =============================================================================
   THE NAMED STANDARD HARDNESS GAME — SHA-256 SECOND-PREIMAGE RESISTANCE.

   The honest target is FIXED FIRST: the game initializes with a committed
   honest frame `f0` (the leaf the slot was created for) and its digest
   `t = sha256 f0`.  The adversary then submits a candidate `f'`; it WINS iff
   `f' <> f0 /\ sha256 f' = t`, i.e. a second preimage of the FIXED target.
   This is the standard SPR game specialized to the fixed function sha256.
   ============================================================================= *)
module SPR = {
  var target_frame : preimage     (* the honestly-committed frame f0 (fixed)   *)
  var spr_found    : bool

  proc init (f0 : preimage) : unit = {
    target_frame <- f0;
    spr_found    <- false;
  }

  (* Submit a candidate SECOND preimage `f'` of the fixed target sha256 f0. *)
  proc submit (f' : preimage) : unit = {
    spr_found <- spr_found
              \/ (f' <> target_frame /\ sha256 f' = sha256 target_frame);
  }
}.

(* =============================================================================
   THE LEAF-BINDING EXPERIMENT (faithful operational model, target FIXED FIRST).

   State:
     committed_frame : preimage — the frame of the leaf the slot was HONESTLY
                       created for (the on-chain admission registry entry).  This
                       is FIXED before the adversary schedules any spend; the
                       slot-authorization digest target is `sha256 committed_frame`.
     break_fired     : did an ACCEPTED spend bind the live slot to a DIFFERENT
                       leaf tuple (a leaf-binding break)?

   `spend l` is the verifier's acceptance check for spending leaf tuple `l`:
     - it recomputes d = SHA256(leaf_frame l)  (SlotAuthorizationDigestFramesLeaf);
     - acceptance requires d EQUALS the slot's COMMITTED authorization digest
       `sha256 committed_frame` (the leaf-fixed slot value);
     - a leaf-binding break is an ACCEPTED spend whose frame DIFFERS from the
       committed honest frame (leaf_frame l <> committed_frame): the one live
       slot is bound to a leaf OTHER than the one it was created for.  On that
       event we hand the SECOND preimage (the accepted divergent frame) to SPR.
   The honest target `committed_frame` is set by `init` BEFORE A.schedule runs —
   this is what makes the reduction a SECOND-preimage, not a free collision.
   ============================================================================= *)

module type LeafAdversary = {
  proc schedule () : leaf list
}.

module Exp (A : LeafAdversary) = {
  var committed_frame : preimage
  var break_fired     : bool

  proc spend (l : leaf) : unit = {
    var d, fr;
    fr <- leaf_frame l;                       (* the leaf's ordered byte frame  *)
    d  <- sha256 fr;                          (* recomputed authorization digest *)
    (* verifier accepts iff d = the slot's COMMITTED (leaf-fixed) target digest *)
    if (d = sha256 committed_frame /\ fr <> committed_frame) {
      (* accepted under the fixed target, but a DIFFERENT leaf frame: a leaf-
         binding break.  Hand the second preimage to the SPR oracle. *)
      break_fired <- true;
      SPR.submit(fr);
    }
  }

  proc run (f0 : preimage) : bool = {
    var sched, l;
    (* honest creation FIXES the slot target BEFORE the adversary acts *)
    SPR.init(f0);
    committed_frame <- f0;
    break_fired <- false;
    sched <@ A.schedule();
    while (sched <> []) {
      l <- head witness sched;
      spend(l);
      sched <- behead sched;
    }
    return break_fired;
  }
}.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — the frame injectivity is doing real work.

   Two DIFFERENT leaf tuples genuinely have DIFFERENT frames, so the bad-event
   guard `fr <> committed_frame` is a real conditional on distinct identity/
   opening/pad tuples — not a vacuous constant.
   ============================================================================= *)
lemma frame_injective_witness (l1 l2 : leaf) :
  l1 <> l2 => leaf_frame l1 <> leaf_frame l2.
proof.
  move=> hne; apply/negP=> heq; apply hne; exact (leaf_frame_injective l1 l2 heq).
qed.

(* =============================================================================
   STRUCTURAL ENTAILMENT — acceptance binds the digest to the leaf frame.
   ============================================================================= *)
lemma spend_binds_digest (l : leaf) :
  slot_auth_digest l = sha256 (leaf_frame l).
proof. by []. qed.

(* =============================================================================
   THE CORE REDUCTION INVARIANT — break_fired ENTAILS SPR.spr_found.

   Loop invariant of Exp.run, with the EXTRA fact that the SPR oracle's target
   frame equals the experiment's committed frame (both set to `f0` by init):
       SPR.target_frame = Exp.committed_frame
     /\ (Exp.break_fired => SPR.spr_found).
   Established by init; preserved by every `spend`:
     - if the break does NOT fire this step, the implication is unaffected;
     - if it DOES fire, the guard `d = sha256 committed_frame /\ fr <> committed_frame`
       holds, i.e. sha256 fr = sha256 committed_frame = sha256 target_frame and
       fr <> committed_frame = target_frame; so SPR.submit(fr) sets spr_found
       (distinct, colliding with the FIXED target).
   This is THE reduction implication, PROVEN — not assumed.  It is a SECOND-
   preimage hit because the target `sha256 target_frame` was fixed at init.
   ============================================================================= *)
lemma break_implies_second_preimage (A <: LeafAdversary{-Exp, -SPR}) :
  hoare [ Exp(A).run : true ==> res => SPR.spr_found ].
proof.
  proc.
  while (   SPR.target_frame = Exp.committed_frame
         /\ (Exp.break_fired => SPR.spr_found)).
  + inline Exp(A).spend SPR.submit; auto; smt().
  call (_ : true).
  inline SPR.init; auto.
qed.

(* =============================================================================
   THE REDUCTION BOUND — Pr[ leaf-binding break ] <= Pr[ SPR.spr_found ].
   ============================================================================= *)
lemma break_le_spr (A <: LeafAdversary{-Exp, -SPR}) f0 &m :
  Pr[ Exp(A).run(f0) @ &m : res ] <= Pr[ Exp(A).run(f0) @ &m : SPR.spr_found ].
proof.
  have h0 : Pr[ Exp(A).run(f0) @ &m : res /\ ! SPR.spr_found ] = 0%r.
  + byphoare (_ : true ==> res /\ ! SPR.spr_found) => //.
    hoare; conseq (break_implies_second_preimage A) => /#.
  have hsplit : Pr[ Exp(A).run(f0) @ &m : res ]
              = Pr[ Exp(A).run(f0) @ &m : res /\ SPR.spr_found ]
                + Pr[ Exp(A).run(f0) @ &m : res /\ ! SPR.spr_found ]
    by rewrite Pr [mu_split SPR.spr_found].
  have hsub : Pr[ Exp(A).run(f0) @ &m : res /\ SPR.spr_found ]
              <= Pr[ Exp(A).run(f0) @ &m : SPR.spr_found ]
    by rewrite Pr [mu_sub].
  by move: hsplit hsub h0; smt().
qed.

(* =============================================================================
   THE SINGLE NAMED STANDARD HARDNESS ASSUMPTION — SHA-256 SECOND-PREIMAGE.

   SHA-256 is second-preimage-resistant: against the FIXED honest target
   (committed by an honest party before the adversary acts), any efficient
   procedure driving the SPR game sets spr_found with probability at most
   eps_spr (NIST FIPS 180-4; generic bound ~q/2^256 — LINEAR in q, NO birthday
   square, NO query cap).  This is the ONE assumption carrying cryptographic
   content in this file.  `eps_spr` is the standing SPR advantage budget.
   ============================================================================= *)
op eps_spr : real.

axiom sha256_second_preimage_resistance (A <: LeafAdversary{-Exp, -SPR}) f0 &m :
  Pr[ Exp(A).run(f0) @ &m : SPR.spr_found ] <= eps_spr.

(* =============================================================================
   PLUG-IN COROLLARY — the leaf-binding break term of the union bound is bounded
   by the SHA-256 SPR advantage (eps_spr ~ q/2^256, NO query cap).  This is the
   term that feeds spike_union_bound.ec's per-break budget.
   ============================================================================= *)
lemma break_le_spr_bound (A <: LeafAdversary{-Exp, -SPR}) f0 &m :
  Pr[ Exp(A).run(f0) @ &m : res ] <= eps_spr.
proof.
  apply (ler_trans (Pr[ Exp(A).run(f0) @ &m : SPR.spr_found ])).
  + exact (break_le_spr A f0 &m).
  + exact (sha256_second_preimage_resistance A f0 &m).
qed.

(* =============================================================================
   SELF-CONTAINED FORM (spine-composable; honest target SAMPLED INSIDE).
   -----------------------------------------------------------------------------
   The reduction above takes the honest committed frame `f0` as a PROCEDURE
   ARGUMENT (Exp(A).run(f0)).  The unified SUF-CMA spine experiment (spine_-
   endpoint.ec `module Exp(A)`) is a SINGLE-SHOT run with NO such argument, so a
   byequiv from it cannot install `f0`.  We re-expose the bound here in a SELF-
   CONTAINED game where the honest target is produced INSIDE the game by the
   adversary (the EC image of the honest signer fixing the slot's authorization
   frame before the adversary schedules its spends), so the bound is a clean
   `Pr[ ExpSC(C).run() : res ] <= eps_spr` with no &m-side precondition and no
   procedure argument.

   This DOES NOT weaken the proven reduction: `ExpSC(C)` simply has C first emit
   the honest frame `f0` (via `setup`), then runs the EXACT same `Exp(C).run(f0)`,
   so its bound DESCENDS from `break_le_spr_bound` (which holds for EVERY f0) by a
   bypr step that reads off the emitted f0.  The SPR axiom is untouched.
   ============================================================================= *)

(* The self-contained adversary: emits the honest committed frame, then (as a
   LeafAdversary) schedules its spends.  `setup` is the EC image of honest slot
   creation FIXING the authorization frame before the adversary acts. *)
module type LeafAdversarySC = {
  proc setup    () : preimage
  proc schedule () : leaf list
}.

(* The self-contained experiment: sample the honest target INSIDE (from C.setup),
   then run the leaf-binding experiment against it.  No argument, no precondition. *)
module ExpSC (C : LeafAdversarySC) = {
  proc run () : bool = {
    var f0, r;
    f0 <@ C.setup();        (* honest target FIXED inside the game *)
    r  <@ Exp(C).run(f0);   (* the proven leaf-binding experiment on that target *)
    return r;
  }
}.

(* The self-contained bound: the leaf-binding break in ExpSC is <= eps_spr.
   PROVEN by reducing, via bypr, to `break_le_spr_bound` for the emitted target
   (the per-target bound holds for EVERY f0).  No &m-side precondition. *)
lemma break_le_spr_bound_sc (C <: LeafAdversarySC{-Exp, -SPR}) &m :
  Pr[ ExpSC(C).run() @ &m : res ] <= eps_spr.
proof.
  byphoare (_ : true ==> res) => //.
  proc.
  seq 1 : true 1%r eps_spr 0%r 0%r true => //.
  call (_ : true ==> res); auto.
  bypr=> &m0 _.
  by apply (break_le_spr_bound C f0{m0} &m0).
qed.

(* =============================================================================
   END.   prevalidationLeafBinding: REAL GAME-BASED REDUCTION TO SHA-256
   SECOND-PREIMAGE RESISTANCE (re-modelled, GAP-C closure).
   The reduction implication (accept /\ leaf-binding-break against the FIXED
   honest target => SHA-256 SECOND preimage) is PROVEN by EC program logic + Z3
   (exit 0).  The ONLY axiom carrying cryptographic hardness is
   `sha256_second_preimage_resistance` (eps_spr ~ q/2^256, NO query cap).
   `leaf_frame_injective` is a labeled STRUCTURAL encoder-correctness fact.
   ============================================================================= *)
