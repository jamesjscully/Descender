# Path2AccountStarkSimulatorNoExportTheoremV1

> Classification: `ACTIVE_SUPPORT`. This theorem supports the active
> Poseidon2/Plonky3 account-creation path (`ADR-0004`, tracker rows
> `DIR-PAYLOAD-001`, `DIR-CREATE-001`, `DIR-PROOF-001`). It is the simulator /
> no-export proof for `Path2AccountStarkCreationPayloadV1`, supporting the
> `Path2AccountCreationNoExportBoundaryReportV1` target in
> `research/path2_fiber_privacy_no_export_v1.md`. Not normative by itself.

Status: zero-knowledge simulator and no-export theorem for the selected
account-creation public bundle.

## Obligation And Scope

Residual matrix row (`spec/security_model.md`, "Zero-knowledge simulator and
no-export statement"): *"Simulator theorem for the selected payload proves the
accepted public bundle can be simulated from public anchors and allowed
commitments without revealing private creation material."* Named break:
`account_stark_simulator_or_no_export_break`.

This proves that obligation for the ACTIVE account-STARK payload. It is distinct
from — and does not inherit anything from — the retired non-active paths:

- the `Path2MaskedState18CreationProofV1` data-availability proof (serialized
  `pad_seed` in clear; NON-HIDING BY DESIGN; non-active);
- the transparent-descent ZK with the in-clear additive blind (HIGH-2; retired,
  `research/cryptanalysis/high2_zk_inclear_blind_leak_VERDICT.md`).

The active path never serializes `pad_seed`: the private witness (pad seed,
account projection, masks, opening-tuple preimage, and authorization response)
lives ONLY in the
**salted, hidden zk-FRI trace** committed under `MerkleTreeHidingMmcs`, never on
the public surface. That is the structural reason the active path discharges
HIGH-2 rather than patching it.

## The Public Bundle And The Simulator Allowlist

`Path2AccountStarkCreationPayloadV1`'s consensus-visible surface, and the only
inputs the simulator may consume (`simulator_public_input_fields` allowlist,
`research/path2_fiber_privacy_no_export_v1.md`):

```text
leaf_context_digest, creation_proof_binding_digest, native_role_root_commitment,
aggregation_context_digest, slot_id, masked_projection,
account_stark_public_statement_bytes_or_digest, pad_seed_nullifier,
deployed_account_id, deployed_account_state_root, deployed_lifecycle_anchor,
deployed_verifier_version, deployed_packed_witness_commitment, public_rejection_code
```

The completed account-STARK proof bundle and its outer
`public_replay_payload_digest` are outputs of simulation and are excluded from
the input allowlist. The public-statement transport contains the decoded
`AccountCreationPublic` statement, including its public commitments and
nullifier, but no proof bytes. This avoids satisfying the theorem by copying the
real proof into the simulated output.

Forbidden (denylist): `pad_seed, account_projection,
opening_tuple_preimage, authorization_witness, mask, private_trace_material,
commitment_nonce`.

The decoded `AccountCreationPublic` statement is public by design and includes
the checked-core `syndrome`. That syndrome is an authorization/public-key
anchor, not a private preimage. The nested statement audit covers all 14 fields
inside the transport bundle instead of treating the bundle as privacy-opaque.

## ZK Parameters Of The Deployed Proof System

From `tools/descender-account-stark/src/config.rs`:

```text
ZkPcs = HidingFriPcs<...>                       (config.rs ZkPcs alias)
HidingValMmcs = MerkleTreeHidingMmcs<..., SALT_ELEMS=4>   (salted leaves)
NUM_RANDOM_CODEWORDS = 4                          (config.rs:90)
PROD_LOG_BLOWUP = 4  (rate 1/16; ZK requires log_blowup >= 2)   (config.rs:51)
ZkCryptoRng = cloneable ChaCha20 CSPRNG with a 256-bit master seed
```

The deployed PCS therefore activates Plonky3's intended hiding mechanisms:
salted Merkle leaves, four random codewords mixed into the
low-degree-extension batch, and blowup ≥ 2. Configuration is not an HVZK proof;
the concrete A1 theorem and advantage bound remain required.
The input MMCS, FRI MMCS, and HidingFriPcs randomness are driven by three
SHA-256 domain-separated sub-seeds (`input-mmcs`, `fri-mmcs`, and
`hiding-fri-pcs`). The production path obtains a fresh 256-bit master seed from
the OS-seeded thread CSPRNG. Deterministic `u64` seeds remain confined to the
explicit test/KAT entry points and are domain-expanded before use.

## Main Theorem

> **Theorem (account-STARK public-bundle simulator + no-export).** There is a
> PPT simulator `S` that, on input only the allowlist fields above (no private
> witness), outputs a bundle `S(pub)` such that for every honest prover producing
> a real accepted bundle `Real(pub, wit)`,
>
> ```text
>     Real(pub, wit)  ≈_c  S(pub)
> ```
>
> (computational indistinguishability), under the named assumptions A1–A3 below.
> Consequently the public bundle is a function of public data up to A1–A3, so it
> exports no private fiber material (`pad_seed`, `account_projection`, `masks`,
> `opening_tuple`, syndrome preimage, gauge, randomizer) beyond the public class
> — `fiber_privacy_no_export` holds modulo A1–A3 and the implementation
> no-export obligation (§ Implementation Residual), or the adversary exports
> `account_stark_simulator_or_no_export_break`.

### Named assumptions

- **A1 (zk-FRI HVZK):** `HidingFriPcs` with `MerkleTreeHidingMmcs` (SALT_ELEMS=4),
  `NUM_RANDOM_CODEWORDS=4`, blowup ≥ 2 is honest-verifier zero-knowledge — a
  simulator produces proof bytes indistinguishable from real given only the
  public statement. (Named-standard zk-STARK; same primitive family as the #2
  extractor's FRI soundness, opposite direction.)
- **A2 (Poseidon2 mask-frame-PRF):** `mask[j] = reduce(Poseidon2-sponge(MASK_DOMAIN
  ‖ pad_seed ‖ slot_id))[j]` is pseudorandom and fresh per admitted C2 frame, so
  `masked_projection[j] = (account_projection[j] + mask[j]) mod q` is
  computationally indistinguishable from uniform over `[0,q)`. (Active-path
  Poseidon2 analogue of the SHA-256 mask-PRF in
  `research/cryptanalysis/deployable_pad_hiding_VERDICT.md` CLAIM 2; requires
  proof-bound `(pad_seed, slot_id)` single-use, A2′.) The selected C2 frame now
  absorbs the explicit `MASK = 0x4D41534B` field lane, then 32 pad-seed bytes and
  8 little-endian slot bytes. It squeezes 36 disjoint KoalaBear coordinates,
  six per each of six q12289 masks. Five digits only supplied about 141 bits per
  mask and fell to about 106 bits after the selected `2^32` target horizon;
  six digits leave more than 137 bits after that full charge.
  `selectedAccountMaskPrfDomainsSeparated_v1`,
  `selectedAccountMaskPrfSqueezeIndexInjective_v1`, and
  `selectedAccountMaskWideReductionError_le_q128_v1` machine-check the frame,
  coordinate separation, and aggregate reduction arithmetic. The sovereign
  verified-nullifier registry supplies admitted-frame freshness through
  `selectedRustAccountMaskPrfFramesFresh_of_verifiedNullifierRegistry_v1`.
  Once the mask vector is uniform,
  `selectedRustUniformMaskTranslationBijective_v1` machine-checks that adding
  any fixed six-lane account projection in `(ZMod 12289)^6` is a bijection, so
  the modular masking algebra adds no further loss. The remaining A2 premise is
  the selected bounded-query Poseidon2 sponge/permutation PRF theorem and review;
  frame freshness and deployment-scale reduction bias are no longer bundled
  into that premise.
- **A3 (Poseidon2 commitment hiding / QROM):** C1 now absorbs `PADS ‖
  pad_seed[32] ‖ nonce[40]`; C6 absorbs `OPEN ‖ account_projection[6] ‖
  lifecycle[32] ‖ nonce[40] ‖ account_id[32]`. The deployment single/batch APIs
  overwrite both nonces with fresh OS-CSPRNG bytes. The former 16-byte nonces
  could not meet q128 under the selected `2^54` query and `2^32` account budgets.
  For two commitments per account, the v3 nonce-search term is bounded by
  `2^(2*54+33-320) = 2^-179`.
  `selectedAccountCommitmentDomainsSeparated_v1`,
  `selectedAccountCommitmentFrameDimensions_v1`, and
  `selectedAccountCommitmentQromSearchError_le_q128_v1` machine-check the local
  frame and search arithmetic. The remaining premise is a reviewed bounded-query
  Poseidon2 commitment/QROM replacement theorem. `role_root` is deterministic
  compression of already-public commitments and `masked_projection`, not a third
  hiding commitment. `packed_witness_commitment`, the checked-core syndrome,
  masked projection, and checked-core key commitment are allowed public anchors;
  the packed witness is reversible framing, not a hiding commitment.

### Proof (component simulation + hybrid)

`S(pub)` builds the bundle component-by-component:

1. **Public statement fields** — `deployed_account_id`, `slot_id`,
   `deployed_account_state_root`, `deployed_verifier_version`,
   `deployed_lifecycle_anchor`, `pad_seed_nullifier`,
   `native_role_root_commitment`, `deployed_packed_witness_commitment`,
   `account_stark_public_statement_bytes_or_digest`, and the remaining
   statement digests are copied verbatim from `pub`. These ARE the public
   statement; no private content (L6/L7/L8 of the arithmetization theorem
   already showed they bind to public values). Identical distribution.
2. **`masked_projection[6]`** — `S` samples 6 uniform residues in `[0,q)`. By A2,
   indistinguishable from the real masked carrier. This is the only projection
   data on the creation surface; the unmasked `account_projection` is NOT on the
   surface (it is recovered only by the separate post-spend charged opening, out
   of scope here).
3. **Proof bytes (`account_stark_bundle`)** — `S` runs the A1 zk-FRI HVZK
   simulator on the public statement (fields from steps 1–2) to produce proof
   bytes, constructs the completed payload, and then derives the outer
   `public_replay_payload_digest`. Indistinguishable by A1; neither output is an
   input to `S`.
4. **Commitments** — taken as the public deployed anchors (step 1) by A3; `S`
   does not need preimages.

Hybridize: `H0 = Real`; `H1` replaces real proof bytes with the A1 simulation
(indistinguishable by A1); `H2 = H1` replaces `masked_projection` with uniform
(indistinguishable by A2); `H2 = S(pub)`. Each hop is computationally
indistinguishable, so `Real ≈_c S(pub)`. ∎

The additive composition of these hops is machine-checked in Lean by
`SelectedAccountStarkSimulatorHybridBounds.advantage_le_totalError`, and
`selectedRustAcceptedBundleSimulatorNoExportFromHybrid_v1` imports concrete
per-package A1/A2/A3 certificates into the accepted-bundle no-export theorem.
The A2 frame, squeeze-coordinate separation, `2^32`-target wide-reduction budget,
verified-nullifier freshness reduction, and uniform-mask translation are
separately discharged by the named Lean theorems above. The A3 PADS/OPEN frame,
320-bit deployment nonces, and `2^-179` search term are likewise discharged
locally. These theorems do not
manufacture the cryptographic hop bounds: the selected HidingFriPcs HVZK,
bounded-query Poseidon2 sponge/permutation PRF, and Poseidon2 commitment/QROM
replacement bounds plus external review remain open.

### No-export corollary

`S(pub)` is computed from public data alone, yet `≈_c Real(pub, wit)`. Hence any
PPT distinguisher extracting a private fiber field from the real bundle would
distinguish `Real` from `S(pub)`, contradicting the theorem. So the bundle
reveals nothing about `pad_seed / account_projection / masks /
opening_tuple_preimage / authorization_witness / commitment_nonce / gauge /
randomizer` beyond the explicitly public statement — the
`forbidden private-material` denylist is upheld at the byte/algebra layer. ∎

## Adversarial Self-Check

- **"Syndrome leaks the witness."** The decoded creation statement carries the
  checked-core `syndrome` directly and in the reversible public
  `packed_witness_commitment = pack(syndrome ‖ masked_projection)`. It is an
  allowed public authorization anchor, not the private signing response;
  `masked_projection` is the A2 hiding surface. No unmasked account projection
  is on the creation surface. The post-spend charged opening that discloses the
  six selected residues is a different, accounted surface
  (`deployable_pad_hiding_VERDICT.md` CLAIM 4, two-phase budget) — not this
  bundle.
- **"`pad_seed` is in the proof."** No. It is a private trace column under the
  salted hiding MMCS (A1); the public bundle has no `pad_seed` field. This is the
  structural HIGH-2 discharge.
- **"Mask not fresh ⇒ masked_projection correlates."** True risk; charged as A2′
  (pad-seed single-use). If a pad seed repeats across slots the mask repeats and
  A2 fails — so A2 is conditioned on the lifecycle uniqueness owned by
  `DIR-LIFE-001` (#9) and the `pad_seed_digest` admission-uniqueness clause
  (`R_namespace`).
- **Honest residual — NOT closed by algebra.** A simulator theorem bounds what
  the *bytes* reveal; it does NOT bound accidental export via reject-path
  diagnostics or helper APIs. That is an implementation obligation (below).
  The selected Rust public-bundle denylist predicate is now machine-checked by
  `selectedRustAccountCreationPublicBundleNoExport_instantiation_v1`, and the
  selected Rust simulator-input allowlist predicate is machine-checked by
  `selectedRustAccountCreationSimulatorInputNoExport_instantiation_v1`. The
  selected Rust accepted public-anchor package binding predicate is
  machine-checked by
  `selectedRustAcceptedPublicAnchorPackageNoExport_instantiation_v1`. The
  selected Rust accepted-bundle public-anchor simulation-boundary predicate is
  machine-checked by
  `selectedRustAcceptedBundlePublicAnchorSimulationBoundary_v1`, the typed
  public-bundle projection from accepted public anchors is machine-checked by
  `selectedRustAcceptedBundlePublicBundleFromPublicAnchors_v1`, and the typed
  simulator-input projection from accepted public anchors is machine-checked by
  `selectedRustAcceptedBundleSimulatorInputFromPublicAnchors_v1`. The selected
  Rust accepted-bundle simulator/no-export reduction is machine-checked by
  `selectedRustAcceptedBundleSimulatorNoExportReduction_v1`, while full
  public-anchor simulator instantiation and external review remain.

## Implementation Residual (test surface, not algebra)

Per `research/path2_fiber_privacy_no_export_v1.md` § Practical Test Surface, the
following must hold in code and stay regression-tested (owned by
`DIR-REJECT-001`/`DIR-PAYLOAD-001`), and are NOT discharged by this theorem:

- reject paths emit only `{reject_code, public field, public offset, named
  break}` — never expected/actual private values, decoded lanes, raw mask/seed;
- no public helper/serializer returns authorization-local, gauge, randomizer, or
  raw hidden-lift material;
- frozen no-export vectors scan all public outputs for raw seed/opening/lane/
  mask/randomizer bytes and remain digest-pinned.

## What This Closes And Hands Off

- **Closes (algebra layer):** the simulator theorem for
  `Path2AccountStarkCreationPayloadV1` — the accepted public bundle is
  simulatable from the public allowlist, reducing no-export to A1–A3.
- **Hands off:** A2′ pad-seed single-use (#9, `DIR-LIFE-001`); the
  implementation no-export scan (#8, `DIR-REJECT-001`/`DIR-PAYLOAD-001`); the
  full public-anchor simulator/no-export predicate; external review. The frozen
  vector corpus, selected Rust public-bundle denylist predicate, and selected
  Rust simulator-input allowlist predicate, plus the selected Rust accepted
  public-anchor package binding predicate and accepted-bundle public-anchor
  simulation-boundary predicate plus the accepted-bundle public-bundle
  projection, simulator-input projection, and simulator/no-export reduction
  predicates, are now closed separately;
  `Path2AccountCreationNoExportBoundaryReportV1` keeps its
  simulator/no-export/deployment flags false pending those.
