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

The checked-in bundle can be regenerated in place with:

```text
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin signature_q12289_implicit_verifier -- \
  --write-conformance-vector \
  research/cryptanalysis/review_readiness_20260519/q12289_conformance_vectors.json
```

The conformance harness that CI should run is:

```text
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin signature_q12289_implicit_verifier -- \
  --verify-conformance-vector \
  research/cryptanalysis/review_readiness_20260519/q12289_conformance_vectors.json
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
- response coordinate `-225`;
- public-key q-coordinate equal to `12289`;
- wrong public key length;
- wrong signature length;
- malformed response length;
- modified message with unchanged signature;
- modified public key with unchanged signature;
- reconstructed commitment length short;
- reconstructed commitment q-coordinate equal to `12289`.

The remaining reject-vector backlog is:

- wrong domain label;
- wrong profile label;
- wrong `q` field;
- wrong `response_bound` field;
- swapped commitment repetition order;

## Harness Requirements

The conformance harness now:

- regenerates the canonical JSON bundle from the verifier and compares it to the
  checked-in file;
- replays verifier-negative vectors through `verify_signature`;
- verifies frame length, SHA-256 digest, and derived challenge;
- independently parses every length-prefixed hash-frame field and checks the
  field order, labels, scalar values, and byte boundaries;
- independently decodes public-key, response, and reconstructed-commitment
  payloads with a separate bitstream parser;
- fails if any label, length prefix, scalar parameter, byte frame, or challenge
  output changes.

## Lean/Rust Boundary

Lean currently proves the production challenge bit-level codec, the frame
length, and defines the frame algebraically as a `ByteString`.  The current
Rust harness now makes the field order explicit in CI:

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

The conformance claim still depends on reviewers checking that this field order,
the q14 encoder theorem, and the production challenge codec theorem match the
Lean endpoints named in the vector bundle.  The current harness closes the Rust
side by making the byte layout and challenge derivation fail closed.
