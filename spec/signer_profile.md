# Signer And Custody Profile

Status: ACTIVE_NORMATIVE

This file defines the signer, keygen, sampler, wallet, and HSM requirements for
Descender direct-mode deployment. It exists because a blockchain authorization
scheme fails in practice if private-key operations are fragile, even when the
public verifier is simple.

## Scope

The profile applies to:

- key generation;
- account/slot creation or refresh;
- hidden-source witness handling;
- spend authorization;
- one-use slot state updates inside a wallet, secure element, HSM, or custody
  service;
- signer-facing APIs and error handling.

It does not replace the public verifier spec. Consensus-visible parse, reject,
and verify rules remain in `spec/consensus_verifier.md`,
`spec/wire_formats.md`, `spec/security_model.md`, and the active
implementation. The deployment-facing public command/result contract is
specified in `spec/signer_api.md`; entropy-source classes and
domain-separated stream requirements are specified in
`spec/entropy_profile.md`; constant-time and leakage evidence requirements are
specified in `spec/leakage_profile.md`; signer-local rollback and fault-state
requirements are specified in `spec/signer_state_profile.md`.

## Non-Negotiable Design Rules

1. No floating-point arithmetic is allowed in secret-bearing keygen, sampling,
   witness handling, signing, or proof construction.
2. Secret-bearing arithmetic must be integer, finite-field, or fixed-width byte
   arithmetic with platform-independent semantics.
3. Secret-dependent branches, table indices, memory addresses, allocation
   shapes, loop counts, and error strings are forbidden unless a proof/review
   explicitly classifies the value as public.
4. Rejection sampling or retry loops must have a bounded, reviewed, and
   secret-independent external shape. If an internal loop can fail, the public
   result must collapse to a fixed failure class.
5. Signer APIs must not return raw hidden witnesses, pad seeds, projection masks,
   sampler randomness, private openings, or internal retry/failure diagnostics.
6. All secret material must be zeroized or held in a documented secure-storage
   object with an explicit lifetime.
7. Faults must fail closed. A fault may invalidate a slot or force recovery, but
   it must not produce a partially checked authorization.

## Public / Secret Boundary

Allowed public outputs:

- account id;
- public-key identity or digest;
- slot id;
- verifier/profile version;
- canonical public slot leaf or digest;
- `slot_authorization_digest`;
- public state roots and lifecycle anchors;
- account-creation proof bytes if the selected proof system marks them public;
- fixed public rejection classes.

Forbidden public outputs:

- secret key bytes;
- hidden account witness;
- pad seed;
- projection masks;
- unmasked private projection residues except where a theorem explicitly marks
  them as public spend disclosure;
- sampler randomness;
- conditioned-sampler retry count;
- internal callback error strings;
- raw private opening frames.

Any exception must be named in the security model and charged to a specific
break or disclosure budget.

## Wallet / HSM API Shape

Tracker ownership: `SPEC-SIGNER-001`, `ADOPT-HW-API-001`, and
`DIR-REJECT-001`.

The preferred direct-mode API separates slot preparation from spend:

```text
create_or_refresh_slot(account_state, policy, entropy) -> public slot record
authorize_spend(slot_id, message_digest, state_context) -> checked-core spend
consume_or_burn_slot(slot_id, result, state_context) -> updated local state
```

Required behavior:

- a slot is either unused, consumed, burned, or recoverable under a named policy;
- a signer must refuse to authorize with a consumed or burned slot;
- local wallet state must be replay-protected against crash/restart races;
- spend authorization must bind the chain/application domain, account id, slot
  id, state root/context, and `slot_authorization_digest`;
- error reporting must use a fixed public enum;
- diagnostic builds that expose hidden material must be compile-time separated
  from deployment builds.

The normative command set, request/response envelopes, fixed public result
classes, diagnostic-build separation, and transport-profile requirements live in
`spec/signer_api.md`.

## Entropy And Domain Separation

Tracker ownership: `SPEC-SIGNER-001`, `ADOPT-HW-ENT-001`, and `DIR-KAT-001`.

Keygen, slot refresh, pad derivation, randomizer derivation, and proof blinding
must use distinct domain-separated streams.

The normative stream registry, entropy-source classes, deterministic test-mode
separation, and mask-PRF drift evidence contract live in
`spec/entropy_profile.md`.

Required evidence:

- domain labels are versioned and fixed;
- entropy source requirements are documented for software wallets, secure
  elements, HSMs, and custody services;
- deterministic test mode is compile-time separated from deployment mode;
- replaying the same entropy input cannot silently create two live slots with
  the same authorization identity;
- all derived public commitments bind the domain, profile version, account id,
  and slot id where applicable.

## Constant-Time And Leakage Evidence

Tracker ownership: `SPEC-SIGNER-001`, `DIR-CT-001`, and `ADOPT-HW-CT-001`.

Before any deployment claim, the project needs all of:

- static review of secret-dependent branches, memory access, arithmetic,
  allocation, logging, error handling, and callbacks;
- leakage tests or equivalent reviewed harnesses for keygen, sampler, pad
  expansion, witness handling, and signing;
- negative tests proving public serialization does not contain forbidden secret
  material;
- fixed-shape failure tests for callback failure, malformed input, consumed slot,
  and fault injection;
- compiler/target/dependency profile for each deployment class;
- documented limits for stack, heap, persistent storage, and secure-element
  command APDUs or HSM messages where relevant.

Timing tests on public verifier parse failures are useful, but they do not prove
secret-side safety. The signer profile is not satisfied until the secret-bearing
operations are reviewed.

The detailed secret/public classification, forbidden behavior list, leakage
harness shape, fixed-shape failure requirements, target/build profiles, and
serialization/logging negative tests live in `spec/leakage_profile.md`.

## Fault And Recovery Requirements

Tracker ownership: `SPEC-SIGNER-001`, `DIR-LIFE-001`, and
`ADOPT-HW-STATE-001`.

Hardware wallets and custody modules must treat authorization state as
transactional:

- mark a slot pending before releasing an authorization if the deployment model
  can crash between signature release and ledger update;
- consume or burn the slot after a successful local release according to the
  chain policy;
- provide a recovery path for uncertain state that cannot replay the same slot
  unsafely;
- bind backups and restores to `lifecycle_anchor` or an equivalent account
  recovery root;
- reject state rollback unless a target-chain reorg/finality policy permits it.

The detailed local state record, restore-anchor, release-atomicity,
crash/retry, finality, and fixed public state/fault result requirements live in
`spec/signer_state_profile.md`.

## Evidence Status Values

Signer/custody work should use these labels in review artifacts:

| Status | Meaning |
|---|---|
| `not_started` | no deployment-relevant evidence exists |
| `memo_only` | analysis note exists, not executable evidence |
| `instrumented` | tests or harnesses exist but are incomplete |
| `reviewed` | code review covers the named surface |
| `tested` | executable leakage/fault/serialization tests cover the named surface |
| `deployment_ready` | review, tests, platform profile, and residual risks are documented |

`deployment_ready` requires external review for hardware wallet, HSM, or custody
claims.

## Non-Claims

The project must not claim:

- constant-time behavior on all platforms;
- side-channel resistance from type safety alone;
- signer safety from public verifier correctness alone;
- hardware-wallet readiness from a desktop reference implementation;
- safe custody integration without a state-rollback/fault model;
- deployability of diagnostic callbacks or research wrappers.

## Relationship To Falcon/FN-DSA Concerns

Descender's active path should avoid the signer-side fragility that motivates
concern about floating-point lattice signatures in hostile wallet hardware. The
project may still have complex proof generation or witness handling, but any
secret-bearing deployment path must be auditable with fixed-width integer/field
operations and explicit leakage evidence.
