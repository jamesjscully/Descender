# Distributional Signature Proof

This note records the formal distributional proof boundary for the
product-block Fiat-Shamir-with-aborts signature route.

## Formalized Theorem

`Descender/SignatureDistribution.lean` defines a finite rejection-sampling
interface:

- `RejectionSampler`: a mask distribution, a response function, an accept
  predicate, and a target response distribution;
- `acceptedTestMass`: the unnormalized mass of accepted responses passing an
  arbitrary Boolean test;
- `conditionalAcceptedProb`: the accepted-response probability after dividing
  by the acceptance mass;
- `RejectionSamplingCertificate`: the exact condition that every accepted
  response test has mass `acceptance * target.acceptProb test`.

Lean proves:

- `acceptedMass_eq_acceptance`: the abort probability is independent of the
  secret and challenge;
- `conditionalAcceptedProb_eq_target`: the conditional accepted-response
  distribution equals the target distribution;
- `secret_independent`: accepted responses reveal no information about the
  secret;
- `challenge_independent`: the fixed-challenge accepted distribution is also
  common across challenges when the same target is used;
- `encoded_secret_independent`: any deterministic response encoding or compact
  transcript projection remains secret-independent.

For product-block signatures, the file also defines the concrete linear
response shape

```text
z = y + chi * s
```

as `productBlockLinearResponse`, and packages it as
`productBlockRejectionSampler`.  The theorem
`productBlock_accepted_response_secret_independent` is the distributional
statement we need for sparse signed product-block challenges.

## What This Proves

If a concrete masking distribution and rejection predicate satisfy the
`RejectionSamplingCertificate`, then the accepted signature response has the
same distribution for every secret.  Therefore the transcript does not leak the
small secret through the linear response `z = y + chi * s`.

This is the formal version of the Lyubashevsky/Dilithium-style rejection
sampling argument, stated in the repo's lightweight finite-probability
framework.

## What Remains

The proof is now factored correctly, and the first concrete exact sampler is
analyzed in `sampler_analysis_n48.md` and `sampler_analysis_n32.md`.  The
sampler uses an enlarged uniform box for the mask and accepts exactly when the
shifted response falls in the target box.  This satisfies the exact
accepted-test-mass equation coordinatewise.

The remaining proof obligation is not distributional secrecy.  It is
block-soundness and extraction for sparse product-block challenges.  Low-weight
challenges produce very small signatures, but a prover missing one block can
hope the challenge avoids that block.  The concrete reports therefore separate:

- entropy-and-retry rows, which can be extremely small but have weak block
  coverage diagnostics;
- intermediate coverage rows, such as `243 B` for the `n = 32` profile;
- full-block rows, such as `304 B` for the `n = 32` profile and `496 B` for
  the `n = 48` profile.

For further size reductions, the likely route is a sharper soundness theorem
for intermediate block coverage, not a more conservative sampler.

## Current State

The product-block route now has three formal layers:

1. block-local challenge equivariance for the public systematic relation;
2. signed-challenge error preservation for noisy and serialized views;
3. rejection-sampling distributional secrecy, conditional on the accepted mass
   matching a secret-independent target law;
4. a concrete exact-box sampler analysis that instantiates the target law and
   quantifies response size versus retry cost.

The remaining blocker is no longer the sampler distribution.  It is whether an
intermediate block-coverage row can be given a sound Fiat-Shamir extraction
argument, or whether we must pay for full-block challenges.
