# Byte-Level Conformance Plan

The proof is only useful for deployment if the implementation matches the
formal byte surface.  This plan makes conformance reviewable.

## Canonical Test Vectors

The current generated bundle is:

```text
research/cryptanalysis/review_readiness_20260519/q12289_conformance_vectors.json
```

It is generated from the Rust verifier with:

```text
cargo run --manifest-path tools/parameter-search/Cargo.toml --release \
  --bin signature_q12289_implicit_verifier -- --emit-conformance-vector
```

The bundle contains:

- public key bytes;
- message bytes;
- signature bytes;
- canonical challenge bytes;
- signed response bytes;
- reconstructed commitment bytes;
- `q12289HashFrame` bytes;
- SHA-256 digest of the hash frame;
- SHA-256-to-challenge output;
- final accept/reject result.

The current pinned Rust fixture records:

```text
message = "descender implicit verifier test"
message bytes = 32
hash frame bytes = 1540
hash frame SHA-256 = 500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c
challenge = 500b7e675f1970875c294ccb0d00eee306
```

## Negative Test Vectors

The generated bundle currently includes reject vectors for:

- challenge high bits set;
- challenge mismatch;
- response mutation causing challenge mismatch;
- response coordinate `225`;
- public-key q-coordinate equal to `12289`;
- wrong signature length.

The remaining reject-vector backlog is:

- wrong public key length;
- malformed response length;
- response coordinate `-225`;
- reconstructed commitment q-coordinate equal to `12289`;
- wrong domain label;
- wrong profile label;
- wrong `q` field;
- wrong `response_bound` field;
- swapped commitment repetition order;
- modified message with unchanged signature;
- modified public key with unchanged signature.

## Harness Requirements

The conformance harness should:

- export vectors from the Rust verifier in JSON and raw hex;
- verify vectors through the Rust verifier;
- verify frame length and SHA digest;
- check every encoded field boundary by offset;
- fail if any label, length prefix, or scalar parameter changes;
- run in CI before any merge touching verifier, profile, or proof code.

## Lean/Rust Boundary

Lean currently proves the production challenge bit-level codec, the frame
length, and defines the frame algebraically as a `ByteString`.  The next
conformance improvement is to compare the generated Rust fixture against the
Lean field order in CI:

```text
domain
profile
q
groups
local_pivot
local_free
repetitions
response_bound
public_key
message
reconstructed_commitments
```

The conformance claim is not complete until every byte in the generated Rust
frame can be explained by this list, the q14 encoder theorem, and the
production challenge codec theorem, with an independent implementation checking
the same vector bundle.
