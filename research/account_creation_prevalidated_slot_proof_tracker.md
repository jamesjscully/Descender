# Account-Creation Prevalidated Slot Proof Tracker

Scope: proof-side work needed to make Path 2 deployable. This tracker covers
the formal and cryptographic obligations for account-creation / prevalidated-slot
authorization. It intentionally excludes production engineering, constant-time
audit, database implementation, and external review except where the proof needs
a precise interface.

Status legend: `[ ]` open, `[~]` in progress, `[x]` complete, `[A]` aggregation
optional.

## Target Theorem

`account_creation_prevalidated_slot_authorization_or_break`

If the spend-time verifier accepts a checked-core Fq12 signature and consumes a
slot, then either:

- the slot was previously admitted by the account-creation verifier as a valid
  hidden-source relation for the same account identity, slot id, verifier
  version, opening commitment, pad-seed digest, hidden-source statement digest,
  and account state root; or
- the adversary exports a named break.

Named breaks:

- `creation_time_hidden_source_soundness_break`;
- `prevalidation_leaf_binding_break`;
- `prevalidation_registry_binding_break`;
- `postchallenge_slot_creation_or_rewrite_break`;
- `slot_replay_or_rollback_break`;
- `checked_core_message_binding_break`;
- `prevalidation_data_availability_break`;
- `live_slot_leakage_or_keygen_break`;
- `qrom_or_domain_separation_break`;
- `optional_aggregation_soundness_break` only when aggregation is enabled.

## Proof-First Milestone Order

1. Freeze the proof object vocabulary.
2. State the direct single-slot theorem.
3. Prove creation-time hidden-source admission soundness.
4. Prove prevalidation leaf/registry binding.
5. Prove spend-time authorization consumes only admitted slots.
6. Compose checked-core message binding with slot authorization.
7. Compose lifecycle/no-postchallenge programming into the same game.
8. Charge leakage and QROM/hash assumptions.
9. Add optional aggregation mode without changing the direct theorem.
10. Produce the final Path 2 theorem statement and proof boundary ledger.

## A. Proof Object Vocabulary

- [x] Define `PrevalidatedSlotLeafV1` byte/field frame.
  Requirement: the proof must name the exact fields consumed by the theorem:
  domain separator, profile version, account id, public-key identity, slot id,
  verifier version, account state root before admission, opening commitment,
  pad-seed digest, hidden-source statement digest, prevalidation transcript
  digest, lifecycle anchor, and live-status marker.
  Evidence: `research/prevalidated_slot_leaf_v1_frame.md` fixes a 353-byte
  binary frame with offsets, field encodings, digest roles, mapping from the
  current state-assisted slot record, required negative controls, and Rust/Lean
  constant plan. Executable closure remains open until constants, KATs, and
  Lean/Rust checks are implemented.

- [x] Define `slot_authorization_digest`.
  Requirement: the digest must be a canonical hash over
  `DESCENDER_SLOT_AUTHORIZATION_V1 || PrevalidatedSlotLeafV1`. The checked-core
  spend frame must bind this digest rather than one concrete state proof format.
  Evidence: `research/prevalidated_slot_leaf_v1_frame.md` defines
  `slot_authorization_digest = SHA256(ASCII("DESCENDER_SLOT_AUTHORIZATION_V1") || PrevalidatedSlotLeafV1_bytes)`.
  Domain-registry implementation remains open under the Path 2 domain item.

- [x] Define creation-time evidence object.
  Requirement: specify whether account creation carries a replayable transcript,
  certificate root, full carrier, compact proof handle, or direct hidden-source
  witness-opening proof. The theorem must know what the creation verifier checks
  and what is only committed.
  Evidence: `research/account_creation_evidence_v1_spec.md` defines
  `AccountCreationEvidenceV1`, its duplicated binding fields, transcript digest
  rule, evidence-kind enum, verifier checks, preferred first target
  `direct_replayable_relation_certificate`, and named break outputs. The
  concrete proof-mechanism decision remains open in section B.

- [x] Define spend-time authorization handle.
  Requirement: normalize direct records, inclusion proofs, and future aggregated
  proof handles into one validated `slot_authorization_digest`. Direct
  single-slot deployment must not consume aggregation assumptions.
  Evidence: `research/slot_authorization_handle_v1_spec.md` defines
  `DirectRecord`, `InclusionProof`, and `AggregatedProofHandle`, the normalized
  `ValidatedSlotAuthorizationV1` output, spend-time binding checks, rejection
  classes, and the theorem hook. Executable closure remains open until validators
  and negative controls exist.

- [x] Reserve all Path 2 domains.
  Requirement: include at least `DESCENDER_SLOT_LEAF_V1`,
  `DESCENDER_SLOT_AUTHORIZATION_V1`, `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1`,
  `DESCENDER_SLOT_PREVALIDATE_BATCH_V1`, `DESCENDER_SLOT_ROOT_UPDATE_V1`,
  `DESCENDER_SLOT_CONSUME_SINGLE_V1`, and `DESCENDER_SLOT_CONSUME_BATCH_V1`.
  Evidence: `scripts/research/pre_signature_projection_global_domain_registry.py`
  now registers all seven Path 2 domains as planning-reserved rows. Regenerated
  `pre_signature_projection_global_domain_registry_n32_q12289` reports 28
  registered domains, zero exact duplicate registered domains, and zero prefix
  relations. Rust constants and domain-mutation KATs remain implementation
  closure work.

## B. Creation-Time Hidden-Source Soundness

- [x] State the creation-time hidden-source relation.
  Requirement: the relation must consume the state-assisted opening tuple,
  account witness commitment, pad-seed digest, public pad-opening statement,
  active-lane hidden-source statement, projection opening, and verifier version.
  It must be the same relation later committed by the prevalidation leaf.
  Evidence: `research/creation_hidden_source_relation_v1_spec.md` defines
  `CreationHiddenSourceRelationV1` as the replayable active-lane
  account-creation relation, including public inputs, certificate payload,
  seven relation-check groups, soundness claim, required negative controls, and
  mapping to existing executable evidence.

- [x] Decide the creation-time proof mechanism.
  Requirement: choose one of: direct replayable proof, state-stored full carrier,
  certificate-root-backed verifier, or another compact proof. The theorem must
  not treat a bare digest as proof semantics.
  Evidence: `research/account_creation_proof_mechanism_decision.md` selects
  `direct_replayable_relation_certificate` for the first proof pass, defines the
  `DirectReplayableRelationCertificateV1` component set, records verifier checks,
  keeps fallbacks reserved, rejects hash-only and spend-only mechanisms, and
  names the next theorem target
  `direct_replayable_relation_certificate_soundness_or_break`.

- [~] Prove account creation implies relation validity or named break.
  Requirement: if `verify_prevalidated_slot_creation` accepts a leaf/evidence
  pair, then either the hidden-source relation is valid for that leaf or the
  adversary exports `creation_time_hidden_source_soundness_break`.
  Progress: `research/direct_replayable_relation_certificate_soundness_theorem.md`
  states `direct_replayable_relation_certificate_soundness_or_break` with exact
  inputs, acceptance hypotheses, conclusions, break meanings, proof skeleton,
  dependencies, and open discharge items. Remaining work is proof discharge:
  negative-control matrix, live-slot leakage ledger, domain/KAT coverage, and
  executable creation verifier or equivalent proof artifact. First executable
  replay progress now exists in
  `tools/parameter-search/src/production_reference.rs`:
  `verify_path2_prevalidated_slot_creation_relation_replay` validates the Path 2
  leaf/evidence/payload bindings, calls the reviewed Fq12 active-lane masked
  projection soundness certificate verifier, and binds the replayed slot id,
  statement digest, opening commitment, and pad-seed digest back to the leaf.
  KATs prove the wrapper accepts a reviewed active-lane certificate and rejects
  wrong pad seed plus statement-digest mismatch. This replay hook also exposed
  the current top blocker: the reviewed active-lane statement frame includes
  spend-time `message_digest` and `signature_digest`, so using that digest
  directly in a Path 2 leaf creates a cycle
  `statement_digest -> signature_digest -> checked-core challenge ->
  slot_authorization_digest -> leaf -> statement_digest`. See
  `research/path2_signature_statement_circularity_blocker.md`. The final Path 2
  creation statement must be split from the spend signature before an accepting
  Path 2 spend fixture is meaningful. First split-statement executable progress
  now exists: `Path2CreationHiddenSourceStatementV1` defines canonical bytes
  without `message_digest` or `signature_digest`, and
  `path2_creation_hidden_source_statement_v1_is_canonical_and_non_circular`
  proves the frame is stable under legacy message/signature rebinding while
  still binding pad-opening data. The Path 2 relation replay fixture now binds
  this non-circular statement digest into the leaf/evidence/certificate path;
  the reviewed Fq12 signature-dependent statement digest is retained only as a
  separate direct-replay payload field, with a negative KAT for substitution.
  `path2_creation_relation_replay_rejects_profile_and_verifier_rebinding`
  additionally proves verifier-version and profile rebinding reject at the
  non-circular creation-statement digest layer instead of silently reusing the
  old replay payload. This pass also found and fixed a real creation-time
  rebinding gap: relation replay now checks that `leaf.public_key_identity`
  equals the digest of the actual replayed public key, and the same KAT rejects
  a fully rebound leaf/evidence/certificate whose public-key identity no longer
  matches `fixture.public_key`. The residual blocker is now narrowed in
  `research/path2_direct_relation_certificate_payload_blocker.md`: the public
  `DirectReplayableRelationCertificateV1` carries a payload digest/handle, while
  the executable relation replay still consumes hidden test-harness inputs. A
  deployable proof needs a concrete public hiding proof payload, or an explicit
  theorem switch to public opening/trusted admission. The code now has a typed
  fail-closed `Path2DirectCreationProofPayloadV1` boundary and
  `verify_path2_direct_creation_proof_payload_v1`; KAT
  `path2_direct_creation_proof_payload_is_fail_closed_until_instantiated` proves
  digest-only and unsupported payload kinds do not accidentally satisfy proof
  semantics. `research/path2_direct_creation_hiding_payload_cost_model.md`
  identifies the proof bottleneck: direct use of current SHA-256 commitments
  makes the hiding payload SHA-circuit dominated, with an estimated 23 SHA-256
  compressions for the lower-bound relation and about 55 if the full public pad
  statement is proven inside the payload. The q12289 arithmetic relation is
  small by comparison. Recommended next path is a proof-friendly inner
  commitment layer that keeps SHA-256 as outer consensus binding but moves the
  payload proof into algebraic commitments.
  `research/path2_proof_friendly_inner_commitment_sketch.md` specifies the first
  version of that redesign: add a first-class
  `creation_proof_binding_digest`/`Path2CreationProofBindingV1` boundary so the
  proof opens pad/opening/projection commitments that are friendly to q12289
  arithmetic. Estimated spend-signature change is zero bytes; the byte cost is
  at account creation/state admission. The Rust boundary now exists:
  `Path2CreationProofBindingV1` has a canonical digest, the direct proof payload
  carries `proof_binding_digest`, and the fail-closed KAT rejects binding digest
  mismatch plus slot rebinding before any unsupported proof kind can accept.
  A fail-closed `Path2PrevalidatedSlotLeafV2` trial now carries
  `creation_proof_binding_digest` directly in the leaf preimage. KAT
  `path2_prevalidated_slot_leaf_v2_binds_creation_proof_binding_digest` proves
  V2 is exactly 32 bytes larger than V1, uses distinct leaf/authorization
  domains, binds the proof-friendly digest into authorization, and keeps the
  checked-core spend signature at 252 bytes. The first proof-friendly payload
  trial is also executable:
  `PATH2_DIRECT_CREATION_PROOF_KIND_TOY_Q_LINEAR` and
  `verify_path2_direct_creation_toy_q_linear_proof_v1` bind through
  `Path2CreationProofBindingV1`, recover masked projection residues from the
  packed witness commitment, and check the reduced relation
  `masked_projection_i = pad_lane_i + account_projection_i mod q` with
  Schnorr-style q12289 response equations. KAT
  `path2_toy_q_linear_creation_proof_accepts_and_rejects_relation_mutation`
  proves an honest structural proof accepts while relation-response mutation
  and malformed commitment encoding reject. This is not deployable: the toy
  commitment group is small `Z_q` and lacks production binding/hiding. The
  remaining proof blocker is now to replace that toy commitment with a real
  proof-friendly commitment/proof system and promote the resulting payload
  theorem. `research/path2_real_commitment_options.md` now records the design
  gate for that replacement: group Pedersen/IPA and pairing/IPA SNARK routes are
  compact but non-PQ, so they are prototype/hybrid paths only; the serious
  production candidates are descent-derived lattice/SIS commitments with a
  linear ZK proof and transparent hash-based proofs over the proof-friendly
  relation. `research/path2_descent_sis_commitment_viability.md` promotes the
  original Descender descent/SIS approach to the primary research target:
  instantiate the existing local-quotient/descent gluing theorem for the Path 2
  account-creation payload, then fall back to transparent hash-based proofs only
  if byte or verifier cost is unacceptable.
  `research/path2_descent_framing_opportunities.md` records the other places
  where descent framing is useful: optional direct/batch/aggregate admission
  equivalence, public-projection/sampler-hiding leakage guards, and future
  data-availability roots. It also marks canonical byte frames, SHA/domain
  binding, checked-core equation checks, and lifecycle storage engineering as
  places where descent language should not be forced. First concrete feasibility
  progress now exists in `research/path2_descent_sis_feasibility.md` and
  `scripts/research/path2_descent_sis_feasibility.py`: the public quotient is
  fixed to the six masked projection lanes, the committed role vectors are
  pinned for pad source, projection relation, and opening tuple, and lower-bound
  account-creation payload rows are generated. The 48-row and 64-row bands
  produce lower bounds of 725 and 893 payload bytes respectively, with 5184 and
  6912 verifier commitment-check mul-adds. These are not security claims; they
  are the first concrete object for SIS binding/hiding/norm/extractor analysis.
  `research/path2_descent_sis_obligation_matrix.md` now pins the nine open rows
  that must close before theorem promotion: public quotient types, matrix
  domains, SIS binding, hiding/sampler, non-public link equation, norm/range,
  Fiat-Shamir/QROM, extractor, and payload no-export. The first row is now
  interface-closed: `Descender/Path2DescentSIS.lean` builds and
  `research/path2_descent_sis_type_map.md` maps `Path2CreationProofBindingV1`
  and payload bytes into `Public`, `Representative`, `MaskedProjection`,
  `HiddenOpening`, `AccountEvidence`, `BreakEvent`, `Predicates`, and
  `localQuotientDescent`. The Lean sidecar now also pins the descent/SIS role
  layout and fixed-length `OpeningNormRangeValid` policy for pad-source,
  projection-relation, opening-tuple, and transcript commitments. The second row
  is now executable-closed: `research/path2_descent_sis_matrix_domains.md` and
  KAT `path2_descent_sis_matrix_domains_are_role_and_row_band_separated` define
  role-separated pad/projection/opening/transcript matrix domains, 48/64 row
  bands, column counts, SHA-256 rejection sampling, transcript digests,
  deterministic regeneration, and global Path 2 domain separation. The third
  row has advanced to assumption-surface pinned, not closed:
  `research/path2_descent_sis_binding_surface.md` defines the exact binding
  game for Path 2 role matrices. A double opening exports
  `delta = v0 - v1` with `A_role,rowband * delta = 0 mod q` for a registered
  role and the 48/64 row band. No numeric binding claim is made yet.
  `p2_sis_norm_range` now has executable policy coverage in the active
  hybrid-base128 shape: pad/projection/opening hidden role values use
  `12/12/16` nonnegative base-128 semantic digit columns, with `36/36/56`
  total role columns and a `128`-column aggregate transcript role.
  `verify_path2_descent_sis_opening_norm_range` enforces exact role column
  counts, canonical q12289 centered lifts, nonnegative semantic digits bounded
  by `127`, blinding bounds `2048`, nonnegative base-128 range digits, and
  range digit maximum `127`. KATs
  `path2_descent_sis_norm_range_policy_rejects_wraps_and_wrong_shapes` and
  `path2_descent_sis_norm_range_policy_covers_range_digits` reject semantic
  overflow, negative semantic digits, blinding overflow, negative range digits,
  noncanonical residues, wrong column counts, and unknown roles; KAT
  `path2_descent_sis_norm_range_policy_covers_transcript_semantic_digits`
  checks that aggregate transcript semantic columns also use the active
  base-128 digit policy. `research/path2_descent_sis_hardness_sanity.md` now
  pins the first executable SIS sanity read over these corrected dimensions:
  full-column-rank private-only surfaces exist for pad/projection and the
  64-row opening tuple, but the 48-row opening tuple and both aggregate
  transcript bands have kernels whose accepted delta bound is dominated by
  `2048` blinding coordinates and is too wide for a clean short-SIS binding
  surface. Therefore the aggregate transcript commitment must not be promoted
  as the active binding theorem under the current blinding policy; binding must
  live in the proof-system/native-boundary trace authentication layer. The
  first rank/hiding probe then found a more urgent design blocker:
  `research/path2_descent_sis_rank_hiding_probe.md` shows that the current
  public pad-source, projection-relation, and opening-tuple matrices are full
  column rank for both 48-row and 64-row bands. Therefore public commitments of
  the form `C = A*v mod q` reveal `v` over `F_q`. This is not a binding failure;
  it means the current three-public-commitment shape cannot be promoted until
  the commitment dimensions/proof transcript are redesigned to provide hiding.
  `research/path2_descent_sis_hiding_redesign_options.md` now pins the viable
  redesign branches. The best next theorem route is likely to keep the
  non-hiding linear commitments private inside a ZK proof transcript if the
  payload system supports that; the main public-object fallback is a
  small-noise computational-hiding commitment, which preserves short openings
  but adds a named hiding assumption. A statistically hiding public kernel is
  possible dimensionally, but broad randomness changes the binding theorem.
  `research/path2_descent_sis_private_zk_payload_surface.md` now pins the
  private-ZK route: the public statement is leaf/proof-binding context, masked
  projection, matrix-domain transcript digest, and ZK transcript digest; pad
  source, projection opening, opening tuple, digit/range witnesses, and any
  internal linear commitment messages are private and must not be exported. On
  this route, `p2_sis_binding` no longer means public `C=A*v` binding for the
  main secret roles. It is consumed by the proof extractor: accepting transcripts
  must yield the hidden relation witness or a named proof-system/binding/range/
  QROM break. `research/path2_descent_sis_private_zk_qrom_frame.md` pins the
  Fiat-Shamir frame fields for this route, including proof-system id, row band,
  matrix transcript digest, leaf context, creation proof binding, masked
  projection, aggregation context, public proof commitment digest, and round
  index. `research/path2_descent_sis_private_zk_no_export_contract.md` pins the
  public serializer allowlist and private denylist; this also pins the
  non-public-link rule that pad lanes and account-projection lanes are never
  public separately. `research/path2_descent_sis_private_zk_proof_family_gate.md`
  selects `lattice_sigma_rejection_zk` as the next concrete probe, with
  Stern-like and transparent hash proofs kept as fallback baselines.
  `research/path2_descent_sis_lattice_sigma_toy_model.md` now pins the sigma
  transcript skeleton, simulator surface, extractor surface, and an optimistic
  byte floor. The key result is sobering: with base-128 digit/range witnesses,
  a 64-repetition floor is already 16416 bytes before concrete rejection and
  proof-family overhead. `research/path2_private_zk_vs_transparent_gate.md`
  therefore promotes the transparent hash proof from passive fallback to active
  comparator before further sigma optimization.
  `research/path2_transparent_proof_byte_model.md` models the same 80-lane /
  66-row private relation and produces planning floors of 4896, 6744, and 8848
  bytes for the 256/512/1024-row comparator bands. Based on that comparison,
  `research/path2_payload_proof_candidate_decision.md` selects
  `transparent_hash_proof_over_path2_descent_relation` as the active deployable
  account-creation payload candidate. The descent/SIS work remains the semantic
  relation, matrix/domain framework, and assumption vocabulary; the
  repetition-heavy lattice-sigma transcript is parked until a non-repetition-
  heavy argument beats the transparent planning rows.
  `research/path2_transparent_payload_contract.md` now pins the selected
  transparent proof frame, query-budget symbols, extractor/soundness surface,
  zero-knowledge/no-export surface, public serializer allowlist, private
  denylist, and negative-control list. The transparent contract supersedes the
  earlier private-ZK frame for the active candidate while preserving the same
  Path 2 descent relation and no-export goals. First executable anchoring is
  now in `tools/parameter-search/src/production_reference.rs`:
  `PATH2_TRANSPARENT_DESCENT_PROOF_DOMAIN`,
  `PATH2_TRANSPARENT_DESCENT_RELATION_DOMAIN`,
  `PATH2_TRANSPARENT_PUBLIC_VIEW_SCHEMA`,
  `path2_transparent_descent_relation_id`, and
  `path2_transparent_descent_transcript_digest`. KAT
  `path2_transparent_descent_transcript_binds_public_context` proves the digest
  is deterministic, binds leaf context, rejects noncanonical masked projection,
  and rejects unsupported trace rows / zero query count; the global Path 2
  domain-disjointness KAT now includes the transparent domains. A typed public
  view is also anchored:
  `path2_transparent_descent_public_view_v1` and
  `path2_transparent_descent_public_view_v1_json` serialize only the allowlisted
  transparent metadata, and KAT
  `path2_transparent_descent_public_view_serializes_only_public_material` scans
  the JSON for the transparent private-field denylist. The Path 2 query-budget
  object now also marks the selected transparent payload symbols active:
  `q_transparent_trace_commit`, `q_transparent_query_index`,
  `q_transparent_merkle_open`, `q_transparent_consistency`,
  `q_transparent_extractor`, and `q_transparent_simulator`.
  `research/path2_transparent_extractor_soundness_bridge.md` now pins the
  bridge from the transparent payload verifier into
  `direct_replayable_relation_certificate_soundness_or_break`: an accepting
  transparent payload either extracts `pad_source_opening`,
  `projection_relation_opening`, `opening_tuple`, and base-128 digit/range
  witnesses satisfying the 80-lane / 66-row Path 2 descent relation for the
  accepted public view, or exports a named transparent-proof, hidden-source,
  norm/range, QROM/domain, or no-export break. This is still a bridge contract,
  not theorem closure. First executable boundary progress is now in
  `verify_path2_direct_creation_transparent_descent_payload_boundary`, which
  binds the transparent proof kind, proof-system id, public view, nonrecursive
  leaf context, creation proof binding, payload digest, and query-opening
  digests before rejecting fail-closed because the transparent proof verifier is
  not implemented. The top proof-risk pass now narrows that blocker:
  `research/path2_transparent_relation_arithmetization_target.md` pins the
  66-row target as `R_core_66`, not the full account-creation payload relation.
  `research/path2_inner_commitment_relation_refinement.md` splits the theorem
  into `R_core_66`, `R_inner_commitment_opening`, `R_account_binding`, and
  `R_outer_sha_binding`. `research/path2_trace_commitment_binding_audit.md`
  then audits the executable boundary and finds a precise two-hop shape: trace
  roots are bound through the transparent transcript and payload digest, while
  `Path2CreationProofBindingV1` binds role commitments and leaf context. The
  next proof blocker is to define compact link equations proving that the
  accepted trace columns open or match those role commitments, or else add
  explicit algebraic opening rows / trace roots to the binding object and update
  the byte model. `research/path2_role_commitment_link_equation_options.md`
  selects the compact interpretation: the role commitments are typed transparent
  trace subcommitments, not SHA digest preimages. This keeps the byte path alive
  but pushes the next proof blocker to the transparent commitment API: parent
  root, role subroots, hiding/blinding, extractor guarantee, and public payload
  bytes. `research/path2_transparent_commitment_api.md` now pins that theorem
  interface and makes the next decision explicit: select a transparent proof
  family that supplies authenticated typed role subcommitments with ZK query
  openings, or fall back to explicit algebraic opening rows.
  `research/path2_transparent_proof_family_gate.md` selects the conditional
  primary family `zk_masked_trace_iop` and marks vanilla Merkle/FRI raw trace
  opening dead for hidden columns. The next proof blocker is now the masked
  column transform for `R_core_66`: committed/opened values, mask consistency,
  role-subroot authentication, and byte-model deltas.
  `research/path2_masked_trace_transform.md` pins that transform and raises the
  private-lane floor from 80 to 138 before range-gadget and account-binding
  extras. `research/path2_transparent_proof_byte_model.md` now treats the old
  4896/6744/8848-byte rows as lower-bound comparators and gives masked floors
  of 5520/7524/9784 bytes for the 256/512/1024-row planning bands.
  `research/path2_masked_trace_range_account_delta.md` adds a conservative
  range/account delta, raising the planning lane floor to 194 and the
  256/512/1024-row floor to 6144/8304/10720 bytes. The next proof blocker is to
  formalize the `zkMaskedTraceIOPBindingAndHiding` assumption and set a fallback
  threshold for concrete range/FRI overhead before any verifier work.
  `research/path2_zk_masked_trace_iop_assumption.md` now pins that assumption
  explicitly: binding extractability, role-subcommitment binding, masked-query
  zero knowledge, mask freshness, and Fiat-Shamir binding. The next proof
  blocker is a proof-family selection checklist against that assumption.
  `research/path2_masked_trace_iop_selection_checklist.md` pins that checklist
  and keeps `zk_masked_trace_iop` as the active candidate while rejecting
  vanilla raw Merkle/FRI as a negative control.
  `research/path2_masked_trace_commitment_layout.md` then pins the minimal
  parent-root/role-subroot layout and exposes the next risk: a role subroot over
  raw hidden leaves is binding but not automatically hiding under public paths.
  `research/path2_masked_trace_leaf_encoding_fork.md` selects masked public
  leaves as the first probe and rejects raw public leaves. The current theorem
  interface is now `research/path2_two_view_trace_theorem_interface.md`: a
  public masked view for openings plus a hidden extractor view for soundness,
  with consistency between them. `research/path2_two_view_consistency_obligations.md`
  splits that theorem into obligations, and
  `research/path2_two_view_linear_leakage_probe.md` checks the highest-risk
  linear leakage case: fresh per-opening masks have zero linear leakage in the
  probe, while role-wide/reused masks leak rank. The resulting rules are pinned
  in `research/path2_two_view_column_profile_rules.md`; the next proof blocker
  has now been turned into executable profile policy:
  `PATH2_TWO_VIEW_COLUMN_PROFILE_MASKED_BASE128_V1`,
  `path2_two_view_column_profile_v1`,
  `verify_path2_two_view_column_profile_id`, and
  `path2_two_view_column_profile_v1_digest` pin the active masked/base-128
  two-view profile consumed by the transparent transcript. The profile table
  requires the nine active roles, separate hidden/masked column sets, unique
  mask domains for hidden/range/blinding roles, the selected hybrid base-128
  norm/range mode, full-trace extraction, full-witness relation knowledge, and
  unknown-profile rejection. KAT
  `path2_two_view_column_profile_is_active_masked_base128_policy` checks those
  rules and negative controls; KAT
  `path2_transparent_descent_transcript_binds_public_context` now also rejects
  unknown column-profile ids before transcript hashing. The query-opening
  no-export surface now has a first executable manifest policy:
  `PATH2_TRANSPARENT_QUERY_OPENING_MANIFEST_SCHEMA`,
  `path2_transparent_query_opening_manifest_v1`, and
  `path2_transparent_query_opening_manifest_digest` bind public query-opening
  metadata to the active profile while rejecting raw hidden visibility classes,
  hidden column-set ids, unknown roles/classes, duplicate query indices,
  missing/duplicate mask domains for hidden roles, and zero component digests.
  KAT `path2_transparent_query_opening_manifest_enforces_no_export_policy`
  covers the positive manifest, private-material serialization denylist, and
  those negative controls; the transparent payload boundary fixture now binds
  `query_openings_digest` to that manifest digest instead of an arbitrary
  placeholder. The `R_core_66` relation encoding is now also executable:
  `PATH2_TRANSPARENT_RELATION_LAYOUT_SCHEMA`,
  `path2_transparent_relation_layout_v1`,
  `verify_path2_transparent_relation_layout`, and
  `path2_transparent_relation_layout_v1_digest` pin the 6 masked-projection
  rows, 20 digit-reconstruction rows, 40 digit-range rows, three semantic
  roles, 40 base-128 digit lanes, 40 range-witness lanes, and the active
  two-view column profile. `path2_transparent_descent_relation_id` now binds
  the relation-layout digest, so row/role/lane drift changes the relation and
  proof-system ids. KAT `path2_transparent_relation_layout_pins_r_core_66`
  checks the positive layout and malformed row/role/digit negative controls.
  The transparent verifier boundary now has a manifest-aware fail-closed entry:
  `verify_path2_direct_creation_transparent_descent_payload_boundary_with_query_manifest`
  reuses the existing payload/public-view binding checks, then validates the
  query-opening manifest object, checks manifest context against the accepted
  public view, and checks the manifest digest against `query_openings_digest`
  before returning the proof-verifier-not-implemented blocker. KAT
  `path2_transparent_descent_payload_boundary_binds_public_view_before_fail_closed`
  now covers the manifest-aware positive path plus mismatched manifest context,
  rebound manifest digest, and raw-hidden-opening negative controls at the
  verifier handoff. Transparent proof-byte parsing now has a canonical
  fail-closed envelope:
  `PATH2_TRANSPARENT_PROOF_BYTES_ENVELOPE_SCHEMA`,
  `path2_transparent_proof_bytes_envelope_v1_bytes`,
  `decode_path2_transparent_proof_bytes_envelope_v1`,
  `verify_path2_transparent_proof_bytes_envelope_v1`, and
  `verify_path2_direct_creation_transparent_descent_payload_boundary_with_proof_bytes`
  bind proof bytes to the accepted public view, `R_core_66` layout digest,
  query-opening manifest digest, column profile, trace rows, query count, and
  an opaque proof-body digest/length before returning the proof-verifier-not-
  implemented blocker. The envelope constructor/parser now also enforce the
  active 16416-byte fallback threshold on the canonical encoded proof bytes, so
  an over-budget transparent proof cannot pass the fail-closed boundary as the
  active route. The boundary KAT now also covers proof-byte digest mismatch,
  body digest mismatch, truncated proof body, envelope context rebinding, empty
  proof-body rejection, oversized proof-body rejection, and oversized envelope
  rejection. The proof-system id now binds the
  active proof-family capability profile:
  `PATH2_TRANSPARENT_PROOF_FAMILY_CAPABILITY_SCHEMA`,
  `path2_transparent_proof_family_capability_v1`,
  `verify_path2_transparent_proof_family_capability`, and
  `path2_transparent_proof_family_capability_v1_digest` pin
  `zk_masked_trace_iop_native_boundary` as the only active transparent family
  shape and require parent-root binding, typed role subcommitments, full-trace
  extraction, full-witness relation knowledge, hidden-column zero knowledge,
  masked-query no-export, QROM schedule binding, norm/range soundness,
  blinding/noise bound soundness, no raw hidden openings, no public linear role
  commitments, and the 16416-byte fallback threshold. KAT
  `path2_transparent_proof_family_capability_pins_active_assumptions` rejects
  sampled-only extraction, raw hidden openings, missing typed role
  subcommitments, missing no-export, and vanilla Merkle/FRI relabeling. The
  active lane-floor mismatch is now executable too:
  `PATH2_NATIVE_BOUNDARY_TRACE_COLUMN_MANIFEST_SCHEMA`,
  `path2_native_boundary_trace_column_manifest_v1`,
  `verify_path2_native_boundary_trace_column_manifest`, and
  `path2_native_boundary_trace_column_manifest_v1_digest` pin the active
  native-boundary private trace as 194 contiguous lanes: the 80-lane
  `R_core_66` semantic core, 40 digit-mask lanes, 18 masked-projection relation
  mask lanes, 40 range-witness ZK mask lanes, and 16 account-binding link
  lanes. The proof-family capability now binds this manifest digest, the
  194-lane active witness floor, and the 176-row working native-boundary
  relation estimate, so an 80-lane semantic-core-only verifier changes the
  proof-system id instead of silently satisfying the active path. KAT
  `path2_native_boundary_trace_column_manifest_pins_active_lane_floor` rejects
  semantic-core-only lane regression, non-contiguous lane coverage, public
  private-lane exposure, and private-lane opening; the capability KAT now also
  rejects trace-column-manifest rebinding and active-lane-floor rebinding. The
  active query schedule is now executable:
  `PATH2_TRANSPARENT_QUERY_SCHEDULE_SCHEMA`,
  `path2_transparent_query_schedule_v1`,
  `verify_path2_transparent_query_schedule`, and
  `path2_transparent_query_schedule_v1_digest` pin the native-boundary planning
  rows to 256/16, 512/20, and 1024/24, bind the trace-column manifest digest
  and fallback threshold, and reject the 128/12 low-security planning row under
  the active proof-system id. `path2_transparent_descent_transcript_digest`
  now rejects row/query mismatches before hashing, and the proof-system id plus
  proof-family capability digest bind the schedule digest. KAT
  `path2_transparent_query_schedule_pins_active_row_query_pairs` rejects
  low-query schedules, 128-row planning schedules, byte-floor drift, and
  manifest rebinding. The
  next executable blocker is an actual concrete transparent proof-system
  verifier with honest/malformed coverage against this capability profile,
  encoded relation, validated query-opening manifest, and canonical proof-byte
  envelope.

- [x] Carry over active-lane negative controls into creation time.
  Requirement: wrong witness digest, wrong pad seed, wrong public pad statement,
  wrong relation projection, wrong opening commitment, wrong verifier version,
  and carrier-context replay must all reject or export named breaks.
  Progress: `research/creation_time_negative_control_matrix.md` defines 29
  creation-time controls across leaf/evidence binding, payload availability,
  active-lane relation mutations, replay context, duplicate admission, live
  marker, and domain confusion. Existing Rust negative controls cover the core
  semantic relation mutations; new Path 2 KATs are still needed for account/key
  context replay, state-root replay, payload availability, and direct/batch
  domain confusion. `research/creation_time_negative_control_kat_plan.md` maps
  existing Rust coverage to reusable controls and defines the new Path 2 KATs,
  fixture shape, file targets, leakage guards, and minimum promotion gate.
  First executable progress is in
  `tools/parameter-search/src/production_reference.rs`: canonical
  `Path2PrevalidatedSlotLeafV1` serialization, authorization digest,
  `Path2AccountCreationEvidenceV1`, the frame-binding verifier, and three
  passing KATs for leaf bytes, binding mutations, and non-circular transcript
  hashing. This work found and fixed two proof-frame issues:
  `prevalidation_transcript_digest` cannot include `slot_leaf_digest` without a
  circular hash definition, and `slot_id` must be duplicated in the evidence
  transcript to prevent cross-slot replay under-specification. Verifier-version,
  transcript mutation, and direct-mode batch-domain rejection KATs now pass.
  Payload availability and account/key/slot/state cross-leaf payload replay KATs also pass
  through `Path2DirectReplayableRelationCertificateV1`. Public-key identity
  rebinding against the actual replayed key, verifier-version rebinding, and
  profile rebinding now reject in
  `path2_creation_relation_replay_rejects_profile_and_verifier_rebinding`.
  The active-lane negative controls have now been carried to the Path 2 wrapper
  boundary: pad-seed substitution, statement mismatch, reviewed statement
  substitution, public-key identity rebinding, verifier-version rebinding,
  profile rebinding, payload replay, duplicate admission, public debug-output
  no-secret guards, and relation-replay rejection no-export KATs all pass.
  Further negative controls are no longer wrapper work; they belong to the
  future public direct-creation proof payload defined by
  `research/path2_direct_relation_certificate_payload_blocker.md`.

- [x] Retire spend-only full-witness extraction as a deployment requirement.
  Requirement: explicitly keep `projection_carrier_tuple_does_not_prove_full_witness_opening`
  as a negative control and state that Path 2 proves creation-time relation
  admission plus spend-time authorization, not spend-only hidden-source
  extraction.
  Evidence: `research/account_creation_prevalidated_slot_proof_boundary_ledger.md`
  lists this as an explicit non-claim, and
  `research/path2_leaf_registry_state_binding_theorem.md` states that leaf,
  registry, data-availability, exact-slot, and consume-once composition do not
  prove creation-time hidden-source validity by themselves. The reviewed
  308-byte/Fq12 projection-carrier route remains a fail-closed baseline and
  negative-control source.

## C. Leaf, Registry, And State Binding

- [x] Prove prevalidation leaf binding.
  Requirement: two accepted meanings for one leaf must imply either identical
  bound fields or a hash/commitment break. Bound fields include account id,
  public-key identity, slot id, verifier version, state root, opening commitment,
  pad-seed digest, hidden-source statement digest, transcript digest, and
  lifecycle anchor.
  Evidence: `research/path2_leaf_registry_state_binding_theorem.md` defines the
  `prevalidation_leaf_binding_break` theorem slice. Executable coverage is
  `verify_path2_prevalidated_slot_creation_frame_bindings` plus KATs
  `path2_prevalidated_slot_leaf_v1_frame_is_canonical`,
  `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations`,
  `path2_prevalidation_transcript_digest_is_not_circular`, and
  `path2_creation_hidden_source_statement_v1_is_canonical_and_non_circular`.

- [x] Prove registry binding.
  Requirement: a spend verifier cannot accept a leaf that was not admitted for
  the current account namespace and state root, unless it exports
  `prevalidation_registry_binding_break`.
  Evidence: `research/path2_leaf_registry_state_binding_theorem.md` defines the
  direct-record registry theorem. Executable coverage is
  `validate_path2_direct_slot_authorization_state_namespace` plus KATs
  `path2_direct_authorization_requires_exact_admitted_leaf`,
  `path2_direct_authorization_requires_state_namespace_admission`, and
  `path2_direct_single_slot_spend_composes_admission_signature_and_consume_once`.

- [x] Prove data-availability fail-closed semantics.
  Requirement: if validators cannot access the direct record or the reviewed
  inclusion proof for a leaf, spend-time authorization rejects. Acceptance
  without data exports `prevalidation_data_availability_break`.
  Evidence: `research/path2_leaf_registry_state_binding_theorem.md` defines the
  fail-closed data-availability interface. Executable coverage is
  `verify_path2_prevalidated_slot_creation_payload_bindings` and KAT
  `path2_prevalidated_slot_creation_payload_kats_reject_missing_or_replayed_payloads`,
  which rejects absent certificate payloads and mismatched/replayed payload
  digests.

- [x] Prove duplicate prevention at proof level.
  Requirement: duplicate `(account_id, slot_id)` and duplicate `pad_seed_digest`
  within the chosen uniqueness domain are rejected at admission or named as
  lifecycle/registry breaks.
  Evidence: `research/path2_leaf_registry_state_binding_theorem.md` defines the
  duplicate-control theorem row. Executable coverage is
  `verify_path2_prevalidated_slot_admission_namespace` and KAT
  `path2_prevalidated_slot_admission_namespace_rejects_duplicates`, which
  rejects duplicate `(account_id, slot_id)` and duplicate `pad_seed_digest`
  admissions in the modeled uniqueness domain.

## D. Spend-Time Authorization

- [x] State the spend-time authorization relation.
  Requirement: the relation consumes message, public key identity, slot id,
  account state root, `slot_authorization_digest`, checked-core signature body,
  and prevalidation handle. It outputs accept only for a live admitted leaf.
  Evidence: `research/spend_authorization_relation_v1_spec.md` defines
  `SpendAuthorizationRelationV1`, public inputs, normalized handle output,
  handle validation, exact field agreement, checked-core message binding,
  lifecycle checks, no spend-time hidden opening rule, theorem hook, direct mode,
  optional modes, and negative controls.

- [x] Prove authorization-to-admission.
  Requirement: if spend authorization accepts, then the normalized leaf was
  admitted by the creation-time verifier under the same account identity and
  state namespace, or a registry/data-availability/lifecycle break is exported.
  Progress: `tools/parameter-search/src/production_reference.rs` now has
  `validate_path2_direct_slot_authorization_admission`, which validates the
  Path 2 leaf/evidence/certificate bindings, recomputes
  `slot_authorization_digest`, and requires that exact digest to be present in
  an admitted-authorization registry. KAT
  `path2_direct_authorization_requires_exact_admitted_leaf` proves non-admitted
  leaves reject, the exact admitted leaf accepts, and a sibling slot leaf
  rejects. `validate_path2_direct_slot_authorization_state_namespace` now
  requires admission under the exact `(account_id, account_state_root_before,
  slot_authorization_digest)` tuple. KAT
  `path2_direct_authorization_requires_state_namespace_admission` proves exact
  namespace acceptance and rejects wrong state-root or wrong-account namespace
  admissions. `verify_path2_direct_single_slot_spend_authorization` now composes
  this namespace admission check with public-key identity binding, checked-core
  signature verification, exact slot-id agreement, and consume-once state
  transition. The composed KAT
  `path2_direct_single_slot_spend_composes_admission_signature_and_consume_once`
  proves the exact admitted authorization is required by the accepting spend
  path. Lifecycle rollback terms are now carried by the one-use successor-state
  theorem.

- [x] Prove exact-slot consumption.
  Requirement: accepted spend consumes the exact leaf named by the checked-core
  challenge frame and cannot consume a sibling slot, stale slot, or batch-domain
  surrogate.
  Evidence: `verify_path2_direct_single_slot_spend_authorization` rejects unless
  the decoded checked-core `slot_id` equals the prevalidated leaf `slot_id`
  before mutating the consume-once set. The negative KAT
  `path2_direct_single_slot_spend_rejects_leaf_slot_rebinding` signs a valid
  checked-core body for account slot 0 against a different leaf's
  `slot_authorization_digest`; the composed verifier rejects with
  `Path 2 checked-core slot id does not match prevalidated leaf` and leaves the
  consumed set unchanged. This closes the executable exact-slot gap that the
  piecewise checked-core and admission tests did not catch.

- [x] Prove one-use capability semantics.
  Requirement: the same prevalidated leaf cannot authorize two accepted spends
  in any valid successor state unless `slot_replay_or_rollback_break` is
  exported.
  Evidence: `verify_path2_direct_single_slot_spend_authorization` now calls
  `verify_path2_prevalidated_slot_consume_once` only after public-key identity,
  state-namespace admission, checked-core verification, and exact slot-id
  agreement succeed. The composed accepting KAT proves the first spend inserts
  the exact authorization digest; the second call rejects with
  `Path 2 slot authorization already consumed`.
  `research/path2_one_use_successor_state_theorem.md` elevates this local model
  to the valid-successor-state theorem and names rollback/reorg escape as
  `slot_replay_or_rollback_break`.

- [x] Prove checked-core message binding composes with slot authorization.
  Requirement: accepted spend must bind message, public key identity, slot id,
  account state root, and `slot_authorization_digest` into the checked-core
  challenge frame. Rebinding any of those fields rejects or exports
  `checked_core_message_binding_break`.
  Progress: `tools/parameter-search/src/production_reference.rs` now defines
  `path2_checked_core_authorization_challenge_frame`,
  `path2_checked_core_authorization_challenge_bytes`, and
  `require_path2_checked_core_authorization_challenge_binding`. The frame binds
  `DESCENDER_SLOT_CONSUME_SINGLE_V1`, public key, message, account id, account
  state root, slot id, checked-core witness commitment,
  `slot_authorization_digest`, and proof commitment. KAT
  `path2_checked_core_authorization_challenge_binds_slot_authorization` proves a
  synthetic Path 2 checked-core body accepts the binding check, changing
  `slot_authorization_digest` rejects, changing account state root rejects, and
  legacy checked-core challenge bytes differ from the Path 2 authorization
  challenge bytes. `verify_path2_checked_core_authorization` now reuses the
  reviewed Fq12 checked-core equation path with the Path 2 challenge binding
  replacing the legacy challenge binding. KAT
  `path2_checked_core_authorization_verifier_uses_path2_challenge_frame` proves
  a legacy checked-core body rejects at the Path 2 challenge gate, while a body
  with only challenge bytes rewritten passes that gate and rejects later in
  checked-core consistency. The first accepting Path 2 spend KAT now passes:
  `path2_checked_core_authorization_accepts_signed_path2_fixture` derives the
  authorization-bound challenge, recomputes challenge-dependent pad openings,
  encodes the checked-core body, and verifies it through
  `verify_path2_checked_core_authorization`.
  `sign_path2_reviewed_fq12_authorization_checked_core` now promotes this into
  a reusable signer helper: it derives the Path 2 challenge, solves the Fq12
  response-syndrome preimage into a 22-residue masked response vector,
  recomputes pad openings for arbitrary compact responses, encodes the
  signature, and verifies the result before returning it. KAT
  `path2_checked_core_authorization_signer_accepts_nonzero_responses` proves
  the helper is not limited to the zero-response fixture.
  `verify_path2_direct_single_slot_spend_authorization` now consumes this
  checked-core verifier only after state-namespace admission and before
  consume-once mutation, so message/account/state/authorization binding is on
  the accepted direct-spend path rather than a standalone helper.

## E. No-Postchallenge Programming And Lifecycle Game

- [x] State the no-postchallenge slot creation rule.
  Requirement: a slot cannot be admitted or rewritten after the adversary sees
  the spend challenge/message for that slot, unless the game exports
  `postchallenge_slot_creation_or_rewrite_break`.
  Evidence: `research/path2_no_postchallenge_creation_timing_game.md` states
  the proof-game order, forbids post-message/postchallenge leaf/statement/
  certificate rewrites, names `postchallenge_slot_creation_or_rewrite_break`,
  and maps the rule to the current Rust hooks.

- [x] Connect account state roots to challenge timing.
  Requirement: the account state root containing the prevalidated leaf must be
  fixed before the checked-core spend challenge is derived.
  Evidence: `research/path2_no_postchallenge_creation_timing_game.md` states
  the state-root timing lemma: accepted checked-core authorization binds
  `account_state_root_before` in the challenge and direct authorization must
  find the exact `(account_id, account_state_root_before,
  slot_authorization_digest)` namespace tuple.

- [x] Compose reorg/rollback assumptions into proof terms.
  Requirement: proof model must state exactly what "valid successor state" means
  and where rollback/reorg behavior is charged. This can remain a carried
  lifecycle assumption, but it must be named in the theorem.
  Evidence: `research/account_creation_prevalidated_slot_proof_boundary_ledger.md`
  defines valid successor state for direct single-slot mode, charges reorg,
  rollback, restore, and mempool reveal reuse to `slot_replay_or_rollback_break`,
  and points to the lifecycle closure policy as a carried deployment assumption.

## F. Leakage, Hiding, And Hash/QROM Budget

- [x] Recompute leakage budget for live prevalidated slots.
  Requirement: public slot metadata, prevalidation transcript digests, roots,
  opening commitments, and pad-seed digests are charged when the slot becomes
  live, including never-spent slots.
  Progress: `research/path2_live_slot_leakage_ledger.md` enumerates admission
  metadata, evidence metadata, acceptable public replay-payload views, forbidden
  raw opening material, never-spent-slot leakage, and required negative
  controls. Executable/API closure now exists for the direct public admission
  view: `path2_public_admission_view_v1_json` verifies leaf/evidence/certificate
  bindings before serialization, emits only public digest/handle fields, and
  KAT `path2_public_admission_view_serializes_only_public_material` proves the
  JSON view excludes raw pad seed, projection masks, unmasked account projection
  residues, and private opening frame markers while rejecting mutated payload
  bindings. KAT
  `path2_creation_relation_replay_rejections_do_not_export_private_material`
  additionally proves representative Path 2 relation-replay rejection paths keep
  forbidden private material out of errors/debug output. The ledger now states
  that the current direct public view preserves the standard pre-spend hiding
  claim at the serialization/error boundary; hidden-source relation soundness
  remains separate.

- [x] Preserve pre-spend witness hiding claim.
  Requirement: publishing the prevalidation leaf must not reveal unmasked
  account projection residues or raw pad seed unless explicitly charged. If the
  creation proof reveals anything, the leakage ledger must name it.
  Progress: `research/path2_live_slot_leakage_ledger.md` defines the carried
  theorem hook `pre_spend_hiding_preserved_or_public_creation_opening_break`.
  It keeps raw pad seed, projection masks, unmasked selected account projection
  residues, and private opening frames out of the public live-slot view.
  The direct public admission view API and KAT now cover the primary
  serialization surface, and
  `path2_creation_relation_replay_rejections_do_not_export_private_material`
  covers representative reject paths. Future compact/hiding proof payload APIs
  must add their own no-export KATs before inheriting this claim.

- [x] Compose mask-PRF assumptions at creation time.
  Requirement: existing mask-PRF leakage simulator work must be reinterpreted
  for creation/admission plus later spend authorization, not only for a 308-byte
  spend-carrier transcript.
  Progress: `research/path2_creation_time_mask_prf_leakage_lift.md` translates
  the existing selected-projection mask-PRF live-slot simulator into the Path 2
  admission model, states the lifted bound shape, separates standard Path 2
  hiding from `public_creation_opening_model`, and lists same-game requirements.
  The direct-mode executable witnesses are now present: public admission JSON
  no-export KAT, relation-replay rejection no-export KAT, duplicate
  `pad_seed_digest` admission rejection, and composed direct-spend authorization
  that consumes only `slot_authorization_digest` and live state.

- [x] Charge SHA-256 roles for Path 2.
  Requirement: leaf hash, authorization digest, creation transcript digest,
  checked-core challenge, pad PRF, and optional batch root domains must be named
  as RO/PRF/collision/binding assumptions with query budgets.
  Evidence: `research/path2_hash_qrom_assumption_ledger.md` enumerates Path 2
  domains, digest roles, PRF/hiding roles, query-budget variables, direct-mode
  advantage shape, optional aggregation extension, and required domain/digest
  KATs. Executable direct-mode query accounting now exists:
  `path2_direct_query_budget_v1` marks `q_leaf`, `q_auth`, `q_evidence`,
  `q_stmt`, `q_pad`, `q_mask`, `q_chal`, and `q_life` active, marks `q_batch`
  inactive, and KAT `path2_direct_query_budget_keeps_batch_inactive` proves
  direct terms do not consume optional aggregation. Numeric caps are now pinned
  from the QROM challenge budget memo: target 128 bits, quantum cap `2^16`,
  classical cap `2^33`, QROM quadratic loss 32 bits, transcript term 130 bits,
  and minimum deployment margin 1 bit. KAT
  `path2_domain_constants_are_disjoint_and_mode_separated` proves current Path 2
  domain/schema constants are duplicate-free, prefix-disjoint, and
  direct/batch/consume separated, including the public admission-view schema and
  direct query-budget schema. External QROM review and future batch-mode KATs
  remain outside direct-mode internal closure.

## G. Optional Aggregation Mode

- [A] Prove direct-vs-batch leaf equivalence.
  Requirement: batch admission cannot admit a leaf with a meaning that direct
  admission would reject. This obligation is inactive unless batch admission is
  enabled.
  Evidence target: optional aggregation theorem mode.

- [A] Prove canonical batch ordering.
  Requirement: sorted leaf order, duplicate rejection, cross-account rejection,
  and deterministic root update are included in the batch theorem. Reordering is
  either canonicalized to the same digest or rejected.
  Evidence target: batch-ordering lemma.

- [A] Instantiate `optional_aggregation_soundness`.
  Requirement: if aggregation is enabled, the theorem must name the concrete
  accumulator/rollup/batch proof system and consume its soundness, binding, and
  data-availability assumptions.
  Evidence target: aggregation assumption instantiation.

- [A] Prove spend signature frame is unchanged by aggregation.
  Requirement: aggregated admission or inclusion changes only how
  `slot_authorization_digest` is validated; it does not alter the checked-core
  spend signature frame.
  Evidence target: representation-independence lemma.

## H. Final Composition

- [x] Compose creation soundness, registry binding, spend authorization, and
  checked-core message binding.
  Requirement: produce the main direct single-slot theorem with explicit
  assumptions and named break outputs.
  Progress: `research/account_creation_prevalidated_slot_proof_boundary_ledger.md`
  states the direct-mode theorem shape
  `account_creation_prevalidated_slot_authorization_or_break`, including
  creation soundness, registry/state namespace binding, checked-core message
  binding, consume-once successor state, and named break outputs. Remaining
  evidence: `research/account_creation_prevalidated_slot_authorization_or_break.md`
  states the final direct-mode theorem, accepted tuple, acceptance hypotheses,
  named breaks, proof-composition table, and direct-vs-batch theorem boundary.

- [x] Add theorem mode switch.
  Requirement: Mode A direct single-slot consumes no aggregation assumption.
  Mode B aggregated admission/consumption consumes `optional_aggregation_soundness`
  plus concrete accumulator/proof assumptions.
  Evidence: `research/path2_theorem_mode_switch.md` defines Mode A direct
  single-slot and Mode B optional aggregated admission, lists consumed
  assumptions, states that the spend signature frame is invariant across modes,
  and forbids Mode B readiness claims until a concrete aggregation proof system
  is named.

- [x] Update production preflight blocker language.
  Requirement: preflight should say Path 2 is fail-closed pending
  account-creation prevalidation proof semantics, not pending a spend-only
  hidden-source extraction theorem.
  Evidence: `tools/parameter-search/src/production_reference.rs` now sets the
  primary preflight blocker to
  "Path 2 account-creation prevalidation proof semantics are not complete; the
  reviewed 308-byte/Fq12 projection-carrier route remains a fail-closed baseline
  and negative-control source". Regenerated
  `production_reference_gap_report_n32_q12289`,
  `production_preflight_certificate_verification_n32_q12289`, and
  `third_party_review_status_index_n32_q12289` consume the updated certificate
  digest chain. `verify_production_preflight_certificate.py`,
  `verify_third_party_review_status_index.py`, and
  `verify_selected_deployment_terminology.py` pass.

- [x] Produce proof boundary ledger.
  Requirement: every remaining unproved term is either proved, generated from an
  executable certificate, or listed as a final carried assumption. No placeholder
  may hide implementation or theorem work.
  Evidence: `research/account_creation_prevalidated_slot_proof_boundary_ledger.md`
  lists executable evidence, theorem-pending boundaries, carried lifecycle/hash
  assumptions, optional aggregation non-dependencies, explicit non-claims, and
  remaining internal closure items.

## Current External Inputs

- `research/account_creation_prevalidated_slot_plan.md`
- `research/account_creation_aggregation_readiness_contract.md`
- `research/selected_hidden_source_path_decision.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_slot_opening_authorization_reframe_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.md`

## Immediate Next Work

1. Follow `research/path2_proof_risk_burn_down_plan.md`: burn down proof risks
   before broad implementation. The current order is transparent relation
   knowledge soundness, query-opening no-export/ZK, norm/range binding
   semantics, public binding composition, QROM parameters, then verifier KATs.
2. Produce the exact Path 2 transparent relation arithmetization and extractor
   target: trace columns, 66 row constraints, public inputs, query-opening
   policy, and extracted opening map. This is proof work; only small executable
   probes should be added if they expose circularity or leakage.
   First pass artifact: `research/path2_transparent_relation_arithmetization_target.md`.
   It pins the 80-lane / 66-row semantic core and exposes two high-risk findings:
   raw transparent openings leak hidden digits/residues unless the proof family
   masks or simulates openings, and the 66-row arithmetic target does not prove
   SHA digest preimages for outer commitments without proof-friendly inner
   commitments or expensive digest circuits.
   Refinement artifact:
   `research/path2_inner_commitment_relation_refinement.md`. It makes the
   current blocker sharper: the 66-row core must be bound to one committed trace
   carried by the accepted payload/certificate path, that trace must link to the
   role commitments named by `Path2CreationProofBindingV1`, and opening-tuple
   account semantics must be arithmetized or carried as a named hidden-source
   theorem. Trace-binding audit:
   `research/path2_trace_commitment_binding_audit.md`; it keeps the current
   two-hop payload binding as preferred but only if the link equations close.
   Link-equation options:
   `research/path2_role_commitment_link_equation_options.md`; it keeps typed
   transparent trace subcommitments as the compact path and marks SHA digest
   handles dead for compact opening. Commitment API:
   `research/path2_transparent_commitment_api.md`; it pins parent root, role
   subroots, column profile, hiding/blinding rule, extractor contract, simulator
   contract, and public payload bytes. Proof-family gate:
   `research/path2_transparent_proof_family_gate.md`; vanilla raw trace opening
   is a negative control, and the active candidate is a ZK masked-trace IOP.
   Masked-transform artifact:
   `research/path2_masked_trace_transform.md`; it raises the private-lane floor
   to 138. Updated byte model:
   `research/path2_transparent_proof_byte_model.md`; masked planning floors are
   5520, 7524, and 9784 bytes for the 256, 512, and 1024 row bands. Range/
   account delta:
   `research/path2_masked_trace_range_account_delta.md`; conservative planning
   floors are now 6144, 8304, and 10720 bytes for those bands. Assumption:
   `research/path2_zk_masked_trace_iop_assumption.md`; it names
   `zkMaskedTraceIOPBindingAndHiding` and sets the failback threshold against
   the 16416-byte lattice-sigma floor. Selection checklist:
   `research/path2_masked_trace_iop_selection_checklist.md`. Commitment layout:
   `research/path2_masked_trace_commitment_layout.md`. Leaf encoding fork:
   `research/path2_masked_trace_leaf_encoding_fork.md`. Current theorem
   interface: `research/path2_two_view_trace_theorem_interface.md`.
   Two-view obligations:
   `research/path2_two_view_consistency_obligations.md`. Linear leakage probe:
   `research/path2_two_view_linear_leakage_probe.md`; fresh masks pass, reused
   masks leak. Column-profile rules:
   `research/path2_two_view_column_profile_rules.md`.
   Resolution pass: `research/path2_top_proof_risk_resolution_options.md`
   marks raw transparent openings and SHA-in-circuit compactness as dead paths.
   The selected theorem shape is a ZK transparent IOP with masked/simulatable
   hidden-column openings over proof-friendly inner commitments; outer SHA-256
   remains a public byte-binding and data-availability layer, not the compact
   arithmetic opening relation.
3. Classify every concrete trace column as public, masked, committed-hidden, or
   derived-public before choosing a final transparent proof family. Any sampled
   query opening that reveals an unmasked pad/projection/opening lane is a
   redesign trigger.
4. Revisit raw-centered versus base-128 digit openings as theorem statements
   before implementing range checks. If the clean binding theorem requires too
   many digit/range columns, update the proof design before verifier work.
5. Only after risks 1-4 have stable theorem statements, instantiate the
   deployable `Path2DirectCreationProofPayloadV1` verifier and promote
   `creation_time_hidden_source_soundness_break` from payload-boundary blocker
   to a concrete theorem assumption or proof.
