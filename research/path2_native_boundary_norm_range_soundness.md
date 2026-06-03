# Path 2 Native Boundary Norm/Range Soundness

Status: theorem interface pinned; hybrid mode selected; not proved.

Purpose: state the norm/range obligation for the hidden representative extracted
from an accepted native-boundary transparent proof. This is required before
binding claims can be reduced to SIS/module-SIS style assumptions.

## Target Claim

```text
NormRangeSoundness:

accepted transparent proof
compatible native boundary b
extracted h in Fiber(B_c, b)
  =>
NormRangeValid(b, h)
```

or the verifier exports:

- `path2_sis_norm_or_range_break`;
- `boundary_binding_break`;
- `descent_soundness_break`;
- `transparentProofSoundnessBreak`;
- `qrom_or_domain_separation_break`.

## Meaning

`NormRangeValid(b, h)` means every semantic value extracted from the hidden view
has the canonical representation and bound required by the binding theorem:

- q12289 residues decode canonically;
- base-128/digit decomposition, if selected, recomposes to the same residues;
- modular wrap is excluded or explicitly named as a break;
- pad-source lanes are in the declared range;
- account projection lanes are in the declared range;
- opening tuple scalars are in the declared range;
- blinding/noise values are in the declared commitment bounds;
- range witnesses correspond to the same role columns authenticated under
  `p_section_root(b)`.

## Why This Remains Separate

Native `b` removes cross-object consistency bloat. It does not by itself prove
that extracted lanes are short/canonical enough for the selected SIS binding
argument.

This is the remaining place where the design may still force a byte-model
change:

- if raw q12289 residues are enough, the transparent relation stays smaller;
- if base-128 digit openings are required, the column/profile and payload size
  increase;
- if the only clean range proof is too large, the boundary construction is not
  enough and the proof family must be redesigned.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now provides:

- `Descender.Path2NativeBoundary.Predicates.normRangeValid`
- `Descender.Path2NativeBoundary.NormRangeSoundness`
- `Descender.Path2NativeBoundary.norm_range_valid_or_break`
- `Descender.Path2NativeBoundary.transparent_view_full_interface_or_break`

The final wrapper composes:

```text
RoleRestrictionSoundness
+ ParentTraceExtractionSoundness
+ TraceAssemblySoundness
+ NoExportSoundness
+ NormRangeSoundness
```

into one transparent native-boundary proof surface.

## Selected Mode

The selected theorem mode is:

```text
hybridBase128HiddenRoles
selectedSisRoleMatrixRows = 64
```

Meaning:

- public `masked_projection[6]` remains canonical raw q12289;
- hidden role openings use base-128 digit witnesses;
- SIS binding vectors use digit coordinates, not raw q12289 residues.

See `research/path2_native_boundary_norm_range_selection.md`.

## Next Proof Move

Instantiate the concrete range gadget for `hybridBase128HiddenRoles` under the
selected 64-row private role-matrix gate.

## Non-Claims

- This chooses hybrid base-128 hidden-role openings, but does not prove them.
- This does not prove SIS hardness.
- This does not update byte counts.
- This does not instantiate concrete range-check columns.
