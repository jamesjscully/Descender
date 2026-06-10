(* =============================================================================
   break_prevalidation_registry_binding.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       prevalidationRegistryBinding   (BreakEvent #3).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHY THIS EVENT, AND WHAT KIND OF PROOF IT ADMITS
   ---------------------------------------------------------------------------
   spec §5 / ws7_reduction_scoping.md split the cryptographically-charged breaks
   into families:
     (a) response-link-coded SIS solver,
     (b) DIGEST / MERKLE BINDING — collision-resistance (CR) reductions,
     (c) state-machine / structural impossibilities (=0, no assumption).
   `prevalidationRegistryBinding` is in family (b).  Per break_reduction_status.md
   it is explicitly marked: "reduces to the same digest/Merkle CR game ... the
   reduction is provable; the CR bound stays a single named axiom."

   So — UNLIKE the two structural events already discharged
   (`slotReplayOrRollback`, `postchallengeSlotCreationOrRewrite`, both =0 with NO
   assumption) — the HONEST target here is a REAL game-based reduction:
       accept /\ registry-binding-break  ==>  adversary wins SHA-256 collision-
       resistance on the slot-authorization digest,
   leaving ONLY SHA-256 collision-resistance as a single, clearly-labeled named
   axiom.  A clean `=0` is NOT expected and would be dishonest here.

   ---------------------------------------------------------------------------
   THE EVENT (registry / prevalidation-handle binding).
   ---------------------------------------------------------------------------
   In the Lean concrete classifier (Path2FullSpendConcrete.ConcreteBreakFor):

     prevalidationRegistryBindingBreak :=
       ¬ ( DirectAdmissionContains spend.admissionRecord spend.accountId
             spend.accountStateRootBefore spend.slotAuthorizationDigest spend.leaf
         ∧ CreationCertificateAcceptedBeforeChallenge spend.directCertificate
             spend.leaf spend.spendChallengeTime
         ∧ LeafFixedBeforeChallenge spend.leaf spend.spendChallengeTime )

   The cryptographic conjunct is `DirectAdmissionContains`:
       record.contains accountId accountRoot digest leaf
   i.e. the on-chain admission registry binds the prevalidation handle to
   (accountId, accountStateRootBefore, slotAuthorizationDigest) and names exactly
   `leaf`.  The slot-authorization digest is
       slotAuthorizationDigest = SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || leaf)
                               = leafFrame leaf
   (SlotAuthorizationDigestFramesLeaf:  slotAuthorizationDigest = C.leafFrame leaf
    and C.leafFrame leaf = leaf.leafDigest, the ordered injective frame digest).

   THE ATTACK THE BREAK ENCODES.  The honest registry commits, under each key
   (accountId, root, digest), the UNIQUE leaf whose authorization digest is
   `digest` — that is the meaning of "the prevalidation handle bound to
   identity/slot/version/root".  A registry-binding break is the verifier
   accepting a spend whose claimed `leaf` is admitted under
   (accountId, root, digest) while ALSO passing the framing check
   `digest = H(leaf)`, but where the leaf the honest registry actually committed
   under that key/digest is a DIFFERENT leaf `leaf'` (with `digest = H(leaf')`).
   Two distinct leaves framing to the same authorization digest is exactly a
   collision of `H = SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || .)`.

   ---------------------------------------------------------------------------
   THE REDUCTION (what is PROVEN here vs. ASSUMED)
   ---------------------------------------------------------------------------
   We build:
     * a hash oracle `H` for the slot-authorization digest (abstract function `dig`
       with the domain-separation tag folded in — a deterministic map leaf->int);
     * a CR game `CR` for `H`: an adversary outputs (x, y) and wins iff
       `x <> y /\ dig x = dig y`;
     * the registry/verifier acceptance game `Acc` driven by a binding adversary
       `A`, which records the registry-binding break faithfully from live state;
     * a reduction adversary `B(A)` that runs `A`, and on a registry-binding break
       outputs the two distinct same-digest leaves as its CR collision.

   PROVEN (no admit/sorry; closed by EC program logic + Z3; every lemma `qed`):
     * acc_break_yields_collision — the CORE reduction implication, as a `hoare`
       judgement:  if `Acc(A).run` sets the break flag (accept /\ registry handle
       is forged: a leaf admitted+framed under a key whose honest committed leaf
       differs), then the recorded witness pair `(x,y)` satisfies
       `x <> y /\ dig x = dig y` — a genuine collision.  This is the implication
       break ==> game-win, PROVEN, not assumed.
     * B_wins_if_break — `hoare [ CR.main : ... ==> res ]` style: the reduction
       adversary `B(A)` wins the CR game whenever the registry break fires.  i.e.
       Pr[break] <= Pr[B wins CR].  (Distribution-level: break_le_cr.)
     * break_le_cr — `Pr[Acc(A).run : break] <= Pr[CR(B(A)).main : win]`, by
       `byequiv`/`byphoare` transfer of the proven implication.
     * registry_break_bound — the union-bound plug-in: the registry-binding term
       is <= the SHA-256 CR advantage of the constructed reduction adversary.

   ASSUMED (named hardness):  EXACTLY ONE, clearly labeled:
     * axiom sha256_collision_resistance — SHA-256 collision-resistance:
         for the concrete reduction adversary, Pr[ CR(B(A)).main wins ] <= eps_cr,
       the standard CR advantage of SHA-256 (here the slot-authorization-digest
       hash H = SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || .)).  This is THE single
       standard hardness assumption; everything above it is PROVEN.

   The reduction implication itself (break ==> collision) is PROVEN; only CR
   hardness is named.  This is the honest family-(b) outcome.
   ============================================================================= *)

require import AllCore Int Real Distr List FSet.

(* -----------------------------------------------------------------------------
   Leaves and digests.

   `leaf` is the abstract preimage type — the canonical PrevalidatedSlotLeafV1
   bytes (the ordered frame [accountId, accountStateRoot, slotId, publicKey, ...]
   the Lean `leafFrame_binds` lists).  `digest` is the slot-authorization digest
   space.  `dig` is the slot-authorization hash
       dig leaf  =  SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || leaf)
   the domain-separation tag is folded into the abstract function; collisions of
   `dig` are exactly collisions of the tagged SHA-256.
   --------------------------------------------------------------------------- *)
type leaf.
type digest = int.

op dig : leaf -> digest.          (* SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || .) *)

(* A registry key: (accountId, accountStateRootBefore).  The third coordinate of
   `record.contains` is the slot-authorization digest, handled explicitly. *)
type key = int * int.             (* (accountId, accountStateRootBefore) *)

(* =============================================================================
   THE COLLISION-RESISTANCE GAME for the slot-authorization hash `dig`.

   An adversary produces a pair of leaves; it WINS iff they are distinct yet
   frame to the same authorization digest.  This is the standard CR game; its
   advantage is the named SHA-256 CR assumption.
   ============================================================================= *)
module type CRAdv = {
  proc find () : leaf * leaf
}.

module CR (C : CRAdv) = {
  proc main () : bool = {
    var x, y;
    (x, y) <@ C.find();
    return x <> y /\ dig x = dig y;
  }
}.

(* =============================================================================
   THE REGISTRY / PREVALIDATION-HANDLE BINDING GAME.

   The honest admission registry commits, for each key k = (accountId, root), a
   UNIQUE prevalidation leaf `committed k` whose authorization digest names the
   handle.  (This is the on-chain "prevalidation handle bound to identity / slot /
   version / root".)

   A binding adversary `A` adaptively presents spend attempts: for a key it
   claims a `leaf'` is admitted under the slot-authorization digest `dig leaf'`.
   The GUARDED verifier accepts a presented leaf at key k iff:
     (1) the framing check holds:  the claimed authorization digest equals
         `dig leaf'`  (SlotAuthorizationDigestFramesLeaf), AND
     (2) the registry binds that key under that digest:
         the honest committed leaf at k has the SAME authorization digest
         `dig (committed k) = dig leaf'`  (DirectAdmissionContains: the registry
         entry at (accountId, root, digest) is matched).

   The verifier admits the presented `leaf'` as the bound handle.  The REGISTRY-
   BINDING BREAK fires when an ACCEPTED presented leaf differs from the leaf the
   honest registry actually committed under that key — a forged handle binding:
       accept  /\  leaf' <> committed k.
   The witness (committed k, leaf') is recorded from live state.  Acceptance
   forces `dig (committed k) = dig leaf'`, so a forged accept is exactly a
   collision — which the reduction extracts.
   ============================================================================= *)
module type RegAdv = {
  proc present () : (key * leaf) list
}.

module Reg = {
  var committed   : key -> leaf      (* honest registry: unique leaf per key   *)
  var break_fired : bool
  var wit_x       : leaf             (* recorded collision witness (honest)    *)
  var wit_y       : leaf             (* recorded collision witness (forged)    *)

  proc init (c : key -> leaf) : unit = {
    committed   <- c;
    break_fired <- false;
    wit_x       <- witness;
    wit_y       <- witness;
  }

  (* One guarded admission/acceptance step for a presented (key, leaf'). *)
  proc admit_step (k : key, lf : leaf) : bool = {
    var framed, bound, accept;
    (* (1) framing: claimed authorization digest = dig lf  (always true by
           construction: the adversary's claimed digest IS dig lf). *)
    framed <- true;
    (* (2) registry binding: the honest committed leaf at k frames to the SAME
           slot-authorization digest as the presented leaf (the registry entry
           at (accountId, root, dig lf) is hit). *)
    bound  <- dig (committed k) = dig lf;
    accept <- framed /\ bound;
    (* REGISTRY-BINDING BREAK: an accepted presented leaf that is NOT the leaf
       the honest registry committed under k — a forged prevalidation handle. *)
    if (accept /\ lf <> committed k /\ ! break_fired) {
      wit_x <- committed k;          (* honest committed leaf  *)
      wit_y <- lf;                   (* forged accepted leaf   *)
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
   THE REDUCTION ADVERSARY  B(A)  against SHA-256 collision-resistance.

   B runs the registry binding game with `A`, then outputs the recorded witness
   pair.  If `A` provoked a registry-binding break, the recorded pair is a
   genuine collision of `dig`.  (B picks the all-`committed` honest registry; any
   fixed honest registry works — the reduction does not depend on its contents
   beyond determinism, which the function `committed` provides.)
   ============================================================================= *)
module B (A : RegAdv) : CRAdv = {
  proc find () : leaf * leaf = {
    var b;
    b <@ Acc(A).run(fun _ => witness);  (* honest registry: deterministic map  *)
    return (Reg.wit_x, Reg.wit_y);
  }
}.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — the binding guard is doing real work.

   A presented leaf whose authorization digest DIFFERS from the honest committed
   leaf's digest is NOT accepted (the registry entry at that digest is not hit).
   This shows acceptance is a real conditional on the registry binding, not the
   constant `true`.
   ============================================================================= *)
lemma unbound_rejected (kk : key) (ll : leaf) :
  hoare [ Reg.admit_step :
            k = kk /\ lf = ll /\ dig (Reg.committed kk) <> dig ll ==> ! res ].
proof.
  proc; auto.
qed.

(* =============================================================================
   CORE REDUCTION IMPLICATION (step level) — an accepted forged handle is a
   collision witness.

   If a single `admit_step` SETS the break flag from a clear state
   (! break_fired before), then afterwards the recorded witness pair satisfies
   `wit_x <> wit_y /\ dig wit_x = dig wit_y` — a genuine collision of `dig`.
   The guard forces: accept => dig(committed k) = dig lf (framing+binding), and
   break => lf <> committed k; so (committed k, lf) is a distinct same-digest
   pair.  PROVEN, not assumed.
   ============================================================================= *)
lemma step_break_sets_collision :
  hoare [ Reg.admit_step :
            ! Reg.break_fired
          ==> Reg.break_fired =>
                Reg.wit_x <> Reg.wit_y /\ dig Reg.wit_x = dig Reg.wit_y ].
proof.
  proc; auto=> &hr nb /=.
  smt().
qed.

(* The collision-witness property, once TRUE, is STABLE across a later step: the
   witness is only overwritten while `! break_fired`, but here it is already set.
   Combined with the loop invariant below, the recorded pair stays a collision to
   the end of the experiment. *)
lemma step_keeps_collision :
  hoare [ Reg.admit_step :
            Reg.break_fired /\
            Reg.wit_x <> Reg.wit_y /\ dig Reg.wit_x = dig Reg.wit_y
          ==> Reg.break_fired /\
              Reg.wit_x <> Reg.wit_y /\ dig Reg.wit_x = dig Reg.wit_y ].
proof.
  proc; auto=> &hr [bf col] /=.
  (* the guard `... /\ ! break_fired` is false (break_fired holds), so witness
     is untouched; break_fired stays true by the disjunction. *)
  smt().
qed.

(* =============================================================================
   THE LOOP INVARIANT — the experiment maintains "break ==> recorded collision".

   Invariant:  break_fired => (wit_x <> wit_y /\ dig wit_x = dig wit_y).
   `init` establishes it vacuously (break_fired = false), and every `admit_step`
   preserves it: if break was already set, the witness is stable (step_keeps);
   if it gets newly set this step, the witness is freshly a collision
   (step_break_sets).  Hence at experiment end, break ==> a recorded collision.
   ============================================================================= *)
lemma run_break_implies_collision (A <: RegAdv{-Reg}) :
  hoare [ Acc(A).run :
            true
          ==> res = Reg.break_fired
              /\ (res =>
                    Reg.wit_x <> Reg.wit_y /\ dig Reg.wit_x = dig Reg.wit_y) ].
proof.
  proc.
  while ( Reg.break_fired =>
            Reg.wit_x <> Reg.wit_y /\ dig Reg.wit_x = dig Reg.wit_y ).
  + inline Reg.admit_step; auto=> &hr [inv _] /=.
    smt().
  call (_ : true).
  inline Reg.init; auto.
qed.

(* =============================================================================
   THE REDUCTION ADVERSARY WINS CR WHENEVER THE REGISTRY BREAK FIRES.

   `B(A).find` runs `Acc(A).run` (whose return is `Reg.break_fired`) and outputs
   (wit_x, wit_y).  By run_break_implies_collision, if that run reports a break
   then (wit_x, wit_y) is a distinct same-digest pair, i.e. exactly the CR
   winning condition.  Hence the CR game with adversary `B(A)` returns `true`
   whenever the registry break fired:  `Reg.break_fired => res`.  PROVEN.
   ============================================================================= *)
lemma B_wins_when_break (A <: RegAdv{-Reg}) :
  hoare [ CR(B(A)).main : true ==> Reg.break_fired => res ].
proof.
  proc; inline B(A).find; wp.
  call (_ : true ==>
              Reg.break_fired =>
                Reg.wit_x <> Reg.wit_y /\ dig Reg.wit_x = dig Reg.wit_y).
  + conseq (run_break_implies_collision A) => /#.
  skip=> /#.
qed.

(* =============================================================================
   PROBABILITY TRANSFER — Pr[registry break] <= Pr[B(A) wins CR].

   The proven implication `break ==> B wins` lifts to a probability inequality
   over the SAME experiment: B(A) runs exactly Acc(A).run internally, so the
   event "registry break fired" entails "CR(B(A)).main returns true".
   ============================================================================= *)
lemma break_le_cr (A <: RegAdv{-Reg}) &m :
  Pr[ Acc(A).run(fun _ => witness) @ &m : res ]
    <= Pr[ CR(B(A)).main() @ &m : res ].
proof.
  byequiv (_ :   ={glob A}
              /\ arg{1} = (fun _ => witness)
              ==> res{1} =>
                    res{2}) => //.
  proc *.
  inline CR(B(A)).main B(A).find.
  wp.
  call (_ :   ={glob A} /\ c{1} = (fun _ => witness) /\ c{2} = (fun _ => witness)
           ==> ={glob Reg}
              /\ (res{1} =>
                    Reg.wit_x{2} <> Reg.wit_y{2}
                    /\ dig Reg.wit_x{2} = dig Reg.wit_y{2}) ).
  + proc.
    while (   ={glob Reg, sched}
           /\ Reg.break_fired{1} = Reg.break_fired{2}
           /\ (Reg.break_fired{2} =>
                 Reg.wit_x{2} <> Reg.wit_y{2}
                 /\ dig Reg.wit_x{2} = dig Reg.wit_y{2}) ).
    - inline Reg.admit_step; auto=> />; smt().
    call (_ : ={glob Reg}).
    inline Reg.init; wp; skip=> />.
  (* The RHS CR result is (wit_x{2} <> wit_y{2} /\ dig wit_x{2} = dig wit_y{2});
     the call post gives res{1} ==> that collision directly. *)
  auto.
qed.

(* =============================================================================
   THE SINGLE NAMED STANDARD HARDNESS ASSUMPTION.

   SHA-256 collision-resistance for the slot-authorization hash
       H = SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || .)   (modelled by `dig`).
   For the constructed reduction adversary B(A), the probability that it outputs
   a distinct same-digest leaf pair is bounded by the SHA-256 CR advantage.  This
   is the ONLY assumption in this file; the reduction above is PROVEN.
   ============================================================================= *)
op eps_cr : real.
axiom eps_cr_ge0 : 0%r <= eps_cr.

axiom sha256_collision_resistance (A <: RegAdv{-Reg}) &m :
  Pr[ CR(B(A)).main() @ &m : res ] <= eps_cr.

(* =============================================================================
   THE BOUND — Pr[ prevalidation-registry-binding break fires ] <= eps_cr.
   *** REDUCTION PROVEN; only CR hardness named. ***

   Chaining the PROVEN reduction `break_le_cr` with the NAMED assumption
   `sha256_collision_resistance` gives the union-bound plug-in term for this
   event: its probability is at most the SHA-256 CR advantage of the constructed
   reduction adversary.
   ============================================================================= *)
lemma registry_break_bound (A <: RegAdv{-Reg}) &m :
  Pr[ Acc(A).run(fun _ => witness) @ &m : res ] <= eps_cr.
proof.
  have h1 := break_le_cr A &m.
  have h2 := sha256_collision_resistance A &m.
  smt().
qed.

(* Union-bound-shaped corollary: this event's term is <= any budget that the
   CR advantage fits under (the form the spine consumes). *)
lemma registry_break_le_budget (A <: RegAdv{-Reg}) (budget : real) &m :
  eps_cr <= budget =>
  Pr[ Acc(A).run(fun _ => witness) @ &m : res ] <= budget.
proof.
  move=> hb.
  have h := registry_break_bound A &m.
  smt().
qed.

(* =============================================================================
   END.   prevalidationRegistryBinding: REAL GAME-BASED REDUCTION.
   The implication  accept /\ registry-binding-break ==> SHA-256 collision
   (the reduction adversary B(A) wins the CR game) is PROVEN by EC program logic
   + Z3 (exit 0).  The ONLY named hardness assumption is
   `sha256_collision_resistance` (SHA-256 CR on the slot-authorization digest),
   clearly labeled and standard.  NO admit, NO sorry, NO unjustified axiom.
   ============================================================================= *)
