(* =============================================================================
   break_checked_core_message_binding.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       checkedCoreMessageBindingBreak  (BreakEvent #6).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   *** GAP-C RE-MODEL (2026-06-10): SECOND-PREIMAGE ON A FIXED 214-BIT TARGET ***
   ---------------------------------------------------------------------------
   #6 VERDICT (rigorous target-vs-free determination): SECOND-PREIMAGE, FINE.

   The previous version modelled the break as a FREE birthday collision: the
   adversary output BOTH frames f1, f2 with H f1 = H f2 (CR game), charging
   eps ~ q^2/2^214 = 2^-86 at q = 2^64 — which BLOWS the budget.  That was the
   weak-binding model and is operationally WRONG.  The classic weak-vs-strong-
   binding determination, done from the actual production structure, is:

   WHY THE TARGET CHALLENGE IS FIXED BEFORE THE ADVERSARY ACTS (not free).
     The checked-core challenge is
        c = H(challenge_hash_frame(pk, message, slot_id, slot_digest,
                                   proof_commitment))
                                   (production_reference.rs:43839/43860, H =
                                    truncate_and_mask o SHA256, 214-bit width).
     The Sigma response the verifier checks is bound to BOTH the challenge AND
     the proof_commitment:  the verifier (production_reference.rs:3159-3173)
       (1) recomputes c from the bound frame (which INCLUDES proof_commitment),
       (2) checks  response == expected_syndrome(c, handle, proof_commitment).
     In the SUF-CMA / message-binding game the adversary FIRST obtains an honest
     signature on message m: an honest transcript whose response is valid only
     against the honest challenge c = H(frame(m, pc)) for the honest
     proof_commitment pc.  Producing a fresh valid response for a DIFFERENT
     challenge requires the SIS-hard secret (that is a SEPARATE event — the
     response-link SIS solver, family (a)); the message-binding adversary does
     NOT get to mint responses.  Hence to transfer the honest response to a
     different message m' (m' <> m) the adversary must keep the SAME
     proof_commitment pc and the SAME challenge c, i.e. find
         frame(m', pc) <> frame(m, pc)   with   H(frame(m', pc)) = c.
     The target c = H(frame(m, pc)) is FIXED by the honest signature BEFORE the
     adversary commits m'.  This is a SECOND-PREIMAGE on the fixed 214-bit
     target, NOT a free collision: the adversary cannot choose both sides
     offline because one side (the honest frame and its challenge) is pinned by
     the honest signing query and the response is bound to it.

   NO FREE-COLLISION SURFACE EXISTS at 214 bits here: a free collision would
   require both m and m' to be INDEPENDENTLY signable with the same challenge
   without a prior honest signature — but the response is the honest signer's
   secret-key-bound object, so one message's challenge is always pinned first.
   The proof_commitment IS in the frame (line 43847) and IS bound to the
   response (line 3168), so the adversary cannot swap in a fresh commitment to
   manufacture a free pair.

   Consequence: second-preimage advantage eps ~ q/2^214 = 2^-150 at q = 2^64
   (LINEAR in q, NO birthday square, NO query cap).  COMFORTABLE — clears the
   2^-133 per-event budget at q = 2^64 with 17 bits to spare, and clears 2^-128
   even at q = 2^80 (2^-134).  *** The deployed 214-bit challenge width is NOT
   changed; no maintainer widening decision is required for #6. ***

   ---------------------------------------------------------------------------
   THE EVENT (checked-core message binding).
   ---------------------------------------------------------------------------
   In the Lean concrete classifier (Path2FullSpendConcrete.ConcreteBreakFor):
       checkedCoreMessageBindingBreak :=
           ¬ CheckedCoreChallengeBindsAcceptedTuple _C spend
   and CheckedCoreChallengeBindsAcceptedTuple requires the accepted signature
   body's checked-core frame EQUALS the canonical frame recomputed from the
   honestly-bound tuple.  A break is an accepted spend whose verified tuple T_v
   differs from the honest tuple T_h (different message) yet yields the SAME
   checked-core challenge as T_h.  Since T_h's challenge is the FIXED honest
   target, T_v is a SECOND preimage of it.

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no `admit`, no `sorry`, closed by EC program logic + Z3; qed):
     * frame_distinct_of_break — a break exhibits a frame DISTINCT from the fixed
         honest frame (injective encoder).  Non-tautology evidence.
     * accept_binds_challenge — an accepted spend's challenge is H of the
         verified frame (the FS verifier recomputes & matches).
     * break_yields_second_preimage — THE REDUCTION CORE: accept /\ binding-break
         against the FIXED honest frame f_h entails f_v <> f_h with H f_v = H f_h,
         i.e. an explicit SECOND-PREIMAGE witness of the fixed target H f_h.
     * R_wins_on_break / break_le_spr — the reduction R, run inside the SPR game
         on the fixed honest target, OUTPUTS that second preimage, so SPR returns
         true; Pr[break] <= Pr[SPR won by R].
     * break_le_bound — compose with the single named SPR axiom: Pr[break] <=
         sha256_second_preimage_bound (~ q/2^214).

   ASSUMED (named hardness): EXACTLY ONE, clearly labelled —
       axiom sha256_second_preimage_resistance :
         Pr[ SPR(R(A)).main(f_h) @ &m : res ] <= sha256_second_preimage_bound.
     SHA-256 (truncated/masked to the 214-bit checked-core width) SECOND-
     preimage-resistance against the FIXED honest target.  Generic bound
     ~q/2^214, NO query cap.
   ============================================================================= *)

require import AllCore Int Real Distr List StdOrder.
import RealOrder.

(* -----------------------------------------------------------------------------
   ABSTRACT TYPES.  As before:
     frame      : challenge_hash_frame output (injective length-prefixed encoding,
                  INCLUDING proof_commitment),
     challenge  : the truncated+masked 214-bit SHA-256 output the verifier matches,
     tuple      : the bound checked-core tuple (message, pk, slot, digest,
                  proof_commitment, state).
   --------------------------------------------------------------------------- *)
type frame.
type challenge.
type tuple.

op encode_frame : tuple -> frame.
axiom encode_frame_injective : injective encode_frame.
(* ^ STRUCTURAL (byte-encoding injectivity), NOT a cryptographic assumption. *)

(* The deployed checked-core hash: SHA-256 truncated+masked to 214 bits. *)
op H : frame -> challenge.

op tuple_challenge (t : tuple) : challenge = H (encode_frame t).

(* =============================================================================
   THE SHA-256 (TRUNCATED, 214-bit) SECOND-PREIMAGE GAME.

   The honest target is FIXED FIRST: the game is initialized with the honest
   frame `f0` (the frame of the honestly-signed tuple T_h, whose challenge H f0
   the honest response is bound to).  The adversary then outputs a candidate
   frame `f'`; it WINS iff `f' <> f0 /\ H f' = H f0` — a SECOND preimage of the
   FIXED target H f0.
   ============================================================================= *)
module type SPRAdv = {
  proc find (f0 : frame) : frame
}.

module SPR (A : SPRAdv) = {
  proc main (f0 : frame) : bool = {
    var f';
    f' <@ A.find(f0);
    return f' <> f0 /\ H f' = H f0;
  }
}.

(* =============================================================================
   THE CHECKED-CORE MESSAGE-BINDING EXPERIMENT (target FIXED FIRST).

   The honest tuple `honest` (its frame and challenge) is fixed by the honest
   signature BEFORE the adversary forges.  A spend TRANSCRIPT carries:
     verified : the tuple the verifier actually checks against,
     tr_chal  : the transcript's claimed checked-core challenge.
   The FS verifier ACCEPTS iff tr_chal = tuple_challenge verified.

   MESSAGE-BINDING BREAK fires when an ACCEPTED transcript verifies against a
   tuple DIFFERING from the fixed honest tuple yet under the honest challenge
   (the response, bound to the honest challenge, transferred) — i.e. the
   verified frame is a SECOND preimage of the honest frame's fixed challenge.
   ============================================================================= *)
type transcript = tuple * challenge.   (* (verified, tr_chal) *)

module type BindAdv = {
  proc forge (honest : tuple) : transcript
}.

(* Acceptance: FS-match against the verified tuple. *)
op fs_accept (tr : transcript) : bool =
  tr.`2 = tuple_challenge tr.`1.

(* The message-binding break against a FIXED honest tuple `h`: accepted, the
   verified tuple differs from the honest one, and the transcript challenge IS
   the honest challenge (the honest response transferred). *)
op binding_break (h : tuple) (tr : transcript) : bool =
     fs_accept tr
  /\ tr.`1 <> h
  /\ tr.`2 = tuple_challenge h.

module Exp (A : BindAdv) = {
  proc run (h : tuple) : bool = {
    var tr;
    tr <@ A.forge(h);
    return binding_break h tr;
  }
}.

(* =============================================================================
   THE REDUCTION MODULE R.

   The SPR game fixes the honest target frame `f0` FIRST.  R recovers the honest
   tuple is NOT needed: R is handed `f0 = encode_frame h` and runs the binding
   adversary on the honest tuple `h`; on a break the verified frame is DISTINCT
   from `f0` and hashes to H f0, so R outputs it as its second preimage.  (We
   model R as taking the honest TUPLE — the EC image of the honest signature
   query — and emitting the verified frame.)
   ============================================================================= *)
module R (A : BindAdv) : SPRAdv = {
  var honest_tuple : tuple
  proc find (f0 : frame) : frame = {
    var tr;
    tr <@ A.forge(honest_tuple);
    return encode_frame tr.`1;
  }
}.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — a break exhibits a frame DISTINCT from the honest one.
   ============================================================================= *)
lemma frame_distinct_of_break (h : tuple) (tr : transcript) :
  binding_break h tr => encode_frame tr.`1 <> encode_frame h.
proof.
  rewrite /binding_break => -[_ [hne _]].
  by apply: contra hne => heq; apply encode_frame_injective.
qed.

(* =============================================================================
   ACCEPTANCE BINDS THE CHALLENGE — an accepted transcript's challenge is H of
   the verified frame.
   ============================================================================= *)
lemma accept_binds_challenge (tr : transcript) :
  fs_accept tr => tr.`2 = H (encode_frame tr.`1).
proof. by rewrite /fs_accept /tuple_challenge. qed.

(* =============================================================================
   THE REDUCTION CORE — break ==> an explicit SHA-256 SECOND preimage of the
   FIXED honest target H(encode_frame h).

   *** PROVEN, no assumption. ***  On a break the verified frame is distinct from
   the honest frame (frame_distinct_of_break) AND hashes to the honest target:
   H(encode_frame verified) = tr_chal = tuple_challenge h = H(encode_frame h).
   ============================================================================= *)
lemma break_yields_second_preimage (h : tuple) (tr : transcript) :
  binding_break h tr =>
     encode_frame tr.`1 <> encode_frame h
  /\ H (encode_frame tr.`1) = H (encode_frame h).
proof.
  move=> hb; split; first by apply (frame_distinct_of_break h).
  move: hb; rewrite /binding_break /fs_accept /tuple_challenge /=.
  by move=> [hacc [_ hh]]; rewrite -hacc hh.
qed.

(* =============================================================================
   THE REDUCTION WINS — running R inside SPR on the fixed honest target makes the
   SPR game return `true` whenever the binding experiment fires the break.

   R is initialized so that R.honest_tuple = h and the SPR target f0 =
   encode_frame h.  Then R forwards A.forge(h) and outputs encode_frame(verified);
   on a break that is distinct from f0 and hashes to H f0 — an SPR win.
   ============================================================================= *)
lemma break_le_spr (A <: BindAdv{-R}) (h : tuple) &m :
  R.honest_tuple{m} = h =>
  Pr[ Exp(A).run(h) @ &m : res ]
    <= Pr[ SPR(R(A)).main(encode_frame h) @ &m : res ].
proof.
  move=> hinit.
  byequiv (_ :   ={glob A, glob R} /\ arg{1} = h /\ arg{2} = encode_frame h
              /\ R.honest_tuple{1} = h
              ==> res{1} => res{2}).
  + proc; inline *.
    wp; call (_ : true).
    auto=> /> *; smt(break_yields_second_preimage).
  + by rewrite hinit.
  + by [].
qed.

(* =============================================================================
   THE NAMED STANDARD HARDNESS ASSUMPTION — and ONLY this.

   SHA-256 (truncated+masked to the 214-bit checked-core width) is SECOND-
   preimage-resistant: against the FIXED honest target frame, no efficient
   adversary wins the SPR game with more than `sha256_second_preimage_bound`
   probability (generic bound ~q/2^214, LINEAR in q, NO query cap).  This is the
   single standard hash hardness assumption charged to
   `checked_core_message_binding_break`.

   *** THIS IS THE ONLY AXIOM CARRYING A BOUND IN THIS FILE. ***
   ============================================================================= *)
op sha256_second_preimage_bound : real.

axiom sha256_second_preimage_resistance (A <: BindAdv{-R}) (f0 : frame) &m :
  Pr[ SPR(R(A)).main(f0) @ &m : res ] <= sha256_second_preimage_bound.

(* =============================================================================
   THE BOUND — Pr[ checked-core message-binding break fires ] <= SPR bound.
   The reduction (PROVEN) composed with the single named SPR assumption.  This
   is the union-bound plug-in `breakBound` term for this event.
   ============================================================================= *)
lemma break_le_bound (A <: BindAdv{-R}) (h : tuple) &m :
  R.honest_tuple{m} = h =>
  Pr[ Exp(A).run(h) @ &m : res ] <= sha256_second_preimage_bound.
proof.
  move=> hinit.
  apply (ler_trans (Pr[ SPR(R(A)).main(encode_frame h) @ &m : res ])).
  + by apply (break_le_spr A h &m).
  + by apply (sha256_second_preimage_resistance A (encode_frame h) &m).
qed.

(* =============================================================================
   PLUG-IN COROLLARY — for any union-bound budget at least the SPR bound, this
   break's term is discharged.
   ============================================================================= *)
lemma break_le_budget (A <: BindAdv{-R}) (h : tuple) (budget : real) &m :
  R.honest_tuple{m} = h =>
  sha256_second_preimage_bound <= budget =>
  Pr[ Exp(A).run(h) @ &m : res ] <= budget.
proof.
  move=> hinit hb; apply (ler_trans sha256_second_preimage_bound).
  + by apply (break_le_bound A h &m).
  + exact hb.
qed.

(* =============================================================================
   END.   checkedCoreMessageBinding: REAL GAME-BASED REDUCTION (family (b)),
   RE-MODELLED as SECOND-PREIMAGE on the FIXED 214-bit honest target (GAP-C
   closure).  PROVEN (EC program logic + Z3, exit 0):
     accept /\ message-binding-break  ==>  R(A) WINS the SHA-256 SECOND-preimage
     game against the FIXED honest frame.
   ASSUMED: EXACTLY ONE named standard hardness assumption —
     sha256_second_preimage_resistance (~q/2^214, NO query cap).
   #6 VERDICT: SECOND-PREIMAGE-FINE.  2^-150 at q=2^64.  The deployed 214-bit
   challenge width is NOT changed; no maintainer widening decision required.
   No admit, no sorry, no other axiom carrying a bound (encode_frame_injective is
   byte-encoding structure, not hardness).
   ============================================================================= *)
