# Path 2 Native Boundary Audit: 308b / Checked-Core Material

Status: factorization audit complete for older 308b material.

Purpose: classify the older reviewed `308 B/Fq12` projection-carrier route
against the native-boundary model:

```text
b = B_c(h)
```

This audit decides which older objects are downstream spend projections, which
remain useful negative controls, and which are dead for the compact
native-boundary account-creation theorem.

## Older 308b Envelope

The older selected envelope was:

```text
reviewed_fq12_account_carried_signature[252]
|| hidden_source_fallback_projection_carrier[56]
```

The reviewed implementation and research artifacts provide:

- checked-core signature parsing and verification;
- projection carrier decoding;
- public carrier lane/check/transcript comparison;
- response-link kernel break export;
- hidden-source opening projection break export;
- account commitment split break export;
- pad PRF projection break export;
- negative controls for helper API export/no-export behavior.

This is useful as a checked-core and negative-control baseline. It is not a
native account-creation boundary by itself.

## Classification Against `Path2NativeBoundaryV1`

| older object | native-boundary classification | disposition |
|---|---|---|
| 252-byte checked-core signature | downstream spend proof over admitted authorization projection | keep |
| checked-core challenge frame | spend transcript projection from admitted slot authorization | keep |
| checked-core response/syndrome rows | spend-time equation witness, not account boundary | keep as checked-core theorem input |
| 56-byte projection carrier | older spend-carried public projection proof/capsule | not part of active account-creation boundary |
| carrier lane/check bytes | public projection quotient evidence for old spend path | superseded by `masked_projection[6]` plus native-boundary admission |
| carrier transcript bytes | old context-binding handle | superseded by leaf/native-boundary/transparent transcript binding |
| state-assisted opening commitment | compatibility projection of `OpeningTuple_c` | keep only as projection from `b` |
| pad seed digest | compatibility projection of `PadSource_c` | keep only as projection from `b` |
| account witness/randomizer commitments | role restrictions of one section | keep only as typed restrictions from `parent_section_root` |
| old 64-variable generalized projection rows | underconstrained relation with inactive-coordinate survivor | dead |
| active-lane replay/certificate checks | negative-control and relation-shape evidence | keep as baseline evidence, not final boundary |
| public helper APIs returning opening material | leakage/no-export negative controls | keep as reject-path/API tests only |

## Factorization Result

The checked-core portion factors through the native boundary only as a
spend-time projection:

```text
Admitted(c, b)
SpendAccept(c, p_slot_authorization(b), sig, msg, state)
```

The spend verifier should not claim:

```text
SpendAccept(...) => exists full hidden-source opening
```

Instead, it consumes an authorization handle projected from the admitted
boundary. Hidden-source validity is proved at account creation:

```text
AcceptAccountCreateNative(c, b, proof)
  => exists h. B_c(h) = b
```

The 56-byte carrier does not belong in the native-boundary spend path. Its role
is replaced by:

- account creation admits `b`;
- `b` projects `masked_projection[6]` and slot authorization;
- spend-time checked-core verifies the message-bound signature and consumes the
  prevalidated slot.

## Dead Paths

Mark these paths dead for the native-boundary proof:

### 1. Old 64-variable generalized projection relation

Reason: existing replay found an inactive-coordinate survivor. It does not give
the native consistency theorem and would require extra bridge rows or a
different relation.

Disposition:

```text
[D] old_64_variable_generalized_projection_account_boundary
```

### 2. Spend-carried 56-byte carrier as the account-creation proof

Reason: the carrier can be useful as a public spend capsule, but it does not
prove that the leaf, pad source, opening tuple, projection relation, and trace
view are all projections of one `B_c(h)`.

Disposition:

```text
[D] spend_carrier_as_native_boundary_proof
```

### 3. Independent opening/pad/carrier commitments linked after the fact

Reason: this recreates the large consistency-proof bottleneck the
native-boundary construction is designed to avoid.

Disposition:

```text
[D] independent_role_commitments_plus_later_equality_proof
```

### 4. SHA preimage semantics for compact role openings

Reason: compact q12289 relation cannot afford SHA-256 preimage proof for
opening, pad, and projection objects.

Disposition:

```text
[D] sha_digest_role_handles_inside_compact_relation
```

## Kept Paths

### Checked-core message binding

The 252-byte checked-core signature remains the spend-time proof that the
message, public key, slot id, challenge, and response equation bind correctly.

Native-boundary interpretation:

```text
checked_core_authorization = p_checked_core(b)
```

where `p_checked_core(b)` is the spend authorization derived from the admitted
slot leaf/native boundary.

### State-assisted slot lifecycle

The account-state lifecycle model remains necessary: live marker, state root,
slot id, lifecycle anchor, uniqueness, and consume-once behavior are all
projections of `b` or invariants under `~safe`.

### Negative controls and fail-closed wrappers

The older 308b helper/API no-export tests remain valuable because they cover
exactly the classes of leakage that the native-boundary proof must avoid:

- raw pad seed export;
- selected account projection opening export;
- partial opening export on failure;
- carrier replay under the wrong account/key/slot/state/verifier context.

These are test or evidence artifacts, not the final theorem shape.

## Consequence For The Active Design

The active account-creation path should not try to prove the 308b carrier is a
full account witness opening. That was the old bottleneck.

The active theorem should instead prove:

```text
AcceptAccountCreateNative(c, b, proof)
  =>
exists h in H_c.
  B_c(h) = b
```

Then spend uses:

```text
p_slot_authorization(b)
```

and the checked-core verifier supplies only message binding and response
equation validity.

## Tracker Update Recommendation

The Path 2 native-boundary tracker should mark:

- current Path 2 context/leaf/transparent public view: partially factorable;
- old 308b checked-core signature: downstream spend projection;
- old 56-byte carrier: negative-control/fail-closed baseline;
- old generalized projection relation: dead;
- independent role commitments with later equality proof: dead unless replaced
  by typed restrictions under one parent section root.

## Non-Claims

- This audit does not prove checked-core soundness.
- This audit does not prove native-boundary admission.
- This audit does not remove lifecycle requirements.
- This audit does not remove no-export/ZK obligations for account creation.
- This audit does not mark the implementation deployable.
