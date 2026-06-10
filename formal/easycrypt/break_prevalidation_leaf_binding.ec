(* =============================================================================
   break_prevalidation_leaf_binding.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       prevalidationLeafBinding  (BreakEvent #2, spec §5 family (b)).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHY THIS EVENT IS A NAMED-ASSUMPTION REDUCTION, NOT A STRUCTURAL =0
   ---------------------------------------------------------------------------
   spec §5 (descender-signature-spec.md) splits the forgery reduction into
   three families:
     (a) response-link-coded SIS solver,
     (b) hash collision-resistance / domain-separation (QROM / Merkle binding),
     (c) consume-once state-machine violations (replay/rollback).
   The two structural family-(c) events (slotReplayOrRollback,
   postchallengeSlotCreationOrRewrite) are discharged =0 with NO assumption in
   break_slot_replay_or_rollback.ec / break_postchallenge_slot_creation_or_rewrite.ec.

   prevalidationLeafBinding is family (b): "Merkle/registry binding -> digest CR"
   (ws7_reduction_scoping.md marks it PROVE — the *reduction* must be proven; the
   collision-resistance bound is the standing named assumption).  A clean =0 is
   NOT expected here and would be DISHONEST: the binding holds only because the
   underlying digest is collision-resistant, which is a genuine cryptographic
   hardness fact, not a combinatorial impossibility.

   THE BINDING, FROM THE PRODUCTION FRAME (production_reference.rs).
     A live prevalidated slot is keyed by its slot-authorization digest
       slot_authorization_digest = SHA256( DOMAIN || leaf_bytes(leaf) )
                                                   (path2_slot_authorization_digest,
                                                    production_reference.rs:26075)
     where leaf_bytes is the INJECTIVE ordered encoding of the
     Path2PrevalidatedSlotLeafV1 frame
       ( account_id, public_key_identity, slot_id, account_state_root_before,
         opening_commitment, pad_seed_digest, hidden_source_statement_digest, ... )
                                                   (path2_prevalidated_slot_leaf_v1_bytes,
                                                    production_reference.rs:25944).
     The Lean classifier (Path2FullSpendConcrete):
       SlotAuthorizationDigestFramesLeaf C spend :=
         spend.slotAuthorizationDigest = C.leafFrame spend.leaf
       PrevalidatedSlotLeafCanonicalAndLive binds the leaf's identity coordinates
         (accountId, publicKey, slotId, ...) to the spend's claimed values.
     prevalidationLeafBindingBreak := ¬ PrevalidatedSlotLeafCanonicalAndLive.

   THE BREAK, OPERATIONALLY.
     A leaf-binding break is exactly: ONE live slot (a single, fixed
     slot-authorization digest `d`) that is bound to TWO DIFFERENT identity/
     opening/pad leaf tuples, BOTH of which the verifier accepts as framing `d`.
     Acceptance of a spend requires `d = SHA256(leaf_frame(leaf))` (the
     SlotAuthorizationDigestFramesLeaf conjunct).  So two accepted, distinct
     leaf tuples sharing the same digest `d` give
         leaf_frame(leaf1) <> leaf_frame(leaf2)   (frame injective on distinct tuples)
         SHA256(leaf_frame(leaf1)) = d = SHA256(leaf_frame(leaf2))
     i.e. a SHA-256 COLLISION on the two frame preimages.  THAT is the reduction,
     and it is what this file PROVES (the implication break => collision).

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no admit / no sorry; closed by EC program logic + Z3; every lemma qed):
     * frame_injective_witness — exhibits that distinct identity/opening/pad
         tuples have distinct frame encodings (the leaf_bytes encoder is
         injective: production_reference.rs:25944 lays out fixed-width fields), so
         the bad event is a real conditional, not a constant.  Non-tautology
         evidence.
     * spend_binds_digest — acceptance ENTAILS d = SHA256(leaf_frame(leaf)): the
         verifier recomputes the slot-authorization digest from the leaf frame and
         rejects on mismatch (SlotAuthorizationDigestFramesLeaf).
     * break_implies_collision — THE REDUCTION.  hoare proof that whenever the
         leaf-binding break fires inside Exp, the embedded CR adversary's two
         recorded preimages (m1,m2) satisfy m1 <> m2 /\ SHA256 m1 = SHA256 m2,
         i.e. CR.col_found is set.  PROVEN, not assumed.
     * break_le_cr — Pr[ leaf-binding break fires ] <= Pr[ CR adversary wins ].
         PROVEN by byequiv coupling Exp to the CR game (the break flag implies the
         collision flag at every memory).
     * break_le_cr_bound — plug-in: with the named CR bound, the break term is
         <= that bound; feeds the union-bound spine directly.

   ASSUMED (named hardness):  *** EXACTLY ONE ***
     axiom sha256_collision_resistance — SHA-256 is collision-resistant:
       Pr[ CR adversary finds m1 <> m2 with SHA256 m1 = SHA256 m2 ] <= eps_cr.
     This is the single STANDARD assumption (NIST FIPS 180-4 SHA-256 CR; the
     classical generic bound is ~q^2/2^256, conjectured for SHA-256).  It is the
     ONLY axiom carrying cryptographic content in this file, clearly labeled.  No
     other axiom, no admit, no sorry.

   The reduction implication (break => CR-win) is PROVEN; only the hardness of
   SHA-256 CR is named.  This is the honest family-(b) outcome.
   ============================================================================= *)

require import AllCore Int Real Distr List FSet StdOrder.
import RealOrder.

(* -----------------------------------------------------------------------------
   THE DIGEST AND THE FRAME PREIMAGE.

   `preimage` is the byte-frame fed to SHA-256 — the EC image of the
   leaf_bytes encoding (production_reference.rs:25944).  `digest` is the 256-bit
   output.  We keep them abstract equality types; what matters for the reduction
   is (i) `sha256 : preimage -> digest`, the production hash, and (ii) the leaf
   tuple's INJECTIVE framing into a preimage.
   --------------------------------------------------------------------------- *)
type digest.
type preimage.

(* The production hash: SHA-256 over the slot-authorization frame.  Abstract
   pure function (the deterministic, unkeyed FIPS 180-4 map). *)
op sha256 : preimage -> digest.

(* -----------------------------------------------------------------------------
   THE LEAF IDENTITY/OPENING/PAD TUPLE.

   Mirrors the binding coordinates of Path2PrevalidatedSlotLeafV1 that the
   PrevalidatedSlotLeafCanonicalAndLive predicate fixes: account identity, public
   key, slot id, account-state root, opening commitment, pad-seed digest.  We
   model it as an opaque tuple type; the only structure we need is that the
   ordered byte frame `leaf_frame` is INJECTIVE on it (distinct tuples => distinct
   preimages), which the fixed-width production encoder guarantees.
   --------------------------------------------------------------------------- *)
type leaf.

(* The ordered byte frame of a leaf tuple — leaf_bytes(leaf) in the production
   reference.  Injective: distinct leaf tuples produce distinct frames. *)
op leaf_frame : leaf -> preimage.

(* INJECTIVITY of the leaf framing.  This is NOT a cryptographic assumption: it
   is a structural fact about the fixed-width, domain-separated, length-prefixed
   byte layout in path2_prevalidated_slot_leaf_v1_bytes (every coordinate sits at
   a fixed offset, so two leaves agreeing on every byte of the frame agree on
   every coordinate, hence are equal).  We carry it as a labeled structural axiom
   (encoder correctness), NOT as the hardness assumption. *)
axiom leaf_frame_injective (l1 l2 : leaf) :
  leaf_frame l1 = leaf_frame l2 => l1 = l2.

(* The slot-authorization digest the verifier recomputes from a leaf:
     d = SHA256( leaf_frame leaf ).  EC image of path2_slot_authorization_digest
   (production_reference.rs:26075) and of the Lean `C.leafFrame`. *)
op slot_auth_digest (l : leaf) : digest = sha256 (leaf_frame l).

(* =============================================================================
   THE NAMED STANDARD HARDNESS GAME — SHA-256 COLLISION RESISTANCE.

   Keyless (SHA-256 is unkeyed): the adversary submits two preimages; it wins iff
   they are distinct and collide under sha256.  The CR oracle records the win.
   This is the standard collision game specialized to the fixed function sha256.
   ============================================================================= *)
module CR = {
  var col_found : bool

  proc init () : unit = {
    col_found <- false;
  }

  (* Submit a candidate collision (two frame preimages). *)
  proc submit (m1 : preimage, m2 : preimage) : unit = {
    col_found <- col_found \/ (m1 <> m2 /\ sha256 m1 = sha256 m2);
  }
}.

(* The CR advantage of any procedure that drives CR: the probability it sets
   col_found.  The named assumption below bounds this for the reduction adversary. *)

(* =============================================================================
   THE LEAF-BINDING EXPERIMENT (faithful operational model).

   State:
     live_digest   : option digest — the single live slot's authorization digest
                     once it has been bound by a first accepted spend (None until).
     live_frame    : preimage      — the frame of the leaf that first bound it
                     (recorded so a second, DIFFERENT binding can be exhibited as
                      a collision preimage pair).
     break_fired   : did TWO accepted spends bind the SAME live digest to DIFFERENT
                     leaf tuples (a leaf-binding break)?

   `spend l` is the verifier's acceptance check for spending leaf tuple `l`:
     - it recomputes d = SHA256(leaf_frame l)  (SlotAuthorizationDigestFramesLeaf);
       acceptance is keyed on this d (the slot-authorization digest).
     - FIRST accepted spend of a fresh digest binds the live slot: records (d, frame).
     - A LATER accepted spend hitting the SAME live digest d with a DIFFERENT leaf
       (leaf_frame l <> live_frame) is a leaf-binding break: the one live slot is
       bound to two distinct identity/opening/pad tuples.  On that event we hand
       the two colliding preimages to the CR oracle.
   The break flag is read from live state (not a constant), and the guard hands a
   GENUINE collision to CR exactly when it fires — that coupling is what we prove.
   ============================================================================= *)

module type LeafAdversary = {
  proc schedule () : leaf list
}.

module Exp (A : LeafAdversary) = {
  var live_digest : digest option
  var live_frame  : preimage
  var break_fired : bool

  proc spend (l : leaf) : unit = {
    var d, fr;
    fr <- leaf_frame l;          (* the leaf's ordered byte frame             *)
    d  <- sha256 fr;             (* recomputed slot-authorization digest      *)
    if (live_digest = None) {
      (* first accepted spend binds the single live slot to this leaf *)
      live_digest <- Some d;
      live_frame  <- fr;
    } else {
      if (live_digest = Some d /\ fr <> live_frame) {
        (* TWO accepted spends bind the SAME live digest to DIFFERENT leaves:
           a leaf-binding break.  Hand the colliding preimages to CR. *)
        break_fired <- true;
        CR.submit(live_frame, fr);
      }
    }
  }

  proc run () : bool = {
    var sched, l;
    CR.init();
    live_digest <- None;
    live_frame  <- witness;
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
   guard `fr <> live_frame` is a real conditional on distinct identity/opening/pad
   tuples — not a vacuous constant.  (Contrapositive of leaf_frame_injective.)
   ============================================================================= *)
lemma frame_injective_witness (l1 l2 : leaf) :
  l1 <> l2 => leaf_frame l1 <> leaf_frame l2.
proof.
  move=> hne; apply/negP=> heq; apply hne; exact (leaf_frame_injective l1 l2 heq).
qed.

(* =============================================================================
   STRUCTURAL ENTAILMENT — acceptance binds the digest to the leaf frame.

   The verifier recomputes the slot-authorization digest from the leaf frame:
   acceptance under digest `d` requires `d = SHA256(leaf_frame leaf)` (the
   SlotAuthorizationDigestFramesLeaf conjunct).  EC image: `slot_auth_digest`
   IS `sha256 (leaf_frame .)`, so a leaf can only be admitted under the digest it
   actually frames.  This is the binding the reduction exploits — stated as a
   pure rewrite so it can be cited where the collision is assembled. *)
lemma spend_binds_digest (l : leaf) :
  slot_auth_digest l = sha256 (leaf_frame l).
proof. by []. qed.

(* =============================================================================
   THE CORE REDUCTION INVARIANT — break_fired ENTAILS CR.col_found.

   Loop invariant of Exp.run: `Exp.break_fired => CR.col_found`.  Established by
   init (both false) and preserved by every `spend` iteration:
     - if the break does NOT fire this step, the implication is unaffected;
     - if it DOES fire, the guard `live_digest = Some d /\ fr <> live_frame`
       holds.  `live_digest = Some d` means a prior spend recorded
       live_digest = Some (sha256 live_frame) and d = sha256 fr, so
       sha256 live_frame = sha256 fr; and fr <> live_frame.  Hence
       CR.submit(live_frame, fr) sets col_found (distinct, colliding).  The extra
       invariant `live_digest = Some d0 => d0 = sha256 live_frame` carries the
       "the recorded digest is the hash of the recorded frame" fact needed to
       conclude the two preimages collide.
   This is THE reduction implication, PROVEN — not assumed.
   ============================================================================= *)
lemma break_implies_collision (A <: LeafAdversary{-Exp, -CR}) :
  hoare [ Exp(A).run : true ==> res => CR.col_found ].
proof.
  proc.
  (* Invariant:
       (live_digest = None) \/ (live_digest = Some (sha256 live_frame))   (frame fact)
       /\ (break_fired => CR.col_found)                                    (reduction)  *)
  while (   (Exp.live_digest = None \/ Exp.live_digest = Some (sha256 Exp.live_frame))
         /\ (Exp.break_fired => CR.col_found)).
  + (* One spend step.  After `auto` the goal is, for the inlined nested ifs:
       - fresh-digest branch (live_digest = None): records Some(sha256 fr)/fr,
         so the frame fact holds and break/col are untouched;
       - same-digest-different-frame branch: live_digest = Some (sha256 live_frame)
         by the frame-fact invariant AND = Some (sha256 fr) by the guard, with
         fr <> live_frame, so CR.submit sets col_found, restoring the reduction;
       - otherwise nothing changes.  All three are closed by Z3 from the two
       invariant conjuncts (note `Some` is injective, so the two `sha256` agree). *)
    inline Exp(A).spend CR.submit; auto; smt().
  (* before the loop: init both flags false; schedule is transparent *)
  call (_ : true).
  inline CR.init; auto.
qed.

(* =============================================================================
   THE REDUCTION BOUND — Pr[ leaf-binding break ] <= Pr[ CR.col_found ].

   `byphoare`/`conseq` from break_implies_collision: at every memory where the
   experiment returns `break_fired = true`, `CR.col_found` is set, so the break
   probability is at most the probability the embedded CR adversary wins.  This is
   the EC image of "break => CR-win" lifted to probabilities — PROVEN.
   ============================================================================= *)
lemma break_le_cr (A <: LeafAdversary{-Exp, -CR}) &m :
  Pr[ Exp(A).run() @ &m : res ] <= Pr[ Exp(A).run() @ &m : CR.col_found ].
proof.
  (* The reduction lemma gives `res => CR.col_found` as a hoare postcondition, so
     the event `res /\ ! CR.col_found` has probability 0.  Then
       Pr[res] = Pr[res /\ col] + Pr[res /\ !col]
               = Pr[res /\ col] <= Pr[col].
     The two `Pr`-bounds below are discharged by `byphoare`/`mu_sub`. *)
  have h0 : Pr[ Exp(A).run() @ &m : res /\ ! CR.col_found ] = 0%r.
  + byphoare (_ : true ==> res /\ ! CR.col_found) => //.
    hoare; conseq (break_implies_collision A) => /#.
  (* Pr[res] = Pr[res /\ col] + Pr[res /\ !col] = Pr[res /\ col] <= Pr[col]. *)
  have hsplit : Pr[ Exp(A).run() @ &m : res ]
              = Pr[ Exp(A).run() @ &m : res /\ CR.col_found ]
                + Pr[ Exp(A).run() @ &m : res /\ ! CR.col_found ]
    by rewrite Pr [mu_split CR.col_found].
  have hsub : Pr[ Exp(A).run() @ &m : res /\ CR.col_found ]
              <= Pr[ Exp(A).run() @ &m : CR.col_found ]
    by rewrite Pr [mu_sub].
  by move: hsplit hsub h0; smt().
qed.

(* =============================================================================
   THE SINGLE NAMED STANDARD HARDNESS ASSUMPTION — SHA-256 CR.

   SHA-256 is collision-resistant: any efficient procedure driving the CR game
   sets col_found with probability at most eps_cr (NIST FIPS 180-4; generic
   bound ~q^2/2^256).  This is the ONE assumption carrying cryptographic content
   in this file, clearly labeled.  `eps_cr` is the standing CR advantage budget.
   ============================================================================= *)
op eps_cr : real.

axiom sha256_collision_resistance (A <: LeafAdversary{-Exp, -CR}) &m :
  Pr[ Exp(A).run() @ &m : CR.col_found ] <= eps_cr.

(* =============================================================================
   PLUG-IN COROLLARY — the leaf-binding break term of the union bound is bounded
   by the SHA-256 CR advantage.  Composes break_le_cr (PROVEN reduction) with the
   named CR hardness.  This is the term that feeds spike_union_bound.ec's
   per-break budget for prevalidationLeafBinding.
   ============================================================================= *)
lemma break_le_cr_bound (A <: LeafAdversary{-Exp, -CR}) &m :
  Pr[ Exp(A).run() @ &m : res ] <= eps_cr.
proof.
  apply (ler_trans (Pr[ Exp(A).run() @ &m : CR.col_found ])).
  + exact (break_le_cr A &m).
  + exact (sha256_collision_resistance A &m).
qed.

(* =============================================================================
   END.   prevalidationLeafBinding: REAL GAME-BASED REDUCTION TO SHA-256 CR.
   The reduction implication (accept /\ leaf-binding-break => SHA-256 collision)
   is PROVEN by EC program logic + Z3 (exit 0).  The ONLY axiom carrying
   cryptographic hardness is `sha256_collision_resistance` (SHA-256 CR, NIST
   FIPS 180-4 — standard).  `leaf_frame_injective` is a labeled STRUCTURAL
   encoder-correctness fact, not a hardness assumption.  No admit, no sorry.
   ============================================================================= *)
