# QROM Wide Challenge-Action Probe

This is a best-case byte/security probe for widening the effective
challenge action. The follow-up algebraic probe selects auxiliary Fq12
action bits rather than compact coefficient widening.

## Parameters

- generated at: `2026-06-02T20:24:29.395565+00:00`
- target bits: `128`
- repetitions: `26`
- current bits per repetition: `5`
- quantum query log2: `16`
- classical query log2: `33`
- current transaction bytes: `252`
- current with hidden-source proof bytes: `284`
- fixed-cost assumption: wider action changes only challenge bytes; response/opening/proof byte counts are held fixed to isolate the best-case cost

## Options

| option | bits/rep | challenge B | tx B | tx+hidden B | delta B | log2(T)=32? | max covered in table | proof risk |
|---|---:|---:|---:|---:|---:|:---:|---:|---|
| current-5bit-action | 5 | 21 | 252 | 284 | 0 | no | 1 | baseline; fails above one-bit target horizon |
| six-bit-action-control | 6 | 24 | 255 | 287 | 3 | no | 24 | still fails log2(T)=32 because algebraic action bits are only 156; useful for mid-sized target ledgers, not full deployment scale |
| seven-bit-effective-action-aux-fq12 | 7 | 27 | 258 | 290 | 6 | yes | 32 | small byte cost under the auxiliary-Fq12 route; remaining proof work is challenge-byte canonicality, Fq12 action-collision/QROM budget, and byte-frame binding. Compact coefficient widening and larger block universes are not the selected route. |

## Security Rows

### current-5bit-action

| log2(T) | quantum | classical | algebraic | min | margin | 128-bit? | bottleneck |
|---:|---:|---:|---:|---:|---:|:---:|---|
| 1 | 129 | 128 | 129 | 128 | 0 | yes | classical_transcript |
| 2 | 128 | 127 | 128 | 127 | -1 | no | classical_transcript |
| 16 | 114 | 113 | 114 | 113 | -15 | no | classical_transcript |
| 24 | 106 | 105 | 106 | 105 | -23 | no | classical_transcript |
| 32 | 98 | 97 | 98 | 97 | -31 | no | classical_transcript |

### six-bit-action-control

| log2(T) | quantum | classical | algebraic | min | margin | 128-bit? | bottleneck |
|---:|---:|---:|---:|---:|---:|:---:|---|
| 1 | 155 | 154 | 155 | 154 | 26 | yes | classical_transcript |
| 2 | 154 | 153 | 154 | 153 | 25 | yes | classical_transcript |
| 16 | 140 | 139 | 140 | 139 | 11 | yes | classical_transcript |
| 24 | 132 | 131 | 132 | 131 | 3 | yes | classical_transcript |
| 32 | 124 | 123 | 124 | 123 | -5 | no | classical_transcript |

### seven-bit-effective-action-aux-fq12

| log2(T) | quantum | classical | algebraic | min | margin | 128-bit? | bottleneck |
|---:|---:|---:|---:|---:|---:|:---:|---|
| 1 | 181 | 180 | 181 | 180 | 52 | yes | classical_transcript |
| 2 | 180 | 179 | 180 | 179 | 51 | yes | classical_transcript |
| 16 | 166 | 165 | 166 | 165 | 37 | yes | classical_transcript |
| 24 | 158 | 157 | 158 | 157 | 29 | yes | classical_transcript |
| 32 | 150 | 149 | 150 | 149 | 21 | yes | classical_transcript |

## Result

- seven-bit best-case tx bytes: `258`
- seven-bit best-case tx+hidden-source-proof bytes: `290`
- seven-bit best-case delta bytes: `6`
- runtime delta if reps fixed: no repetition-scaled runtime increase
- recommendation: Use auxiliary Fq12 action bits as the primary redesign route. It fixes log2(T)=32 in the budget model with about +6 bytes while leaving compact local block/sign semantics and response bounds unchanged.

