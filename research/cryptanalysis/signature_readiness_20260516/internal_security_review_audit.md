# Internal Security Review Audit

Target: `Descender.Signature.q12289.local_delta.stateful.v1`

Date: 2026-05-21

Verdict: byte-path blockers closed, but the current public quotient relation is
not cryptographically sound.  The proof-commitment challenge frame, the
representative 254-byte verifier, and the strict linear sigma-equation verifier
are executable and covered by KAT and mutation tests.  The strict sigma equation
blocks the old fixed-commitment representative forgery, but an adapted attacker
can still choose a public proof commitment, derive the challenge, solve
`L(z) = a + c * digest`, and pick a local representative accepted by the current
opening predicate.  Do not treat the 254-byte path as a sound signature scheme
until acceptance is tied to a prechallenge witness-bound sigma commitment or
another hidden-witness proof.

## Closed Findings

1. Fiat-Shamir/QROM frame mismatch.

   The executable `challenge_hash_frame` now binds:

   ```text
   domain, profile, q, groups, local_pivot, local_free, repetitions,
   slot_id, public_key, message, slot_digest, proof_commitment
   ```

   The 32-byte quotient proof commitment is first-message data for the 73-byte
   proof.  It is now part of the measured random-oracle input, so the executable
   frame certificate matches the same-commitment theorem shape.  The executable
   report now exposes this directly:

   ```text
   fiat_shamir_fork_generation_ledger.qrom_theorem_fits_protocol_shape = true
   ```

2. Public proof path was not integrated end-to-end.

   The executable reference API now signs and verifies the full:

   ```text
   slot_id[4] || challenge[21] || active_response[52] ||
   openings[104] || proof_commitment[32] || masked_response[41]
   ```

   `reference_sign_with_proof` serializes the 73-byte proof, and
   `reference_verify_with_proof` verifies the proof-bound challenge frame,
   public q-syndrome digest match, and local opening relation using the
   serialized proof response rather than account-side ideal witness material.
   The strict sigma-equation verifier is also executable, but the current
   representative-style signer does not yet produce accepting sigma
   transcripts.

## Critical Findings

1. Public conditioned-fiber representative forgery.

   The executable report now includes `public_proof_forgery_probe`.  It builds
   an accepting 254-byte signature using only:

   ```text
   public key bytes,
   public account slot digest,
   slot id,
   message
   ```

   It does not use private key material and strips all ideal hidden witness
   material from the account state before verification.  The probe solves the
   public `12 x 23` quotient equations plus a chosen 2-byte fiber state, derives
   the serialized seed from the resulting representative, computes local
   openings, and verifies successfully under the current representative proof
   verifier.  The strict sigma-equation verifier rejects this exact transcript,
   but `public_sigma_equation_forgery_probe` adapts the attack to the strict
   equation by solving the post-challenge quotient equations directly.

   Current executable result:

   ```text
   verifier_accepts = true
   sigma_equation_verifier_accepts = false
   sigma_equation_reason = linear sigma verifier equation mismatch
   public_sigma_equation_forgery_probe.verifier_accepts = true
   signature_bytes = 254
   accepted_reason = ok-stateful-local-delta-public-quotient-proof-relation
   ```

   Required closure: replace the representative-style path with a real
   hidden-witness proof/opening relation where public post-challenge
   representative reconstruction is insufficient for acceptance.

## High Findings

1. Packed-SIS extraction is currently broken at the reference-relation level.

   The review target is now concrete: after a same-commitment fork with
   non-colliding `Fq12` actions, extraction failure is the named
   `PackedFullResponseExtractionGame.extractionBreak`.  The current probes are
   useful smoke tests:

   - `12 x 23` matrix over `q = 12289`
   - rank `12`, kernel dimension `11`, right inverse present
   - bounded sparse search found no zero relations in the tested region
   - LLL found a syndrome-zero vector, but the sampled multiples did not
     preserve the fixed-statement local fiber state or active pad outputs

   The issue is now sharper than lack of BKZ/SVP margin: the current executable
   public relation exposes enough algebraic structure to construct an accepted
   representative in the reference setting.  Required closure is design-level:
   make the proof knowledge-sound for a hidden witness or bind the account state
   in a way that prevents public representative reconstruction.  After that
   redesign, rerun the lattice/SIS campaign against the new relation.

2. Quotient witness hiding depends on strict sampler discipline.

   The zero-TV certificate is valid only for the production masked-response
   sampler that conditions on:

   ```text
   canonical 23-residue q-packing,
   fixed 2-byte local fiber-state equality,
   response bounds/canonicality
   ```

   It explicitly excludes active pad outputs and active local openings.  If a
   production signer conditions the hidden representative on active pad
   equations, the hiding certificate no longer applies.

   Required closure: enforce the sampler predicate in code and test that active
   pad equations are composed outside the hidden representative sampler.

3. Constant-time signing and stateful lifecycle are not production hardened.

   The reference code is review instrumentation.  It branches on
   secret-derived rejection predicates, and the stateful lifecycle is specified
   but not implemented as durable wallet storage.

   Required closure: write a production-shaped signer/sampler and slot manager
   with constant-time review, fixed failure behavior, crash-safe reservation,
   concurrency control, backup/restore warnings, and reorg-safe slot burning.

## Medium Findings

1. Challenge-action domain drift was found and patched.

   The active action is now degree 12, but the domain label still used an
   `fq10` suffix.  The executable domain is now:

   ```text
   Descender.Signature.q12289.local_delta.challenge_action.fq12.v1
   ```

   The stale internal helper names were also renamed to action-neutral names.
   Required closure: refresh affected KATs if any review vector depends on the
   action-hash output and keep domain labels synchronized with Lean constants.

2. SHA-256 assumptions are explicit but still assumptions.

   The packet correctly separates:

   - QROM random-oracle modeling for the Fiat-Shamir challenge frame
   - SHA-256-derived pad stream pseudorandomness
   - SHA-256 rejection sampling for deterministic matrix generation

   Required closure: prepare a reviewer-facing assumption memo with byte
   domains, adversary interfaces, query limits, and the QROM theorem boundary.
   The current theorem family being referenced is the Don-Fehr-Majenz-Schaffner
   Fiat-Shamir-in-the-QROM line and the later Measure-and-Reprogram 2.0
   treatment:

   - https://arxiv.org/abs/1902.07556
   - https://eprint.iacr.org/2020/282.pdf

## What Is Strong

- The byte layout is now pinned and executable: 172-byte public key, 181-byte
  stateful transaction payload, and 73-byte quotient proof target for 254 bytes
  total.
- The quotient sampler has a concrete finite certificate with zero TV distance
  to the intended conditioned fiber law, provided the allowed conditioning
  predicate is respected.
- The action field is now `F_12289[X] / (X^12 + X + 25)`, with executable
  irreducibility checks and action-collision accounting.
- Parser, mutation, and KAT coverage is broad for the full public proof
  reference surface, and now includes a negative security result for that
  relation.
- The top-level Lean theorem is useful as an assumption ledger: it names the
  exact games and byte assumptions consumed by the deployed-verifier theorem.

## Immediate Next Steps

1. Redesign the public proof/keygen relation so public conditioned-fiber
   representative reconstruction is impossible or no longer sufficient for
   acceptance.
2. Re-run packed-SIS/keygen cryptanalysis for the redesigned relation.
3. Freeze and enforce the production sampler predicate and stateful lifecycle
   before outside review.
