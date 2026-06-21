# Descender Cleanup Queue

Status: ACTIVE_SUPPORT

This queue is subordinate to `research/ARCHIVE_INDEX.md`, `spec/process.md`,
and `TRACKER.md`. It is not a second tracker. It records the cleanup disposition
for every non-active archive row so old paths can be removed, retained, or
relocated deliberately.

## Disposition Vocabulary

| Disposition | Meaning |
|---|---|
| `remove_after_import` | delete after live content is imported into current authority or explicit regression evidence |
| `relocate_after_import` | move out of the active tree after live content is imported into current authority or explicit regression evidence |
| `retain_history` | keep as historical decision or design context while active authority still cites the context |
| `retain_legacy_formal_spine` | keep as legacy formal proof/spec context while active formal authority and regression evidence still replace it |
| `retain_legacy_conformance` | keep as a frozen legacy conformance, byte-stability, or fail-closed regression anchor |
| `retain_negative_control` | keep as attack, failed-route, leakage, or structural-regression evidence |

## Label Compatibility

| Label | Allowed dispositions |
|---|---|
| `ARCHIVED_TARGET` | `remove_after_import`, `relocate_after_import`, `retain_history` |
| `LEGACY_FORMAL` | `remove_after_import`, `relocate_after_import`, `retain_legacy_formal_spine`, `retain_legacy_conformance` |
| `MIXED_STATUS` | `remove_after_import`, `relocate_after_import` |
| `NEGATIVE_CONTROL` | `remove_after_import`, `relocate_after_import`, `retain_negative_control` |
| `SUPERSEDED_DECISION` | `remove_after_import`, `relocate_after_import`, `retain_history` |
| `SUPERSEDED_TRACKER` | `remove_after_import`, `relocate_after_import` |

## Exit Condition Rule

Every cleanup queue entry must have a bounded evidence/blocker sentence.
Retained entries must state the condition with `until` or `while`.
Remove/relocate entries must state the condition with `after` or `once`.
`TBD` and `TODO` are not valid evidence/blocker text.

## Owner Rule

`Owner` must include at least one active `TRACKER.md` row. ADR references may
supplement owner context, but they do not replace live tracker ownership.

## Cleanup Queue

| Target | Label | Disposition | Evidence / blocker | Owner |
|---|---|---|---|---|
| `formal/spec/descender-signature-spec.md` | `LEGACY_FORMAL` | `retain_legacy_formal_spine` | Keep while EasyCrypt/Jasmin/red-team context still imports the legacy projection-carrier formal spine and until current formal specs plus regression evidence fully replace it. | `PROC-ARCHIVE-007`, `PROC-FORMAL-001` |
| `formal/kat/README.md` | `LEGACY_FORMAL` | `retain_legacy_conformance` | Keep while KAT v0 remains a byte-conformance regression in the default gate. | `PROC-ARCHIVE-007`, `PROC-LEGACY-GATE-001` |
| `tools/production-reference/README.md` | `LEGACY_FORMAL` | `retain_legacy_conformance` | Keep while `tools/production-reference` remains a fail-closed regression baseline. | `PROC-ARCHIVE-007`, `PROC-LEGACY-GATE-001` |
| `formal/kat/kat_v0.md` | `LEGACY_FORMAL` | `retain_legacy_conformance` | Keep while KAT v0 corpus reproduction is a default legacy conformance gate. | `PROC-ARCHIVE-007`, `PROC-LEGACY-GATE-001` |
| `research/accepting_308b_verifier_blocker.md` | `NEGATIVE_CONTROL` | `retain_negative_control` | Keep until fail-closed projection-carrier behavior is captured by active regression/KAT coverage. | `PROC-ARCHIVE-007`, `PROC-LEGACY-GATE-001` |
| `research/cryptanalysis/high2_zk_inclear_blind_leak_VERDICT.md` | `NEGATIVE_CONTROL` | `retain_negative_control` | Keep until the hiding failure is represented by active leakage/red-team regression evidence. | `PROC-ARCHIVE-007`, `DIR-CT-001` |
| `research/cryptanalysis/public_creation_verifier_build_result.md` | `NEGATIVE_CONTROL` | `retain_negative_control` | Keep until public masked-state verifier failure is captured by active leakage/rejection tests. | `PROC-ARCHIVE-007`, `DIR-REJECT-001`, `DIR-CT-001` |
| `research/cryptanalysis/RED_TEAM_REPORT.md` | `NEGATIVE_CONTROL` | `retain_negative_control` | Keep as adversarial evidence until every still-relevant break maps to active regression or review rows. | `PROC-ARCHIVE-007`, `DIR-REVIEW-001` |
| `research/cryptanalysis/leakage_battery_findings.md` | `NEGATIVE_CONTROL` | `retain_negative_control` | Keep until leakage findings are captured by active signer/prover leakage harnesses. | `PROC-ARCHIVE-007`, `ADOPT-HW-CT-001` |
| `research/cryptanalysis/n32_m1_32_structure/**` | `NEGATIVE_CONTROL` | `retain_negative_control` | Keep while structural attack evidence remains relevant to parameter/family review. | `PROC-ARCHIVE-007`, `DIR-REVIEW-001` |
