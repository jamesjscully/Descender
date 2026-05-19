# Block-Challenge Soundness

This note records the extraction target for intermediate product-block
signature rows.

## Formal Boundary

`Descender/BlockChallengeSoundness.lean` adds the deterministic block-coverage
side of the signature proof:

- `CoversBlocks`: every block is active in at least one repetition;
- `CoversBlocksOn`: every needed block is active in at least one repetition;
- `MissesNeededBlock`: the complementary failure event;
- `not_missesNeededBlock_of_coversBlocksOn`: coverage rules out the failure;
- `LocalBlockExtractor`: an abstract local extractor for an active block;
- `globalWitness_valid`: if the repeated challenges cover all blocks, local
  extraction assembles a globally valid block witness.

The same file also proves the product-block response algebra used by a future
forking/extraction proof:

```text
(y + chi0 * s) - (y + chi1 * s) = (chi0 - chi1) * s.
```

The theorem is `productBlockLinearResponse_sub`.  If the block-local challenge
delta has a left inverse, `extract_coordinate_of_deltaExtractable` recovers the
secret coordinate.

Lean now also records the one-view obstruction:

- `maskExplainingResponse`;
- `productBlockLinearResponse_maskExplainingResponse`;
- `productBlockLinearResponse_single_view_secret_ambiguous`.

These prove that any single affine response can be explained by any candidate
secret if the mask is allowed to move.  Therefore a linear one-view proof
cannot come from the equation `z = mask + chi * s` alone; it needs an additional
binding/extraction mechanism for the mask, or a different keygen-breaking
witness.

The positive one-view target is also formalized:

- `ScalarExtractable`: a single challenge scalar is invertible on a block;
- `SingleViewLinearRound`: one accepting response with a known mask;
- `SingleViewLinearRound.extractCoordinate_eq_secret`;
- `SingleViewLinearRound.extractedSecret_eq`.
- `CompactSingleViewLinearRound`: active-only version for compact responses;
- `CompactSingleViewLinearRound.extractedSecret_eq`.

This says that if a redesigned clean transcript exposes the relevant mask, then
ordinary active challenge coverage is enough to recover the secret.  That is
the deterministic shape a linear proof would need underneath the byte-level
north-star theorem.

The file now also packages the forked linear extraction boundary:

- `ForkedLinearRound`: two accepting responses for the same mask and two block
  challenges;
- `DeltaActive`: the block-local challenge delta has a left inverse;
- `extractCoordinate_eq_secret`: a delta-active fork extracts the coordinate;
- `extractedSecret_eq`: if forked deltas cover all blocks, the extracted
  product-block secret equals the real secret;
- `OppositeSignedActive` and `deltaExtractable_of_oppositeSignedActive`: for
  compact active-only transcripts, opposite signs are enough when `2` is
  invertible.

The multi-fork deterministic layer is also formalized:

- `MultiForkLinearRound`: several accepting challenge/response draws for one
  fixed commitment/mask;
- `MultiDeltaActive`: a block has two draws with an extractable challenge
  delta;
- `CompactOppositeActive`: a block has opposite signed active openings among
  the draws;
- `coversMultiDelta_of_coversCompactOpposite`: compact opposite-sign coverage
  implies delta coverage when `2` is invertible;
- `extractedSecret_eq`: multi-fork delta coverage across repetitions recovers
  the whole secret;
- `extractedSecret_eq_of_coversCompactOpposite`: compact active-only multi-fork
  coverage recovers the whole secret.

For the concrete modulus used by the current reports, Lean records
`invTwo_zmod12289`, namely `(6145 : ZMod 12289) * 2 = 1`.

`Descender/SignatureForking.lean` builds on this deterministic layer with the
finite-probability accounting for Fiat-Shamir multi-forks.  It proves

```text
Pr[extractable] >= Pr[accepting same-commitment forks]
                  - Pr[compact fork-delta coverage failure].
```

The certificate wrapper then turns an external forking loss into the final
bound:

```text
Pr[extractable] >= forgerSuccess - forkingLoss - coverageFailureBound.
```

That shell has now been refined into random-oracle replay events:

```text
Pr[acceptingForks] >=
  Pr[forgery] - Pr[queryMiss] - Pr[replayFailure].
```

So `forkingLoss` is no longer a black box.  It is the sum of a challenge-point
query miss bound and a replay-schedule failure bound.  For the six-draw
candidate, the replay shell is further specialized to `Draw = Fin 6`: query
miss is bounded by one challenge-guess event, while replay failure is bounded
by the sum of six per-draw failure events.

## Exact Coverage Accounting

`signature_sampler_analysis.rs` now computes exact any-block miss probabilities
by inclusion-exclusion.  The old active-coverage diagnostic asks only whether a
single challenge touches each block.  For `B` blocks, challenge weight `w`, and
`R` repetitions, that failure probability is:

```text
Pr[any missed] =
  sum_{j=1..B} (-1)^(j+1) * binom(B,j) *
    (binom(B-j,w) / binom(B,w))^R.
```

This is useful, but it is not the extraction event for compact signatures.
Forked extraction needs a usable challenge delta.  The report therefore also
computes:

- full-response fork-delta coverage, where zero-vs-nonzero forks can count
  because all response coordinates are available;
- compact fork-delta coverage, where only opposite signed active openings
  count because inactive response coordinates are omitted from the small
  signature.

The compact fork-delta probability is exact.  For each candidate missed block
set, the dynamic program tracks how many fixed blocks are still unseen,
plus-only, and minus-only across the challenge draws for one commitment; the
outer inclusion-exclusion then charges the probability that any block has no
opposite-sign fork in any repetition.

## Current Rows

For the `n = 32`, 16-block profile:

- `197 B`: weight 13, 6 repetitions, compact fork-delta cost `17.08` bits;
- `255 B`: weight 15, 7 repetitions, compact fork-delta cost `27.33` bits;
- `304 B`: full-block response, compact fork-delta cost `36.00` bits.

For the `n = 48`, 16-block profile:

- `281 B`: weight 13, 6 repetitions, compact fork-delta cost `17.08` bits;
- `415 B`: weight 13, 8 repetitions, compact fork-delta cost `24.11` bits;
- `496 B`: full-block response, compact fork-delta cost `36.00` bits.

These rows assume six accepting challenge draws for the same commitment per
repeated round.  A single ordinary fork is much weaker: the `n = 32`, `255 B`
row has only `2.02` bits of compact one-fork delta coverage.

## Consequence

The best small-signature target is now explicit.  The deterministic multi-fork
extractor is formalized: six accepting draws are enough whenever they give
compact opposite-sign coverage.  The finite-probability shell around that
extractor is also formalized.  The remaining question is the concrete
random-oracle bound for `queryMiss` and `replayFailure`: prove that the
Fiat-Shamir/forking game can obtain six accepting same-commitment draws with a
tolerable replay loss.  The current six-draw certificate reduces this to
`2^-133` blind challenge guessing for the `n = 32`, `255 B` challenge space,
plus six transcript-specific replay-draw bounds.  Under that game, the
`n = 32` row at about `255 B` is the leading reusable-signature candidate above
the `24`-bit compact fork-delta line.  If we want a cleaner `32`-bit compact
fork-delta line without relying on sparse intermediate coverage, the
conservative `n = 32` target is about `304 B`.
