# Lee Random-Code Bound

- q: `65537`
- target centered Lee l1: `3277`
- m1: `32`
- failure budget: `2^-80.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 1 | 32 | 16 | 512 | 512 | -83.82 |
| 2 | 16 | 17 | 272 | 544 | -83.87 |
| 4 | 8 | 19 | 152 | 608 | -83.86 |
| 8 | 4 | 23 | 92 | 736 | -85.65 |
| 16 | 2 | 30 | 60 | 960 | -87.62 |
| 32 | 1 | 41 | 41 | 1312 | -81.47 |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
