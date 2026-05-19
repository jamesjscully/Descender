# Theorem Map

This map records what the Lean proof currently establishes for the frozen
q12289 target.  It is the starting point for a third-party proof review.

## Byte Profile And Verifier Boundary

- `N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment`
  records the `269 B` implicit-commitment signature shape.
- `N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq`
  pins the q12289 serialized signature byte count.
- `N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs`
  turns the implicit verifier interface into the generic serialized verifier
  boundary.

Review question: does the generic serialized verifier boundary express the
right Fiat-Shamir transcript semantics for the deployable verifier?

## Concrete Parsing And Reconstruction

- `N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck`
  combines public-key range checks, commitment range checks, and signed
  response range checks.
- `N32Q12289ImplicitCommitmentByteVerifier.responseView`
  interprets the signed 9-bit response bytes as field elements.
- `N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView`
  defines the algebraic reconstructed view `A*z - chi*u`.

Review question: are the parser rejection rules exactly the rules in the
target spec?

## q14 Commitment Encoder

- `N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder`
  is the concrete byte encoder for reconstructed commitment views.
- `N32Q12289ImplicitCommitmentByteVerifier.commitmentView_encodeCommitmentView`
  proves `commitmentView (encodeCommitmentView view) = view`.
- `N32Q12289ImplicitCommitmentByteVerifier.zmod12289Packed14Coeff_encodeCommitmentView`
  proves each decoded 14-bit coefficient recovers the source `ZMod 12289`
  coordinate.

Review question: does the q14 packing order match the implementation and
the public-key codec?

## Canonical Challenge Codec

- `N32CanonicalChallengeByteCodec.productionBitLayout17`
  is the computable Lean endpoint for the frozen production challenge layout:
  per repetition, four omitted-block bits followed by fifteen sign bits, with
  three zero slack bits in the final byte.
- `N32CanonicalChallengeByteCodec.productionDecode_encode`
  proves the production decoder accepts the production encoder and recovers the
  original `N32CanonicalChallenge`.
- `N32CanonicalChallengeByteCodec.productionEncode_injective`
  proves no two canonical challenges share the same production 17-byte
  encoding.

Review question: does the production implementation use this exact little-endian
bit order and reject the same noncanonical slack-bit encodings?

## Hash Frame And Fiat-Shamir Boundary

- `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`
  defines the exact framed byte string sent to SHA-256-to-challenge.
- `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length`
  proves frame length `1508 + messageLen`.
- `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_fixtureMessage_length`
  proves the pinned test message frame length is `1540`.
- `N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate`
  connects the concrete hash frame to encoded reconstruction.
- `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement`
  is the only remaining random-oracle/SHA agreement assumption at this layer.

Review question: is this the right and only Fiat-Shamir hash assumption for the
scheme claim?

## Algebraic Verifier Equation

- `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts`
  says that an accepted one-shot verifier transcript implies the q12289
  product-block verifier equation, assuming the production hash-frame oracle
  agreement.
- `N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts`
  is the generic bridge used by the q12289 specialization.

Review question: does this verifier equation imply the extraction target used
by the global security certificate?

## Local Lee Certificate And Global Bound

- `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate`
  internalizes the local `3 x 2` Lee certificate.
- `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower`
  proves the relevant local lower bound.
- `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate`
  packages the no-structural-failure q12289 security certificate.
- Current proof-side endpoint: `Pr[forgery] <= 2^-133` under the declared
  no-failure and oracle-agreement boundaries.

Review question: are the structural/keygen assumptions strong enough for the
claimed deployment security level?
