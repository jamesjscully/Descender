# KAT — Known-Answer Test Corpus (cross-tool conformance anchor)

**Owns:** byte-for-byte agreement across every implementation (Rust reference,
Jasmin core, any Lean/hax executable model). This is the *one* machine-checkable
artifact that ties the tools together where proofs cannot span them (seams 1/3).

**Status:** KAT v0 delivered — see [`kat_v0.json`](kat_v0.json) (frozen corpus)
and [`kat_v0.md`](kat_v0.md) (human-readable index). 3 positive + 8 negative
vectors (7 distinct reject reasons), generated from the real Rust reference
pipeline (`production_keygen`/`production_sign`/`production_verify` in
`tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`).

Regenerate / check:

```
cd tools/parameter-search
cargo run --bin signature_q12289_local_delta_stateful -- --kat-generate   # freeze
cargo run --bin signature_q12289_local_delta_stateful -- --kat-check      # CI gate
```

CI runs `--kat-check` (and the `kat_v0_corpus_reproduces_frozen_json` test) so
any byte-level drift fails the build.

**First deliverable (KAT v0):** freeze a reference corpus generated from the Rust
reference:
- positive: keygen → sign → verify round-trips for fixed seeds/messages/slots;
- negative: non-canonical encodings, mutated challenge/response/carrier, replayed
  slot, stale account root, public-representative forgery probe;
- each entry: inputs (hex) + expected output (accept/reject + reason + byte
  outputs), with the producing-version digest pinned.

Every implementation must reproduce KAT v0 exactly; CI runs it. When a parameter
changes (cf. the AP2-004 widening that broke the build), the KAT regen is the
forcing function that catches drift across *all* implementations at once.
