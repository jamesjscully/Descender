# HIGH-2: Can the Descent Framework Provide Creation-Time Witness-Hiding Natively?

Status: READ-ONLY exploration. No code edits. Recommendation for maintainer decision.

Question (from maintainer): The public creation proof `Path2MaskedState18CreationProofV1`
(`tools/parameter-search/src/production_reference.rs:1484`) serializes `pad_seed` in the
clear, so any observer recovers the full witness key-free (red-team HIGH-2). The generic fix
is a transparent-descent ZK proof (unimplemented). Can the system's CORE DESCENT FRAMEWORK
hide the witness NATIVELY instead, reusing the masking structure the spend path already uses?

---

## 1. What "descent" IS in this system

Descent here is the project's organizing proof shape, not (yet) a deployed cryptographic
primitive. It is a quotient/fiber gluing frame, modeled abstractly in the Lean sidecar and
described in the research notes:

```text
public verifier object  ->  public quotient/fiber  ->  local representative checks
  ->  gluing to a hidden account-backed state, OR a named binding/soundness/hiding break
```

The geometric dictionary (`research/path2_geometric_descent_lens.md:11-23`) pins the meaning:

```text
public account object = global descended object / quotient class
private signer state   = local representative / trivialization / gauge choice
validity               = vanishing obstruction to gluing
privacy                = nontrivial fiber over the public object
soundness              = all points in the public fiber have the same safe meaning
```

The Lean abstractions that carry this shape (`research/path2_descent_sis_commitment_viability.md:27-36`):
`Descender.DescentInterface.LocalQuotientDescent`, `EffectiveDescentCertificate`,
`HiddenCommittedQuotientLink.Interface`, `accepts_glues_or_breaks_of_effectiveDescent`.
These provide the WRAPPER (local-valid implies glues-or-breaks) but DO NOT instantiate a
cryptographic commitment.

The intended cryptographic instantiation is `Path2DescentSisCommitmentV1`
(`path2_descent_sis_commitment_viability.md`, `path2_descent_sis_feasibility.md`): an
SIS / module-SIS commitment over role-separated witness vectors. The public quotient is the
masked projection relation:

```text
masked_projection_i = pad_lane_i + account_projection_i  (mod q),   q = 12289, 6 lanes
```

drawn from lanes 12..17 of the canonical 18-residue packed witness commitment. Committed
roles are `pad_source`, `projection_relation`, `opening_tuple` (and an aggregate
`transcript_commitment`), each as `C = A_role * v mod q` with semantic + blinding + range
lanes (`path2_descent_sis_binding_surface.md:37-42`). Binding is the
`path2_sis_binding_break` game: a double opening yields a short kernel vector
`delta = v0 - v1, A*delta = 0 mod q` (`path2_descent_sis_binding_surface.md:14-34`).

So "descent" relates witness to statement by: the witness is a local representative `h` in a
fiber; the public statement is the descended quotient class (the masked projection plus
SIS commitment roots); validity is that the local relation glues; and soundness is that the
fiber is safety-saturated (every representative has the same spend meaning).

---

## 2. The key question: is the leak just a lazy shortcut?

YES — the leak is an artifact of the masked-state18 proof choosing a non-ZK shortcut. The
verifier `verify_path2_direct_creation_masked_state18_proof_v1`
(`production_reference.rs:26611-26687`) makes the data-availability choice explicit: it takes
`pad_seed` as a SERIALIZED public field (struct at `:1484-1487`), binds it to
`leaf.pad_seed_digest` (`:26635`), then re-derives `account_projection` and the PRF masks
and replays the harness opening relation (`:26644-26684`). The harness itself holds
`pad_seed` as a HIDDEN input and binds it the same way — the verifier comment at `:26632-26634`
states the public path differs ONLY by "the data-availability step that makes the relation
checkable from public data." Publishing `pad_seed` is therefore a deliberate lazy/non-ZK
shortcut, not an intrinsic requirement of the relation.

And the witness-hiding structure already exists. The spend path hides the same pad via a
PRF-fresh, per-slot per-lane, domain-separated mask (`deployable_pad_hiding_VERDICT.md`):
`masked_projection[idx] = (account_projection[idx] + mask[idx]) mod q`, where
`mask = uniform_mod_from_hash(MASK_DOMAIN, pad_seed||slot_id, ...)` on a domain byte-disjoint
from the pad PRF (`production_reference.rs:6376-6394`, mask domain `:242` vs pad domain `:237`).
The on-chain spend surface is digest/masked-only; the simulator may swap the masked residues
for uniform residues under the mask-PRF assumption. Pad-hiding pre-spend reduces to one named
assumption (`A_sha256_projection_mask_frame_prf`) plus pad-seed single-use.

Critically, the descent-native vehicle for this ALREADY EXISTS as a struct, just unwired:
`Path2TransparentDescentPublicViewV1` (`production_reference.rs:1499-1516`) carries
`masked_projection` + `trace_commitment_root` + `constraint_commitment_root` +
leaf/binding/aggregation digests — and NO `pad_seed`. It is the masked-state18 public view
with the cleartext witness removed and replaced by commitment roots. The verifier
`verify_path2_direct_creation_transparent_descent_selected_view_bridge`
(`:26708-26771`) already checks every binding (leaf context, binding digest, aggregation
context, column profile, transcript digest, commitment-API consistency) — then the actual
relation/opening check is stubbed: `verify_path2_direct_creation_transparent_descent_payload_boundary`
returns "Path 2 transparent descent proof verifier is not implemented" (`:26705`), and the
dispatcher rejects the kind at `:26555-26556`.

So the relation, the masking, and the public-view plumbing are all present and consistent.
What is missing is exactly the ZK opening proof that verifies the relation against the
commitments WITHOUT revealing `pad_seed`.

---

## 3. Feasibility

FEASIBLE: MAYBE (architecturally yes; cryptographically it converges on the same ZK proof).

The descent-native idea is sound and matches the project thesis, but the research has already
hit the wall that determines the answer: **a public linear SIS commitment `C = A*v` is NOT
hiding in the proposed dimensions.** The rank/hiding probe
(`path2_descent_sis_rank_hiding_probe.md`) found the main role matrices are full COLUMN rank
for the 48/64-row bands:

```text
pad_source / projection_relation:  rows 48|64, cols 30, rank 30, kernel 0  -> opening recoverable
opening_tuple:                     rows 48|64, cols 48, rank 48, kernel 0  -> opening recoverable
```

i.e. publishing `C = A*v` lets an observer solve for `v` over F_q. That defeats hiding just as
badly as publishing `pad_seed`. This is a hiding/design failure, not a binding failure.

The redesign options (`path2_descent_sis_hiding_redesign_options.md`) enumerate the only ways
out, and the recommended one is telling:

- `private_linear_commitments_inside_zk` (RECOMMENDED): keep `C = A*v` PRIVATE inside a ZK
  proof transcript; binding is exported via the proof's knowledge soundness. "Best preserves
  the selected Path 2 goal" — but "requires a real ZK payload proof rather than treating C=A*v
  as the public proof object."
- `small_noise_computational_hiding`: an LWE/module-LWE-style noisy commitment — coherent but
  adds a NEW named hiding assumption beyond SIS.
- `statistically_hiding_public_kernel`: broad randomness hides but weakens short-SIS binding;
  needs a dual-mode theorem.
- `dual_commitment_hiding_plus_binding`: works but largest bytes.

The conclusion the notes reach: if commitments are public, hiding and short-SIS binding pull
in opposite directions. The cleanest fix is to put the commitments inside a ZK transcript —
which is the transparent-descent ZK proof.

### Mechanism sketch (descent-native hiding creation proof)

The honest signer holds `pad_seed`; derives `account_projection` and PRF masks; forms
`masked_projection = account_projection + mask mod q`. PUBLIC: the
`Path2TransparentDescentPublicViewV1` — `masked_projection` lanes + `trace_commitment_root` +
`constraint_commitment_root` + leaf/binding/aggregation digests (NO `pad_seed`). The prover
arithmetizes the masked-state18 opening relation as a trace (the same relation
`verify_path2_direct_creation_masked_state18_proof_v1` checks at `:26644-26684`: pad-seed ->
masks -> account_projection -> repack == commitment, plus the mask equation), commits the
trace + constraint columns to the roots, and emits a transparent (hash/FRI-style) query proof.
VERIFIER checks: all bindings (already implemented at `:26716-26757`), then the stubbed step —
that the committed trace satisfies the masked-state18 constraints at the Fiat-Shamir-sampled
query rows and the openings are range-bounded — WITHOUT ever seeing `pad_seed`. Hiding comes
from the PRF-fresh mask on the public `masked_projection` (the spend-path argument) plus the
ZK property of the transcript over the private trace columns. Binding/soundness come from the
trace-commitment binding + the relation constraints (extractor recovers a valid hidden opening
or exports a named break). This is precisely route `private_linear_commitments_inside_zk` /
the transparent-descent proof — i.e., a descent-FRAMED ZK proof, not a separate generic ZK
system bolted onto an unrelated circuit.

### Cost vs the generic ZK bolt-on

There is no meaningful cost difference, because the descent-native route IS a ZK proof — it
just reuses the project's own relation, matrix domains, and public-view plumbing instead of an
arbitrary R1CS/circuit. Lower-bound payload estimates for the SIS-commitment bands
(`path2_descent_sis_feasibility.md:50-55`, lower bounds excluding rejection slack, norm-proof
overhead, repetition, extractor machinery):

```text
rows 48: commitment 252 B, payload floor ~725 B    (primary feasibility band)
rows 64: commitment 336 B, payload floor ~893 B    (primary feasibility band)
rows 96: commitment 504 B, payload floor ~1229 B   (fallback)
verifier mul-adds: 5184 (48 rows) .. 10368 (96 rows)
```

The transparent base-128 hybrid relation floors are similar (~928-1180 B at 48/64 rows,
`path2_descent_sis_hiding_redesign_options.md:25-28`). These are LOWER BOUNDS; a real proof
adds query repetition, Merkle/FRI paths, and norm-proof overhead. State leaf delta is +32 B;
spend signature delta is 0 B (creation-time only). The honest comparison: descent-native and
generic-ZK land in the same hundreds-of-bytes-to-~1.2 KB creation-payload band; the leak fix
costs roughly +700 B-1.2 KB of creation payload either way, vs the current ~38 B
(`pad_seed` 32 B + 6 projection u16 = 12 B) leaky struct.

The genuine WIN of descent-native is NOT bytes or compute. It is that every theorem obligation
is a structural statement over the project's own objects (SIS binding on the registered role
matrices, mask-PRF hiding already analyzed for spend, extractor over the masked-state18
relation) rather than generic ZK claims over an arbitrary witness circuit
(`path2_native_descent_non_zk_alternatives.md:224-236`). It reuses the already-cleared
mask-PRF hiding analysis and the existing public-view/binding plumbing.

---

## 4. Risks

- **Binding-vs-hiding tension (PRIMARY).** Confirmed live blocker
  (`path2_descent_sis_rank_hiding_probe.md`): public `C = A*v` is full-column-rank and reveals
  `v`. Any descent-native design MUST resolve this — either keep commitments private inside the
  ZK transcript (recommended, = the ZK proof) or adopt a noisy/statistically-hiding commitment
  (new assumption). Naively "reusing the SIS commitment as the public object" reintroduces a
  witness leak equivalent to the `pad_seed` leak.
- **Relation-validity / soundness must be re-cleared.** Today HIGH-2's leak makes soundness
  trivial (verifier recomputes everything). A hiding proof replaces recomputation with a
  query/opening argument, so the cleared `creation_time_relation_validity_VERDICT` and
  `creation_time_binding` results must be re-established for the proof-mediated check: an
  accepting transcript must yield an extractor for a valid hidden opening or a named break
  (`path2_descent_sis_feasibility.md:66`, the still-open `p2_sis_extractor`).
- **Norm/range escape.** A linear relation over q12289 alone is insufficient; large modular
  witnesses could satisfy the relation while escaping hidden-source semantics
  (`path2_descent_sis_commitment_viability.md:189-194`). The base-128 hybrid range policy is
  the intended fix but is itself a proof obligation.
- **Response-link-coded SIS interaction.** The accepted-spend forgery bound rests on the
  "response-link-coded" hardness assumption, not the bare quotient lattice
  (`ws3_concrete_security_findings.md:60-70`, `ws7_reduction_scoping.md:99`,
  `SECURITY_DOSSIER.md:370`). A descent-native creation proof publishing trace/constraint
  commitments over the same role matrices must NOT hand an external estimator a cleaner attack
  on that coded problem; the public-view leakage allowlist
  (`path2_public_view_no_export_allowlist_v1.md`) and sampler-conditioning checks must be
  re-run against the new public surface.
- **Sampler conditioning.** The earlier public-link branch failed because public link values
  conditioned the hidden sampler too strongly
  (`path2_descent_sis_commitment_viability.md:83-91,196-204`). Hiding must hold under the exact
  public view `leaf + creation_proof_binding_digest + commitments + transcript`.
- **Carried mask-PRF assumption persists.** Hiding still reduces to
  `A_sha256_projection_mask_frame_prf` + pad-seed single-use (same as spend). Descent-native
  does not remove this; it inherits it (which is fine — it is already carried).

---

## Recommendation

Pursue the descent-native route, but understand that it CONVERGES ON the transparent-descent
ZK proof rather than avoiding it. The maintainer's instinct is partly right and partly not:

- RIGHT: the `pad_seed` leak is a lazy non-ZK shortcut, not intrinsic; the relation, the
  PRF-fresh masking that hides the witness, and the leak-free public view
  (`Path2TransparentDescentPublicViewV1`) and all its bindings ALREADY EXIST and are wired —
  only the opening/relation check is stubbed.
- NOT QUITE: there is no "free" descent-native hiding that skips a ZK proof. The research has
  already proven that the obvious shortcut (publish the SIS commitment as the public object)
  re-leaks the witness via full-column-rank recovery
  (`path2_descent_sis_rank_hiding_probe.md`). Every viable hiding option in
  `path2_descent_sis_hiding_redesign_options.md` either keeps the commitment private inside a
  ZK transcript (= the transparent-descent proof) or adds a new computational-hiding
  assumption.

Therefore: do NOT build a separate generic ZK system over an arbitrary circuit. Instead,
finish the transparent-descent proof as a DESCENT-FRAMED ZK proof — arithmetize the existing
masked-state18 opening relation, keep the SIS/trace commitments private inside the transparent
transcript, and discharge the obligations as structural theorems over the project's own
objects (SIS binding on the registered roles, the already-analyzed mask-PRF hiding, the
masked-state18 extractor, the base-128 range policy). This is "descent-native" in every sense
that matters (own relation, own commitments, own assumptions, reuse of the spend-path hiding
analysis and the existing public-view/binding plumbing) while being a real ZK proof where it
must be. Cost is comparable to the generic bolt-on (~700 B-1.2 KB creation payload,
~5k-10k verifier mul-adds, +32 B state, 0 B spend); the advantage is theorem reuse and a
single coherent assumption surface, not byte savings.

Next concrete gate (already pinned in the notes): select ONE hiding commitment shape
(recommend `private_linear_commitments_inside_zk`), then close `p2_sis_norm_range`,
`path2_sis_hardness_estimator`, and `p2_sis_extractor`, and re-run the public-view leakage /
sampler-conditioning / response-link-coded checks against the new public surface.
