# Path 2 Native Boundary Proof-Family Overhead Budget

Status: planning budget pinned; not a concrete proof-size estimate.

Purpose: set a go/no-go budget for the active masked trace / STARK-like route
after the native-boundary relation overlay.

Comparator:

```text
parked_lattice_sigma_floor = 16416 bytes
```

Source: `research/path2_descent_sis_lattice_sigma_toy_model.md`.

## Native-Boundary Overlay Baseline

From `research/path2_transparent_proof_byte_model.md`:

| trace rows | native optimistic | native working | native stress |
|---:|---:|---:|---:|
| 256 | ~8332 | 8704 | 9728 |
| 512 | ~11039 | 11504 | 12784 |
| 1024 | ~13999 | 14560 | 16096 |

## Remaining Headroom To Sigma Floor

| trace rows | working headroom | stress headroom | read |
|---:|---:|---:|---|
| 256 | 7712 | 6688 | ample |
| 512 | 4912 | 3632 | usable |
| 1024 | 1856 | 320 | tight |

## Decision Rule

For the transparent route to remain the active path:

1. The concrete proof-family overhead for the selected security row must fit
   inside the corresponding headroom.
2. The proof family must provide:

   ```text
   ParentTraceCommitmentExtractionSoundness
   ExtractedTraceRelationKnowledgeSoundness
   NoExportSoundness
   NativeBoundaryTraceQromScheduleSoundness
   ```

3. No hidden SHA preimage rows may enter `NativeBoundaryTraceRelationV1`.

## Go / Watch / Pivot

| condition | decision |
|---|---|
| 256-row or 512-row working row gives adequate security and overhead fits | continue transparent route |
| 1024-row working row is required and overhead stays under 1856 bytes | continue, but mark close-to-floor |
| 1024-row stress row is required | watch: only 320 bytes remain before sigma floor |
| concrete overhead pushes selected row above 16416 bytes | pivot comparison to MPC-in-the-head / lattice-SIS ZK |
| proof-family theorem cannot state full trace extraction and relation knowledge | pivot regardless of bytes |
| no-export requires raw hidden openings | pivot regardless of bytes |

## Practical Read

The row count did not kill the transparent route. The 176-row working estimate
still has room at 256/512 rows and limited room at 1024 rows.

The hard blocker is now proof-family overhead and theorem strength, not the
native-boundary relation size itself.

## Next Move

Pin the proof-family assumption in a deployability-facing way:

```text
NativeBoundaryTransparentProofFamilyV1
```

with explicit sub-assumptions and the overhead budget above. This should make
clear whether the project is comfortable keeping it as a named assumption or
whether it wants to pivot to a more native full-witness proof such as
MPC-in-the-head.

## Non-Claims

- This does not estimate a concrete STARK proof.
- This does not prove the sigma fallback is deployable.
- This does not choose the security row.
