# Cryptanalysis Plan Before Third-Party Review

This plan is adversarial.  Its purpose is to break or de-risk the frozen
q12289 target before external review.

## Keygen And Public-View Attacks

Primary question: can a generated public view admit the relevant short/narrow
witness despite the proof assumptions?

Required work:

- Re-run bounded dual/witness searches against the frozen q12289 structure.
- Search for bad vectors using lattice reduction and meet-in-the-middle
  variants.
- Try attacks that exploit the `3 x 2` repeated local block structure.
- Test whether public-key seed expansion or view construction introduces
  correlations not captured by the proof model.
- Produce a failed-attack report with scripts, parameters, and outputs.

Exit criterion:

- No found witness below the claimed threshold.
- Attack cost estimates support the target security margin.
- Any heuristic estimator caveats are stated plainly.

## Signature Transcript Attacks

Required work:

- Attempt direct key recovery from many valid signatures.
- Attempt transcript malleability: modify response/challenge/hash frame while
  preserving acceptance.
- Attempt omitted-block bias exploitation.
- Attempt repeated-message and repeated-randomness attacks.
- Attempt noncanonical encoding attacks on challenge, public key, and response.
- Attempt hash-domain separation collisions across profile/domain variants.

Exit criterion:

- All attacks fail or produce documented required mitigations.
- Negative vectors cover every parser-related attack that should be rejected.

## Fiat-Shamir And Forking Surface

Required work:

- Review whether `ProductionHashFrameOracleAgreement` is exactly the random
  oracle assumption needed for the global theorem.
- Confirm the hash frame includes every value the verifier semantically binds.
- Confirm the challenge space size and canonicalization match the proof-side
  `2^133` bound.
- Confirm no signing step can bias challenges or responses outside the modeled
  distribution.

Exit criterion:

- The security claim states the exact random-oracle-model theorem and all
  reduction losses.

## Concrete Security Estimate

Required work:

- Produce a single table with attack class, best known cost, memory, data, and
  confidence.
- Include lattice reduction configuration and reproducibility notes.
- Include "not modeled" items, especially side channels and implementation
  bugs.

Exit criterion:

- The review package can justify a concrete security level without relying on
  parameter-search optimism.
