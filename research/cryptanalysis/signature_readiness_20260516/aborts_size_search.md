# Signature Abort-Size Search

- q: `12289`
- n: `48`
- eta: `12`
- target challenge bits: `128.0`
- digest bytes: `32`
- max log2 expected retries: `20.0`

## Best Scalar FS-With-Aborts Rows

| model | challenge | entropy | reps | z bound | z bits | response bytes | sig bytes | log2 retries | noise headroom | verdict |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| scalar-fs-with-aborts | scalar c in [-511,511] \ {0} | 10.00 | 13 | 6144 | 14 | 84 | 1124 | 12.98 | 12 | candidate |
| scalar-fs-with-aborts | scalar c in [-511,511] \ {0} | 10.00 | 13 | 4096 | 14 | 84 | 1124 | 17.16 | 12 | candidate |
| scalar-fs-with-aborts | scalar c in [-383,383] \ {0} | 9.58 | 14 | 3072 | 13 | 78 | 1124 | 18.47 | 1548 | candidate |
| scalar-fs-with-aborts | scalar c in [-511,511] \ {0} | 10.00 | 13 | 8192 | 15 | 90 | 1202 | 10.48 | 12 | candidate |
| scalar-fs-with-aborts | scalar c in [-255,255] \ {0} | 8.99 | 15 | 3072 | 13 | 78 | 1202 | 14.96 | 3084 | candidate |
| scalar-fs-with-aborts | scalar c in [-255,255] \ {0} | 8.99 | 15 | 2048 | 13 | 78 | 1202 | 19.77 | 3084 | candidate |
| scalar-fs-with-aborts | scalar c in [-383,383] \ {0} | 9.58 | 14 | 6144 | 14 | 84 | 1208 | 11.28 | 1548 | candidate |
| scalar-fs-with-aborts | scalar c in [-383,383] \ {0} | 9.58 | 14 | 4096 | 14 | 84 | 1208 | 15.20 | 1548 | candidate |
| scalar-fs-with-aborts | scalar c in [-127,127] \ {0} | 7.99 | 17 | 1536 | 12 | 72 | 1256 | 16.90 | 4620 | candidate |
| scalar-fs-with-aborts | scalar c in [-63,63] \ {0} | 6.98 | 19 | 768 | 11 | 66 | 1286 | 18.78 | 5388 | candidate |
| scalar-fs-with-aborts | scalar c in [-255,255] \ {0} | 8.99 | 15 | 6144 | 14 | 84 | 1292 | 8.75 | 3084 | candidate |
| scalar-fs-with-aborts | scalar c in [-383,383] \ {0} | 9.58 | 14 | 8192 | 15 | 90 | 1292 | 8.99 | 1548 | candidate |
| scalar-fs-with-aborts | scalar c in [-255,255] \ {0} | 8.99 | 15 | 4096 | 14 | 84 | 1292 | 12.07 | 3084 | candidate |
| scalar-fs-with-aborts | scalar c in [-127,127] \ {0} | 7.99 | 17 | 3072 | 13 | 78 | 1358 | 9.88 | 4620 | candidate |
| scalar-fs-with-aborts | scalar c in [-127,127] \ {0} | 7.99 | 17 | 2048 | 13 | 78 | 1358 | 13.64 | 4620 | candidate |
| scalar-fs-with-aborts | scalar c in [-255,255] \ {0} | 8.99 | 15 | 8192 | 15 | 90 | 1382 | 6.87 | 3084 | candidate |
| scalar-fs-with-aborts | scalar c in [-63,63] \ {0} | 6.98 | 19 | 1536 | 12 | 72 | 1400 | 10.97 | 5388 | candidate |
| scalar-fs-with-aborts | scalar c in [-63,63] \ {0} | 6.98 | 19 | 1024 | 12 | 72 | 1400 | 15.15 | 5388 | candidate |
| scalar-fs-with-aborts | scalar c in [-127,127] \ {0} | 7.99 | 17 | 6144 | 14 | 84 | 1460 | 5.43 | 4620 | candidate |
| scalar-fs-with-aborts | scalar c in [-127,127] \ {0} | 7.99 | 17 | 4096 | 14 | 84 | 1460 | 7.76 | 4620 | candidate |

## Best Hypothetical Product-Block Challenge Rows

| model | challenge | entropy | reps | z bound | z bits | response bytes | sig bytes | log2 retries | noise headroom | verdict |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| product-block-sparse-challenge | choose 11 signed product blocks among 16 | 23.09 | 6 | 12 | 5 | 30 | 212 | 5.83 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 10 signed product blocks among 16 | 22.97 | 6 | 12 | 5 | 30 | 212 | 5.83 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 12 signed product blocks among 16 | 22.83 | 6 | 12 | 5 | 30 | 212 | 5.83 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 9 signed product blocks among 16 | 22.48 | 6 | 12 | 5 | 30 | 212 | 5.83 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 13 signed product blocks among 16 | 22.13 | 6 | 12 | 5 | 30 | 212 | 5.83 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 8 signed product blocks among 16 | 21.65 | 6 | 12 | 5 | 30 | 212 | 5.83 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 14 signed product blocks among 16 | 20.91 | 7 | 12 | 5 | 30 | 242 | 6.80 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 7 signed product blocks among 16 | 20.48 | 7 | 12 | 5 | 30 | 242 | 6.80 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 15 signed product blocks among 16 | 19.00 | 7 | 12 | 5 | 30 | 242 | 6.80 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 6 signed product blocks among 16 | 18.97 | 7 | 12 | 5 | 30 | 242 | 6.80 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 11 signed product blocks among 16 | 23.09 | 6 | 24 | 6 | 36 | 248 | 3.45 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 10 signed product blocks among 16 | 22.97 | 6 | 24 | 6 | 36 | 248 | 3.45 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 12 signed product blocks among 16 | 22.83 | 6 | 24 | 6 | 36 | 248 | 3.45 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 9 signed product blocks among 16 | 22.48 | 6 | 24 | 6 | 36 | 248 | 3.45 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 13 signed product blocks among 16 | 22.13 | 6 | 24 | 6 | 36 | 248 | 3.45 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 8 signed product blocks among 16 | 21.65 | 6 | 24 | 6 | 36 | 248 | 3.45 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 11 signed product blocks among 16 | 23.09 | 6 | 16 | 6 | 36 | 248 | 4.73 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 10 signed product blocks among 16 | 22.97 | 6 | 16 | 6 | 36 | 248 | 4.73 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 12 signed product blocks among 16 | 22.83 | 6 | 16 | 6 | 36 | 248 | 4.73 | 6132 | size-promising-proof-gap |
| product-block-sparse-challenge | choose 9 signed product blocks among 16 | 22.48 | 6 | 16 | 6 | 36 | 248 | 4.73 | 6132 | size-promising-proof-gap |

## Conclusions

- Scalar Fiat-Shamir-with-aborts is the conservative algebraic route because the verifier can multiply the public relation by a scalar challenge, but its best small-signature rows are still around the kilobyte scale under practical retry limits.
- A high-entropy product-block challenge with coefficients in {-1,0,1} would be dramatically smaller, because it keeps the secret/noise multiplier at one while extracting many challenge bits per repetition.
- The product-block challenge rows are not currently schemes, but the first algebraic blocker is now resolved for block-diagonal systematic views: the public relation and verifier dual map commute with block-local challenge scaling.
- If sub-kilobyte signatures are mandatory, the research target should now move from equivariance to the actual signature proof: rejection sampling, the noisy/high-bit verifier equation, and Fiat-Shamir soundness for sparse signed product-block challenges.
