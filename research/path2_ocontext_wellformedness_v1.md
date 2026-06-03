# Path 2 `O_context` Well-Formedness V1

Status: first context validity theorem target after context/state domain
binding.

Purpose: specify the public checks that make:

```text
p_context(q)
```

valid, canonical, and non-ambiguous before its fields are used by:

- `StateTransitionClass_c(q)`;
- checked-core frame binding;
- transcript/domain binding;
- aggregation-mode selection;
- account creation and spend verification.

## Context Fields

The active context projection is:

```text
p_context(q) =
  (
    native_boundary_version,
    parameter_set_id,
    verifier_family_id,
    account_creation_domain,
    spend_verifier_domain,
    chain_or_application_domain,
    fork_or_epoch_scope,
    aggregation_mode
  )
```

## Well-Formedness Checks

`O_context(q)=0` must imply all of the following.

### Version Validity

```text
native_boundary_version ∈ SupportedNativeBoundaryVersions
```

and the version selects:

```text
projection layout
canonical encoding rules
allowed aggregation modes
parameter-set table
verifier-family table
```

Reject reserved or unknown versions.

### Parameter Set Validity

```text
parameter_set_id ∈ ParameterSets(native_boundary_version)
```

The parameter set fixes:

```text
q/modulus or byte-domain interpretation
hash/domain suite
checked-core parameter family
allowed digest/class-ref formats
```

Reject if a parameter id is valid under another version but not this one.

### Verifier Family Validity

```text
verifier_family_id ∈ VerifierFamilies(parameter_set_id)
```

and:

```text
verifier_family_id = p_spend(q).spend_verifier_family
```

via pullback.

Reject if verifier family is reserved, disabled, or incompatible with the
parameter set.

### Domain Validity

All domains must be non-ambiguous tagged values:

```text
account_creation_domain
spend_verifier_domain
chain_or_application_domain
fork_or_epoch_scope
```

Rules:

- no empty/default domain unless explicitly versioned;
- domain tags include the protocol/application namespace;
- fork/epoch scope has a canonical encoding and clear reorg semantics;
- account creation and spend domains are distinct unless a version explicitly
  permits sharing with separate sub-tags.

### Aggregation Mode Validity

Allowed first-pass modes:

```text
0 = direct
1 = aggregate-compatible
2..reserved = invalid unless activated by native_boundary_version
```

Checks:

```text
aggregation_mode ∈ AllowedAggregationModes(native_boundary_version)
```

and via pullback:

```text
p_context(q).aggregation_mode = p_spend(q).aggregation_mode
p_context(q).aggregation_mode = p_transcript(q).aggregation_mode
```

Reserved modes must fail closed.

## Theorem Target

```text
ocontext_wellformedness_v1:
  O_context(q)=0
    =>
  ContextVersionValid(q)
  ∧ ParameterSetValid(q)
  ∧ VerifierFamilyValid(q)
  ∧ DomainsCanonical(q)
  ∧ AggregationModeValid(q)
```

and:

```text
O_context(q)=0 ∧ O_pullback(q)=0
  =>
  context fields used by p_spend(q), p_transcript(q), and
  StateTransitionClass_c(q) are equal to p_context(q).
```

## Failure / Break Mapping

| failure | break |
| --- | --- |
| unknown version | `context_binding_break` |
| invalid parameter set | `context_binding_break` |
| verifier family not valid for parameter set | `context_binding_break` |
| context/spend verifier mismatch | `pullback_consistency_break` |
| context/spend domain mismatch | `pullback_consistency_break` |
| ambiguous domain encoding | `domain_binding_or_qrom_break` |
| reserved aggregation mode accepted | `context_binding_break` |
| aggregation mode mismatch across context/spend/transcript | `pullback_consistency_break` |
| unavailable context preimage in digest mode | `byte_data_availability_break` |

## Byte Consequence

No new bytes are required if:

```text
native_boundary_version
parameter_set_id
verifier_family_id
aggregation_mode
```

are packed into the existing:

```text
boundary/version/mode flags: 4-8 bytes
```

and larger domain fields remain digest-bound by `p_context_digest(q)`.

Preferred byte target remains:

```text
account creation public bytes: 244-392
account state bytes:           145-320
```

## Security Consequence

With `O_context` well-formedness and context/state domain binding, an accepted
state transition class is scoped to one:

```text
version / parameter set / verifier family / domain / fork-or-epoch /
aggregation mode
```

This prevents a valid spend/account boundary from being replayed under a
different protocol interpretation.

## Lean Interface Needed

Add a structural interface:

```text
ContextFields
ContextWellFormedness
context_wellformed_of_zero
```

The concrete version and mode tables belong in implementation/spec artifacts.
The Lean interface should state the dependency shape and exported predicates.

## Next Concrete Move

After adding the Lean interface, update the residual assumption ledger:

- `O_context` is no longer just "context is well formed" prose;
- it has named validity predicates and failure breaks;
- the next remaining blocker is concrete checked-core frame byte audit or
  public-view/no-export serializer allowlist, depending on whether we keep
  proof-side-only or start implementation-facing checks.
