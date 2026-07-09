# External Review Package

Status: ACTIVE_NORMATIVE

This file defines the required shape of a Descender external review package.
Internal tests, internal red-team notes, and generated research artifacts can
support the package, but they do not replace independent review.

## Scope

An external review package covers:

- the direct-mode theorem and named break classes;
- coded-SIS/lattice-core assumptions and response-link evidence;
- Poseidon2, FRI, Fiat-Shamir, QROM, and domain-separation framing;
- account-creation proof-family soundness and zero-knowledge claims;
- canonical wire formats, KATs, parser behavior, and deterministic rejection;
- lifecycle, finality, rollback, data availability, and deployment profile
  assumptions;
- signer, wallet, HSM, custody, leakage, and fault behavior;
- Mode B aggregation, rollup, bridge, and light-client claims if those claims
  are in scope.

The package must state which claims are in scope, which are explicitly out of
scope, and which residual assumptions remain after review.

## Review Lanes

External review is not one undifferentiated sign-off. A deployment claim needs
lane-specific evidence.

| Lane | Required reviewer focus |
|---|---|
| cryptographic core | coded-SIS/lattice assumptions, exact response-link certificate, beyond-box hardness, and parameter margins |
| proof system | account-creation proof family, Poseidon2/FRI parameters, extraction, simulation, and soundness composition |
| QROM/hash framing | Fiat-Shamir transcript, domain separation, multi-target ledger, action-map separation, and query bounds |
| consensus engineering | canonical bytes, parser totality, deterministic rejection, state-no-mutation on reject, and independent-client reproducibility |
| lifecycle/deployment | target-chain profile, finality, reorg, rollback, data availability, gas/resource limits, and storage atomicity |
| signer/custody | secret-side arithmetic, leakage, fault behavior, rollback state, API errors, and diagnostic-build separation |
| aggregation/policy | Mode B proof assumptions, ordering, duplicate prevention, policy semantics, committee weights, and light-client proof semantics |

A reviewer may cover multiple lanes, but the package must record which lane each
finding or acceptance applies to.

## Package Contents

A review package must contain:

- current `spec/` authority snapshot and ADR index;
- tracker snapshot identifying all rows in scope and their status;
- canonical wire-format specification and generated KAT corpus;
- direct-mode theorem statement and named break mapping;
- assumption ledger distinguishing proven, named-standard, and external items;
- reproducible scripts or manifests for local certificates and generated
  artifacts;
- negative-control and red-team summary with current disposition;
- deployment profile, if a deployment claim is in scope;
- Ethereum adoption profile, if account-abstraction, delegated-EOA, precompile,
  frame-transaction, or Ethereum rollup claims are in scope;
- signer/custody profile and leakage/fault evidence, if wallet/HSM/custody
  claims are in scope;
- entropy-source and domain-separation profile evidence, if signer or custody
  claims are in scope;
- constant-time, side-channel, fixed-shape failure, and target/build-profile
  evidence, if signer, proof-generation, wallet/HSM, or custody claims are in
  scope;
- signer-state rollback, restore-anchor, crash/retry, and finality evidence, if
  wallet/HSM/custody claims are in scope;
- explicit non-claims and rejected/superseded paths;
- reviewer checklist and expected outputs.

Historical research packages such as `research/cryptanalysis/SECURITY_DOSSIER.md`
may be imported as evidence, but the current review package must route through
the active spec spine and `TRACKER.md`.

## Manifest And Reproducibility

Every package must include a manifest with:

| Field | Requirement |
|---|---|
| `package_id` | stable package identifier |
| `package_version` | monotonic package version or date |
| `authority_snapshot` | spec, ADR, tracker, and relevant implementation revisions |
| `claim_scope` | direct, Mode B, policy, deployment, signer/custody, or other reviewed scope |
| `artifact_hashes` | hashes for package files, generated KATs, and machine-check outputs |
| `reproduction_commands` | commands required to rebuild or verify generated artifacts |
| `review_lanes` | review lanes included and omitted |
| `residual_assumptions` | assumptions left open after local evidence |
| `reviewer_outputs` | expected reviewer reports, acceptances, objections, or reproduction logs |

The manifest must fail closed: if a generated artifact cannot be reproduced, or
if a reviewer output changes a tracked assumption, the package cannot support a
deployment-ready claim until the tracker and specs are updated.

The current fail-closed direct-mode draft is
`formal/kat/path2_review_package_v1.json`, checked by
`scripts/check_path2_review_package.py` in both the local and CI gates. It is a
review-candidate manifest only: it hashes the current authority snapshot and
boundary artifacts, names the included and omitted review lanes, and records the
open residual assumptions that still block deployment or whole-system approval.

## Acceptance Rules

External review can support readiness only when:

1. every in-scope claim maps to current `spec/` authority and `TRACKER.md` rows;
2. every in-scope row needed for deployment readiness is `verified` or is
   explicitly marked as an accepted residual assumption;
3. each accepted residual assumption is named in `spec/security_model.md` or the
   relevant deployment/profile spec;
4. every reviewer objection has a tracker row, ADR, or explicit rejected
   disposition;
5. generated artifacts reproduce from the manifest;
6. no superseded tracker, archived target, or old review packet is imported as
   authority without current-spec routing;
7. the final review status says whether the package supports direct Mode A,
   Mode B, signer/custody, deployment-profile, or policy claims.

An external review that covers only one lane must not be reported as whole-system
approval.

## Required Evidence

Tracker ownership: `SPEC-REVIEW-001`, `DIR-REVIEW-001`, `DIR-SEC-001`,
`DIR-QROM-001`, `DIR-ACTION-001`, `DIR-KAT-001`, `DIR-REJECT-001`,
`DIR-LIFE-001`, `DIR-CT-001`, `SPEC-DEPLOY-001`, `SPEC-SIGNER-001`,
`SPEC-LEAKAGE-001`, `AGG-MODE-001`, `AGG-ROLLUP-001`,
`AGG-COMMITTEE-001`, `POL-REQ-001`, `ADOPT-HW-ENT-001`, `SPEC-STATE-001`,
`SPEC-ETH-001`, and `ADOPT-128-001`.

Before external-review readiness can be marked `verified`, the repo needs:

- current review package manifest and verifier;
- canonical spec/ADR/tracker snapshot used by the package;
- direct-mode KAT and negative-control corpus linked from the package;
- proof, QROM, lifecycle, verifier, signer, and deployment-profile evidence
  linked by lane;
- reviewer outputs or reproduction logs for each in-scope lane;
- tracker/ADR disposition for every reviewer objection and residual assumption;
- explicit statement that old research packets are evidence only unless imported
  by current authority.

Until these artifacts exist and independent review is complete, external review
remains below `verified` in `TRACKER.md` and below `ready` in
`spec/blockchain_requirements.md`.
