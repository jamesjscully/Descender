# Descender Spec Spine

Status: ACTIVE_NORMATIVE

This directory is the current single source of truth for Descender's product
direction and active protocol claims.

Descender's target is a chain-native post-quantum authorization stack, not a
standalone drop-in signature primitive. The active goal is:

```text
compact PQ spend authorization
+ prevalidated one-use account slots
+ consensus/state binding
+ account policy expressiveness
+ proof aggregation / rollup compatibility
```

## Authority Rules

1. Files in `spec/` define the current normative project direction.
2. `decisions/ADR-*.md` records append-only decisions and supersession history.
3. `TRACKER.md` is the consolidated implementation and proof tracker.
4. `research/` files are supporting evidence, design history, negative controls,
   or active research notes. They are not normative unless this spec spine
   explicitly cites them as active support.
5. Legacy formal specs under `formal/spec/` describe older proof/signature
   lineages unless they are updated to cite this spec spine.

When documents disagree, resolve in this order:

```text
spec/README.md
decisions/ADR-*.md
TRACKER.md
other spec/*.md
active implementation comments/tests
research notes
archived/formal legacy notes
```

## Active Architecture

The selected architecture is Path 2: account creation or slot refresh
prevalidates one-use signing slots before spend time.

Mode A is the first deployable mode:

```text
single slot creation -> direct consensus-visible slot record
single slot spend    -> direct state lookup and consume-once transition
```

Mode B is the scale target:

```text
batch slot creation
+ inclusion or aggregated authorization handles
+ block/rollup proof aggregation
+ light-client verification
```

Mode A must not consume `optional_aggregation_soundness`. Mode B may consume it
only after a concrete accumulator, rollup proof, or batch proof system is named.

## Active Components

| Component | Active direction |
|---|---|
| Account creation | Poseidon2 / Plonky3 / KoalaBear STARK path in `tools/descender-account-stark` |
| Spend authorization | Checked-core spend bound to a normalized `slot_authorization_digest` |
| Slot state | `PrevalidatedSlotLeafV1` plus `spec/lifecycle_model.md` consume-once semantics |
| Conformance | Active Path 2 corpus and KAT rules in `spec/conformance_kats.md` |
| Consensus verification | Deterministic public verifier and rejection model in `spec/consensus_verifier.md` |
| Deployment profiles | Target-chain, rollup, bridge, and Ethereum integration profiles in `spec/deployment_profiles.md` |
| Entropy | Entropy-source and domain-separation stream model in `spec/entropy_profile.md` |
| Ethereum adoption | Account-abstraction, delegated-EOA, secondary-signature, frame-transaction, and rollup profiles in `spec/ethereum_profiles.md` |
| External review | Review package, manifest, lane, and acceptance model in `spec/review_package.md` |
| Leakage evidence | Constant-time and side-channel evidence model in `spec/leakage_profile.md` |
| Signer API | Public command/result boundary for wallets, HSMs, and custody services in `spec/signer_api.md` |
| Signer state | Local state, rollback, restore, and fault model in `spec/signer_state_profile.md` |
| Aggregation | Proof/state aggregation, not native BLS-style signature aggregation |
| Recursion | Poseidon2-friendly STARK recursion path in `tools/descender-recursion` |
| Policy | Account/state policy layer in `spec/policy_model.md` over admitted slots, not Schnorr/BLS algebra |
| Security level | Current q128 parameter floor is the width-16 Poseidon2/Merkle 139-bit floor; deployment claims still require the review and production gates |

## Current Non-Active Paths

The following are not the active product target:

- the 260-byte legacy linear-sigma / KAT v0 path;
- the 308/314-byte Fq12 projection-carrier envelope as the final product story;
- spend-time hidden-witness extraction from the old projection-carrier transcript;
- native BLS-style signature aggregation;
- a raw "replace ECDSA with PQ CHECKSIG" framing.

Those paths may remain in the repo as archived history, regression tests,
negative controls, or formal scaffolding.

## Normative Documents

| File | Role |
|---|---|
| `spec/README.md` | Current authority and active product direction |
| `spec/architecture.md` | Chain-native PQ authorization architecture |
| `spec/blockchain_requirements.md` | Blockchain capability requirements and adoption gates |
| `spec/conformance_kats.md` | Active Path 2 conformance and KAT corpus model |
| `spec/consensus_verifier.md` | Consensus verifier, rejection, and client-mapping model |
| `spec/deployment_profiles.md` | Target-chain, rollup, bridge, and Ethereum deployment profile model |
| `spec/entropy_profile.md` | Entropy-source and domain-separation stream model |
| `spec/ethereum_profiles.md` | Ethereum account-abstraction, delegated-EOA, precompile, frame, and rollup adoption profile model |
| `spec/leakage_profile.md` | Constant-time and leakage evidence model |
| `spec/lifecycle_model.md` | Slot lifecycle, replay, rollback, and recovery model |
| `spec/policy_model.md` | Account policy, multisig, threshold, recovery, and committee model |
| `spec/review_package.md` | External review package, manifest, and lane-acceptance model |
| `spec/security_model.md` | Current security claim shape, assumptions, and non-claims |
| `spec/signer_api.md` | Signer public API and wallet/HSM boundary model |
| `spec/signer_profile.md` | Signer, wallet, HSM, and custody safety profile |
| `spec/signer_state_profile.md` | Signer local state, rollback, and fault model |
| `spec/wire_formats.md` | Active public objects and byte-format authority map |
| `spec/process.md` | Documentation, ADR, tracker, archive, and automation discipline |
| `TRACKER.md` | Consolidated implementation/proof/process tracker |
| `decisions/` | Append-only decision records |

## Selected Supporting Active Material

These files are high-value support references for the active direction, but do
not supersede this spec spine. Every selected support item must be classified
as `ACTIVE_SUPPORT` in `research/ARCHIVE_INDEX.md` and must name the current
spec path, ADR, or tracker row it supports. The broader active/historical
classification map lives in `research/ARCHIVE_INDEX.md`.

| File | Anchor |
|---|---|
| `research/account_creation_prevalidated_slot_plan.md` | `ADR-0002`, `SPEC-REQ-001` |
| `research/prevalidated_slot_leaf_v1_frame.md` | `spec/wire_formats.md`, `DIR-LEAF-001` |
| `research/checked_core_fs_poseidon2_rehost.md` | `DIR-CC-001`, `DIR-QROM-001` |
| `research/path2_account_stark_air_soundness_audit.md` | `DIR-STARK-001` |
| `research/account_creation_evidence_v1_spec.md` | `spec/wire_formats.md`, `DIR-EVID-001` |
| `research/spend_authorization_relation_v1_spec.md` | `spec/wire_formats.md`, `DIR-SPEND-001` |
| `research/path2_one_use_successor_state_theorem.md` | `spec/lifecycle_model.md`, `DIR-LIFE-001` |
| `research/account_creation_prevalidated_slot_authorization_or_break.md` | `spec/security_model.md`, `DIR-SEC-001` |
| `research/path2_hash_qrom_assumption_ledger.md` | `spec/security_model.md`, `DIR-QROM-001` |
| `research/path2_spend_closure_final_composition_v1.md` | `spec/security_model.md`, `DIR-SEC-001` |
| `research/account_creation_aggregation_readiness_contract.md` | `AGG-MODE-001`, `AGG-BATCH-001` |
| `research/path2_proof_side_closure_status_v1.md` | `DIR-SEC-001`, `DIR-REVIEW-001` |
| `research/path2_proof_family_aggregation_recursion_analysis.md` | `AGG-MODE-001`, `AGG-ROLLUP-001` |
| `research/path2_recursion_air_composition_architecture.md` | `AGG-ROLLUP-001` |
| `research/path2_recursion_circuit_concrete_design.md` | `AGG-ROLLUP-001`, `ADOPT-ETH-ROLLUP-001` |
| `research/ethereum_account_abstraction_positioning.md` | `ADOPT-ETH-001` |
