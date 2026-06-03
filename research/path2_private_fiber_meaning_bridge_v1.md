# Path 2 Private Fiber Meaning Bridge V1

Status: bridge target for `PrivateFiberPredicates.meaningOfFiber`.

Purpose: identify what must instantiate the private fiber in the active native
Path 2 proof, and separate what is already checked by public assembly from what
still needs honest signer/keygen productization.

## Lean Interface

Lean now requires a non-tautological private meaning map:

```text
PrivateFiberPredicates.meaningOfFiber(q,h)
  = MeaningProjection.meaningOfBoundary(q)
```

for fiber-meaning consistency.

This prevents the proof from defining private fiber consistency as:

```text
meaning_of_boundary(q) = meaning_of_boundary(q)
```

## Concrete Private Fiber Shape

The intended private fiber over concrete `q` is:

```text
H_c(q) =
  LiftCore_c(q)
  × AuthorizationLocal_c(q)
  × Gauge_c(q)
  × Randomizer_c(q)
  × WalletRecoveryMetadata_c(q)
```

Planning byte range:

```text
signer private state: 128-416 bytes
```

from:

```text
authorization-local secret/opening material
gauge/local trivialization
randomizer/mask seed material
recovery/context metadata
```

These are signer-local. They are not consensus account state and are not part
of the 330-byte carrier.

## Public Meaning Boundary

For active native Path 2:

```text
meaningOfBoundary(q)
```

is reconstructed from:

```text
Path2NativeProjectionBundleV1
Path2NativeBoundaryAssemblyV1
```

as specified in:

```text
research/path2_projection_completeness_bridge_v1.md
```

The private fiber may produce proof bytes, commitments, masks, and local
authorization material, but it must not choose a different:

```text
account namespace
public key identity
spend authorization class
checked-core verifier handle
lifecycle/nullifier identity
replay domain
state transition class
aggregation mode
spend verifier domain
```

than the assembled public boundary.

## Current Public Anchors

The following Rust checks already constrain any submitted public/private pair
that reaches the public signer output:

- `assemble_path2_native_boundary_v1`
- `path2_native_spend_authorization_class_digest_v1`
- `path2_native_public_key_identity_digest_v1`
- `path2_native_checked_core_accept_v1`
- `path2_native_signer_pipeline_public_output_v1`

Important facts:

- public-key identity is derived from the checked-core verifier handle;
- spend authorization class is derived from the public spend tuple;
- proof commitment must match the checked-core frame;
- checked-core response syndrome must match the frame-derived challenge and
  verifier handle;
- signer public output is compatible with verifier preflight;
- public signer output is fail-closed until signer/verifier productization is
  complete.

## What Is Still Missing

No current Rust function is yet the final honest native constructor:

```text
HonestCreate(seed) = Some(q,h)
```

with a fully productized secret-sampling and proof-generation path.

The deployment preflight correctly marks:

```text
signer_verifier_pipeline_present = false
constant_time_audit_complete = false
```

Therefore `meaningOfFiber` cannot be marked fully instantiated yet.

## Required Honest Constructor Contract

A productized constructor must output or internally retain:

```text
q_preimage  : Path2NativeProjectionBundleV1
q_assembled : Path2NativeBoundaryAssemblyV1
h          : signer-private fiber state
```

with:

```text
q_assembled = assemble_path2_native_boundary_v1(q_preimage, proof_bytes)
```

and:

```text
meaningOfFiber(q,h) = meaningOfBoundary(q)
```

The bridge must show:

1. `AuthorizationLocal_c(q)` realizes exactly `p_spend(q)`;
2. `Gauge_c(q)` changes no public projection digest;
3. `Randomizer_c(q)` changes no public semantic class;
4. wallet recovery metadata changes no spend/lifecycle/state semantics except
   through public restore/reorg scope already in `p_lifecycle(q)`;
5. rejected signer paths sanitize errors and do not export private fiber
   material.

## Existing Evidence To Reuse

Design docs:

- `research/path2_authorization_local_classification.md`
- `research/path2_native_byte_state_model_v1.md`
- `research/path2_projection_completeness_bridge_v1.md`

Rust KAT anchors:

- `path2_native_honest_constructor_scope_pins_private_fiber_bridge`
- `path2_native_signer_pipeline_public_output_is_public_and_verifier_compatible`
- `path2_native_verifier_pipeline_is_public_only_and_fail_closed_until_productized`
- no-export KATs for active native public outputs
- lifecycle manager reserve/sign/consume/replay KATs
- sanitized callback/error KATs

New executable scope:

```text
Path2NativeHonestConstructorScopeV1
Path2NativeSignerKeygenProductizationScopeV1
```

pins:

- private fiber components;
- public boundary outputs of the constructor;
- meaning-invariance laws;
- required artifacts;
- required red-team/negative-control harnesses;
- explicit non-claims while the constructor is unproductized.

The signer/keygen productization scope pins the remaining deployability blocker
to concrete internal duties:

- production entropy streams for account keygen, authorization-local material,
  gauge/randomizer material, and wallet recovery material;
- single-owner secret-material flow with no debug/serialize surface;
- constructor contracts for proof commitment derivation, shared native-frame
  proof generation, public-output-only emission, and fail-closed deployment
  preflight;
- zeroization requirements for authorization, gauge, randomizer, recovery, and
  proof-randomizer material;
- harnesses for entropy domain separation, ownership/drop zeroization,
  commitment mismatch rejection, generated proof/assembly frame equality, and
  public-output no-export.

Draft executable constructor:

```text
Path2NativePrivateFiberStateV1
Path2NativeHonestConstructorSecretMaterialV1
Path2NativeSignerEntropyRootV1
Path2NativeBoundaryMeaningDigestV1
Path2NativeHonestConstructorDraftV1
Path2NativeEntropyConstructorOutputV1
path2_native_secret_material_from_entropy_root_v1
path2_native_honest_constructor_from_entropy_root_v1
path2_native_honest_constructor_from_os_entropy_v1
path2_native_honest_constructor_draft_v1
path2_native_honest_constructor_from_secret_material_v1
```

The draft constructor is not a productized signer/keygen implementation. It is
now an executable bridge for the theorem shape:

```text
private_fiber.authorization_class_digest
  =
bundle.spend.spend_authorization_class_digest
```

and:

```text
meaningOfFiber(q,h)
  =
path2_native_boundary_meaning_digest_v1(assembly(q))
```

where the public meaning digest consists of the assembled native boundary
digest and the account/spend/lifecycle/transcript/state projection digests.

The secret-material constructor adds one important bridge step beyond the first
draft relation:

```text
secret material -> private fiber -> checked-core proof bytes -> public boundary
```

The entropy-root derivation adds the next bridge step:

```text
owned entropy root + public q binding -> domain-separated secret material
```

Current executable checks:

- entropy-derived authorization/gauge/randomizer/recovery streams are
  domain-separated under `PATH2_NATIVE_SIGNER_ENTROPY_DERIVATION_DOMAIN`;
- entropy-derived proof-randomizer syndrome entries are canonical field
  elements;
- mutating the public account binding changes the derived secret material;
- entropy root buffers are explicitly zeroized;
- OS CSPRNG entropy root generation is wired through
  `path2_native_signer_entropy_root_from_os_v1`;
- q-syndrome derivation uses the fixed-attempt budget
  `PATH2_NATIVE_SIGNER_ENTROPY_Q_FIXED_ATTEMPTS = 4` and uses a deterministic
  fallback if no candidate is under threshold, so sampler exhaustion is not a
  signer rejection surface;
- q-syndrome reduction uses a 14-bit candidate and mask/subtract fallback
  reduction, avoiding runtime `% Q` or division in this signer sampler;
- the entropy-root constructor derives the proof commitment itself before
  generating checked-core proof bytes;
- `proof_randomizer_syndrome` must pack to the public
  `bundle.proof_commitment`;
- authorization class in the private fiber is exactly
  `bundle.spend.spend_authorization_class_digest`;
- checked-core proof bytes are generated from the same derived native frame
  used by `assemble_path2_native_boundary_v1`, via
  `path2_native_checked_core_proof_from_bundle_v1`;
- verifier preflight accepts the public boundary produced from the generated
  proof bytes;
- the signer execution report exposes only public accept/reject booleans,
  coarse rejection codes, public digests, and deployment preflight status;
- constructor and entropy failures are collapsed to public rejection codes and
  do not export root entropy, secret material, private fiber fields, or proof
  bytes;
- the signer constant-time stage report pins the fixed public schedule that
  production signing must satisfy without claiming that the current executable
  path is constant-time;
- the signer q-sampler source-shape audit pins the fixed loop, mask-select,
  no-runtime-modulo, no-exhaustion-error, and fixed digest-byte-load source
  properties while leaving backend timing open;
- the signer q-sampler release-assembly audit pins the current x86_64 optimized
  sampler body to four digest calls, no division instructions, no conditional
  branch instructions, conditional moves, and fixed digest byte loads;
- local seed buffers used by the derivation helper are zeroized before return;
- `Path2NativeHonestConstructorSecretMaterialV1` is not `Copy`, has no
  `Debug`/`Serialize` derivation, and explicitly zeroizes owned seed buffers
  plus proof-randomizer syndrome.

KATs:

- `path2_native_honest_constructor_draft_binds_private_fiber_to_public_q`
- `path2_native_honest_constructor_same_q_gauge_randomizer_invariant`
- `path2_native_honest_constructor_rejects_authorization_class_mismatch`
- `path2_native_checked_core_proof_generation_matches_verifier`
- `path2_native_checked_core_proof_generation_uses_assembly_frame_derivation`
- `path2_native_honest_constructor_from_secret_material_derives_private_fiber`
- `path2_native_entropy_root_derives_domain_separated_secret_material`
- `path2_native_entropy_root_binding_mutation_changes_secret_material`
- `path2_native_entropy_root_zeroize_clears_owned_buffers`
- `path2_native_os_entropy_root_is_nonzero_and_domain_separable`
- `path2_native_entropy_q_sampler_uses_fixed_attempt_budget`
- `path2_native_entropy_q_sampler_has_no_secret_dependent_failure_surface`
- `path2_native_entropy_q_sampler_uses_14_bit_reduction_without_runtime_modulo`
- `path2_native_entropy_constructor_derives_commitment_and_verifier_compatible_output`
- `path2_native_os_entropy_constructor_keeps_public_output_fail_closed`
- `path2_native_signer_execution_report_success_is_digest_only_and_fail_closed`
- `path2_native_signer_execution_report_rejection_is_sanitized_and_digest_only`
- `path2_native_signer_constant_time_stage_report_pins_fixed_public_shape`
- `path2_native_signer_ct_source_audit_pins_sampler_source_shape`
- `path2_native_secret_derivation_zeroizes_local_seed_buffer`
- `path2_native_secret_material_zeroize_clears_all_owned_buffers`
- `path2_native_secret_material_constructor_rejects_randomizer_mismatch`

Lean anchors:

- `PrivateFiberPredicates.meaningOfFiber`
- `FiberMeaningConsistent`
- `HonestCreationSoundness`
- `AuthorizationClassBinding`
- `FiberPrivacy`

## Red-Team Criterion

Before marking this bridge complete, try to construct:

```text
q, h, h'
```

such that:

```text
same q
LocalValid(q,h)
LocalValid(q,h')
meaningOfFiber(q,h) != meaningOfFiber(q,h')
```

or:

```text
AuthClass(h) != p_spend(q)
```

or:

```text
h changes lifecycle/replay/state transition semantics without changing q
```

Any success is a real design break:

```text
meaning_binding_break
authorization_class_binding_break
spend_state_closure_break
fiber_privacy_break
```

## Interim Conclusion

The public side already prevents many private-fiber meaning failures by forcing
the public spend class, public key identity, checked-core verifier handle,
lifecycle marker, and state class through `q`.

The remaining work is not consensus hidden-witness extraction. It is the honest
constructor/productized signer proof:

```text
the wallet-retained private state h is a valid realization of the public
boundary q and cannot alter public meaning inside the same fiber.
```

The blocker is now first-class in Rust as
`Path2NativeHonestConstructorScopeV1`, and the first draft constructor relation
has executable positive and negative controls. The second bridge step now also
derives private-fiber digests and checked-core proof bytes from explicit
`Path2NativeHonestConstructorSecretMaterialV1`, while rejecting proof
randomizer/public commitment mismatches.

This remains open until the draft relation is replaced by the active native
signer/keygen pipeline with production entropy separation, real secret
sampling/keygen ownership, full zeroization across all retained secret paths,
and constant-time coverage. Proof generation is no longer a purely prose gap in
the bridge, but it is still not a production signer claim.

The current bridge now enforces non-`Copy` owned secret material and executable
zeroization for the seed buffers and proof-randomizer syndrome. The remaining
zeroization gap is production integration coverage: proving every real
signer/keygen-owned buffer follows the same ownership/drop discipline and that
constant-time cleanup holds across rejection paths.

The current entropy-root bridge proves domain separation and public-binding
sensitivity for the derived streams. The bridge now also has an OS CSPRNG
entrypoint and a fixed-attempt q sampler, and the constructor derives the public
proof commitment from owned entropy rather than caller-supplied proof material.
The current signer execution report is digest-only and rejection-sanitized for
this constructor surface. The constant-time stage report now fixes the public
stage shape for success and rejection reports, and the q sampler no longer has
a secret-dependent exhaustion rejection or runtime modulo/division reduction in
this signer path. It explicitly keeps the constant-time audit open. It is still
not a full deployment claim: the whole signer/keygen path needs a constant-time
audit of compiler/runtime behavior, secret-dependent branching, memory access,
comparison/mask lowering, SHA-256 behavior, and rejection cleanup before the
deployment preflight can change. The q-sampler source-shape audit narrows that
claim to backend/assembly and timing evidence for the audited source window.
The release-assembly audit closes the current optimized sampler body shape, but
does not by itself prove microarchitectural constant-time behavior or cover
SHA-256, OS entropy, proof generation, or whole-signer cleanup.

The rejection-cleanup public-surface audit now centralizes signer failure
reports and checks success, constructor-failure, and entropy-failure reports
for digest-only/public-code behavior. This closes the current public-error
serialization gap for rejection cleanup. It deliberately does not claim
allocator/runtime zeroization or whole-signer timing behavior; those remain
constant-time/productization blockers.

The SHA-256 boundary audit now checks that signer entropy derivation has a
fixed public source boundary: framed domain, public labels, six-word public
binding, owned entropy words, counter framing, and fixed q-sampler digest
schedule. This removes the source-level secret-dependent SHA input-shape concern
from the bridge. It deliberately leaves the SHA-256 backend/library
constant-time claim open.

The OS entropy boundary audit now checks the signer root acquisition boundary:
two OS entropy calls, zero-initialized owned buffers, root-before-nonce order,
zeroization after either failure, no partial-entropy success, no raw backend
error detail export, and sanitized public entropy-failure reporting. It
deliberately does not claim OS RNG health, hardware entropy quality, or platform
availability.

The runtime-cleanup source audit now checks owned-root and owned-constructor
cleanup through success and early-error paths. The private-fiber constructor now
explicitly zeroizes owned constructor material before proof-commitment packing
or commitment-mismatch errors return. This closes the source-level cleanup
shape; allocator memory inspection and compiler dead-store-elimination
resistance remain open.

The signer-owned zeroization helpers now use volatile writes and compiler
fences. A release-assembly audit confirms the fixed-size byte scrub emits 32
zero stores, the fixed-size u16 scrub emits 12 zero stores, both carry compiler
fence markers, and neither lowers to external memory routines. This closes the
compiler dead-store-resistance item for these fixed-size owned buffers; target
allocator memory inspection remains open.

The release whole-signer timing probe now exists and currently fails as
deployment evidence: constructor rejection is much faster than success in the
reference path. The measured gap is public-input rejection timing, not a private
fiber leak by itself, but it proves the implementation does not yet satisfy the
fixed-stage signer schedule. Production closure needs either a constant-work
rejection wrapper or a prevalidated-public-input boundary that keeps malformed
public projection rejection outside the private signing schedule.

Newest executable blocker scope:

```text
Path2NativeSignerKeygenProductizationScopeV1
path2_native_signer_keygen_productization_scope_v1
Path2NativeSignerExecutionReportV1
path2_native_signer_execution_report_from_entropy_root_v1
path2_native_signer_execution_report_from_os_entropy_v1
Path2NativeSignerConstantTimeStageReportV1
path2_native_signer_constant_time_stage_report_from_reports_v1
Path2NativeSignerCtSourceAuditV1
path2_native_signer_ct_source_audit_v1
Path2NativeSignerRejectionCleanupAuditV1
path2_native_signer_rejection_cleanup_audit_v1
Path2NativeSignerSha256BoundaryAuditV1
path2_native_signer_sha256_boundary_audit_v1
Path2NativeSignerOsEntropyBoundaryAuditV1
path2_native_signer_os_entropy_boundary_audit_v1
Path2NativeSignerRuntimeCleanupAuditV1
path2_native_signer_runtime_cleanup_audit_v1
```

KAT:

```text
path2_native_signer_keygen_productization_scope_pins_secret_ownership_blocker
path2_native_signer_execution_report_success_is_digest_only_and_fail_closed
path2_native_signer_execution_report_rejection_is_sanitized_and_digest_only
path2_native_signer_constant_time_stage_report_pins_fixed_public_shape
path2_native_signer_ct_source_audit_pins_sampler_source_shape
path2_native_signer_rejection_cleanup_audit_pins_public_failure_surface
path2_native_signer_sha256_boundary_audit_pins_source_boundary
path2_native_os_entropy_boundary_audit_pins_sanitized_source_boundary
path2_native_runtime_cleanup_audit_pins_owned_source_cleanup
```
