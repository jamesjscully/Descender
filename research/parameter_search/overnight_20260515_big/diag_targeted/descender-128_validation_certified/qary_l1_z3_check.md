# Exact Q-ary Centered-L1 Dual Check

This report asks Z3 whether there is a nonzero q-ary dual relation
`z` with centered `l1` below the amplitude threshold required by the
deterministic proof boundary.  `unsat` is a proof-side certificate for
the checked row; `sat` is a real counterexample; `unknown` is not a
certificate.

| profile | pk | ct | q | n | m | view | req l1 | Z3 | result |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-128 | 172 | 140 | 65537 | 32 | 104 | 72 | 3277 | unknown | timeout |
