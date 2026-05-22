# Quotient-Level Link Implementation Handoff

Status: no migration.  Keep the current reviewed split account-link design.

This note is for agents working on the deployable q12289 path.  It summarizes
the closed sidecar research program and points to the promotion gate.  It does
not change the verifier, proof path, byte target, or implementation.

## Decision

Use the current reviewed account-carried bounded-kernel design:

- spend transaction target: `250 B`
- account/state hidden-link object: `68 B`
- combined online plus account accounting: `318 B`
- selected response-link code: `[0, 3]`

Do not replace this with a quotient-level public link, hidden-commitment link,
or reused existing-commitment link unless a later promotion gate supersedes the
current one.

## Reason

Public quotient-level links improve bounded-kernel margins in isolation but
fail hiding when the link rows become public/account-visible conditioning data.
The top exact coordinate candidates fall to `106-108` sampler support-floor
bits under public-link conditioning, below the `128`-bit target.

Hidden-commitment quotient links preserve the sampler surface only by adding
new obligations: commitment hiding, binding/extraction, non-public projection
exclusion, and a hidden linear equation proof.  The sidecar Lean interface
clarifies those obligations but does not remove them.

Reusing the existing account commitments also does not provide the link for
free.  The byte-hash commitments bind full hidden preimages after opening, but
they do not expose a hidden quotient-link commitment or a homomorphic equation
for `link(z) = link(r) + c * link(w)`.  Keeping link values hidden would require
a new zero-knowledge/hash-circuit proof; opening them reintroduces the public
link hiding failure.

## Artifacts

- Source of truth:
  `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_research_program.md`
- Promotion gate:
  `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_promotion_gate.md`
- Reproducible promotion gate JSON:
  `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_promotion_gate.json`
- Promotion gate generator:
  `scripts/research/quotient_level_link_promotion_gate.py`
- Import firebreak audit:
  `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_firebreak_audit.md`
- Import firebreak audit command:
  `python3 scripts/research/quotient_level_link_firebreak_audit.py`
- Closure verification command:
  `python3 scripts/research/verify_quotient_level_link_closure.py`
- Review-gate dry-run integration:
  `python3 scripts/research/review_gate_replay.py --dry-run --skip-timing --json /tmp/quotient_review_gate_dry_run.json --md /tmp/quotient_review_gate_dry_run.md`

## Reopen Criteria

Only reopen this branch with all of the following:

- a concrete commitment/proof system with byte and verifier-cost model;
- sampler replay showing no public-link conditioning below the hiding target;
- an exact bounded-kernel certificate for the selected link;
- a q12289-shaped Lean instantiation of the sidecar theorem shape;
- independent cryptanalytic review of any new commitment or ZK/hash-circuit
  assumption.
