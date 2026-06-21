# hax — Rust Reference Bridge Layer

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This package contains hax bridge-spike evidence and implementation-proof
> context. It is not product authority by itself; current authority starts in
> `../../spec/README.md`, ADRs, and `../../TRACKER.md`.

**Owns (interim):** evidence for verifying Rust reference code against the
active spec spine by extracting Rust → F*/Coq and proving correspondence there.
Purpose: close seam 3 properly and **retire the source-inspection "marker
gates"** that gave green-while-broken results.

**Status:** HAX-SPIKE returned **GO (conditional)** in `SPIKE_VERDICT.md`.
Extraction of the self-contained verifier core is tractable, but the bridge
still needs proof-library/toolchain cleanup and real F* lemmas before it can
replace source-inspection gates.

**Next deliverable:** isolate the active Rust verifier core, extract it cleanly,
and prove the correspondence lemmas that matter for the current spec spine.

- *Go:* clean extraction → adopt hax; delete the corresponding marker gate.
- *No-go:* unsupported Rust constructs → fall back to Cryptol+SAW or a Lean
  executable model anchored by `../kat/`.

Notes:
- This is the highest-leverage *tool change* in the stack: it replaces the
  brittlest existing component, not Lean.
- Eventually superseded as the deployable by the Jasmin core; hax verifies the
  prototype during transition.
