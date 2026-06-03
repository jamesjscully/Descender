# Path 2 Native Byte / State Model V1

Status: refreshed after spend-projection strengthening.

Purpose: reset size planning around the active native boundary `q`, rather than
the older masked-trace or lattice-sigma proof envelopes.

This is not a final encoding. It is a planning model for the selected native
design:

```text
q : Q_c
obs_c(q)=0
SpendAccept(c,q,sig,msg,state)
```

## Model Rule

Count only public semantic projections and byte/data-availability bindings for
`q`.

Do not count these as account state unless later required:

- private authorization-local material;
- private gauge/randomizer material;
- typed role restrictions;
- transparent proof metadata;
- old masked-trace proof openings;
- old parked lattice-sigma proof payload.

Those remain either private signer state or fallback proof costs.

## Projection Surfaces

From `research/path2_projection_surfaces_v1.md`:

```text
p_context(q)
p_account(q)
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
```

The byte model has two representations:

1. **Digest-projected boundary**: each semantic surface is represented by a
   domain-separated digest/compact handle.
2. **Expanded canonical boundary**: selected fields are stored explicitly for
   validator/data-availability ergonomics.

## Digest-Projected Boundary

Planning assumption:

```text
digest = 32 bytes
small version/mode fields = packed into context digest or 1-4 bytes when explicit
```

Minimal public account boundary:

| item | bytes | role |
|---|---:|---|
| `native_boundary_digest(q)` | 32 | canonical byte binding for full `q` |
| `p_context_digest(q)` | 32 | parameter/domain context |
| `p_account_digest(q)` | 32 | account namespace |
| `p_spend_digest(q)` | 32 | public authorization class |
| `p_lifecycle_digest(q)` | 32 | replay/nullifier/consume-once class |
| `p_state_digest(q)` | 32 | state transition boundary |
| `p_transcript_digest(q)` | 32 | challenge-binding surface |
| `obs_certificate_digest_or_flags` | 0-32 | only if obstruction check needs a compact public certificate |

Range:

```text
digest-projected q: 224-256 bytes
```

This is the compact semantic-state target.

Important:

If `p_transcript_digest(q)` is defined as a digest over the other projections,
it may not need separate persistent storage. In that case:

```text
digest-projected q: 192-224 bytes
```

## Expanded Canonical Boundary

This representation stores common consensus fields explicitly and keeps digests
for larger semantic surfaces.

Planning fields:

| item | bytes | role |
|---|---:|---|
| boundary/version/mode flags | 4-8 | version, direct/aggregate mode |
| account id / account namespace handle | 32 | `p_account(q)` |
| public key / authorization identity | 32 | `p_spend(q)` or account namespace |
| slot id / lifecycle id | 16-32 | `p_lifecycle(q)` |
| account state root before | 32 | `p_state(q)` |
| lifecycle anchor / replay scope | 32 | `p_lifecycle(q)` |
| spend authorization digest | 32 | byte binding for `p_spend(q)` |
| explicit aggregation mode | 0-4 | usually packed in flags/context; semantic field of `p_spend(q)` |
| explicit state-transition class | 0-32 | only if not already digest-bound through `p_state(q)` |
| native boundary digest | 32 | byte binding for canonical `q` |
| transcript surface digest | 32 | byte binding for `p_transcript(q)` |
| optional data-availability / statement digest | 0-32 | if boundary bytes not stored inline |

Range:

```text
expanded canonical q: 244-296 bytes
```

If both `slot_id` and lifecycle anchor are 32-byte handles and an additional
statement digest is required:

```text
expanded canonical q stress: 328 bytes
```

After `research/path2_spend_projection_completeness_audit_v1.md` and
`research/path2_projection_encoding_instantiation_v1.md`, `p_spend(q)` also
semantically contains:

```text
allowed_state_transition_class
aggregation_mode
```

The active encoding decision edge-links `allowed_state_transition_class`
through `p_state(q)` and packs `aggregation_mode` into existing mode/context
bytes. Byte impact is:

```text
0-4 bytes
```

for explicit mode bits.

If implementation later shows `allowed_state_transition_class` must be stored as
its own digest rather than edge-linked to `p_state(q)`, byte impact is:

```text
+32 bytes
```

New expanded stress range:

```text
expanded canonical q stress after projection strengthening: 328-360 bytes
```

## Account Creation Public Bytes

Native account creation should publish either:

```text
canonical q bytes
```

or:

```text
digest-projected q + data availability for expanded q
```

Planning range:

| representation | account creation public bytes | notes |
|---|---:|---|
| digest-projected inline | 192-256 | compact semantic handles only |
| expanded canonical inline | 244-360 | explicit consensus fields, including strengthened spend class |
| expanded + small obstruction certificate | 276-424 | if `obs_c(q)=0` needs a public cert |
| fallback transparent proof overlay | 8704-16096 | old proof-family route only |
| parked lattice-sigma fallback | 16416 | fallback only |

Current native target:

```text
account creation public bytes: 244-392
```

before any residual proof system if the state-transition class is edge-linked
through `p_state(q)`.

Conservative strengthened range:

```text
account creation public bytes: 244-424
```

This extra 32-byte headroom is not mandatory; it is the cost of storing
`allowed_state_transition_class` as an independent explicit digest instead of a
projection/edge of `p_state(q)`.

## Account State Bytes

Persistent account state should store enough to support:

```text
SpendAccept(c,q,sig,msg,state)
```

without re-opening private data.

Minimal persistent state:

| item | bytes | role |
|---|---:|---|
| native boundary digest | 32 | account boundary identity |
| spend authorization digest / handle | 32 | `p_spend(q)` binding |
| lifecycle/nullifier state | 32 | `p_lifecycle(q)` binding |
| state transition / predecessor root handle | 32 | `p_state(q)` binding |
| context/version handle | 16-32 | parameter/domain binding |
| live/consumed marker | 1-8 | lifecycle status |
| aggregation/state-class flags | 0-4 | if not already packed in context/state handles |

Range:

```text
minimal account state: 145-168 bytes
```

After projection strengthening:

```text
minimal account state: 145-172 bytes
```

if aggregation/state-class fields are packed as small flags. If a separate
state-transition-class digest is stored in persistent account state, add 32
bytes:

```text
minimal account state stress: 177-204 bytes
```

Expanded state with explicit account id, public key identity, slot id,
lifecycle anchor, and data-availability pointer:

```text
expanded account state: 224-320 bytes
```

Strengthened expanded state:

```text
expanded account state: 224-352 bytes
```

with the top end only when a separate state-transition-class digest is persisted
outside the existing `p_state(q)` handle.

This is far below the earlier 1320-byte state pressure, because private proof
payload material and role openings are not stored as semantic state.

## Signer Private State Bytes

Signer private state now lives in:

```text
AuthorizationLocal_c(q)
Gauge_c(q)
Randomizer_c(q)
```

Planning components:

| item | bytes | role |
|---|---:|---|
| authorization-local secret/opening material | 32-96 | spend production |
| gauge/local trivialization | 32-128 | private representative freedom |
| randomizer/mask seed material | 32-96 | hiding/sampling |
| recovery/context metadata | 32-96 | wallet-local, not consensus |

Planning range:

```text
signer private state: 128-416 bytes
```

This can be larger without blockchain state impact.

## Spend Signature Bytes

Native account redesign should not increase spend signature bytes unless
checked-core frame binding over `q` requires extra public inputs in the
signature.

Current target:

```text
spend signature bytes: unchanged from checked-core path
```

Compatibility rule:

Any added spend bytes must be justified as:

```text
needed to bind p_transcript(q) or p_lifecycle(q)
```

not as account-creation proof carryover.

Planning:

| case | spend signature impact |
|---|---:|
| checked-core frame consumes `q` projections from state | 0 |
| signature carries native boundary digest explicitly | +32 |
| signature carries slot authorization digest already present elsewhere | 0-32 |
| fallback public opening/proof material | reject for native path |

## Verifier Compute

Native verification at account creation:

```text
parse canonical q
check context/domain
check pullback equality
check gluing/meaning/spend-state/canonicality defects
compute digest bindings
```

Planning:

| component | compute |
|---|---|
| digest/domain checks | small number of hashes |
| pullback/context checks | equality checks |
| semantic projection checks | equality/digest checks |
| obstruction gluing check | open: likely q/module operations |
| lifecycle/state checks | state DB lookups + equality |

The only potentially nontrivial native compute is:

```text
O_gluing
```

If `O_gluing` can be a small q12289/module syndrome check, verifier compute is
small. If it requires a residual proof, the byte model must add that proof as a
separate fallback layer.

## Residual Assumptions

Native structural assumptions:

- projection completeness;
- obstruction soundness;
- public admission soundness;
- honest creation spendability;
- authorization class binding;
- spend closure over `q`;
- gauge invariance.
- fiber privacy/no-export.

Cryptographic assumptions:

- hash/domain binding for canonical encodings;
- checked-core soundness over `checked_core_frame(q,msg,state)`;
- authorization-local hiding/sampling if private auth is not information
  theoretically hidden;
- QROM only for Fiat-Shamir/challenge frames actually used.

Fallback proof-system assumptions:

- transparent full-trace extraction;
- relation knowledge;
- no-export/ZK;
- lattice/SIS sigma repetition soundness.

These are not part of the native byte target unless the native obstruction/lift
theorems fail.

## Masked-Trace Fallback Decision

Keep the masked-trace route as fallback until two native obligations are
resolved:

```text
1. concrete O_gluing/public admission soundness;
2. honest creation spendability;
3. authorization-local class binding over p_spend(q);
4. checked-core frame binding;
5. fiber privacy/no-export.
```

If both are solved structurally, the masked-trace proof envelope should be
marked fallback-only and excluded from active byte estimates.

If either fails, add the smallest residual proof/certificate needed and update
this model.

## Current Summary

Active native planning numbers:

```text
account creation public bytes: 244-392
account state bytes:           145-320
signer private state bytes:    128-416
spend signature impact:        0-32
```

Conservative stress planning numbers:

```text
account creation public bytes: 244-424
account state bytes:           145-352
signer private state bytes:    128-416
spend signature impact:        0-32
```

Active preferred compact target remains:

```text
account creation public bytes: 244-392
account state bytes:           145-320
```

provided `allowed_state_transition_class` is edge-linked to the canonical
state-transition class derived from `p_state(q)` and `p_context(q)` instead of
carried as a separate digest.

This condition is now stated in:

```text
research/path2_pstate_canonicality_v1.md
```

The preferred range is active only if `pstate_canonicality_v1` holds. If
`p_state(q)` degenerates to only `account_state_root_before`, the +32-byte
state-transition-class digest must become active.

Fallback proof envelope, not active native target:

```text
transparent proof overlay: 8704-16096 bytes
parked lattice-sigma:      16416 bytes
```

The native approach is therefore size-competitive if the structural
obstruction/lift theorems can be made real.

## Next Required Work

Create or update:

```text
research/path2_residual_assumption_ledger_v1.md
```

to separate:

- active native structural assumptions;
- ordinary hash/QROM/domain assumptions;
- checked-core downstream assumptions;
- fallback proof-system assumptions.

Then instantiate concrete byte encodings for `p_spend(q)` and `p_state(q)` so
the 32-byte state-transition-class headroom can be accepted or removed.
