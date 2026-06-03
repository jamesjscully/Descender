# Path 2 Native Boundary Column Profile V1

Status: theorem-level enum/table pinned; not implemented.

Purpose: define the column-profile vocabulary consumed by the native-boundary
two-view theorem. The profile tells the proof system how to interpret
`p_section_root(b)` as typed role restrictions and how to classify query
openings for no-export.

## Profile Object

```text
Path2NativeBoundaryColumnProfileV1 = (
  profile_id,
  role_table,
  visibility_table,
  mask_domain_table,
  range_policy_table,
  restriction_rule_table,
  query_opening_policy,
  rejection_policy
)
```

The transparent public view carries a compact `column_profile` identifier. The
theorem interprets that identifier as this table. Implementations may either
make the table fixed by version or carry authenticated profile metadata for data
availability.

## Role IDs

| role id | role |
|---|---|
| `context_public` | account/slot/verifier/lifecycle public projection columns |
| `packed_witness` | packed residue and masked projection source columns |
| `randomizer` | account randomizer restriction columns |
| `pad_source` | pad source digits, pad lanes, and mask derivation columns |
| `opening_tuple` | opening tuple and account-binding columns |
| `projection_relation` | account projection, pad projection, and active-lane relation columns |
| `range_witness` | digit/canonicality/range witness columns |
| `blinding` | commitment blinding, noise, and masking columns |
| `constraint` | quotient/constraint trace columns |

## Visibility Classes

| class | public opening rule |
|---|---|
| `public` | may be serialized directly |
| `derived_public` | may be serialized if recomputable from `b` and proof metadata |
| `masked_public` | may be queried only as a one-time masked value |
| `committed_hidden` | committed but not directly opened |
| `extractor_hidden` | extractor-only semantic value |
| `range_private` | private digit/range witness |
| `blinding_private` | private blinding/noise/mask material |
| `simulator_only` | public proof contains only simulated transcript material |

## Restriction Rules

Every role handle in `p_role_handles(b)` must satisfy:

```text
role_handle_i =
  Restrict(
    parent_root := p_section_root(b),
    profile_id := p_role_profile(b),
    role_id := i,
    column_set_id := profile.column_set(i)
  )
```

If a role handle is authenticated under a different parent root, different
profile id, duplicate role id, or unknown role id, the verifier rejects or the
theorem exports `inner_commitment_binding_break` /
`boundary_binding_break`.

## Query Opening Policy

Allowed:

- `public` direct openings;
- `derived_public` openings with recomputation from `b`;
- `masked_public` openings only with fresh mask domain labels;
- authentication paths and consistency messages that reveal no private class.

Forbidden:

- direct openings of `committed_hidden`;
- direct openings of `extractor_hidden`;
- direct openings of `range_private`;
- direct openings of `blinding_private`;
- any query using a reused mask domain for safety-distinct cells;
- any query whose label is absent from the profile table.

## Mask Domain Rule

Every `masked_public` opening is labeled by:

```text
DESCENDER_PATH2_NATIVE_BOUNDARY_MASK_V1
|| native_boundary_digest
|| profile_id
|| role_id
|| column_set_id
|| row_index
|| query_index
```

The theorem requires uniqueness of the tuple above for every independent mask.
Reused labels export `live_slot_leakage_or_keygen_break` or
`qrom_or_domain_separation_break`, depending on whether the failure is leakage
or transcript-domain collision.

## Required Negative Controls

- unknown profile id rejects;
- unknown role id rejects;
- duplicate role id rejects;
- role handle under the wrong `parent_section_root` rejects;
- role handle under the wrong `role_profile_digest` rejects;
- `extractor_hidden` direct opening rejects/no-exports;
- `range_private` direct opening rejects/no-exports;
- `blinding_private` direct opening rejects/no-exports;
- reused mask-domain label rejects or exports a leakage/domain break;
- `column_profile` mismatch with `p_role_profile(b)` rejects.

## Selected Norm/Range Mode

The active profile must use:

```text
hybridBase128HiddenRoles
```

This means:

- `masked_projection[6]` is a raw canonical q12289 public quotient;
- `pad_source`, `projection_relation`, and `opening_tuple` hidden role values
  are represented with base-128 digit columns;
- digit/range witnesses are private under the no-export policy;
- reconstruction constraints map base-128 digits to q12289 semantic residues;
- SIS binding uses digit-coordinate bounds.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now names:

- `RoleId`
- `VisibilityClass`
- `ColumnProfileEntry`
- `ColumnProfile`
- `ColumnProfileCompatibleWithBoundary`
- `columnProfileCompatible_profile_eq`

These Lean objects are structural only. They do not instantiate a concrete
proof family or prove no-export.

## Next Proof Move

Define the role-restriction authentication assumption:

```text
RoleRestrictionSoundness:
accepted proof + compatible profile
  =>
each p_role_handles(b).role_i authenticates exactly the role_i columns under
p_section_root(b), or exports inner_commitment_binding_break /
boundary_binding_break.
```

This is the next real cryptographic interface needed before byte-model updates.

## Non-Claims

- This does not choose a concrete Merkle/FRI/STARK layout.
- This does not prove zero knowledge.
- This does not prove vector-commitment binding.
- This does not update byte counts.
