# Direct Dual Distinguisher Analysis

For any integer vector `z` with `A^T z = 0 mod q`, real samples
`t = A s + e` satisfy `<z,t> = <z,e> mod q`.  Uniform samples make
`<z,t>` uniform modulo `q`.  The table uses the deterministic test
`centeredAbs(<z,t>) <= eta * ||z||_1`, which accepts real samples with
probability 1 under the centered coordinate noise envelope.

| id | q | eta | support | l1 | radius | uniform hit | gap lower bound | interpretation |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `0000_descender-128_q65537_n32_m080_m132_k24_eta1_bits12_view80` | 65537 | 10 | 78 | 1464 | 14640 | 0.446786 | 0.553214 | direct_break_under_centered_independent_noise |
| `0001_descender-128_q65537_n32_m080_m132_k32_eta1_bits12_view80` | 65537 | 10 | 78 | 1464 | 14640 | 0.446786 | 0.553214 | direct_break_under_centered_independent_noise |
| `0002_descender-128_q65537_n32_m080_m132_k48_eta1_bits12_view80` | 65537 | 10 | 78 | 1464 | 14640 | 0.446786 | 0.553214 | direct_break_under_centered_independent_noise |
| `0003_descender-128_q65537_n32_m080_m132_k64_eta1_bits12_view80` | 65537 | 10 | 78 | 1464 | 14640 | 0.446786 | 0.553214 | direct_break_under_centered_independent_noise |

Conclusion:

If the KEM public sample/null game is literally `A s + e` versus
uniform over the same coordinate view, then these q-ary dual vectors
give a direct distinguisher.  This is not a proxy-estimator result.
The scheme must either remove these dual vectors from the public view,
make the noise large enough to wrap modulo `q`, or change the security
game/reduction so this statistic is not available.
