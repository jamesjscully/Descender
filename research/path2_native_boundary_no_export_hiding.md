# Path 2 Native Boundary No-Export / Hiding Interface

Status: theorem interface pinned; not proved.

Purpose: state the no-export condition for the native-boundary route. The
account-creation proof may reveal the public boundary class `b`, but must not
export a hidden representative `h` or any safety-relevant private coordinates in
`Fiber(B_c, b)`.

## Target Claim

```text
NoExportSoundness:

accepted transparent proof
compatible native boundary b
compatible column profile
  =>
public transcript reveals only allowed projections of b,
masked public openings, authentication paths, and simulated consistency data
```

or the verifier exports:

- `live_slot_leakage_or_keygen_break`;
- `qrom_or_domain_separation_break`;
- `boundary_binding_break`;
- `transparentProofSoundnessBreak`.

## Allowed Public Material

The public account-creation payload may reveal:

- `native_boundary_digest`;
- `p_context(b)`;
- `p_section_root(b)`;
- `p_role_profile(b)`;
- typed role handles from `p_role_handles(b)`;
- `p_public_projection(b)` / `masked_projection[6]`;
- proof system id, relation id, trace shape, query count, query index;
- matrix-domain transcript digest;
- constraint commitment root;
- authentication paths;
- consistency messages;
- query openings classified as `public`, `derived_public`, or
  `masked_public` by the column profile.

## Forbidden Exports

The public account-creation payload must not reveal:

- raw pad source digits;
- raw pad seed;
- raw account projection lanes;
- unmasked projection relation lanes;
- opening tuple witness scalars;
- packed residue openings beyond allowed public quotient;
- range/digit witnesses;
- blinding/noise/mask secrets;
- extractor-only hidden trace rows;
- reject-path diagnostics containing any value above.

## Boundary-Class Hiding Statement

The public view may reveal the intended boundary class:

```text
b = B_c(h)
```

It must not select a canonical representative from the fiber:

```text
h in Fiber(B_c, b)
```

unless that representative information is already public under `~safe`.

Equivalent formulation:

```text
PublicTranscript(c, b, proof)
is simulatable from:
  b,
  public proof metadata,
  masked_projection,
  allowed query schedule
without:
  pad_source_opening,
  projection_relation_opening,
  opening_tuple,
  range witnesses,
  masks/blinding
```

## Mask Freshness Rule

Every `masked_public` opening must use a one-time domain:

```text
native_boundary_digest
|| role_id
|| column_set_id
|| row_index
|| query_index
```

Reusing a mask domain for two safety-distinct hidden cells exports
`live_slot_leakage_or_keygen_break` unless the reuse is already proved harmless
by the profile's visibility policy.

## Interaction With Role Restrictions

Role handles are public. Their openings are not.

`RoleRestrictionSoundness` proves that role handles are typed restrictions of
`p_section_root(b)`. `NoExportSoundness` proves that authenticating those
restrictions does not reveal the hidden role vectors except through allowed
masked/simulated openings.

These are separate:

- role restriction: "the handles name the right columns";
- no-export: "the proof does not leak the hidden values in those columns."

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now provides:

- `Descender.Path2NativeBoundary.Predicates.noExportHolds`
- `Descender.Path2NativeBoundary.NoExportSoundness`
- `Descender.Path2NativeBoundary.no_export_holds_or_break`
- `Descender.Path2NativeBoundary.transparent_view_authenticates_extracts_no_export_or_break`

These are interface-level definitions only.

## Composition Status

No-export is now composed with the existing native-boundary transparent theorem
at the Lean interface level:

```text
RoleRestrictionSoundness
+ TwoViewExtractionSoundness
+ NoExportSoundness
  =>
accepted transparent proof authenticates roles,
extracts h in Fiber(B_c, b),
and reveals no forbidden representative coordinates
```

After that, the next highest-risk row is norm/range semantics for the extracted
hidden representative.

## Non-Claims

- This does not prove a simulator theorem.
- This does not instantiate query openings.
- This does not prove mask-domain uniqueness.
- This does not update byte counts.
