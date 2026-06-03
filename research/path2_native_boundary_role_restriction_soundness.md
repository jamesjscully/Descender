# Path 2 Native Boundary Role-Restriction Soundness

Status: theorem interface pinned; not proved.

Purpose: state the compactness-critical assumption that lets role handles remain
small projections of one native boundary instead of expanding into explicit
cross-role equality proofs.

## Target Claim

```text
RoleRestrictionSoundness:

accepted transparent proof
compatible native boundary b
compatible column profile
  =>
each role handle in p_role_handles(b) authenticates exactly the intended
role columns under p_section_root(b) and p_role_profile(b)
```

or the verifier exports:

- `inner_commitment_binding_break`;
- `boundary_binding_break`;
- `descent_soundness_break`;
- `transparentProofSoundnessBreak`;
- `qrom_or_domain_separation_break`.

## Role Handles

The theorem covers:

| boundary projection | role id | required interpretation |
|---|---|---|
| `packed_witness_handle` | `packed_witness` | typed restriction for packed residue / masked projection source columns |
| `randomizer_handle` | `randomizer` | typed restriction for account-randomizer columns |
| `pad_source_handle` | `pad_source` | typed restriction for pad source, pad lanes, and mask-source columns |
| `opening_tuple_handle` | `opening_tuple` | typed restriction for opening tuple and account-binding columns |
| `projection_relation_handle` | `projection_relation` | typed restriction for active-lane projection relation columns |

## Formal Shape

For boundary `b`, transparent public view `v`, and column profile `prof`:

```text
TransparentAccepts(b, v)
TransparentViewProjectsFrom(b, v)
ColumnProfileCompatibleWithBoundary(b, v, prof)
  =>
RoleRestrictionsAuthenticate(b, prof)
```

or a named break.

## What This Proves

It proves the public role handles are not independent commitments. They are
authenticated restrictions of:

```text
p_section_root(b)
```

under:

```text
p_role_profile(b)
```

This closes the theorem-level gap that would otherwise require explicit
equality proofs among:

- pad-source witness;
- opening tuple;
- packed residues;
- projection relation;
- trace witness.

## What This Does Not Prove

This does not prove:

- the hidden relation rows are valid;
- query openings are zero-knowledge;
- norm/range constraints are satisfied;
- the transparent proof family is sound;
- the vector commitment is post-quantum secure.

Those remain separate theorem rows. This theorem only states that all role
columns named by the accepted proof are restrictions of one parent native
boundary section.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now provides:

- `Descender.Path2NativeBoundary.Predicates.roleRestrictionsAuthenticate`
- `Descender.Path2NativeBoundary.RoleRestrictionSoundness`
- `Descender.Path2NativeBoundary.role_restrictions_authenticate_or_break`
- `Descender.Path2NativeBoundary.transparent_view_authenticates_roles_and_extracts_or_break`

These are interface-level definitions. A concrete proof family must instantiate
the predicate.

## Composition Status

`RoleRestrictionSoundness` is now integrated with the selected full-trace
extraction theorem at the Lean interface level:

```text
RoleRestrictionSoundness
+ ParentTraceExtractionSoundness
+ TraceAssemblySoundness
  =>
accepted transparent proof extracts h in Fiber(B_c, b)
whose role components are authenticated restrictions of p_section_root(b)
```

After this interface point, the remaining high-risk proof rows are no-export
and norm/range.

## Non-Claims

- This does not update byte counts.
- This does not instantiate the column profile in Rust.
- This does not prove hiding or no-export.
