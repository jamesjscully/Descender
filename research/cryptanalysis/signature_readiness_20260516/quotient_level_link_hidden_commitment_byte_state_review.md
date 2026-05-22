# Hidden-Commitment Quotient-Link Byte/State Review

Status: no deployable byte win found; one proof-organization branch remains.

This sidecar review does not change the q12289 verifier, proof path, byte target, or implementation.

## Summary

| item | value |
|---|---:|
| rows reviewed | `6` |
| current spend bytes | `250` |
| current account bytes | `68` |
| current online plus account bytes | `318` |
| byte-saving candidate found | `False` |
| deployment recommendation | `keep_current_reviewed_split_design` |

No hidden-commitment option currently gives a deployable byte win. The only branch worth further research is reuse of existing account commitments, because it might improve proof organization without increasing state.

## Commitment Options

| candidate | account bytes | tx+account | kernel margin | status | recommendation |
|---|---:|---:|---:|---|---|
| current-reviewed-split-account-link | 68 | 318 | 43 | control | keep |
| public-two-row-quotient-link | 4 | 254 | 94 | rejected | do_not_pursue |
| hash-hidden-link-only | 64 | - | 94 | blocked | reject_unless_full_pre_response_backing_is_recovered |
| hash-hidden-link-plus-current-commitments | 128 | - | 94 | dominated | do_not_pursue |
| pedersen-style-homomorphic-hidden-link | 64 | - | 94 | research_only | only_pursue_for_proof_simplification |
| reuse-existing-account-commitments-for-link | 64 | - | 94 | research_only | best_remaining_branch |

## Proof Obligations

| candidate | hiding | binding/extraction | link equation | non-public projection | sampler surface |
|---|---|---|---|---|---|
| current-reviewed-split-account-link | reviewed_current_surface | reviewed_pre_response_backing | bounded_kernel_response_link_certificate | reviewed_gate | reviewed_133_bit_floor |
| public-two-row-quotient-link | none | public_value_only | direct_public_check | not_sufficient | fails_public_link_conditioning |
| hash-hidden-link-only | computational_hash_preimage_hiding_only | binds_link_values_but_not_full_hidden_state | needs_new_zk_or_opening_proof | possible_if_proof_not_public_projection | preserved_if_link_values_never_open_publicly |
| hash-hidden-link-plus-current-commitments | hash_commitments_hide_link_values | current_backing_plus_new_link_binding | needs_new_zk_or_opening_proof | possible_if_proof_not_public_projection | preserved_if_link_values_never_open_publicly |
| pedersen-style-homomorphic-hidden-link | plausible_with_new_group_assumption | plausible_with_discrete_log_binding_assumption | homomorphic_but_needs_non_public_projection_argument | open | preserved_if_commitments_are_hiding |
| reuse-existing-account-commitments-for-link | inherits_existing_commitment_hiding_if_available | open_needs_link_extraction_from_existing_commitments | open_needs_non_public_link_equation_proof | open | preserved_if_link_values_not_revealed |

## Next Required Work

- Test whether existing witness/randomizer commitments can expose a binding theorem for linkOfWitness/linkOfRandomizer without revealing link values.
- If not, reject hidden-commitment quotient links as dominated by the current 68B split design.
- Do not pursue hash-hidden-link-only unless a separate theorem restores full pre-response backing.
