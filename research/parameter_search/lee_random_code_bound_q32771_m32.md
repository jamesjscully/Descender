# Lee Random-Code Bound

- q: `32771`
- target centered Lee l1: `3277`
- m1: `32`
- failure budget: `2^-80.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 1 | 32 | 18 | 576 | 576 | -80.88 |
| 2 | 16 | 19 | 304 | 608 | -80.24 |
| 4 | 8 | 22 | 176 | 704 | -85.76 |
| 8 | 4 | 26 | 104 | 832 | -84.75 |
| 16 | 2 | - | - | - | - |
| 32 | 1 | - | - | - | - |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
