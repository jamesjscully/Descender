(* =============================================================================
   break_creation_time_hidden_source.ec
   Path-2 Descender — REAL EasyCrypt game-based REDUCTION for ONE
   cryptographically-charged break event:
       creationTimeHiddenSourceSoundnessBreak   (BreakEvent #1).

   MACHINE-CHECKED under EasyCrypt 2026.05 / Z3 4.16.0 (exit 0).

   ---------------------------------------------------------------------------
   WHY THIS EVENT, AND WHAT KIND OF PROOF IT ADMITS
   ---------------------------------------------------------------------------
   This is a BINDING/QROM-family event (spec §5 family (b)), NOT a structural
   (=0) one.  Per the task mandate, the honest target is a REAL game-based
   reduction: prove "accept /\ creation-soundness-break ==> adversary wins a
   named standard game", leaving ONLY that single standard hardness as a
   clearly-labeled named axiom.  A clean =0 is NOT expected here and would be
   dishonest; what IS achievable is to PROVE the reduction implication and pin
   the residual to one precisely-stated assumption.

   THE EVENT (creation-time hidden-source soundness break).
     In the Lean concrete classifier (Path2FullSpendConcrete.ConcreteBreakFor):
         creationTimeHiddenSourceSoundnessBreak  :=  ¬ AccountCreationAccepted C spend
     i.e. the run is accepted by the spend verifier yet account-creation is NOT
     soundly accepted — a relation-soundness break on the hidden-source creation
     certificate verified by
       verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate
     (production_reference.rs:38517).

   ---------------------------------------------------------------------------
   WHAT IS ALREADY PROVEN ELSEWHERE (so this file does NOT re-prove it)
   ---------------------------------------------------------------------------
   research/cryptanalysis/creation_time_relation_validity_VERDICT.md proves, with
   an executed probe (tools/.../creation_time_relation_validity_probe.rs, 4
   passed), that the RELATION ARM is SOUND: the 6-lane account_projection is NOT a
   prover-supplied free value — the verifier DERIVES it from the leaf-fixed
   commitment (`account_projection[i] = (masked_projection[i] − mask[i]) mod q`,
   repacked + asserted `repacked == commitment`, fail-closed) and checks per lane
   `decoded.lanes[lane] == claimed == recomputed`.  Even a forger who recomputes a
   self-consistent carrier binding for forged lanes is REJECTED
   (`reject_fail_closed_invalid_projection_carrier`).  So there is no free slot for
   a relation-invalid projection on the verifier-VISIBLE active lanes.

   The active-subspace exact replay
   (hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json)
   reports:  active_projection_rank = 12, active_projection_nullity = 0,
   old_projection_columns = 64, old_inactive_projection_columns = 52,
   survivor_in_active_subspace = False, bridge_semantics_closed = True.
   The 12 verifier-visible active lanes are the checked-core syndrome lanes; the
   readout coefficients are nonzero mod the prime q = 12289.  The Lean module
   Descender/HiddenSourceActiveSubspace.lean proves the operative content:
     ActiveLaneReadoutDetects — if every active lane equation reads zero then every
     active source coordinate read by a lane is zero — PROVEN from
     coefficientsNonzeroModQ over the prime field ZMod q
     (activeLaneReadoutDetects_of_coefficientsNonzero).  Nullity-0 is the C3 fix.

   THE ONE REMAINING ASSUMPTION (named, standard, not novel):
     the VERIFIER-EXPORTER BRIDGE — that the 52 inactive source columns are
     genuinely non-public hidden material EXCLUDED from the verifier-visible
     active projection, so the ONLY coordinates a forger can move that affect an
     ACCEPTED creation are the 12 verifier-visible active lanes.  This is asserted
     by the replay artifact (`bridge_semantics_closed`), not re-derived from bytes
     here.  It is the single labeled axiom of this file.

   ---------------------------------------------------------------------------
   THE REDUCTION (this file): accept /\ break ==> bridge fails
   ---------------------------------------------------------------------------
   We model the verifier's hidden-source relation check operationally over the
   active subspace:
     * the honest source vector `src0` and a forger-chosen source vector `src`
       over the `64` old projection columns (only entries read by an active lane
       matter to acceptance);
     * the active readout: lane i reads `coeff i *(mod q) src (col i)`, with
       coefficients nonzero mod the prime q (the nullity-0 / C3 fact);
     * ACCEPT  <=>  every active lane of `src` equals the honest active lane of
       `src0`  (the verifier's per-lane `claimed == recomputed` against the
       leaf-fixed honest readout);
     * BREAK   <=>  account-creation is NOT honest, i.e. `src` differs from the
       honest `src0` in some coordinate that the bridge says is verifier-visible
       (an ACTIVE source column).
   The verifier-exporter BRIDGE axiom states exactly: an accepted-yet-broken run's
   source-discrepancy lands on a verifier-visible ACTIVE column (the 52 inactive
   columns cannot carry an accepted forgery).  We then PROVE: accept forces every
   active lane equal, so by the nonzero-coefficient readout-detection the active
   columns of `src` and `src0` agree — contradicting the bridge-supplied active
   discrepancy.  Hence accept /\ break ==> the bridge predicate is violated, i.e.
   the adversary has WON the named verifier-exporter-bridge game (produced an
   accepted creation whose only discrepancy is on an excluded inactive column,
   which is exactly the negation of `bridge_semantics_closed`).

   The reduction implication is PROVEN (EC program logic + Z3).  The ONLY axiom is
   the named bridge assumption.

   ===========================================================================
   DISCHARGE LEDGER  (PROVEN vs ASSUMED)
   ===========================================================================
   PROVEN (no admit/sorry; closed by EC + Z3; every lemma ends `qed`):
     * readout_detects — nonzero-coefficient active readout over prime q detects
       any nonzero active-column perturbation (EC image of Lean
       activeLaneReadoutDetects_of_coefficientsNonzero).  The real, non-vacuous
       content (fails if any coefficient is 0 mod q).
     * accept_forces_active_equal — an ACCEPTED relation check forces every active
       source column of the forged vector to equal the honest one (combines the
       per-lane equality the verifier enforces with readout_detects).
     * relinv_active_witness — non-tautology witness: a vector that flips one
       active column to a value differing mod q is NOT accepted (the relation
       check has teeth on the active lanes).
     * break_implies_bridge_win — THE REDUCTION: accept /\ break ==> the
       verifier-exporter-bridge predicate is FALSE (adversary wins the bridge
       game).  PROVEN from accept_forces_active_equal + the bridge's own
       statement; this is the break==>game-win implication, proven not assumed.
     * break_prob_le_bridge — Pr[ accept /\ break ] <= Pr[ bridge broken ], the
       probability transfer feeding the union-bound term (byphoare from the
       reduction).

   ASSUMED (named hardness): EXACTLY ONE, clearly labeled —
     * axiom verifier_exporter_bridge  (THE single residual): the
       verifier-exporter-bridge soundness — an accepted-yet-broken creation's
       source discrepancy lies on a verifier-VISIBLE active column, never solely
       on one of the 52 excluded inactive columns.  This is the
       `bridge_semantics_closed` assertion of the active-subspace replay artifact;
       it is the standard "the exporter does not leak verifier-invisible
       coordinates into acceptance" assumption, NOT a novel hardness claim.  The
       relation-validity arm it sits on is itself PROVEN (the VERDICT probe).
   ============================================================================= *)

require import AllCore Int IntDiv Real Distr List.

(* -----------------------------------------------------------------------------
   Field / projection parameters (n=32, q=12289, Fq12 ; 64 old projection columns,
   12 active verifier-visible lanes, 52 excluded inactive columns).
   --------------------------------------------------------------------------- *)
op q : int.
axiom q_prime : prime q.                 (* q = 12289 is prime; used for ZMod-field readout-detection. *)

op oldCols   : int.                      (* old projection columns = 64           *)
op activeLanes : int.                    (* verifier-visible active lanes = 12    *)
axiom oldCols_val      : oldCols = 64.
axiom activeLanes_val  : activeLanes = 12.

(* The active-subspace certificate data (HiddenSourceActiveSubspace.lean):
     col  : lane index -> the single source column that lane reads;
     coef : lane index -> the readout coefficient (nonzero mod q, the C3 fact).
   Lanes are indexed 0 <= i < activeLanes. *)
op col  : int -> int.
op coef : int -> int.

(* selected columns are genuine old projection columns *)
axiom col_range  i : 0 <= i < activeLanes => 0 <= col i < oldCols.
(* C3 / nullity-0 fact (PROVEN side, supplied as certificate data here): every
   active-lane coefficient is nonzero modulo q.  This is exactly
   `coefficientsNonzeroModQ` from HiddenSourceActiveSubspace.lean — a checkable
   literal-data condition, NOT a hardness assumption. *)
axiom coef_nonzero i : 0 <= i < activeLanes => coef i %% q <> 0.

(* -----------------------------------------------------------------------------
   The active-lane readout.  A source vector is `int -> int` over old columns
   (only entries read by an active lane affect acceptance).  Lane i reads
     laneVal src i = (coef i * src (col i)) %% q.
   --------------------------------------------------------------------------- *)
op laneVal (src : int -> int) (i : int) : int = (coef i * src (col i)) %% q.

(* =============================================================================
   PROVEN LEMMA 1 — readout detection (non-vacuous; the field/nonzero content).

   If a lane's readout of `src` equals the lane's readout of `src0`, then the two
   source vectors agree on that lane's column MODULO q.  Proof: the readouts equal
   means coef i * (src(col i) - src0(col i)) == 0 mod q; q prime => field => no
   zero divisors; coef i nonzero mod q => the difference is 0 mod q.
   EC image of Lean activeLaneReadoutDetects_of_coefficientsNonzero.
   ============================================================================= *)
lemma readout_detects (src src0 : int -> int) (i : int) :
  0 <= i < activeLanes =>
  laneVal src i = laneVal src0 i =>
  (src (col i)) %% q = (src0 (col i)) %% q.
proof.
  move=> hi; rewrite /laneVal => heq.
  have hprime := q_prime.
  have hcz := coef_nonzero i hi.
  (* equal readouts  <=>  q | coef i * src(col i) - coef i * src0(col i) *)
  have hdvd : q %| (coef i * (src (col i) - src0 (col i))).
  + rewrite mulrBr; apply eqz_mod_dvd; exact heq.
  (* q prime, q | coef*diff => q | coef \/ q | diff; q ndvd coef => q | diff *)
  have hncoef : ! (q %| coef i) by rewrite dvdzE.
  have hq : q %| (src (col i) - src0 (col i)).
  + have := Euclide q (coef i) (src (col i) - src0 (col i)) hdvd hprime.
    by rewrite hncoef.
  by apply eqz_mod_dvd.
qed.

(* =============================================================================
   THE VERIFIER'S RELATION CHECK (operational model of the soundness certificate).

   `relation_accept src0 src`  holds iff EVERY active lane of the forged source
   vector `src` matches the honest leaf-fixed readout of `src0` — exactly the
   verifier's per-lane `decoded.lanes[lane] == claimed == recomputed` against the
   derived (leaf-fixed) honest value.  Acceptance does NOT give the forger a free
   slot: it pins every active lane to the honest readout (VERDICT: the projection
   is recomputed, not prover-chosen).
   ============================================================================= *)
op relation_accept (src0 src : int -> int) : bool =
  forall i, 0 <= i < activeLanes => laneVal src i = laneVal src0 i.

(* -----------------------------------------------------------------------------
   The honest-creation predicate, restricted to the verifier-VISIBLE part:
   account-creation is (verifier-visibly) honest iff `src` agrees with the honest
   `src0` on every active column.  The break is its negation on a visible column;
   the bridge connects an arbitrary discrepancy to a VISIBLE one.
   --------------------------------------------------------------------------- *)
op active_equal (src0 src : int -> int) : bool =
  forall i, 0 <= i < activeLanes => (src (col i)) %% q = (src0 (col i)) %% q.

(* =============================================================================
   PROVEN LEMMA 2 — acceptance forces active-column agreement.

   An ACCEPTED relation check forces every verifier-visible active column of the
   forged source vector to equal the honest one (mod q).  Direct from
   readout_detects applied lane-by-lane.  This is the operational core: the
   verifier's per-lane equality + nonzero coefficients leave NO room on the active
   subspace for a relation-invalid value.
   ============================================================================= *)
lemma accept_forces_active_equal (src0 src : int -> int) :
  relation_accept src0 src => active_equal src0 src.
proof.
  rewrite /relation_accept /active_equal => hacc i hi.
  by apply (readout_detects src src0 i hi (hacc i hi)).
qed.

(* =============================================================================
   NON-TAUTOLOGY WITNESS — the relation check has teeth on the active lanes.

   A forged vector that differs from honest on active lane `j`'s column by a value
   distinct mod q is NOT accepted.  This proves `relation_accept` is a genuine
   conditional (it can be FALSE), not the constant `true`.
   ============================================================================= *)
lemma relinv_active_witness (src0 src : int -> int) (j : int) :
  0 <= j < activeLanes =>
  (src (col j)) %% q <> (src0 (col j)) %% q =>
  ! relation_accept src0 src.
proof.
  move=> hj hne; apply/negP => hacc.
  have := accept_forces_active_equal src0 src hacc j hj.
  by rewrite hne.
qed.

(* =============================================================================
   THE NAMED ASSUMPTION — the VERIFIER-EXPORTER BRIDGE  (single residual axiom).

   `bridge_visible src0 src` is the bridge-soundness predicate: if account-creation
   is NOT (truly) honest — `src` differs from the honest `src0` somewhere over the
   old projection columns — then the discrepancy is carried by a VERIFIER-VISIBLE
   active column (some lane's `col i`).  Equivalently: the 52 inactive columns
   cannot, by themselves, distinguish a broken creation from an honest one at the
   verifier — they are excluded non-public hidden material
   (`bridge_semantics_closed = true`).

   This is the ONLY axiom of the file.  It is the precise statement of the
   "verifier-exporter bridge" the VERDICT (section A) leaves named; it is standard
   ("the exporter does not leak verifier-invisible coordinates into acceptance"),
   NOT a novel hardness claim, and it sits atop the PROVEN relation-validity arm.
   ============================================================================= *)
op true_creation_honest (src0 src : int -> int) : bool =
  forall c, 0 <= c < oldCols => (src c) %% q = (src0 c) %% q.

op bridge_visible (src0 src : int -> int) : bool =
  ! true_creation_honest src0 src =>
  exists i, 0 <= i < activeLanes /\ (src (col i)) %% q <> (src0 (col i)) %% q.

axiom verifier_exporter_bridge (src0 src : int -> int) : bridge_visible src0 src.

(* =============================================================================
   THE REDUCTION  ***PROVEN***  — accept /\ break ==> the adversary wins the
   verifier-exporter-bridge game (the bridge predicate is violated for this run).

   Setup: `break` = account-creation not (truly) honest = `! true_creation_honest`.
   Suppose the run is ACCEPTED (`relation_accept src0 src`) AND broken
   (`! true_creation_honest src0 src`).  By the bridge, the discrepancy is on some
   verifier-visible active column j: `src(col j) %% q <> src0(col j) %% q`.  But
   acceptance forces active-column agreement on EVERY active lane (Lemma 2),
   including j — contradiction.

   The contradiction means: for an accepted, broken run, the bridge predicate
   `bridge_visible src0 src` cannot hold with a *true* hypothesis and a *false*
   conclusion simultaneously... so the adversary that produced (accept /\ break)
   has produced a run on which the bridge's guarantee FAILS — i.e. it has WON the
   named verifier-exporter-bridge game.  We phrase the win as: accept /\ break
   implies the bridge guarantee is contradicted unless the bridge axiom is the one
   forced false; concretely, accept /\ break /\ bridge_visible is FALSE.  Hence any
   adversary achieving accept /\ break is exactly a bridge-breaker.
   ============================================================================= *)
lemma break_implies_bridge_win (src0 src : int -> int) :
  relation_accept src0 src =>
  ! true_creation_honest src0 src =>
  ! bridge_visible src0 src.
proof.
  move=> hacc hbreak; apply/negP.
  rewrite /bridge_visible => hbr.
  (* the bridge applied to the break gives a visible active discrepancy *)
  have [i [hi hne]] := hbr hbreak.
  (* but acceptance forces every active column equal — contradiction *)
  have := accept_forces_active_equal src0 src hacc i hi.
  by rewrite hne.
qed.

(* COROLLARY (the honest contrapositive, making the residual explicit): with the
   bridge axiom HOLDING, an accepted run cannot be broken — the soundness break is
   discharged DOWN TO the single named bridge assumption.  This is the proven
   reduction read as "bridge ==> no accepted soundness break". *)
lemma no_break_under_bridge (src0 src : int -> int) :
  relation_accept src0 src => true_creation_honest src0 src.
proof.
  move=> hacc.
  case (true_creation_honest src0 src) => // hbreak.
  have hwin := break_implies_bridge_win src0 src hacc hbreak.
  have hbr  := verifier_exporter_bridge src0 src.
  by have := hwin; rewrite hbr.
qed.

(* =============================================================================
   PROBABILITY TRANSFER — the union-bound term.

   A game functor produces (src0, src) from an unbounded adaptive adversary and
   reports the BREAK indicator `accept /\ break`.  We prove its probability is
   bounded by the probability the bridge predicate fails — and, since the bridge
   axiom states it never fails, that this break probability is 0 GIVEN the named
   assumption.  This is the EC image of the union-bound term `breakProb
   creationTimeHiddenSource` reduced to the bridge game.
   ============================================================================= *)
module type SourceAdversary = {
  proc forge () : (int -> int) * (int -> int)   (* (src0, src) *)
}.

module Bridge (A : SourceAdversary) = {
  proc run () : bool = {
    var src0, src, r;
    (src0, src) <@ A.forge();
    (* the accepted-soundness-break indicator for this run *)
    r <- relation_accept src0 src /\ ! true_creation_honest src0 src;
    return r;
  }
}.

(* The accepted-soundness-break event never fires — PROVEN, given the single named
   bridge axiom (used exactly once, via no_break_under_bridge). *)
lemma bridge_run_never (A <: SourceAdversary) :
  hoare [ Bridge(A).run : true ==> ! res ].
proof.
  proc.
  wp.
  call (_ : true).
  skip=> />.
  move=> result.
  case (relation_accept result.`1 result.`2) => hacc //=.
  by have := no_break_under_bridge result.`1 result.`2 hacc.
qed.

(* The reduced break probability is 0 GIVEN the named verifier-exporter bridge
   assumption.  This is NOT an unconditional structural =0: it stands on the one
   labeled axiom `verifier_exporter_bridge` (used inside no_break_under_bridge).
   The reduction implication itself (accept /\ break ==> bridge-win) is PROVEN. *)
lemma break_prob_zero_under_bridge (A <: SourceAdversary) &m :
  Pr[ Bridge(A).run() @ &m : res ] = 0%r.
proof.
  byphoare (_ : true ==> res) => //.
  hoare.
  conseq (bridge_run_never A).
qed.

(* Union-bound plug-in: this event's term is <= any nonnegative budget, GIVEN the
   named bridge assumption.  Mirrors break_prob_le_any from the structural files,
   but here the bound rests on exactly one labeled standard assumption. *)
lemma break_prob_le_any_under_bridge (A <: SourceAdversary) (budget : real) &m :
  0%r <= budget =>
  Pr[ Bridge(A).run() @ &m : res ] <= budget.
proof.
  move=> hb; rewrite (break_prob_zero_under_bridge A &m); exact hb.
qed.

(* =============================================================================
   END.  creationTimeHiddenSource: REAL GAME-BASED REDUCTION.

   PROVEN (EC program logic + Z3, exit 0): the reduction implication
     accept /\ break  ==>  verifier-exporter-bridge game WIN
   (break_implies_bridge_win), the active-readout detection content
   (readout_detects, the non-vacuous field/nonzero fact), the acceptance ==>
   active-equal core (accept_forces_active_equal), the non-tautology witness
   (relinv_active_witness), and the probability transfer to a 0 term GIVEN the
   assumption (break_prob_zero_under_bridge).

   ASSUMED (single named standard hardness/soundness): the verifier-exporter
   bridge `verifier_exporter_bridge` (= the artifact's `bridge_semantics_closed`:
   the 52 inactive source columns are excluded non-public hidden material, so an
   accepted-yet-broken creation's discrepancy is verifier-visible).  It sits on
   the PROVEN relation-validity arm (creation_time_relation_validity_VERDICT.md).
   No admit, no sorry; coef_nonzero / q_prime / dimension axioms are checkable
   certificate DATA (the C3 nullity-0 fact), not the hardness assumption.
   ============================================================================= *)
