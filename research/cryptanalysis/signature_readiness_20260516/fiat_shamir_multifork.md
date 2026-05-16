# Fiat-Shamir Multi-Fork Boundary

This note records the probabilistic boundary now formalized for the
product-block signature route.

## Formal Layer

`Descender/SignatureForking.lean` adds a finite probability interface for the
multi-fork Fiat-Shamir game.  The deterministic extractor already lives in
`Descender/BlockChallengeSoundness.lean`; this file supplies the probability
accounting around it.

The game object is `MultiForkFiatShamirGame`.  It contains:

- a finite trace distribution;
- the inverse-of-two witness needed for signed challenge deltas;
- repeated multi-fork rounds, each sharing one commitment/mask;
- an `acceptingForks` event for same-commitment accepting challenge draws;
- a `coverageFailure` event for compact fork-delta miss;
- a proof that accepting forks with no coverage failure give compact
  opposite-sign coverage over every block.

For every extractable trace, Lean proves that the extracted product-block
secret equals the real secret:

```text
extractedSecret_eq
```

## Loss Accounting

The key probability theorem is:

```text
Pr[extractable] >= Pr[acceptingForks] - Pr[coverageFailure].
```

In Lean this is `extractableProb_lower_bound`.  It is proved from a finite
union-bound identity:

```text
Pr[A and not B] >= Pr[A] - Pr[B].
```

The final wrapper is `MultiForkForkingCertificate`.  If a future forking lemma
supplies

```text
Pr[acceptingForks] >= forgerSuccess - forkingLoss
```

and the sampler analysis supplies

```text
Pr[coverageFailure] <= coverageFailureBound,
```

then Lean proves:

```text
Pr[extractable] >= forgerSuccess - forkingLoss - coverageFailureBound.
```

This is `MultiForkForkingCertificate.extractableProb_ge`.

## Concrete Meaning

For the current `n = 32`, `255 B` row, the sampler-side compact coverage
failure is about `2^-27.33` when the forking procedure obtains six accepting
challenge draws for each repeated commitment.  That term is no longer vague:
it plugs directly into `coverageFailureBound`.

The remaining unproved quantity is the concrete `forkingLoss`: the loss for
turning a real forger into six accepting same-commitment draws in the
Fiat-Shamir random-oracle game.  This depends on the exact transcript layout,
hash domain separation, challenge sampling, replay schedule, and acceptance
probability of the signer/verifier relation.

## Next Proof Target

The next theorem should instantiate `forkSuccess_lower` for the actual
signature transcript:

```text
forgerSuccess - forkingLoss <= Pr[acceptingForks].
```

The transcript proof must ensure:

- all six accepting draws are bound to the same public key, message, round
  commitment, and repetition index;
- the challenge distribution is the same signed product-block distribution used
  by the sampler analysis;
- rewinding only changes random-oracle answers for the challenge point being
  forked;
- abort/rejection behavior is already absorbed into the observed accepting
  transcript event;
- the final loss expression is small enough that
  `forgerSuccess - forkingLoss - coverageFailureBound` remains non-negligible
  for any successful forger.
