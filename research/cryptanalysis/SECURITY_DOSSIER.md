# Descender Post-Quantum Signature — SECURITY DOSSIER (external-review package)

> ## SOUNDNESS-LEVEL FRAMING (2026-06-19, decided by James)
> Two distinct properties at two distinct levels — NOT a single weakest-link:
> - **SPEND signature SUF-CMA: ~2⁻¹²⁸.** Charged as SECOND-preimage (target fixed first by an
>   honest party; ε ~ q/2ⁿ, no birthday square): prevalidationLeaf/Registry 2⁻¹⁹²,
>   checkedCoreMessageBinding 2⁻¹⁵⁰, accepted-spend 2⁻¹²⁹. Does NOT use the STARK/Merkle.
> - **Account-CREATION STARK knowledge-soundness: ~2⁻¹²⁴ (industry-standard, ACCEPTED).** The
>   prover commits its own FRI Merkle tree, so the conservative charge is COLLISION on the
>   8-element (~248-bit) Poseidon2 digest + the width-16 sponge capacity, both flooring at
>   c/2 ≈ 123.95. This is THE universal STARK config (Plonky3/Polygon/RISC0/SP1/StarkWare Stone
>   all ship 8×31-bit digests as "128-bit-class"); a hard 2⁻¹²⁸ would need a full WIDTH-24
>   Poseidon2 re-host (9-elem digest needs width ≥ 18; the heaviest change in the project) for
>   4 bits — declined. The deg-8 challenge-field bump (commit 4c06f6b) WAS applied: it lifted
>   the FRI field-bound additive term from ~2⁻¹⁰⁴ (below the ceiling) up to the 124 ceiling, so
>   the field is no longer the binding constraint. See
>   research/cryptanalysis/{fri_aok_extractor_instantiation, degree5_field_rehost_scope,
>   sponge_capacity_c9_hardening_scope}.md.

> ## ⛔ RED-TEAM HOLD (2026-06-10; HIGH-2 IN-CLEAR BLIND FIX APPLIED 2026-06-11) — DO NOT SUBMIT AS-IS
> An authorized internal red-team found **THREE HIGH-severity breaks** (all reproduced;
> see `RED_TEAM_REPORT.md` + `redteam_*.md`). **HIGH-1 and HIGH-3 are FIXED + committed +
> verified.** **HIGH-2's in-clear blind leak is now fixed in the transparent-descent opening
> protocol.** The 2026-06-11 audit found that the prior ZK opening transmitted the per-row
> additive blind `b_row` in clear next to the blinded cells, allowing key-free recovery of
> `account_projection` and `mask = PRF(pad_seed,slot)` by modular subtraction. The Rust
> reference now opens relation-preserving masked shares instead and no longer serializes a
> public `blind` field; regression
> `redteam_transparent_descent_zk_proof_blocks_inclear_blind_recovery` decodes only public
> proof bytes and confirms the legacy recovery path no longer returns the raw witness or PRF
> masks. See **`high2_zk_inclear_blind_leak_VERDICT.md`**. The HIGH-2
> **full-relation backend certificate**, knowledge-soundness extractor
> (`p2_sis_extractor`), and hidden range-proof/range-extractor obligations
> remain NAMED, not-yet-discharged obligations, and the old pad-seed-in-clear
> `Path2MaskedState18CreationProofV1` public path is hard-gated. Production stays
> fail-closed. Do not submit until those remaining HIGH-2 obligations are discharged and
> the §6 external coded-SIS scope is widened.
> - **HIGH-1 (§5 SUF-CMA) — ✅ RESOLVED (2026-06-10):** signature malleability — the local-delta
>   verifier accepted non-canonical openings (`local_state` has an in-box kernel (50,−1) ⇒ two
>   valid openings) and FS did not bind the opening bytes ⇒ keyless 2nd accepting signature.
>   *Fix applied:* canonical-response enforcement (the verifier rejects non-canonical kernel-coset
>   reps; the honest signer already emits the canonical one). No byte/SIS/compute change; the two
>   redteam malleability tests are flipped to assert REJECTION and pass. Commit `03924db`.
> - **HIGH-2 (refuted deployable witness-hiding for the creation-proof path) — ⚠️ LEAKING ACCEPT PATH HARD-GATED; SOUNDNESS WORK OPEN:**
>   the masked-state18 public creation proof serialized `pad_seed` in the clear ⇒ key-free
>   full-witness recovery. *Hiding fix attempted (2026-06-10):* a transparent-descent ZK creation
>   proof (Merkle/FRI) with the verifier sentinel FLIPPED. The verifier IS non-stub (it really runs
>   hash/Merkle/FRI/constraint checks). The 2026-06-11 audit then found and reproduced an in-clear
>   blind leak in that opening protocol. *Fix applied:* the opened row now carries masked-share
>   cells satisfying the affine relation directly and no public deblinding scalar; regression
>   `redteam_transparent_descent_zk_proof_blocks_inclear_blind_recovery` passes. *Soundness hardening
>   applied (2026-06-11):* opened rows now enforce the **same account-share representative** per lane
>   in both equations (`account_share + mask_share = masked_projection` and
>   `account_share = lo_share + 128*hi_share`), not just a linear aggregate residual; regression
>   `path2_transparent_zk_opened_row_relation_prevents_split_witness_cancellation` constructs a
>   split row whose aggregate residual cancels to zero and confirms the per-lane digit check rejects.
>   This clears the immediate sampled-row split-witness/cancellation bug, but it is not the full
>   extractor theorem. *Blocker-1 negative result (2026-06-11):*
>   `redteam_transparent_zk_core_accepts_public_only_synthetic_trace` builds an accepting core proof
>   from public `masked_projection` alone, showing the current opening core still does not enforce
>   pad-seed binding, PRF mask derivation, packed-witness repack equality, or role-commitment
>   consistency. *Hard-gate applied
>   (2026-06-11):* the pad-seed-in-clear masked-state18 public verifier and proof-carrying dispatcher
>   now fail closed with `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR`; the relation checker is
>   internal/debug only. *Open work:* (1) full-relation backend certificate for
>   C1/C2/C4/C6 and trace-to-role binding; (2) `p2_sis_extractor` knowledge soundness; (3) hidden
>   range-proof/range-extractor soundness for the masked digit shares; (4) §6 external coded-SIS
>   scope-widening. The public proof-bytes boundary now runs the incomplete core first, then
>   fail-closes with `PATH2_TRANSPARENT_DESCENT_FULL_HIDDEN_RELATION_DISABLED_ERROR` for otherwise
>   valid transcripts until full hidden-relation constraints are enforced. The production wrapper
>   `verify_path2_direct_creation_transparent_descent_payload_boundary_with_proof_bytes_for_production`
>   therefore also rejects valid transparent payload proofs at that same hidden-relation gate. The
>   production proof-family capability gate
>   `verify_path2_transparent_proof_family_capability_for_production` has the same behavior: validate
>   the capability object first, then reject until the extractor/range obligations are closed. The
>   capability object itself carries a digest of
>   `Path2TransparentProductionSoundnessObligationsV1` with
>   `sampled_row_same_representative_enforced = true`,
>   `aggregate_cancellation_regression_covered = true`,
>   `pad_seed_binding_constraint_enforced = false`,
>   `prf_mask_derivation_constraint_enforced = false`,
>   `packed_witness_repack_constraint_enforced = false`,
>   `role_commitment_consistency_constraint_enforced = false`,
>   `full_hidden_relation_constraints_enforced = false`,
>   the uninstantiated full-relation backend sufficiency target,
>   `full_relation_backend_certificate_instantiated = false`,
>   `p2_sis_extractor_instantiated = false`, `hidden_range_extractor_instantiated = false`, and
>   `external_coded_sis_review_complete = false`; it also carries
>   `production_soundness_obligations_discharged = false`, so this state is transcript-bound rather
>   than just prose. See
>   **`high2_zk_inclear_blind_leak_VERDICT.md`**.
> - **HIGH-3 (§2/§3 GAP-C) — ✅ RESOLVED (2026-06-10):** the checked-core message-binding term
>   is MULTI-TARGET (T·q/2ⁿ over T signing queries), not single-target. *Fix applied:* the
>   checked-core challenge was WIDENED to **246 effective bits** (`CHALLENGE_HARDENING_BYTES`
>   4→8, so `CHALLENGE_BYTES` 27→31, +4 sig bytes; message-binding width = 182 algebraic-after-
>   mask + 64 hardening = 246), and the EC spine now charges the **multi-target term
>   T·eps_spr(n) = T·q/2ⁿ** (`break_checked_core_message_binding.ec` `multi_target_break_le_bound`
>   +  `gap_c_remodel_spine_wiring.ec` `eps_checked = tQ·eps_spr 246`). At T=2³², q=2⁶⁴:
>   T·q/2²⁴⁶ = 2⁻¹⁵⁰ ≤ the 2⁻¹³³ per-event budget (was 2⁻¹¹⁸ at n=214 — a breach). No usage cap
>   beyond T≤2³². Sig sizes: linear-sigma **264 B**, Fq12 envelope **318 B** (pk=172 unchanged).
>
> **SURVIVED** hard attacks (evidence base strengthened, not "unbreakable"): the lattice core
> (coded vector L∞ 114>96; ring/module structure not yet attacked = external residual), the
> 96<114 box frontier, and the consume-once lifecycle. The reductions/proofs below remain valid
> *as stated*; the three findings are where the dossier's claims OVER-reached the as-built scheme.

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

- **Query model.** After the GAP-C re-model + HIGH-3 multi-target widening (2026-06-10),
  the budget **closes at a generic classical query budget q = 2⁶⁴ at the multi-target
  signing horizon T = 2³²**: the binding events are charged at the correct multi-target
  rate **T·q/2ⁿ**, and the checked-core width was widened to **n = 246** so the dominant
  multi-target term is T·q/2²⁴⁶ = 2⁻¹⁵⁰. This is the machine-checked
  `gap_c_remodel_spine_wiring.ec` result (`eps_checked = tQ·eps_spr 246`, `tQ = 2³²`).
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
| 1 | `creationTimeHiddenSourceSoundness` | `break_creation_time_hidden_source.ec` | `break_implies_bridge_win` (qed) → `break_prob_zero_under_bridge` | = 0 (given bridge) | verifier-exporter bridge — residual axiom `verifier_exporter_bridge` (52-inactive-column exclusion); a maximal-rigor attempt to restate it as the contrapositive `creation_validity_is_active` was REVERTED (logically-equivalent, not a discharge — see §3(b)5) | **PROVEN given bridge** |
| 2 | `prevalidationLeafBinding` | `break_prevalidation_leaf_binding.ec` | `break_implies_second_preimage` → `break_le_spr` → `break_le_spr_bound` | 2⁻¹⁹² | SHA-256 second-preimage-resistance (n=256) | **PROVEN** |
| 3 | `prevalidationRegistryBinding` | `break_prevalidation_registry_binding.ec` | `step_break_sets_second_preimage` → `B_wins_when_break` → `break_le_spr` → `registry_break_bound` | 2⁻¹⁹² | SHA-256 second-preimage-resistance (n=256) | **PROVEN** |
| 4 | `prevalidationDataAvailability` | (no EC file — see §8) | — | n/a | held fail-closed; verifier built (§5), not yet an EC reduction | **build-item** |
| 5 | `postchallengeSlotCreationOrRewrite` | `break_postchallenge_slot_creation_or_rewrite.ec` | `accept_implies_prechallenge`, `spend_loop_never_breaks` → `break_prob_zero` | = 0 | **NONE** (structural) | **PROVEN =0** |
| 6 | `checkedCoreMessageBinding` | `break_checked_core_message_binding.ec` | `break_yields_second_preimage` → `break_le_spr` → `break_le_bound` (single-target, PROVEN) → **`multi_target_break_le_bound`** (via `axiom sha256_multi_target_spr`, :362) | **2⁻¹⁵⁰ (multi-target T·q/2²⁴⁶, T=2³²)** | SHA-256 (trunc+masked **246-bit**) second-preimage-resistance + **multi-target-union axiom** (standard T×single-target hybrid; conservative, textbook-provable, axiomatized not machine-proven) | **PROVEN mod multi-target-union axiom** |
| 7 | `slotReplayOrRollback` | `break_slot_replay_or_rollback.ec` | `spend_loop_never_breaks` → `break_prob_zero` | = 0 | **NONE** (structural) | **PROVEN =0** |
| 8 | `qromOrDomainSeparation` | (classifier `False`) | spine `break_zero_of_never` (Goal 2) | = 0 | QROM/domain-sep borne at modeling boundary (ledger), not an EC reduction | **=0 by classifier** |
| 9 | `liveSlotLeakageOrKeygen` | (classifier `False`) | spine `break_zero_of_never` (Goal 2) | = 0 | SHA-256 frame-PRF / keygen borne at modeling boundary (ledger), not an EC reduction | **=0 by classifier** |

**Union-bound composition (the spine + gap_c wiring):**

- `spike_union_bound.ec`: `success_le_union` (Goal 1, union bound — **now PROVEN
  from the pointwise `structural_split` predicate via `mu_le`+`mu_or`+`mu_has_le`,
  no longer an axiom; see §4 GAP B**) + `break_zero_of_never`
  (Goal 2, never-firing ⇒ 0 — full `byphoare`) + `success_le_target` /
  `success_le_2pow_neg128` (Goal 3/3b: per-event ≤2⁻¹³³, accept ≤2⁻¹²⁹ ⇒ ≤2⁻¹²⁸,
  Bigreal.ler_sum + Z3 arithmetic, **no admit**).
- `gap_c_remodel_spine_wiring.ec`: feeds the **concrete multi-target** second-preimage
  ε's into that hypothesis — `multi_target_le_2pow_neg133` (each multi-target break
  term tQ·eps_spr(n) = T·q/2ⁿ ≤2⁻¹³³ at q=2⁶⁴, T=2³²; checked-core n=246),
  `remodelled_budget_closes` (2⁻¹²⁹ + 2⁻¹⁶⁰ + 2⁻¹⁶⁰ + 2⁻¹⁵⁰ + zeros ≤ 2⁻¹²⁸),
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
4. **Projection-mask frame-PRF** (projection-mask / pad hiding; bears #9 nominally).
   *(2026-06-18 RE-CHARGE, branch `deployed-mask-prf-swap`: the deployed mask PRF
   was swapped SHA-256 -> native KoalaBear-Poseidon2 sponge + wide-then-reduce, to
   match the fast/recursion-friendly account-creation proof. Assumption re-tiered
   `A_sha256_projection_mask_frame_prf` -> `A_poseidon2_projection_mask_prf`:
   Poseidon2-over-the-VETTED-31-bit-KoalaBear-field as RO (quantum-indifferentiable,
   arXiv 2504.16887), NAMED-STANDARD. `frame_prf_ro_reduction.ec` re-charged + still
   exit 0; the abstract RO+min-entropy proof is unchanged. ONE honest delta: SHA's
   rejection sampler was EXACTLY uniform; wide-then-reduce is 2^-141-uniform, adding
   6*2^-141 << 2^-128 to the advantage — negligible. Mask stays HIDING-ONLY (forgery
   reduction untouched). The pre-swap description below is retained for the SHA path.)*
   The per-slot per-lane projection mask was `uniform_mod_from_hash` on a domain
   byte-frame-disjoint from the pad PRF (82 B vs 154 B); empirical
   max|Pearson| = 0.0049 over 50k seeds. Pad hiding reduces to the mask PRF being a
   good frame-PRF on the mask domain (`deployable_pad_hiding_VERDICT.md`). *Standard:*
   Poseidon2-over-31-bit-field-as-RO (was SHA-256-as-RO); non-novel.
   *(2026-06-10 — DISCHARGED (classical), after a first faked attempt was caught +
   reverted. `frame_prf_ro_reduction.ec` (exit 0, 7 qed, 0 admit/sorry, verified)
   now proves `frame_prf_advantage`: |Pr[Real]−Pr[Ideal]| ≤ q·2⁻ᴴ, DERIVED via
   `identical_until_bad` (no seed-bearing RO query ⟹ real ≡ ideal mask) composed
   with `bad_event_bound` (Pr[seed-bearing query] ≤ q·2⁻ᴴ over the query list +
   `pad_seed` min-entropy H=256). Every axiom is RO-behaviour / entropy / structural
   — NONE bounds the advantage (audited). So pad-hiding FOLDS into {RO + `pad_seed`
   min-entropy} rather than being a bare named assumption — it reuses the same RO
   primitive as §3(b)(3). **Residual:** the proven bound is CLASSICAL (q·2⁻ᴴ); the
   QROM/quantum variant swaps the union step for the standard O2H/query-extraction
   bound (q²·2⁻ᴴ) — the identical-until-bad + bad-event structure built here is what
   the quantum proof reuses, so only the O2H lemma swap remains. Status:
   classically-discharged to RO+entropy; quantum-structured-pending-O2H.)*
   *(SCOPE — do NOT conflate with HIGH-2. This frame-PRF pad-hiding result is for the
   **spend-path** public surface, which is digest/masked-only and never serializes
   `pad_seed` (`deployable_pad_hiding_VERDICT.md`). It does NOT cover the **creation-proof**
   masked-state18 data-availability path (`Path2MaskedState18CreationProofV1`), which
   serializes `pad_seed` in the clear and is NON-HIDING BY DESIGN (red-team HIGH-2, §5).
   The hiding creation path is the transparent-descent ZK, IN PROGRESS — HIGH-2 not yet
   resolved.)*
5. **Verifier-exporter bridge** (bears #1). An accepted-yet-broken creation's
   source discrepancy lands on a verifier-VISIBLE active column, never solely on
   one of the 52 excluded inactive columns. *Standard:* "the exporter does not leak
   verifier-invisible coordinates into acceptance" — sits atop the PROVEN
   relation-validity arm; not a novel hardness claim. (Listed here as a named
   standard *soundness* assumption; an external reviewer who considers it non-standard
   should escalate it to §6(iv).)
   *(2026-06-10 — a maximal-rigor attempt to restate this as the contrapositive
   `creation_validity_is_active` (`A ⟹ H` vs the old `¬H ⟹ ¬A`) was REVERTED:
   the two are LOGICALLY EQUIVALENT (interderivable in pure FOL with no auxiliary
   axioms, verified both directions in EC), axiom count unchanged 6 → 6 — a
   rephrasing, NOT a discharge or weakening. The adversarial verifier flagged it
   FAKED-as-reduction; the file was reverted to the honest `verifier_exporter_bridge`
   axiom. This item genuinely remains a named-standard soundness assumption, sitting
   atop the PROVEN relation-validity arm + the PROVEN active-subspace nullity-0.)*

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

**Tier counts: PROVEN = 6 classes · NAMED-STANDARD = 4 classical / 5 QROM-conservative
(frame-PRF folds into RO classically; +1 until the quantum O2H swap) · EXTERNAL = 1.**

**FINAL MINIMIZED ASSUMPTION SET (after the 2026-06-10 maximal-rigor pass).** TWO
genuine reductions landed; one attempt was caught faked + reverted:
- **GAP B (seam, §4):** reduced from **two probability-inequality axioms** to **one
  pointwise, probability-free predicate** (`structural_split`), union bound now a
  PROVEN EC lemma. **No carried axiom in the union-bound spine carries probability
  mass anymore.**
- **SHA-256 frame-PRF (§3(b)4):** GENUINELY DISCHARGED (classical) — pad-hiding now
  FOLDS into {RO + `pad_seed` min-entropy} via the proven `frame_prf_advantage`
  lemma (after a first faked attempt was caught + reverted). It reuses the §3(b)(3)
  RO primitive, so it drops out of the independent named-standard set **classically**
  (→ 4); the QROM/quantum bound is structured-pending-the-O2H-swap.
- **Verifier-exporter bridge (§3(b)5):** the attempted "reduction" was a logically-
  equivalent contrapositive (axiom 6→6, no weakening), caught + reverted. It is a
  GENUINE modeling seam — stays named-standard, atop the PROVEN nullity-0.

The irreducible core is therefore: **SHA-256 SPR/CR + action-map QROM collision +
verifier-exporter bridge soundness** (named-standard, **4 classical** — frame-PRF
folded into the RO/QROM; +1 if the quantum O2H swap is not yet done) · **coded-SIS
beyond-box hardness + SHA-256 matrix-sampler pseudorandomness** (external, 1) · plus
the two correspondence seams GAP B (now a single pointwise predicate) and GAP D.

---

## 4. SEAMS / CARRIED MODELING ASSUMPTIONS (GAP B / GAP D)

Cross-tool correspondence is **not** machine-checked end-to-end. Per
`formal/README.md` (seam 1, seam 4) these are the **managed standing assumptions**;
seams 2/3 (EasyCrypt↔Jasmin, spec↔Rust) are closed by proof/conformance.

- **GAP B — Lean↔EC event-coupling (OB-1) — REDUCED 2026-06-09 (the union bound
  is now a PROVEN EC lemma; only a pointwise, probability-free predicate axiom
  remains).** The coupling `success ⇒ accepted ∨ ⋁ break` is **Lean-PROVEN**
  (`Path2AcceptedSpendClosure.lean:109`
  `success_implies_accepted_or_break_of_structural`, derived not assumed — no
  `sorry`; `lake build Descender.Path2AcceptedSpendClosure` exit 0, 3297 jobs).
  Previously the EC side carried **two probability-inequality axioms**
  (`success_implies_split` + a duplicate `success_le_union_c`) that imported the
  whole union bound. These are now **removed**; the EC residual is the single
  pointwise predicate axiom `structural_split (o : outcome) : success_pred o =>
  accepted_pred o \/ has (break_pred o) break_univ` (`spike_union_bound.ec:109`) —
  the verbatim image of the Lean theorem's conclusion, carrying **no probability
  mass**. From it the union bound `success_le_union` is a **PROVEN EC lemma**
  (`mu_le` from the pointwise containment + `mu_or` + the stdlib union bound
  `Distr.mu_has_le`), not an axiom. Teeth-tested: replacing `structural_split` with
  `true` breaks the union-bound proof (exit 1, "does not apply to the goal"), so
  the axiom is genuinely consumed and not vacuously closed. The new axiom is
  **strictly more primitive**: the probability inequality is derivable from the
  pointwise implication (done in-file) but not conversely. Net axiom change in
  `spike_union_bound.ec`: **5 → 4**. Also EC-primitive: `break_univ_uniq`,
  `break_univ_total`, `break_card9` (Lean proves card by `decide`).
- **GAP D — abstract↔EC fidelity bridges.** The registry-binding model (acceptance
  = digest-equality at a digest-keyed entry) and the hidden-source operational
  break (`!true_creation_honest`) are linked to their Lean predicates
  (`DirectAdmissionContains`, `!AccountCreationAccepted`) by in-file prose, not a
  formal cross-tool bridge.

**Reviewer-checklist items a human must confirm (the seam-1 checklist):**
(i) the EC `GAME`/experiment models the same Σ-protocol/SIS relation as the Lean
`AcceptedSpendNumericGame` and the canonical spec §3; (ii) the EC
`structural_split` axiom (`spike_union_bound.ec:109`) is the faithful pointwise
image of the Lean-derived coupling `success_implies_accepted_or_break_of_structural`
(the union bound `success_le_union` is now PROVEN from it in EC, no longer assumed);
(iii) the per-event EC break predicates match `Path2FullSpendConcrete.ConcreteBreakFor`;
(iv) the SPR/coded-SIS/bridge games in EC match the deployed verifier semantics
(`production_reference.rs` line cites in each file header).

---

## 5. DEPLOYMENT POSTURE

- **GAP-A public masked-state18 verifier is HARD-GATED.**
  The old `verify_path2_direct_creation_masked_state18_proof_v1` path no longer accepts any valid
  public proof because that proof serializes `pad_seed` in clear. It still validates common frame
  bindings and then returns `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR`. The historical
  masked-state18 relation equivalence is preserved only in the internal debug checker
  `verify_path2_direct_creation_masked_state18_proof_debug_relation_v1`; negative controls continue
  to exercise that relation (`public_creation_verifier_build_result.md` is now historical).
- **Production REMAINS fail-closed.** `production_deployment_preflight` is
  UNCHANGED — it unconditionally pushes blockers (HIDDEN_SOURCE_PROOF_SEMANTICS,
  CONSTANT_TIME, STORAGE, EXTERNAL_REVIEW, SELECTED_VERIFIER) so
  `deployment_ready == false`. Tests
  `deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land` and
  `path2_masked_state18_path_does_not_flip_production_preflight` PASS. The old
  masked-state18 acceptor is also hard-gated at verifier level.
- **D3 staging ladder** (`cryptanalysis_first_master_tracker.md:258`): (1) testnet
  (valueless) → (2) value-capped mainnet (internal DoD complete **and** external
  review started) → (3) uncapped mainnet (real time-under-attack — **never
  reachable on self-review alone**). The flip from fail-closed to accepting is
  gated on external review, never internal sign-off.
- **HIGH-2 HIDING RESOLVED via masked-share opening, but transparent creation proof is
  PUBLIC-DISABLED until full hidden-relation constraints are enforced (2026-06-10/11).** The non-hiding masked-state18 creation proof
  (`Path2MaskedState18CreationProofV1`) serializes `pad_seed` in the clear ⇒
  key-free full-witness recovery (reproduced by
  `redteam_masked_state18_creation_proof_leaks_full_witness_key_free`); its public verifier is now
  hard-disabled and the relation checker is debug-only. The transparent-descent masked-share prover
  and incomplete debug core are built, but the public verifier is fail-closed because
  `redteam_transparent_zk_core_accepts_public_only_synthetic_trace` shows the core does not enforce
  pad-seed binding, PRF mask derivation, packed-witness repack equality, or role-commitment
  consistency. Prover:
  `path2_transparent_descent_prove_creation_v1`. Verifier:
  `path2_transparent_descent_verify_zk_opening_v1`, wired into
  `verify_path2_direct_creation_transparent_descent_payload_boundary_with_proof_bytes`.
  The public surface
  is `Path2TransparentDescentPublicViewV1` (commitment roots + PRF-masked lanes
  ONLY — NO `pad_seed`, NO raw witness). The debug core recomputes the FS
  challenges, authenticates every Merkle opening to the public roots, evaluates
  the sampled masked-share row relation, checks the C5 public/private masked-projection
  seam, and verifies FRI low-degree folding (final-layer-zero verdict). It does
  not yet enforce C1/C2/C4/C6. HIDING is
  asserted for real by `path2_transparent_zk_proof_hides_pad_seed_and_witness`
  (no `pad_seed`/mask/projection window in the proof bytes; red-team recovery
  surface absent), round-trip + a 6-way negative-control matrix are GREEN. Payload
  ≈ 14.8 KB (uncompacted; under the 16,416 B fallback threshold; the ~0.7–1.3 KB
  design target needs a follow-up compacted opening encoding — size-only).
  The missing full-relation proof is now a typed contract:
  `Path2TransparentFullHiddenRelationProofContractV1` requires pad-seed digest
  binding, PRF mask derivation, packed-witness repack equality, role-commitment
  consistency, leaf-context binding, creation-binding digest binding, and public
  masked-projection binding. The canonical contract is intentionally
  uninstantiated and is bound into both production obligations and the
  proof-family capability digest. The verifier boundary now also canonicalizes
  `Path2TransparentFullHiddenRelationPublicInputsV1`, the public statement a
  future backend must prove against, and calls
  `verify_path2_transparent_full_hidden_relation_subproof_v1` after the
  incomplete Merkle/FRI/row core accepts. The statement binds the contract/plan
  digests, leaf and creation-binding context, all creation role commitments,
	  public masked lanes, trace/constraint roots, and schedule, without revealing
	  `pad_seed` or raw witness bytes. The proof body is now a suite wrapper with
	  separate masked-share-core and full-hidden-relation subproof segments; the
	  latter is a typed uninstantiated-backend envelope with the statement bytes
	  as payload and a zero local trace-binding proof digest in the current
	  fail-closed build. Public-view verification structurally checks any carried
	  full-relation envelope segment before returning the disabled sentinel.
  **SPEND delta 0.** Build report: `high2_transparent_descent_zk_build.md`;
  design: `high2_transparent_descent_zk_design.md`.
  > **NAMED OBLIGATION — transparent-descent knowledge-soundness extractor
  > (`p2_sis_extractor`): testnet-acceptable, discharge before value-capped
  > mainnet.** From any accepting transparent-descent transcript an extractor must
  > recover a C1–C7-satisfying witness (a valid hidden opening) OR export a named
  > binding break (short SIS kernel from a double opening / a hash collision on a
  > commitment root). This is NOT proven and NOT claimed proven; it is the genuine
  > multi-week cryptographic content. Verifier completeness/round-trip do NOT
  > depend on it — the SOUNDNESS claim does. (`path2_descent_sis_feasibility.md:66`,
  > `creation_time_relation_validity_VERDICT.md §B` — every C1–C7-satisfying
  > assignment is already PROVEN relation-valid, so the extractor closes
  > soundness by composition once discharged.)
  *Note — scope:* this NON-hiding property is specific to the **creation-proof**
  masked-state18 data-availability path. It does NOT contradict the **spend-path**
  pad-hiding result (`deployable_pad_hiding_VERDICT.md`), which holds: the on-chain
  spend surface is digest/masked-only and never serializes `pad_seed`.

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
   correspondence — now narrowed to the single pointwise predicate axiom GAP B
   `structural_split` being the faithful image of the Lean theorem (the union bound
   over it is PROVEN in EC) — and the abstract↔EC fidelity bridges (GAP D), since
   these are not machine-checked end-to-end.
4. **The carried named assumptions (§3(b)) if any is considered non-standard** —
   in particular the verifier-exporter bridge (52-inactive-column exclusion) and
   the SHA-256 frame-PRF pad-hiding assumption, if a reviewer does not accept them
   as standard.
5. **NEW (HIGH-2 transparent-descent ZK) — two items:**
   (a) **`p2_sis_extractor` knowledge-soundness/extraction** for the transparent
   FRI-style argument over the masked-state18 relation (C1–C7): from an accepting
   transcript, extract a C1–C7-satisfying witness (valid hidden opening) OR a named
   binding break. NAMED OBLIGATION — testnet-acceptable, discharge before
   value-capped mainnet. The verifier (`path2_transparent_descent_verify_zk_opening_v1`)
   provides completeness + the round-trip/negative matrix WITHOUT it; the soundness
   claim needs it. (b) **New public surface for the coded-SIS estimator:** the
   transparent-descent public view exposes `trace_commitment_root` +
   `constraint_commitment_root` + `masked_projection` over the SAME role matrices as
   the response-link-coded-SIS problem (item 1). The roots are hash digests (should
   leak nothing), but the masked lanes + the existence of FS-sampled openings are a
   surface the estimator scope (item 1) must be WIDENED to cover. The transparent FS
   argument is a new *usage* of the already-carried SHA-256-as-(Q)RO / hash-CR
   assumption — **no new hardness class** (`high2_transparent_descent_zk_design.md §3.2`).

---

## 7. HONEST BOTTOM LINE

Internal verification is **complete to the stated extent**: the full forgery
reduction is reduced to the union-bound spine; all 9 break terms are accounted
(2 structural =0 with zero axioms, 4 binding→SHA-256 second-preimage, 1
creation-soundness→verifier-exporter bridge, 2 classifier-=0) and the
accepted-spend term ≤2⁻¹²⁹ is **derived from a proven reduction** (the inside-box
forgery branch is proven empty by the coeff-114>96 certificate). The budget closes
at q=2⁶⁴ with no query cap. The old masked-state18 accepting public verifier is
hard-disabled, and the transparent public verifier remains fail-closed until the
full hidden relation is enforced and reviewed.

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
  external residual via accepted third-party cryptanalysis; (b) close the *residual*
  GAP B by deriving the pointwise `structural_split` against the actual EC
  experiment distribution (the union bound over it is already proven; the
  probability-mass coupling axioms were removed 2026-06-09);
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
