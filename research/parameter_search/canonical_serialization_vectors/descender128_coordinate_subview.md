# Canonical Serialization Vector: Descender128CoordinateSubviewProfile.serialization

- q: `12289`
- n: `48`
- m0: `80`
- m1: `32`
- eta: `12`
- coordinate count: `80`
- sample bits: `14`
- rounding eta: `0`
- effective eta: `12`
- encoded sample bytes: `140`
- public key bytes: `172`
- ciphertext bytes: `172`
- rounding bound pass: `true`

## Public Key Layout

| field | offset | length | description |
|---|---:|---:|---|
| seed | 0 | 32 | Domain-separated canonical setup seed. |
| packed_public_sample | 32 | 140 | Bit-packed public sample coordinates. |

## Ciphertext Layout

| field | offset | length | description |
|---|---:|---:|---|
| packed_ciphertext_sample | 0 | 140 | Bit-packed ciphertext coordinate vector. |
| ciphertext_overhead | 140 | 32 | Fixed KEM overhead carried outside the coordinate vector. |

## Artifacts

- public key skeleton: `descender128_coordinate_subview.pk.bin`
- ciphertext skeleton: `descender128_coordinate_subview.ct.bin`
