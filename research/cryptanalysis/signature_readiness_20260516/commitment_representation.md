# Commitment Representation Notes

Date: 2026-05-19

## Conclusion

The leading commitment representation is Schnorr-style implicit commitment
reconstruction:

```text
t' = A * z - chi * u
```

The signature carries the challenge and enough response coordinates for the
verifier to reconstruct the full Fiat-Shamir commitment.  It does not carry
`t` as a separate serialized object.

For the compact `n=32`, weight-1, 26-repetition target, the same idea gives
the first signable compact replacement for explicit commitments:

```text
challenge          = 17 B
completed response = 26 * ceil(16 * 2 * 7 / 8) = 728 B
total              = 745 B
```

The verifier carries all response coordinates, reconstructs each block
commitment as `A * z - chi * u` for the one active signed block and as `A * z`
for the inactive blocks, and hashes the full reconstructed commitment view.
This removes the `3640 B` explicit commitment payload without publishing the
mask/free commitment coordinates directly.

For the current q=12289 rows3, 16-block, 2-free profile, the best immediate
variant is the weight-15 challenge with full free-response completion:

```text
challenge = 17 B
response  = 7 * ceil(16 * 2 * 9 / 8) = 7 * 36 B
total     = 269 B
```

This is only 14 B larger than the active-only 255 B row and avoids roughly
980 B of explicit per-repetition commitment data.

## Why Explicit Commitments Lose

The local commitment block has 3 pivot and 2 free q-coordinates:

```text
5 * 14 = 70 bits ~= 9 B per block
16 blocks = 140 B per repetition
7 repetitions = 980 B
```

Sending all commitments gives about:

```text
255 B active response/challenge + 980 B commitments = 1235 B
```

Sending active commitment blocks is not much better because the weight-15
challenge opens 15 of 16 blocks.  Merkle/vector commitments only help if the
active commitment leaves are derived from `(z, chi, u)` rather than serialized.
Even then, the omitted-leaf proof/hash is larger than just sending the omitted
free response coordinates.

## Best Variants

1. **Completed response, no commitment bytes.**
   Send all free response coordinates for each repetition, including the one
   omitted by the challenge.  The verifier reconstructs the whole commitment
   and checks the Fiat-Shamir challenge.  Current estimate: 269 B.

2. **Full-block challenge, no commitment bytes.**
   Use weight-16 challenges and send full responses.  This has simpler
   coverage and no inactive block, but needs 8 repetitions for 128 bits.
   Current estimate: 304 B.

3. **Active response plus omitted commitment data.**
   Reconstruct active commitment blocks from `(z, chi, u)` and serialize only
   omitted data.  This is larger than completed response if omitted data is a
   full 5-coordinate commitment block, and only competitive if the omitted
   free coordinates suffice.

4. **Merkle/rooted active openings.**
   Useful as a binding abstraction, but not byte-optimal for 16 blocks and
   weight 15.  A root or omitted-leaf hash costs much more than the 2 omitted
   free response coordinates.

## Security Constraints

A public seed for the mask is not viable.  If the verifier can recover the
mask `y`, then every active response leaks

```text
s = chi^{-1} * (z - y).
```

Likewise, a systematic public commitment map that exposes the free mask
coordinates is not a cryptographic hiding commitment.  The proof can use the
systematic view as an algebraic model, but a deployable signature needs the
published public map/commitment representation to avoid revealing the signing
mask or the secret.  The implicit reconstruction route is compatible with a
Schnorr-style public map `u = A*s` where `A` is one-way or syndrome-like.

The information-set route must obey the same boundary.  A transparent
low-dimensional mask code is not enough: if the active projection recovers the
mask/codeword, it leaks the active secret.  The viable stateless variant would
need a fibered commitment representation where the active projection determines
the Fiat-Shamir commitment state, but leaves many possible signing masks in the
fiber.  The abstract bound and this distinction are recorded in
`information_set_commitment_frontier.md` and
`Descender.InformationSetCommitment`.

## Next Proof Tasks

The first four proof tasks, the serialized verifier bridge, and the
accepted-transcript product-block bridge are now implemented or prototyped.
The remaining work is the concrete implementation-coherence proof, not another
global reduction theorem.

Implemented endpoints:

```text
SignatureDistribution.productBlockReconstructedCommitment
SignatureDistribution.productBlockReconstructedCommitment_eq_of_verifierEquation
SignatureDistribution.productBlockVerifierEquation_of_reconstructedCommitment_eq
SignatureDistribution.productBlockReconstructedCommitment_honest
N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment
```

The remaining task is the final completed-response verifier-to-extractor
wrapper.

1. **Done.** Define the implicit commitment function in Lean:

```text
reconstructedCommitment(P, chi, u, z) = A*z - chi*u
```

2. **Done.** Prove honest reconstruction:

```text
z = y + chi*s
u = A*s
--------------------------------
reconstructedCommitment(P, chi, u, z) = A*y
```

3. **Done.** Add a completed-response byte profile:

```text
weight 15, reps 7, local_free 2, z_bits 9, signature 269 B
```

4. **Done.** Build the byte verifier:

```text
parse challenge
parse completed response z
range-check z
reconstruct t'
check challenge == H(domain, pk, message, t')
```

Concrete artifact:

```text
tools/parameter-search/src/bin/signature_q12289_implicit_verifier.rs
```

The byte contract is fixed as:

```text
challenge: 7 * 19 bits = 17 B, with top 3 bits zero
response: 7 * 36 B, 32 signed 9-bit two's-complement free coordinates/round
response bound: |z| <= 224
public key payload: 80 q-coordinates at 14 bits each, pivot coordinates first
hash domain: Descender.Signature.q12289.implicit_commitment.v1
hash input: domain/profile/constants, pk, message, reconstructed t' bytes
```

5. Bridge accepted implicit-verifier transcripts into the existing
`HonestVerifierEquationMultiForkRound` / extractor proof.

Current Lean bridge endpoints:

```text
N32ImplicitCommitmentByteVerifierInputs
N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs
N32ImplicitCommitmentByteVerifierInputs.outputAccepts_implies_outputChallenge_eq_actual
N32Q12289ImplicitCommitmentByteVerifier.Inputs
N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq
N32Q12289ImplicitCommitmentByteVerifier.profile_signatureBytes_eq
N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck
N32Q12289ImplicitCommitmentByteVerifier.implicitCommitmentDomainTag
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
N32ImplicitProductBlockVerifierEquationSoundness.reconstructedCommitment_eq_of_acceptsTranscript
N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_acceptsTranscript
N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts
```

This discharges the serialized transcript binding and algebraic bridge parts
of task 5.  The q12289 certificate fixes the domain tag and
`corePayloadCheck` to the concrete Lean parser/range predicate, and the
production q14 commitment packer is now internalized as
`commitmentViewEncoder`.  The hash frame is now internalized as
`q12289HashFrame`, including length-prefixes, labels, profile/domain bytes, and
the seven reconstructed commitments in repetition order; Lean proves the frame
length is `1508 + messageLen`.  The production verifier certificate is now
derived from `ProductionHashFrameOracleAgreement`, so the remaining external
field is exact: prove that SHA-256-to-challenge over this frame agrees with the
query-log oracle on accepted transcripts.  The Rust verifier pins those
boundaries with conformance fixtures for the hash frame and
reconstructed-commitment packing.
