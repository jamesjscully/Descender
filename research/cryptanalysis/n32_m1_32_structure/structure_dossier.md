# n=32, m1=32 Structural Dossier

This report compares the full regenerated adjoint matrix with the selected
public view prefix.  For reduced-view rows, only the view-prefix dual
relations are directly available from the public sample.

| id | q | m0 | view | k | eff eta | req l1 | full best l1 | full non-guard supp | view best l1 | view supp | view counterexample |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `0000_descender-128_q65537_n32_m080_m132_k24_eta1_bits12_view80` | 65537 | 80 | 80 | 24 | 10 | 3277 | 309656 | 0 | 309656 | 34 | False |
| `0001_descender-128_q65537_n32_m080_m132_k32_eta1_bits12_view80` | 65537 | 80 | 80 | 32 | 10 | 3277 | 300885 | 1 | 309656 | 34 | False |
| `0002_descender-128_q65537_n32_m080_m132_k48_eta1_bits12_view80` | 65537 | 80 | 80 | 48 | 10 | 3277 | 274267 | 1 | 309656 | 34 | False |
| `0003_descender-128_q65537_n32_m080_m132_k64_eta1_bits12_view80` | 65537 | 80 | 80 | 64 | 10 | 3277 | 309656 | 0 | 309656 | 34 | False |

## Notes

- `full best l1` may use hidden tilt coordinates when `view <= m0`.
- `view best l1` is the relevant direct-dual statistic for the public view.
- These are heuristic probes, not lower-bound certificates.
