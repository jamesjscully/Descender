# Path 2 Native Boundary V1 Interface

Status: theorem interface draft; not implemented.

Purpose: define the concrete public boundary object `b` that the account
creation theorem should admit. This interface turns the quotient plan

```text
Boundary_c := H_c / ~safe
b := B_c(h)
```

into a deployment-facing digest and projection surface.

## Selected Boundary Shape

`Path2NativeBoundaryV1` is the public encoded representative of `B_c(h)`:

```text
Path2NativeBoundaryV1 = (
  boundary_version,
  context_projection,
  parent_section_root,
  role_profile_digest,
  role_restriction_handles,
  public_projection_quotient,
  lifecycle_authorization_projection,
  creation_statement_projection,
  aggregation_context_digest
)
```

The digest is:

```text
native_boundary_digest =
  H(DESCENDER_PATH2_NATIVE_BOUNDARY_V1 || encode(Path2NativeBoundaryV1))
```

This digest is the semantic boundary handle. SHA-256 remains byte binding; the
theorem meaning comes from the statement that the encoded object is in the image
of `B_c`.

## Fields

| field | source | meaning |
|---|---|---|
| `boundary_version` | fixed constant | separates this native-boundary model from older proof-binding frames |
| `profile_version_digest` | context `c` | profile namespace |
| `verifier_version_digest` | context `c` | creation/spend verifier namespace |
| `account_id` | context `c` | account namespace |
| `public_key_identity` | context `c` | key identity authorized by the account |
| `slot_id` | context `c` | prevalidated slot identity |
| `account_state_root_before` | context `c` | admission-state namespace |
| `lifecycle_anchor` | context `c` | restore/reorg/lifecycle namespace |
| `parent_section_root` | `p_section_root(B_c(h))` | commitment root for one descent section |
| `role_profile_digest` | `p_role_profile(B_c(h))` | typed role/column layout |
| `packed_witness_handle` | `p_packed(B_c(h))` | restriction handle for packed residues |
| `randomizer_handle` | `p_randomizer(B_c(h))` | restriction handle for account randomizer |
| `pad_source_handle` | `p_pad(B_c(h))` | restriction handle for pad source |
| `opening_tuple_handle` | `p_opening(B_c(h))` | restriction handle for opening tuple |
| `projection_relation_handle` | `p_projection_relation(B_c(h))` | restriction handle for active-lane relation |
| `masked_projection[6]` | `p_public_projection(B_c(h))` | public quotient target |
| `live_status_marker` | `p_lifecycle(B_c(h))` | admitted-live state marker |
| `creation_statement_digest` | `p_statement(B_c(h))` | canonical creation statement, not spend-dependent |
| `aggregation_context_digest` | context `c` | direct-mode fixed now, aggregation-compatible later |

## Projection Functions

The boundary interface exports these projections:

```text
p_context(b) =
  (
    profile_version_digest,
    verifier_version_digest,
    account_id,
    public_key_identity,
    slot_id,
    account_state_root_before,
    lifecycle_anchor,
    aggregation_context_digest
  )

p_section_root(b) = parent_section_root

p_role_profile(b) = role_profile_digest

p_role_handles(b) =
  (
    packed_witness_handle,
    randomizer_handle,
    pad_source_handle,
    opening_tuple_handle,
    projection_relation_handle
  )

p_public_projection(b) = masked_projection[6]

p_lifecycle(b) = live_status_marker

p_statement(b) = creation_statement_digest
```

Spend-facing projection:

```text
p_slot_leaf(b) =
  Path2PrevalidatedSlotLeafNative(
    p_context(b),
    opening_commitment := opening_tuple_handle,
    pad_seed_digest := pad_source_handle_or_digest_projection,
    hidden_source_statement_digest := creation_statement_digest,
    creation_proof_binding_digest := native_boundary_digest,
    live_status_marker
  )
```

This preserves the current Path 2 V2 byte intuition: the leaf carries one
additional 32-byte native boundary digest. The existing `opening_commitment` and
`pad_seed_digest` names may remain as compatibility projections only if their
semantics are defined as boundary projections. They must not be independent
commitments with their own hidden witnesses.

Transparent proof projection:

```text
p_transparent_public_view(b, proof_shape) =
  Path2TransparentDescentPublicViewV1(
    leaf_context_digest := H(p_slot_leaf_context(b)),
    creation_proof_binding_digest := native_boundary_digest,
    trace_commitment_root := parent_section_root,
    column_profile := role_profile_digest/profile id,
    masked_projection := p_public_projection(b),
    aggregation_context_digest := p_context(b).aggregation_context_digest,
    ...
  )
```

The ellipsis is proof-family metadata: proof system id, relation id,
trace rows, query count, matrix domain transcript, constraint root, query
index, and transparent transcript digest.

## Native Role Restriction Rule

The key rule is:

```text
role_handle_i = Restrict(parent_section_root, role_profile_digest, role_i)
```

for:

- `packed_witness`;
- `randomizer`;
- `pad_source`;
- `opening_tuple`;
- `projection_relation`.

The theorem must not interpret these handles as unrelated SHA preimage
commitments. They are typed restrictions/subcommitments of one parent section.

If a proof system cannot supply this rule, the compact native-boundary path
falls back to explicit algebraic opening rows or is marked dead for compact
opening.

## Image And Kernel Conditions

Admission accepts only if:

```text
b in Im(B_c)
```

meaning there exists a hidden representative `h : H_c` such that:

```text
B_c(h) = b
LocalValid_c(h)
DescentConsistent_c(h)
```

Binding condition:

```text
B_c(h) = B_c(h') =>
  h ~safe h'
  or boundary_binding_break
  or descent_soundness_break
```

In proof-system terms, a double opening to the same boundary root exports one of
the named binding breaks rather than producing two safety-distinct account
states.

## Account Admission Theorem Interface

Target:

```text
native_boundary_account_admission_soundness_or_break:

AcceptAccountCreateNative(c, b, proof)
  =>
exists h : H_c.
  LocalValid_c(h)
  DescentConsistent_c(h)
  B_c(h) = b
```

or one of:

- `boundary_binding_break`;
- `descent_soundness_break`;
- `inner_commitment_binding_break`;
- `transparentProofSoundnessBreak`;
- `path2_sis_norm_or_range_break`;
- `live_slot_leakage_or_keygen_break`;
- `qrom_or_domain_separation_break`;
- `prevalidation_data_availability_break`.

## Spend Handoff Interface

Target:

```text
native_boundary_spend_handoff_or_break:

Admitted(c, b)
SpendAccept(c, p_slot_authorization(b), sig, msg, state)
  =>
exists h in Fiber(B_c, b).
  SpendWitnessValid_c(h, sig, msg, state)
```

or one of:

- `checked_core_message_binding_break`;
- `slot_replay_or_rollback_break`;
- `prevalidation_leaf_binding_break`;
- `boundary_binding_break`;
- `qrom_or_domain_separation_break`.

The spend verifier does not need to reopen `h`; it consumes the authorization
projection of the admitted boundary.

## Lean Interface Anchors

The proof-side interface is now mirrored in
`Descender/Path2NativeBoundary.lean`.

Lean objects:

- `Descender.Path2NativeBoundary.Context`
- `Descender.Path2NativeBoundary.RoleHandles`
- `Descender.Path2NativeBoundary.Boundary`
- `Descender.Path2NativeBoundary.HiddenRepresentative`
- `Descender.Path2NativeBoundary.TransparentPublicView`
- `Descender.Path2NativeBoundary.SlotAuthorization`
- `Descender.Path2NativeBoundary.BreakEvent`
- `Descender.Path2NativeBoundary.NormRangeMode`
- `Descender.Path2NativeBoundary.BlindingNoisePolicy`
- `Descender.Path2NativeBoundary.selectedNormRangeMode`
- `Descender.Path2NativeBoundary.UsesSelectedNormRangeMode`
- `Descender.Path2NativeBoundary.Predicates`

Lean predicates / assumptions:

- `Descender.Path2NativeBoundary.InFiber`
- `Descender.Path2NativeBoundary.TransparentViewProjectsFrom`
- `Descender.Path2NativeBoundary.AccountAdmissionSoundness`
- `Descender.Path2NativeBoundary.ParentRootRoleBindingSoundness`
- `Descender.Path2NativeBoundary.RoleRestrictionSoundness`
- `Descender.Path2NativeBoundary.NoExportSoundness`
- `Descender.Path2NativeBoundary.NormRangeSoundness`
- `Descender.Path2NativeBoundary.BlindingNoiseBoundSoundness`
- `Descender.Path2NativeBoundary.TwoViewExtractionSoundness`
- `Descender.Path2NativeBoundary.SpendHandoffSoundness`

The predicate package also names `accountCreateAccepts` and `admitted` so the
wrapper theorems are scoped to accepted/admitted boundaries rather than all
possible boundary encodings.

Lean wrapper theorems:

- `Descender.Path2NativeBoundary.account_admission_soundness_or_break`
- `Descender.Path2NativeBoundary.parent_root_binds_private_roles_or_break`
- `Descender.Path2NativeBoundary.role_restrictions_authenticate_or_break`
- `Descender.Path2NativeBoundary.transparent_view_authenticates_roles_and_extracts_or_break`
- `Descender.Path2NativeBoundary.no_export_holds_or_break`
- `Descender.Path2NativeBoundary.norm_range_valid_or_break`
- `Descender.Path2NativeBoundary.blinding_noise_bounded_or_break`
- `Descender.Path2NativeBoundary.transparent_view_authenticates_extracts_no_export_or_break`
- `Descender.Path2NativeBoundary.transparent_view_full_interface_or_break`
- `Descender.Path2NativeBoundary.transparent_view_extracts_fiber_or_break`
- `Descender.Path2NativeBoundary.spend_handoff_or_break`

Lean compatibility projections into the older Path 2 descent/SIS sidecar:

- `Descender.Path2NativeBoundary.toDescentSISMaskedProjection`
- `Descender.Path2NativeBoundary.toDescentSISPublic`
- `Descender.Path2NativeBoundary.toDescentSISRepresentative`
- `Descender.Path2NativeBoundary.toDescentSISPublic_uses_nativeBoundaryDigest`
- `Descender.Path2NativeBoundary.toDescentSISPublic_quotient_eq_boundary_projection`
- `Descender.Path2NativeBoundary.toDescentSISRepresentative_quotient_eq_boundary_projection`

These are interface theorems, not cryptographic proofs. They let later concrete
commitment/extractor work plug into the native-boundary theorem shape without
changing the statement vocabulary.

## Compatibility With Existing Frames

### Keep

- `Path2PrevalidatedSlotLeafV2` remains the right leaf direction if
  `creation_proof_binding_digest` is reinterpreted as
  `native_boundary_digest`.
- `Path2TransparentDescentPublicViewV1` remains close to correct if its
  `creation_proof_binding_digest`, `trace_commitment_root`, `column_profile`,
  and `masked_projection` are all projections from the native boundary.
- `role_commitments_are_trace_subcommitments` remains the selected compact
  interpretation.

### Change Semantics

- `Path2CreationProofBindingV1` should be demoted from "the" semantic boundary
  to a compatibility encoding of the native boundary role handles, or replaced
  by `Path2NativeBoundaryV1`.
- `opening_commitment` and `pad_seed_digest` should be treated as compatibility
  projections from `b`; they should not introduce separate witnesses.
- `hidden_source_statement_digest` should be a canonical statement projection
  from `b` and `c`.

### Dead If Retained As Independent

Mark the path dead for compact opening if any of these are retained as
independent hidden-witness commitments that must later be linked by equality
proof:

- standalone opening commitment;
- standalone pad seed digest;
- standalone role commitments outside the parent section root;
- standalone projection relation commitment;
- SHA preimage semantics for role handles inside the compact q12289 relation.

## Two-View Theorem Delta

Replace:

```text
same_parent_context binds leaf_context_digest,
creation_proof_binding_digest, trace_commitment_root, role handles, ...
```

with:

```text
same_native_boundary binds native_boundary_digest.
All public view fields are projections of b.
The hidden extractor view returns h in Fiber(B_c, b).
```

The two-view proof still needs:

- public masked view simulation;
- hidden extractor relation soundness;
- no-export for query openings;
- norm/range validity;
- proof-system query soundness.

But it should no longer carry the burden of proving that independently chosen
leaf/proof-binding/role roots all mean the same hidden source.

## Immediate Next

Audit the current older 308b / checked-core public material against this
interface:

1. fields that are downstream spend projections from `b`;
2. fields that are negative-control/fail-closed only;
3. fields that would require an independent hidden witness and should be marked
   dead for the native-boundary path.

## Non-Claims

- This interface does not prove `b in Im(B_c)`.
- This interface does not instantiate a vector commitment or transparent proof.
- This interface does not prove hiding/no-export.
- This interface now has Lean definitions, but no Rust structs.
- This interface does not change the 252-byte spend signature.
