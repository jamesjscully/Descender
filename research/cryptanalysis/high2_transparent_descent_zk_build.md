# HIGH-2 — Transparent-Descent ZK Creation Proof: BUILD RESULT

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This is a historical build result for the transparent-descent ZK route. It is
> useful evidence and negative-control context, but the current product path and
> status live in `../../spec/`, `../../decisions/`, and `../../TRACKER.md`.

**Date:** 2026-06-10 · **n=32, q=12289** · 6 projection lanes
**Status:** PARTIAL BUILD, PUBLIC-DISABLED. The transparent-descent masked-share
prover and incomplete debug core are implemented and tested, but the public
proof-bytes verifier now fail-closes because the core does not enforce the full
hidden relation. 2026-06-11 update: the original scalar-blind opening leaked because it
serialized the blind next to blinded cells; the Rust reference now uses masked
shares and the in-clear-blind recovery regression is blocked. The
full hidden-relation backend certificate (including C1/C2/C4/C6),
knowledge-soundness extractor (`p2_sis_extractor`), and hidden
range-proof/range-extractor obligations remain **NAMED, not-yet-discharged
obligations**
(testnet-acceptable; discharge before value-capped mainnet). Design spec:
`high2_transparent_descent_zk_design.md`.

All code is in `tools/parameter-search/src/production_reference.rs`.

---

## 1. What was built (the 6 steps)

The design's 6 additive green-checkpointed steps were implemented as a single
self-contained transparent (hash/Merkle/FRI-style, NO trusted setup) proof module
plus the sentinel flip:

1. **Relation arithmetization (C1–C7).** `path2_transparent_zk_build_trace`
   builds the PRIVATE trace from `(pad_seed, slot_id)` + the leaf-fixed
   commitment. It replays the harness masked-state18 opening
   (`reviewed_fq12_masked_state18_packed_witness_opening`, which fail-closes on the
   repack-equality assert = C4), derives the PRF masks (C2), the masked residues
   and account projection (C3/C5), the role commitments (C6), and the base-128
   range digits (C7). It asserts the per-row constraint residual is 0 for honest
   inputs (fail-closed otherwise). Field q=12289.

2. **Trace commitment (Merkle).** `path2_transparent_zk_merkle_levels` builds a
   SHA-256 Merkle tree over per-row leaves (`path2_transparent_zk_trace_leaf`,
   committing relation-preserving masked-share cells + a per-row nonce). The root IS the public view's
   `trace_commitment_root`. A second Merkle tree over the constraint column
   (`path2_transparent_zk_constraint_leaf`) yields `constraint_commitment_root`.

3. **Query/opening generation (Fiat-Shamir).** The query rows are derived from
   `transparent_transcript_digest` via the already-built
   `path2_transparent_query_row_index` (binds the roots BEFORE the queries are
   sampled). The prover opens trace + constraint leaves at those rows with their
   Merkle authentication paths. `path2_transparent_zk_build_query_opening_manifest`
   produces a manifest the existing no-export verifier-side plumbing accepts;
   the manifest and typed proof-bytes envelope bind the full transparent
   public-view digest, not only the repeated schedule/profile fields.

4. **Constraint evaluation at queried rows.** The verifier
   (`path2_transparent_descent_verify_zk_opening_v1`) checks affine constraints
   directly on opened masked-share cells and requires the local residual to equal
   the committed constraint value AND be zero. The **C5 public/private seam** is
   checked in masked-share form: `account_share + mask_share` equals the PUBLIC
   `masked_projection` lane. No public deblinding scalar is serialized.

5. **FRI low-degree argument.** `path2_transparent_zk_fri_prove` folds the
   constraint codeword by FS-derived fold challenges, committing each layer by a
   Merkle root; the verifier recomputes the fold challenges, re-derives the FRI
   query positions, authenticates BOTH siblings per layer under the layer roots,
   checks the folding relation layer-to-layer and into the final layer, and
   requires the **final layer be the all-zero codeword** (the constraint
   polynomial is identically zero ⇒ the relation holds on the WHOLE domain).

6. **Sentinel FLIPPED.** The
   `verify_path2_direct_creation_transparent_descent_payload_boundary_with_proof_bytes`
   tail (formerly `Err("…not implemented")`) now calls
   `path2_transparent_descent_verify_zk_opening_v1` after the existing
   binding/envelope checks. This is the single behavior-flipping change; the one
   existing test that asserted the sentinel through this boundary was updated in
   lockstep (it uses a placeholder body and now asserts the real verifier's
   rejection).

The prover entry point is `path2_transparent_descent_prove_creation_v1`
(creation-time, NON-DEFAULT). It returns a full
`Path2TransparentDescentProofPayloadV1` + query-opening manifest + proof bytes.

---

## 2. The hiding test (core HIGH-2)

`path2_transparent_zk_proof_hides_pad_seed_and_witness`:

- **No pad_seed in the bytes.** Asserts `pad_seed` does NOT appear as any byte
  window in the serialized ZK proof bytes, NOR does the full projection-mask
  vector, NOR the unmasked `account_projection` vector. The proof body carries
  only relation-preserving masked-share trace cells, Merkle paths, and the FRI
  transcript.
- **Red-team recovery FAILS on the ZK proof.** The masked-state18 leak relied on
  reading `pad_seed` off the wire to recompute the masks and unmask
  `account_projection`. The ZK proof exposes no such field; the test confirms the
  recovery surface is absent.
- **Contrast preserved.** The OLD non-hiding masked-state18 path STILL leaks
  pad_seed (the existing `redteam_masked_state18_creation_proof_leaks_full_witness_key_free`
  test still passes), and this hiding test re-demonstrates that leak as an
  explicit contrast, proving the two paths genuinely differ.

---

## 3. Round-trip + negatives

- `path2_transparent_zk_creation_proof_round_trips`: the incomplete debug core
  accepts an honest prover-produced proof, while the standalone public opening
  verifier and proof-bytes boundary reject with
  `PATH2_TRANSPARENT_DESCENT_FULL_HIDDEN_RELATION_DISABLED_ERROR`.
- `path2_transparent_zk_creation_proof_negatives_reject`: REJECTS each of —
  tampered trace root, tampered constraint root, tampered public masked lane (C5
  seam break), tampered proof body (a trace cell — body-digest guard / Merkle auth
  fails), tampered FRI (re-wrapped mutated body), and a mismatched query-opening
  manifest. The unmodified proof still passes the incomplete debug core but is
  public-disabled.

The debug core is genuine for the subset it checks: it recomputes the FS
challenges, authenticates every Merkle opening against the public roots,
evaluates the sampled masked-share row relation, checks the C5 seam against the
public masked lanes, and verifies FRI. It is NOT a stub and NOT a
trivially-passing check, but it is not a proof of C1/C2/C4/C6.

---

## 4. Payload size

**Measured creation payload = 14,820 bytes** (trace_rows=256, query_count=16,
FRI 6 layers × 2 query repetitions), under the active fallback threshold
(`PATH2_TRANSPARENT_PROOF_FAMILY_FALLBACK_THRESHOLD_BYTES` = 16,416).

This is ABOVE the design's ~0.7–1.3 KB target. The size is dominated by the 16
mandatory trace-row Merkle openings (each carries two 8-deep SHA-256
authentication paths = 512 B/row ≈ 9.8 KB) plus the FRI query openings. The design
figure was an explicit LOWER BOUND excluding Merkle/FRI path overhead (§4 of the
design doc). The straightforward (uncompacted) encoding here lands at 14.8 KB.
**Follow-up to reach the target:** a compacted opening encoding (shared
authentication frontier across the 16 queries / batched Merkle openings / a Merkle
cap), which is a size optimization only and does not change soundness or hiding.
`PATH2_TRANSPARENT_ZK_FRI_QUERY_COUNT` is tunable; it was set to 2 to stay under
the threshold with the current uncompacted encoding.

- **SPEND delta: 0 B.** This is a CREATION-time proof; the spend path is
  untouched. SPEND signature bytes unchanged (264 executable / 318 carried).
- The new public surface is the already-existing `Path2TransparentDescentPublicViewV1`
  (commitment roots + masked lanes only); no `pad_seed`, no raw witness.

---

## 5. The NAMED extractor obligation (NOT faked)

> **`p2_sis_extractor` — transparent-descent knowledge-soundness extractor —
> NAMED OBLIGATION, testnet-acceptable, discharge before value-capped mainnet.**
> From any accepting transparent-descent transcript, an extractor (rewinding the
> FS oracle / using the FRI decommitments) must recover a witness
> `(pad_seed*, account_projection*, mask*, v_role*)` satisfying C1–C7 against the
> public `(leaf, binding, masked_projection)` — i.e., a VALID hidden opening — OR
> export a named binding break (`path2_sis_binding_break`: a short kernel vector
> from a double opening; or a hash collision on a commitment root).

This obligation is **NOT proven** and is NOT claimed proven. It is the genuine
multi-week cryptographic content of the fix (the analog of the spend-path per-event
reductions). The implemented verifier provides COMPLETENESS + the round-trip +
negative-control matrix, which do NOT depend on the extractor. The full SOUNDNESS
(knowledge-soundness) claim DOES depend on it. Recorded as a named obligation in
the code comment at the sentinel-flip site and in `SECURITY_DOSSIER.md`.

**2026-06-11 blocker-1 progress:** the sampled-row same-representative seam is now
enforced directly in the verifier. For every opened lane the same
`account_share` cell must satisfy both the masked-projection equation and the
base-128 reconstruction equation. This closes the immediate split-witness /
linear-aggregate-cancellation bug (`path2_transparent_zk_opened_row_relation_prevents_split_witness_cancellation`),
but it does **not** discharge `p2_sis_extractor`: the remaining theorem is still
the global accepting-transcript-to-valid-hidden-opening extractor, or a named
binding/hash break.

**2026-06-11 blocker-1 negative result:** the current opening core still does
not enforce the full hidden relation. Regression
`redteam_transparent_zk_core_accepts_public_only_synthetic_trace` builds an
accepting core proof from public `masked_projection` alone. It uses the real
envelope, Merkle paths, sampled-row checks, and FRI verifier, but it does not
prove pad-seed binding, PRF mask derivation, packed-witness repack equality, or
role-commitment consistency. Therefore blocker 1 has **not** cleared and blocker
2 should not be pursued until the opening relation is redesigned to include
those constraints or the proof family is replaced.

**2026-06-12 narrow redesign target:** the missing edge is now named as
`Path2TransparentTraceToBindingRelationLayerV1`. The layer is hashed into the
full-hidden-relation proof contract, the public-input statement, production
obligations, and the proof-family capability. It requires the verifier-visible
claim that trace roots, native role root, creation-binding commitments,
pad-seed/slot PRF masks, masked projection, and ZK witness hiding are all one
consistent hidden-opening relation. The canonical layer remains
`trace_to_binding_relation_implemented = false`; the public verifier still
fails closed until a real backend proves that relation without serializing row
blinds or raw openings.

**2026-06-12 trace-root context binding:** trace and constraint Merkle leaves
now hash a public `TraceBindingContext` digest derived from the proof system,
relation, leaf context, creation-binding digest, matrix transcript, masked
projection, aggregation context, schedule, and query index. This closes the
context-free trace-root surface: an opened row authenticates only under the
public statement it was built for, and the regression checks that the old
anonymous leaf domain no longer recovers the public trace root. This is a real
bridge strengthening, but it is not the full hidden-opening proof; public-only
synthetic traces can still be built for arbitrary public contexts, so the
full-relation subproof gate remains fail-closed.

**2026-06-12 public masked-projection bridge:** the verifier now enforces the
public part of the trace-to-binding seam before the fail-closed backend gate.
`verify_path2_transparent_commitment_api` unpacks the reviewed state18 packed
witness commitment and rejects if its projected masked lanes differ from
`Path2TransparentDescentPublicViewV1.masked_projection`; it also recomputes the
pad-source role commitment from that same public masked projection and rejects
rebinding. This is verifier-safe because it uses only public masked lanes. It
does **not** prove C1/C2/C4/C6: pad-seed opening, PRF mask derivation,
packed-witness repack from private account projection, and role-opening
consistency still belong to the full-hidden-relation backend.

**2026-06-12 backend direction pinned:** the hidden-relation plan now hashes
`Path2TransparentProofFriendlyInnerCommitmentPlanV1`. This prevents the next
backend from silently reverting to a SHA-preimage-heavy proof target or treating
`pad_seed_digest` as proof semantics. The deployable bridge must use
proof-friendly inner commitments for the private pad-source/opening-tuple/
projection-relation roles while retaining SHA only for outer consensus handles.
The canonical plan remains uninstantiated and fail-closed.

**2026-06-12 strengthening:** `research/path2_current_core_negative_theorem.md`
turns that finding into a negative theorem for the current executable core.
`redteam_transparent_zk_core_language_exceeds_hidden_opening_language` now shows
the core accepts a proof for arbitrary public context digests and arbitrary
masked lanes `[1,2,3,4,5,6]` constructed without a pad seed, packed-witness
opening, PRF masks, or role witnesses. The current core language is therefore
strictly larger than the hidden-opening language; a positive blocker-1 proof
requires the separate full-relation subproof backend or a replacement proof
family. Lean also checks the minimal row-equation countermodel as
`current_masked_share_core_acceptance_does_not_imply_full_hidden_opening` and
the minimal digest co-location countermodel as
`digest_colocation_does_not_imply_trace_role_binding`; the combined current-route
verdict is `current_transparent_route_blocker1_negative_witness`.

`research/path2_current_view_binding_negative_theorem.md` adds the composition
counterpart: the existing transcript can preserve the same
`creation_proof_binding_digest` while changing the trace root to a synthetic
public-only trace. Therefore the current two-hop binding does not link the
accepted trace to the binding's role commitments unless the full-relation
subproof proves that link.

**Positive fork pinned:** `research/path2_full_relation_backend_sufficiency_theorem.md`
records the Lean certificate
`SelectedTransparentFullRelationBackendCertificate` and the theorem
`selected_transparent_full_relation_backend_certificate_implies_account_admission`.
Thus a real full-relation backend is sufficient for blocker 1 at the interface
level, but the certificate remains uninstantiated in Rust. The executable anchor
for this fork is
`Path2TransparentFullRelationBackendSufficiencyTargetV1`, whose digest names the
Lean certificate, the sufficiency theorem, the `AccountAdmissionSoundness`
conclusion, the 24-field certificate-obligation checklist, the hidden-relation
plan, and the proof-contract schema.

The relation-layout digest now binds this partial status explicitly:
`sampled_masked_share_relation_enforced = true` and
`full_hidden_relation_constraints_enforced = false`. The layout verifier rejects
a forged claim that the full hidden relation is enforced.

The missing C1/C2/C4/C6 work is also captured as a separate
`Path2TransparentHiddenRelationConstraintPlanV1` digest. The plan marks
pad-seed binding, PRF mask derivation, packed-witness repack equality, and
role-commitment consistency as required and not yet implemented; production
obligations carry that plan digest and reject forged aggregate/full-relation
claims that are not backed by the per-constraint plan.

The next proof object is now explicit as
`Path2TransparentFullHiddenRelationProofContractV1`. It requires a backend that
binds the hidden pad seed to the leaf digest, derives the PRF masks, proves the
packed-witness repack equality, proves role-commitment consistency, and binds
the leaf context, creation binding digest, and public masked projection inputs.
The current canonical contract is `backend_kind = UNINSTANTIATED`,
`backend_instantiated = false`, `verifier_instantiated = false`, and
`full_hidden_relation_subproof_implemented = false`; it carries both the
current-core negative-target digest and the sufficiency-target digest, and
production obligations plus the proof-family capability carry both target
digests and the contract digest. The proof-family capability digest now binds
the actual production-obligations digest bytes, not merely a nonzero marker, so
changing the negative target, theorem target, contract, or obligations changes
the proof-system identity.

The public statement for that future backend is also canonicalized as
`Path2TransparentFullHiddenRelationPublicInputsV1`. The proof-bytes boundary now
constructs and validates this frame, runs the incomplete masked-share core first
so malformed transcripts fail at their real Merkle/FRI/row check, and then calls
`verify_path2_transparent_full_hidden_relation_subproof_v1` against the
canonical public-input statement. Its digest binds the contract/plan digests,
proof system and relation IDs, leaf context, creation binding digest, matrix
transcript, aggregation context, slot id, pad-seed digest, hidden-source
statement digest, all creation role commitments, the native role root, public
masked-projection lanes, trace and constraint roots, column profile, and query
schedule. It contains no `pad_seed` or raw witness bytes.

The backend-facing full-hidden statement, backend verifier result, backend
verifier receipt, certificate payload, full-hidden proof payload, and subproof
envelope now carry relation id and column profile explicitly alongside the
transparent transcript digest, trace schedule, public-view digest, creation
binding digest, all six creation role handles (`packed_witness_commitment`,
`randomizer_commitment`, `pad_source_commitment`,
`opening_tuple_commitment`, `projection_relation_commitment`, and
`gauge_commitment`), the recomputed native role root, masked-projection digest,
and trace-binding context digest. A future backend extractor therefore sees the
exact relation language, column policy, transcript, query schedule, and
role-handle target accepted by the opening verifier, not just an opaque
public-view digest or aggregate role root.

The public opening verifier also validates the full-hidden subproof envelope
against the selected public view before it reaches the fail-closed production
obligation gate. The envelope must match the selected public-view digest,
creation binding digest, native role root, masked-projection digest, and
trace-binding context digest; a subproof envelope for a neighboring view is
rejected as a view mismatch, even if its internal statement/envelope bytes are
otherwise well shaped.

The proof-suite full-hidden binding helper also checks the decoded proof-bytes
envelope against the canonical full-hidden public inputs: relation id, active
relation-layout digest, public-view digest, creation binding digest, native role
root, masked-projection digest, trace-binding context digest, transcript digest,
and schedule fields must match before the embedded full-hidden subproof is
considered.

The inner proof body is now a suite wrapper:
`Descender.Path2.TransparentDescent.ProofSuiteBody.v1` contains the existing
masked-share core body plus a distinct full-hidden-relation subproof segment.
The current prover emits a typed
`Descender.Path2.TransparentDescent.FullHiddenRelationSubproofEnvelope.v1` for
the uninstantiated backend with the typed statement bytes as payload and a zero
local trace-binding proof digest, so the named subproof verifier returns the
fail-closed sentinel after the masked-share core accepts. Any payload that does
not exactly match the statement bytes under the uninstantiated backend must
decode as a typed full-hidden proof payload and pass all shape checks up to the
backend-unimplemented gate; that checked-payload envelope carries the nonzero
local trace-binding proof digest and rejects envelope/payload bridge drift. The backend
certificate payload boundary now has an explicit shape/acceptance split: shape
validation can parse and context-check the typed
`Path2TransparentFullRelationBackendCertificatePayloadV1` carrier,
including the backend proof-system id and backend acceptance-verifier digest,
plus the digest of the backend proof payload claimed to have been verified,
the digest of the backend verifier result over that proof payload,
including per-obligation verification bitmap/proof-digest bindings, and the
digest of the typed backend verifier receipt envelope carrying the proof bytes,
but the payload-only validator is permanently non-accepting. The proof-carrying
validator decodes a typed backend verifier receipt and binds the receipt digest,
verifier-result digest, public-view digest, creation binding digest, native
role-root commitment, masked-projection digest, trace-binding context digest,
and backend proof-payload digest before it reaches the still fail-closed
acceptance checks:
`proof_payload_contract_implemented = false` or
`all_obligation_proofs_verified = false` or
`backend_acceptance_proof_verified = false`; future acceptance also requires the
hidden/trace witness-check verified bits and the explicit trace-root and
constraint-root bridge verified bits. The current carrier may bind the full
obligation label set (`obligation_set_bound = true`), and the check-bound
constructor records locally validated hidden/trace witness checks plus the
local trace/constraint-root bridge verified bits and mints a typed
`Path2TransparentLocalTraceBindingProofV1`. Its digest is not one of the 24
full backend obligation proof digests, is repeated by the full-hidden proof
payload, and is carried by the outer subproof envelope. It hashes the hidden
and trace-origin check digests, verified check bits, no-export bit, backend
verifier result's transparent transcript digest, and trace schedule
(`trace_rows`, `query_count`, and `query_index`), so the local bridge identity
cannot drift from the sampled trace domain. The proof payload and subproof
envelope now repeat those transcript and schedule fields from the statement as
explicit wire anchors too. The current statement-only
unimplemented-backend envelope requires that field to be zero; a typed
checked-payload envelope must instead repeat the proof payload's checked
nonzero digest and still stops at the backend-unimplemented gate. The
constructor still does not claim the aggregate backend obligation set or
backend acceptance proof has been proved.
The full-hidden-relation proof payload now accepts only a typed
certificate-plus-receipt proof bundle; bare certificate payload bytes are
rejected before they can masquerade as proof bytes. The backend payload
contract and verifier key bind that bundle schema plus the embedded certificate
payload, backend verifier result, and backend verifier receipt schemas. The
bundle also checks that the receipt's result digest, verified proof-payload
digest, transparent transcript digest, trace schedule, and no-export bit agree
with the certificate carrier, and rejects carried witness-check bytes unless the
certificate payload marks those checks verified.
The statement/result/certificate path also carries the public trace-to-binding
anchors individually: proof-system id, leaf context, creation-proof binding
digest, matrix-domain transcript, masked-projection digest, aggregation context,
all six creation role handles, and the native role-root commitment. Those
anchors are hashed into the carriers and checked across the
statement/payload and payload/receipt boundaries, so the bridge target is
explicit rather than hidden only inside `trace_binding_context_digest` or the
aggregate role root. The public view,
transparent transcript digest, and trace-binding context digest now include the
native role-root commitment as well, so a trace Merkle root is sampled under the
same role-root target later checked by the certificate path. The selected-view
bridge and commitment API verifier now reject a public view whose role-root
field differs from the creation binding's native role root. The top-level
transparent proof payload and typed proof-bytes envelope also repeat the
leaf/matrix/aggregation source anchors, trace and constraint commitment roots,
transparent transcript digest, and trace schedule fields alongside the full
transparent public-view digest, creation binding digest, native role-root
commitment, masked-projection digest, and trace-binding context digest. The
query-opening manifest carries the public-view/context anchors, `query_count`,
and per-query records. Their digest frames hash those anchors, so sampled
openings/proof bytes cannot be replayed onto a neighboring public view,
transcript, schedule, or binding target with only matching profile fields. The
backend-facing full-hidden public inputs, subproof statement, backend verifier
result, certificate payload, hidden/trace witness-check receipts, full-hidden
proof payload, and subproof envelope carry that same relation id, column
profile, transparent transcript digest, trace schedule, public-view digest,
creation binding digest, all six creation role handles, native role-root
commitment, masked-projection digest, and trace-binding context digest. The
backend verifier result, backend verifier
receipt, and certificate payload carry the same transparent transcript digest
and trace schedule fields, so the future backend proof and extractor are pinned
to the verifier-accepted opening view, relation, column policy, transparent
transcript, trace schedule, and binding context. The final full-hidden proof
payload and subproof envelope also repeat those anchors from the
statement/public inputs, and the subproof envelope rejects a forged nonzero
local trace-binding proof digest on today's disabled statement-only route while
allowing a checked typed-payload envelope to carry the proof payload's nonzero
local bridge digest before the backend-unimplemented gate. The
outer subproof envelope, backend verifier receipt, and final proof payload also
repeat the hidden-relation and trace-origin witness-check receipt digests, trace
roots, trace-to-binding layer digest, and leaf/matrix/aggregation source
anchors. The backend verifier receipt also repeats the backend statement digest, public-input digest,
verifier-key digest, backend proof-system id, backend acceptance-verifier digest, and backend
obligation-set digest, plus the local trace-binding proof digest, trace
commitment root, constraint commitment root, trace-to-binding layer digest, and
leaf/matrix/aggregation source anchors; bundle and proof-payload validation
compare those fields against the embedded certificate payload before the backend
acceptance gate. The
hidden-relation and trace-origin witness-check receipts also carry the
proof-system id, trace-to-binding layer digest, leaf/matrix/aggregation source
anchors, creation binding digest, all six creation role handles, native
role-root commitment,
masked-projection digest, transparent transcript digest, trace roots, trace
schedule, and trace-binding context digest, and certificate assembly rejects any
drift from the statement. The backend verifier
receipt repeats the public-view digest,
creation binding digest, all six creation role handles, native role-root
commitment, masked-projection digest, and trace-binding context digest, and
both proof-bundle assembly and
proof-carrying payload verification reject receipt drift from the certificate
payload.
The public-view verifier also structurally
checks any carried full-relation envelope segment before returning the disabled
sentinel, so malformed suite wrappers cannot masquerade as merely
not-yet-implemented proofs.
The production-obligations and proof-family capability records separately mark
the local trace-binding payload-shape check and public envelope check as true,
while keeping full relation constraints, backend certificate instantiation, and
extractor/review discharge false.

---

## 6. Green gates

- `cargo build` / `cargo build --tests`: GREEN (no errors, no new warnings).
- The transparent-descent test set (round-trip, negatives, hiding, the existing
  boundary binding test) and the full `cargo test --lib` suite: GREEN.
- Production deployment preflight: UNCHANGED — stays fail-closed (this ZK path is
  AVAILABLE / NON-DEFAULT; the prover entry point and the flipped verifier do NOT
  touch `production_deployment_preflight`).
- SPEND signature bytes: UNCHANGED (264/318).
- KAT / lake build / EasyCrypt `*.ec`: unaffected (the creation-proof path is not
  the signed-bytes KAT; no `.ec`/Lean spec touched).

---

## 7. What is dormant vs done

The sampled-row debug core is built and remains exercised by tests, but the
public `*_with_proof_bytes` boundary is now fail-closed with
`PATH2_TRANSPARENT_DESCENT_FULL_HIDDEN_RELATION_DISABLED_ERROR` for otherwise
valid transcripts because the core does not yet enforce the full hidden relation.
The two intermediate boundaries
(`_payload_boundary`, `_with_query_manifest`) still return the "not implemented"
sentinel string because, by construction, they lack proof bytes — they are partial
bindings-only stages, not the proof-byte verifier, and their existing tests are unchanged.

**Remaining:** instantiate a full-relation backend certificate for C1/C2/C4/C6,
then discharge `p2_sis_extractor` (§5); instantiate the hidden range
extractor/range-proof obligation for the masked digit shares; widen the external
coded-SIS estimator scope to the new public surface (roots + masked lanes;
dossier §6); the size-compaction follow-up (§4).
