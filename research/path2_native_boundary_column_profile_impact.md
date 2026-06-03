# Path 2 Native Boundary Column-Profile Impact

Status: next-blocker classification complete.

Purpose: decide whether the native boundary `b` removes the need for the
two-view `column_profile` theorem, or only narrows it.

## Result

The column-profile enum is still required, but its burden shrinks.

Before the native-boundary refactor, `column_profile` had to help prove that
these independently supplied public objects described the same hidden source:

- leaf context;
- creation proof binding;
- trace root;
- role handles;
- masked projection;
- transparent transcript.

After the native-boundary refactor, those are projections from one object:

```text
b = B_c(h)
```

The column profile no longer carries the whole cross-object consistency story.
It now has a narrower proof-system role:

```text
column_profile defines how p_section_root(b) is restricted into typed role
columns and how public query openings are classified for no-export.
```

## Remaining Column-Profile Duties

| duty | still needed? | reason |
|---|---:|---|
| role identifiers | yes | the proof must know which columns are pad source, projection relation, opening tuple, packed witness, blinding, range, and public metadata |
| role restriction rule | yes | role handles must be typed restrictions of `p_section_root(b)` under `p_role_profile(b)` |
| query visibility classes | yes | no-export/ZK depends on knowing which openings are public, masked, committed-hidden, blinding, range, or simulator-only |
| mask-domain labels | yes | fresh one-time masking requires row/column/query domain separation |
| duplicate/unknown role rejection | yes | prevents aliasing two roles under one handle or accepting a profile outside the theorem |
| leaf/context equality proof | no | context fields are projections of `b`, not a column-profile responsibility |
| proof-binding/trace-root equality proof | no | `creation_proof_binding_digest = native_boundary_digest` and `trace_commitment_root = p_section_root(b)` are boundary projections |
| role-handle cross-equality proof | mostly no | replaced by typed restriction under one parent section root; only binding of those restrictions remains |

## Revised Enum Scope

The concrete enum should be named something like:

```text
Path2NativeBoundaryColumnProfileV1
```

It should define:

1. `profile_id`;
2. `role_id`;
3. `column_set_id`;
4. `visibility_class`;
5. `mask_domain_label`;
6. `range_policy`;
7. `restriction_rule`;
8. `query_opening_policy`;
9. `rejection_policy`.

## Required Role IDs

Minimum role set:

| role id | meaning |
|---|---|
| `context_public` | public metadata/projection columns |
| `packed_witness` | packed residue / masked projection source |
| `randomizer` | account randomizer restriction |
| `pad_source` | pad source digits, pad lanes, pad masks |
| `opening_tuple` | opening tuple and account-binding columns |
| `projection_relation` | account/pad projection relation columns |
| `range_witness` | digit/range witnesses |
| `blinding` | commitment and ZK blinding columns |
| `constraint` | quotient/constraint trace columns |

## Required Visibility Classes

Minimum visibility set:

| class | public serialization |
|---|---|
| `public` | may be serialized directly |
| `derived_public` | may be serialized if recomputable from `b` and proof metadata |
| `masked_public` | may be opened only as value plus fresh one-time mask |
| `committed_hidden` | committed but not directly opened |
| `extractor_hidden` | extractor-only value |
| `range_private` | range/digit witness, never public except through ZK policy |
| `blinding_private` | blinding/noise/mask material |
| `simulator_only` | no real witness export in public proof |

## Rejection Rules

The verifier/proof theorem must reject or export a named break for:

- unknown profile id;
- unknown role id;
- duplicate role id for the same role namespace;
- role handle not authenticated under `p_section_root(b)`;
- role handle authenticated under a different `role_profile_digest`;
- public opening of `extractor_hidden`, `range_private`, or
  `blinding_private` classes;
- reused mask-domain label for safety-distinct cells;
- query opening whose visibility class is not allowed by the query policy;
- trace root or role profile not equal to the projection from `b`.

## Theorem Delta

Old blocker:

```text
column_profile must help establish same_parent_context across independently
named public objects.
```

New blocker:

```text
column_profile must instantiate typed restrictions and no-export policy for
one native boundary.
```

This is meaningfully smaller. The role-equality work moves into the boundary
projection theorem; the column profile only tells the transparent proof family
how to authenticate and query the projected section.

## Byte-Model Consequence

Do not update the byte floor yet.

The native boundary may reduce role-link equation overhead, but the concrete
payload size still depends on:

- whether typed role restrictions are supplied by the proof family for free;
- whether query openings are masked/simulated without extra commitment paths;
- how range/digit columns are represented;
- whether profile metadata is implicit by `profile_id` or serialized in the
  payload for data availability.

## Next Concrete Move

Define `Path2NativeBoundaryColumnProfileV1` as a theorem-level enum/table and
connect it to:

- `Descender.Path2NativeBoundary.TransparentViewProjectsFrom`;
- `Path2TransparentDescentPublicViewV1.column_profile`;
- role restriction authentication;
- no-export query-opening policy.

## Non-Claims

- This does not instantiate a transparent proof family.
- This does not prove no-export.
- This does not change byte counts.
- This does not remove range/norm obligations.
