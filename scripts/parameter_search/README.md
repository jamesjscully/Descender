# Descender Parameter Search

Status: ACTIVE_RESEARCH

This package is a readable reference harness for parameter exploration, not a
product parameter decision. Current parameter and security authority starts in
`../../spec/README.md`, ADRs, and `../../TRACKER.md`.

This directory is the first concrete-parameter research harness for Descender.
It is intentionally separate from the Lean proof files and from the agent
workflow scripts.

The Rust engine in `tools/parameter-search` is now the preferred path for larger
grids.  This Python harness remains useful as a readable reference
implementation and lightweight report generator.

The search is constraint-driven:

1. Encode hard proof-side requirements from the current Lean/docs layer.
2. Enumerate only small, explicit grids of concrete candidates.
3. Reject candidates that violate hard constraints.
4. Rank the survivors by screening proxies for security margin, compact public
   size, ciphertext size, and correctness slack.
5. Send the Pareto frontier to heavier exact finite-field, projection, dual
   witness, and lattice/coding-estimator checks.

## Current Hard Filters

- `q > n + m0 + m1`: enough distinct field elements for the Cauchy block.
- `m0 >= n`: guard block has the dimension needed for the proven primal-distance
  theorem.
- `eta < q/2`: centered noise representatives have headroom.
- guard distance `m0 - n + 1` exceeds the centered-noise correctness radius.
- profile-specific public-key, ciphertext, secret-key, and reduction-slack
  budgets.

## Screening Proxies

The script reports `primal_proxy_bits`, `dual_proxy_bits`,
`self_randomization_proxy_bits`, and `security_proxy_bits`.

These are not security claims. They are coarse filters for prioritizing which
parameter rows deserve real cryptanalysis and exact theorem-instantiation work.
They should be replaced or supplemented by better estimators as soon as the
concrete attack model stabilizes.

## Run

Preferred Rust path:

```bash
cargo build --release --manifest-path tools/parameter-search/Cargo.toml
tools/parameter-search/target/release/descender-parameter-search --profile descender-128
tools/parameter-search/target/release/descender-parameter-search --profile descender-192
tools/parameter-search/target/release/descender-parameter-search --profile descender-256
```

Reference Python path:

```bash
python3 scripts/parameter_search/search.py --profile descender-128
python3 scripts/parameter_search/search.py --profile descender-192
python3 scripts/parameter_search/search.py --profile descender-256
```

Focused committed-sigma/account-carried cost model:

```bash
python3 scripts/parameter_search/committed_sigma_cost_model.py
```

This model is deliberately narrower than the KEM/view search.  It applies the
current committed-sigma structural gates and keeps spend-transaction bytes
separate from account/state bytes.  The default output is:

```text
research/parameter_search/committed_sigma_account_carried_20260521/
```

Outputs are written under:

```text
research/parameter_search/
```

Each profile gets a JSON artifact and a short Markdown table. The JSON output is
the machine-readable source of truth; the Markdown file is for review.

## Next Harness Extensions

- Exact finite-field rank checks for `A = Pi D`.
- Exact projected-distance computation for toy/scaled instances.
- Dual-witness enumeration for small envelopes.
- Integration with an external lattice/coding estimator.
- Serialization estimates for an actual transparent setup format.
