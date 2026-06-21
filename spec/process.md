# Descender Process Discipline

Status: ACTIVE_NORMATIVE

This file defines the repo process for the pivot to a chain-native
post-quantum authorization stack. It is intentionally operational: it tells
contributors where a claim belongs and how stale paths stay from becoming
accidental authority again.

## Authority Chain

Current product claims must flow through this chain:

```text
spec/README.md
decisions/ADR-*.md
TRACKER.md
other spec/*.md
active implementation comments/tests
research notes
archived/formal legacy notes
```

Research notes can motivate or support a claim, but they do not become product
authority unless the spec spine or an ADR explicitly imports them.

## Specification Rules

- Keep the top-level `README.md` and `spec/README.md` framing synchronized with
  the active chain-native post-quantum authorization objective, the active
  product direction list, and the non-active product path list.
- Keep the top-level `README.md` `## Start Here` table synchronized with every
  `spec/*.md` file, `decisions/`, `TRACKER.md`, and
  `research/ARCHIVE_INDEX.md`.
- Keep the top-level `README.md` `## Supporting Entrypoints` table synchronized
  with routed non-authority entrypoints for research, audit, tools, formal,
  legal, and research automation.
- Each supporting entrypoint listed by the top-level `README.md` must route
  readers back to `spec/README.md` and to the ADR or `TRACKER.md` authority
  layer, so existence in the table cannot make a support directory look
  independently authoritative.
- Put stable current product claims in `spec/`.
- Keep blockchain capability requirements and adoption gates in
  `spec/blockchain_requirements.md`.
- Every blockchain capability row, adoption-gate traceability row, and
  requirement-surface row must cite the `TRACKER.md` rows that carry the work.
- Keep active Path 2 conformance, KAT corpus, vector-versioning, and independent
  implementation reproducibility requirements in `spec/conformance_kats.md`.
- Keep deterministic consensus verification, public rejection classes, and
  consensus-client failure mapping in `spec/consensus_verifier.md`.
- Keep target-chain, rollup, bridge, Ethereum, gas/resource, finality, and data
  availability deployment-profile requirements in `spec/deployment_profiles.md`.
- Keep entropy-source classes, domain-separated stream registry, deterministic
  test-mode separation, and entropy evidence requirements in
  `spec/entropy_profile.md`.
- Keep Ethereum account-abstraction, delegated-EOA, secondary-signature,
  frame-transaction, and rollup adoption-profile requirements in
  `spec/ethereum_profiles.md`.
- Keep constant-time, side-channel, fixed-shape failure, target/build profile,
  and leakage-evidence requirements in `spec/leakage_profile.md`.
- Keep external review package, manifest, lane, and acceptance requirements in
  `spec/review_package.md`.
- Keep signer public command/result classes, wallet/HSM boundary rules,
  diagnostic-build separation, and transport-profile API requirements in
  `spec/signer_api.md`.
- Keep signer-local persistence, rollback, restore, crash/retry, finality, and
  fault-state evidence requirements in `spec/signer_state_profile.md`.
- Adoption-gate and surface readiness in `spec/blockchain_requirements.md` must
  be one of `not_ready`, `partial`, or `ready`; a `ready` row requires all cited
  tracker rows to be `verified`.
- Keep slot lifecycle, replay, rollback, and recovery semantics in
  `spec/lifecycle_model.md`.
- Keep account-policy, multisig, threshold custody, recovery, adaptor-style,
  validator-quorum, and bridge-committee semantics in `spec/policy_model.md`.
- Keep active wire/object commitments in `spec/wire_formats.md`.
- Keep signer, wallet, HSM, and custody safety requirements in
  `spec/signer_profile.md`.
- Keep current assumptions, theorem shapes, and non-claims in
  `spec/security_model.md`.
- Keep the `spec/README.md` `## Normative Documents` table synchronized with
  every `spec/*.md` file, `TRACKER.md`, and `decisions/`.
- The documentation authority check discovers `spec/*.md` dynamically. Adding or
  removing a normative spec file should require updating the authority tables
  and status markers, not hard-coded checker file lists.
- Do not update legacy formal specs to look current. Add a banner and link to
  the active spec instead.
- If supporting material is promoted into the product story, cite it from
  `spec/README.md` under selected supporting active material and add it to
  `research/ARCHIVE_INDEX.md`.
- Selected supporting active material in `spec/README.md` must be an anchored
  table. Each row must be classified with an active label in
  `research/ARCHIVE_INDEX.md` and must name the current spec file, ADR, or
  `TRACKER.md` row that imports the support note.
- `ACTIVE_SUPPORT` archive rows should be directly imported by current
  authority text in `spec/`, ADRs, or `TRACKER.md`. During migration, the active
  support import audit is allowed a fixed unanchored backlog ceiling; new
  unanchored support rows should either be imported by authority, downgraded to
  `ACTIVE_RESEARCH`, or given a concrete tracker/ADR retirement path.
- Normative spec sections that describe required evidence or open deployment
  work must include tracker ownership rather than leaving the work as prose-only
  TODO material.
- Active support files with `spec` in the filename but outside `spec/` are
  supporting spec notes, not normative specs. They must say near the top that
  they are not normative or do not supersede the spec spine, and must point to
  `spec/README.md`.

## Decision Rules

ADRs are append-only. A changed decision gets a new ADR; old ADRs are not edited
to make history look cleaner.

Every ADR must have:

- a filename `ADR-NNNN-short-name.md`;
- a matching `# ADR-NNNN: Title` heading;
- `Status:` and `Date:` fields near the top;
- `## Context`, `## Decision`, and `## Consequences` sections;
- `## Supersedes`, `## Superseded By`, and
  `## Affected Specs / Tracker Items` sections.

Use `None.` when an ADR has no superseded predecessor or successor. If a decision
changes, add a new ADR and update the old ADR's `## Superseded By` section rather
than rewriting the old decision.

The `## Affected Specs / Tracker Items` section must cite current repo paths,
`ADR-NNNN` records, or `TRACKER.md` row IDs in backticks. The documentation
authority check resolves those references so ADRs cannot silently point at stale
or misspelled authority. If an ADR names a `TRACKER.md` row, that tracker row
must reciprocally cite the ADR number so decision impact is visible from both
the decision log and the consolidated tracker.

For final ADRs (`Accepted`, `Superseded`, or `Rejected`), the `## Context`,
`## Decision`, and `## Consequences` sections are immutable decision text.
Their hashes must be listed in `decisions/ADR_IMMUTABLE_SECTIONS.sha256`.
Changing those sections should require a new ADR; lifecycle fields such as
status, supersession, and affected tracker references remain mutable routing
metadata.

`decisions/README.md` must include an `## ADR Index` table listing every ADR by
number, status, date, title, and filename. The documentation authority check
fails if that index drifts from the actual ADR files.

Accepted decision status values are `Accepted`, `Proposed`, `Rejected`, and
`Superseded`.

Active support files with `decision` in the filename are research decision notes,
not append-only ADRs. They must say near the top that they are not ADRs or not
binding by themselves, and must point to `decisions/` or ADRs for binding
project decisions.

## Tracker Rules

`TRACKER.md` is the only top-level implementation/proof/process tracker. Old
trackers are evidence only unless a row in `TRACKER.md` links to them.

Active support files with tracker, todo, status, or checklist style names, or
with a `Status:` line that names a tracker, todo, status note, or checklist
role, must say near the top that they are subordinate or not top-level, and
must point to `TRACKER.md` as the top-level status source. `TRACKER.md` must
also link to each active subordinate detail doc, so the consolidated tracker is
the entrypoint for that detail rather than relying on reverse discovery from
research files.

Tracker rows must use one of:

```text
proposed
selected
in_progress
implemented
verified
blocked
archived
superseded
```

The consolidated tracker's top-level work sections and row-ID prefixes are
fixed:

| Section | Allowed row-ID prefixes |
|---|---|
| Direct Mode: First Deployable Target | `SPEC-*`, `ADR-*`, `DIR-*` |
| Aggregation / Rollup Mode | `AGG-*` |
| Policy Layer | `POL-*` |
| Process / Cleanup | `PROC-*` |
| Hardening / Adoption | `ADOPT-*` |

Use `implemented` when code/docs exist but the evidence is not yet sufficient
for the whole row. Use `verified` only when the named tests, proof artifacts, or
review evidence cover the stated item. A `verified` tracker row must name
concrete evidence such as a check, gate, test, proof, script, audit, or CI-wired
artifact.

Rows with `proposed`, `selected`, `in_progress`, `implemented`, or `blocked`
status must name a concrete next action or blocking dependency in the
evidence/next-action cell. Passive placeholders such as a bare ADR, a topic
list, or an imported support file are not enough for live planning rows.

Rows with `archived` or `superseded` status must name historical retention,
negative-control value, or replacement routing in the evidence/next-action cell.

## Archive Rules

`research/ARCHIVE_INDEX.md` classifies high-confusion files and historical
packages. A file or package needs an archive entry if it can be mistaken for:

- the current product target;
- the current security theorem;
- the current implementation/proof tracker;
- a still-binding decision;
- a current deployability or third-party-review package.

The archive index must also contain a `## Retention Policy` table with one row
for every recognized archive label. Classification is not only a warning label;
it is the default cleanup disposition. The documentation authority check keeps
the label-to-retention/removal rows synchronized with the checked process
contract. Use the label's retention policy to decide whether a stale path is
kept as active support, research evidence, negative-control evidence, legacy
conformance history, archive-only history, or candidate removal after tracker
or ADR retirement.

Use exact file rows for individual files and `path/**` rows for coherent
packages. Package rows require a `README.md` in that package, and active package
READMEs must carry either an explicit `Status: ACTIVE_...` line or a
`MIGRATION STATUS` banner near the top.

Markdown files with tracker, todo, status, checklist, or plan-style names that
are covered only by a `path/**` package archive row must carry a local
package-context banner near the top. The banner must point to
`research/ARCHIVE_INDEX.md`, state that the file is package-scoped,
historical, not current, not top-level, or not product authority, and route
readers to `spec/README.md`, ADRs, or `TRACKER.md`.

Ignored vendored external tools under `tools/external/` may use an upstream
`README.rst` as the package entrypoint when no `README.md` exists. In that case,
Descender-specific routing belongs in `tools/README.md` and
`research/ARCHIVE_INDEX.md`; do not patch third-party upstream documentation
only to carry Descender status text.

Active README entrypoint rows in `research/ARCHIVE_INDEX.md` must carry either
an explicit `Status: ACTIVE_...` line or a `MIGRATION STATUS` banner near the top
of the file. A classified active package entrypoint should not rely on unstated
context to show that it is support or research evidence rather than product
authority. It must also route readers back to `spec/README.md` and to the ADR
or `TRACKER.md` authority layer.

Supporting entrypoints listed in the top-level `README.md` carry explicit
entrypoint-role `Status: ACTIVE_*_ENTRYPOINT` markers when their route is
mechanically fixed. Those markers identify routing responsibility; the archive
classification in `research/ARCHIVE_INDEX.md` remains the source of truth for
whether the material is active support, active research, or legacy material.

`Status: ACTIVE_*` markers are reserved authority/routing markers. New active
status markers must be added deliberately to the documentation authority check;
do not invent ad hoc active status names inside research notes.

Legacy rows with `LEGACY_FORMAL`, `ARCHIVED_TARGET`, `SUPERSEDED_TRACKER`,
`SUPERSEDED_DECISION`, `MIXED_STATUS`, or `NEGATIVE_CONTROL` must also have a
visible `MIGRATION STATUS` banner near the top of the file. For `path/**`
package rows, put that banner near the top of the package `README.md`.
Legacy labels that imply a replacement authority must name that authority near
the top as well: archived targets and legacy formal material route to
`spec/README.md`, superseded trackers route to `TRACKER.md`, superseded
decisions route to ADRs/`decisions/`, and mixed-status material must name a
current spec, ADR, or tracker authority.

Any `MIGRATION STATUS` banner must name a recognized archive label and must
match the corresponding `research/ARCHIVE_INDEX.md` row. The archive index is
the source of truth for classification; banners are visible routing copies, not
separate status records.

Audit notes under `audit/` are evidence unless a current spec, ADR, or
`TRACKER.md` row imports them. The directory entrypoint must route readers back
to the active authority chain, because older audit notes may use historical
"active path" or "deployable target" language. `audit/README.md` must carry the
`Status: ACTIVE_AUDIT_ENTRYPOINT` marker so the documentation authority check can
fail if that routing entrypoint is removed or demoted accidentally.

`formal/README.md` and `legal/README.md` must likewise carry
`Status: ACTIVE_FORMAL_ENTRYPOINT` and `Status: ACTIVE_LEGAL_ENTRYPOINT`
markers. These are routing markers only; formal proof architecture and
legal/commercialization planning are not product authority unless imported by
the current spec spine, ADRs, or `TRACKER.md`.

## Tool Package Rules

`tools/README.md` is the routing entrypoint for implementation packages,
research engines, and legacy baselines under `tools/`. It is not product
authority; it points readers back to the spec spine, ADRs, and `TRACKER.md`.

Every direct tool crate or direct tool package README must be listed in the
`tools/README.md` `## Tool Packages` table. The documentation authority check
discovers these packages dynamically, so adding a new crate or package README
requires updating that table.

Tool package READMEs may describe local APIs and commands, but stable protocol,
security, and adoption claims belong in `spec/`, ADRs, or `TRACKER.md`.

## Legacy Verification Rules

Legacy test gates may remain in CI only when they preserve conformance,
negative-control, archive-reproducibility, or regression evidence. Their labels
must say that role directly. A legacy q12289, KAT v0, projection-carrier, or old
linear-sigma test must not be named as if it were the current deployable product
path.

Legacy deployment-shaped code may keep old symbol names when those names are
part of existing regression, digest, or source-inspection evidence. In that
case, nearby code or crate-level documentation must say the surface is
fail-closed, retained for regression or negative-control evidence, and not
current product authority.

Removing a legacy gate is allowed only after its evidence value is either moved
to an active-path KAT/regression test or explicitly retired by tracker row or
ADR. Keeping a legacy gate does not promote that path back into the product
target.

## Automation Rules

Research automation must default to the current authority chain. Generated tasks
must be marked `pivot_aligned: true` or clearly cite active Path 2, spec, ADR,
or tracker work. Legacy KEM/descent tasks require the explicit
`DESCENDER_ALLOW_LEGACY_TASKS=1` override.

Legacy artifact generators under `scripts/research/` are historical reproducers
unless `scripts/research/README.md` or a current tracker row frames their use as
archive cleanup, negative-control preservation, fail-closed regression evidence,
or migration into the current authority chain.

Queue import and dispatch must share the same pivot guard so stale enabled queue
files cannot silently re-enter active research state.

Local pre-pivot task state should be moved to `pivot_archived` with
`scripts/research/quarantine_non_pivot_state.py` rather than deleted, so the
state history remains append-only enough for audit while the scheduler stops
treating it as active work.

The active-control script list in `scripts/research/README.md` is exact and
mechanically checked. Active import, dispatch, launch, review, quarantine, and
artifact-sync scripts must route through the pivot guard or carry explicit
current-authority prompt text. Scripts outside that table are legacy helpers,
artifact reproducers, negative-control generators, or migration tools; their
count is baseline-capped by the automation routing check and should ratchet
downward as old generators are removed or reclassified.

## Process Gate Coverage

This table is checked by `scripts/check_process_gate_coverage.py`. Every process
area must name local/CI gates and the active `TRACKER.md` rows that own them.

| Area | Gates | Tracker owners |
|---|---|---|
| Process spec coverage | `python3 scripts/check_process_gate_coverage.py` | `PROC-PROCESS-001` |
| Authority entrypoints | `python3 scripts/check_doc_authority.py` | `PROC-ENTRY-001`, `PROC-ENTRY-002`, `PROC-ENTRY-003`, `PROC-ENTRY-004`, `PROC-ENTRY-005` |
| Spec spine | `python3 scripts/check_doc_authority.py`, `python3 scripts/check_requirement_traceability.py` | `PROC-SPEC-001`, `PROC-SPEC-003`, `PROC-SPEC-004`, `PROC-SPEC-005` |
| ADR process | `python3 scripts/check_doc_authority.py`, `python3 scripts/check_adr_immutable_sections.py` | `PROC-ADR-001`, `PROC-ADR-002`, `PROC-ADR-004`, `PROC-ADR-005`, `PROC-ADR-006` |
| Tracker consolidation | `python3 scripts/check_tracker_integrity.py`, `python3 scripts/check_active_support_imports.py --max-unanchored 0`, `python3 scripts/check_requirement_traceability.py` | `PROC-TRACKER-001`, `PROC-TRACKER-002`, `PROC-TRACKER-003`, `PROC-TRACKER-004`, `PROC-TRACKER-005`, `PROC-SUPPORT-001` |
| Archive and classification | `python3 scripts/check_doc_authority.py`, `python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown`, `python3 scripts/check_archive_mixed_status.py --max-mixed 0`, `python3 scripts/check_archive_legacy_counts.py --max-non-active 10 --max ARCHIVED_TARGET=0 --max LEGACY_FORMAL=4 --max MIXED_STATUS=0 --max NEGATIVE_CONTROL=6 --max SUPERSEDED_DECISION=0 --max SUPERSEDED_TRACKER=0`, `python3 scripts/check_cleanup_queue.py`, `python3 scripts/check_retired_path_absence.py` | `PROC-ARCHIVE-001`, `PROC-ARCHIVE-002`, `PROC-ARCHIVE-003`, `PROC-ARCHIVE-004`, `PROC-ARCHIVE-005`, `PROC-ARCHIVE-006`, `PROC-ARCHIVE-007`, `PROC-ARCHIVE-008`, `PROC-CLASSIFY-010` |
| Stale-path framing | `python3 scripts/check_active_authority_stale_terms.py`, `python3 scripts/check_legacy_code_framing.py` | `PROC-STALE-001`, `PROC-STALE-002`, `PROC-LEGACY-GATE-001` |
| Tool and source framing | `python3 scripts/check_tool_manifest_framing.py`, `python3 scripts/check_active_source_framing.py` | `PROC-MANIFEST-001`, `PROC-SOURCE-001`, `PROC-TOOLS-001` |
| Research automation | `python3 scripts/research/check_research_automation_routing.py`, `python3 scripts/research/audit_task_pivot_alignment.py --strict-queue` | `PROC-AUTO-001`, `PROC-AUTO-002`, `PROC-AUTO-003`, `PROC-SCRIPT-001` |
| Wire and requirement authority | `python3 scripts/check_requirement_traceability.py`, `python3 scripts/check_wire_format_authority.py` | `PROC-REQ-001`, `DIR-CC-001` |
| CI and gate ownership | `python3 scripts/check_ci_local_gate_parity.py` | `PROC-CI-001`, `PROC-CI-002` |

## Required Checks

Run this after changing specs, ADRs, trackers, archive labels, or high-confusion
research docs:

```sh
python3 scripts/check_doc_authority.py
python3 scripts/check_adr_immutable_sections.py
python3 scripts/check_process_gate_coverage.py
python3 scripts/check_tracker_integrity.py
```

The process gate coverage check verifies the `## Decision Rules` ADR process
contract, the `## Tracker Rules` tracker status/prefix contract, and the
`## Process Gate Coverage` table against local gates, CI gates, and active
tracker owners.

The documentation authority check verifies active status markers, the top-level
`README.md` chain-native framing and non-active path list, the `spec/README.md`
normative-document index, the ADR process contract in `decisions/README.md`,
the ADR index, archive banners, and active support references. It also verifies
that every recognized archive label has an explicit retention/disposition
policy matching the checked label contract, and that selected supporting active
material rows in `spec/README.md` are classified as `ACTIVE_SUPPORT` and cite
existing spec paths, ADRs, or `TRACKER.md` rows. ADRs that name affected tracker
rows must also be cited back from those tracker rows.

The ADR immutable-section check verifies
`decisions/ADR_IMMUTABLE_SECTIONS.sha256` against every final ADR's `Context`,
`Decision`, and `Consequences` sections. Use
`python3 scripts/check_adr_immutable_sections.py --print-current` to review the
current manifest after adding a new final ADR; any manifest update should be
reviewed as a decision-history change.

It derives the active spec spine from current `spec/*.md` files, so the gate
checks new spec files for authority-table entries and normative status markers
without script-code updates.

To see the remaining high-risk Markdown files that are not yet classified in the
archive index, run:

```sh
python3 scripts/list_doc_classification_candidates.py
```

The local and CI gates run this inventory in strict mode, including explicit
`Status:` lines, every `research/path2*.md` note, and every root-level
`research/cryptanalysis/*.md` security note. The gate also requires every other
Markdown file outside ignored generated/state directories to be either part of
the active authority set or covered by `research/ARCHIVE_INDEX.md`:

```sh
python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown
```

The active-support import audit prevents `ACTIVE_SUPPORT` rows from growing as
free-floating support claims. An active-support row is anchored only when a
current authority file cites its path as a literal Markdown code span; incidental
plain-text mentions do not count. `Accepted` and `Proposed` ADRs count as
current decision importers; `Superseded` and `Rejected` ADRs remain decision
history and do not anchor `ACTIVE_SUPPORT` rows. The gate currently requires
zero unanchored rows, so new `ACTIVE_SUPPORT` rows must be imported immediately
or the build fails:

```sh
python3 scripts/check_active_support_imports.py --max-unanchored 0
```

Use `--list` to inspect unanchored rows and `--show-importers` to inspect the
authority file locations that import each anchored support row.

The archive mixed-status audit prevents `MIXED_STATUS` rows from growing as a
permanent ambiguity bucket. It checks that each mixed row carries
routing/cleanup language and enforces the current migration baseline:

```sh
python3 scripts/check_archive_mixed_status.py --max-mixed 0
```

Use `--list` to inspect the mixed-status backlog. The baseline should ratchet
downward as mixed material is split into active support, active research,
negative controls, or archived targets.

The archive legacy-count audit prevents non-active archive inventory from
growing silently. It enforces the current baseline for archived targets, legacy
formal material, negative controls, superseded decisions, and superseded
trackers. It also rejects exact imports of `SUPERSEDED_TRACKER` and
`SUPERSEDED_DECISION` archive rows from the current authority chain, so old
status files cannot become active by being cited from `spec/`, ADRs, or the
consolidated tracker:

```sh
python3 scripts/check_archive_legacy_counts.py --max-non-active 10 --max ARCHIVED_TARGET=0 --max LEGACY_FORMAL=4 --max MIXED_STATUS=0 --max NEGATIVE_CONTROL=6 --max SUPERSEDED_DECISION=0 --max SUPERSEDED_TRACKER=0
```

Use `--list` to inspect the retained legacy rows. The total and per-label
baselines should ratchet downward as old paths are removed, merged into active
authority, or retired by tracker row or ADR.

The cleanup queue check makes that retirement path explicit. Every non-active
archive row must have exactly one matching row in `research/CLEANUP_QUEUE.md`,
with a cleanup disposition, evidence or blocker text, and at least one active
`TRACKER.md` owner row. ADR references may supplement owner context, but they
do not replace live tracker ownership. The queue also declares a disposition
vocabulary, a label-to-disposition compatibility table, and the owner rule, and
the checker rejects drift between those tables/rules and the accepted cleanup
contract. Retained entries must carry an `until` or `while` exit condition, and
remove/relocate entries must carry an `after` or `once` exit condition:

```sh
python3 scripts/check_cleanup_queue.py
```

Retired stale paths that have been deleted during the migration are separately
gated so old trackers, pre-pivot plans, archived review-readiness packages, and
retired generated probes cannot silently reappear. The retired-path registry is
sorted by path, assigns every retired path to a bounded retirement category, and
ties every entry to an active `TRACKER.md` owner row:

```sh
python3 scripts/check_retired_path_absence.py
```

The classification inventory derives the active authority set from the current
`spec/*.md` spine plus fixed repo entrypoints, so adding or removing spec files
does not require editing the inventory script.

To inspect the same status-line coverage without failing, run the advisory
inventory:

```sh
python3 scripts/list_doc_classification_candidates.py --include-status-lines
```

The advisory status-line backlog should remain zero. If it is nonzero, classify
the file in `research/ARCHIVE_INDEX.md` or remove the stale status claim.

Path 2 research notes should also remain fully classified. To inspect that
coverage directly, run:

```sh
python3 scripts/list_doc_classification_candidates.py --include-path2-research
```

Root-level cryptanalysis notes should also remain fully classified because
filenames such as security dossiers, assumption ledgers, reduction notes, and
red-team reports are easy to mistake for current authority. To inspect that
coverage directly, run:

```sh
python3 scripts/list_doc_classification_candidates.py --include-cryptanalysis-root
```

To inspect complete Markdown classification coverage without failing, run:

```sh
python3 scripts/list_doc_classification_candidates.py --include-all-markdown
```

The gates also run a stale-term guard over active authority docs:

```sh
python3 scripts/check_active_authority_stale_terms.py
```

This does not ban historical mentions. It requires references to legacy KEM,
projection-carrier, and 260/308/314-byte target lines to stay in explicit
non-active, archive, or process contexts.

The legacy code framing check verifies that deployment-shaped legacy code with
old preflight naming carries explicit fail-closed non-authority framing:

```sh
python3 scripts/check_legacy_code_framing.py
```

The tool manifest framing check verifies that direct `tools/*/Cargo.toml`
package metadata does not present active research, signer-side, verifier, or
Mode B crates as deployment-ready production products. The only production-shaped
tool manifest is the explicitly legacy fail-closed `descender-production-reference`
crate:

```sh
python3 scripts/check_tool_manifest_framing.py
```

The active source framing check verifies the top-of-file docs for high-confusion
active source entrypoints. It allows internal API names such as deployed leaf or
admission verifier where those names are part of the implementation, but the
entrypoint docs must frame active code as Path 2 implementation evidence or
research evidence, not product authority:

```sh
python3 scripts/check_active_source_framing.py
```

The local gate and CI must stay in sync. The parity checker verifies that
`scripts/check_build.sh` and `.github/workflows/ci.yml` both cover the same
required documentation, tracker, wire-format, Lean, Rust, KAT, QROM, and
cryptanalysis support gates. It also compares discovered `python3 scripts/...`
gate commands so a newly added Python gate cannot silently exist only locally or
only in CI. Each named required gate must also have at least one `TRACKER.md`
owner row with an active tracker status, so build checks remain tied to live
consolidated-tracker work rather than becoming a separate task list. The gate
inventory itself must have unique labels, non-empty local patterns, and
non-empty CI patterns. Every named gate must also have a matching visible local
`echo "== ... =="` label and a matching GitHub Actions `name:` label, so local
and CI failures remain identifiable by the same canonical gate names:

```sh
python3 scripts/check_ci_local_gate_parity.py
```

The research automation routing check verifies that the active-control script
inventory in `scripts/research/README.md` exists, that listed active scripts are
exactly the approved set, that unlisted legacy/helper scripts have not exceeded
their baseline cap, and that queue import/dispatch/launch/review/quarantine
paths cite the current authority chain and pivot guard:

```sh
python3 scripts/research/check_research_automation_routing.py
```

They also run the tracker integrity check, which verifies unique tracker IDs,
the exact `TRACKER.md` status-vocabulary table and pivot framing, exact tracker
section layout and row-ID prefixes, allowed statuses, actionable next-action
language for live planning rows, existing path references, valid ADR links,
concrete evidence language for `verified` rows, and that every ADR is referenced
from the consolidated tracker:

```sh
python3 scripts/check_tracker_integrity.py
```

The requirement traceability check verifies that
`spec/blockchain_requirements.md` project framing, requirement classes,
readiness semantics, capability rows, direct-mode adoption gates, Mode B
adoption gates, specification baseline surface rows, direct-mode
implementation/proof surface rows, aggregation/rollup surface rows, Ethereum
adoption-surface rows, security-level adoption-surface rows, hardware/custody
adoption-surface rows, policy feature surface rows, policy requirement rows,
non-claims, and product bar cite or preserve the current chain-native
authorization objective. It also derives every required `SPEC-*`, `DIR-*`,
`AGG-*`, `ADOPT-ETH-*`, `ADOPT-128-*`, `ADOPT-HW-*`, and `POL-*` row from
`TRACKER.md`, checks the readiness vocabulary, checks the exact requirement
class definitions, checks the exact capability matrix, checks the exact
Direct/Mode B adoption-gate contracts, checks the exact specification-baseline
surface contract, checks the exact direct-mode implementation/proof surface
contract, checks the exact compact adoption-surface contracts for
aggregation/rollup, security level, Ethereum, hardware/custody, and policy
feature surfaces, checks the exact policy-layer requirement table and tracker
refs, checks the exact non-claims and product bar, and rejects `ready` claims
unless every cited tracker row is `verified`. The
Ethereum positioning support note must also keep its source-context,
non-claims, and `ADOPT-ETH-*` tracker-consequence list synchronized with the
consolidated tracker. Selected normative required-evidence and promotion-rule
sections in the wire-format, security, lifecycle, and signer specs must also
include explicit tracker ownership, including the signer profile's API, entropy,
leakage, and fault/state adoption surfaces:

```sh
python3 scripts/check_requirement_traceability.py
```

The top-level `README.md` local-check section must mirror the Python process
coverage gates, name the full local build gate, and name both active Rust test
scripts. The full local gate remains:

```sh
./scripts/check_build.sh
```

Active Path 2 Rust tests are split into a default fast gate and an explicit
manual slow suite:

```sh
./scripts/check_active_rust_fast_tests.sh
./scripts/check_active_rust_slow_tests.sh
```

The fast script is wired into `scripts/check_build.sh` and CI, and is owned by
`DIR-TEST-001`. The slow script is not part of the default gate; it names the
long-running deployed-verifier, empirical hiding, roundtrip/red-team, and
recursion verifier-AIR suites so they are not rediscovered ad hoc.
