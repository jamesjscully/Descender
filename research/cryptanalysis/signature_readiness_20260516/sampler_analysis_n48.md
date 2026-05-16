# Product-Block Signature Sampler Analysis

- q: `12289`
- n: `48`
- eta: `12`
- product blocks: `16`
- local free coordinates per block: `3`
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
| 1 | 5.00 | 26 | 3 | 96 | 8 | 3 | 17 | 95 | 110 | 13.19 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 3 | 80 | 8 | 3 | 17 | 95 | 110 | 15.64 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 3 | 64 | 8 | 3 | 17 | 95 | 110 | 19.20 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 3 | 96 | 8 | 3 | 17 | 98 | 113 | 13.70 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 3 | 80 | 8 | 3 | 17 | 98 | 113 | 16.24 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 27 | 3 | 64 | 8 | 3 | 17 | 98 | 113 | 19.94 | 0.03 | 0.00 | 0.00 | 11.08 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 28 | 3 | 96 | 8 | 3 | 18 | 102 | 116 | 14.20 | 0.04 | 0.00 | 0.00 | 11.64 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 28 | 3 | 80 | 8 | 3 | 18 | 102 | 116 | 16.84 | 0.04 | 0.00 | 0.00 | 11.64 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 29 | 3 | 96 | 8 | 3 | 19 | 106 | 119 | 14.71 | 0.05 | 0.00 | 0.00 | 12.20 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 29 | 3 | 80 | 8 | 3 | 19 | 106 | 119 | 17.44 | 0.05 | 0.00 | 0.00 | 12.20 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 15 | 6 | 96 | 8 | 6 | 17 | 107 | 122 | 15.22 | 0.09 | 0.01 | 0.00 | 13.34 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 15 | 6 | 80 | 8 | 6 | 17 | 107 | 122 | 18.04 | 0.09 | 0.01 | 0.00 | 13.34 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 30 | 3 | 96 | 8 | 3 | 19 | 109 | 122 | 15.22 | 0.07 | 0.00 | 0.00 | 12.76 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 30 | 3 | 80 | 8 | 3 | 19 | 109 | 122 | 18.04 | 0.07 | 0.00 | 0.00 | 12.76 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 16 | 6 | 96 | 8 | 6 | 18 | 114 | 128 | 16.23 | 0.14 | 0.02 | 0.00 | 14.49 | entropy-retry-pass-weak-fork-delta |
| 2 | 8.91 | 16 | 6 | 80 | 8 | 6 | 18 | 114 | 128 | 19.24 | 0.14 | 0.02 | 0.00 | 14.49 | entropy-retry-pass-weak-fork-delta |
| 3 | 12.13 | 11 | 9 | 96 | 8 | 9 | 17 | 116 | 131 | 16.74 | 0.20 | 0.20 | 0.00 | 15.77 | entropy-retry-pass-weak-fork-delta |
| 3 | 12.13 | 11 | 9 | 80 | 8 | 9 | 17 | 116 | 131 | 19.85 | 0.20 | 0.20 | 0.00 | 15.77 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 3 | 448 | 10 | 4 | 17 | 121 | 136 | 2.97 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |
| 1 | 5.00 | 26 | 3 | 384 | 10 | 4 | 17 | 121 | 136 | 3.46 | 0.02 | 0.00 | 0.00 | 10.53 | entropy-retry-pass-weak-fork-delta |

## Best Rows With >=16 Bits Compact Fork-Delta Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 13 | 22.13 | 6 | 39 | 224 | 9 | 44 | 17 | 281 | 296 | 17.58 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 36 | 224 | 9 | 41 | 20 | 307 | 319 | 18.93 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 39 | 448 | 10 | 49 | 17 | 311 | 326 | 8.91 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 39 | 384 | 10 | 49 | 17 | 311 | 326 | 10.37 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 39 | 320 | 10 | 49 | 17 | 311 | 326 | 12.41 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 39 | 256 | 10 | 49 | 17 | 311 | 326 | 15.44 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |
| 11 | 23.09 | 8 | 33 | 224 | 9 | 38 | 24 | 328 | 336 | 19.83 | 9.42 | 17.24 | 0.20 | 59.08 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 36 | 448 | 10 | 45 | 20 | 335 | 347 | 9.60 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 36 | 384 | 10 | 45 | 20 | 335 | 347 | 11.17 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 36 | 320 | 10 | 45 | 20 | 335 | 347 | 13.36 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 12 | 22.83 | 7 | 36 | 256 | 10 | 45 | 20 | 335 | 347 | 16.62 | 10.00 | 17.50 | 0.28 | 48.00 | candidate-compact-fork-delta-16 |
| 13 | 22.13 | 6 | 39 | 768 | 11 | 54 | 17 | 341 | 356 | 5.23 | 10.49 | 17.08 | 0.34 | 36.74 | candidate-compact-fork-delta-16 |

## Best Rows With >=24 Bits Compact Fork-Delta Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 13 | 22.13 | 8 | 39 | 448 | 10 | 49 | 23 | 415 | 424 | 11.89 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |
| 13 | 22.13 | 8 | 39 | 384 | 10 | 49 | 23 | 415 | 424 | 13.83 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |
| 13 | 22.13 | 8 | 39 | 320 | 10 | 49 | 23 | 415 | 424 | 16.55 | 15.32 | 24.11 | 1.03 | 50.32 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 45 | 448 | 10 | 57 | 17 | 416 | 431 | 12.00 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 45 | 384 | 10 | 57 | 17 | 416 | 431 | 13.97 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 45 | 320 | 10 | 57 | 17 | 416 | 431 | 16.70 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 14 | 20.91 | 8 | 42 | 448 | 10 | 53 | 21 | 445 | 456 | 12.80 | 20.00 | 27.84 | 1.78 | 45.24 | candidate-compact-fork-delta-24 |
| 14 | 20.91 | 8 | 42 | 384 | 10 | 53 | 21 | 445 | 456 | 14.90 | 20.00 | 27.84 | 1.78 | 45.24 | candidate-compact-fork-delta-24 |
| 14 | 20.91 | 8 | 42 | 320 | 10 | 53 | 21 | 445 | 456 | 17.82 | 20.00 | 27.84 | 1.78 | 45.24 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 45 | 768 | 11 | 62 | 17 | 451 | 466 | 7.04 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 45 | 640 | 11 | 62 | 17 | 451 | 466 | 8.44 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |
| 15 | 19.00 | 7 | 45 | 512 | 11 | 62 | 17 | 451 | 466 | 10.52 | 24.00 | 27.33 | 2.02 | 34.91 | candidate-compact-fork-delta-24 |

## Best Rows With >=32 Bits Compact Fork-Delta Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 45 | 448 | 10 | 57 | 22 | 535 | 545 | 15.43 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 45 | 384 | 10 | 57 | 22 | 535 | 545 | 17.96 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 768 | 11 | 66 | 16 | 544 | 560 | 8.58 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 640 | 11 | 66 | 16 | 544 | 560 | 10.28 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 512 | 11 | 66 | 16 | 544 | 560 | 12.82 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 14 | 20.91 | 10 | 42 | 448 | 10 | 53 | 27 | 557 | 562 | 16.00 | 26.00 | 35.81 | 3.04 | 57.55 | candidate-compact-fork-delta-32 |
| 14 | 20.91 | 10 | 42 | 384 | 10 | 53 | 27 | 557 | 562 | 18.62 | 26.00 | 35.81 | 3.04 | 57.55 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 448 | 10 | 60 | 18 | 558 | 572 | 16.46 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 384 | 10 | 60 | 18 | 558 | 572 | 19.15 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 15 | 19.00 | 9 | 45 | 768 | 11 | 62 | 22 | 580 | 590 | 9.05 | 32.00 | 36.28 | 3.57 | 46.03 | candidate-compact-fork-delta-32 |

## Best Full-Block Coverage Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | active any-miss bits | compact fork-delta bits | one-fork compact bits | full-response delta bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 768 | 11 | 66 | 16 | 544 | 560 | 8.58 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 640 | 11 | 66 | 16 | 544 | 560 | 10.28 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 512 | 11 | 66 | 16 | 544 | 560 | 12.82 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 448 | 10 | 60 | 18 | 558 | 572 | 16.46 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 384 | 10 | 60 | 18 | 558 | 572 | 19.15 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 8 | 48 | 1024 | 12 | 72 | 16 | 592 | 608 | 6.45 | full | 36.00 | 4.04 | 36.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 768 | 11 | 66 | 18 | 612 | 626 | 9.66 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 640 | 11 | 66 | 18 | 612 | 626 | 11.57 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 9 | 48 | 512 | 11 | 66 | 18 | 612 | 626 | 14.42 | full | 41.00 | 5.02 | 41.00 | candidate-compact-fork-delta-32 |
| 16 | 16.00 | 10 | 48 | 448 | 10 | 60 | 20 | 620 | 632 | 18.28 | full | 46.00 | 6.01 | 46.00 | candidate-compact-fork-delta-32 |

## Conclusions

- The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1).
- The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes 3 free response coordinates.
- Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.
- Active-challenge coverage and fork-delta coverage are separate diagnostics.  The extraction proof needs a usable challenge delta on every block, not merely a challenge that touches the block once.
- Compact fork-delta coverage assumes 6 challenge draws for the same commitment per repeated round.  It only counts opposite signed active openings, because a compact transcript omits inactive response coordinates.
- The full-response fork-delta column is an idealized diagnostic for a transcript that exposes every response coordinate; it should not be used as the compact-signature security claim.
- All coverage bits are exact any-block-missed probabilities computed by inclusion-exclusion over the product blocks.  They are sharper than a union-bound-only diagnostic while still charging the real failure event.
- The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their fork-delta diagnostic is weak under compact active-only openings.
- Full-block challenges respond on all 48 coordinates; they remove the inactive-coordinate omission issue, but the forking proof still needs enough signed challenge draws to obtain opposite signs blockwise.
- The most promising small-signature route is now an intermediate compact fork-delta row plus an explicit multi-fork Fiat-Shamir theorem, not another scalar-challenge size screen.
