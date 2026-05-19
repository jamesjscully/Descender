# Target Theorem

Date: 2026-05-19

This is the deployable theorem statement the project is aiming to justify for
the frozen q12289 implicit-commitment signature target.  It is written as a
target claim, not as a statement that every premise is already closed.

## Frozen Verifier

Let `V_q12289` be the verifier that:

- parses the `172 B` public key and `269 B` signature from `target_spec.md`;
- decodes the `17 B` challenge with
  `N32CanonicalChallengeByteCodec.productionBitLayout17`;
- decodes seven `36 B` response chunks with signed 9-bit coordinates bounded
  by `224`;
- reconstructs commitments with the q12289 implicit-commitment relation;
- hashes exactly `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`;
- accepts only if the claimed challenge equals the hash-derived canonical
  challenge and all parser/range checks pass.

## ROM EUF-CMA Target

For every adversary `A` in the random-oracle model for the exact hash frame
above, the probability that `A` outputs a fresh accepted forgery for
`V_q12289` is bounded by

```text
epsilon_keygen
+ epsilon_signer
+ epsilon_conformance
+ epsilon_cryptanalysis
+ epsilon_side_channel
+ epsilon_query_model
+ 2^-133
```

where each epsilon is zero only after the corresponding review obligation is
closed.

## Premise Map

- `epsilon_keygen`: key generation never emits a public view with the relevant
  bad narrow witness, or its failure probability is explicitly bounded.
- `epsilon_signer`: the concrete signer instantiates
  `RejectionSamplingCertificate`, including observable retry behavior.
- `epsilon_conformance`: production code implements the Lean byte predicates
  exactly and passes the shared conformance vector suite.
- `epsilon_cryptanalysis`: no stronger attack is known against the frozen
  q12289 target after the archived attack campaign.
- `epsilon_side_channel`: deployment does not leak keygen/signing secrets
  through timing, memory access, randomness failure, panics, logs, or retries,
  except where explicitly scoped out.
- `epsilon_query_model`: the production EUF-CMA game maps to the query-log
  Fiat-Shamir game used by the proof.
- `2^-133`: the Lean-proved blind challenge-guess term for
  `N32CanonicalChallenge`.

## Current Lean Core

The current formal core proves the `2^-133` no-structural-failure route under
the declared verifier certificate and hash-frame oracle agreement.  In theorem
map terms, the core path is:

```text
accepted serialized transcript
  -> concrete parser/range/hash-frame checks
  -> q12289 product-block verifier equation
  -> internal Lee certificate / no structural failure
  -> blind challenge-guess bound 2^-133
```

The theorem is complete only when every nonzero epsilon above is either proven
zero, bounded concretely, or stated as an explicit assumption in the final
claim.

## Current Deployment Status

The frozen challenge byte codec is closed for the current format.  The largest
remaining blockers are keygen/no-bad-public-view, concrete signer distribution,
independent conformance/CI, frozen-target cryptanalysis, and implementation
hardening.
