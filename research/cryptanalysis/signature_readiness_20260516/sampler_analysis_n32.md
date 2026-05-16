# Product-Block Signature Sampler Analysis

- q: `12289`
- n: `32`
- eta: `12`
- product blocks: `16`
- local free coordinates per block: `2`
- target challenge bits: `128.0`
- digest bytes: `32`
- max log2 expected retries: `20.0`

## Exact Sampler

For each active response coordinate with shift `|v| <= eta`, sample `y` uniformly from `[-z-eta,z+eta]`, output `r = y + v`, and accept iff `r in [-z,z]`.  For every allowed shift, the accepted response is exactly uniform on `[-z,z]`.

The per-coordinate acceptance probability is `(2z+1)/(2(z+eta)+1)`.  The total retry exponent is charged over active response coordinates only.

## Best Entropy + Retry Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 5.00 | 26 | 2 | 96 | 8 | 2 | 17 | 69 | 84 | 8.79 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 2 | 80 | 8 | 2 | 17 | 69 | 84 | 10.42 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 2 | 64 | 8 | 2 | 17 | 69 | 84 | 12.80 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 2 | 48 | 7 | 2 | 17 | 69 | 84 | 16.59 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 96 | 8 | 4 | 17 | 77 | 92 | 10.15 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 80 | 8 | 4 | 17 | 77 | 92 | 12.03 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 64 | 8 | 4 | 17 | 77 | 92 | 14.77 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 48 | 7 | 4 | 17 | 77 | 92 | 19.14 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 6 | 96 | 8 | 6 | 17 | 83 | 98 | 11.16 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 6 | 80 | 8 | 6 | 17 | 83 | 98 | 13.23 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 6 | 64 | 8 | 6 | 17 | 83 | 98 | 16.25 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 4 | 14.83 | 9 | 8 | 96 | 8 | 8 | 17 | 89 | 104 | 12.17 | 3.74 | entropy-retry-pass-weak-block-coverage |
| 4 | 14.83 | 9 | 8 | 80 | 8 | 8 | 17 | 89 | 104 | 14.43 | 3.74 | entropy-retry-pass-weak-block-coverage |
| 4 | 14.83 | 9 | 8 | 64 | 8 | 8 | 17 | 89 | 104 | 17.72 | 3.74 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 448 | 10 | 5 | 17 | 92 | 107 | 2.29 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 384 | 10 | 5 | 17 | 92 | 107 | 2.66 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 320 | 10 | 5 | 17 | 92 | 107 | 3.18 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 256 | 10 | 5 | 17 | 92 | 107 | 3.96 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 224 | 9 | 5 | 17 | 92 | 107 | 4.51 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 4 | 192 | 9 | 5 | 17 | 92 | 107 | 5.23 | 2.89 | entropy-retry-pass-weak-block-coverage |

## Best Rows With >=16 Bits Missing-One-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 14 | 20.91 | 7 | 28 | 224 | 9 | 32 | 19 | 243 | 256 | 14.72 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 192 | 9 | 32 | 19 | 243 | 256 | 17.10 | 21.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 224 | 9 | 34 | 17 | 255 | 270 | 15.78 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 192 | 9 | 34 | 17 | 255 | 270 | 18.32 | 28.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 448 | 10 | 35 | 19 | 264 | 277 | 7.47 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 384 | 10 | 35 | 19 | 264 | 277 | 8.69 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 320 | 10 | 35 | 19 | 264 | 277 | 10.39 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 28 | 256 | 10 | 35 | 19 | 264 | 277 | 12.93 | 21.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 448 | 10 | 38 | 17 | 283 | 298 | 8.00 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 384 | 10 | 38 | 17 | 283 | 298 | 9.31 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 320 | 10 | 38 | 17 | 283 | 298 | 11.14 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 30 | 256 | 10 | 38 | 17 | 283 | 298 | 13.85 | 28.00 | candidate-coverage-16 |

## Best Rows With >=32 Bits Missing-One-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 448 | 10 | 40 | 16 | 336 | 352 | 9.75 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 384 | 10 | 40 | 16 | 336 | 352 | 11.35 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 320 | 10 | 40 | 16 | 336 | 352 | 13.58 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 256 | 10 | 40 | 16 | 336 | 352 | 16.89 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 768 | 11 | 44 | 16 | 368 | 384 | 5.72 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 640 | 11 | 44 | 16 | 368 | 384 | 6.86 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 512 | 11 | 44 | 16 | 368 | 384 | 8.55 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 1024 | 12 | 48 | 16 | 400 | 416 | 4.30 | full | candidate-full-block-coverage |

## Best Full-Block Coverage Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 32 | 224 | 9 | 36 | 16 | 304 | 320 | 19.23 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 448 | 10 | 40 | 16 | 336 | 352 | 9.75 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 384 | 10 | 40 | 16 | 336 | 352 | 11.35 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 320 | 10 | 40 | 16 | 336 | 352 | 13.58 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 256 | 10 | 40 | 16 | 336 | 352 | 16.89 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 768 | 11 | 44 | 16 | 368 | 384 | 5.72 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 640 | 11 | 44 | 16 | 368 | 384 | 6.86 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 512 | 11 | 44 | 16 | 368 | 384 | 8.55 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 32 | 1024 | 12 | 48 | 16 | 400 | 416 | 4.30 | full | candidate-full-block-coverage |

## Conclusions

- The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1).
- The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes 2 free response coordinates.
- Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.
- The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their block-coverage diagnostic is weak: a prover missing one block can hope the challenge avoids it too often.
- Full-block challenges remove the coverage caveat at the cost of responding on all 32 coordinates; these rows are the cleanest conservative targets if proof-of-knowledge extraction must touch every block in each repetition.
- The most promising small-signature route is likely an intermediate coverage row plus a sharper soundness/extraction theorem, not a return to scalar challenges.
