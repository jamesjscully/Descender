# Product-Block Signature Sampler Analysis

- q: `12289`
- n: `32`
- eta: `12`
- product blocks: `16`
- local free coordinates per block: `2`
- target challenge bits: `128.0`
- digest bytes: `32`
- max log2 expected retries: `20.0`
- max extra repetitions searched: `4`
- fork challenge draws per repeated commitment: `6`

## Exact Sampler

For each active response coordinate with shift `|v| <= eta`, sample `y` uniformly from `[-z-eta,z+eta]`, output `r = y + v`, and accept iff `r in [-z,z]`.  For every allowed shift, the accepted response is exactly uniform on `[-z,z]`.

The per-coordinate acceptance probability is `(2z+1)/(2(z+eta)+1)`.  The total retry exponent is charged over active response coordinates only.

## Best Entropy + Retry Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 5.00 | 26 | 2 | 96 | 8 | 2 | 17 | 69 | 84 | 8.79 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 2 | 80 | 8 | 2 | 17 | 69 | 84 | 10.42 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 2 | 64 | 8 | 2 | 17 | 69 | 84 | 12.80 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 2 | 48 | 7 | 2 | 17 | 69 | 84 | 16.59 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 2 | 96 | 8 | 2 | 17 | 71 | 86 | 9.13 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 2 | 80 | 8 | 2 | 17 | 71 | 86 | 10.83 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 2 | 64 | 8 | 2 | 17 | 71 | 86 | 13.29 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 2 | 48 | 7 | 2 | 17 | 71 | 86 | 17.22 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 28 | 2 | 96 | 8 | 2 | 18 | 74 | 88 | 9.47 | 0.04 | 0.00 | 0.00 | 11.64 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 28 | 2 | 80 | 8 | 2 | 18 | 74 | 88 | 11.23 | 0.04 | 0.00 | 0.00 | 11.64 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 28 | 2 | 64 | 8 | 2 | 18 | 74 | 88 | 13.78 | 0.04 | 0.00 | 0.00 | 11.64 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 28 | 2 | 48 | 7 | 2 | 18 | 74 | 88 | 17.86 | 0.04 | 0.00 | 0.00 | 11.64 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 29 | 2 | 96 | 8 | 2 | 19 | 77 | 90 | 9.81 | 0.05 | 0.00 | 0.00 | 12.20 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 15 | 4 | 96 | 8 | 4 | 17 | 77 | 92 | 10.15 | 0.09 | 0.01 | 0.00 | 13.34 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 29 | 2 | 80 | 8 | 2 | 19 | 77 | 90 | 11.63 | 0.05 | 0.00 | 0.00 | 12.20 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 15 | 4 | 80 | 8 | 4 | 17 | 77 | 92 | 12.03 | 0.09 | 0.01 | 0.00 | 13.34 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 29 | 2 | 64 | 8 | 2 | 19 | 77 | 90 | 14.28 | 0.05 | 0.00 | 0.00 | 12.20 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 15 | 4 | 64 | 8 | 4 | 17 | 77 | 92 | 14.77 | 0.09 | 0.01 | 0.00 | 13.34 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 29 | 2 | 48 | 7 | 2 | 19 | 77 | 90 | 18.50 | 0.05 | 0.00 | 0.00 | 12.20 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 15 | 4 | 48 | 7 | 4 | 17 | 77 | 92 | 19.14 | 0.09 | 0.01 | 0.00 | 13.34 | entropy-retry-pass-weak-fork-delta |

## Best Rows With >=16 Bits Compact Fork-Delta Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 13 | 22.13 | 6 | 26 | 224 | 9 | 30 | 17 | 197 | 212 | 11.72 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 26 | 192 | 9 | 30 | 17 | 197 | 212 | 13.61 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 26 | 160 | 9 | 30 | 17 | 197 | 212 | 16.23 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 24 | 224 | 9 | 27 | 20 | 209 | 221 | 12.62 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 24 | 192 | 9 | 27 | 20 | 209 | 221 | 14.66 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 24 | 160 | 9 | 27 | 20 | 209 | 221 | 17.48 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 26 | 448 | 10 | 33 | 17 | 215 | 230 | 5.94 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 26 | 384 | 10 | 33 | 17 | 215 | 230 | 6.92 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 26 | 320 | 10 | 33 | 17 | 215 | 230 | 8.27 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 26 | 256 | 10 | 33 | 17 | 215 | 230 | 10.29 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 11 | 23.09 | 8 | 22 | 224 | 9 | 25 | 24 | 224 | 232 | 13.22 | 9.42 | 17.24 | 0.20 | 59.08 | candidate-compact-fork-delta-16 |
| 11 | 23.09 | 8 | 22 | 192 | 9 | 25 | 24 | 224 | 232 | 15.35 | 9.42 | 17.24 | 0.20 | 59.08 | candidate-compact-fork-delta-16 |

## Best Rows With >=24 Bits Compact Fork-Delta Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 15 | 19.00 | 7 | 30 | 224 | 9 | 34 | 17 | 255 | 270 | 15.78 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 30 | 192 | 9 | 34 | 17 | 255 | 270 | 18.32 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 13 | 22.13 | 8 | 26 | 224 | 9 | 30 | 23 | 263 | 272 | 15.63 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |
| 13 | 22.13 | 8 | 26 | 192 | 9 | 30 | 23 | 263 | 272 | 18.15 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |
| 14 | 20.91 | 8 | 28 | 224 | 9 | 32 | 21 | 277 | 288 | 16.83 | 20.00 | 27.84 | 1.78 | 45.24 | candidate-compact-fork-delta-24 |
| 14 | 20.91 | 8 | 28 | 192 | 9 | 32 | 21 | 277 | 288 | 19.54 | 20.00 | 27.84 | 1.78 | 45.24 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 30 | 448 | 10 | 38 | 17 | 283 | 298 | 8.00 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 30 | 384 | 10 | 38 | 17 | 283 | 298 | 9.31 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 30 | 320 | 10 | 38 | 17 | 283 | 298 | 11.14 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 30 | 256 | 10 | 38 | 17 | 283 | 298 | 13.85 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 13 | 22.13 | 8 | 26 | 448 | 10 | 33 | 23 | 287 | 296 | 7.92 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |
| 13 | 22.13 | 8 | 26 | 384 | 10 | 33 | 23 | 287 | 296 | 9.22 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |

## Best Rows With >=32 Bits Compact Fork-Delta Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 448 | 10 | 40 | 16 | 336 | 352 | 9.75 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 384 | 10 | 40 | 16 | 336 | 352 | 11.35 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 320 | 10 | 40 | 16 | 336 | 352 | 13.58 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 256 | 10 | 40 | 16 | 336 | 352 | 16.89 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 30 | 448 | 10 | 38 | 22 | 364 | 374 | 10.29 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 30 | 384 | 10 | 38 | 22 | 364 | 374 | 11.97 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 30 | 320 | 10 | 38 | 22 | 364 | 374 | 14.32 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 30 | 256 | 10 | 38 | 22 | 364 | 374 | 17.81 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 768 | 11 | 44 | 16 | 368 | 384 | 5.72 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 640 | 11 | 44 | 16 | 368 | 384 | 6.86 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 512 | 11 | 44 | 16 | 368 | 384 | 8.55 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |

## Best Full-Block Coverage Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 448 | 10 | 40 | 16 | 336 | 352 | 9.75 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 384 | 10 | 40 | 16 | 336 | 352 | 11.35 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 320 | 10 | 40 | 16 | 336 | 352 | 13.58 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 256 | 10 | 40 | 16 | 336 | 352 | 16.89 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 768 | 11 | 44 | 16 | 368 | 384 | 5.72 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 640 | 11 | 44 | 16 | 368 | 384 | 6.86 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 32 | 512 | 11 | 44 | 16 | 368 | 384 | 8.55 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 32 | 448 | 10 | 40 | 18 | 378 | 392 | 10.97 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 32 | 384 | 10 | 40 | 18 | 378 | 392 | 12.77 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 32 | 320 | 10 | 40 | 18 | 378 | 392 | 15.27 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 32 | 256 | 10 | 40 | 18 | 378 | 392 | 19.00 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |

## Conclusions

- The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1).
- The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes 2 free response coordinates.
- Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.
- Active-challenge coverage and fork-delta coverage are separate diagnostics.  The extraction proof needs a usable challenge delta on every block, not merely a challenge that touches the block once.
- Compact fork-delta coverage assumes 6 challenge draws for the same commitment per repeated round.  It only counts opposite signed active openings, because a compact transcript omits inactive response coordinates.
- The full-response fork-delta column is an idealized diagnostic for a transcript that exposes every response coordinate; it should not be used as the compact-signature security claim.
- All coverage bits are exact any-block-missed probabilities computed by inclusion-exclusion over the product blocks.  They are sharper than a union-bound-only diagnostic while still charging the real failure event.
- The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their fork-delta diagnostic is weak under compact active-only openings.
- Full-block challenges respond on all 32 coordinates; they remove the inactive-coordinate omission issue, but the forking proof still needs enough signed challenge draws to obtain opposite signs blockwise.
- The most promising small-signature route is now an intermediate compact fork-delta row plus an explicit multi-fork Fiat-Shamir theorem, not another scalar-challenge size screen.
