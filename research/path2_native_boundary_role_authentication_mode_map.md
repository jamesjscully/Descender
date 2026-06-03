# Path 2 Native Boundary Role Authentication Mode Map

Status: role authentication mode map pinned.

Purpose: map each native-boundary column-profile role to the allowed
`PrivateRoleRestrictionV1` authentication modes.

## Mode Legend

| mode | meaning |
|---|---|
| `public_path_to_masked_leaf` | public authentication path opens only masked/simulator-safe leaf values |
| `simulator_only_path` | public transcript contains simulated path/consistency material |
| `extractor_only_opening` | no public opening; extractor obtains hidden role columns |
| `public_path_to_raw_leaf` | forbidden for private roles |
| `public_linear_commitment` | forbidden for current private-role dimensions |

## Role Map

| role | required modes | forbidden modes | reason |
|---|---|---|---|
| `context_public` | `public_path_to_masked_leaf` or direct public projection | none for public fields | context is public projection of `b` |
| `packed_witness` | `public_path_to_masked_leaf` for `masked_projection`; `extractor_only_opening` for packed hidden link | `public_path_to_raw_leaf`, `public_linear_commitment` | public quotient is allowed; hidden packed link is not |
| `randomizer` | `extractor_only_opening` or `simulator_only_path` | `public_path_to_raw_leaf`, `public_linear_commitment` | randomizer is hidden representative material |
| `pad_source` | `extractor_only_opening` plus `public_path_to_masked_leaf` for masked query views | `public_path_to_raw_leaf`, `public_linear_commitment` | pad digits and pad seed must not be exported |
| `opening_tuple` | `extractor_only_opening` plus `simulator_only_path` for account-binding auxiliaries | `public_path_to_raw_leaf`, `public_linear_commitment` | opening tuple contains account/lifecycle witness material |
| `projection_relation` | `extractor_only_opening` plus `public_path_to_masked_leaf` for masked relation checks | `public_path_to_raw_leaf`, `public_linear_commitment` | account projection lanes are private |
| `range_witness` | `simulator_only_path` or `extractor_only_opening` | `public_path_to_raw_leaf`, `public_linear_commitment` | range witnesses are denied public exports |
| `blinding` | `simulator_only_path` or `extractor_only_opening` | `public_path_to_raw_leaf`, `public_linear_commitment` | blinding/noise/masks are private |
| `constraint` | `public_path_to_masked_leaf` only if constraint leaves are public/simulated; otherwise `simulator_only_path` | raw private constraints | quotient/constraint trace must not leak hidden columns |

## Consequence

The active proof family must support at least three authentication modes:

1. public path to masked/simulator-safe leaves;
2. simulator-only path/consistency material;
3. extractor-only hidden role opening.

Any concrete proof family that only supports public raw leaf openings fails the
native-boundary no-export theorem.

## Break Mapping

| failure | break |
|---|---|
| role uses forbidden raw public opening | `live_slot_leakage_or_keygen_break` |
| role handle not under parent root/profile | `boundary_binding_break` |
| same role handle authenticates two private vectors | `inner_commitment_binding_break` |
| hidden extractor role differs from authenticated role | `transparentProofSoundnessBreak` or `inner_commitment_binding_break` |
| simulator cannot produce public auth material | `live_slot_leakage_or_keygen_break` |

## Next Move

Define the concrete transcript order for these modes:

```text
native_boundary_digest
parent_section_root
role_profile_digest
role handles
matrix transcript
masked query commitments
query derivation
query openings / simulator messages
```

This is the QROM/query-schedule blocker.

## Non-Claims

- This does not implement authentication.
- This does not prove simulator security.
- This does not update byte counts.
