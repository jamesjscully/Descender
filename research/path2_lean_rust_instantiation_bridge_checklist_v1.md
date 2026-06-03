# Path 2 Lean/Rust Instantiation Bridge Checklist V1

Status: active bridge checklist after core algorithm red-team patches.

Purpose: make the abstract Lean hooks introduced by the core proof review
claim concrete byte-level witnesses before we treat the native Path 2 proof as
deployment-ready.

This checklist is intentionally narrower than the deployment preflight. It asks
only whether the proof-side theorem interfaces are actually instantiated by the
current Rust reference surface.

## Verdict

The core proof interface now names the right obligations, but deployable
proof-side closure still requires an explicit instantiation bridge.

Current split:

- Rust already realizes most public-boundary, digest, frame, and obstruction
  checks in `assemble_path2_native_boundary_v1`.
- Lean now refuses the five earlier tautological or underspecified proof hooks.
- The remaining high-risk work is proving that each Lean field is witnessed by
  the concrete Rust byte checks, not just by similarly named prose.

## Hook 1: `meaningOfFiber`

Lean hook:

```text
PrivateFiberPredicates.meaningOfFiber(q,h) = M.meaningOfBoundary(q)
```

Concrete witness target:

```text
honest keygen/signer private state h
  maps to the same account namespace, spend authorization class,
  lifecycle identity, transcript binding surface, and state boundary
  as the emitted native boundary q
```

Current Rust anchors:

- `path2_native_public_key_identity_digest_v1`
- `assemble_path2_native_boundary_v1`
- account/spend public-key identity equality check
- checked-core verifier handle binding check
- spend authorization-class digest derivation
- lifecycle/state live-marker equality check
- spend/state transition-class equality check

Gap:

No single Rust function is currently identified as the concrete
`meaningOfFiber` witness. The public assembler checks that a submitted
projection bundle is self-consistent, but it does not by itself prove that the
honest private signer state generated the bundle.

Required closure:

- [~] Name the honest private fiber type that instantiates `PrivateFiber`.
  Evidence:
  `research/path2_private_fiber_meaning_bridge_v1.md` defines the intended
  decomposition as `LiftCore_c × AuthorizationLocal_c × Gauge_c × Randomizer_c
  × WalletRecoveryMetadata_c`; Rust `Path2NativeHonestConstructorScopeV1`
  now pins the same decomposition as an executable scope.
- [~] Name the Rust constructor that emits `(q,h)` or equivalent public/private
  paired outputs.
  Progress:
  draft relation `path2_native_honest_constructor_draft_v1` now emits
  `Path2NativeHonestConstructorDraftV1`, pairing the assembled public boundary
  and signer public output with `Path2NativePrivateFiberStateV1`.
  `path2_native_honest_constructor_from_secret_material_v1` now derives that
  private fiber from explicit `Path2NativeHonestConstructorSecretMaterialV1`,
  generates checked-core proof bytes through
  `path2_native_checked_core_proof_from_bundle_v1`, using the same native frame
  derivation as `assemble_path2_native_boundary_v1`, and rejects proof
  randomizer/public commitment mismatches. This remains open because it is not
  yet the productized signer/keygen constructor with production entropy,
  secret ownership, and constant-time coverage.
  Productization scope:
  `Path2NativeSignerKeygenProductizationScopeV1` now pins the remaining
  signer/keygen blocker to entropy streams, secret ownership rules, constructor
  contracts, zeroization requirements, and harnesses.
  Entropy bridge:
  `Path2NativeSignerEntropyRootV1` and
  `path2_native_secret_material_from_entropy_root_v1` now derive
  domain-separated secret material from an owned root and public `q` binding,
  and `path2_native_honest_constructor_from_os_entropy_v1` wires OS CSPRNG
  entropy into an end-to-end constructor wrapper. q-syndrome derivation now uses
  a fixed-attempt sampler budget with deterministic fallback rather than a
  variable retry loop or sampler-exhaustion rejection, and the signer sampler
  avoids runtime modulo/division by using 14-bit candidates with mask/subtract
  fallback reduction. This still remains below a whole-signer constant-time
  audit claim.
  Signer execution report:
  `Path2NativeSignerExecutionReportV1` and
  `path2_native_signer_execution_report_from_entropy_root_v1` now expose only
  public accept/reject booleans, coarse rejection codes, public digests, and the
  fail-closed deployment preflight. Constructor and entropy failures are
  sanitized rather than serialized as internal errors.
  CT stage report:
  `Path2NativeSignerConstantTimeStageReportV1` now pins the fixed public stage
  schedule that the production signer/keygen must satisfy across success and
  rejection paths. It records success/rejection report digests and public-only
  rejection-code status while keeping `audit_complete = false`.
  CT source audit:
  `Path2NativeSignerCtSourceAuditV1` now pins the source shape of
  `path2_native_signer_entropy_q_v1`: fixed loop, deterministic fallback,
  mask-select source marker, no runtime modulo/division tokens, no sampler
  exhaustion error, and fixed digest byte loads. It keeps
  `backend_audit_complete = false`.
  Backend sampler audit:
  `scripts/research/path2_native_signer_q_sampler_backend_audit.sh` now builds
  release assembly, extracts the non-inlined signer q sampler, and verifies the
  optimized body has four digest calls, no division instructions, no
  conditional branches, conditional moves, and fixed digest byte loads.
  CT evidence ledger:
  `Path2NativeSignerCtEvidenceLedgerV1` now separates what is pinned from what
  remains open. It records source evidence and backend script pinning for the q
  sampler, SHA-256 selected backend, fixed private digest frames, in-tree
  private SHA-256 CT shape/workspace cleanup, zeroize helpers, production
  routing, and prevalidated timing probes, while explicitly leaving private
  SHA-256 target-CPU timing, whole-signer microarchitectural timing, and
  post-cleanup register/spill residual lifetime incomplete.
  Bundle runner:
  `scripts/research/path2_native_signer_ct_evidence_bundle.sh` now executes the
  individual CT/backend probes as one hash-addressed evidence bundle and emits
  `constant_time_claim_complete=0`; the ledger pins that bundle script but does
  not treat target-host execution as already complete. The same runner now
  supports `PATH2_NATIVE_SIGNER_CT_EVIDENCE_OUT_DIR` preserved-output mode, so a
  target-host run can archive the manifest, environment manifest, and each
  hashed child audit transcript. The target-host campaign wrapper ties the
  preserved bundle, environment, timing stability output, and register-scrub
  policy output together by hash; the promotion gate separately enforces
  timing iteration/trial floors and hash replay before emitting
  `release_promotion_claim_complete=0`.
- [~] Add a bridge statement that each private semantic component projects to
  the public component checked by `assemble_path2_native_boundary_v1`.
  Evidence:
  `research/path2_private_fiber_meaning_bridge_v1.md` scopes this to the
  productized honest constructor and lists the required component laws;
  `path2_native_honest_constructor_scope_pins_private_fiber_bridge` checks the
  required laws and harness names are present in the Rust scope. The secret
  material constructor now adds executable evidence for authorization-class
  binding, proof-randomizer commitment binding, generated checked-core proof
  compatibility, and local derivation seed zeroization.
  `path2_native_signer_keygen_productization_scope_pins_secret_ownership_blocker`
  records the remaining signer/keygen closure criteria as executable scope
  data.
- [x] Add mutation/KAT coverage showing private-only gauge/randomizer changes
  do not alter `meaningOfFiber`.
  Evidence:
  `path2_native_honest_constructor_same_q_gauge_randomizer_invariant`,
  `path2_native_checked_core_proof_generation_matches_verifier`,
  `path2_native_checked_core_proof_generation_uses_assembly_frame_derivation`,
  `path2_native_honest_constructor_from_secret_material_derives_private_fiber`,
  `path2_native_entropy_root_derives_domain_separated_secret_material`,
  `path2_native_entropy_root_binding_mutation_changes_secret_material`,
  `path2_native_entropy_root_zeroize_clears_owned_buffers`,
  `path2_native_os_entropy_root_is_nonzero_and_domain_separable`,
  `path2_native_entropy_q_sampler_uses_fixed_attempt_budget`,
  `path2_native_entropy_q_sampler_has_no_secret_dependent_failure_surface`,
  `path2_native_entropy_q_sampler_uses_14_bit_reduction_without_runtime_modulo`,
  `path2_native_entropy_constructor_derives_commitment_and_verifier_compatible_output`,
  `path2_native_os_entropy_constructor_keeps_public_output_fail_closed`,
  `path2_native_signer_execution_report_success_is_digest_only_and_fail_closed`,
  `path2_native_signer_execution_report_rejection_is_sanitized_and_digest_only`,
  `path2_native_signer_constant_time_stage_report_pins_fixed_public_shape`,
  `path2_native_signer_ct_source_audit_pins_sampler_source_shape`,
  `path2_native_secret_derivation_zeroizes_local_seed_buffer`,
  `path2_native_secret_material_zeroize_clears_all_owned_buffers`, and
  `path2_native_secret_material_constructor_rejects_randomizer_mismatch`.

Risk if skipped:

The proof would show public boundary consistency but not honest private
spendability of the retained account material.

## Hook 2: `ProjectionCompleteness`

Lean hook:

```text
Proj.pSpend(q)      = M.meaningOfBoundary(q).spendAuthorizationClass
Proj.pLifecycle(q)  = M.meaningOfBoundary(q).lifecycleIdentity
Proj.pTranscript(q) = M.meaningOfBoundary(q).transcriptBindingSurface
Proj.pState(q)      = M.meaningOfBoundary(q).stateTransitionBoundary
```

Concrete witness target:

Every consensus-facing semantic field is derived from canonical projection
bytes, and no verifier-facing field is independently supplied after projection
assembly.

Current Rust anchors:

- `path2_native_context_projection_v1_bytes`
- `path2_native_account_projection_v1_bytes`
- `path2_native_spend_projection_v1_bytes`
- `path2_native_lifecycle_projection_v1_bytes`
- `path2_native_state_projection_v1_bytes`
- `path2_native_transcript_projection_v1_bytes`
- `path2_native_boundary_v1_digest`
- `path2_native_projection_preimage_da_digest_v1`
- KATs:
  - `path2_native_projection_preimage_encoders_have_stable_layouts`
  - `path2_native_projection_mutations_cascade_to_transcript_and_boundary`
  - `path2_native_boundary_assembly_derives_one_consistent_public_surface`
  - `path2_native_boundary_assembly_rejects_independent_digest_inputs`

Gap:

Projection bytes and mutation tests exist, but the formal bridge should state
that the Rust projection bundle is the concrete representation of `q` and that
the public account/spend/carrier/preflight outputs are views of that same `q`.

Required closure:

- [x] Define the concrete `q` carrier as the canonical projection bundle plus
  derived boundary/transcript/frame fields.
  Evidence:
  `research/path2_projection_completeness_bridge_v1.md`.
- [x] State that public account view, public spend view, and consensus carrier
  contain only assembled fields derived from this concrete `q`.
  Evidence:
  `research/path2_projection_completeness_bridge_v1.md`.
- [x] Link each `ProjectionCompleteness` field to the exact Rust projection
  encoder and rejection check.
  Evidence:
  `research/path2_projection_completeness_bridge_v1.md`.

Risk if skipped:

An implementation could satisfy the Lean theorem with an abstract projection
map while the Rust verifier accepts a public field that bypasses that map.

## Hook 3: `CheckedCoreSpendReduction`

Lean hook:

```text
V.spendAccept(q,sig,msg,state)
  => exists evidence.
       checkedCoreAccepts(q,msg,state,evidence)
       and checked-core frame binds pSpend, pLifecycle,
       pTranscript, pState, msg, state, proofCommitment
```

Concrete witness target:

The checked-core proof challenge is computed from the assembled native frame,
and any accepted proof commitment/response is bound to that frame and verifier
handle.

Current Rust anchors:

- `Path2NativeCheckedCoreFrameV1`
- `path2_native_checked_core_frame_v1_challenge`
- `path2_native_checked_core_accept_v1`
- `reviewed_fq12_state_assisted_expected_response_syndrome`
- `reviewed_fq12_account_carried_response_syndrome`
- KATs:
  - `path2_native_checked_core_frame_v1_golden_vector_and_roundtrip`
  - `path2_native_checked_core_frame_v1_mutations_change_challenge`
  - `path2_native_checked_core_frame_v1_rejects_invalid_context`
  - `path2_native_checked_core_frame_v1_excludes_legacy_private_handles`
  - `path2_native_boundary_assembly_rejects_independent_digest_inputs`

Gap:

The Rust function is the likely concrete witness. The remaining proof bridge is
to state that the checked-core verifier predicate in Lean is exactly acceptance
by `path2_native_checked_core_accept_v1` over the frame assembled by
`assemble_path2_native_boundary_v1`.

Required closure:

- [x] Define `checkedCoreAccepts` as the Rust checked-core accept relation over
  serialized `Path2NativeCheckedCoreFrameV1`.
  Evidence:
  `research/path2_checked_core_spend_reduction_bridge_v1.md`.
- [x] State the evidence object is the decoded
  `ProductionReviewedFq12AccountCarriedProof`.
  Evidence:
  `research/path2_checked_core_spend_reduction_bridge_v1.md`.
- [x] Link proof-commitment timing to frame construction and mutation tests.
  Evidence:
  `research/path2_checked_core_spend_reduction_bridge_v1.md`.
- [x] Name the downstream checked-core equation soundness assumption exported
  if the Fq12 relation accepts an invalid response.
  Evidence:
  `research/path2_checked_core_spend_reduction_bridge_v1.md`.
- [x] Add a Lean selected-implementation source contract that decomposes
  `path2_native_checked_core_accept_v1` into parser/native-handle agreement,
  proof length and canonicality, decoded commitment, parsed response syndrome,
  frame bytes, verifier-handle binding, challenge derivation, and expected/actual
  syndrome equality.
  Evidence:
  `Descender/Path2CheckedCoreByteBridge.lean`,
  `SelectedRustCheckedCoreByteSource`.
- [ ] Instantiate `SelectedRustCheckedCoreByteSource` against the concrete Rust
  `Path2NativeCheckedCoreFrameV1`, `path2_native_checked_core_frame_v1_bytes`,
  `decode_reviewed_fq12_account_carried_proof`,
  `path2_native_checked_core_frame_v1_challenge`,
  `reviewed_fq12_state_assisted_expected_response_syndrome`, and
  `reviewed_fq12_account_carried_response_syndrome` functions.

Risk if skipped:

The final spend theorem could still be read as using an abstract spend
acceptance predicate rather than the actual native checked-core verifier.

## Hook 4: `OrderedFrameEncoder`

Lean hook:

```text
OrderedFrameEncoder.encode fields
```

with injective ordered-field semantics.

Concrete witness target:

Rust byte encoders are prefix/domain separated, fixed-width where needed, and
unambiguous enough that equality of encoded frames implies equality of ordered
fields, up to the named hash-binding assumption when only digests are compared.

Current Rust anchors:

- projection byte encoders
- `encode_path2_native_checked_core_frame_v1`
- `path2_native_projection_preimage_da_v1_bytes`
- domain constants registered in the production domain/schema registry
- KATs:
  - projection layout golden checks
  - checked-core frame golden vector and roundtrip
  - active native domain/schema uniqueness checks

Gap:

Lean models an injective ordered encoder directly. Rust has concrete byte
encoders and hash wrappers. The bridge must separate:

```text
byte encoder injectivity/canonicality
```

from:

```text
SHA-256/QROM digest binding
```

Required closure:

- [x] For each active native encoder, state whether the proof uses raw bytes or
  only digest equality.
  Evidence:
  `research/path2_ordered_encoder_hash_binding_bridge_v1.md`.
- [x] For raw bytes, cite fixed-width/domain/prefix parsing as canonical.
  Evidence:
  `research/path2_ordered_encoder_hash_binding_bridge_v1.md`.
- [x] For digest equality, export a named `hash_binding_or_qrom_break`.
  Evidence:
  `research/path2_ordered_encoder_hash_binding_bridge_v1.md`.
- [x] Add this distinction to the residual assumption ledger.
  Evidence:
  `research/path2_residual_assumption_ledger_v1.md`.
- [x] Add a Lean byte-domain instantiation contract that separates numeric
  active tags from concrete Rust domain bytes.
  Evidence:
  `Descender/Path2HashDomainAndMultiTarget.lean`,
  `ActiveHashByteDomainInstantiation`.
- [ ] Instantiate `ActiveHashByteDomainInstantiation` from the production domain
  registry, including `PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN` and the active
  hidden-source statement/proof/opening/pad/carrier/source-route domains, then
  connect those domain bytes to the actual hash inputs.

Risk if skipped:

The proof could silently treat hash digest equality as byte-level injectivity
without charging the collision/QROM assumption.

## Hook 5: `CheckedObstructionZero`

Lean hook:

```text
CheckedObstructionZero(O,CObs,q)
```

requiring the obstruction value plus component checks:

```text
contextChecked
pullbackChecked
gluingChecked
meaningChecked
spendStateChecked
canonicalityChecked
```

Concrete witness target:

The canonical no-obstruction digest is accepted only after all public component
checks have been performed.

Current Rust anchors:

- `path2_native_no_obstruction_flags_digest`
- `assemble_path2_native_boundary_v1`
- context validation through projection encoders and checked-core frame
  validation
- verifier family equality
- aggregation-mode equality
- spend-domain equality
- account/spend public-key identity equality
- checked-core verifier handle identity derivation
- spend/state transition-class equality
- lifecycle/state live-marker equality
- obstruction digest equality
- KAT `path2_native_boundary_assembly_rejects_independent_digest_inputs`

Gap:

The no-obstruction digest is checked after several component checks, but the
project has not yet named the concrete predicate corresponding to each
`ObstructionComponentPredicates` field.

Required closure:

- [~] Define `contextChecked` by exact context field validation and domain
  checks.
  Evidence:
  `research/path2_checked_obstruction_zero_bridge_v1.md`.
- [~] Define `pullbackChecked` by verifier family, aggregation, spend-domain,
  account/spend identity, and state/lifecycle edge equalities.
  Evidence:
  `research/path2_checked_obstruction_zero_bridge_v1.md`.
- [~] Define `gluingChecked` by boundary/transcript/frame assembly from derived
  projection digests.
  Evidence:
  `research/path2_checked_obstruction_zero_bridge_v1.md`.
- [~] Define `meaningChecked` by the `ProjectionCompleteness` bridge.
  Evidence:
  `research/path2_projection_completeness_bridge_v1.md`.
- [~] Define `spendStateChecked` by transition-class and live-marker checks
  plus lifecycle consume-once/storage semantics.
  Evidence:
  `research/path2_checked_obstruction_zero_bridge_v1.md`.
- [~] Define `canonicalityChecked` by canonical encoder/parser/registry checks.
  Evidence:
  `research/path2_ordered_encoder_hash_binding_bridge_v1.md` and KAT
  `path2_native_projection_preimage_da_frames_roundtrip`.

Risk if skipped:

The proof no longer has a self-attested obstruction flag, but implementation
reviewers could still miss which concrete checks justify each obstruction
component.

## Immediate Work Order

1. Close Hook 3 first: it is closest to current code and most directly affects
   accepted spend soundness.
2. Close Hook 5 next: it prevents the obstruction-zero condition from becoming
   a named but under-instantiated predicate.
3. Close Hook 2: it ties all public outputs to a single concrete `q`.
4. Close Hook 1: the draft/secret-material constructor now covers the bridge
   shape, proof generation, commitment mismatch rejection, and seed-buffer
   zeroization; `Path2NativeSignerKeygenProductizationScopeV1` pins the
   remaining production entropy, secret-ownership, full-zeroization, and
   constant-time evidence requirements. The entropy-root derivation now covers
   domain separation, public binding, OS CSPRNG root generation, proof
   commitment derivation, fixed-attempt q sampling with deterministic fallback,
   no-runtime-modulo signer q reduction, and a digest-only rejection-sanitized
   signer execution report. The CT stage report now pins the required fixed
   public schedule. Remaining work is a whole-signer
   SHA/timing/rejection-cleanup audit, not a missing constructor bridge,
   q-sampler source/backend-shape gap, or public-error serialization gap. The
   rejection-cleanup public-surface audit now checks success, constructor
   rejection, and entropy rejection reports for digest absence, public rejection
   codes, and absence of private-material sentinels while explicitly leaving
   allocator/runtime cleanup unclaimed. The SHA-256 boundary audit now removes
   source-level secret-dependent SHA input-shape risk from the signer entropy
   derivation by pinning framed public labels, a fixed six-word public binding,
   owned entropy words, counter framing, and a fixed q-sampler digest schedule.
   The SHA-256 call-schedule audit now pins the valid private path to 52
   entropy-derivation digest calls plus 4 secret-component digest calls;
   SHA-256 backend/library timing evidence remains open. The SHA-256
  selected-backend source audit now pins locked `sha2 0.10.9` with active
   `force-soft`, no active `sha2-asm`, runtime x86/SHA-NI dispatch not
   selected, and the Rust soft backend selected. The SHA-256 selected-backend
   release assembly audit now pins the compiled portable `compress256` object,
   absence of x86 `digest_blocks`, absence of `shani_cpuid` storage, absence of
   public CPU-feature dispatch, zero allocator/memory-routine calls in the
   audited compression body, zero SHA-NI instruction counts, and a public
   block-count guard. A dedicated soft-backend CT-shape audit pins no
   source-level `if`/`match`/`while`, only the public block-count loop branches
   in release assembly, zero calls/cmovs/setcc, zero variable shifts, and zero
   allocator/memory/SHA-NI/CPU-dispatch markers. The internal-state lifetime
   audit now pins the concrete remaining gap: upstream soft `sha2` owns local
   `block_u32` schedule state and `state_cpy`, exposes no explicit zeroization
   markers, and the compiled `compress256` frame reserves `0xa8` stack bytes
   with zero explicit zero stores before return; portable backend timing and
   SHA internal compression-state lifetime remain open. The OS entropy
   boundary audit now pins two OS entropy calls, zero-initialized owned buffers,
   zeroization after either failure, no partial-entropy success, no raw backend
   error detail export, and sanitized public entropy-failure reporting; OS RNG
   health and platform support remain external/backend productization evidence.
   The runtime-cleanup source audit now pins owned-root and owned-constructor
   source cleanup through success and early-error paths, including explicit
   zeroization before private-fiber proof-commitment packing or mismatch errors
   return. Signer-owned private digest frames are now fixed stack-backed frames,
   not heap `Vec` frames, and are explicitly zeroized after hashing. The
   signer-owned zeroization helpers now use volatile writes plus compiler
   fences, and the release-assembly zeroize audit confirms fixed-size
   seed/root/syndrome zero-store lowering plus the fixed 448-byte digest-frame
   zeroize loop with no external memory-routine calls. The private digest-frame
   constructor backend audit confirms fixed stack allocation, zero allocator
   calls, fixed-frame zeroization after hashing, and seed zeroization after
   secret-component hashing. Whole-execution register-spill/lifetime review and
   SHA-256 compression-internal state lifetime remain open. The
   release whole-signer
   timing probe is now wired and exposed a public rejection short path in the
   legacy direct signer-report path: constructor rejection is materially faster
   than success. The Rust implementation now includes a prevalidated
   public-input boundary that rejects malformed public projection/account input
   before entropy acquisition and private signing. The production-style OS
   entropy report entrypoint now routes through that wrapper, and a source-level
   production-routing audit pins this entrypoint shape while retaining the
   legacy direct path only for deterministic tests and negative controls. The
   remaining fixed-stage claim is narrowed to timing evidence for
   valid/prevalidated private signing cases.
5. Close Hook 4 throughout: it is mostly an assumption-ledger and encoder
   canonicality bridge.

## Deployable Proof-Side Exit Condition

The proof side can be called internally closed when every hook above has:

- a named Lean field or theorem interface;
- a named Rust relation/function instantiating it;
- KAT or mutation evidence for the public byte surface;
- a named cryptographic break for any remaining hash/equation assumption;
- no requirement that consensus extract or reveal hidden signer-local fiber
  material.
