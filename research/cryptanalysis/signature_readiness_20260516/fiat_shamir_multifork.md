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

The replay decomposition has now been sharpened again.  For each draw:

```text
drawFailure d <= acceptFailure d + pointFailure d
```

where `acceptFailure d` means the programmed replay did not produce an
accepting transcript, and `pointFailure d` means it did not bind to the same
public key/message/commitment challenge point after accepting.  The component interface
`N32SixDrawComponentInputs` proves:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_d (acceptFailureBound[d] + pointFailureBound[d])
  - coverageFailureBound.
```

For a deterministic byte-level transcript binding proof, the expected
`pointFailureBound[d]` is zero.  The real remaining forking loss is therefore
the six accepting-replay failure probabilities.

That byte-level binding proof is now formalized.  `N32ChallengePointByteCodec`
uses a fixed-layout domain-separated frame containing the public key, message,
and seven commitments, and proves:

```text
encodePoint p = encodePoint q => p = q.
```

The canonical challenge codec is now named too:

```text
N32CanonicalChallengeByteCodec.packed17
```

It packs the `2^133` challenge space into 17 bytes and proves the decoder
round trip.  The associated leading-byte profile
`N32CompactSignatureByteProfile.leadingN32_9bit_2free` certifies the current
255-byte row: 17 challenge bytes plus seven 34-byte compact active responses.
The byte-vector route is now explicit through
`N32ByteSerializedVerifierInputs`, `N32FixedCompactResponseByteDecoder`, and
`N32ByteConcreteCompactSixReplayExtractionInputs`.

`N32ByteReplayBinding` applies this to accepting replay draws, and
`N32SixDrawByteBindingInputs` gives the byte-bound final form:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_d acceptFailureBound[d]
  - coverageFailureBound.
```

The accepting-replay term is first reduced to a density lower bound.  For each
draw `d`, `N32ReplayAcceptanceProfile` records a value `alpha[d]` such that
every adversary-side state accepts at least an `alpha[d]` fraction of the
canonical challenge space.  Lean proves the miss bound:

```text
Pr[acceptFailure d] <= 1 - alpha[d].
```

The resulting final form is:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_d (1 - alpha[d])
  - coverageFailureBound.
```

That "every state" version is too strong to derive from an arbitrary forger.
The heavy-state version is now formalized as
`N32SixDrawHeavyAcceptanceInputs`.  It uses the state-local density

```text
p_d[state] = Pr_c[replay draw d accepts after programming c]
```

and a heavy predicate satisfying

```text
heavy[state] => alpha[d] <= p_d[state].
```

Light states are charged once through `lightReplayBound`, not once per draw:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - lightReplayBound
  - sum_d (1 - alpha[d])
  - coverageFailureBound.
```

Lean also proves the threshold source for these quantities.  If
`heavy_alpha[state]` is defined by `alpha <= p[state]`, then accepting on a
light state has probability at most `alpha`.  This is the formal answer to
where `alpha[d]` comes from: it is a threshold on state-local challenge
density, plus an explicit light-state loss.

The stronger moment route is now formalized too.  The generic theorem
`productFunctionProduct_acceptProb_forall_eq_state_product` proves:

```text
Pr[all replay draws accept]
  = E_state[ product_d p_d[state] ].
```

When all six replay draws use the same state-local accepting set,
`productFunctionProduct_identical_acceptProb_eq_state_moment` gives:

```text
Pr[all six replay draws accept] = E_state[p[state]^6].
```

This is the proof shape that should replace the additive
`sum_d (1 - alpha[d])` loss for ordinary forgers whose success probability is
not close to one.  The convexity/Jensen bridge is now formalized:

```text
E[p^6] >= E[p]^6.
```

Lean also proves the first-run bridge:

```text
Pr[forgery] - Pr[queryMiss] <= E[p].
```

The concrete N32 endpoint is:

```text
L^6 <= Pr[all six replay draws accept]
```

for any nonnegative lower bound `L <= E[p]`.

The serialized verifier implication is now formalized.  The Lean interface
`N32SerializedVerifierProductMomentInputs` proves:

```text
Pr[forgery] - 2^-133 <= E_state[p[state]]

L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 <= Pr[all six independent replay draws accept]
```

where `p[state]` is the density of challenges accepted by the verifier after
programming the challenge point for that state.

The product-moment fork wrapper is now formalized as
`N32SixReplayProductMomentForkInputs`.  It uses the six-replay trace

```text
State × (Fin 6 -> N32CanonicalChallenge)
```

and assumes the semantic replay implication:

```text
all six one-shot accepting programmed transcripts
  => acceptingForks.
```

It then proves:

```text
L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 - coverageFailureBound <= Pr[extractable].
```

Equivalently, when `Pr[forgery] >= 2^-133`:

```text
(Pr[forgery] - 2^-133)^6 - coverageFailureBound <= Pr[extractable].
```

The compact active-only path is now formalized separately.  The old
`MultiForkLinearRound` boundary stored full responses, while the actual small
signature target should only serialize active blocks.  The new deterministic
object is:

```text
CompactMultiForkLinearRound
```

It stores response coordinates only for signed-unit challenge blocks and proves
that opposite signed active compact openings recover the secret.  The N32
transcript layer adds:

```text
N32CompactResponseCodec
N32CompactProductBlockVerifierSoundness
N32CompactSixReplayExtractionInputs
```

The last interface proves:

```text
L <= Pr[forgery] - 2^-133
0 <= L
-------------------------------------------------------
L^6 - coverageFailureBound <= Pr[compactExtractable].
```

For every `compactExtractable` trace, Lean proves the active-only extractor
returns the product-block secret.

The concrete compact coverage event is now fixed as
`N32CompactCoverage.failure`.  It misses exactly when some product block has no
opposite signed active replay pair in any repetition.  Lean proves:

```text
not failure => CoversBlocks replayOppositeActive.
```

The union-bound target is:

```text
N32CompactCoverage.failureUnionBound
  = 16 * (2 * (17/32)^6)^7
  ~= 2^-27.326561.
```

`N32ConcreteCompactSixReplayExtractionInputs` uses that concrete event.  Lean
now proves the fixed-block coverage probability and derives the canonical
query-miss certificate internally for the compact product-moment wrapper:

```text
Pr[fixed block is missed] <= (2 * (17/32)^6)^7.
queryMiss = forgery && !pointQueried.
```

This is the sign-union bound.  The exact inclusion-exclusion value subtracts
`(1/16)^6` before the seventh power, but the difference is negligible at this
candidate scale.

With the certified coverage and query-miss pieces in the wrapper, Lean gives:

```text
(Pr[forgery] - 2^-133)^6
  - N32CompactCoverage.failureUnionBound
  <= Pr[compactExtractable].
```

The concrete compact wrapper also proves that a positive lower bound yields an
actual extractable trace and recovered-secret equality:

```text
exists compactExtractable trace whose extractedSecret = secret.
```

That endpoint is connected to keygen security through
`N32ProductBlockSecretRecoveryGame`.  Once a concrete scheme supplies a
`BreaksKeygen` predicate and a sound builder from recovered product-block
secret to breaker, Lean derives:

```text
exists keygen breaker.
```

The transcript file also instantiates this as direct secret recovery:

```text
N32ProductBlockSecretRecoveryGame.secretRecovery
exists_secretRecoveryBreaker_of_positive_lower_bound
```

It also proves the no-break contrapositive:

```text
forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
```

so a no-keygen-break assumption forces
`(Pr[forgery] - 2^-133)^6` below the certified compact coverage loss.
`N32ByteConcreteSignatureSecurityCertificate` packages that final endpoint for
the byte-vector scheme interface.

The remaining concrete transcript proof must ensure:

- all six accepting draws are bound to the same public key, message, round
  commitment, and repetition index;
- the challenge distribution is the same signed product-block distribution used
  by the sampler analysis;
- rewinding only changes random-oracle answers for the challenge point being
  forked;
- abort/rejection behavior is already absorbed into the observed accepting
  transcript event;
- the implementation's compact response parser satisfies the active linear
  response equation on every accepted one-shot transcript;
- the implementation's keygen security game treats recovered product-block
  secret as a break.
