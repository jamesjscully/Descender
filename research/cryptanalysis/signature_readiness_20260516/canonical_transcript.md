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

Lean now also names the canonical packed challenge codec:

```text
N32CanonicalChallengeByteCodec.packed17
```

It proves that the `2^133` challenge space is injected into 17 bytes, with a
decoder round trip:

```text
decode(encode(c)) = c.
```

The byte space has `2^136` elements, so this leaves exactly three slack bits.
The leading compact signature byte profile is:

```text
N32CompactSignatureByteProfile.leadingN32_9bit_2free
```

and certifies:

```text
17 challenge bytes
30 active response coordinates per repetition
34 response bytes per repetition
255 total signature bytes
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

## Byte-Level Binding

The byte-level binding boundary is now formalized as a fixed-layout
domain-separated frame:

```text
N32ChallengePointByteFrame
```

The frame contains:

- a domain tag;
- public-key bytes;
- message bytes;
- seven commitment byte strings in `Fin 7` repetition order.

`N32ChallengePointByteCodec` supplies the concrete component encoders and their
injectivity proofs.  Lean proves:

```text
encodePoint p = encodePoint q => p = q.
```

This is the byte-level transcript binding theorem: equality of the verifier's
challenge-point bytes implies equality of the semantic public key, message, and
commitment vector.

For byte-vector scheme objects, the concrete instantiation is now named:

```text
N32ChallengePointByteCodec.byteVector
N32ByteSerializedVerifierInputs
```

This fixes public keys, messages, and commitments as already-serialized byte
vectors and uses `N32CanonicalChallengeByteCodec.packed17` for the challenge.

`N32ByteReplayBinding` applies this to replay draws.  If an accepting replay
draw has byte equality with the expected challenge point, then the draw is
semantically same-point:

```text
drawAccepts d => sameChallengePoint d.
```

Therefore:

```text
Pr[pointFailure d] <= 0.
```

This removes the same-point replay-loss term for the byte-vector route.

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

The draw-failure probability is now split into two concrete component events:

```text
acceptFailure d = not drawAccepts d
pointFailure d = drawAccepts d and not sameChallengePoint d
drawFailure d <= acceptFailure d + pointFailure d.
```

`N32ReplayScheduleBounds` packages per-draw upper bounds for those two
components and proves:

```text
Pr[drawFailure d]
  <= acceptFailureBound[d] + pointFailureBound[d].
```

There are also zero-bound lemmas for deterministic cases:

```text
always drawAccepts d        => Pr[acceptFailure d] <= 0
drawAccepts d => samePoint d => Pr[pointFailure d] <= 0
```

This matters for the actual transcript.  If the byte-level verifier binds the
public key, message, commitment vector, and repetition indices into the
challenge point deterministically, the `pointFailureBound` term should be zero.
The nontrivial replay loss is then the accepting-replay term.

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

`N32SixDrawComponentInputs` is the preferred interface for the next
instantiation.  It takes the component bounds directly and proves:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_{d : Fin 6}
      (acceptFailureBound[d] + pointFailureBound[d])
  - coverageFailureBound.
```

With byte-level binding, `N32SixDrawByteBindingInputs` is sharper:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_{d : Fin 6} acceptFailureBound[d]
  - coverageFailureBound.
```

## Accepting Replay Density

The accepting-replay loss is now connected to a concrete density statement.
For each replay draw `d` and adversary-side state, define the set of programmed
canonical challenges that make that replay accept:

```text
acceptingChallenge[d, state] : N32CanonicalChallenge -> Bool
```

`N32ReplayAcceptanceProfile` records a lower bound `alpha[d]` such that every
state accepts at least that fraction of the canonical challenge space:

```text
alpha[d] <= Pr_{c uniform}[acceptingChallenge[d, state](c)]
```

Lean proves:

```text
Pr[acceptFailure d] <= 1 - alpha[d].
```

Combining this with byte-level binding gives
`N32SixDrawAcceptanceDensityInputs`:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - sum_{d : Fin 6} (1 - alpha[d])
  - coverageFailureBound.
```

This is the current sharp formal boundary.  The remaining replay task is to
prove useful lower bounds on `alpha[d]` for the concrete replay/programming
schedule.

## Deriving Alpha

The next obstruction is now explicit: a global successful-forgery probability
does not imply that every adversary-side state has large accepting-challenge
density.  Lean now records the state-local quantity:

```text
p[state] = Pr_{c uniform}[acceptingChallenge[state](c)].
```

`n32StateHeavy` defines the threshold predicate

```text
heavy_alpha[state] := alpha <= p[state].
```

and proves:

```text
heavy_alpha[state] => alpha <= p[state].
```

It also proves the light-state accounting lemma:

```text
Pr[not heavy_alpha[state] and acceptingChallenge[state](c)] <= alpha.
```

So `alpha` is no longer an unexplained modeling constant.  It is a chosen
threshold on the state-local accepting density, and the proof must either
condition on states above that threshold or pay the corresponding light-state
loss.

The usable six-draw endpoint is now `N32SixDrawHeavyAcceptanceInputs`.  It
charges light replay once, then uses the six per-draw density lower bounds only
on heavy states:

```text
Pr[extractable] >=
  Pr[forgery]
  - 2^-133
  - lightReplayBound
  - sum_{d : Fin 6} (1 - alpha[d])
  - coverageFailureBound.
```

This is the right interface for a concrete forking lemma.  To instantiate it,
the replay proof must supply:

- the state-local accepting set for each draw;
- the heavy-state predicate used by the replay schedule;
- `alpha[d] <= p_d[state]` on heavy states;
- one bound for the light replay event.

## Product-Moment Replay

The stronger route is now formalized at the finite-probability layer.  Lean
defines a product distribution over a finite function space:

```text
functionProductFiniteProbability
```

and proves the independent-draw product rule:

```text
Pr_x[forall d, A[d](x[d])]
  = product_d Pr[A[d]].
```

The state-local replay theorem is:

```text
productFunctionProduct_acceptProb_forall_eq_state_product
```

It says that, after cutting the execution at the challenge point and sampling
independent programmed challenges for each draw,

```text
Pr[all replay draws accept]
  = E_state[ product_d p_d[state] ].
```

The identical-draw specialization is:

```text
productFunctionProduct_identical_acceptProb_eq_state_moment
```

For `N32ReplayDraw = Fin 6`, Lean also records `Fintype.card N32ReplayDraw =
6`, so the identical-draw replay success term is the sixth moment:

```text
Pr[all six replay draws accept] = E_state[p[state]^6].
```

This is the proof shape we want when `alpha` is not close to one.  Instead of
paying `sum_d (1 - alpha[d])`, Lean now lower-bounds the sixth moment from the
first-run accepting probability.  The generic Jensen certificate is:

```text
E_state[p[state]^6] >= E_state[p[state]]^6.
```

In Lean this is `finiteProbability_mean_pow_le_moment`, lifted to replay by
`productFunctionProduct_identical_acceptProb_ge_mean_pow`.

The first-run bridge is formalized too:

```text
n32_product_acceptProb_eq_state_acceptProbability
n32_stateAcceptMean_ge_forgery_sub_queryMiss
```

Together these support the concrete N32 endpoint:

```text
n32_six_replay_identical_acceptProb_ge_lowerBound_sixth
```

Given any nonnegative lower bound `L` on the average state-local accepting
density, Lean proves:

```text
L^6 <= Pr[all six replay draws accept].
```

## Serialized Verifier Implication

The first-run implication is now formalized.  Lean defines:

```text
N32SerializedVerifier
```

This is the verifier-facing boundary for a parsed signature transcript.  It
contains:

- the fixed-layout challenge-point byte codec;
- the response recovered from the adversary-side state;
- the verifier accept predicate for
  `N32AcceptingTranscript PublicKey Message Commitment Response`;
- the byte-frame obligation saying accepting transcripts use the canonical
  encoding of the semantic public key, message, and seven commitments;
- the Fiat-Shamir recomputation obligation saying an accepting transcript's
  output challenge equals the random-oracle value bound to that point.

From this, Lean defines the one-shot accepting set

```text
oneShotAccepts[state, challenge]
```

and proves:

```text
outputAccepts(trace) => oneShotAccepts(trace.state, trace.actualChallenge).
```

The product-moment interface is:

```text
N32SerializedVerifierProductMomentInputs
```

It combines the serialized verifier implication with the query-miss
certificate and proves:

```text
Pr[queryMiss] <= 2^-133

Pr[forgery] - 2^-133
  <= E_state[p[state]]
```

where

```text
p[state] = Pr_c[oneShotAccepts[state, c]].
```

The concrete endpoint is now:

```text
L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 <= Pr[all six independent replay draws accept].
```

## Product-Moment Fork Wrapper

The six-replay/fork wrapper is now formalized too:

```text
N32SixReplayProductMomentForkInputs
```

This is the non-additive replay interface.  Its multi-fork game trace is:

```text
State × (Fin 6 -> N32CanonicalChallenge)
```

with distribution:

```text
stateDist × six independent uniform programmed challenges.
```

The wrapper assumes the concrete semantic implication:

```text
all six oneShotAccepts[state, challenge[d]]
  => acceptingForks
```

and then proves:

```text
L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 <= Pr[acceptingForks].
```

The final extractor-facing theorem is:

```text
L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 - coverageFailureBound <= Pr[extractable].
```

The common direct corollary is:

```text
0 <= Pr[forgery] - 2^-133
---------------------------------------------------------------
(Pr[forgery] - 2^-133)^6 - coverageFailureBound <= Pr[extractable].
```

## Compact Active-Only Extraction

The compact-response path is now separated from the older full-response
idealization.  `BlockChallengeSoundness.lean` defines:

```text
CompactMultiForkLinearRound
```

Here a replay draw only exposes response coordinates for blocks whose scalar
is a signed unit.  The deterministic extractor now proves:

```text
opposite signed active compact responses cover every block
----------------------------------------------------------
extractedSecret = secret.
```

`SignatureTranscript.lean` also defines the N32 challenge scalar map:

```text
N32RoundChallenge.scalar : N32Block -> ZMod q
```

and proves every scalar is `0`, `1`, or `-1`.  The compact response boundary is:

```text
N32CompactResponseCodec
N32CompactProductBlockVerifierSoundness
```

The verifier soundness obligation is exactly active-only:

```text
oneShotAccepts[state, challenge]
active block b in repetition r
----------------------------------------------------
decode(response[state, r], challenge[r], b)
  = mask[state, r] + scalar(challenge[r], b) * secret.
```

Finally, `N32CompactSixReplayExtractionInputs` combines the product-moment
replay proof with active-only extraction:

```text
L <= Pr[forgery] - 2^-133
0 <= L
-------------------------------------------------------
L^6 - coverageFailureBound <= Pr[compactExtractable].
```

On every `compactExtractable` trace, Lean proves that the active-only compact
extractor recovers the secret.

## Concrete Coverage Event

The compact coverage event is now named in Lean:

```text
N32CompactCoverage.failure
```

It is the real event for the six programmed N32 challenges:

```text
exists block b such that no repetition r has
two replay draws with opposite signed active scalars on b.
```

Lean proves the deterministic side:

```text
not N32CompactCoverage.failure
  => CoversBlocks replayOppositeActive.
```

The union-bound target is also fixed:

```text
N32CompactCoverage.singleBlockMissBound
  = (2 * (17/32)^6)^7

N32CompactCoverage.failureUnionBound
  = 16 * (2 * (17/32)^6)^7
  ~= 2^-27.326561.
```

The wrapper

```text
N32ConcreteCompactSixReplayExtractionInputs
```

uses this concrete coverage event directly.  It no longer has a separate
coverage-probability input; Lean supplies the canonical one-block bound:

```text
Pr[block b is missed] <= (2 * (17/32)^6)^7.
```

It also no longer has a separate query-miss certificate input.  The wrapper
defines query miss as:

```text
forgery && !pointQueried
```

and proves the blind-guess condition from serialized verifier acceptance and
challenge recomputation.

The certified value uses the sign-union bound; exact inclusion-exclusion would
subtract `(1/16)^6` per repetition and improves the displayed bit count by only
about `0.000014` bits.  Lean supplies the sixteen-block union bound and proves:

```text
(Pr[forgery] - 2^-133)^6
  - N32CompactCoverage.failureUnionBound
  <= Pr[compactExtractable].
```

It also proves the positive-witness form:

```text
0 < (Pr[forgery] - 2^-133)^6
    - N32CompactCoverage.failureUnionBound
  => exists compactExtractable trace whose extractedSecret = secret.
```

The transcript module then exposes this as an abstract keygen-break endpoint:

```text
N32ProductBlockSecretRecoveryGame
exists_keygenBreaker_of_positive_lower_bound
```

A concrete signature scheme closes that interface by defining its keygen break
predicate and proving that the recovered product-block secret constructs such a
breaker.

Lean also includes the plain secret-recovery instantiation:

```text
N32ProductBlockSecretRecoveryGame.secretRecovery
exists_secretRecoveryBreaker_of_positive_lower_bound
```

where the break predicate is exactly recovered product-block secret equality.

The final contrapositive security theorem is also available:

```text
forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
```

Under a no-keygen-break assumption, the positive compact-extraction lower bound
is impossible.

The byte-concrete packaged endpoint is:

```text
N32ByteConcreteSignatureSecurityCertificate
```

Its theorem `forgeryMinusGuess_sixth_le_failureUnionBound` gives the final
forger-success bound from the byte transcript obligations and the keygen
no-break assumption.

## Remaining Work

The abstract forking layer and the canonical challenge-count layer now meet at
the right active-only product-moment boundary.  The next proof can now take
either of two forms.

For the heavy-state route, instantiate:

```text
heavy[state] =>
  alpha[d] <= Pr_{c uniform}[acceptingChallenge[d, state](c)]
```

For the product-moment route, instantiate:

```text
all six independent one-shot accepts => acceptingForks
```

The abstract wrapper and active-only extraction path for that route are
formalized.  The remaining product-moment work is now implementation-specific
instantiation:

- verifier recomputation of the random-oracle challenge point for the concrete
  transcript bytes;
- the concrete compact response parser satisfies the active linear response
  equation on every accepted one-shot transcript;
- the actual keygen security predicate treats recovered product-block secret as
  a break.
