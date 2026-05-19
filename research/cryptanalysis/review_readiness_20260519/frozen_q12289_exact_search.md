# Frozen q12289 Exact Search

- target l1: `512`
- group count: `16`
- checked local pairs: `523264`
- min local centered l1: `533`
- counterexample below target: `False`

## Top Local Pairs

| pair | free l1 | centered l1 | support | relation |
|---|---:|---:|---:|---|
| `(-78, -75)` | 153 | 533 | 5 | `[-83, 89, -208, -78, -75]` |
| `(78, 75)` | 153 | 533 | 5 | `[83, -89, 208, 78, 75]` |
| `(-248, -158)` | 406 | 558 | 5 | `[56, 91, 5, -248, -158]` |
| `(248, 158)` | 406 | 558 | 5 | `[-56, -91, -5, 248, 158]` |
| `(-109, -113)` | 222 | 570 | 5 | `[59, -187, -102, -109, -113]` |
| `(109, 113)` | 222 | 570 | 5 | `[-59, 187, 102, 109, 113]` |
| `(-139, -45)` | 184 | 572 | 5 | `[-3, 278, 107, -139, -45]` |
| `(139, 45)` | 184 | 572 | 5 | `[3, -278, -107, 139, 45]` |
| `(-7, -229)` | 236 | 574 | 5 | `[-10, -55, 273, -7, -229]` |
| `(7, 229)` | 236 | 574 | 5 | `[10, 55, -273, 7, 229]` |
| `(-163, 146)` | 309 | 575 | 5 | `[149, 57, -60, -163, 146]` |
| `(163, -146)` | 309 | 575 | 5 | `[-149, -57, 60, 163, -146]` |

## Derived Multi-Block Bounds

The frozen public view is a direct sum of identical 3x2 local blocks. Global centered-l1 is therefore additive across active blocks, so the best h-block witness has cost at least h * min_local_centered_l1.

| active blocks | derived global min centered l1 | below target? |
|---:|---:|---|
| 1 | 533 | False |
| 2 | 1066 | False |
| 3 | 1599 | False |
| 4 | 2132 | False |
| 5 | 2665 | False |
| 6 | 3198 | False |
| 7 | 3731 | False |
| 8 | 4264 | False |
