# ADR-0005: Archive Legacy Projection-Carrier Product Target

Status: Accepted
Date: 2026-06-20

## Context

The repository contains a large body of work around 308/314-byte projection-carrier
spend envelopes and related formal/KAT artifacts. Those artifacts record useful
cryptanalysis and negative controls, but they no longer describe the selected
product architecture.

## Decision

The legacy projection-carrier spend envelope is archived as a product target. It
is retained for:

- negative controls;
- regression tests;
- formal-history references;
- cryptanalysis evidence;
- compatibility investigation.

It is not the current deployable product story.

## Consequences

- Old docs must stop calling this path the current normative target.
- New specs should describe Path 2 prevalidated slots instead.
- Any revival of the projection-carrier target requires a new ADR.

## Supersedes

This ADR supersedes old projection-carrier product-target notes and package
claims as current product authority.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/README.md`
- `spec/wire_formats.md`
- `research/ARCHIVE_INDEX.md`
- `TRACKER.md`: `PROC-STALE-001`, `PROC-STALE-002`
