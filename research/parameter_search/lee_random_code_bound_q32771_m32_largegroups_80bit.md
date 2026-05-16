# Lee Random-Code Bound

- q: `32771`
- target centered Lee l1: `3277`
- m1: `32`
- failure budget: `2^-80.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 8 | 4 | 26 | 104 | 832 | -84.75 |
| 16 | 2 | 33 | 66 | 1056 | -81.35 |
| 32 | 1 | 46 | 46 | 1472 | -83.32 |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
