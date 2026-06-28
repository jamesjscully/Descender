# ADR-0010: Pad-Seed Registry & Spend-Lifecycle Consensus Wiring + Finality (D2)

Status: Accepted
Date: 2026-06-28

## Context

`ADR-0009` makes Descender a sovereign post-quantum validity rollup that owns its
consensus, data availability, and finality. Two consensus-state mechanisms then need
a defined home and a finality profile:

- **F2 single-use (pad-seed reuse under reorg).** Closed in-circuit by the nonce-free
  `nullifier` public output (CLASS A-2: `Poseidon2(DOMAIN ‖ pad_seed ‖ slot)`, which —
  unlike the nonce-blinded `pad_source_commitment` — is identical across reuses) plus
  the reorg-safe registry `descender_account_stark::pad_seed_registry::PadSeedRegistry`
  (injective `admitted_for` + `reorg_to` freeing of orphaned admissions + strictly
  monotone admission heights). The Lean `GlobalPadSeedRegistry` models the injective
  canonical guard; the Rust adds reorg-freeing.
- **Spend single-use (consume-once).** A spend authorizes a live prevalidated slot;
  the same slot must not authorize a second accepted spend in any valid successor
  state (`research/spend_authorization_relation_v1_spec.md`).

Both need a finality profile (deployment decision "D2") that defines when an admission
or consumption is permanent versus reorg-freeable. The registry already exposes
`reorg_to(new_head)`, which frees admissions recorded above `new_head`.

## Decision

Wire both mechanisms into the sovereign rollup's consensus and bind their reorg-free
boundary to the chain's finality gadget:

1. **Admission.** At block validation, the admission path checks each creation's
   in-circuit `nullifier` against the registry. A nullifier already `admitted_for` on
   the canonical chain is rejected (the cross-account, persistent single-use guard).
   The key is the in-circuit-bound nullifier, never an off-circuit-chosen value.

2. **Spend lifecycle.** A spend is admitted only if its slot — keyed on the
   in-circuit-bound slot / `slot_authorization_digest` — is live and absent from the
   consume-once set; acceptance moves the slot to consumed. The key is an
   in-circuit-bound public value, never off-circuit-chosen.

3. **Finality (D2).** The canonical (permanent) boundary for both the registry and the
   consume-once set is the chain's FINALIZED head. Admissions and consumptions at or
   below the finalized head are permanent; the reorg-able window `[finalized, head]` is
   freed on reorg by `reorg_to(new_head)` with `new_head` at or above the finalized
   head. Strictly-monotone admission heights (already enforced by the registry) order
   this window. The finality gadget itself is the rollup's consensus concern; this ADR
   fixes only that the registry/lifecycle permanence boundary equals finalized head.

## Consequences

- The pad-seed registry and the consume-once set are consensus state, updated at block
  validation and rewound on reorg via `reorg_to`; this requires `ADR-0009`'s consensus
  to expose a finalized head.
- Single-use (F2) and consume-once both key on in-circuit-bound public outputs
  (`nullifier`, slot/authorization digest), so an adversary cannot present a
  fresh-looking duplicate; the in-circuit binding is the load-bearing part, the
  registry/ledger are the persistence layer.
- Below finality, reorgs free admissions/consumptions (liveness under reorg); at and
  below finality they are irrevocable (safety). This is the explicit D2 profile that
  the F2 boundary note (`research/cryptanalysis/assumption_boundary_stress_test_20260627.md`)
  left to a deployment decision.
- A future implementation must add the consume-once ledger alongside the existing
  `PadSeedRegistry` and call both from the validation path; the in-circuit pieces
  (nullifier, slot binding, key binding) already exist.
- External third-party review remains required before production (`ADR-0008`).

## Supersedes

None. Depends on `ADR-0009` (sovereign rollup) and `ADR-0002` (Path 2 prevalidated
slots).

## Superseded By

None.

## Affected Specs / Tracker Items

- `research/spend_authorization_relation_v1_spec.md`
- `research/cryptanalysis/assumption_boundary_stress_test_20260627.md`
- `research/path2_q128_build_status_and_architecture_v1.md`
- `TRACKER.md` (`AGG-ROLLUP-001`)
