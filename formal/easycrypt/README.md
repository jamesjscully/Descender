# EasyCrypt — Security Reduction Layer

**Owns:** EUF/SUF-CMA → response-link-coded SIS + QROM hash, concrete advantage
≤ 2⁻¹²⁸ at T=2³². Attaches to `../spec/descender-signature-spec.md` §3–§5.

**Status:** not started — gated on the **EC-SPIKE** (`../VERIFICATION_PLAN.md`).

**First deliverable (EC-SPIKE):** formalize one game hop — the union-bound step
plus a single break-event reduction — and write its success criterion *before*
starting, then compare effort/clarity to `../../Descender/Path2AcceptedSpendClosure.lean`.

Notes:
- Seam 1 (Lean↔EC spec correspondence) is a named assumption — the EC game must
  cite the exact spec § values it models.
- QROM support in EasyCrypt is research-level; the QROM break terms are the
  hardest part and should be scoped honestly in the spike.
- Precedent: Formosa Crypto's ML-KEM proof is the reference style.
