# Path 2 Projection Encoding Instantiation V1

Status: first concrete encoding decision for strengthened `p_spend(q)`.

Purpose: decide how to encode:

```text
p_spend(q).allowed_state_transition_class
p_spend(q).aggregation_mode
```

without unnecessarily increasing account creation or account state bytes.

## Decision

Use edge-linked encoding for `allowed_state_transition_class`:

```text
p_spend(q).allowed_state_transition_class
  :=
DigestOrClassRef(p_state(q).state_transition_class)
```

with `E_SD` enforcing:

```text
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

Do not store a separate 32-byte state-transition-class digest in `p_spend(q)`
unless implementation ergonomics later require it.

Use packed explicit encoding for `aggregation_mode`:

```text
aggregation_mode : 1-4 bytes
```

stored in context/version/mode flags and included semantically in:

```text
p_context(q)
p_spend(q)
p_transcript(q)
```

with pullback/transcript edges enforcing equality.

## Rationale

The state transition boundary is already represented by:

```text
p_state(q)
```

and account state already stores a state transition/predecessor root handle. A
second digest inside `p_spend(q)` duplicates the same semantic class.

The theorem needs:

```text
AuthClass_c(auth).allowed_state_transition_class
  =
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

It does not require two independent byte handles.

## Encoding Shape

### Compact Digest-Projected Mode

Store:

```text
p_spend_digest(q)
p_state_digest(q)
```

where the canonical preimage of `p_spend_digest(q)` includes either:

```text
state_transition_class_ref = p_state_digest(q)
```

or:

```text
state_transition_class_ref = canonical_state_class_id(p_state(q))
```

The canonical preimage of `p_state_digest(q)` contains the full state transition
class.

Byte impact:

```text
0 bytes
```

beyond the existing `p_spend_digest` and `p_state_digest`.

### Expanded Canonical Mode

Store explicit:

```text
aggregation_mode      : 1-4 bytes
state_transition_ref  : 0 bytes if implied by p_state handle
```

or:

```text
state_transition_ref  : small enum/index if multiple state classes share one state handle
```

Preferred explicit overhead:

```text
1-4 bytes
```

Conservative maximum:

```text
32 bytes
```

only if the state class cannot be derived from the stored `p_state(q)` handle.

## Required Edge Checks

### Spend-State Edge `E_SD`

```text
E_SD(q)=0
  =>
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

This is the core byte-saving edge.

### Spend-Transcript Edge `E_ST`

```text
E_ST(q)=0
  =>
p_transcript(q) binds p_spend(q)
∧ p_transcript(q).aggregation_mode = p_spend(q).aggregation_mode
```

### Context Pullback

```text
O_pullback(q)=0
  =>
p_context(q).aggregation_mode = p_spend(q).aggregation_mode
```

and:

```text
p_context(q).verifier_family_id = p_spend(q).spend_verifier_family
```

## Byte Consequence

Preferred active planning numbers remain:

```text
account creation public bytes: 244-392
account state bytes:           145-320
signer private state bytes:    128-416
spend signature impact:        0-32
```

The conservative stress headroom:

```text
account creation public bytes: 244-424
account state bytes:           145-352
```

is retained only as a fallback if implementation later proves the state class
cannot be derived from `p_state(q)`.

## Security Consequence

This encoding is sound if:

```text
p_state(q)
```

is itself canonical and collision/domain bound. Otherwise, an adversary could
make `p_spend(q)` authorize one state class while `p_state(q)` denotes another.

Named breaks:

```text
spend_state_closure_break
canonicality_break
domain_binding_or_qrom_break
byte_data_availability_break
```

## Aggregation Consequence

Including `aggregation_mode` as a small semantic field now preserves optional
future aggregation without requiring it.

Rules:

```text
direct mode
aggregate-compatible mode
reserved modes rejected unless activated by context version
```

Aggregation mode must be in the transcript domain so a signature generated for
one mode cannot be replayed as another mode.

## Pass Criteria

Pass if:

- `p_state(q)` uniquely determines the state transition class;
- `E_SD` enforces spend-state equality;
- `aggregation_mode` is included in context, spend, and transcript semantics;
- all mode/version bytes are domain separated;
- no checked-core frame can omit the aggregation/state-class coordinates.

## Fail Criteria

Fail and use +32-byte headroom if:

- `p_state(q)` is only a predecessor root and does not determine transition
  class;
- multiple transition classes share one `p_state(q)` handle;
- checked-core needs a standalone state authorization digest;
- implementation cannot make `E_SD` cheap and deterministic.

## Next Concrete Move

Update `research/path2_native_byte_state_model_v1.md` to mark the preferred
range as active under the edge-linked encoding decision, and keep the +32-byte
range as stress fallback only.
