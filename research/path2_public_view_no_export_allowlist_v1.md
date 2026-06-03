# Path 2 Public View No-Export Allowlist V1

Status: first concrete public-output contract for the active native path.

Purpose: define what account creation, spend verification, error paths, JSON
serializers, and helper APIs may output without violating:

```text
FiberNoExportSoundness
```

for the active compact native account model.

## Scope

This contract applies to the active native path:

```text
q : Q_c
obs_c(q)=0
SpendAccept(c,q,sig,msg,state)
```

It does not bless old fallback/transparent proof serializers as active native
serializers. Fallback serializers remain useful negative controls and test
fixtures, but their public fields must be reclassified before reuse.

## Public Serializer Allowlist

The active native public view may serialize:

- `native_boundary_version`;
- `parameter_set_id`;
- `verifier_family_id`;
- `aggregation_mode`;
- canonical `q` bytes, if the mode stores expanded `q`;
- `native_boundary_digest(q)`;
- `p_context_digest(q)`;
- `p_account_digest(q)`;
- `p_spend_digest(q)`;
- `p_lifecycle_digest(q)`;
- `p_transcript_digest(q)`;
- `p_state_digest(q)`;
- public lifecycle marker;
- public account/state root handles;
- public obstruction flags/check summaries;
- `Path2NativeCheckedCoreFrameV1` digest;
- checked-core proof commitment;
- checked-core response/proof bytes, if bound to the native frame;
- spend message digest or canonical message bytes;
- current state root/digest;
- named break code;
- public field name and public byte offset for reject diagnostics.

## Private Denylist

The active native public view must never serialize:

- `AuthorizationLocal_c(q)`;
- private signing/opening witness;
- private trapdoor material;
- raw pad seed;
- raw pad source coordinates;
- pad PRF seed or expanded pads;
- `opening_tuple` witness scalars;
- raw opening commitment preimages;
- unmasked projection lanes;
- account projection lanes not explicitly part of public `q`;
- gauge/trivialization coordinates;
- randomizer/blinding/noise values;
- raw lift-core representative coordinates unless explicitly declared public in
  `q`;
- transparent hidden-view query openings;
- range/digit witnesses;
- reject-path expected private values;
- reject-path decoded private lanes;
- helper API outputs returning any value above.

## Conditional / Fallback-Only Fields

These fields are not active native semantic fields:

| field | active native disposition |
| --- | --- |
| `opening_commitment` | forbidden as semantic output; allowed only as private proof evidence or typed fallback restriction |
| `pad_seed_digest` | forbidden as semantic output; fallback metadata only unless digest of public projection policy |
| `hidden_source_statement_digest` | fallback proof metadata only |
| old `carrier_transcript_binding` | fallback carrier evidence only |
| old 56-byte projection carrier | not active native public view |
| checked-core fallback response transcript | proof evidence only, not native challenge frame |
| transparent query openings | allowed only if separately proven masked/simulated |

If any conditional field appears in active public output, the serializer must
also expose a proof that it is a byte binding for `q` or a projection digest.
Otherwise it is a no-export violation.

## Error Path Rule

Reject paths may output only:

```text
reject_code
named_break
public_field_name
public_byte_offset
public_expected_tag_or_length
public_actual_tag_or_length
```

Reject paths must not output:

```text
private expected value
private actual value
decoded hidden lane
partial opening
raw mask
seed material
```

Implementation rule:

```text
debug logs are public outputs unless explicitly compiled out and unavailable to
validators/users.
```

## Helper API Rule

Public helper APIs may return only:

```text
PublicNativeAccountViewV1
ProjectionDigest_i(q)
Path2NativeCheckedCoreFrameV1 or its digest
NamedBreak
VerifierResult
```

Signer-local helper APIs may return private fiber material only if:

- the API is not linked into verifier/validator public surfaces;
- the type name includes signer/private/local ownership;
- serializers for that type are disabled or explicitly private;
- tests assert it is absent from public outputs.

## Suggested Active Public View Type

```text
PublicNativeAccountViewV1 =
  (
    native_boundary_version,
    parameter_set_id,
    verifier_family_id,
    aggregation_mode,
    native_boundary_digest,
    p_context_digest,
    p_account_digest,
    p_spend_digest,
    p_lifecycle_digest,
    p_transcript_digest,
    p_state_digest,
    public_lifecycle_marker,
    public_state_root,
    obstruction_flags_digest_or_bits
  )
```

Spend public view:

```text
PublicNativeSpendViewV1 =
  (
    native_boundary_digest_or_account_handle,
    Path2NativeCheckedCoreFrameV1_digest,
    message_digest,
    current_state_digest,
    proof_commitment,
    checked_core_signature_or_proof_bytes
  )
```

## No-Export Theorem Target

```text
public_view_allowlist_no_export_v1:
  PublicOutputProduced(q,h,output)
  ∧ output conforms to PublicNativeAccountViewV1
    =>
  output contains no field in PrivateDenylist
```

and:

```text
public_view_simulation_v1:
  output conforms to the allowlist
    =>
  output is simulatable from q, public projections, msg, state, and proof bytes.
```

## Test/KAT Requirements

Before deployability, implement scans over accepted and rejected outputs for:

- raw pad seed bytes;
- pad source lanes;
- opening tuple bytes;
- projection lane bytes;
- randomizer/gauge bytes;
- hidden query openings;
- private helper type names in public JSON;
- forbidden conditional fields without a `byte_binding_for_q` proof.

Mutation tests:

- mutate every allowlisted digest and verify challenge/view changes or rejects;
- mutate every forbidden private field and verify it is absent from public view;
- force reject after decoding private material and verify reject output contains
  only public code/name/offset.

## Relation To Existing Serializers

Existing structs such as:

```text
Path2PublicAdmissionViewV1
Path2TransparentDescentPublicViewV1
```

are not automatically active-native compliant.

They contain useful public fields, but older fields like:

```text
opening_commitment
pad_seed_digest
hidden_source_statement_digest
slot_authorization_digest
```

must be reinterpreted or removed for active native output unless they are
specified byte bindings for projections of `q`.

## Current Verdict

The no-export surface is controllable if the active native implementation uses
a new narrow public-view serializer.

Do not reuse fallback/transparent public views as active native output without a
field-by-field wrapper proving each field is allowlisted.

## Next Concrete Move

Add a Lean structural allowlist/denylist interface:

```text
PublicOutputPolicy
PublicOutputNoExport
public_output_no_export
```

Implementation note:
`tools/parameter-search/src/production_reference.rs` now contains the first
typed active-native serializers:

```text
PublicNativeAccountViewV1
PublicNativeSpendViewV1
```

The account view is restricted to context IDs, aggregation mode, native
boundary/projection digests, lifecycle marker, public state root, and
obstruction digest. The spend view is restricted to native boundary digest,
checked-core frame digest, message/state digests, proof commitment, and a
digest/length pair for checked-core proof bytes. KATs assert allowlisted field
presence, private/fallback denylist absence, raw proof byte exclusion, mutation
sensitivity for proof bytes, and invalid-context rejection.

The next blocker is now projection preimage encoding: define and test the
canonical byte encoders that produce each `p_*_digest(q)` from the native
boundary object without depending on private representative/gauge material.
