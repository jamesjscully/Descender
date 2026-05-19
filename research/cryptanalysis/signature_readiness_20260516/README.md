# Signature Readiness Cryptanalysis

This pass tests whether the 172-byte coordinate-subview KEM/public-view
candidate should immediately be used as a signature primitive.

## Target

The default probe targets the current leading 128-bit coordinate-subview row:

- proof profile: `Descender128CoordinateSubviewProfile.serialization`
- `q = 12289`
- `n = 48`
- `m0 = 80`
- `m1 = 32`
- public view coordinates: `80`
- secret/noise radius used by the row: `eta = 12`
- public key size: `172 B`
- ciphertext size: `172 B`

## Probe Model

The implemented probe attacks the most direct Fiat-Shamir-style signing idea:

```text
z = y + c*s mod q
```

where `s` is the small secret, `c` is a public scalar challenge, and `y` is a
bounded signing mask.  Given several transcripts `(c, z)`, the attacker
enumerates each possible small secret coordinate and scores whether the residual
`z - c*s'` lies inside the mask interval.

This is not an attack on the KEM/public-view candidate.  It is an attack on a
naive signature transcript relation that exposes linearly masked small secrets.

## Result

The direct scalar-response design is not viable.  With `secret_eta = 12`, the
coordinate-wise maximum-likelihood attack recovers the secret quickly:

- mask bound 12, 32, or 64: usually broken with one transcript;
- mask bound 128 or 256: essentially broken by two transcripts;
- mask bound 1024: broken by four to eight transcripts;
- mask bound 2048: broken by eight transcripts.

The problem is structural: each coordinate of `s` has only 25 possible values,
and the transcript gives a public noisy equation for that coordinate.

## Abort-Size Search

The follow-up size search asks whether Fiat-Shamir-with-aborts can still be
small if the response distribution is made independent of the secret by
rejection sampling.

Two challenge models were compared:

1. **Scalar challenges.**  These are compatible with the current plain linear
   public relation because the verifier can compute `c * t`.
2. **Product-block sparse challenges.**  These are hypothetical.  They assume a
   new block-local challenge action with roughly 16 product blocks and signed
   sparse challenges, so each challenge has high entropy while each affected
   secret coefficient is still multiplied only by `-1`, `0`, or `1`.

The scalar route is not obviously impossible, but it is not tiny.  The best
screened rows are around `1124 B` with roughly `2^13` to `2^18` expected retry
cost, depending on the scalar bound.  This is likely above the target if small
signatures are the main goal.

The product-block challenge route is much more interesting.  The original
size model found rows around `212 B` to `248 B`, but that was a proof-shape
screen: it did not charge the rejection exponent over every active response
coordinate.  The concrete exact-box sampler analysis gives a sharper picture:
for `n = 48`, the clean full-block target is around `496 B`; for the `n = 32`
candidate, the clean full-block target is around `304 B`.  Intermediate rows
are smaller, but they must now be judged by fork-delta coverage, not just by
whether a single challenge touches each block.  With six challenge draws per
repeated commitment, the compact active-only transcript has a `255 B` `n = 32`
row above `24` bits of exact any-block fork-delta coverage.

This is the most promising small-signature research target found so far.

## Product-Block Challenge Action

The first algebraic blocker for the product-block route is now discharged for
block-diagonal systematic views.  `Descender/SystematicLee.lean` defines
`blockScaleFree`, `blockScalePivot`, and `blockScaleView`, then proves:

- `blockDiagonalSystematicPivotImage_blockScale`
- `blockScaleView_systematicRelation`
- `blockScalePivot_systematicDualMap`

Together these say that block-local challenge scaling commutes with the
systematic public relation and with the verifier-facing dual map.  In signature
terms, the verifier can apply a sparse signed product-block challenge directly
to the public view, while the response bound only sees coefficients in
`{-1, 0, 1}` instead of a large scalar challenge.

This was the original correctness-side boundary.  The later transcript proof
now supplies the rejection-sampling abstraction, canonical challenge
serialization, Fiat-Shamir replay/product-moment theorem, compact extraction,
and keygen-break bridge.  The remaining scheme-specific obligations are the
implementation's serialized verifier equation, concrete response-parser
soundness, and key-generation cryptanalysis for the structured view.

`Descender/ProductBlockSignature.lean` also proves the first correctness-side
lemma for this route: sparse signed block challenges preserve the centered
coordinate noise bound, and a noisy systematic public relation remains noisy
systematic after challenge application with the same `eta`.  The theorem also
has a serialized-error form that preserves the combined `eta + roundingEta`
bound when rounding is represented as an additional bounded error term.

## Distributional Rejection Proof

`Descender/SignatureDistribution.lean` now factors the Fiat-Shamir-with-aborts
distributional proof.  A `RejectionSamplingCertificate` states that every
accepted-response test has unnormalized mass
`acceptance * target.acceptProb test`; Lean proves from this that:

- the acceptance mass is independent of the secret and challenge;
- the conditional accepted-response distribution equals the target distribution;
- accepted responses, and any deterministic compact encoding of them, are
  secret-independent.

For product-block signatures this is specialized to the response
`z = y + chi * s` via `productBlockLinearResponse` and
`productBlockRejectionSampler`.  See `distributional_signature_proof.md`.

## Concrete Sampler Analysis

`tools/parameter-search/src/bin/signature_sampler_analysis.rs` analyzes an
exact finite sampler: for each active coordinate with shift `|v| <= eta`, it
samples `y` uniformly from `[-z-eta,z+eta]`, emits `y + v`, and accepts exactly
when the response lies in `[-z,z]`.  This exactly realizes a
secret-independent uniform target law on `[-z,z]`.

The key correction is that rejection cost is paid per active response
coordinate.  This makes very sparse challenges look tiny but weak in
block-coverage soundness.  The current reports distinguish three coverage
notions:

- active challenge coverage: a single challenge touches a block;
- full-response fork-delta coverage: an idealized transcript exposes all
  response coordinates, so zero-vs-nonzero forks can extract;
- compact fork-delta coverage: the actual small transcript exposes only active
  coordinates, so extraction only counts opposite signed active openings.

The compact fork-delta metric is the relevant one for small signatures.  The
current reports use six challenge draws for the same commitment per repeated
round:

- `sampler_analysis_n48.md`: best full-block row `496 B` with log2 retry
  `14.63`; best compact fork-delta `>=24` row `415 B`; best compact
  fork-delta `>=16` row `281 B`.
- `sampler_analysis_n32.md`: best full-block row `304 B` with log2 retry
  `19.23`; best compact fork-delta `>=24` row `255 B`; best compact
  fork-delta `>=16` row `197 B`.

Sizes use canonical challenge encoding.  Carrying a fixed 32-byte challenge
digest instead adds about `13 B` to `16 B` in these rows.

## Block-Challenge Soundness

`Descender/BlockChallengeSoundness.lean` now formalizes the deterministic
extraction target behind those coverage rows.  It proves that if repeated
challenges cover every needed block, local active-block extractors assemble a
globally valid block witness.  It also proves the response-difference identity
and forked extraction theorem needed for Fiat-Shamir analysis:

```text
(y + chi0 * s) - (y + chi1 * s) = (chi0 - chi1) * s.
```

For compact active-only transcripts, the formal boundary also records that
opposite signed active challenges give an extractable delta when `2` is
invertible.  The multi-fork boundary is now formalized too: several accepting
draws for the same commitment/mask recover the whole product-block secret if
their compact opposite-sign deltas cover every block.  For the concrete report
modulus, Lean records `(6145 : ZMod 12289) * 2 = 1`.  The sampler reports
compute exact any-block-missed probabilities for both active coverage and
compact fork-delta coverage.  See `block_challenge_soundness.md`.

## Fiat-Shamir Forking

`Descender/SignatureForking.lean` now adds the finite-probability shell around
the deterministic multi-fork extractor.  The main loss statement is:

```text
Pr[extractable] >= Pr[accepting same-commitment forks]
                  - Pr[compact fork-delta coverage failure].
```

The wrapper `MultiForkForkingCertificate` packages the end-to-end accounting:

```text
Pr[extractable] >= forgerSuccess - forkingLoss - coverageFailureBound.
```

Thus the `2^-27.33` compact coverage-failure term for the `255 B`, `n = 32`
row now plugs into a formal theorem.  The still-open part is the concrete
random-oracle replay analysis.  This is now split into two events:
`queryMiss`, where the forger did not expose the challenge hash point needed
for replay, and `replayFailure`, where replaying that point fails to obtain six
accepting same-commitment draws.  Lean proves

```text
Pr[acceptingForks] >= Pr[forgery] - Pr[queryMiss] - Pr[replayFailure].
```

For the six-draw candidate, Lean also provides a transcript-facing wrapper:
`SixDrawForkingBounds` reduces the open loss terms to one blind challenge-guess
event and six per-draw replay-failure events.  For the `n = 32`, `255 B` row,
the canonical challenge space is `(binom(16,15) * 2^15)^7 = 2^133`, so the
query-miss target is `2^-133` once the challenge serializer/verifier is fixed.
The remaining proof work is to instantiate the six draw-failure bounds for the
actual random-oracle replay schedule.

See `fiat_shamir_multifork.md`.

## Canonical Transcript

`Descender/SignatureTranscript.lean` now fixes the first concrete transcript
surface for the `n = 32`, `255 B` row.  A round challenge is represented by the
one omitted block among sixteen plus signs on the remaining fifteen active
blocks, and a full challenge is seven such round challenges.  Lean proves:

```text
Fintype.card N32CanonicalChallenge = 2^133.
```

It also proves the exact blind challenge-guess loss and its finite
union-bound form:

```text
Pr[actual_challenge = guess] = 2^-133
Pr[exists g, guess[g] = actual_challenge] <= #Guess / 2^133.
```

The query-log layer now factors the transcript as adversary state times an
independent uniform challenge and proves the concrete query-miss bound:

```text
Pr[queryMiss] <= 2^-133.
```

The replay adapter defines six per-draw failures as failed acceptance or failed
same-point binding, then packages everything into the existing
`SixDrawForkingBounds` theorem:

```text
Pr[extractable] >= Pr[forgery]
  - 2^-133
  - sum drawFailureBound
  - coverageFailureBound.
```

The replay loss is now split one step further:

```text
Pr[drawFailure d]
  <= Pr[acceptFailure d] + Pr[pointFailure d].
```

`N32SixDrawComponentInputs` therefore gives the transcript-facing bound:

```text
Pr[extractable] >= Pr[forgery]
  - 2^-133
  - sum_d (acceptFailureBound[d] + pointFailureBound[d])
  - coverageFailureBound.
```

For deterministic transcript binding, `pointFailureBound[d]` should be zero;
the remaining replay work is bounding each accepting-replay failure.

The byte-level binding proof is now in Lean.  `N32ChallengePointByteCodec`
models a fixed-layout domain-separated byte frame for the public key, message,
and seven commitments, and proves that equal challenge-point bytes imply equal
semantic challenge points.  `N32CanonicalChallengeByteCodec.packed17` is the
canonical challenge serializer: it injects the `2^133` challenge space into a
17-byte space and proves `decode(encode(c)) = c`, with three slack bits.
For already-serialized scheme objects, `N32ChallengePointByteCodec.byteVector`
instantiates the public-key, message, and commitment codecs as identity maps on
fixed byte vectors.  `N32ByteSerializedVerifierInputs` fixes this byte-vector
route together with the packed challenge codec.
`N32SixDrawByteBindingInputs` uses this to set all same-point replay terms to
zero:

```text
Pr[extractable] >= Pr[forgery]
  - 2^-133
  - sum_d acceptFailureBound[d]
  - coverageFailureBound.
```

The accepting-replay term is now density-based.  If `alpha[d]` lower-bounds,
for every adversary-side state, the fraction of canonical challenges that make
replay draw `d` accept, then Lean proves:

```text
Pr[extractable] >= Pr[forgery]
  - 2^-133
  - sum_d (1 - alpha[d])
  - coverageFailureBound.
```

The proof now also records how such an `alpha` can be derived.  For a
state-local accepting density

```text
p[state] = Pr_c[acceptingChallenge[state](c)],
```

Lean defines threshold-heavy states by `alpha <= p[state]` and proves that
accepting on a light state costs at most `alpha`.  The practical final
interface is therefore heavy-state based:

```text
Pr[extractable] >= Pr[forgery]
  - 2^-133
  - lightReplayBound
  - sum_d (1 - alpha[d])
  - coverageFailureBound.
```

This makes the remaining task precise: derive useful heavy-state thresholds
and a small light-replay bound for the concrete replay schedule.

The stronger replay route is now formalized as a product-moment theorem.  After
cutting the execution at the challenge point and sampling independent replay
challenges, Lean proves:

```text
Pr[all replay draws accept]
  = E_state[ product_d p_d[state] ].
```

For identical draw distributions and six draws this becomes:

```text
Pr[all six replay draws accept] = E_state[p[state]^6].
```

This is the route that avoids an additive `sum_d (1 - alpha[d])` loss when the
forger's success probability is not near one.  The Jensen-style lower bound
from first-run success to the sixth moment is now formalized, along with the
bridge

```text
Pr[forgery] - Pr[queryMiss] <= E_state[p[state]].
```

See `canonical_transcript.md`.

## Pay-To-Hash

Hash-only authorization can be tiny, but it is not a reusable signature scheme.
For example, a blockchain account can commit to `H(secret)` and authorize once
by revealing `secret`, giving a very small witness.  That is a stateful one-time
spend primitive, not a general signature:

- the secret must not be reused;
- the transaction format must bind the preimage to the intended spend to avoid
  theft/front-running;
- account state must rotate after every spend;
- it does not give ordinary reusable message signatures.

Pay-to-hash may be viable as a blockchain-specific mode, but it is separate
from a reusable Descender signature scheme.

## Signature Constraints

A signature construction should not expose responses of the form
`small mask + public challenge * small secret` unless it has a full
Lyubashevsky/Dilithium-style rejection-sampling proof with masks large enough
to make the response distribution statistically independent of `s`.

More promising directions:

1. A ZK/MPC proof of knowledge for the public-view relation, where linear
   masked secrets are not exposed directly.
2. A lattice-signature design with a large, carefully analyzed masking
   distribution and rejection sampling, treated as a separate signature scheme
   rather than a simple wrapper around the KEM row.
3. A hash-and-sign route only if we can construct and analyze a trapdoor sampler
   for the structured public view, which is not available yet.

## Current Position

The 172-byte row remains the leading compact KEM/public-view candidate.  It is
not yet a safe signature primitive.  The smallest plausible reusable-signature
route is now a product-block Fiat-Shamir-with-aborts design.  The
verifier-computable block challenge action is formalized for block-diagonal
systematic views, and the basic signed-challenge noise bound is formalized.
The distributional rejection-sampling theorem is also formalized as a generic
finite-probability proof.  The exact-box sampler, compact fork-delta coverage
analysis, deterministic multi-fork extractor, and finite-probability
random-oracle replay shell are now concrete.  The honest signing relation is
now named as well: the signer publishes a product-block commitment
`t = A * mask`, uses public key `u = A * secret`, returns
`z = mask + chi * secret`, and the verifier checks
`A * z = t + chi * u`.  Lean also has the compact active-only version where
only signed active block coordinates of `z` are serialized, and it proves that
forked accepting compact transcripts instantiate the existing product-block
secret extractor.  The serialized verifier
implication is formalized too: an accepting output transcript recomputes the
random-oracle challenge point and lands in the one-shot accepting set, giving
the product-moment bound

```text
L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 <= Pr[all six independent replay draws accept].
```

The six-replay/fork wrapper is formalized as well.  It proves the extractor
form:

```text
L <= Pr[forgery] - 2^-133
0 <= L
---------------------------------------------
L^6 - coverageFailureBound <= Pr[extractable].
```

The compact active-only response path is formalized too.  The deterministic
extractor now has a `CompactMultiForkLinearRound` form that only uses active
response coordinates, and `N32CompactSixReplayExtractionInputs` proves:

```text
L <= Pr[forgery] - 2^-133
0 <= L
-------------------------------------------------------
L^6 - coverageFailureBound <= Pr[compactExtractable].
```

Every `compactExtractable` trace is proven to recover the product-block
secret.

The concrete compact coverage event is now fixed as
`N32CompactCoverage.failure`, with union-bound target:

```text
16 * (2 * (17/32)^6)^7 ~= 2^-27.326561.
```

`N32ConcreteCompactSixReplayExtractionInputs` uses that event directly, and
Lean now proves the fixed-block miss calculation needed by the wrapper.  The
same wrapper also internalizes the canonical query-miss event as
`forgery && !pointQueried`; the serialized verifier's recomputed-challenge
lemma supplies the blind-guess condition, so no separate query-miss certificate
is needed there.  The certified compact-coverage constant uses the six-draw
sign-union bound; the sharper inclusion-exclusion constant changes the budget
by less than `0.000014` bits.

The current N32 byte profile is also named in Lean:

```text
N32CompactSignatureByteProfile.leadingN32_9bit_2free
```

It certifies the 255-byte accounting for the leading row:

```text
challenge bytes = 17
response coordinates per repetition = 30
response bytes per repetition = 34
signature bytes = 255
```

`N32FixedCompactResponseByteDecoder` and
`N32ByteCompactProductBlockVerifierSoundnessInputs` now fix the response side
as fixed-length bytes plus a decoder proof.  `N32ByteConcreteCompactSixReplayExtractionInputs`
bundles the byte-vector public data, `packed17` challenge bytes, response-byte
decoder, canonical query-miss event, and certified compact coverage event.
The remaining implementation-specific proof is to show that the actual verifier
recomputes the random-oracle challenge from these bytes and that the concrete
response parser satisfies the active linear equation on accepted transcripts.

Lean also has the positive-witness endpoint: if

```text
0 < (Pr[forgery] - 2^-133)^6
    - N32CompactCoverage.failureUnionBound,
```

then there exists a concrete six-replay trace whose active-only extractor
returns the product-block secret.

Lean now also has the abstract keygen-break bridge:

```text
N32ProductBlockSecretRecoveryGame
```

packages a concrete keygen security game by giving a `BreaksKeygen` predicate
and a builder from a recovered product-block secret to a breaker.  The theorem
`exists_keygenBreaker_of_positive_lower_bound` says that the same positive
lower bound above yields an actual keygen breaker for every instantiation that
can validate that builder.  This is the formal interface where the remaining
scheme-specific keygen game plugs into the transcript proof.

There is also a canonical secret-recovery instantiation:

```text
N32ProductBlockSecretRecoveryGame.secretRecovery
exists_secretRecoveryBreaker_of_positive_lower_bound
```

where `BreaksKeygen recovered` is exactly `recovered = secret`.

Finally, Lean has the contrapositive security form:

```text
forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
```

If no keygen breaker exists and `Pr[forgery] >= 2^-133`, then

```text
(Pr[forgery] - 2^-133)^6
  <= N32CompactCoverage.failureUnionBound.
```

The packaged endpoint is:

```text
N32ByteConcreteSignatureSecurityCertificate
```

Given the byte-concrete transcript inputs, a keygen no-break assumption, and
`invTwo`, it yields the final bound directly.

The practical reading of this endpoint is tracked in
`practical_soundness_bottleneck.md`.  The current certificate is still a
sixth-moment statement: for the leading `255 B` row, the `2^-27.326561`
compact coverage bound becomes only about `2^-4.554429` direct residual
forgery advantage after taking the sixth root.  The sampler reports now expose
this as `direct proof bits`; practical security requires a lower-arity or
tighter proof, not just more compact-coverage filtering.

The replacement target theorem is also now in Lean:

```text
LinearStructuralSoundnessTarget
forgeryProb_le_queryMiss_add_structuralFailure_of_no_keygenBreaker
```

It proves the desired linear form:

```text
no keygen breaker
=> Pr[forgery] <= queryMissBound + structuralFailureBound.
```

The extracted object is abstract, so future work can instantiate the target
with full-secret recovery, partial product-block recovery, or any smaller
keygen-breaking witness justified by the concrete keygen game.

The byte-concrete bridge to this target is:

```text
N32ByteLinearStructuralSoundnessInputs
forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
```

It reuses the serialized verifier to prove the canonical query-miss loss, so a
future candidate only has to prove the queried clean-forgery extraction
obligation and bound its structural failure probability.  The resulting bound
has the practical form `Pr[forgery] <= 2^-133 + structuralFailureBound`.

The first obstruction is now formalized in `BlockChallengeSoundness`: a single
affine response `z = mask + chi * secret` is compatible with any candidate
secret if the mask is free
(`productBlockLinearResponse_single_view_secret_ambiguous`).  The matching
positive target is `SingleViewLinearRound.extractedSecret_eq`: if a clean
transcript gives a bound/extractable mask, then one active signed view recovers
the active secret coordinates.  So the concrete algorithmic target is no longer
"more forks"; it is a verifier/query structure that makes the relevant mask or
a smaller keygen-breaking witness extractable for clean queried forgeries
without leaking the key from honest signatures.

That target now has code behind it.  `CompactSingleViewLinearRound` is the
active-only deterministic extractor, and `N32ByteKnownMaskSingleViewInputs`
instantiates the byte-level linear theorem when a clean forgery supplies an
extractable mask and active signed challenge coverage.  Its endpoint is:

```text
N32ByteKnownMaskSingleViewInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
```

which proves the same practical bound
`Pr[forgery] <= 2^-133 + structuralFailureBound`.

The proof target has now pivoted to partial witnesses:

```text
N32PartialKeygenBreakGame
N32BytePartialWitnessLinearInputs
```

This keeps the same linear byte theorem but replaces "recover the full
product-block secret" with "extract any witness that the concrete keygen game
declares to be a break."  The first concrete test type is
`N32RecoveredBlockWitness`, and the multi-block version is
`N32RecoveredBlocksWitness` with a `minBlocks` threshold.  One block is probably
too weak by itself; the stronger candidates are multi-block local preimage
recovery and public-view relation or narrow-dual certificate violations that
contradict the keygen/public-view certificate directly.

The public-view relation candidate is now formalized at the certificate layer:

```text
PublicViewNarrowDualWitness
SystematicLeeViolation
```

`PublicViewNarrowDualWitness` contradicts `NoNarrowQaryDualRelations` and hence
any `PublicViewLeeDualCertificate` or `EfficientPublicViewCertificate`.
`SystematicLeeViolation` is the systematic `A = [I | P]` version: a nonzero
free vector whose relation `(-P y, y)` is below the certified Lee bound.  This
is now the best formal target for a smaller partial-witness proof: make clean
forgeries yield a certificate violation, rather than a full recovered secret.

The byte-level instantiation is also present in
`Descender.SignaturePublicViewWitness`:

```text
N32BytePublicViewNarrowDualInputs
N32ByteSystematicLeeViolationInputs
N32BytePublicViewNarrowDualSecurityCertificate
N32ByteEfficientPublicViewNarrowDualSecurityCertificate
ShortSystematicVerifierRelation
N32SystematicRelationByteParser
N32SystematicRelationByteParser.verifierChecks
N32ExplicitSystematicRelationByteDecoder
N32FixedOffsetSystematicRelationByteLayout
N32ContiguousSystematicRelationByteLayout
N32ContiguousSystematicRelationByteLayout.exact
N32ContiguousPackedSystematicRelationByteLayout
zmod12289Packed14Coeff
zmod12289Packed14RangeCheck
N32Q12289SystematicRelationLayout.target32x32
N32Q12289SystematicRelationLayout.target32x32Decoder
N32Q12289SystematicRelationLayout.target32x32SingleCarrierSignatureBytes
N32Q12289SystematicRelationLayout.Target32x32SingleCarrierVerifierInputs
N32Q12289SystematicRelationLayout.validated48x32
N32Q12289SystematicRelationLayout.validated48x32Decoder
N32Q12289SystematicRelationLayout.validated48x32SingleCarrierSignatureBytes
N32Q12289SystematicRelationLayout.Validated48x32SingleCarrierVerifierInputs
SharedRowProductBlock12289Rows3
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.layout
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.decoder
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.rangeCheck
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.payloadRangeCheck
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.baseAcceptsPayload
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockSingleCarrierVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockCanonicalBaseVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockCanonicalBaseVerifierInputs.toSingleCarrierVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockFSCoreVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockFSCoreVerifierInputs.toCanonicalBaseVerifierInputs
N32Q12289Validated48x32SingleCarrierSecurityCertificate
N32Q12289Validated48x32SingleCarrierSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32SingleCarrierSecurityCertificate.forgerSuccessProb_le_guess_add_structuralFailure
N32Q12289Validated48x32CanonicalBaseSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.toSingleCarrierSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.forgerSuccessProb_le_guess_add_structuralFailure
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.toCanonicalBaseSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.toCanonicalNoFailureSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.forgerSuccessProb_le_guess
N32RelationCheckingByteVerifierInputs
N32SingleCarrierRelationPayloadVerifierInputs
N32ByteExplicitCheckedSystematicVerifierEquationSoundness
N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate
N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
N32ByteCheckedSystematicVerifierEquationSoundness
N32ByteCheckedSystematicVerifierEquationSoundnessInputs
N32ByteCheckedSystematicVerifierEquationSecurityCertificate
N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate
N32ByteSystematicVerifierEquationSoundness
N32ByteSystematicVerifierEquationSoundnessInputs
N32ByteSystematicVerifierEquationSoundnessSecurityCertificate
N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate
N32ByteSystematicVerifierEquationInputs
N32ByteSystematicVerifierEquationSecurityCertificate
N32ByteSystematicPublicViewVerifierEquationSecurityCertificate
N32ByteShortSystematicRelationInputs
N32ByteShortSystematicRelationSecurityCertificate
N32ByteShortSystematicPublicViewRelationSecurityCertificate
N32ByteSystematicLeeViolationSecurityCertificate
N32ByteSystematicPublicViewLeeViolationSecurityCertificate
```

These wrappers replace full-secret extraction with the obligation that a clean
queried forgery extracts one of the certificate-violation witnesses.  The
public-view or systematic Lee certificate then supplies the no-break side and
Lean gives the linear bound
`Pr[forgery] <= 2^-133 + structuralFailureBound`.

For the systematic route the extractor can now target the concrete verifier
equation directly: produce pivot/free coordinates satisfying
`pivot + P * free = 0`, with nonzero free part and Lee norm below the certified
bound.  Lean converts that `ShortSystematicVerifierRelation` into a
`SystematicLeeViolation`.

The newest interface pushes this to response bytes: a
`N32SystematicRelationByteParser` reads pivot/free coordinates from the seven
response byte strings and the challenge.  `N32ByteSystematicVerifierEquationInputs`
then requires only the parser equation, parser nonzero proof, and parser
shortness proof on clean accepted transcripts.
`N32ByteSystematicVerifierEquationSoundness` moves those facts to the actual
serialized-verifier condition `oneShotAccepts = true`, and the wrapper derives
them for forgeries via `oneShotAccepts_of_outputAccepts`.
The checked route is the recommended instantiation: make the verifier perform
`N32SystematicRelationByteParser.verifierChecks`, then prove accepted
non-failing transcripts passed that single Boolean check.

The honest-relation endpoints are:

```text
SignatureDistribution.productBlockVerifierEquation
SignatureDistribution.productBlockVerifierEquation_honest
SignatureDistribution.productBlockActiveVerifierEquation
SignatureDistribution.productBlockActiveVerifierEquation_honest
SignatureDistribution.productBlockReconstructedCommitment
SignatureDistribution.productBlockReconstructedCommitment_honest
BlockChallengeSoundness.HonestVerifierEquationMultiForkRound.toMultiForkLinearRound
BlockChallengeSoundness.CompactHonestVerifierEquationMultiForkRound.toCompactMultiForkLinearRound
BlockChallengeSoundness.CompactHonestVerifierEquationMultiForkRound.extractedSecret_eq
```

The implicit-commitment byte profile is now named too:

```text
N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment
```

It certifies the completed-response accounting:

```text
challenge bytes = 17
response coordinates per repetition = 32
response bytes per repetition = 36
signature bytes = 269
```

The matching concrete verifier prototype is:

```text
tools/parameter-search/src/bin/signature_q12289_implicit_verifier.rs
```

It parses the 269-byte signature as a 17-byte canonical challenge followed by
seven completed 36-byte response chunks, checks signed 9-bit responses against
`|z| <= 224`, reconstructs `t' = A*z - chi*u`, and binds the transcript with
the fixed domain
`Descender.Signature.q12289.implicit_commitment.v1`.

The Lean bridge for this verifier shape is now named:

```text
N32ImplicitCommitmentByteVerifierInputs
N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs
N32ImplicitCommitmentByteVerifierInputs.outputAccepts_implies_outputChallenge_eq_actual
N32Q12289ImplicitCommitmentByteVerifier.Inputs
N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq
N32Q12289ImplicitCommitmentByteVerifier.profile_signatureBytes_eq
N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck
N32Q12289ImplicitCommitmentByteVerifier.implicitCommitmentDomainTag
N32Q12289ImplicitCommitmentByteVerifier.publicKeyRangeCheck
N32Q12289ImplicitCommitmentByteVerifier.responseRangeCheck
N32Q12289ImplicitCommitmentByteVerifier.responseView
N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView
N32Q12289ImplicitCommitmentByteVerifier.productBlockEquationSoundness
N32Q12289ImplicitCommitmentByteVerifier.productBlockEquationSoundnessFromReconstruction
N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder
N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts
N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame
N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length
N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts
N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement
N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts
SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower
N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate.verifierEquation_of_oneShotAccepts
N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_acceptsTranscript
N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts
```

It packages the implicit transcript binding proof: accepted transcripts have
core payload checks, reconstructed commitments equal the output challenge
point, and recomputed challenge bytes agree with the query-log challenge.
The q12289 refinement also fixes the concrete parser/range predicates and the
algebraic bridge.  The concrete `commitmentViewEncoder` now proves the q14
packing fact: decoding the encoded 48-pivot/32-free view recovers the
requested `ZMod 12289` coordinates.  Given this encoder, the
`EncodedReconstructionVerifierCertificate` defines reconstructed bytes as the
encoding of the formal value `A*z - chi*u`, so accepted byte transcripts imply
the product-block verifier equation used by the extractor layer.  The remaining
semantic field is now isolated by `HashFrameEncodedReconstructionVerifierCertificate`:
SHA-256-to-challenge agreement for the exact `q12289HashFrame` byte string.
That frame uses the Rust verifier's length-prefixed field format, fixes the
labels and profile bytes, flattens the seven reconstructed commitments in
repetition order, and proves length `1508 + messageLen` bytes.  The production
certificate is derived by
`ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate`,
so the external assumption is only the SHA/query-log agreement record.

The implementation-facing refinement is
`N32ExplicitSystematicRelationByteDecoder`.  It fixes one response repetition
as the carrier for the systematic relation bytes and supplies `decodePivot` and
`decodeFree` for that byte string.  Lean converts this explicit carried layout
to the generic parser and exposes the same checked security certificates.  The
remaining byte-packing choice is therefore isolated to the concrete decoders;
the proof boundary is already the serialized verifier implication:

```text
oneShotAccepts(state, challenge) = true
not structural failure
---------------------------------
decoder.toParser.verifierChecks response challenge = true
```

Status: the reduction theorem, q12289 implicit-verifier bridge, concrete q14
commitment encoder, and hash-frame certificate wrapper are packaged.  The
remaining deployment obligation is
`ProductionHashFrameOracleAgreement`: SHA-256-to-challenge over
`q12289HashFrame` must agree with the query-log oracle on accepted transcripts.

The byte layout is now fixed at the proof-interface level.  A concrete
implementation can use `N32ContiguousSystematicRelationByteLayout.exact` to
store all pivot coordinates followed by all free coordinates in one carrier
response, with no tags or gaps:

```text
responseLen = (pivotCount + freeCount) * bytesPerCoeff
```

`N32FixedOffsetSystematicRelationByteLayout` handles the lower-level byte
slices, while `N32RelationCheckingByteVerifierInputs` constructs a serialized
verifier whose accept predicate is:

```text
baseAccepts(transcript)
&& decoder.toParser.verifierChecks(P, leeMin, response, challenge)
```

Lean proves the verifier implication directly from that construction:

```text
oneShotAccepts(state, challenge) = true
--------------------------------------
decoder.toParser.verifierChecks response challenge = true
```

For the `q = 12289` profile the concrete coefficient codec is now instantiated
as a packed 14-bit little-endian field element decoder:

```text
zmod12289Packed14Coeff
zmod12289Packed14RangeCheck
```

The range check rejects the unused 14-bit codes above `12288`; it belongs in
`baseAccepts` when canonical signatures are required.  The algebraic decoder
still maps bytes into `ZMod 12289`, so the proof remains about the verifier's
actual field equation.

Two concrete packed layouts are named:

```text
N32Q12289SystematicRelationLayout.target32x32
  responseLen = 112 B
  Pivot = Fin 32, Free = Fin 32

N32Q12289SystematicRelationLayout.validated48x32
  responseLen = 140 B
  Pivot = Fin 48, Free = Fin 32
```

The first matches the original `n=32, m1=32` relation target; the second
matches the current validated `q=12289, n=48, m1=32` coordinate-subview
profile's 80 packed public coordinates.

The response serialization is now single-carrier.  `N32SingleCarrierRelationPayloadVerifierInputs`
models a signature that carries only one relation payload byte string.  The
proof interface still expands it to the existing seven-response verifier by
placing the payload at the decoder carrier and filling the other six
repetitions with zero bytes.  The decoder reads only its carrier, so the proof
does not charge seven copies of the payload.

The named single-carrier sizes are:

```text
N32Q12289SystematicRelationLayout.target32x32SingleCarrierSignatureBytes
  challenge 17 B + payload 112 B = 129 B

N32Q12289SystematicRelationLayout.validated48x32SingleCarrierSignatureBytes
  challenge 17 B + payload 140 B = 157 B
```

The corresponding verifier-input aliases are
`Target32x32SingleCarrierVerifierInputs` and
`Validated48x32SingleCarrierVerifierInputs`; both feed the same checked
soundness and public-view certificate route.

The validated `q = 12289, n = 48, m1 = 32` rows-per-group-3 sidecar is now
wired as the concrete product-block matrix `P`:

```text
SharedRowProductBlock12289Rows3
local block =
  [[10849, 1171],
   [1597,  1615],
   [6102,  11189]]
16 repeated blocks
target Lee lower bound = 512
eta = 12
```

`N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.layout` maps the
16 block-local `3 x 2` product layout into the packed 48-pivot/32-free byte
view, and `Validated48x32ProductBlockSingleCarrierVerifierInputs` fixes the
single 140-byte relation payload against `SharedRowProductBlock12289Rows3.globalMatrix`.
The canonical base predicate is now named too:

```text
Validated48x32ProductBlock.baseAcceptsPayload(core, point, challenge, payload)
  = core(point, challenge, payload)
    && Validated48x32ProductBlock.payloadRangeCheck(payload)
```

`Validated48x32ProductBlockCanonicalBaseVerifierInputs` builds the
single-carrier verifier from that predicate, so canonical 14-bit range checking
is part of the concrete verifier shape rather than a loose side condition.
The implementation-facing Fiat-Shamir route is
`Validated48x32ProductBlockFSCoreVerifierInputs`: it packages a core payload
check, a challenge recomputation function over the byte frame, and the query-log
agreement lemma for accepted payloads.  Lean turns that directly into the
canonical-base verifier inputs.
The final named wrappers are:

```text
N32Q12289Validated48x32SingleCarrierSecurityCertificate
N32Q12289Validated48x32SingleCarrierSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32SingleCarrierSecurityCertificate.forgerSuccessProb_le_guess_add_structuralFailure

N32Q12289Validated48x32CanonicalBaseSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.forgerSuccessProb_le_guess_add_structuralFailure

N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess

N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.forgerSuccessProb_le_guess
```

It proves:

```text
Pr[forgery] <= 2^-133 + structuralFailureBound.
```

The proof is complete at the global reduction boundary, and the q12289 local
bounded-free Lee check is now internalized by Lean:

```text
SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower
```

It is not yet an unconditional deployment proof: it still takes the concrete
implementation obligations through `verifierInputs` and the implicit verifier's
encoded-reconstruction certificate, including transcript/challenge
recomputation, q14 commitment-packer correctness, and the structural-failure
bound.

The local sidecar is retained as an independently rechecked audit artifact via
`verify_shared_lee_cert`:

```text
research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.verify.json
research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.verify.md
```

The verifier recomputes all `523264` centered nonzero local pairs with local
free L1 below `512`, finds independent minimum L1 `533`, and confirms the
artifact fields are consistent.  The implementation-facing benchmark is:

```text
research/cryptanalysis/signature_readiness_20260516/q12289_validated_signature_profile.json
research/cryptanalysis/signature_readiness_20260516/q12289_validated_signature_profile.md
```

It records the current concrete byte target:

```text
public key = 172 B
implicit-commitment signature = 269 B
proof endpoint = Pr[forgery] <= 2^-133
```

That last no-failure endpoint is conditional on keeping structural failures at
zero by making canonical payload range checking and the systematic relation
check explicit verifier checks, which is exactly what the canonical wrapper
does.  The Rust verifier now fixes the byte codec and recompute frame; the
Lean implicit verifier bridge now fixes the serialized transcript implication.
The remaining implementation obligation is now concentrated in
`N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement`:
prove that SHA-256-to-challenge over the exact `q12289HashFrame` agrees with
the query-log oracle on accepted transcripts.  The q14 commitment packer is
internalized as `N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder`,
and the Rust verifier materializes the same hash frame before hashing while
pinning both the q14 packing and zero-public-key hash-frame SHA fixture.
