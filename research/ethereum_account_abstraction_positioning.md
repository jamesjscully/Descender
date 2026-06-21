# Ethereum Account-Abstraction Positioning

Status: ACTIVE_SUPPORT
Date: 2026-06-20

This note maps Descender's chain-native PQ authorization objective to Ethereum's
current post-quantum and account-abstraction direction. It is an adoption
positioning note, not a claim that Ethereum has selected Descender or any
Descender-specific primitive.

## Source Context

Primary-source snapshot:

- `https://pq.ethereum.org/` says Ethereum's post-quantum work is organized
  around crypto agility and a multi-layer migration. For the execution layer, it
  highlights account abstraction, post-quantum signature verification, and
  gradual opt-in migration. For consensus, it highlights hash-based signatures
  plus proof aggregation because post-quantum signatures lack BLS-style native
  aggregation.
- `https://ethereum.org/roadmap/future-proofing/quantum-resistance/` says account
  signatures are one of Ethereum's quantum-vulnerable areas and describes
  account abstraction, specifically EIP-8141 under consideration, as the
  pragmatic route for signature agility. It also says PQ milestones are planning
  milestones, not guaranteed commitments.
- `https://blog.ethereum.org/2026/02/18/protocol-priorities-update-2026` says
  EIP-7702 was an important account-abstraction step and that EIP-7701/EIP-8141
  are pushing toward embedding smart-account logic into the protocol; it links
  that work to post-quantum readiness.
- `https://eips.ethereum.org/EIPS/eip-4337` defines the current no-consensus-
  change account-abstraction path where account validation logic is supplied by
  smart contract code.
- `https://eips.ethereum.org/EIPS/eip-7702` is explicitly forward-compatible
  with future account abstraction and lets EOAs use code paths that can align
  with ERC-4337 wallets.
- `https://eips.ethereum.org/EIPS/eip-7932` is a draft secondary-signature
  algorithm registry/precompile proposal.
- `https://eips.ethereum.org/EIPS/eip-8141` is a draft frame-transaction
  proposal where validity and gas payment are defined through validation,
  payment, and execution frames; its motivation includes a native off-ramp from
  elliptic-curve authentication to post-quantum systems.

## Position

Descender should not present itself to Ethereum as merely a raw PQ signature.
Ethereum's direction favors programmable account validation and crypto agility.
The right Descender story is:

```text
PQ spend authorization
+ one-use account slots
+ account/state lifecycle binding
+ optional proof aggregation
+ smart-account / native-AA integration
```

That matches Ethereum's pain points better than a standalone "new CHECKSIG"
primitive. It also avoids overclaiming: Ethereum's own post-quantum materials
call out aggregation as a hard part, so Descender must be explicit that its
aggregation path is proof/state aggregation, not native BLS-style signature
aggregation.

## Integration Surfaces

### ERC-4337 / Smart-Account Verifier

Near-term path: implement Descender verification as smart-account validation
logic.

Required Descender work:

- define an EVM verifier ABI for direct Mode A;
- define calldata layout for `slot_authorization_digest`, checked-core spend
  material, and direct slot record evidence;
- measure gas for pure-EVM verification or a hybrid verifier using existing
  precompiles;
- prove rejection is deterministic and free of chain-dependent implementation
  behavior;
- define account storage for live/consumed slot records.

This is the fastest way to test Ethereum usability without waiting for a hard
fork. It is also the right surface for early wallets and paymasters.

### EIP-7702 Delegated EOA Path

EIP-7702 makes an EOA-compatible delegation path available while keeping a path
toward later account abstraction. Descender should treat this as a migration UX
surface, not as a final cryptographic target.

Required Descender work:

- specify how an existing EOA delegates to a Descender-verifying account;
- define one-use slot initialization under delegated account code;
- define safe key rotation and recovery behavior after delegation;
- avoid any design that requires long-term reuse of the old ECDSA key.

### EIP-7932-Style Precompile / Algorithm Registry

If Ethereum standardizes a secondary-signature registry and precompile, Descender
needs a clean algorithm-profile story.

Required Descender work:

- algorithm/profile identifier;
- canonical byte encoding for public key or account identity;
- verifier ABI and return semantics;
- gas model for direct Mode A verification;
- strict version and domain separation;
- negative KATs for malformed frames, stale slots, wrong account roots, and
  consumed slots.

A precompile profile must not include Mode B claims unless the aggregate proof
system is also specified.

### EIP-8141 / Native Frame Transaction Path

Frame transactions are the most natural native-account-abstraction target if
they land. Descender should model a validation frame that checks the same direct
Mode A object it would check in a smart account.

Required Descender work:

- `VERIFY`/validation-frame payload mapping;
- gas-payment separation: validation must not smuggle authorization assumptions
  into fee payment;
- account-state access rules for live and consumed slot records;
- deterministic failure modes suitable for mempool validation;
- compatibility with static validation constraints if final EIP-8141 rules
  require them.

### Rollup / Proof-Aggregated Path

Ethereum's PQ roadmap repeatedly points at aggregation and proof compression.
Descender's Mode B should target rollup and bridge contexts first, not L1 raw
transaction replacement.

Required Descender work:

- aggregate proof handle format;
- canonical ordered batch statement;
- duplicate-slot and duplicate-consume prevention;
- data availability rules for slot records not directly stored on L1;
- a PQ-safe recursion endpoint. If the final proof is wrapped in a pairing SNARK,
  the outer proof is not post-quantum.

## Non-Goals For Ethereum Positioning

Do not claim:

- Ethereum has selected Descender;
- Descender replaces Ethereum BLS consensus signatures;
- Descender solves validator aggregation without a concrete Mode B proof system;
- EIP-8141, EIP-7932, or any other draft EIP is guaranteed to ship unchanged;
- a smart-account prototype is equivalent to L1 protocol adoption.

## Tracker Consequences

The Ethereum adoption work should split into concrete artifacts:

- `ADOPT-ETH-001`: Ethereum/account-abstraction positioning baseline.
- `ADOPT-ETH-AA-001`: ERC-4337 smart-account verifier ABI and gas model.
- `ADOPT-ETH-7702-001`: delegated-EOA migration profile.
- `ADOPT-ETH-7932-001`: secondary-signature/precompile profile.
- `ADOPT-ETH-8141-001`: frame-transaction validation mapping.
- `ADOPT-ETH-ROLLUP-001`: rollup aggregate proof handle.

The first useful artifact is `ADOPT-ETH-AA-001`, because it can be tested today
without assuming any draft EIP becomes final.
