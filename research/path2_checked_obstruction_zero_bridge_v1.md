# Path 2 Checked Obstruction-Zero Bridge V1

Status: concrete bridge target for `CheckedObstructionZero`.

Purpose: map each Lean obstruction component predicate to concrete native Path
2 public checks. This prevents the canonical no-obstruction digest from being
treated as a self-attested proof of validity.

## Lean Interface

The active Lean condition is:

```text
CheckedObstructionZero(O,CObs,q)
```

meaning:

```text
ObstructionZero(O,q)
∧ CObs.contextChecked(q)
∧ CObs.pullbackChecked(q)
∧ CObs.gluingChecked(q)
∧ CObs.meaningChecked(q)
∧ CObs.spendStateChecked(q)
∧ CObs.canonicalityChecked(q)
```

## Concrete Acceptance Rule

The Rust obstruction digest check:

```text
bundle.obstruction_flags_digest
  == path2_native_no_obstruction_flags_digest()
```

is valid only inside the larger acceptance relation:

```text
assemble_path2_native_boundary_v1(bundle, checked_core_proof_bytes)
```

It is not a standalone proof that `obs_c(q)=0`.

## Component Mapping

### `contextChecked(q)`

Concrete checks:

- active context projection bytes are encodable;
- native boundary version, parameter set, verifier family, and aggregation mode
  satisfy `validate_path2_native_checked_core_frame_v1`;
- account-creation, spend-verifier, chain/application, and fork/epoch domains
  are bound into `Path2NativeCheckedCoreFrameV1`;
- active native domain/schema registry has no local collision.

Rust anchors:

- `path2_native_context_projection_v1_bytes`
- `validate_path2_native_context_fields`
- `validate_path2_native_checked_core_frame_v1`
- `path2_active_native_domains_and_public_schemas_are_unique`

Bridge status: partially instantiated.

Open item:

- [ ] state exactly which context values are admissible, not merely encodable.

### `pullbackChecked(q)`

Concrete checks:

```text
context.verifier_family_id == spend.spend_verifier_family_id
context.aggregation_mode == spend.aggregation_mode
context.spend_verifier_domain_digest == spend.spend_domain_separator_digest
account.public_key_identity == spend.public_key_identity
account.public_key_identity
  == path2_native_public_key_identity_digest_v1(
       spend.spend_verifier_family_id,
       spend.checked_core_verifier_handle_digest)
```

Rust anchor:

- `assemble_path2_native_boundary_v1`

Bridge status: mostly instantiated.

Open item:

- [ ] decide whether account/lifecycle and account/state edges need explicit
  additional equalities or are already covered by shared projection digests and
  lifecycle/state checks.

### `gluingChecked(q)`

Concrete checks:

```text
p_context_digest    = H(context projection bytes)
p_account_digest    = H(account projection bytes)
p_spend_digest      = H(spend projection bytes)
p_lifecycle_digest  = H(lifecycle projection bytes)
p_state_digest      = H(state projection bytes)
native_boundary_digest =
  H(p_context,p_account,p_spend,p_lifecycle,p_state)
p_transcript_digest = H(transcript projection bytes binding boundary)
checked_core_frame  = assembled from the same digests
```

Rust anchors:

- `path2_native_*_projection_v1_digest`
- `path2_native_boundary_v1_digest`
- `path2_native_transcript_projection_v1_digest`
- `assemble_path2_native_boundary_v1`

KAT anchors:

- `path2_native_projection_mutations_cascade_to_transcript_and_boundary`
- `path2_native_boundary_assembly_derives_one_consistent_public_surface`
- `path2_native_boundary_assembly_rejects_independent_digest_inputs`

Bridge status: instantiated up to hash-binding assumptions.

Open item:

- [ ] explicitly export `domain_binding_or_qrom_break` if distinct projection
  preimages collide under the same digest.

### `meaningChecked(q)`

Concrete checks:

Meaning is not a separate Rust field. It is reconstructed from projections:

```text
meaning_of_boundary(q)
  = account/spend/lifecycle/transcript/state/context projections
```

Rust anchors:

- projection encoders;
- public account view assembly;
- public spend view assembly;
- consensus carrier assembly.

Bridge status: depends on the projection-completeness bridge.

Open item:

- [ ] close `ProjectionCompleteness` before marking this component complete.

### `spendStateChecked(q)`

Concrete checks:

```text
spend.allowed_state_transition_class_digest
  == state.consumed_successor_state_class
lifecycle.live_status_marker
  == state.live_state_requirement
```

and checked-core frame binds:

```text
p_state_digest
p_lifecycle_digest
current_state_digest
message_digest
proof_commitment
```

Rust anchors:

- `assemble_path2_native_boundary_v1`
- `Path2NativeCheckedCoreFrameV1`
- `path2_native_checked_core_accept_v1`

Bridge status: partially instantiated.

Open item:

- [ ] lifecycle consume-once/storage semantics are outside compact boundary
  assembly and must remain a separate storage/lifecycle gate.

### `canonicalityChecked(q)`

Concrete checks:

- fixed active native projection encoders;
- fixed checked-core frame encoder;
- fixed public account/spend serializers;
- fixed consensus carrier bytes;
- registered domains and schemas;
- fixed checked-core proof byte length.

Rust anchors:

- `path2_native_projection_preimage_encoders_have_stable_layouts`
- `path2_native_checked_core_frame_v1_golden_vector_and_roundtrip`
- `public_native_account_view_v1_serializes_only_allowlisted_fields`
- `public_native_spend_view_v1_serializes_digest_only_proof_surface`
- `path2_active_native_domains_and_public_schemas_are_unique`

Bridge status: instantiated up to byte parser/hash assumptions.

Open item:

- [ ] add the raw-byte-vs-digest distinction from the ordered frame encoder
  bridge.

## Main Finding

`CheckedObstructionZero` is viable with the current design, but it must be
understood as a conjunction over the whole native assembly relation.

The canonical no-obstruction digest only says:

```text
the obstruction flag coordinate claims the zero class
```

The component predicates say:

```text
the boundary really was checked in every place where the obstruction class
could otherwise be forged, bypassed, or interpreted differently
```

## Current Blockers

1. `meaningChecked(q)` depends on the projection-completeness bridge.
2. `spendStateChecked(q)` depends on lifecycle consume-once/storage semantics
   outside compact boundary assembly.
3. `canonicalityChecked(q)` depends on the ordered encoder/hash-binding bridge.

These are not new design contradictions. They are the exact sub-obligations
that must be closed before claiming deployable proof-side obstruction
soundness.
