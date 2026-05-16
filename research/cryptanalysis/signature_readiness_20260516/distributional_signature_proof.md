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

The proof is now factored correctly, but the concrete analytic sampler
instantiation is still open.  The next proof obligation is to exhibit a
practical product-block mask distribution and rejection predicate satisfying
the exact accepted-test-mass equation, or a statistically close variant with an
explicit distance bound.

The exact certificate is idealized.  For deployed parameters, we probably need
one of:

- an exact finite distribution where the accepted response is literally a fixed
  target law;
- a bounded statistical-distance theorem with rejection loss;
- a computationally generated certificate for the finite distribution at the
  target parameter sizes.

## Current State

The product-block route now has three formal layers:

1. block-local challenge equivariance for the public systematic relation;
2. signed-challenge error preservation for noisy and serialized views;
3. rejection-sampling distributional secrecy, conditional on the accepted mass
   matching a secret-independent target law.

The remaining blocker is no longer the shape of the proof.  It is the concrete
sampler analysis: can we make the accepted-test-mass equation true, or close
enough, for the small `212 B` to `248 B` signature rows?
