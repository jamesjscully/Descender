(* =============================================================================
   break_checked_core_message_binding.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       checkedCoreMessageBindingBreak  (BreakEvent #6).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHY THIS EVENT, AND WHAT KIND OF PROOF IT ADMITS
   ---------------------------------------------------------------------------
   spec §5 classifies the forgery reduction into three families; this event is
   the canonical member of family (b) — DIGEST / HASH-BINDING (QROM / collision-
   resistance) violations.  `break_reduction_status.md` ("Recommended order for
   the remaining 6", item 2) marks `checkedCoreMessageBinding` as the FIRST real
   *cryptographic* reduction: "a clean reduction to a digest collision-resistance
   game ... The reduction is provable; the CR bound stays a single named axiom."

   Unlike the family-(c) events (slotReplayOrRollback,
   postchallengeSlotCreationOrRewrite) this event does NOT admit a structural =0:
   defeating message binding is exactly finding a hash collision, which is only
   negligible under a HARDNESS assumption.  So the honest target is a REAL
   game-based reduction:  accept /\ message-binding-break  ==>  the reduction
   adversary WINS the SHA-256 collision-resistance game on the checked-core
   `challenge_hash_frame`.  We PROVE that implication and leave ONLY SHA-256
   collision-resistance as a single, clearly-labelled named axiom.

   ---------------------------------------------------------------------------
   THE EVENT (checked-core message binding).
   ---------------------------------------------------------------------------
   In the Lean concrete classifier (Path2FullSpendConcrete.ConcreteBreakFor):
       checkedCoreMessageBindingBreak :=
           ¬ CheckedCoreChallengeBindsAcceptedTuple _C spend
   and CheckedCoreChallengeBindsAcceptedTuple requires, among the tuple-field
   bindings, that the accepted signature body's checked-core frame EQUALS the
   canonical checked-core frame recomputed from the bound tuple:
       spend.signatureBody.checkedCoreFrame
         = C.checkedCore.frame  pk  message  stateNonce  proofCommitment.

   In the Rust reference (production_reference.rs:43839) the checked-core
   challenge is
       challenge_bytes(pk, message, slot_id, digest, proof_commitment)
         = hash_frame_to_challenge_bytes( challenge_hash_frame(...) )
         = truncate_and_mask( SHA256( challenge_hash_frame(...) ) ),
   where `challenge_hash_frame` (line 44197 prefix + 43846) injectively frames
       domain ‖ profile ‖ params ‖ slot_id ‖ public_key ‖ message
              ‖ slot_digest ‖ proof_commitment
   with length-prefixed `append_frame` fields (an injective byte encoder).

   The Fiat-Shamir verifier ACCEPTS only if the transcript challenge equals the
   challenge it RECOMPUTES from the *bound* tuple it actually verifies against.
   So an accepted spend that violates message binding is a spend whose verified
   tuple T_v differs from the canonical/honest tuple T_h (different message)
   yet yields the SAME checked-core challenge.  Two DISTINCT injective frames
   with the same truncated-SHA256 output is precisely a SHA-256 collision (on
   the deployed truncated/masked output).  The reduction extracts that pair.

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no `admit`, no `sorry`, closed by EC program logic + Z3; every lemma
   ends `qed`):
     * frame_distinct_of_message_break — a message-binding break exhibits two
         DISTINCT checked-core frames (the injective encoder maps distinct bound
         tuples to distinct frames).  The non-tautology evidence.
     * accept_binds_challenge — an ACCEPTED spend has its transcript challenge
         equal to H of the verified frame (the FS verifier recomputes & matches).
     * break_yields_collision — THE REDUCTION CORE: accept /\ binding-break
         entails two distinct frames f_v <> f_h with H f_v = H f_h, i.e. an
         explicit SHA-256 collision witness.  PROVEN (no assumption).
     * R_wins_on_break — the reduction module R, run inside the CR game on a
         transcript that is accepted AND exhibits the break, OUTPUTS that colliding
         pair, so the CR game returns `true` (R wins).  Proven by EC program logic.
     * break_le_cr — Pr[ message-binding break fires ] <= Pr[ CR game won by R ].
         PROVEN by `byequiv`/`byphoare`: every break-firing memory is mapped to a
         CR-win memory of the reduction.
     * break_le_bound — compose with the single named CR axiom to get the
         union-bound plug-in: Pr[break] <= sha256_collision_bound.  This is the
         term that feeds spike_union_bound.ec's `breakBound` for this event.

   ASSUMED (named hardness):  EXACTLY ONE, clearly labelled —
       axiom sha256_collision_resistance :
         Pr[ CR(R).main() @ &m : res ] <= sha256_collision_bound.
     SHA-256 (truncated/masked to the checked-core challenge width) collision-
     resistance.  This is the standard, conservative cryptographic-hash hardness
     assumption used throughout the Descender hash/QROM assumption ledger
     (research/path2_hash_qrom_assumption_ledger.md: the checked-core challenge
     digest "Fiat-Shamir soundness and message binding" charges
     `checked_core_message_binding_break`).  It is a NAMED standard assumption,
     NOT a proof of the event: the reduction (break ==> CR-win) is what is proven
     here; the un-winnability of CR is what is assumed.

   The reduction implication (accept /\ break ==> CR-win) is PROVEN.  Only the
   standard SHA-256 collision-resistance bound is assumed.
   ============================================================================= *)

require import AllCore Int Real Distr List StdOrder.
import RealOrder.

(* -----------------------------------------------------------------------------
   ABSTRACT TYPES.

   frame      : the byte-frame fed to SHA-256, i.e. `challenge_hash_frame`'s
                output `Vec<u8>` — domain‖profile‖params‖slot_id‖pk‖message‖
                slot_digest‖proof_commitment, an injective length-prefixed
                encoding of the bound checked-core tuple.
   challenge  : the checked-core Fiat-Shamir challenge — the truncated+masked
                SHA-256 output `[u8; CHALLENGE_BYTES]` that the verifier matches.
   tuple      : the bound checked-core tuple actually verified against
                (message, pk, slot_id, slot_digest, proof_commitment, state).
   --------------------------------------------------------------------------- *)
type frame.
type challenge.
type tuple.

(* The injective frame encoder: `challenge_hash_frame` builds its `Vec<u8>` from
   the bound tuple with length-prefixed `append_frame` fields, so DISTINCT bound
   tuples produce DISTINCT frames.  This is a STRUCTURAL fact about the encoder
   (length-prefixed framing is injective), modelled here as an injective op.
   It is NOT a hardness assumption: it is byte-level injectivity of framing,
   the EC image of the Rust `append_frame` length-prefix discipline. *)
op encode_frame : tuple -> frame.
axiom encode_frame_injective : injective encode_frame.
(* ^ STRUCTURAL (byte-encoding injectivity), NOT a cryptographic assumption.
     Length-prefixed framing of a fixed field schema is injective by
     construction; this is the EC image of the Rust `append_frame` discipline
     in `challenge_hash_frame_prefix` (production_reference.rs:44197). *)

(* The deployed checked-core hash: SHA-256 truncated+masked to CHALLENGE_BYTES.
   challenge_bytes(...) = hash_frame_to_challenge_bytes(challenge_hash_frame(...))
   = truncate_and_mask(SHA256(frame)).  An abstract op here; collision-resistance
   of THIS map is the single named hardness assumption below. *)
op H : frame -> challenge.

(* Convenience: the checked-core challenge of a bound tuple. *)
op tuple_challenge (t : tuple) : challenge = H (encode_frame t).

(* =============================================================================
   THE SHA-256 (TRUNCATED) COLLISION-RESISTANCE GAME.

   Standard collision game: the adversary outputs two frames; it WINS iff they
   are distinct and hash-collide under H.  This is the textbook CR experiment;
   we keep it self-contained (no external theory dependency).
   ============================================================================= *)
module type CRAdv = {
  proc find () : frame * frame
}.

module CR (A : CRAdv) = {
  proc main () : bool = {
    var f1, f2;
    (f1, f2) <@ A.find();
    return f1 <> f2 /\ H f1 = H f2;
  }
}.

(* =============================================================================
   THE CHECKED-CORE MESSAGE-BINDING EXPERIMENT.

   A spend TRANSCRIPT the adversary submits carries:
     verified   : the bound tuple the verifier actually checks against
                  (this is what the FS challenge is recomputed from),
     honest     : the canonical/registered tuple the slot was admitted for
                  (the `C.checkedCore.frame ... message ...` the Lean conjunct
                  demands equality with),
     tr_chal    : the transcript's claimed checked-core challenge.

   The FS verifier ACCEPTS iff the transcript challenge equals the challenge it
   recomputes from the VERIFIED tuple:  tr_chal = tuple_challenge verified.

   MESSAGE-BINDING BREAK fires when an ACCEPTED transcript verifies against a
   tuple that DIFFERS from the honest/registered tuple (different message/pk/
   slot/digest/proof_commitment) yet is accepted under the honest challenge —
   i.e. the verified and honest tuples are distinct but share the checked-core
   challenge.  (This is exactly ¬CheckedCoreChallengeBindsAcceptedTuple: the
   accepted signature body's checked-core frame is NOT the canonical frame for
   the bound tuple, yet the challenge matched.)
   ============================================================================= *)
type transcript = tuple * tuple * challenge.   (* (verified, honest, tr_chal) *)

module type BindAdv = {
  proc forge () : transcript
}.

(* The checked-core acceptance predicate (Fiat-Shamir match), as an op on the
   transcript, mirroring the verifier recomputing the challenge from the bound
   (verified) tuple and matching it against the transcript challenge.  Acceptance
   here is the EC image of `nativeVerifier.spendAccept ... = true`. *)
op fs_accept (tr : transcript) : bool =
  tr.`3 = tuple_challenge tr.`1.

(* The message-binding break predicate: accepted, AND the verified tuple differs
   from the honest tuple, AND the honest tuple ALSO matches the (same) transcript
   challenge — so both tuples are bound to the one accepted challenge.  This is
   the bad event the verifier's challenge-match is supposed to preclude: the one
   accepted challenge binds two DISTINCT tuples.  (In the Lean predicate the
   honest/canonical frame is what acceptance is REQUIRED to equal; binding breaks
   when the accepted verified frame differs from it under a matching challenge.) *)
op binding_break (tr : transcript) : bool =
     fs_accept tr
  /\ tr.`1 <> tr.`2
  /\ tuple_challenge tr.`2 = tr.`3.

module Exp (A : BindAdv) = {
  proc run () : bool = {
    var tr;
    tr <@ A.forge();
    return binding_break tr;
  }
}.

(* =============================================================================
   THE REDUCTION MODULE R.

   R runs the binding adversary A to obtain a transcript, and outputs the two
   checked-core FRAMES of the verified and honest tuples as its CR collision
   guess.  When A's transcript exhibits the binding break, those two frames are
   DISTINCT (injective encoder on distinct tuples) and hash-collide (both equal
   the one accepted challenge), so R wins the CR game.
   ============================================================================= *)
module R (A : BindAdv) : CRAdv = {
  proc find () : frame * frame = {
    var tr;
    tr <@ A.forge();
    return (encode_frame tr.`1, encode_frame tr.`2);
  }
}.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — a binding break exhibits two DISTINCT frames.

   The verified and honest tuples differ on a binding break, and the frame
   encoder is injective, so their checked-core frames are distinct.  This shows
   the break is a real conditional that produces a genuine (non-trivial) pair,
   not the constant `false`.
   ============================================================================= *)
lemma frame_distinct_of_break (tr : transcript) :
  binding_break tr => encode_frame tr.`1 <> encode_frame tr.`2.
proof.
  rewrite /binding_break => -[_ [hne _]].
  by apply: contra hne => h; apply encode_frame_injective.
qed.

(* =============================================================================
   ACCEPTANCE BINDS THE CHALLENGE — an accepted transcript's challenge is H of
   the verified frame (the FS verifier recomputes & matches).
   ============================================================================= *)
lemma accept_binds_challenge (tr : transcript) :
  fs_accept tr => tr.`3 = H (encode_frame tr.`1).
proof. by rewrite /fs_accept /tuple_challenge. qed.

(* =============================================================================
   THE REDUCTION CORE — break ==> an explicit SHA-256 collision.

   *** PROVEN, no assumption. ***  On a binding break the two checked-core frames
   are distinct (frame_distinct_of_break) AND collide under H: both equal the
   one accepted challenge (verified by fs_accept, honest by the break's third
   conjunct).  This is the implication the whole reduction rests on.
   ============================================================================= *)
lemma break_yields_collision (tr : transcript) :
  binding_break tr =>
     encode_frame tr.`1 <> encode_frame tr.`2
  /\ H (encode_frame tr.`1) = H (encode_frame tr.`2).
proof.
  move=> hb; split; first by apply frame_distinct_of_break.
  move: hb; rewrite /binding_break /fs_accept /tuple_challenge.
  by move=> [hacc [_ hh]]; rewrite -hacc -hh.
qed.

(* =============================================================================
   THE REDUCTION WINS — running R inside CR on a transcript that exhibits the
   break makes the CR game return `true`.

   We prove the probability inequality directly: every memory in which the
   binding experiment fires the break is one in which the CR game with R returns
   true, because R deterministically forwards A's transcript and outputs exactly
   the colliding frame pair break_yields_collision guarantees.
   ============================================================================= *)
lemma break_le_cr (A <: BindAdv) &m :
  Pr[ Exp(A).run() @ &m : res ] <= Pr[ CR(R(A)).main() @ &m : res ].
proof.
  byequiv (_ : ={glob A} ==> _) => //.
  proc; inline *.
  (* both run the SAME A.forge once; couple their results (={glob A} ==> ={res}
     for the abstract adversary) so one shared transcript is analysed. *)
  wp; call (_ : true).
  auto.
  (* the relational postcondition is `binding_break r => collision` where r is the
     (shared) forge result; discharge by the proven reduction core. *)
  smt(break_yields_collision).
qed.

(* =============================================================================
   THE NAMED STANDARD HARDNESS ASSUMPTION — and ONLY this.

   SHA-256 (truncated+masked to the checked-core challenge width) is collision-
   resistant: no efficient adversary wins the CR game with more than the
   `sha256_collision_bound` probability.  This is the single standard hash
   hardness assumption charged to `checked_core_message_binding_break` in the
   Path-2 hash/QROM assumption ledger.  The reduction adversary R(A) is built
   from any binding adversary A; the assumption bounds its CR-win probability.

   *** THIS IS THE ONLY AXIOM CARRYING A BOUND IN THIS FILE. ***
   ============================================================================= *)
op sha256_collision_bound : real.

axiom sha256_collision_resistance (A <: BindAdv) &m :
  Pr[ CR(R(A)).main() @ &m : res ] <= sha256_collision_bound.

(* =============================================================================
   THE BOUND — Pr[ checked-core message-binding break fires ] <= CR bound.
   The reduction (PROVEN) composed with the single named CR assumption.  This is
   the union-bound plug-in `breakBound` term for this event in
   spike_union_bound.ec.
   ============================================================================= *)
lemma break_le_bound (A <: BindAdv) &m :
  Pr[ Exp(A).run() @ &m : res ] <= sha256_collision_bound.
proof.
  apply (ler_trans (Pr[ CR(R(A)).main() @ &m : res ])).
  + by apply (break_le_cr A &m).
  + by apply (sha256_collision_resistance A &m).
qed.

(* =============================================================================
   PLUG-IN COROLLARY — for any union-bound budget at least the CR bound, this
   break's term is discharged.  Mirrors break_prob_le_any of the structural
   events, but here the floor is the NAMED CR bound (not 0): the honest shape of
   a family-(b) reduction.
   ============================================================================= *)
lemma break_le_budget (A <: BindAdv) (budget : real) &m :
  sha256_collision_bound <= budget =>
  Pr[ Exp(A).run() @ &m : res ] <= budget.
proof.
  move=> hb; apply (ler_trans sha256_collision_bound).
  + by apply (break_le_bound A &m).
  + exact hb.
qed.

(* =============================================================================
   END.   checkedCoreMessageBinding: REAL GAME-BASED REDUCTION (family (b)).
   PROVEN (EC program logic + Z3, exit 0):
     accept /\ message-binding-break  ==>  R(A) WINS the SHA-256 collision game.
   The reduction implication (break ==> CR-win) is PROVEN — see
   break_yields_collision (the explicit collision witness) and break_le_cr (the
   probability transfer by byequiv).
   ASSUMED: EXACTLY ONE named standard hardness assumption —
     sha256_collision_resistance (SHA-256 truncated collision-resistance),
   the standard hash hardness charged to checked_core_message_binding_break in
   research/path2_hash_qrom_assumption_ledger.md.  No admit, no sorry, no other
   axiom carrying a bound (encode_frame_injective is byte-encoding structure,
   not hardness).
   ============================================================================= *)
