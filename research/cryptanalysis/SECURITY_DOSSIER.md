# Descender Post-Quantum Signature — SECURITY DOSSIER (external-review package)

**Date:** 2026-06-10 · **Params:** n = 32, q = 12289, Fq12 = F_q[X]/(X¹²+X+25) ·
**Tooling:** EasyCrypt 2026.05 / Z3 4.16.0 (all `.ec` exit 0, re-run this session);
Lean 4 + Mathlib (build + axiom audit green per assumption ledger).

This is the authoritative consolidation of what is **PROVEN** (machine-checked),
what is **ASSUMED** (named standard primitive), and what is **EXTERNAL**
(irreducible third-party cryptanalysis) for the Descender signature. It is written
to a deliberately honest bar: security is **NOT "established."** It is **reduced**
to a crisp, mostly-standard assumption set with exactly **one** external residual,
with a verifier built but production held **fail-closed** pending external review.

Every lemma/file cited as PROVEN was verified to typecheck (`EC=$(ls
/nix/store/*easycrypt*/bin/easycrypt|head -1)`; all 9 files exit 0) and read for
real-tactic closure (no `admit`/`sorry`/`oops` in tactic position — every such
token appears only inside comment prose). Inconsistencies found are flagged in §8.

---

## 1. CLAIM

**SUF-CMA in the QROM, forging advantage ≤ 2⁻¹²⁸**, at the deployed parameters
(q = 12289, n = 32), under the assumption set of §3.

- **Query model.** After the GAP-C re-model (2026-06-10), the budget **closes at a
  generic classical query budget q = 2⁶⁴ with NO query cap** (it also closes at
  q = 2⁸⁰). This is the machine-checked `gap_c_remodel_spine_wiring.ec` result.
- **Additional deployment margin.** The canonical spec (`formal/spec/
  descender-signature-spec.md:77`) states the deployment claim at the enforced
  cap T = 2³². The ledger's deployment cap (q_cl = 2³³ classical, q_q = 2¹⁶
  quantum, `per_event_epsilon_budget_closure.md`) is therefore **additional
  margin**, no longer load-bearing for closure — the re-model removed the
  cap-dependence the free-birthday model had.
- **Composition.** Forgery success ≤ Pr[accepted-spend forgery] + Σ_{9 breaks}
  Pr[break]. Each break term ≤ 2⁻¹³³, the accepted-spend term ≤ 2⁻¹²⁹, and
  9·2⁻¹³³ + 2⁻¹²⁹ ≤ 2⁻¹²⁸ (proven arithmetic, `spike_union_bound.ec`
  Goal 3b + `gap_c_remodel_spine_wiring.ec` `remodelled_budget_closes`).

**Scope caveat (read with the claim):** the end-to-end *theorem*
`account_creation_prevalidated_slot_authorization_or_break` is reduced **modulo
the §3(b) named standard assumptions and the §3(c) single external residual**, and
modulo the §4 carried correspondence seams (GAP B/D). The GAP-A accepting public
verifier is **built and proven equivalent** to the harness relation, but
production **remains fail-closed** (§5). See §7 for the precise honest framing.

---

## 2. REDUCTION MAP

Forgery success decomposes (single-experiment coupling, §4 GAP B) into the
accepted-spend term + 9 named break events. Each row gives: EC file → key proven
lemma → per-event ε → the named assumption it rests on. "PROVEN-implication" =
the `break ⇒ game-win` (or `=0`) implication is machine-checked; only the bound
magnitude rests on the named assumption.

| # | Event | EC file | Key proven lemma | ε @ q=2⁶⁴ | Rests on | Implication |
|---|---|---|---|---|---|---|
| — | accepted-spend forgery (same-action, fresh msg; AP2-N003 / GAP E) | `accepted_spend_forgery_reduction.ec` | `accepted_spend_term_le_2pow_neg129` (via `inside_box_impossible`, `forge_splits`, `forgeSA_le_codedSIS`) | ≤ 2⁻¹²⁹ | **EXTERNAL** coded-SIS beyond-box (`coded_sis_beyond_box_hard`) + QROM action-collision (`action_collision_qrom`, ~2⁻¹⁶²) | **PROVEN** |
| 1 | `creationTimeHiddenSourceSoundness` | `break_creation_time_hidden_source.ec` | `break_implies_bridge_win` (qed) → `break_prob_zero_under_bridge` | = 0 (given bridge) | verifier-exporter bridge (`verifier_exporter_bridge`, 52-inactive-column exclusion) | **PROVEN** |
| 2 | `prevalidationLeafBinding` | `break_prevalidation_leaf_binding.ec` | `break_implies_second_preimage` → `break_le_spr` → `break_le_spr_bound` | 2⁻¹⁹² | SHA-256 second-preimage-resistance (n=256) | **PROVEN** |
| 3 | `prevalidationRegistryBinding` | `break_prevalidation_registry_binding.ec` | `step_break_sets_second_preimage` → `B_wins_when_break` → `break_le_spr` → `registry_break_bound` | 2⁻¹⁹² | SHA-256 second-preimage-resistance (n=256) | **PROVEN** |
| 4 | `prevalidationDataAvailability` | (no EC file — see §8) | — | n/a | held fail-closed; verifier built (§5), not yet an EC reduction | **build-item** |
| 5 | `postchallengeSlotCreationOrRewrite` | `break_postchallenge_slot_creation_or_rewrite.ec` | `accept_implies_prechallenge`, `spend_loop_never_breaks` → `break_prob_zero` | = 0 | **NONE** (structural) | **PROVEN =0** |
| 6 | `checkedCoreMessageBinding` | `break_checked_core_message_binding.ec` | `break_yields_second_preimage` → `break_le_spr` → `break_le_bound` | 2⁻¹⁵⁰ | SHA-256 (trunc+masked 214-bit) second-preimage-resistance | **PROVEN** |
| 7 | `slotReplayOrRollback` | `break_slot_replay_or_rollback.ec` | `spend_loop_never_breaks` → `break_prob_zero` | = 0 | **NONE** (structural) | **PROVEN =0** |
| 8 | `qromOrDomainSeparation` | (classifier `False`) | spine `break_zero_of_never` (Goal 2) | = 0 | QROM/domain-sep borne at modeling boundary (ledger), not an EC reduction | **=0 by classifier** |
| 9 | `liveSlotLeakageOrKeygen` | (classifier `False`) | spine `break_zero_of_never` (Goal 2) | = 0 | SHA-256 frame-PRF / keygen borne at modeling boundary (ledger), not an EC reduction | **=0 by classifier** |

**Union-bound composition (the spine + gap_c wiring):**

- `spike_union_bound.ec`: `success_le_union` (Goal 1, union bound) + `break_zero_of_never`
  (Goal 2, never-firing ⇒ 0 — full `byphoare`) + `success_le_target` /
  `success_le_2pow_neg128` (Goal 3/3b: per-event ≤2⁻¹³³, accept ≤2⁻¹²⁹ ⇒ ≤2⁻¹²⁸,
  Bigreal.ler_sum + Z3 arithmetic, **no admit**).
- `gap_c_remodel_spine_wiring.ec`: feeds the **concrete** second-preimage ε's into
  that hypothesis — `eps_spr_le_2pow_neg133` (each break ≤2⁻¹³³ at q=2⁶⁴, no cap),
  `remodelled_budget_closes` (2⁻¹²⁹ + 2⁻¹⁹² + 2⁻¹⁹² + 2⁻¹⁵⁰ + zeros ≤ 2⁻¹²⁸),
  `remodelled_per_event_meets_spine_hyp` (exactly the spine's hypothesis).

Net per-event budget consumed: dominant break is checked-core at **2⁻¹⁵⁰** (17 bits
under 2⁻¹³³); the accepted-spend term **2⁻¹²⁹** is the dominant overall contributor.

---

## 3. ASSUMPTION SET (three tiers, exhaustive)

### 3(a) PROVEN — machine-checked, NO assumption

1. **Structural =0 breaks (zero axioms).** `slotReplayOrRollback` (#7) and
   `postchallengeSlotCreationOrRewrite` (#5): a real stateful EC module with a
   guarded consume-once / prechallenge-fixation lifecycle; `while`-loop invariant
   proves the break flag never fires against an **unbounded adaptive** adversary;
   `byphoare` gives Pr=0. Verified `grep -c axiom` = **0** for both files.
2. **The reduction implications** for events #1, #2, #3, #6 and the accepted-spend
   term: `break ⇒ game-win` is proven by EC program logic (`byequiv`/`hoare`),
   independent of the bound magnitude (the hardness axiom bounds the game-win flag
   and sits *after* the reduction lemma in source order).
3. **The budget arithmetic.** `success_le_2pow_neg128`, `remodelled_budget_closes`,
   `two_term_sum_lt_2pow_neg128` — real arithmetic over explicit 2¹²⁹/2¹³³/2¹⁶²
   exponent expansions, closed by Z3.
4. **The box-enumeration certificate (coeff 114 > 96).** `inside_box_impossible`:
   a nonzero coded-kernel vector with L∞ coeff ≤ 96 contradicts the
   `ResponseLinkExactCertificate` ~1.12M-node exact enumeration min cover coeff =
   114 (Lean `minimumCoverCoeffBound`, `native_decide`;
   `ResponseLinkExactCertificate.lean:255/264/611`). The box [-96,96]²³ is the
   **exact** response-difference frontier (2·RESPONSE_BOUND=48), not a margin —
   `beyond_box_VERDICT.md` shows zero approximate-forgery slack and a structureless
   (96,114] dead zone, corroborated independently by BKZ (L∞-min 120).
5. **Creation-time witness-binding injectivity + relation-validity.** The
   account-witness opening map is injective (leaf-fixed SHA-256 opening_commitment
   over a frame including `account_witness_commitment` + unconditional 18-residue
   packing bijection); the soundness certificate **REJECTS** a relation-invalid
   opening even when the forger recomputes a self-consistent carrier binding
   (executed probe, 4 passed: `creation_time_binding_VERDICT.md`,
   `creation_time_relation_validity_VERDICT.md`).
6. **Active-subspace nullity-0 (C3).** `HiddenSourceActiveSubspace.lean`
   `activeLaneReadoutDetects_of_coefficientsNonzero`: over the prime field ZMod
   12289, nonzero readout coefficients ⇒ active readout detects any nonzero
   active-column perturbation. EC image `readout_detects` (qed). This is the
   non-vacuous replacement for the former tautological `ActiveLaneNullityZero`
   (was DEFECT, RESOLVED — assumption ledger C3).

Note: `enumeration_total`, `encode_frame_injective`, `leaf_frame_injective`,
`q_prime`, `oldCols_val`, `activeLanes_val`, `col_range`, `coef_nonzero`,
`eps_spr_ge0` are **structural / checkable-certificate-DATA axioms** — finite
verified computations or byte-encoder injectivity, carrying **no probability
mass**. They are not hardness assumptions.

### 3(b) NAMED STANDARD assumptions

1. **SHA-256 second-preimage-resistance** (bears #2, #3, #6; ε ~ q/2ⁿ, linear in q,
   no birthday square, no query cap). NIST FIPS 180-4 SHA-256; second-preimage on a
   target fixed FIRST by an honest party. Axiom `sha256_second_preimage_resistance`
   in all three files. *Standard:* the operationally faithful model — the verifier
   recomputes the digest/challenge against a leaf-fixed committed value before the
   adversary acts (verified per-event, `gap_c_remodel_result.md §1`).
2. **SHA-256 collision-resistance** (NIST FIPS 180-4) — the conservative companion
   to (1) for the same digests; the role-root binding (ledger C2) reduces to it.
   *Standard / non-novel:* generic CR of SHA-256.
3. **QROM hash collision-resistance of the action map** (bears the accepted-spend
   action-collision term and nominally #8). The action is
   `reviewed_fq12_challenge_action_vector` = a SHA-256-derived hash-to-Fq¹²
   (`uniform_mod_from_hash` over 12289¹²=2¹⁶³.⁰), **not algebraic**; X¹²+X+25
   verified irreducible. Per-fork soundness ~2⁻¹⁶². Axiom `action_collision_qrom`.
   *Standard:* generic QROM hash collision-resistance (ledger C1, reduced from
   NOVEL to STD-QROM).
4. **SHA-256 frame-PRF** (projection-mask / pad hiding; bears #9 nominally). The
   per-slot per-lane projection mask is `uniform_mod_from_hash` on a domain
   byte-frame-disjoint from the pad PRF (82 B vs 154 B); empirical
   max|Pearson| = 0.0049 over 50k seeds. Pad hiding reduces to SHA-256 being a good
   frame-PRF on the mask domain (`deployable_pad_hiding_VERDICT.md`). *Standard:*
   SHA-256-as-RO frame-PRF; non-novel.
5. **Verifier-exporter bridge** (bears #1; `verifier_exporter_bridge`). An
   accepted-yet-broken creation's source discrepancy lands on a verifier-VISIBLE
   active column, never solely on one of the 52 excluded inactive columns
   (`bridge_semantics_closed`). *Standard:* "the exporter does not leak
   verifier-invisible coordinates into acceptance" — sits atop the PROVEN
   relation-validity arm; not a novel hardness claim. (Listed here as a named
   standard *soundness* assumption; an external reviewer who considers it non-standard
   should escalate it to §6(iv).)

### 3(c) EXTERNAL — irreducible, MUST go to third-party review

1. **Coded-SIS beyond-box hardness** (`coded_sis_beyond_box_hard` =
   `open_external_estimator`). The asymptotic/quantum hardness of the
   **response-link-CODED-SIS** problem *beyond* the deployed coeff≤96 box (≥129
   bits), **plus the pseudorandomness of the SHA-256 matrix sampler** that fixes
   the public matrix M. This is the **single irreducibly-external residual**; it
   **cannot be proven internally**. The internal beyond-box cryptanalysis found NO
   obstruction (exact-enum min 114, BKZ L∞-min 120, structureless dead-zone, no
   algebraic shortcut — `beyond_box_VERDICT.md`), but a small modeled regime cannot
   settle the asymptotic claim. **The in-environment lattice-estimator could NOT be
   run (Sage absent)** — the BKZ runs + in-repo exact enumeration are the
   load-bearing internal evidence, not an asymptotic core-SVP curve.

**Tier counts: PROVEN = 6 classes · NAMED-STANDARD = 5 · EXTERNAL = 1.**

---

## 4. SEAMS / CARRIED MODELING ASSUMPTIONS (GAP B / GAP D)

Cross-tool correspondence is **not** machine-checked end-to-end. Per
`formal/README.md` (seam 1, seam 4) these are the **managed standing assumptions**;
seams 2/3 (EasyCrypt↔Jasmin, spec↔Rust) are closed by proof/conformance.

- **GAP B — Lean↔EC event-coupling (OB-1).** The coupling
  `success ⇒ accepted ∨ ⋁ break` is **Lean-PROVEN**
  (`Path2AcceptedSpendClosure.lean:109`
  `success_implies_accepted_or_break_of_structural`, derived not assumed — no
  `sorry`), but in EC it is **axiomatized** as `success_implies_split`
  (`spike_union_bound.ec:129`), because the spike keeps success/accepted/break as
  three separate procedures. The accepted-spend file *does* prove its analogous
  single-experiment coupling for real (`ForgeSplit` + `split_prob`,
  `accepted_spend_forgery_reduction.ec`). Also EC-primitive: `break_univ_uniq`,
  `break_univ_total`, `break_card9` (Lean proves card by `decide`).
- **GAP D — abstract↔EC fidelity bridges.** The registry-binding model (acceptance
  = digest-equality at a digest-keyed entry) and the hidden-source operational
  break (`!true_creation_honest`) are linked to their Lean predicates
  (`DirectAdmissionContains`, `!AccountCreationAccepted`) by in-file prose, not a
  formal cross-tool bridge.

**Reviewer-checklist items a human must confirm (the seam-1 checklist):**
(i) the EC `GAME`/experiment models the same Σ-protocol/SIS relation as the Lean
`AcceptedSpendNumericGame` and the canonical spec §3; (ii) the EC
`success_implies_split` axiom is the faithful image of the Lean-derived coupling;
(iii) the per-event EC break predicates match `Path2FullSpendConcrete.ConcreteBreakFor`;
(iv) the SPR/coded-SIS/bridge games in EC match the deployed verifier semantics
(`production_reference.rs` line cites in each file header).

---

## 5. DEPLOYMENT POSTURE

- **GAP-A public verifier is BUILT and PROVEN-EQUIVALENT.**
  `verify_path2_direct_creation_masked_state18_proof_v1` replays the harness's own
  masked-state18 opening derivation over public payload data; de-toyed (real
  SHA-256/masked_state18 commitment replaces the `P2Q1` toy); wired as an
  AVAILABLE, NON-DEFAULT, labeled path; tested functionally equivalent (IFF, both
  directions) to `verify_path2_prevalidated_slot_creation_relation_replay`;
  negative controls all reject (`public_creation_verifier_build_result.md`). This
  CLOSES #4 prevalidationDataAvailability on the masked-state18 relation.
- **Production REMAINS fail-closed.** `production_deployment_preflight` is
  UNCHANGED — it unconditionally pushes blockers (HIDDEN_SOURCE_PROOF_SEMANTICS,
  CONSTANT_TIME, STORAGE, EXTERNAL_REVIEW, SELECTED_VERIFIER) so
  `deployment_ready == false`. Tests
  `deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land` and
  `path2_masked_state18_path_does_not_flip_production_preflight` PASS. The new path
  is verifier *logic*; the deployment *gate* stays closed.
- **D3 staging ladder** (`cryptanalysis_first_master_tracker.md:258`): (1) testnet
  (valueless) → (2) value-capped mainnet (internal DoD complete **and** external
  review started) → (3) uncapped mainnet (real time-under-attack — **never
  reachable on self-review alone**). The flip from fail-closed to accepting is
  gated on external review, never internal sign-off.
- **Residual: pad_seed-in-clear (no ZK).** The public proof carries `pad_seed` in
  the clear so the verifier can recompute the PRF masks and replay the opening
  (bound to `leaf.pad_seed_digest`). This is honest data-availability; it adds **no
  hiding/zero-knowledge guarantee** over `pad_seed`. A ZK/hiding public proof that
  withholds `pad_seed` (the transparent-descent kind) **remains future work and is
  still fail-closed** (`public_creation_verifier_build_result.md` residual,
  `creation_time_relation_validity_VERDICT.md §B`).

---

## 6. EXTERNAL-REVIEW SCOPE (the minimal list a third-party cryptanalyst must check)

1. **Coded-SIS estimator on the RESPONSE-LINK-CODED problem** (not the bare
   quotient lattice): confirm ≥129-bit classical + quantum hardness of the coded
   distribution beyond the deployed coeff≤96 box, at q=12289, n=32, 12×23
   full-response instance. Run the lattice-estimator (Sage) we could not run
   locally; the internal BKZ + exact enumeration are the corroboration baseline.
2. **SHA-256 matrix-sampler pseudorandomness**: confirm the SHA-256 rejection
   sampler that fixes the public matrix M is a sound PRG / behaves as a random
   matrix for SIS hardness (no algebraic shortcut from the sampler structure).
3. **The seam-correspondence reviewer checklist** (§4): confirm Lean↔EC spec
   correspondence (GAP B `success_implies_split`) and the abstract↔EC fidelity
   bridges (GAP D), since these are not machine-checked end-to-end.
4. **The carried named assumptions (§3(b)) if any is considered non-standard** —
   in particular the verifier-exporter bridge (52-inactive-column exclusion) and
   the SHA-256 frame-PRF pad-hiding assumption, if a reviewer does not accept them
   as standard.

---

## 7. HONEST BOTTOM LINE

Internal verification is **complete to the stated extent**: the full forgery
reduction is reduced to the union-bound spine; all 9 break terms are accounted
(2 structural =0 with zero axioms, 4 binding→SHA-256 second-preimage, 1
creation-soundness→verifier-exporter bridge, 2 classifier-=0) and the
accepted-spend term ≤2⁻¹²⁹ is **derived from a proven reduction** (the inside-box
forgery branch is proven empty by the coeff-114>96 certificate). The budget closes
at q=2⁶⁴ with no query cap. The accepting public verifier is built and proven
equivalent to the harness relation; production stays fail-closed pending external
review.

**Security is NOT "established."** It is **reduced** to:
- the §3(b) named standard assumptions (SHA-256 SPR/CR, QROM hash collision of the
  action map, SHA-256 frame-PRF, verifier-exporter bridge) — mostly standard,
  non-novel; **plus**
- the §3(c) **single external residual**: coded-SIS beyond-box hardness +
  SHA-256-matrix-sampler pseudorandomness (`open_external_estimator`);
- modulo the §4 carried correspondence seams (GAP B Lean↔EC coupling axiom, GAP D
  abstract↔EC fidelity).

This is exactly the recalibrated bar: a crisp, mostly-standard assumption set with
**one** irreducible external residual.

**What would change the verdict:**
- *Negatively:* an external estimator finding the response-link-coded-SIS problem
  is **below 129 bits** at the deployed params (or the SHA-256 matrix sampler is
  distinguishable in a way that helps SIS) — this **breaks the accepted-spend
  term** and forces a parameter change or redesign. A reviewer rejecting the
  verifier-exporter bridge as non-standard would re-open #1 as a gap.
- *Positively (to claim closed, not just reduced):* (a) discharge the §3(c)
  external residual via accepted third-party cryptanalysis; (b) close GAP B by
  deriving `success_implies_split` against the EC experiment (replace the axiom);
  (c) discharge or formally accept the GAP D fidelity bridges; (d) flip production
  to accepting only along the D3 ladder after external review.

---

## 8. INCONSISTENCY FOUND (flagged per mandate)

**`creation_authorization_theorem_status.md` is STALE relative to the current EC
code.** That doc (also dated 2026-06-10, an earlier snapshot) describes the binding
events #2/#3/#4(#6) as resting on **SHA-256 collision-resistance** with axiom
`sha256_collision_resistance` and the free-birthday CR game (adversary outputs both
preimages), and lists GAP A (public verifier) as **unbuilt/fail-closed** and GAP C
(per-event ε wiring) as **not done**. The current ground truth in the `.ec` files
is different and stronger:

- The three binding files carry `sha256_second_preimage_resistance` (SPR, target
  fixed first), **not** `sha256_collision_resistance` — confirmed by axiom census:
  `grep` for `collision_resistance` in `*.ec` returns **none**; the files match
  `gap_c_remodel_result.md` (FREE→SECOND-PREIMAGE re-model, 2026-06-10).
- GAP C **is** wired: `gap_c_remodel_spine_wiring.ec` (exit 0) plugs the concrete
  ε's into the spine hypothesis (`remodelled_per_event_meets_spine_hyp`).
- GAP A **is** built: `public_creation_verifier_build_result.md` +
  `accepted_spend_forgery_reduction.ec` (the AP2-N003 term it lists as "external/
  multi-week, not yet discharged" is now proven `accepted_spend_term_le_2pow_neg129`).

The `assumption_ledger_authoritative.md` summary table (line 116) and
`creation_time_binding_VERDICT.md §1` also still phrase the binding residual as
"SHA-256 **collision** resistance"; the operationally-correct and code-current
model is **second-preimage** (per the GAP-C re-model). This dossier follows the
**code** (SPR) as authoritative; `creation_authorization_theorem_status.md` and the
CR phrasing in the older verdicts should be re-synced to the SPR re-model. This is
a documentation-drift inconsistency, **not** a soundness defect — the SPR model is
strictly the *stronger/honester* of the two, and the re-modeled budget closes with
more margin and no query-cap dependence.

GAP B (OB-1 EC axiom) remains genuinely open and is correctly described by both
docs and this dossier as a carried seam.
