# Path 2 Native Checked-Core Frame KAT Plan V1

Status: implementation-facing KAT plan for `Path2NativeCheckedCoreFrameV1`.

Purpose: define tests that connect the native checked-core frame spec to
concrete encoder behavior.

Spec source:

```text
research/path2_native_checked_core_frame_v1_spec.md
```

## Encoder Under Test

Target function:

```text
encode_path2_native_checked_core_frame_v1(input) -> bytes
```

where `input` contains:

```text
native_boundary_version
parameter_set_id
verifier_family_id
account_creation_domain_digest
spend_verifier_domain_digest
chain_or_application_domain_digest
fork_or_epoch_scope_digest
aggregation_mode
native_boundary_digest
p_account_digest
p_spend_digest
p_lifecycle_digest
p_transcript_digest
p_state_digest
message_digest
current_state_digest
proof_commitment
```

## Golden Vector Test

Create one deterministic test vector with small patterned bytes:

```text
version = 1
parameter_set_id = 1
verifier_family_id = 1
aggregation_mode = 0
digest_i = repeated byte i
proof_commitment = repeated byte 0x51
```

Assert:

- frame starts with ASCII domain tag;
- `u16` fields are little-endian;
- aggregation mode is one byte;
- digest fields appear in the exact spec order;
- total length equals:

  ```text
  len(domain_tag) + 423
  ```

## Decode / Round Trip

If a decoder exists:

```text
decode(encode(input)) = input
```

If no decoder exists, add offset assertions for every field.

## Mutation KATs

For each field, mutate one byte and assert the challenge digest changes:

```text
challenge = H_domain(frame)
```

Fields:

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

## Rejection KATs

Reject or fail before challenge for:

- unsupported native boundary version;
- parameter set invalid for version;
- verifier family invalid for parameter set;
- reserved aggregation mode;
- digest field with wrong length;
- noncanonical domain encoding, if expanded domain preimages are accepted;
- missing data availability preimage in expanded mode.

## Legacy Handle KATs

Ensure these fields cannot change native challenge unless explicitly included as
allowed projection digests:

- `opening_commitment`;
- `pad_seed_digest`;
- `hidden_source_statement_digest`;
- old carrier bytes;
- fallback checked-core response transcript.

Test shape:

```text
native_frame_input has no legacy field
legacy field mutation cannot affect native_frame bytes
legacy field accepted only as proof evidence outside native frame
```

## Cross-Projection Mismatch KATs

Build frames with mismatched projections:

- `p_spend_digest` from account A and `p_state_digest` from account B;
- aggregation mode in context set to direct but spend/transcript digest built
  for aggregate mode;
- `p_state_digest` for one fork/epoch and context digest for another;
- message digest from one transaction and state root from another.

Expected:

```text
challenge changes or verifier rejects through O_gluing / O_context / frame
binding checks.
```

## Timing KAT

Assert proof commitment is included before challenge:

```text
mutate proof_commitment => challenge changes
```

and no API can compute final challenge before receiving the proof commitment.

## No-Export KAT Link

The frame encoder must not serialize denylisted private fields:

```text
authorization_local
opening witness
pad seed
gauge
randomizer
raw representative coordinates
```

This overlaps with:

```text
research/path2_public_view_no_export_allowlist_v1.md
```

but should be tested locally for the frame encoder.

## Pass Criteria

Pass only if:

- byte order exactly matches `Path2NativeCheckedCoreFrameV1`;
- every semantic field mutation changes the challenge or rejects;
- unsupported versions/modes reject;
- legacy handles have no semantic authority;
- proof commitment is challenge-bound;
- no private denylist field appears in frame bytes.

## Failure Consequence

If this plan cannot be implemented cleanly, then:

```text
checked_core_frame_binding_v1
```

is not implementation-connected, even if the theorem interface exists.

Named break:

```text
checked_core_frame_binding_break
```

## Next Concrete Move

After KAT planning, implement:

```text
Path2NativeCheckedCoreFrameV1
```

encoder/decoder in the production reference or parameter-search reference
module, then add the KATs above.
