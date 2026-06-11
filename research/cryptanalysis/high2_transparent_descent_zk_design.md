# HIGH-2 — Transparent-Descent ZK Creation Proof: DESIGN SPEC

**Date:** 2026-06-10 · **n=32, q=12289, Fq12 = F_q[X]/(X¹²+X+25)** · 6 projection lanes
**Status:** DESIGN ONLY — no prover implemented. Resolves the rank blocker; pins the proof
structure, soundness re-clearing, cost, and an incremental green-checkpointed build plan.
**Decision context:** maintainer chose the descent-framed transparent-descent ZK recommendation
(`high2_descent_native_hiding_exploration.md`). This spec is the buildable elaboration of that
recommendation.

**What this fixes.** Red-team HIGH-2: `Path2MaskedState18CreationProofV1`
(`tools/parameter-search/src/production_reference.rs:~1484`) serializes `pad_seed` in the clear, so
any observer recovers the full hidden-source witness key-free (test
`redteam_masked_state18_creation_proof_leaks_full_witness_key_free`). That masked-state18 creation
proof is now explicitly labeled **NON-HIDING / data-availability-only** in code and dossier (Part 1).
This spec is the **hiding** replacement: the transparent-descent ZK creation proof whose leak-free
public vehicle (`Path2TransparentDescentPublicViewV1`) and verifier-side plumbing
(`verify_path2_direct_creation_transparent_descent_selected_view_bridge`) already exist, stubbed at
the relation/opening step.

---

## 0. What already exists (the scaffold this spec fills)

Verified present in `production_reference.rs`:

- **Leak-free public view** `Path2TransparentDescentPublicViewV1` (`:1500`): carries
  `masked_projection` (6 lanes), `trace_commitment_root`, `constraint_commitment_root`,
  leaf/binding/aggregation/transcript digests, `trace_rows`, `query_count`, `column_profile`,
  `query_index`. **NO `pad_seed`, NO unmasked witness.** Serialization is asserted private-material-
  free by `path2_transparent_descent_public_view_serializes_only_public_material` (calls
  `assert_no_path2_transparent_private_material`).
- **Payload envelope** `Path2TransparentDescentProofPayloadV1` (`:1764`): `public_view` +
  `payload_digest` + `payload_byte_len` + `proof_bytes_digest` + `query_openings_digest` +
  `merkle_paths_digest` + `consistency_messages_digest`.
- **Bound verifier-side plumbing** (all implemented, all GREEN): public-view binding
  (`verify_path2_transparent_descent_public_view` `:25416`), transcript digest
  (`path2_transparent_descent_transcript_digest` `:25345`), commitment-API consistency
  (`path2_transparent_commitment_api_v1` `:24416`), column-profile check
  (`verify_path2_two_view_column_profile_id` `:24877`), query-schedule pair
  (`verify_path2_transparent_query_schedule_pair` `:23528`), query-opening manifest binding
  (`verify_path2_transparent_query_opening_manifest_for_public_view` `:25314`), proof-bytes envelope
  decode + context bind (`verify_path2_transparent_proof_bytes_envelope_v1` `:25775`), relation
  layout (`path2_transparent_relation_layout_v1_digest`), and the leaf/binding/aggregation binding in
  the bridge (`verify_path2_direct_creation_transparent_descent_selected_view_bridge` `:26708`).
- **The single stubbed step:** every `verify_path2_direct_creation_transparent_descent_*` boundary
  returns `"Path 2 transparent descent proof verifier is not implemented"` AFTER all bindings pass
  (`:26705`, `:26802`, `:26841`). The dispatcher rejects the kind at `:26555`. The query-budget
  symbols `q_transparent_{trace_commit,query_row,merkle_open,consistency,extractor,simulator}` are
  already declared active (`path2_direct_query_budget_v1`, test at `:53118`).

So the binding/transcript/commitment-root/query-manifest machinery is done; **only the cryptographic
opening argument (commit-and-prove relation check) is missing.** This spec specifies exactly that.

---

## 1. RANK BLOCKER RESOLUTION

### 1.1 The blocker

The naive shortcut "publish the SIS commitment `C = A·v mod q` as the public proof object" RE-LEAKS
the witness. `path2_descent_sis_rank_hiding_probe.md`: the role matrices are **full COLUMN rank** in
the 48/64-row bands —

```
pad_source / projection_relation:  rows 48|64, cols 30, rank 30, kernel 0  -> v recoverable
opening_tuple:                     rows 48|64, cols 48, rank 48, kernel 0  -> v recoverable
```

— so an observer solves `A·v = C` for `v` over F_q. Publishing `C` is as bad as publishing
`pad_seed`. Hiding and short-SIS binding pull in opposite directions WHEN `C` is public
(`path2_descent_sis_hiding_redesign_options.md §Interpretation`).

### 1.2 Resolution — `private_linear_commitments_inside_zk` (commit-and-prove)

**Keep `C = A·v` INSIDE the ZK transcript. It is NEVER a public field.** This is the RECOMMENDED
option (`...redesign_options.md` row 3) and the one matching the already-built public view: the public
object is the **transparent commitment ROOTS only**, not the SIS commitments.

Concretely, the PUBLIC surface is exactly `Path2TransparentDescentPublicViewV1`:

| public field | what it is | why it does not leak |
|---|---|---|
| `masked_projection[6]` | `account_projection[i] + mask[i] mod q` | hidden by the PRF-fresh mask (see §1.3); same object the SPEND path publishes and that `deployable_pad_hiding_VERDICT.md` already cleared |
| `trace_commitment_root` | Merkle/hash root over the committed private trace columns (incl. the SIS commitments `C_role`, `pad_seed`-derived lanes, `account_projection`, masks) | a hash root is binding-only; it reveals nothing about leaves under collision-resistant RO |
| `constraint_commitment_root` | root over the committed constraint/quotient columns | same — root only |
| `query_index`, `trace_rows`, `query_count`, `column_profile` | Fiat-Shamir-derived public parameters | structural, witness-independent |
| leaf / binding / aggregation / transcript digests | context binding (already checked) | digests of public context |

What stays **PRIVATE** (never serialized; lives only as committed trace leaves opened at FS-sampled
query rows under the no-export contract `path2_descent_sis_private_zk_no_export_contract.md`):
`pad_seed`; the PRF masks `mask[i]`; the unmasked `account_projection[i]`; the per-role SIS
commitments `C_role = A_role · v_role`; the witness vectors `v_role` (semantic + blinding + range
lanes); all intermediate trace cells; the per-row blinding randomness.

The hidden objects appear in the proof ONLY as (a) Merkle leaves under the public roots and (b)
openings at the small set of FS-sampled query rows, masked by per-row blinding. The verifier never
receives `C_role` or `v_role` directly; it receives openings sufficient to check the constraints at
the sampled rows and the Merkle authentication paths to the public roots. Binding is exported via the
proof's **knowledge soundness** (extractor §3), not by publishing `C`.

### 1.3 The hiding argument (why the observer cannot recover the witness)

Two independent hiding layers, both reusing already-cleared analysis:

1. **`masked_projection` is hidden by the mask-PRF** — the SAME argument as the spend path. The mask
   `mask[i] = uniform_mod_from_hash(MASK_DOMAIN, pad_seed‖slot_id, …, i)` is PRF-fresh, per-slot,
   per-lane, on a domain byte-frame-disjoint from the pad PRF (82 B vs 154 B input frames). Under the
   SHA-256 frame-PRF / RO assumption the masked lane is computationally indistinguishable from a
   uniform F_q residue. This is `deployable_pad_hiding_VERDICT.md` CLAIM 2 + the proven
   `frame_prf_advantage` lemma (`|Pr[Real]−Pr[Ideal]| ≤ q·2⁻ᴴ`, H = 256), already discharged
   classically to {RO + `pad_seed` min-entropy}. **No NEW hiding assumption for the masked lanes.**

2. **The trace/constraint commitments are hidden by the transparent-ZK transcript.** The commitment
   roots are collision-resistant-hash roots (reveal nothing about leaves); the openings are at a small
   FS-sampled query set and are blinded per row (the column profile carries `mask_domain_id` /
   `visibility_class`, and `verify_path2_transparent_query_opening_manifest` already rejects an
   unmasked hidden role — `"hidden role is not masked"`). The honest-verifier zero-knowledge
   simulator (the `q_transparent_simulator` budget symbol) produces an identically-distributed
   transcript from the public view alone (masked lanes ← uniform residues under layer 1; opened query
   rows ← simulated from the blinding distribution), so the transcript carries no extractable witness
   information. This is exactly route `private_linear_commitments_inside_zk`: hiding is delegated to
   the simulator.

**Observer-cannot-recover claim.** Given the public view, an observer sees only masked residues
(layer 1) and hash roots + a simulatable query transcript (layer 2). Recovering any of
{`pad_seed`, `account_projection`, `v_role`} from this requires either (a) inverting the mask-PRF
(breaks SHA-256 frame-PRF / `pad_seed` min-entropy) or (b) distinguishing the real transcript from the
simulator's (breaks the transparent-ZK HVZK property = collision-resistance of the commitment hash).
Both are named/standard. There is no full-column-rank linear system to solve, because `C` is private.
The rank blocker is **resolved cleanly: YES.**

### 1.4 Why not the other options

- `small_noise_computational_hiding` (publish a noisy `C`): coherent but adds a NEW named module-LWE-
  style hiding assumption beyond the assumption set — rejected (we keep the assumption surface to
  {RO frame-PRF, hash CR, the existing SIS binding, coded-SIS external}).
- `statistically_hiding_public_kernel`: broad randomness hides but weakens short-SIS binding; needs a
  dual-mode theorem — rejected as heavier and binding-fragile.
- `dual_commitment_hiding_plus_binding`: works but largest bytes — fallback only.

The chosen route adds **no** new hiding assumption (reuses spend-path mask-PRF + transparent-ZK
HVZK/CR), which is the decisive reason to prefer it.

---

## 2. PROOF STRUCTURE

### 2.1 The relation to arithmetize

The statement is the masked-state18 OPENING relation that
`verify_path2_direct_creation_masked_state18_proof_v1` (`:26611`) checks today over PUBLIC
`pad_seed`, restated as a relation over a PRIVATE trace. Per
`verify_path2_prevalidated_slot_creation_relation_replay` (`:26423`) /
`creation_time_relation_validity_VERDICT.md`, the relation is **fully determined by the leaf-fixed
commitment + `pad_seed`** — there is no free prover slot — which is exactly why a ZK proof of it is
sound. The constraints (one private trace per creation):

```
R_masked_state18(public: masked_projection[6], leaf.pad_seed_digest, binding.packed_witness_commitment,
                          binding.pad_source_commitment, binding.projection_relation_commitment,
                          slot_id ;
                 private: pad_seed, account_projection[6], mask[6], masked_proj_residues[18], … ):

 (C1) pad-seed binding:      reviewed_fq12_pad_seed_opening_digest(pad_seed) == leaf.pad_seed_digest
 (C2) mask derivation:       mask[i]  == reviewed_fq12_packed_witness_state18_projection_masks(pad_seed,slot_id)[i]
 (C3) opening derivation:    account_projection[i] == (masked_proj_residues[12+i] - mask[i]) mod q   (lanes 12..17)
 (C4) repack equality:       reviewed_fq12_pack_masked_witness_state18_from_parts(syndrome, account_projection,
                                pad_seed, slot_id) == binding.packed_witness_commitment       (fail-closed)
 (C5) mask equation:         masked_projection[i] == (account_projection[i] + mask[i]) mod q          (public lhs)
 (C6) commitment roles:      pad_source_commitment   == commit(PAD_SOURCE, masked_projection)
                             projection_relation_commitment == commit(PROJECTION_RELATION, account_projection)
 (C7) range/norm:            account_projection[i], mask[i] in canonical [0,q); base-128 hybrid range
                             witnesses bound the SIS witness lanes (path2_descent_sis_norm_range_surface.md)
```

C1–C6 are the exact checks of `verify_path2_direct_creation_masked_state18_proof_v1` steps 2–6; C7 is
the base-128 hybrid range policy (`PATH2_NATIVE_BOUNDARY_TRACE_RANGE_HYBRID_BASE128`, already the
declared `norm_range_mode` of the bound interface). The arithmetization commits these as polynomial
constraints over the trace rows; the role layout is already typed
(`Path2TransparentRelationLayoutV1`, `Path2TransparentRelationRoleLayoutV1` —
`semantic_residues / base128_digits / range_witnesses` per role).

### 2.2 Commitment scheme (transparent)

**Hash/Merkle-committed trace with FRI-style low-degree / query argument** — transparent (no trusted
setup), standard RO/hash assumption only:

- **`trace_commitment_root`** = Merkle root (SHA-256) over the private trace columns (the witness +
  intermediate cells laid out per `Path2TransparentRelationLayoutV1`). One leaf per (row, column
  group), per-row blinded.
- **`constraint_commitment_root`** = Merkle root over the constraint/quotient columns (the
  evaluations witnessing C1–C7 vanish on the trace domain). FRI-style low-degree test proves the
  quotient is a genuine low-degree polynomial ⇒ the constraints hold on the whole domain, not just at
  the queried rows.
- **Fiat-Shamir.** `query_index` / the sampled query rows are derived from
  `transparent_transcript_digest` (already computed by `path2_transparent_descent_transcript_digest`
  over proof_system_id ‖ relation_id ‖ trace_rows ‖ query_count ‖ column_profile ‖ leaf/binding/
  aggregation digests ‖ masked_projection ‖ both roots ‖ query_index). The transcript binds the roots
  BEFORE the queries are sampled — so the prover cannot adapt the trace to the queries.

### 2.3 The query argument (what the verifier runs at the stubbed step)

At `verify_path2_direct_creation_transparent_descent_selected_view_bridge` (currently returns "not
implemented" after the bindings), the implemented relation check is:

1. Re-derive the FS query set from `transparent_transcript_digest` and confirm it matches
   `view.query_index` / the manifest's `records` (already half-checked by
   `verify_path2_transparent_query_opening_manifest_for_public_view`).
2. For each query row: verify the Merkle authentication path of each opened trace/constraint leaf to
   `trace_commitment_root` / `constraint_commitment_root` (the `merkle_paths_digest` already binds the
   path set).
3. Evaluate constraints C1–C7 at the opened rows over F_q (the consistency messages, bound by
   `consistency_messages_digest`): each opened row must satisfy the local algebraic constraint and the
   base-128 range relations.
4. Run the FRI low-degree verification on the constraint/quotient openings (folding rounds bound in
   the proof bytes; `q_transparent_merkle_open` budget).
5. Confirm C5 (`masked_projection[i] == account_projection_opened[i] + mask_opened[i] mod q`) ties the
   PRIVATE opened lanes to the PUBLIC `masked_projection` lanes — this is the seam that makes the
   public masked lanes the image of the proven private witness.
6. On all-pass: `Ok(())`. The "not implemented" sentinel is removed.

Soundness error ≈ `(1 − δ)^query_count + FRI_error`, tuned by `query_count` to ≤ 2⁻¹²⁸ (the
query-budget already reserves `q_transparent_query_row` / `q_transparent_consistency` for this).

---

## 3. SOUNDNESS

### 3.1 Re-clearing relation-validity through a proof-mediated extractor

`creation_time_relation_validity_VERDICT.md` PROVED the relation arm SOUND for the **harness-replay**
path: there is no relation-invalid opening, because `account_projection` is DERIVED from the leaf-fixed
commitment (not a free prover value), and any mismatch fail-closes via the repack-equality assert +
the Export classifier. HIGH-2's leak made the public verifier's soundness *trivial* (it recomputed
everything from the published `pad_seed`). The hiding proof replaces recomputation with a query
argument, so soundness must be **re-established as knowledge soundness**:

> **Obligation `p2_sis_extractor` (`path2_descent_sis_feasibility.md:66`, still open).** From any
> accepting transparent-descent transcript, an extractor (rewinding the FS oracle / using the FRI
> decommitments) recovers a witness `(pad_seed*, account_projection*, mask*, v_role*)` that satisfies
> C1–C7 against the public `(leaf, binding, masked_projection)` — i.e., a VALID hidden opening — OR
> exports a named break (`path2_sis_binding_break`: a short kernel vector `Δ = v0 − v1`, `A·Δ = 0`,
> from a double opening; or a hash collision on a commitment root).

Because C1–C7 are exactly the relation `creation_time_relation_validity_VERDICT.md` already showed has
no relation-invalid satisfying assignment, the extracted witness is automatically relation-VALID.
So relation-validity is re-cleared by composition: **(extractor yields a satisfying assignment) ∘
(every satisfying assignment is relation-valid, already PROVEN) ⇒ accepting transcript ⇒ valid hidden
creation, or a named break.** This is the descent shape "accepts ⇒ glues (valid hidden opening) OR a
named binding/soundness break."

### 3.2 New assumption added by the transparent commitment

**One, and it is standard: the random-oracle / collision-resistant-hash model for the FRI/Merkle
transparent commitment (SHA-256-as-RO).** Specifically:

- Merkle/commitment binding ⇐ SHA-256 collision-resistance — ALREADY in the assumption set (dossier
  §3(b)2, role-root binding C2).
- Fiat-Shamir soundness of the query argument ⇐ SHA-256-as-RO (QROM for quantum) — the SAME RO
  primitive already carried for the action map (§3(b)3) and the frame-PRF (§3(b)4). The QROM
  Fiat-Shamir loss for the round-by-round transparent argument is the standard quadratic O2H term,
  already budgeted (`qrom_quadratic_loss_bits_at_deployment_bound = 32`,
  `path2_descent_sis_private_zk_qrom_frame.md`).
- HVZK of the transparent transcript ⇐ collision-resistance + the per-row blinding distribution — no
  new primitive.

**No new hardness assumption beyond the RO/hash already carried.** The hiding layer adds nothing
(reuses the cleared mask-PRF). The binding layer reuses the existing SIS binding + hash CR. **Net new
assumption surface vs the current dossier: zero new *classes*; the transparent FS argument is a new
*usage* of the already-carried SHA-256-as-(Q)RO assumption.** This is the cleanest possible outcome
and the decisive advantage of the descent-framed route over a generic ZK bolt-on.

### 3.3 What the extractor obligation still requires (honest residual)

`p2_sis_extractor` is **not yet proven** — it is the genuine multi-week cryptographic content of this
fix (the analog of the spend-path's per-event reductions). It is a standard but non-trivial knowledge-
soundness/extraction argument for a FRI-style argument over this specific relation. It is *scoped and
named*, not hand-waved: see §5 step 6.

---

## 4. COST

Estimates from `path2_descent_sis_feasibility.md:50-55` and
`path2_descent_sis_hiding_redesign_options.md:25-28` (LOWER BOUNDS; a real proof adds query
repetition, Merkle/FRI paths, norm-proof overhead, rejection slack):

| band | SIS commitment | base-128 payload floor | verifier mul-adds | notes |
|---|---|---|---|---|
| **48 rows (primary)** | 252 B | **~725–928 B** | **~5,184** | primary feasibility band |
| **64 rows (primary)** | 336 B | **~893–1,180 B** | **~10,368** | primary feasibility band |
| 96 rows (fallback) | 504 B | ~1,229 B | ~10,368 | fallback only |

- **Creation payload:** target **~700 B – 1.2 KB** (48-row band lands ~725–928 B; 64-row ~893–1180 B).
  Realistically budget the upper end (~1.0–1.3 KB) once FRI folding rounds + Merkle paths + query
  repetition for 128-bit soundness are added. vs the current ~38 B leaky struct (`pad_seed` 32 B + 6×
  u16 = 12 B). **The leak fix costs ~+0.7–1.3 KB of CREATION payload.**
- **Verifier cost:** ~5k (48-row) to ~10k (64/96-row) F_q mul-adds for the constraint evaluation at
  queried rows, PLUS the FRI low-degree verification (≈ `query_count · log(trace_rows)` hash + field
  ops for Merkle paths and folding) — order ~10⁴–10⁵ hash/field ops total. Dominated by hashing, not
  field arithmetic.
- **Prover cost:** build + low-degree-extend the trace, Merkle-commit two column sets, run FRI
  folding: ~`O(trace_rows · log trace_rows)` field ops + ~`trace_rows · columns` SHA-256 leaf hashes.
  Heavier than the current trivial prover but bounded (small trace; ~hundreds of rows).
- **State delta:** **+32 B** per account-creation leaf (a commitment root or proof anchor digest in
  the slot leaf). Creation-time only.
- **SPEND delta: 0 B.** This is a CREATION-time proof; the spend path (consume-once slot + checked-core
  binding) is untouched. Confirmed by the exploration doc and the query budget (the spend-side symbols
  are unchanged; only `q_transparent_*` creation symbols are added, already active).

---

## 5. IMPLEMENTATION PLAN (discrete buildable steps, dependency order, green checkpoints)

Each step keeps the tree GREEN: the boundary verifiers KEEP returning the "not implemented" sentinel
until the FINAL step flips it, so existing tests that assert the sentinel stay valid until the last
checkpoint (where those specific assertions are updated in lockstep). Build the prover behind a NEW,
NON-DEFAULT entry point; do NOT touch `production_deployment_preflight` (stays fail-closed).

1. **Relation arithmetization (pure, testable in isolation).** Implement
   `path2_transparent_relation_layout_v1()` population for C1–C7 into the existing
   `Path2TransparentRelationLayoutV1` / role layouts (it is already a typed struct + digest). Add a
   `path2_masked_state18_relation_trace_v1(pad_seed, slot_id, binding, masked_projection) -> Trace`
   builder that produces the private trace and asserts C1–C7 hold for honest inputs. *Checkpoint:* a
   unit test that the honest fixture's trace satisfies every constraint; sentinel still returned by all
   boundaries. GREEN.

2. **Trace commitment (Merkle).** Implement `path2_transparent_trace_commit_v1(Trace) ->
   (trace_commitment_root, leaves)` and `..._constraint_commit_v1` over SHA-256 with per-row blinding;
   wire into the existing `path2_transparent_commitment_api_v1` consistency the prover side that
   PRODUCES the roots the verifier already checks. *Checkpoint:* roundtrip test (prover root ==
   the value the existing `path2_transparent_commitment_api_v1` binds). GREEN.

3. **Query manifest + opening generation (prover).** Implement the prover that derives the FS query
   set from `path2_transparent_descent_transcript_digest`, opens the trace/constraint leaves at those
   rows, and produces `Path2TransparentQueryOpeningManifestV1` + the Merkle paths. The verifier side
   (`verify_path2_transparent_query_opening_manifest_for_public_view`,
   `verify_path2_transparent_proof_bytes_envelope_v1`) is ALREADY built — this step makes a manifest
   those accept. *Checkpoint:* generated manifest passes the existing manifest/envelope verifiers;
   boundaries still return sentinel (they call those checks then sentinel). GREEN.

4. **Constraint evaluation at queried rows (verifier core).** Implement the per-row C1–C7 evaluation
   + the C5 public/private seam check as a private helper
   `path2_transparent_relation_query_check_v1(view, manifest, openings) -> Result<(),String>`. Do NOT
   yet call it from the boundary (keep sentinel). *Checkpoint:* unit test it accepts honest openings
   and rejects each mutated constraint; sentinel unchanged. GREEN.

5. **FRI low-degree argument.** Implement `path2_transparent_fri_verify_v1` (folding-round check over
   the constraint openings in `proof_bytes`) + the matching prover folding. *Checkpoint:* FRI accepts
   honest, rejects a non-low-degree constraint column; sentinel unchanged. GREEN.

6. **Flip the sentinel (the soundness-bearing step).** Replace the
   `Err("…not implemented")` tail in `verify_path2_direct_creation_transparent_descent_selected_view_bridge`
   (and the `_payload_boundary*` wrappers) with: call step-4 + step-5 checks; on all-pass `Ok(())`. In
   the SAME change, update the existing tests that assert the sentinel string to assert ACCEPT on an
   honest transparent payload and REJECT on the negative-control matrix (mutated root, mutated opened
   lane, wrong query set, wrong masked lane, non-low-degree column). Add the end-to-end
   prover→verifier round-trip test + the `p2_sis_extractor` knowledge-soundness obligation as a
   documented (initially `#[ignore]`-able or research-doc-tracked) gap if the full extraction proof is
   not yet machine-checked. *Checkpoint:* full `cargo test` GREEN with the flipped semantics; preflight
   STILL fail-closed (unchanged); KAT unaffected (creation-proof path, not the signed-bytes KAT).

7. **(Parallel, research) Discharge `p2_sis_extractor` + re-run leakage/sampler checks.** Prove (or
   externally scope) the extractor/knowledge-soundness obligation; re-run the public-view leakage
   allowlist (`path2_public_view_no_export_allowlist_v1.md`), sampler-conditioning, and the
   response-link-coded-SIS checks against the NEW public surface (commitment roots + masked lanes).
   This is the cryptographic-content milestone, not a code checkpoint.

Steps 1–5 are additive and cannot regress existing behavior (sentinel preserved). Step 6 is the single
behavior-flipping commit, with its test updates in lockstep — mirroring the project's marker-gate
discipline. Step 7 is the genuine multi-week proof effort.

---

## 6. RISKS / OPEN QUESTIONS (maintainer sign-off)

1. **`p2_sis_extractor` is unproven (PRIMARY).** The knowledge-soundness/extraction argument (§3.1,
   §3.3) is the real cryptographic content and is multi-week. The code can be built and pass
   round-trip + negative-control tests WITHOUT it, but **HIGH-2 is not truly resolved until the
   extractor obligation is discharged or externally accepted** — otherwise the proof is "accepts honest,
   rejects the controls we tried," not "knowledge-sound." Sign-off needed: is a scoped+named extractor
   obligation (like the existing carried assumptions) acceptable for the testnet rung, with full
   discharge gated before value-capped mainnet?

2. **New public surface → coded-SIS estimator risk (NEEDS SIGN-OFF).** The new public view exposes
   `trace_commitment_root` + `constraint_commitment_root` + `masked_projection` over the SAME role
   matrices used by the response-link-coded-SIS problem (the single external residual, dossier §3(c)).
   A descent-native creation proof MUST NOT hand the external estimator a cleaner attack on that coded
   problem (`high2_descent_native_hiding_exploration.md §4`, `ws3_concrete_security_findings.md:60-70`).
   The roots are hash digests (should leak nothing), but the masked lanes + the existence of openings
   are a new surface the external estimator scope (§6 of the dossier) must be widened to cover. **This
   is the item most likely to need maintainer + external-reviewer sign-off.**

3. **Byte cost could exceed the ~700 B–1.2 KB estimate.** The figures are LOWER BOUNDS excluding query
   repetition, FRI folding rounds, and norm-proof overhead for full 128-bit soundness. A realistic
   proof may land 1.3–1.8 KB at the 64-row band. If the creation payload must stay under ~1 KB, the
   48-row band + aggressive FRI parameters are required, which tightens the soundness/byte trade — a
   parameter decision the maintainer should fix BEFORE step 5.

4. **Base-128 range policy (C7) is itself a proof obligation.** A pure linear relation over q12289 is
   insufficient — large modular witnesses could satisfy C1–C6 while escaping hidden-source semantics
   (`path2_descent_sis_commitment_viability.md:189-194`). The base-128 hybrid range witnesses must be
   correctly arithmetized and their soundness argued; getting this wrong forces a redesign of the trace
   layout and adds bytes.

5. **Sampler-conditioning under the exact new public view.** An earlier public-link branch failed
   because public values conditioned the hidden sampler too strongly
   (`...commitment_viability.md:83-91`). Hiding must be re-verified under the EXACT public view
   `leaf + creation_proof_binding_digest + commitment roots + masked_projection + transcript`. If the
   roots/lanes condition the sampler, hiding could degrade — low-probability given hash roots, but it
   must be re-run (step 7).

6. **FRI/QROM Fiat-Shamir loss.** The quantum Fiat-Shamir soundness of the round-by-round transparent
   argument carries the standard O2H quadratic loss (budgeted at 32 bits,
   `path2_descent_sis_private_zk_qrom_frame.md`). If the chosen `query_count` / folding schedule pushes
   the QROM-adjusted soundness below 128 bits, more queries (more bytes) are required — interacts with
   risk 3.

---

## SUMMARY (for the maintainer)

- **Rank blocker resolvable cleanly? YES.** Mechanism: `private_linear_commitments_inside_zk` —
  keep `C = A·v` PRIVATE inside the transparent transcript; publish ONLY the commitment ROOTS +
  PRF-masked projection lanes (exactly the already-built `Path2TransparentDescentPublicViewV1`).
  Hiding = spend-path mask-PRF (already cleared) + transparent-ZK HVZK; no public linear system to
  solve, so no rank leak.
- **New assumption? Effectively NONE NEW IN CLASS.** Adds only a new *usage* of the already-carried
  SHA-256-as-(Q)RO / collision-resistance assumption (Merkle binding + FRI Fiat-Shamir). No new
  hardness class beyond {RO/hash, existing SIS binding, the existing external coded-SIS}.
- **Cost:** creation payload ~0.7–1.3 KB (vs ~38 B leaky); verifier ~5k–10k mul-adds + FRI hashing;
  state +32 B; **SPEND delta 0**.
- **Risk needing sign-off:** (a) the `p2_sis_extractor` knowledge-soundness obligation is the genuine
  unproven cryptographic content (HIGH-2 not truly closed until discharged/accepted); (b) the new
  public surface (roots + masked lanes over the coded-SIS role matrices) must be added to the external
  estimator scope — most likely to require external-reviewer sign-off.
