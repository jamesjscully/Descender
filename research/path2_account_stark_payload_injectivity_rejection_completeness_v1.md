# Path2AccountStarkPayloadInjectivityRejectionCompletenessV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Poseidon2/Plonky3 path
> (`ADR-0004`, tracker rows `DIR-STARK-001`, `DIR-REJECT-001`,
> `spec/conformance_kats.md`, `spec/consensus_verifier.md`). It closes the
> proof-relevant slice of the frozen-payload/rejection obligation (parser
> injectivity + rejection-class completeness) and scopes the implementation
> residual (frozen byte-for-byte vectors). Not normative by itself.

Status: payload byte-model injectivity and rejection-class completeness.

## Obligation and the proof/consensus split

`DIR-STARK-001` needs "complete frozen byte-format KATs"; `DIR-REJECT-001` needs
"panic-free parse/reject tests for every public object, state-no-mutation checks
for every direct-mode rejection, and target-chain consensus-client failure
mapping." Per the risk ordering, this is **lower proof-collapse risk, higher
consensus risk**: missing malformed-payload coverage usually causes client
divergence or parser ambiguity, not collapse of the mathematical proof — *unless*
the ambiguity changes the proved statement. So the obligation factors:

```text
#8 = PARSER-INJECTIVITY (proof-relevant: byte model can't change the proved statement)
   ∧ REJECTION-COMPLETENESS (consensus-relevant: every malformed payload → one public class)
   ∧ FROZEN-CORPUS (implementation: byte-for-byte reproducible vectors from the real prover)
```

This document closes the first at paper level, specifies the second as a
completeness matrix, and scopes the third as slow-gate implementation work.

## Lemma I — Parser injectivity (the proof-relevant guard)

> **Lemma I.** The canonical `Path2AccountStarkCreationPayloadV1` frame is an
> injective encoding: distinct accepted public statements have distinct canonical
> byte encodings, and every accepted byte string decodes to exactly one public
> statement. Hence no parser ambiguity can make a single proof authorize two
> different proved statements.

**Proof.** `Path2AccountStarkPayloadByteModelReportV1` pins a *canonical* frame:
fixed field order, fixed overhead, fixed verifier version, fixed digest function,
non-empty bundle rule, and non-inlined proof-byte policy (`DIR-STARK-001`). Each
public field has a fixed offset and length (the verifier decodes by fixed-width
slices, `descender-account-verifier/src/lib.rs` `vals_to_u32` / fixed arrays), so
decoding is a total function on accepted byte strings and the re-encoding of a
decoded statement reproduces the bytes (round-trip pinned by
`path2_account_stark_creation_payload_v1_digest`). Distinct statements differ in
at least one fixed-offset field ⇒ distinct bytes (injectivity); a single byte
string yields one field assignment ⇒ one statement (well-definedness). Therefore
the *proved statement is a deterministic function of the canonical bytes* — the
only way the byte model could endanger the proof (two readings of one proof) is
excluded. ∎

This is the property that matters for proof-collapse: combined with #1 (the
statement the AIR proves) and #5 (the statement binds to the deployed anchors),
Lemma I ensures there is exactly one proved statement per accepted payload.

## Rejection-class completeness matrix (consensus-relevant)

Every consensus-visible malformed payload must map to a deterministic public
rejection class in `spec/consensus_verifier.md`, with no panic and no state
mutation. The required coverage (from `spec/conformance_kats.md` § Required
Evidence) is:

| mutation family | covered objects | required public outcome |
|---|---|---|
| domain/version/length/reserved | every public object | deterministic reject class, no panic |
| admission negative | wrong leaf digest, wrong evidence field, wrong account, wrong state root, stale version, malformed proof, duplicate admission | reject class per `consensus_verifier.md` |
| anchor mutation | account_id, state_root, verifier_version, masked_projection[6], role_root export, packed_witness_commitment | binding-reject (#5) |
| spend negative | wrong message/account/state/slot/lifecycle/proof-commitment, checked-core mutation | reject class, consume-once state preserved |
| empty/corrupted bundle | proof bytes | fail-closed reject, no panic |

Current evidence (already wired): `Path2AccountStarkPayloadKatBoundaryReportV1`
maps mutation cases to public classes; the panic-free KATs
(`path2_direct_single_slot_spend_*_kats_are_panic_free`) and the
consume-once-preserving rejection KAT
(`path2_direct_single_slot_spend_rejections_preserve_consume_once_state`) exist.
**Completeness gap:** the matrix above must have a frozen vector for *every* row,
not the current draft subset, and the target-chain consensus-client failure
mapping (`spec/consensus_verifier.md` client mapping) must be enumerated.

## Implementation residual (slow gate — not closeable by markdown)

The `frozen` status requires byte-for-byte reproducible corpora generated from
the REAL account-STARK prover, which runs under the slow gate
(`scripts/check_active_rust_slow_tests.sh`) with a single-thread KAT proof pool
for deterministic proof-of-work witness selection (the deployment producer stays
fresh-randomized). Concretely, to move `frozen_status: draft → frozen`:

1. generate a frozen positive vector for direct admission + spend;
2. generate a frozen negative vector for every matrix row above;
3. assert each negative vector's public rejection class + panic-free + state-no-
   mutation;
4. enumerate the target-chain consensus-client failure mapping;
5. flip `Path2AccountStarkPayloadKatBoundaryReportV1` `frozen` only after 1–4 and
   the consistency gates pass.

This is genuine prover-pipeline + test work, not a theorem; it is scoped here so
it cannot be mistaken for closed.

## What This Closes And Hands Off

- **Closes (proof-relevant):** parser injectivity — the canonical byte model
  yields exactly one proved statement per accepted payload, so #8 carries no
  proof-collapse risk (the residual is consensus divergence, bounded by the
  rejection matrix).
- **Hands off (implementation):** the frozen byte-for-byte corpora and the
  full rejection-class + client-failure mapping (slow-gate Rust work,
  `DIR-STARK-001`/`DIR-REJECT-001`); the `frozen` flag stays draft until then.
