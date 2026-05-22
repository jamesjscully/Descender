# Quotient-Level Coordinate Pair Sweep

Status: LLL sweep complete; partial exact enumeration complete.

This is a sidecar Milestone 1 artifact. It does not change the current
q12289 account-carried proof path or implementation.

## Summary

| item | value |
|---|---:|
| pairs tested | `253` |
| exact margin floor | `43` |
| control candidate | `coordinate-link-0-3` |
| control LLL margin | `43` |
| control reviewed exact margin | `43` |
| max LLL candidate | `coordinate-link-1-14` |
| max LLL margin | `94` |
| exact-enum artifacts completed | `4` |
| max exact margin | `94` |
| standalone semantic-factor candidates | `0` |
| exact completed standalone semantic-factor candidates | `0` |
| remaining pairs at or above exact floor | `77` |

Semantic screen: no two-coordinate projection candidate standalone
determines the 12-row semantic quotient.  Exact-passed candidates are
therefore auxiliary account/bounded-kernel rows for the product quotient
`semantic x link`, not standalone replacements for the semantic digest.

## Top Candidates

| candidate | pair | LLL margin | exact margin | rank | fiber dim | semantic class | status |
|---|---|---:|---:|---:|---:|---|---|
| coordinate-link-1-14 | [1, 14] | 94 | 94 | 14 | 9 | auxiliary_product_quotient_only | exact_infeasible |
| coordinate-link-8-20 | [8, 20] | 94 | 94 | 14 | 9 | auxiliary_product_quotient_only | exact_infeasible |
| coordinate-link-14-15 | [14, 15] | 94 | 94 | 14 | 9 | auxiliary_product_quotient_only | exact_infeasible |
| coordinate-link-3-15 | [3, 15] | 91 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-11-14 | [11, 14] | 89 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-2-11 | [2, 11] | 86 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-6-22 | [6, 22] | 86 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-11-20 | [11, 20] | 86 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-1-8 | [1, 8] | 82 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-1-3 | [1, 3] | 81 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-3-17 | [3, 17] | 81 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-0-14 | [0, 14] | 78 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-1-15 | [1, 15] | 78 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-2-5 | [2, 5] | 77 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-2-14 | [2, 14] | 77 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-2-13 | [2, 13] | 75 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-5-17 | [5, 17] | 74 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-16-17 | [16, 17] | 74 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-8-21 | [8, 21] | 72 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |
| coordinate-link-14-20 | [14, 20] | 69 | - | 14 | 9 | auxiliary_product_quotient_only | needs_exact_enum |

## Exact-Enum Completed

| candidate | pair | semantic class | status | shortest bound | exact margin | artifact |
|---|---|---|---|---:|---:|---|
| coordinate-link-1-14 | [1, 14] | auxiliary_product_quotient_only | infeasible | 190 | 94 | `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-1-14_b96.json` |
| coordinate-link-8-20 | [8, 20] | auxiliary_product_quotient_only | infeasible | 190 | 94 | `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-8-20_b96.json` |
| coordinate-link-14-15 | [14, 15] | auxiliary_product_quotient_only | infeasible | 190 | 94 | `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-14-15_b96.json` |
| coordinate-link-1-2 | [1, 2] | auxiliary_product_quotient_only | infeasible | 115 | 19 | `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-1-2_b96.json` |

## Exact-Enum Queue

Rows below met or exceeded the configured LLL margin floor. They are
not certified until exact enumeration completes.

| candidate | pair | LLL margin |
|---|---|---:|
| coordinate-link-3-15 | [3, 15] | 91 |
| coordinate-link-11-14 | [11, 14] | 89 |
| coordinate-link-2-11 | [2, 11] | 86 |
| coordinate-link-6-22 | [6, 22] | 86 |
| coordinate-link-11-20 | [11, 20] | 86 |
| coordinate-link-1-8 | [1, 8] | 82 |
| coordinate-link-1-3 | [1, 3] | 81 |
| coordinate-link-3-17 | [3, 17] | 81 |
| coordinate-link-0-14 | [0, 14] | 78 |
| coordinate-link-1-15 | [1, 15] | 78 |
| coordinate-link-2-5 | [2, 5] | 77 |
| coordinate-link-2-14 | [2, 14] | 77 |
| coordinate-link-2-13 | [2, 13] | 75 |
| coordinate-link-5-17 | [5, 17] | 74 |
| coordinate-link-16-17 | [16, 17] | 74 |
| coordinate-link-8-21 | [8, 21] | 72 |
| coordinate-link-14-20 | [14, 20] | 69 |
| coordinate-link-20-21 | [20, 21] | 69 |
| coordinate-link-1-18 | [1, 18] | 67 |
| coordinate-link-1-16 | [1, 16] | 66 |
| coordinate-link-5-18 | [5, 18] | 66 |
| coordinate-link-20-22 | [20, 22] | 66 |
| coordinate-link-0-15 | [0, 15] | 65 |
| coordinate-link-1-19 | [1, 19] | 65 |
| coordinate-link-4-5 | [4, 5] | 65 |
| coordinate-link-4-12 | [4, 12] | 65 |
| coordinate-link-7-18 | [7, 18] | 64 |
| coordinate-link-4-22 | [4, 22] | 61 |
| coordinate-link-8-16 | [8, 16] | 60 |
| coordinate-link-5-19 | [5, 19] | 59 |
| coordinate-link-5-20 | [5, 20] | 59 |
| coordinate-link-6-13 | [6, 13] | 59 |
| coordinate-link-3-13 | [3, 13] | 57 |
| coordinate-link-6-9 | [6, 9] | 56 |
| coordinate-link-8-18 | [8, 18] | 56 |
| coordinate-link-1-4 | [1, 4] | 54 |
| coordinate-link-3-22 | [3, 22] | 54 |
| coordinate-link-6-8 | [6, 8] | 54 |
| coordinate-link-9-11 | [9, 11] | 54 |
| coordinate-link-9-12 | [9, 12] | 54 |
| coordinate-link-13-16 | [13, 16] | 54 |
| coordinate-link-6-21 | [6, 21] | 53 |
| coordinate-link-9-14 | [9, 14] | 53 |
| coordinate-link-18-19 | [18, 19] | 53 |
| coordinate-link-0-6 | [0, 6] | 52 |
| coordinate-link-17-18 | [17, 18] | 52 |
| coordinate-link-1-9 | [1, 9] | 51 |
| coordinate-link-6-18 | [6, 18] | 51 |
| coordinate-link-6-19 | [6, 19] | 51 |
| coordinate-link-6-20 | [6, 20] | 51 |
| coordinate-link-9-21 | [9, 21] | 51 |
| coordinate-link-10-18 | [10, 18] | 51 |
| coordinate-link-10-21 | [10, 21] | 51 |
| coordinate-link-18-21 | [18, 21] | 51 |
| coordinate-link-2-22 | [2, 22] | 49 |
| coordinate-link-11-18 | [11, 18] | 49 |
| coordinate-link-0-18 | [0, 18] | 48 |
| coordinate-link-1-17 | [1, 17] | 48 |
| coordinate-link-0-4 | [0, 4] | 47 |
| coordinate-link-0-11 | [0, 11] | 47 |
| coordinate-link-1-13 | [1, 13] | 47 |
| coordinate-link-3-9 | [3, 9] | 47 |
| coordinate-link-3-14 | [3, 14] | 47 |
| coordinate-link-7-13 | [7, 13] | 47 |
| coordinate-link-8-15 | [8, 15] | 47 |
| coordinate-link-5-6 | [5, 6] | 46 |
| coordinate-link-6-11 | [6, 11] | 46 |
| coordinate-link-7-17 | [7, 17] | 46 |
| coordinate-link-0-16 | [0, 16] | 45 |
| coordinate-link-1-21 | [1, 21] | 45 |
| coordinate-link-7-10 | [7, 10] | 45 |
| coordinate-link-17-22 | [17, 22] | 45 |
| coordinate-link-8-17 | [8, 17] | 44 |
| coordinate-link-9-16 | [9, 16] | 44 |
| coordinate-link-11-13 | [11, 13] | 44 |
| coordinate-link-0-3 | [0, 3] | 43 |
| coordinate-link-9-13 | [9, 13] | 43 |

## Interpretation

LLL sweep ranks coordinate pairs only. Rows with strong LLL margins must be promoted to exact enumeration before being considered bounded-kernel passes. Coordinate candidates with exact-enum artifacts have passed only this local sidecar enumeration; they still require hiding review and independent cryptanalytic replay before any deployment claim.
