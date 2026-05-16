# Solved Lee Certificate

| field | value |
|---|---:|
| family | optimized-scalar-block |
| q | 32771 |
| n | 64 |
| m1 | 32 |
| rows per free coordinate | 2 |
| used pivot rows | 64 |
| unused pivot rows | 0 |
| certified centered Lee l1 | 1705 |
| target l1 | 3277 |
| passes target | false |
| keygen multiply ops | 64 |
| worst coordinate | 0 |
| worst free value mod q | 413 |
| worst relation support | 3 |

Exact check: `block_diagonal_scalar_exact_over_all_nonzero_free_values`

This certificate is rigorous for the block-diagonal scalar solved-map family. It should be treated as a construction-side certificate for `direct-solved-lee`, not as evidence about arbitrary generated tilt matrices.
