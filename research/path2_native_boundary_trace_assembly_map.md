# Path 2 Native Boundary Trace Assembly Map

Status: first-pass assembly map pinned; concrete column indices not assigned.

Purpose: test the design-controlled half of knowledge soundness:

```text
TraceAssemblySoundness
```

If a proof-family extractor gives a full committed trace, this map states how
the trace becomes the hidden representative `h : H_c` and why `B_c(h) = b`.

## Assembly Target

Given:

```text
trace : ExtractedTrace
TraceOpensBoundary(b, profile, trace)
TraceRelationValid(b, profile, trace)
```

assemble:

```text
h =
  (
    Section_c,
    PackedResidues_c,
    PadSource_c,
    OpeningTuple_c,
    ProjectionRelation_c,
    LifecycleSlot_c,
    TraceWitness_c,
    Gauge_c
  )
```

with:

```text
B_c(h) = b
LocalValid_c(h)
DescentConsistent_c(h)
ExtractedRolesComplete(b, profile, h)
```

## Source Columns

| `H_c` component | trace source | role/profile requirement |
|---|---|---|
| `Section_c` | parent trace opened by `trace.parentSectionRoot` | `parentSectionRoot = p_section_root(b)` and profile id matches `p_role_profile(b)` |
| `PackedResidues_c` | `packed_witness` role columns | typed restriction under `packedWitnessHandle`; includes packed residue/source coordinates and public masked-projection link columns |
| `PadSource_c` | `pad_source` role columns | typed restriction under `padSourceHandle`; includes base-128 pad digits, pad lane residues, mask derivation source, and mask-domain labels |
| `OpeningTuple_c` | `opening_tuple` role columns | typed restriction under `openingTupleHandle`; includes account witness opening, randomizer link, restore/lifecycle anchor, and spend-opening coordinates |
| `ProjectionRelation_c` | `projection_relation` role columns | typed restriction under `projectionRelationHandle`; includes account projection lanes, pad projection lanes, and active-lane relation constraints |
| `LifecycleSlot_c` | context/public and opening tuple columns | fixed by `p_context(b)`, live marker, slot id, lifecycle anchor, and consume-once namespace |
| `TraceWitness_c` | full hidden extractor view | includes all private role vectors, masks, range witnesses, account-binding witnesses, and local constraint witnesses |
| `Gauge_c` | blinding/range/mask auxiliary columns | quotient-safe choices, private masks, encoding choices, and blinding/noise auxiliaries |

## Boundary Projection Checks

Trace assembly must imply these equalities:

| boundary projection | trace-derived source |
|---|---|
| `p_context(b)` | public context columns and lifecycle columns |
| `p_section_root(b)` | `trace.parentSectionRoot` |
| `p_role_profile(b)` | `trace.roleProfileDigest` |
| `p_role_handles(b).packedWitnessHandle` | restriction handle for `packed_witness` columns |
| `p_role_handles(b).randomizerHandle` | restriction handle for `randomizer` columns |
| `p_role_handles(b).padSourceHandle` | restriction handle for `pad_source` columns |
| `p_role_handles(b).openingTupleHandle` | restriction handle for `opening_tuple` columns |
| `p_role_handles(b).projectionRelationHandle` | restriction handle for `projection_relation` columns |
| `p_public_projection(b)` | six masked projection lanes reconstructed from pad/projection relation columns |
| `p_statement(b)` | canonical creation statement digest derived from context, roles, profile, and masked projection |
| `native_boundary_digest` | digest of the encoded native boundary object whose projections match the trace |

Any mismatch exports `boundary_binding_break`, `inner_commitment_binding_break`,
or `qrom_or_domain_separation_break`.

## Local Validity From TraceRelationValid

`TraceRelationValid(b, profile, trace)` must include:

1. canonical q12289 decoding for public masked projection lanes;
2. base-128 digit reconstruction for hidden role residues;
3. packed-residue reconstruction for the account witness state;
4. pad-source derivation for pad lanes and mask domains;
5. active-lane equations linking pad lanes, projection lanes, and
   `p_public_projection(b)`;
6. opening-tuple/account-binding equations;
7. checked-core residue compatibility for carried rows;
8. selected `hybridBase128HiddenRoles` range predicates;
9. no postchallenge dependence on spend message or spend authorization digest.

These are relation checks over the extracted trace, not public openings.

## Descent Consistency From Role Restrictions

`DescentConsistent_c(h)` follows if each role is a typed restriction of one
parent section:

```text
role_handle_i =
  Restrict(p_section_root(b), p_role_profile(b), role_i, column_set_i)
```

and the trace relation checks cross-role links:

- packed witness and opening tuple use the same account witness/randomizer;
- pad source and opening tuple use the same pad seed/source;
- projection relation uses pad lanes derived from pad source;
- packed/projection relation reconstructs the six public masked lanes;
- lifecycle slot uses the same slot/account/lifecycle context;
- range/blinding auxiliaries are bound to the same role cells they justify.

## Current Assessment

Trace assembly looks achievable by construction. The hard part is not the map
from full trace to `h`; it is obtaining the full trace without verifier-visible
raw hidden openings.

The design constraints for assembly are:

- keep all semantic roles under one parent root;
- keep role handles typed by role id and column set;
- make every cross-role equality a trace relation check;
- make `native_boundary_digest` the digest of the projection object, not a
  separate witness commitment;
- reject profiles that omit any role needed to assemble `H_c`.

## Open Details

- assign concrete column-set ids for each role;
- decide whether `randomizer` remains a separate role handle or is absorbed
  into `opening_tuple`;
- pin exact base-128 digit counts for each hidden q12289 role value;
- pin exact lifecycle consume-once columns needed for `LifecycleSlot_c`;
- specify the canonical statement digest recipe from assembled projections.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now names the abstract assembly surface:

- `ExtractedTrace`
- `Predicates.traceOpensBoundary`
- `Predicates.traceRelationValid`
- `Predicates.traceAssemblesRepresentative`
- `TraceAssemblySoundness`

## Next Move

Pin a role-completeness table that lists mandatory roles and whether each is:

- required for `LocalValid_c`;
- required for `DescentConsistent_c`;
- required for `B_c(h) = b`;
- allowed to vary under `~safe`.

That table will expose any missing role before we spend more effort on the
proof-family extractor.

Done:
`research/path2_native_boundary_role_completeness_table.md` pins the mandatory
roles, and Lean now requires `ProfileRoleComplete profile` before
trace-assembly or extracted-role completeness can be claimed.

## Non-Claims

- This does not prove `TraceAssemblySoundness`.
- This does not assign final column indices.
- This does not prove the proof family extracts the full trace.
