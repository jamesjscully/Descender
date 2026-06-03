# Path 2 Native Rust / Lean Assembly Cross-Check V1

Status: proof/implementation cross-check note.

Purpose: map the active Rust reference KATs for native boundary assembly to the
Lean theorem interfaces, and name the remaining assumptions outside Lean's
arithmetic model.

## Objects Compared

Rust reference:

```text
Path2NativeProjectionBundleV1
Path2NativeBoundaryAssemblyV1
assemble_path2_native_boundary_v1
Path2NativeCheckedCoreFrameV1
PublicNativeAccountViewV1
PublicNativeSpendViewV1
```

Lean interfaces:

```text
NativeProjectionDigestFields
NativeBoundaryDigestAssembly
NativeTranscriptDigestAssembly
NativeAcyclicAssembly
NativeCheckedCoreFrameDigestBinding
NativePublicViewAssembly
```

Lean theorem hooks:

```text
native_boundary_digest_acyclic_eq
native_transcript_digest_binds_boundary_eq
native_acyclic_assembly_eqs
native_assembled_checked_core_frame_eq
native_public_views_assembled_eqs
```

## Assembly Order

The byte and theorem models now use the same non-recursive order:

```text
1. Encode/digest p_context(q)
2. Encode/digest p_account(q)
3. Encode/digest p_spend(q)
4. Encode/digest p_lifecycle(q)
5. Encode/digest p_state(q)
6. native_boundary_digest =
     H(boundary_domain,
       p_context_digest,
       p_account_digest,
       p_spend_digest,
       p_lifecycle_digest,
       p_state_digest)
7. Encode/digest p_transcript(q), binding native_boundary_digest
8. Build Path2NativeCheckedCoreFrameV1
9. Build PublicNativeAccountViewV1 and PublicNativeSpendViewV1
```

Critical invariant:

```text
native_boundary_digest does not include p_transcript_digest
```

because `p_transcript(q)` binds `native_boundary_digest`. Including both inside
each other would create a circular byte definition.

## KAT To Theorem Mapping

### Projection preimage KATs

Rust tests:

```text
path2_native_projection_preimage_encoders_have_stable_layouts
path2_native_projection_mutations_cascade_to_transcript_and_boundary
path2_native_projection_preimages_exclude_private_sentinels
```

Lean coverage:

```text
NativeProjectionDigestFields
NativeBoundaryDigestAssembly
NativeTranscriptDigestAssembly
native_boundary_digest_acyclic_eq
native_transcript_digest_binds_boundary_eq
native_acyclic_assembly_eqs
```

Meaning: Rust checks byte order, digest computation, mutation propagation, and
denylisted sentinel absence. Lean checks the abstract dependency graph: child
projection digests determine boundary digest; transcript digest binds boundary
digest afterward.

### Boundary assembly KATs

Rust tests:

```text
path2_native_boundary_assembly_derives_one_consistent_public_surface
path2_native_boundary_assembly_rejects_independent_digest_inputs
```

Lean coverage:

```text
NativeAcyclicAssembly
NativeCheckedCoreFrameDigestBinding
native_assembled_checked_core_frame_eq
```

Meaning: Rust checks that one bundle derives all emitted digests/views and that
mutating a projection changes boundary/frame/view consistently. Lean checks the
checked-core frame equation expands through assembled digest fields rather than
through independently supplied digest handles.

### Checked-core frame KATs

Rust tests:

```text
path2_native_checked_core_frame_v1_golden_vector_and_roundtrip
path2_native_checked_core_frame_v1_mutations_change_challenge
path2_native_checked_core_frame_v1_rejects_invalid_context
path2_native_checked_core_frame_v1_excludes_legacy_private_handles
```

Lean coverage:

```text
NativeCheckedCoreFrameFields
NativeCheckedCoreFrameBinding
native_checked_core_frame_eq
native_assembled_checked_core_frame_eq
```

Meaning: Rust checks exact bytes, context rejection, and challenge sensitivity.
Lean checks the checked-core frame is a deterministic function of the named
projection fields once those fields are bound to assembly.

### Public-view no-export KATs

Rust tests:

```text
public_native_account_view_v1_serializes_only_allowlisted_fields
public_native_spend_view_v1_serializes_digest_only_proof_surface
```

Lean coverage:

```text
PublicOutputPolicy
PublicOutputNoExport
public_output_no_export
NativePublicViewFields
NativePublicViewAssembly
native_public_views_assembled_eqs
```

Meaning: Rust checks actual serializer output and raw proof-byte exclusion.
Lean checks the public-view equations depend only on assembled projection
digests and public message/state/proof handles, not private fiber material.

## Outside-Lean Assumptions

Lean currently models digest fields as `Nat` equations. The following remain
implementation/cryptographic assumptions, not proven inside Lean:

### SHA-256 Binding / Collision Resistance

Required use:

```text
same digest means same canonical byte preimage
```

or else the adversary exports:

```text
qrom_or_domain_separation_break
hash_binding_break
```

### Canonical Byte Encoding

Required use:

```text
one semantic projection has one byte preimage
```

Rust KATs cover field order, little-endian integers, fixed-width digest fields,
domain tags, and context rejection. Lean does not parse bytes.

### Serializer Denylist Enforcement

Required use:

```text
public JSON/debug/API output contains only allowlisted fields
```

Rust KATs scan active native serializers for denylisted private/fallback names
and raw proof bytes. Lean only states the abstract `PublicOutputNoExport`
policy.

### Domain Separation

Required use:

```text
projection domains, boundary domain, transcript domain, checked-core frame
domain, and public-view schemas are disjoint
```

Rust KATs use distinct domain constants. The residual assumption ledger should
track SHA/domain separation as a named assumption until a broader domain
registry audit is run.

Implemented registry KAT:

```text
path2_active_native_domains_and_public_schemas_are_unique
```

This checks the active native projection domains, boundary domain,
checked-core frame domain, and public-native account/spend schemas for Path 2
namespace shape, explicit version suffixes, pairwise uniqueness, and no
domain/schema collision. This closes the local constant-collision risk for the
active native surface, while cryptographic domain separation still depends on
SHA-256/QROM assumptions.

## Current Closure Status

Closed at interface level:

- child projection digest dependency order;
- acyclic boundary/transcript assembly;
- checked-core frame binding to assembled digest fields;
- public-view binding to assembled public digest/proof handles.

Still outside Lean:

- SHA-256 collision resistance and random-oracle/QROM modeling;
- byte parser uniqueness and serde JSON behavior;
- denylist scanning completeness;
- constant-time behavior and deployment callback isolation.

## Next Engineering Gate

Add a compact domain/serializer registry check that verifies active native
domains and schemas are unique and that active public serializers are the only
validator-facing JSON/API surfaces for native Path 2.
