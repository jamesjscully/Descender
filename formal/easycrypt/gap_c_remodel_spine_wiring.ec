(* =============================================================================
   gap_c_remodel_spine_wiring.ec
   Path-2 Descender — GAP-C closure: wire the CONCRETE second-preimage epsilon
   for each re-modelled binding event into the union-bound spine arithmetic and
   PROVE the budget closes at q = 2^64 WITHOUT any classical query cap.

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHAT THIS FILE DISCHARGES
   ---------------------------------------------------------------------------
   The three live cryptographic binding events (#2 leaf, #3 registry, #6
   checked-core) were RE-MODELLED in
     break_prevalidation_leaf_binding.ec        (SPR, n = 256)
     break_prevalidation_registry_binding.ec    (SPR, n = 256)
     break_checked_core_message_binding.ec      (MULTI-TARGET SPR, n = 246)
   from FREE birthday collisions (eps ~ q^2/2^n) to SECOND-PREIMAGE games against
   an honestly-FIXED target (eps ~ q/2^n).  The operational justification (the
   verifier recomputes the digest/challenge against a LEAF-FIXED committed value
   before the adversary acts) is recorded in each file's header and in
   research/cryptanalysis/gap_c_remodel_result.md.

   *** HIGH-3 (2026-06-10): the binding events are MULTI-TARGET over the T = 2^32
   signing queries, so the OPERATIVE term is tQ * eps_spr(n) = T*q/2^n, NOT the
   single-target q/2^n.  The checked-core width was WIDENED 214 -> 246 so this
   multi-target term clears the budget. ***

   Here we wire the concrete numeric MULTI-TARGET bound  tQ * eps_spr(n) =
   T*q/2^n  at q = 2^64, T = 2^32, and PROVE:
     (1) each re-modelled MULTI-TARGET break term <= 2^-133  (NO cap beyond T);
     (2) the SUM of the re-modelled break terms + the structural zeros +
         the accepted-spend placeholder (2^-129) <= 2^-128.

   All real-arithmetic facts are proven by EC + Z3 with explicit exponent
   expansions (Z3 cannot expand symbolic powers).  No admit, no sorry.
   ============================================================================= *)

require import AllCore Int Real RealExp StdOrder.
import RealOrder.

(* -----------------------------------------------------------------------------
   The classical query model: q = 2^64.  This is the GENERIC adversary model,
   NOT the deployment query cap (2^33).  The re-model closes WITHOUT relying on
   the cap; we pick q = 2^64 (and note q = 2^80 still closes for #6).
   --------------------------------------------------------------------------- *)
op q : real = 2%r ^ 64.

(* The CONCRETE second-preimage advantage at digest width n bits: eps = q/2^n.
   LINEAR in q (NO birthday square), the generic SPR bound for a random-oracle-
   modelled / ideal hash; this is the named bound the re-modelled axioms
   `sha256_second_preimage_resistance` carry. *)
op eps_spr (n : int) : real = q / (2%r ^ n).

(* -----------------------------------------------------------------------------
   MULTI-TARGET HORIZON (HIGH-3).  T = 2^32 honest signing queries.  The checked-
   core / leaf / registry binding events are MULTI-TARGET second preimages: over T
   signing queries the adversary obtains T honest targets and a break against ANY
   of them suffices, so the OPERATIVE advantage is the multi-target term
   T * eps_spr(n) (the T-fold union of the per-target single-target reductions —
   see break_checked_core_message_binding.ec `multi_target_break_le_bound`), NOT
   the single-target eps_spr(n).  This T-multiplied term is what we charge below.
   --------------------------------------------------------------------------- *)
op tQ : real = 2%r ^ 32.   (* T = 2^32 signing-query horizon (maintainer cap) *)

(* The concrete per-event MULTI-TARGET epsilons at the deployed digest widths.
   #6 checked-core was WIDENED 214 -> 246 bits (HIGH-3) precisely so that the
   multi-target term tQ * eps_spr 246 = 2^32 * 2^64 / 2^246 = 2^-150 stays under
   the 2^-133 per-event budget; at the old n=214 it was 2^-118 (a breach). *)
op eps_leaf     : real = tQ * eps_spr 256.   (* #2 leaf binding   (multi-target, n=256) *)
op eps_registry : real = tQ * eps_spr 256.   (* #3 registry       (multi-target, n=256) *)
op eps_checked  : real = tQ * eps_spr 246.   (* #6 checked-core   (multi-target, n=246) *)

(* Accepted-spend term (AP2-N003).  No longer a bare placeholder: it is now
   DISCHARGED by the reduction in accepted_spend_forgery_reduction.ec
   (`accepted_spend_term_le_2pow_neg129`: the same-action fresh-message accepted
   forgery is <= 2^-129), modulo the single EXTERNAL coded-SIS estimator axiom
   (open_external_estimator) + the negligible QROM action-collision term.  The
   2^-129 value below is the bound that reduction establishes. *)
op eps_accepted : real = 1%r / (2%r ^ 129).

(* =============================================================================
   HELPER — eps_spr n = q / 2^n is positive, and monotone-decreasing in n.
   ============================================================================= *)
lemma pow2_gt0 (n : int) : 0 <= n => 0%r < 2%r ^ n.
proof. move=> _; exact RealOrder.expr_gt0. qed.

lemma eps_spr_ge0 (n : int) : 0 <= n => 0%r <= eps_spr n.
proof.
  move=> hn; rewrite /eps_spr /q.
  by apply divr_ge0; [apply ltrW; apply RealOrder.expr_gt0 | apply ltrW; apply (pow2_gt0 n hn)].
qed.

(* =============================================================================
   GOAL (1) — each re-modelled MULTI-TARGET break term <= 2^-133 at q = 2^64,
   T = 2^32, NO query cap beyond T.

   The OPERATIVE term is the MULTI-TARGET advantage tQ * eps_spr n = 2^32 * 2^64
   / 2^n = 2^96 / 2^n (HIGH-3).
   #2/#3 (n=256):  2^96/2^256 = 2^-160 <= 2^-133.
   #6   (n=246):  2^96/2^246 = 2^-150 <= 2^-133.
   We prove the dominant (largest) one, eps_checked (n=246) <= 2^-133, and the
   n=256 ones a fortiori (they are strictly smaller).
   ============================================================================= *)

(* General MULTI-TARGET arithmetic lemma: for any width n with n - 133 >= 0 and
   2^(n-133) >= 2^96 = tQ * q (the multi-target numerator), the multi-target term
   tQ * eps_spr n = 2^96/2^n <= 2^-133.  We instantiate it at n = 246 (n-133=113)
   and n = 256 (n-133=123).  The proof feeds Z3 the explicit factorization
   2^n = 2^133 * 2^(n-133) and 2^(n-133) >= 2^96 = tQ*q, so
   2^96/2^n = (2^96/2^(n-133))/2^133 <= 1/2^133. *)
lemma multi_target_le_2pow_neg133 (n : int) :
  2%r ^ 96 <= 2%r ^ (n - 133) =>
  0 <= n - 133 =>
  tQ * eps_spr n <= 1%r / (2%r ^ 133).
proof.
  move=> hk hk0.
  rewrite /eps_spr /q /tQ.
  have hq96 : 2%r ^ 32 * 2%r ^ 64 = 2%r ^ 96.
  + by rewrite -RField.exprD_nneg.
  have hn : 2%r ^ n = 2%r ^ 133 * 2%r ^ (n - 133).
  + rewrite -RField.exprD_nneg 1:// 1:/#; congr; smt().
  have p133 : 0%r < 2%r ^ 133  by apply RealOrder.expr_gt0.
  have pk   : 0%r < 2%r ^ (n - 133) by apply RealOrder.expr_gt0.
  (* 2^32 * (2^64 / 2^n) = 2^96 / 2^n *)
  have -> : 2%r ^ 32 * (2%r ^ 64 / 2%r ^ n) = 2%r ^ 96 / 2%r ^ n.
  + rewrite -hq96; field; smt(RealOrder.expr_gt0).
  rewrite hn.
  (* 2^96 / (2^133 * 2^(n-133)) <= 1/2^133 *)
  have -> : 2%r ^ 96 / (2%r ^ 133 * 2%r ^ (n - 133))
          = (2%r ^ 96 / 2%r ^ (n - 133)) / 2%r ^ 133.
  + by field; smt().
  apply ler_wpmul2r; first by smt().
  by apply ler_pdivr_mulr; smt().
qed.

(* Power monotonicity: 0 <= m <= n => 2^m <= 2^n.  (RealOrder.ler_weexpn2l.) *)
lemma le_2pow (m n : int) : 0 <= m => m <= n => 2%r ^ m <= 2%r ^ n.
proof. move=> h1 h2; apply ler_weexpn2l => //. qed.

(* 2^96 <= 2^113 (= 2^(246-133)) and 2^96 <= 2^123 (= 2^(256-133)). *)
lemma le_2pow_96_113 : 2%r ^ 96 <= 2%r ^ 113 by apply le_2pow.
lemma le_2pow_96_123 : 2%r ^ 96 <= 2%r ^ 123 by apply le_2pow.

lemma eps_checked_le_2pow_neg133 : eps_checked <= 1%r / (2%r ^ 133).
proof.
  rewrite /eps_checked.
  apply (multi_target_le_2pow_neg133 246) => //.
  by rewrite (_ : 246 - 133 = 113) 1:// le_2pow_96_113.
qed.

lemma eps_leaf_le_2pow_neg133 : eps_leaf <= 1%r / (2%r ^ 133).
proof.
  rewrite /eps_leaf.
  apply (multi_target_le_2pow_neg133 256) => //.
  by rewrite (_ : 256 - 133 = 123) 1:// le_2pow_96_123.
qed.

lemma eps_registry_le_2pow_neg133 : eps_registry <= 1%r / (2%r ^ 133).
proof. rewrite /eps_registry -/eps_leaf; exact eps_leaf_le_2pow_neg133. qed.

(* =============================================================================
   GOAL (2) — the summed budget closes at q = 2^64 WITHOUT a query cap.

   Active terms (everything else is a structural 0; MULTI-TARGET, T = 2^32):
     accepted-spend     eps_accepted   = 2^-129
     #2 leaf            eps_leaf       = tQ*q/2^256 = 2^-160
     #3 registry        eps_registry   = tQ*q/2^256 = 2^-160
     #6 checked-core    eps_checked    = tQ*q/2^246 = 2^-150
   plus the 5 structural-zero / unmodelled-as-0 events (= 0).

   Claim:  eps_accepted + eps_leaf + eps_registry + eps_checked <= 2^-128.

   Bound each break by 2^-133 (Goal 1); then
     2^-129 + 3 * 2^-133 = 2^-129 + 3/16 * 2^-129 = (19/16) 2^-129
       ... but more cleanly  2^-129 + 3*2^-133 <= 2^-129 + 2^-129 = 2^-128
   since 3*2^-133 = (3/16) 2^-129 <= 2^-129.  Z3 closes with the exponent
   expansions.
   ============================================================================= *)
lemma remodelled_budget_closes :
  eps_accepted + eps_leaf + eps_registry + eps_checked <= 1%r / (2%r ^ 128).
proof.
  have hL := eps_leaf_le_2pow_neg133.
  have hR := eps_registry_le_2pow_neg133.
  have hC := eps_checked_le_2pow_neg133.
  have hA : eps_accepted = 1%r / (2%r ^ 129) by rewrite /eps_accepted.
  (* sum <= 2^-129 + 3*2^-133 <= 2^-128 *)
  have e129 : 2%r ^ 129 = 2%r ^ 128 * 2%r.
  + by rewrite (_ : 129 = 128 + 1) 1:// RField.exprD_nneg // RField.expr1.
  have e133 : 2%r ^ 133 = 2%r ^ 128 * 32%r.
  + rewrite (_ : 133 = 128 + 5) 1:// RField.exprD_nneg //.
    by rewrite (_ : 5 = 1+1+1+1+1) 1:// !RField.exprD_nneg // !RField.expr1.
  have pos128 : 0%r < 2%r ^ 128 by apply RealOrder.expr_gt0.
  move: hL hR hC hA e129 e133 pos128; smt().
qed.

(* -----------------------------------------------------------------------------
   COROLLARY — restate against the spike spine's per-event hypothesis shape:
   every re-modelled break term <= 2^-133 and the accepted-spend term <= 2^-129,
   which is EXACTLY the hypothesis of spike_union_bound.success_le_2pow_neg128.
   So feeding these CONCRETE second-preimage bounds (eps_spr at q=2^64, NO cap)
   into the spine yields successProb <= 2^-128.
   --------------------------------------------------------------------------- *)
lemma remodelled_per_event_meets_spine_hyp :
     eps_accepted <= 1%r / (2%r ^ 129)
  /\ eps_leaf     <= 1%r / (2%r ^ 133)
  /\ eps_registry <= 1%r / (2%r ^ 133)
  /\ eps_checked  <= 1%r / (2%r ^ 133).
proof.
  split; [ by rewrite /eps_accepted; smt() | ].
  split; [ exact eps_leaf_le_2pow_neg133 | ].
  split; [ exact eps_registry_le_2pow_neg133 | ].
  exact eps_checked_le_2pow_neg133.
qed.

(* =============================================================================
   END.   GAP-C re-model spine wiring (HIGH-3 multi-target).
   PROVEN (EC + Z3, exit 0): at q = 2^64, T = 2^32 (no cap beyond T),
     * each re-modelled MULTI-TARGET binding break term
       (tQ * eps_spr at n=256 / n=246) <= 2^-133;
     * the sum  accepted-spend(2^-129) + leaf(2^-160) + registry(2^-160)
       + checked-core(2^-150) + structural-zeros  <= 2^-128.
   This discharges GAP C / HIGH-3: the binding events are charged at the CORRECT
   multi-target rate T*q/2^n.  The checked-core width was WIDENED 214 -> 246
   precisely so the dominant multi-target term tQ*q/2^246 = 2^-150 (was 2^-118 at
   n=214 — a 2^-128 breach) stays comfortably under the 2^-133 per-event budget.
   ============================================================================= *)
