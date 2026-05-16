# Lee Random-Code Bound

- q: `32771`
- target centered Lee l1: `4097`
- m1: `32`
- failure budget: `2^-80.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 1 | 32 | 19 | 608 | 608 | -81.08 |
| 2 | 16 | 21 | 336 | 672 | -86.45 |
| 4 | 8 | 23 | 184 | 736 | -84.13 |
| 8 | 4 | 27 | 108 | 864 | -80.92 |
| 16 | 2 | 35 | 70 | 1120 | -80.88 |
| 32 | 1 | 49 | 49 | 1568 | -83.16 |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
