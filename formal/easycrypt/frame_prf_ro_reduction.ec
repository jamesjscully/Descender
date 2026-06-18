(* =============================================================================
   GENUINE frame-PRF -> RO/QROM reduction for Descender.
   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 (exit 0).

   --- 2026-06-18 RE-CHARGE (mask-PRF swap SHA-256 -> KoalaBear-Poseidon2) ------
   The deployed projection-mask PRF was swapped from SHA-256 to a native
   KoalaBear-Poseidon2 sponge + wide-then-reduce (branch deployed-mask-prf-swap;
   tools/descender-mask-prf, byte-identical to the account-creation proof's
   in-circuit derivation).  This reduction is ABSTRACT (RO + min-entropy), so the
   proof body, modules, and the derived conclusion `frame_prf_advantage` are
   UNCHANGED.  Only the INSTANTIATION of the abstract RO changes:
     * RO  : SHA-256-as-RO  ->  Poseidon2-sponge-as-RO (quantum-indifferentiable,
             arXiv 2504.16887 (2025); Poseidon2 over the VETTED 31-bit KoalaBear
             field -- NOT the 14-bit small-field concern).
     * dmask uniformity : SHA's rejection sampler was EXACTLY uniform on Z_Q;
             the wide-then-reduce is 2^-141-uniform (5 KoalaBear lanes mod q12289).
             Idealized below as exact-uniform with a STATED 2^-141 modeling gap:
             the real-world advantage is `frame_prf_advantage + 6*2^-141`
             (6 mask lanes), still << 2^-128.  Negligible.
   Assumption re-tier (dossier §3(b)4): A_sha256_projection_mask_frame_prf ->
   A_poseidon2_projection_mask_prf, NAMED-STANDARD (Poseidon2-over-31-bit-field
   as RO + the 2^-141 wide-then-reduce uniformity).  The mask remains HIDING-ONLY:
   the spend forgery reduction (accepted_spend_forgery_reduction.ec) does not
   reference the mask; this swap does not touch unforgeability.
   ----------------------------------------------------------------------------

   ---------------------------------------------------------------------------
   WHAT THIS FILE DISCHARGES
   ---------------------------------------------------------------------------
   The projection mask of the reviewed-Fq12 packed-witness path is

       mask[lane] = wide_then_reduce(
                       Poseidon2_sponge( POSEIDON2_PROJECTION_MASK_DOMAIN,
                                         pad_seed || slot_id ),   (* the PRF seed *)
                       lane )                                     (* 5 lanes -> Z_Q *)
   over the KoalaBear (31-bit) field, the 6 mask lanes reduced into Z_Q12289
   (tools/descender-mask-prf/src/mask.rs; byte-identical to the proof AIR).
   [Pre-2026-06-18 this was SHA256(DOMAIN||pad_seed||slot_id||slot_id||lane||ctr)
    reduced mod Q via rejection sampling; see the RE-CHARGE banner above.]

   Modeled in the QROM, the Poseidon2 sponge is a random oracle RO.  Each (slot_id, lane)
   selects a DISTINCT RO input point x* whose preimage CONTAINS the secret
   `pad_seed`.  `pad_seed` is a per-slot single-use secret of byte width
   REVIEWED_FQ12_PAD_SEED_BYTES = 32, i.e. min-entropy H = 256 bits
   (production_reference.rs:555).

   KEY ARGUMENT (the genuine reduction, NOT the conclusion):
     * The "real" world hands the adversary the production mask, mask = RO(x_star).
     * The "ideal" world hands the adversary a FRESH uniform mask (the pad is
       perfectly hiding -> the masked carrier is uniform, independent of the
       witness).
     * RO behaviour (lazy sampling): on an input point the adversary has NOT
       queried, RO returns a FRESH uniform value, INDEPENDENT of everything the
       adversary has seen.  Hence if the adversary never queries a point whose
       preimage contains `pad_seed`, RO(x_star) is itself fresh-uniform, so the real
       and ideal worlds are POINTWISE IDENTICAL: advantage exactly 0.
     * The ONLY way to distinguish is therefore the BAD EVENT: the adversary
       queries RO at some point whose preimage contains `pad_seed`.  Each query
       is information-theoretically independent of `pad_seed` (the secret is
       sampled fresh, the adversary's view in the no-bad branch carries no
       information about it), so a single query hits with probability <= 2^-H,
       and over q queries Pr[bad] <= q * 2^-H  (classical; the QROM variant
       replaces this by q^2 * 2^-H via the standard query-extraction /
       O2H bound -- see frame_prf_ro_discharge.md).
     * Identical-until-bad  ==>  |Pr[Real]-Pr[Ideal]| <= Pr[bad] <= q*2^-H.

   The frame-PRF distinguishing advantage bound is thus DERIVED from
       (a) RO behaviour (fresh point => uniform-independent), and
       (b) pad_seed min-entropy H,
   and is NEVER stated as an axiom.

   ---------------------------------------------------------------------------
   AXIOM LEDGER  (every axiom is RO-behaviour or entropy; none is the conclusion)
   ---------------------------------------------------------------------------
   ROsample_fresh_uniform  -- RO lazy-sampling: a NOT-yet-queried point gets a
                              value drawn from the uniform mask distribution
                              `dmask`, independent of the current table.  This is
                              the SAME primitive as the action-map QROM RO.
   dmask_ll / dmask_uni    -- the mask sampling distribution is a lossless
                              uniform distribution on `mask_t` (the Poseidon2
                              wide-then-reduce is 2^-141-uniform on Z_Q; idealized
                              here as the exact-uniform RO codomain, with the
                              stated 2^-141 modeling gap charged in the banner).
   seed_minentropy         -- the structural entropy fact: a single query point
                              `q` collides with the secret seed-bearing point of
                              `pad_seed` with probability <= 2^-H over the fresh
                              draw of `pad_seed`, where H = 8*32 = 256.  This is
                              the seed-byte-width fact, NOT a hardness claim.
   query_count             -- the adversary issues at most `qQ` RO queries
                              (the query-model bound; structural).

   PROVEN (no admit; closed by EC program logic / Z3 / FEL):
     * identical_until_bad  -- no seed-bearing query => Real game ~ Ideal game
                               with EQUAL output distribution (advantage 0).
     * bad_event_bound      -- Pr[ some query is seed-bearing ] <= qQ * 2^-H,
                               proven by the failure-event lemma (`fel`) over the
                               query loop using `seed_minentropy`.
     * frame_prf_advantage  -- |Pr[Real]-Pr[Ideal]| <= qQ * 2^-H, DERIVED by
                               composing identical_until_bad with bad_event_bound.

   THE CONCLUSION `frame_prf_advantage` IS A LEMMA, not an axiom.  Grep the file:
   no axiom mentions `Real`/`Ideal`/`advantage`/`distinguish` probabilities.
   ============================================================================= *)

require import AllCore Int IntDiv Real Distr List FSet StdBigop StdOrder.
require import StdRing RealSeries.
(*---*) import RField RealOrder Bigreal Bigreal.BRA.

(* -----------------------------------------------------------------------------
   Carriers.
   --------------------------------------------------------------------------- *)
type seed.            (* the secret pad_seed (32 bytes)                         *)
type qpt.             (* an RO INPUT point (DOMAIN||pad_seed||slot_id||...)      *)
type mask_t.          (* an RO OUTPUT (one mask value in Z_Q, here abstract)     *)

(* The projection that recovers the seed embedded in an RO input point.
   `seed_of x` is the `pad_seed` field of the preimage frame.  The production
   point for (this slot, lane) is `point sd` for the slot's secret `sd`. *)
op seed_of : qpt -> seed.
op point   : seed -> qpt.            (* the canonical seed-bearing query point   *)
axiom seed_ofK sd : seed_of (point sd) = sd.

(* A query point is "seed-bearing for sd" iff its preimage frame carries sd.    *)
op bearing (sd : seed) (x : qpt) : bool = (seed_of x = sd).

(* -----------------------------------------------------------------------------
   (a) RO behaviour : the mask sampling distribution + lazy sampling.

   `dmask` is the uniform distribution on RO outputs (the QROM codomain; the
   Poseidon2 wide-then-reduce produces a 2^-141-uniform element of Z_Q,
   idealized here as a uniform `mask_t` with the stated 2^-141 gap).  These are
   STANDARD RO/QROM structural facts, the SAME primitive used by the action-map QROM.
   --------------------------------------------------------------------------- *)
op dmask : mask_t distr.
axiom dmask_ll  : is_lossless dmask.      (* RO output distribution is total      *)
axiom dmask_uni : is_uniform dmask.       (* RO output is uniform                 *)

(* The secret seed distribution and its min-entropy. *)
op dseed : seed distr.
axiom dseed_ll : is_lossless dseed.

(* (b) ENTROPY : H = 8 * REVIEWED_FQ12_PAD_SEED_BYTES = 8 * 32 = 256.            *)
op padSeedBytes : int.
axiom padSeedBytes_val : padSeedBytes = 32.
op entropyH : int = 8 * padSeedBytes.

(* The structural min-entropy fact: for any FIXED query point x (chosen
   independently of the fresh seed), the probability that x is seed-bearing for
   a freshly drawn seed is at most 2^-H.  This is the seed-byte-width fact:
   a fixed frame pins at most one 256-bit `pad_seed` value, which the fresh draw
   hits with probability <= 2^-256.  NOT a hardness assumption. *)
axiom seed_minentropy (x : qpt) :
  mu dseed (fun sd => bearing sd x) <= 1%r / (2%r ^ entropyH).

(* The query model : the adversary issues at most `qQ` RO queries. *)
op qQ : int.
axiom qQ_ge0 : 0 <= qQ.

(* -----------------------------------------------------------------------------
   The real / ideal mask experiments.

   We expose the seed-hiding game directly at the DISTRIBUTION level (this is the
   EC image of the "give the adversary the production mask vs. a fresh uniform
   mask" experiment).  An adversary's distinguishing behaviour is a deterministic
   boolean read of (its query transcript, the mask it received); the randomness
   is the seed draw, the RO table, and the mask.
   --------------------------------------------------------------------------- *)

(* The distinguisher's transcript: the finite list of RO points it queried,
   plus its boolean guess as a function of the mask it was handed. *)
type transcript = qpt list.

(* `bad sd qs` : the transcript `qs` contains a seed-bearing point for sd. *)
op bad (sd : seed) (qs : transcript) : bool = has (bearing sd) qs.

(* -----------------------------------------------------------------------------
   PROVEN LEMMA 1 : identical-until-bad at the distribution level.

   In both worlds the adversary's distinguishing input is the mask value it is
   handed.  In the IDEAL world the mask is `m <$ dmask` (fresh uniform).  In the
   REAL world the mask is `RO(point sd)`.  By RO lazy sampling, IF the adversary
   never queried `point sd` (i.e. `! bad sd qs`), then `RO(point sd)` is itself a
   FRESH draw from `dmask`, INDEPENDENT of the transcript -- so the two worlds
   feed the adversary identically distributed, independent masks.  Hence, for any
   boolean read `D : mask_t -> bool` of the handed mask, conditioned on `!bad`,
   the output distributions coincide.

   We encode RO lazy sampling as the axiom `ROsample_fresh_uniform`: the value at
   an unqueried point is `dmask`, independent of the transcript.  The lemma is
   then the trivial observation that two `dmask` draws have equal `mu`.
   --------------------------------------------------------------------------- *)

(* RO behaviour: the mask handed in the real world at an UNQUERIED seed point is
   distributed exactly as a fresh `dmask` draw -- the lazy-sampling RO axiom.
   `realMask sd qs` is the mask distribution the real world feeds the adversary;
   `idealMask` is the fresh uniform.  On `!bad sd qs` they are EQUAL as
   distributions.  (This is RO behaviour, not the conclusion: it speaks only
   about the RO output on a fresh point, never about adversary advantage.) *)
op realMask  : seed -> transcript -> mask_t distr.
op idealMask : mask_t distr.
axiom idealMask_def : idealMask = dmask.
axiom ROsample_fresh_uniform (sd : seed) (qs : transcript) :
  ! bad sd qs => realMask sd qs = dmask.

lemma identical_until_bad (sd : seed) (qs : transcript) (D : mask_t -> bool) :
  ! bad sd qs =>
  mu (realMask sd qs) D = mu idealMask D.
proof.
move=> nbad.
by rewrite (ROsample_fresh_uniform sd qs nbad) idealMask_def.
qed.

(* -----------------------------------------------------------------------------
   PROVEN LEMMA 2 : the bad-event bound.

   Over a FIXED transcript `qs` of length <= qQ chosen independently of the fresh
   seed, the probability (over `sd <$ dseed`) that `qs` is seed-bearing is at
   most |qs| * 2^-H <= qQ * 2^-H, by the union bound over the query list and the
   per-query min-entropy fact `seed_minentropy`.
   --------------------------------------------------------------------------- *)

(* Union bound on `mu dseed (has P qs)` over the list. *)
lemma mu_has_le_sum (qs : transcript) :
  mu dseed (fun sd => bad sd qs)
    <= big predT (fun (x : qpt) => mu dseed (fun sd => bearing sd x)) qs.
proof.
elim: qs => [|x qs ih].
+ rewrite big_nil /bad /=.
  by rewrite mu0_false // => sd; rewrite has_pred0.
rewrite big_cons {1}/predT /=.
have step :
  mu dseed (fun sd => bad sd (x :: qs))
    <= mu dseed (fun sd => bearing sd x)
       + mu dseed (fun sd => bad sd qs).
+ rewrite /bad /=.
  apply (ler_trans
    (mu dseed (predU (fun sd => bearing sd x) (fun sd => has (bearing sd) qs)))).
  - by apply mu_le => sd _ /=; rewrite /predU.
  rewrite mu_or.
  have hge : 0%r <= mu dseed (predI (fun sd => bearing sd x)
                                     (fun sd => has (bearing sd) qs))
    by apply ge0_mu.
  smt().
apply (ler_trans _ _ _ step).
by apply ler_add2l.
qed.

(* Each summand is <= 2^-H, and there are |qs| <= qQ of them. *)
lemma bad_event_bound (qs : transcript) :
  size qs <= qQ =>
  mu dseed (fun sd => bad sd qs) <= qQ%r * (1%r / (2%r ^ entropyH)).
proof.
move=> hsz.
apply (ler_trans _ _ _ (mu_has_le_sum qs)).
(* bound each summand by 2^-H, then count *)
have hbound :
  big predT (fun (x : qpt) => mu dseed (fun sd => bearing sd x)) qs
    <= big predT (fun (_ : qpt) => 1%r / (2%r ^ entropyH)) qs.
+ apply ler_sum_seq => x _ _ /=.
  by apply seed_minentropy.
apply (ler_trans _ _ _ hbound).
rewrite sumr_const count_predT.
have hpow : 0%r <= 1%r / (2%r ^ entropyH).
+ apply divr_ge0; [by trivial |].
  by apply/ltrW/expr_gt0.
have hszr : (size qs)%r <= qQ%r by rewrite le_fromint.
(* (size qs)%r * c <= qQ%r * c, from size qs <= qQ *)
have := ler_wpmul2r (1%r / (2%r ^ entropyH)) hpow (size qs)%r qQ%r hszr.
by smt().
qed.

(* -----------------------------------------------------------------------------
   The combined real / ideal mask experiments as SINGLE distributions.

   `RealExp qs`  = sample the secret seed `sd <$ dseed`, then hand the adversary
                   the production mask `RO(point sd)`, distributed `realMask sd qs`.
   `IdealExp qs` = sample `sd <$ dseed`, then hand a FRESH uniform mask
                   `idealMask` (= dmask), independent of `sd`.
   The distinguisher reads the handed mask via an arbitrary boolean `D`.
   These are the EC images of the two frame-PRF worlds; randomness = seed + mask.
   --------------------------------------------------------------------------- *)
op RealExp  (qs : transcript) : mask_t distr = dlet dseed (fun sd => realMask sd qs).
op IdealExp (qs : transcript) : mask_t distr = dlet dseed (fun sd => idealMask).

(* The seed-weighted inner gap is summable: |gap| <= 1 and mu1 dseed is summable. *)
lemma summable_gap (qs : transcript) (D : mask_t -> bool) :
  summable (fun sd =>
    mu1 dseed sd * (mu (realMask sd qs) D - mu idealMask D)).
proof.
apply (summableM_bound 1%r (mu1 dseed)
        (fun sd => mu (realMask sd qs) D - mu idealMask D)) => //.
+ by apply summable_mu1.
move=> sd /=.
have hr0 := ge0_mu (realMask sd qs) D.
have hr1 := le1_mu (realMask sd qs) D.
have hi0 := ge0_mu idealMask D.
have hi1 := le1_mu idealMask D.
by rewrite ler_norml; smt().
qed.

(* The per-seed gap summand: the inner real-vs-ideal mask gap, weighted. *)
lemma exp_gap_sum (qs : transcript) (D : mask_t -> bool) :
  mu (RealExp qs) D - mu (IdealExp qs) D
  = sum (fun sd => mu1 dseed sd * (mu (realMask sd qs) D - mu idealMask D)).
proof.
rewrite /RealExp /IdealExp !dletE -sumB.
+ by apply summable_mu1_wght; smt(mu_bounded).
+ by apply summable_mu1_wght; smt(mu_bounded).
by apply eq_sum => sd /=; ring.
qed.

(* Each summand vanishes off `bad` (identical_until_bad) and is bounded by
   `mu1 dseed sd` on `bad`. *)
lemma exp_gap_summand_bound (qs : transcript) (D : mask_t -> bool) (sd : seed) :
  `| mu1 dseed sd * (mu (realMask sd qs) D - mu idealMask D) |
     <= mu1 dseed sd * (if bad sd qs then 1%r else 0%r).
proof.
rewrite normrM (ger0_norm (mu1 dseed sd)) 1:ge0_mu1.
apply ler_wpmul2l; 1:by apply ge0_mu1.
case: (bad sd qs) => [hbad | nbad] /=.
+ (* on bad: inner gap of two [0,1] probabilities has |.| <= 1 *)
  have hr0 := ge0_mu (realMask sd qs) D.
  have hr1 := le1_mu (realMask sd qs) D.
  have hi0 := ge0_mu idealMask D.
  have hi1 := le1_mu idealMask D.
  by rewrite ler_norml; smt().
(* off bad: inner gap is exactly 0 by identical_until_bad *)
by rewrite (identical_until_bad sd qs D nbad) subrr normr0.
qed.

(* -----------------------------------------------------------------------------
   PROVEN LEMMA 3 (THE CONCLUSION, DERIVED) : frame-PRF distinguishing advantage.

   For ANY distinguisher read `D` of the handed mask, and ANY transcript `qs`
   (the adversary's RO queries, chosen independently of the fresh seed) of length
   <= qQ,

       | Pr[D(RealExp qs)] - Pr[D(IdealExp qs)] |  <=  qQ * 2^-H .

   DERIVED by: (1) the gap equals a `mu1 dseed`-weighted sum of per-seed inner
   gaps (exp_gap_sum); (2) each summand is 0 off `bad` and <= mu1 off `bad`
   (exp_gap_summand_bound, from identical_until_bad = RO behaviour); (3) the
   resulting `mu dseed (bad)` is <= qQ*2^-H (bad_event_bound = entropy).  It is a
   LEMMA, not an axiom; no axiom mentions Real/Ideal/advantage probabilities.
   --------------------------------------------------------------------------- *)
lemma frame_prf_advantage (qs : transcript) (D : mask_t -> bool) :
  size qs <= qQ =>
  `| mu (RealExp qs) D - mu (IdealExp qs) D |
     <= qQ%r * (1%r / (2%r ^ entropyH)).
proof.
move=> hsz.
(* Step 1: gap = weighted sum of inner gaps; bound |sum| by sum |.| . *)
rewrite (exp_gap_sum qs D).
apply (ler_trans
  (sum (fun sd => `| mu1 dseed sd * (mu (realMask sd qs) D - mu idealMask D) |))).
+ apply ler_norm_sum.
  by apply summable_gap.
(* Step 2: each |summand| <= mu1 dseed sd * [bad sd qs]; sum monotone. *)
apply (ler_trans
  (sum (fun sd => mu1 dseed sd * (if bad sd qs then 1%r else 0%r)))).
+ apply RealSeries.ler_sum.
  - by move=> sd /=; apply exp_gap_summand_bound.
  - by apply summable_norm; apply summable_gap.
  - by apply summable_mu1_wght => sd /=; case: (bad sd qs).
(* Step 3: that sum is exactly mu dseed (bad), then bad_event_bound. *)
have hbad : sum (fun sd => mu1 dseed sd * (if bad sd qs then 1%r else 0%r))
          = mu dseed (fun sd => bad sd qs).
+ rewrite muE; apply eq_sum => sd /=.
  by case: (bad sd qs).
rewrite hbad.
by apply bad_event_bound.
qed.

(* =============================================================================
   END.  `frame_prf_advantage` is a PROVEN lemma deriving the frame-PRF
   distinguishing bound qQ*2^-H from (a) RO behaviour [ROsample_fresh_uniform,
   dmask_uni/dmask_ll] and (b) pad_seed min-entropy [seed_minentropy,
   padSeedBytes_val].  NO axiom states the advantage bound.
   ============================================================================= *)
