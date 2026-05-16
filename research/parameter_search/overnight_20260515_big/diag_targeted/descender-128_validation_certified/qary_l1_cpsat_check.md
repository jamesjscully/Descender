# Exact Q-ary Centered-L1 Dual Check

This report uses OR-Tools CP-SAT to search for a nonzero q-ary dual
relation with centered `l1` below the amplitude threshold.  `INFEASIBLE`
is the exact certificate; `FEASIBLE`/`OPTIMAL` is a counterexample;
`UNKNOWN` is not a certificate.

| profile | pk | ct | q | n | m | view | req l1 | status | result |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-128 | 172 | 140 | 65537 | 32 | 104 | 72 | 3277 | UNKNOWN | unknown |
