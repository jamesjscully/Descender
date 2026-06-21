# Descender Tool Packages

Status: ACTIVE_TOOLS_ENTRYPOINT

This directory contains active implementation packages, research engines, and
legacy baselines. It is not the product authority. Current protocol authority
starts at `spec/README.md`, append-only decisions live in `decisions/`, and
implementation/proof status lives in `TRACKER.md`.

The active first target is the Path 2 Poseidon2 / Plonky3 / KoalaBear account
creation path plus checked-core spend authorization. Packages listed here are
evidence or implementation surfaces for that target unless the role column says
they are legacy or research-only.

## Tool Packages

| Package | Role |
|---|---|
| `tools/cryptanalysis/` | Active research harness for exporting candidate rows to external cryptanalysis tools; not a security claim by itself. |
| `tools/descender-account-stark/` | Active account-creation STARK relation and proof API using the pinned Plonky3/KoalaBear/Poseidon2 path. |
| `tools/descender-account-verifier/` | Active verifier transport bridge for account-creation evidence and admission leaf binding. |
| `tools/descender-mask-prf/` | Active no-Plonky3 signer-side mask PRF with equivalence tests against the account-creation proof path. |
| `tools/descender-recursion/` | In-progress Mode B recursion and aggregation proof work for account-proof verification. |
| `tools/parameter-search/` | Active research engine and checked-core evidence host; generated rows remain research evidence until imported by spec or tracker rows. |
| `tools/production-reference/` | Legacy fail-closed production-shaped baseline retained for regression and negative-control evidence, not the active product target. |
| `tools/stark-field-bench/` | Active research benchmark for Plonky3 field and Poseidon2/STARK configuration choices. |

## External Tooling

Vendored or setup-managed external tools under `tools/external/` are research
dependencies. They are not part of the core build gate unless a current tracker
row names the exact use.

## Routing Rules

- Add new direct tool crates or direct tool package READMEs to the
  `## Tool Packages` table in this file.
- Keep product claims in `spec/`, decisions in ADRs, and work status in
  `TRACKER.md`.
- Keep direct `Cargo.toml` descriptions aligned with the package role here.
  Active packages should not describe themselves as deployment-ready production
  products.
- Keep legacy package READMEs visibly framed as retained evidence, not current
  authority.
- Treat generated tool outputs as research artifacts until the spec spine or a
  tracker row imports them explicitly.
