# Descender Formal Verification Architecture

This directory is the spine of the multi-tool formal-verification effort for the
Descender post-quantum signature. It exists because **no single tool covers all
the proof obligations**, and because the project's worst failure mode to date was
an ad-hoc correspondence layer (Rust source-text "marker gates") that read green
while the tree was broken. This architecture replaces that with explicit,
tool-appropriate layers joined by a single canonical spec and machine-checkable
conformance vectors.

## Principle: the spec is the spine

There is exactly one **canonical specification** (`spec/descender-signature-spec.md`):
keygen, sign, verify, the SIS relation, the Fq12 action, parameters, byte
layouts. Every tool attaches to *that* spec. Tools never attach to each other.
The correspondence between a tool's model and the spec is established either
by proof (where the tool can) or by a shared **KAT** (known-answer test) corpus
(`kat/`) that every executable artifact must reproduce byte-for-byte.

## Layers and tool assignment

| Layer | Obligation | Tool | Location |
|---|---|---|---|
| **Math** | lattice/SIS/field facts, concrete-security arithmetic, parameter justification, the union-bound + closure skeleton | **Lean 4 + Mathlib** | `../Descender/` (`lean/` brief) |
| **Security** | EUF-CMA / SUF-CMA → SIS + QROM hash assumptions, concrete advantage ε ≤ 2⁻¹²⁸ | **EasyCrypt** (Formosa) | `easycrypt/` |
| **Implementation** | deployable verifier/signer: functionally correct vs spec + constant-time, compiled to verified asm | **Jasmin** (Formosa) | `jasmin/` |
| **Bridge (interim)** | the *existing Rust reference* matches the spec; retire the marker-gate hack | **hax** (Rust → F*/Coq) | `hax/` |
| **Conformance** | byte-for-byte agreement across every implementation | **KAT vectors** | `kat/` |

Why this split (the short version): Mathlib is too valuable to give up for the
lattice/field/probability math, so Lean keeps the math + the reduction skeleton.
EasyCrypt is purpose-built for game-based reductions (pRHL, game hops) — the part
hardest to express in Lean. Jasmin+EasyCrypt is the battle-tested *deployable*
PQ stack (Formosa Crypto / ML-KEM). hax extracts the current Rust reference so
the prototype is verified during transition and the brittle gates die.

## The seams (explicit trust boundaries — read this)

Cross-tool correspondence is **not** machine-checked end-to-end. The residual
trust seams, and how each is managed:

1. **Lean spec ≟ EasyCrypt spec.** Both model the same Σ-protocol/SIS relation.
   Managed by: the canonical spec doc is normative; both formalizations cite the
   exact spec section they implement; a reviewer checklist cross-reads them. (No
   shared kernel — this is a genuine, named assumption, as it is in Formosa.)
2. **EasyCrypt spec ≟ Jasmin code.** Managed by **proof**: EasyCrypt proves the
   Jasmin implementation matches the EasyCrypt functional spec (this is what the
   Jasmin/EasyCrypt pipeline does natively). This seam is closed, not assumed.
3. **Spec ≟ Rust reference (interim).** Managed by **hax extraction + KAT**: hax
   extracts Rust to F*; correctness proven there; and every implementation
   (Rust, Jasmin, any Lean executable model) must reproduce `kat/` byte-for-byte.
4. **Assumptions ≟ reality.** SIS/QROM hardness at the deployed parameters is
   **external** — no tool closes it. Owned by `research/cryptanalysis/` +
   third-party review. Tool choice never changes this.

A claim is only as strong as its weakest seam. Seam 1 and seam 4 are the
standing assumptions; seams 2 and 3 are closed by proof/conformance. This is
stated up front so no layer's "verified" is mistaken for end-to-end security.

## Status

Bootstrapping (2026-06-09). Lean math + reduction skeleton is live
(`Descender/Path2AcceptedSpendClosure.lean`, build + axiom audit green). The
EasyCrypt/Jasmin/hax layers are scaffolded with concrete first tasks; none is
started. See `VERIFICATION_PLAN.md` for the phased roadmap and decision gates,
including the EasyCrypt spike that must precede any EasyCrypt commitment.
