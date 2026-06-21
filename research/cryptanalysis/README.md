# Descender Cryptanalysis Notes

Status: ACTIVE_RESEARCH_ENTRYPOINT

This directory contains cryptanalysis evidence, red-team reports, reduction
notes, assumption inventories, negative controls, and review packages. It is not
the current product security specification.

Current security authority starts in:

```text
../../spec/README.md
../../spec/security_model.md
../../spec/blockchain_requirements.md
../../decisions/
../../TRACKER.md
../ARCHIVE_INDEX.md
```

Treat root-level files in this directory as evidence unless `spec/` or an ADR
explicitly imports the claim. New root-level Markdown notes must be classified in
`../ARCHIVE_INDEX.md`; the local and CI documentation inventory gates this so a
new security dossier, assumption ledger, red-team report, or reduction note
cannot become accidental authority.

Historical files with migration banners are retained for auditability and
negative-control value. Active research files can support the current Path 2 /
Poseidon2 / Plonky3 direction, but they still remain subordinate to the spec
spine and consolidated tracker.
