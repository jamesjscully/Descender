# Selected Hidden-Source Path Decision

Status: current internal decision for deployable-signature work.

## Decision

The selected deployable path is now the account-creation / prevalidated-slot
authorization route.

The existing `308 B/Fq12` active-lane projection-carrier envelope remains the
executable baseline and negative-control source, but it is no longer the
long-term spend format target. The deployment direction is to move
hidden-source validation into account/slot creation, then make spend-time
verification prove authorization to consume a previously validated one-use slot
plus checked-core message binding.

Target spend shape:

- checked-core Fq12 signature body: `252 B`
- projection-carrier material removed from the spend path when its
  authorization record is consensus-visible: `56 B`
- account/slot state grows by a prevalidation handle, certificate root,
  carrier material, or authorization leaf depending on the final account-state
  layout.

The direct expanded account-opening slot commitment path is archival and
non-deployment. It remains useful as a reference predicate and negative-control
source, but it is not the selected account-state model for an accepting
verifier.

## Why

The state-assisted tuple exists to bind the hidden-source opening to more than
the six account projection coordinates. Its frame includes slot id, account
witness commitment, account randomizer commitment, restore anchor,
witness-opening digest, pad-seed digest, and carrier-handle secret digest. That
is the shape needed for prechallenge fixation, transcript/carrier binding,
restore rollback resistance, and single-use lifecycle semantics.

The new design choice is where the hard hidden-source proof is discharged. The
prior 308-byte spend-carrier path tried to make each spend transcript carry
enough public material to justify the hidden-source relation. The rank probe for
simple extra carrier lanes shows that this does not compress: extra lanes buy
only one linear constraint each and do not address the checked-core kernel. The
account-creation path instead validates each one-use slot before it becomes
live, commits the validation result into account state, and lets later spends
consume that prevalidated capability.

This is also the aggregation-friendlier blockchain architecture. Slot creation
can be batched, rooted, or proven in a rollup/block proof, while spends become
ordinary state transitions against prevalidated one-use capabilities.
Aggregation is not a first-deployment requirement; the model only requires that
single-slot records use canonical leaf and authorization-digest frames that can
later be consumed by optional batch or accumulator proofs. See
`research/account_creation_aggregation_readiness_contract.md`.

The direct expanded-opening path stores only the expanded commitment to the
selected account projection lanes. That is too narrow for the final deployment
claim because it does not by itself bind the pad seed, carrier handle, restore
anchor, or the public hidden-source statement. It also risks letting an
account-opening frame be reasoned about independently from the account witness
commitment.

## What Remains To Prove

- Account/slot creation proves or verifies the canonical hidden-source relation
  before a slot becomes live.
- The prevalidation record is bound to public-key identity, slot id, verifier
  version, opening commitment, pad-seed digest, hidden-source statement digest,
  and account state root.
- The spend verifier proves authorization to consume the exact live
  prevalidated slot named by the checked-core challenge frame.
- The checked-core signature remains message-bound at spend time; the
  prevalidated slot is only a one-use signing capability, not a reusable
  signature.
- The account-state model prevents postchallenge slot creation or adaptive
  prevalidation rewrite for the same spend challenge.
- The account-state verifier exposes enough prevalidation data, or a reviewed
  succinct commitment to it, for validators and aggregators to fail closed when
  the record is missing or stale.
- The state-assisted opening tuple is fixed before the Fiat-Shamir challenge
  and before any spend-time authorization proof.
- `witness_opening_digest` in that tuple is exactly the selected active-lane
  account-witness projection opening consumed by account creation.
- `pad_seed_digest` in that tuple opens the public pad-opening statement used
  by the checked core and by the creation-time hidden-source validation.
- the direct expanded-opening state-slot helpers are not reachable from the
  deployment verifier or deployment preflight as accepting paths.
- the extractor/authorization theorem consumes the prevalidated-slot model, not
  the archived direct expanded-opening model and not a spend-only extraction
  claim.

## Current Executable Closure

- `verify_reviewed_fq12_state_assisted_account_witness_projection_opening`
  binds the projection-opening frame to the state-assisted opening tuple and
  hidden-source statement.
- `verify_reviewed_fq12_state_assisted_pad_seed_opening` binds the pad seed to
  the same opening tuple and the verifier-owned pad-opening statement.
- `verify_reviewed_fq12_state_assisted_projection_carrier_tuple` composes the
  slot opening, projection-opening digest, pad-seed opening, hidden-material
  projection lanes, and 56-byte carrier check/transcript binding for the
  selected 308-byte path.

The remaining proof gap is no longer framed as "every accepting spend transcript
must extract the full hidden-source witness." The selected theorem target is:

> If the spend-time verifier accepts a checked-core signature and consumes an
> account slot, then either that slot was previously accepted by the
> account-creation verifier as a valid hidden-source relation bound to the same
> identity, slot, opening commitment, pad-seed digest, verifier version, and
> account state root, or the adversary exports a named creation-soundness,
> registry-binding, lifecycle, checked-core, QROM, or leakage break.

The prior spend-only theorem remains useful as a negative control. It is refuted
for the current byte relation. The regression test
`projection_carrier_tuple_does_not_prove_full_witness_opening` keeps the same
six-value projection-opening frame and selected carrier lanes, rebinds the tuple
to two different packed Fq12 account-witness commitments, and verifies that both
tuples accept after recomputing the allowed transcript binding. The executable
closure therefore proves a projection opening only. Under the selected Path 2
direction, this no longer blocks as a spend-only extraction theorem; it becomes
evidence that account creation must carry the hidden-source validation burden.

## Selected Planning Track

Next artifact:
`account_creation_prevalidated_slot_authorization_theorem`.

Proof-side progress tracker:
`research/account_creation_prevalidated_slot_proof_tracker.md`.

First proof-frame artifact:
`research/prevalidated_slot_leaf_v1_frame.md`.

Initial planning work:

- define the account-creation verifier inputs and exact state transition from
  absent/reserved slot to live prevalidated slot;
- choose the prevalidation state layout: minimal authorization leaf, full
  carrier, certificate root plus carrier, or accumulator-backed leaf;
- specify the creation-time hidden-source relation and its negative controls;
- define the spend-time authorization verifier over checked-core signature,
  public key, message, slot id, state root, and prevalidation handle;
- reserve aggregation-ready leaf, authorization-digest, direct-record,
  inclusion-proof, and aggregated-proof-handle frames without making aggregated
  admission a deployment dependency;
- add byte/cost accounting for direct consensus-state lookup versus
  Merkle/accumulator inclusion proofs;
- update production preflight so the primary hidden-source blocker points at
  account-creation prevalidation semantics rather than spend-only extraction.

## Archival Paths

- `284 B/Fq12` signature-carried hidden-source proof handle.
- `state_assisted_32_byte_carrier` / RO-PoK handle route.
- direct expanded account-opening slot commitment for the selected projection
  lanes.
- historical 64-variable hidden-source projection model.
- simple extra carrier-lane expansion for the 56-byte projection carrier.
- spend-time raw `pad_seed` reveal as a default deployment path.
- Path 3 state-stored lanes/checks as the primary route; it remains a useful
  conservative prototype, not the selected long-term blockchain design.

These paths may stay in research code as probes, regression fixtures, or break
exporters, but they are not deployable surfaces.
