# Quotient-Level Link Account Semantics

Status: hidden-commitment branch selected for research; current
deployable split design remains the control.

This sidecar artifact does not change the current q12289 verifier, proof
path, byte target, or implementation.

## Summary

| item | value |
|---|---|
| public quotient link | `rejected_by_hiding_support` |
| hidden commitment link | `research_candidate` |
| recommended research branch | `hidden-commitment-quotient-link` |
| deployment recommendation | `keep_current_reviewed_split_design` |

## Account Object Candidates

| candidate | visibility | account bytes | kernel margin | hiding/support | status |
|---|---|---:|---:|---|---|
| current-reviewed-split-account-link | response-link code public in account state; hidden representatives committed separately | 68 | 43 | reviewed_current_control / 133 bits | keep_as_deployable_control |
| public-quotient-link-top-coordinate-shortlist | public/account-visible quotient data | 4 | 94 | rejected_public_link_hiding / 108 bits | rejected |
| hidden-commitment-quotient-link | hidden behind binding/hiding commitments | - | 94 | plausible_if_commitment_hides_link_values / 133 bits | research_candidate |

## Hidden Commitment Requirements

- `commitment_hiding`: The account object must not reveal link(w) or link(r), otherwise the sampler is conditioned on public link rows and the support floor falls below target.
- `commitment_binding_or_extraction`: A successful opening must determine the committed link values well enough to apply bounded-kernel soundness, or produce a named binding break.
- `non_public_projection`: The link equation proof cannot be a public projection of the sigma equation; that shortcut is already known to add no security.
- `homomorphic_or_linear_link_proof`: The verifier needs evidence for link(z) = link(r) + c * link(w) without learning the hidden link values.
- `sampler_surface_preservation`: The hidden representative sampler may condition on the semantic quotient, canonical packing, fiber-state equality, and bounds, but not on revealed link rows or active pads.

## Lean-Facing Shape

- structure: `HiddenCommittedQuotientLink.Interface`
- target theorem: `accepted + hidden_committed_link + no binding/link/projection/hiding breaks -> account-backed quotient witness`

Fields:
- `link : R -> L`
- `commit : L -> Nonce -> Commitment`
- `hiding : commitment view does not condition the representative sampler on L`
- `binding_or_break : two openings of one commitment imply equal link value or BindingBreak`
- `link_equation_proof : accepts proof of committed link(z) = committed link(r) + c * committed link(w)`
- `non_public_projection : link equation acceptance is not implied by the public sigma equation alone`
- `bounded_kernel : ker(semantic) cap ker(link) cap responseBox = {0} or LinkKernelBreak`

## Next Required Work

- Record byte/state possibilities for hash commitment, Pedersen-style homomorphic commitment, and existing account commitment reuse.
- Compare each commitment option against the sidecar HiddenCommittedQuotientLink.Interface theorem shape.
- Reject the branch if the proof object requires public link values or consumes more state than the current 68-byte account object without proof simplification.
- Instantiate the interface abstractly against coordinate-link-1-14 only after a commitment option survives the byte/state screen.
