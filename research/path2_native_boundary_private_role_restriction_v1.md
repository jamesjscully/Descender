# Path 2 Native Boundary PrivateRoleRestrictionV1

Status: theorem object pinned; active profile auth-mode gate implemented.

Purpose: define the concrete object needed after parent-root binding: a private
role restriction authenticated under `p_section_root(b)` without exposing raw
private role vectors.

## Object

```text
PrivateRoleRestrictionV1 = (
  native_boundary_digest,
  parent_section_root,
  role_profile_digest,
  role_id,
  column_set_id,
  role_handle,
  visibility_class,
  authentication_mode,
  query_policy_id
)
```

## Field Meaning

| field | public? | meaning |
|---|---:|---|
| `native_boundary_digest` | yes | binds restriction to one `b` |
| `parent_section_root` | yes | root projected from `b` |
| `role_profile_digest` | yes | profile projected from `b` |
| `role_id` | yes | typed role namespace |
| `column_set_id` | yes | fixed column set under the profile |
| `role_handle` | yes | opaque handle to the private role restriction |
| `visibility_class` | yes | public/masked/hidden/range/blinding classification |
| `authentication_mode` | yes | public path, masked path, simulator-only, or extractor-only |
| `query_policy_id` | yes | allowed query/opening behavior |

The role vector itself is not a field. It is extractor-private.

## Authentication Modes

| mode | public material | allowed for private roles? | notes |
|---|---|---:|---|
| `public_path_to_masked_leaf` | authentication path to masked/simulator-safe leaves | yes | active mode for query-visible masked columns |
| `simulator_only_path` | simulated path/consistency messages | yes | active mode when proof family supplies simulator theorem |
| `extractor_only_opening` | no public opening; extractor recovers role vector | yes | active mode for committed-hidden columns |
| `public_path_to_raw_leaf` | authentication path to raw hidden leaf | no | dead path for private roles |
| `public_linear_commitment` | standalone `C=A*v` public role commitment | no for current dimensions | rejected by rank/hiding probe |

The Lean active-boundary interface now mirrors this rule through
`RoleAuthenticationMode`, `ColumnProfileEntry.authenticationMode`, and
`ColumnProfilePrivateAuthModesSafe`.  `ColumnProfileCompatibleWithBoundary`
requires that every private/hidden/simulator-only entry uses only
`publicPathToMaskedLeaf`, `simulatorOnlyPath`, or `extractorOnlyOpening`; raw
hidden leaves and public linear commitments cannot discharge the active
theorem gate.

The Rust active two-view column profile also carries `authentication_mode` in
`Path2TwoViewColumnProfileEntryV1`, includes it in
`path2_two_view_column_profile_v1_digest`, and rejects
`public_path_to_raw_leaf`, `public_linear_commitment`, and unknown modes in the
profile verifier.

## Required Role Instances

| role | visibility | authentication mode |
|---|---|---|
| `pad_source` | `range_private` / `committed_hidden` plus masked public views | `extractor_only_opening` or `public_path_to_masked_leaf` |
| `projection_relation` | `range_private` / `committed_hidden` plus masked public views | `extractor_only_opening` or `public_path_to_masked_leaf` |
| `opening_tuple` | `range_private` / `committed_hidden` plus masked public views | `extractor_only_opening` or `public_path_to_masked_leaf` |
| `packed_witness` | public masked projection plus hidden packed link | mixed public/masked/extractor |
| `randomizer` | committed hidden | extractor-only or simulator-only |

## Soundness Requirement

For every accepted proof:

```text
PrivateRoleRestrictionV1.Authenticates(b, role)
```

must imply the role vector used by the hidden extractor view is the same vector
named by the role handle, or the proof exports:

- `inner_commitment_binding_break`;
- `boundary_binding_break`;
- `transparentProofSoundnessBreak`.

## Hiding Requirement

For private roles, public authentication material must be simulatable from:

```text
b,
profile,
query schedule,
masked_projection,
proof metadata
```

without the raw role vector.

If authentication requires a public raw leaf or public full-rank linear role
commitment, the candidate fails.

## Next Proof Move

Instantiate the concrete transparent proof-family verifier so these allowed
authentication modes correspond to actual parent-root role subcommitment,
simulator, and extractor behavior, not just profile metadata.

## Non-Claims

- This does not implement the authentication path.
- This does not choose Merkle/FRI/STARK internals.
- This does not prove simulator security.
- This does not update byte counts.
