# Implementation Hardening Plan

This plan is for a reference implementation whose first priority is security
and reviewability.  Speed optimizations are explicitly out of scope until the
review gate is satisfied.

## Required Components

- Key generation.
- Signing.
- Verification.
- Deterministic test mode.
- Production randomness mode.
- Test-vector exporter.
- Test-vector verifier.

## Constant-Time And Secret Handling

Required checks:

- Secret-key parsing and storage have a single canonical representation.
- Signing avoids secret-dependent branches where practical.
- Signing avoids secret-dependent memory indexing where practical.
- Rejection sampling does not leak reusable information through timing or
  retry counts without a documented mitigation.
- Secret material is zeroized where supported.
- Randomness failures are explicit hard errors.

## Parser Hardening

Verifier must reject:

- noncanonical challenge high bits;
- public-key q-coordinate `>= q`;
- response outside `[-224, 224]`;
- wrong lengths;
- malformed frame/profile/domain variants;
- trailing bytes;
- alternate encodings of the same semantic value.

Parser code should be small, auditable, and covered by negative vectors.

## Fuzzing

Fuzz targets:

- public-key decoder;
- challenge decoder;
- response decoder;
- full verifier;
- hash-frame builder;
- q14 field packer/unpacker;
- signed 9-bit packer/unpacker.

Fuzz properties:

- no panic on arbitrary input;
- accepted inputs round-trip through canonical encoders;
- mutation of any bound byte causes reject unless the mutation is semantically
  irrelevant and documented.

## CI Gates

Before third-party review, CI should run:

```text
lake build Descender
cargo test --manifest-path tools/parameter-search/Cargo.toml --release
cargo fmt --manifest-path tools/parameter-search/Cargo.toml --check
test-vector conformance harness
fuzz smoke targets
git diff --check
```

The current checked subset is smaller.  The gap must be closed before review.
