# Agent Handoff

Status: ACTIVE_REPO_ENTRYPOINT

This file is a quick orientation note for coding and research agents. It does
not supersede the spec spine, ADRs, or tracker.

## Start Here

Read these files before making technical or planning claims:

1. `README.md`
2. `spec/README.md`
3. `TRACKER.md`
4. `spec/blockchain_requirements.md`
5. `spec/process.md`

When documents disagree, follow the authority order in `spec/README.md`.
Research, audit, formal, legal, and tool-package notes are support material
unless current specs, ADRs, or `TRACKER.md` import them.

## Current Target

Descender is being pivoted into a chain-native post-quantum authorization stack:

```text
compact PQ spend authorization
+ prevalidated one-use account slots
+ consensus/state binding
+ account policy expressiveness
+ proof aggregation / rollup compatibility
```

The active technical path is Path 2 with Poseidon2 / Plonky3 / KoalaBear account
creation evidence, checked-core spend authorization, consume-once slot state,
and Mode B proof/state aggregation as the scale target.

## Non-Active Paths

Do not revive or present these as current product targets without a new ADR:

- the 260-byte legacy linear-sigma / KAT v0 path;
- the 308/314-byte Fq12 projection-carrier envelope as the final product story;
- spend-time hidden-witness extraction from the old projection-carrier
  transcript;
- native BLS-style signature aggregation;
- a raw "replace ECDSA with PQ CHECKSIG" framing.

Legacy q12289, KAT v0, projection-carrier, and production-reference code may
still appear in gates as conformance, negative-control, archive, or fail-closed
regression evidence. Those gates do not make the old path active again.

## Tracker Discipline

Use `TRACKER.md` as the single live implementation, proof, process, and adoption
tracker. Do not create a parallel top-level TODO or tracker. Subordinate
checklists must say they are subordinate and must be linked from `TRACKER.md`
when the documentation authority gate requires it.

Use these readiness meanings:

- `not_ready`: no deployable evidence yet;
- `partial`: some spec, proof, implementation, or test evidence exists, but not
  adoption evidence;
- `ready`: all cited tracker rows are `verified` for that scope.

Do not call a deployment surface ready because a spec exists.

## Required Gates

Run the full gate before committing cleanup, spec, tracker, CI, or active-path
implementation changes:

```sh
bash scripts/check_build.sh
```

For scoped documentation cleanup, run at least:

```sh
python3 scripts/check_doc_authority.py
python3 scripts/check_tracker_integrity.py
python3 scripts/check_requirement_traceability.py
python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown
```

If a gate fails, update the authority routing or tracker ownership rather than
loosening the check.
