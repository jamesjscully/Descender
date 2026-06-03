# Path 2 Native Boundary ZK Trace QROM Schedule

Status: transcript schedule pinned; QROM budget not instantiated.

Purpose: define the Fiat-Shamir/QROM binding order for
`NativeBoundaryZkTraceBindingAndHiding`.

## Rule

No query index, challenge, mask-opening request, or consistency check may be
derived before all native-boundary public commitments and role authentication
modes are bound.

## Transcript Order

```text
T0 = domain_separator
T1 = proof_system_id
T2 = relation_id
T3 = native_boundary_digest
T4 = leaf_context_digest
T5 = parent_section_root
T6 = role_profile_digest / column_profile
T7 = role_handle_table
T8 = role_authentication_mode_table
T9 = matrix_domain_transcript_digest
T10 = aggregation_context_digest
T11 = trace_shape
T12 = constraint_commitment_root
T13 = masked_query_commitment_roots
T14 = no_export_policy_id
T15 = norm_range_mode
T16 = blinding_noise_policy_id
Q = H(T0 || ... || T16)
```

Only after `Q` is fixed may the verifier/proof derive:

- query indices;
- challenge scalars;
- mask opening domains;
- consistency check selectors;
- range-check challenge material.

## Must Bind

The transcript must bind:

- `native_boundary_digest`;
- all projections used by `TransparentViewProjectsFrom`;
- `p_section_root(b)`;
- `p_role_profile(b)`;
- every `PrivateRoleRestrictionV1.role_handle`;
- every `PrivateRoleRestrictionV1.authentication_mode`;
- matrix-domain transcript;
- aggregation context;
- selected `hybridBase128HiddenRoles` mode;
- blinding/noise policy id;
- no-export/query policy id.

## Forbidden Ordering

Reject or export `qrom_or_domain_separation_break` if:

- query indices are sampled before role handles are fixed;
- query indices are sampled before authentication modes are fixed;
- mask domains omit native boundary digest, role id, column set, row, query, or
  transcript digest;
- role handles can be changed after seeing query indices;
- column profile can be changed after seeing query indices;
- proof-system id or relation id can be rebound after query derivation;
- aggregation context can be changed after query derivation.

## Mask Domain

For masked openings:

```text
mask_domain =
  DESCENDER_PATH2_NATIVE_BOUNDARY_MASK_V1
  || native_boundary_digest
  || proof_system_id
  || relation_id
  || role_profile_digest
  || role_id
  || column_set_id
  || row_index
  || query_index
  || transparent_transcript_digest
```

Reuse of the full tuple is allowed only for the same masked cell in the same
query context. Any reuse across safety-distinct hidden values exports
`live_slot_leakage_or_keygen_break` or `qrom_or_domain_separation_break`.

## QROM Ledger Inputs

The final QROM ledger must count at least:

- transcript hash for public binding;
- query-index derivation;
- per-query mask-domain derivations;
- per-query authentication-path challenge derivations;
- range-check challenge derivations;
- simulator challenge programming terms;
- extractor rewinding/forking terms if used.

## Current Status

This schedule closes the theorem-order ambiguity. It does not instantiate:

- concrete query count;
- QROM loss;
- proof-system challenge space;
- simulator programming bound.

## Next Move

Update `path2_hash_qrom_assumption_ledger.md` with this schedule and create a
native-boundary QROM row using symbolic query counts.

## Non-Claims

- This does not prove QROM soundness.
- This does not choose query counts.
- This does not implement transcript framing in Rust.
