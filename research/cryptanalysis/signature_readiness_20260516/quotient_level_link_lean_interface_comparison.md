# Hidden Committed Quotient Link Lean Interface Comparison

Status: sidecar theorem-shape prototype builds.

This note compares the sidecar `Descender.HiddenCommittedQuotientLink` module
against the existing `CommittedSigma.AccountLinkProofSystem`.  It does not
change the deployable q12289 proof path.

## Build

```text
lake build Descender.HiddenCommittedQuotientLink
```

Result on 2026-05-22: builds successfully.

## New Sidecar Names

| name | role |
|---|---|
| `HiddenCommittedQuotientLink.HiddenCommittedLinkBreaks` | Names the hidden-link break buckets: commitment binding, link equation, public projection, and sampler hiding. |
| `HiddenCommittedQuotientLink.Interface` | Packages the hidden-commitment quotient-link theorem shape. |
| `HiddenCommittedQuotientLink.Interface.toAccountLinkProofSystem` | Converts the hidden-commitment interface into the existing account-link proof system. |
| `HiddenCommittedQuotientLink.Interface.accepts_accountBacked_of_noBreaks` | Proves accepted account-aware transcripts are backed by hidden witness/randomizer state when all hidden-link breaks are ruled out. |
| `HiddenCommittedQuotientLink.Interface.accepts_glues_or_breaks_of_effectiveDescent` | Reuses `Descender.DescentInterface` as the generic descent wrapper. |

## Comparison

| question | existing account-link proof system | hidden committed quotient link |
|---|---|---|
| accepted transcript consequence | `AccountBackedByHiddenWitness ∨ accountLinkProofBreak` | same consequence after converting through `toAccountLinkProofSystem` |
| break granularity | one abstract `accountLinkProofBreak` | named union: commitment binding, link equation, public projection, sampler hiding |
| public-projection guard | present in surrounding structural gates and homomorphic negative theorem | explicit field `nonPublicProjection` plus named `publicProjectionBreak` |
| hiding surface | external structural gate/witness-hiding review | explicit field `samplerSurfacePreserved` plus named `samplerHidingBreak` |
| quotient-level link values | not named in the generic system | explicit abstract `LinkValue` with `linkOfWitness`, `linkOfRandomizer`, and `linkOfResponse` |
| proof simplification | current deployable adapter remains canonical | no simplification yet; the sidecar only sharpens which hidden-link obligations must be proven |

## Interpretation

The sidecar interface does not replace the current proof path.  It factors the
surviving research branch into the existing account-link proof system and makes
two formerly narrative gates explicit: the link proof must not be a public
projection of the sigma equation, and the commitment view must not condition the
hidden representative sampler on public link values.

This is useful, but it is not a migration argument.  The next useful test is a
byte/state review of concrete commitment choices.  If every hidden-commitment
choice costs at least the current `68 B` account object without removing proof
obligations, this branch should be rejected as terminology rather than an
improvement.

## Next Required Work

- Compare hash commitment, homomorphic commitment, and reuse-of-existing-account-commitment options.
- For each option, record account bytes, spend bytes, proof bytes, and whether the link equation proof can avoid public link values.
- Reject any option whose proof is a public projection or whose account object exceeds the current `68 B` without removing a meaningful obligation.
