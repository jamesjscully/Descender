# Path 2 Native Boundary Parent-Root Role Binding

Status: theorem interface pinned; not proved.

Purpose: isolate the first blocking gate in
`NativeBoundaryZkTraceBindingAndHiding`: one public parent section root must
authenticate private typed role restrictions without publishing full-rank role
commitments.

## Target Claim

```text
ParentRootRoleBindingSoundness:

TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
  =>
ParentRootBindsPrivateRoles(b, profile)
```

or exports:

- `transparentProofSoundnessBreak`;
- `inner_commitment_binding_break`;
- `boundary_binding_break`;
- `qrom_or_domain_separation_break`.

## Meaning

`ParentRootBindsPrivateRoles(b, profile)` means:

- `p_section_root(b)` authenticates the column sets named by `profile`;
- each private role handle in `p_role_handles(b)` is a typed restriction under
  that parent root;
- the role restrictions are private/proof-system objects, not standalone public
  linear commitments;
- two safety-distinct role vectors under the same handle export
  `inner_commitment_binding_break`;
- a role handle cannot be moved to another role id, column set, profile, trace
  root, account, slot, or native boundary.

## Role Coverage

Required roles:

- `packed_witness`;
- `randomizer`;
- `pad_source`;
- `opening_tuple`;
- `projection_relation`;
- range/blinding/constraint roles when referenced by the profile.

## Rejection Rules

Reject or export a named break for:

- unknown role id;
- duplicate role id;
- wrong parent root;
- wrong column profile;
- role handle not authenticated under `p_section_root(b)`;
- role handle authenticated under a different native boundary digest;
- role restriction whose leaf/opening serializes raw hidden values.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now provides:

- `Descender.Path2NativeBoundary.Predicates.parentRootBindsPrivateRoles`
- `Descender.Path2NativeBoundary.ParentRootRoleBindingSoundness`
- `Descender.Path2NativeBoundary.parent_root_binds_private_roles_or_break`

The composed wrappers now include parent-root binding as a separate conclusion.

## Next Proof Move

Define the concrete private role-restriction object:

```text
PrivateRoleRestrictionV1 = (
  parent_section_root,
  native_boundary_digest,
  role_id,
  column_set_id,
  role_handle,
  authentication_path_or_opening_policy
)
```

and specify whether the authentication path is public, masked, or simulator
only.

## Non-Claims

- This does not instantiate the commitment tree/IOP.
- This does not prove role handle hiding.
- This does not update byte counts.
