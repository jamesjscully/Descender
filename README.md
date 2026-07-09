# Descender

Status: ACTIVE_REPO_ENTRYPOINT

Descender is being pivoted into a chain-native post-quantum authorization stack.
It is not currently presented as a standalone drop-in signature primitive or as a
deployment-ready cryptographic system.

The active product direction is:

```text
compact PQ spend authorization
+ prevalidated one-use account slots
+ consensus/state binding
+ account policy expressiveness
+ proof aggregation / rollup compatibility
```

## Start Here

Use these files as the current authority chain:

| File | Role |
|---|---|
| `spec/README.md` | current product direction and authority rules |
| `spec/architecture.md` | chain-native authorization architecture |
| `spec/blockchain_requirements.md` | blockchain capability requirements and adoption gates |
| `spec/conformance_kats.md` | active Path 2 conformance and KAT corpus model |
| `spec/consensus_verifier.md` | consensus verifier, rejection, and client-mapping model |
| `spec/deployment_profiles.md` | target-chain, rollup, bridge, and Ethereum deployment profile model |
| `spec/entropy_profile.md` | entropy-source and domain-separation stream model |
| `spec/ethereum_profiles.md` | Ethereum account-abstraction, delegated-EOA, precompile, frame, and rollup adoption profiles |
| `spec/leakage_profile.md` | constant-time and leakage evidence model |
| `spec/lifecycle_model.md` | slot lifecycle, replay, rollback, and recovery model |
| `spec/policy_model.md` | account policy, multisig, threshold, recovery, and committee model |
| `spec/review_package.md` | external review package, manifest, and lane-acceptance model |
| `spec/security_model.md` | security claim shape, assumptions, and non-claims |
| `spec/signer_api.md` | signer public API and wallet/HSM boundary model |
| `spec/signer_profile.md` | signer, wallet, HSM, and custody safety profile |
| `spec/signer_state_profile.md` | signer local state, rollback, and fault model |
| `spec/wire_formats.md` | active public object and legacy format map |
| `spec/process.md` | documentation, ADR, tracker, archive, and automation discipline |
| `AGENTS.md` | quick handoff note for coding and research agents |
| `decisions/` | append-only architecture decision records |
| `TRACKER.md` | consolidated implementation, proof, process, and adoption tracker |
| `research/ARCHIVE_INDEX.md` | labels for historical and supporting research docs |

When older research notes disagree with `spec/` or the ADRs, treat the older
notes as history unless the current spec explicitly cites them as active support.

## Supporting Entrypoints

These entrypoints route implementation, research, audit, and operating material
back to the authority chain above. They are not product authority by themselves.

| File | Role |
|---|---|
| `research/README.md` | research notes, generated reports, and automation state |
| `audit/README.md` | audit outputs, red-team notes, and proof-obligation history |
| `tools/README.md` | implementation packages, research engines, and legacy baselines |
| `formal/README.md` | formal verification layers and legacy formal scaffolding |
| `legal/README.md` | legal and commercialization planning support |
| `scripts/research/README.md` | local research automation controls and routing rules |

## Active Path

The selected architecture is Path 2: account creation or slot refresh
prevalidates one-use signing slots before spend time. Spend authorization then
binds a checked-core spend to a canonical `slot_authorization_digest` and a
consume-once state transition.

Mode A is the first deployable target: a direct single-slot creation/spend path
with consensus-visible state. Mode B is the scale target: batch creation,
recursive/rollup proof aggregation, and light-client-friendly authorization
handles.

## Non-Active Product Paths

The following remain useful as history, formal scaffolding, executable baselines,
or negative controls, but they are not the current product target:

- the 260-byte legacy linear-sigma / KAT v0 path;
- the 308/314-byte Fq12 projection-carrier envelope as the final product story;
- spend-time hidden-witness extraction from the old projection-carrier transcript;
- native BLS-style signature aggregation;
- a raw "replace ECDSA with PQ CHECKSIG" framing.

## Local Checks

Run the documentation authority check after editing specs, ADRs, trackers, or
high-confusion research docs:

```sh
python3 scripts/check_doc_authority.py
python3 scripts/check_adr_immutable_sections.py
python3 scripts/check_process_gate_coverage.py
python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown
python3 scripts/check_archive_mixed_status.py --max-mixed 0
python3 scripts/check_archive_legacy_counts.py --max-non-active 10 --max ARCHIVED_TARGET=0 --max LEGACY_FORMAL=4 --max MIXED_STATUS=0 --max NEGATIVE_CONTROL=6 --max SUPERSEDED_DECISION=0 --max SUPERSEDED_TRACKER=0
python3 scripts/check_cleanup_queue.py
python3 scripts/check_retired_path_absence.py
python3 scripts/check_active_support_imports.py --max-unanchored 0
python3 scripts/check_active_authority_stale_terms.py
python3 scripts/check_legacy_code_framing.py
python3 scripts/check_tool_manifest_framing.py
python3 scripts/check_active_source_framing.py
python3 scripts/check_active_secret_float_surface.py
python3 scripts/research/check_research_automation_routing.py
python3 scripts/research/audit_task_pivot_alignment.py --strict-queue
python3 scripts/check_path2_review_package.py
python3 scripts/check_ci_local_gate_parity.py
python3 scripts/check_tracker_integrity.py
python3 scripts/check_requirement_traceability.py
python3 scripts/check_wire_format_authority.py
```

The full local build gate is:

```sh
./scripts/check_build.sh
```

Active Path 2 Rust tests are split by runtime:

```sh
./scripts/check_active_rust_fast_tests.sh
./scripts/check_active_rust_slow_tests.sh
```

The fast script is part of the full local/CI gate. The slow script is explicit
manual coverage for empirical hiding batteries, deployed verifier integration,
and the larger recursion verifier-AIR suite.
