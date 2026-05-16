# Product-Block Signature Sampler Analysis

- q: `12289`
- n: `48`
- eta: `12`
- product blocks: `16`
- local free coordinates per block: `3`
- target challenge bits: `128.0`
- digest bytes: `32`
- max log2 expected retries: `20.0`

## Exact Sampler

For each active response coordinate with shift `|v| <= eta`, sample `y` uniformly from `[-z-eta,z+eta]`, output `r = y + v`, and accept iff `r in [-z,z]`.  For every allowed shift, the accepted response is exactly uniform on `[-z,z]`.

The per-coordinate acceptance probability is `(2z+1)/(2(z+eta)+1)`.  The total retry exponent is charged over active response coordinates only.

## Best Entropy + Retry Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 5.00 | 26 | 3 | 96 | 8 | 3 | 17 | 95 | 110 | 13.19 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 80 | 8 | 3 | 17 | 95 | 110 | 15.64 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 64 | 8 | 3 | 17 | 95 | 110 | 19.20 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 96 | 8 | 6 | 17 | 107 | 122 | 15.22 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 80 | 8 | 6 | 17 | 107 | 122 | 18.04 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 9 | 96 | 8 | 9 | 17 | 116 | 131 | 16.74 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 3 | 12.13 | 11 | 9 | 80 | 8 | 9 | 17 | 116 | 131 | 19.85 | 3.30 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 448 | 10 | 4 | 17 | 121 | 136 | 2.97 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 384 | 10 | 4 | 17 | 121 | 136 | 3.46 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 320 | 10 | 4 | 17 | 121 | 136 | 4.14 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 256 | 10 | 4 | 17 | 121 | 136 | 5.15 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 224 | 9 | 4 | 17 | 121 | 136 | 5.86 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 192 | 9 | 4 | 17 | 121 | 136 | 6.80 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 160 | 9 | 4 | 17 | 121 | 136 | 8.11 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 1 | 5.00 | 26 | 3 | 128 | 9 | 4 | 17 | 121 | 136 | 10.05 | 2.42 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 224 | 9 | 7 | 17 | 122 | 137 | 6.76 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 192 | 9 | 7 | 17 | 122 | 137 | 7.85 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 160 | 9 | 7 | 17 | 122 | 137 | 9.36 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 2 | 8.91 | 15 | 6 | 128 | 9 | 7 | 17 | 122 | 137 | 11.59 | 2.89 | entropy-retry-pass-weak-block-coverage |
| 4 | 14.83 | 9 | 12 | 96 | 8 | 12 | 17 | 125 | 140 | 18.26 | 3.74 | entropy-retry-pass-weak-block-coverage |

## Best Rows With >=16 Bits Missing-One-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 14 | 20.91 | 7 | 42 | 448 | 10 | 53 | 19 | 390 | 403 | 11.20 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 384 | 10 | 53 | 19 | 390 | 403 | 13.04 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 320 | 10 | 53 | 19 | 390 | 403 | 15.59 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 256 | 10 | 53 | 19 | 390 | 403 | 19.39 | 21.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 45 | 448 | 10 | 57 | 17 | 416 | 431 | 12.00 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 45 | 384 | 10 | 57 | 17 | 416 | 431 | 13.97 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 45 | 320 | 10 | 57 | 17 | 416 | 431 | 16.70 | 28.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 768 | 11 | 58 | 19 | 425 | 438 | 6.57 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 640 | 11 | 58 | 19 | 425 | 438 | 7.87 | 21.00 | candidate-coverage-16 |
| 14 | 20.91 | 7 | 42 | 512 | 11 | 58 | 19 | 425 | 438 | 9.82 | 21.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 45 | 768 | 11 | 62 | 17 | 451 | 466 | 7.04 | 28.00 | candidate-coverage-16 |
| 15 | 19.00 | 7 | 45 | 640 | 11 | 62 | 17 | 451 | 466 | 8.44 | 28.00 | candidate-coverage-16 |

## Best Rows With >=32 Bits Missing-One-Block Coverage

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 768 | 11 | 66 | 16 | 544 | 560 | 8.58 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 640 | 11 | 66 | 16 | 544 | 560 | 10.28 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 512 | 11 | 66 | 16 | 544 | 560 | 12.82 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 1024 | 12 | 72 | 16 | 592 | 608 | 6.45 | full | candidate-full-block-coverage |

## Best Full-Block Coverage Rows

| weight | entropy | reps | active coords | z bound | z bits | response/rep | challenge bytes | sig bytes | sig bytes w/32B hash | log2 retries | missing-one-block bits | verdict |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 16 | 16.00 | 8 | 48 | 448 | 10 | 60 | 16 | 496 | 512 | 14.63 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 384 | 10 | 60 | 16 | 496 | 512 | 17.03 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 768 | 11 | 66 | 16 | 544 | 560 | 8.58 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 640 | 11 | 66 | 16 | 544 | 560 | 10.28 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 512 | 11 | 66 | 16 | 544 | 560 | 12.82 | full | candidate-full-block-coverage |
| 16 | 16.00 | 8 | 48 | 1024 | 12 | 72 | 16 | 592 | 608 | 6.45 | full | candidate-full-block-coverage |

## Conclusions

- The exact-box sampler is concrete: for each active coordinate, sample y uniformly from [-z-eta,z+eta], output y+shift, and accept iff the response lies in [-z,z].  This exactly realizes the target law with per-coordinate acceptance (2z+1)/(2(z+eta)+1).
- The rejection exponent is active coordinates, not one scalar.  For this profile each active product block contributes 3 free response coordinates.
- Signature sizes are reported with canonical challenge encoding.  Carrying a fixed 32-byte hash/digest instead adds the difference shown in the digest-size column.
- The smallest rows pass challenge-entropy and retry accounting by using very low block weight, but their block-coverage diagnostic is weak: a prover missing one block can hope the challenge avoids it too often.
- Full-block challenges remove the coverage caveat at the cost of responding on all 48 coordinates; these rows are the cleanest conservative targets if proof-of-knowledge extraction must touch every block in each repetition.
- The most promising small-signature route is likely an intermediate coverage row plus a sharper soundness/extraction theorem, not a return to scalar challenges.
