# Path 2 Descent/SIS Feasibility

Status: candidate shape pinned; not deployable.

## Decision

- primary path: `Path2DescentSisCommitmentV1`
- fallback: `transparent_hash_based_proof_over_proof_friendly_relation`
- spend signature byte delta: `0`
- state leaf byte delta: `32`
- reason: descent/SIS matches the existing Descender gluing theorem shape and preserves a post-quantum account-creation proof path
- current shape blocker: the current three-public-commitment shape is full-column-rank for the main pad/projection/opening roles and therefore reveals openings if those commitments are public

## Public Quotient

```text
masked_projection_i = pad_lane_i + account_projection_i mod q
```

- q: `12289`
- lanes: `6`
- source: lanes 12..17 of the canonical 18-residue packed witness commitment

## Active Hybrid Role Vectors

The active norm/range policy no longer uses raw q12289 hidden semantic residues
as the binding surface. Hidden role values are represented by base-128 digit
columns; reconstruction to q12289 residues is a proof relation obligation.

| role | semantic digit lanes | blinding lanes | range lanes | total | notes |
|---|---:|---:|---:|---:|---|
| pad_source | 12 | 24 | 0 | 36 | six hidden pad residues as two base-128 digits each |
| projection_relation | 12 | 24 | 0 | 36 | six hidden account-projection residues as two base-128 digits each |
| opening_tuple | 16 | 32 | 8 | 56 | eight opening-tuple values as two base-128 digits each plus range/aux lanes |
| transcript_commitment | 40 | 80 | 8 | 128 | aggregate role; not selected as the active binding theorem under current blinding bounds |

Executable sanity over this corrected layout is recorded in
`research/path2_descent_sis_hardness_sanity.md`.

## Historical Raw Role Vectors

| role | secret lanes | blinding lanes | range lanes | total | notes |
|---|---:|---:|---:|---:|---|
| pad_source | 6 | 24 | 0 | 30 | six hidden pad lanes plus randomness/noise lanes for hiding |
| projection_relation | 6 | 24 | 0 | 30 | six hidden account-projection lanes plus randomness/noise lanes |
| opening_tuple | 8 | 32 | 8 | 48 | placeholder scalarization of opening tuple context; exact frame mapping is still a proof obligation |

## Candidate Byte And Q-Op Rows

| candidate | rows/commitment | columns | commitment bytes | response lower bound | payload lower bound | verifier mul-adds | status |
|---|---:|---:|---:|---:|---:|---:|---|
| path2_descent_sis_rows_32 | 32 | 108 | 168 | 189 | 557 | 3456 | too_small_for_security_claim |
| path2_descent_sis_rows_48 | 48 | 108 | 252 | 189 | 725 | 5184 | primary_feasibility_band |
| path2_descent_sis_rows_64 | 64 | 108 | 336 | 189 | 893 | 6912 | primary_feasibility_band |
| path2_descent_sis_rows_96 | 96 | 108 | 504 | 189 | 1229 | 10368 | larger_fallback_band |

## Required Theorem Obligations

- instantiate LocalQuotientDescent public/representative/hidden/evidence types
- define role-separated matrix derivation domains
- prove commitment binding or export SIS/module-SIS binding break
- prove hiding/sampler preservation or export samplerHidingBreak
- prove link equation is non-public and not a projection shortcut
- define norm/range bounds for all committed openings
- define Fiat-Shamir/QROM challenge budget for the payload proof
- define extractor from accepting payload to hidden account opening or named break

## Negative Controls

- wrong masked projection rejects
- wrong leaf or creation_proof_binding_digest rejects
- public projection shortcut is not accepted as proof
- non-short modular witness opening rejects
- duplicate or malleated commitment rejects
- sampler-conditioning replay stays above hiding target
- reject paths do not export pad lanes, projection lanes, masks, or opening frames

## Non-Claims

- lower-bound payload bytes are not a deployable proof-size estimate
- no SIS hardness level is claimed by this model
- opening_tuple scalarization is a placeholder until the exact frame map is defined
- the model does not replace production preflight's fail-closed blocker
- the initial public commitment dimensions are blocked by the rank/hiding probe

## Interpretation

The 48-row and 64-row bands remain useful engineering probes, but the old
raw-vector byte floors are not the active deployable proof-size estimate. The
active proof candidate is the transparent payload over the hybrid base-128
descent relation; this artifact now preserves the historical feasibility rows
only as context for the SIS/hiding/extractor analysis.
