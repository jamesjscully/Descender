# Shared Row Lee Certificate

| field | value |
|---|---:|
| family | shared-random-pair-block |
| q | 32771 |
| n | 80 |
| m1 | 32 |
| group size | 2 |
| group count | 16 |
| rows per group | 5 |
| used pivot rows | 80 |
| unused pivot rows | 0 |
| target centered Lee l1 | 3277 |
| certified centered Lee l1 | 3277 |
| passes target | true |
| global systematic Lee certified | true |
| keygen multiply ops | 160 |
| selected trial | 2 |
| checked centered pairs | 21470904 |
| worst pair | (-614, -14) |
| worst relation support | 7 |
| Lean route | productBlockComposableSystematicLeeCertificate -> composableSystematicLeeDualCertificate |

Exact check: `enumerated_all_centered_pairs_with_free_l1_below_target`

Global statement: For the direct sum of 16 identical certified pair blocks, every nonzero global free vector activates a certified local slice; therefore SystematicLeeDistanceAtLeast q P target_l1 holds globally when passes_target is true.

Coefficients for the selected two-free-coordinate block:

```json
[
  [
    3604,
    19764
  ],
  [
    17179,
    2683
  ],
  [
    26747,
    13757
  ],
  [
    5265,
    17244
  ],
  [
    27751,
    30589
  ]
]
```
