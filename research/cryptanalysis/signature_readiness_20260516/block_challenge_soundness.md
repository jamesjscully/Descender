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

## Exact Coverage Accounting

`signature_sampler_analysis.rs` now computes the exact probability that any
block is missed after repeated sparse challenges.  For `B` blocks, challenge
weight `w`, and `R` repetitions, the failure probability is computed by
inclusion-exclusion:

```text
Pr[any missed] =
  sum_{j=1..B} (-1)^(j+1) * binom(B,j) *
    (binom(B-j,w) / binom(B,w))^R.
```

This is the right event for block extraction.  It is sharper than treating a
fixed missing block as the soundness event, but it still charges the real
failure condition.

## Current Rows

For the `n = 32`, 16-block profile:

- `243 B`: weight 14, 7 repetitions, exact any-block miss cost `17` bits;
- `255 B`: weight 15, 7 repetitions, exact any-block miss cost `24` bits;
- `304 B`: full-block challenge, no block-coverage failure.

For the `n = 48`, 16-block profile:

- `390 B`: weight 14, 7 repetitions, exact any-block miss cost `17` bits;
- `416 B`: weight 15, 7 repetitions, exact any-block miss cost `24` bits;
- `496 B`: full-block challenge, no block-coverage failure.

## Consequence

The best small-signature target is now explicit.  If the Fiat-Shamir extraction
argument can tolerate an additional exact block-coverage failure term around
`2^-24`, the `n = 32` row at about `255 B` is the leading reusable-signature
candidate.  If the proof must eliminate block-coverage failure entirely, the
conservative `n = 32` target is about `304 B`.
