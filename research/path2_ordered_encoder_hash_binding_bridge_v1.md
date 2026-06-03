# Path 2 Ordered Encoder / Hash-Binding Bridge V1

Status: concrete bridge target for `OrderedFrameEncoder`.

Purpose: separate byte-level canonicality from cryptographic digest binding so
the Lean injective ordered encoder is not accidentally interpreted as
collision-free SHA-256.

## Lean Interface

Lean uses:

```text
OrderedFrameEncoder.encode : List Nat -> Nat
```

with:

```text
encode lhs = encode rhs -> lhs = rhs
```

This is an ordered-frame abstraction. It is valid for concrete Rust only in
one of two modes:

1. raw canonical bytes are compared or decoded directly; or
2. digest equality is used and the proof exports a hash/QROM binding break if
   two distinct byte preimages collide.

## Concrete Encoder Classes

### Fixed-Width Domain Encoders

These encoders are raw-byte canonical because their fields have fixed widths
after a fixed domain prefix:

```text
path2_native_context_projection_v1_bytes
path2_native_account_projection_v1_bytes
path2_native_spend_projection_v1_bytes
path2_native_lifecycle_projection_v1_bytes
path2_native_state_projection_v1_bytes
path2_native_transcript_projection_v1_bytes
path2_native_checked_core_frame_v1_bytes
path2_native_consensus_carrier_v1_bytes
```

Evidence:

- fixed domain prefixes;
- fixed-width `u16`, `u64`, `u8`, and `[u8; DIGEST_BYTES]` fields;
- checked-core frame decoder roundtrip;
- consensus carrier decoder roundtrip;
- layout KATs for projections and checked-core frame.

Bridge rule:

```text
raw_bytes_a = raw_bytes_b
  =>
same ordered fields
```

provided the relevant parser/decoder accepts the bytes.

### Length-Prefixed Data-Availability Frame

The projection preimage DA encoder uses:

```text
append_frame(label, data)
  = u16(label_len) || label || u64(data_len) || data
```

inside:

```text
path2_native_projection_preimage_da_v1_bytes
```

This is intended to be canonical for variable-length projection preimages.

Bridge rule:

```text
raw DA bytes equality
  =>
same ordered labels and same ordered data bytes
```

Open item:

- [x] add or cite a DA-frame parser/roundtrip KAT for `append_frame` sequences.
  Evidence:
  `path2_native_projection_preimage_da_frames_roundtrip` parses the
  length-prefixed DA frame sequence, checks ordered labels/data against the
  canonical projection bundle, and verifies the DA digest equals SHA-256 of the
  canonical bytes.

### Serde JSON Public Views

These are public reporting surfaces, not primary proof encoders:

```text
public_native_account_view_v1_json
public_native_spend_view_v1_json
```

Bridge rule:

Serializer KATs can support no-export and API stability, but proof binding must
rely on the typed Rust view object and digest/frame encoders, not JSON parser
injectivity.

## Digest-Bound Surfaces

When only these digests are compared:

```text
p_context_digest
p_account_digest
p_spend_digest
p_lifecycle_digest
p_state_digest
p_transcript_digest
native_boundary_digest
checked_core_frame_digest
checked_core_proof_bytes_digest
projection_preimage_da_digest
consensus_carrier_digest
public_key_identity
spend_authorization_class_digest
obstruction_flags_digest
```

the proof must not use byte injectivity directly. It must use:

```text
same digest means same canonical preimage
```

or export:

```text
hash_binding_break
domain_binding_or_qrom_break
byte_data_availability_break
```

depending on whether the failure is collision, domain confusion, or missing
expanded preimage availability.

## Mapping To Lean `OrderedFrameEncoder`

For Lean frame equations, instantiate `OrderedFrameEncoder` with the ordered
field list before hashing:

```text
encode_context_projection_fields
encode_account_projection_fields
encode_spend_projection_fields
encode_lifecycle_projection_fields
encode_state_projection_fields
encode_transcript_projection_fields
encode_checked_core_frame_fields
encode_consensus_carrier_fields
```

Then add a separate bridge theorem:

```text
digest(encode(fields_a)) = digest(encode(fields_b))
  =>
fields_a = fields_b
```

or:

```text
hash_binding_break
```

This keeps Lean’s ordered encoder honest: it models canonical field order, not
the cryptographic hash assumption itself.

## Current Evidence

Rust KAT anchors:

- `path2_native_projection_preimage_encoders_have_stable_layouts`
- `path2_native_projection_mutations_cascade_to_transcript_and_boundary`
- `path2_native_checked_core_frame_v1_golden_vector_and_roundtrip`
- `path2_native_checked_core_frame_v1_mutations_change_challenge`
- `path2_native_boundary_assembly_derives_one_consistent_public_surface`
- carrier byte/roundtrip/preflight KATs
- active native domain/schema uniqueness KATs

Lean anchors:

- `OrderedFrameEncoder`
- `ordered_frame_encode_injective`
- `CheckedCoreSpendReduction.frameEncoder`
- `CheckedCoreFrameBinding.frameEncoder`
- `NativeBoundaryDigestAssembly.boundaryEncoder`
- `NativeTranscriptDigestAssembly.transcriptEncoder`
- `NativePublicViewAssembly` encoders

## Required Closure

- [ ] Add a proof note or KAT that all fixed-width active native byte encoders
  are parser-injective on accepted bytes.

- [x] Add a DA-frame parser/roundtrip KAT for `append_frame` sequences used by
  `path2_native_projection_preimage_da_v1_bytes`.
  Evidence:
  `path2_native_projection_preimage_da_frames_roundtrip`.

- [ ] For every theorem step using digest equality, state the named exported
  break:

```text
hash_binding_break
domain_binding_or_qrom_break
byte_data_availability_break
```

- [ ] Update the bridge checklist to mark which surfaces are raw-byte
  canonical and which are digest-bound.

## Interim Conclusion

No redesign is needed here. The current Rust encoders are compatible with the
Lean ordered-frame abstraction if the proof treats SHA-256 as a separate
binding/QROM assumption rather than as part of `OrderedFrameEncoder`
injectivity.

The concrete DA parser/KAT gap is now closed. Remaining work is documentation
discipline plus keeping digest assumptions named at each theorem boundary.
