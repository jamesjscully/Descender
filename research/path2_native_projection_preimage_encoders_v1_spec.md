# Path 2 Native Projection Preimage Encoders V1

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting byte-encoder spec note, not a normative spec. Current
> normative authority starts at `../spec/README.md`.

Status: implementation-facing byte specification.

Purpose: define the canonical public byte preimages for:

```text
p_context_digest(q)
p_account_digest(q)
p_spend_digest(q)
p_lifecycle_digest(q)
p_state_digest(q)
p_transcript_digest(q)
```

These encoders are projections of one native boundary object `q`. They are not
independent commitments and must not take private representative, gauge,
opening, pad, carrier, or witness material as inputs.

## Digest Rule

Every projection digest is:

```text
SHA256(domain_tag || canonical_projection_bytes)
```

where `canonical_projection_bytes` uses fixed little-endian integers and
fixed-width 32-byte digest fields.

All domain tags are ASCII and included directly as bytes, with no length prefix.
Each projection has a distinct domain tag.

## Context Projection

Domain:

```text
Descender.Path2.NativeProjection.Context.v1
```

Fields:

```text
native_boundary_version_u16_le
parameter_set_id_u16_le
verifier_family_id_u16_le
account_creation_domain_digest[32]
spend_verifier_domain_digest[32]
chain_or_application_domain_digest[32]
fork_or_epoch_scope_digest[32]
aggregation_mode_u8
```

Validity:

- version must be the active native boundary version;
- parameter set must be the q12289 native Path 2 set;
- verifier family must be the Fq12 checked-core family;
- aggregation mode must be direct or aggregate-compatible.

## Account Projection

Domain:

```text
Descender.Path2.NativeProjection.Account.v1
```

Fields:

```text
account_id[32]
account_namespace_digest[32]
public_key_identity[32]
creation_namespace_digest[32]
admission_state_namespace_digest[32]
```

Meaning: consensus-visible account namespace and public key/account identity.
No opening, pad, hidden-source statement, randomizer, or carrier field is
allowed here.

## Spend Projection

Domain:

```text
Descender.Path2.NativeProjection.Spend.v1
```

Fields:

```text
spend_verifier_family_id_u16_le
spend_authorization_class_digest[32]
checked_core_verifier_handle_digest[32]
public_key_identity[32]
allowed_authorization_local_mode_u8
allowed_state_transition_class_digest[32]
aggregation_mode_u8
spend_domain_separator_digest[32]
```

Meaning: public authorization class consumed by spend verification.
Compatibility aliases such as `slot_authorization_digest` are valid only if
they are derived from this projection or from the transcript projection that
binds this projection.

## Lifecycle Projection

Domain:

```text
Descender.Path2.NativeProjection.Lifecycle.v1
```

Fields:

```text
slot_id_u64_le
live_status_marker_u8
lifecycle_anchor[32]
nullifier_namespace_digest[32]
consume_once_rule_digest[32]
replay_domain_digest[32]
restore_or_reorg_scope_digest[32]
```

Meaning: replay, nullifier, restore, and consume-once behavior. Any private
material that can change lifecycle behavior must either be moved into this
projection as public meaning or treated as a safety-saturation break.

## State Projection

Domain:

```text
Descender.Path2.NativeProjection.State.v1
```

Fields:

```text
account_state_root_before[32]
admitted_boundary_id[32]
live_state_requirement_u8
consumed_successor_state_class[32]
data_availability_requirement[32]
aggregation_state_boundary[32]
```

Meaning: canonical state-transition class source. The state-transition class is
the canonical digest/class of this projection plus `p_context(q)`.

## Transcript Projection

Domain:

```text
Descender.Path2.NativeProjection.Transcript.v1
```

Fields:

```text
native_boundary_digest[32]
p_context_digest[32]
p_account_digest[32]
p_spend_digest[32]
p_lifecycle_digest[32]
p_state_digest[32]
checked_core_frame_domain_digest[32]
aggregation_mode_u8
```

Meaning: public challenge-binding surface for spend and residual account
certificate logic. It must bind all other semantic projection digests and the
checked-core frame namespace.

## Boundary Digest

The native boundary digest remains:

```text
SHA256(
  "Descender.Path2.NativeBoundary.v1"
  || p_context_digest
  || p_account_digest
  || p_spend_digest
  || p_lifecycle_digest
  || p_state_digest
)
```

This is the compact public handle for the semantic boundary class.

Important acyclicity rule:

```text
native_boundary_digest must not include p_transcript_digest
```

because `p_transcript(q)` binds `native_boundary_digest`. Including each inside
the other would create a recursive byte definition rather than a canonical
preimage. The assembly order is:

```text
child projection digests
  -> native_boundary_digest
  -> p_transcript_digest
  -> checked-core frame / public views
```

## No-Export Rule

The projection encoders must not accept these inputs:

```text
AuthorizationLocal_c(q)
private witness/opening material
pad seed or expanded pads
opening tuple witness scalars
unmasked projection lanes
gauge/trivialization coordinates
randomizer/blinding/noise values
transparent query openings
fallback carrier bytes
```

If an implementation needs one of those values to compute a projection digest,
the projection is not native-boundary compliant.

## KAT Requirements

For each projection:

- fixed golden vector with patterned fields;
- total length and offset checks;
- digest equals `SHA256(domain || bytes)`;
- every public field mutation changes the digest;
- invalid context fields reject where context appears;
- denylisted private sentinel bytes do not appear in bytes or JSON/debug views.

Cross-projection KATs:

- transcript digest changes when any child projection digest changes;
- native boundary digest changes when any child projection digest changes;
- checked-core frame built from these digests changes when any child projection
  changes;
- changing signer-local private/gauge sentinel variables has no effect because
  they are not inputs.

## Implementation Status

The Rust reference implementation now exposes:

```text
Path2NativeContextProjectionV1
Path2NativeAccountProjectionV1
Path2NativeSpendProjectionV1
Path2NativeLifecycleProjectionV1
Path2NativeStateProjectionV1
Path2NativeTranscriptProjectionV1
```

with canonical byte encoders, digest helpers, and
`path2_native_boundary_v1_digest`. Tests cover fixed byte layouts, digest
equality to `SHA256(bytes)`, transcript/boundary mutation cascade, invalid
context rejection, spend verifier-family rejection, and private sentinel
absence.

The reference implementation now includes `Path2NativeProjectionBundleV1`,
`Path2NativeBoundaryAssemblyV1`, and `assemble_path2_native_boundary_v1`.
Assembly derives the projection digests, acyclic boundary digest, transcript
digest, checked-core frame, and active native public views from one projection
bundle. KATs reject inconsistent public-key identity or aggregation mode and
prove projection mutation cascades through the boundary, frame, and spend view.
