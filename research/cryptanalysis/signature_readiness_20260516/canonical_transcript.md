# Canonical Transcript Boundary

This note records the concrete transcript layer now formalized for the
`n = 32`, `255 B` product-block signature candidate.

## Lean Objects

`Descender/SignatureTranscript.lean` defines the canonical challenge space for
the current small row:

- `N32Block = Fin 16`;
- `N32Repetition = Fin 7`;
- `N32ReplayDraw = Fin 6`;
- `N32RoundChallenge`, represented by one omitted block plus one sign bit for
  each of the remaining fifteen blocks;
- `N32CanonicalChallenge = Fin 7 -> N32RoundChallenge`.

The omitted-block representation is exactly the weight-15 challenge model:
choosing fifteen active blocks out of sixteen is equivalent to choosing the one
inactive block.

Lean proves:

```text
Fintype.card (N32ActiveBlock omitted) = 15
Fintype.card N32RoundChallenge = 16 * 2^15
Fintype.card N32CanonicalChallenge = 2^133
```

The file also defines the byte-level binding surface, abstracting over the
actual byte types for now:

```text
N32ChallengePoint PublicKey Message Commitment
N32AcceptingTranscript PublicKey Message Commitment Response
N32SixDrawReplay PublicKey Message Commitment Response
```

`N32ChallengePoint` binds the public key, message, and seven repeated
commitments.  `N32SixDrawReplay` records six accepting transcripts sharing that
same challenge point.

## Query-Miss Bound

The file adds a reusable uniform finite distribution and proves the exact
blind-guess bound:

```text
Pr[actual_challenge = guess] = 1 / 2^133.
```

In Lean this is:

```text
n32_single_challenge_guess_acceptProb
```

For a finite family of blind guesses, Lean proves the union-bound form:

```text
Pr[exists g, guess[g] = actual_challenge]
  <= Fintype.card Guess / 2^133.
```

This is:

```text
n32_challenge_guess_family_acceptProb_le
```

This is the concrete target for `queryMiss`: if the adversary outputs a valid
forgery without querying the random-oracle challenge point, the output
challenge must be one of its blind guesses in a `2^133` challenge space.

The next layer is now formalized too.  `N32QueryLogExperiment` factors a trace
as:

```text
adversary_state × actual_challenge
```

where the challenge is sampled independently and uniformly after the
adversary-side state is fixed.  The query log records which challenge points
the adversary queried, and the output challenge is a function only of the
adversary state when the challenge point was not queried.

Lean proves:

```text
Pr[actual_challenge = outputChallenge(state)] = 2^-133.
```

`QueryMissCertificate.queryMiss_acceptProb_le` then turns the verifier-side
condition

```text
queryMiss => challenge point was not queried
queryMiss => actual_challenge = outputChallenge(state)
```

into the concrete bound:

```text
Pr[queryMiss] <= 2^-133.
```

This is the first transcript-specific loss term connected to the abstract
Fiat-Shamir theorem.

## Replay Boundary

`N32ReplaySchedule` defines the six replay-draw diagnostics.  A draw is good
when it both accepts and is bound to the same challenge point:

```text
drawGood d = drawAccepts d && sameChallengePoint d
drawFailure d = not drawGood d
replayFailure = exists d : Fin 6, drawFailure d
```

Lean proves:

```text
replayFailure => exists d : Fin 6, drawFailure d.
```

Finally, `N32SixDrawForkingInputs` packages the query-log proof, the six replay
draw failures, their probability bounds, and the compact coverage-failure
bound.  It produces the existing `SixDrawForkingBounds` certificate and proves:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_{d : Fin 6} drawFailureBound[d]
  - coverageFailureBound.
```

## Remaining Work

The abstract forking layer and the canonical challenge-count layer now meet at
the right boundary.  The next proof should instantiate the replay probabilities:

```text
Pr[drawFailure d] <= drawFailureBound[d]
```

That requires the serialized verifier model:

- concrete byte encoding for the public key, message, and seven commitments;
- canonical encoding/decoding for `N32CanonicalChallenge`;
- compact active-only response encoding;
- verifier recomputation of the random-oracle challenge point;
- a replay/programming schedule proving that each failed accepting draw has a
  named per-draw failure reason.
