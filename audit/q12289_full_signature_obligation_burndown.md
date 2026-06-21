# q12289 Full Signature Obligation Burn-Down

> MIGRATION STATUS: `ACTIVE_RESEARCH`.
> This q12289 burn-down is retained research evidence and reference/fallback
> proof structure, not the current deployable target or top-level tracker.
> Current authority lives in
> `spec/README.md`, ADRs, and `TRACKER.md`.

Date: 2026-06-01

Scope: non-active q12289 checked-systematic reference/fallback path after
retiring the compact six-replay endpoint from deployable security.

## Active-Target Status

This file is not the active deployable-signature target. The active target is
the Path 2 native / reviewed Fq12 active-lane projection-carrier route tracked in
`audit/path2_native_full_signature_obligation_burndown.md` and represented in
Lean by `NativeFullSignatureSecurityBoundary`.

Use this q12289 FS-core file only as a reference theorem shape or fallback
evidence source unless deployment is explicitly switched back to q12289 FS-core.

## Current Position

The q12289 Lean endpoint now has two layers:

1. `N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate` proves the
   accepted-transcript bound `Pr[accepted transcript forgery] <= 2^-133`.
2. `N32Q12289FullSignatureSecurityBoundary` is the full-signature boundary. It
   can prove `Pr[full signature forgery] <= 2^-133`, but only after the
   boundary supplies:
   - keygen distribution modeling,
   - signing oracle modeling,
   - completeness modeling,
   - implementation-verifier agreement,
   - EUF-CMA forgery to accepted-transcript reduction,
   - hash frame domain separation,
   - query-budget pricing,
   - multi-target loss pricing.

This is the correct shape for the q12289 reference endpoint: the full claim is
no longer hidden inside the accepted-transcript theorem.

Important deployment finding: current Rust production preflight does not select
the q12289 FS-core verifier as the deployment verifier. It names the selected
verifier blocker as the `308-byte/Fq12 active-lane projection-carrier`
deployment route and keeps that verifier fail-closed. Therefore q12289 is a
reviewed theorem target / fallback evidence source unless the deployment target
is explicitly changed back to q12289.

## Obligation Table

| ID | Boundary field | Current status | Next proof move | Redesign trigger |
|---|---|---|---|---|
| QFS-001 | `hashBoundary.frameDomainSeparated` | Mostly ready. Lean fixes the q12289 frame shape and Rust conformance docs pin the hash frame, but the new boundary does not yet connect to a concrete domain-separation theorem. | Add a Lean theorem/record tying `q12289HashFrame`, `implicitCommitmentDomainTag`, and the Rust profile domain string to the FS-core boundary. | Redesign only if the deployed frame is not injective/domain-separated or is shared with another challenge domain. |
| QFS-002 | `hashBoundary.queryBudgetPriced` | Open cryptographic accounting. The theorem has `2^-133` challenge guess, but no production query budget is priced in the full boundary. | Add a concrete ROM/QROM budget record with max signing queries, verification/adversary hash queries, and the resulting loss term. | Redesign if realistic blockchain/multi-target budgets consume too much of the 133-bit challenge space. |
| QFS-003 | `hashBoundary.multiTargetLossPriced` | Open deployment accounting. Existing docs mention multi-target losses, but the active Lean boundary has no numeric network/account budget. | Add an explicit account/network target count and theorem-level loss expression. | Redesign if global account count or long-term network horizon makes `2^-133` too small. |
| QFS-004 | `keygenDistributionModeled` | High-risk open item. Existing frozen q12289 review says no frozen keygen algorithm is closed. Path 2 native signer has a separate productization track, but it is not yet linked to this q12289 theorem boundary. | Define the keygen game for the selected production account/signature path and prove it emits only public views covered by the Lee/no-bad-witness certificate, or exports a named keygen break. | Redesign if production keygen can emit public views outside the fixed checked-systematic safe family with non-negligible probability. |
| QFS-005 | `signingOracleModeled` | High-risk open item. Existing review says no frozen signing algorithm was closed for the old q12289 target; the newer Path 2 signer has substantial implementation evidence but not a Lean signing-oracle theorem. | Model signing as a distribution over valid FS-core transcripts and prove signer outputs map into `acceptedTranscriptCertificate.forgery` only under the EUF-CMA reduction conditions. | Redesign if rejection sampling, state lifecycle, or hidden-source opening can bias/expose transcript material outside the theorem model. |
| QFS-006 | `completenessModeled` | Open but likely tractable. The verifier-side relation is strong; honest signer/keygen completeness is not yet a q12289 full-game theorem. | Add honest-generation completeness statement: generated signatures verify under the FS-core verifier except for named sampler/RNG failures. | Redesign if honest signing requires witness data not represented in the q12289 checked-systematic relation. |
| QFS-007 | `verifierRelationMatchesImplementation` | Redesign trigger fired for deployment. Rust production preflight currently targets the Path 2 native / reviewed Fq12 route, not the frozen q12289 FS-core implicit verifier. | Do not spend proof effort turning q12289 into the final EUF-CMA theorem unless deployment is intentionally switched back to q12289. Instead, port the useful q12289 theorem shape into the active Path 2/Fq12 verifier relation. | Active deployment remains Path 2/Fq12; q12289 full-game proof would certify the wrong verifier. |
| QFS-008 | `eufCmaForgeryImpliesAcceptedTranscriptForgery` and `signatureForgerySuccess_le_acceptedTranscript` | Open theorem lift. The new Lean boundary requires a numeric reduction, but no EUF-CMA game object has been instantiated yet. | Define the full EUF-CMA game over keygen/sign/verify and prove every successful forgery induces an accepted transcript event, plus priced bad events. | Redesign if the blockchain lifecycle permits valid post-state/replay outcomes that do not correspond to a fresh accepted transcript event. |

## Redesign Assessment

No redesign is forced inside the current Lean q12289 algebraic theorem. The
proof kernel that maps accepted q12289 transcripts to the blind challenge bound
is healthy.

However, deployment-level redesign is required relative to the q12289 full-game
plan: the active production verifier is not q12289 FS-core. The next theorem
target should be the Path 2 native / reviewed Fq12 verifier relation, reusing
q12289 only as a pattern and negative-control/reference proof.

The two places most likely to force redesign are:

- **Keygen distribution**: if the production account/key generation path cannot
  be shown to stay inside the fixed safe public-view family, the q12289 theorem
  is the wrong endpoint for deployment.
- **Signer/oracle modeling**: if the actual signer uses Path 2 hidden-source
  state, lifecycle material, or proof payloads whose distribution cannot be
  represented as the q12289 FS-core transcript game, the full proof target must
  shift to the deployed Path 2 native verifier relation.

The hash/query-budget items are less likely to force algebraic redesign, but
they may force parameter changes if the final blockchain multi-target budget is
too large for a 133-bit challenge space.

## Recommended Order

1. Treat QFS-007 as failed for active deployment: q12289 is not the selected
   verifier.
2. Create the analogous full-signature boundary for the Path 2 native / Fq12
   verifier relation.
3. Port only the useful structure from q12289: accepted-transcript theorem
   shape, explicit hash/query budget, verifier implementation bridge, and
   full-forgery-to-accepted-transcript reduction field.
4. Close keygen/signing/completeness against the Path 2 native signer, not
   against q12289 FS-core.
5. Keep q12289 full-game closure as optional/fallback work, not as a blocker for
   the deployable active path.
