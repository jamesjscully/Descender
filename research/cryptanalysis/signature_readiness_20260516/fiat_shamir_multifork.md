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

## Random-Oracle Replay Loss

The file now also instantiates `forkSuccess_lower` through a random-oracle
replay layer, `RandomOracleMultiForkReplay`.

It separates the forking loss into two concrete events:

- `queryMiss`: the adversary forged but the reduction did not obtain the
  challenge random-oracle point needed for replay;
- `replayFailure`: the challenge point was available, but the replay schedule
  did not produce the required accepting same-commitment multi-forks.

The replay invariant is:

```text
forgery and not queryMiss and not replayFailure
  => accepting same-commitment forks.
```

Lean proves:

```text
Pr[acceptingForks] >=
  Pr[forgery] - Pr[queryMiss] - Pr[replayFailure].
```

This is `RandomOracleMultiForkReplay.forkSuccessProb_lower_bound`.

The final certificate is `RandomOracleForkingBounds`.  Given concrete upper
bounds for `queryMiss`, `replayFailure`, and compact coverage failure, Lean
proves:

```text
Pr[extractable] >=
  Pr[forgery]
  - queryMissBound
  - replayFailureBound
  - coverageFailureBound.
```

This is `RandomOracleForkingBounds.extractableProb_ge`.

## Six-Draw Replay Certificate

The six-draw candidate now has its own certificate wrapper,
`SixDrawForkingBounds`, specialized to `Draw = Fin 6`.

Instead of treating replay failure as one opaque number, the theorem asks for
seven transcript-local inputs:

- one `queryGuess` event;
- six per-draw `drawFailure d` events;
- upper bounds for those seven events;
- the compact coverage-failure bound from the sampler report.

The required implications are simple:

```text
queryMiss       => queryGuess
replayFailure   => exists d : Fin 6, drawFailure d
```

Lean then proves the concrete union-bound certificate:

```text
Pr[extractable] >=
  Pr[forgery]
  - challengeGuessBound
  - sum_{d : Fin 6} drawFailureBound[d]
  - coverageFailureBound.
```

This is `SixDrawForkingBounds.extractableProb_ge`.  The point of this theorem
is that the next transcript proof does not need to manipulate the global
forking theorem directly.  It only has to show that each failed replay draw has
one named reason, and that each reason has a concrete probability bound.

## Concrete Meaning

For the current `n = 32`, `255 B` row, the sampler-side compact coverage
failure is about `2^-27.33` when the forking procedure obtains six accepting
challenge draws for each repeated commitment.  That term is no longer vague:
it plugs directly into `coverageFailureBound`.

For this row, a canonical product-block challenge has

```text
(binom(16,15) * 2^15)^7 = (16 * 2^15)^7 = 2^133
```

possible challenge vectors.  `Descender/SignatureTranscript.lean` now
formalizes this exact challenge space as `N32CanonicalChallenge` and proves
`Fintype.card N32CanonicalChallenge = 2^133`.

The same file proves the uniform blind-guess bound:

```text
Pr[actual_challenge = guess] = 2^-133
Pr[exists g, guess[g] = actual_challenge] <= #Guess / 2^133.
```

So the query-miss term is no longer just an intended value.  The remaining
query-miss work was to connect a concrete forger transcript to this uniform
challenge experiment.  That bridge is now formalized by
`N32QueryLogExperiment`: the trace factors as adversary state times an
independent uniform challenge, and `QueryMissCertificate` proves
`Pr[queryMiss] <= 2^-133` whenever a query-miss transcript both missed the
challenge point and accepted only by outputting the hidden challenge.

The remaining uninstantiated quantity is now the replay schedule, not a generic
forking loss.  For the six-draw candidate, instantiate the replay model with
`Draw = Fin 6`; prove that each failed replay draw is covered by its
corresponding `drawFailure d`; then bound those six events for the actual
rewinding/programming schedule.

## Next Proof Target

The next theorem should instantiate `SixDrawForkingBounds` for the actual
signature transcript:

```text
Pr[drawFailure d] <= drawFailureBound[d]   for each d : Fin 6
```

`N32SixDrawForkingInputs` already packages the query-log proof and replay
decomposition into the final extraction bound:

```text
Pr[extractable] >=
  Pr[forgery] - 2^-133 - sum drawFailureBound - coverageFailureBound.
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
  `forgerSuccess - 2^-133 - sum drawFailureBound -
  coverageFailureBound` remains non-negligible for any successful forger.
