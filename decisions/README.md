# Decision Records

Status: ACTIVE_NORMATIVE_PROCESS

Decision records are append-only. Do not rewrite old decisions to make history
look cleaner. If a decision changes, add a new ADR and mark the old one
`Superseded` with a pointer to the replacement.

Each ADR must include:

- `Status:` and `Date:` fields;
- `## Context`, `## Decision`, and `## Consequences` sections;
- `## Supersedes`, `## Superseded By`, and
  `## Affected Specs / Tracker Items` sections.

Use `None.` for `Supersedes` or `Superseded By` when there is no prior or later
decision. Do not remove old ADRs when a decision changes; add a new ADR and
update the supersession fields.

Put affected specs, ADRs, and tracker row IDs in backticks. The authority check
resolves those references against the current repo and `TRACKER.md`. If an ADR
names a tracker row, that row must cite the ADR number as reciprocal decision
coverage.

For final ADRs (`Accepted`, `Superseded`, or `Rejected`), the `## Context`,
`## Decision`, and `## Consequences` sections are treated as immutable decision
text. Their hashes live in `ADR_IMMUTABLE_SECTIONS.sha256` and are checked by
`scripts/check_adr_immutable_sections.py`. If that text is wrong or obsolete,
add a new ADR and update lifecycle fields instead of silently rewriting the
old decision.

Accepted decision status values are `Accepted`, `Proposed`, `Rejected`, and
`Superseded`.

## ADR Index

This index is checked by `scripts/check_doc_authority.py`; update it when adding
a new ADR.

| ADR | Status | Date | Title | File |
|---|---|---|---|---|
| `ADR-0001` | `Accepted` | `2026-06-20` | Pivot To Chain-Native PQ Authorization | `ADR-0001-pivot-to-chain-native-pq-authorization.md` |
| `ADR-0002` | `Accepted` | `2026-06-20` | Select Path 2 Prevalidated Slots | `ADR-0002-select-path2-prevalidated-slots.md` |
| `ADR-0003` | `Accepted` | `2026-06-20` | Direct Mode Before Aggregation | `ADR-0003-direct-mode-before-aggregation.md` |
| `ADR-0004` | `Accepted` | `2026-06-20` | Poseidon2 / Plonky3 Is The Active Account-Creation Path | `ADR-0004-poseidon2-plonky3-active-account-creation-path.md` |
| `ADR-0005` | `Accepted` | `2026-06-20` | Archive Legacy Projection-Carrier Product Target | `ADR-0005-archive-legacy-projection-carrier-product-target.md` |
| `ADR-0006` | `Accepted` | `2026-06-20` | Security-Level Position For The Active Path | `ADR-0006-security-level-position.md` |
| `ADR-0007` | `Accepted` | `2026-06-20` | Research Automation Follows Current Authority | `ADR-0007-research-automation-follows-current-authority.md` |
| `ADR-0008` | `Accepted` | `2026-06-27` | Commit To Hard Quantum-128 Security Target | `ADR-0008-commit-hard-quantum-128-target.md` |
| `ADR-0009` | `Accepted` | `2026-06-28` | Native Sovereign Post-Quantum Validity-Rollup Settlement | `ADR-0009-native-sovereign-pq-rollup-settlement.md` |
| `ADR-0010` | `Accepted` | `2026-06-28` | Pad-Seed Registry & Spend-Lifecycle Consensus Wiring + Finality (D2) | `ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md` |
