# Path 2 Spend Projection Completeness Audit V1

Status: first audit of the key premise behind authorization class binding.

Purpose: check whether:

```text
p_spend(q)
```

and edge-linked projections of `q` contain every public spend-semantic
coordinate consumed by verification.

If yes, authorization class binding is a clean typed theorem. If no, hidden
authorization material still has semantic authority and the missing coordinate
must move into `q`.

## Audit Rule

For every value consumed by:

```text
SpendAccept(c,q,sig,msg,state)
```

one of the following must hold:

```text
value = field of p_spend(q)
```

or:

```text
value = field of p_lifecycle(q), p_transcript(q), p_state(q), p_context(q),
        with an O_gluing edge tying it to p_spend(q).
```

or:

```text
value is message/state/signature input, not account semantics.
```

Everything else is a projection completeness failure.

## Spend Semantic Coordinates

| coordinate | active location | edge if not in `p_spend` | status |
| --- | --- | --- | --- |
| spend verifier family | `p_spend(q)` and `p_context(q)` | context-spend / pullback | pass if equalized |
| public authorization identity | `p_spend(q)` and `p_account(q)` | `E_AS` | pass |
| spend authorization class | `p_spend(q)` | none | pass |
| checked-core/native verifier handle | `p_spend(q)` | `E_ST` binds transcript | pass with frame audit |
| allowed authorization-local mode | `p_spend(q)` | none | pass |
| spend domain separator | `p_spend(q)` and `p_context(q)` | context-spend / transcript | pass if equalized |
| message | external spend input | none | not account semantics |
| current consensus state | external spend input | state membership check | not fixed account semantics |
| predecessor/account state root | `p_state(q)` | `E_AD`, `E_SD` | pass |
| successor/consumed state class | `p_state(q)` | `E_SD`, `E_LD` | pass |
| slot/nullifier id | `p_lifecycle(q)` | `E_AL`, `E_LD`, `E_LT` | pass |
| replay domain | `p_lifecycle(q)` and `p_context(q)` | lifecycle-transcript | pass if transcript binds |
| challenge/transcript domain | `p_transcript(q)` and `p_context(q)` | `E_ST`, `E_LT` | pass with frame audit |
| aggregation mode | `p_context(q)` and `p_transcript(q)` | transcript/context pullback | pass if semantics-affecting mode is included |
| boundary/account id | `p_account(q)` and `p_transcript(q)` | `E_AS`, `E_AD`, transcript edges | pass |

## Current Gaps

### Gap 1: Verifier Family Duplication

`p_context(q)` contains:

```text
verifier_family_id
spend_verifier_domain
aggregation_mode
```

and `p_spend(q)` contains:

```text
spend_verifier_family
spend_domain_separator
```

This is acceptable only if pullback/context checks enforce:

```text
p_spend(q).spend_verifier_family = p_context(q).verifier_family_id
```

or if one is explicitly a subtype/refinement of the other.

Required edge:

```text
E_context_spend
```

or include this equality in `O_context × O_pullback`.

### Gap 2: Aggregation Mode Placement

Aggregation mode appears in:

```text
p_context(q)
p_transcript(q)
```

but not explicitly in the earlier `p_spend(q)` draft.

If aggregation mode changes spend acceptance semantics, it must be included in:

```text
AuthClass_c(auth)
```

and either directly in:

```text
p_spend(q)
```

or edge-linked with a theorem:

```text
p_spend(q).aggregation_mode = p_context(q).aggregation_mode
```

Recommendation: include aggregation mode in `p_spend(q)` as a semantic field,
even if encoded compactly through context. This supports future aggregation
without making aggregation mandatory.

### Gap 3: Checked-Core Handle Semantics

`p_spend(q)` contains:

```text
checked_core_or_native_verifier_handle
```

but spend closure requires the whole frame:

```text
frame(
  p_spend(q),
  p_lifecycle(q),
  p_transcript(q),
  p_state(q),
  msg,
  state,
  p_context(q)
)
```

The handle is safe only if it selects a verifier relation, not an independent
semantic account or authorization object.

Required theorem:

```text
checked_core_frame_binding_v1:
  checked_core_frame(q,msg,state)
    =
  frame(p_spend(q), p_lifecycle(q), p_transcript(q),
        p_state(q), msg, state, p_context(q))
```

This remains a real proof item.

### Gap 4: State Transition Class In `p_spend`

The authorization class proof note included:

```text
allowed_state_transition_class
```

as part of the spend authorization class. The current projection surface puts
state transition mostly in:

```text
p_state(q)
```

This is safe if `E_SD` proves:

```text
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

But the earlier `p_spend(q)` contents did not explicitly list
`allowed_state_transition_class`.

Recommendation: add a compact state-transition-class field to `p_spend(q)`, or
define `p_spend(q)` as containing an edge-linked reference to `p_state(q)`.

For authorization binding, direct inclusion is cleaner.

## Revised `p_spend(q)` Contents

Update the active semantic definition to:

```text
p_spend(q) =
  (
    spend_verifier_family,
    spend_authorization_class,
    checked_core_or_native_verifier_handle,
    public_key_identity_or_authorization_identity,
    allowed_authorization_local_mode,
    allowed_state_transition_class,
    aggregation_mode,
    spend_domain_separator
  )
```

This does not necessarily add many bytes if these are already represented by
digests or compact handles, but it removes ambiguity from the theorem.

## Revised `AuthClass_c(auth)` Contents

Match `p_spend(q)` exactly:

```text
AuthClass_c(auth) =
  (
    spend_verifier_family(auth),
    spend_authorization_class(auth),
    checked_core_or_native_verifier_handle(auth),
    public_key_identity_or_authorization_identity(auth),
    allowed_authorization_local_mode(auth),
    allowed_state_transition_class(auth),
    aggregation_mode(auth),
    spend_domain_separator(auth)
  )
```

Private auth data may contain more, but all public spend semantics must project
to this tuple.

## Consequence For Byte Model

This likely does not change the current planning range if `p_spend(q)` remains a
32-byte digest/handle in compact mode:

```text
account creation public bytes: 244-392
account state bytes:           145-320
```

Expanded canonical mode may add explicit bytes if we choose to store:

```text
aggregation_mode
allowed_state_transition_class
verifier_family
```

outside existing context/state handles.

Estimated explicit overhead:

```text
aggregation_mode:                 1-4 bytes
verifier/state class mode fields:  2-8 bytes if small enums
state transition class digest:     0 or 32 bytes if not already in p_state
```

Recommendation: keep explicit enum/mode fields small and digest-bind the larger
state class through `p_state(q)` plus `E_SD`.

## Verdict

Spend projection completeness is achievable, but the active `p_spend(q)` draft
should be strengthened.

The two fields to add or explicitly edge-link are:

```text
allowed_state_transition_class
aggregation_mode
```

The checked-core handle also needs a separate frame-binding theorem, but that is
a spend-closure proof item rather than a reason to abandon authorization class
binding.

## Next Concrete Move

Update:

```text
research/path2_projection_surfaces_v1.md
```

so `p_spend(q)` includes:

```text
allowed_state_transition_class
aggregation_mode
```

and update `O_gluing` to make the `E_SD` and context/transcript edges explicit
for those fields.
