# Path 2 Native Boundary Blinding/Noise Decision

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting blinding/noise decision note, not an ADR and not binding
> project authority by itself. Binding decisions live in `../decisions/`.

Status: blocker decision pinned.

Purpose: decide whether a bounded public blinding/noise policy can rescue the
current public linear role-commitment shape.

## Decision

Do not use public linear role commitments of the form:

```text
C_role = A_role * v_role mod q
```

as public account-creation objects for private roles, even with the selected
hybrid base-128 digit bounds.

Reason: the existing rank/hiding probe found the role matrices full column rank
for the public role commitments. If `C_role` is public, the role opening is
recoverable over `F_q` for the probed shapes. Bounded blinding does not fix
that unless the commitment dimension is redesigned to have a hiding kernel or a
separate hiding assumption.

## Consequence

The active native-boundary path should use:

```text
private_linear_commitments_inside_zk
```

as the next theorem shape.

Meaning:

- role vectors remain committed/authenticated inside the proof-system trace;
- public `b` carries `parent_section_root`, role handles, and
  `native_boundary_digest`;
- public payload does not expose standalone `C_role = A_role*v_role`;
- binding is exported through the proof-system extractor plus typed role
  restriction authentication;
- hiding/no-export is provided by the ZK/two-view proof policy, not by public
  linear commitments alone.

## Rejected For Active Path

| option | disposition | reason |
|---|---|---|
| bounded public blinding/noise with current role dimensions | dead | full-column-rank public commitments reveal openings |
| broad full-field public blinding | parked | can hide, but double openings export large randomness deltas and weaken short-SIS binding |
| small-noise computational hiding | fallback | coherent, but adds a new LWE/leftover-hash-style hiding assumption and changes the proof surface |
| dual commitment hiding plus binding | fallback | likely larger account-creation payload |

## Active Blinding/Noise Policy

For the active path, `Path2NativeBoundaryBlindingNoiseBoundV1` is not a public
commitment opening bound. It becomes an extractor-side bound:

```text
BlindingNoiseBounded(b, policy, h)
```

means:

- hidden blinding/noise columns extracted from the proof are within the selected
  binding range;
- those columns are not directly serialized as public commitments;
- query openings for those columns are masked/simulated under NoExportSoundness;
- a double accepting proof exports a short role-restriction / proof-system
  binding break, not a public linear commitment opening.

## Updated Theorem Shape

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)
UsesSelectedBlindingNoisePolicy(policy)
  =>
role restrictions authenticate under p_section_root(b)
and extractor obtains h in Fiber(B_c, b)
and NormRangeValid(b, h)
and BlindingNoiseBounded(b, policy, h)
and NoExportHolds(b, view, profile)
```

or a named break.

The theorem no longer accepts an arbitrary `policy`: Lean now requires
`UsesSelectedBlindingNoisePolicy(policy)`, and the Rust proof-family interface
digest binds policy id `1` plus centered bound `2048`. A candidate with a
looser or different blinding/noise policy must change the active proof-system
interface instead of silently satisfying this theorem.

This matches the existing Lean wrapper:

```text
transparent_view_full_interface_or_break
```

## Byte-Model Consequence

Do not use the old 725/893-byte public linear commitment lower bound as the
active payload estimate. It is now a rejected lower-bound comparator.

The active transparent/two-view planning floor remains:

```text
6144 / 8304 / 10720 bytes for 256 / 512 / 1024 rows
```

until a concrete proof family shows typed private role restrictions and ZK query
openings at a smaller cost.

## Next Move

Instantiate the decomposed blinding/noise bound route inside the proof-family
gate:

- `BlindingNoiseBoundRowSoundness`: accepted selected bound rows bind
  blinding/noise auxiliaries to the extracted representative;
- `BlindingNoiseRowsToBoundSoundness`: those rows imply the selected
  `BlindingNoiseBounded(b, policy, h)` predicate;
- dead path: public linear role commitments with current dimensions;
- fallback: small-noise computational hiding commitment if private role
  commitments inside ZK cannot be instantiated compactly.

Then instantiate the concrete proof-family assumption:

```text
NativeBoundaryZkTraceBindingAndHiding
```

## Non-Claims

- This does not instantiate the ZK proof family.
- This does not prove hiding.
- This does not prove SIS binding.
- This does not change spend signature bytes.
