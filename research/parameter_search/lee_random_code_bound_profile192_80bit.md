# Lee Random-Code Bound

- q: `16411`
- target centered Lee l1: `1173`
- m1: `56`
- failure budget: `2^-80.0`

| group size | groups | min rows/group | total rows | qamp ops | log2 expected bad |
|---:|---:|---:|---:|---:|---:|
| 1 | 56 | 17 | 952 | 952 | -83.23 |
| 2 | 28 | 18 | 504 | 1008 | -84.40 |
| 4 | 14 | 20 | 280 | 1120 | -86.59 |
| 7 | 8 | 22 | 176 | 1232 | -83.19 |
| 8 | 7 | 23 | 161 | 1288 | -84.85 |
| 14 | 4 | 27 | 108 | 1512 | -81.56 |
| 28 | 2 | 36 | 72 | 2016 | -82.55 |
| 56 | 1 | 51 | 51 | 2856 | -87.78 |

This is a random-code design bound, not a deterministic sidecar by itself.
It predicts how many shared random rows should make low-Lee relations rare.
A concrete deployment still needs a deterministic sidecar checker for the selected family.
