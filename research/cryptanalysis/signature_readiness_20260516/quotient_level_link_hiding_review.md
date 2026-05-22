# Quotient-Level Link Hiding Review

Status: candidate-specific sampler replays reject the current
public-link-conditioned shortlist.

This sidecar artifact does not change the current q12289 verifier, proof
path, byte target, or implementation.

## Summary

| item | value |
|---|---:|
| control candidate | `current-coordinate-link-0-3` |
| control bounded-kernel margin | `43` |
| review rows | `6` |
| top exact coordinate shortlist rows | `3` |
| top shortlist sampler certificates complete | `0` |
| top shortlist public-link hiding failures | `3` |
| top shortlist status | `rejected_as_public_quotient_links_under_current_sampler_surface` |

## Reviewed Sampler Boundary

| item | value |
|---|---:|
| sampler | `direct-conditioned-quotient-fiber-sampler` |
| boundary status | `checks_passed` |
| exact support formula | `12289^9 * 3073` |
| support bits | `133.851151` |
| support floor | `133` |
| target bits | `128` |
| support floor margin | `5` |
| retry loop required | `False` |

## Candidate Ledger

| candidate | priority | bytes | rank | raw bits floor | public-link support floor | kernel margin | delta vs control | sampler | usable floor | status |
|---|---|---:|---:|---:|---:|---:|---:|---|---:|---|
| current-coordinate-link-0-3 | control | 4 | 14 | 122 | 120 | 43 | 0 | reviewed_current_control | 133 | control_pass_but_public_link_replacement_fails_hiding |
| coordinate-link-1-14 | top_exact_coordinate_shortlist | 4 | 14 | 122 | 108 | 94 | 51 | public_link_conditioned_sampler_replay_fails | - | rejected_public_link_hiding |
| coordinate-link-8-20 | top_exact_coordinate_shortlist | 4 | 14 | 122 | 106 | 94 | 51 | public_link_conditioned_sampler_replay_fails | - | rejected_public_link_hiding |
| coordinate-link-14-15 | top_exact_coordinate_shortlist | 4 | 14 | 122 | 106 | 94 | 51 | public_link_conditioned_sampler_replay_fails | - | rejected_public_link_hiding |
| coordinate-link-1-2 | deprioritized_lower_kernel_margin | 4 | 14 | 122 | 108 | 19 | -24 | public_link_conditioned_sampler_replay_fails | - | rejected_public_link_hiding |
| sparse-difference-link-0m3-1m2 | deprioritized_lower_kernel_margin | 4 | 14 | 122 | 106 | 21 | -22 | public_link_conditioned_sampler_replay_fails | - | rejected_public_link_hiding |

## Interpretation

- The exact-passed top coordinate rows match the current control on rank, packed link bytes, and raw product-fiber dimension, while improving the bounded-kernel margin from 43 to 94.
- Raw q-residue fiber capacity at rank 14 is only about 122.27 bits, below the 128-bit target.  The current control relies on the reviewed packed low-bit support formula to reach a 133-bit floor.
- The first candidate-specific sampler replays reject the top coordinate rows as public quotient links: once their link rows condition the representative, support falls below the 128-bit hiding target.

## Next Required Work

- Search for link rows that improve bounded-kernel margin without being public conditioning data, or increase product-fiber dimension before public-link promotion.
- Add a candidate family for hidden commitment links where account binding does not reveal link row values.
- Only promote a public quotient-level link if its sampler replay keeps at least 128 support-floor bits under public-link conditioning.
