# Account-Creation Prevalidated Slot Plan

Status: planning-stage selected path.

## Selected Direction

Descender's blockchain deployment direction is Path 2: account creation or slot
refresh prevalidates one-use signing slots before spend time. The spend
transaction then proves authorization to consume a live prevalidated slot plus
checked-core message binding.

This replaces the prior long-term goal of extracting a full hidden-source
opening from each `308 B/Fq12` spend transcript. The 308-byte candidate remains
the executable baseline and negative-control source while the protocol moves
toward a state-backed authorization model.

Proof-side closure is tracked in
`research/account_creation_prevalidated_slot_proof_tracker.md`. That tracker is
the working checklist for the Path 2 theorem, proof boundaries, assumption
ledger, and optional aggregation mode.

The first proof object is specified in
`research/prevalidated_slot_leaf_v1_frame.md`: a fixed 353-byte
`PrevalidatedSlotLeafV1` frame plus the normalized
`slot_authorization_digest`.
The account-creation evidence interface is specified in
`research/account_creation_evidence_v1_spec.md`.
The selected first proof mechanism is specified in
`research/account_creation_proof_mechanism_decision.md`.
The creation-time relation checked by that mechanism is specified in
`research/creation_hidden_source_relation_v1_spec.md`.
The local creation-time soundness theorem target is stated in
`research/direct_replayable_relation_certificate_soundness_theorem.md`.
The creation-time negative-control matrix is in
`research/creation_time_negative_control_matrix.md`.
The spend-time authorization-handle interface is specified in
`research/slot_authorization_handle_v1_spec.md`.

## Intended Spend Shape

- checked-core Fq12 signature body: `252 B`
- projection carrier removed from spend when authorization data is already in
  consensus-visible state: `56 B`
- target spend envelope after migration: approximately `252 B`, plus any
  account-state inclusion proof required by the target chain

If the spend transaction must carry a Merkle or accumulator path for the
prevalidated slot, that proof size is added back to the transaction. Path 2 is
most attractive when validators already have direct account-state access or when
block/rollup proof aggregation can amortize inclusion checks.

## Account-State Layout Candidates

| candidate | extra state | spend bytes | notes |
|---|---:|---:|---|
| minimal prevalidation leaf | `+32 B/slot` plus optional account root | `252 B` plus inclusion proof if needed | best aggregation shape, but the leaf must commit to a real creation-time validation transcript |
| full carrier in slot | `+56 B/slot` | `252 B` | direct availability of carrier material, larger state |
| carrier plus certificate root | `+88 B/slot` | `252 B` | cleaner audit story, probably too much permanent state unless slots are scarce |
| lanes/checks prototype | `+27 B/slot` | `281 B` | conservative stepping stone, not the selected long-term target |

The selected planning bias is the minimal prevalidation leaf or account-root plus
per-slot authorization leaf. Full-carrier state remains a fallback if the leaf
cannot support a credible creation-time validation theorem.

## Creation-Time Verifier

The creation verifier accepts a candidate slot only if it validates and commits
all hidden-source material needed for later authorization:

- public-key identity;
- slot id;
- verifier/profile version;
- account state root or account update root;
- state-assisted opening commitment;
- `pad_seed_digest`;
- hidden-source statement digest;
- creation-time validation transcript digest or certificate root;
- leakage budget classification for the public slot metadata.

It must reject if any of those fields are missing, stale, not domain separated,
or not bound to the same account state transition.

## Spend-Time Verifier

The spend verifier checks:

- the checked-core Fq12 signature against message, public key, slot id, and
  challenge frame;
- the slot is live and prevalidated under the current account state;
- the prevalidation handle names the same public-key identity, slot id,
  verifier version, opening commitment, and pad-seed digest;
- the state transition consumes or burns the exact slot;
- missing or stale prevalidation data fails closed.

Spend-time verification no longer claims to extract the hidden-source witness
from the spend transcript itself. It proves one-use authorization plus message
binding.

## Theorem Target

`account_creation_prevalidated_slot_authorization_or_break`:

If the spend-time verifier accepts and consumes a slot, then either:

- the consumed slot was previously accepted by the account-creation verifier for
  the same public-key identity, slot id, verifier version, opening commitment,
  pad-seed digest, hidden-source statement digest, and account state root; or
- the adversary exports a named break.

Named break classes:

- `creation_time_hidden_source_soundness_break`;
- `prevalidation_registry_binding_break`;
- `postchallenge_slot_creation_or_rewrite_break`;
- `slot_replay_or_rollback_break`;
- `checked_core_message_binding_break`;
- `prevalidation_data_availability_break`;
- `live_slot_leakage_or_keygen_break`;
- `qrom_or_domain_separation_break`.

## Aggregation Implications

Path 2 is the most aggregation-friendly route because slot validation is a
homogeneous setup operation:

- wallets can precompute batches of slots;
- account refresh can publish a root of many prevalidated one-use capabilities;
- rollups can prove many slot-creation validations together;
- block producers can aggregate state-inclusion and consume-once checks;
- spend transactions stay close to checked-core signature verification plus
  ordinary account-state transition logic.

Descender should not be described as naturally supporting BLS-style native
signature aggregation. The realistic aggregation targets are batch verification,
state/proof aggregation, and rollup/block-level validation of slot creation and
slot consumption.

Aggregation readiness is tracked as an optional model contract in
`research/account_creation_aggregation_readiness_contract.md`. The baseline
single-slot verifier must not depend on aggregation soundness, but every
prevalidated slot should have a canonical leaf and authorization digest that can
later be used by direct records, inclusion proofs, or aggregated proof handles.

## Non-Selected Paths

- Simple extra carrier lanes are rejected as a compact proof replacement. The
  rank probe shows one lane buys one linear constraint and does not close the
  hidden-source gap.
- Spend-time raw `pad_seed` reveal remains a research/debugging model. It costs
  `+32 B` per spend and changes the leakage theorem.
- Path 3, state-stored projection lanes/checks, remains a conservative prototype
  if the team wants an incremental implementation step, but it is not the
  selected long-term blockchain design.

## Immediate Planning Tasks

1. Define the exact prevalidation leaf byte frame.
2. Define the account-creation verifier API and fail-closed errors.
3. Decide whether prevalidation data is stored directly in consensus account
   state or committed through an account root with inclusion proofs.
4. Reserve the aggregation-ready domain frames and normalized slot
   authorization digest from
   `research/account_creation_aggregation_readiness_contract.md`.
5. Add a creation-time negative-control matrix for wrong key, wrong slot, wrong
   verifier version, wrong opening commitment, wrong pad-seed digest, wrong
   statement digest, and stale state root.
6. Add a spend-time authorization negative-control matrix for stale leaf,
   missing leaf, wrong slot, already consumed slot, wrong message binding, wrong
   public key, wrong state root, and duplicate consumption.
7. Update production preflight and status artifacts to name Path 2 as the
   selected planning direction while keeping deployability fail-closed.
8. Estimate compute separately for slot creation, slot refresh batches, and
   spend verification.
9. Work through
   `research/account_creation_prevalidated_slot_proof_tracker.md` before
   promoting implementation hardening as the main blocker.

## Open Design Questions

- Does the target chain expose direct account-state reads to validators, or must
  each transaction carry an inclusion proof?
- Is slot creation a separate transaction, an account-refresh transaction, or a
  wallet-local preparation step followed by a chain-visible commitment update?
- Is the prevalidation leaf enough, or do validators need full carrier/certificate
  data availability for independent replay?
- What is the maximum live-slot count per account before state rent, pruning, or
  aggregation becomes mandatory?
- Which creation-time proof object is most compact while still independently
  replayable by validators or rollup provers?
