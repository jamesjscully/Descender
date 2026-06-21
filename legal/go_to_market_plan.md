# Go-To-Market Plan: PQ Authorization Infrastructure

> MIGRATION STATUS (2026-06-21): `ACTIVE_RESEARCH`.
> This is a legal/commercialization plan, not the top-level project tracker and
> not technical product authority. Technical authority starts in
> `../spec/README.md`, adoption readiness lives in
> `../spec/blockchain_requirements.md`, and live work is tracked in
> `../TRACKER.md`.

Status: active checklist
Date: 2026-06-21

This plan encodes the commercial direction for Descender after the pivot to a
chain-native post-quantum authorization stack.

The commercial thesis is:

```text
Own PQ migration infrastructure for high-value crypto control planes.
```

Descender should not lead as "a new PQ signature algorithm" or as a speculative
token project. The strongest money path is to become the authorization,
conformance, integration, and support stack that high-value crypto systems use
when they need post-quantum migration without breaking wallet safety, state
semantics, rollup compression, or operational key management.

This document is not legal, financial, tax, or investment advice. Customer
commitments, warranties, export-control positions, token decisions, patent
pledges, and commercial licenses require qualified review before external use.

## Strategic Positioning

The market-facing positioning should be:

```text
Descender: post-quantum authorization infrastructure for wallets, custody,
bridges, rollups, and protocol control planes.
```

The product should sell:

- migration certainty;
- audited signer and verifier engineering;
- conformance evidence;
- wallet/HSM integration;
- Ethereum account-abstraction integration;
- custody policy modules;
- rollup and bridge authorization compression;
- external-review and certification packages;
- high-liability support, warranties, and indemnity where counsel approves.

The core algorithm, public encodings, conformance tests, and reference profiles
should trend open enough for standards and ecosystem trust. Revenue should come
from integration quality, certification, support, review packages,
implementation hardening, hosted tooling, and high-liability deployment
services.

## Market Priority

The commercial priority order is:

| Priority | Segment | Why it comes first | Initial offer |
|---|---|---|---|
| 1 | Custody, exchanges, institutional wallets | concentrated assets, exposed operational keys, real budget, strong compliance pressure | PQ-readiness assessment, signer/HSM migration plan, rollback-safe key policy, paid integration support |
| 2 | Ethereum L2s, bridges, and protocol admin keys | high-value control planes, rollup/bridge risk, natural fit for proof/state aggregation | bridge/admin-key PQ migration package, rollup aggregate-handle design, governance-key recovery profile |
| 3 | ERC-4337 and EIP-7702 smart-account ecosystem | final near-term Ethereum surfaces with programmable validation and migration UX | Descender smart-account verifier, delegated-EOA migration demo, gas/KAT report |
| 4 | Conformance and certification | trust bottleneck for wallets, custody vendors, rollups, and standards adoption | test vectors, compatibility harness, certified implementation program |
| 5 | Bitcoin and UTXO ecosystems | strategically important but slower L1 path; better near-term fit through custody, L2, BitVM/covenant research | UTXO-native profile research, custody/L2 integration, no Bitcoin L1 adoption claim |

This order is a resource-allocation rule. It does not mean other ecosystems are
irrelevant. It means the first revenue and credibility wedge should come from
buyers with immediate high-value key risk and deployment budgets.

## Product Lines

### Descender Core

Open, reviewable protocol artifacts:

- current spec spine and ADRs;
- direct Mode A verifier profile;
- public wire formats;
- conformance KAT corpus;
- reference implementation;
- independent implementation guidance;
- external-review package.

Commercial goal: ecosystem trust and standards credibility.

### Descender Conformance

Paid and open conformance infrastructure:

- canonical KAT corpus;
- malformed-input and state-no-mutation tests;
- signer API compatibility tests;
- deployment-profile test suites;
- certification reports;
- logo/trademark rules for compatible implementations.

Commercial goal: become the test and certification authority for Descender
deployments.

### Descender Wallet/HSM SDK

Signer-side deployment tooling:

- integer/field-only signer profile;
- no-floating-point implementation discipline;
- wallet/HSM command API;
- entropy and domain-separation registry;
- rollback/fault-state persistence;
- leakage/fixed-shape test harnesses;
- custody integration support.

Commercial goal: sell into custody providers, exchanges, hardware-wallet
vendors, and institutional key-management systems.

### Descender Ethereum Kit

Ethereum-facing integration artifacts:

- ERC-4337 smart-account verifier;
- EIP-7702 delegated-EOA migration demo;
- secondary-signature/precompile profile support if the ecosystem converges;
- frame-transaction profile support if the ecosystem converges;
- gas reports;
- bundler/paymaster simulation tests;
- profile-specific KATs.

Commercial goal: make Descender the obvious PQ account-authorization candidate
for Ethereum applications and L2s.

### Descender Custody Kit

Policy and operational-key modules:

- M-of-N account policy;
- threshold-custody route decision;
- guardian and timelock recovery;
- key rotation;
- burn/restore semantics;
- admin/governance key migration playbooks;
- audit evidence for custody and protocol risk teams.

Commercial goal: monetize high-value key migration without waiting for protocol
L1 adoption.

### Descender Rollup Kit

Mode B and bridge-facing artifacts:

- aggregate handle format;
- ordered batch statements;
- duplicate-slot and duplicate-consume rejection;
- data-availability rules;
- light-client semantics;
- recursion endpoint profile;
- residual-assumption ledger for pairing wrappers or non-PQ outer proofs.

Commercial goal: make Descender useful where raw PQ signatures are too large or
too hard to aggregate natively.

## Near-Term Deliverables

The next commercially useful artifacts are:

| Deliverable | Tracker anchor | Commercial use |
|---|---|---|
| ERC-4337 demo verifier with gas report | `ADOPT-ETH-AA-001` | customer demo for Ethereum wallets/L2s |
| EIP-7702 delegated-EOA migration demo | `ADOPT-ETH-7702-001` | migration UX story for existing accounts |
| Wallet/HSM signer API implementation | `ADOPT-HW-API-001` | custody and hardware-wallet integration |
| Concrete KAT corpus and manifest | `DIR-KAT-001`, `SPEC-KAT-001` | conformance and certification foundation |
| Leakage/fixed-shape signer harness | `ADOPT-HW-CT-001`, `DIR-CT-001` | answer Falcon-style signer-safety objections |
| Persistent signer-state schema and crash tests | `ADOPT-HW-STATE-001` | custody safety and operational reliability |
| Custody policy demo | `POL-MSIG-001`, `POL-RECOVERY-001`, `POL-THRESH-001` | institutional wallet sales wedge |
| Rollup aggregate-handle prototype | `ADOPT-ETH-ROLLUP-001`, `AGG-ROLLUP-001`, `AGG-DA-001` | L2/bridge adoption wedge |
| External review package v0 | `DIR-REVIEW-001`, `SPEC-REVIEW-001` | enterprise trust and fundraising evidence |
| One-page institutional pitch | `ADOPT-COMM-001` | customer/investor communication |

## Explicit Non-Directions

Do not lead with:

- a token;
- Bitcoin L1 consensus adoption;
- a closed core algorithm;
- a patent toll on ordinary interoperable use;
- marketing claims that Descender is deployment-ready before tracker rows are
  `verified`;
- claims that Mode B aggregation is solved before the aggregate proof system,
  data availability rules, and light-client semantics are concrete.

A token, if ever considered, belongs after real usage or a credible
decentralized prover/attestation market exists. Before that point it is likely
to damage trust with serious cryptographers, custody buyers, standards
reviewers, and protocol teams.

Bitcoin work should be framed as a later UTXO-native profile and as near-term
custody/L2 research. It should not consume the first commercialization cycle.

## Sales Motion

The first sales motion should be consultative and evidence-driven:

1. Identify high-value control planes: custody hot wallets, bridge/admin keys,
   protocol governance multisigs, rollup sequencer/admin keys, and treasury
   controls.
2. Offer a PQ-readiness assessment tied to current exposed public keys, account
   migration paths, signer hardware, operational recovery, and rollback risks.
3. Deliver a migration plan that distinguishes direct Mode A, policy-layer
   custody, and Mode B aggregation assumptions.
4. Convert qualified buyers into paid integration, conformance, and support
   engagements.
5. Reuse sanitized findings to strengthen public specs, KATs, and review
   packages.

The project should sell discipline: explicit non-claims, tests, review lanes,
and hard failure modes. That posture is more valuable to serious buyers than
aggressive cryptographic marketing.

## Fundraising Story

The fundraising story should be:

```text
Quantum migration is a key-management and authorization migration, not only a
signature swap. Descender is building the open, testable, chain-native
authorization stack for high-value crypto systems that cannot wait for a single
L1 standard to solve wallets, bridges, custody, rollups, and governance keys.
```

Evidence to assemble before fundraising conversations:

- active spec spine and consolidated tracker;
- green local/CI authority gates;
- Ethereum account-abstraction positioning;
- signer safety profile;
- KAT/conformance plan;
- external-review package outline;
- one credible demo path, preferably ERC-4337 plus signer API;
- clear legal/IP posture reviewed by counsel.

## Required Evidence

Tracker ownership: `ADOPT-COMM-001`, `ADOPT-ETH-AA-001`,
`ADOPT-ETH-7702-001`, `ADOPT-ETH-ROLLUP-001`, `ADOPT-HW-API-001`,
`ADOPT-HW-CT-001`, `ADOPT-HW-STATE-001`, `DIR-KAT-001`,
`SPEC-KAT-001`, `DIR-REVIEW-001`, `SPEC-REVIEW-001`, `POL-MSIG-001`,
`POL-THRESH-001`, `POL-RECOVERY-001`, `AGG-ROLLUP-001`, and
`AGG-DA-001`.

Before this go-to-market plan can be treated as externally usable, the repo
needs:

- counsel-reviewed public/non-public disclosure policy;
- one-page institutional pitch;
- customer-facing non-claim language;
- at least one executable demo path;
- conformance/KAT artifact suitable for external review;
- support and certification packaging;
- trademark/certification rules if compatibility marks are used;
- explicit pricing and support boundaries outside this public repo.
