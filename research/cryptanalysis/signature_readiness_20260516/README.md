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
not yet a safe signature primitive.  Before implementation, the next
signature-specific work should define a transcript relation that is either
zero-knowledge by construction or has a formal response-distribution
independence argument.
