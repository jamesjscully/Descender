# Path2AccountStarkReplayAnchorBindingTheoremV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Poseidon2/Plonky3
> account-creation path (`ADR-0004`, tracker row `DIR-BIND-001`). It is the
> theorem-level replay-payload / deployed-anchor binding for the account-STARK
> creation proof, discharging the `R_wiring` clause handed off by
> `research/path2_account_stark_arithmetization_soundness_theorem_v1.md`. Not
> normative by itself.

Status: replay-payload and deployed-anchor binding theorem.

## Obligation

A valid creation proof must not be replayable across **payloads, verifier
versions, masked projections, role-root exports, or packed-witness commitments**;
a proof accepted for one deployed slot must not authorize a different
slot/account state. `DIR-BIND-001` pins the bindings in code
(`tools/descender-account-verifier/src/lib.rs`, negative-tested by
`tests/deployed_binding.rs`) but leaves theorem-level closure open. Named break:
`account_stark_payload_binding_break`.

## The deployed anchors

`DeployedCreationAnchors` (`descender-account-verifier/src/lib.rs:83-98`) and the
verifier bind the proven public statement `pv` to the deployed leaf:

| anchor | binding | source |
|---|---|---|
| `account_id` (32 B) | direct equality, in-AIR-bound | L6, verifier eq |
| `account_state_root` (32 B) | direct equality, transcript-bound | L6, verifier eq |
| `verifier_version` (u32) | direct equality, transcript-bound | L6, verifier eq |
| `masked_projection[6]` (u16) | direct equality | L4, verifier eq |
| `native_role_root_commitment` (32 B) | **export bridge** `sha256(DS ‖ role_root)` | L7 + `role_root_export_digest` |
| `packed_witness_commitment` (32 B) | direct equality | L8, verifier eq |

"L#" = the lemma of the arithmetization-soundness theorem (#1) that forces `pv`'s
field to equal the in-circuit witness/commitment; "verifier eq" = the boundary
equality check that forces `pv`'s field to equal the deployed anchor.

## Theorem

> **Theorem (deployed-anchor binding).** Let `π` be an account-STARK proof
> accepted by `verify_path2_direct_creation_account_stark_payload_boundary`
> against anchors `A`. Then the proven statement `pv` satisfies, for every anchor
> field `x`: `pv.x = A.x` (direct fields) and `role_root_export_digest(pv.role_root)
> = A.native_role_root_commitment` (role-root export). Consequently `π` cannot be
> accepted against any `A' ≠ A` differing in any anchor field, unless either
> (a) the prover produces a *different* accepting `pv'` — a fresh proof governed by
> the #1+#2+#4 soundness/knowledge chain, not a replay — or (b) exhibits a
> SHA-256 second-preimage/collision on the role-root export bridge
> (`domain_binding_or_qrom_break`).

### Proof

For each direct-equality anchor (`account_id`, `account_state_root`,
`verifier_version`, `masked_projection`, `packed_witness_commitment`): the
verifier rejects unless `pv.x = A.x` (boundary check), and by #1 (L6/L4/L8) `pv.x`
is forced to equal the in-circuit witness/commitment column. So acceptance against
`A` pins `pv.x = A.x`; acceptance against `A'` with `A'.x ≠ A.x` would require
`pv.x = A'.x ≠ A.x` for the *same* `π`, impossible since `π` fixes one `pv`. For
the role-root export anchor: by L7 `pv.role_root` is the Poseidon2 squeeze over the
bound commitment columns, and the verifier checks `sha256(DS ‖ pv.role_root) =
A.native_role_root_commitment`; matching a different `A'.native_role_root_commitment`
needs a SHA-256 collision/second-preimage (named break). Hence `π` binds to exactly
one anchor tuple `A`. ∎

### Replay-payload corollary

The payload binds to its replay certificate via
`path2_account_stark_creation_payload_v1_digest` (`DIR-STARK-001` roundtrip), and
`relation_id` is pinned in-AIR to the constant `RELATION_ID` (#1 L7). So:

- replaying the payload under a different certificate/context changes the payload
  digest ⟹ reject;
- reinterpreting `π` as a proof for a different relation/version is blocked by the
  pinned `relation_id` and the `verifier_version` equality;
- the six `masked_projection` residues and the `packed_witness_commitment` are
  pinned, so the syndrome + masked projection the spend's checked-core frame
  consumes are the exact ones `π` proved (L8, MED-2 closure).

### Composition: this is the `R_wiring` factor of `R_create`

`research/creation_hidden_source_relation_v1_spec.md` clause 1 also requires
`slot_authorization_digest = SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 ‖ leaf)` and
the evidence-duplication equalities. Those are SHA-256 digest equalities checked at
the verifier boundary (not in the AIR), and bind the deployed anchors `A` to the
canonical `PrevalidatedSlotLeafV1` and the `slot_authorization_digest` the spend
consumes — under SHA-256 collision/second-preimage resistance (named-standard).
Together:

```text
R_create = R_air (#1)  ∧  R_wiring (this theorem: anchor binding + SHA leaf/auth digest)  ∧  R_namespace (#9)
```

so #1 + #5 close the in-circuit relation AND its binding to the exact deployed
slot, leaving only the admission-uniqueness `R_namespace` (#9) and the lifecycle
profile.

## Adversarial Self-Check

- **Swap one anchor, keep the proof.** Rejected: the swapped field breaks its
  direct equality (or the export bridge), and `π` fixes one `pv`. Negative-tested
  (`tests/deployed_binding.rs`: a disagreeing `packed_witness_commitment` rejects).
- **Forge a colliding role_root export.** Requires SHA-256 collision — named break,
  not a binding gap.
- **Replay across verifier versions.** `verifier_version` is a bound anchor AND a
  transcript field AND (single mode) an in-AIR-pinned ctx column (#1 L6, MED-1), so
  a proof for version `v` cannot be admitted under version `v' ≠ v`.
- **Honest residual.** (1) The SHA-256 leaf/auth-digest and role-root export bridge
  rest on SHA-256 CR/SPR — named-standard. (2) Frozen mutation KATs for every
  anchor field are required (overlaps #8, `DIR-REJECT-001`). (3) External review.
  No theorem flag flipped; the binding stays code+test+this-theorem, not `verified`.

## What This Closes And Hands Off

- **Closes:** the theorem-level deployed-anchor + replay-payload binding (the
  `R_wiring` factor), composing #1's in-AIR bindings with the verifier-boundary
  equalities and SHA-256 CR/SPR.
- **Hands off:** frozen anchor-mutation KATs (#8); admission-uniqueness
  `R_namespace` and lifecycle (#9); external review. Unblocks #1's full
  accepting-verifier closure (its `R_wiring` hand-off is now discharged at paper
  level).
