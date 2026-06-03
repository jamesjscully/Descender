# Path 2 Native Consensus Carrier V1

Status: selected first consensus carrier for admitted native `q`.

Purpose: define the byte object validators/store nodes receive for an admitted
native boundary.

## Decision

Use a hybrid carrier:

```text
compact projection digest surface
+ data-availability digest for expanded projection preimages
```

Validators bind the compact surface on chain and require the expanded
projection preimages to be available through the named DA digest when replay or
preflight needs semantic field inspection.

This avoids carrying all expanded projection preimages in account creation
bytes, while avoiding a digest-only design with no availability rule.

## Domain

```text
Descender.Path2.NativeConsensusCarrier.v1
```

## Field Order

Canonical byte order:

```text
domain_tag
carrier_version_u16_le
native_boundary_version_u16_le
parameter_set_id_u16_le
verifier_family_id_u16_le
aggregation_mode_u8
public_lifecycle_marker_u8
native_boundary_digest[32]
p_context_digest[32]
p_account_digest[32]
p_spend_digest[32]
p_lifecycle_digest[32]
p_state_digest[32]
p_transcript_digest[32]
public_state_root[32]
obstruction_flags_digest[32]
projection_preimage_da_digest[32]
```

Body length, excluding domain:

```text
2 + 2 + 2 + 2 + 1 + 1 + 32*10 = 330 bytes
```

The domain is included in the canonical byte preimage for carrier digesting,
but the consensus-size budget tracks the fixed 330-byte body.

## Derivation Rule

The carrier is derived from:

```text
Path2NativeBoundaryAssemblyV1
projection_preimage_da_digest
```

and must not accept independently supplied projection digests.

Required equalities:

```text
carrier.native_boundary_digest = assembly.native_boundary_digest
carrier.p_context_digest       = assembly.p_context_digest
carrier.p_account_digest       = assembly.p_account_digest
carrier.p_spend_digest         = assembly.p_spend_digest
carrier.p_lifecycle_digest     = assembly.p_lifecycle_digest
carrier.p_state_digest         = assembly.p_state_digest
carrier.p_transcript_digest    = assembly.p_transcript_digest
```

## What The DA Digest Binds

`projection_preimage_da_digest` binds the canonical bytes needed to replay:

```text
Path2NativeContextProjectionV1
Path2NativeAccountProjectionV1
Path2NativeSpendProjectionV1
Path2NativeLifecycleProjectionV1
Path2NativeStateProjectionV1
Path2NativeTranscriptProjectionV1
```

It is a data-availability and replay handle, not a hidden-witness commitment.

## Not Included

The consensus carrier does not include:

- message digest;
- checked-core proof commitment;
- checked-core proof bytes or digest;
- private authorization-local material;
- pad seed or expanded pads;
- opening tuple witnesses;
- unmasked projection lanes;
- gauge/randomizer/blinding coordinates;
- fallback carrier bytes.

Spend-specific fields stay in the checked-core frame and spend public view.

## Validation

Reject if:

- carrier version is unsupported;
- native boundary version is unsupported;
- parameter set or verifier family is unsupported;
- aggregation mode is reserved;
- domain tag is wrong;
- decoded byte length is not exact.

## KAT Requirements

Reference KATs must cover:

- canonical field order and length;
- decode round trip;
- carrier digest equals `SHA256(carrier_bytes)`;
- mutation of any projection digest changes carrier digest;
- context/version/aggregation rejection;
- no private/fallback field names in JSON/debug output;
- no message/proof-specific checked-core fields in account-creation carrier.

## Implementation Status

The Rust reference implementation exposes:

```text
Path2NativeConsensusCarrierV1
Path2NativeConsensusCarrierPreflightReportV1
path2_native_consensus_carrier_v1_from_assembly
path2_native_consensus_carrier_v1_bytes
decode_path2_native_consensus_carrier_v1
path2_native_consensus_carrier_v1_digest
path2_native_consensus_carrier_preflight_report_v1
verify_path2_native_consensus_carrier_v1_against_bundle
```

The preflight helper recomputes `assemble_path2_native_boundary_v1` from the
available projection bundle and rejects unless the consensus carrier equals the
assembly-derived carrier for the supplied projection-preimage DA digest. KATs
cover successful recomputation plus carrier, bundle, and DA digest mismatch
rejection. The public preflight report contains only public digests,
acceptance status, and coarse rejection codes; KATs scan accepted and rejected
JSON/debug outputs for private fiber, pad, opening, gauge, fallback carrier,
raw proof, and decoded private-lane material.
