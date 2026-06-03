# Path 2 Native Boundary Role Completeness Table

Status: mandatory role coverage pinned; concrete column ids pending.

Purpose: ensure `ExtractedRolesComplete(b, profile, h)` is not a vague claim.
Every role needed for local validity, descent consistency, or the boundary
projection must be present in the profile and extractable from the full trace.

## Role Coverage

| role | required for `LocalValid_c` | required for `DescentConsistent_c` | required for `B_c(h)=b` | may vary under `~safe` | notes |
|---|---:|---:|---:|---:|---|
| `context_public` | yes | yes | yes | no | fixes account, key, slot, verifier, lifecycle, state-root, and aggregation context |
| `packed_witness` | yes | yes | yes | no | carries packed residue/source coordinates and links to masked projection |
| `randomizer` | yes | yes | yes | no, except quotient-safe encoding changes | may be absorbed into `opening_tuple` if the handle map stays injective for safety behavior |
| `pad_source` | yes | yes | yes | no, except fresh mask/gauge choices | provides pad seed/source, pad lanes, and mask derivations |
| `opening_tuple` | yes | yes | yes | no | binds account witness opening, randomizer link, lifecycle anchor, and spend-opening coordinates |
| `projection_relation` | yes | yes | yes | no | links account projection lanes, pad projection lanes, and six public masked lanes |
| `range_witness` | yes | yes | no direct public projection | yes only among witnesses proving the same canonical residues | must be hidden under `hybridBase128HiddenRoles` |
| `blinding` | yes for binding bounds | yes | no direct public projection | yes if bounded and no safety behavior changes | extractor-side only; not a public hiding commitment |
| `constraint` | yes | yes | no direct public projection | no for accepting relation semantics | quotient/constraint trace enforces local row equations |
| `mask_domain` | yes for no-export | yes | yes through transcript semantics | no if reuse changes leakage/replay surface | binds native boundary, role, column set, row, query, and transcript |
| `lifecycle_slot` | yes | yes | yes | no | may be represented across context/opening columns rather than a separate role handle |
| `statement_digest_source` | yes | yes | yes | no | canonical source for `p_statement(b)` / creation statement digest |

## Completeness Predicate

`ExtractedRolesComplete(b, profile, h)` requires:

1. every mandatory role above is present in `profile` or derived by a pinned
   rule from another mandatory role;
2. every private role vector is extractable from `trace`;
3. every public projection in `b` is determined by extracted or public role
   data;
4. every cross-role equality needed by descent consistency is checked by
   `TraceRelationValid`;
5. every role handle in `p_role_handles(b)` authenticates exactly the matching
   role/column set under `p_section_root(b)`;
6. no role required for safety behavior is simulator-only without an extractor
   witness.

## Derived-Versus-Explicit Roles

Some roles may be derived rather than separate handles:

| role | allowed derivation |
|---|---|
| `lifecycle_slot` | from `context_public`, `opening_tuple`, and live-status columns |
| `statement_digest_source` | from canonical projections of `b` plus relation/profile ids |
| `mask_domain` | from QROM schedule and profile table |
| `constraint` | from the proof-family constraint trace, not a boundary handle |

Derived roles still count as mandatory. A profile that omits their derivation
rule is incomplete.

## Dead Profiles

Reject or export a named break for any profile that:

- omits `pad_source`, `opening_tuple`, or `projection_relation`;
- omits range witnesses while selecting `hybridBase128HiddenRoles`;
- makes `range_witness` or `blinding` publicly opened raw;
- omits mask-domain labels for masked public openings;
- treats `lifecycle_slot` as external state not bound to `b`;
- lets `randomizer` vary while preserving the same boundary but changing spend
  or account-opening behavior;
- maps two mandatory roles to the same column set without an explicit equality
  theorem;
- marks a safety-critical role as `simulator_only`.

## Current Assessment

No missing mandatory role is visible in the active native-boundary profile. The
main cleanup is to decide whether `randomizer` and `lifecycle_slot` are
explicit role handles or derived subroles of `opening_tuple` and context.

That is not a cryptographic blocker as long as the profile states the
derivation rule and `TraceAssemblySoundness` consumes it.

## Next Move

Promote this table into the Lean interface by adding role-completeness
well-formedness predicates for:

```text
MandatoryRoleCovered(profile, role)
ProfileRoleComplete(profile)
```

The predicates can remain abstract, but naming them prevents a future proof
from satisfying `ExtractedRolesComplete` with a partial profile.

Done:
`Descender/Path2NativeBoundary.lean` now defines `MandatoryRoleCovered` and
`ProfileRoleComplete`. The final active endpoint now routes through
`ParentTraceExtractionSoundness` and `TraceAssemblySoundness`, and
`TraceAssemblySoundness` requires `ProfileRoleComplete profile`, so a partial
profile cannot satisfy the active extractor-completeness surface.

## Non-Claims

- This does not assign concrete column indices.
- This does not prove the mandatory roles are extractable.
- This does not decide final encoding of derived roles.
