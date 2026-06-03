# Path 2 Descent/SIS Norm/Range Surface

Status: active hybrid-base128 norm/range policy selected and executable-pinned.

## Core Issue

SIS binding needs two accepting openings to export a nonzero short kernel vector. The current feasibility vectors name q12289 semantic lanes directly; canonical residue uniqueness is not the same as shortness.

The binding surface already gives the algebraic reduction:
`delta = v0 - v1` and `A_role,rowband * delta = 0 mod q`. The missing
piece is whether `delta` is short enough to support the intended SIS
binding claim.

## Raw Centered Residue Surface

| role | semantic lanes | canonical interval | delta Linf/lane | semantic delta L2 | consequence |
|---|---:|---|---:|---:|---|
| pad_source | 6 | `[-6144, 6144]` | 12288 | 30099.33 | valid canonical residues are unique, but the exported kernel vector is not small in the sense needed for a clean SIS binding claim |
| projection_relation | 6 | `[-6144, 6144]` | 12288 | 30099.33 | valid canonical residues are unique, but the exported kernel vector is not small in the sense needed for a clean SIS binding claim |
| opening_tuple | 8 | `[-6144, 6144]` | 12288 | 34755.713 | valid canonical residues are unique, but the exported kernel vector is not small in the sense needed for a clean SIS binding claim |

## Candidate Representation Options

| id | status | advantage | problem |
|---|---|---|---|
| `raw_centered_residue` | not_sufficient_as_binding_hardness_surface | minimal columns and keeps the current 725/893 byte lower-bound shape | semantic deltas can be as large as q-1, so the exported kernel vector is not a compelling short-SIS witness without a separate, likely weak, bounded-SIS assumption |
| `base_128_digit_opening` | selected_active_policy | keeps semantic deltas within 127 per digit while adding only one extra semantic column per residue | still requires proof constraints that the digits reconstruct the q12289 residue and that each digit is in [0,127] |
| `base_16_digit_opening` | fallback_if_estimator_dislikes_base_128 | much smaller per-coordinate delta bound | doubles digit columns again and likely raises account-creation proof bytes |
| `masked_short_lift` | research_only_until_hiding_sampler_is_fixed | most aligned with the descent thesis if a natural short lift exists | risks reintroducing sampler-conditioning or projection-link leakage unless the hiding sampler row closes at the same time |

## Digit Encoding Cost Probe

| role | residues | base-128 columns | base-128 delta L2 | base-16 columns | base-16 delta L2 |
|---|---:|---:|---:|---:|---:|
| pad_source | 6 | 12 | 439.941 | 24 | 73.485 |
| projection_relation | 6 | 12 | 439.941 | 24 | 73.485 |
| opening_tuple | 8 | 16 | 508.0 | 32 | 84.853 |

## Executable Policy

The active executable policy is pinned by:

```text
verify_path2_descent_sis_opening_norm_range
```

Role columns are:

| role | semantic digit columns | blinding columns | range/aux columns | total |
|---|---:|---:|---:|---:|
| `pad_source` | 12 | 24 | 0 | 36 |
| `projection_relation` | 12 | 24 | 0 | 36 |
| `opening_tuple` | 16 | 32 | 8 | 56 |
| `transcript_commitment` | 40 | 80 | 8 | 128 |

Semantic and range/aux digits must be nonnegative and at most `127`.
Blinding coordinates are centered and at most `2048`.

## Current Sanity Result

`research/path2_descent_sis_hardness_sanity.md` runs the first executable
rank/Gaussian-heuristic sanity read over the corrected dimensions. The result
does not reject the base-128 digit representation; it rejects using the current
kernel-bearing role surfaces as a clean short-SIS binding theorem while
blinding deltas may be as large as `4096` per blinding coordinate.

## Recommended Next Step

Keep the active transparent payload route on proof-system/native-boundary trace
authentication. Do not promote the aggregate transcript commitment, or the
48-row opening-tuple kernel surface, as the active short-SIS binding theorem
without redesigning blinding bounds, row counts, or the commitment model.

## Non-Claims

- this artifact does not prove digit reconstruction soundness
- this artifact does not prove range-check soundness
- this artifact does not update the deployable byte estimate
- this artifact does not close the proof-system extractor row

## Interpretation

This is the first place where the descent/SIS path changed shape: raw hidden
q12289 semantic residues are no longer the active binding surface. Base-128
digits are the selected hidden-role representation, but the blinding/noise
coordinates now dominate any extracted kernel vector. The active theorem route
therefore needs proof-system/native-boundary trace authentication rather than a
public aggregate SIS commitment under the current bounds.
