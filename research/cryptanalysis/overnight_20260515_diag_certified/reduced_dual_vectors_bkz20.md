# Reduced Dual Vectors In Descender Coordinates

Each row is the first BKZ-reduced vector from the exported q-ary dual lattice.
Coordinates are split as guard coordinates first and tilt coordinates second.

| id | beta | support | guard support | tilt support | l2^2 | linf | mod-q residual ok | cert violation |
|---|---:|---:|---:|---:|---:|---:|---|---|
| `0000_descender-128_q65537_n32_m080_m132_k24_eta1_bits12_view80` | 20 | 106 | 76 | 30 | 7939 | 19 | True | False |
| `0001_descender-128_q65537_n32_m080_m132_k32_eta1_bits12_view80` | 20 | 103 | 74 | 29 | 8217 | 26 | True | False |
| `0002_descender-128_q65537_n32_m080_m132_k48_eta1_bits12_view80` | 20 | 109 | 78 | 31 | 9137 | 30 | True | False |
| `0003_descender-128_q65537_n32_m080_m132_k64_eta1_bits12_view80` | 20 | 111 | 80 | 31 | 10115 | 32 | True | False |
| `0004_descender-192_q131071_n40_m096_m140_k24_eta1_bits15_view96` | 20 | 130 | 91 | 39 | 32897 | 56 | True | False |
| `0005_descender-192_q131071_n40_m096_m140_k32_eta1_bits15_view96` | 20 | 135 | 95 | 40 | 32643 | 41 | True | False |
| `0006_descender-192_q131071_n40_m096_m140_k48_eta1_bits15_view96` | 20 | 130 | 92 | 38 | 35860 | 43 | True | False |
| `0007_descender-192_q131071_n40_m096_m140_k64_eta1_bits15_view96` | 20 | 136 | 96 | 40 | 32961 | 49 | True | False |
| `0008_descender-256_q524309_n48_m0112_m148_k32_eta1_bits16_view112` | 20 | 156 | 110 | 46 | 149447 | 78 | True | False |
| `0009_descender-256_q524309_n48_m0112_m148_k48_eta1_bits16_view112` | 20 | 158 | 110 | 48 | 145963 | 100 | True | False |
| `0010_descender-256_q524309_n48_m0112_m148_k64_eta1_bits16_view112` | 20 | 158 | 110 | 48 | 166847 | 107 | True | False |
| `0011_descender-256_q131071_n56_m0128_m156_k32_eta1_bits15_view128` | 20 | 179 | 126 | 53 | 138583 | 70 | True | False |

Interpretation:

- `mod-q residual ok` confirms the vector is a valid q-ary dual vector for the exported public adjoint matrix.
- `cert violation` only checks the recorded Hamming-weight certificate boundary.
- High-support short Euclidean vectors are cryptanalytic warning signs, but not direct proof-certificate counterexamples.
