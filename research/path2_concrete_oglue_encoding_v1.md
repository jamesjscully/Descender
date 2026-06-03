# Path 2 Concrete `O_gluing` Encoding V1

Status: first concrete public-check encoding for the edge-linked active path.

Purpose: instantiate byte/equality checks for:

```text
E_SD
E_ST
context / pullback checks
```

under the active decision:

```text
allowed_state_transition_class is edge-linked through p_state(q)
aggregation_mode is packed in context/mode bytes and edge-linked through
p_spend(q), p_transcript(q), and p_context(q).
```

## Encoding Principle

All checks are public, deterministic, and canonical.

The verifier computes semantic projections:

```text
p_context(q)
p_spend(q)
p_transcript(q)
p_state(q)
```

from canonical `q` bytes or from data-available canonical preimages named by
`q`. It then checks equality of typed fields. No check opens:

```text
AuthorizationLocal_c(q)
Gauge_c(q)
Randomizer_c(q)
```

or any private representative coordinate.

## Field Names

For this instantiation, use these field projections:

```text
ctx.verifier_family_id
ctx.spend_verifier_domain
ctx.aggregation_mode

spend.spend_verifier_family
spend.spend_domain_separator
spend.checked_core_or_native_verifier_handle
spend.spend_authorization_class
spend.public_authorization_identity
spend.allowed_state_transition_class
spend.aggregation_mode

transcript.boundary_id
transcript.spend_digest_or_class
transcript.state_digest_or_class
transcript.lifecycle_digest_or_class
transcript.context_digest_or_class
transcript.aggregation_mode
transcript.transcript_domain_separator

state.state_transition_class
state.account_state_root_before
state.consumed_successor_state_class
```

The concrete byte layout may pack these fields into digests or handles. The
semantic equality checks are still defined over the decoded canonical fields.

## `E_SD`: Spend-State Edge

Purpose:

```text
p_spend(q)
```

must authorize exactly the state transition class represented by:

```text
p_state(q)
```

Check:

```text
E_SD(q)=0 iff
  spend.allowed_state_transition_class = state.state_transition_class
```

If `spend.allowed_state_transition_class` is represented as:

```text
DigestOrClassRef(p_state(q).state_transition_class)
```

then the check is:

```text
decode_class_ref(spend.allowed_state_transition_class)
  =
decode_state_class(state.state_transition_class)
```

or, in digest-projected mode:

```text
spend.state_transition_class_ref = p_state_digest(q)
```

provided `p_state_digest(q)` is canonical and domain separated as a state-class
handle.

Accepted zero:

```text
r_SD(q.spend,q.state)=0
```

Breaks if false:

```text
spend_state_closure_break
canonicality_break
byte_data_availability_break
domain_binding_or_qrom_break
```

## `E_ST`: Spend-Transcript Edge

Purpose:

The challenge/transcript surface must bind the public spend class and
aggregation mode used by the spend verifier.

Checks:

```text
E_ST(q)=0 iff
  transcript.spend_digest_or_class = canonical_digest_or_class(p_spend(q))
  ∧ transcript.aggregation_mode = spend.aggregation_mode
  ∧ transcript.transcript_domain_separator is valid for spend.spend_domain_separator
```

Checked-core frame dependency:

```text
checked_core_frame(q,msg,state,commitment)
```

must be built from:

```text
p_spend(q)
p_transcript(q)
p_state(q)
p_lifecycle(q)
p_context(q)
msg
state
commitment
```

and no independent slot/opening/pad/carrier handle.

Accepted zero:

```text
r_ST(q.spend,q.transcript)=0
```

Breaks if false:

```text
checked_core_frame_binding_break
domain_binding_or_qrom_break
canonicality_break
byte_data_availability_break
```

## Context / Pullback Checks

Purpose:

All faces must live over the same parameter, verifier, domain, and aggregation
context.

Minimum checks:

```text
ctx.verifier_family_id = spend.spend_verifier_family
ctx.spend_verifier_domain = spend.spend_domain_separator
ctx.aggregation_mode = spend.aggregation_mode
ctx.aggregation_mode = transcript.aggregation_mode
```

and:

```text
transcript.context_digest_or_class = canonical_digest_or_class(p_context(q))
```

These may be placed in:

```text
O_context
O_pullback
E_ST
```

The recommended split:

```text
O_context:
  well-formed version/mode/domain values

O_pullback:
  ctx.verifier_family_id = spend.spend_verifier_family
  ctx.spend_verifier_domain = spend.spend_domain_separator
  ctx.aggregation_mode = spend.aggregation_mode

E_ST:
  transcript binds p_spend(q)
  transcript binds ctx aggregation mode
```

Breaks if false:

```text
context_binding_break
pullback_consistency_break
domain_binding_or_qrom_break
```

## Aggregate Mode Rules

Allowed modes:

```text
0 = direct
1 = aggregate-compatible
2..reserved = invalid unless activated by context version
```

Checks:

```text
ctx.aggregation_mode ∈ allowed_modes(ctx.native_boundary_version)
spend.aggregation_mode = ctx.aggregation_mode
transcript.aggregation_mode = ctx.aggregation_mode
```

This bakes aggregation assumptions into the model without requiring aggregation
for direct-mode accounts.

## Canonical Encoding Checks

Each field comparison must be made after canonical decoding:

```text
decode_canonical(field_bytes) = semantic_field
```

Reject if:

- non-canonical integer encoding;
- wrong domain tag;
- wrong version tag;
- digest preimage unavailable when the mode requires data availability;
- reserved aggregation mode;
- digest/class reference type mismatch.

Named breaks:

```text
canonicality_break
byte_data_availability_break
domain_binding_or_qrom_break
```

## Obstruction Map Placement

The active obstruction map remains:

```text
O_c =
  O_context
  × O_pullback
  × O_gluing
  × O_meaning
  × O_spend_state
  × O_canonicality
```

Concrete placement:

```text
O_context:
  mode/domain/version well-formedness

O_pullback:
  context-spend equality
  context-transcript equality

O_gluing:
  E_AS, E_AL, E_ST, E_LT, E_LD, E_AD, E_SD

O_spend_state:
  duplicated summary bit for E_SD and lifecycle/state consume-once compatibility

O_canonicality:
  canonical field decoding and digest/class-ref uniqueness
```

`O_spend_state` may be redundant with `E_SD`, `E_LD`, and `E_AD` in the first
implementation. Keep it as a named summary component until the concrete
obstruction object is minimized.

## Public Boundary Theorem

Concrete target:

```text
concrete_oglue_encoding_sound_v1:
  O_context(q)=0
  ∧ O_pullback(q)=0
  ∧ r_ST(q.spend,q.transcript)=0
  ∧ r_SD(q.spend,q.state)=0
  ∧ O_canonicality(q)=0
    =>
  p_spend(q).allowed_state_transition_class
    =
  p_state(q).state_transition_class
  ∧ p_spend(q).aggregation_mode
    =
  p_context(q).aggregation_mode
  ∧ p_transcript(q).aggregation_mode
    =
  p_context(q).aggregation_mode
  ∧ p_transcript(q) binds p_spend(q)
```

This is the exact theorem needed to justify the byte model's edge-linked
state-transition-class encoding.

## Byte Consequence

Because `E_SD` links spend to state by equality/reference, no independent
32-byte state-transition-class digest is required in `p_spend(q)`.

Active preferred range remains:

```text
account creation public bytes: 244-392
account state bytes:           145-320
```

Stress fallback remains:

```text
account creation public bytes: 244-424
account state bytes:           145-352
```

only if implementation cannot make `E_SD` deterministic over existing `p_state`
data.

## Fail Conditions

Mark the edge-linked encoding failed if:

- `p_state(q)` does not uniquely determine `state_transition_class`;
- transcript aggregation mode can differ from spend/context aggregation mode;
- checked-core frame omits the state transition class or aggregation mode;
- digest/class-ref decoding is not canonical;
- an old slot authorization/opening/pad/carrier handle retains semantic
  authority outside `q`.

## Next Concrete Move

Represent these checks in the Lean theorem interface as named concrete gluing
predicates:

```text
spendStateEdgeSound
spendTranscriptEdgeSound
contextPullbackSound
```

and compose them into a theorem that exports the equality facts above.
