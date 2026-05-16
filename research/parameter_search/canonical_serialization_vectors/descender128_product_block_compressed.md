# Canonical Serialization Vector: Descender128ProductBlockCompressedProfile.serialization

- q: `32771`
- n: `64`
- m0: `104`
- m1: `32`
- eta: `8`
- coordinate count: `136`
- sample bits: `12`
- rounding eta: `5`
- effective eta: `13`
- encoded sample bytes: `204`
- public key bytes: `268`
- ciphertext bytes: `236`
- rounding bound pass: `true`

## Public Key Layout

| field | offset | length | description |
|---|---:|---:|---|
| seed | 0 | 64 | Domain-separated canonical setup seed. |
| packed_public_sample | 64 | 204 | Bit-packed public sample coordinates. |

## Ciphertext Layout

| field | offset | length | description |
|---|---:|---:|---|
| packed_ciphertext_sample | 0 | 204 | Bit-packed ciphertext coordinate vector. |
| ciphertext_overhead | 204 | 32 | Fixed KEM overhead carried outside the coordinate vector. |

## Artifacts

- public key skeleton: `descender128_product_block_compressed.pk.bin`
- ciphertext skeleton: `descender128_product_block_compressed.ct.bin`
