# Path 2 Native Projection Surfaces V1

Status: first concrete projection-surface draft.

Purpose: define the contents of the public projections exported by the active
native boundary:

```text
p_context(q)
p_account(q)
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
```

These are semantic surfaces. Byte encodings and digest choices come later.

## Design Rule

Every consensus-visible semantic input must be computed from:

```text
q : Q_c
```

If a verifier needs a handle that is not one of these projections, the handle is
valid only as byte/data-availability binding for a projection, or it is dead.

## `p_context(q)`

Semantic role:

Parameter and domain interpretation for every other projection.

Contents:

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

Required laws:

```text
same q => same parameter interpretation
all faces of q have context = p_context(q)
```

Failure:

If a digest or verifier frame can be reused under a different context, export
`context_binding_break` or `domain_binding_or_qrom_break`.

## `p_account(q)`

Semantic role:

Consensus-visible account namespace.

Contents:

```text
p_account(q) =
  (
    account_id,
    account_namespace,
    public_key_identity_or_account_identity,
    creation_namespace,
    admission_state_namespace
  )
```

Required laws:

```text
meaning_of_boundary(q).accountNamespace = p_account(q)
```

and:

```text
p_account(q) participates in p_lifecycle(q), p_transcript(q), and p_state(q)
```

Failure:

If account identity is reconstructed from an independent opening/pad/proof
handle, the path is a `projection_factorization_break`.

## `p_spend(q)`

Semantic role:

Public authorization class consumed by spend verification.

Contents:

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

Compatibility aliases:

```text
slot_authorization_digest(q)
checked_core_authorization_handle(q)
```

are allowed only if they are byte encodings/digests of `p_spend(q)` or
`p_transcript(q)`.

Required law:

```text
auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)
```

Additional completeness laws:

```text
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

and:

```text
p_spend(q).aggregation_mode
  =
p_context(q).aggregation_mode
  =
p_transcript(q).aggregation_mode
```

These may be direct fields or digest-bound compact encodings, but the semantic
equalities must be enforced by `O_gluing`/pullback checks.

Failure:

An independent opening commitment, slot authorization digest, or checked-core
handle that does not publicly factor through `p_spend(q)` is dead.

## `p_lifecycle(q)`

Semantic role:

Replay/nullifier/consume-once behavior.

Contents:

```text
p_lifecycle(q) =
  (
    slot_id,
    live_status_marker,
    lifecycle_anchor,
    nullifier_namespace,
    consume_once_rule,
    replay_domain,
    restore_or_reorg_scope
  )
```

Required laws:

```text
boundary_c(h)=boundary_c(h') => same p_lifecycle(q)
```

and:

```text
SpendAccept(c,q,sig,msg,state)
  => lifecycle behavior is determined by p_lifecycle(q)
```

Failure:

If private authorization-local or gauge material changes lifecycle behavior,
move that behavior into `p_lifecycle(q)` or mark `safety_saturation_break`.

## `p_transcript(q)`

Semantic role:

Challenge-binding surface for downstream spend and any residual account
certificate.

Contents:

```text
p_transcript(q) =
  (
    native_boundary_identifier,
    p_context(q),
    p_account(q),
    p_spend(q),
    p_lifecycle(q),
    p_state(q),
    aggregation_mode,
    transcript_domain_separator
  )
```

Optional byte handles:

```text
native_boundary_digest(q)
slot_authorization_digest(q)
creation_statement_digest(q)
transparent_transcript_digest(q)
```

are byte bindings only if their semantic preimage is this transcript surface or
another projection of `q`.

Required law:

```text
accepted challenge binds p_transcript(q)
```

Failure:

If a checked-core or Fiat-Shamir challenge binds an old digest whose semantics
do not factor through `p_transcript(q)`, export
`checked_core_frame_binding_break` or `domain_binding_or_qrom_break`.

## `p_state(q)`

Semantic role:

Consensus state transition authorized by the admitted account.

Contents:

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

Derived state transition class:

```text
p_state(q).state_transition_class =
  CanonicalStateTransitionClass(
    p_state(q),
    p_context(q).native_boundary_version,
    p_context(q).parameter_set_id,
    p_context(q).verifier_family_id,
    p_context(q).spend_verifier_domain,
    p_context(q).chain_or_application_domain,
    p_context(q).fork_or_epoch_scope,
    p_context(q).aggregation_mode
  )
```

Important:

```text
account_state_root_before
```

alone is not the state transition class. The class is the canonical,
domain-separated transition boundary containing predecessor state, admitted
boundary id, live-state requirement, successor/consumed class,
data-availability requirement, aggregation state boundary, and full verifier
context. In particular it binds aggregation mode, so direct-mode state classes
cannot be replayed as aggregate-compatible state classes.

Required laws:

```text
SpendAccept(c,q,sig,msg,state)
  => state transition is determined by p_state(q)
```

and:

```text
p_state(q) is compatible with p_lifecycle(q)
```

and:

```text
O_canonicality(q)=0 ∧ O_context(q)=0
  =>
exists! cls.
  cls = p_state(q).state_transition_class
```

Failure:

If spend can consume a different state transition than the one projected from
`q`, export `spend_state_closure_break`.

## Compatibility Field Mapping

| old/current name | active projection |
|---|---|
| `account_id` | `p_account(q)` |
| `public_key_identity` | `p_account(q)` and/or `p_spend(q)` |
| `slot_id` | `p_lifecycle(q)` |
| `account_state_root_before` | `p_state(q)` |
| `lifecycle_anchor` | `p_lifecycle(q)` |
| `live_status_marker` | `p_lifecycle(q)` |
| `slot_authorization_digest` | byte binding for `p_spend(q)` / `p_transcript(q)` |
| `native_boundary_digest` | byte binding for canonical `q` / `p_transcript(q)` |
| `creation_statement_digest` | byte binding or projection of `p_transcript(q)` |
| `opening_commitment` | dead unless typed restriction or authorization-local alias |
| `pad_seed_digest` | dead unless typed restriction or gauge/randomizer byte binding |
| checked-core challenge frame | derived from `p_transcript(q)` |
| checked-core verifier handle | `p_spend(q)` |
| allowed state transition class | `p_spend(q)` edge-linked to `p_state(q)` |
| aggregation mode | `p_spend(q)`, `p_context(q)`, and `p_transcript(q)` |

## Projection Completeness Theorem V1

Target:

```text
projection_completeness_v1:
  meaning_of_boundary(q) =
    (
      p_account(q),
      p_spend(q),
      p_lifecycle(q),
      p_transcript(q),
      p_state(q),
      p_context(q)
    )
```

and every public consensus field is either:

```text
p_i(q)
```

or a byte binding/data-availability handle for some `p_i(q)`.

## Immediate Consequence

The byte model should use these projection surfaces as the source of account
state and account-creation public bytes.

Do not count:

- private authorization-local material;
- private gauge/randomizer material;
- residual proof metadata;
- typed role restrictions;

as semantic account state unless a later theorem says they must be public.

## Next Required Work

Create:

```text
research/path2_native_byte_state_model_v1.md
```

to reset the byte/state model around projection surfaces rather than the old
6-16 KB proof-family envelope.
