# Path 2 Native Boundary Projection Audit

Status: first projection audit for the native `q` design.

Purpose: classify current Path 2 / native-boundary public fields under the new
hybrid model:

```text
q : Q_c
obs_c(q)=0
SpendAccept(c,q,sig,msg,state)
H_c(q) = LiftCore × AuthorizationLocal × Gauge × Randomizer
```

Each current field must be one of:

- projection of `q`;
- byte/data-availability binding for `q`;
- private authorization-local;
- pure gauge/randomizer;
- dead independent semantic handle.

## Classification Legend

`projection_of_q`
: The field is semantic and must be computed from the native public boundary.

`byte_binding_for_q`
: The field binds bytes, encodings, or data availability for `q`, but does not
  define independent account meaning.

`authorization_local`
: Private signer-held material used to produce spend evidence over
  `p_spend(q)`.

`gauge_or_randomizer`
: Private representation/hiding material that must not affect public meaning.

`typed_restriction_of_q`
: A public/proof handle allowed only as a typed restriction of one native
  boundary/root, not as an independent semantic commitment.

`dead_independent_handle`
: Invalid unless redesigned to factor through `q`.

## Top-Level Native Boundary Fields

| current field | new classification | required interpretation |
|---|---|---|
| `boundary_version` | `projection_of_q` | native boundary version in `p_context(q)` |
| `profile_version_digest` | `projection_of_q` | profile/parameter context |
| `verifier_version_digest` | `projection_of_q` | verifier/domain context |
| `account_id` | `projection_of_q` | account namespace component of `M_c` |
| `public_key_identity` | `projection_of_q` | part of `p_spend(q)` or account namespace |
| `slot_id` | `projection_of_q` | lifecycle/nullifier identity |
| `account_state_root_before` | `projection_of_q` | predecessor state boundary |
| `lifecycle_anchor` | `projection_of_q` | lifecycle/reorg/restore namespace |
| `aggregation_context_digest` | `projection_of_q` | context/transcript projection; direct-mode fixed value acceptable |
| `native_boundary_digest` | `byte_binding_for_q` | digest of canonical encoding of `q`; not independent semantics |

## Section/Role Fields

| current field | new classification | required interpretation |
|---|---|---|
| `parent_section_root` | `typed_restriction_of_q` or `byte_binding_for_q` | root for one section/lift family over `q`; not a separate account object |
| `role_profile_digest` | `projection_of_q` | typed role layout/projection policy for `q` |
| `packed_witness_handle` | `typed_restriction_of_q` | allowed only as restriction under `parent_section_root` and `role_profile_digest` |
| `randomizer_handle` | `typed_restriction_of_q` | proof/hiding handle only; semantic class must be in `q` |
| `pad_source_handle` | `typed_restriction_of_q` or `gauge_or_randomizer` | dead if interpreted as independent public commitment to pad source |
| `opening_tuple_handle` | `typed_restriction_of_q` or `authorization_local` | dead if independent semantic opening commitment |
| `projection_relation_handle` | `typed_restriction_of_q` | relation/proof handle under one native boundary |

Rule:

```text
role_handle_i = Restrict(parent_section_root, role_profile_digest, role_i)
```

or the role handle is dead for the compact native path.

## Public Projection And Meaning Fields

| current field | new classification | required interpretation |
|---|---|---|
| `masked_projection[6]` | `projection_of_q` | public quotient/check face, not hidden witness fragment |
| `live_status_marker` | `projection_of_q` | lifecycle state from `p_lifecycle(q)` |
| `creation_statement_digest` | `byte_binding_for_q` or `projection_of_q` | semantic only if it is canonical statement surface of `q`; otherwise byte binding |
| `creation_proof_binding_digest` | `byte_binding_for_q` | should equal/bind `native_boundary_digest` or proof bytes for `q` |
| `leaf_context_digest` | `byte_binding_for_q` | digest of `p_context(q)` / leaf context encoding |
| `slot_leaf_digest` | `byte_binding_for_q` | data-availability/canonical leaf binding for `q` |

## Spend-Facing Fields

| current/older field | new classification | required interpretation |
|---|---|---|
| `slot_authorization_digest` | `projection_of_q` or `byte_binding_for_q` | valid only if its semantic meaning is `p_spend(q)` / `p_transcript(q)` |
| `opening_commitment` | `typed_restriction_of_q` or `authorization_local` | dead as independent semantic handle |
| `pad_seed_digest` | `typed_restriction_of_q` or `gauge_or_randomizer` | dead if spend semantics depend on it independently |
| checked-core public key / verifier handle | `projection_of_q` | part of `p_spend(q)` |
| checked-core challenge frame | `projection_of_q` | must be derived from `p_transcript(q)` |
| checked-core response/syndrome rows | `authorization_local` / signature evidence | downstream spend proof data, not account boundary |
| checked-core signature bytes | downstream spend proof | not part of account creation `q` |

Spend closure requirement:

```text
SpendAccept(c,q,sig,msg,state)
```

must consume these only through:

```text
p_spend(q), p_lifecycle(q), p_transcript(q), p_state(q)
```

## Transparent / Proof-Family Fields

| field | new classification | required interpretation |
|---|---|---|
| `trace_commitment_root` | `typed_restriction_of_q` / proof metadata | only valid if it authenticates restrictions for the admitted `q` |
| `column_profile` | `projection_of_q` or proof metadata | must match role/profile projection policy of `q` |
| `matrix_domain_transcript` | `byte_binding_for_q` / proof metadata | parameter/domain binding only |
| `constraint_root` | proof metadata | not semantic unless projected from `q` |
| `query_index` | proof metadata/randomness | not semantic |
| `transparent_transcript_digest` | `byte_binding_for_q` | proof transcript binding, not account meaning |
| public query openings | proof evidence | must obey no-export; dead if raw private representative coordinates |

The native route should minimize reliance on these fields. If a residual proof
remains, its semantic inputs must be projections of `q`.

## Private Fiber Fields

| field family | new classification | rule |
|---|---|---|
| private pad source | `gauge_or_randomizer` | must not affect `M_c` |
| private mask source | `gauge_or_randomizer` | must not affect `M_c` |
| blinding/noise | `gauge_or_randomizer` | hiding only |
| opening-local coordinates | `authorization_local` unless proven gauge | must map to `p_spend(q)` |
| local section coordinates | `LiftCore_c(q)` plus gauge | public semantic projection fixed by `q` |
| randomizer state | `gauge_or_randomizer` | no public semantic effect |

## Dead Or Conditional Paths

### Dead: Independent Opening Commitment

Dead shape:

```text
q contains or references opening_commitment
SpendAccept consumes opening_commitment
no public theorem opening_commitment = p_spend(q)
```

Reason:

This gives spend a semantic handle outside `q`.

### Dead: Independent Pad Digest

Dead shape:

```text
pad_seed_digest
```

used as semantic evidence rather than typed restriction/gauge/randomizer
binding.

Reason:

Pad source is private hiding material unless explicitly part of public
semantics. Making it semantic recreates leakage/proof burden.

### Dead: Role Handles As SHA Preimage Commitments

Dead shape:

```text
role_handle = SHA(hidden_role_bytes)
```

with compact relation expected to prove preimage semantics.

Reason:

This is neither native nor compact; role handles must be typed restrictions of
one boundary/root or moved out of the semantic surface.

### Conditional: `parent_section_root`

Allowed only if one of:

1. it is a byte/data-availability root for a public/canonical lift surface;
2. it authenticates typed restrictions under a theorem tied to `q`;
3. it is proof metadata for a residual proof whose semantic inputs factor
   through `q`.

Dead if it becomes an independent semantic account root.

## Projection Completeness Gaps

Open audit items:

1. Define exact `p_spend(q)` encoding.
2. Define exact `p_transcript(q)` challenge frame.
3. Define exact `p_lifecycle(q)` consume-once/nullifier frame.
4. Define exact `p_state(q)` predecessor/successor boundary.
5. Decide whether `parent_section_root` is needed in the native route or only
   in residual proof/fallback paths.
6. Decide whether compatibility names `opening_commitment` and
   `pad_seed_digest` remain in leaf bytes as aliases for projections.

## Current Assessment

The projection audit supports the hybrid native route, with one major
condition:

```text
all semantic handles must be projected from q.
```

The old field names can remain for compatibility only if their semantics are
rewritten as projections of `q` or byte bindings for `q`.

The highest-risk fields are:

```text
opening_commitment
pad_seed_digest
slot_authorization_digest
parent_section_root
role handles
```

because each can accidentally become an independent semantic handle.

## Next Required Work

Create:

```text
research/path2_active_native_account_design_v1.md
```

to consolidate the selected hybrid into one active design:

```text
Q_c
M_c
O_c
obs_c
H_c(q)
G_c(q)
p_spend
p_lifecycle
p_transcript
p_state
named break surface
```

This should be the handoff point before adding a Lean theorem-interface module.

