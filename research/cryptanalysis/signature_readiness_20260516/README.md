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

The current committed-sigma/account-carried route has a concrete 250-byte
transaction target for the `n = 32`, `q = 12289` local-delta matrix.  The
response-link bottleneck is now reduced to a bounded joint-kernel certificate
for the two-coordinate account code `[0, 3]`.  The repo checker
`scripts/research/response_link_bounded_kernel_check.py --solver lattice-enum`
enumerates the full intersection lattice inside the Euclidean cover ball for
the bound-96 response-difference box and finds no nonzero bounded vector.  The
checker also has `--solver exact-enum`, which replays the same enumeration with
rational Gram-Schmidt pruning and exact integer leaf-norm checks.  The
certificate artifacts are
`response_link_bounded_kernel_enum_n32_q12289_coords_0_3_b96.json` and
`response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json`.
Both artifacts replay with `--verify-json` and no mismatched certificate fields.
Their canonical-basis hash is
`c6a0cd8f470d1ec80886be3352f6144a90642826f0962ad2d077e727217fcf69`; their
reduced-basis hash is
`1348db737dfb311a4d53b3e5969774b82515a7e7f5b76689e39ee173ee9a44f0`; their
cover-ball vector hash is
`e269022d387b191cc3d926a32a746e8e87b9e89e720dd04fb1dc91f6be4a0c95`.  This is
strong internal computational evidence.  The exact replay now enumerates from
the embedded reduced basis after verifying that it is the same integer lattice
as the canonical intersection basis: rows satisfy the modular relation, rows
are integer combinations of the canonical basis, and the absolute determinant
matches
`1791561079655532333095965150039192629866166308243408855041`.  Before external
review this should still be independently reproduced or packaged as a small
checkable enumeration certificate.

The first small checkable verifier is now available:

```text
scripts/research/verify_response_link_bounded_kernel_certificate.py
```

It uses only Python's standard library, reads the JSON certificate, validates
the embedded basis against the canonical intersection lattice, and replays the
exact bounded enumeration.  It verifies both current `[0, 3]` artifacts with no
mismatches.

The manifest-level audit entry point is:

```text
scripts/research/audit_response_link_bounded_kernel_manifest.py
```

By default it performs a fast static audit of the manifest payload hash,
certificate hash, verifier hash, statement fields, basis hashes, determinant
fields, and recorded enumeration counts.  With `--replay` it invokes the
standalone verifier and checks for `status = verified` with no mismatches.

There is also a local lattice attack-review harness:

```text
scripts/research/response_link_lattice_attack_review.py
```

It rebuilds the same intersection lattice, validates the certificate hashes and
determinant, reports determinant/cover-radius/Gaussian-heuristic metrics, and
runs local `fpylll` LLL/BKZ reductions when available.  The current generated
artifact is:

```text
response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json
```

It finds no vector inside the accepted response-difference box under LLL or BKZ
block sizes `10,15,20,23`; the shortest reduced row has coefficient bound `139`,
which is `43` above the allowed bound `96`.  This is attack-review evidence,
not a replacement for the exact enumeration proof object.

The broader quotient/SIS lattice review harness is:

```text
scripts/research/quotient_sis_lattice_attack_review.py
```

It rebuilds the full `12 x 23` quotient q-kernel lattice and runs LLL/BKZ plus
the bounded signed sparse relation probe.  The current artifact is:

```text
quotient_sis_lattice_attack_review_n32_q12289.json
```

It finds a full-kernel syndrome-zero vector with coefficient bound `68`, inside
the response-difference bound `96`.  This is the expected reason the bare
quotient is not sound by itself.  The vector has nonzero coordinates at `[0,3]`,
so the selected account response-link code catches it; the response-link
certificate then proves no bounded vector survives after those coordinates are
fixed to zero.

The exact certificate also has a standalone audit manifest:

```text
response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json
```

It was emitted by the stdlib verifier with `--write-manifest` and binds the
certificate hash, verifier script hash, canonical statement, replay command,
basis validation, exact search counts, shortest cover-ball vector, and
cover-ball transcript hash.  Current manifest payload hash:

```text
da10b823ad8f4142dfeccdbf7bbcddbb5e7148f420d8ff36687bae412d09fbf0
```

The reviewed account-state layout for this route is now pinned in Lean as
`ConcreteReviewedAccountCommitmentRepresentation`: 32 bytes for the witness
commitment, 32 bytes for the randomizer commitment, and 4 bytes for the `[0, 3]`
response-link code, for 68 account/state bytes total.  These bytes are not
added to the spend signature; the corresponding Lean theorem keeps the
account-carried link at zero spend-signature bytes, preserves the 250-byte
transaction target under the reviewed no-break hypothesis, and proves the
combined online-plus-account accounting is `318 B`.  The same wrapper now
directly exposes the selected `AccountLinkProofSystem`, its `anyBreak` surface,
and structural admissibility under `¬ anyBreak`.

The first committed-sigma/account-carried cost-model artifact is:

```text
research/parameter_search/committed_sigma_account_carried_20260521/committed_sigma_account_carried_cost_model.json
```

Its Markdown companion is:

```text
research/parameter_search/committed_sigma_account_carried_20260521/committed_sigma_account_carried_cost_model.md
```

It ranks verifier architectures after applying the proof gates and now includes
external byte-size comparator rows with source links.  The current Pareto row is
`reviewed-account-carried-bounded-kernel-37b`: `250 B` spend transaction,
`68 B` account/state bytes, and `318 B` if one simply adds the online
transaction and account-link state.  The `252 B` fiber-state-preserving row
remains an admissible robustness fallback; the old `254 B` public sigma-equation
row is explicitly rejected by the structural gates.  The report now also
includes security-review gates that link the response-link exact certificate,
response-link LLL/BKZ review, quotient/SIS bare-quotient control, and the
reference timing smoke benchmark, while keeping external replay and
production-hardening items open.

The sidecar quotient-level account-link research program is closed with
`no_replacement`: implementation should keep the reviewed split
account-carried bounded-kernel design.  The promotion gate is:

```text
research/cryptanalysis/signature_readiness_20260516/quotient_level_link_promotion_gate.md
```

The implementation-facing handoff is:

```text
research/cryptanalysis/signature_readiness_20260516/quotient_level_link_implementation_handoff.md
```

The import firebreak audit for the sidecar Lean modules is:

```text
research/cryptanalysis/signature_readiness_20260516/quotient_level_link_firebreak_audit.md
```

To verify the closed quotient-link bundle before implementation work, run:

```text
python3 scripts/research/verify_quotient_level_link_closure.py
```

The broader review-gate replay dry run also includes this closure check:

```text
python3 scripts/research/review_gate_replay.py --dry-run --skip-timing --json /tmp/quotient_review_gate_dry_run.json --md /tmp/quotient_review_gate_dry_run.md
```

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

For the conditioned shifted discrete-Gaussian route, the same Lean file now
adds `RejectionSampler.LeakageBoundedCertificate`.  The product-block theorems
`productBlock_accepted_response_leakage_bounded` and
`productBlock_encoded_response_leakage_bounded` prove that once a concrete row
supplies a finite leakage budget, every accepted response test and every
deterministic byte-level encoding are bounded by that same budget.

## Concrete Sampler Analysis

`tools/parameter-search/src/bin/signature_sampler_analysis.rs` analyzes an
exact finite sampler: for each active coordinate with shift `|v| <= eta`, it
samples `y` uniformly from `[-z-eta,z+eta]`, emits `y + v`, and accepts exactly
when the response lies in `[-z,z]`.  This exactly realizes a
secret-independent uniform target law on `[-z,z]`.

The same report now screens leakage-bounded alternatives.  It computes exact
finite total-variation and KL diagnostics for shifted bounded-interval
responses and conditioned shifted discrete-Gaussian responses, then multiplies
the KL budget over the serialized response coordinates.  The shifted interval
fast path is useful for implementation smoke tests but is not a security proof
candidate.  Wide conditioned discrete-Gaussian rows give the first concrete
bounded-leakage candidates, subject to an explicit product KL/TV theorem and
retry-timing discipline.

The conditioned Gaussian rows are also emitted as checkable certificate
artifacts:

- `conditioned_gaussian_leakage_certificate_n32.json`
- `conditioned_gaussian_leakage_certificate_n48.json`

They can be verified by rerunning
`signature_sampler_analysis --verify-certificate <path>`.  The verifier parses
the certificate, reconstructs the analysis configuration, recomputes the finite
shifted-Gaussian TV/KL table metrics, and then checks the rational proof rows:
each proof row carries a 20-bit dyadic integer weight table for the actual
quantized sampler distribution.  The verifier rejects any table that does not
match the canonical dyadic Gaussian table for the row parameters, then
recomputes the total-variation leakage bound exactly as a rational interval.
This is the proof object intended to instantiate the Lean leakage budget.

The verifier also emits Lean modules from already-checked certificates:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin signature_sampler_analysis -- \
  --verify-certificate research/cryptanalysis/signature_readiness_20260516/conditioned_gaussian_leakage_certificate_n32.json \
  --output-lean-certificate Descender/Generated/ConditionedGaussianLeakageN32.lean
```

The generated modules import `Descender.SignatureLeakageCertificate`, record
the source certificate SHA-256, and expose each checked rational signature
leakage bound as a Lean `ℝ` constant with nonnegativity and `≤ 1` theorems.
They are imported by `Descender.lean`.

There is also a proof object for the executable full-response reference
verifier profile:

- `current_verifier_conditioned_gaussian_leakage_certificate_n32_w15_r7.json`

This row uses the current verifier's active challenge shape (`n=32` free
coordinates, block weight `15`, `7` repetitions, `210` active response
coordinates), with `z=224`, `eta=12`, and `sigma=896`.  The current verifier
still serializes inactive coordinates too, but those coordinates have
challenge coefficient zero and do not carry secret-shift leakage.  The
reference signer mode `--reference-certified-dyadic` samples masks from that
row's canonical dyadic integer table and aborts unless active responses land
in the certified box.  This is distinct from the smaller compact
`block_weight=1, repetitions=26` candidate; it is the sampler bridge for the
executable verifier format that exists today.

The fast compact-signature target is tracked in
`compact_signature_todos.md`.  Its first implementation artifact is
`tools/parameter-search/src/bin/signature_q12289_compact_format.rs`, which pins
the 69-byte compact active payload and reconstructs active local commitments.
Its default verifier now checks the signable completed-response implicit
commitment representation:
`17 B challenge || 728 B completed responses = 745 B`.  The verifier
reconstructs all commitment blocks as `A * z - chi * u` and hashes those
reconstructed commitments, so no explicit commitment bytes are carried.  The
older `3709 B` explicit-commitment baseline remains available as
`--explicit-commitment`, and `--structural-only` still isolates the 69-byte
active payload.

The first-principles obstruction for pushing back toward `69 B` is documented
in `information_set_commitment_frontier.md` and formalized abstractly in
`Descender.InformationSetCommitment`: if an active-only stateless verifier can
reconstruct a prechallenge commitment from the active view, that active
projection must be injective on the allowed commitment code.  Thus the allowed
commitment-code state cannot have larger cardinality than the active view.  For
the product-block active view this is now specialized as
`|Codeword| <= q^(#active blocks * #local free)`, and the `w=1`,
two-free-coordinate case gives `|Codeword| <= q^2`.  The Lean model now also
separates commitment state from sampled mask state: exact reconstruction
bounds the full mask space only when the mask-to-state map is injective.  This
rejects transparent information-set mask codes, since known active masks expose
signed active secret coordinates, and leaves two plausible structural paths:
externalized/stateful commitments or a fibered information-set commitment code
with a low-dimensional reconstructed state and high-entropy mask fibers.  The
fibered path now has a sharper proof target: forked openings either cancel the
same mask and extract the secret, or the mask delta is a same-state
low-norm-collision witness.  This algebraic boundary is recorded by
`productBlockLinearResponse_sub_with_mask_delta` in
`Descender.BlockChallengeSoundness`.  The deterministic alternative itself is
now formalized as
`FiberedForkedLinearRound.extraction_or_collision_of_cover`.  The generic
bounded-mask bridge is now explicit:
`SameStateMaskCollision.toModqLeeKernelViolation_of_boundedMasks` produces a
`ModqLeeKernelViolation`, and `modqLeeKernelViolationKeygenBreakGame` connects
that witness to the partial keygen-break interface ruled out by
`ModqNoLeeKernelVectorBelow`.  The packaged construction obligation is
`FiberedMaskStateCollisionCertificate.no_sameStateMaskCollision`.

The near-69-byte fibered path now has a more concrete state candidate in
`Descender.LocallyRecoverableSyndrome`.  `LocalLinearSyndrome` makes every
block compute a local linear state and declares masks valid only when all
block-local states agree.  Lean proves active reconstruction
(`LocalLinearSyndrome.active_reconstructs_state`), the local non-injectivity
boundary needed for hiding (`LocalLinearSyndrome.localStateHiding_of_hasLocalKernel`),
and the key collision reduction (`LocalLinearSyndrome.collisionToLeeViolation`
and `LocalLinearSyndrome.CollisionCertificate.no_sameStateMaskCollision`).  This
moves the remaining obligation to a concrete coefficient family and a
`ModqNoLeeKernelVectorBelow q S.collisionMap leeMin` certificate, rather than
asking the tiny reconstructed state map itself to have no short kernel.

The first concrete coefficient pass shows why the certificate has to be sharper
than global Lee distance.  `N32Q12289LocalSyndrome` uses the direct local form
`x + 50*y` over `q=12289` and proves a coordinate-wise box certificate:
there is no nonzero local kernel vector in the fork-delta box `[-24,24]^2`.
This is a formal test for the current `BoundedMaskValid eta` abstraction; a
deployed sampler with a wider mask range must rerun the box certificate at that
larger fork-delta bound.  Lean packages this through
`LocalLinearSyndrome.BoxCollisionCertificate` and proves
`N32Q12289LocalSyndrome.no_sameStateMaskCollision`.  The same pass also
formalizes the obstruction:
`N32Q12289LocalSyndrome.stateOfMask_injectiveOn_validMask`.  A direct
mask-syndrome state that is binding on bounded accepted masks is injective on
that bounded mask space, so it does not by itself give preimage hiding.  The next
deployable step is to keep the box-collision idea but move the state readout to
a commitment-domain or trapdoor/preimage construction where active mask recovery
remains hard.

That commitment-domain split is now part of the Lean boundary as well.
`LocalLinearSyndrome.CommittedLocallyRecoverableState` wraps a hidden
locally-recoverable state with a public commitment map.  Lean proves that any
same-public-state collision splits into either a hidden-state collision or a
commitment collision
(`CommittedLocallyRecoverableState.hiddenCollisionOfPublicCollisionOrCommitmentCollision`),
and that a box certificate plus a no-commitment-collision assumption rules out
public-state collisions
(`LocalLinearSyndrome.no_publicStateCollision_of_boxCertificate_noCommitmentCollision`).
For the concrete `x + 50*y` candidate this is packaged as
`N32Q12289LocalSyndrome.no_publicStateCollision_of_noCommitmentCollision`.  This
is still an assumption-parametric proof surface: the remaining implementation
work is a high-entropy hidden preimage/trapdoor instantiation and active-opening
byte accounting, not another direct bounded-mask readout.

The active-opening and byte-accounting layer is now formalized.  The opened
state interface is `LocalLinearSyndrome.OpenedCommittedLocallyRecoverableState`;
the salted constructor is `LocalLinearSyndrome.toSaltedOpenedCommittedState`.
It proves the same public-collision split with explicit opening data, then
charges the compact `n=32`, `w=1`, `26`-repetition payload as
`69 + 26 * opening_bytes_per_rep`.  The named accounting endpoints are:

```text
N32Q12289LocalSyndrome.saltedOpening_signatureBytes_eq
N32Q12289LocalSyndrome.salted128Opening_signatureBytes_eq  -- 485 B
N32Q12289LocalSyndrome.salted256Opening_signatureBytes_eq  -- 901 B
```

The same Lean pass records why this is not yet the deployable hiding
construction:

```text
N32Q12289LocalSyndrome.saltedHiddenState_injectiveOn_validMask
```

If the active opening reveals the hidden salted state, then under the box
certificate it identifies the bounded valid mask.  The deployable target must
therefore use a trapdoor/ZK-style opening that lets the verifier bind the public
state without learning the injective hidden preimage, or change the hidden
domain so the active-recoverable component is not injective on accepted masks.

Both escape routes have now been trialed in Lean.  The trapdoor/ZK-shaped route
is represented by
`LocalLinearSyndrome.PublicOpenedCommittedLocallyRecoverableState`: the active
opening reconstructs only the public committed state, not the hidden preimage.
The binding theorem is
`LocalLinearSyndrome.no_publicOpenedStateCollision_of_boxCertificate_noCommitmentCollision`,
with the concrete N32 wrapper
`N32Q12289LocalSyndrome.no_publicOpenedStateCollision_of_noCommitmentCollision`.
This is the preferred next construction surface; the missing work is a concrete
`openPublicState` with a zero-knowledge/preimage-hiding argument and byte cost.

The raw non-injective hidden-domain route is blocked by
`LocalLinearSyndrome.no_validStateNonInjective_of_boxCertificate`: under the
current box-collision proof, valid-mask non-injectivity is exactly the collision
branch we are trying to rule out.  The viable variant is
`FiberDeltaOpening.correctedExtractCoordinate_eq_secret`: if same-state active
openings reveal enough to recover the fork mask delta, corrected extraction can
still work.  That route trades the no-collision certificate for delta-opening
byte cost and leakage analysis.

The delta-opening route now has a concrete algebraic construction target:
`LocalFiberDeltaOpening.additiveOneTimePad`.  The active opening is a
one-time-padded local mask view, so one transcript is hiding
(`LocalFiberDeltaOpening.additiveMaskedLocalOpening_perfectHiding`), while two
same-fiber fork openings subtract to the local mask delta
(`LocalFiberDeltaOpening.correctedExtractCoordinate_eq_secret`).  This is the
right abstraction for carrying enough algebraic structure without revealing the
mask itself.  The simple byte-aligned N32 account is
`N32Q12289LocalSyndrome.localDeltaOpening_signatureBytes_eq`, giving `173 B`
before tighter bit packing.

The pad-binding layer is now explicit too.  The pad source is hidden inside the
committed preimage, not publicly derived from the state:
`LocalFiberDeltaOpening.additiveCommittedOneTimePad` uses commitment binding to
force pad reuse across same-public-state forks.  The transcript side is
captured by `LocalFiberDeltaOpening.StateDigestBinding` and
`LocalFiberDeltaOpening.HashBoundLocalDeltaForkedRound.correctedExtractCoordinate_eq_secret`:
equal prechallenge state digests give equal public fiber state, and the local
padded openings provide the mask-delta correction for extraction.

The concrete byte bridge is `Descender.LocalDeltaByteCommitment`.  It defines
`ByteCommittedPadPreimage` as `fiber-state bytes || pad-seed bytes`,
`ByteCommitmentConfig` as the domain-separated hash/binding assumption, and
`byteCommittedLocalDeltaOpening` as the byte-instantiated local-delta opening.
The N32/q12289 instantiation now pins the byte layout:

```text
commitment domain: "Descender.Signature.q12289.local_delta.v1"
hidden preimage:  fiber-state[2] || pad-seed[32]
pad PRF domain:   "Descender.Signature.q12289.local_delta.pad.v1"
pad frame:        pad-domain[45] || block-index[1]
pad output:       rejection-sampled SHA-256 stream -> two q-residues
```

The exact assumption slots are
`LocalDeltaByteCommitment.N32Q12289.CommitmentHashBinding` for collision
resistance of the 34-byte hidden preimage hash and
`LocalDeltaByteCommitment.N32Q12289.PadPRFBinding` for replacing ideal local
one-time pads with seeded PRF output.  The account-state side is
`LocalDeltaByteCommitment.N32Q12289.BlockchainAccountState`.
The executable fixture
`tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`
implements the matching SHA-256 layout and pins a sample digest/pad/opening
vector.  Its Fiat-Shamir frame hashes only the prechallenge material
`profile || parameters || slot id || public key || message || slot digest`.
The local openings are response/proof material checked after challenge
derivation; hashing them into the challenge would create a fixed-point signing
problem rather than a usable signature format.
For N32/q12289 it records the deployment split:

```text
LocalDeltaByteCommitment.N32Q12289.statefulSpend_signatureBytes_eq    -- 177 B
LocalDeltaByteCommitment.N32Q12289.statefulTransaction_signatureBytes_eq -- 181 B
LocalDeltaByteCommitment.N32Q12289.statelessSha256_signatureBytes_eq  -- 1009 B
```

The small number is therefore a stateful/precommitted spend payload; the
transaction-level form is 181 B once the 4-byte slot id is included.  This
includes a 17-byte algebraic challenge plus a 4-byte transcript-hardening
suffix.  A stateless reusable signature has to carry, compress, aggregate, or
otherwise recover the per-repetition commitment digests.

The executable verifier is still an ideal relation checker: it stores the
hidden preimage in the test account so it can check the opening relation
directly.  A deployed public verifier needs the corresponding compact public
proof or commitment-opening representation in place of that ideal preimage
oracle.

That boundary is now formalized.  `PublicOpeningProofSystem` is the plug-in
interface for a public proof of the hidden opening relation, and
`StatefulPublicOpeningVerifierRelation.algebraicVerifierObligations` proves
that any sound instantiation drops into the stateful verifier relation.  The
byte formula is explicit:

```text
LocalDeltaByteCommitment.N32Q12289.statefulTransactionWithProofBytes_eq
  -- total = 181 B + proof_bytes
```

The Rust fixture also includes a public-checkable control mode that appends the
34-byte hidden preimage:

```text
LocalDeltaByteCommitment.N32Q12289.revealedPreimageProofSystem
LocalDeltaByteCommitment.N32Q12289.revealedPreimageTransactionSignatureBytes_eq
revealed-preimage public check = 181 B + 34 B = 215 B
```

This is useful as a verifier sanity baseline, but not a production candidate:
it proves public checkability only by revealing the witness.  The real
remaining construction target is a witness-hiding proof/opening whose byte cost
is small enough to keep the total close to the 177-byte stateful target.

The concrete proof-byte budgets are:

```text
proof <= 75 B   -> total <= 256 B
proof <= 87 B   -> total < 269 B completed-response verifier
proof <= 88 B   -> total <= 269 B completed-response verifier
proof <= 827 B  -> total < 1009 B stateless SHA-digest mode
```

This strongly argues against a generic ZK proof of SHA-256/PRF evaluation as
the next implementation path.  The promising route is to change the commitment
representation so the public opening relation is algebraic and can plausibly
fit under roughly 90 bytes, rather than proving a hash circuit.

The first concrete design target is an algebraic linear sigma-style opening:

```text
one proof commitment:        32 B
masked hidden-witness reply: 37 B
target proof size:           69 B
target transaction size:     250 B
```

Lean records this as
`LocalDeltaByteCommitment.N32Q12289.LinearSigmaOpeningTarget` and
`LocalDeltaByteCommitment.N32Q12289.linearSigmaTargetTransactionSignatureBytes_eq`.
This is not yet a secure instantiation; it is the byte target for the next
commitment design.  To realize it, the account commitment must be algebraic
enough that a masked witness response verifies the pad/opening relation without
exposing the hidden preimage.

The target is now instantiated as a byte-level proof shape:
`AlgebraicLinearSigmaOpeningConfig` supplies a field verifier over
`proof_commitment[32] || masked_witness_response[37]`, and
`toLinearSigmaOpeningTarget` turns it into the public opening proof interface.
The Rust fixture parses and round-trips exactly the same 69-byte field layout.

The chosen primitive shape is now the compressed linear-sigma opening:

```text
L_statement(z) = a + challenge * target(statement)

a: 32-byte proof commitment
z: 37-byte masked hidden-witness response
```

Lean records this as `CompressedLinearSigmaAlgebra`.  It gives an honest
prover equation and packages the candidate into the 250-byte
`LinearSigmaOpeningTarget` once a concrete relation map, computational
knowledge-soundness assumption, and witness-hiding argument are supplied.

The concrete path now uses a restricted witness domain:

```text
hidden_preimage = derive_fiber_state(seed)[2] || seed[32]
```

Lean records this as
`SeedRestrictedCompressedLinearSigmaAlgebra`.  It converts directly to
`LinearSigmaOpeningTarget` and proves that the restricted hidden-preimage map is
injective over seeds.  The executable report now labels the selected primitive
as `seed-restricted-compressed-linear-sigma-opening` and emits a sample
seed-derived fiber state/preimage.

The SIS-style boundary is also explicit now.  Lean records
`SISStyleSeedOpeningMap`, fixes the public target to `statement.digest`, and
packages it into the same 250-byte linear-sigma opening target.  The two
remaining cryptographic obligations are named precisely:

- `relationMapSound`: any seed satisfying
  `L_statement(seed) = statement.digest` satisfies the byte opening relation for
  the statement;
- `sisKnowledgeSoundness`: any accepting equation
  `L_statement(z) = a + challenge * statement.digest` yields such a seed.

The first byte-level packing target for that map is also pinned:
`sisSeedResidues = 18`, `sisSyndromeResidues = 18`,
`sisResidueBits = 14`, and both packed seed and syndrome fields are `32` bytes
with `4` slack bits.  Interpreted over `q = 12289`, this means the 32-byte
masked response can be treated as a packed q-vector and the 32-byte account
digest/proof commitment can be treated as a packed q-syndrome.

Lean now records the concrete reduction surface as
`PackedSISSeedOpeningReduction`.  It requires the implementation to supply the
matrix, canonical packed seed and digest validity checks, byte-level linearity,
and an
`extracts_or_break` theorem: every accepting algebraic equation either extracts
a seed hitting the statement digest or produces the named SIS/algebraic binding
break.  With `noAlgebraicBindingBreak`, this instantiates
`SISStyleSeedOpeningMap`.  This is still not a security claim for a specific
matrix; it is the exact gate a concrete matrix/domain choice must pass.  The
canonical digest check is essential: an 18-coordinate q-syndrome embeds into
32 bytes but cannot represent every 32-byte string.

The executable report now pins the candidate matrix domain string
`Descender.Signature.q12289.local_delta.sis_matrix.v1` and derives sample
entries by SHA-256 counter expansion with rejection sampling modulo `12289`.
That gives reviewers a deterministic matrix-generation target while keeping the
actual `extracts_or_break` security analysis as the remaining cryptanalytic
obligation.

The first matrix sanity check is negative for the bare linear interpretation.
The generated `18 x 18` matrix has full rank modulo `12289`
(`det = 11243`).  Lean records the corresponding structural obstruction through
`NontrivialSeedFiber`, `noNontrivialSeedFiber_of_injective`, and
`PublicSeedRecoverer`: if the public linear syndrome map is injective, a public
left inverse recovers the seed from the digest.  Therefore the current
deterministic square matrix gives trivial extraction but cannot be the deployed
witness-hiding opening by itself.  A viable next candidate must add a genuinely
hard inversion layer, a hidden/trapdoor component, or change the dimensions so
the public map has nontrivial fibers while preserving the binding proof.

The nontrivial-fiber variant has also been checked at the byte-capacity level.
For a public map with `r` q-syndrome rows, the public capacity is `14r` bits and
the remaining packed hidden-fiber capacity is `14(18-r)+4` bits.  Lean proves
`noBalanced128BitFiberSplit`: no such split inside the same 32-byte q-vector
packing can give both `>=128` public bits and `>=128` hidden-fiber bits.  The
balanced-looking `r = 9` row gives `126` public bits and `130` hidden-capacity
bits; `r = 10` gives enough public capacity but only `116` hidden-capacity bits.
So nontrivial fibers help conceptually, but the 18-residue response field is too
tight to carry both sides of a 128-bit target without another hardness layer or
more bytes.

The obvious q-linear hidden-component repair has the same capacity problem.
For a Pedersen-style split of the 18 response residues into message-seed
residues and blinding residues, perfect linear hiding needs at least as many
blinding residues as public syndrome rows.  Lean proves
`noPedersenStyle128BitSplit`: inside the same 18-residue response, one cannot
simultaneously fit `>=128` bits of seed capacity, `>=128` bits of public
syndrome capacity, and enough blinding dimension for perfect hiding.  This
rules out the direct linear hidden-component version of the 250-byte target.

The selected path now enlarges the full-response carrier to 37 bytes, rather
than forcing the 34-byte semantic preimage to carry all algebraic randomness.
A 37-byte response packs `21` q-residues (`294` bits) with `2` slack bits.
With `10` public syndrome rows, the public side has `140` bits and the raw
fiber has `11 * 14 + 2 = 156` bits.  After the fixed two-byte local fiber-state
constraint, the modeled hiding budget is `140` bits.  Lean records this as
`fullResponseFiber_publicRows10_meets128`, and the Rust fixture checks that the
deterministic `10 x 21` matrix prefix has rank `10`.  This gives the selected
69-byte proof size:

```text
proof_commitment[32] || masked_response[37]
```

The cost is conceptual rather than byte-level: the relation must be rebuilt
around full-response q-vectors instead of treating the response as
`fiber_state[2] || seed[32]` with the first two bytes ignored by the algebra.
Lean now has that target interface as `FullResponseFiberedLinearSigmaAlgebra`.
It converts to the same `LinearSigmaOpeningTarget`, preserves the 250-byte
total, and states the remaining proof obligation in the right form:
`responseKnowledgeSoundness` extracts a full 37-byte response vector, and
`relationMapSound` must show that every accepted extracted vector in the fiber
implies the byte opening relation.

The byte-level specialization is now explicit as
`FullResponseByteOpeningAlgebra`.  Its deployed target relation is exactly the
chosen `idealRelation` on the decoded full response
`hiddenPreimageFromLinearSigmaWitnessBytes response`, so the proof no longer
relies on a seed-only interpretation of the extracted vector.  The remaining
instantiation work is concrete: define the selected q-ary `relationMapResponse`
and prove its soundness, completeness, and extraction-or-break fields.

That gate is now formalized as `PackedSISFullResponseOpeningReduction`.  It
fixes the concrete dimensions for the no-extra-byte path:

```text
response carrier:       37 B = 21 q-residues + 2 slack bits
public syndrome target: 10 q-residues = 140 bits
digest/commitment slot: 32 B, with canonical 10-residue packing and 116 slack bits
Lean conversion:        PackedSISFullResponseOpeningReduction
                        -> FullResponseByteOpeningAlgebra
                        -> LinearSigmaOpeningTarget
```

So the proof architecture has moved from an informal “maybe use the full
response” idea to a named assumption slot.  A concrete instantiation must still
provide the matrix, canonical packing predicates, relation-map soundness,
honest completeness, and extraction-or-algebraic-binding-break theorem for that
slot.  The Rust fixture now names the selected primitive as
`full-response-packed-sis-linear-sigma-opening` and reports the deterministic
matrix domain
`Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1`.

Trying to close that gate exposes the decisive constraint.  Lean now proves
`FullResponseByteOpeningAlgebra.acceptedFiber_relationSaturated`: for a
full-response public map, if one response hits the digest, then every response
with the same public syndrome must also decode to a valid byte-level witness.
It also proves
`FullResponseByteOpeningAlgebra.acceptedFiber_subsingleton_of_commitmentBinding`:
if the ideal relation binds every byte of `fiber_state[2] || pad_seed[32]`
through an injective commitment, then accepted fibers collapse to singletons.
The same theorems are exposed for `PackedSISFullResponseOpeningReduction`.

The deterministic `10 x 21` matrix does have the intended nontrivial fibers:
the executable report now emits a kernel sanity certificate with rank `10`,
kernel dimension `11`, and a sampled nonzero kernel vector whose public syndrome
is zero.  Therefore the current SHA-style relation that binds the entire
34-byte semantic hidden preimage is not a valid full-response instantiation by
itself.  To keep the 250-byte target, the relation must be redesigned as a
fiber-saturated quotient relation: the hidden 11-residue fiber may provide
hiding entropy, but it cannot be part of the committed semantic witness checked
byte-for-byte.  Otherwise we have to spend more public bytes/rows and give up
the nontrivial fiber.

The quotient direction is now a first-class proof surface.  Lean adds
`FullResponseQuotientRelation`, where the semantic witness is the public
full-response syndrome quotient and the hidden fiber is representative
randomness.  It proves:

```text
FullResponseQuotientRelation.acceptedFiber_relationSaturated
FullResponseQuotientRelation.no_fullPreimageCommitmentBinding_of_nontrivialFiber
```

The packed construction gate is
`PackedSISFullResponseQuotientOpeningReduction`, which converts into the
existing `PackedSISFullResponseOpeningReduction` and then into the same
250-byte `LinearSigmaOpeningTarget`.  This is the route we should pursue for
the 69-byte proof: instantiate a concrete public
`quotientOpensStatement` over the 10-residue quotient, and treat the 11-residue
kernel as hiding randomness only.  The Rust report now records this split as
`semantic_quotient_residues = 10`, `hidden_randomizer_residues = 11`, and
`full_preimage_binding_compatible = false`.

The 250-byte completion proof is now existential at the quotient layer.  The
generic packed opening gate no longer requires the 34-byte semantic hidden
witness to determine the full 37-byte algebraic representative.  Instead,
`relationMapCompleteSyndrome` supplies some representative in the digest
syndrome for any ideal witness, and
`PackedSISFullResponseQuotientOpeningReduction.toPackedSISFullResponseOpeningReduction`
extracts that representative directly from the quotient ideal relation.  This
is the formal pad-constraint split: active pad equations prove that the public
quotient opens by existence of a valid representative, while witness hiding is
analyzed over the masked response in the quotient fiber rather than over a
representative additionally conditioned on every active pad output.

Lean also adds `FullResponseExistentialQuotientRelation`.  This is the bridge
from the old representative-level local-opening semantics to the quotient
semantics: a quotient opens a statement when there exists some response
representative in that fiber satisfying the representative predicate.  The
extractor may return any response in the same accepted fiber; the semantic
claim is the quotient-level existence statement, not validity of every hidden
randomizer as a SHA-bound preimage.

The local-delta representative predicate is now instantiated as
`ByteLocalDeltaRepresentativeSemantics`, and the leading N32/q12289 layout is
now wired through `concreteByteLocalDeltaRepresentativeSemantics`.  Lean fixes
the response byte layout (`52` bytes as two active signed response coordinates
per repetition), local opening byte decoding (`4` bytes per repetition), active
mask reconstruction, the `x + 50y` local-state equation, the padded-opening
equation, the 17-byte algebraic challenge decoder with 4 transcript-hardening
bytes, the 172-byte public-key packed14 free-view decoder, and the
`PadPRFBinding.expandPad` pad hook.  The
Rust-layout lemmas in `ByteLocalDeltaRustLayout` name the exact byte positions
for response centering, public-key free coordinates, packed q canonicality, and
pad-frame construction.  Its quotient predicate is exactly:

```text
exists response in this 10-residue quotient fiber
  such that response satisfies the concrete local-delta byte equations
```

The Rust report now names this as
`ByteLocalDeltaRepresentativeSemantics.quotientOpensStatement`, and records
the concrete decoder and honest-representative gates as
`concrete_decoder_gate` and `honest_representative_gate`.

The honest-completeness side is also explicit now.  An
`HonestLocalDeltaQuotientStatement` contains the representative response, the
proof that it maps to the digest syndrome, and the proof that it satisfies the
concrete local-delta byte equations.  From that, Lean derives
`targetRepresentativeExists_of_honestStatements` and the direct
`quotientOpensStatement_of_honestStatement`, which are the facts needed by the
fiber-saturated quotient gate.  The remaining generated-system obligation is
to prove that the actual stateful account/signature generator supplies those
honest statement objects with the concrete SHA-256/rejection-sampled pad source
and q-syndrome digest builder.

That generated-system obligation is now split into executable and formal
pieces.  Lean adds `ConcreteHonestLocalDeltaSigningTrace`, which proves the
concrete byte equations imply `representativeOpensStatement`, and
`ConcreteHonestFullResponseQuotientSigningTrace`, which proves that a
canonical packed q-syndrome account digest yields
`HonestLocalDeltaQuotientStatement` and
`targetRepresentativeExists_of_traces`.  On the Rust side, the reference code
now has `full_response_syndrome_digest`: it decodes the 37-byte masked
response as 21 canonical 14-bit q-residues, applies the deterministic 10x21
full-response matrix, and packs the resulting 10-residue public syndrome into
the existing 32-byte digest field.  The test
`quotient_stateful_fixture_emits_honest_local_delta_trace` checks that a
rejection-sampled canonical hidden preimage with that digest satisfies the
challenge-bound local-state and padded-opening equations.

The verifier gap is now bridged in the reference implementation.  The shared
stateful verifier has two explicit wrappers: the legacy comparison path checks
`SHA256(commitment_domain || hidden_preimage)`, while
`verify_quotient_stateful_signature` checks
`full_response_syndrome_digest(hidden_preimage) == slot.digest`.  The
production reference test
`quotient_stateful_verifier_accepts_and_consumes_slot` uses the quotient
digest path, and `legacy_sha_verifier_rejects_quotient_digest` confirms the old
SHA-hidden-preimage relation does not accidentally accept quotient accounts.
Lean also records the deployed-verifier bridge:
`PackedSISFullResponseQuotientOpeningReduction.statefulVerifier_quotientIdealWitness_of_accepts`
and `statefulVerifier_targetQuotientOpens_of_accepts` connect an accepting
stateful public-opening verifier using the quotient linear-sigma target to the
fiber-saturated quotient relation.

The concrete codec/reduction surface is now internalized rather than only
described by the Rust fixture.  `FullResponseCodec` names the 37-byte response
decoder, the deterministic full-response matrix domain, the canonical
10-residue digest decoder, and the exact q-syndrome digest-match predicate.
`FullResponseMatrixSamplerAgreement` isolates the remaining byte-for-byte
agreement check between Lean's matrix function and the deployed SHA-256
rejection sampler.  `ConcreteFullResponseQuotientReductionAssumptions` fixes
the executable codec, matrix agreement, digest packing, honest-representative
bridge, extraction-or-break slot, witness-hiding slot, and the 250-byte total
for the leading quotient target.  The pad source is similarly packaged as
`SHA256RejectionPadPRFAssumption`: the concrete frame is fixed, while SHA-256
PRF-style pseudorandomness remains a named deployment assumption rather than a
Lean theorem.

The review surface is now split into executable and formal closure artifacts.
The executable report's `assumption_review_targets` table names the five
deployment gates: full-response matrix generation, packed full-response
extraction, quotient witness hiding, pad-source pseudorandomness, and
Fiat-Shamir challenge-frame agreement.  Its `assumption_closure_report` table
then states what is executable and what remains a cryptographic assumption.
The concrete artifacts are:

```text
MatrixSamplerCertificate:
  domain = Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1
  rows x cols = 10 x 21
  rejection cutoff = 61445
  matrix transcript SHA-256 =
    09b7793ca756c28d6b02e84e6f902e4d0e5c17dad47891d4ecc4e88c9d5e4d94

FullResponseQuotientAlgebraCertificate:
  rank = 10
  kernel dimension = 11
  right-inverse check = true
  semantic quotient capacity = 140 bits
  hidden fiber capacity = 156 raw bits

PadSamplerCertificate:
  pad frame = pad-domain[45] || block[1]
  counter encoding = u32 little-endian
  rejection cutoff = 61445

FiatShamirFrameCertificate:
  frame labels =
    domain, profile, q, groups, local_pivot, local_free, repetitions,
    slot_id, public_key, message, slot_digest
  response/opening bytes are excluded from the prechallenge frame
```

Lean mirrors this with `FullResponseQuotientAlgebraCertificate` and
`FullResponseDeploymentReviewCertificate`.  The former states the quotient
map's surjectivity and kernel facts.  The latter bundles the pad assumption,
concrete quotient reduction assumptions, quotient algebra certificate, and
Fiat-Shamir frame agreement into one review object with direct consequences:
`totalBytes`, `padPseudorandom`, `matrixSamplerAgreement`,
`quotientSyndromeSurjective`, `quotientFiberNonempty`,
`quotientWitnessHiding`, and `fiatShamirFrame`.

The extraction assumption is now sharpened into a concrete break condition:
`ConcreteFullResponseQuotientReductionAssumptions.extractionFailure` is the
existence of an accepting packed full-response sigma equation from which no
valid quotient representative can be extracted.  Lean proves
`no_extractionFailure` from the explicit `extracts_or_break` and
`noAlgebraicBindingBreak` fields.  This does not prove the algebraic binding
assumption itself, but it gives reviewers the exact failure event the
assumption rules out.

The first real top-level theorem for the stateful candidate is now
`FullResponseDeploymentReviewCertificate.statefulVerifier_conditionalSoundness`.
Given the explicit deployment review certificate, every accepting stateful
public-opening verifier instance yields:

```text
slot accepted before verification
target quotient opens at the statement digest
active product-block verifier equation holds
slot is consumed and cannot be reused
transaction size = 250 B
pad pseudorandomness, witness hiding, matrix agreement, and Fiat-Shamir
  agreement are exactly the named assumptions from the certificate
```

So the proof is now real in the reduction sense: the theorem has no hidden
informal step at the verifier/composition layer.  What remains outside Lean is
the standard cryptographic review of the explicitly named assumptions, mainly
packed full-response extraction, quotient witness hiding, SHA-256 pad PRF
security, and Fiat-Shamir/random-oracle agreement.

Those assumptions are now named as games rather than broad anonymous
propositions:

```text
PackedFullResponseExtractionGame:
  break event: extractionFailure
  claim: noExtractionBreak => no_extractionFailure

QuotientWitnessHidingGame:
  break event: witnessHidingBreak
  claim: noWitnessHidingBreak => witnessHiding

SHA256PadPRFGame:
  break event: padPRFBreak
  claim: noPadPRFBreak => pseudorandomLocalPad

FiatShamirFrameAgreementGame:
  break event: frameMismatchBreak
  claim: noFrameMismatchBreak => fiatShamirFrameAgreement
```

The executable `assumption_closure_report` now emits the same `lean_game`,
`break_event`, and `no_break_claim` fields.  The remaining hardening step is
quantitative: attach adversary classes, query limits, and concrete advantage
bounds to these four games.

The budget slot is now formal too.  `GameSecurityBudget` carries
`adversaryQueries`, `classicalSecurityBits`, and `quantumSecurityBits`; each
game has a `budget_meets128` field, and the conditional deployed-verifier
theorem returns those facts.  The executable report mirrors this with
`target_classical_security_bits = 128`, `target_quantum_security_bits = 128`,
and a `query_bound_status` note for each gate.  The remaining review work is to
fill those budget slots with externally reviewed query bounds and advantage
estimates.

The executable report now adds `concrete_security_budgets`, a first generic
query-loss budget table:

```text
PackedFullResponseExtractionGame:
  raw public quotient target = 168 bits
  generic quantum-query margin for 128 bits = 2^20

QuotientWitnessHidingGame:
  raw hidden-fiber capacity = 160 bits
  modeled post-fiber-state capacity = 144 bits
  exact q=12289 conditioned support = 12289^9 * 3073
  exact min-entropy floor = 133 bits
  generic support-search fallback margin for 128 bits = 2^2
  game-specific statistical hiding advantage = 0 under the sampler certificate

SHA256PadPRFGame:
  raw primitive/seed target = 256 bits
  generic quantum-query margin for 128 bits = 2^64

FiatShamirFrameAgreementGame, hardened 21-byte challenge:
  algebraic challenge entropy = 130 bits
  transcript-bound challenge entropy = 162 bits
  generic quantum-query margin for 128 bits = 2^17
  generic classical-query margin for 128 bits = 2^34
```

The conclusion is that the hardened 254-byte candidate is still size-excellent
and no longer has the pathological Fiat-Shamir multi-query margin.  The
SHA-256 pad source has comfortable primitive margin; the quotient/extraction
budgets are still narrow but reviewable; the Fiat-Shamir challenge now carries
32 extra transcript-bound bits at a direct 4-byte cost.

The cryptoanalysis track has started in the executable report.  The new
`cryptoanalysis_attack_surfaces` table names five concrete attack gates:
packed full-response extraction, quotient witness hiding, pad-source
pseudorandomness, Fiat-Shamir frame agreement, and stateful key/account
lifecycle.  The first two gates also get bounded smoke tests:

```text
matrix_short_relation_probe:
  target = 10x21 full-response quotient matrix
  coefficients = {-2,-1,1,2}
  support <= 3
  candidates tested = 88564
  exact zero relations found = 0
  shortest nonzero syndrome Lee weight found = 10458

hidden_fiber_short_combination_probe:
  target = certified 11-vector hidden fiber basis
  coefficients = {-1,0,1}
  nonzero combinations tested = 177146
  shortest hidden-fiber Lee weight found = 9023
```

These probes are deliberately not proofs.  They are CI-sized cryptanalytic
smoke tests that rule out obvious low-support column dependencies and tiny
kernel-basis combinations before we run heavier lattice-reduction/SIS searches.

The first heavier SIS probe is now included too.  `lattice_sis_reduction_probe`
builds an integer kernel-lattice basis from the 11 modular q-kernel vectors plus
`q` times the 10 pivot coordinates, then runs a deterministic LLL pass with
`delta = 0.99`.  The current result is:

```text
lattice dimension = 21
basis vectors = 21
LLL iterations = 4063
shortest syndrome-zero vector found:
  Euclidean norm^2 = 9825
  Lee weight       = 341
  support          = 21
  coefficient max  = 50
```

This is not automatically an attack, because a quotient-kernel delta only
matters if it carries one accepting local-delta representative to another
accepting representative under the byte-level opening equations.  It is
important evidence, though: the matrix has short kernel directions, so the next
cryptanalytic question is no longer “does the public quotient have any short
kernel vector?” but “are the short kernel directions excluded by the
representative relation, response bounds, pad equations, and Fiat-Shamir
challenge constraints?”

The first hiding-distribution boundary is also executable as
`quotient_witness_hiding_distribution_report`:

```text
public quotient rows:                       10 q-residues
hidden kernel dimension:                    11
algebraic hidden-fiber capacity:            156 raw bits
fixed local fiber-state constraint:         16 bits
estimated post-fiber-state freedom:         140 bits
current fixture representatives sampled:    1
current fixture uses hidden fiber entropy:  false
```

So the algebraic object has hiding capacity, but the current fixture is only an
honest representative generator and verifier test.  A production hiding proof
still needs a representative sampler over the accepted quotient fiber and a
finite TV/KL or exact-counting bound after conditioning on canonical packing,
the fixed 2-byte local state, and response bounds.  Pad-opening acceptance is
now handled by the existential quotient-opening predicate rather than by
conditioning the hidden representative.

That sampler obligation is now named in both Lean and the executable report.
Lean records it as `FullResponseMaskedResponseSamplerCertificate`: the
certificate fixes `156` raw algebraic fiber bits, subtracts only the `16`
fiber-state bits in the 14-bit capacity model, records the exact finite
`q = 12289` support formula `12289^9 * 3073`, and pins the conservative
min-entropy floor to `133` bits.  The exact count uses the rank-2 projection of
the hidden kernel onto packed coordinates `[0, 1]`: fixing the two-byte fiber
state fixes residue `0` and forces residue `1 = 0 mod 4`, leaving `q^9 * 3073`
representatives.  The certificate has an explicit `excludesActivePadOutputs`
field.  Its theorem
`FullResponseMaskedResponseSamplerCertificate.witnessHiding` is the bridge from
a concrete fiber-symmetric sampler proof to the `witnessHiding` assumption
consumed by `QuotientWitnessHidingGame`.  The certificate also carries a
checkable finite-distance slot; for the selected rejection-sampling law the
total-variation numerator is `0`, because uniform kernel-coefficient sampling
conditioned by the same predicate as the ideal fiber law is exactly the ideal
conditioned distribution.  The Rust report mirrors this as
`masked_response_sampler_certificate`, including the allowed conditioning set,
the excluded active-pad-output conditioning set, and the `0/1` TV-distance
certificate.  This replaces the previous implicit reuse of the 14-bit capacity
approximation: the hiding budget now uses `133` raw bits and a `2^2` generic
quantum-query margin for 128-bit support-search auditing.

The sharper game-specific accounting is better than that generic fallback.
`QuotientWitnessHidingStatisticalBound` records that the stateful hiding game is
a one-shot transcript game for each slot: the adversary receives a sampled proof
transcript, not an equality oracle for the hidden representative.  Under the
sampler certificate, both witness branches induce the same conditioned fiber
law, so the transcript total-variation distance is exactly `0/1`.  Independent
stateful slots compose as product distributions, and zero per-slot distance
stays zero for any fixed number of independently sampled slots.  The `133`-bit
support floor therefore remains important for sampler population, collision,
and retry sanity, but it is not the distinguishing advantage of
`QuotientWitnessHidingGame`.

The executable reference now implements the efficient sampler that this proof
surface wants.  Instead of rejection over hidden-kernel coefficients, the
`conditioned_quotient_fiber_sampler_report` sampler solves the `10` public
q-syndrome equations together with the two packed fiber-state equations
(`residue0 = fiber_state`, `residue1 = 0 mod 4`).  It then samples the remaining
`9` free q-residues directly and chooses residue `1` uniformly from the `3073`
allowed values.  The report records:

```text
conditioned_quotient_fiber_sampler_report:
  total linear rows:        12
  linear rank:              12
  sampled free residues:    9
  residue1 allowed values:  3073
  exact support:            12289^9 * 3073
  retry loop required:      false
  expected retries:         1
```

The sample checks preserve the public syndrome, preserve the two-byte fiber
state, and decode as canonical packed q.  This closes the practical retry
leakage issue for the quotient-fiber sampler itself; remaining timing review is
now about hash-to-field sampling and wallet state management, not an
unbounded representative rejection loop.

The accepted-fiber constraint report makes this sharper, and it is the current
cryptanalytic blocker for the representative-level quotient design as written:

```text
accepted_fiber_constraint_report:
  repetitions:                                      26
  distinct active blocks in the sample challenge:   12
  algebraic hidden fiber:                           156 raw bits
  fixed local fiber-state constraint:               16 bits
  max bits after fiber-state equality:              140 bits
  meets 128 after fiber-state equality:             true
  active pad output constraints:                    12 * 28 = 336 bits
  estimated accepted-fiber freedom:                 0 bits
```

So raw quotient capacity is not enough if every pad check remains inside the
hidden representative.  For a fixed public statement, another representative
must preserve the same decoded local fiber state and the same pad outputs on
every distinct active block.  The 250-byte route covers the two-byte fiber-state
equality, but the active-block pad constraints still overdetermine the fiber.
This does not break the verifier, but it means the current representative
semantics cannot discharge the `QuotientWitnessHidingGame` assumption unless
the pad checks are composed outside the hidden quotient representative.

The LLL vector was also tested as an actual statement-preserving delta:

```text
kernel_delta_acceptance_probe:
  delta source:                         LLL shortest syndrome-zero vector
  tested multipliers:                   +/-1..+/-8
  public syndrome preserved:            true
  canonical packing preserved:          true
  fiber-state-preserving multipliers:   none
  active-pad-preserving multipliers:    none
  accepting multipliers:                none
```

That narrows the failure mode.  The short kernel vector does not immediately
give a same-statement forgery in the tested range; it fails the representative
acceptance constraints.  The real issue is constructive: the proof needs a
different hiding surface, for example by decoupling the public quotient witness
from the pad seed/fiber-state bytes, increasing hidden fiber capacity, or
making the relation genuinely fiber-saturated without rechecking byte-for-byte
seed-derived pads inside the hidden representative.

Both repair routes are now represented in the executable
`hiding_route_frontier`:

```text
decoupled quotient witness:
  response:     19 q-residues = 34 B
  proof:        66 B
  transaction:  247 B
  residual hiding model: 132 bits
  meaning: move pad/fiber-state checks out of the hidden quotient witness

larger fiber, keep local fiber-state equality:
  response:     21 q-residues = 37 B
  proof:        69 B
  transaction:  250 B
  capacity model: 140 bits after the 16-bit fiber-state constraint
  exact q=12289 conditioned min-entropy floor: 133 bits

larger fiber, sample active-pad constraints:
  response:     49 q-residues = 86 B
  proof:        118 B
  transaction:  294 B
  residual hiding model: 140 bits after sample active-pad constraints

larger fiber, worst-case active-pad constraints:
  response:     53 q-residues = 93 B
  proof:        125 B
  transaction:  308 B
  residual hiding model: 140 bits after all 16 blocks are constrained
```

This makes the tradeoff clear.  If we can decouple the quotient witness from
the pad seed/fiber-state byte checks, the 247-byte target survives.  If we only
need to survive the local fiber-state equality, a 250-byte target is enough.
If we keep byte-for-byte active pad checks inside the hidden representative,
the signature jumps to roughly 294-308 bytes, which is still compact by PQ
standards but no longer the same design point.

The deployable blockchain target has now been hardened from the tight
`d = 10` quotient/action profile to a `d = 12` profile.  The old 250-byte row is
still useful as a minimal research candidate, but it left the action-collision
event as the generic quantum bottleneck.  The current leading byte target is:

```text
Descender quotient candidate:
  public key:              172 B
  spend signature payload: 250 B = 177 B hardened stateful spend + 73 B quotient proof
  transaction signature:   254 B including the 4-byte slot id
```

The hardened proof uses a 41-byte response carrying 23 q-residues, a
12-residue public quotient, and the pinned action field
`F_12289[X] / (X^12 + X + 25)`.  The comparator rows carry source URLs for
FIPS 204, FIPS 205, the Falcon
specification, and RFC 8032.  These numbers are benchmark targets under the
stated assumptions, not a claim that the assumptions are already
cryptographically discharged.

The reference implementation surface is now executable in the report rather
than only sketched.  It exposes `reference_keygen(slot_count)`,
`reference_sign(keypair, slot_id, message)`, and `reference_verify(...)`, all
wired to the quotient digest verifier path.  The `reference_implementation`
report emits deterministic known-answer hashes for the public key, slot digest,
and transaction signature, and the unit tests cover successful sign/verify,
slot consumption, consumed-slot replay rejection, message mutation, opening
mutation, and slot-id mutation.

The stateful lifecycle policy is also explicit: a slot is burned by any
released signature, including signatures seen only on a reorged fork; signing
must reserve/consume atomically before exposing bytes; concurrent signing must
serialize or use compare-and-swap reservation; crash recovery treats
reserved-unknown slots as consumed; and restoring old backups is unsafe unless
chain-observed slot consumption or hardware counters prevent reuse.

For extraction soundness, the implementation now separates closed plumbing from
the remaining assumption.  The report's
`extraction_soundness_closure_report` states that canonical 41-byte response
parsing, deterministic q-syndrome recomputation, and the quotient verifier
bridge are executable.  The remaining assumption is still knowledge soundness
of the compressed linear-sigma/SIS opening relation, with
`extractionFailure` as the named break event for third-party review.

That assumption is now split one level further.  Lean defines
`FullResponseForkExtractionCertificate`, which is the algebraic fork-extraction
surface for the 73-byte proof.  A fork is two accepting transcripts with the
same proof commitment and the same public opening statement data, except for
the 21-byte challenge.  The certificate records that the relation map is stable
across such forks and that two accepting fork equations with different
challenges extract a response whose q-syndrome is the statement digest, or
produce a named `forkExtractorBreak`.  The executable report mirrors this as
`fork_extraction_review_report`.

This leaves two sharply separated review obligations:

```text
algebraic fork extraction:
  hash the 21-byte challenge to 12 q-residues
  interpret those residues as an element of F_{12289^12}
  prove the pinned degree-12 extension polynomial is irreducible
  bad fork differences are exactly equal Fq12 action vectors, probability
    1 / 12289^12 in the ideal hash-to-Fq12 model
  prove the extracted 41-byte response is canonical or yields the named break

Fiat-Shamir fork generation:
  classical: prove the accepting adversary yields a same-commitment fork with
    the stated query loss
  quantum: choose and account for the QROM measure-and-reprogram/forking lemma
```

So the remaining extraction item is no longer an undifferentiated
knowledge-soundness assumption.  It is the concrete fork extractor plus the
standard Fiat-Shamir fork-generation loss.

The executable `challenge_action_invertibility_report` now pins the intended
challenge action.  It derives a 12-residue action vector from each 21-byte
challenge by domain-separated SHA-256 rejection sampling into `F_12289`; this
vector is the coordinate representation of an extension-field element.  The
extension polynomial is now pinned as `p(X) = X^12 + X + 25`.  The executable
Rabin certificate checks `x^(q^12)-x = 0 mod p` and
`gcd(p, x^(q^6)-x) = gcd(p, x^(q^4)-x) = 1`, proving irreducibility over
`F_12289` for the report.  The bad-difference denominator is `12289^12`, giving
floor `163` bits and a generic quantum margin of `2^17` for 128 bits.

The fork-equation algebra is now executable as
`fork_equation_algebra_report` and surfaced in Lean as
`FullResponseForkEquationAlgebraCertificate`.  The report instantiates the
two accepted equations
`relationMap(statement_i, z_i) = proof_commitment + action_i * digest` in
`F_12289[X]/(X^12 + X + 25)`, subtracts them, solves for the inverse of
`action_0 - action_1`, and verifies that
`(action_0 - action_1)^-1 * (relationMap(z_0) - relationMap(z_1))` recovers
the original 12-residue digest exactly.  This closes the concrete algebraic
division step for non-colliding action forks; remaining extraction work is the
byte-canonical response lift and Fiat-Shamir/QROM fork generation.

The byte-canonical response lift is now split out as well.  Lean defines
`FullResponseCanonicalResponseLiftCertificate`, and the executable
`canonical_response_lift_report` uses the rank-12 right inverse of the
deterministic `12 x 23` matrix to lift a 12-residue digest to a canonical
23-residue, 41-byte response.  The report checks that all lifted residues are
below `q`, the six slack bits are zero, decoding accepts the byte string, the
syndrome equals the input digest, and repacking the syndrome returns the same
digest bytes.  The remaining fork-extraction obligation is therefore the actual
byte-level response arithmetic for subtracting/scaling the two masked
responses, plus Fiat-Shamir/QROM fork generation.

That byte-level response arithmetic is now explicit too.  Lean exposes
`FullResponseByteResponseArithmeticCertificate`, and the executable
`byte_response_arithmetic_report` implements quotient response operations:
decode a 41-byte response to its 12-residue syndrome, perform the `Fq12`
add/subtract/scale operation, then lift back through the canonical response
right inverse.  The report builds two canonical masked responses satisfying the
two fork equations, subtracts them as byte responses, inverse-scales the
difference response, and verifies that the extracted 41-byte response is
canonical and repacks to the original digest.  The remaining extraction work is
now probabilistic fork generation and concrete advantage accounting, not
algebraic response manipulation.

The probabilistic fork-generation budget is now separated from the algebraic
extractor.  Lean records `FullResponseFiatShamirForkAccountingCertificate`, and
the executable `fiat_shamir_fork_accounting_report` charges two independent
bad-event families.  The 21-byte Fiat-Shamir challenge has 162 transcript-bound
bits, so the generic challenge-oracle margin is `2^34` classical or `2^17`
quantum for a 128-bit target.  The derived action collision event is smaller:
`challenge_action_vector(c0) = challenge_action_vector(c1)` has ideal-model
probability `1 / 12289^12`, floor `163` bits, giving `2^35` classical or
`2^17` generic quantum query margin for a strict 128-bit target.  Thus the
remaining review target is not hidden algebra; it is the same-commitment
classical/QROM fork lemma plus this explicit action-collision bad event.  The
hardened `d = 12` profile makes the action field no longer worse than the
21-byte transcript challenge under the generic quantum accounting model.

The deployment ledger now selects option 2 explicitly: a QROM
measure-and-reprogram theorem boundary.  The theorem family is the
Don-Fehr-Majenz-Schaffner Fiat-Shamir-in-the-QROM result and the cleaner
Don-Fehr-Majenz measure-and-reprogram 2.0 single-input statement, which uses a
quadratic `(2q + 1)^2` loss for `q` quantum random-oracle queries.  The local
instantiation requires exactly these protocol-shape facts:

```text
measured oracle input:
  the prechallenge frame containing public key, message, slot id, slot digest,
  proof commitment, and domain labels

reprogrammed oracle output:
  the 21-byte Fiat-Shamir challenge

same-commitment condition:
  the proof commitment is fixed before the measured challenge query, and the
  signature response/opening bytes are excluded from the challenge frame
```

Internal audit update: the concrete proof frame now matches this target shape.
The executable `challenge_hash_frame` hashes the public key, message, slot id,
slot digest, and the 32-byte quotient proof commitment under the
domain-separated profile/parameter frame.  The executable report now marks
`fiat_shamir_fork_generation_ledger.qrom_theorem_fits_protocol_shape = true`.
The legacy 181-byte ideal verifier still uses a pre-proof challenge frame
because that payload carries no proof commitment; the deployed 254-byte public
verifier must use the proof-bound frame.

Sources:

```text
DFMS19:
  https://arxiv.org/abs/1902.07556

Measure-and-reprogram 2.0:
  https://eprint.iacr.org/2020/282.pdf
```

The deployment ledger also separates single-term margins from the multi-term
union-bound budget.  The executable
`fiat_shamir_fork_generation_ledger` and Lean
`FullResponseFiatShamirForkGenerationLedger` record:

```text
single bad-event term:
  quantum query budget for 128 bits:     2^17
  classical query budget for 128 bits:   2^34

deployment union ledger:
  quantum query budget used:             2^16
  classical query budget used:           2^33
  transcript term at quantum budget:     130 bits
  Fq12 action-collision term at budget:  131 bits
```

That one-bit reserve avoids pretending that several independent bad-event
terms can each sit exactly at `2^-128` and still compose to a `2^-128` total
failure bound.  The same-commitment classical/QROM fork lemma remains an
explicit theorem/assumption slot, but its hash/action bad-event ledger is now
byte-concrete and union-bound safe.

The QROM fork is now connected to the byte extractor rather than only sitting
beside it.  Lean defines `FullResponseQROMSameCommitmentFork`, whose two
branches share the proof commitment and prechallenge statement frame, and
`FullResponseQROMExtractionBranchTheorem`, which proves the exact branch:

```text
same-commitment QROM fork
  + non-colliding Fq12 challenge actions
  + byte-response arithmetic certificate
    => canonical 41-byte response opening the 12-residue quotient digest

otherwise:
  action-collision break, extraction break, matrix mismatch, frame mismatch,
  or the named QROM/fork-generation bad event is charged.
```

The executable `final_soundness_advantage_formula` records the current
top-level bound:

```text
Adv_sound <= Adv_QROM_measure_reprogram
          + Pr[Fq12 action collision]
          + Adv_extraction_break
          + Adv_matrix_mismatch
          + Adv_pad_prf
          + Adv_frame_mismatch
```

At the deployment `q <= 2^16` quantum hash-query ledger, the transcript term
has `130` bits and the action-collision term has `131` bits before adding the
remaining primitive and extraction assumptions.

The quotient-hiding review is now explicit as well:
`quotient_hiding_sampler_review` says the game-specific per-slot total
variation distance is `0` if production signing samples uniform kernel
coefficients and rejects only on canonical packing, fixed fiber-state equality,
and response bounds.  The exact `133`-bit support floor remains an audit metric
for sampler population/collision/retry behavior, not the hiding advantage.

The third-party review packet is now updated around the active account-carried
committed-sigma target.  It lists the `250 B` spend transaction, `68 B`
account-link state, `318 B` online-plus-account accounting, the response-link
certificate replay command, expected hashes, external size benchmark artifact,
the reference timing smoke benchmark, the production-reference gap report, and
the legacy `254 B` public verifier as a negative control.  The
human-readable entry point is:

```text
research/cryptanalysis/signature_readiness_20260516/third_party_review_packet.md
```

The executable `third_party_review_packet` section in the Rust reference still
tracks the legacy public quotient verifier and should be read as regression and
negative-control coverage until the production account-carried reference
implementation is split out.  The internal pre-review audit is:

```text
research/cryptanalysis/signature_readiness_20260516/internal_security_review_audit.md
```

It records that the proof-commitment frame blocker and representative public
verifier byte-path blocker are closed at the executable reference level.  The
254-byte representative path now verifies the proof-bound challenge frame,
q-syndrome digest match, local opening relation, proof-commitment mutation
rejection, masked proof response mutation rejection, and consumed-slot replay.
The strict linear sigma-equation verifier is also implemented and checks
`L(z) = a + c * digest`, but the current representative-style signer does not
yet produce a sound deployed relation.

The packed-SIS/keygen review also found a critical issue:
`public_proof_forgery_probe` constructs an accepting 254-byte signature from
public key bytes, public slot digest, slot id, and message, without private key
material or account hidden witness material under the representative verifier.
The strict sigma-equation verifier rejects that exact forgery with
`linear sigma verifier equation mismatch`.  However,
`public_sigma_equation_forgery_probe` adapts the attack: it chooses the public
proof commitment first, derives the Fiat-Shamir challenge, solves the public
post-challenge equation `L(z) = a + c * digest`, and then chooses an accepted
local representative.  That adapted transcript verifies under the strict
sigma-equation path without private key material.  This means the next design
step is not just increasing lattice margin or adding the equation; acceptance
must be tied to a prechallenge witness-bound sigma commitment or another
hidden-witness proof so public post-challenge representative reconstruction is
insufficient.

The committed-sigma byte frontier is now formalized in Lean as
`N32Q12289CommittedSigmaByteFrontier`.  The key byte result is that keeping the
current 41-byte masked response gives a 254-byte transaction before any
hidden-link proof bytes, so a sub-256 design has room for at most 2 additional
bytes.  That is not a credible proof budget.  The low-byte path therefore has to
shift structure into the fiber-saturated quotient: a 37-byte response gives a
250-byte transaction before the hidden-link layer and leaves 6 bytes below 256;
a 39-byte fiber-state-preserving response gives 252 bytes and leaves 4 bytes.
The follow-on object, `AccountCarriedHiddenLinkInstantiation`, now makes the
stateful tradeoff explicit: the preferred target is a 250-byte transaction with
zero extra hidden-link bytes in the signature and the link evidence carried in
the consumed account slot.  This does not prove the link mechanism; it pins the
exact object that must instantiate `openingInterfaceHolds`,
`postChallengeRepresentativeRejected`, and `witnessHidingArgument`.
The verifier-facing version is now `AccountCarriedHiddenLinkInterface`: an
accepted account-carried transcript must be backed by a single hidden witness
and prechallenge randomizer explaining `statementDigest`, `witnessCommitment`,
`randomizerCommitment`, `hiddenLinkCommitment`, `proofCommitment`, and
`response`, unless `accountLinkProofBreak` occurs.
The contrapositive is now explicit: an accepting unbacked public representative
forces `accountLinkProofBreak`.  This also rules out a bare account hash/link as
a proof strategy; storing link bytes in account state is useful only if the
serialized opening predicate actually proves the response is tied to those bytes.
The account-carried path is now packaged as `AccountCarriedSoundnessCertificate`:
accepted account-carried verifier transcripts are account-backed or trigger a
named `SoundnessBreaks.any` event.
The opening layer is now split into `AccountCarriedOpening.localOpening` and
`AccountCarriedOpening.linkProof`.  `AccountLinkProofSystem` is the adapter for
candidate constructions: local opening acceptance plus account-link proof
acceptance must imply `AccountBackedByHiddenWitness` or `accountLinkProofBreak`.
The candidate frontier is also formalized: reveal-based links, local-opening-only
links, and bare account hashes are structurally rejected.  The live class is
`accountCarriedLinkProofCandidate`: zero link-proof bytes in the signature, with
the remaining link evidence carried by account state and a proof obligation to
instantiate `AccountLinkProofSystem`.
The slot-bound first-message invariant is now separated from that proof:
`SlotBindsProofCommitment` is necessary, but the
`slot-bound-first-message-only` candidate is rejected because binding `a` does
not by itself bind the post-challenge response to hidden `w,r`.
The first concrete zero-signature-byte live construction class is now
`HomomorphicAccountLinkInterface`: the verifier checks
`Commit(response) = Commit(r) + challenge * Commit(w)` against account-stored
commitments.  This converts to `AccountLinkProofSystem` with `LinkProof = Unit`;
the remaining hard assumption is `linkEquationSoundness`.  The degenerate
public-projection version is explicitly rejected: Lean theorem
`linkProofAccepts_of_publicProjection` shows that if those commitments are only
homomorphic projections of the public sigma equation, the link check adds no
security beyond `L(response) = proofCommitment + challenge * statementDigest`.
This is now also a structural gate on `LinkProofInstantiationCandidate`, not
only a prose warning.  The homomorphic proof obligation is now further split
into `AccountBackedExceptResponse` plus `ResponseLinkSoundness`: first bind the
account state/opening to the same hidden `w,r`, then prove the deterministic
link equation forces `response = r + challenge*w` or a link-kernel break.  Lean
now has `BoundedResponseLinkKernelCertificate.responseLinkSoundness`, which
turns a bounded joint-kernel certificate plus response-code equality into
`ResponseLinkSoundness`.  The executable report now records the first
response-link code frontier: an
11-residue/20-byte coordinate code kills the full public quotient kernel, but
is not a public-account solution because `L(w)` plus that code reveals `w`.
The viable target is therefore a smaller bounded-box kernel certificate, not a
globally injective public code.  The current `[0, 3]` code row is 4 bytes,
leaves a 9-dimensional algebraic kernel for hiding, and the LLL probe finds
shortest intersection coefficient bound 139 against the response-difference
bound 96.  The repo now includes
`scripts/research/response_link_bounded_kernel_check.py` to reproduce the LLL
evidence, run exact `z3`, `cp-sat`, or `scip` checks, and run both float-guided
and exact-rational bounded-box enumeration.  The current exact-rational
enumeration artifact checks the whole Euclidean cover ball for the `[0, 3]`,
bound-96 target and finds no nonzero bounded vector; `--verify-json` replays it
with no mismatched certificate fields.  The main remaining proof obligation is
independent audit replay of that certificate, not an unresolved search for a
bounded kernel vector.

Important deployment finding: the reference signer is not production
constant-time.  This is now an explicit review-packet limitation and deployment
blocker for wallet code, not an implicit assumption.

There is also a hard boundary:
`noInformationTheoreticBinding_34_to_32` proves that a 32-byte public value
cannot be injective over all 34-byte hidden preimages.  So this route cannot use
a literal information-theoretic `noCollision` assumption.  It needs either a
computational algebraic binding assumption, a restricted witness domain, or
more account-state bytes.

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
