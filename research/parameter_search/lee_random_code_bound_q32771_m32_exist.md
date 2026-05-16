# Lee Random-Code Bound

- q: `32771`
- target centered Lee l1: `3277`
- m1: `32`
- failure budget: `2^-0.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 1 | 32 | 5 | 160 | 160 | -3.43 |
| 2 | 16 | 7 | 112 | 224 | -5.37 |
| 4 | 8 | 10 | 80 | 320 | -5.85 |
| 8 | 4 | 15 | 60 | 480 | -5.86 |
| 16 | 2 | 23 | 46 | 736 | -3.40 |
| 32 | 1 | - | - | - | - |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
