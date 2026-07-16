# Consensus Verifier And Rejection Model

Status: ACTIVE_NORMATIVE

This file defines the consensus-facing verifier contract for Descender direct
mode. A blockchain integration needs every accepting and rejecting path to be
deterministic, panic-free, implementation-independent, and state-transition
safe.

## Scope

The model covers:

- public parsing and canonical decoding;
- direct-mode account-slot admission checks;
- checked-core spend verification;
- lifecycle state checks and consume-once transition discipline;
- deterministic public rejection classes;
- consensus-client and wallet/HSM failure mapping;
- positive and negative KAT requirements for malformed and mutated public data.

It does not define secret-bearing signer behavior. Secret-side failures,
constant-time behavior, and hardware-wallet state handling remain governed by
`spec/signer_profile.md`; deployment-facing signer command/result behavior is
governed by `spec/signer_api.md`.

## Public Verification Pipeline

Direct Mode A verification is a fixed public pipeline:

```text
decode public frame
-> check domain/version/length/canonical encoding
-> normalize slot_authorization_digest
-> validate account-creation evidence for admission or load and recheck the
   exact canonical admission record
-> bind checked-core frame to account, state, message, leaf, and lifecycle data
-> verify checked-core spend
-> check lifecycle state is live
-> atomically apply accepted transition
```

Each step consumes only public bytes and public state. If any step rejects, the
verifier must return one public rejection class, leave consensus state
unchanged, and not expose internal proof, parser, or signer diagnostics.

## Public Result Classes

A consensus verifier must expose only these top-level result classes unless a
later ADR or spec update adds a versioned class.

| Class | Meaning | State effect |
|---|---|---|
| `accept_admit_slot` | account-creation evidence admits a new live slot | `absent -> live` |
| `accept_spend_slot` | checked-core spend verifies and consumes one live slot | `live -> consumed` |
| `reject_malformed_public_input` | public bytes are missing, wrong length, wrong domain, wrong version, or noncanonical | none |
| `reject_unsupported_version` | version or mode is recognized as unsupported by this verifier profile | none |
| `reject_binding_mismatch` | duplicated public fields, digest bindings, account ids, state roots, or lifecycle anchors disagree | none |
| `reject_creation_evidence` | account-creation evidence or proof/certificate does not admit the leaf | none |
| `reject_checked_core` | checked-core frame, challenge, response, proof commitment, or relation check fails | none |
| `reject_lifecycle_state` | slot is absent, consumed, burned, recoverable, stale, duplicated, or not live | none |
| `reject_policy` | account policy denies the otherwise well-formed authorization | none |
| `reject_resource_limit` | public input exceeds a deterministic consensus resource limit before semantic verification | none |
| `internal_error_fail_closed` | implementation cannot complete verification without risking inconsistent state | none |

The public class may be coarser than the first internal failure. It must never
be finer when that would reveal hidden witness material, diagnostic callback
state, retry counts, private proof openings, or signer-internal timing.

## Canonical Parsing Rules

Consensus parsing must be total over byte strings:

- every byte string returns either one public rejection class or one accepted
  transition candidate;
- length, domain, version, reserved-bit, and canonical-field checks are fixed by
  the verifier profile;
- noncanonical encodings must not be normalized into accepted encodings;
- unknown future versions must reject as `reject_unsupported_version` rather
  than falling through to a legacy parser;
- parser panics, unchecked integer overflows, host-endian behavior, allocation
  failures, and dependency exceptions must collapse to fail-closed rejection.

The parser may run different work for different public input lengths. It must
not branch on hidden signer material because hidden signer material is outside
the public verifier boundary.

## State Transition Discipline

Admission and spend verification are state-transition functions, not pure
signature checks.

An implementation must maintain these rules:

1. A rejecting verification path leaves consensus state unchanged.
2. An accepting admission records exactly one live slot under the account
   namespace and state scope. For the selected sovereign profile, the key is
   `(account_id, account_state_root_before, slot_authorization_digest)` and the
   value includes the canonical leaf bytes, leaf digest, evidence transcript
   digest, and admission height.
3. An accepting spend consumes exactly the live `slot_authorization_digest`
   that was checked by the spend frame.
4. The consume-once update is atomic with the accepted spend result.
5. Replays, duplicate admissions, stale leaves, wrong account roots, wrong
   lifecycle anchors, and policy-denied spends reject deterministically.
6. Recovery, burn, and rotation hooks must obey `spec/lifecycle_model.md` and
   `spec/policy_model.md`.

## Consensus-Client Mapping

A target-chain integration must map the public result classes to deterministic
client behavior:

| Client surface | Required behavior |
|---|---|
| transaction validation | reject classes produce a consensus-invalid transaction or validation failure without state mutation |
| mempool policy | may use coarser local policy, but consensus acceptance must match the verifier profile |
| state transition | accepted results update slot lifecycle atomically with account state |
| gas or fee accounting | resource limits and charged work must be deterministic for public input and profile version |
| logs/events | may include public result class and public identifiers, never hidden signer or proof internals |
| diagnostics | deployment builds must not expose internal parser/proof callback details through consensus APIs |

Different chains may choose different mempool policy, gas schedules, or
precompile interfaces, but they must not change the canonical accept/reject
semantics for the same verifier profile.

## KAT And Regression Requirements

Every consensus-visible public object requires positive and negative KATs.
Negative KATs must mutate one semantic field at a time where possible and record
the expected public rejection class.

Required direct-mode KAT groups:

- valid admission;
- valid spend;
- replay of consumed slot;
- duplicate admission;
- wrong domain and wrong version;
- wrong account id, state root, lifecycle anchor, slot id, and message digest;
- malformed length, noncanonical integer, reserved-bit, and trailing-byte cases;
- creation evidence/leaf mismatch;
- checked-core frame/proof commitment mismatch;
- policy denial;
- unsupported future version;
- deterministic resource-limit rejection for oversized public inputs.

The KAT corpus must be stable enough for independent consensus clients to
compare byte-for-byte outputs and public rejection classes.

The current account-creation leaf/evidence frame slice is frozen as one honest
case plus seventeen one-surface rejection cases. The generated boundary and
vector artifacts cover every current frame row, all fifteen duplicated
leaf/evidence binding fields, and the corresponding public result classes.
This closes the direct theorem's canonical/live leaf and evidence/leaf binding
rows when composed with the verified lifecycle profile; it does not complete
the broader byte-string parser, resource-limit, or independent-client KAT
requirements in this section.

The selected sovereign admission-state slice is also executable.
The account-STARK verifier returns opaque verified public anchors including the
canonical `Poseidon2(domain, pad_seed, slot)` nullifier. Only an opaque
`Path2VerifiedSovereignCreationAdmissionV1` can enter
`Path2SovereignDirectAdmissionStateV1`; decode-only or caller-supplied leaf
fields are not admission authority. The state preflights tuple and raw
nullifier keys for the entire candidate block before atomically advancing the
head or mutating either index, loads and
rechecks the full record on spend, rejects missing records, fails closed on
corruption, rejects mutation at an already-finalized head, snapshots both
indexes, binds restore to a supplied finalized checkpoint/state-digest anchor,
and rebuilds the nullifier index under
ADR-0010 D2 finalized-head reorg semantics.
`Path2DirectAdmissionBoundaryReportV1` and
`formal/kat/path2_direct_admission_boundary_v1.json` close the direct theorem's
admission namespace/data-availability row for this state model. Production
database binding, parser corpora, storage review, and independent-client
integration remain outside that theorem-scoped closure.

## Required Evidence

Tracker ownership: `SPEC-VERIFY-001`, `DIR-REJECT-001`, `DIR-KAT-001`,
`DIR-CC-001`, `DIR-LIFE-001`, `POL-REQ-001`, `ADOPT-ETH-AA-001`,
`ADOPT-ETH-7932-001`, `ADOPT-ETH-8141-001`, and `ADOPT-HW-API-001`.

Current executable precursor coverage includes `Path2PublicResultClassV1`,
which pins the consensus-visible result-class names from this table,
`Path2DirectSpendPublicResultV1`, which wraps direct spend outcomes without
internal diagnostics, plus `path2_direct_account_binding_public_result_class`,
which classifies selected direct spend/account-binding failures. The
`path2_direct_single_slot_spend_rejections_preserve_consume_once_state` test
checks that selected direct spend rejections leave consume-once state unchanged
before the accepted spend transition and map to the expected public classes.
`path2_direct_single_slot_spend_malformed_signature_kats_are_panic_free` adds
selected malformed Fq12 signature byte strings and checks they reject without
panic, map to `reject_malformed_public_input`, and leave consume-once state
unchanged.
`path2_direct_single_slot_spend_public_object_rejection_kats_are_panic_free`
adds selected direct leaf/evidence/certificate public-object failures and checks
they reject without panic, map to their public result classes, and leave
consume-once state unchanged.
`path2_direct_single_slot_spend_public_result_wrapper_hides_internal_diagnostics`
checks accept, malformed, checked-core, and lifecycle outcomes through the
public wrapper, returning fixed public result class names without exposing
internal diagnostics and restoring consume-once state on rejection.
`PATH2_DIRECT_PUBLIC_OBJECT_MUTATION_KAT_CASES_V1` enumerates the nine typed
public request/state objects at that wrapper, and
`path2_direct_all_consensus_visible_public_object_mutations_are_panic_free`
executes one semantic mutation per object through the real wrapper.
`PATH2_SOVEREIGN_DIRECT_CLIENT_MAPPING_ROWS_V1` maps all eleven public result
classes to unique consensus codes and state effects for the selected ADR-0009
sovereign-rollup plus ADR-0010 D2 lifecycle profile;
`path2_sovereign_direct_client_mapping_is_exhaustive_and_state_preserving`
checks every reject class is transaction-invalid and state-preserving. This
closes the typed direct-theorem rejection obligation, not the complete
byte-parser or deployment-profile evidence below.

Before the consensus verifier surface can be marked `verified`, the repo needs:

- malformed-frame, mutation, replay, lifecycle, and policy negative KATs;
- byte-for-byte positive KATs for admission and spend;
- complete panic-free parse/reject corpora over malformed public byte strings;
- deterministic state-no-mutation checks for every reject class;
- resource-limit tests for oversized public inputs;
- executable integration of the sovereign-client mapping into block validation,
  plus mappings for any account-abstraction, precompile, or frame-transaction
  profile that claims adoption evidence;
- independent implementation guidance showing which result classes are
  consensus-visible and which diagnostics are development-only.

Until these artifacts exist, consensus-safe rejection remains below `verified`
in `TRACKER.md` and below `ready` in `spec/blockchain_requirements.md`.
