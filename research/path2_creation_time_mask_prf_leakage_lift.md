# Path 2 Creation-Time Mask-PRF Leakage Lift

Status: proof-side lift of the existing selected-projection mask-PRF leakage
work into the account-creation / prevalidated-slot model.

## Purpose

The existing mask-PRF leakage composition was written for the selected
308-byte/Fq12 live-slot and spend-opening route. Path 2 changes the phase
boundary: the hidden-source relation is checked at account creation, and spend
only consumes an admitted authorization digest.

This document states how the same mask-PRF hiding claim is reused for live
prevalidated slots.

## Phase Translation

| old 308-byte route | Path 2 route |
|---|---|
| live account slot exists before spend | live prevalidated slot exists after account creation |
| 308-byte spend envelope carries projection carrier | creation evidence/certificate validates carrier relation before admission |
| accepted spend may export selected opening fields | standard Path 2 spend does not reopen hidden-source material |
| live-slot leakage is charged before spend | live-slot leakage is charged at admission, including never-spent slots |

The simulator target becomes:

```text
Adv_path2_live_slot_masked_projection_leakage(q_slot, q_open, q_life, q_hash, q_reject, t)
```

where `q_slot` counts admitted prevalidated slots, not only slots that later
produce accepted spends.

## Standard Path 2 Public View

The standard public view includes:

- `PrevalidatedSlotLeafV1` metadata or a direct record/root that reconstructs it;
- `slot_authorization_digest`;
- public creation-evidence metadata;
- public digest/commitment bindings;
- any public proof/certificate bytes classified as hiding or public relation
  statements;
- masked live-slot state already allowed by the older simulator.

The standard public view excludes:

- raw pad seed;
- projection-mask residues;
- unmasked selected account projection residues;
- private account-witness projection opening frames;
- raw state-assisted opening tuple fields that reveal private witness material.

## Lifted Hybrid

The existing hybrid step still applies:

```text
masked_projection_i = account_projection_i + projection_mask_i mod q
```

Before a public opening is charged, replace the six masked projection residues
with independent uniform residues in `Z_q`, preserving all public fields not
deterministically derived from those residues.

Path 2 changes the conditioning event:

- old route: before accepted spend opening;
- Path 2: before any public creation-opening model is enabled.

Under the standard Path 2 theorem, no spend-time hidden-source opening is
published, so there is no ordinary spend-opening disclosure term. If creation
admission publishes raw opening material, the theorem must switch to the
`public_creation_opening_model` named in the live-slot leakage ledger.

## Bound Shape

The lifted direct-mode leakage term is:

```text
Adv_path2_live_slot_leakage <=
    Adv_mask_prf_projection_domain(q_mask, t)
  + Adv_pad_seed_preimage_or_reuse(q_pad, q_slot)
  + Adv_projection_overread(q_reject)
  + Adv_public_creation_opening(q_open)
  + Adv_lifecycle_rewrite_or_replay(q_life)
  + Adv_qrom_domain_separation(q_hash)
```

`Adv_public_creation_opening(q_open)` is zero only if the creation certificate
API is constrained to public relation/proof views and the KATs prove no raw pad
seed, projection masks, or unmasked selected account projection residues are
serialized or emitted.

## New Same-Game Requirements

The old simulator work is reusable only after these Path 2 conditions are in
the same game:

- slot admission fixes `pad_seed_digest`, `slot_id`, lifecycle anchor, verifier
  version, and account namespace before spend challenge derivation;
- duplicate `pad_seed_digest` is rejected in the selected uniqueness domain;
- creation evidence payload is available and bound to the same leaf;
- rejecting creation-verifier paths emit public errors and no private opening
  material;
- spend authorization consumes only `slot_authorization_digest` and live state,
  not raw hidden-source openings;
- lifecycle rollback cannot restore a consumed or rewritten live slot without a
  named lifecycle break.

## Executable Evidence Already Present

| evidence | role |
|---|---|
| selected-projection no-export Rust tests | prove malformed active-lane paths do not emit selected projection material |
| public account-state digest-only tests | prove account state snapshots expose commitments/digests, not private openings |
| Path 2 frame/evidence KATs | prove leaf/evidence/payload binding and cross-leaf replay rejection for the wrapper |
| `path2_public_admission_view_serializes_only_public_material` | proves direct public admission JSON excludes raw pad seed, projection masks, unmasked projection residues, and private opening markers |
| `path2_creation_relation_replay_rejections_do_not_export_private_material` | proves representative Path 2 relation-replay rejection paths do not export private material |
| `path2_prevalidated_slot_admission_namespace_rejects_duplicates` | rejects duplicate slot ids and duplicate pad-seed digests before admission |
| `path2_direct_single_slot_spend_composes_admission_signature_and_consume_once` | proves spend authorization consumes only the admitted authorization digest and live state, not hidden-source openings |
| domain registry artifact | reserves direct/batch creation and consume domains without duplicate/prefix collisions |

## Current Executable Closure

For direct single-slot mode, the same-game requirements above now have
executable witnesses for the current public admission API and representative
relation-replay rejection paths. Future compact proof payloads or batch
aggregation payloads must add their own serialization/no-export KATs before
they can inherit this leakage lift.

## Conclusion

The mask-PRF assumption is reusable for Path 2 direct mode under the constrained
creation certificate API now modeled by the public admission view. The proof
obligation is no longer "hide selected projection residues until spend
opening"; it is "hide selected projection residues for every live prevalidated
slot unless the creation payload itself publishes an explicitly charged
opening." The current direct public view stays in the standard hiding model;
any future public payload that serializes raw opening material must switch to
`public_creation_opening_model`.
