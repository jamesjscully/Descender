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

## Exact Sampler

For each active response coordinate with shift `|v| <= eta`, sample `y` uniformly from `[-z-eta,z+eta]`, output `r = y + v`, and accept iff `r in [-z,z]`.  For every allowed shift, the accepted response is exactly uniform on `[-z,z]`.

The per-coordinate acceptance probability is `(2z+1)/(2(z+eta)+1)`.  The total retry exponent is charged over active response coordinates only.

## Best Entropy + Retry Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 5.00 | 26 | 2 | 96 | 8 | 2 | 17 | 69 | 84 | 8.79 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 2 | 80 | 8 | 2 | 17 | 69 | 84 | 10.42 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 2 | 64 | 8 | 2 | 17 | 69 | 84 | 12.80 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 2 | 48 | 7 | 2 | 17 | 69 | 84 | 16.59 | 0.02 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 2 | 96 | 8 | 2 | 17 | 71 | 86 | 9.13 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 2 | 80 | 8 | 2 | 17 | 71 | 86 | 10.83 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 2 | 64 | 8 | 2 | 17 | 71 | 86 | 13.29 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 27 | 2 | 48 | 7 | 2 | 17 | 71 | 86 | 17.22 | 0.03 | 2.51 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 28 | 2 | 96 | 8 | 2 | 18 | 74 | 88 | 9.47 | 0.04 | 2.61 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 28 | 2 | 80 | 8 | 2 | 18 | 74 | 88 | 11.23 | 0.04 | 2.61 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 28 | 2 | 64 | 8 | 2 | 18 | 74 | 88 | 13.78 | 0.04 | 2.61 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 28 | 2 | 48 | 7 | 2 | 18 | 74 | 88 | 17.86 | 0.04 | 2.61 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 29 | 2 | 96 | 8 | 2 | 19 | 77 | 90 | 9.81 | 0.05 | 2.70 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 96 | 8 | 4 | 17 | 77 | 92 | 10.15 | 0.09 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 29 | 2 | 80 | 8 | 2 | 19 | 77 | 90 | 11.63 | 0.05 | 2.70 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 80 | 8 | 4 | 17 | 77 | 92 | 12.03 | 0.09 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 29 | 2 | 64 | 8 | 2 | 19 | 77 | 90 | 14.28 | 0.05 | 2.70 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 64 | 8 | 4 | 17 | 77 | 92 | 14.77 | 0.09 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 29 | 2 | 48 | 7 | 2 | 19 | 77 | 90 | 18.50 | 0.05 | 2.70 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 48 | 7 | 4 | 17 | 77 | 92 | 19.14 | 0.09 | 2.89 | entropy-retry-pass-weak-block-coverage |

## Best Rows With >=16 Bits Any-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 14 | 20.91 | 7 | 28 | 224 | 9 | 32 | 19 | 243 | 256 | 14.72 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 192 | 9 | 32 | 19 | 243 | 256 | 17.10 | 17.00 | 21.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 224 | 9 | 34 | 17 | 255 | 270 | 15.78 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 192 | 9 | 34 | 17 | 255 | 270 | 18.32 | 24.00 | 28.00 | candidate-coverage-24 |
| 14 | 20.91 | 7 | 28 | 448 | 10 | 35 | 19 | 264 | 277 | 7.47 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 384 | 10 | 35 | 19 | 264 | 277 | 8.69 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 320 | 10 | 35 | 19 | 264 | 277 | 10.39 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 256 | 10 | 35 | 19 | 264 | 277 | 12.93 | 17.00 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 8 | 28 | 224 | 9 | 32 | 21 | 277 | 288 | 16.83 | 20.00 | 24.00 | candidate-coverage-16 |
| 14 | 20.91 | 8 | 28 | 192 | 9 | 32 | 21 | 277 | 288 | 19.54 | 20.00 | 24.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 448 | 10 | 38 | 17 | 283 | 298 | 8.00 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 384 | 10 | 38 | 17 | 283 | 298 | 9.31 | 24.00 | 28.00 | candidate-coverage-24 |

## Best Rows With >=24 Bits Any-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 15 | 19.00 | 7 | 30 | 224 | 9 | 34 | 17 | 255 | 270 | 15.78 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 192 | 9 | 34 | 17 | 255 | 270 | 18.32 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 448 | 10 | 38 | 17 | 283 | 298 | 8.00 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 384 | 10 | 38 | 17 | 283 | 298 | 9.31 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 320 | 10 | 38 | 17 | 283 | 298 | 11.14 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 256 | 10 | 38 | 17 | 283 | 298 | 13.85 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 8 | 30 | 224 | 9 | 34 | 19 | 291 | 304 | 18.03 | 28.00 | 32.00 | candidate-coverage-24 |
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | full | candidate-full-block-coverage |
| 15 | 19.00 | 7 | 30 | 768 | 11 | 42 | 17 | 311 | 326 | 4.69 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 640 | 11 | 42 | 17 | 311 | 326 | 5.62 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 7 | 30 | 512 | 11 | 42 | 17 | 311 | 326 | 7.01 | 24.00 | 28.00 | candidate-coverage-24 |
| 15 | 19.00 | 8 | 30 | 448 | 10 | 38 | 19 | 323 | 336 | 9.14 | 28.00 | 32.00 | candidate-coverage-24 |

## Best Rows With >=32 Bits Any-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 448 | 10 | 40 | 16 | 336 | 352 | 9.75 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 384 | 10 | 40 | 16 | 336 | 352 | 11.35 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 320 | 10 | 40 | 16 | 336 | 352 | 13.58 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 256 | 10 | 40 | 16 | 336 | 352 | 16.89 | full | full | candidate-full-block-coverage |
| 15 | 19.00 | 9 | 30 | 448 | 10 | 38 | 22 | 364 | 374 | 10.29 | 32.00 | 36.00 | candidate-coverage-32 |
| 15 | 19.00 | 9 | 30 | 384 | 10 | 38 | 22 | 364 | 374 | 11.97 | 32.00 | 36.00 | candidate-coverage-32 |
| 15 | 19.00 | 9 | 30 | 320 | 10 | 38 | 22 | 364 | 374 | 14.32 | 32.00 | 36.00 | candidate-coverage-32 |
| 15 | 19.00 | 9 | 30 | 256 | 10 | 38 | 22 | 364 | 374 | 17.81 | 32.00 | 36.00 | candidate-coverage-32 |
| 16 | 16.00 | 8 | 32 | 768 | 11 | 44 | 16 | 368 | 384 | 5.72 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 640 | 11 | 44 | 16 | 368 | 384 | 6.86 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 512 | 11 | 44 | 16 | 368 | 384 | 8.55 | full | full | candidate-full-block-coverage |

## Best Full-Block Coverage Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | any-block miss bits | one-block miss bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 448 | 10 | 40 | 16 | 336 | 352 | 9.75 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 384 | 10 | 40 | 16 | 336 | 352 | 11.35 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 320 | 10 | 40 | 16 | 336 | 352 | 13.58 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 256 | 10 | 40 | 16 | 336 | 352 | 16.89 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 768 | 11 | 44 | 16 | 368 | 384 | 5.72 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 640 | 11 | 44 | 16 | 368 | 384 | 6.86 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 512 | 11 | 44 | 16 | 368 | 384 | 8.55 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 32 | 448 | 10 | 40 | 18 | 378 | 392 | 10.97 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 32 | 384 | 10 | 40 | 18 | 378 | 392 | 12.77 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 32 | 320 | 10 | 40 | 18 | 378 | 392 | 15.27 | full | full | candidate-full-block-coverage |
| 16 | 16.00 | 9 | 32 | 256 | 10 | 40 | 18 | 378 | 392 | 19.00 | full | full | candidate-full-block-coverage |

## Conclusions

- The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1).
- The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes 2 free response coordinates.
- Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.
- Coverage bits are exact any-block-missed probabilities computed by inclusion-exclusion over the product blocks.  They are sharper than a union-bound-only diagnostic while still charging the real failure event.
- The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their block-coverage diagnostic is weak: a prover missing one block can hope the challenge avoids it too often.
- Full-block challenges remove the coverage caveat at the cost of responding on all 32 coordinates; these rows are the cleanest conservative targets if proof-of-knowledge extraction must touch every block in each repetition.
- The most promising small-signature route is likely an intermediate coverage row plus a sharper soundness/extraction theorem, not a return to scalar challenges.
