# Compact Signature Target TODOs

This file is a research checklist, not just an implementation backlog.  It now
keeps the historical branches because they explain why the current design has
the shape it has, but stale branches are marked explicitly.

Legend:

- `ACTIVE`: still part of the current committed-sigma/account-carried research
  and review plan.
- `STALE`: preserved for historical context, but superseded by a later branch
  or rejected by the current proof/attack evidence.
- checked items remain completed artifacts or completed negative results.

## Current Status Snapshot

Current leading direction: blockchain-oriented, stateful/account-carried
committed-sigma signatures over the `n=32`, `q=12289` local-delta matrix.  The
public representative verifier and strict public sigma equation are not sound
by themselves; both admit public post-challenge reconstruction attacks.  The
active proof direction is therefore:

```text
prechallenge witness-bound commitment
  + account-carried hidden-link evidence
  + strict sigma equation
  + bounded response-link kernel certificate
  => account-backed hidden witness or named break
```

Current concrete response-link target:

- response-link code: coordinate projection `[0, 3]`;
- response-difference box: `[-96,96]^23`;
- canonical matrix hash:
  `b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6`;
- internal exact enumeration status: closed, no nonzero bounded vector;
- standalone verifier:
  `scripts/research/verify_response_link_bounded_kernel_certificate.py`.
- audit manifest:
  `response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json`;
  payload hash
  `da10b823ad8f4142dfeccdbf7bbcddbb5e7148f420d8ff36687bae412d09fbf0`.
- Lean bridge:
  `Descender/ResponseLinkExactCertificate.lean` defines the exact-certificate
  summary shape, and
  `scripts/research/generate_response_link_exact_certificate_lean.py` generates
  `Descender/Generated/ResponseLinkExactCertificateN32Q12289Coords03B96.lean`.
  Lean now checks the audited summary status, response bound `96`, shortest
  coefficient bound `139`, and the arithmetic consequence that no nonzero
  response-link kernel vector with coefficient bound at most `96` can satisfy a
  shortest-bound lower bound of `139`.

Current byte posture:

- `250 B` is the exact account-carried bounded-kernel spend-transaction target,
  but it is not the current robust implementation target unless we prove a
  10-row challenge-action adapter with multi-query margin.
- `252 B` is the current smallest robust implementation target under the same
  reviewed account-link assumptions: it preserves the existing `Fq12` action
  budget while adding one masked-response byte.
- `254 B` hardened public transaction path is preserved only as a negative
  control: it is byte-competitive but structurally rejected because the public
  sigma equation still admits post-challenge representative reconstruction.
- The current reviewed account-state layout prices the account-carried link at
  `68 B`: 32-byte witness commitment, 32-byte randomizer commitment, and a
  4-byte `[0, 3]` response-link code.  These are account/state bytes, not
  spend-signature bytes.
- The original `69 B` active payload and `181 B` ideal stateful verifier are
  historical lower-bound/control artifacts, not deployable signature claims.

Current confidence posture:

- This is **not** deployable cryptography yet and should not be described as
  fully trusted or externally reviewed.
- Strong local evidence exists for the selected `[0,3]` response-link target:
  exact bounded enumeration excludes nonzero bad vectors in `[-96,96]^23`, and
  the red-team BKZ-23 sweep did not find a selected-target break.
- The design is structurally brittle: the bare public quotient path is
  confirmed broken, `22/23` one-coordinate response-link projections fail in
  the local sweep, and `7/253` two-coordinate alternatives have exact bad
  integer witnesses.  Therefore `[0,3]` is a fixed security parameter, not a
  tunable implementation choice.
- The Rust library now has canonical reviewed `250 B` and `252 B/Fq12`
  account-carried codecs, strict quotient digest maps, Fiat-Shamir transcript
  challenge-binding checkers, the `252 B/Fq12` sigma-equation checker, and a
  necessary public local-fiber consistency check plus public pad-opening
  consistency across repeated challenge blocks.  It also derives a canonical
  public pad-opening statement digest from those inferred block pads, implements
  the executable 32-byte hidden pad-seed-to-public-statement witness relation,
  and an executable hidden-source witness checker tying the account witness
  syndrome plus pad seed to a canonical byte-level hidden-source statement for
  the public signature context.  It also exposes a sanitized hidden-source
  proof callback boundary and a 32-byte opaque hidden-source proof carrier that
  gives the eventual compact proof verifier only the public statement plus
  carrier bytes, mapping proof failures to a fixed public reason.  There is now
  a strict `284 B` signature-carried envelope parser/encoder for `252 B/Fq12`
  plus the 32-byte carrier.  The named deployment verifier remains fail-closed
  on an explicit hidden-source proof-semantics blocker for the `284 B` path
  until those carrier bytes have real compact-proof semantics.  The production
  facade still wraps the legacy `254 B` callback path.
- For the selected `308 B/Fq12` projection-carrier route, the Rust boundary now
  exposes fail-closed verifier/exporter API stubs.  The 56-byte carrier decoder
  checks length and `12` canonical q12289 little-endian lanes, and the public
  bridge verifier checks those lanes, check bytes, and transcript binding
  against public statement inputs.  The checked-core and hidden-source break
  classifiers now produce typed external break records for supplied concrete
  failure classes, but the verifier still lacks executable semantic predicates
  that decide when those classifiers must be invoked.
- Named assumptions remain open around SHA-256 pad PRF behavior, QROM/Forking
  model losses, matrix/keygen distribution hardness, constant-time
  signer/sampler/keygen/verifier behavior, durable account-state lifecycle, and
  independent external replay/review.
- Deployment trust target: do not claim deployable cryptography until the
  fail-closed `308 B/Fq12` projection-carrier envelope has accepting
  hidden-source proof semantics and an accepting-verifier theorem.  Third-party
  review and battle testing are outside this local checklist, but the local
  target is still a complete, internally reviewable cryptographic argument:
  accepting signatures must imply either a valid hidden account witness/pad
  source or one of the named assumption breaks, with explicit leakage, sampler,
  state, and implementation boundaries.

Active next goals:

- [ ] ACTIVE: implement the semantic predicate replays for the selected
  `308 B/Fq12` projection-carrier envelope.  The current API boundary now
  decodes canonical carrier lanes and checks the public lane/check/transcript
  bridge, and all five classifier invocation boundaries can now export typed
  break records.  This still does not make the route deployable: several
  boundaries receive recomputed semantic values as inputs instead of deriving
  response differences, hidden opening projections, and pad PRF projections
  internally from byte-level material.  Internalizing those derivations is now
  the primary cryptographic blocker.
- [x] Add a hidden-source proof-semantics target artifact.
  `scripts/research/hidden_source_proof_semantics_target.py` generates
  `hidden_source_proof_semantics_target_n32_q12289.{json,md}` and pins the
  canonical public inputs, hidden witness inputs, carrier options, accepting
  verifier obligations, decision gates, and next proof steps for the
  fail-closed `284 B/Fq12` account-carried hidden-source envelope.
- [x] Add the exact hidden-source accepting-relation artifact.
  `scripts/research/hidden_source_accepting_relation_spec.py` generates
  `hidden_source_accepting_relation_spec_n32_q12289.{json,md}` and fixes the
  byte-level verifier phases, witness predicates, failure modes, theorem hooks,
  and negative controls that any accepting compact carrier must satisfy.
- [x] Add the hidden-source carrier construction decision artifact.
  `scripts/research/hidden_source_carrier_construction_decision.py` generates
  `hidden_source_carrier_construction_decision_n32_q12289.{json,md}`, rejects
  digest-only 32-byte acceptance, selects the state-assisted 32-byte carrier as
  the next research track, keeps a larger algebraic carrier as fallback, and
  records the state-creation/carrier-handle/extractor/leakage work needed to
  keep the `284 B` envelope plausible.
- [x] Add the state-assisted 32-byte carrier relation artifact.
  `scripts/research/hidden_source_state_assisted_carrier_relation.py` generates
  `hidden_source_state_assisted_carrier_relation_n32_q12289.{json,md}` and
  fixes the account-state creation frame, hidden-source opening commitment,
  carrier-as-handle predicates, single-use lifecycle binding,
  extractor-or-break cases, implementation hooks, and negative controls.  This
  still does not permit acceptance; it makes the state-assisted theorem target
  explicit.
- [x] Add the state-assisted extractor obligation artifact.
  `scripts/research/hidden_source_state_assisted_extractor_obligations.py`
  generates
  `hidden_source_state_assisted_extractor_obligations_n32_q12289.{json,md}` and
  records the exact extracted witness items, named-break rows, open assumptions,
  proof order, and fallback triggers for the 32-byte state-assisted route.
- [x] Add the state-assisted negative-control artifact.
  `scripts/research/hidden_source_state_assisted_negative_controls.py`
  generates
  `hidden_source_state_assisted_negative_controls_n32_q12289.{json,md}` and
  records digest-only, postchallenge opening, wrong-opening, replay, rollback,
  witness/pad mismatch, and cached-statement controls, distinguishing partially
  covered lifecycle tests from carrier-specific executable gaps.
- [x] Add the state-assisted opening-frame artifact.
  `scripts/research/hidden_source_state_assisted_opening_frame.py` generates
  `hidden_source_state_assisted_opening_frame_n32_q12289.{json,md}` and pins
  the SHA-256 opening-commitment and carrier-handle byte frames, field order,
  public/private byte accounting, rejection rules, and required test vectors
  for the 32-byte state-assisted carrier route.
- [x] Add deterministic opening-frame test vectors.
  `scripts/research/hidden_source_state_assisted_opening_frame_test_vectors.py`
  generates
  `hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.{json,md}`
  with canonical opening/carrier frame SHA-256 outputs plus field-order,
  domain/profile, public-context, slot, and private-opening mutation vectors
  that must change the digest and later become executable verifier tests.
- [x] Promote the opening-frame vectors into the Rust reference boundary.
  `tools/parameter-search/src/production_reference.rs` now exposes the
  state-assisted opening commitment and carrier-handle frame/digest helpers,
  plus a fail-closed carrier-handle check against the 32-byte hidden-source
  proof carrier.  `tools/production-reference/src/lib.rs` tests the canonical
  digests and mutation digests from
  `hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json`;
  `cargo test --manifest-path tools/production-reference/Cargo.toml` passes.
- [x] Add the state-assisted carrier verifier contract artifact.
  `scripts/research/hidden_source_state_assisted_carrier_verifier_contract.py`
  generates
  `hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.{json,md}`
  and pins the public inputs, state inputs, witness inputs, fail-closed
  verifier phases, theorem hooks, Rust carrier-handle boundary, and remaining
  blockers for composing the 32-byte carrier with the selected envelope.
- [x] Add the state-assisted account-state schema contract.
  `tools/parameter-search/src/production_reference.rs` now exposes a typed
  sidecar account-state slot/state for the prechallenge
  `hidden_source_opening_commitment`, restore anchor, account commitment, and
  unused bit, plus `verify_reviewed_fq12_state_assisted_slot_opening`.
  `scripts/research/hidden_source_state_assisted_account_state_schema_contract.py`
  records the schema, Rust boundary, negative-control coverage, and remaining
  storage/creation-time/hidden-relation blockers.
- [x] Add the state-assisted hidden-relation verifier contract.
  `scripts/research/hidden_source_state_assisted_hidden_relation_verifier_contract.py`
  generates
  `hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.{json,md}`
  and pins the public statement fields, private opening fields, verifier
  checks, extraction cases, implementation targets, and blockers that separate
  account-state opening equality from a real proof of the hidden witness/pad
  relation.
- [x] Add the state-assisted hidden-relation negative-control artifact.
  `scripts/research/hidden_source_state_assisted_hidden_relation_negative_controls.py`
  generates
  `hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.{json,md}`
  and records semantic attacks that must fail after carrier-handle and
  account-state checks pass: witness digest substitution, pad seed
  substitution, public pad-statement mismatch, relation projection mismatch,
  carrier context replay, false-statement acceptance, and precommitment bypass.
- [x] Add the state-assisted hidden-relation extractor theorem target.
  `scripts/research/hidden_source_state_assisted_hidden_relation_extractor_theorem.py`
  generates
  `hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.{json,md}`
  and states the extractor-or-break theorem for the hidden-relation layer:
  accepted semantics must yield the canonical hidden-source statement,
  account-witness opening, pad-seed opening, carrier-handle opening, and
  lifecycle witness, or name a concrete hash/RO, commitment, proof, pad,
  lifecycle, checked-core, QROM, or keygen break.
- [x] Add the compact hidden-relation proof-instantiation decision artifact.
  `scripts/research/hidden_source_state_assisted_compact_proof_instantiation.py`
  generates
  `hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.{json,md}`
  and rejects digest-only acceptance, keeps the 32-byte state-assisted
  random-oracle proof-handle route as a fail-closed research track, records the
  explicit nonstandard proof-handle assumption required to keep `284 B`, and
  names the `40` to `64` byte algebraic carrier fallback if executable controls
  or review reject that assumption.
- [x] Add the state-assisted RO proof-handle assumption/API artifact.
  `scripts/research/hidden_source_state_assisted_ro_pok_assumption_api.py`
  generates `hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.{json,md}`
  and fixes the nonstandard `state_assisted_hidden_relation_ro_pok` assumption,
  the public-only verifier API, transcript frame, forbidden secret inputs,
  fail-closed acceptance rules, and review questions that must be answered
  before the 32-byte proof-handle route can be promoted.
- [x] Add the state-assisted RO proof-handle byte negative-test artifact.
  `scripts/research/hidden_source_state_assisted_ro_pok_byte_negative_tests.py`
  generates
  `hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.{json,md}`
  and converts the semantic RO-PoK controls into serialized-frame mutations:
  statement-frame byte/length changes, opening-commitment changes, transcript
  digest changes, carrier mutation, domain/profile confusion, digest-only
  oracle acceptance, and fixed-error-surface probing.
- [x] Add the default fail-closed Rust boundary for the state-assisted RO-PoK
  proof handle.  `ProductionReviewedFq12StateAssistedRoPokPublicInput`,
  `reviewed_fq12_state_assisted_ro_pok_transcript_frame`,
  `reviewed_fq12_state_assisted_ro_pok_transcript_digest`, and
  `verify_reviewed_fq12_state_assisted_ro_pok` now expose the public-only byte
  surface.  The Rust test
  `state_assisted_ro_pok_boundary_is_byte_bound_and_fail_closed` checks
  domain/profile constants, statement-frame byte/length binding,
  opening/transcript/carrier digest binding, digest-only carrier rejection, and
  fixed public rejection.  This does not make the 32-byte proof handle
  accepting or discharge the explicit RO-PoK assumption.
- [x] Add the algebraic carrier fallback byte model.
  `scripts/research/hidden_source_algebraic_carrier_fallback_byte_model.py`
  generates
  `hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.{json,md}`
  and fixes the projection-carrier decision surface for `40`, `48`, `56`, and
  `64` byte carriers.  The selected branch is now the fail-closed `308 B/Fq12`
  projection-carrier hidden-source envelope, using the `56` byte carrier
  because it exposes twelve packed q12289 elements plus check/flag bytes while
  staying small enough for the blockchain target.
- [x] Instantiate the 56-byte fallback carrier projection-relation target.
  `scripts/research/hidden_source_fallback_carrier_projection_relation.py`
  generates
  `hidden_source_fallback_carrier_projection_relation_n32_q12289.{json,md}`.
  This makes the selected `308 B/Fq12` route concrete: a `56` byte carrier
  with a 32-byte context digest, `21`
  bytes for twelve packed q12289 projection elements, and `3` check/flag
  bytes.  It records the frame, provisional acceptance predicate, negative
  controls, and proof obligations needed for the projection-carrier route.
- [x] Add the 56-byte fallback projection-map search plan.
  `scripts/research/hidden_source_fallback_projection_map_search.py`
  generates `hidden_source_fallback_projection_map_search_n32_q12289.{json,md}`
  and fixes the candidate families, scoring criteria, rejection gates, and
  required experiments for choosing the twelve q12289 projection elements.  The
  selected family to certificate first is `balanced_witness_pad_subview`,
  because the projection route exists to make witness and pad extraction
  public-checkable instead of relying on an opaque 32-byte proof handle.
- [x] Add the deterministic fallback projection-map enumerator.
  `scripts/research/hidden_source_fallback_projection_map_enumerator.py`
  generates
  `hidden_source_fallback_projection_map_enumerator_n32_q12289.{json,md}` and
  materializes four candidate maps for the selected `56` byte projection
  carrier.  The current top-ranked candidate is
  `balanced_witness_pad_subview`; it is bound to the next exact certificate
  attempt but does not make the verifier accepting.
- [x] Add the fallback projection-map certificate-attempt target.
  `scripts/research/hidden_source_fallback_projection_map_certificate_attempt.py`
  generates
  `hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.{json,md}`
  and binds the top-ranked `balanced_witness_pad_subview` map digest to the
  exact bounded-kernel, semantic-control, decoder, and leakage checks required
  before the selected `308 B/Fq12` route can become accepting.
- [x] Add the selected projection-map exact certificate specification.
  `scripts/research/hidden_source_fallback_projection_map_exact_certificate_spec.py`
  generates
  `hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.{json,md}`.
  It canonicalizes and hashes the selected twelve projection rows, runs cheap
  structural controls, and records that this is not the same as the existing
  23-column response-link coordinate certificate.  The exact generalized
  hidden-source projection replay remains open.
- [x] Pin Rust projection lane constants to the selected map certificate.
  `scripts/research/hidden_source_fallback_projection_lane_map_consistency.py`
  generates
  `hidden_source_fallback_projection_lane_map_consistency_n32_q12289.{json,md}`.
  It verifies that `HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS` and
  `HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_COEFFICIENTS` match the selected
  projection rows by lane order.  This caught and fixed a real drift: the Rust
  constants had the same twelve symbols in stale order, and the pad PRF
  derivation compared raw pad residues instead of coefficient-weighted
  projection values.
- [x] Build the selected projection matrix.
  `scripts/research/hidden_source_fallback_projection_matrix_builder.py`
  generates
  `hidden_source_fallback_projection_matrix_builder_n32_q12289.{json,md}`.
  It materializes the selected twelve rows as a `12x64` q12289 sparse matrix,
  computes rank/nullity, and records that the projection-only matrix is not a
  certificate because its remaining kernel must be closed only after the
  checked-core and semantic decoder lift.
- [x] Add the selected projection checked-core lift ledger.
  `scripts/research/hidden_source_fallback_projection_checked_core_lift.py`
  generates
  `hidden_source_fallback_projection_checked_core_lift_n32_q12289.{json,md}`.
  It combines the replayed checked-core response-link certificate with the
  selected projection matrix at the rank-accounting boundary: checked-core
  rank `14` plus projection rank `12` over `87` direct-sum variables leaves
  direct-sum nullity `61` until the bridge equations and semantic decoder
  constraints are instantiated.  This is intentionally fail-closed and does
  not reuse the 23-column response-link certificate as a hidden-source
  projection certificate.
- [x] Specify the selected projection semantic bridge.
  `scripts/research/hidden_source_fallback_projection_semantic_bridge_spec.py`
  generates
  `hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.{json,md}`.
  It fixes the `56` byte projection carrier layout as `24` bytes of canonical
  q12289 projection lanes, `3` check bytes, and `29` reserved/transcript
  binding bytes.  The carrier-to-projection lane equations are specified for
  all twelve lanes, while the projection-to-checked-core and
  projection-to-hidden-source equations remain open and fail-closed for the
  next exact replay.
- [x] Replay the selected projection carrier bridge boundary.
  `scripts/research/hidden_source_fallback_projection_bridge_exact_replay.py`
  generates
  `hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.{json,md}`.
  It verifies the twelve carrier-to-projection lane equations have rank `12`
  and records the double-counting guard: treating carrier lanes as public
  constants leaves the same `61` open nullity as the checked-core lift, while
  treating carrier lane deltas as variables adds `12` equations and `12`
  variables and also leaves nullity `61`.  The exact infeasibility solver is
  intentionally blocked until projection-to-checked-core and
  projection-to-hidden-source bridge equations are derived.
- [x] Classify the selected projection bridge equation derivation gap.
  `scripts/research/hidden_source_fallback_projection_bridge_equation_derivation.py`
  generates
  `hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.{json,md}`.
  It derives all `12` public carrier lane definition equations, records that
  they add `0` checked-core coupling equations, and keeps the bounded solver
  blocked with the current open nullity `61`.  The next decision is now
  narrower: choose a checked-core coupling route or reduce the missing coupling
  to a named `checked_core_or_response_link_break`.
- [x] Rank checked-core coupling options for the selected projection route.
  `scripts/research/hidden_source_fallback_projection_checked_core_coupling_options.py`
  generates
  `hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.{json,md}`.
  It compares direct linear lift, checked-core break reduction, response-link
  anchored map redesign, and hash-seeded statement-local projection.  The
  selected next attempt is `direct_linear_lift` because it is the least
  speculative byte-preserving route; acceptance remains fail-closed and the
  fallback route is a named `checked_core_or_response_link_break` reduction
  before any map redesign.
- [x] Attempt the direct linear checked-core lift.
  `scripts/research/hidden_source_fallback_projection_direct_linear_lift_attempt.py`
  generates
  `hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.{json,md}`.
  It confirms the response-link matrix and selected projection matrix are both
  present, but no public `23` checked-core-response-column to `64`
  hidden-source-projection-column map exists in the current artifacts.  It
  derives `0` checked-core coupling rows, keeps the exact solver blocked,
  preserves the `308 B/Fq12` byte target fail-closed, and moves to the
  `checked_core_break_reduction` fallback before any map redesign.
- [x] Start the checked-core break-reduction fallback.
  `scripts/research/hidden_source_fallback_projection_checked_core_break_reduction.py`
  generates
  `hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.{json,md}`.
  It confirms the named `checked_core_or_response_link_break` target exists in
  the extractor theorem and records the reduction shape, while keeping the
  proof fail-closed until a public bridge predicate, concrete break-export
  contract, and negative acceptance test are instantiated.
- [x] Specify the public bridge predicate for the projection carrier.
  `scripts/research/hidden_source_fallback_projection_public_bridge_predicate.py`
  generates
  `hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.{json,md}`.
  It makes the verifier-visible bridge concrete: `56` carrier bytes, `12`
  canonical q12289 lanes in `bytes[0:24]`, fail-closed check bytes in
  `bytes[24:27]`, transcript-binding bytes in `bytes[27:56]`, and all `12`
  carrier-to-projection lane equations.  It confirms the lane symbols match
  the selected projection symbols, but still leaves checked-core and
  hidden-source opening bridge failures to the break-export contract.
- [x] Specify the projection bridge break-export contract.
  `scripts/research/hidden_source_fallback_projection_break_export_contract.py`
  generates
  `hidden_source_fallback_projection_break_export_contract_n32_q12289.{json,md}`.
  It defines concrete external review records for
  `checked_core_or_response_link_break`, `hidden_relation_proof_soundness_break`,
  `account_commitment_binding_break`, and `pad_prf_or_pad_opening_break`.  These
  records include carrier bytes, decoded q12289 lanes, checked-core response
  vectors or hidden-source opening frames as appropriate, and add `0` deployed
  signature bytes.  The exporter code and negative tests remain open.
- [x] Specify the projection bridge break-export negative tests.
  `scripts/research/hidden_source_fallback_projection_break_export_negative_tests.py`
  generates
  `hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.{json,md}`.
  It records `31` negative tests: noncanonical and mismatched mutations for all
  `12` carrier lanes, check-byte and transcript-binding mutations, checked-core
  response and response-link mismatches, hidden-source opening mismatch,
  account commitment split opening, and pad-PRF projection mismatch.  Every
  export-producing test references a concrete break-export record and adds `0`
  deployed signature bytes; implementation remains open.
- [x] Close the map-redesign versus bridge-row fork exposed by exact replay.
  `scripts/research/hidden_source_fallback_projection_map_redesign_or_bridge_rows.py`
  generates
  `hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.{json,md}`.
  It consumes the exported exact survivor, confirms the survivor sits in an
  inactive projection coordinate, rejects accepting the current map, and rejects
  a map-only lane expansion under the fixed `56 B`/`12`-lane carrier.  The
  selected byte-preserving path is bridge rows or break-export implementation;
  map redesign is deferred to fresh relation research if that path fails.
- [x] Bind bridge negative tests to a verifier/exporter contract.
  `scripts/research/hidden_source_fallback_projection_verifier_exporter_contract.py`
  generates
  `hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.{json,md}`.
  It routes all `31` negative tests to terminal verifier states:
  `reject_before_acceptance`, `accept`, `export_break_record`, or
  `internal_error_fail_closed`.  It specifies four API requirements for
  carrier decoding, public bridge verification, checked-core break
  classification, and hidden-source break classification.  Those API
  boundaries are now implemented as fail-closed replay/export boundaries, but
  semantic predicate replay is still incomplete, so the proof remains
  fail-closed and adds `0` deployed signature bytes.
- [x] Audit the verifier/exporter implementation gap.
  `scripts/research/hidden_source_fallback_projection_verifier_exporter_implementation_gap.py`
  generates
  `hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.{json,md}`.
  It scans the production-reference boundary for the required projection-carrier
  decoder, bridge verifier, checked-core classifier, hidden-source classifier,
  break-export type, and negative-test harness.  The audit is now state-aware:
  it records present versus missing symbols and still keeps acceptance
  fail-closed until the concrete bridge predicates and classifiers are
  implemented.
- [x] Add fail-closed verifier/exporter API stubs.
  `scripts/research/hidden_source_fallback_projection_verifier_exporter_api_stubs.py`
  generates
  `hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.{json,md}`.
  The production-reference boundary now exposes all six expected symbols:
  carrier decoder, public bridge verifier, checked-core classifier,
  hidden-source classifier, break-export type, and negative-test harness.  Only
  carrier length/canonical-lane decoding, public bridge equality checking, and
  typed external break-record packaging are implemented.  This adds `0`
  deployed signature bytes and does not make the `308 B/Fq12` route deployable.
- [x] Add the semantic classifier invocation contract.
  `scripts/research/hidden_source_fallback_projection_semantic_classifier_invocation_contract.py`
  generates
  `hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.{json,md}`.
  It records the five remaining invocation rules: checked-core response
  mismatch, response-link kernel certificate mismatch, hidden-source opening
  projection mismatch, account commitment split opening, and pad PRF/projection
  derivation mismatch.  All classifier export boundaries exist, but all five
  semantic predicate replays remain open and acceptance is still fail-closed.
- [x] Add the first semantic predicate invocation boundary.
  `scripts/research/hidden_source_fallback_projection_semantic_predicate_implementation_plan.py`
  generates
  `hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.{json,md}`.
  The Rust boundary now includes
  `verify_hidden_source_fallback_checked_core_or_export`, which replays the
  existing reviewed Fq12 checked-core verifier and invokes the typed
  checked-core/response-link export classifier on failure.  This implements
  one of the five invocation boundaries, while response-difference derivation
  and the other four semantic predicate replays remain open.
- [x] Add the response-link kernel replay boundary.
  `scripts/research/hidden_source_fallback_projection_response_link_kernel_replay_boundary.py`
  generates
  `hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.{json,md}`.
  The Rust boundary now exposes the pinned `[0,3]`, bound-`96`
  response-link certificate summary and
  `verify_hidden_source_fallback_response_link_kernel_or_export`.  It validates
  the certificate identity, enforces canonical bounded response-difference
  coordinates, accepts the zero vector, and exports a
  `checked_core_or_response_link_break` for any nonzero bounded vector that
  survives the `[0,3]` link.  This makes two of the five semantic invocation
  boundaries executable at this checkpoint.
- [x] Add the hidden opening projection equality boundary.
  `scripts/research/hidden_source_fallback_projection_opening_projection_boundary.py`
  generates
  `hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.{json,md}`.
  The Rust boundary now exposes
  `HiddenSourceFallbackProjectionOpeningProjectionReplayInput` and
  `verify_hidden_source_fallback_opening_projection_or_export`.  It decodes and
  replays the 56-byte carrier, rejects noncanonical claimed or recomputed
  q12289 lanes, accepts only when carrier lanes, claimed projection symbols,
  and recomputed opening projections agree across all 12 lanes, and otherwise
  exports the typed hidden-opening break record with a deterministic first-lane
  mismatch label.  This makes three of the five semantic invocation boundaries
  executable; account split and pad PRF projection replays remain open.
- [x] Add the account commitment split-opening boundary.
  `scripts/research/hidden_source_fallback_projection_account_commitment_split_boundary.py`
  generates
  `hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.{json,md}`.
  The Rust boundary now exposes
  `HiddenSourceFallbackProjectionAccountCommitmentSplitReplayInput` and
  `verify_hidden_source_fallback_account_commitment_split_or_export`.  It
  replays the carrier, rejects empty commitments or opening frames, accepts
  equal semantic values, and exports the typed account-commitment binding break
  when two supplied openings for the same commitment carry different semantic
  values.  This makes four of the five semantic invocation boundaries
  executable; pad PRF projection derivation remains open.
- [x] Add the pad PRF projection derivation boundary.
  `scripts/research/hidden_source_fallback_projection_pad_prf_projection_boundary.py`
  generates
  `hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.{json,md}`.
  The Rust boundary now exposes
  `HiddenSourceFallbackProjectionPadPrfProjectionReplayInput` and
  `verify_hidden_source_fallback_pad_prf_projection_or_export`.  It replays the
  carrier, rejects unknown or non-pad projection symbols, rejects noncanonical
  q12289 recomputed pad values, accepts only when recomputed values match their
  selected carrier lanes, and exports the typed pad/opening break on mismatch.
  This makes all five semantic invocation boundaries executable, while
  byte-level derivation of the recomputed values remains to be internalized.
- [x] Internalize pad PRF projection derivation from the byte-level pad seed.
  `scripts/research/hidden_source_fallback_projection_byte_level_derivation_internalization.py`
  generates
  `hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.{json,md}`.
  The Rust boundary now exposes
  `HiddenSourceFallbackProjectionPadPrfSeedReplayInput`,
  `hidden_source_fallback_pad_prf_projection_values_from_seed`, and
  `verify_hidden_source_fallback_pad_prf_seed_or_export`.  This derives the
  selected pad projection values from the 32-byte pad seed through the existing
  reviewed Fq12 SHA-256 rejection-sampled pad PRF, then feeds those values into
  the pad projection export boundary.  Mutating the pad seed now exports a
  pad/opening break against the same carrier.
- [x] Internalize account-witness projection derivation from an expanded opening.
  `tools/parameter-search/src/production_reference.rs` now exposes
  `HiddenSourceFallbackProjectionAccountWitnessOpening`,
  `hidden_source_fallback_account_projection_values_from_opening`,
  `hidden_source_fallback_projection_values_from_hidden_material`, and
  `verify_hidden_source_fallback_hidden_material_projection_or_export`.  The
  expanded opening carries exactly the six selected account source values, and
  the verifier applies the pinned lane coefficients before comparing all 12
  recomputed hidden-material lanes against the carrier.  This removes the
  immediate account-lane derivation blocker, but the expanded opening still
  needs commitment binding to the account witness commitment before the route
  can become accepting.
- [x] Add fail-closed commitment binding for the expanded account opening.
  `scripts/research/hidden_source_fallback_projection_expanded_opening_commitment_binding.py`
  generates
  `hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.{json,md}`.
  The Rust boundary now exposes
  `HIDDEN_SOURCE_FALLBACK_EXPANDED_OPENING_COMMITMENT_DOMAIN`,
  `HiddenSourceFallbackProjectionExpandedOpeningCommitmentReplayInput`,
  `hidden_source_fallback_expanded_account_opening_commitment`, and
  `verify_hidden_source_fallback_expanded_opening_commitment`.  The verifier
  hashes the six selected account source values together with the selected
  lane indices, symbols, coefficients, and q/layout metadata, and rejects
  before acceptance unless the digest equals the supplied expanded-opening
  commitment.  Account-state wiring and full opening-frame decoding are now
  covered by the selected projection lifecycle record below, so the remaining
  work is carrying that durable reference state into the target-chain storage
  model.
- [x] Wire the projection expanded-opening commitment into account state.
  `scripts/research/hidden_source_fallback_projection_account_state_opening_commitment_wiring.py`
  generates
  `hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.{json,md}`.
  The Rust boundary now exposes
  `HiddenSourceFallbackProjectionAccountSlotFrameSeed`,
  `hidden_source_fallback_projection_account_state_from_slot_frames`,
  `ProductionReviewedFq12StateAssistedAccountSlotRecord`,
  `reviewed_fq12_state_assisted_account_state_from_slot_records`,
  `ProductionReviewedFq12StateAssistedLifecycleSnapshot`,
  `ProductionReviewedFq12StateAssistedAccountLifecycleManager`,
  `ProductionReviewedFq12StateAssistedLifecycleFileStore`,
  `ProductionSelectedProjectionConsensusTransition`,
  `apply_selected_projection_consensus_opening_frame`,
  `verify_hidden_source_fallback_expanded_opening_state_slot`, and
  `verify_hidden_source_fallback_expanded_opening_state_slot_frame`.  Account
  creation decodes the canonical account-opening frame, derives the expanded
  opening commitment, stores it as the slot's
  `hidden_source_opening_commitment`, and serializes that field in the
  state-assisted slot record.  The selected projection lifecycle store now
  snapshots that richer record, enforces monotonic snapshot/restore-anchor
  checks, burns reserved projection slots, and consumes only after the
  expanded-opening frame matches the stored commitment.  The consensus
  transition also checks spent-ledger uniqueness, records the spent key, and
  returns both the next lifecycle snapshot and next spent-record list.  This
  makes the selected account projection values state-bound and
  durable-reference-visible without adding signature bytes.  The later
  target-chain adapter item carries this durable state into a deployment-facing
  database contract; remaining work is binding that contract to a specific
  node database and completing third-party storage review.
- [x] Add selected projection target-chain storage wiring and external storage
  review packet.  `ProductionSelectedProjectionChainStateFileStores` wires the
  selected projection lifecycle store, spent-slot ledger store, and separate
  `ProductionTrustedRestoreAnchorFileStore` together.  The deployment-facing
  reference path now rejects missing trusted anchors, rejects unapproved anchor
  placements such as `account_state_backup`, updates anchors monotonically
  under the canonical public-key identity, applies selected projection spends
  through `apply_opening_frame_and_save`, serializes lifecycle/spent-ledger/
  trusted-anchor writers, and records spent slots before exposing the consumed
  lifecycle state.  `scripts/research/external_storage_review_packet.py`
  generates `external_storage_review_packet_n32_q12289.{json,md}` with the
  external reviewer checklist for atomic triple-write semantics, trusted-anchor
  separation, identity-bound spent keys, crash recovery, and target-chain
  database replacement.
- [x] Add target-chain database adapter contract.  `ProductionSelectedProjectionTargetChainDb`
  defines the deployment-facing load/commit contract for selected projection
  lifecycle snapshots, spent-slot records, and trusted restore anchors.
  `ProductionSelectedProjectionTargetChainDbAdapter::apply_opening_frame_and_commit`
  runs the reviewed consensus transition and commits the next lifecycle,
  spent-ledger, and monotonic anchor outputs through one target-chain hook.
  `ProductionInMemorySelectedProjectionTargetChainDb` is the executable
  conformance backend used by tests; the remaining deployment work is binding
  this contract to a specific node database and getting external storage review.
- [x] Add transactional KV database binding.  `ProductionTargetChainTransactionalKv`
  is the node-database shape: `get` plus atomic `commit_batch`.
  `ProductionSelectedProjectionTargetChainKvDb` maps the selected projection
  lifecycle snapshot, spent-slot records, and trusted anchor to deterministic
  schema-tagged keys under `PRODUCTION_TARGET_CHAIN_KV_NAMESPACE`.
  `ProductionInMemoryTransactionalKv` tests successful commits, replay
  rejection, and failed transaction rollback/no partial mutation.  Remaining
  deployment work is implementing this KV trait for a concrete chain database
  engine and external storage review.
- [x] Add durable file-backed transactional KV reference engine.
  `ProductionFileBackedTransactionalKv` implements the same target-chain KV
  trait using a schema-checked JSON map, writer lock, temp-file fsync, and
  atomic rename.  Tests reload initialized state, reload a committed selected
  projection spend, reject concurrent writers, and ignore an uncommitted temp
  file.  This gives the external storage reviewer an executable durable
  baseline; production still needs the target chain's canonical database
  engine wired behind the same trait.
- [x] Add target-chain database adapter profile gate.
  `ProductionTargetChainDatabaseAdapterProfile` is the production-claim object
  for a concrete chain integration: it names the chain id, canonical database
  engine, single-transaction semantics, trusted-anchor placement, deterministic
  KV namespace/value schemas, public-key identity rule, spent-key shape, and
  external storage review digest.  The validator rejects reference engines,
  restorable account-backup anchors, non-atomic semantics, schema drift,
  ambiguous identity/key rules, and a missing review digest.  This turns the
  remaining target-chain binding into a checkable object; production still
  needs an actual chain DB implementation and reviewed digest.
- [x] Gate the KV-backed adapter behind the target-chain profile.
  `ProductionProfiledTargetChainKvDb` wraps the selected projection
  transactional-KV adapter only after the adapter profile validates, rejects
  the in-memory/file-backed reference KV engines by construction, and enforces
  the profile's trusted-anchor placement on account initialization and commit.
  Tests use a canonical-chain test double to prove the positive path while
  keeping the reference engines barred from production-shaped construction.
- [x] Add canonical-node KV store boundary.
  `ProductionCanonicalNodeKvStore` is the chain-facing database trait:
  implementations must expose their canonical database engine, load committed
  values under the validated adapter profile, and commit a full write batch
  under that same profile.  `ProductionCanonicalNodeKvAdapter` checks the
  profile/engine match, rejects reference stores, implements the transactional
  KV trait, and `production_profiled_target_chain_db_from_canonical_store`
  builds the profiled selected-projection DB from a canonical store.  Tests
  cover engine mismatch and the positive canonical-store path.
- [x] Add concrete local canonical node KV backend.
  `ProductionLocalCanonicalNodeKvStore` implements the canonical-node store
  trait with a schema-checked JSON map, database-engine tag, adapter-profile
  digest, writer lock, temp-file fsync, and atomic rename.  The production
  reference tests initialize selected-projection state through the profiled
  canonical adapter, reload it from disk, reject concurrent writers, and reject
  reads under a different adapter profile.  This is the executable local
  canonical backend for storage-review development; the production target chain
  still needs its own `ProductionCanonicalNodeKvStore` implementation.
- [x] Add reusable canonical-store conformance gate.
  `production_canonical_node_store_conformance_check` is the test harness that
  every candidate target-chain `ProductionCanonicalNodeKvStore` implementation
  must pass before review.  It validates the adapter profile, initializes
  selected-projection account state, commits an account-opening spend through
  the profiled canonical store, reopens the store, confirms lifecycle, trusted
  anchor, and spent-ledger persistence, and confirms replay rejection from the
  reloaded state.  The local canonical JSON backend passes this harness; a real
  chain DB adapter still needs to be plugged in and externally reviewed.
- [x] Add concrete SQLite canonical-store adapter.
  `ProductionSqliteCanonicalNodeKvStore` implements the same
  `ProductionCanonicalNodeKvStore` boundary using an embedded SQLite database
  with schema metadata, canonical-engine metadata, adapter-profile digest
  binding, WAL journaling, FULL synchronous durability, and one SQL transaction
  per canonical batch.  It passes the canonical-store conformance gate, reloads
  committed state after reopen, rejects profile-mismatched reads, and gives
  reviewers a database-level stand-in before the final chain node database is
  selected.
- [x] Add mock finalized blockchain reference.
  `ProductionMockBlockchain` wraps the local canonical backend in a small
  finalized-block simulator.  It initializes selected-projection account state,
  finalizes blocks of account-opening transactions through the profiled
  canonical DB, records accepted spend receipts, publishes deterministic
  finalized state roots, rejects replay in later blocks, and reloads finalized
  state from disk.  This gives reviewers an end-to-end local chain harness for
  storage, replay, and anchor semantics without claiming to be a production
  blockchain runtime.
- [x] Close verifier/exporter bridge semantics fail-closed.
  `scripts/research/hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure.py`
  generates
  `hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.{json,md}`.
  It ties together carrier decoding, public bridge verification, break-export
  classifiers, all five semantic predicate invocations, byte-level derivations,
  and leakage accounting.  All verifier/exporter bridge semantics rows are
  closed with `0` deployed signature bytes, but the route still cannot accept:
  exact infeasibility is not proved and the bounded survivor remains exported.
- [x] Resolve the exported exact survivor through active-subspace replay.
  `scripts/research/hidden_source_fallback_projection_active_subspace_exact_replay.py`
  generates
  `hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.{json,md}`.
  The old bounded vector is outside the verifier-visible relation because it
  mutates an inactive hidden-source coordinate, not one of the `12` selected
  projection lanes decoded and checked by the verifier/exporter bridge.  The
  active projection subspace has rank `12`, nullity `0`, imports the exact
  response-link infeasibility certificate, excludes `52` inactive coordinates
  from the accepting relation, and remains fail-closed with `deployment_ready =
  false` pending production preflight and third-party review.
- [x] Add a Lean bridge for active-subspace survivor exclusion.
  `Descender/HiddenSourceActiveSubspace.lean` defines the active-subspace
  certificate shape, and
  `scripts/research/generate_hidden_source_active_subspace_lean.py` generates
  `Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean` from the
  active-subspace replay JSON.  Lean now checks the concrete `12` active
  lanes, no-duplicate lane/source-column facts, old survivor exclusion from the
  active source-column list, inactive-column count `52`, and active-lane
  nullity zero.  The review replay builds this generated Lean target so the
  exact survivor closure is no longer only a JSON/prose claim.
- [x] Add the RO-PoK-vs-fallback decision gate.
  `scripts/research/hidden_source_ro_pok_vs_fallback_decision_gate.py`
  generates `hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.{json,md}`
  and records the route pivot: the fail-closed `308 B/Fq12`
  projection-carrier hidden-source envelope is now selected, while the
  `284 B/Fq12` RO-PoK handle route is archived as an aggressive research
  option.
- [x] Pivot the selected deployment target to the 308-byte projection route.
  The selected branch is now the fail-closed `308 B/Fq12` projection-carrier hidden-source envelope.  The `284 B/Fq12` RO-PoK handle route remains useful
  research context, but it is no longer the deployment-track target because it
  depends on a nonstandard proof-handle assumption.  The selected 308-byte/Fq12 projection-carrier deployment verifier is fail-closed until exact fallback certificate and leakage semantics are implemented; there is no accepting 308-byte/Fq12 verifier yet.
- [x] Add the 32-byte RO-PoK external-review packet.
  `scripts/research/hidden_source_ro_pok_external_review_packet.py`
  generates `hidden_source_ro_pok_external_review_packet_n32_q12289.{json,md}`
  and isolates the review question for the fail-closed `284 B/Fq12`
  account-carried hidden-source envelope: either reviewers accept the
  nonstandard RO-PoK assumption with explicit extractor, byte-binding, and
  leakage conditions, or we activate the fail-closed `308 B/Fq12` projection
  fallback certificate path.
- [x] Add the conditional RO-PoK leakage/global-composition theorem target.
  `scripts/research/hidden_source_ro_pok_leakage_global_composition_theorem.py`
  generates
  `hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.{json,md}`.
  This records the leakage channels, composition edges, proof obligations, and
  named breaks that must close if external review accepts the 32-byte RO-PoK
  assumption.  It remains fail-closed and does not make the verifier accepting.
- [x] Add the RO-PoK leakage simulator contract.
  `scripts/research/hidden_source_ro_pok_leakage_simulator.py` generates
  `hidden_source_ro_pok_leakage_simulator_n32_q12289.{json,md}` and specifies
  the public inputs, forbidden hidden values, simulator steps,
  indistinguishability claims, required tests, and open gaps for the selected
  32-byte RO-PoK route.  It remains fail-closed because the simulator still
  depends on external acceptance of the nonstandard RO-PoK simulation oracle,
  uniform reject behavior, and checked-core leakage composition.
- [ ] ACTIVE: instantiate the hidden-source proof carrier.  Specify the exact
  relation, public inputs, witness inputs, transcript frame, carrier byte
  layout, acceptance predicate, domain separators, rejection behavior, and test
  vectors.  The carrier must bind the account witness commitment opening,
  pad-seed opening, signature digest, public pad-opening statement digest, slot
  id, public-key digest, and message digest.
- [ ] ACTIVE: prove hidden-source proof soundness.  Show that any accepting
  `284 B/Fq12` signature yields either a valid hidden witness/pad source for the
  canonical statement or a named break in the proof assumption, Fiat-Shamir/QROM
  model, transcript binding, response-link certificate, pad PRF, or matrix
  keygen distribution.
- [ ] ACTIVE: prove hidden-source zero-knowledge/leakage bounds.  Account for
  every byte in the 32-byte carrier or revised carrier, the public statement,
  repeated-block pad-opening residues, observed-block bitmap/count, and
  transcript digest.  The proof must explicitly rule out unintended leakage of
  the account witness syndrome, pad seed, per-slot state, or signer randomness.
- [ ] ACTIVE: redo sampler and pad-source cryptoanalysis against the final
  accepting proof semantics.  The current sampler/pad reviews are boundary
  checks for the fail-closed path; after carrier semantics are fixed, confirm
  that the final production sampler is secret-independent or leakage-bounded in
  exactly the way charged by the theorem.
- [ ] ACTIVE: produce the accepting-verifier theorem.  The final theorem should
  state the exact game: for the selected `n=32`, `q=12289`, `252 B/Fq12 + carrier`
  account-carried signature, any successful forgery against the deployment
  verifier implies a break of one of the named assumptions or a violation of the
  exact bounded-kernel response-link certificate.  It must bind concrete byte
  sizes, transcript domains, challenge-query budgets, account-state rules, and
  deployment preflight blocker removal conditions.
- [ ] ACTIVE: add negative-control attacks for the accepting theorem.  Preserve
  the confirmed broken bare quotient path, failed one-coordinate response-link
  variants, bad two-coordinate witnesses, malformed carrier semantics, replayed
  account slots, state rollback, transcript mismatch, and sampler-boundary
  violations as executable or proof-level tests that fail if the final theorem
  accidentally accepts an unsafe variant.
- [ ] ACTIVE: complete implementation-hardening cryptoanalysis after the
  accepting semantics exist.  Audit constant-time signer/sampler/keygen/verifier
  behavior, callback sanitization, retry shape, storage atomicity, trusted
  restore-anchor placement, replay ledger behavior, and chain reorg policy
  against the final accepting verifier, not the current fail-closed surface.
- [ ] ACTIVE: define deployment-readiness exit criteria.  A local deployment
  candidate requires: accepting carrier semantics, soundness theorem,
  zero-knowledge/leakage theorem, sampler/pad-source theorem, QROM/query budget
  review, matrix/keygen hardness review, constant-time/storage audit, passing
  negative controls, passing packet reproducibility, and a preflight certificate
  with no terminal deployment blockers.  External review remains required before
  real-world use even if all local criteria pass.
- [x] Instantiate the concrete account commitment representation as a
  `ReviewedBoundedKernelAccountLinkInstantiation` and pin its online-plus-state
  byte accounting.
- [ ] ACTIVE: complete external response-link certificate review by independent
  replay of the audit manifest and, ideally, third-party SVP/BKZ validation.
- [x] Add a committed-sigma/account-carried byte cost model with external
  state-of-the-art size benchmarks and source links.
- [ ] ACTIVE: extend the committed-sigma/account-carried cost model with
  independently reviewed security estimates and production implementation
  timings.  Local review gates, smoke timings, and account-state lifecycle
  costs are now included, but they are not a substitute for external review.
- [x] Add the reviewed `252 B/Fq12` fallback codec, strict 12-residue quotient
  digest map, and transcript-binding checker to the Rust production-reference
  boundary.
- [x] Add the reviewed `252 B/Fq12` sigma-equation checker:
  `response_syndrome = randomizer + challenge_action * witness` over
  `F_12289[X]/(X^12 + X + 25)`.
- [x] Add the necessary public local-fiber consistency check for the
  `252 B/Fq12` verifier path: challenge-selected masks reconstructed from the
  public key, compact challenge, and active responses must all agree on one
  local fiber state.
- [x] Add the public pad-opening consistency check: for every repeated active
  block, the verifier reconstructs the same block-local pad value from
  `opening - mask_view`.  This is a necessary public check, not the hidden
  pad-seed binding proof.
- [x] Add the canonical public pad-opening statement digest.  This pins the
  exact byte-level statement the future hidden pad/witness proof must bind to:
  domain-separated profile parameters, observed-block bitmap, and the inferred
  per-block pad residues.
- [x] Add the executable hidden pad-seed witness relation for the `252 B/Fq12`
  pad-opening statement.  The reference library now derives block-local pad
  residues from a 32-byte domain-separated pad seed, builds the matching public
  pad-opening statement, and verifies that a supplied seed reproduces the
  statement digest.  This is the reference relation a future signature-carried
  proof must bind to.
- [x] Add the executable hidden-source witness checker for the `252 B/Fq12`
  path.  It checks that the supplied witness syndrome packs to the account
  witness commitment and that the supplied pad seed reproduces the public
  pad-opening statement inferred from the checked signature.  This narrows the
  deployment blocker to carrying/proving that relation inside the signature,
  not defining the relation itself.
- [x] Add the canonical hidden-source public statement for the future compact
  proof.  The statement digest is domain-separated and binds the slot id,
  account witness/randomizer commitments, public-key digest, message digest,
  signature digest, public pad-opening statement digest, and observed-block
  bitmap/count.  This gives the next proof object an exact byte statement
  instead of an informal verifier-side context.
- [x] Add the hidden-source proof callback boundary.  It constructs the checked
  `252 B/Fq12` hidden-source public statement and passes only that statement to
  an external proof verifier callback, with any callback rejection collapsed to
  a fixed public error.  This is the integration point for a future compact
  in-signature proof; it is not by itself a deployment-ready verifier.
- [x] Add the hidden-source proof byte carrier/parser.  The current carrier is
  an opaque fixed 32-byte object, giving a byte-accounted `252 + 32 = 284 B`
  candidate envelope for the future hidden-source proof.  It intentionally has
  no accepting semantics by itself; the next cryptographic task is to make
  those 32 bytes prove the hidden-source relation or justify increasing the
  carrier.
- [x] Add the strict signature-carried hidden-source proof envelope.  The Rust
  boundary now encodes/decodes `252 B` reviewed `Fq12` signature bytes followed
  by the 32-byte hidden-source proof carrier, rejects malformed envelope lengths,
  and exposes a verifier entry point that feeds the decoded signature and
  carrier into the sanitized hidden-source proof callback.
- [x] Add the fail-closed `284 B` envelope deployment verifier.  It parses the
  signature-carried proof envelope, reconstructs the canonical hidden-source
  statement through the checked core, and then rejects with the explicit
  hidden-source proof-semantics blocker.  This keeps the deployment-shaped entry
  point aligned with the current byte envelope without accepting opaque proof
  bytes.
- [x] Add the hidden-source proof transcript digest.  The public helper hashes a
  domain-separated frame containing the profile, q/lattice dimensions, proof
  byte count, the canonical recomputed hidden-source statement digest, and
  32-byte carrier.  This pins the Fiat-Shamir/public-input transcript that
  future compact-proof semantics must use without trusting a cached statement
  digest field.
- [x] Add the transcript-aware hidden-source proof callback boundary.  The
  stricter verifier API now constructs a typed transcript object containing the
  digest, recomputed statement digest, and proof byte count, and passes it to
  the proof verifier callback.  The older callback shape remains as a wrapper,
  but the deployment callback profile points at the transcript-aware boundary.
- [x] Add the signature-carried hidden-source proof context builder.  The
  public helper decodes the strict `284 B` envelope and returns the exact
  statement, proof carrier, and typed transcript object that the future compact
  proof verifier must consume.  The callback verifier and fail-closed
  deployment verifier now share this builder.
- [x] Add the signature-carried deployment blocker report.  The public helper
  returns the hidden-source context plus the exact proof-semantics blocker
  reason for a syntactically valid `284 B` envelope, giving review tooling and
  future proof semantics one fail-closed handoff object.  The report now
  carries a domain-separated digest that recomputes the statement and proof
  transcript from the context fields before hashing.
- [x] Add the hidden-source proof semantics gate.  The gate binds the
  deployment report digest, transcript digest, proof byte count, and named
  semantics profile.  It now carries its own domain-separated digest, including
  the acceptance bit, so a future accepting profile must change the byte-level
  gate digest.  The only current profile is explicitly unimplemented, so the
  verifier returns the proof-semantics blocker instead of accepting opaque proof
  bytes.
- [x] Add the hidden-source proof acceptance contract.  The contract is a
  domain-separated digest over the semantics gate plus the obligations required
  before acceptance can turn on: statement binding, witness-commitment opening,
  pad-seed opening, transcript binding, soundness analysis, and
  zero-knowledge analysis.  It now binds both the readable boolean obligations
  and a canonical required/implemented/missing obligation bitmap plus counts, so
  review tooling can detect partial implementation states.  An explicit
  contract validator rejects digest-correct but semantically inconsistent
  bitmap/count/acceptance-flag states.  The six obligations are now also
  exposed as first-class digest-bound review rows, so scripts and reviewers can
  track each missing proof component independently.  A canonical obligation
  report binds the contract digest, all six obligation row digests, and the
  implemented/missing counts into one reviewable status digest.  A
  signature-carried readiness report now starts from the strict `284 B`
  envelope and binds the deployment report, semantics gate, acceptance
  contract, obligation report, statement digest, transcript digest, proof byte
  count, readiness bits, and blocker into one diagnostic root.  A compact
  readiness verdict object then classifies that diagnostic root as `blocked` or
  `deployable`, binding the readiness report digest, deployment-ready bit,
  missing-obligation count, and blocker.  Explicit deployability assertion
  helpers now reject the current verdict with the proof-semantics blocker, so
  callers do not have to inspect verdict fields manually.  The global
  deployment preflight now reports the readiness report, verdict, and
  deployability assertion surfaces as present while still marking deployment
  not ready.  The preflight itself now has a canonical digest-bound report
  object, first-class digest-bound blocker rows, an aggregate blocker-report
  digest, a compact preflight verdict that binds that blocker-report root, a
  canonical digest-bound ready assertion, and a top-level preflight certificate
  that binds the whole chain.  CI and reviewers can now pin the complete
  blocker set, track individual blocker changes, cite one preflight root, and
  fail closed on the first deployment blocker.  The production gap report now
  exports that certificate root, its linked report/blocker/verdict/assertion
  digests, and the canonical per-blocker row digests directly from the Rust
  facade.  The generated status index now consumes that Rust-exported blocker
  ledger instead of a hand-written Python summary, and promotes the preflight
  certificate digest to a named top-level review field.  A dedicated preflight
  certificate verifier now reruns the Rust exporter and checks the production
  gap report and status index against that live root before the final manifest
  is accepted; the verifier also emits hashed JSON/Markdown artifacts for the
  review packet, and the reviewer status index links those verifier artifacts
  next to the promoted certificate digest.  A selected-deployment terminology
  verifier now also fails if review-facing artifacts regress to stale
  exact-250-byte deployment language, emits hashed JSON/Markdown artifacts, and
  is linked from both the status index and manifest.  It remains fail-closed
  until all obligations are implemented.
- [x] Add a named `252 B/Fq12` checked-core verifier and a fail-closed
  deployment verifier API.  The deployment entry point must return the explicit
  pad-opening witness-binding blocker until that proof layer is implemented.
- [ ] ACTIVE: choose and finish the deployment verifier branch.  The exact
  `250 B` branch requires a reviewed 10-row challenge-action algebra/domain
  with multi-query margin.  The `252 B/Fq12` branch already has the canonical
  codec/map/binding/equation surface plus public local-fiber and pad-opening
  consistency plus a canonical pad-opening statement and executable pad-seed
  witness/hidden-source relation plus a strict 284-byte signature-carried proof
  envelope, but still needs actual compact-proof semantics for that carrier and
  facade promotion.
- [ ] ACTIVE: complete the external security review packet: assumptions, query budgets,
  side-channel/state lifecycle, benchmarks, and verifier/account-state costs.
- [x] Generate assumption-by-assumption red-team closure.  For each row in
  `keygen_security_assumption_surface_n32_q12289.json` and
  `assumption_attack_review_matrix_n32_q12289.json`, record one of:
  concrete proof/local executable evidence, concrete attack, or explicit
  external-review dependency.  `assumption_red_team_closure_n32_q12289.{json,md}`
  is now fail-closed: only the bare quotient is closed as a rejected negative
  control; five rows remain deployment-blocking until independent review and
  implementation hardening close them.
- [ ] ACTIVE: independently review the SHA-256 pad PRF/random-oracle
  assumption and final sampler boundary.  Local arithmetic and boundary checks
  pass, but pseudorandomness/RO modeling and production sampler code remain
  assumption-level.
- [ ] ACTIVE: independently review the QROM/Forking game hop and blockchain
  query ledger.  Local budget arithmetic passes, but the model choice and query
  envelope need external cryptographic judgment.
- [ ] ACTIVE: run independent SIS/lattice-estimator review for the exact
  SHA-256 rejection-sampled `12x23` matrix and public-key distribution.  The
  local lattice harness catches obvious relations and negative controls, but it
  is not a full estimator-based security claim.
- [ ] ACTIVE: implement and audit the final account-carried verifier in Rust
  for the selected branch, replacing the legacy `254 B` proof callback path
  before any deployment benchmark or security claim is treated as
  representative.
- [ ] ACTIVE: complete constant-time and state-lifecycle hardening review:
  signer/sampler/keygen/verifier timing, retry shape, rollback/reorg behavior,
  backup restore, trusted anchor placement, and mempool replay/slot-burning
  policy.

## Historical Starting Row

Original compact-active row: `n=32`, `block_weight=1`, `repetitions=26`, `52`
active response coordinates, `z=48`, `eta=12`, `sigma=896`.  This row remains
useful for explaining the lower-bound pressure, but it is not the current
deployable proof target.

## 1. Compact Byte Format And Active Relation

- [x] Define the canonical 17-byte challenge codec for 26 one-block signed
  rounds.
- [x] Define the canonical 52-byte active-response codec for 26 pairs of
  coordinates in `[-48,48]`.
- [x] Implement the active local relation reconstruction
  `t_active = A * z_active - chi * u_active`.
- [x] Pin the structural 69-byte signature payload:
  `17 B challenge || 52 B active responses`.
- [x] Connect this structural payload to a signable Fiat-Shamir verifier using
  completed-response implicit commitment reconstruction.

## 2. Certified Compact Sampler

- [ ] STALE: make compact signing sample masks from
  `conditioned_gaussian_leakage_certificate_n32.json` row 0.  Superseded by
  the quotient-fiber sampler and then the committed-sigma/account-carried
  proof track; keep only as a historical compact-active sampler benchmark.
- [ ] STALE: enforce row parameters in code:
  `block_weight=1`, `repetitions=26`, `serialized_response_coordinates=52`,
  `z=48`, `eta=12`, `sigma=896`, `weight_precision_bits=20`.  These are not
  the current committed-sigma parameters.
- [ ] STALE: add exact table-alignment tests against the JSON certificate.
  Superseded for the active path by the exact conditioned quotient-fiber
  sampler certificate.
- [ ] STALE: account for retry/failure timing with either fixed attempts or an
  explicit leakage/failure term.  The current quotient-fiber sampler records
  `retry_loop_required = false`; any remaining timing work belongs to the
  production reference implementation.

## 3. Compact Commitment Representation

- [x] Add the explicit full-commitment baseline representation:
  `17 B challenge || 52 B active responses || 3640 B commitments = 3709 B`.
- [x] Keep that baseline as `--explicit-commitment` and reject bare structural
  payloads outside `--structural-only`.
- [x] Replace the leading default with the completed-response implicit
  commitment representation:
  `17 B challenge || 728 B completed responses = 745 B`.
- [x] Choose a genuinely compact commitment strategy to replace the 3640-byte
  explicit commitment payload.
- [ ] STALE: prove in Lean that the completed-response verifier's Fiat-Shamir
  challenge point is bound to the same
  commitment/mask state used by the signer.  The completed-response verifier is
  now a superseded intermediate path; the active binding target is the
  committed-sigma proof-bound challenge frame plus account-carried hidden link.
- [x] Avoid the active-only implicit-commitment fixed-point trap: a verifier
  cannot hash only challenge-selected active commitments unless the signer can
  commit to them before knowing the challenge.
- [x] Benchmark the explicit baseline signature size.
- [x] Benchmark the compact replacement.
- [x] Record the information-set lower-bound abstraction in
  `information_set_commitment_frontier.md` and
  `Descender.InformationSetCommitment`.
- [x] Prove the product-block active-view specialization:
  `|Codeword| <= q^(#active blocks * #local free)`, including the `q^2`
  single-block/two-free corollary.
- [x] Separate commitment-state reconstruction from mask recovery:
  exact active reconstruction only bounds the full mask space when the
  mask-to-commitment-state map is injective, with product-block `q^2`
  corollaries for the `w=1`, two-free-coordinate case.
- [x] Reject transparent information-set mask codes as a deployable 69-byte
  route: known active masks expose signed active secret coordinates.
- [x] Add the fork mask-delta identity:
  different same-state masks add a `y0-y1` term to fork subtraction, so a
  fibered proof needs an extraction-or-collision alternative.
- [x] Formalize the deterministic fibered fork alternative:
  `FiberedForkedLinearRound.extraction_or_collision_of_cover`.
- [x] Instantiate the collision branch as a q-ary/keygen witness for bounded
  masks: `SameStateMaskCollision.toModqLeeKernelViolation_of_boundedMasks` and
  `modqLeeKernelViolationKeygenBreakGame`.
- [x] Package the concrete no-collision obligation as
  `FiberedMaskStateCollisionCertificate.no_sameStateMaskCollision`.
- [x] Define the locally recoverable syndrome candidate in
  `Descender.LocallyRecoverableSyndrome`.
- [x] Prove active reconstruction for locally consistent masks:
  `LocalLinearSyndrome.active_reconstructs_state`.
- [x] Prove same-state locally consistent bounded collisions reduce to q-ary
  Lee-kernel violations:
  `LocalLinearSyndrome.collisionToLeeViolation`.
- [x] Record the local non-injectivity/hiding boundary:
  `LocalLinearSyndrome.localStateHiding_of_hasLocalKernel`.
- [x] Add the sharper coordinate-wise collision target:
  `ModqBoxKernelViolation`,
  `LocalLinearSyndrome.collisionToBoxKernelViolation`, and
  `LocalLinearSyndrome.BoxCollisionCertificate.no_sameStateMaskCollision`.
- [x] Instantiate a concrete direct mask-syndrome coefficient family:
  `N32Q12289LocalSyndrome.localState_b(x,y) = x + 50*y`, with a checked
  `[-24,24]^2` local box-kernel certificate.
- [x] Record the preimage-hiding obstruction for direct mask syndromes:
  `N32Q12289LocalSyndrome.stateOfMask_injectiveOn_validMask`.
- [x] Formalize the commitment-domain split:
  public-state collisions reduce to hidden-state collisions or commitment
  collisions via
  `CommittedLocallyRecoverableState.hiddenCollisionOfPublicCollisionOrCommitmentCollision`.
- [x] Compose the box-collision certificate with the commitment assumption:
  `LocalLinearSyndrome.no_publicStateCollision_of_boxCertificate_noCommitmentCollision`
  and `N32Q12289LocalSyndrome.no_publicStateCollision_of_noCommitmentCollision`.
- [x] Add explicit active-opening support:
  `LocalLinearSyndrome.OpenedCommittedLocallyRecoverableState`,
  `LocalLinearSyndrome.toSaltedOpenedCommittedState`, and
  `LocalLinearSyndrome.no_saltedPublicStateCollision_of_boxCertificate_noCommitmentCollision`.
- [x] Add active-opening byte accounting:
  `N32Q12289LocalSyndrome.saltedOpening_signatureBytes_eq`, with 128-bit
  per-repetition salt at `485 B` and 256-bit per-repetition salt at `901 B`.
- [x] Prove the direct salted-opening obstruction:
  `N32Q12289LocalSyndrome.saltedHiddenState_injectiveOn_validMask`.
- [ ] STALE: replace revealed hidden-state openings with a trapdoor/ZK-style opening,
  or a non-injective accepted-mask hidden domain, so active verification binds
  the public state without recovering the mask.  This branch was explored; the
  live route moved to local delta openings, quotient fibers, and then
  committed-sigma/account-carried linking.
- [x] Trial the trapdoor/ZK-shaped public-only opening route:
  `LocalLinearSyndrome.PublicOpenedCommittedLocallyRecoverableState`,
  `PublicOpenedCommittedLocallyRecoverableState.NoPerfectHiddenExtractor`, and
  `N32Q12289LocalSyndrome.no_publicOpenedStateCollision_of_noCommitmentCollision`.
- [x] Trial the raw non-injective hidden-domain route and record the obstruction:
  `LocalLinearSyndrome.no_validStateNonInjective_of_boxCertificate` and
  `N32Q12289LocalSyndrome.no_validStateNonInjective`.
- [x] Add the corrected-extraction variant for non-injective fibers with
  recoverable mask deltas:
  `FiberDeltaOpening.correctedExtractCoordinate_eq_secret`.
- [x] Construct the local algebraic version of the delta-opening route:
  `LocalFiberDeltaOpening`, `LocalFiberDeltaOpening.additiveOneTimePad`, and
  `LocalFiberDeltaOpening.additiveMaskedLocalOpening_perfectHiding`.
- [x] Add the byte-aligned N32 account for one local delta opening per
  repetition:
  `N32Q12289LocalSyndrome.localDeltaOpening_signatureBytes_eq` (`173 B`).
- [x] Bind the local pad to the committed hidden preimage instead of deriving
  it publicly:
  `LocalFiberDeltaOpening.additiveCommittedOneTimePad`.
- [x] Add the transcript/digest binding theorem for this route:
  `LocalFiberDeltaOpening.StateDigestBinding` and
  `LocalFiberDeltaOpening.HashBoundLocalDeltaForkedRound.correctedExtractCoordinate_eq_secret`.
- [x] Instantiate the commitment/hash and pad-source route at the byte level:
  `Descender.LocalDeltaByteCommitment`,
  `LocalDeltaByteCommitment.ByteCommittedPadPreimage`,
  `LocalDeltaByteCommitment.ByteCommitmentConfig`,
  `LocalDeltaByteCommitment.byteCommittedLocalDeltaOpening`, and
  `LocalDeltaByteCommitment.byteCommittedLocalDeltaFork_correctedExtractCoordinate_eq_secret`.
- [x] Pin the concrete N32/q12289 byte layout and assumption slots:
  `LocalDeltaByteCommitment.N32Q12289.commitmentDomainTag`,
  `LocalDeltaByteCommitment.N32Q12289.encodeHiddenPreimage`,
  `LocalDeltaByteCommitment.N32Q12289.CommitmentHashBinding`,
  `LocalDeltaByteCommitment.N32Q12289.padDomainTag`,
  `LocalDeltaByteCommitment.N32Q12289.padFrame`,
  `LocalDeltaByteCommitment.N32Q12289.PadPRFBinding`, and
  `LocalDeltaByteCommitment.N32Q12289.BlockchainAccountState`.
- [x] Record the N32/q12289 byte split:
  `LocalDeltaByteCommitment.N32Q12289.statefulSpend_signatureBytes_eq`
  (`177 B`) and
  `LocalDeltaByteCommitment.N32Q12289.statefulTransaction_signatureBytes_eq`
  (`181 B`, including the 4-byte slot id), plus
  `LocalDeltaByteCommitment.N32Q12289.statelessSha256_signatureBytes_eq`
  (`1009 B`).  The delta from the original compact target is the 4-byte
  transcript-hardening suffix on top of the 17-byte algebraic challenge.
- [x] Add an executable byte fixture for the blockchain/stateful route:
  `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`.
- [x] Pin the usable Fiat-Shamir frame: hash prechallenge account material
  only (`profile`, parameters, slot id, public key, message, slot digest), then
  verify active responses and local openings after challenge derivation.
- [x] Model slot consumption:
  `LocalDeltaByteCommitment.N32Q12289.ConsumeSlotTransition` and
  `StatefulVerifierRelation.slot_not_reusable`.
- [x] Formalize the public opening proof plug-in boundary:
  `LocalDeltaByteCommitment.N32Q12289.PublicOpeningProofSystem`,
  `StatefulPublicOpeningVerifierRelation.openingWitness_of_accepts`, and
  `StatefulPublicOpeningVerifierRelation.algebraicVerifierObligations`.
- [x] Record proof-byte accounting:
  `LocalDeltaByteCommitment.N32Q12289.statefulTransactionWithProofBytes_eq`
  (`181 B + proof_bytes`).
- [x] Record competitive proof-byte budgets:
  `statefulTransactionWithProofBytes_under256` (`proof <= 75 B`),
  `statefulTransactionWithProofBytes_beats269` (`proof <= 87 B`),
  `statefulTransactionWithProofBytes_atMost269` (`proof <= 88 B`), and
  `statefulTransactionWithProofBytes_beatsStatelessSha256`
  (`proof <= 827 B`).
- [x] Add the public-checkable revealed-preimage control verifier:
  `LocalDeltaByteCommitment.N32Q12289.revealedPreimageProofSystem`,
  `LocalDeltaByteCommitment.N32Q12289.revealedPreimageTransactionSignatureBytes_eq`,
  and `REVEALED_PREIMAGE_TRANSACTION_SIGNATURE_BYTES = 211` in
  `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`.
- [ ] STALE: instantiate the public-only opening route concretely, or instantiate a
  fiber-delta opening with explicit byte cost and leakage analysis.
  Superseded by the full-response quotient and committed-sigma tracks.
- [ ] STALE: replace the revealed-preimage control proof with a compact
  witness-hiding public proof/opening, or prove that no small instantiation can
  satisfy the required opening relation.  The revealed-preimage proof remains a
  control; the active proof is the account-carried committed-sigma hidden-link
  relation.
- [x] Prefer algebraic commitment/opening candidates over generic SHA-256 ZK:
  the competitive proof budget is roughly 90 bytes, which is too small for a
  generic hash-circuit proof path.  The project pivoted to algebraic
  committed-sigma/account-carried response-link structure.
- [x] STALE: promote the then-selected algebraic sigma target to the 69-byte /
  250-byte enlarged-fiber route:
  `LinearSigmaOpeningTarget`,
  `linearSigmaTargetProofBytes_eq` and
  `linearSigmaTargetTransactionSignatureBytes_eq` (`250 B total`).
- [x] Add the 69-byte proof field layout:
  `LinearSigmaProofFields` and
  `AlgebraicLinearSigmaOpeningConfig.toLinearSigmaOpeningTarget`, with the
  matching Rust parser/encoder for
  `proof_commitment[32] || masked_witness_response[34]`.
- [x] Choose and formalize the candidate primitive shape:
  `CompressedLinearSigmaAlgebra`, verifying
  `L_statement(z) = a + challenge * target(statement)`.
- [x] Instantiate the restricted-domain route:
  `SeedRestrictedCompressedLinearSigmaAlgebra`, where
  `hidden_preimage = derive_fiber_state(seed)[2] || seed[32]`, with
  `restrictedDomain_not_pigeonholeBlocked`.
- [x] Record the binding obstruction:
  `noInformationTheoreticBinding_34_to_32`; a 32-byte public value cannot
  injectively bind every 34-byte hidden preimage, so the algebraic branch needs
  computational binding, a restricted witness domain, or extra account-state
  bytes.
- [x] Formalize the SIS-style seed-opening boundary:
  `SISStyleSeedOpeningMap`, with
  `target(statement) = statement.digest`,
  `L_statement(z) = a + challenge * statement.digest`,
  `relationMapSound`, and `sisKnowledgeSoundness`.
- [x] Pin the first byte-level q-ary syndrome packing target:
  `18` seed residues and `18` syndrome residues at `14` bits each, packed into
  existing `32`-byte fields with `4` slack bits.
- [x] Add the concrete reduction gate:
  `PackedSISSeedOpeningReduction`, where a matrix/domain candidate supplies
  canonical packed q-vector validity, canonical digest validity, byte-level
  linearity, relation-map soundness, and `extracts_or_break`;
  `noAlgebraicBindingBreak` converts it to
  `SISStyleSeedOpeningMap.sisKnowledgeSoundness`.
- [x] Pin the first deterministic matrix-generation domain in the executable
  report:
  `Descender.Signature.q12289.local_delta.sis_matrix.v1`, with SHA-256 counter
  expansion and rejection sampling modulo `12289`.
- [x] Run the first matrix sanity check: the generated `18 x 18` matrix is
  full-rank mod `12289` (`det = 11243`), so the bare square linear syndrome map
  is publicly invertible.
- [x] Formalize the square-map hiding obstruction:
  `NontrivialSeedFiber`, `noNontrivialSeedFiber_of_injective`, and
  `PublicSeedRecoverer`.
- [x] Try the rectangular nontrivial-fiber repair and record the byte-capacity
  obstruction:
  `noBalanced128BitFiberSplit`; the balanced `9/9` split is `126` public bits
  and `130` hidden-capacity bits, while `10/8` is `140/116`.
- [x] Try the direct Pedersen-style linear hidden-component repair and record
  the byte-capacity obstruction:
  `noPedersenStyle128BitSplit`.
- [x] Try the enlarged full-response q-vector route:
  use the whole `37`-byte masked response as `21` q-residues.  At `10` public
  rows it gives `140` public bits and `156` raw hidden-fiber bits, with a
  rank-`10` deterministic matrix prefix and `69` proof bytes.
- [x] Add the full-response proof interface:
  `FullResponseFiberedLinearSigmaAlgebra`, converting to the same 250-byte
  `LinearSigmaOpeningTarget`.
- [x] Instantiate the opening relation around full-response q-vectors so that
  `relationMapSound` applies to every accepted extracted vector in the fiber,
  not only to a unique 32-byte seed.  Lean now has
  `FullResponseByteOpeningAlgebra`: its target relation is exactly the
  byte-level `idealRelation` applied to the decoded full 37-byte response.
- [ ] STALE: instantiate the concrete full-response `relationMapResponse` and prove
  its `relationMapSound`, `relationMapComplete`, and
  `responseKnowledgeSoundness` fields for the selected q-ary construction.
  The reduction gate is now formalized as
  `PackedSISFullResponseOpeningReduction`, which converts to
  `FullResponseByteOpeningAlgebra`.  This direct opening target was superseded
  by the quotient/fiber-saturated relation and then by the committed-sigma
  account-carried proof surface.
- [x] Derive the full-response fiber-saturation obstruction:
  `FullResponseByteOpeningAlgebra.acceptedFiber_relationSaturated` proves that
  every response in an accepted public fiber must decode to a valid witness,
  while `acceptedFiber_subsingleton_of_commitmentBinding` proves that a
  relation binding every byte of `fiber_state[2] || seed[32]` by an injective
  commitment collapses accepted fibers to singletons.
- [x] Add the executable kernel sanity certificate for the deterministic
  full-response matrix: the selected `10 x 21` public map has rank `10`, kernel
  dimension `11`, and a sampled nonzero kernel vector whose public syndrome is
  zero.
- [x] Replace the current full-hidden-preimage SHA binding relation with a
  quotient/fiber-saturated opening relation, or increase public bytes/rows until
  the accepted relation is allowed to be singleton-binding again.  The leading
  target now uses the quotient route; the hidden 11-residue fiber is randomness,
  not a byte-for-byte SHA-bound semantic witness.
- [x] Add the quotient/fiber-saturated opening relation surface:
  `FullResponseQuotientRelation` makes the public syndrome quotient the semantic
  witness and proves quotient-level fiber saturation, while
  `PackedSISFullResponseQuotientOpeningReduction` converts that relation into
  the existing 250-byte full-response linear-sigma target.
- [x] Add the existential quotient bridge:
  `FullResponseExistentialQuotientRelation` lets the old representative-level
  local opening predicate witness a quotient by existence of some valid
  representative in the accepted fiber, without requiring the extracted
  representative itself to be byte-for-byte committed.
- [x] Instantiate the concrete public quotient predicate
  `quotientOpensStatement`: it must express the local-delta opening semantics
  using only the accepted 10-residue quotient, with the 11-residue kernel treated
  purely as hiding randomness.
- [x] Instantiate the local-delta quotient predicate shape:
  `ByteLocalDeltaRepresentativeSemantics.representativeOpensStatement` fixes the
  response/opening byte decoding, active mask reconstruction, local-state check,
  and padded-opening equation; its `quotientOpensStatement` is the existential
  lift to the accepted public quotient fiber.
- [x] Supply the final concrete decoders for `challengeValid`, active block /
  challenge coefficient decoding, public-key active-view decoding, and pad PRF
  expansion.  `concreteByteLocalDeltaRepresentativeSemantics` fixes the
  17-byte packed algebraic challenge plus 4 transcript-hardening bytes,
  52-byte signed active response, 172-byte public-key packed14 free-view
  coordinates, and `PadPRFBinding.expandPad` pad source.
- [x] Add Rust-layout decoder spec lemmas:
  `ByteLocalDeltaRustLayout.activeResponseCoeff_eq_centeredByte`,
  `concretePublicKeyFreeView_eq_packedFreeCoordinate`,
  `publicKeyFieldCanonical_iff_rustPackedQElements`, `padFrame_domain`, and
  `padFrame_block` name the byte positions used by the executable verifier.
- [x] Prove the honest representative existence bridge:
  `HonestLocalDeltaQuotientStatement`,
  `targetRepresentativeExists_of_honestStatements`,
  `toFullResponseExistentialQuotientRelationOfHonestStatements`, and
  `quotientOpensStatement_of_honestStatement` turn an honest stateful statement
  representative into the quotient existence theorem needed by
  `PackedSISFullResponseQuotientOpeningReduction`.
- [x] Add the honest generator trace theorem:
  `ConcreteHonestLocalDeltaSigningTrace` proves the concrete local byte
  equations imply `representativeOpensStatement`, and
  `ConcreteHonestFullResponseQuotientSigningTrace` proves a packed
  q-syndrome digest equality implies `HonestLocalDeltaQuotientStatement` and
  `targetRepresentativeExists_of_traces`.
- [x] Add the executable q-syndrome digest builder for the quotient target:
  the Rust reference now decodes the 34-byte full response as 19 canonical
  14-bit q-residues, applies the deterministic 10x21 full-response matrix, and
  packs the 10-residue syndrome into the 32-byte account digest field with
  slack bits zero.
- [x] Internalize the same full-response q-syndrome codec in Lean:
  `FullResponseCodec` names the 37-byte response decoder, deterministic
  10x21 matrix domain, canonical 10-residue digest decoder, syndrome map, and
  byte-level digest-match predicate used by the executable reference.
- [x] Add a Rust honest quotient fixture test:
  `quotient_stateful_fixture_emits_honest_local_delta_trace` rejection-samples
  a canonical full-response hidden preimage, derives the q-syndrome digest, and
  checks the challenge-bound local-state and padded-opening equations.
- [x] Bridge the verifier gap:
  `verify_quotient_stateful_signature` now checks
  `full_response_syndrome_digest(hidden_preimage) == slot.digest` and
  `quotient_stateful_verifier_accepts_and_consumes_slot` exercises this as the
  production reference path.  The legacy SHA verifier remains only as an
  explicit comparison artifact, and
  `legacy_sha_verifier_rejects_quotient_digest` confirms it rejects quotient
  accounts.
- [x] Package the concrete quotient reduction wrapper:
  `ConcreteFullResponseQuotientReductionAssumptions` fixes the executable
  codec, deterministic full-response matrix, digest-packing checks, honest
  representative bridge, extraction-or-break slot, witness-hiding slot, and
  total byte accounting for the 250-byte quotient target.
- [x] Package the deployable SHA-256/rejection-sampled pad source as a named
  assumption object:
  `SHA256RejectionPadPRFAssumption` records that the concrete pad frame and
  rejection sampler match the Rust implementation and exposes the inherited
  local pad pseudorandomness assumption.
- [x] Emit executable assumption-review targets:
  `signature_q12289_local_delta_stateful.rs` now reports the five deployment
  gates for matrix generation, packed full-response extraction, quotient
  witness hiding, pad-source pseudorandomness, and Fiat-Shamir challenge-frame
  agreement.
- [x] Emit executable closure artifacts for all five gates:
  `MatrixSamplerCertificate` fixes the full-response matrix domain, encodings,
  rejection cutoff, transcript digest, and sample rows;
  `FullResponseQuotientAlgebraCertificate` fixes rank `10`, the `9`-vector
  kernel basis, right-inverse check, and quotient/fiber capacities;
  `PadSamplerCertificate` fixes the pad frame, `u32` counter encoding,
  rejection cutoff, sample digest, and sample residues; and
  `FiatShamirFrameCertificate` fixes the prechallenge frame labels, frame
  digest, challenge truncation, and response/opening exclusion.
- [x] Add the Lean deployment-review bundle:
  `FullResponseDeploymentReviewCertificate` composes the pad assumption,
  concrete quotient reduction assumptions, full-response quotient algebra
  certificate, and Fiat-Shamir frame agreement into one object with the
  immediate consequences `totalBytes`, `padPseudorandom`,
  `matrixSamplerAgreement`, `quotientSyndromeSurjective`,
  `quotientFiberNonempty`, `quotientWitnessHiding`, and `fiatShamirFrame`.
- [x] Turn the extraction gate into a falsifiable Lean break condition:
  `ConcreteFullResponseQuotientReductionAssumptions.extractionFailure` names an
  accepting packed full-response sigma equation for which no valid quotient
  representative can be extracted, and `no_extractionFailure` proves that the
  current `extracts_or_break + noAlgebraicBindingBreak` package rules out
  exactly that failure.
- [x] Add the first top-level conditional deployed-verifier theorem:
  `FullResponseDeploymentReviewCertificate.statefulVerifier_conditionalSoundness`
  proves that, under the explicit review certificate, every accepting stateful
  public-opening verifier has a pre-accepted slot, opens the target quotient,
  satisfies the active product-block equation, consumes the slot, hits the
  250-byte target, and uses exactly the named pad, hiding, matrix-agreement, and
  Fiat-Shamir assumptions.
- [x] Replace anonymous deployment assumptions with named game wrappers:
  `PackedFullResponseExtractionGame`,
  `QuotientWitnessHidingGame`, `SHA256PadPRFGame`, and
  `FiatShamirFrameAgreementGame`.  The top-level theorem now returns the
  corresponding no-break facts:
  `noExtractionBreak`, `noWitnessHidingBreak`, `noPadPRFBreak`, and
  `noFrameMismatchBreak`, plus `no_extractionFailure`.
- [x] Expose the same game names in the executable report:
  `assumption_closure_report` now includes `lean_game`, `break_event`, and
  `no_break_claim` fields for each deployment gate.
- [x] Add explicit 128-bit review budget slots:
  `GameSecurityBudget` carries adversary query count, classical security bits,
  and quantum security bits; each named game has a `budget_meets128` field, and
  `statefulVerifier_conditionalSoundness` now returns those budget facts.  The
  executable report mirrors the target with `target_classical_security_bits`,
  `target_quantum_security_bits`, and `query_bound_status`.
- [x] Add concrete generic-query budget rows:
  `concrete_security_budgets` reports the raw bit margins and square-root-loss
  query margins for extraction (`140` raw bits, `2^6` quantum-query budget),
  quotient hiding (`133` exact conditioned min-entropy bits, `2^2`
  quantum-query budget), SHA-256 pad PRF
  (`256` raw bits, `2^64` quantum-query budget), and the hardened 21-byte
  Fiat-Shamir challenge (`130` algebraic bits plus `32` transcript-bound bits,
  giving `2^17` quantum-query budget or `2^34` classical-query budget for a
  128-bit margin).
- [x] Emit an executable blockchain-size comparison table for the leading
  stateful quotient candidate against ML-DSA-44/Dilithium2, Falcon-512,
  SLH-DSA-SHA2-128s/SPHINCS+, and Ed25519, with source URLs in each comparator
  row.
- [ ] STALE: replace the bare square public linear map with a witness-hiding candidate:
  add a hard inversion/trapdoor layer, add a hidden component, or change the
  public-map dimensions so there are nontrivial fibers while preserving the
  extraction-or-break proof.  The bare square map was rejected; the live path is
  quotient/fiber saturation plus account-carried response-link soundness.
- [ ] ACTIVE: externally review the named SHA-256/rejection-sampled `PadPRFBinding`
  assumption; Lean now exposes the deployment assumption and Rust emits the
  byte-level sampler certificate, but SHA-256 pseudorandomness remains an
  explicit cryptographic assumption.
- [x] Fill the local game-budget ledger with quantitative security-loss
  accounting.
  `conditional_soundness_budget_review_n32_q12289.{json,md}` translates
  `noExtractionBreak`, `noWitnessHidingBreak`, `noPadPRFBreak`,
  `noFrameMismatchBreak`, and `matrixKeygenBreak` into concrete adversary
  classes, query limits, local evidence, charged bounds, and residual
  assumptions.  Local checks pass with a `128`-bit minimum checked term, while
  the ledger stays `deployment_ready = false` pending external/model review and
  production preflight blockers.
- [x] Harden the Fiat-Shamir challenge by adding 4 transcript-bound bytes.
  The blockchain target is now `250 B`, and the generic
  Fiat-Shamir budget moves from `2^1` to `2^17` quantum-query margin.

## 3a. Deployment Testing And Cryptoanalysis Gate

- [x] Add a separate production-reference crate boundary.
  `tools/production-reference` now builds `descender-production-reference`,
  re-exports the production-reference API surface without exposing research
  binaries, and tests that account-carried constants, sanitized callback
  profile, and fail-closed deployment preflight are visible through that crate.
  This is an intermediate split: it still wraps the existing library module and
  does not implement accepting hidden-source proof semantics for the selected
  `284 B/Fq12` verifier.
- [x] Add sanitized deployment facade helpers to the production-reference crate.
  `deployment_sign_with_secret_key`, `deployment_sign_reserved_and_consume`,
  and `deployment_verify_with_account_binding` route callers through sanitized
  callback boundaries and account-state transition helpers.  The crate tests now
  cover sanitized signer failure, sanitized verifier failure, accepted
  verify-and-consume, and reserved-slot sign-and-consume.
- [ ] ACTIVE: complete the production-shaped reference crate by moving the
  heavy `keygen`, `sign`, and `verify` callback implementations out of the
  research fixture and replacing the legacy `254 B` proof callback with the
  reviewed `250 B` account-carried verifier.
- [x] Generate known-answer tests for public keys, slot digests, challenge
  frames, 21-byte challenges, active responses, openings, quotient proof
  fields, full signatures, and verifier accept/reject results.  The executable
  `third_party_review_packet.known_answer_vectors` now covers public key, slot
  digest, challenge frame, challenge bytes, stateful signature, and 73-byte
  proof layout hashes, while the stable sample-vector test locks parser,
  verifier, and matrix certificate outputs.
- [x] Add mutation tests for every serialized region: slot id, algebraic
  challenge bytes, 4 hardening bytes, forbidden high challenge bits, public key,
  message, active responses, openings, quotient proof commitment, masked
  witness response, and consumed-slot replay.  The
  `serialized_region_mutations_reject_or_change_proof_layout` test covers these
  mutation surfaces.
- [x] Add property/fuzz tests for parser canonicality and verifier rejection:
  malformed lengths, noncanonical q residues, out-of-bound responses, frame
  mismatch, and cross-account/cross-message binding.  The
  `parser_property_rejections_cover_malformed_and_noncanonical_inputs` test
  covers malformed lengths, high challenge bits, response bounds, noncanonical
  q residues, full-response slack, and public-key canonicality; mutation tests
  cover frame/message/account binding.
- [x] Audit constant-time behavior for signing paths and any verifier code that
  could branch on secret material in wallet-side precomputation.  Finding:
  reference code is not production constant-time; the third-party review packet
  marks production signer side-channel hardening as a deployment blocker.
- [x] Specify state-management semantics for deployment: monotonic slot
  consumption, crash consistency, concurrent signing attempts, backup/restore
  behavior, and blockchain replay rules.  The executable
  `stateful_lifecycle_policy` and review packet document these semantics.
- [x] Start the concrete cryptoanalysis report:
  `cryptoanalysis_attack_surfaces` names the five gates, while
  `matrix_short_relation_probe` and `hidden_fiber_short_combination_probe`
  run bounded smoke tests against the 10x21 quotient matrix and its hidden
  fiber basis.
- [x] Extend the matrix cryptoanalysis beyond smoke tests: run lattice
  reduction/SIS-style short-relation searches against the public quotient
  matrix and record reproducible parameters, seeds, and negative/positive
  witnesses.  The first LLL probe over the integer SIS kernel finds a
  syndrome-zero vector with Lee weight `341`, support `21`, and coefficient
  bound `50`; the next question is whether such deltas can connect two
  accepting local-delta representatives.
- [x] Derive the first quotient witness-hiding distribution boundary:
  the algebraic quotient has `156` raw hidden fiber bits, and fixing the 2-byte
  local fiber state leaves an estimated `140` bits of representative freedom,
  but the current fixture samples one representative and therefore does not
  itself discharge hiding.
- [x] Test the LLL short kernel delta against fixed-statement acceptance:
  multipliers `±1..±8` preserve the public syndrome and canonical packing but
  do not preserve the decoded fiber state or active-block pad outputs, so this
  vector is not an immediate same-statement representative forgery.
- [x] Identify the current quotient-hiding blocker:
  the enlarged 250-byte route covers the fixed local fiber-state equality, but
  sample active-block pad constraints still overdetermine the fiber.  The
  current representative semantics therefore cannot discharge a 128-bit
  `QuotientWitnessHidingGame` unless pad checks are composed outside the hidden
  quotient representative.
- [x] Define the production masked-response sampler certificate surface:
  Lean now has `FullResponseMaskedResponseSamplerCertificate`, which pins the
  `156 - 16 = 140` bit accounting, the allowed conditioning set, and the
  explicit exclusion of active pad outputs.  The executable report mirrors this
  as `masked_response_sampler_certificate`.
- [x] Compare the production masked-response sampler against the ideal
  fiber-symmetric sampler with a finite TV bound: for the selected uniform
  kernel-coefficient rejection sampler, the executable certificate records
  total-variation distance `0/1` to the ideal conditioned fiber law.  This
  closes the finite-distance part of `QuotientWitnessHidingGame`.
- [x] Add exact support-size/min-entropy accounting for the concrete
  `q = 12289` conditioned fiber sampler.  The executable certificate proves the
  hidden-kernel projection to packed coordinates `[0, 1]` has rank `2`; the
  fixed two-byte fiber state therefore leaves exactly `12289^9 * 3073`
  representatives, with log2 support `133.85...` and conservative min-entropy
  floor `133` bits.  The quotient witness-hiding budget now uses this exact
  floor, not the 14-bit capacity approximation, so the generic support-search
  fallback margin for 128-bit hiding is `2^2`.
- [x] Separate quotient-hiding advantage from support-search accounting:
  Lean now has `QuotientWitnessHidingStatisticalBound`, and the executable
  report emits `quotient_witness_hiding_game_bound`.  Under the sampler
  certificate, the stateful one-shot transcript law has total-variation
  distance `0/1` from the ideal conditioned fiber law, so the
  `QuotientWitnessHidingGame` distinguishing advantage is `0`.  The `133`-bit
  support floor remains an audit metric for sampler population/collision sanity
  rather than the advantage bound charged through a generic square-root
  query-loss model.
- [ ] STALE: redesign the hiding surface: decouple the quotient witness from
  byte-for-byte pad seed checks, increase hidden fiber capacity further, or
  make the accepted relation genuinely fiber-saturated while preserving the
  250-byte target as closely as possible.  The quotient-fiber route used the
  enlarged fiber and exact sampler; later public-forgery analysis moved the
  active proof target to committed-sigma/account-carried response linking.
- [x] STALE: explore the two concrete repair routes in the executable frontier:
  decoupling keeps the `247 B` target with `132` modeled hiding bits; increasing
  hidden capacity to survive the 16-bit local fiber-state equality gave the
  historical `250 B` target; keeping current active-pad checks inside the
  representative pushes the model to about `294-308 B` under the current
  proof-bound challenge sample and worst-case active-block coverage.
- [x] STALE: choose the repair route.  The selected route at that stage was the
  `250 B` enlarged-fiber variant, with an explicit
  canonical-representative/composition proof obligation for the 37-byte
  response.  Later public-forgery analysis moved the active deployment branch
  to the fail-closed `284 B/Fq12` account-carried hidden-source envelope.
- [x] Close the 250-byte canonical-representative obligation in Lean:
  `PackedSISFullResponseQuotientOpeningReduction.toPackedSISFullResponseOpeningReduction`
  now states completeness over an existential 37-byte representative instead of
  a zero-extended 34-byte semantic witness.  This is the core pad-constraint
  composition move: pad equations witness that the quotient opens by existence
  of some accepted representative, while the sigma response used for hiding is
  only required to stay in the accepted public quotient fiber.
- [x] Prove the production masked-response sampler finite-distance bound at the
  certificate level: the selected law samples uniform kernel coefficients and
  rejects only on canonical packing, fixed local fiber-state equality, and
  response bounds, giving exact TV distance `0` to the corresponding ideal
  conditioned fiber distribution.  Active pad outputs are excluded.
- [x] Implement the efficient conditioned quotient-fiber sampler in the
  executable reference: instead of rejection over kernel coefficients, it solves
  the `10` public q-syndrome equations plus the two packed fiber-state
  equations, samples the remaining `9` free q-residues directly, and chooses
  residue `1` uniformly from the `3073` values congruent to `0 mod 4`.  The
  report records `retry_loop_required = false`, `expected_retries = 1`, and
  checks that the sample preserves the public syndrome, preserves the two-byte
  fiber state, and is canonical packed q.
- [x] Add the first production-shaped reference API surface in the executable
  fixture: `reference_keygen`, `reference_sign`, `reference_verify`, and the
  quotient verifier path.  The API emits deterministic known-answer digests for
  public key, slot digest, and signature hash, and the test suite covers
  sign/verify, slot consumption, replay rejection, message mutation, opening
  mutation, and slot-id mutation.
- [x] Record the stateful lifecycle policy: one signature per slot, atomic
  reserve/consume before signature release, consumed-on-reorg behavior,
  compare-and-swap/serialized concurrent signing, crash recovery treating
  reserved-unknown slots as consumed, and backup/restore replay warnings.
- [x] Add the extraction-soundness closure report.  The executable report now
  lists the closed parser/syndrome/verifier bridge obligations and isolates the
  remaining cryptographic assumption as knowledge soundness of the compressed
  linear-sigma/SIS opening relation, with `extractionFailure` as the named
  break event.
- [x] Split extraction soundness into fork extraction plus Fiat-Shamir fork
  generation: Lean now has `FullResponseForkExtractionCertificate`, and the
  executable report mirrors it as `fork_extraction_review_report`.  The
  algebraic review target is now concrete: two accepting transcripts with the
  same proof commitment and same statement data except for challenge bytes must
  extract a 41-byte response hitting the quotient digest, or produce
  `forkExtractorBreak`.  The remaining probabilistic part is the classical/QROM
  Fiat-Shamir forking loss.
- [x] Define the fork-extraction challenge action and bad-difference budget:
  Lean now records `FullResponseChallengeActionCertificate`; the executable
  `challenge_action_invertibility_report` hashes each 21-byte challenge to
  `12` q-residues and interprets them as an element of `F_{12289^12}`.  The bad
  fork event is equality of these 12-residue action vectors, with ideal-model
  probability `1 / 12289^12`, floor `163` bits, and `2^17` generic quantum
  margin for 128 bits.
- [x] Pin and certify the degree-12 extension polynomial: the action field uses
  `p(X) = X^12 + X + 25`; the executable Rabin test verifies
  `x^(q^12)-x = 0 mod p` and the two proper-factor checks
  `gcd(p, x^(q^6)-x) = gcd(p, x^(q^4)-x) = 1`.  Lean records the corresponding
  certificate slots on `FullResponseChallengeActionCertificate`.
- [x] Add the concrete fork-equation algebra certificate: Lean now exposes
  `FullResponseForkEquationAlgebraCertificate`, and the executable
  `fork_equation_algebra_report` verifies the field subtraction/division step
  for two accepting equations under the pinned `F_{12289^12}` action.  The
  remaining fork-extraction work is now the byte-canonical response lift plus
  Fiat-Shamir/QROM fork generation, not the basic action-difference algebra.
- [x] Add the byte-canonical response-lift certificate: Lean now exposes
  `FullResponseCanonicalResponseLiftCertificate`, and the executable
  `canonical_response_lift_report` verifies that the rank-12 right-inverse lift
  maps a 12-residue digest to a canonical 41-byte response whose syndrome
  repacks to the same digest.  The remaining algebraic fork obligation is now
  byte-level masked-response subtraction/scaling, not existence of a canonical
  response representative.
- [x] Close the byte-level masked-response arithmetic bridge: Lean now exposes
  `FullResponseByteResponseArithmeticCertificate`, and the executable
  `byte_response_arithmetic_report` verifies that quotient add/subtract/scale
  operations on canonical 41-byte responses realize the same `Fq12` fork
  equation on the public syndrome.  The extracted response is canonical and
  repacks to the original digest.
- [x] Add same-commitment fork-generation accounting: Lean now exposes
  `FullResponseFiatShamirForkAccountingCertificate`, and the executable
  `fiat_shamir_fork_accounting_report` separates the 162-bit transcript
  challenge budget from the 163-bit action-collision budget.  The action
  collision event now has `2^17` generic quantum-query margin for a strict
  128-bit target, matching the transcript challenge budget.
- [x] Harden the deployable blockchain target to the `d = 12` action/public
  quotient profile: the active candidate now uses 23 response residues, a
  41-byte masked response, a 73-byte proof, and a 254-byte transaction
  signature.  The pinned action field is
  `F_12289[X] / (X^12 + X + 25)`, with action-collision floor `163` bits and
  `2^17` generic quantum-query margin for a 128-bit target.  The older
  `d = 10` / 250-byte row remains a tight research comparison point, not the
  deployable target.
- [x] Add the deployment-safe Fiat-Shamir fork-generation ledger: Lean now has
  `FullResponseFiatShamirForkGenerationLedger`, the top-level conditional
  soundness theorem returns its same-commitment fork, QROM fork, and
  union-bound assumptions, and the executable report distinguishes the
  single-term `2^17` quantum margin from the conservative all-terms `2^16`
  deployment budget.
- [x] Instantiate option 2 as the selected QROM proof boundary: Lean now has
  `FullResponseQROMMeasureAndReprogramInstantiation`, the fork-generation
  ledger carries that instantiation, and the executable report names the
  Don-Fehr-Majenz-Schaffner / measure-and-reprogram theorem family with its
  `(2q + 1)^2` quadratic loss and `2^16` deployment query budget.
- [x] Bridge QROM fork generation to byte extraction: Lean now has
  `FullResponseQROMSameCommitmentFork` and
  `FullResponseQROMExtractionBranchTheorem`, proving that a same-commitment
  fork with non-colliding Fq12 actions extracts a canonical 41-byte response
  opening the quotient digest; otherwise the proof branches to the named bad
  events.
- [x] Write the concrete soundness advantage formula in the executable report:
  `final_soundness_advantage_formula` now records
  `Adv_sound <= Adv_QROM_measure_reprogram + Pr[Fq12 action collision] +
  Adv_extraction_break + Adv_matrix_mismatch + Adv_pad_prf +
  Adv_frame_mismatch`, with the `2^16` QROM deployment ledger giving `130`
  transcript bits and `131` action-collision bits.
- [x] Tighten quotient-hiding sampler review: `quotient_hiding_sampler_review`
  records zero per-slot TV distance under the allowed rejection sampler,
  active-pad-output exclusion, independent-slot composition, and the role of
  the `133`-bit support floor as audit-only.
- [x] Turn the remaining primitive assumptions into a third-party-review
  packet: exact SHA-256 random-oracle/PRF assumptions, extraction-break
  cryptanalysis, matrix sampler implementation review, KATs, and fuzz/property
  tests.  See
  `research/cryptanalysis/signature_readiness_20260516/third_party_review_packet.md`
  and the executable `third_party_review_packet` JSON section.
- [x] Package a third-party-review bundle: specification, KATs, fuzz corpus,
  cryptoanalysis report, Lean theorem map, benchmark harness, and known
  limitations.  The packet cross-links the executable report, KAT command,
  test command, soundness formula, matrix review target, and known limitations.
- [x] Add the internal security-review audit:
  `research/cryptanalysis/signature_readiness_20260516/internal_security_review_audit.md`
  and the executable `internal_security_review_audit` JSON section.  It records
  the now-closed QROM frame and full 254-byte public verifier byte-path items,
  plus packed-SIS cryptanalysis depth, sampler discipline,
  side-channel/stateful lifecycle work, and SHA-256 assumption memo.
- [x] Bind the 32-byte quotient proof commitment into the concrete
  Fiat-Shamir challenge frame, update `FiatShamirFrameCertificate`, refresh
  KATs, and change `qrom_theorem_fits_protocol_shape` back to true.  The
  deployed proof frame is now
  `H(public_key, message, slot_id, slot_digest, proof_commitment)` under the
  domain-separated profile/parameter frame; the legacy 181-byte ideal verifier
  remains explicitly outside this deployed proof path.
- [x] Replace the 181-byte ideal-reference verifier as the main public path
  with a full 254-byte `reference_sign_with_proof` /
  `reference_verify_with_proof` flow that verifies the 73-byte quotient proof.
  The public path verifies the proof-bound challenge frame, q-syndrome digest
  match, local opening relation, proof-commitment mutation rejection, masked
  response mutation rejection, and consumed-slot replay.
- [x] Extend the cryptoanalysis campaign from smoke probes to an executable
  public-forgery probe.  `public_proof_forgery_probe` constructs an accepting
  254-byte signature from public key bytes, public slot digest, slot id, and
  message, without private key material or account hidden witness material.
- [x] Implement the strict linear sigma equation in the executable verifier.
  `verify_public_quotient_sigma_equation_signature` checks
  `L(z) = a + c * digest`; the public-forgery probe now records
  `sigma_equation_verifier_accepts = false`.
- [x] Test the adapted public post-challenge sigma-equation attack.
  `public_sigma_equation_forgery_probe` chooses `a`, derives `c`, solves the
  public equation for `z`, and verifies without private key material.  This
  shows the equation by itself is not enough.
- [x] Pivot the public proof/keygen relation so honest signatures satisfy the
  strict sigma equation through a prechallenge witness-bound commitment, making
  public conditioned-fiber representative reconstruction impossible or
  insufficient for acceptance.  This is now the committed-sigma/account-carried
  proof track below; remaining work is captured by the account-link and
  response-link certificate tasks.
- [x] Open the replacement proof surface in `Descender.CommittedSigma` and
  document the committed-sigma architecture in
  `committed_sigma_proof_plan.md`.
- [x] Instantiate the quotient-level committed-sigma algebra adapter.
  `FullResponseQuotient.statementAlgebra` packages the full-response
  quotient operations as `LinearSigmaAlgebra` and proves the verifier equation
  agrees with the existing full-response field equation.
- [x] Define the minimal prechallenge randomizer opening interface.
  `PrechallengeRandomizerOpeningInterface` states that
  `prechallengeWellFormed`, the sigma equation, and `openingAccepts` imply
  `BackedByHiddenWitness` or `openingProofBreak`.
- [x] Formalize the reveal-r exclusion and opening candidate gates.
  `RevealedRandomizerWitnessRecovery` proves backed transcripts with revealed
  randomizer recover the witness, and `OpeningInstantiationCandidate` rejects
  candidates with `revealsRandomizer = true` before byte ranking.
- [x] Formalize the first opening-instantiation frontier.  Reveal-r, bare
  Hash(r), and z-only openings each have Lean `not_structurallyAdmissible`
  theorems; `abstractHiddenLinkProofCandidate` is the remaining target class.
- [x] Formalize the committed-sigma byte-pressure frontier for the n32/q12289
  target.  `N32Q12289CommittedSigmaByteFrontier` proves that the current
  41-byte masked response gives a 254-byte transaction before any hidden-link
  proof and leaves only 2 bytes under a 256-byte cap, while the 37-byte and
  39-byte fiber-saturated targets leave 6 and 4 bytes respectively.
- [x] Split hidden-link bytes into signature bytes vs account-state bytes.
  `AccountCarriedHiddenLinkInstantiation` and
  `accountCarriedFiberSaturatedTarget` make the live 250-byte path explicit:
  zero extra hidden-link bytes in the signature, with the remaining link
  evidence carried by spendable slot/account state.
- [x] Formalize the verifier-facing account-carried hidden-link contract.
  `AccountCarriedHiddenLinkInterface` says account slot well-formedness, the
  sigma equation, and public opening acceptance imply
  `AccountBackedByHiddenWitness` or `accountLinkProofBreak`; backed account
  transcripts then convert to the generic `BackedByHiddenWitness`.
- [x] Formalize the bare-account-hash rejection.  An accepting but unbacked
  account-carried transcript forces `accountLinkProofBreak`, and
  `bareAccountHashLinkTarget` is structurally inadmissible because account-state
  bytes alone do not instantiate the opening/link proof.
- [x] Package the account-carried target as a top-level soundness certificate.
  `AccountCarriedVerifierAccepts` and `AccountCarriedSoundnessCertificate`
  state that accepted account-carried transcripts are account-backed or trigger
  `SoundnessBreaks.any`; `fromHiddenLinkInterface` derives the certificate from
  `AccountCarriedHiddenLinkInterface`.
- [x] Decompose the concrete opening layer into local opening plus account-link
  proof.  `AccountCarriedOpening`,
  `AccountCarriedVerifierAcceptsWithAccountOpening`, and
  `AccountLinkProofSystem` now state the adapter a concrete link proof must
  instantiate.
- [x] Formalize and report the account-link proof candidate frontier.
  `LinkProofInstantiationCandidate` rejects reveal-link, local-opening-only, and
  bare-account-hash strategies; `accountCarriedLinkProofCandidate` is the live
  zero-signature-byte class, conditional on instantiating `AccountLinkProofSystem`.
  The candidate type now also requires
  `nonPublicProjectionOfSigmaEquation`, so public projections of the sigma
  equation cannot pass the structural gate.
- [x] Separate slot-bound first-message binding from hidden-link soundness.
  `SlotBindsProofCommitment` is a necessary account-state invariant, but
  `slot-bound-first-message-only` is rejected because binding `a` alone does not
  link the post-challenge response to hidden `w,r`.
- [x] Add the first concrete zero-signature-byte link construction class.
  `HomomorphicAccountLinkInterface` checks
  `Commit(response) = Commit(r) + challenge * Commit(w)` and converts to
  `AccountLinkProofSystem` with `LinkProof = Unit`; the remaining obligation is
  `linkEquationSoundness`.
- [x] Exclude the degenerate public-projection homomorphic shortcut.
  `linkProofAccepts_of_publicProjection` proves that if `Commit` is just a
  homomorphic projection of the public sigma equation, the link check is implied
  by `L(response) = proofCommitment + challenge * statementDigest` and cannot
  discharge hidden-link soundness.
- [x] Split homomorphic soundness into pre-response backing and response-link
  kernel soundness.  `AccountBackedExceptResponse` captures account/opening
  backing of the same hidden `w,r`; `ResponseLinkSoundness` captures the exact
  deterministic-code obligation that the link equation forces
  `response = r + challenge*w` or a link-kernel break.
- [x] Add the Lean bridge from bounded joint-kernel certificates to response
  link soundness.  `BoundedResponseLinkKernelCertificate.responseLinkSoundness`
  now reduces the `[0, 3]` code target to the byte-level bounded-box kernel
  certificate plus response-code equality.
- [x] Add the first executable response-link code frontier.  The full-kernel
  coordinate code uses 11 q-residues (20 bytes) and kills the whole public
  quotient kernel, but it is rejected for public account state because
  `L(w) + C(w)` reveals the witness.  The live route is now a bounded-box
  kernel certificate for a smaller code; the current `[0, 3]` target has a
  4-byte code, leaves kernel dimension 9, and the LLL probe finds coefficient
  bound 139 versus the response-difference bound 96.
- [x] Add a reusable bounded-kernel checker script.
  `scripts/research/response_link_bounded_kernel_check.py` reconstructs the
  canonical matrix and runs `lll`, `lattice-enum`, `exact-enum`, `z3`,
  `cp-sat`, `scip`, or split exact checks for the `[0, 3]` response-link box.
- [x] Close the concrete `[0, 3]`, bound-96 bounded-box kernel search with
  replayable computational enumeration.  The fast `lattice-enum` run enumerates
  the full intersection lattice inside the Euclidean cover ball
  `23 * 96^2 = 211968`, checks `99458` nonzero vectors, and finds no
  coefficient-bound-96 vector.  Artifact:
  `response_link_bounded_kernel_enum_n32_q12289_coords_0_3_b96.json`.
  The slower `exact-enum` run replays the same enumeration with rational
  Gram-Schmidt pruning and exact integer leaf-norm checks, also returning
  `infeasible`.  Artifact:
  `response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json`.
- [x] Add certificate replay and deterministic fingerprints.  `--verify-json`
  replays `lattice-enum` or `exact-enum` artifacts and compares the stable
  certificate fields.  The current `[0, 3]` artifacts verify with no mismatches;
  `canonical_basis_sha256` is
  `c6a0cd8f470d1ec80886be3352f6144a90642826f0962ad2d077e727217fcf69`,
  `reduced_basis_sha256` is
  `1348db737dfb311a4d53b3e5969774b82515a7e7f5b76689e39ee173ee9a44f0`, and
  `cover_ball_vector_sha256` is
  `e269022d387b191cc3d926a32a746e8e87b9e89e720dd04fb1dc91f6be4a0c95`.
- [x] Make the exact replay independent of LLL regeneration.  The artifacts now
  embed the reduced basis; `--verify-json` checks its rows satisfy the modular
  relation, checks integer membership in the canonical basis, checks the
  determinant matches
  `1791561079655532333095965150039192629866166308243408855041`, and then runs
  exact-rational enumeration from the embedded basis.
- [x] Add a standalone stdlib-only certificate verifier.
  `scripts/research/verify_response_link_bounded_kernel_certificate.py` reads a
  JSON artifact, reconstructs the canonical matrix, validates the embedded
  basis, and replays the exact enumeration without `fpylll`, OR-Tools, Z3, or
  the broader research harness.  It verifies both current `[0, 3]` artifacts
  with no mismatches.
- [x] Emit an audit manifest for the bounded-kernel certificate.  The standalone
  verifier now supports `--write-manifest`; the current manifest binds the exact
  certificate hash, verifier script hash, canonical statement, replay command,
  basis validation, search counts, shortest cover-ball vector, and cover-ball
  transcript hash.  Manifest:
  `response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json`;
  payload hash
  `da10b823ad8f4142dfeccdbf7bbcddbb5e7148f420d8ff36687bae412d09fbf0`.
- [x] Add a manifest auditor for review replay.
  `scripts/research/audit_response_link_bounded_kernel_manifest.py` performs a
  fast static audit of the manifest payload hash, certificate hash, verifier
  hash, statement fields, lattice-basis hashes, determinant fields, and recorded
  enumeration counts.  Its `--replay` mode invokes the standalone verifier and
  checks for `status = verified` with no mismatches.
- [x] Wire the bounded response-link certificate into the account-link proof
  system interface.  Lean now has `AccountPreResponseBackingSystem`,
  `AccountPreResponseBackingSystem.toAccountLinkProofSystem`, and
  `AccountPreResponseBackingSystem.toAccountLinkProofSystemOfBoundedKernel`.
  The global composition is closed: pre-response account backing plus
  `BoundedResponseLinkKernelCertificate` yields an `AccountLinkProofSystem`
  whose breaks are exactly the pre-response backing break or link-kernel break.
- [x] Add the account-aware top-level soundness certificate bridge.
  `AccountCarriedAccountOpeningSoundnessCertificate` is the deployment-facing
  certificate for openings that inspect account state, and
  `AccountLinkProofSystem.toAccountOpeningSoundnessCertificate` converts an
  account-link proof system into that certificate.  The bounded `[0,3]`
  response-link path is now connected through to a top-level account-carried
  soundness certificate.
- [x] Add the one-step bounded-kernel deployment adapter.
  `AccountPreResponseBackingSystem.toAccountOpeningSoundnessCertificateOfBoundedKernel`
  turns pre-response backing plus `BoundedResponseLinkKernelCertificate` into
  the account-aware top-level certificate directly, and
  `accepts_accountBacked_of_noBreaks_ofBoundedKernel` states the no-break
  accepting-transcript consequence.
- [x] Package the concrete bounded-kernel deployment premises.
  `AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations` names the
  exact object the N32/q12289 account-link instantiation must provide:
  bounded kernel certificate, proof-commitment agreement, response-commitment
  link equation, and response-difference-box condition.  Its adapters derive
  the account-link proof system and account-aware top-level soundness
  certificate.
- [x] Connect bounded-kernel deployment obligations to the candidate frontier.
  `LinkProofInstantiationCandidate.fromBoundedKernelDeploymentObligations`
  builds a candidate whose proof-system gate is
  `Nonempty AccountLinkProofSystem`, witnessed by the deployment obligations,
  and `fromBoundedKernelDeploymentObligations_structurallyAdmissible` discharges
  that gate.  Remaining frontier gates are representative rejection,
  non-public-projection, and witness hiding.
- [x] Pin the N32/q12289 zero-link-byte candidate to bounded-kernel deployment
  obligations.  `accountCarriedBoundedKernelLinkProofCandidate` wraps the generic
  frontier constructor with `signatureLinkProofBytes = 0`, and its structural
  admissibility theorem reuses the bounded-kernel deployment obligation object.
- [x] Tie the 250-byte fiber-saturated target to bounded-kernel deployment
  obligations.  `accountCarriedBoundedKernelFiberSaturatedTarget` uses the
  account-aware soundness certificate derived from the deployment obligations as
  the opening-interface witness; Lean proves `transactionBytes = 250`, under
  256 bytes, and structural admissibility once representative rejection and
  witness hiding are supplied.
- [x] Bundle the remaining N32/q12289 structural gates.
  `BoundedKernelTargetStructuralGates` packages representative rejection,
  non-public-projection, and witness hiding; the `OfGates` constructors derive
  both the structurally admissible zero-link-byte candidate and the 250-byte
  fiber-saturated target from the gates object.
- [x] Make the bounded-kernel structural gates review-facing.
  `BoundedKernelTargetStructuralReview` names the three remaining break buckets:
  post-challenge representative acceptance, public-projection leakage, and
  witness-hiding failure.  Lean proves `allHold_of_noBreaks`, and the `OfReview`
  constructors derive the zero-link-byte candidate and 250-byte target from the
  reviewed no-break hypothesis.
- [x] Compile the reviewed 250-byte target theorem.
  `accountCarriedBoundedKernelReviewedTarget_accepts_accountBacked_and_structurallyAdmissible`
  combines bounded-kernel deployment obligations, structural review, and the
  three no-break hypotheses.  For an accepting account-aware transcript it
  proves `AccountBackedByHiddenWitness`, structural admissibility, and
  `transactionBytes = 250`.
- [x] Bundle the concrete account-link review object.
  `ReviewedBoundedKernelAccountLinkInstantiation` packages the pre-response
  backing system, bounded-kernel deployment obligations, and structural review
  gates into one object.  Its `anyBreak` predicate is the union of
  `preResponseBackingBreak`, `linkKernelBreak`, and
  `structuralReview.breaks.any`; under `¬ anyBreak`, Lean proves accepted
  transcripts are account-backed and the target is structurally admissible with
  `transactionBytes = 250`.
- [x] Pin the reviewed account-link byte layout.
  `ConcreteReviewedAccountCommitmentRepresentation` ties a reviewed account-link
  instantiation to the current account-state layout:
  `32 B` witness commitment + `32 B` randomizer commitment + `4 B` `[0, 3]`
  response-link code = `68 B` account/state bytes.  Lean proves the target keeps
  zero spend-signature link bytes and `transactionBytes = 250`.
- [x] Expose the concrete online-plus-account byte theorem.
  `ConcreteReviewedAccountCommitmentRepresentation.target_onlinePlusAccountBytes_eq`
  proves that the reviewed representation's spend transaction plus account-link
  bytes total `318 B`, matching the committed-sigma/account-carried cost-model
  benchmark row.
- [x] Add the first committed-sigma/account-carried parameter rerank.
  `scripts/parameter_search/committed_sigma_cost_model.py` applies the current
  proof gates before ranking byte costs and emits
  `research/parameter_search/committed_sigma_account_carried_20260521/`.
  The current Pareto row is `reviewed-account-carried-bounded-kernel-37b`:
  `250 B` transaction, `68 B` account/state, `69 B` proof payload, and `318 B`
  if transaction and account-link state are simply added.  The `252 B`
  fiber-state-preserving row remains an admissible fallback; the old `254 B`
  public sigma-equation row is retained as a structurally rejected control.
- [x] Add external byte-size comparators to the committed-sigma/account-carried
  cost model.
  The generated Markdown/JSON now includes Descender target/fallback rows
  alongside Ed25519, Falcon/FN-DSA-512, ML-DSA-44, ML-DSA-65,
  SLH-DSA-SHA2-128s, and SLH-DSA-SHA2-128f, with source URLs per row.  These
  rows are size comparators only; they do not upgrade Descender's review status.
- [ ] ACTIVE: complete external response-link certificate review.
  The exact-rational artifact now has a standalone stdlib verifier and an audit
  manifest; the remaining review task is independent replay by someone outside
  this workspace and, ideally, third-party SVP/BKZ validation of the exact
  matrix and accepted-response constraints.
- [x] Add a reproducible local BKZ/LLL attack-review harness for the
  response-link certificate.
  `scripts/research/response_link_lattice_attack_review.py` rebuilds the exact
  lattice, validates the pinned determinant and hashes, reports the geometric
  margin, and runs local `fpylll` LLL/BKZ reductions.  The current artifact
  `response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json` reports
  no box vector found by LLL or BKZ block sizes `10,15,20,23`; the shortest
  reduced row has coefficient bound `139`, `43` above the allowed bound `96`.
- [x] Finish the proof-interface account-link instantiation wrapper.
  The target is no longer a generic `Hash(r)`-style
  `PrechallengeRandomizerOpeningInterface`; it is exposed as an
  `AccountLinkProofSystem` whose response commitment is not a public projection
  of the sigma equation and whose `[0, 3]` bounded-kernel certificate discharges
  `ResponseLinkSoundness`.  `ConcreteReviewedAccountCommitmentRepresentation`
  now directly exposes its `accountLinkProofSystem`, `anyBreak` surface,
  structural admissibility under `¬ anyBreak`, zero spend-signature link bytes,
  `250 B` spend transaction bytes, `68 B` account-link bytes, and `318 B`
  online-plus-account accounting.  The remaining implementation work is the
  production-shaped account-carried reference crate and constant-time/state
  lifecycle hardening.
- [x] Integrate local security-review gates into the committed-sigma/account-carried
  cost model.
  `committed_sigma_account_carried_cost_model.json` and its Markdown companion
  now include response-link exact-certificate status, response-link LLL/BKZ
  attack-review status, quotient/SIS bare-quotient control status, and explicit
  open gates for external independent replay and production reference timings.
- [x] Wire the current assumption/budget review graph into the cost model.
  The cost model now imports the assumption attack-review matrix, QROM
  security-loss review, pad/sampler boundary review, conditional soundness
  budget ledger, and local timing summary.  It reports five executable-backed
  assumption rows, zero memo-only rows, `128` minimum checked budget bits, and
  keeps deployment/review gates open.
- [x] Add reference timing smoke benchmarks.
  `signature_q12289_local_delta_stateful --bench` measures the current reference
  `reference_keygen`, `reference_sign_with_proof`, and
  `reference_verify_with_proof` paths.  `scripts/research/reference_timing_report.py`
  generated `reference_timing_report_n32_q12289.{json,md}` with 10,000
  iterations: keygen for two slots averages about `421 us`, signing about
  `27 us`, and verification about `27 us` on the current machine.  This is a
  reference/research timing smoke benchmark for the legacy 254-byte proof path,
  not a production constant-time claim.
- [x] Add a production-reference split gap report.
  `scripts/research/production_reference_gap_report.py` generates
  `production_reference_gap_report_n32_q12289.{json,md}` from the current source
  and review artifacts.  It turns the production split into concrete API rows:
  `keygen`, `account_commit`, `sign`, `verify`, `consume_slot`, canonical
  encoders/decoders, and constant-time signing, with explicit partial/open
  status for each.
- [x] Add the first production-shaped reference facade.
  `signature_q12289_local_delta_stateful.rs` now exposes
  `production_keygen`, `production_sign`, and `production_verify` around
  explicit production/public/account/signature types, with account-state
  assembly now delegated to the library `account_state_from_slot_seeds` path.
  The facade pins the reviewed `68 B` account-link layout as
  32-byte witness commitment, 32-byte randomizer commitment, and 4-byte
  `[0,3]` response-link code; tests show signing, verification, slot
  consumption, replay rejection, account randomizer binding, and response-link
  code rejection.  It intentionally still reports `254 B` executable
  signatures because it wraps the legacy public quotient proof path rather than
  claiming the final `250 B` Rust verifier is implemented.
- [x] Extract the reviewed account-link byte layout into a small Rust library
  module.
  `tools/parameter-search/src/production_reference.rs` now defines
  `ProductionAccountCommitment`, the `68 B` reviewed account-link constants,
  canonical packed-q-syndrome validation, and malformed-layout tests.  The
  research binary's production facade uses this module as the single source of
  truth for account commitment construction and validation; challenge,
  response, opening, proof encoders, and the sign/verify facade still need to
  move out of the large research binary.
- [x] Extract production account-state assembly into the Rust library module.
  `ProductionAccountSlotSeed` and `account_state_from_slot_seeds` now assemble
  sorted production account slots from legacy slot witness commitments plus the
  reviewed randomizer commitment, then run the library account-state
  validation path.  The keygen facade no longer hand-builds
  `ProductionAccountSlot` values in the research binary.
- [x] Extract the production account/signature state boundary into the Rust
  library module.
  `ProductionPublicKey`, `ProductionAccountSlot`, `ProductionAccountState`,
  and `ProductionSignature` now live beside `ProductionAccountCommitment`.
  The account-state type validates sorted canonical slots, exposes a
  `mark_consumed` transition, and is used by the research binary facade for
  in-memory slot consumption.  The secret key wrapper, signing algorithm,
  verifier algorithm, and canonical encoders still live in the research binary
  until the selected `284 B/Fq12` proof-semantics verifier replacement is ready.
- [x] Extract public-key byte validation into the Rust library module.
  `ProductionPublicKey::new` now enforces the canonical `172 B` public-key
  layout (`32 B` seed plus `140 B` packed q-coordinate payload), rejects bad
  lengths and noncanonical q residues, and the research binary's keygen facade
  constructs public keys through that checked boundary.  Test-only unchecked
  construction is explicitly named for fixtures that need malformed values.
- [x] Extract the production keypair/secret-key API wrapper into the Rust
  library module.
  `ProductionSecretKey<T>` and `ProductionKeypair<T>` now live with the other
  production reference boundary types.  The `T` parameter is still the
  research binary's legacy `ReferenceKeypair`, so this is an API-shape
  extraction rather than a claim that production keygen or constant-time secret
  handling is complete.
- [x] Extract the production secret-key signing handoff into the Rust library
  module.
  `sign_with_secret_key` now keeps the secret-key wrapper opaque to the
  research binary and gives the signer callback only the wrapped reference
  material, checked slot id, and checked witness commitment.  It composes with
  `sign_with_account_binding`, so slot-state, slot-id, and account-randomizer
  binding remain library-owned before the legacy `254 B` callback is invoked.
- [x] Extract the production verifier handoff into the Rust library module.
  `verify_with_account_binding` now owns the public-key/account/signature
  callback boundary: it validates the signature layout, slot state, account
  commitment, and account randomizer link before passing a checked context to
  the verifier callback.  The callback is still the legacy `254 B` public
  quotient verifier, so this closes a facade boundary without claiming the
  selected `284 B/Fq12` verifier has accepting proof semantics.
- [x] Extract checked account-state record export into the Rust library module.
  `ProductionAccountState::checked_slot_records` now revalidates sorted slots
  and account-link commitments before exporting only slot id, witness
  commitment, randomizer commitment, and unused bit to callback glue.  The
  legacy verifier conversion uses this record surface instead of walking
  production commitment internals directly.
- [x] Extract checked account-state projection into the Rust library module.
  `ProductionAccountState::project_checked_slot_records` now combines
  revalidation and callback projection, so legacy verifier glue receives a
  library-checked record slice and supplies only the old account-state shape.
- [x] Extract accepted-verification slot consumption into the Rust library
  module.
  `verify_with_account_binding` now returns the checked signature witness
  alongside the verifier callback result, and `consume_verified_signature`
  consumes exactly that checked slot after acceptance.  The production facade
  no longer mirrors the legacy verifier's internal account state to decide
  which slot was consumed.
- [x] Package completed production verification in the Rust library module.
  `ProductionVerification<R>` and `complete_verified_signature` now bundle the
  checked signature witness, verifier callback report, and next account state.
  The research binary still supplies the legacy report type, but it no longer
  hand-assembles the accepted/rejected account transition around that report.
- [x] Extract typed production verification rejection into the Rust library
  module.
  `ProductionVerificationRejection` now carries slot id plus rejection reason
  for both pre-callback account-binding failures and post-callback completion
  failures.  The research binary's `production_reject` adapter consumes this
  typed object instead of accepting loose slot/reason pairs.
- [x] Extract production rejection report-field construction into the Rust
  library module.
  `ProductionRejectionReportFields` and `rejection_report_fields` now derive
  the proof model, expected signature length, optional slot id, claimed
  challenge bytes, and rejection reason from the typed rejection plus signature
  bytes.  The research binary still maps those fields into its legacy
  `StatefulVerifyReport`, but no longer owns the production signature offset
  logic or rejection-profile metadata.
- [x] Make the production gap report read executable signature bytes from the
  Rust library constants.
  `production_reference_gap_report.py` now evaluates simple `usize` constant
  expressions from `production_reference.rs`, records that file as the byte
  source, and no longer treats the research binary as authoritative for the
  executable `254 B` reference-path length.
- [x] Make the production gap report read account-link and reviewed target bytes
  from the Rust library constants.
  The generated production facade section now uses `REVIEWED_ACCOUNT_LINK_BYTES`
  and `REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES` from
  `production_reference.rs`, while the cost-model frontier remains recorded
  separately in the target summary.
- [x] Add explicit production gap report consistency gates between cost-model
  and Rust-library byte targets.
  The generated report now records whether the cost-model transaction target
  matches `REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES` and whether the
  cost-model account-link bytes match `REVIEWED_ACCOUNT_LINK_BYTES`, so byte
  drift between the search model and reference library is visible.
- [x] Centralize production gap report library capability gates.
  `production_reference_gap_report.py` now computes one
  `library_capabilities` map and feeds both the production-facade summary
  booleans and the generated API rows from it.  This keeps extraction-status
  claims for account state, codecs, challenge framing, and sign/verify
  boundaries from drifting between the Markdown summary and the machine-readable
  JSON.
- [x] Extract verifier-facing signature decoders into the Rust library module.
  `production_reference.rs` now parses the executable signature layout into
  slot id, compact challenge, compact active responses, local openings, and the
  linear-sigma proof.  The production facade calls `ProductionSignature::decode`
  before account binding and legacy verifier delegation, so malformed
  signature layouts are rejected at the library boundary.
- [x] Extract response/opening/proof encoders into the Rust library module.
  `production_reference.rs` now owns canonical encoding for active responses,
  local openings, linear-sigma proof bytes, and full executable signature
  assembly.  The research binary's encoder wrappers delegate to the library,
  and library tests cover round-trip signature assembly plus out-of-range
  response/opening rejection.  The challenge hash-frame encoder, sign/verify
  algorithms, and final `250 B` account-carried verifier remain to be
  moved/implemented.
- [x] Extract challenge hash-frame construction into the Rust library module.
  `production_reference.rs` now owns the proof-bound Fiat-Shamir frame,
  legacy no-proof frame, and SHA-256-to-challenge byte derivation.  The frame
  uses the existing byte-compatible length prefixes (`u16` label length,
  `u64` data length), and the research binary's wrappers delegate to the
  library implementation.  The first attempted extraction used a different
  prefix width and failed the stable-vector test, so this is now covered by the
  existing binary KAT plus a library domain-separation test.  The sign/verify
  algorithms and final `250 B` account-carried verifier remain to be
  moved/implemented.
- [x] Extract the production sign/verify account-binding boundary into the Rust
  library module.
  `production_reference.rs` now exposes `sign_with_account_binding` and
  `verify_account_binding`.  Signing validates slot state, hands the witness
  commitment to a signer callback, decodes the returned signature, and checks
  slot-id/randomizer binding.  Verification decodes the signature, checks slot
  state, validates the account commitment, and checks randomizer binding before
  the research binary delegates to the legacy public quotient verifier.  The
  heavy signing/verifier callbacks still live in the research binary and still
  implement the `254 B` path; replacing that callback with the reviewed
  `250 B` account-carried verifier remains open.
- [x] Add account-state lifecycle cost rows to the committed-sigma/account-carried
  cost model.
  `committed_sigma_account_carried_cost_model.{json,md}` now separates the
  `250 B` spend transaction from the 68-byte live account-link slot and reports
  rolling one-slot state, 16/256/1024-slot precommitted pools, and the stateless
  control row.  The rolling target is `318 B` transaction plus live read and
  touches `136 B` of account state per spend if the consumed slot is atomically
  replaced.  Finite precommitted pools keep the marginal live read at `68 B`
  but move cost into persistent account storage and require ledger-level
  spent-slot/nullifier accounting outside the proof byte model.
- [x] Add production-facade rows to the reference timing smoke benchmark.
  `signature_q12289_local_delta_stateful --bench` now reports both direct
  reference rows and `production_facade_keygen`, `production_facade_sign`, and
  `production_facade_verify` rows.  These rows measure the checked
  production-shaped account/public/signature boundary but still wrap the
  legacy `254 B` proof callback, so they are implementation-shape smoke
  timings rather than accepting `284 B/Fq12` proof-semantics or constant-time
  production timings.
- [x] Add production-reference crate facade timing.
  `tools/production-reference/src/bin/production_reference_facade_timing.rs`
  measures the separated `descender-production-reference` crate facade with
  deterministic dummy callbacks.  `scripts/research/production_reference_facade_timing_report.py`
  generates
  `production_reference_facade_timing_report_n32_q12289.{json,md}` and the
  production gap report now cross-links it.  The current 10,000-iteration local
  run reports about `0.69 us` for sanitized secret-key facade signing,
  `0.41 us` for sanitized account-binding accept verification, `0.40 us` for
  sanitized callback rejection, and `1.18 us` for fresh reserve-sign-consume
  setup.  This is API-wrapper timing only: the callbacks are dummy callbacks,
  the executable signature length is still `254 B`, and there is no
  constant-time claim.
- [x] Wire production-reference crate facade timing into the cost model.
  `scripts/parameter_search/committed_sigma_cost_model.py` now imports
  `production_reference_facade_timing_report_n32_q12289.json`, adds a
  `production_reference_crate_facade_timing` review gate, and labels timing
  rows by layer so the main byte/security dashboard distinguishes the legacy
  research-reference binary rows from separated crate-wrapper timing.
- [x] Add a one-command review-gate replay runner.
  `scripts/research/review_gate_replay.py` runs the generated-artifact pipeline
  in dependency order, optionally reruns timing, optionally asks for exact
  response-link replay, validates the expected JSON artifacts, and writes
  `review_gate_replay_report_n32_q12289.{json,md}`.  The current local
  `--skip-timing --replay-exact` replay passes with `82` generation/audit
  steps, including a standalone response-link certificate replay,
  `0` failed steps, and `79` valid JSON artifacts with byte counts and
  SHA-256 hashes.
- [x] Add a hashed third-party review manifest.
  `scripts/research/third_party_review_manifest.py` generates
  `third_party_review_manifest_n32_q12289.{json,md}` as a top-level table of
  contents for the review packet.  It currently indexes `271` entrypoint,
  closed-sidecar, byte-dashboard, response-link, red-team, assumption-review,
  QROM/sampler, proof-target, production-reference, timing, script, and
  crate-boundary files with file sizes and SHA-256 hashes, and records that
  the latest exact replay passed.
- [x] Add a verifier for the third-party review manifest.
  `scripts/research/verify_third_party_review_manifest.py` checks every file
  indexed by `third_party_review_manifest_n32_q12289.json` against its recorded
  presence, byte count, SHA-256 hash, and JSON schema, and also fails on an
  unexpected manifest schema, stale summary counters, or duplicate indexed
  paths.  The current verification passes for `271` indexed files with `0`
  failed files and no manifest-level failures.
- [x] Add a local red-team cryptoanalysis harness.
  `scripts/research/red_team_cryptoanalysis.py` confirms the bare public
  quotient negative-control break, sweeps zero/single/two-coordinate
  response-link projections with BKZ-23, records exact integer witnesses for
  broken nearby coordinate-code variants, and reports no local reduced-basis
  break against the selected `[0,3]` target.
- [x] Add a fail-closed assumption closure ledger.
  `scripts/research/assumption_red_team_closure.py` generates
  `assumption_red_team_closure_n32_q12289.{json,md}` from the assumption
  surface, attack matrix, red-team sweep, conditional budget, and production
  gap report.  It records each row's kill strategy, local attacks attempted,
  promotion criteria, and residual risk while keeping deployment readiness
  false.
- [x] Add the reviewed `250 B` account-carried signature codec to the
  production-reference library.
  `tools/parameter-search/src/production_reference.rs` now exposes
  `ProductionReviewedAccountCarriedProof`,
  `encode_reviewed_account_carried_signature`,
  `decode_reviewed_account_carried_signature`, canonical `37 B` masked-response
  packing for the 21 non-link q-residues, and
  `verify_reviewed_account_carried_account_binding`.  This closes the byte-format
  gap but not the cryptographic equation verifier gap: the executable facade
  still wraps the legacy `254 B` callback path.
- [x] Move the reviewed quotient digest map into the production-reference
  library.  `reviewed_account_carried_response_syndrome_digest` now decodes the
  `37 B` response as 21 q-residues, applies the deterministic
  `10 x 21` q-ary quotient matrix, packs the 10-residue syndrome into the
  32-byte digest field, and `verify_reviewed_account_carried_account_binding`
  enforces strict 10-residue digest canonicality on the reviewed path.  The
  remaining gap is the final Fiat-Shamir/linear-sigma verifier equation over
  this quotient map.
- [x] Add reviewed `250 B` transcript challenge binding at the library
  boundary.  `sign_reviewed_account_carried_with_transcript_binding` and
  `verify_reviewed_account_carried_transcript_binding` recompute the
  Fiat-Shamir challenge over public key, message, slot id, account witness
  digest, and proof commitment, then reject mismatched challenge bytes.  This
  closes the frame-binding sub-obligation for the reviewed codec; the remaining
  verifier gap is the algebraic `37 B` masked-response equation itself.
- [x] Add a fail-closed reviewed-equation spec gap artifact.
  `scripts/research/reviewed_equation_spec_gap.py` generates
  `reviewed_equation_spec_gap_n32_q12289.{json,md}` and records the current
  dimension mismatch explicitly: the legacy executable equation has a pinned
  12-row/`Fq12` challenge action, while the reviewed `250 B` byte target is a
  10-row `10 x 21` quotient with no pinned reviewed challenge-action
  algebra/domain yet.  This blocks implementing the final equation by guesswork.
- [x] Analyze the concrete challenge-action byte/security options.
  `scripts/research/reviewed_challenge_action_options.py` generates
  `reviewed_challenge_action_options_n32_q12289.{json,md}`.  It shows the exact
  `250 B`/10-row row keeps hiding but fails the `2^16` quantum action-collision
  ledger, while a `252 B`/12-row row is the smallest listed option that keeps
  public quotient bits, post-local-state hiding bits, and the existing `Fq12`
  action-collision budget at or above 128 bits.  The exact `250 B` target
  remains possible only with a new nontrivial action/adapter argument.
- [x] Add a reviewer-facing status index.
  `scripts/research/third_party_review_status_index.py` generates
  `third_party_review_status_index_n32_q12289.{json,md}` as the landing page
  for third-party review.  It summarizes target bytes, replay status,
  review-gate counts, per-gate status rows, key metrics, terminal deployment
  blockers, the selected-deployment terminology guard, and the external review
  work that remains before deployment claims.
- [x] Add a verifier for the reviewer-facing status index.
  `scripts/research/verify_third_party_review_status_index.py` rebuilds the
  status index from source artifacts and compares all stable fields except the
  generation timestamp, making stale/manual status edits fail closed.
- [x] Add a selected-deployment terminology guard.
  `scripts/research/verify_selected_deployment_terminology.py` scans the review
  packet, cost-model outputs, production-reference crate, and relevant
  generators for stale wording that would present an exact 250-byte verifier as
  the selected deployment target.  It emits
  `selected_deployment_terminology_verification_n32_q12289.{json,md}`, requires
  the selected `284 B/Fq12` fail-closed deployment language to appear, and is
  now consumed by the status index and hashed manifest.  The current check
  embeds synthetic negative controls for stale 250-byte wording and missing
  required selected-deployment phrases, and passes for `230` scanned files with
  `0` terminology failures.
- [x] Add a compact-signature TODO consistency guard.
  `scripts/research/verify_compact_signature_todos_consistency.py` checks the
  main checklist against live replay and manifest counts, rejects stale
  selected-250 wording, and emits
  `compact_signature_todos_consistency_n32_q12289.{json,md}` for the review
  packet.  It embeds negative controls for stale manifest counts, stale replay
  counts, and selected-250 wording.  The status index and manifest now consume
  that guard so stale TODO edits fail closed.
- [x] Add a terminal finalization-report verifier.
  `scripts/research/verify_third_party_review_finalization.py` checks the
  intentionally manifest-excluded terminal finalization report after the full
  finalizer run.  It verifies the eight-step order, per-step success,
  manifest-verification result, finalizer contract, terminal-report exclusion
  set, recommended terminal verification command, and referenced output paths,
  and independently reopens the manifest to confirm terminal reports are absent
  and finalization scripts are indexed, then recomputes the manifest shape,
  byte-count, hash, and JSON-schema checks before emitting
  `third_party_review_packet_finalization_verification_n32_q12289.{json,md}`.
  It also has `--self-test` negative controls for command drift, step-order
  drift, step failure, terminal-report-set drift, and missing manifest output;
  the normal verification artifact now embeds that self-test result.
  Those verifier outputs remain terminal artifacts rather than manifest inputs
  to avoid a cyclic hash claim; the finalizer contract now explicitly lists
  both terminal report pairs as manifest-excluded, while the verifier script
  itself is indexed by the manifest.
- [x] Add a review-packet finalization wrapper.
  `scripts/research/finalize_third_party_review_packet.py` runs
  `review_gate_replay.py`, regenerates standalone verification artifacts for
  the production preflight certificate and selected-deployment terminology
  guard, regenerates and verifies `third_party_review_status_index`, regenerates
  `third_party_review_manifest`, then runs
  `verify_third_party_review_manifest.py`.  The order matters because the
  status index consumes the standalone verification artifacts and the manifest
  hashes the replay report and status index; the finalization wrapper keeps
  this as a one-way dependency instead of a cyclic hash claim.  It also records
  a finalizer-contract check: the manifest must recommend the finalizer command,
  index the finalizer script, exclude both terminal finalization report pairs,
  and publish the exact follow-up terminal verifier command.  The current
  finalization passes all `8` ordered steps and verifies `136` manifest files.
- [ ] ACTIVE: replace the reference timing smoke benchmark with production-shaped
  account-carried timings after the selected `284 B/Fq12` verifier has
  accepting proof semantics, then complete constant-time/state-lifecycle review.
- [x] Add the first quotient/SIS lattice attack-review harness.
  `scripts/research/quotient_sis_lattice_attack_review.py` rebuilds the full
  `12 x 23` quotient matrix, constructs the integer q-kernel lattice, runs
  local LLL/BKZ reductions, and repeats the signed sparse relation probe.  The
  generated artifact
  `quotient_sis_lattice_attack_review_n32_q12289.json` confirms the expected
  structural point: the full quotient kernel has a short syndrome-zero vector
  with coefficient bound `68` inside the response-difference bound `96`, so the
  bare quotient is not sound by itself.  The vector has nonzero `[0, 3]`
  coordinates, so the selected account response-link code catches it.
- [x] Add a keygen/security-estimator assumption-surface report.
  `scripts/research/keygen_security_assumption_surface.py` generates
  `keygen_security_assumption_surface_n32_q12289.{json,md}`.  It turns the
  remaining matrix/keygen, bare-quotient, response-link, QROM, pad/sampler, and
  production lifecycle assumptions into a machine-readable review map with
  local artifact status, current evidence, and required external review action.
  The committed-sigma cost model now imports this report as an explicit
  `keygen_security_assumption_surface` review gate.
- [x] Add the reviewer-facing QROM challenge-budget memo.
  `scripts/research/qrom_challenge_budget_memo.py` generates
  `qrom_challenge_budget_memo_n32_q12289.{json,md}` from the Rust
  production-reference constants.  It pins the challenge frame labels, `u16/u64`
  length encoding, prechallenge `proof_commitment` field, 21-byte challenge
  derivation, `162` transcript-bound challenge bits, and deployment
  `2^16` quantum challenge-frame query ledger.  The keygen/security-estimator
  assumption surface now links to this memo and marks the remaining QROM task
  as external review of the memo rather than writing the memo.
- [x] Add the pad/sampler boundary memo.
  `scripts/research/pad_sampler_boundary_memo.py` generates
  `pad_sampler_boundary_memo_n32_q12289.{json,md}`.  It pins the SHA-256 pad
  domain/frame, `u32` counter encoding, direct conditioned quotient-fiber
  sampler support, fixed no-retry policy, and the forbidden active-pad /
  local-opening conditions that would invalidate the zero-TV hiding certificate.
  The keygen/security-estimator assumption surface now links to this memo and
  marks the remaining pad/sampler task as external sampler-code review.
- [x] Add the matrix/keygen estimator memo.
  `scripts/research/matrix_keygen_estimator_memo.py` generates
  `matrix_keygen_estimator_memo_n32_q12289.{json,md}` from the quotient/SIS
  attack-review artifact.  It pins the 12x23 SHA-256 rejection-sampled matrix,
  transcript hash, rank/kernel evidence, local LLL/BKZ rows, sparse probe, and
  the short bare-quotient negative-control vector that is caught by the `[0,3]`
  response-link code.  The keygen/security-estimator assumption surface now
  links to this memo while keeping the external estimator/SIS review open.
- [x] Add the production lifecycle hardening memo.
  `scripts/research/production_lifecycle_hardening_memo.py` generates
  `production_lifecycle_hardening_memo_n32_q12289.{json,md}` from the
  production gap and timing artifacts.  It pins slot reservation/consumption,
  replay rejection, crash recovery, concurrency, reorg, backup/restore,
  spent-ledger, constant-time, and logging requirements.  The keygen/security
  assumption surface now links to this memo while keeping production hardening
  open because the executable facade still wraps the legacy `254 B` proof
  callback rather than the reviewed `250 B` account-carried verifier.
- [x] Add the first executable production lifecycle manager.
  `tools/parameter-search/src/production_reference.rs` now exposes
  `ProductionAccountLifecycleManager`, `ProductionSlotReservation`, and
  `ProductionLifecycleError`.  The manager reserves slots, blocks duplicate
  reservations, signs reserved slots while consuming them before release,
  consumes accepted verification, rejects replay against consumed slots, and
  burns ambiguous reserved slots during recovery.  This closes the in-memory
  lifecycle invariant harness; durable storage, interprocess locking,
  backup/restore, reorg, and consensus spent-ledger integration remain open.
- [x] Add canonical lifecycle snapshot and crash-recovery semantics.
  `ProductionLifecycleSnapshot` records checked account-slot records, active
  reservation tokens, and the next reservation epoch.  `from_snapshot` rejects
  unsorted account records, duplicate active reservations, consumed reserved
  slots, and non-monotonic reservation epochs.  `recover_snapshot_burning_reservations`
  restores by burning every ambiguous reserved slot, giving the durable backend
  a precise crash-recovery policy to implement.
- [x] Add a file-backed lifecycle reference store.
  `ProductionLifecycleFileStore` persists `ProductionLifecycleSnapshot` with
  schema checks and atomic temp-write/rename commits.  It can load a manager,
  save a manager, and recover by burning active reservations and rewriting the
  consumed snapshot.  This makes crash/restart behavior executable in tests;
  production-grade storage locking, backup/restore, reorg, and consensus
  spent-ledger integration remain open.
- [x] Add lifecycle store writer serialization.
  `ProductionLifecycleStoreLock` uses atomic lock-file creation to serialize
  file-store writers.  Duplicate acquisition returns `lifecycle store is
  locked`, and dropping the guard removes the lock.  This closes the first
  interprocess locking harness; stale-lock policy, backup/restore, reorg, and
  consensus spent-ledger integration remain open.
- [x] Add timestamped stale-lock policy to the lifecycle store.
  `ProductionLifecycleLockRecord` persists the lock schema, pid, store path,
  and acquisition timestamp.  `break_stale_lock` refuses future timestamps and
  non-stale locks, and only removes the lock once the caller-provided age
  threshold is met.  Backup/restore, reorg, and consensus spent-ledger
  integration remain open.
- [x] Add monotonic backup/restore rollback protection.
  `ProductionLifecycleSnapshot` now carries a monotonic `snapshot_sequence`,
  and every reserve/consume/recovery state transition increments it.
  `ProductionLifecycleRestoreAnchor` lets callers reject restored snapshots
  whose sequence is older than the trusted external anchor.  The reference file
  store exposes `load_manager_with_restore_anchor`; trusted anchor placement,
  reorg, and consensus spent-ledger integration remain open.
- [x] Add local reorg burn semantics.
  `ProductionReorgBurnRecord` and
  `burn_reorg_revealed_checked_signature` encode the rule that an already
  account-checked revealed signature burns its slot across chain rollback.  The
  transition consumes the slot, increments the snapshot sequence, and clears any
  local reservation for that slot.  Chain reorg event integration and consensus
  spent-ledger integration remain open.
- [x] Add the consensus-facing spent-slot ledger primitive.
  `ProductionSpentSlotLedger` keys spent slots by SHA-256(public key) plus slot
  id, records accepted verification and reorg-burn sources, and rejects stale
  wallet replay with `slot is already in spent ledger`.  The lifecycle manager
  exposes `verify_consume_and_record_spent` and
  `burn_reorg_revealed_checked_signature_and_record_spent`.  Real chain/state
  spent-ledger integration remains open.
- [x] Add a file-backed spent-slot ledger reference store.
  `ProductionSpentSlotLedgerFileStore` persists spent-slot records with
  `PRODUCTION_SPENT_SLOT_LEDGER_SCHEMA`, atomic temp-write/rename commits, and
  duplicate-record rejection during load.  This makes full-node style
  replay-state persistence executable in tests; production storage hardening,
  trusted anchor placement, and real chain/state integration remain open.
- [x] Add a combined chain-state file-store transition.
  `ProductionChainStateFileStores` locks both the lifecycle and spent-ledger
  stores, loads account and ledger state, verifies/consumes/records the spent
  slot, then persists the spent ledger before the account snapshot.  The test
  demonstrates stale local account replay is still rejected from the durable
  spent ledger.  Production storage hardening, trusted anchor placement, and
  deployment-specific chain/state integration remain open.
- [x] Add the blockchain deployment integration memo.
  `scripts/research/blockchain_deployment_integration_memo.py` generates
  `blockchain_deployment_integration_memo_n32_q12289.{json,md}`.  It records
  the node validation flow, wallet flow, spent-ledger binding, trusted restore
  anchor, reorg reveal-burn policy, storage atomicity expectations, and the
  remaining non-claims around the legacy `254 B` executable path and unreviewed
  production storage.
- [x] Add the constant-time and side-channel audit memo.
  `scripts/research/constant_time_side_channel_audit_memo.py` generates
  `constant_time_side_channel_audit_memo_n32_q12289.{json,md}`.  It classifies
  public parsing, account lifecycle state, file stores, logging/error reports,
  secret-side signing, and keygen/sampler paths; the keygen/security assumption
  surface now links to it while keeping constant-time production review open.
- [x] Add initial public-state serialization KATs for side-channel review.
  `production_reference.rs` now tests that lifecycle snapshots and spent-ledger
  records serialize public account/spent-state fields while excluding
  instrumented secret sentinels, secret-key labels, hidden-witness labels,
  randomizer preimage labels, active-pad labels, sampler-randomness labels, and
  masked-response material.  Logging/error-report negative tests remain open.
- [x] Add generated rejection-report negative logging KATs.
  `production_reference.rs` now checks that generated rejection reports,
  account-binding errors, and lifecycle errors expose public metadata without
  echoing instrumented secret sentinels, even when an attacker-controlled public
  signature payload contains the sentinel bytes outside the claimed challenge.
  Callback-originated error-string sanitization remains open for the secret-side
  production policy.
- [x] Add sanitized production callback error wrappers.
  `production_reference.rs` now exposes sanitized signer/verifier wrapper
  variants for account binding, lifecycle consume, spent-ledger consume, and
  combined chain-state file-store transitions.  These collapse
  callback-originated failures to fixed public reasons while preserving the
  existing research wrappers for diagnostics.  Secret-side timing, retry shape,
  and constant-time review remain open.
- [x] Add sanitized callback failure state-shape KATs.
  The sanitized callback tests now also assert that callback failure does not
  consume the account slot, advance verification snapshot sequence, write a
  spent-ledger record, or persist a chain-state transition.  This closes the
  public state-mutation part of fixed failure behavior; timing and retry-count
  analysis remain open.
- [x] Add sanitized callback wrapper invocation-shape KATs.
  `production_reference.rs` now checks that sanitized wrapper paths invoke the
  signer/verifier callback exactly once after public prechecks pass and zero
  times when public prechecks reject.  This closes wrapper-level retry shape;
  the internal callback implementation still needs timing/leakage review.
- [x] Add explicit callback-boundary deployment classification.
  `production_reference.rs` now exposes `PRODUCTION_CALLBACK_BOUNDARIES` and
  marks raw diagnostic wrappers as non-deployment while sanitized fixed-public
  error wrappers are deployment-facing.  A regression test verifies that only
  sanitized paths are classified as deployment-facing.
- [x] Add deployment callback-boundary gate.
  `require_deployment_callback_boundaries` rejects unknown callback names and
  raw diagnostic wrappers, and accepts the sanitized deployment-facing set.
  This gives deployment integration code a single guardrail against accidentally
  wiring diagnostic callback paths into production validation/signing.
- [x] Add canonical deployment callback-boundary profile.
  `PRODUCTION_DEPLOYMENT_REQUIRED_CALLBACK_BOUNDARIES` names the sanitized
  signer/verifier/lifecycle/spent-ledger/chain-state wrappers required for a
  deployment-facing integration.  `verify_default_deployment_callback_boundaries`
  and its regression test ensure the default profile remains complete and
  sanitized.
- [x] Add fail-closed production deployment preflight.
  `production_deployment_preflight` reports callback-boundary readiness,
  executable signature bytes, reviewed target bytes, account-link bytes, and
  remaining deployment blockers.  The current preflight intentionally fails
  closed because the executable path is still `254 B` rather than the reviewed
  `250 B` verifier and constant-time/storage/external reviews remain open.
- [x] Wire deployment preflight into generated readiness reports.
  `production_reference_gap_report.py` now exports a `deployment_preflight`
  section, and the keygen/security assumption surface imports its readiness bit
  and blocker list.  This makes the fail-closed deployment gate visible in the
  top-level review artifacts instead of only in Rust.
- [x] Surface deployment preflight in the human-readable assumption report.
  `keygen_security_assumption_surface.md` now prints the preflight present/ready
  bits and blocker list so reviewers can see the fail-closed deployment state
  without opening the JSON artifact.
- [x] Extend the reproducible attack review to the full assumption surface.
  `assumption_attack_review_matrix_n32_q12289.{json,md}` now maps every
  keygen/security assumption to its local executable evidence, memo-only
  obligations, residual risk, and next review step.  The first matrix had
  three executable-backed rows, two memo-only rows, and zero deployment-ready
  rows.
- [x] Convert the QROM budget memo into a checkable security-loss object.
  `qrom_security_loss_review_n32_q12289.{json,md}` checks the concrete
  transcript/challenge ledger: `162` transcript bits, `2^16` quantum
  challenge-frame queries, `2^33` classical challenge-frame queries, and a
  `129`-bit minimum deployment term.  The assumption matrix now reports four
  executable-backed rows, one memo-only row, and zero deployment-ready rows.
- [x] Convert the pad/sampler assumption into a checkable boundary object.
  `pad_sampler_boundary_review_n32_q12289.{json,md}` checks pad-frame length,
  counter encoding, rejection cutoff arithmetic, sampled-free-residue count,
  exact support formula, no representative-level retry loop, forbidden
  active-pad/opening conditioning, verifier order, and source KAT coverage.
  The assumption matrix now reports five executable-backed rows, zero
  memo-only rows, and zero deployment-ready rows.
- [ ] ACTIVE: externally review the QROM game hop, SHA-256 pad PRF assumption,
  and final production sampler/API boundary against the generated review
  objects.

## 4. Compact Verifier Soundness

- [ ] STALE: prove byte decoding implies the active product-block verifier
  equation.  Superseded by the full-response quotient verifier and
  committed-sigma byte verifier surfaces.
- [ ] STALE: prove inactive coordinates are either absent from the claim or
  secret-independent.
- [ ] STALE: connect the compact verifier output to the existing Lean
  `CompactHonestVerifierEquationMultiForkRound` extraction theorem.  The active
  extraction branch is the full-response/QROM committed-sigma fork theorem.
- [ ] STALE: instantiate the six-replay compact coverage theorem for the
  selected challenge profile.  The deployable challenge-action profile is now
  the hardened `d = 12` QROM/fork-generation ledger.

## 5. Distributional And Top-Level Theorem

- [ ] STALE: instantiate `LeakageBoundedCertificate` with
  `ConditionedGaussianLeakageN32.row0SignatureLeakage`.  This belongs to the
  old compact-active Gaussian sampler route, not the current quotient-fiber /
  committed-sigma route.
- [ ] STALE: prove deterministic byte serialization inherits that leakage
  bound.  Superseded by the exact quotient-fiber sampler and account-carried
  hidden-link proof obligations.
- [ ] STALE: compose hash binding, compact extraction, sampler leakage, retry
  leakage, and keygen/public-view hardness into the top-level target theorem.
  The current theorem composition is committed-sigma/account-carried soundness,
  QROM fork generation, response-link bounded kernel, pad PRF, and reviewed
  matrix/keygen assumptions.
- [x] Compose the quotient opening, active-equation, slot-consumption, byte-size,
  pad, hiding, matrix-agreement, and Fiat-Shamir review assumptions into the
  conditional stateful deployed-verifier theorem.
- [ ] ACTIVE: replace the fail-closed theorem target with an accepting
  hidden-source deployment theorem.  The theorem must start from the strict
  `284 B/Fq12` signature-carried envelope, reconstruct the canonical public
  hidden-source statement, verify the carrier semantics, and conclude
  extraction of an account witness/pad source or a named break.  This is the
  new north-star theorem for local deployability.
- [ ] ACTIVE: write the exact accepting relation before optimizing bytes.  The
  relation must specify which witness components open the account witness
  commitment and public pad-opening statement, which public fields are bound
  into the hidden-source statement digest, and how the verifier recomputes every
  digest from bytes.  No cached digest or callback-provided statement may be
  trusted as an oracle.
- [ ] ACTIVE: decide the carrier construction path.  Compare at least:
  32-byte hash/Fiat-Shamir proof object, slightly larger algebraic opening
  object, account-state-assisted proof object, and any pay-to-hash/stateful
  precommitment variant that keeps spend signatures small.  Record why the
  chosen path is sound, what assumptions it adds, and the precise byte cost.
- [ ] ACTIVE: update the proof budget after carrier semantics are chosen.
  Recompute challenge entropy, multi-query/QROM loss, proof-carrier binding
  loss, pad PRF loss, response-link extraction loss, matrix/keygen assumption
  margin, and any account-state replay/rollback terms.  The budget must be
  stated for blockchain-scale multi-query use, not a single-signature toy game.
- [ ] ACTIVE: connect accepting proof semantics to the Rust deployment facade.
  Replace the explicit proof-semantics blocker only after the proof carrier has
  real semantics, tests, negative controls, and theorem hooks.  The facade must
  still fail closed on malformed carrier bytes, transcript mismatch, state
  mismatch, rollback/replay, or any unimplemented obligation.
- [ ] ACTIVE: produce final reviewed benchmark numbers against state-of-the-art
  blockchain signature options, including implementation speed, verifier
  memory/state assumptions, and independently reviewed security estimates.
