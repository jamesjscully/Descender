# Path 2 `p_state(q)` Canonicality V1

Status: first canonicality theorem target for the byte-saving `E_SD` path.

Purpose: prove or specify that:

```text
p_state(q)
```

uniquely determines:

```text
state_transition_class
```

so `p_spend(q)` can edge-link to `p_state(q)` without carrying a separate
32-byte state-transition-class digest.

## Main Finding

The current `p_state(q)` surface is sufficient only if we define the state
transition class as a canonical tuple derived from the full state projection,
not merely as the predecessor account state root.

The correct definition is:

```text
state_transition_class(q) =
  CanonicalStateTransitionClass(
    account_state_root_before,
    admitted_leaf_or_boundary_id,
    live_state_requirement,
    consumed_successor_state_class,
    data_availability_requirement,
    aggregation_state_boundary,
    context_version,
    parameter_set_id,
    verifier_family_id,
    spend_verifier_domain,
    chain_or_application_domain,
    fork_or_epoch_scope,
    aggregation_mode
  )
```

Then:

```text
p_state(q) uniquely determines state_transition_class(q)
```

by canonical decoding and domain separation.

If `p_state(q)` is interpreted only as:

```text
account_state_root_before
```

then uniqueness fails and the +32-byte explicit state-transition-class digest
becomes active.

## Required `p_state(q)` Contents

Use the existing projection surface:

```text
p_state(q) =
  (
    account_state_root_before,
    admitted_leaf_or_boundary_id,
    live_state_requirement,
    consumed_successor_state_class,
    data_availability_requirement,
    aggregation_state_boundary
  )
```

and define:

```text
p_state(q).state_transition_class
```

as the canonical digest/class id of the whole tuple plus context:

```text
StateTransitionClass_c(q) =
  H_domain(
    "Descender.Path2.StateTransitionClass.v1",
    p_context(q).native_boundary_version,
    p_context(q).parameter_set_id,
    p_context(q).verifier_family_id,
    p_context(q).spend_verifier_domain,
    p_context(q).chain_or_application_domain,
    p_context(q).fork_or_epoch_scope,
    p_context(q).aggregation_mode,
    p_state(q).account_state_root_before,
    p_state(q).admitted_leaf_or_boundary_id,
    p_state(q).live_state_requirement,
    p_state(q).consumed_successor_state_class,
    p_state(q).data_availability_requirement,
    p_state(q).aggregation_state_boundary
  )
```

or an equivalent canonical non-hash class id if the fields are carried
expanded.

## Canonicality Theorem

Target:

```text
pstate_canonicality_v1:
  O_canonicality(q)=0
  ∧ O_context(q)=0
    =>
  forall class0 class1.
    DecodeStateClass(p_state(q))=class0
    ∧ DecodeStateClass(p_state(q))=class1
      =>
    class0=class1
```

Equivalent functional form:

```text
exists! cls.
  StateTransitionClassOf(q)=cls
```

This is the byte-saving premise behind:

```text
E_SD(q)=0
  =>
p_spend(q).allowed_state_transition_class
  =
StateTransitionClassOf(q)
```

## What Must Be Canonical

The following must have unique encodings:

- state root bytes;
- admitted leaf or boundary id;
- live/consumed marker;
- successor state class;
- data-availability requirement;
- aggregation state boundary;
- native boundary version;
- parameter set id;
- verifier family id;
- spend verifier domain;
- context domain and fork/epoch scope;
- aggregation mode;
- version tag for the state transition class frame.

Reject if:

- any integer has multiple byte encodings;
- any digest/class reference lacks a domain tag;
- any optional field can be absent in one encoding and defaulted in another;
- state class decoding can ignore aggregation mode;
- data availability is required but the preimage is not available.

Named breaks:

```text
canonicality_break
context_binding_break
byte_data_availability_break
domain_binding_or_qrom_break
```

## Interaction With `E_SD`

Concrete `E_SD` should compare:

```text
spend.allowed_state_transition_class
```

against:

```text
StateTransitionClassOf(q)
```

not against only:

```text
account_state_root_before
```

Digest-projected mode:

```text
spend.state_transition_class_ref
  =
StateTransitionClassDigest(p_state(q), p_context(q))
```

Expanded mode:

```text
spend.state_transition_class_ref
  =
CanonicalTupleId(p_state(q), p_context(q))
```

## Why This Avoids The Extra 32 Bytes

Account state already carries or data-availability-binds:

```text
p_state(q)
```

If `state_transition_class` is a canonical function of `p_state(q)` and context,
then an additional digest inside `p_spend(q)` is duplicative. `E_SD` only needs
to prove that the spend class references the state class computed from
`p_state(q)`.

Byte impact:

```text
0 bytes
```

relative to the preferred model.

## When The +32 Bytes Become Active

Promote the separate state-transition-class digest into active state if:

- `p_state(q)` stores only predecessor root and not successor/live/DA class;
- multiple transition classes can share the same `p_state(q)` encoding;
- the checked-core verifier needs a standalone state authorization digest that
  is not reproducible from `p_state(q)`;
- state transition class depends on runtime policy not included in
  `p_context(q)`;
- aggregation state boundary can change acceptance but is not in `p_state(q)`.

In that case:

```text
account creation public bytes: 244-424
account state bytes:           145-352
```

becomes the active planning range.

## Current Assessment

The byte-saving path is viable if we strengthen the definition of `p_state(q)`
from "state root handle" to "canonical state transition boundary."

This is consistent with the existing projection surface, which already lists:

```text
account_state_root_before
admitted_leaf_or_boundary_id
live_state_requirement
consumed_successor_state_class
data_availability_requirement
aggregation_state_boundary
```

The only change is to make the derived class explicit and domain separated.

## Required Doc Update

Update `research/path2_projection_surfaces_v1.md` so `p_state(q)` explicitly
defines:

```text
state_transition_class =
  CanonicalStateTransitionClass(p_state(q), p_context(q))
```

and clarify that `account_state_root_before` alone is not the transition class.

## Required Lean Update

Add a skeletal canonicality interface:

```text
StateProjectionFields
StateProjectionCanonicality
pstate_transition_class_unique
```

This should remain structural; concrete hash/domain encodings belong in the
implementation/spec layer.

## Next Concrete Move

After this canonicality interface is in place, update the byte model to mark the
preferred range as active conditional on `pstate_canonicality_v1`, not merely on
the informal edge-linked decision.
