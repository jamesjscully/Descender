# Lee Random-Code Bound

- q: `65537`
- target centered Lee l1: `2979`
- m1: `64`
- failure budget: `2^-80.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 1 | 64 | 16 | 1024 | 1024 | -85.16 |
| 2 | 32 | 17 | 544 | 1088 | -85.49 |
| 4 | 16 | 19 | 304 | 1216 | -86.02 |
| 8 | 8 | 22 | 176 | 1408 | -80.50 |
| 16 | 4 | 29 | 116 | 1856 | -83.97 |
| 32 | 2 | 40 | 80 | 2560 | -80.86 |
| 64 | 1 | 59 | 59 | 3776 | -82.79 |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
