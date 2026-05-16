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

## Exact Sampler

For each active response coordinate with shift `|v| <= eta`, sample `y` uniformly from `[-z-eta,z+eta]`, output `r = y + v`, and accept iff `r in [-z,z]`.  For every allowed shift, the accepted response is exactly uniform on `[-z,z]`.

The per-coordinate acceptance probability is `(2z+1)/(2(z+eta)+1)`.  The total retry exponent is charged over active response coordinates only.

## Best Entropy + Retry Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 5.00 | 26 | 3 | 96 | 8 | 3 | 17 | 95 | 110 | 13.19 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 80 | 8 | 3 | 17 | 95 | 110 | 15.64 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 64 | 8 | 3 | 17 | 95 | 110 | 19.20 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 3 | 96 | 8 | 3 | 17 | 98 | 113 | 13.70 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 3 | 80 | 8 | 3 | 17 | 98 | 113 | 16.24 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 3 | 64 | 8 | 3 | 17 | 98 | 113 | 19.94 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 28 | 3 | 96 | 8 | 3 | 18 | 102 | 116 | 14.20 | 0.04 | 2.61 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 28 | 3 | 80 | 8 | 3 | 18 | 102 | 116 | 16.84 | 0.04 | 2.61 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 29 | 3 | 96 | 8 | 3 | 19 | 106 | 119 | 14.71 | 0.05 | 2.70 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 29 | 3 | 80 | 8 | 3 | 19 | 106 | 119 | 17.44 | 0.05 | 2.70 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 96 | 8 | 6 | 17 | 107 | 122 | 15.22 | 0.09 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 80 | 8 | 6 | 17 | 107 | 122 | 18.04 | 0.09 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 30 | 3 | 96 | 8 | 3 | 19 | 109 | 122 | 15.22 | 0.07 | 2.79 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 30 | 3 | 80 | 8 | 3 | 19 | 109 | 122 | 18.04 | 0.07 | 2.79 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 16 | 6 | 96 | 8 | 6 | 18 | 114 | 128 | 16.23 | 0.14 | 3.08 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 16 | 6 | 80 | 8 | 6 | 18 | 114 | 128 | 19.24 | 0.14 | 3.08 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 9 | 96 | 8 | 9 | 17 | 116 | 131 | 16.74 | 0.20 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 9 | 80 | 8 | 9 | 17 | 116 | 131 | 19.85 | 0.20 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 448 | 10 | 4 | 17 | 121 | 136 | 2.97 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 384 | 10 | 4 | 17 | 121 | 136 | 3.46 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |

## Best Rows With >=16 Bits Any-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 14 | 20.91 | 7 | 42 | 448 | 10 | 53 | 19 | 390 | 403 | 11.20 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 384 | 10 | 53 | 19 | 390 | 403 | 13.04 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 320 | 10 | 53 | 19 | 390 | 403 | 15.59 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 256 | 10 | 53 | 19 | 390 | 403 | 19.39 | 17.00 | 21.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 45 | 448 | 10 | 57 | 17 | 416 | 431 | 12.00 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 384 | 10 | 57 | 17 | 416 | 431 | 13.97 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 320 | 10 | 57 | 17 | 416 | 431 | 16.70 | 24.00 | 28.00 | candidate-coverage-24 |
| 14 | 20.91 | 7 | 42 | 768 | 11 | 58 | 19 | 425 | 438 | 6.57 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 640 | 11 | 58 | 19 | 425 | 438 | 7.87 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 512 | 11 | 58 | 19 | 425 | 438 | 9.82 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 8 | 42 | 448 | 10 | 53 | 21 | 445 | 456 | 12.80 | 20.00 | 24.00 | candidate-coverage-16 |
| 14 | 20.91 | 8 | 42 | 384 | 10 | 53 | 21 | 445 | 456 | 14.90 | 20.00 | 24.00 | candidate-coverage-16 |

## Best Rows With >=24 Bits Any-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 15 | 19.00 | 7 | 45 | 448 | 10 | 57 | 17 | 416 | 431 | 12.00 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 384 | 10 | 57 | 17 | 416 | 431 | 13.97 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 320 | 10 | 57 | 17 | 416 | 431 | 16.70 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 768 | 11 | 62 | 17 | 451 | 466 | 7.04 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 640 | 11 | 62 | 17 | 451 | 466 | 8.44 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 512 | 11 | 62 | 17 | 451 | 466 | 10.52 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 8 | 45 | 448 | 10 | 57 | 19 | 475 | 488 | 13.71 | 28.00 | 32.00 | candidate-coverage-24 |
| 15 | 19.00 | 8 | 45 | 384 | 10 | 57 | 19 | 475 | 488 | 15.96 | 28.00 | 32.00 | candidate-coverage-24 |
| 15 | 19.00 | 8 | 45 | 320 | 10 | 57 | 19 | 475 | 488 | 19.09 | 28.00 | 32.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 45 | 1024 | 12 | 68 | 17 | 493 | 508 | 5.29 | 24.00 | 28.00 | candidate-coverage-24 |
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | full | candidate-full-block-coverage |

## Best Rows With >=32 Bits Any-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | full | candidate-full-block-coverage |
| 15 | 19.00 | 9 | 45 | 448 | 10 | 57 | 22 | 535 | 545 | 15.43 | 32.00 | 36.00 | candidate-coverage-32 |
| 15 | 19.00 | 9 | 45 | 384 | 10 | 57 | 22 | 535 | 545 | 17.96 | 32.00 | 36.00 | candidate-coverage-32 |
| 16 | 16.00 | 8 | 48 | 768 | 11 | 66 | 16 | 544 | 560 | 8.58 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 640 | 11 | 66 | 16 | 544 | 560 | 10.28 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 512 | 11 | 66 | 16 | 544 | 560 | 12.82 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 448 | 10 | 60 | 18 | 558 | 572 | 16.46 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 384 | 10 | 60 | 18 | 558 | 572 | 19.15 | full | full | candidate-full-block-coverage |
| 15 | 19.00 | 9 | 45 | 768 | 11 | 62 | 22 | 580 | 590 | 9.05 | 32.00 | 36.00 | candidate-coverage-32 |
| 15 | 19.00 | 9 | 45 | 640 | 11 | 62 | 22 | 580 | 590 | 10.85 | 32.00 | 36.00 | candidate-coverage-32 |
| 15 | 19.00 | 9 | 45 | 512 | 11 | 62 | 22 | 580 | 590 | 13.52 | 32.00 | 36.00 | candidate-coverage-32 |

## Best Full-Block Coverage Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 768 | 11 | 66 | 16 | 544 | 560 | 8.58 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 640 | 11 | 66 | 16 | 544 | 560 | 10.28 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 512 | 11 | 66 | 16 | 544 | 560 | 12.82 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 448 | 10 | 60 | 18 | 558 | 572 | 16.46 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 384 | 10 | 60 | 18 | 558 | 572 | 19.15 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 1024 | 12 | 72 | 16 | 592 | 608 | 6.45 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 768 | 11 | 66 | 18 | 612 | 626 | 9.66 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 640 | 11 | 66 | 18 | 612 | 626 | 11.57 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 48 | 512 | 11 | 66 | 18 | 612 | 626 | 14.42 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 10 | 48 | 448 | 10 | 60 | 20 | 620 | 632 | 18.28 | full | full | candidate-full-block-coverage |

## Conclusions

- The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1).
- The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes 3 free response coordinates.
- Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.
- Coverage bits are exact any-block-missed probabilities computed by inclusion-exclusion over the product blocks.  They are sharper than a union-bound-only diagnostic while still charging the real failure event.
- The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their block-coverage diagnostic is weak: a prover missing one block can hope the challenge avoids it too often.
- Full-block challenges remove the coverage caveat at the cost of responding on all 48 coordinates; these rows are the cleanest conservative targets if proof-of-knowledge extraction must touch every block in each repetition.
- The most promising small-signature route is likely an intermediate coverage row plus a sharper soundness/extraction theorem, not a return to scalar challenges.
