# Direct Dual Distinguisher Analysis

For any integer vector `z` with `A^T z = 0 mod q`, real samples
`t = A s + e` satisfy `<z,t> = <z,e> mod q`.  Uniform samples make
`<z,t>` uniform modulo `q`.  The table uses the deterministic test
`centeredAbs(<z,t>) <= eta * ||z||_1`, which accepts real samples with
probability 1 under the centered coordinate noise envelope.

| id | q | eta | support | l1 | radius | uniform hit | gap lower bound | interpretation |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `0000_descender-128_q65537_n32_m080_m132_k24_eta1_bits12_view80` | 65537 | 1 | 106 | 775 | 775 | 0.023666 | 0.976334 | direct_break_under_centered_independent_noise |
| `0001_descender-128_q65537_n32_m080_m132_k32_eta1_bits12_view80` | 65537 | 1 | 103 | 735 | 735 | 0.022445 | 0.977555 | direct_break_under_centered_independent_noise |
| `0002_descender-128_q65537_n32_m080_m132_k48_eta1_bits12_view80` | 65537 | 1 | 109 | 805 | 805 | 0.024582 | 0.975418 | direct_break_under_centered_independent_noise |
| `0003_descender-128_q65537_n32_m080_m132_k64_eta1_bits12_view80` | 65537 | 1 | 111 | 823 | 823 | 0.025131 | 0.974869 | direct_break_under_centered_independent_noise |
| `0004_descender-192_q131071_n40_m096_m140_k24_eta1_bits15_view96` | 131071 | 1 | 130 | 1643 | 1643 | 0.025078 | 0.974922 | direct_break_under_centered_independent_noise |
| `0005_descender-192_q131071_n40_m096_m140_k32_eta1_bits15_view96` | 131071 | 1 | 135 | 1667 | 1667 | 0.025444 | 0.974556 | direct_break_under_centered_independent_noise |
| `0006_descender-192_q131071_n40_m096_m140_k48_eta1_bits15_view96` | 131071 | 1 | 130 | 1734 | 1734 | 0.026467 | 0.973533 | direct_break_under_centered_independent_noise |
| `0007_descender-192_q131071_n40_m096_m140_k64_eta1_bits15_view96` | 131071 | 1 | 136 | 1757 | 1757 | 0.026818 | 0.973182 | direct_break_under_centered_independent_noise |
| `0008_descender-256_q524309_n48_m0112_m148_k32_eta1_bits16_view112` | 524309 | 1 | 156 | 3825 | 3825 | 0.014593 | 0.985407 | direct_break_under_centered_independent_noise |
| `0009_descender-256_q524309_n48_m0112_m148_k48_eta1_bits16_view112` | 524309 | 1 | 158 | 3821 | 3821 | 0.014577 | 0.985423 | direct_break_under_centered_independent_noise |
| `0010_descender-256_q524309_n48_m0112_m148_k64_eta1_bits16_view112` | 524309 | 1 | 158 | 4233 | 4233 | 0.016149 | 0.983851 | direct_break_under_centered_independent_noise |
| `0011_descender-256_q131071_n56_m0128_m156_k32_eta1_bits15_view128` | 131071 | 1 | 179 | 4167 | 4167 | 0.063591 | 0.936409 | direct_break_under_centered_independent_noise |

Conclusion:

If the KEM public sample/null game is literally `A s + e` versus
uniform over the same coordinate view, then these q-ary dual vectors
give a direct distinguisher.  This is not a proxy-estimator result.
The scheme must either remove these dual vectors from the public view,
make the noise large enough to wrap modulo `q`, or change the security
game/reduction so this statistic is not available.
