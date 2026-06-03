# Path 2 Checked-Core Frame Byte Audit V1

Status: first byte-level audit against the native projection frame.

Purpose: connect the proof-side theorem:

```text
checked_core_frame(q,msg,state,commitment)
```

to concrete checked-core/frame bytes, and identify whether existing checked-core
helpers already bind the native projections or are only fallback transcript
evidence.

## Main Finding

The currently visible checked-core byte helper:

```text
encode_hidden_source_fallback_checked_core_transcript_frame(response)
```

encodes:

```text
domain || profile || checked-core response residues
```

and rejects wrong domain/profile or noncanonical q12289 response coordinates.

This is useful as fallback checked-core transcript evidence, but it is not the
native spend challenge frame because it does not itself bind:

```text
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
p_context(q)
message
state
proof_commitment
```

Therefore, for the active native path, this helper must be classified as:

```text
checked-core proof/transcript evidence
```

not:

```text
the native checked_core_frame(q,msg,state,commitment)
```

## Required Native Frame Bytes

Define a native checked-core challenge frame:

```text
Path2NativeCheckedCoreFrameV1 =
  domain_tag
  || native_boundary_version
  || parameter_set_id
  || verifier_family_id
  || spend_verifier_domain
  || chain_or_application_domain
  || fork_or_epoch_scope
  || aggregation_mode
  || native_boundary_digest(q)
  || p_spend_digest_or_encoding(q)
  || p_lifecycle_digest_or_encoding(q)
  || p_transcript_digest_or_encoding(q)
  || p_state_digest_or_encoding(q)
  || message_digest_or_canonical_message
  || current_state_digest_or_root
  || proof_commitment
```

This frame must be the byte preimage for checked-core challenge derivation or a
domain-separated subframe included in that challenge derivation.

## Old Fields And Native Classification

| existing / older byte | active native classification | allowed condition |
| --- | --- | --- |
| `slot_authorization_digest` | byte binding for `p_spend(q)` / `p_transcript(q)` | must be recomputed from canonical native projection preimage |
| `account_state_root_before` | field of `p_state(q)` | not sufficient alone as state class |
| `opening_commitment` | private/proof evidence or typed restriction | dead as semantic frame input |
| `pad_seed_digest` | gauge/randomizer byte binding or fallback proof metadata | dead as semantic frame input |
| `hidden_source_statement_digest` | fallback proof metadata | dead as active native semantic input |
| `carrier_transcript_binding` | old fallback carrier evidence | not active native frame |
| checked-core response residues | signature/proof evidence | must be bound by native challenge frame |
| checked-core response transcript frame | fallback evidence frame | not native projection frame |
| `proof_commitment` | accepted native frame input | must be fixed before challenge |

## Audit Against Production Reference

Observed helper:

```text
hidden_source_fallback_checked_core_transcript_frame_bytes()
encode_hidden_source_fallback_checked_core_transcript_frame(response)
decode_hidden_source_fallback_checked_core_transcript_frame(frame)
```

Observed frame shape:

```text
HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_DOMAIN
|| HIDDEN_SOURCE_FALLBACK_CHECKED_CORE_TRANSCRIPT_FRAME_PROFILE
|| response[0..SIS_FULL_RESPONSE_RESIDUES]
```

Good properties:

- domain tag checked;
- profile tag checked;
- response length checked;
- response residues must be canonical `< q`;
- wrong domain/profile negative tests exist.

Missing for native frame binding:

- no native boundary digest;
- no `p_context(q)` fields;
- no `p_spend(q)` digest/class;
- no `p_lifecycle(q)` digest/class;
- no `p_state(q)` digest/class;
- no message;
- no current state;
- no proof commitment in this helper's frame;
- no aggregation mode.

Verdict:

```text
fallback_checked_core_transcript_frame != native_checked_core_frame
```

The existing helper can remain as checked-core evidence but cannot discharge
`checked_core_frame_binding_v1`.

## Native Binding Requirement

The active verifier must compute:

```text
native_frame = encode_path2_native_checked_core_frame_v1(q,msg,state,commitment)
challenge = H_domain(native_frame)
```

or prove that an existing checked-core challenge frame has this exact frame as a
domain-separated subframe.

Required equality:

```text
checked_core_challenge_preimage
  =
Path2NativeCheckedCoreFrameV1(q,msg,state,commitment)
```

or:

```text
checked_core_challenge_preimage
  =
legacy_checked_core_prefix
  || H_domain(Path2NativeCheckedCoreFrameV1(q,msg,state,commitment))
```

with an explicit theorem that the legacy prefix cannot change semantics.

## Negative Controls Required

Native frame tests must reject/mutate:

- native boundary digest;
- parameter set id;
- verifier family id;
- spend verifier domain;
- chain/application domain;
- fork/epoch scope;
- aggregation mode;
- `p_spend` digest/class;
- `p_lifecycle` digest/class;
- `p_transcript` digest/class;
- `p_state` digest/class;
- message digest;
- current state root;
- proof commitment.

Each mutation must change the challenge or be rejected before challenge.

## Break Mapping

| failure | break |
| --- | --- |
| checked-core uses fallback response transcript as challenge frame | `checked_core_frame_binding_break` |
| slot authorization digest not recomputed from `q` | `checked_core_frame_binding_break` |
| message omitted | `domain_binding_or_qrom_break` |
| state root omitted | `spend_state_closure_break` |
| aggregation mode omitted | `context_binding_break` or `checked_core_frame_binding_break` |
| proof commitment chosen after challenge | `checked_core_frame_binding_break` |
| legacy opening/pad/carrier handle changes semantics | `checked_core_frame_binding_break` |

## Byte Impact

If the spend verifier can load projections from account state, the signature
does not need to carry the full frame.

Spend signature byte impact remains:

```text
0-32 bytes
```

depending on whether `native_boundary_digest(q)` or
`slot_authorization_digest(q)` must be carried explicitly in the signature.

The frame bytes are verifier/challenge preimage bytes, not necessarily stored
on chain as signature payload.

## Current Verdict

The active proof theorem is still viable, but the implementation-facing frame is
not yet instantiated.

The old checked-core transcript frame is a useful evidence object, but the
native path needs a new:

```text
Path2NativeCheckedCoreFrameV1
```

or a wrapper proving an existing frame includes the native projection digest.

## Next Concrete Move

Create:

```text
research/path2_native_checked_core_frame_v1_spec.md
```

with exact field order, domain tags, canonical encodings, and mutation tests.

Do not claim checked-core frame binding closed until that spec exists and the
implementation or theorem interface points at it.
