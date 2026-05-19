# Frozen Target Specification

This is the review target for the q12289 implicit-commitment signature
candidate.  It is intentionally narrow: reviewers should not need to infer
parameters from historical search logs.

## Parameters

| item | value |
|---|---:|
| modulus `q` | `12289` |
| groups | `16` |
| rows per group | `3` |
| local free coordinates | `2` |
| pivot coordinates | `48` |
| free coordinates | `32` |
| total view coordinates | `80` |
| repetitions | `7` |
| response bound | `224` |
| target Lee lower bound | `512` |

## Byte Sizes

| object | bytes |
|---|---:|
| public key seed | `32` |
| public key q-coordinate payload | `140` |
| public key | `172` |
| challenge | `17` |
| response per repetition | `36` |
| signature | `269` |
| reconstructed commitment per repetition | `140` |
| explicit commitments, if serialized | `980` |

The implicit signature is:

```text
signature = challenge[17] || response[7][36]
```

The verifier reconstructs the omitted commitment bytes and hashes those bytes
instead of receiving explicit commitments in the signature.

## Encodings

Challenge:

- `7` repetitions.
- Each repetition uses `19` bits.
- First `4` bits encode the omitted block.
- The remaining `15` bits encode signs for all non-omitted blocks.
- Little-endian bit order.
- Top `3` bits of the final challenge byte are zero.

Public key:

- First `32` bytes are seed/domain material for the current profile.
- Remaining `140` bytes encode `80` q-coordinates.
- Coordinates are packed little-endian, `14` bits each.
- Order is `48` pivot coordinates followed by `32` free coordinates.
- Any decoded q-coordinate `>= 12289` is noncanonical and rejected.

Response:

- Each repetition has one `36 B` response chunk.
- Each chunk packs `32` signed 9-bit two's-complement free coordinates.
- A decoded coordinate is accepted only if `|z| <= 224`.

Reconstructed commitment:

- For each repetition, the verifier computes the formal q12289 view
  `t' = A*z - chi*u`.
- The view is encoded by
  `N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder`.
- Lean proves decoding that q14 byte representation recovers the requested
  `48` pivot and `32` free coordinates.

Hash frame:

- Lean endpoint: `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`.
- The frame is a sequence of length-prefixed fields.
- Field header format is `u16_le(label_len) || label || u64_le(data_len)`.
- Fields are: `domain`, `profile`, `q`, `groups`, `local_pivot`,
  `local_free`, `repetitions`, `response_bound`, `public_key`, `message`,
  `reconstructed_commitments`.
- Lean proves frame length is `1508 + messageLen`.
- The pinned fixture message has length `32`, so the pinned frame has length
  `1540`.

## Verification Algorithm

Given `(public_key, message, signature)`:

1. Reject if public key or signature length is not canonical.
2. Decode the challenge; reject noncanonical high bits.
3. Decode the public-key q-coordinate payload; reject noncanonical q values.
4. Decode every response chunk; reject any response outside the signed bound.
5. Reconstruct each commitment as `A*z - chi*u`.
6. Encode reconstructed commitments with `commitmentViewEncoder`.
7. Build `q12289HashFrame`.
8. Compute SHA-256-to-challenge using the production challenge map.
9. Accept only if the recomputed challenge equals the signed challenge.
