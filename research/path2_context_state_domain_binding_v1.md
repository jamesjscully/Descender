# Path 2 Context-State Domain Binding V1

Status: first domain-binding theorem target for `StateTransitionClass_c(q)`.

Purpose: prove or specify that the context fields used by:

```text
StateTransitionClass_c(q)
```

are uniquely bound to:

```text
p_context(q)
```

so state transition classes cannot be replayed across:

- chain/application domains;
- fork or epoch scopes;
- verifier families;
- spend verifier domains;
- aggregation modes.

## Main Finding

`pstate_canonicality_v1` is not enough by itself. It gives local uniqueness of
the state transition tuple. Deployment safety also requires domain uniqueness:

```text
same state bytes under a different context
  !=
same state transition class
```

unless the two contexts are explicitly equal in the relevant fields.

The derived class must include:

```text
p_context(q).chain_or_application_domain
p_context(q).fork_or_epoch_scope
p_context(q).verifier_family_id
p_context(q).spend_verifier_domain
p_context(q).aggregation_mode
```

not only the state fields.

## Corrected State Class Frame

Use:

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

or the equivalent expanded canonical tuple.

## Domain Binding Theorem

Target:

```text
context_state_domain_binding_v1:
  StateTransitionClass_c(q0)=StateTransitionClass_c(q1)
  ∧ O_canonicality(q0)=0
  ∧ O_canonicality(q1)=0
  ∧ O_context(q0)=0
  ∧ O_context(q1)=0
    =>
  (
    p_state(q0)=p_state(q1)
    ∧ p_context(q0).native_boundary_version =
      p_context(q1).native_boundary_version
    ∧ p_context(q0).parameter_set_id =
      p_context(q1).parameter_set_id
    ∧ p_context(q0).verifier_family_id =
      p_context(q1).verifier_family_id
    ∧ p_context(q0).spend_verifier_domain =
      p_context(q1).spend_verifier_domain
    ∧ p_context(q0).chain_or_application_domain =
      p_context(q1).chain_or_application_domain
    ∧ p_context(q0).fork_or_epoch_scope =
      p_context(q1).fork_or_epoch_scope
    ∧ p_context(q0).aggregation_mode =
      p_context(q1).aggregation_mode
  )
```

or:

```text
domain_binding_or_qrom_break
canonicality_break
context_binding_break
byte_data_availability_break
```

For hash-backed compact encodings, this theorem rests on the ordinary
domain-separated digest binding assumption already carried by the active model.

## Why Aggregation Mode Must Be Included

Aggregation mode changes how a spend may be verified or batched. If the state
transition class omits it, a state class created in:

```text
direct mode
```

could be replayed in:

```text
aggregate-compatible mode
```

if every other state field matches.

That is not necessarily a forgery by itself, but it breaks the promise that
accepted spend semantics are fully determined by:

```text
meaning_of_boundary_c(q)
```

because aggregation behavior is part of `p_context(q)`, `p_spend(q)`, and
`p_transcript(q)`.

## Pullback Requirements

The concrete gluing/pullback layer must enforce:

```text
p_context(q).verifier_family_id = p_spend(q).spend_verifier_family
p_context(q).spend_verifier_domain = p_spend(q).spend_domain_separator
p_context(q).aggregation_mode = p_spend(q).aggregation_mode
p_context(q).aggregation_mode = p_transcript(q).aggregation_mode
```

and `StateTransitionClass_c(q)` must use the same context values.

This ties together:

```text
O_context
O_pullback
E_ST
E_SD
pstate_canonicality_v1
```

## Canonicality Requirements

Reject if:

- any context field has multiple byte encodings;
- reserved aggregation mode is accepted without a context version enabling it;
- fork/epoch scope can be absent and defaulted inconsistently;
- chain/application domain is not domain-tagged;
- digest preimages for compact handles are unavailable when required;
- state class frame uses context bytes that differ from `p_context(q)`.

## Byte Consequence

This theorem does not add bytes if context fields are already counted in:

```text
p_context(q)
```

and `StateTransitionClass_c(q)` is a derived digest/class id.

Preferred active range remains:

```text
account creation public bytes: 244-392
account state bytes:           145-320
```

The +32-byte fallback is still needed only if `StateTransitionClass_c(q)` cannot
be derived from existing `p_state(q)` and `p_context(q)` fields.

## Security Consequence

With this theorem:

```text
E_SD(q)=0
```

proves not merely:

```text
spend authorizes this state tuple
```

but:

```text
spend authorizes this state tuple in this verifier/domain/fork/aggregation
context.
```

That is the exact property needed for blockchain replay safety.

## Required Doc Update

Update `research/path2_pstate_canonicality_v1.md` and
`research/path2_projection_surfaces_v1.md` so the derived state transition class
lists:

```text
native_boundary_version
parameter_set_id
verifier_family_id
spend_verifier_domain
chain_or_application_domain
fork_or_epoch_scope
aggregation_mode
```

not only chain/app domain and fork/epoch scope.

## Required Lean Update

Extend `StateProjectionFields` with explicit context components:

```text
stateContextVersion
stateContextParameterSet
stateContextVerifierFamily
stateContextSpendDomain
stateContextChainApplicationDomain
stateContextForkEpochScope
stateContextAggregationMode
```

and update `StateProjectionCanonicality` so `stateTransitionClass` is a
deterministic function of both state and context fields.

## Next Concrete Move

After this interface update, the next blocker is to specify exact
`O_context` well-formedness for aggregation mode and reserved mode activation.
