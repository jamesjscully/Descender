# Descender External Cryptanalysis Export

This report packages validator-passing Descender rows for external tools.
It is not a cryptanalytic security estimate by itself.

External tool runs are recorded in `external_tool_results.md`.  BKZ-reduced
dual vectors converted back into Descender coordinates are recorded in
`reduced_dual_vectors_bkz20.md` and `reduced_dual_vectors_bkz20.json`.
The direct dual distinguisher calculation is recorded in
`direct_dual_distinguisher.md` and `direct_dual_distinguisher.json`.

| id | profile | q | n | m0 | m1 | view | eta | pk | ct | route | margins | flags |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|
| `0000_descender-128_q65537_n32_m080_m132_k24_eta1_bits12_view80` | descender-128 | 65537 | 32 | 80 | 32 | 80 | 1 | 184 | 152 | guard-block-lossy-diagonal-certified | corr 28; diag 9.5; avg2wc 156.6 | small_n_extrapolation, reduced_view, quantized_public_sample, thin_diagonal_slack, bounded_dual_search_only |
| `0001_descender-128_q65537_n32_m080_m132_k32_eta1_bits12_view80` | descender-128 | 65537 | 32 | 80 | 32 | 80 | 1 | 184 | 152 | guard-block-lossy-diagonal-certified | corr 28; diag 9.5; avg2wc 156.6 | small_n_extrapolation, reduced_view, quantized_public_sample, thin_diagonal_slack, bounded_dual_search_only |
| `0002_descender-128_q65537_n32_m080_m132_k48_eta1_bits12_view80` | descender-128 | 65537 | 32 | 80 | 32 | 80 | 1 | 184 | 152 | guard-block-lossy-diagonal-certified | corr 28; diag 9.5; avg2wc 156.6 | small_n_extrapolation, reduced_view, quantized_public_sample, thin_diagonal_slack, bounded_dual_search_only |
| `0003_descender-128_q65537_n32_m080_m132_k64_eta1_bits12_view80` | descender-128 | 65537 | 32 | 80 | 32 | 80 | 1 | 184 | 152 | guard-block-lossy-diagonal-certified | corr 28; diag 9.5; avg2wc 156.6 | small_n_extrapolation, reduced_view, quantized_public_sample, thin_diagonal_slack, bounded_dual_search_only |
| `0004_descender-192_q131071_n40_m096_m140_k24_eta1_bits15_view96` | descender-192 | 131071 | 40 | 96 | 40 | 96 | 1 | 244 | 212 | guard-block-lossy-diagonal-certified | corr 50; diag 78.1; avg2wc 405.5 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0005_descender-192_q131071_n40_m096_m140_k32_eta1_bits15_view96` | descender-192 | 131071 | 40 | 96 | 40 | 96 | 1 | 244 | 212 | guard-block-lossy-diagonal-certified | corr 50; diag 78.1; avg2wc 405.5 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0006_descender-192_q131071_n40_m096_m140_k48_eta1_bits15_view96` | descender-192 | 131071 | 40 | 96 | 40 | 96 | 1 | 244 | 212 | guard-block-lossy-diagonal-certified | corr 50; diag 78.1; avg2wc 405.5 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0007_descender-192_q131071_n40_m096_m140_k64_eta1_bits15_view96` | descender-192 | 131071 | 40 | 96 | 40 | 96 | 1 | 244 | 212 | guard-block-lossy-diagonal-certified | corr 50; diag 78.1; avg2wc 405.5 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0008_descender-256_q524309_n48_m0112_m148_k32_eta1_bits16_view112` | descender-256 | 524309 | 48 | 112 | 48 | 112 | 1 | 288 | 256 | guard-block-lossy-diagonal-certified | corr 52; diag 21.2; avg2wc 491.6 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0009_descender-256_q524309_n48_m0112_m148_k48_eta1_bits16_view112` | descender-256 | 524309 | 48 | 112 | 48 | 112 | 1 | 288 | 256 | guard-block-lossy-diagonal-certified | corr 52; diag 21.2; avg2wc 491.6 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0010_descender-256_q524309_n48_m0112_m148_k64_eta1_bits16_view112` | descender-256 | 524309 | 48 | 112 | 48 | 112 | 1 | 288 | 256 | guard-block-lossy-diagonal-certified | corr 52; diag 21.2; avg2wc 491.6 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |
| `0011_descender-256_q131071_n56_m0128_m156_k32_eta1_bits15_view128` | descender-256 | 131071 | 56 | 128 | 56 | 128 | 1 | 304 | 272 | guard-block-lossy-diagonal-certified | corr 66; diag 18.9; avg2wc 585.7 | small_n_extrapolation, reduced_view, quantized_public_sample, bounded_dual_search_only |

## External Tool Plan

1. Run `run_lattice_estimator_jobs.py` only after reviewing the LWE-like mapping caveats.
2. Run `run_fpylll_jobs.py` or the `fplll` CLI on `fplll/*.dual_qary_basis.txt` for concrete reduction sanity checks.
3. Use `estimator_jobs.json` as the handoff point for code/syndrome-decoding estimator mappings.
4. Treat any unexpectedly low cost or short vector as a blocker for the candidate row.
