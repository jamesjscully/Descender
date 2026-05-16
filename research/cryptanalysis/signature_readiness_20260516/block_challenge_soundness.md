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

These rows assume six challenge draws for the same commitment per repeated
round.  A single ordinary fork is much weaker: the `n = 32`, `255 B` row has
only `2.02` bits of compact one-fork delta coverage.

## Consequence

The best small-signature target is now explicit.  If the Fiat-Shamir extraction
argument can justify six challenge draws per repeated commitment, the `n = 32`
row at about `255 B` is the leading reusable-signature candidate above the
`24`-bit compact fork-delta line.  If we want a cleaner `32`-bit compact
fork-delta line without relying on sparse intermediate coverage, the
conservative `n = 32` target is about `304 B`.
