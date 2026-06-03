# Path 2 Native Boundary Hidden Extractor View

Status: first-pass extractor obligation pinned; concrete proof family not
proved.

Purpose: close the ambiguity in `hidden_extractor_view`. The extractor must
return a complete hidden representative in the native-boundary fiber, not just
the cells opened by verifier queries.

## Target

For an accepted transparent account-creation view:

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
```

the extractor must output:

```text
h in Fiber(B_c, b)
```

such that:

```text
LocalValid_c(h)
DescentConsistent_c(h)
ExtractedRolesComplete(b, profile, h)
NormRangeValid(b, h)
BlindingNoiseBounded(b, policy, h)
```

or the adversary exports a named break.

## Completeness Meaning

`ExtractedRolesComplete(b, profile, h)` means the extractor recovers enough
private material to instantiate every role handle named by `p_role_handles(b)`:

| role | extractor output required |
|---|---|
| `packed_witness` | packed residue/source coordinates and digit/range witnesses |
| `randomizer` | account randomizer coordinates or their quotient-safe representative |
| `pad_source` | pad source coordinates, mask seeds, and derivation witnesses |
| `opening_tuple` | opening coordinates consumed by account/spend handoff |
| `projection_relation` | hidden source projection coordinates linking packed residues to public masked projection |
| `rangeWitness` | base-128 digit witnesses for private hidden roles |
| `blinding` | blinding/noise auxiliaries and bounds needed by the binding proof |
| `constraint` | trace-local constraint witnesses required for `LocalValid_c` |

The extractor may output quotient-safe representatives. It does not have to
recover gauge choices that are inside `ker(B_c) subset ~safe`.

## Non-Sufficient Outputs

The following do not satisfy the hidden extractor gate:

- sampled query openings only;
- public masked projection lanes only;
- authentication paths without underlying private cells;
- a witness for one role handle but not the others;
- a satisfying tuple not authenticated under `parent_section_root`;
- role vectors authenticated under the wrong role id, column set, profile,
  account, slot, or native boundary digest;
- raw private values exposed to the verifier instead of recovered only by the
  extractor/simulator proof.

## Break Surface

Failure to extract a full role-complete `h` must be charged as one of:

- `transparentProofSoundnessBreak`: accepted proof has no extractable witness;
- `inner_commitment_binding_break`: two safety-distinct openings authenticate
  under one role handle or parent root;
- `boundary_binding_break`: extracted roles do not project to the admitted
  `native_boundary_digest`;
- `descent_soundness_break`: extracted roles are locally plausible but do not
  glue into one descent representative;
- `path2_sis_norm_or_range_break`: extracted hidden digits do not satisfy the
  selected hybrid base-128 range policy;
- `blinding_noise_bound_break`: extracted blinding/noise auxiliaries exceed the
  binding-compatible policy;
- `qrom_or_domain_separation_break`: extraction relies on challenge/order
  material not bound before query derivation.

## Main Bottleneck

The proof-family extractor must bridge a gap between two different views:

```text
public verifier view: masked/simulated query openings
extractor view: full private role columns under one parent root
```

This is only credible if the proof family has an extraction theorem for the
committed trace, not merely a local query-checking theorem. In other words, the
parent section root must be an extractable commitment to a complete trace, and
role handles must be typed restrictions of that trace.

If the candidate transparent system only proves that queried cells were opened
consistently, it is insufficient for the native-boundary theorem unless the
query protocol has a separate proximity/extraction theorem that reconstructs
the full trace from accepting transcripts.

## Proof Strategy

The desired proof decomposition is:

1. Parent-root extractability:

   ```text
   Accept(view) => exists trace. Opens(parent_section_root, trace)
   ```

2. Typed restriction extraction:

   ```text
   Opens(parent_section_root, trace)
   role_handle_i = Restrict(parent_section_root, profile, role_i)
   =>
   role_i(trace) is fixed
   ```

3. Native-boundary assembly:

   ```text
   role_i(trace) for all required roles
   =>
   assemble h
   ```

4. Descent and local validity:

   ```text
   trace constraints + role restrictions
   =>
   LocalValid_c(h) and DescentConsistent_c(h)
   ```

5. Boundary fiber:

   ```text
   projections(trace) = b
   =>
   B_c(h) = b
   ```

## Immediate Test

The next concrete proof task is to classify candidate proof families by whether
their published/assumed soundness theorem gives:

```text
full trace extraction
```

or only:

```text
queried cell consistency
```

Only the first can directly satisfy this gate. The second requires an additional
proximity/list-decoding/extractor theorem before it is usable.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now names the additional obligation:

- `ExtractedTrace`
- `Predicates.transparentExtractsTrace`
- `Predicates.extractedRolesComplete`
- `ParentTraceExtractionSoundness`
- `TraceAssemblySoundness`
- `HiddenExtractorRoleCompletenessSoundness`
- `hidden_extractor_roles_complete_or_break`

The composed theorem `transparent_view_full_interface_or_break` now requires
and returns `P.extractedRolesComplete b profile h`.
The bridge theorem
`trace_extraction_implies_hidden_representative_or_break` states the intended
route from a full extracted trace to a role-complete hidden representative.

## Non-Claims

- This does not prove trace extraction.
- This does not choose Merkle/FRI/STARK/MPC-in-the-head internals.
- This does not update byte counts.
