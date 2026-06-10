# hax — Rust Reference Bridge Layer

**Owns (interim):** verifying that the *existing* Rust reference
(`tools/parameter-search/src/production_reference.rs`) matches the spec, by
extracting Rust → F*/Coq and proving correspondence there. Purpose: close seam 3
properly and **retire the source-inspection "marker gates"** that gave
green-while-broken results.

**Status:** not started — gated on the **HAX-SPIKE** (`../VERIFICATION_PLAN.md`).

**First deliverable (HAX-SPIKE):** run `hax` on one self-contained verifier
function (e.g. the Fq12 sigma-equation check), extract to F*, and judge whether
the obligation is tractable.

- *Go:* clean extraction → adopt hax; delete the corresponding marker gate.
- *No-go:* unsupported Rust constructs → fall back to Cryptol+SAW or a Lean
  executable model anchored by `../kat/`.

Notes:
- This is the highest-leverage *tool change* in the stack: it replaces the
  brittlest existing component, not Lean.
- Eventually superseded as the deployable by the Jasmin core; hax verifies the
  prototype during transition.
