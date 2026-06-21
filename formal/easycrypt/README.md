# EasyCrypt — Security Reduction Layer

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This package contains machine-checked reduction evidence and formal-methods
> context. It is not product authority by itself; current authority starts in
> `../../spec/README.md`, ADRs, and `../../TRACKER.md`.

**Owns:** EUF/SUF-CMA → response-link-coded SIS + QROM hash, concrete advantage
≤ 2⁻¹²⁸ at T=2³². Attaches to `../spec/descender-signature-spec.md` §3–§5.

**Status:** EC-SPIKE returned **GO (qualified)** in `SPIKE_VERDICT.md`; the
reduction spine may use EasyCrypt for probability/game-hop bookkeeping while
Lean retains field, lattice, and concrete-security arithmetic. Subsequent
break-event work is tracked in `break_reduction_status.md` and the current
project tracker.

**First deliverable after the spike:** keep reducing the named break events
against the active spec spine; do not treat this legacy package as product
authority without an explicit current spec or tracker import.

Notes:
- Seam 1 (Lean↔EC spec correspondence) is a named assumption — the EC game must
  cite the exact spec § values it models.
- QROM support in EasyCrypt is research-level; the QROM break terms are the
  hardest part and should be scoped honestly in the spike.
- Precedent: Formosa Crypto's ML-KEM proof is the reference style.
