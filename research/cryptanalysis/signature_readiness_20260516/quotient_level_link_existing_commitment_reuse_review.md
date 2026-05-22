# Existing Commitment Reuse Review

Status: reject as a deployable hidden quotient-link replacement.

This sidecar review does not change the current q12289 verifier, proof path, byte target, or implementation.

## Summary

| item | value |
|---|---|
| previous best branch | `reuse-existing-account-commitments-for-link` |
| reuse status | `rejected_for_deployment` |
| deployment recommendation | `keep_current_reviewed_split_design` |
| byte-saving candidate found | `False` |
| proof-simplification candidate found | `False` |

Existing account commitments bind full hidden preimages only after opening; they do not expose a hidden quotient-link commitment or homomorphic link equation.  Any route that keeps the link hidden needs a new zero-knowledge/hash-circuit proof, and any route that opens link values reintroduces the public link hiding failure.

## Source Checks

| source | symbol | present | implication |
|---|---|---:|---|
| Descender/LocalDeltaByteCommitment.lean | `ByteCommitmentConfig` | `True` | binds bytes only through collision exclusion |
| Descender/LocalDeltaByteCommitment.lean | `ByteHashBinding` | `True` | collision-free digest gives equality of encoded preimages after equal digest |
| Descender/LocalDeltaByteCommitment.lean | `collisionFreeOnDomain` | `True` | no homomorphic operation or committed linear link value is exposed |
| Descender/CommittedSigma.lean | `HomomorphicAccountLinkInterface` | `True` | the current byte hash commitment is not itself this interface |
| Descender/CommittedSigma.lean | `linkProofAccepts_of_publicProjection` | `True` | a reused commitment proof must avoid becoming a public projection shortcut |
| Descender/CommittedSigma.lean | `reviewedAccountHiddenLinkBytes_eq` | `True` | reuse is only interesting if it adds no bytes and no larger proof object |
| Descender/HiddenCommittedQuotientLink.lean | `HiddenCommittedQuotientLink.Interface` | `True` | requires binding, hiding, non-public projection, and sampler-surface gates |

## Requirement Ledger

| requirement | available | usable | reason |
|---|---:|---|---|
| `binds_full_hidden_preimage_after_opening` | `True` | `partial` | The byte-hash path can bind an opened encoded hidden state, assuming collision freedom and an injective encoding. |
| `exposes_link_value_without_opening_hidden_state` | `False` | `no` | A digest of the full hidden preimage does not expose a committed quotient-link value or a verifier-checkable link projection. |
| `homomorphic_link_equation_over_commitments` | `False` | `no` | The byte-hash commitment has no add/scale structure for checking link(z) = link(r) + c * link(w) on commitments. |
| `preserves_sampler_surface_without_public_link_rows` | `True` | `conditional` | The sampler surface is preserved only if link values remain hidden; opening them publicly reintroduces the public-link hiding failure. |
| `proves_response_link_soundness_without_new_proof` | `False` | `no` | Existing commitment binding can support an opening check, but does not by itself prove the hidden linear link equation. |
| `deployable_byte_win_over_68b_split_design` | `False` | `no` | No extra account bytes are needed only if reuse is free, but the missing non-public equation proof would add unknown proof bytes or assumptions. |

## Proof Routes

| route | proof bytes | hiding | binding | decision |
|---|---|---|---|---|
| `open_full_hidden_preimages` | `large_or_contextual` | `fails_if_link_rows_become_public` | `works_after_opening` | `reject` |
| `zk_proof_from_hash_commitments` | `unknown_nonzero` | `plausible` | `requires_new_hash_circuit_soundness_argument` | `research_only` |
| `derive_homomorphic_link_commitment_from_hash_digest` | `-` | `not_applicable` | `not_available` | `reject` |

## Decision

The existing commitments do not give a free quotient-level commitment link. They are useful for full-preimage binding after an opening, but the hidden link equation still needs a separate proof object or a different commitment algebra.

## Next Required Work

- Create the Milestone 5 promotion gate and classify the result as no replacement.
- Record artifact paths, replay commands, Lean theorem names, and assumption deltas.
- Tell the implementation agent to keep ignoring this sidecar research unless a later promotion gate changes the recommendation.
