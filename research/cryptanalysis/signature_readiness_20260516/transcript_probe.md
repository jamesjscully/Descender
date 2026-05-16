# Signature Transcript Leakage Probe

- model: `scalar Fiat-Shamir leakage probe: z = y + c*s mod q, with bounded y and small centered s`
- q: `12289`
- n: `48`
- secret eta: `12`
- trials per row: `64`

| mask bound | transcripts | full recovery | coordinate recovery | avg score gap | verdict |
|---:|---:|---:|---:|---:|---|
| 12 | 1 | 0.875 | 0.958 | 0.93 | break |
| 12 | 2 | 1.000 | 1.000 | 1.98 | break |
| 12 | 4 | 1.000 | 1.000 | 3.82 | break |
| 12 | 8 | 1.000 | 1.000 | 7.82 | break |
| 12 | 16 | 1.000 | 1.000 | 15.68 | break |
| 12 | 32 | 1.000 | 1.000 | 31.39 | break |
| 32 | 1 | 0.906 | 0.979 | 0.95 | break |
| 32 | 2 | 1.000 | 1.000 | 1.88 | break |
| 32 | 4 | 1.000 | 1.000 | 3.79 | break |
| 32 | 8 | 1.000 | 1.000 | 7.52 | break |
| 32 | 16 | 1.000 | 1.000 | 15.19 | break |
| 32 | 32 | 1.000 | 1.000 | 30.77 | break |
| 64 | 1 | 0.875 | 0.955 | 0.92 | break |
| 64 | 2 | 1.000 | 1.000 | 1.60 | break |
| 64 | 4 | 1.000 | 1.000 | 3.61 | break |
| 64 | 8 | 1.000 | 1.000 | 7.26 | break |
| 64 | 16 | 1.000 | 1.000 | 14.92 | break |
| 64 | 32 | 1.000 | 1.000 | 30.26 | break |
| 128 | 1 | 0.406 | 0.801 | 0.67 | partial-leak |
| 128 | 2 | 0.969 | 0.996 | 1.55 | break |
| 128 | 4 | 1.000 | 1.000 | 3.17 | break |
| 128 | 8 | 1.000 | 1.000 | 6.97 | break |
| 128 | 16 | 1.000 | 1.000 | 14.47 | break |
| 128 | 32 | 1.000 | 1.000 | 29.41 | break |
| 256 | 1 | 0.000 | 0.672 | 0.43 | partial-leak |
| 256 | 2 | 0.969 | 0.999 | 1.25 | break |
| 256 | 4 | 1.000 | 1.000 | 2.83 | break |
| 256 | 8 | 1.000 | 1.000 | 6.47 | break |
| 256 | 16 | 1.000 | 1.000 | 13.64 | break |
| 256 | 32 | 1.000 | 1.000 | 28.13 | break |
| 512 | 1 | 0.000 | 0.449 | 0.06 | inconclusive |
| 512 | 2 | 0.594 | 0.926 | 0.86 | break |
| 512 | 4 | 1.000 | 1.000 | 2.28 | break |
| 512 | 8 | 1.000 | 1.000 | 5.70 | break |
| 512 | 16 | 1.000 | 1.000 | 12.45 | break |
| 512 | 32 | 1.000 | 1.000 | 26.13 | break |
| 1024 | 1 | 0.000 | 0.230 | 0.00 | inconclusive |
| 1024 | 2 | 0.062 | 0.771 | 0.58 | partial-leak |
| 1024 | 4 | 0.781 | 0.987 | 1.65 | break |
| 1024 | 8 | 1.000 | 1.000 | 4.39 | break |
| 1024 | 16 | 1.000 | 1.000 | 10.47 | break |
| 1024 | 32 | 1.000 | 1.000 | 22.42 | break |
| 2048 | 1 | 0.000 | 0.131 | 0.00 | inconclusive |
| 2048 | 2 | 0.000 | 0.350 | 0.03 | inconclusive |
| 2048 | 4 | 0.031 | 0.819 | 0.72 | partial-leak |
| 2048 | 8 | 1.000 | 1.000 | 2.81 | break |
| 2048 | 16 | 1.000 | 1.000 | 6.93 | break |
| 2048 | 32 | 1.000 | 1.000 | 16.06 | break |

## Conclusions

- Direct linear responses with bounded masks are unsafe when the secret has small coordinate support; each coordinate can be attacked independently by maximum-likelihood enumeration.
- This probe is not an attack on the KEM/public-view candidate; it is a warning against naive Schnorr/Lyubashevsky-style signing without a masking and rejection proof sized for transcript leakage.
- Signature work should start from a transcript relation whose responses are statistically independent of the secret, or from a proof system that does not expose linear masked secrets.
