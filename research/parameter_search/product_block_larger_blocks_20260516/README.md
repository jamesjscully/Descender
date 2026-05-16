# Larger Product-Block Design Bound, 2026-05-16

This directory records the random-code design bound for larger local block
sizes at eta 5.  It is not an exact sidecar library; it identifies where the
next proof-object generalization is likely to pay off.

For each `q`, the run used:

```text
group sizes: 2,4,8,16,32
m1: 32
failure budget: 2^-80
```

## Minimum Rows Under The Bound

| q | target l1 | g=2 rows | g=4 rows | g=8 rows | g=16 rows | g=32 rows |
|---:|---:|---:|---:|---:|---:|---:|
| 12289 | 1229 | 304 | 168 | 96 | 60 | 41 |
| 32771 | 3277 | 304 | 176 | 104 | 66 | 46 |
| 40961 | 4096 | 320 | 176 | 104 | 68 | 47 |
| 65537 | 6554 | 320 | 176 | 108 | 70 | 50 |

Interpretation: larger local blocks should reduce pivot-row cost dramatically,
but they need a different exact verifier than the current pair enumerator.
The next proof extension should target `g=8` or `g=16` first: they capture most
of the row reduction without the full complexity of a `g=32` local code.
