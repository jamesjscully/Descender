# Path 2 Native Checked-Core Frame V1 Spec

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting byte-frame spec note, not a normative spec. Current
> normative authority starts at `../spec/README.md` and `../spec/wire_formats.md`.

Status: first concrete byte-frame specification.

Purpose: define the exact native checked-core challenge frame required by:

```text
checked_core_frame_binding_v1
```

and by the byte audit:

```text
research/path2_checked_core_frame_byte_audit_v1.md
```

## Frame Name

```text
Path2NativeCheckedCoreFrameV1
```

Domain tag:

```text
"Descender.Path2.NativeCheckedCoreFrame.v1"
```

This frame is the canonical challenge preimage for native Path 2 checked-core
spend verification, or a domain-separated subframe hashed into the legacy
checked-core challenge preimage.

## Field Order

Canonical byte order:

```text
domain_tag
native_boundary_version_u16_le
parameter_set_id_u16_le
verifier_family_id_u16_le
account_creation_domain_digest[32]
spend_verifier_domain_digest[32]
chain_or_application_domain_digest[32]
fork_or_epoch_scope_digest[32]
aggregation_mode_u8
native_boundary_digest[32]
p_account_digest[32]
p_spend_digest[32]
p_lifecycle_digest[32]
p_transcript_digest[32]
p_state_digest[32]
message_digest[32]
current_state_digest[32]
proof_commitment[32]
```

Total fixed digest/mode body, excluding variable-length ASCII domain tag:

```text
2 + 2 + 2 + 32*13 + 1 = 423 bytes
```

including:

```text
13 digest fields
```

The frame is a challenge preimage. It is not necessarily stored in the
signature.

## Field Semantics

### Version / Parameter / Verifier

```text
native_boundary_version
parameter_set_id
verifier_family_id
```

must equal fields in:

```text
p_context(q)
```

and satisfy:

```text
O_context(q)=0
```

### Domains

```text
account_creation_domain_digest
spend_verifier_domain_digest
chain_or_application_domain_digest
fork_or_epoch_scope_digest
```

are canonical domain bindings for the corresponding fields of `p_context(q)`.

They may be stored as direct 32-byte handles or computed from expanded domain
strings/tuples.

### Aggregation Mode

```text
aggregation_mode_u8
```

must equal:

```text
p_context(q).aggregation_mode
p_spend(q).aggregation_mode
p_transcript(q).aggregation_mode
```

and must satisfy:

```text
AggregationModeValid(q)
```

### Projection Digests

The projection digests bind:

```text
p_account(q)
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
```

`p_spend_digest` includes or references:

```text
allowed_state_transition_class = StateTransitionClass_c(q)
aggregation_mode
```

`p_state_digest` binds the canonical state transition boundary plus the context
fields listed in `path2_context_state_domain_binding_v1.md`.

### Message / State / Commitment

```text
message_digest
```

is the digest of the canonical spend message.

```text
current_state_digest
```

is the digest/root of the current consensus state input used by spend
verification.

```text
proof_commitment
```

is the checked-core commitment fixed before challenge derivation.

## Allowed Legacy Wrapper

If the existing checked-core implementation requires a legacy prefix, the only
allowed wrapper is:

```text
legacy_checked_core_prefix
|| H_domain(Path2NativeCheckedCoreFrameV1)
|| legacy_response_or_relation_metadata
```

with a theorem:

```text
legacy prefix and metadata do not select account/spend/state semantics.
```

The fallback helper:

```text
encode_hidden_source_fallback_checked_core_transcript_frame(response)
```

does not satisfy this spec by itself.

## Disallowed Fields

The native frame must not include any of these as semantic inputs:

```text
opening_commitment
pad_seed_digest
hidden_source_statement_digest
carrier_transcript_binding
old 56-byte carrier
unchecked slot_authorization_digest
```

`slot_authorization_digest` may appear only if it is exactly:

```text
H_domain(Path2NativeCheckedCoreFrameV1 without proof_commitment?)
```

or another specified digest of native projections. It must not be an independent
semantic handle.

## Mutation Tests

A correct implementation must reject or change challenge when mutating any of:

- domain tag;
- native boundary version;
- parameter set id;
- verifier family id;
- account creation domain digest;
- spend verifier domain digest;
- chain/application domain digest;
- fork/epoch scope digest;
- aggregation mode;
- native boundary digest;
- `p_account_digest`;
- `p_spend_digest`;
- `p_lifecycle_digest`;
- `p_transcript_digest`;
- `p_state_digest`;
- message digest;
- current state digest;
- proof commitment.

Additional negative tests:

- reserved aggregation mode rejects;
- unknown version rejects;
- verifier family not valid for parameter set rejects;
- `p_spend_digest` for one `q` with `p_state_digest` from another `q` rejects or
  changes challenge;
- legacy `opening_commitment`/`pad_seed_digest` mutation has no semantic effect
  unless it changes an allowed proof-evidence field.

## Theorem Target

```text
native_checked_core_frame_v1_sound:
  O_context(q)=0
  ∧ O_pullback(q)=0
  ∧ O_gluing(q)=0
  ∧ O_canonicality(q)=0
    =>
  encode_path2_native_checked_core_frame_v1(q,msg,state,commitment)
    =
  CanonicalEncode(
    p_context(q),
    p_account(q),
    p_spend(q),
    p_lifecycle(q),
    p_transcript(q),
    p_state(q),
    msg,
    state,
    commitment
  )
```

Any accepted spend whose checked-core challenge is not derived from this frame
exports:

```text
checked_core_frame_binding_break
```

or:

```text
domain_binding_or_qrom_break
```

## Byte / State Impact

The frame is not stored as account state. Validators reconstruct it from state,
transaction message, and signature/proof commitment.

Expected signature impact:

```text
0 bytes
```

if `native_boundary_digest` and projection digests are available from account
state.

Possible impact:

```text
+32 bytes
```

if the transaction must carry `native_boundary_digest` or an equivalent account
authorization handle explicitly.

## Next Concrete Move

Add a Lean structural interface:

```text
NativeCheckedCoreFrameFields
NativeCheckedCoreFrameBinding
native_checked_core_frame_eq
```

parallel to the existing skeletal `CheckedCoreFrameBinding`, but with explicit
field components matching this spec.

Implementation can wait; the proof side needs the exact field contract first.
