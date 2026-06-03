# Path 2 Real Commitment Options

Status: design gate for replacing the toy q-linear payload trial.

## Decision Context

Path 2 now has a proof-friendly payload boundary and a structural q-linear trial.
The remaining problem is choosing a real commitment/proof family for the
account-creation hiding proof.

The chosen system must preserve these constraints:

- spend signature remains 252 bytes;
- extra proof bytes live at account creation/admission, not spend;
- validators can independently check the proof in permissionless mode;
- public bytes do not reveal raw pad seed, projection masks, unmasked account
  projection residues, or private opening frames;
- `Path2CreationProofBindingV1` remains the binding surface;
- optional aggregation can be added later without changing the single-slot
  theorem;
- if Descender is claimed as post-quantum, the proof payload assumptions must
  also be post-quantum.

The last point is the main filtering rule. A non-PQ account-creation proof would
let a quantum adversary attack slot admission even if the checked-core signature
remains lattice/PQ.

## Option A: Pedersen Or IPA Over A Prime-Order Group

This is the closest production analogue to the toy q-linear verifier:

```text
C_pad = Commit(pad_lanes; r_pad)
C_proj = Commit(projection_lanes; r_proj)
prove masked_projection = pad_lanes + projection_lanes
```

### Advantages

- Very clean continuation of the toy q-linear proof.
- Small public commitments and compact linear-relation proofs.
- Fast validator checks compared with generic SNARK/STARK routes.
- Easy to reason about aggregation later, because vector commitments and IPA
  proofs compose naturally.

### Problems

- Standard Pedersen/IPA commitments rely on discrete-log hardness.
- That is not post-quantum.
- Using this for production would change the security story from PQ signature
  security to hybrid/PQ-plus-classical admission security.

### Verdict

Good engineering prototype. Poor production fit if the deployable claim is
post-quantum. Keep it only as a regression/prototyping target unless the theorem
explicitly switches to a hybrid assumption.

## Option B: Pairing Or IPA SNARK

Use a general-purpose succinct proof system for the algebraic Path 2 relation.
The proof opens proof-friendly commitments and proves the q12289 relation inside
a circuit or arithmetization.

### Advantages

- Small proofs and verifier time can be excellent.
- Existing ecosystems provide mature proving APIs.
- The proof can absorb many relation details, including range checks and
  binding checks.

### Problems

- Pairing SNARKs are not post-quantum.
- IPA/KZG-style polynomial commitments are also non-PQ in the usual groups.
- Trusted setup or structured reference string questions may add governance
  burden.
- If the circuit reintroduces SHA-256 preimages, we are back at the bottleneck
  identified in the cost model.

### Verdict

Attractive for a non-PQ or hybrid prototype. Not the default deployable path for
a PQ blockchain signature.

## Option C: Hash-Based STARK Or Transparent IOP

Use a transparent proof system whose soundness rests on collision-resistant
hashes plus low-degree testing or related IOP assumptions.

### Advantages

- Plausibly post-quantum under hash/RO assumptions.
- Transparent setup is operationally attractive for a blockchain.
- Aggregation/rollup integration is conceptually natural.
- Can prove the complete account-creation relation without relying on
  discrete-log commitments.

### Problems

- Account-creation proof bytes are likely much larger than group-based proofs.
- Validator cost may be acceptable for account creation but is not free.
- If the relation includes SHA-256-heavy derivations, proof generation remains
  expensive unless we keep the proof-friendly inner commitment redesign.
- Formalizing the exact proof-system assumption is a larger theorem task than a
  custom linear Sigma protocol.

### Verdict

Best general-purpose PQ-compatible fallback if proof bytes in the KB-to-tens-of-
KB range are acceptable at account creation. It is likely too heavy to be the
first thing to hand-code, but it is the safest broad category for an end-to-end
PQ claim.

## Option D: Lattice/SIS Commitments With A Linear ZK Proof

Replace the toy q commitments with commitments based on SIS/module-SIS or a
closely related lattice assumption. Prove the q12289 linear relation with a
lattice-compatible Sigma protocol or Fiat-Shamir proof.

### Advantages

- Post-quantum assumption family matches the signature direction.
- q12289 arithmetic is native rather than awkward.
- The proof can stay focused on the small relation instead of a full generic
  circuit.
- The theorem boundary is crisp: binding from SIS/module-SIS, hiding from
  randomness/noise distribution, soundness from the proof of knowledge.
- More compatible with future lattice-native aggregation than group Pedersen.

### Problems

- Proof bytes may be substantially larger than group Pedersen/IPA.
- Careful parameter selection is nontrivial: dimensions, norm bounds,
  rejection sampling, soundness slack, and zero-knowledge leakage must all be
  named.
- Validator time is mostly matrix/vector arithmetic over q; probably acceptable
  at account creation, but must be measured.
- Implementing this correctly is more cryptographic engineering work than the
  toy q-linear verifier.

### Verdict

Most plausible production path if we require end-to-end post-quantum security
and want to avoid generic STARK size. This should be the next primary
investigation target.

## Option E: MPC-In-The-Head / VOLE-In-The-Head Proof

Use a post-quantum zero-knowledge proof family based on symmetric primitives or
correlation checks to prove the account-creation relation.

### Advantages

- Can be post-quantum under symmetric/hash assumptions.
- Avoids discrete-log commitments.
- Often practical for moderate-size circuits.

### Problems

- Proof sizes are typically larger than group-based proofs.
- The relation must be represented as a circuit, so SHA-heavy parts must still
  be avoided or paid for.
- Aggregation story is less clean than STARK or commitment-native approaches.
- More tooling dependency and less direct alignment with the current q-linear
  trial.

### Verdict

Reasonable fallback if lattice/SIS proof design gets stuck and STARK overhead is
too high. Not the most direct continuation of the current model.

## Option F: Keep Existing SHA Commitments And Prove The Hashes

Use a generic PQ proof system but prove the current SHA-shaped commitments
directly.

### Advantages

- Minimal state schema redesign.
- Simple semantic story: existing digest fields keep their current meaning.

### Problems

- Already identified as SHA-circuit dominated.
- The lower-bound route needs about 23 SHA-256 compressions in the proof; the
  fuller route needs about 55.
- It spends proof complexity on compatibility rather than the actual q-linear
  relation.

### Verdict

Keep only as a baseline and negative-control option. It should not be the main
path unless all proof-friendly commitment paths fail.

## Option G: Public Opening Payload

Publish enough private opening material for validators to replay the relation.

### Advantages

- Small theorem burden.
- Easy validator implementation.
- No new proof-system dependency.

### Problems

- Changes the leakage theorem.
- Likely breaks or substantially weakens the pre-spend hiding claim.
- Conflicts with the reason Path 2 moved to a hiding proof payload.

### Verdict

Dead for the current deployable target unless we deliberately switch to a
public-creation-opening theorem.

## Ranking

| rank | option | production suitability | reason |
|---:|---|---|---|
| 1 | descent-derived lattice/SIS commitments with linear ZK proof | best primary path | PQ-compatible, aligned with q12289 relation, and matches the existing Descender descent theorem shape |
| 2 | hash-based STARK/transparent IOP | best broad fallback | PQ-compatible, larger but conceptually robust |
| 3 | MPC/VOLE-in-the-head | fallback | PQ-compatible but less aligned with q-linear commitment boundary |
| 4 | Pedersen/IPA group commitments | prototype only | compact but not PQ |
| 5 | pairing/IPA SNARK | prototype/hybrid only | compact but not PQ and setup assumptions may be awkward |
| 6 | prove existing SHA commitments | baseline only | known SHA-circuit bottleneck |
| 7 | public opening payload | dead for current theorem | changes leakage mode |

## Recommended Next Step

Do not choose a final proof system yet. First run a concrete descent/SIS
feasibility pass, using `research/path2_descent_sis_commitment_viability.md` as
the target:

1. Define the exact committed vector:
   `pad_lanes`, `projection_lanes`, private opening tuple scalars, blindings,
   and any range/norm witnesses.
2. Define a candidate module-SIS commitment equation and dimensions.
3. Estimate proof bytes for one slot under conservative soundness repetition or
   Fiat-Shamir challenge size.
4. Estimate validator q-ops and memory.
5. Check whether the proof can express:
   `masked_projection_i = pad_lane_i + account_projection_i mod q`
   without revealing either side.
6. Add a dead-path marker if the byte or verification budget is worse than a
   transparent STARK fallback.

The decision gate should compare only two serious PQ candidates next:

- descent-derived lattice/SIS linear proof;
- transparent hash-based proof over the proof-friendly relation.

Group Pedersen/IPA can remain useful for testing the algebraic API, but should
not be allowed to satisfy production preflight.

## Descent/SIS Feasibility Progress

The first concrete pass now exists:

```text
scripts/research/path2_descent_sis_feasibility.py
research/path2_descent_sis_feasibility.md
research/path2_descent_sis_feasibility.json
```

It fixes the initial public quotient, committed role vectors, and lower-bound
byte/q-op rows for `Path2DescentSisCommitmentV1`.

Current interpretation:

- 32-row commitments are wiring-only and too small for any security claim.
- 48-row and 64-row commitments are the first feasibility band to analyze.
- 96-row commitments are a fallback band and should be compared against
  transparent proof overhead before being promoted.
- Spend signature bytes remain unchanged.
- V2 state leaf cost remains the already-modeled +32-byte
  `creation_proof_binding_digest`.

This does not close the commitment choice. It turns the next question into a
specific cryptanalytic/proof obligation: whether the 48-row or 64-row band can
support binding, hiding, norm/range, and extraction with acceptable payload
bytes.
